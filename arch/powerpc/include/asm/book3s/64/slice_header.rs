/* SPDX-License-Identifier: GPL-2.0 */

/*
 * The following architecture feature definitions are controlled by the C
 * build configuration.  Keep their intent here for configuration integration.
 */
// Under CONFIG_PPC_64S_HASH_MMU:
//   under CONFIG_HUGETLB_PAGE: HAVE_ARCH_HUGETLB_UNMAPPED_AREA
//   HAVE_ARCH_UNMAPPED_AREA
//   HAVE_ARCH_UNMAPPED_AREA_TOPDOWN

pub const SLICE_LOW_SHIFT: u32 = 28;
pub const SLICE_LOW_TOP: u64 = 0x1_0000_0000;
pub const SLICE_NUM_LOW: u64 = SLICE_LOW_TOP >> SLICE_LOW_SHIFT;

#[inline]
pub const fn GET_LOW_SLICE_INDEX(addr: u64) -> u64 {
    addr >> SLICE_LOW_SHIFT
}

pub const SLICE_HIGH_SHIFT: u32 = 40;
pub const SLICE_NUM_HIGH: u64 = H_PGTABLE_RANGE >> SLICE_HIGH_SHIFT;

#[inline]
pub const fn GET_HIGH_SLICE_INDEX(addr: u64) -> u64 {
    addr >> SLICE_HIGH_SHIFT
}

pub const SLB_ADDR_LIMIT_DEFAULT: u64 = DEFAULT_MAP_WINDOW_USER64;

#[repr(C)]
pub struct mm_struct {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn slice_get_unmapped_area(
        addr: u64,
        len: u64,
        flags: u64,
        psize: u32,
        topdown: i32,
    ) -> u64;

    pub fn get_slice_psize(mm: *mut mm_struct, addr: u64) -> u32;

    pub fn slice_set_range_psize(
        mm: *mut mm_struct,
        start: u64,
        len: u64,
        psize: u32,
    );

    pub fn slice_init_new_context_exec(mm: *mut mm_struct);
    pub fn slice_setup_new_exec();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
