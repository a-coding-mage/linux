/* SPDX-License-Identifier: GPL-2.0-only */
#include <linux/moduleparam.h>
#include <linux/string.h>
#include <linux/printk.h>
#include <linux/ctype.h>
#include <linux/err.h>
#include <asm/word-at-a-time.h>

/* This caller always supplies no ordinary parameters. Fail if it ever enters
 * their unrelated lock/security paths; retain the entire original parse_one. */
static bool param_check_unsafe(const struct kernel_param *parameter)
{
	__builtin_trap();
}
void kernel_param_lock(struct module *module) { __builtin_trap(); }
void kernel_param_unlock(struct module *module) { __builtin_trap(); }
#undef irqs_disabled
#define irqs_disabled() 1

#define parse_args original_parse_args
char *parse_args(const char *, char *, const struct kernel_param *, unsigned int,
		s16, s16, void *, parse_unknown_fn);
#include "params.inc"
#undef parse_args

unsigned int parser_calls;
char *parse_args(const char *doing, char *args, const struct kernel_param *params,
		unsigned int num, s16 min_level, s16 max_level, void *arg,
		parse_unknown_fn unknown)
{
	if (strcmp(doing, "early options") || params || num || min_level ||
	    max_level || arg || !unknown)
		__builtin_trap();
	parser_calls++;
	return original_parse_args(doing, args, params, num, min_level, max_level,
				   arg, unknown);
}

#ifdef __BIG_ENDIAN
#define ALLBUTLAST_BYTE_MASK (~255ul)
#else
#define ALLBUTLAST_BYTE_MASK (~0ul >> 8)
#endif
/* The supplied copy bound is COMMAND_LINE_SIZE, never the WARN condition. */
#undef WARN_ON_ONCE
#define WARN_ON_ONCE(condition) ({ bool bad = (condition); if (bad) __builtin_trap(); bad; })
#include "strings.inc"
