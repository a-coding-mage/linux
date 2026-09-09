/* SPDX-License-Identifier: GPL-2.0 */

/*
 * C dependencies:
 *   linux/compiler.h
 *   linux/pgtable.h
 *
 * The __idmap macro tags a function as requiring execution via an identity
 * mapping: __section(".idmap.text") noinline notrace.
 */

extern "C" {
    pub static mut idmap_pgd: *mut pgd_t;

    pub fn setup_mm_for_reboot();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
