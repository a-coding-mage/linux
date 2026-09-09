// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2020 SiFive
 */

use core::sync::atomic::{AtomicI32, Ordering};

#[repr(C)]
struct patch_insn {
    addr: *mut core::ffi::c_void,
    insns: *mut u32,
    len: usize,
    cpu_count: AtomicI32,
}

static mut riscv_patch_in_stop_machine: bool = false;

// External kernel declarations supplied by the surrounding build.
extern "C" {
    static mut system_state: i32;
    static __exittext_begin: u8;
    static __exittext_end: u8;
    static text_mutex: core::ffi::c_void;
    static cpu_online_mask: core::ffi::c_void;
    static mut num_online_cpus: i32;

    fn core_kernel_text(addr: usize) -> bool;
    fn __pa_symbol(addr: *mut core::ffi::c_void) -> usize;
    fn vmalloc_to_page(addr: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn page_to_phys(page: *mut core::ffi::c_void) -> usize;
    fn offset_in_page(addr: *const core::ffi::c_void) -> usize;
    fn set_fixmap_offset(fixmap: i32, phys: usize) -> *mut core::ffi::c_void;
    fn clear_fixmap(fixmap: i32);
    fn local_flush_icache_range(start: usize, end: usize);
    fn flush_icache_range(start: usize, end: usize);
    fn local_flush_icache_all();
    fn copy_to_kernel_nofault(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, len: usize) -> i32;
    fn stop_machine_cpuslocked(
        callback: unsafe extern "C" fn(*mut core::ffi::c_void) -> i32,
        data: *mut core::ffi::c_void,
        mask: *const core::ffi::c_void,
    ) -> i32;
    fn cpu_relax();
}

const PAGE_SIZE: usize = 4096;
const FIX_TEXT_POKE0: i32 = 0;
const FIX_TEXT_POKE1: i32 = 1;
const SYSTEM_RUNNING: i32 = 0;
const EINVAL: i32 = 22;

#[cfg(feature = "CONFIG_MMU")]
#[inline(always)]
unsafe fn is_kernel_exittext(addr: usize) -> bool {
    system_state < SYSTEM_RUNNING
        && addr >= (&__exittext_begin as *const u8 as usize)
        && addr < (&__exittext_end as *const u8 as usize)
}

#[cfg(feature = "CONFIG_MMU")]
#[inline(always)]
unsafe fn patch_map(addr: *mut core::ffi::c_void, fixmap: u32) -> *mut core::ffi::c_void {
    let uintaddr = addr as usize;
    let phys: usize;

    if core_kernel_text(uintaddr) || is_kernel_exittext(uintaddr) {
        phys = __pa_symbol(addr);
    } else if cfg!(feature = "CONFIG_STRICT_MODULE_RWX") {
        let page = vmalloc_to_page(addr);
        assert!(!page.is_null());
        phys = page_to_phys(page) + offset_in_page(addr);
    } else {
        return addr;
    }

    set_fixmap_offset(fixmap as i32, phys)
}

#[cfg(feature = "CONFIG_MMU")]
unsafe fn patch_unmap(fixmap: i32) {
    clear_fixmap(fixmap);
}

#[cfg(feature = "CONFIG_MMU")]
unsafe fn __patch_insn_set(addr: *mut core::ffi::c_void, c: u8, len: usize) -> i32 {
    let across_pages = offset_in_page(addr) + len > PAGE_SIZE;
    let mut waddr = addr;
    if len + offset_in_page(addr) > 2 * PAGE_SIZE { return -EINVAL; }
    // lockdep_assert_held(&text_mutex);
    core::arch::asm!("", options(nomem, nostack, preserves_flags));
    if across_pages { patch_map(addr.add(PAGE_SIZE), FIX_TEXT_POKE1 as u32); }
    waddr = patch_map(addr, FIX_TEXT_POKE0 as u32);
    core::ptr::write_bytes(waddr as *mut u8, c, len);
    local_flush_icache_range(waddr as usize, waddr as usize + len);
    patch_unmap(FIX_TEXT_POKE0);
    if across_pages { patch_unmap(FIX_TEXT_POKE1); }
    0
}

#[cfg(not(feature = "CONFIG_MMU"))]
unsafe fn __patch_insn_set(addr: *mut core::ffi::c_void, c: u8, len: usize) -> i32 {
    core::ptr::write_bytes(addr as *mut u8, c, len);
    0
}

#[cfg(feature = "CONFIG_MMU")]
unsafe fn __patch_insn_write(addr: *mut core::ffi::c_void, insn: *const core::ffi::c_void, len: usize) -> i32 {
    let across_pages = offset_in_page(addr) + len > PAGE_SIZE;
    let mut waddr = addr;
    if len + offset_in_page(addr) > 2 * PAGE_SIZE { return -EINVAL; }
    if !riscv_patch_in_stop_machine { /* lockdep_assert_held(&text_mutex); */ }
    if across_pages { patch_map(addr.add(PAGE_SIZE), FIX_TEXT_POKE1 as u32); }
    waddr = patch_map(addr, FIX_TEXT_POKE0 as u32);
    let ret = copy_to_kernel_nofault(waddr, insn, len);
    local_flush_icache_range(waddr as usize, waddr as usize + len);
    patch_unmap(FIX_TEXT_POKE0);
    if across_pages { patch_unmap(FIX_TEXT_POKE1); }
    ret
}

#[cfg(not(feature = "CONFIG_MMU"))]
unsafe fn __patch_insn_write(addr: *mut core::ffi::c_void, insn: *const core::ffi::c_void, len: usize) -> i32 {
    copy_to_kernel_nofault(addr, insn, len)
}

unsafe fn patch_insn_set(mut addr: *mut core::ffi::c_void, c: u8, mut len: usize) -> i32 {
    while len != 0 {
        let size = core::cmp::min(len, PAGE_SIZE * 2 - offset_in_page(addr));
        let ret = __patch_insn_set(addr, c, size);
        if ret != 0 { return ret; }
        addr = addr.add(size);
        len -= size;
    }
    0
}

pub unsafe fn patch_text_set_nosync(addr: *mut core::ffi::c_void, c: u8, len: usize) -> i32 {
    let ret = patch_insn_set(addr, c, len);
    if ret == 0 { flush_icache_range(addr as usize, addr as usize + len); }
    ret
}

pub unsafe fn patch_insn_write(mut addr: *mut core::ffi::c_void, mut insn: *const core::ffi::c_void, mut len: usize) -> i32 {
    while len != 0 {
        let size = core::cmp::min(len, PAGE_SIZE * 2 - offset_in_page(addr));
        let ret = __patch_insn_write(addr, insn, size);
        if ret != 0 { return ret; }
        addr = addr.add(size);
        insn = insn.add(size);
        len -= size;
    }
    0
}

pub unsafe fn patch_text_nosync(addr: *mut core::ffi::c_void, insns: *const core::ffi::c_void, len: usize) -> i32 {
    let ret = patch_insn_write(addr, insns, len);
    if ret == 0 { flush_icache_range(addr as usize, addr as usize + len); }
    ret
}

unsafe extern "C" fn patch_text_cb(data: *mut core::ffi::c_void) -> i32 {
    let patch = &mut *(data as *mut patch_insn);
    let mut ret = 0;
    if patch.cpu_count.fetch_add(1, Ordering::Relaxed) + 1 == num_online_cpus {
        ret = patch_insn_write(patch.addr, patch.insns as *const _, patch.len);
        patch.cpu_count.fetch_add(1, Ordering::Release);
    } else {
        while patch.cpu_count.load(Ordering::Acquire) <= num_online_cpus { cpu_relax(); }
        local_flush_icache_all();
    }
    ret
}

pub unsafe fn patch_text(addr: *mut core::ffi::c_void, insns: *mut u32, len: usize) -> i32 {
    let mut patch = patch_insn { addr, insns, len, cpu_count: AtomicI32::new(0) };
    // lockdep_assert_held(&text_mutex);
    riscv_patch_in_stop_machine = true;
    let ret = stop_machine_cpuslocked(patch_text_cb, &mut patch as *mut _ as *mut _, &cpu_online_mask);
    riscv_patch_in_stop_machine = false;
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
