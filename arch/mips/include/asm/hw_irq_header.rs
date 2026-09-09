/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2000, 2001, 2002 by Ralf Baechle
 */

// Dependency supplied by the surrounding translation environment:
// #include <linux/atomic.h>

extern "C" {
    pub static mut irq_err_count: atomic_t;
}

/*
 * interrupt-retrigger: NOP for now. This may not be appropriate for all
 * machines, we'll see ...
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
