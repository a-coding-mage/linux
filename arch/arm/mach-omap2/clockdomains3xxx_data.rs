// SPDX-License-Identifier: GPL-2.0
/* OMAP3xxx clockdomains -- direct Rust translation of clockdomains3xxx_data.c. */

use core::ffi::c_char;

// Definitions supplied by the surrounding kernel translation.
extern "C" {
    static mut wkup_common_clkdm: clockdomain;
    static omap3_clkdm_operations: clkdm_ops;
    fn cpu_is_omap34xx() -> bool;
    fn omap_rev() -> u32;
    fn clkdm_register_platform_funcs(ops: *const clkdm_ops);
    fn clkdm_register_clkdms(clkdms: *mut *mut clockdomain);
    fn clkdm_register_autodeps(deps: *mut clkdm_autodep);
    fn clkdm_complete_init();
}

#[repr(C)] pub struct clkdm_dep { pub clkdm_name: *const c_char }
#[repr(C)] pub struct powerdomain { pub name: *const c_char }
#[repr(C)] pub struct clockdomain {
    pub name: *const c_char, pub pwrdm: powerdomain, pub flags: u32,
    pub dep_bit: u32, pub wkdep_srcs: *mut clkdm_dep, pub sleepdep_srcs: *mut clkdm_dep,
    pub clktrctrl_mask: u32,
}
#[repr(C)] pub struct clkdm_autodep { pub clkdm: powerdomain }
#[repr(C)] pub struct clkdm_ops { _private: [u8; 0] }

macro_rules! s { ($x:literal) => { concat!($x, "\0").as_ptr() as *const c_char }; }
const NULL: *const c_char = core::ptr::null();

static mut gfx_sgx_3xxx_wkdeps: [clkdm_dep; 4] = [
    clkdm_dep { clkdm_name: s!("iva2_clkdm") }, clkdm_dep { clkdm_name: s!("mpu_clkdm") },
    clkdm_dep { clkdm_name: s!("wkup_clkdm") }, clkdm_dep { clkdm_name: NULL }];
static mut gfx_sgx_am35x_wkdeps: [clkdm_dep; 3] = [
    clkdm_dep { clkdm_name: s!("mpu_clkdm") }, clkdm_dep { clkdm_name: s!("wkup_clkdm") }, clkdm_dep { clkdm_name: NULL }];
static mut per_wkdeps: [clkdm_dep; 6] = [clkdm_dep {clkdm_name:s!("core_l3_clkdm")},clkdm_dep {clkdm_name:s!("core_l4_clkdm")},clkdm_dep {clkdm_name:s!("iva2_clkdm")},clkdm_dep {clkdm_name:s!("mpu_clkdm")},clkdm_dep {clkdm_name:s!("wkup_clkdm")},clkdm_dep {clkdm_name:NULL}];
static mut per_am35x_wkdeps: [clkdm_dep; 5] = [clkdm_dep {clkdm_name:s!("core_l3_clkdm")},clkdm_dep {clkdm_name:s!("core_l4_clkdm")},clkdm_dep {clkdm_name:s!("mpu_clkdm")},clkdm_dep {clkdm_name:s!("wkup_clkdm")},clkdm_dep {clkdm_name:NULL}];
static mut usbhost_wkdeps: [clkdm_dep; 6] = [clkdm_dep {clkdm_name:s!("core_l3_clkdm")},clkdm_dep {clkdm_name:s!("core_l4_clkdm")},clkdm_dep {clkdm_name:s!("iva2_clkdm")},clkdm_dep {clkdm_name:s!("mpu_clkdm")},clkdm_dep {clkdm_name:s!("wkup_clkdm")},clkdm_dep {clkdm_name:NULL}];
static mut usbhost_am35x_wkdeps: [clkdm_dep; 5] = [clkdm_dep {clkdm_name:s!("core_l3_clkdm")},clkdm_dep {clkdm_name:s!("core_l4_clkdm")},clkdm_dep {clkdm_name:s!("mpu_clkdm")},clkdm_dep {clkdm_name:s!("wkup_clkdm")},clkdm_dep {clkdm_name:NULL}];
static mut mpu_3xxx_wkdeps: [clkdm_dep; 6] = [clkdm_dep {clkdm_name:s!("core_l3_clkdm")},clkdm_dep {clkdm_name:s!("core_l4_clkdm")},clkdm_dep {clkdm_name:s!("iva2_clkdm")},clkdm_dep {clkdm_name:s!("dss_clkdm")},clkdm_dep {clkdm_name:s!("per_clkdm")},clkdm_dep {clkdm_name:NULL}];
static mut mpu_am35x_wkdeps: [clkdm_dep; 5] = [clkdm_dep {clkdm_name:s!("core_l3_clkdm")},clkdm_dep {clkdm_name:s!("core_l4_clkdm")},clkdm_dep {clkdm_name:s!("dss_clkdm")},clkdm_dep {clkdm_name:s!("per_clkdm")},clkdm_dep {clkdm_name:NULL}];
static mut iva2_wkdeps: [clkdm_dep; 7] = [clkdm_dep {clkdm_name:s!("core_l3_clkdm")},clkdm_dep {clkdm_name:s!("core_l4_clkdm")},clkdm_dep {clkdm_name:s!("mpu_clkdm")},clkdm_dep {clkdm_name:s!("wkup_clkdm")},clkdm_dep {clkdm_name:s!("dss_clkdm")},clkdm_dep {clkdm_name:s!("per_clkdm")},clkdm_dep {clkdm_name:NULL}];
static mut cam_wkdeps: [clkdm_dep; 4] = [clkdm_dep {clkdm_name:s!("iva2_clkdm")},clkdm_dep {clkdm_name:s!("mpu_clkdm")},clkdm_dep {clkdm_name:s!("wkup_clkdm")},clkdm_dep {clkdm_name:NULL}];
static mut dss_wkdeps: [clkdm_dep; 4] = [clkdm_dep {clkdm_name:s!("iva2_clkdm")},clkdm_dep {clkdm_name:s!("mpu_clkdm")},clkdm_dep {clkdm_name:s!("wkup_clkdm")},clkdm_dep {clkdm_name:NULL}];
static mut dss_am35x_wkdeps: [clkdm_dep; 3] = [clkdm_dep {clkdm_name:s!("mpu_clkdm")},clkdm_dep {clkdm_name:s!("wkup_clkdm")},clkdm_dep {clkdm_name:NULL}];
static mut neon_wkdeps: [clkdm_dep; 2] = [clkdm_dep {clkdm_name:s!("mpu_clkdm")},clkdm_dep {clkdm_name:NULL}];

macro_rules! deps { ($n:ident, [$($x:literal),*]) => { static mut $n: [clkdm_dep; deps!(@c $($x),*)] = [$(clkdm_dep {clkdm_name:s!($x)}),*,clkdm_dep {clkdm_name:NULL}]; }; (@c $($x:literal),*) => { 1usize $(+ {let _ = $x; 1usize})* }; }
deps!(dss_sleepdeps, ["mpu_clkdm", "iva2_clkdm"]); deps!(dss_am35x_sleepdeps, ["mpu_clkdm"]);
deps!(per_sleepdeps, ["mpu_clkdm", "iva2_clkdm"]); deps!(per_am35x_sleepdeps, ["mpu_clkdm"]);
deps!(usbhost_sleepdeps, ["mpu_clkdm", "iva2_clkdm"]); deps!(usbhost_am35x_sleepdeps, ["mpu_clkdm"]);
deps!(cam_sleepdeps, ["mpu_clkdm"]); deps!(gfx_sgx_sleepdeps, ["mpu_clkdm"]);

macro_rules! cd { ($n:ident, $name:literal, $pd:literal, $flags:expr, $dep:expr, $wk:expr, $sl:expr, $mask:expr) => {
    static mut $n: clockdomain = clockdomain { name:s!($name), pwrdm:powerdomain{name:s!($pd)}, flags:$flags, dep_bit:$dep, wkdep_srcs:$wk, sleepdep_srcs:$sl, clktrctrl_mask:$mask };
}; }
// Bit and flag constants are provided by the translated OMAP headers.
extern "C" { static CLKDM_CAN_HWSUP: u32; static CLKDM_CAN_FORCE_WAKEUP: u32; static CLKDM_CAN_HWSUP_SWSUP: u32; static CLKDM_CAN_SWSUP: u32; static CLKDM_CAN_ENABLE_AUTO: u32; static CLKDM_MISSING_IDLE_REPORTING: u32; }
extern "C" { static OMAP3430_EN_MPU_SHIFT:u32; static OMAP3430_PM_WKDEP_MPU_EN_IVA2_SHIFT:u32; static OMAP3430_EN_CORE_SHIFT:u32; static OMAP3430_PM_WKDEP_MPU_EN_DSS_SHIFT:u32; static OMAP3430_EN_PER_SHIFT:u32; static OMAP3430_CLKTRCTRL_MPU_MASK:u32; static OMAP3430_CLKTRCTRL_NEON_MASK:u32; static OMAP3430_CLKTRCTRL_IVA2_MASK:u32; static OMAP3430ES1_CLKTRCTRL_GFX_MASK:u32; static OMAP3430ES2_CLKTRCTRL_SGX_MASK:u32; static OMAP3430ES1_CLKTRCTRL_D2D_MASK:u32; static OMAP3430_CLKTRCTRL_L3_MASK:u32; static OMAP3430_CLKTRCTRL_L4_MASK:u32; static OMAP3430_CLKTRCTRL_DSS_MASK:u32; static OMAP3430_CLKTRCTRL_CAM_MASK:u32; static OMAP3430ES2_CLKTRCTRL_USBHOST_MASK:u32; static OMAP3430_CLKTRCTRL_PER_MASK:u32; static OMAP3430_CLKTRCTRL_EMU_MASK:u32; }

macro_rules! cv { ($x:ident) => { unsafe { $x } }; }
cd!(mpu_3xxx_clkdm,"mpu_clkdm","mpu_pwrdm",cv!(CLKDM_CAN_HWSUP)|cv!(CLKDM_CAN_FORCE_WAKEUP),cv!(OMAP3430_EN_MPU_SHIFT),unsafe{mpu_3xxx_wkdeps.as_mut_ptr()},core::ptr::null_mut(),cv!(OMAP3430_CLKTRCTRL_MPU_MASK));
cd!(mpu_am35x_clkdm,"mpu_clkdm","mpu_pwrdm",cv!(CLKDM_CAN_HWSUP)|cv!(CLKDM_CAN_FORCE_WAKEUP),cv!(OMAP3430_EN_MPU_SHIFT),unsafe{mpu_am35x_wkdeps.as_mut_ptr()},core::ptr::null_mut(),cv!(OMAP3430_CLKTRCTRL_MPU_MASK));
cd!(neon_clkdm,"neon_clkdm","neon_pwrdm",cv!(CLKDM_CAN_HWSUP_SWSUP),0,unsafe{neon_wkdeps.as_mut_ptr()},core::ptr::null_mut(),cv!(OMAP3430_CLKTRCTRL_NEON_MASK));
cd!(iva2_clkdm,"iva2_clkdm","iva2_pwrdm",cv!(CLKDM_CAN_SWSUP),cv!(OMAP3430_PM_WKDEP_MPU_EN_IVA2_SHIFT),unsafe{iva2_wkdeps.as_mut_ptr()},core::ptr::null_mut(),cv!(OMAP3430_CLKTRCTRL_IVA2_MASK));
cd!(gfx_3430es1_clkdm,"gfx_clkdm","gfx_pwrdm",cv!(CLKDM_CAN_HWSUP_SWSUP),0,unsafe{gfx_sgx_3xxx_wkdeps.as_mut_ptr()},unsafe{gfx_sgx_sleepdeps.as_mut_ptr()},cv!(OMAP3430ES1_CLKTRCTRL_GFX_MASK));
cd!(sgx_clkdm,"sgx_clkdm","sgx_pwrdm",cv!(CLKDM_CAN_HWSUP_SWSUP),0,unsafe{gfx_sgx_3xxx_wkdeps.as_mut_ptr()},unsafe{gfx_sgx_sleepdeps.as_mut_ptr()},cv!(OMAP3430ES2_CLKTRCTRL_SGX_MASK));
cd!(sgx_am35x_clkdm,"sgx_clkdm","sgx_pwrdm",cv!(CLKDM_CAN_HWSUP_SWSUP),0,unsafe{gfx_sgx_am35x_wkdeps.as_mut_ptr()},unsafe{gfx_sgx_sleepdeps.as_mut_ptr()},cv!(OMAP3430ES2_CLKTRCTRL_SGX_MASK));
cd!(d2d_clkdm,"d2d_clkdm","core_pwrdm",cv!(CLKDM_CAN_HWSUP_SWSUP),0,core::ptr::null_mut(),core::ptr::null_mut(),cv!(OMAP3430ES1_CLKTRCTRL_D2D_MASK));
cd!(core_l3_3xxx_clkdm,"core_l3_clkdm","core_pwrdm",cv!(CLKDM_CAN_HWSUP),cv!(OMAP3430_EN_CORE_SHIFT),core::ptr::null_mut(),core::ptr::null_mut(),cv!(OMAP3430_CLKTRCTRL_L3_MASK));
cd!(core_l4_3xxx_clkdm,"core_l4_clkdm","core_pwrdm",cv!(CLKDM_CAN_HWSUP),cv!(OMAP3430_EN_CORE_SHIFT),core::ptr::null_mut(),core::ptr::null_mut(),cv!(OMAP3430_CLKTRCTRL_L4_MASK));
cd!(dss_3xxx_clkdm,"dss_clkdm","dss_pwrdm",cv!(CLKDM_CAN_HWSUP_SWSUP),cv!(OMAP3430_PM_WKDEP_MPU_EN_DSS_SHIFT),unsafe{dss_wkdeps.as_mut_ptr()},unsafe{dss_sleepdeps.as_mut_ptr()},cv!(OMAP3430_CLKTRCTRL_DSS_MASK));
cd!(dss_am35x_clkdm,"dss_clkdm","dss_pwrdm",cv!(CLKDM_CAN_HWSUP_SWSUP),cv!(OMAP3430_PM_WKDEP_MPU_EN_DSS_SHIFT),unsafe{dss_am35x_wkdeps.as_mut_ptr()},unsafe{dss_am35x_sleepdeps.as_mut_ptr()},cv!(OMAP3430_CLKTRCTRL_DSS_MASK));
cd!(cam_clkdm,"cam_clkdm","cam_pwrdm",cv!(CLKDM_CAN_HWSUP_SWSUP),0,unsafe{cam_wkdeps.as_mut_ptr()},unsafe{cam_sleepdeps.as_mut_ptr()},cv!(OMAP3430_CLKTRCTRL_CAM_MASK));
cd!(usbhost_clkdm,"usbhost_clkdm","usbhost_pwrdm",cv!(CLKDM_CAN_HWSUP_SWSUP),0,unsafe{usbhost_wkdeps.as_mut_ptr()},unsafe{usbhost_sleepdeps.as_mut_ptr()},cv!(OMAP3430ES2_CLKTRCTRL_USBHOST_MASK));
cd!(usbhost_am35x_clkdm,"usbhost_clkdm","core_pwrdm",cv!(CLKDM_CAN_HWSUP_SWSUP),0,unsafe{usbhost_am35x_wkdeps.as_mut_ptr()},unsafe{usbhost_am35x_sleepdeps.as_mut_ptr()},cv!(OMAP3430ES2_CLKTRCTRL_USBHOST_MASK));
cd!(per_clkdm,"per_clkdm","per_pwrdm",cv!(CLKDM_CAN_HWSUP_SWSUP),cv!(OMAP3430_EN_PER_SHIFT),unsafe{per_wkdeps.as_mut_ptr()},unsafe{per_sleepdeps.as_mut_ptr()},cv!(OMAP3430_CLKTRCTRL_PER_MASK));
cd!(per_am35x_clkdm,"per_clkdm","per_pwrdm",cv!(CLKDM_CAN_HWSUP_SWSUP),cv!(OMAP3430_EN_PER_SHIFT),unsafe{per_am35x_wkdeps.as_mut_ptr()},unsafe{per_am35x_sleepdeps.as_mut_ptr()},cv!(OMAP3430_CLKTRCTRL_PER_MASK));
cd!(emu_clkdm,"emu_clkdm","emu_pwrdm",cv!(CLKDM_CAN_ENABLE_AUTO)|cv!(CLKDM_CAN_SWSUP)|cv!(CLKDM_MISSING_IDLE_REPORTING),0,core::ptr::null_mut(),core::ptr::null_mut(),cv!(OMAP3430_CLKTRCTRL_EMU_MASK));
cd!(dpll1_clkdm,"dpll1_clkdm","dpll1_pwrdm",0,0,core::ptr::null_mut(),core::ptr::null_mut(),0); cd!(dpll2_clkdm,"dpll2_clkdm","dpll2_pwrdm",0,0,core::ptr::null_mut(),core::ptr::null_mut(),0); cd!(dpll3_clkdm,"dpll3_clkdm","dpll3_pwrdm",0,0,core::ptr::null_mut(),core::ptr::null_mut(),0); cd!(dpll4_clkdm,"dpll4_clkdm","dpll4_pwrdm",0,0,core::ptr::null_mut(),core::ptr::null_mut(),0); cd!(dpll5_clkdm,"dpll5_clkdm","dpll5_pwrdm",0,0,core::ptr::null_mut(),core::ptr::null_mut(),0);

static mut clkdm_autodeps: [clkdm_autodep;3] = [clkdm_autodep{clkdm:powerdomain{name:s!("mpu_clkdm")}},clkdm_autodep{clkdm:powerdomain{name:s!("iva2_clkdm")}},clkdm_autodep{clkdm:powerdomain{name:NULL}}];
static mut clkdm_am35x_autodeps: [clkdm_autodep;2] = [clkdm_autodep{clkdm:powerdomain{name:s!("mpu_clkdm")}},clkdm_autodep{clkdm:powerdomain{name:NULL}}];

static mut clockdomains_common: [*mut clockdomain;9] = [unsafe{&mut wkup_common_clkdm},unsafe{&mut neon_clkdm},unsafe{&mut core_l3_3xxx_clkdm},unsafe{&mut core_l4_3xxx_clkdm},unsafe{&mut emu_clkdm},unsafe{&mut dpll1_clkdm},unsafe{&mut dpll3_clkdm},unsafe{&mut dpll4_clkdm},core::ptr::null_mut()];
static mut clockdomains_omap3430: [*mut clockdomain;8] = [unsafe{&mut mpu_3xxx_clkdm},unsafe{&mut iva2_clkdm},unsafe{&mut d2d_clkdm},unsafe{&mut dss_3xxx_clkdm},unsafe{&mut cam_clkdm},unsafe{&mut per_clkdm},unsafe{&mut dpll2_clkdm},core::ptr::null_mut()];
static mut clockdomains_omap3430es1: [*mut clockdomain;2] = [unsafe{&mut gfx_3430es1_clkdm},core::ptr::null_mut()];
static mut clockdomains_omap3430es2plus: [*mut clockdomain;4] = [unsafe{&mut sgx_clkdm},unsafe{&mut dpll5_clkdm},unsafe{&mut usbhost_clkdm},core::ptr::null_mut()];
static mut clockdomains_am35x: [*mut clockdomain;7] = [unsafe{&mut mpu_am35x_clkdm},unsafe{&mut sgx_am35x_clkdm},unsafe{&mut dss_am35x_clkdm},unsafe{&mut per_am35x_clkdm},unsafe{&mut usbhost_am35x_clkdm},unsafe{&mut dpll5_clkdm},core::ptr::null_mut()];

pub unsafe extern "C" fn omap3xxx_clockdomains_init() {
    if !cpu_is_omap34xx() { return; }
    clkdm_register_platform_funcs(&omap3_clkdm_operations); clkdm_register_clkdms(clockdomains_common.as_mut_ptr());
    let rev = omap_rev();
    if rev == AM35XX_REV_ES1_0 || rev == AM35XX_REV_ES1_1 { clkdm_register_clkdms(clockdomains_am35x.as_mut_ptr()); clkdm_register_autodeps(clkdm_am35x_autodeps.as_mut_ptr()); }
    else { clkdm_register_clkdms(clockdomains_omap3430.as_mut_ptr()); let sc = if rev == OMAP3430_REV_ES1_0 { clockdomains_omap3430es1.as_mut_ptr() } else { clockdomains_omap3430es2plus.as_mut_ptr() }; clkdm_register_clkdms(sc); clkdm_register_autodeps(clkdm_autodeps.as_mut_ptr()); }
    clkdm_complete_init();
}

extern "C" { static AM35XX_REV_ES1_0:u32; static AM35XX_REV_ES1_1:u32; static OMAP3430_REV_ES1_0:u32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
