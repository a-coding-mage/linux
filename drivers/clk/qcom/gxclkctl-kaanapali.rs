// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
 */

// Translated from the Linux kernel C implementation.  The following names
// are supplied by the corresponding kernel and Qualcomm clock dependencies.

#[repr(C)]
pub struct Gdsc {
    pub gdscr: u32,
    pub en_rest_wait_val: u32,
    pub en_few_wait_val: u32,
    pub clk_dis_wait_val: u32,
    pub pd: GdscPowerDomain,
    pub pwrsts: u32,
    pub flags: u32,
}

#[repr(C)]
pub struct GdscPowerDomain {
    pub name: *const core::ffi::c_char,
    pub power_on: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i32>,
    pub power_off: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i32>,
}

#[repr(C)]
pub struct RegmapConfig {
    pub reg_bits: u32,
    pub reg_stride: u32,
    pub val_bits: u32,
    pub max_register: u32,
    pub fast_io: bool,
}

#[repr(C)]
pub struct QcomCcDesc {
    pub config: *const RegmapConfig,
    pub gdscs: *const *mut Gdsc,
    pub num_gdscs: usize,
    pub use_rpm: bool,
}

#[repr(C)]
pub struct OfDeviceId {
    pub compatible: *const core::ffi::c_char,
}

#[repr(C)]
pub struct PlatformDevice {
    pub dev: core::ffi::c_void,
}

#[repr(C)]
pub struct PlatformDriver {
    pub probe: Option<unsafe extern "C" fn(*mut PlatformDevice) -> i32>,
    pub driver: Driver,
}

#[repr(C)]
pub struct Driver {
    pub name: *const core::ffi::c_char,
    pub of_match_table: *const OfDeviceId,
}

// External kernel symbols and constants supplied by included headers.
extern "C" {
    fn gdsc_gx_do_nothing_enable(dev: *mut core::ffi::c_void) -> i32;
    fn gdsc_gx_disable(dev: *mut core::ffi::c_void) -> i32;
    fn qcom_cc_probe(pdev: *mut PlatformDevice, desc: *const QcomCcDesc) -> i32;
    fn pm_runtime_disable(dev: *mut core::ffi::c_void);
    fn module_platform_driver(driver: *mut PlatformDriver);
}

const GX_CLKCTL_GX_GDSC: usize = 0;
const PWRSTS_OFF_ON: u32 = 0;
const POLL_CFG_GDSCR: u32 = 0;
const RETAIN_FF_ENABLE: u32 = 0;

enum {
    DT_BI_TCXO,
}

static mut GX_CLKCTL_GX_GDSC: Gdsc = Gdsc {
    gdscr: 0x4024,
    en_rest_wait_val: 0x2,
    en_few_wait_val: 0x2,
    clk_dis_wait_val: 0xf,
    pd: GdscPowerDomain {
        name: b"gx_clkctl_gx_gdsc\0".as_ptr() as *const core::ffi::c_char,
        power_on: Some(gdsc_gx_do_nothing_enable),
        power_off: Some(gdsc_gx_disable),
    },
    pwrsts: PWRSTS_OFF_ON,
    flags: POLL_CFG_GDSCR | RETAIN_FF_ENABLE,
};

static mut GX_CLKCTL_GDSCS: [*mut Gdsc; 1] = [core::ptr::addr_of_mut!(GX_CLKCTL_GX_GDSC)];

static GX_CLKCTL_REGMAP_CONFIG: RegmapConfig = RegmapConfig {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: 0x4038,
    fast_io: true,
};

static GX_CLKCTL_KAANAPALI_DESC: QcomCcDesc = QcomCcDesc {
    config: &GX_CLKCTL_REGMAP_CONFIG,
    gdscs: unsafe { GX_CLKCTL_GDSCS.as_ptr() },
    num_gdscs: 1,
    use_rpm: true,
};

static GX_CLKCTL_KAANAPALI_MATCH_TABLE: [OfDeviceId; 5] = [
    OfDeviceId { compatible: b"qcom,glymur-gxclkctl\0".as_ptr() as *const core::ffi::c_char },
    OfDeviceId { compatible: b"qcom,kaanapali-gxclkctl\0".as_ptr() as *const core::ffi::c_char },
    OfDeviceId { compatible: b"qcom,milos-gxclkctl\0".as_ptr() as *const core::ffi::c_char },
    OfDeviceId { compatible: b"qcom,sm8750-gxclkctl\0".as_ptr() as *const core::ffi::c_char },
    OfDeviceId { compatible: core::ptr::null() },
];

unsafe extern "C" fn gx_clkctl_kaanapali_probe(pdev: *mut PlatformDevice) -> i32 {
    let ret = qcom_cc_probe(pdev, &GX_CLKCTL_KAANAPALI_DESC);
    if ret != 0 {
        return ret;
    }

    pm_runtime_disable(core::ptr::addr_of_mut!((*pdev).dev));
    ret
}

static mut GX_CLKCTL_KAANAPALI_DRIVER: PlatformDriver = PlatformDriver {
    probe: Some(gx_clkctl_kaanapali_probe),
    driver: Driver {
        name: b"gxclkctl-kaanapali\0".as_ptr() as *const core::ffi::c_char,
        of_match_table: GX_CLKCTL_KAANAPALI_MATCH_TABLE.as_ptr(),
    },
};

// module_platform_driver(gx_clkctl_kaanapali_driver);
// MODULE_DESCRIPTION("QTI GXCLKCTL Kaanapali Driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
