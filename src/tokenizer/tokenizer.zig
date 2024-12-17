pub const tokenslib = @import("tokens.zig");
pub const Token = tokenslib.Token;
pub const TokenType = tokenslib.TokenType;
pub const Literal = tokenslib.Literal;

const std = @import("std");

const ArrayList = std.ArrayList;

var source: []const u8 = undefined;
var start: usize = 0;
var current: usize = 0;
var line: usize = 1;

pub fn scan_tokens(src: []const u8, tokens: *ArrayList(Token)) !void {
    source = src;
    while (!is_at_end()) {
        start = current;
        try scan_token(tokens);
    }

    try tokens.append(Token{
        .token_type = TokenType.EOF,
        .literal = Literal{ .EOF = null },
        .lexeme = "",
        .line = line,
    });
}

fn is_at_end() bool {
    return current >= source.len;
}

fn scan_token(tokens: *ArrayList(Token)) !void {
    const c: u8 = advance();
    switch (c) {
        '(' => {
            if (matches('*')) {
                while (peek() != '*' and peek_next() != ')' and !is_at_end()) _ = advance();
                _ = advance(); // Extra advance to advance over the closing )
            } else {
                try add_token(TokenType.LPAREN, Literal{ .OTHER = "" }, tokens);
            }
        },
        ')' => try add_token(TokenType.RPAREN, Literal{ .OTHER = "" }, tokens),
        '{' => try add_token(TokenType.LBRACE, Literal{ .OTHER = "" }, tokens),
        '}' => try add_token(TokenType.RBRACE, Literal{ .OTHER = "" }, tokens),
        ';' => try add_token(TokenType.SEMICOLON, Literal{ .OTHER = "" }, tokens),
        '.' => try add_token(TokenType.DOT, Literal{ .OTHER = "" }, tokens),
        ',' => try add_token(TokenType.COMMA, Literal{ .OTHER = "" }, tokens),
        ':' => try add_token(TokenType.COLON, Literal{ .OTHER = "" }, tokens),
        '!' => {
            if (matches('=')) {
                try add_token(TokenType.NE, Literal{ .OTHER = "" }, tokens);
            } else {
                try add_token(TokenType.BANG_NOT, Literal{ .OTHER = "" }, tokens);
            }
        },
        '=' => {
            if (matches('=')) {
                try add_token(TokenType.EQ, Literal{ .OTHER = "" }, tokens);
            } else {
                try add_token(TokenType.ASSIGN, Literal{ .OTHER = "" }, tokens);
            }
        },
        '>' => {
            if (matches('=')) {
                try add_token(TokenType.GTE, Literal{ .OTHER = "" }, tokens);
            } else {
                try add_token(TokenType.GT, Literal{ .OTHER = "" }, tokens);
            }
        },
        '<' => {
            if (matches('=')) {
                try add_token(TokenType.LTE, Literal{ .OTHER = "" }, tokens);
            } else {
                try add_token(TokenType.LT, Literal{ .OTHER = "" }, tokens);
            }
        },
        '+' => {
            if (matches('=')) {
                try add_token(TokenType.PLUS_ASSIGN, Literal{ .OTHER = "" }, tokens);
            } else {
                try add_token(TokenType.PLUS, Literal{ .OTHER = "" }, tokens);
            }
        },
        '-' => {
            if (matches('=')) {
                try add_token(TokenType.MINUS_ASSIGN, Literal{ .OTHER = "" }, tokens);
            } else {
                try add_token(TokenType.MINUS, Literal{ .OTHER = "" }, tokens);
            }
        },
        '*' => {
            if (matches('=')) {
                try add_token(TokenType.STAR_ASSIGN, Literal{ .OTHER = "" }, tokens);
            } else {
                try add_token(TokenType.STAR, Literal{ .OTHER = "" }, tokens);
            }
        },
        '/' => {
            if (matches('=')) {
                try add_token(TokenType.SLASH, Literal{ .OTHER = "" }, tokens);
            } else {
                try add_token(TokenType.SLASH_ASSIGN, Literal{ .OTHER = "" }, tokens);
            }
        },
        ' ', '\r', '\t' => {},
        '\n' => line += 1,
        '"' => try string(tokens),
        else => {
            if (is_digit(c)) {
                try number(tokens);
            } else if (is_alpha(c)) {
                try identifier(tokens);
            } else {
                // TODO: Throws Error!
            }
        },
    }
}

fn advance() u8 {
    current += 1;
    return source[current - 1];
}

fn add_token(token_type: TokenType, literal: Literal, tokens: *ArrayList(Token)) !void {
    const text: []const u8 = source[start..current];
    try tokens.append(Token{
        .token_type = token_type,
        .literal = literal,
        .lexeme = text,
        .line = line,
    });
}

fn matches(char: u8) bool {
    if (is_at_end()) {
        return false;
    }
    if (source[current] != char) {
        return false;
    }

    current += 1;
    return true;
}

fn peek() u8 {
    if (is_at_end()) return 0;
    return source[current];
}

fn peek_next() u8 {
    if (current + 1 >= source.len) return 0;
    return source[current + 1];
}

fn string(tokens: *ArrayList(Token)) !void {
    while (peek() != '"' and !is_at_end()) {
        if (peek() == '\n') line += 1;
        _ = advance();
    }

    if (is_at_end()) {
        // TODO: Throws Error!
    }
    _ = advance();
    const value = source[start + 1 .. current - 1];
    try add_token(TokenType.STRING, Literal{ .STRING = value }, tokens);
}

fn is_digit(char: u8) bool {
    // 48 stands for '0', 57 stands for '9'
    return char >= 48 and char <= 57;
}

fn number(tokens: *ArrayList(Token)) !void {
    while (is_digit(peek())) _ = advance();

    var float = false;
    if (peek() == '.') {
        float = true;
        _ = advance();
        while (is_digit(peek())) _ = advance();
    }

    const literal = if (float)
        Literal{ .FLOAT = parse_float(source[start..current]) }
    else
        Literal{ .INTEGER = parse_int(source[start..current]) };

    try add_token(TokenType.NUMBER, literal, tokens);
}

fn parse_float(text: []const u8) f64 {
    var val: f64 = 0;
    var point_pos: usize = undefined;
    for (text, 0..) |n, curr| {
        if (n == '.') {
            point_pos = curr;
        } else {
            val += @as(f64, @floatFromInt(n - 48));
            val *= 10;
        }
    }

    return val / std.math.pow(f64, 10.0, @as(f64, @floatFromInt(text.len - point_pos + 1)));
}

fn parse_int(text: []const u8) i64 {
    var val: i64 = 0;
    for (text) |n| {
        val *= 10;
        std.debug.print("{d}\n", .{n});
        val += n - 48;
    }

    return val;
}

fn is_alpha(char: u8) bool {
    //              a               z               A              Z               _
    return (char >= 97 and char <= 122) or (char >= 65 and char <= 90) or char == 95;
}

fn is_alnum(char: u8) bool {
    return is_alpha(char) or is_digit(char);
}

fn get_keyword(text: []const u8) TokenType {
    if (std.mem.eql(u8, text, "const")) return TokenType.CONST;
    if (std.mem.eql(u8, text, "var")) return TokenType.VAR;
    if (std.mem.eql(u8, text, "and")) return TokenType.AND;
    if (std.mem.eql(u8, text, "or")) return TokenType.OR;
    if (std.mem.eql(u8, text, "not")) return TokenType.NOT;
    if (std.mem.eql(u8, text, "if")) return TokenType.IF;
    if (std.mem.eql(u8, text, "elif")) return TokenType.ELIF;
    if (std.mem.eql(u8, text, "else")) return TokenType.ELSE;
    if (std.mem.eql(u8, text, "while")) return TokenType.WHILE;
    if (std.mem.eql(u8, text, "for")) return TokenType.FOR;
    if (std.mem.eql(u8, text, "fn")) return TokenType.FN;
    if (std.mem.eql(u8, text, "scene")) return TokenType.SCENE;
    if (std.mem.eql(u8, text, "camera")) return TokenType.CAMERA;
    if (std.mem.eql(u8, text, "object")) return TokenType.OBJECT;
    if (std.mem.eql(u8, text, "light")) return TokenType.LIGHT;
    if (std.mem.eql(u8, text, "false")) return TokenType.FALSE;
    if (std.mem.eql(u8, text, "true")) return TokenType.TRUE;
    return TokenType.IDENTIFIER;
}

fn identifier(tokens: *ArrayList(Token)) !void {
    while (is_alnum(peek())) _ = advance();

    const text = source[start..current];
    const tt = get_keyword(text);

    try add_token(tt, Literal{ .IDENTIFIER = source[start..current] }, tokens);
}
