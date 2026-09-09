// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the surrounding kernel translation unit:
// linux/pgtable.h, asm/abs_lowcore.h, and asm/sections.h.

#[repr(C)]
pub struct lowcore {
    _private: [u8; 0],
}

// The original declaration is annotated by __bootdata_preserved.
#[no_mangle]
pub static mut __abs_lowcore: usize = 0;

extern "C" {
    fn __pa(lc: *mut lowcore) -> usize;
    fn __vmem_map_4k_page(
        addr: usize,
        phys: usize,
        prot: usize,
        alloc: bool,
    ) -> i32;
    fn vmem_unmap_4k_page(addr: usize);
}

extern "C" {
    static PAGE_KERNEL: usize;
    static PAGE_SIZE: usize;
    static LC_PAGES: i32;
}

pub unsafe fn abs_lowcore_map(cpu: i32, lc: *mut lowcore, alloc: bool) -> i32 {
    let mut addr = __abs_lowcore.wrapping_add(
        (cpu as usize).wrapping_mul(core::mem::size_of::<lowcore>()),
    );
    let mut phys = __pa(lc);
    let mut rc: i32;
    let mut i: i32;

    i = 0;
    while i < LC_PAGES {
        rc = __vmem_map_4k_page(addr, phys, PAGE_KERNEL, alloc);
        if rc != 0 {
            /*
             * Do not unmap allocated page tables in case the
             * allocation was not requested. In such a case the
             * request is expected coming from an atomic context,
             * while the unmap attempt might sleep.
             */
            if alloc {
                i -= 1;
                while i >= 0 {
                    addr = addr.wrapping_sub(PAGE_SIZE);
                    vmem_unmap_4k_page(addr);
                    i -= 1;
                }
            }
            return rc;
        }
        addr = addr.wrapping_add(PAGE_SIZE);
        phys = phys.wrapping_add(PAGE_SIZE);
        i += 1;
    }
    0
}

pub unsafe fn abs_lowcore_unmap(cpu: i32) {
    let mut addr = __abs_lowcore.wrapping_add(
        (cpu as usize).wrapping_mul(core::mem::size_of::<lowcore>()),
    );
    let mut i: i32;

    i = 0;
    while i < LC_PAGES {
        vmem_unmap_4k_page(addr);
        addr = addr.wrapping_add(PAGE_SIZE);
        i += 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
