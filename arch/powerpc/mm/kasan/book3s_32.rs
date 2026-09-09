// SPDX-License-Identifier: GPL-2.0

// DISABLE_BRANCH_PROFILING
// Dependencies supplied by the kernel build are intentionally not redefined here.

extern "C" {
    fn kasan_mem_to_shadow(addr: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn bat_block_size(start: usize, end: usize) -> u32;
    fn find_free_bat() -> i32;
    fn memblock_phys_alloc_range(size: usize, align: usize, min: usize, max: usize) -> phys_addr_t;
    fn setbat(idx: i32, virt: usize, phys: phys_addr_t, size: u32, flags: usize);
    fn update_bats();
    fn kasan_init_shadow_page_tables(start: usize, end: usize) -> i32;
    fn kasan_update_early_region(start: usize, end: usize, pte: pte_t);
    fn pfn_pte(pfn: usize, prot: usize) -> pte_t;
    fn pmd_off_k(addr: usize) -> *mut pmd_t;
    fn pte_offset_kernel(pmd: *mut pmd_t, addr: usize) -> *mut pte_t;
    fn __set_pte_at(mm: *mut mm_struct, addr: usize, ptep: *mut pte_t, pte: pte_t, flags: i32);
    fn flush_tlb_kernel_range(start: usize, end: usize);
    static mut init_mm: mm_struct;
}

extern "C" {
    fn memset(dst: *mut core::ffi::c_void, value: i32, len: usize) -> *mut core::ffi::c_void;
}

// External kernel types and constants.
type phys_addr_t = usize;
type pmd_t = usize;
type pte_t = usize;
type mm_struct = usize;

const SZ_128K: u32 = 128 * 1024;
const PAGE_SIZE: usize = 4096;
const PAGE_KERNEL: usize = 0;
const MEMBLOCK_ALLOC_ANYWHERE: usize = usize::MAX;
const ENOMEM: i32 = 12;

#[no_mangle]
pub unsafe extern "C" fn kasan_init_region(
    start: *mut core::ffi::c_void,
    size: usize,
) -> i32 {
    let k_start = kasan_mem_to_shadow(start) as usize;
    let k_end = kasan_mem_to_shadow(start.add(size)) as usize;
    let mut k_nobat = k_start;
    let mut k_cur: usize;
    let mut phys: phys_addr_t;
    let ret: i32;

    while k_nobat < k_end {
        let k_size = bat_block_size(k_nobat, k_end);
        let idx = find_free_bat();

        if idx == -1 {
            break;
        }
        if k_size < SZ_128K {
            break;
        }
        phys = memblock_phys_alloc_range(
            k_size as usize,
            k_size as usize,
            0,
            MEMBLOCK_ALLOC_ANYWHERE,
        );
        if phys == 0 {
            break;
        }

        setbat(idx, k_nobat, phys, k_size, PAGE_KERNEL);
        k_nobat += k_size as usize;
    }
    if k_nobat != k_start {
        update_bats();
    }

    if k_nobat < k_end {
        phys = memblock_phys_alloc_range(
            k_end - k_nobat,
            PAGE_SIZE,
            0,
            MEMBLOCK_ALLOC_ANYWHERE,
        );
        if phys == 0 {
            return -ENOMEM;
        }
    }

    ret = kasan_init_shadow_page_tables(k_start, k_end);
    if ret != 0 {
        return ret;
    }

    kasan_update_early_region(k_start, k_nobat, 0);

    k_cur = k_nobat;
    while k_cur < k_end {
        let pmd = pmd_off_k(k_cur);
        let pte = pfn_pte(
            (phys.wrapping_add(k_cur.wrapping_sub(k_nobat))) / PAGE_SIZE,
            PAGE_KERNEL,
        );

        __set_pte_at(&mut init_mm, k_cur, pte_offset_kernel(pmd, k_cur), pte, 0);
        k_cur += PAGE_SIZE;
    }
    flush_tlb_kernel_range(k_start, k_end);
    memset(kasan_mem_to_shadow(start), 0, k_end - k_start);

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
