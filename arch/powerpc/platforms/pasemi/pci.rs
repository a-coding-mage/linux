// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2006 PA Semi, Inc
 *
 * Authors: Kip Walker, PA Semi
 *          Olof Johansson, PA Semi
 *
 * Maintained by: Olof Johansson <olof@lixom.net>
 *
 * Based on arch/powerpc/platforms/maple/pci.c
 */

// C dependencies supplied by the surrounding kernel translation.

const PA_PXP_CFA: fn(u32, u32, u32) -> u32 = |bus, devfn, off| {
    (bus << 20) | (devfn << 12) | off
};

#[inline]
unsafe fn pa_pxp_offset_valid(bus: u8, devfn: u8, offset: i32) -> i32 {
    /* Device 0 Function 0 is special: Its config space spans function 1 as
     * well, so allow larger offset. It's really a two-function device but the
     * second function does not probe.
     */
    if bus == 0 && devfn == 0 {
        (offset < 8192) as i32
    } else {
        (offset < 4096) as i32
    }
}

unsafe fn pa_pxp_cfg_addr(
    hose: *mut pci_controller,
    bus: u8,
    devfn: u8,
    offset: i32,
) -> *mut core::ffi::c_void {
    (*hose).cfg_data.add(PA_PXP_CFA(bus as u32, devfn as u32, offset as u32) as usize)
}

#[inline]
unsafe fn is_root_port(busno: i32, devfn: i32) -> i32 {
    ((busno == 0) &&
        (PCI_FUNC(devfn) < 4) &&
        ((PCI_SLOT(devfn) == 16) || (PCI_SLOT(devfn) == 17))) as i32
}

#[inline]
fn is_5945_reg(reg: i32) -> i32 {
    (((reg >= 0x18) && (reg < 0x34)) ||
        ((reg >= 0x158) && (reg < 0x178))) as i32
}

unsafe fn workaround_5945(
    bus: *mut pci_bus,
    devfn: u32,
    offset: i32,
    len: i32,
    val: *mut u32,
) -> i32 {
    if is_root_port((*bus).number, devfn as i32) == 0 || is_5945_reg(offset) == 0 {
        return 0;
    }

    let hose = pci_bus_to_host(bus);
    let addr = pa_pxp_cfg_addr(hose, (*bus).number as u8, devfn as u8, offset & !0x3);
    let byte = offset & 0x3;

    /* Workaround bug 5945: write 0 to a dummy register before reading,
     * and write back what we read. We must read/write the full 32-bit
     * contents so we need to shift and mask by hand.
     */
    let dummy = pa_pxp_cfg_addr(hose, (*bus).number as u8, devfn as u8, 0x10);
    out_le32(dummy, 0);
    let tmp = in_le32(addr);
    out_le32(addr, tmp);

    match len {
        1 => *val = (tmp >> (8 * byte)) & 0xff,
        2 => {
            if byte == 0 { *val = tmp & 0xffff; }
            else { *val = (tmp >> 16) & 0xffff; }
        }
        _ => *val = tmp,
    }
    1
}

// Preserved conditional configuration intent: CONFIG_PPC_PASEMI_NEMO.
const PXP_ERR_CFG_REG: usize = 0x4;
const PXP_IGNORE_PCIE_ERRORS: u32 = 0x800;
const SB600_BUS: i32 = 5;

unsafe fn sb600_set_flag(bus: i32) {
    static mut iob_mapbase: *mut u8 = core::ptr::null_mut();

    // The non-NEMO build supplies an empty implementation; external OF/IO
    // symbols are intentionally left as dependencies of the surrounding tree.
    if iob_mapbase != core::ptr::null_mut() {
        let reg = iob_mapbase.add(PXP_ERR_CFG_REG);
        if bus == SB600_BUS {
            out_le32(reg as *mut core::ffi::c_void,
                in_le32(reg as *mut core::ffi::c_void) | PXP_IGNORE_PCIE_ERRORS);
        } else {
            out_le32(reg as *mut core::ffi::c_void,
                in_le32(reg as *mut core::ffi::c_void) & !PXP_IGNORE_PCIE_ERRORS);
        }
    }
}

unsafe fn pa_pxp_read_config(
    bus: *mut pci_bus,
    devfn: u32,
    offset: i32,
    len: i32,
    val: *mut u32,
) -> i32 {
    let hose = pci_bus_to_host(bus);
    if hose.is_null() { return PCIBIOS_DEVICE_NOT_FOUND; }
    if pa_pxp_offset_valid((*bus).number as u8, devfn as u8, offset) == 0 {
        return PCIBIOS_BAD_REGISTER_NUMBER;
    }
    if workaround_5945(bus, devfn, offset, len, val) != 0 {
        return PCIBIOS_SUCCESSFUL;
    }
    let addr = pa_pxp_cfg_addr(hose, (*bus).number as u8, devfn as u8, offset);
    sb600_set_flag((*bus).number);
    match len {
        1 => *val = in_8(addr),
        2 => *val = in_le16(addr),
        _ => *val = in_le32(addr),
    }
    PCIBIOS_SUCCESSFUL
}

unsafe fn pa_pxp_write_config(
    bus: *mut pci_bus,
    devfn: u32,
    offset: i32,
    len: i32,
    val: u32,
) -> i32 {
    let hose = pci_bus_to_host(bus);
    if hose.is_null() { return PCIBIOS_DEVICE_NOT_FOUND; }
    if pa_pxp_offset_valid((*bus).number as u8, devfn as u8, offset) == 0 {
        return PCIBIOS_BAD_REGISTER_NUMBER;
    }
    let addr = pa_pxp_cfg_addr(hose, (*bus).number as u8, devfn as u8, offset);
    sb600_set_flag((*bus).number);
    match len {
        1 => out_8(addr, val),
        2 => out_le16(addr, val),
        _ => out_le32(addr, val),
    }
    PCIBIOS_SUCCESSFUL
}

static mut pa_pxp_ops: pci_ops = pci_ops {
    read: Some(pa_pxp_read_config),
    write: Some(pa_pxp_write_config),
};

unsafe fn setup_pa_pxp(hose: *mut pci_controller) {
    (*hose).ops = &mut pa_pxp_ops;
    (*hose).cfg_data = ioremap(0xe0000000, 0x10000000);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
