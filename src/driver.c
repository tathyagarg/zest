#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

FILE *driver_1(FILE *output, int lines, char *_fname);
int driver_2(FILE *output, int lines);

FILE *input_file = NULL;
int input_line;
char fname[80];

FILE *driver_1(FILE *output, int lines, char *_fname) {
  char path[80];

  if (!(input_file = fopen(_fname, "r"))) {
    fprintf(stderr, "Can't open %s\n", _fname);
    exit(1);
  }

  strncpy(fname, _fname, sizeof(fname));
  input_line = 0;
  driver_2(output, lines);

  return input_file;
}

int driver_2(FILE *output, int lines) {
  static char buffer[256];
  char *p;
  int processing_comment = 0;

  if (!input_file) {
    fprintf(stderr, "No input file\n");
    exit(1);
  }

  if (lines) {
    fprintf(output, "#line %d \"%s\"\n", input_line + 1, fname);
  }

  while (fgets(buffer, sizeof(buffer), input_file)) {
    input_line++;
    if (*buffer == '\f') {
      break;
    }

    for (p = buffer; isspace(*p); ++p)
      ;

    if (*p == '^') {
      processing_comment = 1;
      continue;
    } else if (processing_comment) {
      processing_comment = 0;
      if (lines) {
        fprintf(output, "#line %d \"%s\"\n", input_line, fname);
      }
    }
    fputs(buffer, output);
  }

  return feof(input_file);
}
