const ArrayList = @import("std").ArrayList;

const Expression = @import("expressions.zig").Expression;
const Token = @import("../tokenizer/tokenizer.zig").Token;

pub const ConstExpression = struct {
    name: Token,
    initializer: Expression,
};

pub const VarExpression = struct {
    name: Token,
    initializer: Expression,
};

pub const Statement = union(enum) {
    expr: Expression,
    const_assign: ConstExpression,
    var_assign: VarExpression,
    block: ArrayList(Statement),
};
