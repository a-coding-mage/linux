/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_BOOT_CPU_BINDINGS_H
#define LUPOS_BOOT_CPU_BINDINGS_H
#ifdef __BINDGEN__
#pragma GCC visibility push(default)
#endif
#include <linux/types.h>
#include "../cpuflags.h"

/* Isolated from the kernel cpuid_count API, whose signature is different. */
enum {
	LUPOS_BOOT_X86_CR0_EM = X86_CR0_EM,
	LUPOS_BOOT_X86_CR0_TS = X86_CR0_TS,
	LUPOS_BOOT_X86_EFLAGS_ID = X86_EFLAGS_ID,
};
#ifdef __BINDGEN__
#pragma GCC visibility pop
#endif
#endif
