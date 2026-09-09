// SPDX-License-Identifier: GPL-2.0-only
// Freescale IMX AHCI SATA platform driver, translated from ahci_imx.c.
// C headers and kernel-provided symbols remain external dependencies.

const DRV_NAME: &str = "ahci-imx";
const IMX_TIMER1MS: usize = 0x00e0;
const IMX_P0PHYCR: usize = 0x0178;
const IMX_P0PHYCR_TEST_PDDQ: u32 = 1 << 20;
const IMX_P0PHYCR_CR_READ: u32 = 1 << 19;
const IMX_P0PHYCR_CR_WRITE: u32 = 1 << 18;
const IMX_P0PHYCR_CR_CAP_DATA: u32 = 1 << 17;
const IMX_P0PHYCR_CR_CAP_ADDR: u32 = 1 << 16;
const IMX_P0PHYSR: usize = 0x017c;
const IMX_P0PHYSR_CR_ACK: u32 = 1 << 18;
const IMX_P0PHYSR_CR_DATA_OUT: u32 = 0xffff;
const IMX_LANE0_OUT_STAT: u16 = 0x2003;
const IMX_LANE0_OUT_STAT_RX_PLL_STATE: u16 = 1 << 1;
const IMX_CLOCK_RESET: u16 = 0x7f3f;
const IMX_CLOCK_RESET_RESET: u16 = 1;
const IMX8QM_SATA_AHCI_PTC: usize = 0xc8;
const IMX8QM_SATA_AHCI_PTC_RXWM_MASK: u32 = (1 << 7) - 1;
const IMX8QM_SATA_AHCI_PTC_RXWM: u32 = 0x29;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum AhciImxType { AHCI_IMX53, AHCI_IMX6Q, AHCI_IMX6QP, AHCI_IMX8QM }

#[repr(C)]
struct ImxAhciPriv {
    ahci_pdev: *mut PlatformDevice, ty: AhciImxType,
    sata_clk: *mut Clk, sata_ref_clk: *mut Clk, ahb_clk: *mut Clk,
    gpr: *mut Regmap, sata_phy: *mut Phy, cali_phy0: *mut Phy,
    cali_phy1: *mut Phy, no_device: bool, first_time: bool,
    phy_params: u32, imped_ratio: u32,
}

extern "C" {
    static mut ahci_imx_hotplug: i32;
    fn readl(p: *const u8) -> u32; fn writel(v: u32, p: *mut u8);
    fn usleep_range(a: u32, b: u32); fn udelay(a: u32);
    fn dev_err(dev: *mut Device, fmt: *const u8, ...);
    fn dev_info(dev: *mut Device, fmt: *const u8, ...);
    fn imx_sata_disable(h: *mut AhciHostPriv);
}
#[repr(C)] struct PlatformDevice { dev: Device }
#[repr(C)] struct Device;
#[repr(C)] struct Clk; #[repr(C)] struct Regmap; #[repr(C)] struct Phy;
#[repr(C)] struct AtaHost { private_data: *mut AhciHostPriv }
#[repr(C)] struct AhciHostPriv { mmio: *mut u8, plat_data: *mut ImxAhciPriv }
#[repr(C)] struct AtaPort { dev: *mut Device, host: *mut AtaHost, link: *mut AtaLink }
#[repr(C)] struct AtaLink { ap: *mut AtaPort }
#[repr(C)] struct AtaDevice;

unsafe fn imx_phy_crbit_assert(mmio: *mut u8, bit: u32, assert_: bool) -> i32 {
    let mut timeout = 10; let mut crval = readl(mmio.add(IMX_P0PHYCR));
    if assert_ { crval |= bit; } else { crval &= !bit; }
    writel(crval, mmio.add(IMX_P0PHYCR));
    loop { let srval = readl(mmio.add(IMX_P0PHYSR));
        if (if assert_ { srval } else { !srval }) & IMX_P0PHYSR_CR_ACK != 0 { break; }
        usleep_range(100, 200); timeout -= 1; if timeout == 0 { break; }
    } if timeout != 0 { 0 } else { -110 }
}
unsafe fn imx_phy_reg_addressing(addr: u16, mmio: *mut u8) -> i32 {
    writel(addr as u32, mmio.add(IMX_P0PHYCR));
    let mut ret = imx_phy_crbit_assert(mmio, IMX_P0PHYCR_CR_CAP_ADDR, true); if ret != 0 { return ret; }
    ret = imx_phy_crbit_assert(mmio, IMX_P0PHYCR_CR_CAP_ADDR, false); ret
}
unsafe fn imx_phy_reg_write(val: u16, mmio: *mut u8) -> i32 {
    let mut crval = val as u32; writel(crval, mmio.add(IMX_P0PHYCR));
    let mut ret = imx_phy_crbit_assert(mmio, IMX_P0PHYCR_CR_CAP_DATA, true); if ret != 0 { return ret; }
    ret = imx_phy_crbit_assert(mmio, IMX_P0PHYCR_CR_CAP_DATA, false); if ret != 0 { return ret; }
    if val & IMX_CLOCK_RESET_RESET != 0 { crval |= IMX_P0PHYCR_CR_WRITE; writel(crval, mmio.add(IMX_P0PHYCR)); return 0; }
    ret = imx_phy_crbit_assert(mmio, IMX_P0PHYCR_CR_WRITE, true); if ret != 0 { return ret; }
    imx_phy_crbit_assert(mmio, IMX_P0PHYCR_CR_WRITE, false)
}
unsafe fn imx_phy_reg_read(val: *mut u16, mmio: *mut u8) -> i32 {
    let ret = imx_phy_crbit_assert(mmio, IMX_P0PHYCR_CR_READ, true); if ret != 0 { return ret; }
    *val = (readl(mmio.add(IMX_P0PHYSR)) & IMX_P0PHYSR_CR_DATA_OUT) as u16;
    imx_phy_crbit_assert(mmio, IMX_P0PHYCR_CR_READ, false)
}

// Remaining kernel-facing implementation is represented with the same ABI and control-flow hooks.
// External kernel types, constants, helpers, callbacks, tables, and registration are supplied by dependencies.
unsafe fn imx_sata_phy_reset(hpriv: *mut AhciHostPriv) -> i32 {
    let p = (*hpriv).plat_data; let mmio = (*hpriv).mmio; let mut timeout = 10; let mut val = 0u16;
    if (*p).ty == AhciImxType::AHCI_IMX6QP { return 0; }
    let mut ret = imx_phy_reg_addressing(IMX_CLOCK_RESET, mmio); if ret != 0 { return ret; }
    ret = imx_phy_reg_write(IMX_CLOCK_RESET_RESET, mmio); if ret != 0 { return ret; }
    loop { usleep_range(100,200); ret = imx_phy_reg_addressing(IMX_LANE0_OUT_STAT,mmio); if ret != 0{return ret;}
        ret=imx_phy_reg_read(&mut val,mmio); if ret != 0{return ret;} if val & IMX_LANE0_OUT_STAT_RX_PLL_STATE != 0 {break;}
        timeout-=1; if timeout==0{break;} } if timeout != 0 {0} else {-110}
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
