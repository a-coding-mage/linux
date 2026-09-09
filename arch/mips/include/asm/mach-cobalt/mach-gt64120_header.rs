/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  Copyright (C) 2006 Yoichi Yuasa <yuasa@linux-mips.org>
 */

/*
 * Cobalt uses GT64111. GT64111 is almost the same as GT64120.
 */

/*
 * C dependency: CKSEG1ADDR and GT_DEF_BASE are supplied by other headers.
 * This constant preserves the original GT64120_BASE definition.
 */
pub const GT64120_BASE: usize = CKSEG1ADDR(GT_DEF_BASE);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
