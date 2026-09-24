/* SPDX-License-Identifier: GPL-2.0-only */
/* Private real C indirect-call consumer and incompatible-type negative control. */
#include <linux/uuid.h>
#include <linux/random.h>
extern void abort(void);
static void (*volatile ru)(unsigned char *) = generate_random_uuid;
static void (*volatile rg)(unsigned char *) = generate_random_guid;
static void (*volatile ug)(uuid_t *) = uuid_gen;
static void (*volatile gg)(guid_t *) = guid_gen;
static bool (*volatile valid)(const char *) = uuid_is_valid;
static int (*volatile up)(const char *, uuid_t *) = uuid_parse;
static int (*volatile gp)(const char *, guid_t *) = guid_parse;
static bool (*volatile bad)(const char *) = (void *)guid_gen;
void get_random_bytes(void *p, size_t n)
{
	unsigned char *b = p;
	if (n != 16) abort();
	while (n) b[--n] = 0xff;
}
int main(int argc, char **argv);
int main(int argc, char **argv)
{
	uuid_t u;
	guid_t g;
	const char *s = "01234567-89ab-cdef-0123-456789abcdef";
	(void)argv;
	if (argc > 1) {
		bad(s);
		abort();
	}
	ru(u.b); rg(g.b); ug(&u); gg(&g);
	if (!valid(s) || up(s, &u) || gp(s, &g)) abort();
	return 0;
}
