// SPDX-License-Identifier: GPL-2.0
/* Export metadata only; int_math_rust.o supplies the function definitions. */
#include <linux/export.h>
#include <linux/math.h>

EXPORT_SYMBOL_GPL(int_pow);
EXPORT_SYMBOL(int_sqrt);
#if BITS_PER_LONG < 64
EXPORT_SYMBOL(int_sqrt64);
#endif
