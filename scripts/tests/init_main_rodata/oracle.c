/* SPDX-License-Identifier: GPL-2.0-only */
#define rodata_enabled original_rodata_enabled
#define rodata_full original_rodata_full
#include "canonical.h"
#ifdef FIXTURE_ARM64
#include "arm-parser.inc"
#define arch_parse_debug_rodata arch_parse_debug_rodata
#endif
#include "original.inc"

int original_set(char *value)
{
#if defined(CONFIG_STRICT_KERNEL_RWX) || defined(CONFIG_STRICT_MODULE_RWX)
	return set_debug_rodata(value);
#else
	return 0;
#endif
}
void original_mark(void) { mark_readonly(); }
