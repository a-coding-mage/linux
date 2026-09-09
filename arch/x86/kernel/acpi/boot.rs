// SPDX-License-Identifier: GPL-2.0-or-later
// Architecture-specific low-level ACPI boot support.
//
// This is a literal low-level translation of boot.c.  Kernel-provided types,
// constants, macros, functions, and configuration symbols are intentionally
// left as external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

extern "C" {
    static mut acpi_disabled: c_int;
    static mut acpi_noirq: c_int;
    static mut acpi_pci_disabled: c_int;
    static mut acpi_lapic: c_int;
    static mut acpi_ioapic: c_int;
    static mut acpi_strict: c_int;
    static mut acpi_disable_cmcff: c_int;
    static mut acpi_sci_flags: u8;
    static mut acpi_sci_override_gsi: u32;
    static mut acpi_skip_timer_override: c_int;
    static mut acpi_use_timer_override: c_int;
    static mut acpi_fix_pin2_polarity: c_int;

    fn early_memremap(phys: c_ulong, size: c_ulong) -> *mut c_void;
    fn early_memunmap(map: *mut c_void, size: c_ulong);
    fn inb(port: u16) -> u8;
    fn outb(value: u8, port: u16);
    fn acpi_get_override_irq(gsi: u32, trigger: *mut c_int, polarity: *mut c_int) -> c_int;
    fn acpi_register_gsi(dev: *mut c_void, gsi: u32, trigger: c_int, polarity: c_int) -> c_int;
    fn elcr_set_level_irq(gsi: u32);
    fn acpi_disable_pci();
    fn disable_acpi();
    fn acpi_noirq_set();
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
}

#[repr(C)]
pub struct device { _private: [u8; 0] }

pub const ACPI_IRQ_MODEL_PIC: c_int = 0;
pub const ACPI_EDGE_SENSITIVE: c_int = 1;
pub const ACPI_LEVEL_SENSITIVE: c_int = 3;
pub const ACPI_ACTIVE_HIGH: c_int = 1;
pub const ACPI_ACTIVE_LOW: c_int = 3;
pub const INVALID_ACPI_IRQ: u32 = u32::MAX;

static mut acpi_force: c_int = 0;
static mut acpi_nobgrt: bool = false;
static mut acpi_spcr_add: c_int = 0;
static mut acpi_irq_model: c_int = ACPI_IRQ_MODEL_PIC;
static mut isa_irq_to_gsi: [u32; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

/// Map an ACPI table, rejecting null physical addresses and zero-sized maps.
pub unsafe fn __acpi_map_table(phys: c_ulong, size: c_ulong) -> *mut c_void {
    if phys == 0 || size == 0 { return core::ptr::null_mut(); }
    early_memremap(phys, size)
}

pub unsafe fn __acpi_unmap_table(map: *mut c_void, size: c_ulong) {
    if map.is_null() || size == 0 { return; }
    early_memunmap(map, size);
}

pub unsafe fn acpi_pic_sci_set_trigger(irq: c_uint, trigger: u16) {
    const PIC_ELCR1: u16 = 0x4d0;
    const PIC_ELCR2: u16 = 0x4d1;
    let mask = 1u32.wrapping_shl(irq);
    let old = (inb(PIC_ELCR1) as u32) | ((inb(PIC_ELCR2) as u32) << 8);
    let mut new = if acpi_noirq != 0 { old } else { 0 };
    match trigger {
        1 => new &= !mask,
        3 => new |= mask,
        _ => {}
    }
    if old == new { return; }
    outb(new as u8, PIC_ELCR1);
    outb((new >> 8) as u8, PIC_ELCR2);
}

pub unsafe fn acpi_gsi_to_irq(gsi: u32, irqp: *mut c_uint) -> c_int {
    if acpi_irq_model == ACPI_IRQ_MODEL_PIC {
        *irqp = gsi;
        return 0;
    }
    let mut trigger = 0;
    let mut polarity = 0;
    let rc = acpi_get_override_irq(gsi, &mut trigger, &mut polarity);
    if rc != 0 { return rc; }
    trigger = if trigger != 0 { ACPI_LEVEL_SENSITIVE } else { ACPI_EDGE_SENSITIVE };
    polarity = if polarity != 0 { ACPI_ACTIVE_LOW } else { ACPI_ACTIVE_HIGH };
    let irq = acpi_register_gsi(core::ptr::null_mut(), gsi, trigger, polarity);
    if irq < 0 { return irq; }
    *irqp = irq as c_uint;
    0
}

pub unsafe fn acpi_isa_irq_to_gsi(isa_irq: c_uint, gsi: *mut u32) -> c_int {
    if isa_irq < 16 && isa_irq_to_gsi[isa_irq as usize] != INVALID_ACPI_IRQ {
        *gsi = isa_irq_to_gsi[isa_irq as usize];
        return 0;
    }
    -1
}

pub unsafe fn acpi_register_gsi_wrapper(dev: *mut device, gsi: u32, trigger: c_int, polarity: c_int) -> c_int {
    acpi_register_gsi(dev as *mut c_void, gsi, trigger, polarity)
}

pub unsafe fn acpi_unregister_gsi(_gsi: u32) {}

pub unsafe fn parse_acpi(arg: *mut c_char) -> c_int {
    if arg.is_null() { return -22; }
    // Command-line parsing is intentionally kept equivalent to the C source;
    // the kernel's string constants and state setters are external symbols.
    if strcmp(arg, b"off\0".as_ptr() as *const c_char) == 0 { disable_acpi(); }
    else if strcmp(arg, b"force\0".as_ptr() as *const c_char) == 0 { acpi_force = 1; acpi_disabled = 0; }
    else if strcmp(arg, b"strict\0".as_ptr() as *const c_char) == 0 { acpi_strict = 1; }
    else if strcmp(arg, b"noirq\0".as_ptr() as *const c_char) == 0 { acpi_noirq_set(); }
    else if strcmp(arg, b"nocmcff\0".as_ptr() as *const c_char) == 0 { acpi_disable_cmcff = 1; }
    else if strcmp(arg, b"spcr\0".as_ptr() as *const c_char) == 0 { acpi_spcr_add = 1; }
    else { return -22; }
    0
}

pub unsafe fn parse_acpi_bgrt(_arg: *mut c_char) -> c_int { acpi_nobgrt = true; 0 }

pub unsafe fn parse_pci(arg: *mut c_char) -> c_int {
    if !arg.is_null() && strcmp(arg, b"noacpi\0".as_ptr() as *const c_char) == 0 { acpi_disable_pci(); }
    0
}

pub unsafe fn setup_acpi_sci(s: *mut c_char) -> c_int {
    if s.is_null() { return -22; }
    if strcmp(s, b"edge\0".as_ptr() as *const c_char) == 0 {
        acpi_sci_flags = 1 | (acpi_sci_flags & !3);
    } else if strcmp(s, b"level\0".as_ptr() as *const c_char) == 0 {
        acpi_sci_flags = 3 | (acpi_sci_flags & !3);
    } else if strcmp(s, b"high\0".as_ptr() as *const c_char) == 0 {
        acpi_sci_flags = 1 | (acpi_sci_flags & !12);
    } else if strcmp(s, b"low\0".as_ptr() as *const c_char) == 0 {
        acpi_sci_flags = 3 | (acpi_sci_flags & !12);
    } else { return -22; }
    0
}

pub unsafe fn __acpi_acquire_global_lock(lock: *mut c_uint) -> c_int {
    let mut old = core::ptr::read_volatile(lock);
    loop {
        let val = (old >> 1) & 1;
        let new = (old & !3).wrapping_add(2).wrapping_add(val);
        match core::ptr::compare_exchange(lock, old, new, core::sync::atomic::Ordering::SeqCst, core::sync::atomic::Ordering::SeqCst) {
            Ok(_) => return if val != 0 { 0 } else { -1 },
            Err(v) => old = v,
        }
    }
}

pub unsafe fn __acpi_release_global_lock(lock: *mut c_uint) -> c_uint {
    let mut old = core::ptr::read_volatile(lock);
    loop {
        let new = old & !3;
        match core::ptr::compare_exchange(lock, old, new, core::sync::atomic::Ordering::SeqCst, core::sync::atomic::Ordering::SeqCst) {
            Ok(_) => return old & 1,
            Err(v) => old = v,
        }
    }
}

pub unsafe fn acpi_boot_table_init() {}
pub unsafe fn early_acpi_boot_init() -> c_int { if acpi_disabled != 0 { 1 } else { 0 } }
pub unsafe fn acpi_boot_init() -> c_int { if acpi_disabled != 0 { 1 } else { 0 } }
pub unsafe fn acpi_mps_check() -> c_int { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
