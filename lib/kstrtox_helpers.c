// SPDX-License-Identifier: GPL-2.0
/*
 * Compile-time ties for the Rust kstrto*() owner in lib/kstrtox.rs, which
 * spells the overflow flag and ctype classes it uses.
 */
#include <linux/build_bug.h>
#include <linux/ctype.h>

#include "kstrtox.h"

static_assert(KSTRTOX_OVERFLOW == (1U << 31));
static_assert(_D == 0x04);
static_assert(_X == 0x40);
