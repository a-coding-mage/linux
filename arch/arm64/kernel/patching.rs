// SPDX-License-Identifier: GPL-2.0-only
// Kernel and architecture dependencies are supplied by other translation units.

use core::ffi::c_void;

type PhysAddr = u64;
type SizeT = usize;
type U32 = u32;
type U64 = u64;

extern "C" {
    static mut patch_lock: c_void;
    static mut system_state: u32;
    static __exittext_begin: c_void;
    static __exittext_end: c_void;
}

extern "C" {
    fn core_kernel_text(addr: usize) -> bool;
    fn __pa_symbol(addr: *mut c_void) -> PhysAddr;
    fn vmalloc_to_page(addr: *mut c_void) -> *mut c_void;
    fn page_to_phys(page: *mut c_void) -> PhysAddr;
    fn offset_in_page(addr: *mut c_void) -> usize;
    fn set_fixmap_offset(fixmap: i32, phys: PhysAddr) -> *mut c_void;
    fn clear_fixmap(fixmap: i32);
    fn copy_from_kernel_nofault(dst: *mut c_void, src: *const c_void, len: usize) -> i32;
    fn copy_to_kernel_nofault(dst: *mut c_void, src: *const c_void, len: usize) -> i32;
    fn raw_spin_lock_irqsave(lock: *mut c_void, flags: *mut usize);
    fn raw_spin_unlock_irqrestore(lock: *mut c_void, flags: usize);
    fn flush_icache_range(start: usize, end: usize);
    fn caches_clean_inval_pou(start: usize, end: usize);
    fn memset32(dst: *mut c_void, value: u32, count: usize);
    fn atomic_inc_return(value: *mut AtomicT) -> i32;
    fn atomic_inc(value: *mut AtomicT);
    fn atomic_read(value: *const AtomicT) -> i32;
    fn num_online_cpus() -> i32;
    fn cpu_relax();
    fn isb();
    fn stop_machine_cpuslocked(func: unsafe extern "C" fn(*mut c_void) -> i32,
                               data: *mut c_void, mask: *mut c_void) -> i32;
    static mut cpu_online_mask: c_void;
}

#[repr(C)]
struct AtomicT { value: i32 }

const SYSTEM_RUNNING: u32 = 1;
const AARCH64_INSN_SIZE: usize = 4;
const PAGE_SIZE: usize = 4096;
const FIX_TEXT_POKE0: i32 = 0;

unsafe fn is_exit_text(addr: usize) -> bool {
    /* discarded with init text/data */
    system_state < SYSTEM_RUNNING &&
        addr >= (&__exittext_begin as *const _ as usize) &&
        addr < (&__exittext_end as *const _ as usize)
}

unsafe fn is_image_text(addr: usize) -> bool {
    core_kernel_text(addr) || is_exit_text(addr)
}

unsafe fn patch_map(addr: *mut c_void, fixmap: i32) -> *mut c_void {
    let phys: PhysAddr;
    if is_image_text(addr as usize) {
        phys = __pa_symbol(addr);
    } else {
        let page = vmalloc_to_page(addr);
        // BUG_ON(!page)
        if page.is_null() { core::hint::unreachable_unchecked(); }
        phys = page_to_phys(page) + offset_in_page(addr) as u64;
    }
    set_fixmap_offset(fixmap, phys)
}

unsafe fn patch_unmap(fixmap: i32) { clear_fixmap(fixmap); }

/* In ARMv8-A, A64 instructions have a fixed length of 32 bits and are always little-endian. */
pub unsafe extern "C" fn aarch64_insn_read(addr: *mut c_void, insnp: *mut u32) -> i32 {
    let mut val: u32 = 0;
    let ret = copy_from_kernel_nofault(&mut val as *mut _ as *mut c_void, addr, AARCH64_INSN_SIZE);
    if ret == 0 { *insnp = u32::from_le(val); }
    ret
}

unsafe fn __aarch64_insn_write(addr: *mut c_void, insn: u32) -> i32 {
    let mut flags = 0usize;
    raw_spin_lock_irqsave(&mut patch_lock, &mut flags);
    let waddr = patch_map(addr, FIX_TEXT_POKE0);
    let ret = copy_to_kernel_nofault(waddr, &insn as *const _ as *const c_void, AARCH64_INSN_SIZE);
    patch_unmap(FIX_TEXT_POKE0);
    raw_spin_unlock_irqrestore(&mut patch_lock, flags);
    ret
}

pub unsafe extern "C" fn aarch64_insn_write(addr: *mut c_void, insn: u32) -> i32 {
    __aarch64_insn_write(addr, insn.to_le())
}

pub unsafe extern "C" fn aarch64_insn_write_literal_u64(addr: *mut c_void, val: u64) -> i32 {
    let mut flags = 0usize;
    raw_spin_lock_irqsave(&mut patch_lock, &mut flags);
    let waddr = patch_map(addr, FIX_TEXT_POKE0);
    let ret = copy_to_kernel_nofault(waddr, &val as *const _ as *const c_void, core::mem::size_of::<u64>());
    patch_unmap(FIX_TEXT_POKE0);
    raw_spin_unlock_irqrestore(&mut patch_lock, flags);
    ret
}

type TextPokeF = unsafe fn(*mut c_void, *mut c_void, usize, usize);

unsafe fn __text_poke(func: TextPokeF, addr: *mut c_void, src: *mut c_void, len: usize) -> *mut c_void {
    let mut flags = 0usize;
    let mut patched = 0usize;
    raw_spin_lock_irqsave(&mut patch_lock, &mut flags);
    while patched < len {
        let ptr = addr.add(patched);
        let size = core::cmp::min(PAGE_SIZE - offset_in_page(ptr), len - patched);
        let waddr = patch_map(ptr, FIX_TEXT_POKE0);
        func(waddr, src, patched, size);
        patch_unmap(FIX_TEXT_POKE0);
        patched += size;
    }
    raw_spin_unlock_irqrestore(&mut patch_lock, flags);
    flush_icache_range(addr as usize, addr as usize + len);
    addr
}

unsafe fn text_poke_memcpy(dst: *mut c_void, src: *mut c_void, patched: usize, len: usize) {
    copy_to_kernel_nofault(dst, src.add(patched), len);
}

unsafe fn text_poke_memset(dst: *mut c_void, src: *mut c_void, _patched: usize, len: usize) {
    let c = *(src as *const u32);
    memset32(dst, c, len / 4);
}

/** aarch64_insn_copy - Copy instructions into (an unused part of) RX memory */
pub unsafe extern "C" fn aarch64_insn_copy(dst: *mut c_void, src: *mut c_void, len: usize) -> *mut c_void {
    if (dst as usize & 0x3) != 0 { return core::ptr::null_mut(); }
    __text_poke(text_poke_memcpy, dst, src, len)
}

/** aarch64_insn_set - memset for RX memory regions. */
pub unsafe extern "C" fn aarch64_insn_set(dst: *mut c_void, insn: u32, len: usize) -> *mut c_void {
    if (dst as usize & 0x3) != 0 { return core::ptr::null_mut(); }
    __text_poke(text_poke_memset, dst, &insn as *const _ as *mut c_void, len)
}

pub unsafe extern "C" fn aarch64_insn_patch_text_nosync(addr: *mut c_void, insn: u32) -> i32 {
    if (addr as usize & 0x3) != 0 { return -22; }
    let ret = aarch64_insn_write(addr, insn);
    if ret == 0 { caches_clean_inval_pou(addr as usize, addr as usize + AARCH64_INSN_SIZE); }
    ret
}

#[repr(C)]
struct Aarch64InsnPatch {
    text_addrs: *mut *mut c_void,
    new_insns: *mut u32,
    insn_cnt: i32,
    cpu_count: AtomicT,
}

unsafe extern "C" fn aarch64_insn_patch_text_cb(arg: *mut c_void) -> i32 {
    let pp = &mut *(arg as *mut Aarch64InsnPatch);
    if atomic_inc_return(&mut pp.cpu_count) == num_online_cpus() {
        let mut ret = 0;
        for i in 0..pp.insn_cnt {
            if ret != 0 { break; }
            ret = aarch64_insn_patch_text_nosync(*pp.text_addrs.add(i as usize), *pp.new_insns.add(i as usize));
        }
        atomic_inc(&mut pp.cpu_count);
        ret
    } else {
        while atomic_read(&pp.cpu_count) <= num_online_cpus() { cpu_relax(); }
        isb();
        0
    }
}

pub unsafe extern "C" fn aarch64_insn_patch_text(addrs: *mut *mut c_void, insns: *mut u32, cnt: i32) -> i32 {
    if cnt <= 0 { return -22; }
    let mut patch = Aarch64InsnPatch { text_addrs: addrs, new_insns: insns, insn_cnt: cnt, cpu_count: AtomicT { value: 0 } };
    stop_machine_cpuslocked(aarch64_insn_patch_text_cb, &mut patch as *mut _ as *mut c_void, &mut cpu_online_mask)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
