// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (C) 2020 SiFive
 * Copyright (C) 2025 Chen Miao
 */

// Dependencies corresponding to the C includes are supplied by the kernel.

static mut patch_lock: raw_spinlock_t = raw_spinlock_t::new();

#[inline(always)]
unsafe fn patch_map(addr: *mut core::ffi::c_void, fixmap: i32) -> *mut core::ffi::c_void {
    let uaddr = addr as usize;
    let phys: phys_addr_t;

    if core_kernel_text(uaddr) {
        phys = __pa_symbol(addr);
    } else {
        let page: *mut page = vmalloc_to_page(addr);
        BUG_ON(page.is_null());
        phys = page_to_phys(page) + offset_in_page(addr);
    }

    set_fixmap_offset(fixmap, phys) as *mut core::ffi::c_void
}

unsafe fn patch_unmap(fixmap: i32) {
    clear_fixmap(fixmap);
}

unsafe fn __patch_insn_write(addr: *mut core::ffi::c_void, insn: u32) -> i32 {
    let mut waddr = addr;
    let mut flags: unsigned_long = 0;

    raw_spin_lock_irqsave(&raw mut patch_lock, &mut flags);

    waddr = patch_map(addr, FIX_TEXT_POKE0);

    let ret = copy_to_kernel_nofault(
        waddr,
        &insn as *const u32 as *const core::ffi::c_void,
        OPENRISC_INSN_SIZE,
    );
    if !IS_ENABLED(CONFIG_DCACHE_WRITETHROUGH) {
        local_dcache_range_flush(
            waddr as unsigned_long,
            waddr as unsigned_long + OPENRISC_INSN_SIZE,
        );
    }
    local_icache_range_inv(
        waddr as unsigned_long,
        waddr as unsigned_long + OPENRISC_INSN_SIZE,
    );

    patch_unmap(FIX_TEXT_POKE0);

    raw_spin_unlock_irqrestore(&raw mut patch_lock, flags);

    ret
}

/*
 * patch_insn_write - Write a single instruction to a specified memory location
 * This API provides a single-instruction patching, primarily used for runtime
 * code modification.
 * By the way, the insn size must be 4 bytes.
 */
#[no_mangle]
pub unsafe extern "C" fn patch_insn_write(addr: *mut core::ffi::c_void, insn: u32) -> i32 {
    let tp = addr as *mut u32;
    let ret: i32;

    if (tp as usize) & 0x3 != 0 {
        return -EINVAL;
    }

    ret = __patch_insn_write(tp as *mut core::ffi::c_void, insn);

    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
