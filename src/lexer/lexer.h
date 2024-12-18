#include "tokens.h"

// Lex the next token in the input buffer.
int lex();

// Checks if the next token matches some given token input.
int match(int token);

// Set lookahead to the next token, moving forward in the lex process.
void advance();
