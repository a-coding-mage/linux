/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Canonical declarations for the unselected Rust init/main implementation.
 * Include this before other kernel headers: normal Rust bindings use MODULE,
 * which otherwise hides the built-in setup records behind init.h's guard.
 * This file supplies no C implementation or duplicate boot state.
 */
#ifndef _RUST_BINDINGS_INIT_MAIN_H
#define _RUST_BINDINGS_INIT_MAIN_H

#ifdef _LINUX_INIT_H
#error "init_main.h must precede linux/init.h in the binding input"
#endif

#pragma push_macro("MODULE")
#undef MODULE
#include <linux/init.h>
#pragma pop_macro("MODULE")

#include <linux/string.h>
#include <linux/binfmts.h>
#include <linux/bootconfig.h>
#include <linux/cache.h>
#include <linux/completion.h>
#include <linux/dynamic_debug.h>
#include <linux/file.h>
#include <linux/fs.h>
#include <linux/init_syscalls.h>
#include <linux/initrd.h>
#include <linux/jump_label.h>
#include <linux/kallsyms.h>
#include <linux/memblock.h>
#include <linux/mm.h>
#include <linux/moduleparam.h>
#include <linux/moduleloader.h>
#include <linux/panic.h>
#include <linux/poison.h>
#include <linux/pid.h>
#include <linux/printk.h>
#include <linux/ptdump.h>
#include <linux/rodata_test.h>
#include <linux/sched/task.h>
#include <linux/timekeeping.h>
#include <linux/tracepoint.h>
#include <asm/setup.h>
#include <asm/sections.h>
#ifdef CONFIG_ARM64
#include <asm/mmu_context.h>
#endif
#include <trace/events/initcall.h>

enum {
	RUST_INIT_MAIN_COMMAND_LINE_SIZE = COMMAND_LINE_SIZE,
	RUST_INIT_MAIN_MAX_INIT_ARGS = CONFIG_INIT_ENV_ARG_LIMIT,
	RUST_INIT_MAIN_MAX_INIT_ENVS = CONFIG_INIT_ENV_ARG_LIMIT,
	RUST_INIT_MAIN_SMP_CACHE_BYTES = SMP_CACHE_BYTES,
	RUST_INIT_MAIN_CMDLINE_LOG_WRAP_IDEAL_LEN = CONFIG_CMDLINE_LOG_WRAP_IDEAL_LEN,
	RUST_INIT_MAIN_DPRINTK_CLASS_BITS = CLS_BITS,
	RUST_INIT_MAIN_DPRINTK_CLASS_DEFAULT = _DPRINTK_CLASS_DFLT,
	RUST_INIT_MAIN_DPRINTK_FLAGS_PRINT = _DPRINTK_FLAGS_PRINT,
	RUST_INIT_MAIN_DPRINTK_FLAGS_STACK = _DPRINTK_FLAGS_INCL_STACK,
	RUST_INIT_MAIN_THREAD_SIZE = THREAD_SIZE,
	RUST_INIT_MAIN_PAGE_SIZE = PAGE_SIZE,
	RUST_INIT_MAIN_KSYM_SYMBOL_LEN = KSYM_SYMBOL_LEN,
};

#ifdef CONFIG_LIST_HARDENED
/* The actual forward-only ABI boundary is defined in rust/helpers/list.c.
 * The original reporter can use __preserve_most, unsupported by bindgen/Rust. */
bool rust_helper___list_add_valid_or_report(struct list_head *new,
					  struct list_head *prev,
					  struct list_head *next);
#endif

#define RUST_INIT_MAIN_DEFAULT_INIT CONFIG_DEFAULT_INIT

const phys_addr_t RUST_INIT_MAIN_MEMBLOCK_LOW_LIMIT = MEMBLOCK_LOW_LIMIT;
const phys_addr_t RUST_INIT_MAIN_MEMBLOCK_ALLOC_ACCESSIBLE = MEMBLOCK_ALLOC_ACCESSIBLE;

/* SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783 */
#endif
