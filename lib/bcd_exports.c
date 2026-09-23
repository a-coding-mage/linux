// SPDX-License-Identifier: GPL-2.0
/* Export metadata only; the selected Rust object defines both functions. */
#include <linux/bcd.h>
#include <linux/export.h>

EXPORT_SYMBOL(_bcd2bin);
EXPORT_SYMBOL(_bin2bcd);
