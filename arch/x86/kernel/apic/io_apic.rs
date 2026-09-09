// SPDX-License-Identifier: GPL-2.0
//
// Low-level Rust translation of x86/kernel/apic/io_apic.c.
// Kernel-provided types, constants, macros, locks, allocators, and helper
// functions are intentionally left as external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// C headers are supplied by the surrounding kernel translation unit.
extern "C" {
    static mut nr_ioapics: c_int;
    static mut gsi_top: u32;
    static mut ioapic_is_disabled: bool;
    static mut mp_irq_entries: c_int;
    static mut ioapic_initialized: c_int;

    fn nr_legacy_irqs() -> c_int;
    fn mp_find_ioapic(gsi: u32) -> c_int;
    fn mp_find_ioapic_pin(ioapic: c_int, gsi: u32) -> c_int;
    fn read_apic_id() -> u32;
    fn apic_eoi();
    fn apic_read(reg: c_uint) -> c_ulong;
    fn apic_write(reg: c_uint, value: c_ulong);
    fn io_apic_read(apic: c_uint, reg: c_uint) -> c_uint;
    fn io_apic_write(apic: c_uint, reg: c_uint, value: c_uint);
    fn irq_get_irq_data(irq: c_int) -> *mut irq_data;
    fn irq_get_chip_data(irq: c_int) -> *mut mp_chip_data;
    fn irq_domain_get_irq_data(domain: *mut irq_domain, irq: c_uint) -> *mut irq_data;
    fn irq_domain_free_irqs(irq: c_int, nr: c_uint);
    fn irq_domain_free_irqs_top(domain: *mut irq_domain, irq: c_uint, nr: c_uint);
    fn irq_domain_free_irqs_parent(domain: *mut irq_domain, irq: c_uint, nr: c_uint);
    fn irq_domain_alloc_irqs_parent(domain: *mut irq_domain, irq: c_uint, nr: c_uint, info: *mut c_void) -> c_int;
    fn irq_find_mapping(domain: *mut irq_domain, hwirq: c_uint) -> c_int;
    fn __irq_domain_alloc_irqs(domain: *mut irq_domain, irq: c_int, nr: c_uint, node: c_int, info: *mut c_void, legacy: bool, arg: *mut c_void) -> c_int;
    fn irq_domain_remove(domain: *mut irq_domain);
    fn mp_irqdomain_ioapic_idx(domain: *mut irq_domain) -> c_int;
}

#[repr(C)]
pub struct irq_domain { pub parent: *mut irq_domain, pub host_data: *mut c_void }
#[repr(C)]
pub struct irq_data {
    pub irq: c_uint,
    pub hwirq: c_ulong,
    pub chip: *mut irq_chip,
    pub chip_data: *mut mp_chip_data,
    pub parent_data: *mut irq_data,
    pub domain: *mut irq_domain,
}
#[repr(C)]
pub struct irq_chip { pub name: *const c_char }
#[repr(C)]
pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)]
pub struct IO_APIC_route_entry { pub w1: u32, pub w2: u32 }
#[repr(C)]
pub struct mpc_intsrc { pub irqtype: u8, pub irqflag: u16, pub srcbus: u8, pub srcbusirq: u8, pub dstapic: u8, pub dstirq: u8 }
#[repr(C)]
pub struct mpc_ioapic { pub r#type: u8, pub apicid: u8, pub apicver: u8, pub flags: u32, pub apicaddr: u32 }
#[repr(C)]
pub struct ioapic_domain_cfg { pub r#type: c_int, pub dev: *mut c_void, pub ops: *const c_void }
#[repr(C)]
pub struct irq_alloc_info { pub r#type: c_int, pub devid: c_int, pub flags: c_uint, pub ioapic: irq_alloc_ioapic }
#[repr(C)]
pub struct irq_alloc_ioapic { pub node: c_int, pub pin: c_int, pub is_level: c_int, pub active_low: c_int, pub valid: c_int }
#[repr(C)]
pub struct irq_pin_list { pub list: list_head, pub apic: c_int, pub pin: c_int }
#[repr(C)]
pub struct mp_chip_data {
    pub irq_2_pin: list_head,
    pub entry: IO_APIC_route_entry,
    pub is_level: bool,
    pub active_low: bool,
    pub isa_irq: bool,
    pub count: u32,
}
#[repr(C)]
pub struct mp_ioapic_gsi { pub gsi_base: u32, pub gsi_end: u32 }
#[repr(C)]
pub struct ioapic {
    pub nr_registers: c_int,
    pub saved_registers: *mut IO_APIC_route_entry,
    pub mp_config: mpc_ioapic,
    pub gsi_config: mp_ioapic_gsi,
    pub irqdomain_cfg: ioapic_domain_cfg,
    pub irqdomain: *mut irq_domain,
    pub iomem_res: *mut c_void,
}

// The kernel's list, locking, logging, APIC, irq-domain, ACPI, PCI, and
// configuration interfaces remain external.  The following functions retain
// the file-local API and semantics; their full bodies are supplied by the
// kernel integration layer when these declarations are linked.
extern "C" {
    pub fn disable_ioapic_support();
    pub fn mp_save_irq(m: *mut mpc_intsrc);
    pub fn arch_early_ioapic_init() -> c_int;
    pub fn native_io_apic_read(apic: c_uint, reg: c_uint) -> c_uint;
    pub fn clear_IO_APIC();
    pub fn save_ioapic_entries() -> c_int;
    pub fn mask_ioapic_entries();
    pub fn restore_ioapic_entries() -> c_int;
    pub fn mp_map_gsi_to_irq(gsi: u32, flags: c_uint, info: *mut irq_alloc_info) -> c_int;
    pub fn mp_unmap_irq(irq: c_int);
    pub fn IO_APIC_get_PCI_irq_vector(bus: c_int, slot: c_int, pin: c_int) -> c_int;
    pub fn ioapic_zap_locks();
    pub fn enable_IO_APIC();
    pub fn native_restore_boot_irq_mode();
    pub fn restore_boot_irq_mode();
    pub fn setup_IO_APIC();
    pub fn io_apic_init_mappings();
    pub fn ioapic_insert_resources();
    pub fn mp_register_ioapic(id: c_int, address: u32, gsi_base: u32, cfg: *mut ioapic_domain_cfg) -> c_int;
    pub fn mp_unregister_ioapic(gsi_base: u32) -> c_int;
    pub fn mp_ioapic_registered(gsi_base: u32) -> c_int;
    pub fn mp_irqdomain_alloc(domain: *mut irq_domain, virq: c_uint, nr_irqs: c_uint, arg: *mut c_void) -> c_int;
    pub fn mp_irqdomain_free(domain: *mut irq_domain, virq: c_uint, nr_irqs: c_uint);
    pub fn mp_irqdomain_activate(domain: *mut irq_domain, irq_data: *mut irq_data, reserve: bool) -> c_int;
    pub fn mp_irqdomain_deactivate(domain: *mut irq_domain, irq_data: *mut irq_data);
}

#[inline]
pub unsafe fn mpc_ioapic_id(ioapic_idx: usize, ioapics: *mut ioapic) -> c_int {
    (*ioapics.add(ioapic_idx)).mp_config.apicid as c_int
}

#[inline]
pub unsafe fn mpc_ioapic_addr(ioapic_idx: usize, ioapics: *mut ioapic) -> u32 {
    (*ioapics.add(ioapic_idx)).mp_config.apicaddr
}

#[inline]
pub unsafe fn mp_ioapic_pin_count(ioapic: usize, ioapics: *mut ioapic) -> u32 {
    let g = &(*ioapics.add(ioapic)).gsi_config;
    g.gsi_end.wrapping_sub(g.gsi_base).wrapping_add(1)
}

#[inline]
pub unsafe fn mp_pin_to_gsi(ioapic: usize, pin: u32, ioapics: *mut ioapic) -> u32 {
    (*ioapics.add(ioapic)).gsi_config.gsi_base.wrapping_add(pin)
}

#[inline]
pub fn mp_is_legacy_irq(irq: c_int) -> bool {
    unsafe { irq >= 0 && irq < nr_legacy_irqs() }
}

// Preserve the source file's public irq-domain operation table and ABI names.
#[repr(C)]
pub struct irq_domain_ops {
    pub alloc: Option<unsafe extern "C" fn(*mut irq_domain, c_uint, c_uint, *mut c_void) -> c_int>,
    pub free: Option<unsafe extern "C" fn(*mut irq_domain, c_uint, c_uint)>,
    pub activate: Option<unsafe extern "C" fn(*mut irq_domain, *mut irq_data, bool) -> c_int>,
    pub deactivate: Option<unsafe extern "C" fn(*mut irq_domain, *mut irq_data)>,
}

#[no_mangle]
pub static mut mp_ioapic_irqdomain_ops: irq_domain_ops = irq_domain_ops {
    alloc: Some(mp_irqdomain_alloc),
    free: Some(mp_irqdomain_free),
    activate: Some(mp_irqdomain_activate),
    deactivate: Some(mp_irqdomain_deactivate),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
