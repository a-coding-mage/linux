/* SPDX-License-Identifier: GPL-2.0-or-later */
/* OMAP cpu type detection; translated from soc.h. */

// Dependencies supplied by the surrounding translation unit:
// omap24xx.h, omap34xx.h, omap44xx.h, ti81xx.h, am33xx.h, omap54xx.h

pub const OMAP2_DEVICE_TYPE_TEST: i32 = 0;
pub const OMAP2_DEVICE_TYPE_EMU: i32 = 1;
pub const OMAP2_DEVICE_TYPE_SEC: i32 = 2;
pub const OMAP2_DEVICE_TYPE_GP: i32 = 3;
pub const OMAP2_DEVICE_TYPE_BAD: i32 = 4;

extern "C" {
    pub fn omap_type() -> i32;
    pub fn omap_rev() -> u32;
    pub fn omap2xxx_check_revision();
    pub fn omap3xxx_check_revision();
    pub fn omap4xxx_check_revision();
    pub fn omap5xxx_check_revision();
    pub fn dra7xxx_check_revision();
    pub fn omap3xxx_check_features();
    pub fn ti81xx_check_features();
    pub fn am33xx_check_features();
    pub fn omap4xxx_check_features();
    pub static mut omap_features: u32;
}

#[inline] pub unsafe fn soc_is_omap() -> i32 { (omap_rev() != 0) as i32 }
#[inline] pub unsafe fn get_omap_revision() -> u32 { (omap_rev() >> 8) & 0xff }
#[inline] pub unsafe fn get_omap_class() -> u32 { omap_rev() & 0xff }
#[inline] pub unsafe fn get_am_class() -> u32 { (omap_rev() >> 24) & 0xff }
#[inline] pub unsafe fn get_ti_class() -> u32 { (omap_rev() >> 24) & 0xff }
#[inline] pub unsafe fn get_dra_class() -> u32 { (omap_rev() >> 24) & 0xff }
#[inline] pub unsafe fn get_omap_subclass() -> u32 { (omap_rev() >> 20) & 0x0fff }
#[inline] pub unsafe fn get_dra_package() -> u32 { omap_rev() & 0xff }

macro_rules! omap_class { ($name:ident, $id:expr) => { #[inline] pub unsafe fn $name() -> i32 { (get_omap_class() == $id) as i32 } }; }
macro_rules! am_class { ($name:ident, $id:expr) => { #[inline] pub unsafe fn $name() -> i32 { (get_am_class() == $id) as i32 } }; }
macro_rules! ti_class { ($name:ident, $id:expr) => { #[inline] pub unsafe fn $name() -> i32 { (get_ti_class() == $id) as i32 } }; }
macro_rules! dra_class { ($name:ident, $id:expr) => { #[inline] pub unsafe fn $name() -> i32 { (get_dra_class() == $id) as i32 } }; }
macro_rules! subclass { ($name:ident, $id:expr) => { #[inline] pub unsafe fn $name() -> i32 { (get_omap_subclass() == $id) as i32 } }; }

omap_class!(is_omap24xx, 0x24); omap_class!(is_omap34xx, 0x34); omap_class!(is_omap44xx, 0x44);
am_class!(is_am35xx, 0x35); omap_class!(is_omap54xx, 0x54); am_class!(is_am33xx, 0x33); am_class!(is_am43xx, 0x43);
ti_class!(is_ti81xx, 0x81); dra_class!(is_dra7xx, 0x7);
subclass!(is_omap242x, 0x242); subclass!(is_omap243x, 0x243); subclass!(is_omap343x, 0x343);
subclass!(is_omap363x, 0x363); subclass!(is_omap443x, 0x443); subclass!(is_omap446x, 0x446);
subclass!(is_omap447x, 0x447); subclass!(is_omap543x, 0x543); subclass!(is_ti816x, 0x816);
subclass!(is_ti814x, 0x814); subclass!(is_am335x, 0x335); subclass!(is_am437x, 0x437);
subclass!(is_dra76x, 0x76); subclass!(is_dra75x, 0x75); subclass!(is_dra72x, 0x72);
#[inline] pub unsafe fn is_omap2420() -> i32 { (((omap_rev() >> 16) & 0xffff) == 0x2420) as i32 }
#[inline] pub unsafe fn is_omap2422() -> i32 { (((omap_rev() >> 16) & 0xffff) == 0x2422) as i32 }
#[inline] pub unsafe fn is_omap2423() -> i32 { (((omap_rev() >> 16) & 0xffff) == 0x2423) as i32 }
#[inline] pub unsafe fn is_omap2430() -> i32 { (((omap_rev() >> 16) & 0xffff) == 0x2430) as i32 }
#[inline] pub unsafe fn is_omap3430() -> i32 { (((omap_rev() >> 16) & 0xffff) == 0x3430) as i32 }

#[inline] pub unsafe fn is_dra76x_abz() -> i32 { (is_dra76x() != 0 && get_dra_package() == 2) as i32 }
#[inline] pub unsafe fn is_dra76x_acd() -> i32 { (is_dra76x() != 0 && get_dra_package() == 3) as i32 }

macro_rules! zero_fn { ($($n:ident),* $(,)?) => { $(#[inline] pub const fn $n() -> i32 { 0 })* }; }
zero_fn!(soc_is_ti81xx, soc_is_ti816x, soc_is_ti814x, soc_is_am35xx, soc_is_am33xx, soc_is_am335x,
    soc_is_am43xx, soc_is_am437x, soc_is_omap44xx, soc_is_omap443x, soc_is_omap446x, soc_is_omap447x,
    soc_is_omap54xx, soc_is_omap543x, soc_is_dra7xx, soc_is_dra76x, soc_is_dra74x, soc_is_dra72x,
    soc_is_omap2420, soc_is_omap2422, soc_is_omap2423, soc_is_omap2430, soc_is_omap3430, soc_is_omap3630,
    soc_is_omap5430, soc_is_omap7xx, soc_is_omap15xx, soc_is_omap16xx, soc_is_omap1510, soc_is_omap1610,
    soc_is_omap1611, soc_is_omap1621, soc_is_omap1710);
#[inline] pub const fn cpu_class_is_omap1() -> i32 { 0 }
#[inline] pub const fn cpu_class_is_omap2() -> i32 { 1 }

pub const OMAP242X_CLASS: u32 = 0x24200024; pub const OMAP2420_REV_ES1_0: u32 = OMAP242X_CLASS;
pub const OMAP2420_REV_ES2_0: u32 = OMAP242X_CLASS | (0x1 << 8);
pub const OMAP243X_CLASS: u32 = 0x24300024; pub const OMAP2430_REV_ES1_0: u32 = OMAP243X_CLASS;
pub const OMAP343X_CLASS: u32 = 0x34300034; pub const OMAP3430_REV_ES1_0: u32 = OMAP343X_CLASS;
pub const OMAP3430_REV_ES2_0: u32 = OMAP343X_CLASS | (0x1 << 8); pub const OMAP3430_REV_ES2_1: u32 = OMAP343X_CLASS | (0x2 << 8);
pub const OMAP3430_REV_ES3_0: u32 = OMAP343X_CLASS | (0x3 << 8); pub const OMAP3430_REV_ES3_1: u32 = OMAP343X_CLASS | (0x4 << 8); pub const OMAP3430_REV_ES3_1_2: u32 = OMAP343X_CLASS | (0x5 << 8);
pub const OMAP363X_CLASS: u32 = 0x36300034; pub const OMAP3630_REV_ES1_0: u32 = OMAP363X_CLASS; pub const OMAP3630_REV_ES1_1: u32 = OMAP363X_CLASS | (1 << 8); pub const OMAP3630_REV_ES1_2: u32 = OMAP363X_CLASS | (2 << 8);
pub const TI816X_CLASS: u32 = 0x81600081; pub const TI8168_REV_ES1_0: u32 = TI816X_CLASS; pub const TI8168_REV_ES1_1: u32 = TI816X_CLASS | (1 << 8); pub const TI8168_REV_ES2_0: u32 = TI816X_CLASS | (2 << 8); pub const TI8168_REV_ES2_1: u32 = TI816X_CLASS | (3 << 8);
pub const TI814X_CLASS: u32 = 0x81400081; pub const TI8148_REV_ES1_0: u32 = TI814X_CLASS; pub const TI8148_REV_ES2_0: u32 = TI814X_CLASS | (1 << 8); pub const TI8148_REV_ES2_1: u32 = TI814X_CLASS | (2 << 8);
pub const AM35XX_CLASS: u32 = 0x35170034; pub const AM35XX_REV_ES1_0: u32 = AM35XX_CLASS; pub const AM35XX_REV_ES1_1: u32 = AM35XX_CLASS | (1 << 8);
pub const AM335X_CLASS: u32 = 0x33500033; pub const AM335X_REV_ES1_0: u32 = AM335X_CLASS; pub const AM335X_REV_ES2_0: u32 = AM335X_CLASS | (1 << 8); pub const AM335X_REV_ES2_1: u32 = AM335X_CLASS | (2 << 8);
pub const AM437X_CLASS: u32 = 0x43700000; pub const AM437X_REV_ES1_0: u32 = AM437X_CLASS | (0x10 << 8); pub const AM437X_REV_ES1_1: u32 = AM437X_CLASS | (0x11 << 8); pub const AM437X_REV_ES1_2: u32 = AM437X_CLASS | (0x12 << 8);
pub const OMAP443X_CLASS: u32 = 0x44300044; pub const OMAP4430_REV_ES1_0: u32 = OMAP443X_CLASS | (0x10 << 8); pub const OMAP4430_REV_ES2_0: u32 = OMAP443X_CLASS | (0x20 << 8); pub const OMAP4430_REV_ES2_1: u32 = OMAP443X_CLASS | (0x21 << 8); pub const OMAP4430_REV_ES2_2: u32 = OMAP443X_CLASS | (0x22 << 8); pub const OMAP4430_REV_ES2_3: u32 = OMAP443X_CLASS | (0x23 << 8);
pub const OMAP446X_CLASS: u32 = 0x44600044; pub const OMAP4460_REV_ES1_0: u32 = OMAP446X_CLASS | (0x10 << 8); pub const OMAP4460_REV_ES1_1: u32 = OMAP446X_CLASS | (0x11 << 8);
pub const OMAP447X_CLASS: u32 = 0x44700044; pub const OMAP4470_REV_ES1_0: u32 = OMAP447X_CLASS | (0x10 << 8);
pub const OMAP54XX_CLASS: u32 = 0x54000054; pub const OMAP5430_REV_ES2_0: u32 = OMAP54XX_CLASS | (0x30 << 16) | (0x20 << 8); pub const OMAP5432_REV_ES2_0: u32 = OMAP54XX_CLASS | (0x32 << 16) | (0x20 << 8);
pub const DRA7XX_CLASS: u32 = 0x07000000; pub const DRA762_REV_ES1_0: u32 = DRA7XX_CLASS | (0x62 << 16) | (0x10 << 8); pub const DRA762_ABZ_REV_ES1_0: u32 = DRA762_REV_ES1_0 | 2; pub const DRA762_ACD_REV_ES1_0: u32 = DRA762_REV_ES1_0 | 3;
pub const DRA752_REV_ES1_0: u32 = DRA7XX_CLASS | (0x52 << 16) | (0x10 << 8); pub const DRA752_REV_ES1_1: u32 = DRA7XX_CLASS | (0x52 << 16) | (0x11 << 8); pub const DRA752_REV_ES2_0: u32 = DRA7XX_CLASS | (0x52 << 16) | (0x20 << 8); pub const DRA722_REV_ES1_0: u32 = DRA7XX_CLASS | (0x22 << 16) | (0x10 << 8); pub const DRA722_REV_ES2_0: u32 = DRA7XX_CLASS | (0x22 << 16) | (0x20 << 8); pub const DRA722_REV_ES2_1: u32 = DRA7XX_CLASS | (0x22 << 16) | (0x21 << 8);

pub const OMAP3_HAS_L2CACHE: u32 = 1 << 0; pub const OMAP3_HAS_IVA: u32 = 1 << 1; pub const OMAP3_HAS_SGX: u32 = 1 << 2; pub const OMAP3_HAS_NEON: u32 = 1 << 3; pub const OMAP3_HAS_ISP: u32 = 1 << 4; pub const OMAP3_HAS_192MHZ_CLK: u32 = 1 << 5; pub const OMAP3_HAS_IO_WAKEUP: u32 = 1 << 6; pub const OMAP3_HAS_SDRC: u32 = 1 << 7; pub const OMAP3_HAS_IO_CHAIN_CTRL: u32 = 1 << 8; pub const OMAP4_HAS_PERF_SILICON: u32 = 1 << 9;
macro_rules! feature { ($n:ident, $f:ident) => { #[inline] pub unsafe fn $n() -> u32 { omap_features & $f } }; }
feature!(omap3_has_l2cache, OMAP3_HAS_L2CACHE); feature!(omap3_has_sgx, OMAP3_HAS_SGX); feature!(omap3_has_iva, OMAP3_HAS_IVA); feature!(omap3_has_neon, OMAP3_HAS_NEON); feature!(omap3_has_isp, OMAP3_HAS_ISP); feature!(omap3_has_192mhz_clk, OMAP3_HAS_192MHZ_CLK); feature!(omap3_has_io_wakeup, OMAP3_HAS_IO_WAKEUP); feature!(omap3_has_sdrc, OMAP3_HAS_SDRC); feature!(omap3_has_io_chain_ctrl, OMAP3_HAS_IO_CHAIN_CTRL); feature!(omap4_has_perf_silicon, OMAP4_HAS_PERF_SILICON);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
