/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// Dependencies supplied by the Linux memory-management and MIPS page-table
// headers are intentionally left as external Rust items.

extern "C" {
    fn __pgd_alloc(mm: *mut mm_struct, order: usize) -> *mut pgd_t;
    fn pgd_offset(mm: *mut mm_struct, address: usize) -> *mut pgd_t;
    fn pgd_init(pgd: *mut pgd_t);
    static mut init_mm: mm_struct;
}

#[no_mangle]
pub unsafe extern "C" fn pgd_alloc(mm: *mut mm_struct) -> *mut pgd_t {
    let init: *mut pgd_t;
    let ret: *mut pgd_t;

    ret = __pgd_alloc(mm, PGD_TABLE_ORDER);
    if !ret.is_null() {
        init = pgd_offset(&mut init_mm, 0usize);
        pgd_init(ret);
        core::ptr::copy_nonoverlapping(
            init.add(USER_PTRS_PER_PGD).cast::<u8>(),
            ret.add(USER_PTRS_PER_PGD).cast::<u8>(),
            (PTRS_PER_PGD - USER_PTRS_PER_PGD) * core::mem::size_of::<pgd_t>(),
        );
    }

    ret
}

// EXPORT_SYMBOL_GPL(pgd_alloc)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
