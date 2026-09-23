// SPDX-License-Identifier: GPL-2.0-only
/*
 * Export metadata for the Rust implementation; no conversion code lives here.
 * Use the public C prototypes for caller-compatible symbol versions. GCC omits
 * parameter names from declaration-only DWARF, so switching implementations
 * requires rebuilding modules even though the machine-level ABI is unchanged.
 * This uses the same export boundary as rust/exports.c.
 */
#include <linux/export.h>
#include <linux/hex.h>
#include <linux/printk.h>

/* Complete the public array declarations for the defined object sizes. */
extern const char hex_asc[17];
extern const char hex_asc_upper[17];

EXPORT_SYMBOL(hex_asc);
EXPORT_SYMBOL(hex_asc_upper);
EXPORT_SYMBOL(hex_to_bin);
EXPORT_SYMBOL(hex2bin);
EXPORT_SYMBOL(bin2hex);
EXPORT_SYMBOL(hex_dump_to_buffer);
#ifdef CONFIG_PRINTK
EXPORT_SYMBOL(print_hex_dump);
#endif
