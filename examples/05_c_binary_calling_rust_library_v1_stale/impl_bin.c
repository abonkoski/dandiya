#include "example.h"
#include <stdlib.h>
#include <stdio.h>

#define FAIL(...) do { fprintf(stderr, "FAIL: "); fprintf(stderr, __VA_ARGS__); fprintf(stderr, "\n"); abort(); } while(0)

int main()
{
  example_ctx_t *ctx = example_new();
  if (!ctx) FAIL("Failed to create context");

  example_batch_t batch[1] = {{}};
  for (size_t i = 0; i < 5; i++) {
    uint64_t ret = example_fetch_batch(ctx, batch);
    if (ret != EXAMPLE_SUCCESS) FAIL("Failed to fetch batch: %lu", ret);
    printf("batch %zu: %u\n", i, batch->num);
  }

  example_delete(ctx);
  return 0;
}
