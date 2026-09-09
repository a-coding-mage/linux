/* SPDX-License-Identifier: GPL-2.0 */

/*
 * The C header guard and __ASSEMBLY__ conditional are intentionally omitted
 * from executable Rust.  This translation applies to the non-assembly part
 * of the header.
 *
 * Dependencies supplied by the included headers:
 * - `vdso_time_data`
 * - `virt_to_page`
 * - `flush_dcache_page`
 */

pub unsafe fn __arch_sync_vdso_time_data(vdata: *mut vdso_time_data) {
    flush_dcache_page(virt_to_page(vdata));
}

/* The C macro aliases the function to itself. */

/* The asm-generic header is included after the definitions above in C. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
