/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_STARTUP_GDT_BINDINGS_H
#define LUPOS_STARTUP_GDT_BINDINGS_H
#ifdef __BINDGEN__
/* hidden.h remains active for native compilation; expose declarations here. */
#pragma GCC visibility push(default)
#endif
#include <asm/desc.h>
#include <asm/sev.h>
#include <asm/trapnr.h>

/* Native macro values only; no translated behavior lives in C helpers. */
enum {
	LUPOS_STARTUP_GDT_PAGE_SIZE = PAGE_SIZE,
	LUPOS_STARTUP_GDT_KERNEL_CS = __KERNEL_CS,
	LUPOS_STARTUP_GDT_KERNEL_DS = __KERNEL_DS,
};
#ifdef __BINDGEN__
#pragma GCC visibility pop
#endif
#endif
