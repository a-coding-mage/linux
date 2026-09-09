/* SPDX-License-Identifier: GPL-2.0 */
/* Low-Level PCI Access for i386 machines. */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

/* Kernel includes and DEBUG/DBG preprocessor definitions are represented by
 * the declarations and conditional interfaces below. */

pub const PCI_PROBE_BIOS: u32 = 0x0001;
pub const PCI_PROBE_CONF1: u32 = 0x0002;
pub const PCI_PROBE_CONF2: u32 = 0x0004;
pub const PCI_PROBE_MMCONF: u32 = 0x0008;
pub const PCI_PROBE_MASK: u32 = 0x000f;
pub const PCI_PROBE_NOEARLY: u32 = 0x0010;
pub const PCI_NO_CHECKS: u32 = 0x0400;
pub const PCI_USE_PIRQ_MASK: u32 = 0x0800;
pub const PCI_ASSIGN_ROMS: u32 = 0x1000;
pub const PCI_BIOS_IRQ_SCAN: u32 = 0x2000;
pub const PCI_ASSIGN_ALL_BUSSES: u32 = 0x4000;
pub const PCI_CAN_SKIP_ISA_ALIGN: u32 = 0x8000;
pub const PCI_USE__CRS: u32 = 0x10000;
pub const PCI_CHECK_ENABLE_AMD_MMCONF: u32 = 0x20000;
pub const PCI_HAS_IO_ECS: u32 = 0x40000;
pub const PCI_NOASSIGN_ROMS: u32 = 0x80000;
pub const PCI_ROOT_NO_CRS: u32 = 0x100000;
pub const PCI_NOASSIGN_BARS: u32 = 0x200000;
pub const PCI_BIG_ROOT_WINDOW: u32 = 0x400000;
pub const PCI_USE_E820: u32 = 0x800000;
pub const PCI_NO_E820: u32 = 0x1000000;

extern "C" {
    pub static mut pci_probe: u32;
    pub static mut pirq_table_addr: c_ulong;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum pci_bf_sort_state {
    pci_bf_sort_default,
    pci_force_nobf,
    pci_force_bf,
    pci_dmi_bf,
}

extern "C" {
    pub fn pcibios_resource_survey();
    pub fn pcibios_set_cache_line_size();
    pub static mut pcibios_last_bus: c_int;
    pub static mut pci_root_ops: pci_ops;
    pub fn pcibios_scan_specific_bus(busn: c_int);
    pub fn pcibios_irq_init();
    pub fn pcibios_init() -> c_int;
    pub fn pci_legacy_init() -> c_int;
    pub fn pcibios_fixup_irqs();
}

pub struct pci_dev;

#[repr(C, packed)]
pub struct irq_info {
    pub bus: u8,
    pub devfn: u8,
    pub irq: [irq_info_irq; 4],
    pub slot: u8,
    pub rfu: u8,
}

#[repr(C, packed)]
pub struct irq_info_irq {
    pub link: u8,
    pub bitmap: u16,
}

#[repr(C, packed)]
pub struct irq_routing_table {
    pub signature: u32,
    pub version: u16,
    pub size: u16,
    pub rtr_bus: u8,
    pub rtr_devfn: u8,
    pub exclusive_irqs: u16,
    pub rtr_vendor: u16,
    pub rtr_device: u16,
    pub miniport_data: u32,
    pub rfu: [u8; 11],
    pub checksum: u8,
    pub slots: [irq_info; 0],
}

#[repr(C, packed)]
pub struct irt_routing_table {
    pub signature: u32,
    pub size: u8,
    pub used: u8,
    pub exclusive_irqs: u16,
    pub slots: [irq_info; 0],
}

extern "C" {
    pub static mut pcibios_irq_mask: u32;
    pub static mut pci_config_lock: raw_spinlock_t;
    pub static mut pcibios_enable_irq: Option<unsafe extern "C" fn(*mut pci_dev)>;
    pub static mut pcibios_disable_irq: Option<unsafe extern "C" fn(*mut pci_dev)>;
    pub fn mp_should_keep_irq(dev: *mut device) -> bool;
}

#[repr(C)]
pub struct pci_raw_ops {
    pub read: Option<unsafe extern "C" fn(u32, u32, u32, c_int, c_int, *mut u32) -> c_int>,
    pub write: Option<unsafe extern "C" fn(u32, u32, u32, c_int, c_int, u32) -> c_int>,
}

extern "C" {
    pub static raw_pci_ops: *const pci_raw_ops;
    pub static raw_pci_ext_ops: *const pci_raw_ops;
    pub static pci_mmcfg: pci_raw_ops;
    pub static pci_direct_conf1: pci_raw_ops;
    pub static mut port_cf9_safe: bool;
}

#[cfg(CONFIG_PCI_DIRECT)]
extern "C" {
    pub fn pci_direct_probe() -> c_int;
    pub fn pci_direct_init(r#type: c_int);
}
#[cfg(not(CONFIG_PCI_DIRECT))]
pub unsafe fn pci_direct_probe() -> c_int { -1 }
#[cfg(not(CONFIG_PCI_DIRECT))]
pub unsafe fn pci_direct_init(_type: c_int) {}

#[cfg(CONFIG_PCI_BIOS)]
extern "C" { pub fn pci_pcbios_init(); }
#[cfg(not(CONFIG_PCI_BIOS))]
pub unsafe fn pci_pcbios_init() {}

extern "C" {
    pub fn dmi_check_pciprobe();
    pub fn dmi_check_skip_isa_align();
}

#[cfg(CONFIG_PCI)]
extern "C" { pub fn pci_acpi_init() -> c_int; }
#[cfg(not(CONFIG_PCI))]
pub unsafe fn pci_acpi_init() -> c_int { -22 }

pub const PCI_MMCFG_RESOURCE_NAME_LEN: usize = 22 + 4 + 2 + 2;

#[repr(C)]
pub struct pci_mmcfg_region {
    pub list: list_head,
    pub res: resource,
    pub address: u64,
    pub virt: *mut c_char,
    pub segment: u16,
    pub start_bus: u8,
    pub end_bus: u8,
    pub name: [c_char; PCI_MMCFG_RESOURCE_NAME_LEN],
}

extern "C" {
    pub fn pci_mmcfg_arch_init() -> c_int;
    pub fn pci_mmcfg_arch_free();
    pub fn pci_mmcfg_arch_map(cfg: *mut pci_mmcfg_region) -> c_int;
    pub fn pci_mmcfg_arch_unmap(cfg: *mut pci_mmcfg_region);
    pub fn pci_mmconfig_insert(dev: *mut device, seg: u16, start: u8, end: u8, addr: phys_addr_t) -> c_int;
    pub fn pci_mmconfig_delete(seg: u16, start: u8, end: u8) -> c_int;
    pub fn pci_mmconfig_lookup(segment: c_int, bus: c_int) -> *mut pci_mmcfg_region;
    pub fn pci_mmconfig_add(segment: c_int, start: c_int, end: c_int, addr: u64) -> *mut pci_mmcfg_region;
    pub static mut pci_mmcfg_list: list_head;
}

#[inline]
pub const fn PCI_MMCFG_BUS_OFFSET(bus: u32) -> u32 { bus << 20 }

#[inline]
pub unsafe fn mmio_config_readb(pos: *mut c_void) -> u8 {
    let val: u8;
    core::arch::asm!("movb ({p}),%al", p = in(reg) pos, out("al") val);
    val
}
#[inline]
pub unsafe fn mmio_config_readw(pos: *mut c_void) -> u16 {
    let val: u16;
    core::arch::asm!("movw ({p}),%ax", p = in(reg) pos, out("ax") val);
    val
}
#[inline]
pub unsafe fn mmio_config_readl(pos: *mut c_void) -> u32 {
    let val: u32;
    core::arch::asm!("movl ({p}),%eax", p = in(reg) pos, out("eax") val);
    val
}
#[inline]
pub unsafe fn mmio_config_writeb(pos: *mut c_void, val: u8) {
    core::arch::asm!("movb %al,({p})", p = in(reg) pos, in("al") val, options(nostack, preserves_flags));
}
#[inline]
pub unsafe fn mmio_config_writew(pos: *mut c_void, val: u16) {
    core::arch::asm!("movw %ax,({p})", p = in(reg) pos, in("ax") val, options(nostack, preserves_flags));
}
#[inline]
pub unsafe fn mmio_config_writel(pos: *mut c_void, val: u32) {
    core::arch::asm!("movl %eax,({p})", p = in(reg) pos, in("eax") val, options(nostack, preserves_flags));
}

/* x86_default_pci_* are build-time aliases in C; preserve their intent here. */
#[cfg(all(CONFIG_PCI, CONFIG_ACPI))]
pub use pci_acpi_init as x86_default_pci_init;
#[cfg(all(CONFIG_PCI, not(CONFIG_ACPI)))]
pub use pci_legacy_init as x86_default_pci_init;
#[cfg(CONFIG_PCI)]
pub use pcibios_irq_init as x86_default_pci_init_irq;
#[cfg(CONFIG_PCI)]
pub use pcibios_fixup_irqs as x86_default_pci_fixup_irqs;

#[cfg(all(CONFIG_PCI, CONFIG_ACPI))]
extern "C" { pub static mut pci_use_e820: bool; }
#[cfg(not(all(CONFIG_PCI, CONFIG_ACPI)))]
pub const pci_use_e820: bool = false;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
