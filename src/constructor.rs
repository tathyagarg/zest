use crate::tokeniser;
use std::collections::VecDeque;

const SCENE: &str = "SCENE";
const OBJECT: &str = "OBJECT";
const CAMERA: &str = "CAMERA";
const LIGHT: &str = "LIGHT";
const PHYSICS: &str = "PHYSICS";
const MATERIAL: &str = "MATERIAL";
const CONTROLLER: &str = "CONTROLLER";
const SPHERE: &str = "SPHERE";
const RECTANGLE: &str = "RECTANGLE";
const IMAGE: &str = "IMAGE";
const ACTIVE: &str = "ACTIVE";

const OBJECT_TYPES: [&str; 10] = [
    OBJECT, CAMERA, LIGHT, PHYSICS, MATERIAL, CONTROLLER, SPHERE, RECTANGLE, IMAGE, ACTIVE,
];

pub struct Constructor {
    pub tokens: VecDeque<tokeniser::Token>,
    pub engine: Engine,
    step: Step,
}

#[derive(Debug)]
pub struct Engine {
    pub scene: Scene,
}

#[derive(Debug)]
pub struct Scene {
    pub name: String,
    pub objects: Vec<Object>,
}

#[derive(Debug)]
pub struct Object {
    pub name: String,
    pub obj_type: ObjectType,
    pub properties: Vec<Property>,
}

#[derive(Debug)]
pub enum ObjectType {
    Camera,
    Light,
    Physics,
    Material,
    Controller,
    Sphere,
    Rectangle,
    Image,
    Active,
}

impl std::fmt::Display for ObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ObjectType::Camera => write!(f, "engine.Camera"),
            ObjectType::Light => write!(f, "graphics.Light"),
            ObjectType::Physics => write!(f, "physics.PhysicsEngine"),
            ObjectType::Material => write!(f, "graphics.material.Material"),
            ObjectType::Controller => write!(f, "event_handler.EventHandler"),
            ObjectType::Sphere => write!(f, "objects.Sphere"),
            ObjectType::Rectangle => write!(f, "objects.Rectangle"),
            ObjectType::Image => write!(f, "images.Image"),
            ObjectType::Active => write!(f, "Active"),
        }
    }
}

#[derive(Debug)]
pub struct Property {
    pub name: String,
    pub value: Expression,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Number(String),
    Identifier(String),
    String(String),
    Group(Vec<Expression>),
    Empty,
}

// display for expression
impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Expression::Number(num) => write!(f, "{}", num),
            Expression::Identifier(idtfr) => write!(f, "{}", idtfr),
            Expression::String(str) => write!(f, "{}", str),
            Expression::Group(exprs) => {
                write!(f, "(")?;
                for expr in exprs {
                    write!(f, "{}, ", expr)?;
                }
                write!(f, ")")
            }
            Expression::Empty => write!(f, ""),
        }
    }
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
                    name: "scene".to_string(),
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
        self.ensure(tokeniser::Token::Identifier(SCENE.to_string()));
        if let tokeniser::Token::Identifier(name) = self.pop_front() {
            self.engine.scene.name = name;
        }

        Step::Scene
    }

    pub fn scene(&mut self) -> Step {
        self.ensure(tokeniser::Token::LBrace);
        Step::Object
    }

    pub fn object(&mut self) -> Step {
        let obj_type = self.pop_front();
        let obj_string = match obj_type {
            tokeniser::Token::Identifier(obj_string) => {
                if !OBJECT_TYPES.contains(&obj_string.as_str()) {
                    panic!("Unexpected object type: {}", obj_string);
                }
                obj_string
            }
            _ => panic!("Unexpected token: {:?}", obj_type),
        };

        if let tokeniser::Token::Identifier(name) = self.pop_front() {
            self.engine.scene.objects.push(Object {
                name,
                obj_type: match obj_string.as_str() {
                    CAMERA => ObjectType::Camera,
                    LIGHT => ObjectType::Light,
                    PHYSICS => ObjectType::Physics,
                    MATERIAL => ObjectType::Material,
                    CONTROLLER => ObjectType::Controller,
                    SPHERE => ObjectType::Sphere,
                    RECTANGLE => ObjectType::Rectangle,
                    IMAGE => ObjectType::Image,
                    ACTIVE => ObjectType::Active,
                    _ => panic!("Unexpected object type: {}", obj_string),
                },
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
                    value: Expression::Empty,
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

    pub fn make_expression(&mut self, token: &tokeniser::Token) -> Expression {
        match token {
            tokeniser::Token::Number(num) => Expression::Number(num.clone()),
            tokeniser::Token::Identifier(idtfr) => Expression::Identifier(idtfr.clone()),
            tokeniser::Token::String(str) => Expression::String(str.clone()),
            _ => panic!("Unexpected token {:?}", token),
        }
    }

    pub fn expression(&mut self) -> Expression {
        let front = self.pop_front();
        match front {
            tokeniser::Token::Number(num) => Expression::Number(num),
            tokeniser::Token::Identifier(idtfr) => Expression::Identifier(idtfr),
            tokeniser::Token::String(str) => Expression::String(str),
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
                Expression::Group(tokens.iter().map(|t| self.make_expression(t)).collect())
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

    pub fn print(&self) {
        println!("{:#?}", self.engine);
    }
}
