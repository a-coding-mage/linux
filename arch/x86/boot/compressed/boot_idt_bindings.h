/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_BOOT_IDT_BINDINGS_H
#define LUPOS_BOOT_IDT_BINDINGS_H
#ifdef __BINDGEN__
/* Expose declarations to bindgen without changing their native C types. */
#pragma GCC visibility push(default)
#endif
#include "misc.h"
#include <asm/segment.h>
#include <asm/trapnr.h>

/* Evaluate the exact selector and status test used by native idt_64.c. */
enum {
	LUPOS_BOOT_IDT_KERNEL_CS = __KERNEL_CS,
	LUPOS_BOOT_IDT_SEV_ES_ENABLED = BIT(1),
};
#ifdef __BINDGEN__
#pragma GCC visibility pop
#endif
#endif
