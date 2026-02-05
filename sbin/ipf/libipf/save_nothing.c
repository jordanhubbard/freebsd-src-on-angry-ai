#include "ipf.h"
#include "ipmon.h"

static void *nothing_parse(char **);
static void nothing_destroy(void *);
static int nothing_send(void *, ipmon_msg_t *);

typedef struct nothing_opts_s {
	FILE	*fp;
	int	raw;
	char	*path;
} nothing_opts_t;

ipmon_saver_t nothingsaver = {
	"nothing",
	nothing_destroy,
	NULL,		/* dup */
	NULL,		/* match */
	nothing_parse,
	NULL,		/* print */
	nothing_send
};


static void *
nothing_parse(char **strings)
{
    nothing_opts_t *ctx = calloc(1, sizeof(*ctx));
    if (!ctx) {
        return NULL;
    }
    /* Initialise fields (even though they are not used now) */
    ctx->fp = NULL;
    ctx->raw = 0;
    ctx->path = NULL;
    return (void *)ctx;
}


static void
nothing_destroy(void *ctx)
{
	free(ctx);
}


static int
nothing_send(void *ctx, ipmon_msg_t *msg)
{
#if 0
	ctx = ctx;	/* gcc -Wextra */
	msg = msg;	/* gcc -Wextra */
#endif
	/*
	 * Do nothing
	 */
	return (0);
}

