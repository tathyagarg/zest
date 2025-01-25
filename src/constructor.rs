use crate::tokeniser;
use std::collections::VecDeque;

const KEYWORDS: [&str; 3] = ["scene", "object", "camera"];

pub struct Constructor {
    pub tokens: VecDeque<tokeniser::Token>,
    engine: Engine,
    step: Step,
}

#[derive(Debug)]
pub struct Engine {
    pub scene: Scene,
}

#[derive(Debug)]
pub struct Scene {
    pub objects: Vec<Object>,
}

#[derive(Debug)]
pub struct Object {
    pub name: String,
    pub properties: Vec<Property>,
}

#[derive(Debug)]
pub struct Property {
    pub name: String,
    pub value: Expression,
}

#[derive(Debug)]
pub struct Expression {
    pub tokens: Vec<tokeniser::Token>,
}

#[derive(Copy, Eq, PartialEq, Clone)]
pub enum Step {
    Start,
    Scene,
    Object,
    PropertyName,
    PropertyValue,
    ObjectEnd,
    End,
}

impl Constructor {
    pub fn new(tokens: VecDeque<tokeniser::Token>) -> Self {
        Self {
            tokens,
            engine: Engine {
                scene: Scene {
                    objects: Vec::new(),
                },
            },
            step: Step::Start,
        }
    }

    pub fn construct(&mut self) {
        let mut curr = self.step;
        while curr != Step::End {
            curr = match curr {
                Step::Start => self.start(),
                Step::Scene => self.scene(),
                Step::Object => self.object(),
                Step::PropertyName => self.property_name(),
                Step::PropertyValue => self.property_value(),
                Step::ObjectEnd => self.object_end(),
                Step::End => Step::End,
            };
        }
    }

    pub fn pop_front(&mut self) -> tokeniser::Token {
        match self.tokens.pop_front() {
            Some(token) => token.clone(),
            None => tokeniser::Token::Unknown,
        }
    }

    pub fn peek(&self) -> tokeniser::Token {
        match self.tokens.front() {
            Some(token) => token.clone(),
            None => tokeniser::Token::Unknown,
        }
    }

    pub fn ensure(&mut self, token: tokeniser::Token) {
        let t = self.pop_front();
        if t != token {
            panic!("Expected {:?}, got {:?}", token, t);
        }
    }

    pub fn start(&mut self) -> Step {
        self.ensure(tokeniser::Token::Identifier("scene".to_string()));
        Step::Scene
    }

    pub fn scene(&mut self) -> Step {
        self.ensure(tokeniser::Token::LBrace);
        Step::Object
    }

    pub fn object(&mut self) -> Step {
        let name = self.pop_front();
        if let tokeniser::Token::Identifier(name) = name {
            self.engine.scene.objects.push(Object {
                name,
                properties: Vec::new(),
            });
        }
        self.ensure(tokeniser::Token::LBrace);

        Step::PropertyName
    }

    pub fn property_name(&mut self) -> Step {
        if self.peek() == tokeniser::Token::RBrace {
            self.ensure(tokeniser::Token::RBrace);
            return Step::ObjectEnd;
        }

        self.ensure(tokeniser::Token::Dot);

        let name = self.pop_front();
        if let tokeniser::Token::Identifier(name) = name {
            self.engine
                .scene
                .objects
                .last_mut()
                .unwrap()
                .properties
                .push(Property {
                    name,
                    value: Expression { tokens: Vec::new() },
                });
        }

        self.ensure(tokeniser::Token::Equal);
        Step::PropertyValue
    }

    pub fn property_value(&mut self) -> Step {
        let value = self.expression();
        let obj = self.engine.scene.objects.last_mut().unwrap();
        let prop = obj.properties.last_mut().unwrap();
        prop.value = value;

        if self.peek() != tokeniser::Token::RBrace {
            self.ensure(tokeniser::Token::Comma);
        }

        Step::PropertyName
    }

    pub fn expression(&mut self) -> Expression {
        let front = self.pop_front();
        match front {
            tokeniser::Token::Number(_) | tokeniser::Token::Identifier(_) => Expression {
                tokens: vec![front],
            },
            tokeniser::Token::LParen => {
                let mut tokens = Vec::new();
                loop {
                    let token = self.pop_front();
                    match token {
                        tokeniser::Token::RParen => break,
                        _ => tokens.push(token),
                    }
                    if self.peek() != tokeniser::Token::RParen {
                        self.ensure(tokeniser::Token::Comma);
                    }
                }
                Expression { tokens }
            }
            _ => panic!("Unexpected token {:?}", front),
        }
    }

    pub fn object_end(&mut self) -> Step {
        if self.peek() == tokeniser::Token::RBrace {
            self.ensure(tokeniser::Token::RBrace);
            return Step::End;
        }
        Step::Object
    }
}
