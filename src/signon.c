#include <stdio.h>

void signon(void) {
  FILE *screen;
  fprintf(screen, "Zircon 1.0.");

  fclose(screen);
}
