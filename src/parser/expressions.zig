const tokenslib = @import("../tokenizer/tokenizer.zig");
const Token = tokenslib.Token;
pub const Literal = tokenslib.Literal;

pub const Expression = union(enum) {
    binary: Binary,
    unary: Unary,
    literal: Literal,
    grouping: Grouping,
    eol: u1,
};

pub const Binary = struct {
    left: *Expression,
    operator: Token,
    right: *const Expression,
};

pub const Unary = struct {
    operator: Token,
    right: *const Expression,
};

pub const Grouping = struct {
    expression: *const Expression,
};
