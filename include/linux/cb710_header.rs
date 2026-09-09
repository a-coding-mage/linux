/* SPDX-License-Identifier: GPL-2.0-only */
/* Translated from cb710.h and sgbuf2.h. */

use core::ffi::{c_int, c_void};

/* Kernel-provided types and operations. */
#[repr(C)] pub struct pci_dev { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct device { pub parent: *mut device, pub driver_data: *mut c_void }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct atomic_t { _private: [u8; 0] }
#[repr(C)] pub struct sg_mapping_iter { _private: [u8; 0] }

extern "C" {
    fn ioread8(addr: *mut c_void) -> u8;
    fn ioread16(addr: *mut c_void) -> u16;
    fn ioread32(addr: *mut c_void) -> u32;
    fn iowrite8(value: u8, addr: *mut c_void);
    fn iowrite16(value: u16, addr: *mut c_void);
    fn iowrite32(value: u32, addr: *mut c_void);
    fn cb710_pci_update_config_reg(pdev: *mut pci_dev, reg: c_int, and: u32, xor: u32);
    fn cb710_set_irq_handler(slot: *mut cb710_slot, handler: cb710_irq_handler_t);
    fn cb710_sg_dwiter_read_next_block(miter: *mut sg_mapping_iter) -> u32;
    fn cb710_sg_dwiter_write_next_block(miter: *mut sg_mapping_iter, data: u32);
}

pub type cb710_irq_handler_t = Option<unsafe extern "C" fn(*mut cb710_slot) -> c_int>;

#[repr(C)]
pub struct cb710_slot {
    pub pdev: platform_device,
    pub iobase: *mut c_void,
    pub irq_handler: cb710_irq_handler_t,
}

#[repr(C)]
pub struct cb710_chip {
    pub pdev: *mut pci_dev,
    pub iobase: *mut c_void,
    pub platform_id: u32,
    #[cfg(CONFIG_CB710_DEBUG_ASSUMPTIONS)]
    pub slot_refs_count: atomic_t,
    pub slot_mask: u32,
    pub slots: u32,
    pub irq_lock: spinlock_t,
    pub slot: [cb710_slot; 0],
}

pub const CB710_SLOT_MMC: u32 = 1;
pub const CB710_SLOT_MS: u32 = 2;
pub const CB710_SLOT_SM: u32 = 4;

#[inline]
pub unsafe fn cb710_write_port_8(slot: *mut cb710_slot, port: u32, value: u8) { iowrite8(value, (*slot).iobase.add(port as usize)); }
#[inline]
pub unsafe fn cb710_read_port_8(slot: *mut cb710_slot, port: u32) -> u8 { ioread8((*slot).iobase.add(port as usize)) }
#[inline]
pub unsafe fn cb710_modify_port_8(slot: *mut cb710_slot, port: u32, set: u8, clear: u8) { iowrite8((ioread8((*slot).iobase.add(port as usize)) & !clear) | set, (*slot).iobase.add(port as usize)); }
#[inline]
pub unsafe fn cb710_write_port_16(slot: *mut cb710_slot, port: u32, value: u16) { iowrite16(value, (*slot).iobase.add(port as usize)); }
#[inline]
pub unsafe fn cb710_read_port_16(slot: *mut cb710_slot, port: u32) -> u16 { ioread16((*slot).iobase.add(port as usize)) }
#[inline]
pub unsafe fn cb710_modify_port_16(slot: *mut cb710_slot, port: u32, set: u16, clear: u16) { iowrite16((ioread16((*slot).iobase.add(port as usize)) & !clear) | set, (*slot).iobase.add(port as usize)); }
#[inline]
pub unsafe fn cb710_write_port_32(slot: *mut cb710_slot, port: u32, value: u32) { iowrite32(value, (*slot).iobase.add(port as usize)); }
#[inline]
pub unsafe fn cb710_read_port_32(slot: *mut cb710_slot, port: u32) -> u32 { ioread32((*slot).iobase.add(port as usize)) }
#[inline]
pub unsafe fn cb710_modify_port_32(slot: *mut cb710_slot, port: u32, set: u32, clear: u32) { iowrite32((ioread32((*slot).iobase.add(port as usize)) & !clear) | set, (*slot).iobase.add(port as usize)); }

#[inline] pub unsafe fn cb710_pdev_to_slot(pdev: *mut platform_device) -> *mut cb710_slot { pdev as *mut cb710_slot }
#[inline] pub unsafe fn cb710_slot_to_chip(slot: *mut cb710_slot) -> *mut cb710_chip { (*(*slot).pdev.dev.parent).driver_data as *mut cb710_chip }
#[inline] pub unsafe fn cb710_slot_dev(slot: *mut cb710_slot) -> *mut device { &mut (*slot).pdev.dev }
#[inline] pub unsafe fn cb710_chip_dev(chip: *mut cb710_chip) -> *mut device { chip as *mut device }

#[cfg(CONFIG_CB710_DEBUG)]
extern "C" { pub fn cb710_dump_regs(chip: *mut cb710_chip, dump: u32); }
#[cfg(not(CONFIG_CB710_DEBUG))]
#[inline] pub unsafe fn cb710_dump_regs(_c: *mut cb710_chip, _d: u32) {}

pub const CB710_DUMP_REGS_MMC: u32 = 0x0F;
pub const CB710_DUMP_REGS_MS: u32 = 0x30;
pub const CB710_DUMP_REGS_SM: u32 = 0xC0;
pub const CB710_DUMP_REGS_ALL: u32 = 0xFF;
pub const CB710_DUMP_REGS_MASK: u32 = 0xFF;
pub const CB710_DUMP_ACCESS_8: u32 = 0x100;
pub const CB710_DUMP_ACCESS_16: u32 = 0x200;
pub const CB710_DUMP_ACCESS_32: u32 = 0x400;
pub const CB710_DUMP_ACCESS_ALL: u32 = 0x700;
pub const CB710_DUMP_ACCESS_MASK: u32 = 0x700;

#[inline]
pub unsafe fn cb710_sg_dwiter_write_from_io(miter: *mut sg_mapping_iter, port: *mut c_void, mut count: usize) {
    while count > 0 { count -= 1; cb710_sg_dwiter_write_next_block(miter, ioread32(port)); }
}
#[inline]
pub unsafe fn cb710_sg_dwiter_read_to_io(miter: *mut sg_mapping_iter, port: *mut c_void, mut count: usize) {
    while count > 0 { count -= 1; iowrite32(cb710_sg_dwiter_read_next_block(miter), port); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
