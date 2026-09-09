// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 1999, 2000, 2004  MIPS Technologies, Inc.
 *	All rights reserved.
 *	Authors: Carsten Langgaard <carstenl@mips.com>
 *		 Maciej W. Rozycki <macro@mips.com>
 *
 * Copyright (C) 2009 Lemote Inc.
 * Author: Wu Zhangjin <wuzhangjin@gmail.com>
 */

// C dependencies: linux/types.h, linux/pci.h, linux/kernel.h,
// linux/export.h, loongson.h, and (when CONFIG_CS5536 is enabled)
// cs5536/cs5536_pci.h and cs5536/cs5536.h.

const PCI_ACCESS_READ: u8 = 0;
const PCI_ACCESS_WRITE: u8 = 1;
const ID_SEL_BEGIN: u32 = 11;
const MAX_DEV_NUM: u32 = 31 - ID_SEL_BEGIN;

// CFG_SPACE_REG(offset) expands to (void *)CKSEG1ADDR(LOONGSON_PCICFG_BASE | (offset)).
#[inline]
unsafe fn cfg_space_reg(offset: u32) -> *mut core::ffi::c_void {
    ckseg1addr(loongson_pcicfg_base | offset) as *mut core::ffi::c_void
}

unsafe fn loongson_pcibios_config_access(
    access_type: u8,
    bus: *mut pci_bus,
    devfn: u32,
    where_: i32,
    data: *mut u32,
) -> i32 {
    let busnum: u32 = (*bus).number;
    let addr: u32;
    let type_: u32;
    let dummy: u32;
    let addrp: *mut core::ffi::c_void;
    let device: i32 = pci_slot(devfn) as i32;
    let function: u32 = pci_func(devfn);
    let reg: i32 = where_ & !3;

    if busnum == 0 {
        // Board-specific part: only CS5536 accesses below PCI_MSR_CTRL.
        #[cfg(feature = "CONFIG_CS5536")]
        {
            if (PCI_IDSEL_CS5536 == device as u32) && (reg < PCI_MSR_CTRL as i32) {
                match access_type {
                    PCI_ACCESS_READ => *data = cs5536_pci_conf_read4(function, reg as u32),
                    PCI_ACCESS_WRITE => cs5536_pci_conf_write4(function, reg as u32, *data),
                    _ => {}
                }
                return 0;
            }
        }

        // Type 0 configuration for onboard PCI bus.
        if device as u32 > MAX_DEV_NUM {
            return -1;
        }

        addr = (1u32 << (device as u32 + ID_SEL_BEGIN))
            | (function << 8)
            | reg as u32;
        type_ = 0;
    } else {
        // Type 1 configuration for offboard PCI bus.
        addr = (busnum << 16)
            | ((device as u32) << 11)
            | (function << 8)
            | reg as u32;
        type_ = 0x10000;
    }

    // Clear aborts.
    loongson_pcicmd |= loongson_pcicmd_mabort_clr | loongson_pcicmd_mtabort_clr;
    loongson_pcimap_cfg = (addr >> 16) | type_;

    // Flush Bonito register block.
    dummy = loongson_pcimap_cfg;
    let _ = dummy;
    mmiowb();

    addrp = cfg_space_reg(addr & 0xffff);
    if access_type == PCI_ACCESS_WRITE {
        writel(cpu_to_le32(*data), addrp);
    } else {
        *data = le32_to_cpu(readl(addrp));
    }

    // Detect Master/Target abort.
    if (loongson_pcicmd & (loongson_pcicmd_mabort_clr | loongson_pcicmd_mtabort_clr)) != 0 {
        // Error occurred; clear bits.
        loongson_pcicmd |= loongson_pcicmd_mabort_clr | loongson_pcicmd_mtabort_clr;
        return -1;
    }

    0
}

/*
 * We can't address 8 and 16 bit words directly. Instead we have to
 * read/write a 32bit word and mask/modify the data we actually want.
 */
unsafe fn loongson_pcibios_read(
    bus: *mut pci_bus,
    devfn: u32,
    where_: i32,
    size: i32,
    val: *mut u32,
) -> i32 {
    let mut data: u32 = 0;

    if (size == 2) && (where_ & 1) != 0 || (size == 4) && (where_ & 3) != 0 {
        return PCIBIOS_BAD_REGISTER_NUMBER;
    }
    if loongson_pcibios_config_access(PCI_ACCESS_READ, bus, devfn, where_, &mut data) != 0 {
        return -1;
    }
    if size == 1 {
        *val = (data >> (((where_ & 3) << 3) as u32)) & 0xff;
    } else if size == 2 {
        *val = (data >> (((where_ & 3) << 3) as u32)) & 0xffff;
    } else {
        *val = data;
    }
    PCIBIOS_SUCCESSFUL
}

unsafe fn loongson_pcibios_write(
    bus: *mut pci_bus,
    devfn: u32,
    where_: i32,
    size: i32,
    val: u32,
) -> i32 {
    let mut data: u32 = 0;

    if (size == 2) && (where_ & 1) != 0 || (size == 4) && (where_ & 3) != 0 {
        return PCIBIOS_BAD_REGISTER_NUMBER;
    }
    if size == 4 {
        data = val;
    } else {
        if loongson_pcibios_config_access(PCI_ACCESS_READ, bus, devfn, where_, &mut data) != 0 {
            return -1;
        }
        if size == 1 {
            data = (data & !(0xff << (((where_ & 3) << 3) as u32)))
                | (val << (((where_ & 3) << 3) as u32));
        } else if size == 2 {
            data = (data & !(0xffff << (((where_ & 3) << 3) as u32)))
                | (val << (((where_ & 3) << 3) as u32));
        }
    }
    if loongson_pcibios_config_access(PCI_ACCESS_WRITE, bus, devfn, where_, &mut data) != 0 {
        return -1;
    }
    PCIBIOS_SUCCESSFUL
}

pub static mut loongson_pci_ops: pci_ops = pci_ops {
    read: Some(loongson_pcibios_read),
    write: Some(loongson_pcibios_write),
};

#[cfg(feature = "CONFIG_CS5536")]
pub static mut msr_lock: raw_spinlock_t = DEFINE_RAW_SPINLOCK();

#[cfg(feature = "CONFIG_CS5536")]
pub unsafe fn _rdmsr(msr: u32, hi: *mut u32, lo: *mut u32) {
    let mut bus = pci_bus { number: PCI_BUS_CS5536 };
    let devfn: u32 = pci_devfn(PCI_IDSEL_CS5536, 0);
    let mut flags: c_ulong = 0;

    raw_spin_lock_irqsave(&mut msr_lock, &mut flags);
    loongson_pcibios_write(&mut bus, devfn, PCI_MSR_ADDR, 4, msr);
    loongson_pcibios_read(&mut bus, devfn, PCI_MSR_DATA_LO, 4, lo);
    loongson_pcibios_read(&mut bus, devfn, PCI_MSR_DATA_HI, 4, hi);
    raw_spin_unlock_irqrestore(&mut msr_lock, flags);
}

#[cfg(feature = "CONFIG_CS5536")]
pub unsafe fn _wrmsr(msr: u32, hi: u32, lo: u32) {
    let mut bus = pci_bus { number: PCI_BUS_CS5536 };
    let devfn: u32 = pci_devfn(PCI_IDSEL_CS5536, 0);
    let mut flags: c_ulong = 0;

    raw_spin_lock_irqsave(&mut msr_lock, &mut flags);
    loongson_pcibios_write(&mut bus, devfn, PCI_MSR_ADDR, 4, msr);
    loongson_pcibios_write(&mut bus, devfn, PCI_MSR_DATA_LO, 4, lo);
    loongson_pcibios_write(&mut bus, devfn, PCI_MSR_DATA_HI, 4, hi);
    raw_spin_unlock_irqrestore(&mut msr_lock, flags);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
