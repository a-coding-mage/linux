// SPDX-License-Identifier: GPL-2.0
/***************************************************************************/

/*
 *	cache.rs -- general ColdFire Cache maintenance code
 *
 *	Copyright (C) 2010, Greg Ungerer (gerg@snapgear.com)
 */

/***************************************************************************/

// Dependencies supplied by the surrounding kernel translation:
// linux/kernel.h, asm/coldfire.h, and asm/mcfsim.h

/***************************************************************************/
// CACHE_PUSH is a build-time preprocessor condition in the C source.  The
// corresponding Rust feature is preserved here to retain that intent.
#[cfg(feature = "CACHE_PUSH")]
/***************************************************************************/

/*
 *	Use cpushl to push all dirty cache lines back to memory.
 *	Older versions of GAS don't seem to know how to generate the
 *	ColdFire cpushl instruction... Oh well, bit stuff it for now.
 */

#[cfg(feature = "CACHE_PUSH")]
pub unsafe fn mcf_cache_push() {
    core::arch::asm!(
        "clrl\td0",
        "1:",
        "movel\td0,a0",
        "2:",
        ".word 0xf468",
        "addl {line_size},a0",
        "cmpl {cache_size},a0",
        "blt 2b",
        "addql #1,d0",
        "cmpil {cache_ways},d0",
        "bne 1b",
        line_size = const CACHE_LINE_SIZE,
        cache_size = const (DCACHE_SIZE / CACHE_WAYS),
        cache_ways = const CACHE_WAYS,
        out("d0") _,
        out("a0") _,
    );
}

/***************************************************************************/

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
