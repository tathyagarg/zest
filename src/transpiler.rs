use crate::constructor;

const BEGIN: &str = "const z3d = @import(\"z3d\")
const std = @import(\"std\")

// Don't make this too big. Ensure HEIGHT == WIDTH
const HEIGHT = 400;
const WIDTH = 400;

pub fn main() void {
    var allocator = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer allocator.deinit();

    var scene_objects = std.ArrayList(z3d.objects.Object).init(allocator);
    defer scene_objects.deinit();

    var lights = std.ArrayList(z3d.graphics.Light).init(allocator);
    defer lights.deinit();

    var gui_layer = z3d.gui.GUI_Layer.init(allocator);
    defer gui_layer.deinit();
";

const END: &str = "
    var engine = try z3d.engine.Engine.init(
        \"Z3D\",
        0,
        0,
        WIDTH,
        HEIGHT,
        z3d.engine.WindowFlags.default(),
        scene,
        allocator,
    );
    defer engine.deinit();

    try engine.mainloop();
}";

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
        output.push_str(END);
        output
    }

    pub fn transpile_expression(&self, expression: &constructor::Expression) -> String {
        //let mut output = String::new();
        //for token in &expression.tokens {
        //    match token {
        //        constructor::Token::Number(n) => output.push_str(&n.to_string()),
        //        constructor::Token::String(s) => output.push_str(&format!("\"{}\"", s)),
        //        constructor::Token::Identifier(i) => output.push_str(i),
        //        constructor::Token::Plus => output.push_str("+"),
        //        constructor::Token::Minus => output.push_str("-"),
        //        constructor::Token::Star => output.push_str("*"),
        //        constructor::Token::Slash => output.push_str("/"),
        //        _ => {}
        //    }
        //}
        //output
        String::from("expr")
    }
}
