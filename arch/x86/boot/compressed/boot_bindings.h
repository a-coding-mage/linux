/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_COMPRESSED_RUST_BINDINGS_H
#define LUPOS_COMPRESSED_RUST_BINDINGS_H
#ifdef __BINDGEN__
/* Bindgen omits hidden function declarations; visibility changes no C type. */
#pragma GCC visibility push(default)
#endif
#include "misc.h"
#include "error.h"
#include "../string.h"
#include "../voffset.h"
#define _SETUP
#include <asm/setup.h>
#undef _SETUP
#include <asm/kexec_handover.h>
#include <asm/e820/types.h>
#include <asm/archrandom.h>
#include <asm/msr-index.h>
#include <linux/zlib.h>
#include <linux/zutil.h>
#include "../../../../lib/zlib_inflate/inftrees.h"
#include "../../../../lib/zlib_inflate/inflate.h"
#include "../../../../lib/zlib_inflate/infutil.h"

typedef memptr boot_memptr;
unsigned long get_cmd_line_ptr(void);
unsigned long lupos_boot_maxmem(void);

/* Clang evaluates compound macros using this exact compressed C environment. */
enum {
	LUPOS_BOOT_LOAD_PHYSICAL_ADDR = LOAD_PHYSICAL_ADDR,
	LUPOS_BOOT_BOOT_HEAP_SIZE = BOOT_HEAP_SIZE,
	LUPOS_BOOT_MIN_KERNEL_ALIGN = MIN_KERNEL_ALIGN,
	LUPOS_BOOT_KERNEL_IMAGE_SIZE = KERNEL_IMAGE_SIZE,
	LUPOS_BOOT_START_KERNEL_MAP = __START_KERNEL_map,
	LUPOS_BOOT_VO_START_RODATA = VO___start_rodata,
	LUPOS_BOOT_VO_TEXT = VO__text,
	LUPOS_BOOT_VO_SINITTEXT = VO__sinittext,
	LUPOS_BOOT_VO_INITTEXT_END = VO___inittext_end,
	LUPOS_BOOT_VO_END = VO__end,
	LUPOS_BOOT_VO_BSS_START = VO___bss_start,
	LUPOS_BOOT_PUD_SIZE = PUD_SIZE,
	LUPOS_BOOT_PUD_SHIFT = PUD_SHIFT,
	LUPOS_BOOT_SEV_ES_ENABLED = MSR_AMD64_SEV_ES_ENABLED,
	LUPOS_BOOT_EFI_MEMORY_MORE_RELIABLE = EFI_MEMORY_MORE_RELIABLE,
	LUPOS_BOOT_EFI_MEMORY_SP = EFI_MEMORY_SP,
	LUPOS_BOOT_E820_TYPE_RAM = E820_TYPE_RAM,
};
#ifdef __BINDGEN__
#pragma GCC visibility pop
#endif
#endif
