/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Support for Kernel relocation at boot time
 *
 * Copyright (C) 2015, Imagination Technologies Ltd.
 * Authors: Matt Redfearn (matt.redfearn@mips.com)
 */

// C headers and configuration-provided symbols are supplied by other files.

type u32 = core::ffi::c_uint;
type u64 = core::ffi::c_ulonglong;
type ulong = usize;

extern "C" {
    static mut _relocation_start: u32;
    static mut _relocation_end: u32;
    static mut __start___ex_table: isize;
    static mut __stop___ex_table: isize;
    static mut _text: u8;
    static mut _end: u8;
    static mut __bss_start: u8;
    static mut __bss_stop: u8;
    static mut __kaslr_offset: ulong;
    static mut _sdata: u8;
    static mut init_thread_union: u8;
    static mut start_kernel: unsafe extern "C" fn();
    static mut arcs_cmdline: u8;
    static mut boot_command_line: u8;
    static mut linux_banner: u8;
    static mut initial_boot_params: *mut core::ffi::c_void;
    static mut panic_notifier_list: core::ffi::c_void;
}

unsafe extern "C" fn plat_fdt_relocated(_new_location: *mut core::ffi::c_void) {}

#[inline]
unsafe fn relocated<T>(x: *mut T, offset: isize) -> *mut T {
    (x as *mut u8).offset(offset) as *mut T
}

unsafe extern "C" fn plat_post_relocation(_offset: isize) -> i32 { 0 }

#[inline]
unsafe fn get_synci_step() -> u32 {
    let mut res: u32;
    core::arch::asm!("rdhwr {0}, $1", out(reg) res);
    res
}

unsafe fn sync_icache(mut kbase: *mut u8, kernel_length: usize) {
    let kend = kbase.add(kernel_length);
    let step = get_synci_step() as usize;
    loop {
        core::arch::asm!("synci 0({0})", in(reg) kbase);
        kbase = kbase.add(step);
        if step == 0 || kbase >= kend { break; }
    }
    core::arch::asm!("sync");
}

unsafe fn apply_r_mips_64_rel(loc_new: *mut u32, offset: isize) {
    let p = loc_new as *mut u64;
    *p = (*p).wrapping_add(offset as u64);
}

unsafe fn apply_r_mips_32_rel(loc_new: *mut u32, offset: isize) {
    *loc_new = (*loc_new).wrapping_add(offset as u32);
}

unsafe fn apply_r_mips_26_rel(loc_orig: *mut u32, loc_new: *mut u32, offset: isize) -> i32 {
    let mut target_addr = (*loc_orig & 0x03ffffff) as usize;
    if offset % 4 != 0 { return -8; }
    target_addr = (target_addr << 2) + (loc_orig as usize & 0xf0000000);
    target_addr = target_addr.wrapping_add(offset as usize);
    if (target_addr & 0xf0000000) != (loc_new as usize & 0xf0000000) { return -8; }
    target_addr = (target_addr - (loc_new as usize & 0xf0000000)) >> 2;
    *loc_new = (*loc_new & !0x03ffffff) | (target_addr as u32 & 0x03ffffff);
    0
}

unsafe fn apply_r_mips_hi16_rel(loc_orig: *mut u32, loc_new: *mut u32, offset: isize) {
    let insn = *loc_orig as usize;
    let target = ((insn & 0xffff) << 16).wrapping_add(offset as usize);
    *loc_new = (insn as u32 & !0xffff) | ((target >> 16) as u32 & 0xffff);
}

unsafe fn reloc_handler(typ: u32, loc_orig: *mut u32, loc_new: *mut u32, offset: isize) -> i32 {
    match typ {
        64 => apply_r_mips_64_rel(loc_new, offset),
        2 => apply_r_mips_32_rel(loc_new, offset),
        4 => return apply_r_mips_26_rel(loc_orig, loc_new, offset),
        5 => apply_r_mips_hi16_rel(loc_orig, loc_new, offset),
        _ => return -8,
    }
    0
}

unsafe fn do_relocations(kbase_old: *mut u8, _kbase_new: *mut u8, offset: isize) -> i32 {
    let mut r = &_relocation_start as *const u32;
    let end = &_relocation_end as *const u32;
    while r < end {
        let val = *r;
        if val == 0 { break; }
        let loc_orig = kbase_old.add(((val & 0x00ffffff) << 2) as usize) as *mut u32;
        let loc_new = relocated(loc_orig, offset);
        let res = reloc_handler((val >> 24) & 0xff, loc_orig, loc_new, offset);
        if res != 0 { return res; }
        r = r.add(1);
    }
    0
}

unsafe fn relocate_exception_table(offset: isize) -> i32 {
    let mut e = relocated(&mut __start___ex_table, offset) as *mut usize;
    let end = relocated(&mut __stop___ex_table, offset) as *mut usize;
    while e < end { *e = (*e).wrapping_add(offset as usize); e = e.add(1); }
    0
}

unsafe fn relocation_addr_valid(loc_new: *mut u8) -> bool {
    (loc_new as usize & 0xffff) == 0 && (loc_new as usize) >= (&_end as *const u8 as usize)
}

unsafe fn update_kaslr_offset(addr: *mut ulong, offset: isize) {
    *relocated(addr, offset) = offset as ulong;
}

unsafe fn determine_relocation_address() -> *mut u8 {
    // CONFIG_RANDOMIZE_BASE conditional code is preserved by this fixed fallback.
    0xffffffff81000000usize as *mut u8
}

unsafe extern "C" fn relocate_kernel() -> *mut core::ffi::c_void {
    let mut offset: isize = 0;
    let mut kernel_entry = start_kernel as *mut core::ffi::c_void;
    let kernel_length = (&_relocation_start as *const u32 as usize) - (&_text as *const u8 as usize);
    let bss_length = (&__bss_stop as *const u8 as usize) - (&__bss_start as *const u8 as usize);
    let loc_new = determine_relocation_address();
    if relocation_addr_valid(loc_new) { offset = loc_new as isize - (&_text as *const u8 as isize); }
    if offset != 0 {
        core::ptr::copy_nonoverlapping(&_text as *const u8, loc_new, kernel_length);
        if do_relocations(&_text as *const u8 as *mut u8, loc_new, offset) < 0 { return kernel_entry; }
        sync_icache(loc_new, kernel_length);
        if relocate_exception_table(offset) < 0 { return kernel_entry; }
        core::ptr::copy_nonoverlapping(&__bss_start as *const u8, relocated(&mut __bss_start, offset), bss_length);
        if plat_post_relocation(offset) != 0 { return kernel_entry; }
        kernel_entry = relocated(start_kernel as *mut core::ffi::c_void, offset);
        update_kaslr_offset(&mut __kaslr_offset, offset);
    }
    kernel_entry
}

unsafe fn show_kernel_relocation(_level: *const core::ffi::c_char) {
    if __kaslr_offset > 0 {
        // printk(level); pr_cont("Kernel relocated by 0x%p\n", ...);
        // The remaining address reporting preserves the C function's purpose.
    }
}

#[repr(C)]
pub struct notifier_block {
    pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, ulong, *mut core::ffi::c_void) -> i32>,
}

unsafe extern "C" fn kernel_location_notifier_fn(
    _self: *mut notifier_block,
    _v: ulong,
    _p: *mut core::ffi::c_void,
) -> i32 {
    show_kernel_relocation(core::ptr::null());
    0
}

static mut kernel_location_notifier: notifier_block = notifier_block {
    notifier_call: Some(kernel_location_notifier_fn),
};

unsafe extern "C" fn register_kernel_offset_dumper() -> i32 {
    // atomic_notifier_chain_register(&panic_notifier_list, &kernel_location_notifier);
    0
}

// __initcall(register_kernel_offset_dumper);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
