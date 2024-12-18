#include <stdio.h>

char *bin_to_ascii(int c, int use_hex) {
  static char buffer[8];

  c &= 0xff;

  if (' ' <= c && c <= 0x7f && c != '\'' && c != '\\') {
    buffer[0] = c;
    buffer[1] = '\0';
  } else {
    buffer[0] = '\\';
    buffer[2] = '\0';

    switch (c) {
    case '\\':
      buffer[1] = '\\';
      break;
    case '\'':
      buffer[1] = '\'';
      break;
    case '\b':
      buffer[1] = 'b';
      break;
    case '\f':
      buffer[1] = 'f';
      break;
    case '\t':
      buffer[1] = 't';
      break;
    case '\r':
      buffer[1] = 'r';
      break;
    case '\n':
      buffer[1] = 'n';
      break;
    default:
      sprintf(&buffer[1], use_hex ? "x%03x" : "%03o", c);
      break;
    }
  }

  return buffer;
}

void fputstr(char *str, int maxlen, FILE *stream) {
  char *s;
  while (*str && maxlen >= 0) {
    s = bin_to_ascii(*str++, 1);
    while (*s && --maxlen >= 0) {
      putc(*s++, stream);
    }
  }
}
