// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2013 Samsung Electronics Co., Ltd.
 * Author: Padmavathi Venna <padma.v@samsung.com>
 *
 * Common Clock Framework support for Audio Subsystem Clock Controller.
 */

// Linux kernel dependencies and dt-bindings are supplied externally.

static mut LOCK: Spinlock = DEFINE_SPINLOCK!();
static mut REG_BASE: *mut core::ffi::c_void = core::ptr::null_mut();
static mut CLK_DATA: *mut ClkHwOnecellData = core::ptr::null_mut();
/*
 * On Exynos5420 this will be a clock which has to be enabled before any
 * access to audss registers. Typically a child of EPLL.
 *
 * On other platforms this will be -ENODEV.
 */
static mut EPLL: *mut Clk = core::ptr::null_mut();

const ASS_CLK_SRC: usize = 0x0;
const ASS_CLK_DIV: usize = 0x4;
const ASS_CLK_GATE: usize = 0x8;

static mut REG_SAVE: [[u64; 2]; 3] = [
    [ASS_CLK_SRC as u64, 0],
    [ASS_CLK_DIV as u64, 0],
    [ASS_CLK_GATE as u64, 0],
];

unsafe fn exynos_audss_clk_suspend(_dev: *mut Device) -> i32 {
    for i in 0..REG_SAVE.len() {
        REG_SAVE[i][1] = readl(REG_BASE.add(REG_SAVE[i][0] as usize)) as u64;
    }
    0
}

unsafe fn exynos_audss_clk_resume(_dev: *mut Device) -> i32 {
    for i in 0..REG_SAVE.len() {
        writel(REG_SAVE[i][1] as u32, REG_BASE.add(REG_SAVE[i][0] as usize));
    }
    0
}

#[repr(C)]
struct ExynosAudssClkDrvdata {
    has_adma_clk: u32,
    has_mst_clk: u32,
    enable_epll: u32,
    num_clks: u32,
}

static EXYNOS4210_DRVDATA: ExynosAudssClkDrvdata = ExynosAudssClkDrvdata {
    num_clks: EXYNOS_AUDSS_MAX_CLKS - 1,
    enable_epll: 1,
    has_adma_clk: 0,
    has_mst_clk: 0,
};
static EXYNOS5410_DRVDATA: ExynosAudssClkDrvdata = ExynosAudssClkDrvdata {
    num_clks: EXYNOS_AUDSS_MAX_CLKS - 1,
    has_mst_clk: 1,
    has_adma_clk: 0,
    enable_epll: 0,
};
static EXYNOS5420_DRVDATA: ExynosAudssClkDrvdata = ExynosAudssClkDrvdata {
    num_clks: EXYNOS_AUDSS_MAX_CLKS,
    has_adma_clk: 1,
    enable_epll: 1,
    has_mst_clk: 0,
};

static EXYNOS_AUDSS_CLK_OF_MATCH: [OfDeviceId; 5] = [
    OfDeviceId { compatible: "samsung,exynos4210-audss-clock", data: &EXYNOS4210_DRVDATA },
    OfDeviceId { compatible: "samsung,exynos5250-audss-clock", data: &EXYNOS4210_DRVDATA },
    OfDeviceId { compatible: "samsung,exynos5410-audss-clock", data: &EXYNOS5410_DRVDATA },
    OfDeviceId { compatible: "samsung,exynos5420-audss-clock", data: &EXYNOS5420_DRVDATA },
    OfDeviceId { compatible: "", data: core::ptr::null() },
];

unsafe fn exynos_audss_clk_teardown() {
    let mut i = EXYNOS_MOUT_AUDSS;
    while i < EXYNOS_DOUT_SRP {
        if !IS_ERR((*CLK_DATA).hws[i]) { clk_hw_unregister_mux((*CLK_DATA).hws[i]); }
        i += 1;
    }
    while i < EXYNOS_SRP_CLK {
        if !IS_ERR((*CLK_DATA).hws[i]) { clk_hw_unregister_divider((*CLK_DATA).hws[i]); }
        i += 1;
    }
    while i < (*CLK_DATA).num {
        if !IS_ERR((*CLK_DATA).hws[i]) { clk_hw_unregister_gate((*CLK_DATA).hws[i]); }
        i += 1;
    }
}

// The probe, remove, PM, platform-driver, and module declarations retain the
// Linux kernel interfaces and are expressed using their externally supplied
// Rust bindings.
unsafe fn exynos_audss_clk_probe(pdev: *mut PlatformDevice) -> i32 {
    let mut mout_audss_p = ["fin_pll", "fout_epll"];
    let mut mout_i2s_p = ["mout_audss", "cdclk0", "sclk_audio0"];
    let mut sclk_pcm_p = "sclk_pcm0";
    let dev = &mut (*pdev).dev;
    let variant = of_device_get_match_data(dev);
    if variant.is_null() { return -EINVAL; }
    REG_BASE = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(REG_BASE) { return PTR_ERR(REG_BASE); }
    EPLL = ERR_PTR(-ENODEV);
    CLK_DATA = devm_kzalloc(dev, struct_size::<ClkHwOnecellData>(EXYNOS_AUDSS_MAX_CLKS), GFP_KERNEL) as *mut ClkHwOnecellData;
    if CLK_DATA.is_null() { return -ENOMEM; }
    (*CLK_DATA).num = (*variant).num_clks;
    let clk_table = (*CLK_DATA).hws;
    let pll_ref = devm_clk_get(dev, "pll_ref");
    let pll_in = devm_clk_get(dev, "pll_in");
    if !IS_ERR(pll_ref) { mout_audss_p[0] = __clk_get_name(pll_ref); }
    if !IS_ERR(pll_in) {
        mout_audss_p[1] = __clk_get_name(pll_in);
        if (*variant).enable_epll != 0 {
            EPLL = pll_in;
            let ret = clk_prepare_enable(EPLL);
            if ret != 0 { dev_err(dev, "failed to prepare the epll clock\n"); return ret; }
        }
    }
    pm_runtime_get_noresume(dev); pm_runtime_set_active(dev); pm_runtime_enable(dev);
    clk_table[EXYNOS_MOUT_AUDSS] = clk_hw_register_mux(dev, "mout_audss", mout_audss_p.as_ptr(), 2, CLK_SET_RATE_NO_REPARENT | CLK_SET_RATE_PARENT, REG_BASE.add(ASS_CLK_SRC), 0, 1, 0, &mut LOCK);
    let cdclk = devm_clk_get(dev, "cdclk"); let sclk_audio = devm_clk_get(dev, "sclk_audio");
    if !IS_ERR(cdclk) { mout_i2s_p[1] = __clk_get_name(cdclk); }
    if !IS_ERR(sclk_audio) { mout_i2s_p[2] = __clk_get_name(sclk_audio); }
    clk_table[EXYNOS_MOUT_I2S] = clk_hw_register_mux(dev, "mout_i2s", mout_i2s_p.as_ptr(), 3, CLK_SET_RATE_NO_REPARENT, REG_BASE.add(ASS_CLK_SRC), 2, 2, 0, &mut LOCK);
    clk_table[EXYNOS_DOUT_SRP] = clk_hw_register_divider(dev, "dout_srp", "mout_audss", CLK_SET_RATE_PARENT, REG_BASE.add(ASS_CLK_DIV), 0, 4, 0, &mut LOCK);
    clk_table[EXYNOS_DOUT_AUD_BUS] = clk_hw_register_divider(dev, "dout_aud_bus", "dout_srp", CLK_SET_RATE_PARENT, REG_BASE.add(ASS_CLK_DIV), 4, 4, 0, &mut LOCK);
    clk_table[EXYNOS_DOUT_I2S] = clk_hw_register_divider(dev, "dout_i2s", "mout_i2s", 0, REG_BASE.add(ASS_CLK_DIV), 8, 4, 0, &mut LOCK);
    clk_table[EXYNOS_SRP_CLK] = clk_hw_register_gate(dev, "srp_clk", "dout_srp", CLK_SET_RATE_PARENT, REG_BASE.add(ASS_CLK_GATE), 0, 0, &mut LOCK);
    clk_table[EXYNOS_I2S_BUS] = clk_hw_register_gate(dev, "i2s_bus", "dout_aud_bus", CLK_SET_RATE_PARENT, REG_BASE.add(ASS_CLK_GATE), 2, 0, &mut LOCK);
    clk_table[EXYNOS_SCLK_I2S] = clk_hw_register_gate(dev, "sclk_i2s", "dout_i2s", CLK_SET_RATE_PARENT, REG_BASE.add(ASS_CLK_GATE), 3, 0, &mut LOCK);
    clk_table[EXYNOS_PCM_BUS] = clk_hw_register_gate(dev, "pcm_bus", "sclk_pcm", CLK_SET_RATE_PARENT, REG_BASE.add(ASS_CLK_GATE), 4, 0, &mut LOCK);
    let sclk_pcm_in = devm_clk_get(dev, "sclk_pcm_in"); if !IS_ERR(sclk_pcm_in) { sclk_pcm_p = __clk_get_name(sclk_pcm_in); }
    clk_table[EXYNOS_SCLK_PCM] = clk_hw_register_gate(dev, "sclk_pcm", sclk_pcm_p, CLK_SET_RATE_PARENT, REG_BASE.add(ASS_CLK_GATE), 5, 0, &mut LOCK);
    if (*variant).has_adma_clk != 0 { clk_table[EXYNOS_ADMA] = clk_hw_register_gate(dev, "adma", "dout_srp", CLK_SET_RATE_PARENT, REG_BASE.add(ASS_CLK_GATE), 9, 0, &mut LOCK); }
    for i in 0..(*CLK_DATA).num { if IS_ERR(clk_table[i]) { dev_err(dev, "failed to register clock %d\n", i); exynos_audss_clk_teardown(); pm_runtime_put_sync(dev); pm_runtime_disable(dev); if !IS_ERR(EPLL) { clk_disable_unprepare(EPLL); } return PTR_ERR(clk_table[i]); } }
    let ret = of_clk_add_hw_provider((*dev).of_node, of_clk_hw_onecell_get, CLK_DATA); if ret != 0 { exynos_audss_clk_teardown(); pm_runtime_put_sync(dev); pm_runtime_disable(dev); if !IS_ERR(EPLL) { clk_disable_unprepare(EPLL); } return ret; }
    pm_runtime_put_sync(dev); 0
}

unsafe fn exynos_audss_clk_remove(pdev: *mut PlatformDevice) {
    of_clk_del_provider((*pdev).dev.of_node);
    exynos_audss_clk_teardown();
    pm_runtime_disable(&mut (*pdev).dev);
    if !IS_ERR(EPLL) { clk_disable_unprepare(EPLL); }
}

static EXYNOS_AUDSS_CLK_PM_OPS: DevPmOps = DevPmOps {
    runtime_suspend: Some(exynos_audss_clk_suspend),
    runtime_resume: Some(exynos_audss_clk_resume),
    suspend: Some(pm_runtime_force_suspend),
    resume: Some(pm_runtime_force_resume),
};

static mut EXYNOS_AUDSS_CLK_DRIVER: PlatformDriver = PlatformDriver {
    driver: Driver {
        name: "exynos-audss-clk",
        of_match_table: EXYNOS_AUDSS_CLK_OF_MATCH.as_ptr(),
        pm: &EXYNOS_AUDSS_CLK_PM_OPS,
    },
    probe: Some(exynos_audss_clk_probe),
    remove: Some(exynos_audss_clk_remove),
};

module_platform_driver!(EXYNOS_AUDSS_CLK_DRIVER);

module_author!("Padmavathi Venna <padma.v@samsung.com>");
module_description!("Exynos Audio Subsystem Clock Controller");
module_license!("GPL v2");
module_alias!("platform:exynos-audss-clk");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
