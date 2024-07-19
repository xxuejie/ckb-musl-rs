#include <stdio.h>
#include <stdlib.h>

void call_to_c(const char *s) {
  printf("Received in C: %d\n", atoi(s));

  {
    char buffer[1024];

    void *p1 = malloc(1234);
    snprintf(buffer, 1024, "p1: %p\n", p1);
    printf("%s", buffer);

    void *p2 = malloc(1000);
    snprintf(buffer, 1024, "p2: %p\n", p2);
    printf("%s", buffer);

    void *p3 = malloc(500);
    snprintf(buffer, 1024, "p3: %p\n", p3);
    printf("%s", buffer);

    if (p1 != NULL) { free(p1); }
    if (p3 != NULL) { free(p3); }

    void *p4 = malloc(400);
    snprintf(buffer, 1024, "p4: %p\n", p4);
    printf("%s", buffer);

    void *p5 = malloc(30000);
    snprintf(buffer, 1024, "p5: %p\n", p5);
    printf("%s", buffer);

    if (p2 != NULL) { free(p2); }
    if (p4 != NULL) { free(p4); }
    if (p5 != NULL) { free(p5); }
  }
}
