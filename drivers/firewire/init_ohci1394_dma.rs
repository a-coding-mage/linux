// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * init_ohci1394_dma.c - Initializes physical DMA on all OHCI 1394 controllers
 *
 * Copyright (C) 2006-2007      Bernhard Kaindl <bk@suse.de>
 *
 * Derived from drivers/ieee1394/ohci1394.c and arch/x86/kernel/early-quirks.c
 * this file has functions to:
 * - scan the PCI very early on boot for all OHCI 1394-compliant controllers
 * - reset and initialize them and make them join the IEEE1394 bus and
 * - enable physical DMA on them to allow remote debugging
 */

// Kernel and architecture declarations are supplied by the surrounding kernel.

#[repr(C)]
pub struct ohci {
    pub registers: *mut core::ffi::c_void,
}

pub static mut init_ohci1394_dma_early: i32 = 0;

#[inline]
unsafe fn reg_write(ohci: *const ohci, offset: i32, data: u32) {
    writel(data, ((*ohci).registers as *mut u8).offset(offset as isize));
}

#[inline]
unsafe fn reg_read(ohci: *const ohci, offset: i32) -> u32 {
    readl(((*ohci).registers as *mut u8).offset(offset as isize))
}

const OHCI_LOOP_COUNT: i32 = 100;

/* Reads a PHY register of an OHCI-1394 controller */
#[inline]
unsafe fn get_phy_reg(ohci: *mut ohci, addr: u8) -> u8 {
    reg_write(ohci, OHCI1394_PhyControl, ((addr as u32) << 8) | 0x00008000);
    for _i in 0..OHCI_LOOP_COUNT {
        if reg_read(ohci, OHCI1394_PhyControl) & 0x80000000 != 0 {
            break;
        }
        mdelay(1);
    }
    let r = reg_read(ohci, OHCI1394_PhyControl);
    ((r & 0x00ff0000) >> 16) as u8
}

/* Writes to a PHY register of an OHCI-1394 controller */
#[inline]
unsafe fn set_phy_reg(ohci: *mut ohci, addr: u8, data: u8) {
    reg_write(ohci, OHCI1394_PhyControl,
        ((addr as u32) << 8) | data as u32 | 0x00004000);
    for _i in 0..OHCI_LOOP_COUNT {
        if reg_read(ohci, OHCI1394_PhyControl) & 0x00004000 == 0 {
            break;
        }
        mdelay(1);
    }
}

/* Resets an OHCI-1394 controller (for sane state before initialization) */
#[inline]
unsafe fn init_ohci1394_soft_reset(ohci: *mut ohci) {
    reg_write(ohci, OHCI1394_HCControlSet, OHCI1394_HCControl_softReset);
    for _i in 0..OHCI_LOOP_COUNT {
        if reg_read(ohci, OHCI1394_HCControlSet) & OHCI1394_HCControl_softReset == 0 {
            break;
        }
        mdelay(1);
    }
}

const OHCI1394_MAX_AT_REQ_RETRIES: u32 = 0xf;
const OHCI1394_MAX_AT_RESP_RETRIES: u32 = 0x2;
const OHCI1394_MAX_PHYS_RESP_RETRIES: u32 = 0x8;

/* Basic OHCI-1394 register and port inititalization */
#[inline]
unsafe fn init_ohci1394_initialize(ohci: *mut ohci) {
    let mut bus_options = reg_read(ohci, OHCI1394_BusOptions);
    bus_options |= 0x60000000;
    bus_options &= !0x00ff0000;
    bus_options &= !0x18000000;
    reg_write(ohci, OHCI1394_BusOptions, bus_options);
    reg_write(ohci, OHCI1394_NodeID, 0x0000ffc0);
    reg_write(ohci, OHCI1394_HCControlSet, OHCI1394_HCControl_postedWriteEnable);
    reg_write(ohci, OHCI1394_LinkControlClear, 0xffffffff);
    reg_write(ohci, OHCI1394_LinkControlSet, OHCI1394_LinkControl_rcvPhyPkt);
    reg_write(ohci, OHCI1394_LinkControlClear, 0x00000400);
    reg_write(ohci, OHCI1394_IsoRecvIntMaskClear, 0xffffffff);
    reg_write(ohci, OHCI1394_IsoRecvIntEventClear, 0xffffffff);
    reg_write(ohci, OHCI1394_IsoXmitIntMaskClear, 0xffffffff);
    reg_write(ohci, OHCI1394_IsoXmitIntEventClear, 0xffffffff);
    reg_write(ohci, OHCI1394_AsReqFilterHiSet, 0x80000000);
    reg_write(ohci, OHCI1394_ATRetries,
        OHCI1394_MAX_AT_REQ_RETRIES |
        (OHCI1394_MAX_AT_RESP_RETRIES << 4) |
        (OHCI1394_MAX_PHYS_RESP_RETRIES << 8));
    reg_write(ohci, OHCI1394_HCControlClear, OHCI1394_HCControl_noByteSwapData);
    reg_write(ohci, OHCI1394_HCControlSet, OHCI1394_HCControl_linkEnable);
    let num_ports = (get_phy_reg(ohci, 2) & 0xf) as i32;
    for i in 0..num_ports {
        set_phy_reg(ohci, 7, i as u8);
        let status = get_phy_reg(ohci, 8);
        if status & 0x20 != 0 {
            set_phy_reg(ohci, 8, status & !1);
        }
    }
}

#[inline]
unsafe fn init_ohci1394_wait_for_busresets(ohci: *mut ohci) {
    for _i in 0..9 {
        mdelay(200);
        let events = reg_read(ohci, OHCI1394_IntEventSet);
        if events & OHCI1394_busReset != 0 {
            reg_write(ohci, OHCI1394_IntEventClear, OHCI1394_busReset);
        }
    }
}

#[inline]
unsafe fn init_ohci1394_enable_physical_dma(ohci: *mut ohci) {
    reg_write(ohci, OHCI1394_PhyReqFilterHiSet, 0xffffffff);
    reg_write(ohci, OHCI1394_PhyReqFilterLoSet, 0xffffffff);
    reg_write(ohci, OHCI1394_PhyUpperBound, 0xffff0000);
}

#[inline]
unsafe fn init_ohci1394_reset_and_init_dma(ohci: *mut ohci) {
    init_ohci1394_soft_reset(ohci);
    reg_write(ohci, OHCI1394_HCControlSet, OHCI1394_HCControl_LPS);
    reg_write(ohci, OHCI1394_IntEventClear, 0xffffffff);
    reg_write(ohci, OHCI1394_IntMaskClear, 0xffffffff);
    mdelay(50);
    init_ohci1394_initialize(ohci);
    init_ohci1394_wait_for_busresets(ohci);
    init_ohci1394_enable_physical_dma(ohci);
}

#[inline]
unsafe fn init_ohci1394_controller(num: i32, slot: i32, func: i32) {
    printk(KERN_INFO, b"init_ohci1394_dma: initializing OHCI-1394 at %02x:%02x.%x\n\0".as_ptr(), num, slot, func);
    let ohci_base = read_pci_config(num, slot, func, PCI_BASE_ADDRESS_0 + (0 << 2))
        & PCI_BASE_ADDRESS_MEM_MASK;
    set_fixmap_nocache(FIX_OHCI1394_BASE, ohci_base);
    let mut controller = ohci { registers: fix_to_virt(FIX_OHCI1394_BASE) as *mut core::ffi::c_void };
    init_ohci1394_reset_and_init_dma(&mut controller);
}

pub unsafe fn init_ohci1394_dma_on_all_controllers() {
    if !early_pci_allowed() {
        return;
    }
    for num in 0..32 {
        for slot in 0..32 {
            for func in 0..8 {
                let class = read_pci_config(num, slot, func, PCI_CLASS_REVISION);
                if class == 0xffffffff || (class >> 8) != PCI_CLASS_SERIAL_FIREWIRE_OHCI {
                    continue;
                }
                init_ohci1394_controller(num, slot, func);
                break;
            }
        }
    }
    printk(KERN_INFO, b"init_ohci1394_dma: finished initializing OHCI DMA\n\0".as_ptr());
}

unsafe fn setup_ohci1394_dma(opt: *mut i8) -> i32 {
    if strcmp(opt, b"early\0".as_ptr() as *const i8) == 0 {
        init_ohci1394_dma_early = 1;
    }
    0
}

// early_param("ohci1394_dma", setup_ohci1394_dma);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
