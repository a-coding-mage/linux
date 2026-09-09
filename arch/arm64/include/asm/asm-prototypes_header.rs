/* SPDX-License-Identifier: GPL-2.0 */
/*
 * CONFIG_MODVERSIONS requires a C declaration to generate the appropriate CRC
 * for each symbol. Since commit:
 *
 *   4efca4ed05cbdfd1 ("kbuild: modversions for EXPORT_SYMBOL() for asm")
 *
 * ... kbuild will automatically pick these up from <asm/asm-prototypes.h> and
 * feed this to genksyms when building assembly files.
 *
 * Dependencies supplied by the original includes:
 * linux/arm-smccc.h, asm/ftrace.h, asm/page.h, asm/string.h,
 * asm/uaccess.h, and asm-generic/asm-prototypes.h.
 */

extern "C" {
    pub fn __ashlti3(a: i64, b: i32) -> i64;
    pub fn __ashrti3(a: i64, b: i32) -> i64;
    pub fn __lshrti3(a: i64, b: i32) -> i64;
}

/*
 * This function uses a custom calling convention and cannot be called from C so
 * this prototype is not entirely accurate.
 */
extern "C" {
    pub fn __hwasan_tag_mismatch(addr: u64, access_info: u64);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
