// SPDX-License-Identifier: GPL-2.0-or-later
/* Broadcom SATA3 AHCI Controller Driver */

// Linux dependencies supplied by the surrounding translation unit.

const DRV_NAME: &str = "brcm-ahci";
const SATA_TOP_CTRL_BUS_CTRL: usize = 0x4;
const DMADESC_ENDIAN_SHIFT: u32 = 2;
const DMADATA_ENDIAN_SHIFT: u32 = 4;
const MMIO_ENDIAN_SHIFT: u32 = 0;
const PIODATA_ENDIAN_SHIFT: u32 = 6;
const ENDIAN_SWAP_FULL: u32 = 2;
const SATA_TOP_CTRL_PHY_CTRL: usize = 0xc;
const SATA_TOP_CTRL_PHY_CTRL_1: usize = 0x0;
const SATA_TOP_CTRL_1_PHY_DEFAULT_POWER_STATE: u32 = 1 << 14;
const SATA_TOP_CTRL_PHY_CTRL_2: usize = 0x4;
const SATA_TOP_CTRL_2_SW_RST_MDIOREG: u32 = 1;
const SATA_TOP_CTRL_2_SW_RST_OOB: u32 = 1 << 1;
const SATA_TOP_CTRL_2_SW_RST_RX: u32 = 1 << 2;
const SATA_TOP_CTRL_2_SW_RST_TX: u32 = 1 << 3;
const SATA_TOP_CTRL_2_PHY_GLOBAL_RESET: u32 = 1 << 14;
const SATA_TOP_CTRL_PHY_OFFS: usize = 0x8;
const SATA_TOP_MAX_PHYS: i32 = 2;
const SATA_FIRST_PORT_CTRL: usize = 0x700;
const SATA_NEXT_PORT_CTRL_OFFSET: usize = 0x80;
const BUS_CTRL_ENDIAN_CONF_MASK: u32 = (0x3 << MMIO_ENDIAN_SHIFT) | (0x3 << DMADESC_ENDIAN_SHIFT) | (0x3 << DMADATA_ENDIAN_SHIFT) | (0x3 << PIODATA_ENDIAN_SHIFT);
const DATA_ENDIAN: u32 = 0;
const MMIO_ENDIAN: u32 = 0;
const BUS_CTRL_ENDIAN_CONF: u32 = (DATA_ENDIAN << DMADATA_ENDIAN_SHIFT) | (DATA_ENDIAN << DMADESC_ENDIAN_SHIFT) | (MMIO_ENDIAN << MMIO_ENDIAN_SHIFT);
const BUS_CTRL_ENDIAN_NSP_CONF: u32 = (0x02 << DMADATA_ENDIAN_SHIFT) | (0x02 << DMADESC_ENDIAN_SHIFT);

#[repr(C)]
pub enum BrcmAhciVersion { BrcmSataBcm7425 = 1, BrcmSataBcm7445, BrcmSataNsp, BrcmSataBcm7216 }
const BRCM_AHCI_QUIRK_SKIP_PHY_ENABLE: u32 = 1 << 0;

#[repr(C)]
pub struct BrcmAhciPriv {
    pub dev: *mut device,
    pub top_ctrl: *mut core::ffi::c_void,
    pub port_mask: u32,
    pub quirks: u32,
    pub version: BrcmAhciVersion,
    pub rcdev_rescal: *mut reset_control,
    pub rcdev_ahci: *mut reset_control,
}

unsafe fn brcm_sata_readreg(addr: *mut core::ffi::c_void) -> u32 {
    if IS_ENABLED(CONFIG_MIPS) && IS_ENABLED(CONFIG_CPU_BIG_ENDIAN) { __raw_readl(addr) } else { readl_relaxed(addr) }
}
unsafe fn brcm_sata_writereg(val: u32, addr: *mut core::ffi::c_void) {
    if IS_ENABLED(CONFIG_MIPS) && IS_ENABLED(CONFIG_CPU_BIG_ENDIAN) { __raw_writel(val, addr) } else { writel_relaxed(val, addr) }
}

unsafe fn brcm_sata_alpm_init(hpriv: *mut ahci_host_priv) {
    let priv_ = (*hpriv).plat_data as *mut BrcmAhciPriv;
    let mut port_ctrl = SATA_FIRST_PORT_CTRL;
    let host_caps = readl((*hpriv).mmio.add(HOST_CAP));
    if host_caps & HOST_CAP_ALPM == 0 { (*hpriv).flags |= AHCI_HFLAG_YES_ALPM; }
    for i in 0..SATA_TOP_MAX_PHYS { if (*priv_).port_mask & (1 << i) != 0 { writel(0xff1003fc, (*hpriv).mmio.add(port_ctrl + 0x18)); } port_ctrl += SATA_NEXT_PORT_CTRL_OFFSET; }
}

unsafe fn brcm_sata_phy_enable(priv_: *mut BrcmAhciPriv, port: i32) {
    let phyctrl = (*priv_).top_ctrl.add(SATA_TOP_CTRL_PHY_CTRL + (port as usize) * SATA_TOP_CTRL_PHY_OFFS);
    if (*priv_).quirks & BRCM_AHCI_QUIRK_SKIP_PHY_ENABLE != 0 { return; }
    let p = phyctrl.add(SATA_TOP_CTRL_PHY_CTRL_1); let mut reg = brcm_sata_readreg(p); brcm_sata_writereg(reg & !SATA_TOP_CTRL_1_PHY_DEFAULT_POWER_STATE, p);
    let p = phyctrl.add(SATA_TOP_CTRL_PHY_CTRL_2); reg = brcm_sata_readreg(p); reg &= !(SATA_TOP_CTRL_2_SW_RST_MDIOREG | SATA_TOP_CTRL_2_SW_RST_OOB | SATA_TOP_CTRL_2_SW_RST_RX); reg |= SATA_TOP_CTRL_2_SW_RST_TX; brcm_sata_writereg(reg, p); reg = brcm_sata_readreg(p); reg |= SATA_TOP_CTRL_2_PHY_GLOBAL_RESET; brcm_sata_writereg(reg,p); reg = brcm_sata_readreg(p); reg &= !SATA_TOP_CTRL_2_PHY_GLOBAL_RESET; brcm_sata_writereg(reg,p); let _ = brcm_sata_readreg(p);
}
unsafe fn brcm_sata_phy_disable(priv_: *mut BrcmAhciPriv, port: i32) {
    let phyctrl = (*priv_).top_ctrl.add(SATA_TOP_CTRL_PHY_CTRL + (port as usize) * SATA_TOP_CTRL_PHY_OFFS); if (*priv_).quirks & BRCM_AHCI_QUIRK_SKIP_PHY_ENABLE != 0 { return; }
    let p = phyctrl.add(SATA_TOP_CTRL_PHY_CTRL_2); let mut reg = brcm_sata_readreg(p); reg |= SATA_TOP_CTRL_2_SW_RST_MDIOREG | SATA_TOP_CTRL_2_SW_RST_OOB | SATA_TOP_CTRL_2_SW_RST_RX | SATA_TOP_CTRL_2_SW_RST_TX | SATA_TOP_CTRL_2_PHY_GLOBAL_RESET; brcm_sata_writereg(reg,p); let p = phyctrl.add(SATA_TOP_CTRL_PHY_CTRL_1); reg=brcm_sata_readreg(p); brcm_sata_writereg(reg | SATA_TOP_CTRL_1_PHY_DEFAULT_POWER_STATE,p);
}
unsafe fn brcm_sata_phys_enable(p: *mut BrcmAhciPriv) { for i in 0..SATA_TOP_MAX_PHYS { if (*p).port_mask & (1<<i) != 0 { brcm_sata_phy_enable(p,i); } } }
unsafe fn brcm_sata_phys_disable(p: *mut BrcmAhciPriv) { for i in 0..SATA_TOP_MAX_PHYS { if (*p).port_mask & (1<<i) != 0 { brcm_sata_phy_disable(p,i); } } }

unsafe fn brcm_sata_init(priv_: *mut BrcmAhciPriv) { let ctrl=(*priv_).top_ctrl.add(SATA_TOP_CTRL_BUS_CTRL); let mut data=brcm_sata_readreg(ctrl); data &= !BUS_CTRL_ENDIAN_CONF_MASK; data |= if (*priv_).version as u32 == BrcmAhciVersion::BrcmSataNsp as u32 { BUS_CTRL_ENDIAN_NSP_CONF } else { BUS_CTRL_ENDIAN_CONF }; brcm_sata_writereg(data,ctrl); }

// Remaining driver callbacks and registration are declarations translated from the C implementation;
// their kernel ABI definitions are supplied by the surrounding repository.
extern "C" { pub fn brcm_ahci_probe(pdev: *mut platform_device) -> i32; pub fn brcm_ahci_remove(pdev: *mut platform_device); pub fn brcm_ahci_shutdown(pdev: *mut platform_device); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
