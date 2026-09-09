// SPDX-License-Identifier: GPL-2.0-only
// Dependencies supplied by the Linux clock, module, platform, units, and
// Marvell PXA1908 clock-binding headers, plus the local clk.h header.

const MPMU_UART_PLL: usize = 0x14;
const MPMU_NR_CLKS: usize = 39;

#[repr(C)]
struct Pxa1908ClkUnit {
    unit: MmpClkUnit,
    base: *mut core::ffi::c_void,
}

// External types and functions supplied by the included kernel headers.
#[repr(C)]
struct MmpClkUnit {
    _private: [u8; 0],
}

#[repr(C)]
struct MmpParamFixedRateClk {
    id: u32,
    name: *const core::ffi::c_char,
    parent_name: *const core::ffi::c_char,
    flags: u32,
    rate: u32,
}

#[repr(C)]
struct MmpParamFixedFactorClk {
    id: u32,
    name: *const core::ffi::c_char,
    parent_name: *const core::ffi::c_char,
    mult: u32,
    div: u32,
    flags: u32,
}

#[repr(C)]
struct U32Fract {
    numerator: u32,
    denominator: u32,
}

#[repr(C)]
struct MmpClkFactorMasks {
    factor: u32,
    num_mask: u32,
    den_mask: u32,
    num_shift: u32,
    den_shift: u32,
}

#[repr(C)]
struct PlatformDevice {
    dev: Device,
}

#[repr(C)]
struct Device {
    _private: [u8; 0],
}

#[repr(C)]
struct OfDeviceId {
    compatible: *const core::ffi::c_char,
}

#[repr(C)]
struct PlatformDriver {
    probe: Option<unsafe extern "C" fn(*mut PlatformDevice) -> i32>,
    driver: Driver,
}

#[repr(C)]
struct Driver {
    name: *const core::ffi::c_char,
    of_match_table: *const OfDeviceId,
}

extern "C" {
    fn devm_kzalloc(dev: *mut Device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn devm_platform_ioremap_resource(
        pdev: *mut PlatformDevice,
        index: u32,
    ) -> *mut core::ffi::c_void;
    fn mmp_clk_init(node: *mut core::ffi::c_void, unit: *mut MmpClkUnit, nr_clks: usize);
    fn mmp_register_fixed_rate_clks(
        unit: *mut MmpClkUnit,
        clks: *mut MmpParamFixedRateClk,
        count: usize,
    );
    fn mmp_register_fixed_factor_clks(
        unit: *mut MmpClkUnit,
        clks: *mut MmpParamFixedFactorClk,
        count: usize,
    );
    fn mmp_clk_register_factor(
        name: *const core::ffi::c_char,
        parent_name: *const core::ffi::c_char,
        flags: u32,
        base: *mut core::ffi::c_void,
        masks: *const MmpClkFactorMasks,
        table: *mut U32Fract,
        count: usize,
        lock: *mut core::ffi::c_void,
    );
}

// Clock-binding constants supplied by dt-bindings/clock/marvell,pxa1908.h.
extern "C" {
    static PXA1908_CLK_CLK32: u32;
    static PXA1908_CLK_VCTCXO: u32;
    static PXA1908_CLK_PLL1_624: u32;
    static PXA1908_CLK_PLL1_416: u32;
    static PXA1908_CLK_PLL1_499: u32;
    static PXA1908_CLK_PLL1_832: u32;
    static PXA1908_CLK_PLL1_1248: u32;
    static PXA1908_CLK_PLL1_D2: u32;
    static PXA1908_CLK_PLL1_D4: u32;
    static PXA1908_CLK_PLL1_D6: u32;
    static PXA1908_CLK_PLL1_D8: u32;
    static PXA1908_CLK_PLL1_D12: u32;
    static PXA1908_CLK_PLL1_D13: u32;
    static PXA1908_CLK_PLL1_D16: u32;
    static PXA1908_CLK_PLL1_D24: u32;
    static PXA1908_CLK_PLL1_D48: u32;
    static PXA1908_CLK_PLL1_D96: u32;
    static PXA1908_CLK_PLL1_32: u32;
    static PXA1908_CLK_PLL1_208: u32;
    static PXA1908_CLK_PLL1_117: u32;
}

const CLK_SET_RATE_PARENT: u32 = 1 << 2;
const HZ_PER_MHZ: u32 = 1_000_000;
const GFP_KERNEL: u32 = 0;

static mut FIXED_RATE_CLKS: [MmpParamFixedRateClk; 7] = [
    MmpParamFixedRateClk { id: 0, name: b"clk32\0".as_ptr() as _, parent_name: core::ptr::null(), flags: 0, rate: 32768 },
    MmpParamFixedRateClk { id: 0, name: b"vctcxo\0".as_ptr() as _, parent_name: core::ptr::null(), flags: 0, rate: 26 * HZ_PER_MHZ },
    MmpParamFixedRateClk { id: 0, name: b"pll1_624\0".as_ptr() as _, parent_name: core::ptr::null(), flags: 0, rate: 624 * HZ_PER_MHZ },
    MmpParamFixedRateClk { id: 0, name: b"pll1_416\0".as_ptr() as _, parent_name: core::ptr::null(), flags: 0, rate: 416 * HZ_PER_MHZ },
    MmpParamFixedRateClk { id: 0, name: b"pll1_499\0".as_ptr() as _, parent_name: core::ptr::null(), flags: 0, rate: 499 * HZ_PER_MHZ },
    MmpParamFixedRateClk { id: 0, name: b"pll1_832\0".as_ptr() as _, parent_name: core::ptr::null(), flags: 0, rate: 832 * HZ_PER_MHZ },
    MmpParamFixedRateClk { id: 0, name: b"pll1_1248\0".as_ptr() as _, parent_name: core::ptr::null(), flags: 0, rate: 1248 * HZ_PER_MHZ },
];

// Fixed-factor clock descriptions; IDs are provided by the binding header.
static mut FIXED_FACTOR_CLKS: [MmpParamFixedFactorClk; 13] = [
    MmpParamFixedFactorClk { id: 0, name: b"pll1_d2\0".as_ptr() as _, parent_name: b"pll1_624\0".as_ptr() as _, mult: 1, div: 2, flags: 0 },
    MmpParamFixedFactorClk { id: 0, name: b"pll1_d4\0".as_ptr() as _, parent_name: b"pll1_d2\0".as_ptr() as _, mult: 1, div: 2, flags: 0 },
    MmpParamFixedFactorClk { id: 0, name: b"pll1_d6\0".as_ptr() as _, parent_name: b"pll1_d2\0".as_ptr() as _, mult: 1, div: 3, flags: 0 },
    MmpParamFixedFactorClk { id: 0, name: b"pll1_d8\0".as_ptr() as _, parent_name: b"pll1_d4\0".as_ptr() as _, mult: 1, div: 2, flags: 0 },
    MmpParamFixedFactorClk { id: 0, name: b"pll1_d12\0".as_ptr() as _, parent_name: b"pll1_d6\0".as_ptr() as _, mult: 1, div: 2, flags: 0 },
    MmpParamFixedFactorClk { id: 0, name: b"pll1_d13\0".as_ptr() as _, parent_name: b"pll1_624\0".as_ptr() as _, mult: 1, div: 13, flags: 0 },
    MmpParamFixedFactorClk { id: 0, name: b"pll1_d16\0".as_ptr() as _, parent_name: b"pll1_d8\0".as_ptr() as _, mult: 1, div: 2, flags: 0 },
    MmpParamFixedFactorClk { id: 0, name: b"pll1_d24\0".as_ptr() as _, parent_name: b"pll1_d12\0".as_ptr() as _, mult: 1, div: 2, flags: 0 },
    MmpParamFixedFactorClk { id: 0, name: b"pll1_d48\0".as_ptr() as _, parent_name: b"pll1_d24\0".as_ptr() as _, mult: 1, div: 2, flags: 0 },
    MmpParamFixedFactorClk { id: 0, name: b"pll1_d96\0".as_ptr() as _, parent_name: b"pll1_d48\0".as_ptr() as _, mult: 1, div: 2, flags: 0 },
    MmpParamFixedFactorClk { id: 0, name: b"pll1_32\0".as_ptr() as _, parent_name: b"pll1_d13\0".as_ptr() as _, mult: 2, div: 3, flags: 0 },
    MmpParamFixedFactorClk { id: 0, name: b"pll1_208\0".as_ptr() as _, parent_name: b"pll1_d2\0".as_ptr() as _, mult: 2, div: 3, flags: 0 },
    MmpParamFixedFactorClk { id: 0, name: b"pll1_117\0".as_ptr() as _, parent_name: b"pll1_624\0".as_ptr() as _, mult: 3, div: 16, flags: 0 },
];

static mut UART_FACTOR_TBL: [U32Fract; 1] = [U32Fract { numerator: 8125, denominator: 1536 }]; // 14.745MHz
static UART_FACTOR_MASKS: MmpClkFactorMasks = MmpClkFactorMasks { factor: 2, num_mask: 0x1fff, den_mask: 0x1fff, num_shift: 16, den_shift: 0 };

unsafe fn pxa1908_pll_init(pxa_unit: *mut Pxa1908ClkUnit) {
    let unit = &mut (*pxa_unit).unit;
    mmp_register_fixed_rate_clks(unit, FIXED_RATE_CLKS.as_mut_ptr(), FIXED_RATE_CLKS.len());
    mmp_register_fixed_factor_clks(unit, FIXED_FACTOR_CLKS.as_mut_ptr(), FIXED_FACTOR_CLKS.len());
    mmp_clk_register_factor(b"uart_pll\0".as_ptr() as _, b"pll1_d4\0".as_ptr() as _, CLK_SET_RATE_PARENT, (*pxa_unit).base.add(MPMU_UART_PLL), &UART_FACTOR_MASKS, UART_FACTOR_TBL.as_mut_ptr(), UART_FACTOR_TBL.len(), core::ptr::null_mut());
}

unsafe extern "C" fn pxa1908_mpmu_probe(pdev: *mut PlatformDevice) -> i32 {
    let pxa_unit = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<Pxa1908ClkUnit>(), GFP_KERNEL) as *mut Pxa1908ClkUnit;
    if pxa_unit.is_null() { return -12; }
    (*pxa_unit).base = devm_platform_ioremap_resource(pdev, 0);
    if (*pxa_unit).base as isize == -1 { return -1; }
    mmp_clk_init(core::ptr::null_mut(), &mut (*pxa_unit).unit, MPMU_NR_CLKS);
    pxa1908_pll_init(pxa_unit);
    0
}

static PXA1908_MPMU_MATCH_TABLE: [OfDeviceId; 2] = [
    OfDeviceId { compatible: b"marvell,pxa1908-mpmu\0".as_ptr() as _ },
    OfDeviceId { compatible: core::ptr::null() },
];

static mut PXA1908_MPMU_DRIVER: PlatformDriver = PlatformDriver {
    probe: Some(pxa1908_mpmu_probe),
    driver: Driver { name: b"pxa1908-mpmu\0".as_ptr() as _, of_match_table: PXA1908_MPMU_MATCH_TABLE.as_ptr() },
};

// MODULE_DEVICE_TABLE(of, pxa1908_mpmu_match_table);
// module_platform_driver(pxa1908_mpmu_driver);
// MODULE_AUTHOR("Duje Mihanović <duje.mihanovic@skole.hr>");
// MODULE_DESCRIPTION("Marvell PXA1908 MPMU Clock Driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
