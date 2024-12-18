#ifndef DFA_H
#define DFA_H

#include <stdio.h>

#define DFA_MAX 256

typedef unsigned char TTYPE;

#define F -1
#define MAX_CHARS 128

typedef int ROW[MAX_CHARS];

typedef struct ACCEPT {
  char *string;
  int anchor;
} ACCEPT;

void pheader(FILE *fp, ROW dtran[], int nrows, ACCEPT *accept);
void pdriver(FILE *output, int nrows, ACCEPT *accept);

#endif // DFA_H
