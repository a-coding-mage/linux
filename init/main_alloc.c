// SPDX-License-Identifier: GPL-2.0-only
/*
 * Compiler boundary selected with the staged Rust init/main owner.
 * kzalloc's macro expansion creates per-callsite allocation tags/per-CPU
 * counters and, for typed slab partitions, a compiler-inferred allocation
 * token. Rust cannot express that Clang builtin. Keep only this allocation
 * expression here; phase ordering, failure handling and lifetime stay in Rust.
 */
#include <linux/init.h>
#include <linux/slab.h>

char *rust_init_main_kzalloc_command_line(size_t length);

char *__init rust_init_main_kzalloc_command_line(size_t length)
{
	char *command_line = kzalloc(length, GFP_KERNEL);

	return command_line;
}

/* SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783 */
