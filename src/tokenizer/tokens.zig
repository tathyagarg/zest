pub const TokenType = enum {
    // Keywords
    CONST, // const
    VAR, // var
    AND, // and
    OR, // or
    NOT, // not
    IF, // if
    ELIF, // elif
    ELSE, // else
    WHILE, // while
    FOR, // for
    FN, // fn
    SCENE, // scene
    CAMERA, // camera
    OBJECT, // object
    LIGHT, // light
    FALSE, // false
    TRUE, // true

    // Symbols
    PLUS, // +
    MINUS, // -
    STAR, // *
    SLASH, // /
    LPAREN, // (
    RPAREN, // )
    LBRACE, // {
    RBRACE, // }
    EQ, // ==
    NE, // !=
    GT, // >
    GTE, // >=
    LT, // <
    LTE, // <=
    COLON, // :
    SEMICOLON, // ;
    ASSIGN, // =
    DOT, // .
    COMMA, // ,
    COMMENT_START, // (*
    COMMENT_END, // *)
    PLUS_ASSIGN, // +=
    MINUS_ASSIGN, // -=
    STAR_ASSIGN, // *=
    SLASH_ASSIGN, // /=
    ADDRESS_OF, // &
    POW, // ^

    // Literals
    IDENTIFIER,
    STRING,
    NUMBER,
    UNDEF,
    NULL,

    // Alternatives
    AMP_AND, // &&
    PIPE_OR, // ||
    BANG_NOT, // !

    // Misc
    EOF,
};

pub const Literal = union(enum) {
    IDENTIFIER: []const u8,
    STRING: []const u8,
    OTHER: []const u8,
    EOF: ?u1,

    // These will be mapped to shorter/appropriate sizes
    FLOAT: f64,
    INTEGER: i64,
};

pub const Token = struct {
    token_type: TokenType,
    literal: Literal,
    lexeme: []const u8,
    line: usize,
};
