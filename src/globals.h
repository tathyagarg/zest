#include <stdio.h>

#ifdef ALLOC
#define CLASS
#define I(x) x
#else
#define CLASS extern
#define I(x)
#endif
#define MAXINP 2048

extern int verbose I(= 0);
extern int no_lines I(= 0);
extern int _unix I(= 0);
extern int _public I(= 0);
extern int actual_line_no I(= 1);
extern int line_no I(= 0);

extern char *_template I(= "zircon.par");

extern char input_buf[MAXINP];
extern char *ifname;
extern FILE *ifile;
extern FILE *ofile;
