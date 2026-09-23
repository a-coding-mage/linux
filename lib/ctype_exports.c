// SPDX-License-Identifier: GPL-2.0
/* Export metadata only; the selected Rust object defines the complete table. */
#include <linux/ctype.h>
#include <linux/export.h>

/* Clang needs the complete bound to retain the definition's DWARF type. */
extern const unsigned char _ctype[256];

EXPORT_SYMBOL(_ctype);
