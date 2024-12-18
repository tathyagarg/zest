#include "dfa.h"
#include "globals.h"
#include "signon.c"
#include <ctype.h>
#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>

void error(int usage, char *fmt, ...);
int main(int argc, char **argv);
int getfline(char **stringp, int n, FILE *stream);
void do_file(void);
void tail(void);
extern char *get_expr(void);

#define ERR(x) frpintf(stderr, "%s\n", x)

int column_compress = 1024;
int no_compress = 0;
int threshold = 4;
int no_header = 0;
int header_only = 0;

extern int verbose;
extern int no_lines;
extern int line_no;

void cmd_line_err(int usage, char *fmt, ...) {
  extern char *sys_errlist[];
  extern int errno;
  va_list args;

  va_start(args, fmt);
  fprintf(stderr, "zircon: ");
  vfprintf(stderr, fmt, args);

  fprintf(stderr, "(%s)\n", sys_errlist[errno]);

  exit(1);
  va_end(args);
}

void line_err(int status, char *fmt, ...) {
  extern int errno;
  va_list args;

  va_start(args, fmt);
  fprintf(stderr, "zircon, input line %d: ", actual_line_no);
  vfprintf(stderr, fmt, args);

  if (status) {
    exit(status);
  }
  va_end(args);
}

int main(int argc, char **argv) {
  static char *p;
  static int use_stdout = 0;

  signon();

  for (++argv, --argc; argc > 0 && *(p = *argv) == '-'; ++argv, --argc) {
    while (*++p) {
      switch (*p) {
      case 'f':
        no_compress = 1;
        break;
      case 'h':
        no_header = 1;
        break;
      case 'H':
        header_only = 1;
        break;
      case 'l':
        no_lines = 1;
        break;
      case 'm':
        _template = p + 1;
        goto out;
      case 'p':
        _public = 1;
        break;
      case 't':
        use_stdout = 1;
        break;
      case 'u':
        _unix = 1;
        break;
      case 'v':
        verbose = 1;
        break;
      case 'c':
        column_compress = 0;
        if (!isdigit(p[1]))
          threshold = 4;
        else {
          threshold = atoi(++p);
          while (*p && isdigit(p[1])) {
            ++p;
          }
        }
        break;
      default:
        cmd_line_err(1, "unknown option: -%c", *p);
        break;
      }
    }
  out:;
  }

  if (argc > 1) {
    cmd_line_err(1, "too many arguments");
  } else if (argc <= 0)
    cmd_line_err(1, "no input file");
  else {
    if ((ifile = fopen(*argv, "r")))
      ifname = *argv;
    else
      cmd_line_err(1, "can't open %s", *argv);
  }

  if (!use_stdout) {
    if (!(ofile = fopen(header_only ? "zirconyy.h" : "zirconyy.c", "w"))) {
      cmd_line_err(1, "can't open output file");
    }
  }

  do_file();
  fclose(ofile);
  fclose(ifile);

  exit(0);
}

void do_file() {
  int nstates;
  ROW *dtran;
  ACCEPT *accept;
  FILE *input;
  int i;

  nstates = min_dfa(&dtran, &accept);
}
