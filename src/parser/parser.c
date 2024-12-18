#include "parser.h"
#include "../lexer/lexer.h"
#include "../lexer/tokens.h"
#include <stdio.h>

void statements() {
  expression();
  if (match(SEMICOLON)) {
    advance();
  } else {
    printf("Error: expected ';'\n");
    return;
  }

  if (!match(EOI)) {
    statements();
  }
}

void expression() {
  term();
  expr_prime();
}

void expr_prime() {
  if (match(PLUS)) {
    advance();
    term();
    expr_prime();
  } else if (match(MINUS)) {
    advance();
    term();
    expr_prime();
  }

  return;
}

void term() {
  factor();
  term_prime();
}

void term_prime() {
  if (match(TIMES)) {
    advance();
    factor();
    term_prime();
  } else if (match(DIVIDE)) {
    advance();
    factor();
    term_prime();
  }

  return;
}

void factor() {
  if (match(NUM)) {
    advance();
  } else if (match(LPAREN)) {
    advance();
    expression();
    if (match(RPAREN)) {
      advance();
    } else {
      printf("Error: expected ')'\n");
      return;
    }
  } else {
    printf("Error: expected number or '('\n");
    return;
  }

  return;
}
