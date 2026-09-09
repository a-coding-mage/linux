/*
 * arch/arm/plat-orion/pcie.c
 *
 * Marvell Orion SoC PCIe handling.
 *
 * This file is licensed under the terms of the GNU General Public
 * License version 2.  This program is licensed "as is" without any
 * warranty of any kind, whether express or implied.
 */

// External Linux/kernel, PCI, MBUS, and platform symbols are supplied by dependencies.

const PCIE_DEV_ID_OFF: usize = 0x0000;
const PCIE_CMD_OFF: usize = 0x0004;
const PCIE_DEV_REV_OFF: usize = 0x0008;
const PCIE_HEADER_LOG_4_OFF: usize = 0x0128;
const PCIE_WIN5_CTRL_OFF: usize = 0x1880;
const PCIE_WIN5_BASE_OFF: usize = 0x1884;
const PCIE_WIN5_REMAP_OFF: usize = 0x188c;
const PCIE_CONF_ADDR_OFF: usize = 0x18f8;
const PCIE_CONF_ADDR_EN: u32 = 0x80000000;
const PCIE_CONF_DATA_OFF: usize = 0x18fc;
const PCIE_MASK_OFF: usize = 0x1910;
const PCIE_CTRL_OFF: usize = 0x1a00;
const PCIE_CTRL_X1_MODE: u32 = 0x0001;
const PCIE_STAT_OFF: usize = 0x1a04;
const PCIE_STAT_DEV_OFFS: u32 = 20;
const PCIE_STAT_DEV_MASK: u32 = 0x1f;
const PCIE_STAT_BUS_OFFS: u32 = 8;
const PCIE_STAT_BUS_MASK: u32 = 0xff;
const PCIE_STAT_LINK_DOWN: u32 = 1;
const PCIE_DEBUG_CTRL: usize = 0x1a60;
const PCIE_DEBUG_SOFT_RESET: u32 = 1 << 20;

const fn pcie_bar_lo_off(n: usize) -> usize { 0x0010 + (n << 3) }
const fn pcie_bar_hi_off(n: usize) -> usize { 0x0014 + (n << 3) }
const fn pcie_bar_ctrl_off(n: usize) -> usize { 0x1804 + ((n - 1) * 4) }
const fn pcie_win04_ctrl_off(n: usize) -> usize { 0x1820 + (n << 4) }
const fn pcie_win04_base_off(n: usize) -> usize { 0x1824 + (n << 4) }
const fn pcie_win04_remap_off(n: usize) -> usize { 0x182c + (n << 4) }
const fn pcie_conf_reg(r: u32) -> u32 { ((r & 0xf00) << 16) | (r & 0xfc) }
const fn pcie_conf_bus(b: u32) -> u32 { (b & 0xff) << 16 }
const fn pcie_conf_dev(d: u32) -> u32 { (d & 0x1f) << 11 }
const fn pcie_conf_func(f: u32) -> u32 { (f & 0x7) << 8 }

extern "C" {
    fn readl(addr: *const u8) -> u32;
    fn readw(addr: *const u8) -> u16;
    fn writel(value: u32, addr: *mut u8);
    fn writew(value: u16, addr: *mut u8);
    fn writeb(value: u8, addr: *mut u8);
    fn mdelay(ms: u32);
    fn mv_mbus_dram_info() -> *const MbusDramTargetInfo;
    fn fls(x: u32) -> u32;
}

#[repr(C)]
pub struct MbusDramWindow { pub base: u32, pub size: u32, pub mbus_attr: u32 }
#[repr(C)]
pub struct MbusDramTargetInfo {
    pub num_cs: usize,
    pub cs: *const MbusDramWindow,
    pub mbus_dram_target_id: u32,
}
#[repr(C)]
pub struct PciBus { pub number: u32 }

const PCI_COMMAND_IO: u16 = 1;
const PCI_COMMAND_MEMORY: u16 = 2;
const PCI_COMMAND_MASTER: u16 = 4;
const PCIBIOS_SUCCESSFUL: i32 = 0;
const PCIBIOS_BAD_REGISTER_NUMBER: i32 = 0x86;

#[inline] fn pci_slot(devfn: u32) -> u32 { (devfn >> 3) & 0x1f }
#[inline] fn pci_func(devfn: u32) -> u32 { devfn & 0x07 }

pub unsafe fn orion_pcie_dev_id(base: *mut u8) -> u32 { readl(base.add(PCIE_DEV_ID_OFF)) >> 16 }
pub unsafe fn orion_pcie_rev(base: *mut u8) -> u32 { readl(base.add(PCIE_DEV_REV_OFF)) & 0xff }
pub unsafe fn orion_pcie_link_up(base: *mut u8) -> i32 { if readl(base.add(PCIE_STAT_OFF)) & PCIE_STAT_LINK_DOWN == 0 { 1 } else { 0 } }
pub unsafe fn orion_pcie_x4_mode(base: *mut u8) -> i32 { if readl(base.add(PCIE_CTRL_OFF)) & PCIE_CTRL_X1_MODE == 0 { 1 } else { 0 } }
pub unsafe fn orion_pcie_get_local_bus_nr(base: *mut u8) -> u32 { (readl(base.add(PCIE_STAT_OFF)) >> PCIE_STAT_BUS_OFFS) & PCIE_STAT_BUS_MASK }

pub unsafe fn orion_pcie_set_local_bus_nr(base: *mut u8, nr: i32) {
    let mut stat = readl(base.add(PCIE_STAT_OFF));
    stat &= !(PCIE_STAT_BUS_MASK << PCIE_STAT_BUS_OFFS);
    stat |= (nr as u32) << PCIE_STAT_BUS_OFFS;
    writel(stat, base.add(PCIE_STAT_OFF));
}

pub unsafe fn orion_pcie_reset(base: *mut u8) {
    /*
     * MV-S104860-U0, Rev. C:
     * PCI Express Unit Soft Reset
     * When set, generates an internal reset in the PCI Express unit.
     * This bit should be cleared after the link is re-established.
     */
    let mut reg = readl(base.add(PCIE_DEBUG_CTRL));
    reg |= PCIE_DEBUG_SOFT_RESET;
    writel(reg, base.add(PCIE_DEBUG_CTRL));
    for _i in 0..20 {
        mdelay(10);
        if orion_pcie_link_up(base) != 0 { break; }
    }
    reg &= !PCIE_DEBUG_SOFT_RESET;
    writel(reg, base.add(PCIE_DEBUG_CTRL));
}

/*
 * Setup PCIE BARs and Address Decode Wins:
 * BAR[0,2] -> disabled, BAR[1] -> covers all DRAM banks
 * WIN[0-3] -> DRAM bank[0-3]
 */
unsafe fn orion_pcie_setup_wins(base: *mut u8) {
    let dram = &*mv_mbus_dram_info();
    let mut size: u32 = 0;
    for i in 1..=2 {
        writel(0, base.add(pcie_bar_ctrl_off(i))); writel(0, base.add(pcie_bar_lo_off(i))); writel(0, base.add(pcie_bar_hi_off(i)));
    }
    for i in 0..5 { writel(0, base.add(pcie_win04_ctrl_off(i))); writel(0, base.add(pcie_win04_base_off(i))); writel(0, base.add(pcie_win04_remap_off(i))); }
    writel(0, base.add(PCIE_WIN5_CTRL_OFF)); writel(0, base.add(PCIE_WIN5_BASE_OFF)); writel(0, base.add(PCIE_WIN5_REMAP_OFF));
    for i in 0..dram.num_cs {
        let cs = &*dram.cs.add(i);
        writel(cs.base & 0xffff0000, base.add(pcie_win04_base_off(i)));
        writel(0, base.add(pcie_win04_remap_off(i)));
        writel(((cs.size - 1) & 0xffff0000) | (cs.mbus_attr << 8) | (dram.mbus_dram_target_id << 4) | 1, base.add(pcie_win04_ctrl_off(i)));
        size += cs.size;
    }
    if size & (size - 1) != 0 { size = 1 << fls(size); }
    writel(dram.cs.read().base, base.add(pcie_bar_lo_off(1)));
    writel(0, base.add(pcie_bar_hi_off(1)));
    writel(((size - 1) & 0xffff0000) | 1, base.add(pcie_bar_ctrl_off(1)));
}

pub unsafe fn orion_pcie_setup(base: *mut u8) {
    orion_pcie_setup_wins(base);
    let mut cmd = readw(base.add(PCIE_CMD_OFF));
    cmd |= PCI_COMMAND_IO | PCI_COMMAND_MEMORY | PCI_COMMAND_MASTER;
    writew(cmd, base.add(PCIE_CMD_OFF));
    let mut mask = readl(base.add(PCIE_MASK_OFF)); mask |= 0x0f000000; writel(mask, base.add(PCIE_MASK_OFF));
}

pub unsafe fn orion_pcie_rd_conf(base: *mut u8, bus: *mut PciBus, devfn: u32, where_: i32, size: i32, val: *mut u32) -> i32 {
    writel(pcie_conf_bus((*bus).number) | pcie_conf_dev(pci_slot(devfn)) | pcie_conf_func(pci_func(devfn)) | pcie_conf_reg(where_ as u32) | PCIE_CONF_ADDR_EN, base.add(PCIE_CONF_ADDR_OFF));
    *val = readl(base.add(PCIE_CONF_DATA_OFF));
    if size == 1 { *val = (*val >> (8 * ((where_ as u32) & 3))) & 0xff; } else if size == 2 { *val = (*val >> (8 * ((where_ as u32) & 3))) & 0xffff; }
    PCIBIOS_SUCCESSFUL
}

pub unsafe fn orion_pcie_rd_conf_tlp(base: *mut u8, bus: *mut PciBus, devfn: u32, where_: i32, size: i32, val: *mut u32) -> i32 {
    writel(pcie_conf_bus((*bus).number) | pcie_conf_dev(pci_slot(devfn)) | pcie_conf_func(pci_func(devfn)) | pcie_conf_reg(where_ as u32) | PCIE_CONF_ADDR_EN, base.add(PCIE_CONF_ADDR_OFF));
    *val = readl(base.add(PCIE_CONF_DATA_OFF));
    if (*bus).number != orion_pcie_get_local_bus_nr(base) || pci_func(devfn) != 0 { *val = readl(base.add(PCIE_HEADER_LOG_4_OFF)); }
    if size == 1 { *val = (*val >> (8 * ((where_ as u32) & 3))) & 0xff; } else if size == 2 { *val = (*val >> (8 * ((where_ as u32) & 3))) & 0xffff; }
    PCIBIOS_SUCCESSFUL
}

pub unsafe fn orion_pcie_rd_conf_wa(wa_base: *mut u8, bus: *mut PciBus, devfn: u32, where_: i32, size: i32, val: *mut u32) -> i32 {
    let off = pcie_conf_bus((*bus).number) | pcie_conf_dev(pci_slot(devfn)) | pcie_conf_func(pci_func(devfn)) | pcie_conf_reg(where_ as u32);
    *val = readl(wa_base.add(off as usize));
    if size == 1 { *val = (*val >> (8 * ((where_ as u32) & 3))) & 0xff; } else if size == 2 { *val = (*val >> (8 * ((where_ as u32) & 3))) & 0xffff; }
    PCIBIOS_SUCCESSFUL
}

pub unsafe fn orion_pcie_wr_conf(base: *mut u8, bus: *mut PciBus, devfn: u32, where_: i32, size: i32, val: u32) -> i32 {
    let mut ret = PCIBIOS_SUCCESSFUL;
    writel(pcie_conf_bus((*bus).number) | pcie_conf_dev(pci_slot(devfn)) | pcie_conf_func(pci_func(devfn)) | pcie_conf_reg(where_ as u32) | PCIE_CONF_ADDR_EN, base.add(PCIE_CONF_ADDR_OFF));
    if size == 4 { writel(val, base.add(PCIE_CONF_DATA_OFF)); } else if size == 2 { writew(val as u16, base.add(PCIE_CONF_DATA_OFF + (where_ as usize & 3))); } else if size == 1 { writeb(val as u8, base.add(PCIE_CONF_DATA_OFF + (where_ as usize & 3))); } else { ret = PCIBIOS_BAD_REGISTER_NUMBER; }
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
