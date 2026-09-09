/*
 * Copyright (C) 2009 Wind River Systems Inc
 *   Implemented by fredrik.markstrom@gmail.com and ivarholmqvist@gmail.com
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

/* pteaddr:
 *   ptbase | vpn* | zero
 *   31-22  | 21-2 | 1-0
 *
 *   *vpn is preserved on double fault
 *
 * tlbacc:
 *   IG   |*flags| pfn
 *   31-25|24-20 | 19-0
 *
 *   *crwxg
 *
 * tlbmisc:
 *   resv  |way   |rd | we|pid |dbl|bad|perm|d
 *   31-24 |23-20 |19 | 20|17-4|3  |2  |1   |0
 *
 */

/*
 * Initialize a new pgd / pmd table with invalid pointers.
 */
unsafe fn pgd_init(pgd: *mut pgd_t) {
    let p = pgd as *mut ::core::ffi::c_ulong;
    let mut i: ::core::ffi::c_int = 0;

    while i < USER_PTRS_PER_PGD {
        *p.add((i + 0) as usize) = invalid_pte_table as usize as ::core::ffi::c_ulong;
        *p.add((i + 1) as usize) = invalid_pte_table as usize as ::core::ffi::c_ulong;
        *p.add((i + 2) as usize) = invalid_pte_table as usize as ::core::ffi::c_ulong;
        *p.add((i + 3) as usize) = invalid_pte_table as usize as ::core::ffi::c_ulong;
        *p.add((i + 4) as usize) = invalid_pte_table as usize as ::core::ffi::c_ulong;
        *p.add((i + 5) as usize) = invalid_pte_table as usize as ::core::ffi::c_ulong;
        *p.add((i + 6) as usize) = invalid_pte_table as usize as ::core::ffi::c_ulong;
        *p.add((i + 7) as usize) = invalid_pte_table as usize as ::core::ffi::c_ulong;
        i += 8;
    }
}

pub unsafe extern "C" fn pgd_alloc(mm: *mut mm_struct) -> *mut pgd_t {
    let ret: *mut pgd_t;
    let init: *mut pgd_t;

    ret = __pgd_alloc(mm, 0);
    if !ret.is_null() {
        init = pgd_offset(&raw mut init_mm, 0usize);
        pgd_init(ret);
        ::core::ptr::copy_nonoverlapping(
            init.add(USER_PTRS_PER_PGD as usize),
            ret.add(USER_PTRS_PER_PGD as usize),
            (PTRS_PER_PGD - USER_PTRS_PER_PGD) as usize,
        );
    }

    ret
}

pub unsafe extern "C" fn pagetable_init() {
    /* Initialize the entire pgd.  */
    pgd_init(swapper_pg_dir);
    pgd_init(swapper_pg_dir.add(USER_PTRS_PER_PGD as usize));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
