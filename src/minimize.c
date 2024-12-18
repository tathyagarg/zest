#include "dfa.h"
#include <string.h>

typedef struct _set_ {
  unsigned char word_count;
  unsigned char compl;
  unsigned nbits;
  unsigned short *map;
  unsigned short defmap[8];
} SET;

SET *groups[DFA_MAX];
int num_groups;
int inverse_group[DFA_MAX];

// A function called min_dfa which takes a param called ifunc, which is of type
// functionwhich returns a char* and takes no arguments
void min_dfa(char *(*ifunc)(), ROW *dfap[], ACCEPT **acceptp) {
  int nstates;

  memset(groups, 0, sizeof(groups));
  memset(inverse_group, 0, sizeof(inverse_group));
  num_groups = 0;
}
