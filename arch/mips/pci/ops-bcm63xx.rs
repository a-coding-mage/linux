/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2008 Maxime Bizon <mbizon@freebox.fr>
 */

use core::ffi::c_int;

extern "C" {
    static mut pci_iospace_start: *mut u8;
    fn bcm_mpi_writel(value: u32, reg: u32);
    fn bcm_mpi_readl(reg: u32) -> u32;
    fn bcm_pcie_readl(reg: u32) -> u32;
    fn bcm_pcie_writel(value: u32, reg: u32);
    fn bcm63xx_get_cpu_id() -> u32;
    fn iob();
    fn wmb();
    fn udelay(usecs: u32);
    fn __raw_readl(addr: *mut u8) -> u32;
    fn __raw_writel(value: u32, addr: *mut u8);
}

#[repr(C)]
pub struct pci_bus {
    pub parent: *mut pci_bus,
    pub number: u8,
}

#[repr(C)]
pub struct pci_dev {
    pub bus: *mut pci_bus,
    pub devfn: u32,
}

#[repr(C)]
pub struct pci_ops {
    pub read: Option<unsafe extern "C" fn(*mut pci_bus, u32, c_int, c_int, *mut u32) -> c_int>,
    pub write: Option<unsafe extern "C" fn(*mut pci_bus, u32, c_int, c_int, u32) -> c_int>,
}

extern "C" {
    static PCIBIOS_DEVICE_NOT_FOUND: c_int;
    static PCIBIOS_SUCCESSFUL: c_int;
}

fn postprocess_read(data: u32, where_: c_int, size: u32) -> u32 {
    let mut ret = 0;
    match size {
        1 => ret = (data >> (((where_ as u32 & 3) << 3))) & 0xff,
        2 => ret = (data >> (((where_ as u32 & 3) << 3))) & 0xffff,
        4 => ret = data,
        _ => {}
    }
    ret
}

fn preprocess_write(orig_data: u32, val: u32, where_: c_int, size: u32) -> u32 {
    let mut ret = 0;
    match size {
        1 => ret = (orig_data & !(0xff << ((where_ as u32 & 3) << 3))) |
            (val << ((where_ as u32 & 3) << 3)),
        2 => ret = (orig_data & !(0xffff << ((where_ as u32 & 3) << 3))) |
            (val << ((where_ as u32 & 3) << 3)),
        4 => ret = val,
        _ => {}
    }
    ret
}

unsafe fn bcm63xx_setup_cfg_access(type_: c_int, _busn: u32, devfn: u32, where_: c_int) -> c_int {
    let slot = (devfn >> 3) & 0x1f;
    let func = devfn & 7;
    let reg = where_ as u32 >> 2;
    if slot > (MPI_L2PCFG_DEVNUM_MASK >> MPI_L2PCFG_DEVNUM_SHIFT) ||
       func > (MPI_L2PCFG_FUNC_MASK >> MPI_L2PCFG_FUNC_SHIFT) ||
       reg > (MPI_L2PCFG_REG_MASK >> MPI_L2PCFG_REG_SHIFT) { return 1; }
    let mut val = (reg << MPI_L2PCFG_REG_SHIFT) |
        (func << MPI_L2PCFG_FUNC_SHIFT) |
        (slot << MPI_L2PCFG_DEVNUM_SHIFT) |
        MPI_L2PCFG_CFG_USEREG_MASK | MPI_L2PCFG_CFG_SEL_MASK;
    if type_ != 0 { val |= 1 << MPI_L2PCFG_CFG_TYPE_SHIFT; }
    bcm_mpi_writel(val, MPI_L2PCFG_REG);
    0
}

unsafe fn do_cfg_read(type_: c_int, busn: u32, devfn: u32, where_: c_int, size: c_int, val: *mut u32) -> c_int {
    if bcm63xx_setup_cfg_access(type_, busn, devfn, where_) != 0 { return PCIBIOS_DEVICE_NOT_FOUND; }
    iob();
    let data = u32::from_le(__raw_readl(pci_iospace_start));
    bcm_mpi_writel(0, MPI_L2PCFG_REG);
    *val = postprocess_read(data, where_, size as u32);
    PCIBIOS_SUCCESSFUL
}

unsafe fn do_cfg_write(type_: c_int, busn: u32, devfn: u32, where_: c_int, size: c_int, val: u32) -> c_int {
    if bcm63xx_setup_cfg_access(type_, busn, devfn, where_) != 0 { return PCIBIOS_DEVICE_NOT_FOUND; }
    iob();
    let mut data = u32::from_le(__raw_readl(pci_iospace_start));
    data = preprocess_write(data, val, where_, size as u32);
    __raw_writel(data.to_le(), pci_iospace_start);
    wmb();
    udelay(500);
    bcm_mpi_writel(0, MPI_L2PCFG_REG);
    PCIBIOS_SUCCESSFUL
}

unsafe extern "C" fn bcm63xx_pci_read(bus: *mut pci_bus, devfn: u32, where_: c_int, size: c_int, val: *mut u32) -> c_int {
    let type_ = if (*bus).parent.is_null() { 0 } else { 1 };
    if type_ == 0 && ((devfn >> 3) & 0x1f) == CARDBUS_PCI_IDSEL { return PCIBIOS_DEVICE_NOT_FOUND; }
    do_cfg_read(type_, (*bus).number as u32, devfn, where_, size, val)
}

unsafe extern "C" fn bcm63xx_pci_write(bus: *mut pci_bus, devfn: u32, where_: c_int, size: c_int, val: u32) -> c_int {
    let type_ = if (*bus).parent.is_null() { 0 } else { 1 };
    if type_ == 0 && ((devfn >> 3) & 0x1f) == CARDBUS_PCI_IDSEL { return PCIBIOS_DEVICE_NOT_FOUND; }
    do_cfg_write(type_, (*bus).number as u32, devfn, where_, size, val)
}

pub static mut bcm63xx_pci_ops: pci_ops = pci_ops { read: Some(bcm63xx_pci_read), write: Some(bcm63xx_pci_write) };

/* The CONFIG_CARDBUS section is retained below in the source-level translation. */

unsafe fn pcie_can_access(bus: *mut pci_bus, devfn: c_int) -> bool {
    match (*bus).number as c_int {
        PCIE_BUS_BRIDGE => ((devfn >> 3) & 0x1f) == 0,
        PCIE_BUS_DEVICE => {
            if ((devfn >> 3) & 0x1f) == 0 { bcm_pcie_readl(PCIE_DLSTATUS_REG) & DLSTATUS_PHYLINKUP != 0 } else { false }
        }
        _ => false,
    }
}

unsafe extern "C" fn bcm63xx_pcie_read(bus: *mut pci_bus, devfn: u32, where_: c_int, size: c_int, val: *mut u32) -> c_int {
    if !pcie_can_access(bus, devfn as c_int) { return PCIBIOS_DEVICE_NOT_FOUND; }
    let mut reg = where_ as u32 & !3;
    if (*bus).number as c_int == PCIE_BUS_DEVICE { reg += PCIE_DEVICE_OFFSET; }
    *val = postprocess_read(bcm_pcie_readl(reg), where_, size as u32);
    PCIBIOS_SUCCESSFUL
}

unsafe extern "C" fn bcm63xx_pcie_write(bus: *mut pci_bus, devfn: u32, where_: c_int, size: c_int, val: u32) -> c_int {
    if !pcie_can_access(bus, devfn as c_int) { return PCIBIOS_DEVICE_NOT_FOUND; }
    let mut reg = where_ as u32 & !3;
    if (*bus).number as c_int == PCIE_BUS_DEVICE { reg += PCIE_DEVICE_OFFSET; }
    let data = preprocess_write(bcm_pcie_readl(reg), val, where_, size as u32);
    bcm_pcie_writel(data, reg);
    PCIBIOS_SUCCESSFUL
}

pub static mut bcm63xx_pcie_ops: pci_ops = pci_ops { read: Some(bcm63xx_pcie_read), write: Some(bcm63xx_pcie_write) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
