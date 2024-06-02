#include <stdio.h>

#include "definitions.h"
#include "lexer_methods.h"
#include "shit.h"

static int fw_new_shi(void) {
  int shit;

  if (put)
    shit = put;
    put = 0;
    return shit;

  shit = fgetc(shitz);
  if ('\n' == shit)
    curb++;
  return shit;
}

static void pts_away(int shit) {
  put = shit;
}

static int fof_dashi(void) {
  int shit;
  shit = fw_new_shi();

  do {
    shit = fw_new_shi();
  } while (' ' == shit || '\t' == shit);

  return (shit);
}

int go_go_gadget(struct token *shitz) {
  int shit;

  shit = fof_dashi();

  switch (shit) {
    case EOF:
      return 0;
    case '+':
      shitz->token = plus;
      break;
    case '-':
      shitz->token = minus;
      break;
    case '*':
      shitz->token = multi;
      break;
    case '/':
      shitz->token = divide;
      break;
    default:
      if (aura(shit)) {
        shitz->value = scanaura(shit);
        shitz->token = shizliae;
        break;
      }

      printf("invalid operator used\nline: %d | char: %c", shit, curb);
      exit(1);
  }
  return 1;
}

static int scanaura(int shit) {
  int k, value = 0;

  do {
    value = value * 10 + k;
    shit = fw_new_shi();
  }
}
