// SPDX-License-Identifier: GPL-2.0
/*
 * Generic SH7786 PCI-Express operations.
 *
 *  Copyright (C) 2009 - 2010  Paul Mundt
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

#[repr(C)]
pub struct pci_bus {
    pub sysdata: *mut core::ffi::c_void,
    pub number: u8,
    pub parent: *mut pci_bus,
    pub dev: core::ffi::c_void,
}

#[repr(C)]
pub struct pci_channel {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_ops {
    pub read: Option<unsafe extern "C" fn(*mut pci_bus, u32, i32, i32, *mut u32) -> i32>,
    pub write: Option<unsafe extern "C" fn(*mut pci_bus, u32, i32, i32, u32) -> i32>,
}

extern "C" {
    static mut pci_config_lock: core::ffi::c_void;
    fn pci_read_reg(chan: *mut pci_channel, reg: u32) -> u32;
    fn pci_write_reg(chan: *mut pci_channel, value: u32, reg: u32);
    fn pci_is_root_bus(bus: *mut pci_bus) -> bool;
    fn raw_spin_lock_irqsave(lock: *mut core::ffi::c_void, flags: *mut usize);
    fn raw_spin_unlock_irqrestore(lock: *mut core::ffi::c_void, flags: usize);
    fn pci_slot(devfn: u32) -> i32;
    fn pci_func(devfn: u32) -> i32;
    fn dev_dbg(dev: *mut core::ffi::c_void, format: *const u8, ...);
}

const PCI_ACCESS_READ: u8 = 0;
const PCI_ACCESS_WRITE: u8 = 1;

const PCIBIOS_SUCCESSFUL: i32 = 0;
extern "C" {
    static PCIBIOS_FUNC_NOT_SUPPORTED: i32;
    static PCIBIOS_DEVICE_NOT_FOUND: i32;
    static PCIBIOS_BAD_REGISTER_NUMBER: i32;
}

const SH4A_PCIEERRFR: u32 = 0;
const SH4A_PCIEPAR: u32 = 0;
const SH4A_PCIEPCTLR: u32 = 0;
const SH4A_PCIEPCICONF1: u32 = 0;
const SH4A_PCIEPDR: u32 = 0;

extern "C" { fn PCI_REG(reg: i32) -> u32; }

unsafe extern "C" fn sh7786_pcie_config_access(
    access_type: u8,
    bus: *mut pci_bus,
    devfn: u32,
    where_: i32,
    data: *mut u32,
) -> i32 {
    let chan = (*bus).sysdata as *mut pci_channel;
    let dev = pci_slot(devfn);
    let func = pci_func(devfn);
    let type_ = if !(*bus).parent.is_null() { 1 } else { 0 };
    let reg = where_ & !3;

    if (*bus).number > 255 || dev > 31 || func > 7 {
        return PCIBIOS_FUNC_NOT_SUPPORTED;
    }

    /*
     * While each channel has its own memory-mapped extended config
     * space, it's generally only accessible when in endpoint mode.
     * When in root complex mode, the controller is unable to target
     * itself with either type 0 or type 1 accesses, and indeed, any
     * controller initiated target transfer to its own config space
     * result in a completer abort.
     *
     * Each channel effectively only supports a single device, but as
     * the same channel <-> device access works for any PCI_SLOT()
     * value, we cheat a bit here and bind the controller's config
     * space to devfn 0 in order to enable self-enumeration. In this
     * case the regular PAR/PDR path is sidelined and the mangled
     * config access itself is initiated as a SuperHyway transaction.
     */
    if pci_is_root_bus(bus) {
        if dev == 0 {
            if access_type == PCI_ACCESS_READ {
                *data = pci_read_reg(chan, PCI_REG(reg));
            } else {
                pci_write_reg(chan, *data, PCI_REG(reg));
            }
            return PCIBIOS_SUCCESSFUL;
        } else if dev > 1 {
            return PCIBIOS_DEVICE_NOT_FOUND;
        }
    }

    pci_write_reg(chan, pci_read_reg(chan, SH4A_PCIEERRFR), SH4A_PCIEERRFR);
    pci_write_reg(chan, ((*bus).number as u32) << 24 | (dev as u32) << 19 |
        (func as u32) << 16 | reg as u32, SH4A_PCIEPAR);
    pci_write_reg(chan, (1u32 << 31) | (type_ << 8), SH4A_PCIEPCTLR);

    if pci_read_reg(chan, SH4A_PCIEERRFR) & 0x10 != 0 {
        return PCIBIOS_DEVICE_NOT_FOUND;
    }
    if pci_read_reg(chan, SH4A_PCIEPCICONF1) & ((1 << 29) | (1 << 28)) != 0 {
        return PCIBIOS_DEVICE_NOT_FOUND;
    }
    if access_type == PCI_ACCESS_READ {
        *data = pci_read_reg(chan, SH4A_PCIEPDR);
    } else {
        pci_write_reg(chan, *data, SH4A_PCIEPDR);
    }
    pci_write_reg(chan, 0, SH4A_PCIEPCTLR);
    PCIBIOS_SUCCESSFUL
}

unsafe extern "C" fn sh7786_pcie_read(
    bus: *mut pci_bus, devfn: u32, where_: i32, size: i32, val: *mut u32,
) -> i32 {
    let mut flags = 0usize;
    let mut data = 0u32;
    if (size == 2 && where_ & 1 != 0) || (size == 4 && where_ & 3 != 0) {
        return PCIBIOS_BAD_REGISTER_NUMBER;
    }
    raw_spin_lock_irqsave(&mut pci_config_lock, &mut flags);
    let ret = sh7786_pcie_config_access(PCI_ACCESS_READ, bus, devfn, where_, &mut data);
    if ret != PCIBIOS_SUCCESSFUL {
        *val = 0xffff_ffff;
    } else if size == 1 {
        *val = (data >> (((where_ & 3) << 3) as u32)) & 0xff;
    } else if size == 2 {
        *val = (data >> (((where_ & 2) << 3) as u32)) & 0xffff;
    } else {
        *val = data;
    }
    raw_spin_unlock_irqrestore(&mut pci_config_lock, flags);
    ret
}

unsafe extern "C" fn sh7786_pcie_write(
    bus: *mut pci_bus, devfn: u32, where_: i32, size: i32, val: u32,
) -> i32 {
    let mut flags = 0usize;
    let mut data = 0u32;
    if (size == 2 && where_ & 1 != 0) || (size == 4 && where_ & 3 != 0) {
        return PCIBIOS_BAD_REGISTER_NUMBER;
    }
    raw_spin_lock_irqsave(&mut pci_config_lock, &mut flags);
    let mut ret = sh7786_pcie_config_access(PCI_ACCESS_READ, bus, devfn, where_, &mut data);
    if ret == PCIBIOS_SUCCESSFUL {
        if size == 1 {
            let shift = ((where_ & 3) << 3) as u32;
            data = (data & !(0xff << shift)) | ((val & 0xff) << shift);
        } else if size == 2 {
            let shift = ((where_ & 2) << 3) as u32;
            data = (data & !(0xffff << shift)) | ((val & 0xffff) << shift);
        } else {
            data = val;
        }
        ret = sh7786_pcie_config_access(PCI_ACCESS_WRITE, bus, devfn, where_, &mut data);
    }
    raw_spin_unlock_irqrestore(&mut pci_config_lock, flags);
    ret
}

#[no_mangle]
pub static mut sh7786_pci_ops: pci_ops = pci_ops {
    read: Some(sh7786_pcie_read),
    write: Some(sh7786_pcie_write),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
