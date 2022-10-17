#include "example.h"
#include <stdlib.h>
#include <assert.h>

#define ARRAY_SIZE(arr) (sizeof(arr)/sizeof((arr)[0]))

static void random_packet(example_packet_t *pkt)
{
  size_t len = (size_t)rand() % ARRAY_SIZE(pkt->dat);
  for (size_t i = 0; i < len; i++) {
    pkt->dat[i] = (uint8_t)(rand() & 0xff);
  }
  pkt->len = (uint16_t)len;
}

struct example_ctx
{
  uint8_t state;
};

example_ctx_t* example_new_v1(void)
{
  return example_new_v2(1);
}

example_ctx_t* example_new_v2(uint8_t n)
{
  example_ctx_t *ctx = calloc(1, sizeof(example_ctx_t));
  ctx->state = n;
  return ctx;
}

void example_delete_v1(example_ctx_t* ctx)
{
  if (ctx) free(ctx);
}

uint64_t example_fetch_batch_v1(example_ctx_t* ctx, example_batch_t* b)
{
  size_t n = ctx->state;
  assert(n <= ARRAY_SIZE(b->pkts));

  for (size_t i = 0; i < n; i++) {
    random_packet(&b->pkts[i]);
  }
  b->num = (uint8_t)n;

  ctx->state = (ctx->state + 13) % ARRAY_SIZE(b->pkts);

  return EXAMPLE_SUCCESS;
}
