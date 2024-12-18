#include "tokens.h"
#include <ctype.h>
#include <stdio.h>

char *yytext = "";
int yyleng = 0;
int yylineno = 0;

int lex() {
  static char input_buffer[128];
  char *current;
  current = yytext + yyleng;

  while (1) {
    while (!*current) {
      current = input_buffer;
      if (!fgets(input_buffer, 128, stdin)) {
        *current = '\0';
        return EOI;
      }
      ++yylineno;
      while (isspace(*current)) {
        ++current;
      }
    }

    for (; *current; ++current) {
      yytext = current;
      yyleng = 1;
      switch (*current) {
      case EOF:
        return EOI;
      case ';':
        return SEMICOLON;
      case '+':
        return PLUS;
      case '-':
        return MINUS;
      case '*':
        return TIMES;
      case '/':
        return DIVIDE;
      case '(':
        return LPAREN;
      case ')':
        return RPAREN;
      case '\n':
      case '\t':
      case ' ':
        break;
      default:
        if (!isdigit(*current)) {
          fprintf(stderr, "Ignoring illegal input <%c>\n", *current);
        } else {
          while (isdigit(*current)) {
            ++current;
          }
          yyleng = current - yytext;
          return NUM;
        }
        break;
      }
    }
  }
}

// The lookahead allows the compiler to get the next input without actually
// reading it.
static int lookahead = -1;

// Checks if the next token matches some given token input.
int match(int token) {
  if (lookahead == -1)
    lookahead = lex();
  return token == lookahead;
}

// Set lookahead to the next token, moving forward in the lex process.
void advance() { lookahead = lex(); }
