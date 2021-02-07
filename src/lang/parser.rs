use super::lexer::Token;
use logos::Logos;

/**
 * Grammar:
 * scene := definition +
 * definition := identifier "{"
 *  assignment+
 * "}"
 * assignment = identifier "=" value
 * identifier = [a-zA-Z]+
 * value = f64 | String | (f64, f64, f64)
 */

#[derive(Debug)]
pub(crate) enum Definition {
    PointLight {
        intensity: f64,
        position: (f64, f64, f64),
    },
    DirectionLight {
        intensity: f64,
        direction: (f64, f64, f64),
    },
    AmbientLight {
        intensity: f64,
    },
    Sphere {
        color: (f64, f64, f64),
        center: (f64, f64, f64),
        radius: f64,
    },
    Window {
        title: Option<String>,
        width: Option<f64>,
        height: Option<f64>,
    },
}

impl Definition {
    fn from_raw(raw: RawDefinition) -> Result<Self, String> {
        match &raw.def_type[..] {
            "window" => Self::window_from_raw(raw),
            "light" => Self::light_from_raw(raw),
            "sphere" => Self::sphere_from_raw(raw),
            t => Err(format!("Unsupported definition type: {}", t)),
        }
    }

    fn window_from_raw(raw: RawDefinition) -> Result<Self, String> {
        let mut title = None;
        let mut width = None;
        let mut height = None;

        for assignment in raw.assignments {
            match &assignment.name[..] {
                "width" => width = Some(Self::numeric_value(assignment.value, "width")?),
                "height" => height = Some(Self::numeric_value(assignment.value, "height")?),
                "title" => title = Some(Self::string_value(assignment.value, "title")?),
                s => {
                    return Err(format!(
                        "Expected properties: [width, height, title] but got: '{}'",
                        s
                    ))
                }
            }
        }

        Ok(Definition::Window {
            title,
            width,
            height,
        })
    }

    fn light_from_raw(raw: RawDefinition) -> Result<Self, String> {
        let mut light_type = None;
        let mut intensity = None;
        let mut position = None;
        let mut direction = None;

        for assignment in raw.assignments {
            match &assignment.name[..] {
                "type" => light_type = Some(Self::string_value(assignment.value, "type")?),
                "intensity" => {
                    intensity = Some(Self::numeric_value(assignment.value, "intensity")?)
                }
                "position" => position = Some(Self::tuple_value(assignment.value, "position")?),
                "direction" => direction = Some(Self::tuple_value(assignment.value, "direction")?),
                s => {
                    return Err(format!(
                        "Expected properties: [type, intensity, position, direction] but got: '{}'",
                        s
                    ))
                }
            }
        }
        if light_type.is_none() || intensity.is_none() {
            return Err("light definitions require a type and an intensity".into());
        }

        match &light_type.unwrap()[..] {
            "ambient" => {
                if position.is_some() || direction.is_some() {
                    return Err("Only type and intensity are supported for ambient lights".into());
                }
                Ok(Definition::AmbientLight {
                    intensity: intensity.unwrap(),
                })
            }
            "point" => {
                if position.is_none() {
                    Err("point lights require a position".into())
                } else if direction.is_some() {
                    Err("point lights do not support the direction property".into())
                } else {
                    Ok(Definition::PointLight {
                        intensity: intensity.unwrap(),
                        position: position.unwrap(),
                    })
                }
            }
            "directional" => {
                if direction.is_none() {
                    Err("directional lights require a direction".into())
                } else if position.is_some() {
                    Err("directional lights do not support the position property".into())
                } else {
                    Ok(Definition::DirectionLight {
                        intensity: intensity.unwrap(),
                        direction: direction.unwrap(),
                    })
                }
            }
            s => Err(format!("Unsupported light type: {}", s)),
        }
    }

    fn sphere_from_raw(raw: RawDefinition) -> Result<Self, String> {
        let mut color = None;
        let mut center = None;
        let mut radius = None;

        for assignment in raw.assignments {
            match &assignment.name[..] {
                "color" => color = Some(Self::tuple_value(assignment.value, "color")?),
                "center" => center = Some(Self::tuple_value(assignment.value, "center")?),
                "radius" => radius = Some(Self::numeric_value(assignment.value, "radius")?),
                s => {
                    return Err(format!(
                        "Expected properties: [color, center, radius] but got: '{}'",
                        s
                    ))
                }
            }
        }
        if color.is_none() || center.is_none() || radius.is_none() {
            Err(
                "Sphere definitions require [color, center, radius] but some values are missing"
                    .into(),
            )
        } else {
            Ok(Definition::Sphere {
                color: color.unwrap(),
                center: center.unwrap(),
                radius: radius.unwrap(),
            })
        }
    }

    fn numeric_value(value: Value, property: &str) -> Result<f64, String> {
        match value {
            Value::Num(n) => Ok(n),
            _ => Err(format!(
                "Expected number for property {} but got {:?}",
                property, value
            )),
        }
    }

    fn string_value(value: Value, property: &str) -> Result<String, String> {
        match value {
            Value::VString(s) => Ok(s),
            _ => Err(format!(
                "Expected string for property {} but got {:?}",
                property, value
            )),
        }
    }

    fn tuple_value(value: Value, property: &str) -> Result<(f64, f64, f64), String> {
        match value {
            Value::Tuple(t) => Ok(t),
            _ => Err(format!(
                "Expected tuple for property {} but got {:?}",
                property, value
            )),
        }
    }
}

struct RawDefinition {
    def_type: String,
    assignments: Vec<Assignment>,
}

struct Assignment {
    name: String,
    value: Value,
}

#[derive(Debug)]
enum Value {
    Num(f64),
    VString(String),
    Tuple((f64, f64, f64)),
}

pub(crate) struct Parser {
    src: Vec<Token>,
    position: usize,
}

impl Parser {
    pub(crate) fn new(src: &str) -> Self {
        let lexemes = Token::lexer(src);
        Self {
            src: lexemes.collect(),
            position: 0,
        }
    }

    fn peek<'a>(&'a self) -> Option<&'a Token> {
        if self.position >= self.src.len() {
            None
        } else {
            Some(&self.src[self.position])
        }
    }

    fn next<'a>(&'a mut self) -> Option<&'a Token> {
        if self.position >= self.src.len() {
            None
        } else {
            let res = Some(&self.src[self.position]);
            self.position += 1;
            res
        }
    }

    pub(crate) fn parse(&mut self) -> Result<Vec<Definition>, String> {
        let mut definitions = vec![];
        self.munch_newlines();
        while let Some(x) = self.peek() {
            definitions.push(self.parse_raw_definition()?);
            self.munch_newlines();
        }

        definitions
            .into_iter()
            .map(|def| Definition::from_raw(def))
            .collect()
    }

    fn munch_newlines(&mut self) {
        while let Some(Token::NewLine) = self.peek() {
            self.next();
        }
    }

    fn parse_raw_definition(&mut self) -> Result<RawDefinition, String> {
        let def_type =
            self.expect_ident("Object definitions should start with a definition type")?;
        self.expect(
            &Token::LBrace,
            "A definition should be opened by a curly brace '{'",
        )?;
        self.munch_newlines();
        let mut assignments = vec![];
        assignments.push(self.parse_assignment()?);
        loop {
            match self.peek() {
                None => return Err("Unexpected EOF when parsing definition".into()),
                Some(Token::NewLine) => { self.next(); }
                Some(Token::RBrace) => {
                    self.next();
                    break
                },
                Some(Token::Identifier(_)) => {
                    assignments.push(self.parse_assignment()?)
                }
                Some(tok) => return Err(format!("Unexpected token {:?} when parsing definition; expected assignment or closing brace", tok))
            }
        }

        Ok(RawDefinition {
            def_type,
            assignments,
        })
    }

    fn parse_assignment(&mut self) -> Result<Assignment, String> {
        let name = self.expect_ident("Assignments should start with identifiers")?;
        self.expect(&Token::Equal, "Expected = when parsing assignment")?;
        let value = self.parse_value()?;
        self.expect(
            &Token::NewLine,
            "Expect assignments to be terminated by newlines",
        )?;

        Ok(Assignment { name, value })
    }

    fn parse_value(&mut self) -> Result<Value, String> {
        match self.next() {
            None => Err("Unexpected EOF when parsing a value".into()),
            Some(v) => match v {
                Token::Number(n) => Ok(Value::Num(*n)),
                Token::VString(s) => Ok(Value::VString(strip_speechmarks(s.clone()))),
                Token::Identifier(s) => Ok(Value::VString(s.clone())),
                Token::LParen => self.parse_tuple(),
                _ => Err(format!("Invalid value: {:?}", v)),
            },
        }
    }

    fn parse_tuple(&mut self) -> Result<Value, String> {
        let num1 = self.expect_number("Tuples can only contain numbers")?;
        self.expect(
            &Token::Comma,
            "Expected a comma to separate values in tuple",
        )?;
        let num2 = self.expect_number("Tuples can only contain numbers")?;
        self.expect(
            &Token::Comma,
            "Expected a comma to separate values in tuple",
        )?;
        let num3 = self.expect_number("Tuples can only contain number")?;
        self.expect(&Token::RParen, "Expected a right paren to close tuple")?;

        Ok(Value::Tuple((num1, num2, num3)))
    }

    fn expect<'a>(&'a mut self, expected: &Token, failed_match: &str) -> Result<&'a Token, String> {
        let actual = self
            .next()
            .ok_or(format!("Expected {:?} but got EOF", expected))?;

        if actual == expected {
            Ok(actual)
        } else {
            return Err(format!("Error: {} on token {:?}", failed_match, actual));
        }
    }

    fn expect_number(&mut self, err: &str) -> Result<f64, String> {
        let value = self
            .next()
            .ok_or(format!("Expected a number but got EOF"))?;

        if let Token::Number(n) = value {
            Ok(*n)
        } else {
            return Err(format!("Error: {} on token {:?}", err, value));
        }
    }

    fn expect_ident(&mut self, err: &str) -> Result<String, String> {
        let value = self
            .next()
            .ok_or(format!("Expected an ident but got EOF"))?;

        if let Token::Identifier(s) = value {
            Ok(s.clone())
        } else {
            return Err(format!("Error: {} on token {:?}", err, value));
        }
    }
}

fn strip_speechmarks(src: String) -> String {
    src.trim_matches('"').into()
}
