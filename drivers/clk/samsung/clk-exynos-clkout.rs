// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2014 Samsung Electronics Co., Ltd.
 * Author: Tomasz Figa <t.figa@samsung.com>
 *
 * Clock driver for Exynos clock output
 */

const DRV_NAME: &[u8] = b"exynos-clkout\0";

const EXYNOS_CLKOUT_NR_CLKS: usize = 1;
const EXYNOS_CLKOUT_PARENTS: usize = 32;

const EXYNOS_PMU_DEBUG_REG: usize = 0xa00;
const EXYNOS_CLKOUT_DISABLE_SHIFT: u32 = 0;
const EXYNOS_CLKOUT_MUX_SHIFT: u32 = 8;
const EXYNOS4_CLKOUT_MUX_MASK: u32 = 0xf;
const EXYNOS5_CLKOUT_MUX_MASK: u32 = 0x1f;

#[repr(C)]
struct ExynosClkout {
    gate: ClkGate,
    mux: ClkMux,
    slock: Spinlock,
    reg: *mut u8,
    np: *mut DeviceNode,
    pmu_debug_save: u32,
    data: ClkHwOnecellData,
}

#[repr(C)]
struct ExynosClkoutVariant {
    mux_mask: u32,
}

static EXYNOS_CLKOUT_EXYNOS4: ExynosClkoutVariant = ExynosClkoutVariant {
    mux_mask: EXYNOS4_CLKOUT_MUX_MASK,
};

static EXYNOS_CLKOUT_EXYNOS5: ExynosClkoutVariant = ExynosClkoutVariant {
    mux_mask: EXYNOS5_CLKOUT_MUX_MASK,
};

static EXYNOS_CLKOUT_IDS: [OfDeviceId; 9] = [
    OfDeviceId { compatible: b"samsung,exynos3250-pmu\0".as_ptr(), data: &EXYNOS_CLKOUT_EXYNOS4 as *const _ as *const core::ffi::c_void },
    OfDeviceId { compatible: b"samsung,exynos4210-pmu\0".as_ptr(), data: &EXYNOS_CLKOUT_EXYNOS4 as *const _ as *const core::ffi::c_void },
    OfDeviceId { compatible: b"samsung,exynos4212-pmu\0".as_ptr(), data: &EXYNOS_CLKOUT_EXYNOS4 as *const _ as *const core::ffi::c_void },
    OfDeviceId { compatible: b"samsung,exynos4412-pmu\0".as_ptr(), data: &EXYNOS_CLKOUT_EXYNOS4 as *const _ as *const core::ffi::c_void },
    OfDeviceId { compatible: b"samsung,exynos5250-pmu\0".as_ptr(), data: &EXYNOS_CLKOUT_EXYNOS5 as *const _ as *const core::ffi::c_void },
    OfDeviceId { compatible: b"samsung,exynos5410-pmu\0".as_ptr(), data: &EXYNOS_CLKOUT_EXYNOS5 as *const _ as *const core::ffi::c_void },
    OfDeviceId { compatible: b"samsung,exynos5420-pmu\0".as_ptr(), data: &EXYNOS_CLKOUT_EXYNOS5 as *const _ as *const core::ffi::c_void },
    OfDeviceId { compatible: b"samsung,exynos5433-pmu\0".as_ptr(), data: &EXYNOS_CLKOUT_EXYNOS5 as *const _ as *const core::ffi::c_void },
    OfDeviceId { compatible: core::ptr::null(), data: core::ptr::null() },
];

/* Device is instantiated as a child of the PMU device without its own node. */
unsafe fn exynos_clkout_match_parent_dev(dev: *mut Device, mux_mask: *mut u32) -> i32 {
    if (*dev).parent.is_null() {
        dev_err(dev, b"not instantiated from MFD\0".as_ptr());
        return -22;
    }

    let matched = of_match_device(EXYNOS_CLKOUT_IDS.as_ptr(), (*dev).parent);
    if matched.is_null() {
        dev_err(dev, b"cannot match parent device\0".as_ptr());
        return -22;
    }

    let variant = (*matched).data as *const ExynosClkoutVariant;
    *mux_mask = (*variant).mux_mask;
    0
}

unsafe fn exynos_clkout_probe(pdev: *mut PlatformDevice) -> i32 {
    let mut parent_names: [*const u8; EXYNOS_CLKOUT_PARENTS] = [core::ptr::null(); EXYNOS_CLKOUT_PARENTS];
    let mut parents: [*mut Clk; EXYNOS_CLKOUT_PARENTS] = [core::ptr::null_mut(); EXYNOS_CLKOUT_PARENTS];
    let mut mux_mask = 0u32;

    let clkout = devm_kzalloc(&mut (*pdev).dev as *mut Device, core::mem::size_of::<ExynosClkout>(), GFP_KERNEL) as *mut ExynosClkout;
    if clkout.is_null() { return -12; }

    let mut ret = exynos_clkout_match_parent_dev(&mut (*pdev).dev, &mut mux_mask);
    if ret != 0 { return ret; }

    (*clkout).np = (*pdev).dev.of_node;
    if (*clkout).np.is_null() { (*clkout).np = (*(*pdev).dev.parent).of_node; }
    platform_set_drvdata(pdev, clkout as *mut core::ffi::c_void);
    spin_lock_init(&mut (*clkout).slock);

    let mut parent_count = 0usize;
    for i in 0..EXYNOS_CLKOUT_PARENTS {
        let name = alloc_name(i);
        parents[i] = of_clk_get_by_name((*clkout).np, name.as_ptr());
        if is_err(parents[i] as *const core::ffi::c_void) {
            parent_names[i] = b"none\0".as_ptr();
            continue;
        }
        parent_names[i] = __clk_get_name(parents[i]);
        parent_count = i + 1;
    }
    if parent_count == 0 { return -22; }

    (*clkout).reg = of_iomap((*clkout).np, 0);
    if (*clkout).reg.is_null() { ret = -19; goto clks_put; }

    (*clkout).gate.reg = (*clkout).reg.add(EXYNOS_PMU_DEBUG_REG);
    (*clkout).gate.bit_idx = EXYNOS_CLKOUT_DISABLE_SHIFT;
    (*clkout).gate.flags = CLK_GATE_SET_TO_DISABLE;
    (*clkout).gate.lock = &mut (*clkout).slock;
    (*clkout).mux.reg = (*clkout).reg.add(EXYNOS_PMU_DEBUG_REG);
    (*clkout).mux.mask = mux_mask;
    (*clkout).mux.shift = EXYNOS_CLKOUT_MUX_SHIFT;
    (*clkout).mux.lock = &mut (*clkout).slock;
    (*clkout).data.num = EXYNOS_CLKOUT_NR_CLKS;
    (*clkout).data.hws[0] = clk_hw_register_composite(core::ptr::null_mut(), b"clkout\0".as_ptr(), parent_names.as_ptr(), parent_count, &mut (*clkout).mux.hw, &CLK_MUX_OPS, core::ptr::null_mut(), core::ptr::null_mut(), &mut (*clkout).gate.hw, &CLK_GATE_OPS, CLK_SET_RATE_PARENT | CLK_SET_RATE_NO_REPARENT);
    if is_err((*clkout).data.hws[0]) { ret = ptr_err((*clkout).data.hws[0]); goto err_unmap; }
    ret = of_clk_add_hw_provider((*clkout).np, of_clk_hw_onecell_get, &mut (*clkout).data);
    if ret != 0 { clk_hw_unregister((*clkout).data.hws[0]); goto err_unmap; }
    return 0;

err_unmap:
    iounmap((*clkout).reg);
clks_put:
    for parent in parents { if !is_err(parent as *const core::ffi::c_void) { clk_put(parent); } }
    dev_err(&mut (*pdev).dev, b"failed to register clkout clock\0".as_ptr());
    ret
}

unsafe fn exynos_clkout_remove(pdev: *mut PlatformDevice) {
    let clkout = platform_get_drvdata(pdev) as *mut ExynosClkout;
    of_clk_del_provider((*clkout).np);
    clk_hw_unregister((*clkout).data.hws[0]);
    iounmap((*clkout).reg);
}

unsafe fn exynos_clkout_suspend(dev: *mut Device) -> i32 {
    let clkout = dev_get_drvdata(dev) as *mut ExynosClkout;
    (*clkout).pmu_debug_save = readl((*clkout).reg.add(EXYNOS_PMU_DEBUG_REG));
    0
}

unsafe fn exynos_clkout_resume(dev: *mut Device) -> i32 {
    let clkout = dev_get_drvdata(dev) as *mut ExynosClkout;
    writel((*clkout).pmu_debug_save, (*clkout).reg.add(EXYNOS_PMU_DEBUG_REG));
    0
}

/* SIMPLE_DEV_PM_OPS(exynos_clkout_pm_ops, exynos_clkout_suspend, exynos_clkout_resume); */
static mut EXYNOS_CLKOUT_DRIVER: PlatformDriver = PlatformDriver {
    driver: Driver { name: DRV_NAME.as_ptr(), pm: core::ptr::null() },
    probe: Some(exynos_clkout_probe),
    remove: Some(exynos_clkout_remove),
};

/* module_platform_driver(exynos_clkout_driver); */
/* MODULE_AUTHOR("Krzysztof Kozlowski <krzk@kernel.org>"); */
/* MODULE_AUTHOR("Tomasz Figa <tomasz.figa@gmail.com>"); */
/* MODULE_DESCRIPTION("Samsung Exynos clock output driver"); */
/* MODULE_ALIAS("platform:" DRV_NAME); */
/* MODULE_LICENSE("GPL"); */

// External kernel types, constants, operations, and functions are supplied by dependencies.
extern "C" {
    type ClkGate; type ClkMux; type Spinlock; type DeviceNode; type Device; type PlatformDevice; type Clk; type ClkHwOnecellData; type OfDeviceId; type PlatformDriver; type Driver;
    static GFP_KERNEL: u32; static CLK_MUX_OPS: ClkOps; static CLK_GATE_OPS: ClkOps;
    static CLK_GATE_SET_TO_DISABLE: u32; static CLK_SET_RATE_PARENT: u32; static CLK_SET_RATE_NO_REPARENT: u32;
    fn devm_kzalloc(dev: *mut Device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn of_match_device(ids: *const OfDeviceId, dev: *mut Device) -> *const OfDeviceId;
    fn dev_err(dev: *mut Device, msg: *const u8); fn platform_set_drvdata(p: *mut PlatformDevice, d: *mut core::ffi::c_void);
    fn spin_lock_init(lock: *mut Spinlock); fn of_clk_get_by_name(np: *mut DeviceNode, name: *const u8) -> *mut Clk;
    fn __clk_get_name(clk: *mut Clk) -> *const u8; fn of_iomap(np: *mut DeviceNode, index: i32) -> *mut u8;
    fn clk_hw_register_composite(a: *mut core::ffi::c_void, name: *const u8, parents: *const *const u8, count: usize, mux: *mut ClkHw, mo: *const ClkOps, x: *mut core::ffi::c_void, y: *mut core::ffi::c_void, gate: *mut ClkHw, go: *const ClkOps, flags: u32) -> *mut ClkHw;
    fn of_clk_add_hw_provider(np: *mut DeviceNode, get: unsafe extern "C" fn(), data: *mut ClkHwOnecellData) -> i32; fn of_clk_hw_onecell_get(); fn clk_hw_unregister(hw: *mut ClkHw); fn iounmap(reg: *mut u8); fn clk_put(clk: *mut Clk);
    fn platform_get_drvdata(p: *mut PlatformDevice) -> *mut core::ffi::c_void; fn of_clk_del_provider(np: *mut DeviceNode); fn dev_get_drvdata(d: *mut Device) -> *mut core::ffi::c_void; fn readl(reg: *mut u8) -> u32; fn writel(value: u32, reg: *mut u8);
}

#[repr(C)] struct ClkHw { _private: [u8; 0] }
#[repr(C)] struct ClkOps { _private: [u8; 0] }
unsafe fn is_err<T>(p: *const T) -> bool { (p as usize) >= (-4095isize as usize) }
unsafe fn ptr_err<T>(p: *const T) -> i32 { p as isize as i32 }
fn alloc_name(i: usize) -> [u8; 16] { let mut n = [0u8; 16]; n[..7].copy_from_slice(b"clkout"); let s = i.to_string(); n[6..6+s.len()].copy_from_slice(s.as_bytes()); n }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
