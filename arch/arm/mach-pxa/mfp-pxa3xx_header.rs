/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: <linux/soc/pxa/mfp.h>

pub const MFPR_BASE: u32 = 0x40e10000;

/* PXA3xx common MFP configurations - processor specific ones defined
 * in mfp-pxa300.h and mfp-pxa320.h
 */
macro_rules! GPIO0_GPIO { () => { MFP_CFG!(GPIO0, AF0) }; }
macro_rules! GPIO1_GPIO { () => { MFP_CFG!(GPIO1, AF0) }; }
macro_rules! GPIO2_GPIO { () => { MFP_CFG!(GPIO2, AF0) }; }
macro_rules! GPIO3_GPIO { () => { MFP_CFG!(GPIO3, AF0) }; }
macro_rules! GPIO4_GPIO { () => { MFP_CFG!(GPIO4, AF0) }; }
macro_rules! GPIO5_GPIO { () => { MFP_CFG!(GPIO5, AF0) }; }
macro_rules! GPIO6_GPIO { () => { MFP_CFG!(GPIO6, AF0) }; }
macro_rules! GPIO7_GPIO { () => { MFP_CFG!(GPIO7, AF0) }; }
macro_rules! GPIO8_GPIO { () => { MFP_CFG!(GPIO8, AF0) }; }
macro_rules! GPIO9_GPIO { () => { MFP_CFG!(GPIO9, AF0) }; }
macro_rules! GPIO10_GPIO { () => { MFP_CFG!(GPIO10, AF0) }; }
macro_rules! GPIO11_GPIO { () => { MFP_CFG!(GPIO11, AF0) }; }
macro_rules! GPIO12_GPIO { () => { MFP_CFG!(GPIO12, AF0) }; }
macro_rules! GPIO13_GPIO { () => { MFP_CFG!(GPIO13, AF0) }; }
macro_rules! GPIO14_GPIO { () => { MFP_CFG!(GPIO14, AF0) }; }
macro_rules! GPIO15_GPIO { () => { MFP_CFG!(GPIO15, AF0) }; }
macro_rules! GPIO16_GPIO { () => { MFP_CFG!(GPIO16, AF0) }; }
macro_rules! GPIO17_GPIO { () => { MFP_CFG!(GPIO17, AF0) }; }
macro_rules! GPIO18_GPIO { () => { MFP_CFG!(GPIO18, AF0) }; }
macro_rules! GPIO19_GPIO { () => { MFP_CFG!(GPIO19, AF0) }; }
macro_rules! GPIO20_GPIO { () => { MFP_CFG!(GPIO20, AF0) }; }
macro_rules! GPIO21_GPIO { () => { MFP_CFG!(GPIO21, AF0) }; }
macro_rules! GPIO22_GPIO { () => { MFP_CFG!(GPIO22, AF0) }; }
macro_rules! GPIO23_GPIO { () => { MFP_CFG!(GPIO23, AF0) }; }
macro_rules! GPIO24_GPIO { () => { MFP_CFG!(GPIO24, AF0) }; }
macro_rules! GPIO25_GPIO { () => { MFP_CFG!(GPIO25, AF0) }; }
macro_rules! GPIO26_GPIO { () => { MFP_CFG!(GPIO26, AF0) }; }
macro_rules! GPIO27_GPIO { () => { MFP_CFG!(GPIO27, AF0) }; }
macro_rules! GPIO28_GPIO { () => { MFP_CFG!(GPIO28, AF0) }; }
macro_rules! GPIO29_GPIO { () => { MFP_CFG!(GPIO29, AF0) }; }
macro_rules! GPIO30_GPIO { () => { MFP_CFG!(GPIO30, AF0) }; }
macro_rules! GPIO31_GPIO { () => { MFP_CFG!(GPIO31, AF0) }; }
macro_rules! GPIO32_GPIO { () => { MFP_CFG!(GPIO32, AF0) }; }
macro_rules! GPIO33_GPIO { () => { MFP_CFG!(GPIO33, AF0) }; }
macro_rules! GPIO34_GPIO { () => { MFP_CFG!(GPIO34, AF0) }; }
macro_rules! GPIO35_GPIO { () => { MFP_CFG!(GPIO35, AF0) }; }
macro_rules! GPIO36_GPIO { () => { MFP_CFG!(GPIO36, AF0) }; }
macro_rules! GPIO37_GPIO { () => { MFP_CFG!(GPIO37, AF0) }; }
macro_rules! GPIO38_GPIO { () => { MFP_CFG!(GPIO38, AF0) }; }
macro_rules! GPIO39_GPIO { () => { MFP_CFG!(GPIO39, AF0) }; }
macro_rules! GPIO40_GPIO { () => { MFP_CFG!(GPIO40, AF0) }; }
macro_rules! GPIO41_GPIO { () => { MFP_CFG!(GPIO41, AF0) }; }
macro_rules! GPIO42_GPIO { () => { MFP_CFG!(GPIO42, AF0) }; }
macro_rules! GPIO43_GPIO { () => { MFP_CFG!(GPIO43, AF0) }; }
macro_rules! GPIO44_GPIO { () => { MFP_CFG!(GPIO44, AF0) }; }
macro_rules! GPIO45_GPIO { () => { MFP_CFG!(GPIO45, AF0) }; }
macro_rules! GPIO47_GPIO { () => { MFP_CFG!(GPIO47, AF0) }; }
macro_rules! GPIO48_GPIO { () => { MFP_CFG!(GPIO48, AF0) }; }
macro_rules! GPIO53_GPIO { () => { MFP_CFG!(GPIO53, AF0) }; }
macro_rules! GPIO54_GPIO { () => { MFP_CFG!(GPIO54, AF0) }; }
macro_rules! GPIO55_GPIO { () => { MFP_CFG!(GPIO55, AF0) }; }
macro_rules! GPIO57_GPIO { () => { MFP_CFG!(GPIO57, AF0) }; }
macro_rules! GPIO63_GPIO { () => { MFP_CFG!(GPIO63, AF0) }; }
macro_rules! GPIO64_GPIO { () => { MFP_CFG!(GPIO64, AF0) }; }
macro_rules! GPIO65_GPIO { () => { MFP_CFG!(GPIO65, AF0) }; }
macro_rules! GPIO66_GPIO { () => { MFP_CFG!(GPIO66, AF0) }; }
macro_rules! GPIO67_GPIO { () => { MFP_CFG!(GPIO67, AF0) }; }
macro_rules! GPIO68_GPIO { () => { MFP_CFG!(GPIO68, AF0) }; }
macro_rules! GPIO69_GPIO { () => { MFP_CFG!(GPIO69, AF0) }; }
macro_rules! GPIO70_GPIO { () => { MFP_CFG!(GPIO70, AF0) }; }
macro_rules! GPIO71_GPIO { () => { MFP_CFG!(GPIO71, AF0) }; }
macro_rules! GPIO72_GPIO { () => { MFP_CFG!(GPIO72, AF0) }; }
macro_rules! GPIO73_GPIO { () => { MFP_CFG!(GPIO73, AF0) }; }
macro_rules! GPIO74_GPIO { () => { MFP_CFG!(GPIO74, AF0) }; }
macro_rules! GPIO75_GPIO { () => { MFP_CFG!(GPIO75, AF0) }; }
macro_rules! GPIO76_GPIO { () => { MFP_CFG!(GPIO76, AF0) }; }
macro_rules! GPIO77_GPIO { () => { MFP_CFG!(GPIO77, AF0) }; }
macro_rules! GPIO78_GPIO { () => { MFP_CFG!(GPIO78, AF0) }; }
macro_rules! GPIO79_GPIO { () => { MFP_CFG!(GPIO79, AF0) }; }
macro_rules! GPIO80_GPIO { () => { MFP_CFG!(GPIO80, AF0) }; }
macro_rules! GPIO81_GPIO { () => { MFP_CFG!(GPIO81, AF0) }; }
macro_rules! GPIO82_GPIO { () => { MFP_CFG!(GPIO82, AF0) }; }
macro_rules! GPIO83_GPIO { () => { MFP_CFG!(GPIO83, AF0) }; }
macro_rules! GPIO84_GPIO { () => { MFP_CFG!(GPIO84, AF0) }; }
macro_rules! GPIO85_GPIO { () => { MFP_CFG!(GPIO85, AF0) }; }
macro_rules! GPIO86_GPIO { () => { MFP_CFG!(GPIO86, AF0) }; }
macro_rules! GPIO87_GPIO { () => { MFP_CFG!(GPIO87, AF0) }; }
macro_rules! GPIO88_GPIO { () => { MFP_CFG!(GPIO88, AF0) }; }
macro_rules! GPIO89_GPIO { () => { MFP_CFG!(GPIO89, AF0) }; }
macro_rules! GPIO90_GPIO { () => { MFP_CFG!(GPIO90, AF0) }; }
macro_rules! GPIO91_GPIO { () => { MFP_CFG!(GPIO91, AF0) }; }
macro_rules! GPIO92_GPIO { () => { MFP_CFG!(GPIO92, AF0) }; }
macro_rules! GPIO93_GPIO { () => { MFP_CFG!(GPIO93, AF0) }; }
macro_rules! GPIO94_GPIO { () => { MFP_CFG!(GPIO94, AF0) }; }
macro_rules! GPIO95_GPIO { () => { MFP_CFG!(GPIO95, AF0) }; }
macro_rules! GPIO96_GPIO { () => { MFP_CFG!(GPIO96, AF0) }; }
macro_rules! GPIO97_GPIO { () => { MFP_CFG!(GPIO97, AF0) }; }
macro_rules! GPIO98_GPIO { () => { MFP_CFG!(GPIO98, AF0) }; }
macro_rules! GPIO99_GPIO { () => { MFP_CFG!(GPIO99, AF0) }; }
macro_rules! GPIO100_GPIO { () => { MFP_CFG!(GPIO100, AF0) }; }
macro_rules! GPIO101_GPIO { () => { MFP_CFG!(GPIO101, AF0) }; }
macro_rules! GPIO102_GPIO { () => { MFP_CFG!(GPIO102, AF0) }; }
macro_rules! GPIO103_GPIO { () => { MFP_CFG!(GPIO103, AF0) }; }
macro_rules! GPIO104_GPIO { () => { MFP_CFG!(GPIO104, AF0) }; }
macro_rules! GPIO105_GPIO { () => { MFP_CFG!(GPIO105, AF0) }; }
macro_rules! GPIO106_GPIO { () => { MFP_CFG!(GPIO106, AF0) }; }
macro_rules! GPIO107_GPIO { () => { MFP_CFG!(GPIO107, AF0) }; }
macro_rules! GPIO108_GPIO { () => { MFP_CFG!(GPIO108, AF0) }; }
macro_rules! GPIO109_GPIO { () => { MFP_CFG!(GPIO109, AF0) }; }
macro_rules! GPIO110_GPIO { () => { MFP_CFG!(GPIO110, AF0) }; }
macro_rules! GPIO111_GPIO { () => { MFP_CFG!(GPIO111, AF0) }; }
macro_rules! GPIO112_GPIO { () => { MFP_CFG!(GPIO112, AF0) }; }
macro_rules! GPIO113_GPIO { () => { MFP_CFG!(GPIO113, AF0) }; }
macro_rules! GPIO114_GPIO { () => { MFP_CFG!(GPIO114, AF0) }; }
macro_rules! GPIO115_GPIO { () => { MFP_CFG!(GPIO115, AF0) }; }
macro_rules! GPIO116_GPIO { () => { MFP_CFG!(GPIO116, AF0) }; }
macro_rules! GPIO117_GPIO { () => { MFP_CFG!(GPIO117, AF0) }; }
macro_rules! GPIO118_GPIO { () => { MFP_CFG!(GPIO118, AF0) }; }
macro_rules! GPIO119_GPIO { () => { MFP_CFG!(GPIO119, AF0) }; }
macro_rules! GPIO120_GPIO { () => { MFP_CFG!(GPIO120, AF0) }; }
macro_rules! GPIO121_GPIO { () => { MFP_CFG!(GPIO121, AF0) }; }
macro_rules! GPIO122_GPIO { () => { MFP_CFG!(GPIO122, AF0) }; }
macro_rules! GPIO123_GPIO { () => { MFP_CFG!(GPIO123, AF0) }; }
macro_rules! GPIO124_GPIO { () => { MFP_CFG!(GPIO124, AF0) }; }
macro_rules! GPIO125_GPIO { () => { MFP_CFG!(GPIO125, AF0) }; }
macro_rules! GPIO126_GPIO { () => { MFP_CFG!(GPIO126, AF0) }; }
macro_rules! GPIO127_GPIO { () => { MFP_CFG!(GPIO127, AF0) }; }

macro_rules! GPIO0_2_GPIO { () => { MFP_CFG!(GPIO0_2, AF0) }; }
macro_rules! GPIO1_2_GPIO { () => { MFP_CFG!(GPIO1_2, AF0) }; }
macro_rules! GPIO2_2_GPIO { () => { MFP_CFG!(GPIO2_2, AF0) }; }
macro_rules! GPIO3_2_GPIO { () => { MFP_CFG!(GPIO3_2, AF0) }; }
macro_rules! GPIO4_2_GPIO { () => { MFP_CFG!(GPIO4_2, AF0) }; }
macro_rules! GPIO5_2_GPIO { () => { MFP_CFG!(GPIO5_2, AF0) }; }
macro_rules! GPIO6_2_GPIO { () => { MFP_CFG!(GPIO6_2, AF0) }; }

/* NOTE: usage of these two functions is not recommended,
 * use pxa3xx_mfp_config() instead.
 */

#[inline]
pub unsafe fn pxa3xx_mfp_read(mfp: core::ffi::c_int) -> c_ulong {
    mfp_read(mfp)
}

#[inline]
pub unsafe fn pxa3xx_mfp_write(mfp: core::ffi::c_int, val: c_ulong) {
    mfp_write(mfp, val);
}

#[inline]
pub unsafe fn pxa3xx_mfp_config(mfp_cfg: *mut c_ulong, num: core::ffi::c_int) {
    mfp_config(mfp_cfg, num);
}

unsafe extern "C" {
    fn mfp_read(mfp: core::ffi::c_int) -> c_ulong;
    fn mfp_write(mfp: core::ffi::c_int, val: c_ulong);
    fn mfp_config(mfp_cfg: *mut c_ulong, num: core::ffi::c_int);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
