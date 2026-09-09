// SPDX-License-Identifier: GPL-2.0
// C dependencies: linux/kernel.h, linux/spinlock.h, linux/kprobes.h,
// linux/mm.h, linux/stop_machine.h, asm/cacheflush.h, asm/fixmap.h,
// asm/smp_plat.h, asm/opcodes.h, and asm/text-patching.h.

#[repr(C)]
pub struct patch {
    pub addr: *mut core::ffi::c_void,
    pub insn: u32,
}

#[cfg(feature = "CONFIG_MMU")]
extern "C" {
    static mut patch_lock: core::ffi::c_void;
    fn core_kernel_text(addr: u32) -> bool;
    fn vmalloc_to_page(addr: *mut core::ffi::c_void) -> *mut page;
    fn virt_to_page(addr: *mut core::ffi::c_void) -> *mut page;
    fn page_to_phys(page: *mut page) -> usize;
    fn set_fixmap(fixmap: i32, phys: usize);
    fn __fix_to_virt(fixmap: i32) -> usize;
    fn clear_fixmap(fixmap: i32);
    fn raw_spin_lock_irqsave(lock: *mut core::ffi::c_void, flags: *mut usize);
    fn raw_spin_unlock_irqrestore(lock: *mut core::ffi::c_void, flags: usize);
}

#[cfg(feature = "CONFIG_MMU")]
#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_MMU")]
unsafe fn patch_map(
    addr: *mut core::ffi::c_void,
    fixmap: i32,
    flags: *mut usize,
) -> *mut core::ffi::c_void {
    let uintaddr = addr as usize as u32;
    let module = !core_kernel_text(uintaddr);
    let page: *mut page;

    if module && cfg!(feature = "CONFIG_STRICT_MODULE_RWX") {
        page = vmalloc_to_page(addr);
    } else if !module && cfg!(feature = "CONFIG_STRICT_KERNEL_RWX") {
        page = virt_to_page(addr);
    } else {
        return addr;
    }

    if !flags.is_null() {
        raw_spin_lock_irqsave(&mut patch_lock, flags);
    }

    set_fixmap(fixmap, page_to_phys(page));
    (__fix_to_virt(fixmap) + (uintaddr as usize & !((4096usize) - 1))) as *mut u8
        .wrapping_add(uintaddr as usize & ((4096usize) - 1)) as *mut core::ffi::c_void
}

#[cfg(feature = "CONFIG_MMU")]
unsafe fn patch_unmap(fixmap: i32, flags: *mut usize) {
    clear_fixmap(fixmap);
    if !flags.is_null() {
        raw_spin_unlock_irqrestore(&mut patch_lock, *flags);
    }
}

#[cfg(not(feature = "CONFIG_MMU"))]
unsafe fn patch_map(
    addr: *mut core::ffi::c_void,
    _fixmap: i32,
    _flags: *mut usize,
) -> *mut core::ffi::c_void {
    addr
}

#[cfg(not(feature = "CONFIG_MMU"))]
unsafe fn patch_unmap(_fixmap: i32, _flags: *mut usize) {}

extern "C" {
    fn __opcode_is_thumb16(insn: u32) -> bool;
    fn __opcode_to_mem_thumb16(insn: u32) -> u16;
    fn __opcode_thumb32_first(insn: u32) -> u16;
    fn __opcode_thumb32_second(insn: u32) -> u16;
    fn __opcode_to_mem_thumb32(insn: u32) -> u32;
    fn __opcode_to_mem_arm(insn: u32) -> u32;
    fn flush_kernel_vmap_range(addr: *mut core::ffi::c_void, size: usize);
    fn flush_icache_range(start: usize, end: usize);
    fn stop_machine_cpuslocked(
        callback: unsafe extern "C" fn(*mut core::ffi::c_void) -> i32,
        data: *mut core::ffi::c_void,
        arg: *mut core::ffi::c_void,
    );
    fn __patch_text(addr: *mut core::ffi::c_void, insn: u32);
}

const FIX_TEXT_POKE0: i32 = 0;
const FIX_TEXT_POKE1: i32 = 1;
const PAGE_SIZE: usize = 4096;

#[no_mangle]
pub unsafe extern "C" fn __patch_text_real(
    addr: *mut core::ffi::c_void,
    mut insn: u32,
    remap: bool,
) {
    let thumb2 = cfg!(feature = "CONFIG_THUMB2_KERNEL");
    let uintaddr = addr as usize as u32;
    let mut twopage = false;
    let mut flags: usize = 0;
    let mut waddr = addr;
    let size: usize;

    if remap {
        waddr = patch_map(addr, FIX_TEXT_POKE0, &mut flags);
    }

    if thumb2 && __opcode_is_thumb16(insn) {
        *(waddr as *mut u16) = __opcode_to_mem_thumb16(insn);
        size = core::mem::size_of::<u16>();
    } else if thumb2 && (uintaddr & 2) != 0 {
        let first = __opcode_thumb32_first(insn);
        let second = __opcode_thumb32_second(insn);
        let addrh0 = waddr as *mut u16;
        let mut addrh1 = waddr.add(2) as *mut u16;

        twopage = (uintaddr as usize & !(PAGE_SIZE - 1)) == PAGE_SIZE - 2;
        if twopage && remap {
            addrh1 = patch_map(addr.add(2), FIX_TEXT_POKE1, core::ptr::null_mut()) as *mut u16;
        }
        *addrh0 = __opcode_to_mem_thumb16(first);
        *addrh1 = __opcode_to_mem_thumb16(second);
        if twopage && addrh1 != addr.add(2) as *mut u16 {
            flush_kernel_vmap_range(addrh1 as *mut core::ffi::c_void, 2);
            patch_unmap(FIX_TEXT_POKE1, core::ptr::null_mut());
        }
        size = core::mem::size_of::<u32>();
    } else {
        insn = if thumb2 { __opcode_to_mem_thumb32(insn) } else { __opcode_to_mem_arm(insn) };
        *(waddr as *mut u32) = insn;
        size = core::mem::size_of::<u32>();
    }

    if waddr != addr {
        flush_kernel_vmap_range(waddr, if twopage { size / 2 } else { size });
        patch_unmap(FIX_TEXT_POKE0, &mut flags);
    }
    flush_icache_range(addr as usize, addr as usize + size);
}

unsafe extern "C" fn patch_text_stop_machine(data: *mut core::ffi::c_void) -> i32 {
    let patch = &*(data as *const patch);
    __patch_text(patch.addr, patch.insn);
    0
}

#[no_mangle]
pub unsafe extern "C" fn patch_text(addr: *mut core::ffi::c_void, insn: u32) {
    let mut patch = patch { addr, insn };
    stop_machine_cpuslocked(patch_text_stop_machine, &mut patch as *mut _ as *mut _, core::ptr::null_mut());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
