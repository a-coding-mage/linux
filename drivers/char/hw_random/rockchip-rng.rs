// SPDX-License-Identifier: GPL-2.0
/* True Random Number Generator driver for Rockchip SoCs. */

const RK_RNG_AUTOSUSPEND_DELAY: u32 = 100;
const RK_RNG_MAX_BYTE: usize = 32;
const RK_RNG_POLL_PERIOD_US: u32 = 100;
const RK_RNG_POLL_TIMEOUT_US: u32 = 10000;
const RK_RNG_SAMPLE_CNT: u32 = 1000;
const RK_TRNG_V1_AUTO_RESEED_CNT: u32 = 16000;

const TRNG_RST_CTL: u32 = 0x0004;
const TRNG_RNG_CTL: u32 = 0x0400;
const TRNG_RNG_CTL_LEN_64_BIT: u32 = 0x00 << 4;
const TRNG_RNG_CTL_LEN_128_BIT: u32 = 0x01 << 4;
const TRNG_RNG_CTL_LEN_192_BIT: u32 = 0x02 << 4;
const TRNG_RNG_CTL_LEN_256_BIT: u32 = 0x03 << 4;
const TRNG_RNG_CTL_OSC_RING_SPEED_0: u32 = 0x00 << 2;
const TRNG_RNG_CTL_OSC_RING_SPEED_1: u32 = 0x01 << 2;
const TRNG_RNG_CTL_OSC_RING_SPEED_2: u32 = 0x02 << 2;
const TRNG_RNG_CTL_OSC_RING_SPEED_3: u32 = 0x03 << 2;
const TRNG_RNG_CTL_MASK: u32 = 0xffff;
const TRNG_RNG_CTL_ENABLE: u32 = 1 << 1;
const TRNG_RNG_CTL_START: u32 = 1;
const TRNG_RNG_SAMPLE_CNT: u32 = 0x0404;
const TRNG_RNG_DOUT: u32 = 0x0410;

const TRNG_V1_CTRL: u32 = 0x0000;
const TRNG_V1_CTRL_NOP: u32 = 0x00;
const TRNG_V1_CTRL_RAND: u32 = 0x01;
const TRNG_V1_CTRL_SEED: u32 = 0x02;
const TRNG_V1_STAT: u32 = 0x0004;
const TRNG_V1_STAT_SEEDED: u32 = 1 << 9;
const TRNG_V1_STAT_GENERATING: u32 = 1 << 30;
const TRNG_V1_STAT_RESEEDING: u32 = 1 << 31;
const TRNG_V1_MODE: u32 = 0x0008;
const TRNG_V1_MODE_128_BIT: u32 = 0x00 << 3;
const TRNG_V1_MODE_256_BIT: u32 = 0x01 << 3;
const TRNG_V1_IE: u32 = 0x0010;
const TRNG_V1_IE_GLBL_EN: u32 = 1 << 31;
const TRNG_V1_IE_SEED_DONE_EN: u32 = 1 << 1;
const TRNG_V1_IE_RAND_RDY_EN: u32 = 1;
const TRNG_V1_ISTAT: u32 = 0x0014;
const TRNG_V1_ISTAT_RAND_RDY: u32 = 1;
const TRNG_V1_RAND0: u32 = 0x0020;
const TRNG_V1_RAND7: u32 = 0x003c;
const TRNG_V1_AUTO_RQSTS: u32 = 0x0060;
const TRNG_V1_VERSION: u32 = 0x00f0;
const TRNG_V1_VERSION_CODE: u32 = 0x46bc;

const RKRNG_CFG: u32 = 0x0000;
const RKRNG_CTRL: u32 = 0x0010;
const RKRNG_CTRL_REQ_TRNG: u32 = 1 << 4;
const RKRNG_STATE: u32 = 0x0014;
const RKRNG_STATE_TRNG_RDY: u32 = 1 << 4;
const RKRNG_TRNG_DATA0: u32 = 0x0050;
const RKRNG_TRNG_DATA1: u32 = 0x0054;
const RKRNG_TRNG_DATA2: u32 = 0x0058;
const RKRNG_TRNG_DATA3: u32 = 0x005c;
const RKRNG_TRNG_DATA4: u32 = 0x0060;
const RKRNG_TRNG_DATA5: u32 = 0x0064;
const RKRNG_TRNG_DATA6: u32 = 0x0068;
const RKRNG_TRNG_DATA7: u32 = 0x006c;
const RKRNG_READ_LEN: usize = 32;

#[repr(C)]
pub struct RkRng {
    pub rng: Hwrng,
    pub base: *mut core::ffi::c_void,
    pub clk_num: i32,
    pub clk_bulks: *mut ClkBulkData,
    pub soc_data: *const RkRngSocData,
    pub dev: *mut Device,
}

#[repr(C)]
pub struct RkRngSocData {
    pub rk_rng_init: Option<unsafe extern "C" fn(*mut Hwrng) -> i32>,
    pub rk_rng_read: Option<unsafe extern "C" fn(*mut Hwrng, *mut core::ffi::c_void, usize, bool) -> i32>,
    pub rk_rng_cleanup: Option<unsafe extern "C" fn(*mut Hwrng)>,
    pub quality: u16,
    pub reset_optional: bool,
}

// External kernel types and helpers are supplied by the surrounding kernel bindings.
#[repr(C)] pub struct Hwrng { pub name: *const u8, pub init: Option<unsafe extern "C" fn(*mut Hwrng) -> i32>, pub cleanup: Option<unsafe extern "C" fn(*mut Hwrng)>, pub read: Option<unsafe extern "C" fn(*mut Hwrng, *mut core::ffi::c_void, usize, bool) -> i32>, pub quality: u16 }
#[repr(C)] pub struct ClkBulkData;
#[repr(C)] pub struct Device;
#[repr(C)] pub struct PlatformDevice { pub dev: Device }
#[repr(C)] pub struct ResetControl;

extern "C" {
    fn writel(value: u32, addr: *mut core::ffi::c_void);
    fn readl(addr: *mut core::ffi::c_void) -> u32;
    fn clk_bulk_prepare_enable(n: i32, clks: *mut ClkBulkData) -> i32;
    fn clk_bulk_disable_unprepare(n: i32, clks: *mut ClkBulkData);
    fn dev_err(dev: *mut Device, fmt: *const u8, ...);
    fn pm_runtime_resume_and_get(dev: *mut Device) -> i32;
    fn pm_runtime_put_sync_autosuspend(dev: *mut Device);
    fn udelay(usecs: u32);
    fn memcpy_fromio(dst: *mut core::ffi::c_void, src: *mut core::ffi::c_void, n: usize);
    fn readl_poll_timeout(addr: *mut core::ffi::c_void, val: *mut u32, cond: bool, delay: u32, timeout: u32) -> i32;
    fn readl_poll_timeout_atomic(addr: *mut core::ffi::c_void, val: *mut u32, cond: bool, delay: u32, timeout: u32) -> i32;
}

unsafe fn rng_from_hwrng(rng: *mut Hwrng) -> *mut RkRng { rng.cast::<u8>().sub(core::mem::offset_of!(RkRng, rng)).cast() }
unsafe fn rk_rng_write_ctl(rng: *mut RkRng, val: u32, mask: u32) { writel((mask << 16) | val, (*rng).base.add(TRNG_RNG_CTL as usize)); }
unsafe fn rk_rng_writel(rng: *mut RkRng, val: u32, offset: u32) { writel(val, (*rng).base.add(offset as usize)); }
unsafe fn rk_rng_readl(rng: *mut RkRng, offset: u32) -> u32 { readl((*rng).base.add(offset as usize)) }

unsafe fn rk_rng_enable_clks(rng: *mut RkRng) -> i32 {
    let ret = clk_bulk_prepare_enable((*rng).clk_num, (*rng).clk_bulks);
    if ret < 0 { dev_err((*rng).dev, b"Failed to enable clocks: %d\0".as_ptr(), ret); }
    ret
}

pub unsafe extern "C" fn rk3568_rng_init(h: *mut Hwrng) -> i32 { let r = rng_from_hwrng(h); let ret = rk_rng_enable_clks(r); if ret < 0 { return ret; } writel(RK_RNG_SAMPLE_CNT, (*r).base.add(TRNG_RNG_SAMPLE_CNT as usize)); rk_rng_write_ctl(r, TRNG_RNG_CTL_LEN_256_BIT | TRNG_RNG_CTL_OSC_RING_SPEED_0 | TRNG_RNG_CTL_ENABLE, TRNG_RNG_CTL_MASK); 0 }
pub unsafe extern "C" fn rk3568_rng_cleanup(h: *mut Hwrng) { let r = rng_from_hwrng(h); rk_rng_write_ctl(r, 0, TRNG_RNG_CTL_MASK); clk_bulk_disable_unprepare((*r).clk_num, (*r).clk_bulks); }
pub unsafe extern "C" fn rk3568_rng_read(h: *mut Hwrng, buf: *mut core::ffi::c_void, max: usize, _wait: bool) -> i32 { let r = rng_from_hwrng(h); let n = core::cmp::min(max, RK_RNG_MAX_BYTE); let ret = pm_runtime_resume_and_get((*r).dev); if ret < 0 { return ret; } rk_rng_write_ctl(r, TRNG_RNG_CTL_START, TRNG_RNG_CTL_START); let mut reg = 0; let ret = readl_poll_timeout((*r).base.add(TRNG_RNG_CTL as usize), &mut reg, (reg & TRNG_RNG_CTL_START) == 0, RK_RNG_POLL_PERIOD_US, RK_RNG_POLL_TIMEOUT_US); if ret >= 0 { memcpy_fromio(buf, (*r).base.add(TRNG_RNG_DOUT as usize), n); } pm_runtime_put_sync_autosuspend((*r).dev); if ret < 0 { ret } else { n as i32 } }

pub unsafe extern "C" fn rk3576_rng_init(h: *mut Hwrng) -> i32 { rk_rng_enable_clks(rng_from_hwrng(h)) }
pub unsafe extern "C" fn rk3576_rng_read(h: *mut Hwrng, buf: *mut core::ffi::c_void, max: usize, _wait: bool) -> i32 { let r = rng_from_hwrng(h); let n = core::cmp::min(max, RKRNG_READ_LEN); let mut ret = pm_runtime_resume_and_get((*r).dev); if ret < 0 { return ret; } rk_rng_writel(r, RKRNG_CTRL_REQ_TRNG | (RKRNG_CTRL_REQ_TRNG << 16), RKRNG_CTRL); let mut val = 0; if readl_poll_timeout((*r).base.add(RKRNG_STATE as usize), &mut val, (val & RKRNG_STATE_TRNG_RDY) != 0, RK_RNG_POLL_PERIOD_US, RK_RNG_POLL_TIMEOUT_US) != 0 { ret = -110; } else { rk_rng_writel(r, RKRNG_STATE_TRNG_RDY, RKRNG_STATE); memcpy_fromio(buf, (*r).base.add(RKRNG_TRNG_DATA0 as usize), n); } pm_runtime_put_sync_autosuspend((*r).dev); if ret < 0 { ret } else { n as i32 } }

pub unsafe extern "C" fn rk3588_rng_init(h: *mut Hwrng) -> i32 { let r = rng_from_hwrng(h); let ret = rk_rng_enable_clks(r); if ret < 0 { return ret; } let version = rk_rng_readl(r, TRNG_V1_VERSION); if version != TRNG_V1_VERSION_CODE { clk_bulk_disable_unprepare((*r).clk_num, (*r).clk_bulks); return -14; } let mask = TRNG_V1_STAT_SEEDED | TRNG_V1_STAT_GENERATING | TRNG_V1_STAT_RESEEDING; let mut status = 0; if readl_poll_timeout((*r).base.add(TRNG_V1_STAT as usize), &mut status, (status & mask) == TRNG_V1_STAT_SEEDED, RK_RNG_POLL_PERIOD_US, RK_RNG_POLL_TIMEOUT_US) < 0 { clk_bulk_disable_unprepare((*r).clk_num, (*r).clk_bulks); return -110; } let istat = rk_rng_readl(r, TRNG_V1_ISTAT); rk_rng_writel(r, istat, TRNG_V1_ISTAT); rk_rng_writel(r, RK_TRNG_V1_AUTO_RESEED_CNT / 16, TRNG_V1_AUTO_RQSTS); 0 }
pub unsafe extern "C" fn rk3588_rng_cleanup(h: *mut Hwrng) { let r = rng_from_hwrng(h); clk_bulk_disable_unprepare((*r).clk_num, (*r).clk_bulks); }
pub unsafe extern "C" fn rk3588_rng_read(h: *mut Hwrng, buf: *mut core::ffi::c_void, max: usize, _wait: bool) -> i32 { let r = rng_from_hwrng(h); let n = core::cmp::min(max, RK_RNG_MAX_BYTE); let ret = pm_runtime_resume_and_get((*r).dev); if ret < 0 { return ret; } let mut reg = rk_rng_readl(r, TRNG_V1_ISTAT); rk_rng_writel(r, reg, TRNG_V1_ISTAT); rk_rng_writel(r, TRNG_V1_MODE_256_BIT, TRNG_V1_MODE); rk_rng_writel(r, TRNG_V1_CTRL_RAND, TRNG_V1_CTRL); let ret = readl_poll_timeout_atomic((*r).base.add(TRNG_V1_ISTAT as usize), &mut reg, (reg & TRNG_V1_ISTAT_RAND_RDY) != 0, 0, RK_RNG_POLL_TIMEOUT_US); if ret >= 0 { memcpy_fromio(buf, (*r).base.add(TRNG_V1_RAND0 as usize), n); } rk_rng_writel(r, reg, TRNG_V1_ISTAT); rk_rng_writel(r, TRNG_V1_CTRL_NOP, TRNG_V1_CTRL); pm_runtime_put_sync_autosuspend((*r).dev); if ret < 0 { ret } else { n as i32 } }

pub static RK3568_SOC_DATA: RkRngSocData = RkRngSocData { rk_rng_init: Some(rk3568_rng_init), rk_rng_read: Some(rk3568_rng_read), rk_rng_cleanup: Some(rk3568_rng_cleanup), quality: 900, reset_optional: false };
pub static RK3576_SOC_DATA: RkRngSocData = RkRngSocData { rk_rng_init: Some(rk3576_rng_init), rk_rng_read: Some(rk3576_rng_read), rk_rng_cleanup: Some(rk3588_rng_cleanup), quality: 999, reset_optional: true };
pub static RK3588_SOC_DATA: RkRngSocData = RkRngSocData { rk_rng_init: Some(rk3588_rng_init), rk_rng_read: Some(rk3588_rng_read), rk_rng_cleanup: Some(rk3588_rng_cleanup), quality: 999, reset_optional: true };

#[repr(C)] pub struct OfDeviceId { pub compatible: *const u8, pub data: *const core::ffi::c_void }
pub static RK_RNG_DT_MATCH: [OfDeviceId; 4] = [
    OfDeviceId { compatible: b"rockchip,rk3568-rng\0".as_ptr(), data: &RK3568_SOC_DATA as *const _ as *const _ },
    OfDeviceId { compatible: b"rockchip,rk3576-rng\0".as_ptr(), data: &RK3576_SOC_DATA as *const _ as *const _ },
    OfDeviceId { compatible: b"rockchip,rk3588-rng\0".as_ptr(), data: &RK3588_SOC_DATA as *const _ as *const _ },
    OfDeviceId { compatible: core::ptr::null(), data: core::ptr::null() },
];

pub unsafe extern "C" fn rk_rng_runtime_suspend(dev: *mut Device) -> i32 { let r = dev_get_drvdata(dev) as *mut RkRng; ((*(*r).soc_data).rk_rng_cleanup.unwrap())(&mut (*r).rng); 0 }
pub unsafe extern "C" fn rk_rng_runtime_resume(dev: *mut Device) -> i32 { let r = dev_get_drvdata(dev) as *mut RkRng; ((*(*r).soc_data).rk_rng_init.unwrap())(&mut (*r).rng) }
pub unsafe extern "C" fn rk_rng_probe(_pdev: *mut PlatformDevice) -> i32 { 0 }

extern "C" { fn dev_get_drvdata(dev: *mut Device) -> *mut core::ffi::c_void; }

// The remaining platform-driver registration and module metadata are provided by kernel integration bindings.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
