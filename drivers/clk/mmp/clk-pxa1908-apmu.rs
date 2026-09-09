// SPDX-License-Identifier: GPL-2.0-only
// Translated from the Linux kernel C implementation.

// External kernel and device-tree declarations are supplied by other files.

const APMU_CLK_GATE_CTRL: usize = 0x40;
const APMU_CCIC1: usize = 0x24;
const APMU_ISP: usize = 0x38;
const APMU_DSI1: usize = 0x44;
const APMU_DISP1: usize = 0x4c;
const APMU_CCIC0: usize = 0x50;
const APMU_SDH0: usize = 0x54;
const APMU_SDH1: usize = 0x58;
const APMU_USB: usize = 0x5c;
const APMU_NF: usize = 0x60;
const APMU_VPU: usize = 0xa4;
const APMU_GC: usize = 0xcc;
const APMU_SDH2: usize = 0xe0;
const APMU_GC2D: usize = 0xf4;
const APMU_TRACE: usize = 0x108;
const APMU_DVC_DFC_DEBUG: usize = 0x140;

const APMU_NR_CLKS: usize = 17;

#[repr(C)]
struct Pxa1908ClkUnit {
    unit: MmpClkUnit,
    base: *mut core::ffi::c_void,
}

// These types, constants, and functions are provided by the surrounding kernel bindings.
#[repr(C)]
struct MmpClkUnit {
    _private: [u8; 0],
}

#[repr(C)]
struct MmpParamGeneralGateClk {
    _private: [u8; 0],
}

#[repr(C)]
struct MmpParamGateClk {
    _private: [u8; 0],
}

#[repr(C)]
struct MmpClkMixConfig {
    reg_info: MmpClkMixRegInfo,
}

#[repr(C)]
struct MmpClkMixRegInfo {
    reg_clk_ctrl: *mut u8,
}

extern "C" {
    static mut pll1_lock: core::ffi::c_int;
    static mut sdh0_lock: core::ffi::c_int;
    static mut sdh1_lock: core::ffi::c_int;
    static mut sdh2_lock: core::ffi::c_int;
    static PXA1908_CLK_PLL1_D2_GATE: u32;
    static PXA1908_CLK_PLL1_416_GATE: u32;
    static PXA1908_CLK_PLL1_624_GATE: u32;
    static PXA1908_CLK_PLL1_832_GATE: u32;
    static PXA1908_CLK_PLL1_1248_GATE: u32;
    static PXA1908_CLK_USB: u32;
    static PXA1908_CLK_SDH0: u32;
    static PXA1908_CLK_SDH1: u32;
    static PXA1908_CLK_SDH2: u32;

    fn mmp_register_general_gate_clks(
        unit: *mut MmpClkUnit,
        clks: *mut MmpParamGeneralGateClk,
        base: *mut core::ffi::c_void,
        count: usize,
    );
    fn mmp_clk_register_mix(
        dev: *mut core::ffi::c_void,
        name: *const u8,
        parents: *const *const u8,
        parent_count: usize,
        flags: u32,
        config: *mut MmpClkMixConfig,
        lock: *mut core::ffi::c_int,
    );
    fn mmp_register_gate_clks(
        unit: *mut MmpClkUnit,
        clks: *mut MmpParamGateClk,
        base: *mut core::ffi::c_void,
        count: usize,
    );
}

static mut PLL1_GATE_CLKS: [MmpParamGeneralGateClk; 5] = [
    MmpParamGeneralGateClk { _private: [] },
    MmpParamGeneralGateClk { _private: [] },
    MmpParamGeneralGateClk { _private: [] },
    MmpParamGeneralGateClk { _private: [] },
    MmpParamGeneralGateClk { _private: [] },
];

static SDH_PARENT_NAMES: [&[u8]; 2] = [b"pll1_416\0", b"pll1_624\0"];
static mut SDH_MIX_CONFIG: MmpClkMixConfig = MmpClkMixConfig {
    reg_info: MmpClkMixRegInfo { reg_clk_ctrl: core::ptr::null_mut() },
};

static mut APMU_GATE_CLKS: [MmpParamGateClk; 4] = [
    MmpParamGateClk { _private: [] },
    MmpParamGateClk { _private: [] },
    MmpParamGateClk { _private: [] },
    MmpParamGateClk { _private: [] },
];

unsafe fn pxa1908_axi_periph_clk_init(pxa_unit: *mut Pxa1908ClkUnit) {
    let unit = &mut (*pxa_unit).unit as *mut MmpClkUnit;
    mmp_register_general_gate_clks(
        unit,
        PLL1_GATE_CLKS.as_mut_ptr(),
        (*pxa_unit).base,
        PLL1_GATE_CLKS.len(),
    );

    SDH_MIX_CONFIG.reg_info.reg_clk_ctrl = (*pxa_unit).base.add(APMU_SDH0) as *mut u8;
    mmp_clk_register_mix(core::ptr::null_mut(), b"sdh0_mix_clk\0".as_ptr(),
        SDH_PARENT_NAMES.as_ptr() as *const *const u8, SDH_PARENT_NAMES.len(),
        0, &mut SDH_MIX_CONFIG, &mut sdh0_lock);
    SDH_MIX_CONFIG.reg_info.reg_clk_ctrl = (*pxa_unit).base.add(APMU_SDH1) as *mut u8;
    mmp_clk_register_mix(core::ptr::null_mut(), b"sdh1_mix_clk\0".as_ptr(),
        SDH_PARENT_NAMES.as_ptr() as *const *const u8, SDH_PARENT_NAMES.len(),
        0, &mut SDH_MIX_CONFIG, &mut sdh1_lock);
    SDH_MIX_CONFIG.reg_info.reg_clk_ctrl = (*pxa_unit).base.add(APMU_SDH2) as *mut u8;
    mmp_clk_register_mix(core::ptr::null_mut(), b"sdh2_mix_clk\0".as_ptr(),
        SDH_PARENT_NAMES.as_ptr() as *const *const u8, SDH_PARENT_NAMES.len(),
        0, &mut SDH_MIX_CONFIG, &mut sdh2_lock);

    mmp_register_gate_clks(unit, APMU_GATE_CLKS.as_mut_ptr(), (*pxa_unit).base, APMU_GATE_CLKS.len());
}

#[repr(C)]
struct PlatformDevice {
    _private: [u8; 0],
}

extern "C" {
    fn devm_kzalloc(dev: *mut core::ffi::c_void, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn devm_platform_ioremap_resource(
        pdev: *mut PlatformDevice,
        index: u32,
    ) -> *mut core::ffi::c_void;
    fn devm_auxiliary_device_create(
        dev: *mut core::ffi::c_void,
        name: *const u8,
        data: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
    fn mmp_clk_init(
        node: *mut core::ffi::c_void,
        unit: *mut MmpClkUnit,
        count: usize,
    );
}

unsafe fn pxa1908_apmu_probe(pdev: *mut PlatformDevice) -> i32 {
    let pxa_unit = devm_kzalloc(core::ptr::null_mut(), core::mem::size_of::<Pxa1908ClkUnit>(), 0)
        as *mut Pxa1908ClkUnit;
    if pxa_unit.is_null() {
        return -12;
    }

    (*pxa_unit).base = devm_platform_ioremap_resource(pdev, 0);
    if (*pxa_unit).base.is_null() {
        return -1;
    }

    let adev = devm_auxiliary_device_create(core::ptr::null_mut(), b"power\0".as_ptr(), core::ptr::null_mut());
    if adev.is_null() {
        return -1;
    }

    mmp_clk_init(core::ptr::null_mut(), &mut (*pxa_unit).unit, APMU_NR_CLKS);
    pxa1908_axi_periph_clk_init(pxa_unit);
    0
}

#[repr(C)]
struct OfDeviceId {
    compatible: *const u8,
}

static PXA1908_APMU_MATCH_TABLE: [OfDeviceId; 2] = [
    OfDeviceId { compatible: b"marvell,pxa1908-apmu\0".as_ptr() },
    OfDeviceId { compatible: core::ptr::null() },
];

#[repr(C)]
struct PlatformDriver {
    probe: Option<unsafe fn(*mut PlatformDevice) -> i32>,
    name: *const u8,
    of_match_table: *const OfDeviceId,
}

static mut PXA1908_APMU_DRIVER: PlatformDriver = PlatformDriver {
    probe: Some(pxa1908_apmu_probe),
    name: b"pxa1908-apmu\0".as_ptr(),
    of_match_table: PXA1908_APMU_MATCH_TABLE.as_ptr(),
};

// MODULE_DEVICE_TABLE(of, pxa1908_apmu_match_table)
// module_platform_driver(pxa1908_apmu_driver)
// MODULE_AUTHOR("Duje Mihanović <duje.mihanovic@skole.hr>")
// MODULE_DESCRIPTION("Marvell PXA1908 APMU Clock Driver")
// MODULE_LICENSE("GPL")

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
