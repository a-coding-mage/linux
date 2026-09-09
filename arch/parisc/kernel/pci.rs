/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1997, 1998 Ralf Baechle
 * Copyright (C) 1999 SuSE GmbH
 * Copyright (C) 1999-2001 Hewlett-Packard Company
 * Copyright (C) 1999-2001 Grant Grundler
 */

/* Linux headers and architecture headers are supplied by the surrounding tree. */

pub const DEBUG_RESOURCES: i32 = 0;
pub const DEBUG_CONFIG: i32 = 0;
pub const PCI_HBA_MAX: usize = 32;

extern "C" {
    static mut pci_port: *mut pci_port_ops;
    static mut pci_bios: *mut pci_bios_ops;
    static mut pci_hba_count: i32;
    static mut parisc_pci_hba: [*mut pci_hba_data; PCI_HBA_MAX];
    static mut pci_cache_line_size: u8;
    static pci_dfl_cache_line_size: u8;
    static EISA_bus: bool;
}

#[repr(C)] pub struct pci_port_ops {
    pub inb: unsafe extern "C" fn(*mut pci_hba_data, i32) -> u8,
    pub inw: unsafe extern "C" fn(*mut pci_hba_data, i32) -> u16,
    pub inl: unsafe extern "C" fn(*mut pci_hba_data, i32) -> u32,
    pub outb: unsafe extern "C" fn(*mut pci_hba_data, i32, u8),
    pub outw: unsafe extern "C" fn(*mut pci_hba_data, i32, u16),
    pub outl: unsafe extern "C" fn(*mut pci_hba_data, i32, u32),
}
#[repr(C)] pub struct pci_bios_ops {
    pub init: Option<unsafe extern "C" fn()>,
    pub fixup_bus: Option<unsafe extern "C" fn(*mut pci_bus)>,
}
#[repr(C)] pub struct pci_hba_data { pub hba_num: i32 }
#[repr(C)] pub struct pci_bus { _private: [u8; 0] }
#[repr(C)] pub struct pci_dev { pub class: u32, pub dev: device }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct resource { pub parent: *mut resource, pub start: resource_size_t, pub end: resource_size_t, pub flags: u32 }
pub type resource_size_t = u64;

extern "C" {
    fn eisa_inb(addr: i32) -> u8; fn eisa_inw(addr: i32) -> u16; fn eisa_inl(addr: i32) -> u32;
    fn eisa_outb(d: u8, addr: i32); fn eisa_outw(d: u16, addr: i32); fn eisa_outl(d: u32, addr: i32);
    fn PCI_PORT_HBA(addr: i32) -> usize;
    fn PCI_PORT_ADDR(addr: i32) -> i32;
    fn pci_read_config_byte(dev: *mut pci_dev, where_: u32, val: *mut u8) -> i32;
    fn pci_read_config_word(dev: *mut pci_dev, where_: u32, val: *mut u16) -> i32;
    fn pci_write_config_word(dev: *mut pci_dev, where_: u32, val: u16) -> i32;
    fn pci_write_config_byte(dev: *mut pci_dev, where_: u32, val: u8) -> i32;
    fn pci_enable_resources(dev: *mut pci_dev, mask: i32) -> i32;
    fn pci_align_resource(dev: *mut pci_dev, res: *const resource, empty_res: *const resource, size: resource_size_t, alignment: resource_size_t) -> resource_size_t;
    fn pci_name(dev: *mut pci_dev) -> *const core::ffi::c_char;
    fn printk(fmt: *const core::ffi::c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const core::ffi::c_char, ...);
}

pub unsafe fn inb(addr: i32) -> u8 { let b = PCI_PORT_HBA(addr); if EISA_bus && b == 0 { return eisa_inb(addr); } if parisc_pci_hba[b].is_null() { return u8::MAX; } ((*pci_port).inb)(parisc_pci_hba[b], PCI_PORT_ADDR(addr)) }
pub unsafe fn inw(addr: i32) -> u16 { let b = PCI_PORT_HBA(addr); if EISA_bus && b == 0 { return eisa_inw(addr); } if parisc_pci_hba[b].is_null() { return u16::MAX; } ((*pci_port).inw)(parisc_pci_hba[b], PCI_PORT_ADDR(addr)) }
pub unsafe fn inl(addr: i32) -> u32 { let b = PCI_PORT_HBA(addr); if EISA_bus && b == 0 { return eisa_inl(addr); } if parisc_pci_hba[b].is_null() { return u32::MAX; } ((*pci_port).inl)(parisc_pci_hba[b], PCI_PORT_ADDR(addr)) }
pub unsafe fn outb(d: u8, addr: i32) { let b = PCI_PORT_HBA(addr); if EISA_bus && b == 0 { eisa_outb(d, addr); return; } if !parisc_pci_hba[b].is_null() { ((*pci_port).outb)(parisc_pci_hba[b], PCI_PORT_ADDR(addr), d); } }
pub unsafe fn outw(d: u16, addr: i32) { let b = PCI_PORT_HBA(addr); if EISA_bus && b == 0 { eisa_outw(d, addr); return; } if !parisc_pci_hba[b].is_null() { ((*pci_port).outw)(parisc_pci_hba[b], PCI_PORT_ADDR(addr), d); } }
pub unsafe fn outl(d: u32, addr: i32) { let b = PCI_PORT_HBA(addr); if EISA_bus && b == 0 { eisa_outl(d, addr); return; } if !parisc_pci_hba[b].is_null() { ((*pci_port).outl)(parisc_pci_hba[b], PCI_PORT_ADDR(addr), d); } }

pub unsafe extern "C" fn pcibios_init() -> i32 { if pci_bios.is_null() { return -1; } if let Some(init) = (*pci_bios).init { init(); } else { /* printk(KERN_WARNING ...) */ } pci_cache_line_size = pci_dfl_cache_line_size; 0 }
pub unsafe extern "C" fn pcibios_fixup_bus(bus: *mut pci_bus) { if let Some(fixup) = (*pci_bios).fixup_bus { fixup(bus); } }

pub unsafe extern "C" fn pcibios_set_master(dev: *mut pci_dev) { let mut lat = 0u8; pci_read_config_byte(dev, 0x0d, &mut lat); if lat >= 16 { return; } pci_write_config_word(dev, 0x0c, (0x80u16 << 8) | pci_cache_line_size as u16); }
pub unsafe extern "C" fn pcibios_init_bridge(dev: *mut pci_dev) { if dev.is_null() || ((*dev).class >> 8) != 0x0604 { return; } pci_write_config_byte(dev, 0x1b, 32); let mut bridge_ctl = 0u16; pci_read_config_word(dev, 0x3e, &mut bridge_ctl); let bridge_ctl_new = bridge_ctl | 1 | 2 | 4; pci_write_config_word(dev, 0x3e, bridge_ctl_new); }

pub unsafe extern "C" fn pcibios_align_resource(data: *mut core::ffi::c_void, res: *const resource, empty_res: *const resource, size: resource_size_t, alignment: resource_size_t) -> resource_size_t { let dev = data as *mut pci_dev; let mut start = (*res).start; let align = if (*res).flags & 0x100 == 0x100 { 0x1000 } else { 0x100000 }; if align > alignment { start = (start + align - 1) & !(align - 1); } else { start = pci_align_resource(dev, res, empty_res, size, alignment); } start }

pub unsafe extern "C" fn pcibios_enable_device(dev: *mut pci_dev, mask: i32) -> i32 { let err = pci_enable_resources(dev, mask); if err < 0 { return err; } let mut cmd = 0u16; pci_read_config_word(dev, 4, &mut cmd); let old_cmd = cmd; cmd |= 0x400 | 0x40; if cmd != old_cmd { pci_write_config_word(dev, 4, cmd); } 0 }
pub unsafe extern "C" fn pcibios_register_hba(hba: *mut pci_hba_data) { if pci_hba_count >= PCI_HBA_MAX as i32 { return; } parisc_pci_hba[pci_hba_count as usize] = hba; (*hba).hba_num = pci_hba_count; pci_hba_count += 1; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
