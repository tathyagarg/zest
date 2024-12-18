#include "dfa.h"

int dfa(char *(*ifunc)(), ROW *dfap[], ACCEPT **acceptp) {
  ACCEPT *accept_states;
  int i;
  int start;

  start = nfa(ifunc);
}
