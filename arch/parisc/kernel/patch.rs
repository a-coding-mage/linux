// SPDX-License-Identifier: GPL-2.0
/*
 * functions to patch RO kernel text during runtime
 *
 * Copyright (c) 2019 Sven Schnelle <svens@stackframe.org>
 */

// Kernel and architecture dependencies supplied by other translation units.

#[repr(C)]
struct patch {
    addr: *mut core::ffi::c_void,
    insn: *mut u32,
    len: u32,
}

// Corresponds to DEFINE_RAW_SPINLOCK(patch_lock).
static mut patch_lock: RawSpinlock = RawSpinlock::new();

unsafe fn patch_map(
    addr: *mut core::ffi::c_void,
    fixmap: i32,
    flags: *mut usize,
    need_unmap: *mut i32,
) -> *mut core::ffi::c_void {
    let uintaddr = addr as usize;
    let module = !core_kernel_text(uintaddr);
    let page: *mut Page;

    *need_unmap = 0;
    if module && IS_ENABLED_CONFIG_STRICT_MODULE_RWX {
        page = vmalloc_to_page(addr);
    } else if !module && IS_ENABLED_CONFIG_STRICT_KERNEL_RWX {
        page = virt_to_page(addr);
    } else {
        return addr;
    }

    *need_unmap = 1;
    set_fixmap(fixmap, page_to_phys(page));
    raw_spin_lock_irqsave(&mut patch_lock, flags);

    (__fix_to_virt(fixmap) + (uintaddr & !PAGE_MASK)) as *mut core::ffi::c_void
}

unsafe fn patch_unmap(fixmap: i32, flags: *mut usize) {
    clear_fixmap(fixmap);
    raw_spin_unlock_irqrestore(&mut patch_lock, flags);
}

pub unsafe fn __patch_text_multiple(
    mut addr: *mut core::ffi::c_void,
    mut insn: *mut u32,
    mut len: u32,
) {
    let start = addr as usize;
    let end = start + len as usize;
    let mut flags: usize = 0;
    let mut p: *mut u32;
    let mut fixmap: *mut u32;
    let mut mapped: i32 = 0;

    /* Make sure we don't have any aliases in cache */
    flush_kernel_dcache_range_asm(start, end);
    flush_kernel_icache_range_asm(start, end);
    flush_tlb_kernel_range(start, end);

    p = patch_map(addr, FIX_TEXT_POKE0, &mut flags, &mut mapped) as *mut u32;
    fixmap = p;

    while len >= 4 {
        *p = *insn;
        p = p.add(1);
        insn = insn.add(1);
        addr = (addr as usize + core::mem::size_of::<u32>()) as *mut core::ffi::c_void;
        len -= core::mem::size_of::<u32>() as u32;
        if len != 0 && offset_in_page(addr as usize) == 0 {
            /*
             * We're crossing a page boundary, so
             * need to remap
             */
            flush_kernel_dcache_range_asm(fixmap as usize, p as usize);
            flush_tlb_kernel_range(fixmap as usize, p as usize);
            if mapped != 0 {
                patch_unmap(FIX_TEXT_POKE0, &mut flags);
            }
            p = patch_map(addr, FIX_TEXT_POKE0, &mut flags, &mut mapped) as *mut u32;
            fixmap = p;
        }
    }

    flush_kernel_dcache_range_asm(fixmap as usize, p as usize);
    flush_tlb_kernel_range(fixmap as usize, p as usize);
    if mapped != 0 {
        patch_unmap(FIX_TEXT_POKE0, &mut flags);
    }
}

pub unsafe fn __patch_text(addr: *mut core::ffi::c_void, insn: u32) {
    __patch_text_multiple(addr, &insn as *const u32 as *mut u32, core::mem::size_of::<u32>() as u32);
}

unsafe fn patch_text_stop_machine(data: *mut core::ffi::c_void) -> i32 {
    let patch = &*(data as *const patch);
    __patch_text_multiple(patch.addr, patch.insn, patch.len);
    0
}

pub unsafe fn patch_text(addr: *mut core::ffi::c_void, insn: u32) {
    let patch = patch {
        addr,
        insn: &insn as *const u32 as *mut u32,
        len: core::mem::size_of::<u32>() as u32,
    };
    stop_machine_cpuslocked(patch_text_stop_machine, &patch as *const patch as *mut core::ffi::c_void, core::ptr::null_mut());
}

pub unsafe fn patch_text_multiple(addr: *mut core::ffi::c_void, insn: *mut u32, len: u32) {
    let patch = patch { addr, insn, len };
    stop_machine_cpuslocked(patch_text_stop_machine, &patch as *const patch as *mut core::ffi::c_void, core::ptr::null_mut());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
