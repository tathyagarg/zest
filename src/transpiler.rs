use crate::constructor::{self, Expression};

const BEGIN: &str = "const z3d = @import(\"root.zig\");
const std = @import(\"std\");

const Vec3 = z3d.math.Vec3(f32);

// Don't make this too big. Ensure HEIGHT == WIDTH
const HEIGHT = 400;
const WIDTH = 400;

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    var scene_objects = std.ArrayList(z3d.graphics.objects.Object).init(allocator);
    defer scene_objects.deinit();

    var lights = std.ArrayList(z3d.graphics.Light).init(allocator);
    defer lights.deinit();

    var gui_layer = z3d.gui.GUI_Layer.init(allocator);
    defer gui_layer.deinit();
";

const END: &str = "
    var engine = try z3d.engine.Engine.init(\"Z3D\", 0, 0, WIDTH, HEIGHT, z3d.engine.WindowFlags.default(), scene, allocator);
    defer engine.deinit();

    try engine.mainloop();
}";

struct Sphere {
    pub position: (Expression, Expression, Expression),
    pub radius: Expression,
    pub material: Expression,
}

#[derive(Debug)]
struct Rectangle {
    pub points: ((Expression, Expression, Expression), (Expression, Expression, Expression)),
    pub material: Expression,
    pub inverted: Expression,
}

struct Camera {
    pub position: (Expression, Expression, Expression),
    pub direction: (Expression, Expression, Expression),
    pub event_handler: Expression,
}

struct PhysicsEngine {
    pub gravity: Expression,
    pub object: Expression,
}

pub struct Transpiler {
    pub engine: constructor::Engine,
}

impl Transpiler {
    pub fn new(engine: constructor::Engine) -> Self {
        Self { engine }
    }

    pub fn transpile(&self) -> String {
        let mut output = String::new();
        output.push_str(BEGIN);
        for object in &self.engine.scene.objects {
            output.push('\n');
            output.push_str(self.transpile_element(object).as_str());
        }

        output.push_str(END);
        output
    }

    pub fn transpile_element(&self, object: &constructor::Object) -> String {
        match object.obj_type {
            constructor::ObjectType::Sphere => self.transpile_sphere(object),
            constructor::ObjectType::Material => self.transpile_material(object),
            constructor::ObjectType::Image => self.transpile_image(object),
            constructor::ObjectType::Controller => self.transpile_controller(object),
            constructor::ObjectType::Camera => self.transpile_camera(object),
            constructor::ObjectType::Physics => self.transpile_physics(object),
            constructor::ObjectType::Light => self.transpile_light(object),
            constructor::ObjectType::Rectangle => self.transpile_rectangle(object),
            constructor::ObjectType::Active => self.transpile_active(object),
        }
    }

    pub fn transpile_sphere(&self, object: &constructor::Object) -> String {
        let mut output = String::new();
        output.push_str(
            format!(
                "    const {} = z3d.graphics.objects.Sphere.init(", 
                object.name
            )
            .as_str()
        );

        let mut sphere = Sphere {
            position: (Expression::Empty, Expression::Empty, Expression::Empty),
            radius: Expression::Empty,
            material: Expression::Empty,
        };

        for prop in &object.properties {
            match prop.name.as_str() {
                "position" => {
                    let mut values = match &prop.value {
                        constructor::Expression::Group(values) => values.iter(),
                        _ => panic!("Expected group"),
                    };

                    sphere.position.0 = values.next().unwrap().clone();
                    sphere.position.1 = values.next().unwrap().clone();
                    sphere.position.2 = values.next().unwrap().clone();
                }
                "radius" => {
                    sphere.radius = prop.value.clone();
                }
                "material" => {
                    sphere.material = prop.value.clone();
                }
                _ => {}
            }
        }

        output.push_str(
            format!(
                "@constCast(&Vec3.init({}, {}, {})), {}, &{}",
                sphere.position.0,
                sphere.position.1,
                sphere.position.2,
                sphere.radius,
                sphere.material
            )
            .as_str(),
        );
        output.push_str(");\n");
        output.push_str(
            format!(
                "    var obj_{} = z3d.graphics.objects.Object{{ .sphere = {} }};\n",
                object.name, object.name
            )
            .as_str()
        );
        output.push_str(format!("    z3d.graphics.objects.assigned(&obj_{});\n", object.name).as_str());
        output.push_str(format!("    try scene_objects.append(obj_{});\n", object.name).as_str());

        output
    }

    pub fn transpile_material(&self, object: &constructor::Object) -> String {
        let mut output = String::new();
        output.push_str(
            format!("    const {} = z3d.graphics.material.Material{{ .texture = z3d.graphics.material.Texture {{", object.name).as_str(),
        );

        if object.properties.len() > 1 {
            panic!("Too many properties for material");
        }

        let property = &object.properties[0];

        match property.name.as_str() {
            "color" => {
                let mut values = match &property.value {
                    constructor::Expression::Group(values) => values.iter(),
                    _ => panic!("Expected group"),
                };

                output.push_str(
                    format!(
                        " .SOLID_COLOR = z3d.graphics.RGB{{ .r = {}, .g = {}, .b = {} }}",
                        values.next().unwrap(),
                        values.next().unwrap(),
                        values.next().unwrap()
                    )
                    .as_str(),
                );
            }
            "image" => {
                let image = match &property.value {
                    constructor::Expression::Identifier(x) => x.clone(),
                    _ => panic!("Expected identifier"),
                };
                output.push_str(format!(" .TEXTURE_FILE = {}", image).as_str());
            }
            _ => {}
        }

        output.push_str(" }};\n");

        output
    }

    pub fn transpile_image(&self, object: &constructor::Object) -> String {
        let mut output = String::new();
        output.push_str(format!("    var {} = try z3d.images.Image.init(", object.name).as_str());

        if object.properties.len() > 1 {
            panic!("Too many properties for image");
        }

        let property = &object.properties[0];
        if let Expression::String(file) = &property.value {
            output.push_str(format!("\"{}\"", file).as_str());
        }

        output.push_str(");\n");
        output.push_str(format!("    defer {}.deinit();\n", object.name).as_str());

        output
    }

    pub fn transpile_controller(&self, object: &constructor::Object) -> String {
        let mut output = String::new();
        output.push_str(
            format!(
                "    const {} = z3d.event_handler.EventHandler{{",
                object.name
            )
            .as_str(),
        );

        for prop in &object.properties {
            match prop.name.as_str() {
                "keyboard_movement" => {
                    let key = match &prop.value {
                        Expression::Identifier(x) => x.clone(),
                        _ => panic!("Expected identifier"),
                    };
                    output.push_str(format!(" .keyboard_movement = {},", key).as_str());
                }
                "mouse_movement" => {
                    let mouse = match &prop.value {
                        Expression::Identifier(x) => x.clone(),
                        _ => panic!("Expected identifier"),
                    };
                    output.push_str(format!(" .mouse_movement = {},", mouse).as_str());
                }
                _ => {}
            }
        }

        output.push_str(" .width = WIDTH, .height = HEIGHT };\n");

        output
    }

    pub fn transpile_camera(&self, object: &constructor::Object) -> String {
        let mut output = String::new();
        output.push_str(format!("    const {} = z3d.engine.Camera{{", object.name).as_str());

        let mut camera = Camera {
            position: (Expression::Empty, Expression::Empty, Expression::Empty),
            direction: (Expression::Empty, Expression::Empty, Expression::Empty),
            event_handler: Expression::Empty,
        };

        for prop in &object.properties {
            match prop.name.as_str() {
                "position" => {
                    let mut values = match &prop.value {
                        constructor::Expression::Group(values) => values.iter(),
                        _ => panic!("Expected group"),
                    };

                    camera.position.0 = values.next().unwrap().clone();
                    camera.position.1 = values.next().unwrap().clone();
                    camera.position.2 = values.next().unwrap().clone();
                }
                "direction" => {
                    let mut values = match &prop.value {
                        constructor::Expression::Group(values) => values.iter(),
                        _ => panic!("Expected group"),
                    };

                    camera.direction.0 = values.next().unwrap().clone();
                    camera.direction.1 = values.next().unwrap().clone();
                    camera.direction.2 = values.next().unwrap().clone();
                }

                "event_handler" => {
                    camera.event_handler = prop.value.clone();
                }
                _ => {}
            }
        }

        output.push_str(
            format!(
                " .position = &z3d.transform.PositionHandler{{ .single = z3d.transform.SinglePointHandler{{ .point = @constCast(&Vec3.init({}, {}, {})) , .direction = @constCast(&Vec3.init({}, {}, {})) }} }}, .event_handler = &{} }};\n", 
                camera.position.0, camera.position.1, camera.position.2, 
                camera.direction.0, camera.direction.1, camera.direction.2, 
                camera.event_handler
            )
            .as_str()
        );

        output
    }

    pub fn transpile_physics(&self, object: &constructor::Object) -> String {
        let mut output = String::new();
        output.push_str(
            format!(
                "    var {} = z3d.physics.PhysicsEngine.init(",
                object.name
            )
            .as_str(),
        );

        let mut physics = PhysicsEngine {
            gravity: Expression::Empty,
            object: Expression::Empty,
        };

        if object.properties.len() > 2 {
            panic!("Too many properties for physics");
        }

        for prop in &object.properties {
            match prop.name.as_str() {
                "gravity" => {
                    physics.gravity = prop.value.clone();
                }
                "object" => {
                    physics.object = prop.value.clone();
                }
                _ => {}
            }
        }

        output.push_str(
            format!(
                "obj_{}, .{{}});\n",
                physics.object
            )
            .as_str(),
        );
        if self.evaluate_bool(&physics.gravity) {
            output.push_str(
                format!(
                    "    {}.apply_gravity(null);\n",
                    object.name
                )
                .as_str()
            );
        }

        output
    }

    pub fn transpile_light(&self, object: &constructor::Object) -> String {
        let mut output = String::new();
        output.push_str(
            format!(
                "    const {} = z3d.graphics.Light{{",
                object.name
            )
            .as_str(),
        );

        if object.properties.len() > 2 {
            panic!("Too many properties for light");
        }

        for prop in &object.properties {
            match prop.name.as_str() {
                "position" => {
                    let mut values = match &prop.value {
                        constructor::Expression::Group(values) => values.iter(),
                        _ => panic!("Expected group"),
                    };

                    output.push_str(
                        format!(
                            ".position = Vec3.init({}, {}, {}), ",
                            values.next().unwrap(), 
                            values.next().unwrap(), 
                            values.next().unwrap()
                        )
                        .as_str()
                    );
                }
                "intensity" => {
                    let mut values = match &prop.value {
                        constructor::Expression::Group(values) => values.iter(),
                        _ => panic!("Expected group"),
                    };

                    output.push_str(
                        format!(
                            ".intensity = Vec3.init({}, {}, {}), ",
                            values.next().unwrap(), 
                            values.next().unwrap(), 
                            values.next().unwrap()
                        )
                        .as_str()
                    );
                }
                _ => {}
            }
        }

        output.push_str("};\n");
        output.push_str(format!("    try lights.append({});\n", object.name).as_str());

        output
    }

    pub fn transpile_rectangle(&self, object: &constructor::Object) -> String {
        let mut output = String::new();
        output.push_str(
            format!(
                "    const {} = z3d.graphics.objects.Rectangle.init(", 
                object.name
            )
            .as_str()
        );

        if object.properties.len() > 4 {
            panic!("Too many properties for rectangle");
        }

        let mut rectangle = Rectangle {
            points: ((Expression::Empty, Expression::Empty, Expression::Empty), (Expression::Empty, Expression::Empty, Expression::Empty)),
            material: Expression::Empty,
            inverted: Expression::Empty,
        };

        for prop in &object.properties {
            match prop.name.as_str() {
                "v0" => {
                    let mut values = match &prop.value {
                        constructor::Expression::Group(values) => values.iter(),
                        _ => panic!("Expected group"),
                    };

                    rectangle.points.0.0 = values.next().unwrap().clone();
                    rectangle.points.0.1 = values.next().unwrap().clone();
                    rectangle.points.0.2 = values.next().unwrap().clone();
                }
                "v1" => {
                    let mut values = match &prop.value {
                        constructor::Expression::Group(values) => values.iter(),
                        _ => panic!("Expected group"),
                    };

                    rectangle.points.1.0 = values.next().unwrap().clone();
                    rectangle.points.1.1 = values.next().unwrap().clone();
                    rectangle.points.1.2 = values.next().unwrap().clone();
                }
                "material" => {
                    rectangle.material = prop.value.clone();
                }
                "inverted" => {
                    rectangle.inverted = prop.value.clone();
                }
                _ => {}
            }
        }
        output.push_str(
            format!(
                "Vec3.init({}, {}, {}), Vec3.init({}, {}, {}), Vec3.init({}, {}, {}), Vec3.init({}, {}, {}), &{}, {});\n",
                rectangle.points.0.0,
                rectangle.points.0.1,
                rectangle.points.0.2,
                rectangle.points.0.0,
                rectangle.points.1.1,
                rectangle.points.1.2,
                rectangle.points.1.0,
                rectangle.points.1.1,
                rectangle.points.1.2,
                rectangle.points.1.0,
                rectangle.points.1.1,
                rectangle.points.0.2,
                rectangle.material,
                rectangle.inverted
            )
            .as_str()
        );
        output.push_str(
            format!(
                "    var obj_{} = z3d.graphics.objects.Object{{ .rectangle = {} }};\n",
                object.name, object.name
            )
            .as_str()
        );
        output.push_str(format!("    z3d.graphics.objects.assigned(&obj_{});\n", object.name).as_str());
        output.push_str(format!("    try scene_objects.append(obj_{});\n", object.name).as_str());

        output
    }

    pub fn transpile_active(&self, object: &constructor::Object) -> String {
        let mut output = String::new();

        if object.properties.len() > 1 {
            panic!("Too many properties for active");
        }

        let property = &object.properties[0];
        let object = match &property.value {
            Expression::Identifier(x) => x.clone(),
            _ => panic!("Expected identifier"),
        };

        output.push_str(
            format!(
                "    const scene = try z3d.engine.Scene.init({}, &scene_objects, &lights, .{{ .ray_casting_options = &z3d.graphics.RayCastingOptions{{ .width = WIDTH, .height = HEIGHT, .fov = 90 }} }}, &gui_layer);\n",
                object
            )
            .as_str()
        );

        output
    }

    pub fn evaluate_bool(&self, expression: &Expression) -> bool {
        match expression {
            Expression::Identifier(b) => {
                if ![ "true", "false" ].contains(&b.as_str()) {
                    panic!("Expected true or false");
                }

                b == "true"
            },
            _ => panic!("Expected bool"),
        }
    }
}
