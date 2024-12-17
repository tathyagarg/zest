const std = @import("std");

const tokenslib = @import("../tokenizer/tokenizer.zig");
const Token = tokenslib.Token;
const TokenType = tokenslib.TokenType;

const exprlib = @import("expressions.zig");
const Expression = exprlib.Expression;

const stmtlib = @import("statement.zig");
const ConstExpression = stmtlib.ConstExpression;
const VarExpression = stmtlib.VarExpression;

pub const Statement = @import("statement.zig").Statement;

const ArrayList = std.ArrayList;

var toks: *ArrayList(Token) = undefined;
var current: usize = 0;

pub fn parse(tokens: *ArrayList(Token), statements: *ArrayList(Statement)) !void {
    toks = tokens;

    // while (!is_at_end()) {
    try statements.append(statement());
    // }
}

fn expression() Expression {
    return equality();
}

fn equality() Expression {
    var expr = comparison();

    while (match(TokenType.EQ) or match(TokenType.NE)) {
        const operator = previous();
        const right = comparison();
        expr = Expression{
            .binary = exprlib.Binary{
                .left = &expr,
                .operator = operator,
                .right = &right,
            },
        };
    }

    return expr;
}

fn comparison() Expression {
    var expr = term();

    while (match(TokenType.GT) or match(TokenType.GTE) or
        match(TokenType.LT) or match(TokenType.LTE))
    {
        const operator = previous();
        const right = term();
        expr = Expression{
            .binary = exprlib.Binary{
                .left = &expr,
                .operator = operator,
                .right = &right,
            },
        };
    }

    return expr;
}

fn term() Expression {
    var expr = factor();

    while (match(TokenType.SLASH) or match(TokenType.STAR)) {
        const operator = previous();
        const right = factor();
        expr = Expression{
            .binary = exprlib.Binary{
                .left = &expr,
                .operator = operator,
                .right = &right,
            },
        };
    }

    return expr;
}

fn factor() Expression {
    var expr = unary();

    while (match(TokenType.SLASH) or match(TokenType.STAR)) {
        const operator = previous();
        const right = unary();
        expr = Expression{
            .binary = exprlib.Binary{
                .left = &expr,
                .operator = operator,
                .right = &right,
            },
        };
    }

    return expr;
}

fn unary() Expression {
    if (match(TokenType.MINUS)) {
        const operator = previous();
        const right = unary();
        return Expression{
            .unary = exprlib.Unary{
                .operator = operator,
                .right = &right,
            },
        };
    }

    return primary();
}

fn primary() Expression {
    if (match(TokenType.FALSE)) return Expression{ .literal = exprlib.Literal{ .IDENTIFIER = "false" } };
    if (match(TokenType.TRUE)) return Expression{ .literal = exprlib.Literal{ .IDENTIFIER = "true" } };
    if (match(TokenType.UNDEF)) return Expression{ .literal = exprlib.Literal{ .IDENTIFIER = "undefined" } };
    if (match(TokenType.NULL)) return Expression{ .literal = exprlib.Literal{ .IDENTIFIER = "null" } };
    if (match(TokenType.NUMBER) or match(TokenType.STRING) or
        match(TokenType.CONST)) return Expression{ .literal = previous().literal };

    if (match(TokenType.IDENTIFIER)) return Expression{};

    if (match(TokenType.LPAREN)) {
        const expr = expression();
        // _ = consume(TokenType.RPAREN);
        _ = consume();
        return Expression{ .grouping = exprlib.Grouping{ .expression = &expr } };
    }

    if (match(TokenType.SEMICOLON)) {
        return Expression{ .eol = 1 };
    } else {
        std.debug.print("{any}\n", .{toks.items[current]});
        unreachable;
    }
}

fn consume() Token {
    return advance();
}

fn match(tt: TokenType) bool {
    if (check(tt)) {
        _ = advance();
        return true;
    }
    return false;
}

fn check(tt: TokenType) bool {
    if (is_at_end()) return false;
    return peek().token_type == tt;
}

fn is_at_end() bool {
    return peek().token_type == TokenType.EOF;
}

fn peek() Token {
    return toks.items[current];
}

fn advance() Token {
    if (!is_at_end()) current += 1;
    return previous();
}

fn previous() Token {
    return toks.items[current - 1];
}

fn statement() Statement {
    if (match(TokenType.CONST)) return const_assign_stmt();
    if (match(TokenType.VAR)) return var_assign_stmt();
    if (match(TokenType.LBRACE)) return block_stmt();

    unreachable;
    // return expr_stmt();
}

fn const_assign_stmt() Statement {
    const name = consume();
    var init: Expression = undefined;

    if (match(TokenType.ASSIGN)) {
        init = expression();
    }

    _ = consume();
    return Statement{ .const_assign = ConstExpression{ .name = name, .initializer = init } };
}

fn var_assign_stmt() Statement {
    const name = consume();
    var init: Expression = undefined;

    if (match(TokenType.ASSIGN)) {
        init = expression();
    }

    _ = consume();
    return Statement{ .const_assign = VarExpression{ .name = name, .initializer = init } };
}

fn block_stmt(stmt: *ArrayList(Statement)) !Statement {
    while (!check(TokenType.RBRACE) and !is_at_end()) {
        try stmt.append(statement());
    }

    _ = consume(TokenType.RBRACE);
    return Statement{ .block = stmt.* };
}

