/* SPDX-License-Identifier: GPL-2.0-only */
#include "canonical.h"
#include <linux/ctype.h>
#include <linux/err.h>
#include <asm/word-at-a-time.h>

/* No ordinary module parameters are supplied by bootconfig opt-in parsing. */
static bool param_check_unsafe(const struct kernel_param *parameter) { __builtin_trap(); }
void kernel_param_lock(struct module *module) { __builtin_trap(); }
void kernel_param_unlock(struct module *module) { __builtin_trap(); }
#undef irqs_disabled
#define irqs_disabled() 1
#include "params.inc"

#ifdef __BIG_ENDIAN
#define ALLBUTLAST_BYTE_MASK (~255ul)
#else
#define ALLBUTLAST_BYTE_MASK (~0ul >> 8)
#endif
#undef WARN_ON_ONCE
#define WARN_ON_ONCE(condition) ({ bool bad = (condition); if (bad) __builtin_trap(); bad; })
#include "strings.inc"
