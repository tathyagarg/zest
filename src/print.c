#include "dfa.h"
#include "driver.c"
#include "globals.h"
#include "tools.c"
#include <ctype.h>
#include <stdio.h>
#include <string.h>

void pheader(FILE *fp, ROW dtran[], int nrows, ACCEPT *accept) {
  int i, j;
  int last_trans;
  int chars_printed;

  fprintf(fp, "#ifdef __NEVER__\n");
  fprintf(fp, "// DFA (start start is 0) is:\n\n");

  for (i = 0; i < nrows; i++) {
    if (!accept[i].string) {

      fprintf(fp, "State %d [nonaccepting]", i);
    } else {
      fprintf(fp, "State %d [accepting, line %d <", i,
              ((int *)(accept[i].string))[-1]);
      fputstr(accept[i].string, 20, fp);
      fprintf(fp, ">]");

      if (accept[i].anchor) {
        fprintf(fp, "Anchor: %s%s",
                accept[i].anchor & 1 ? "start" : "", // 1 = START
                accept[i].anchor & 2 ? "end" : "");  // 2 = END
      }
    }

    last_trans = -1;
    for (j = 0; j < MAX_CHARS; j++) {
      if (dtran[i][j] != F) {
        if (dtran[i][j] != last_trans) {
          fprintf(fp, "\n * goto %2d on ", dtran[i][j]);
          chars_printed = 0;
        }

        fprintf(fp, "%s", bin_to_ascii(j, 1));

        if ((chars_printed += strlen(bin_to_ascii(j, 1))) > 56) {
          fprintf(fp, "\n * ");
          chars_printed = 0;
        }

        last_trans = dtran[i][j];
      }
    }
    fprintf(fp, "\n");
  }
  fprintf(fp, "*/\n\n");
  fprintf(fp, "#endif\n");
}

void pdriver(FILE *output, int nrows, ACCEPT *accept) {
  int i;
  fprintf(output, "static short accept[] = \n");
  fprintf(output, "{\n");

  for (i = 0; i < nrows; i++) {
    if (!accept[i].string) {
      fprintf(output, "\t0");
    } else {
      fprintf(output, "\t%-3d", accept[i].anchor ? accept[i].anchor : 4);
    }
    fprintf(output, "%c /* State %-3d */\n", i == (nrows - 1) ? ' ' : ',', i);
  }

  fprintf(output, "};\n\n");

  driver_2(output, 1);

  for (i = 0; i < nrows; i++) {
    if (accept[i].string) {
      fprintf(output, "case %d: /* State %-3d */\n", i, i);
      fprintf(output, "#line %d \"%s\"\n", *((int *)(accept[i].string) - 1),
              ifname);

      fprintf(output, "\t\t%s\n", accept[i].string);
      fprintf(output, "\t\tbreak;\n\n");
    }
  }

  driver_2(output, 1);
}
