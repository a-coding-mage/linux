/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of the C IO-APIC header. */

/* Intel IO-APIC support for SMP and UP systems. */

#[repr(C)]
pub union IO_APIC_reg_00 {
    pub raw: u32,
    pub bits: IO_APIC_reg_00_bits,
}
#[repr(C)]
pub struct IO_APIC_reg_00_bits { pub raw: u32 }

#[repr(C)]
pub union IO_APIC_reg_01 {
    pub raw: u32,
    pub bits: IO_APIC_reg_01_bits,
}
#[repr(C)]
pub struct IO_APIC_reg_01_bits { pub raw: u32 }

#[repr(C)]
pub union IO_APIC_reg_02 {
    pub raw: u32,
    pub bits: IO_APIC_reg_02_bits,
}
#[repr(C)]
pub struct IO_APIC_reg_02_bits { pub raw: u32 }

#[repr(C)]
pub union IO_APIC_reg_03 {
    pub raw: u32,
    pub bits: IO_APIC_reg_03_bits,
}
#[repr(C)]
pub struct IO_APIC_reg_03_bits { pub raw: u32 }

#[repr(C, packed)]
pub struct IO_APIC_route_entry {
    pub raw: IO_APIC_route_entry_raw,
}
#[repr(C)]
pub union IO_APIC_route_entry_raw {
    pub words: IO_APIC_route_words,
    pub ir: IO_APIC_route_ir,
    pub w: IO_APIC_route_w,
}
#[repr(C)]
pub struct IO_APIC_route_words { pub w1: u32, pub w2: u32 }
#[repr(C)]
pub struct IO_APIC_route_ir { pub raw: u64 }
#[repr(C)]
pub struct IO_APIC_route_ir_shared { pub raw: u64 }

pub struct irq_alloc_info;
pub struct ioapic_domain_cfg;

pub const IOAPIC_MAP_ALLOC: u32 = 0x1;
pub const IOAPIC_MAP_CHECK: u32 = 0x2;

#[cfg(feature = "CONFIG_X86_IO_APIC")]
extern "C" {
    pub static mut nr_ioapics: i32;
    pub fn mpc_ioapic_id(ioapic: i32) -> i32;
    pub fn mpc_ioapic_addr(ioapic: i32) -> u32;
    pub static mut mp_irq_entries: i32;
    pub static mut mp_irqs: [mpc_intsrc; MAX_IRQ_SOURCES];
    pub static mut ioapic_is_disabled: bool;
    pub static mut noioapicquirk: i32;
    pub static mut noioapicreroute: i32;
    pub static mut gsi_top: u32;
    pub static mut io_apic_irqs: usize;
    pub fn ioapic_insert_resources();
    pub fn arch_early_ioapic_init() -> i32;
    pub fn save_ioapic_entries() -> i32;
    pub fn mask_ioapic_entries();
    pub fn restore_ioapic_entries() -> i32;
    pub fn setup_ioapic_ids_from_mpc();
    pub fn mp_find_ioapic(gsi: u32) -> i32;
    pub fn mp_find_ioapic_pin(ioapic: i32, gsi: u32) -> i32;
    pub fn mp_map_gsi_to_irq(gsi: u32, flags: u32, info: *mut irq_alloc_info) -> i32;
    pub fn mp_unmap_irq(irq: i32);
    pub fn mp_register_ioapic(id: i32, address: u32, gsi_base: u32, cfg: *mut ioapic_domain_cfg) -> i32;
    pub fn mp_unregister_ioapic(gsi_base: u32) -> i32;
    pub fn mp_ioapic_registered(gsi_base: u32) -> i32;
    pub fn ioapic_set_alloc_attr(info: *mut irq_alloc_info, node: i32, trigger: i32, polarity: i32);
    pub fn mp_save_irq(m: *mut mpc_intsrc);
    pub fn disable_ioapic_support();
    pub fn io_apic_init_mappings();
    pub fn native_io_apic_read(apic: u32, reg: u32) -> u32;
    pub fn native_restore_boot_irq_mode();
    pub fn setup_IO_APIC();
    pub fn enable_IO_APIC();
    pub fn clear_IO_APIC();
    pub fn restore_boot_irq_mode();
    pub fn IO_APIC_get_PCI_irq_vector(bus: i32, devfn: i32, pin: i32) -> i32;
    pub fn print_IO_APICs();
}

/* mpc_intsrc, MAX_IRQ_SOURCES, NR_IRQS_LEGACY, and ENOMEM are supplied by
 * the architecture and kernel headers. */

#[cfg(not(feature = "CONFIG_X86_IO_APIC"))]
pub const nr_ioapics: i32 = 0;

#[cfg(not(feature = "CONFIG_X86_IO_APIC"))]
pub const IO_APIC_IRQ: i32 = 0;
#[cfg(not(feature = "CONFIG_X86_IO_APIC"))]
pub const io_apic_assign_pci_irqs: i32 = 0;
#[cfg(not(feature = "CONFIG_X86_IO_APIC"))]
pub const gsi_top: u32 = NR_IRQS_LEGACY;

#[cfg(not(feature = "CONFIG_X86_IO_APIC"))]
pub unsafe fn ioapic_insert_resources() {}
#[cfg(not(feature = "CONFIG_X86_IO_APIC"))]
pub unsafe fn arch_early_ioapic_init() -> i32 { 0 }
#[cfg(not(feature = "CONFIG_X86_IO_APIC"))]
pub unsafe fn print_IO_APICs() {}
#[cfg(not(feature = "CONFIG_X86_IO_APIC"))]
pub unsafe fn mp_find_ioapic(_gsi: u32) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_X86_IO_APIC"))]
pub unsafe fn mp_map_gsi_to_irq(gsi: u32, _flags: u32, _info: *mut irq_alloc_info) -> i32 { gsi as i32 }
#[cfg(not(feature = "CONFIG_X86_IO_APIC"))]
pub unsafe fn mp_unmap_irq(_irq: i32) {}
#[cfg(not(feature = "CONFIG_X86_IO_APIC"))]
pub unsafe fn save_ioapic_entries() -> i32 { -ENOMEM }
#[cfg(not(feature = "CONFIG_X86_IO_APIC"))]
pub unsafe fn mask_ioapic_entries() {}
#[cfg(not(feature = "CONFIG_X86_IO_APIC"))]
pub unsafe fn restore_ioapic_entries() -> i32 { -ENOMEM }
#[cfg(not(feature = "CONFIG_X86_IO_APIC"))]
pub unsafe fn mp_save_irq(_m: *mut mpc_intsrc) {}
#[cfg(not(feature = "CONFIG_X86_IO_APIC"))]
pub unsafe fn disable_ioapic_support() {}
#[cfg(not(feature = "CONFIG_X86_IO_APIC"))]
pub unsafe fn io_apic_init_mappings() {}
#[cfg(not(feature = "CONFIG_X86_IO_APIC"))]
pub unsafe fn setup_IO_APIC() {}
#[cfg(not(feature = "CONFIG_X86_IO_APIC"))]
pub unsafe fn enable_IO_APIC() {}
#[cfg(not(feature = "CONFIG_X86_IO_APIC"))]
pub unsafe fn restore_boot_irq_mode() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
