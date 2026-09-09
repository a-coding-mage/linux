// SPDX-License-Identifier: GPL-2.0
/*
 * VBATTB clock driver
 *
 * Copyright (C) 2024 Renesas Electronics Corp.
 */

const VBATTB_BKSCCR: usize = 0x1c;
const VBATTB_BKSCCR_SOSEL: u32 = 6;
const VBATTB_SOSCCR2: usize = 0x24;
const VBATTB_SOSCCR2_SOSTP2: u32 = 0;
const VBATTB_XOSCCR: usize = 0x30;
const VBATTB_XOSCCR_OUTEN: u32 = 16;
const VBATTB_XOSCCR_XSEL: u32 = 0x3;
const VBATTB_XOSCCR_XSEL_4_PF: u32 = 0x0;
const VBATTB_XOSCCR_XSEL_7_PF: u32 = 0x1;
const VBATTB_XOSCCR_XSEL_9_PF: u32 = 0x2;
const VBATTB_XOSCCR_XSEL_12_5_PF: u32 = 0x3;

/* Values supplied by the Renesas r9a08g045-vbattb clock binding. */
const VBATTB_XC: usize = 0;
const VBATTB_XBYP: usize = 1;
const VBATTB_MUX: usize = 2;
const VBATTB_VBATTCLK: usize = 3;

/**
 * struct vbattb_clk - VBATTB clock data structure
 * @base: base address
 * @lock: lock
 */
#[repr(C)]
struct vbattb_clk {
    base: *mut core::ffi::c_void,
    lock: spinlock_t,
}

unsafe fn vbattb_clk_validate_load_capacitance(reg_lc: *mut u32, of_lc: u32) -> i32 {
    match of_lc {
        4000 => *reg_lc = VBATTB_XOSCCR_XSEL_4_PF,
        7000 => *reg_lc = VBATTB_XOSCCR_XSEL_7_PF,
        9000 => *reg_lc = VBATTB_XOSCCR_XSEL_9_PF,
        12500 => *reg_lc = VBATTB_XOSCCR_XSEL_12_5_PF,
        _ => return -EINVAL,
    }

    0
}

unsafe extern "C" fn vbattb_clk_action(data: *mut core::ffi::c_void) {
    let dev = data as *mut device;
    let rstc = dev_get_drvdata(dev) as *mut reset_control;
    let mut ret: i32;

    ret = reset_control_assert(rstc);
    if ret != 0 {
        dev_err(dev, "Failed to de-assert reset!\n");
    }

    ret = pm_runtime_put_sync(dev);
    if ret < 0 {
        dev_err(dev, "Failed to runtime suspend!\n");
    }

    of_clk_del_provider((*dev).of_node);
}

unsafe extern "C" fn vbattb_clk_probe(pdev: *mut platform_device) -> i32 {
    let np = (*pdev).dev.of_node;
    let mut parent_data: clk_parent_data = core::mem::zeroed();
    let mut clk_data: *mut clk_hw_onecell_data;
    let mut parent_hws: [*const clk_hw; 2] = [core::ptr::null(); 2];
    let dev = &mut (*pdev).dev as *mut device;
    let mut rstc: *mut reset_control;
    let mut vbclk: *mut vbattb_clk;
    let mut of_lc: u32 = 4000;
    let mut reg_lc: u32 = 0;
    let mut hw: *mut clk_hw;
    /* 4 clocks are exported: VBATTB_XC, VBATTB_XBYP, VBATTB_MUX, VBATTB_VBATTCLK. */
    let num_clks: u8 = 4;
    let mut ret: i32;

    /* Default to 4pF as this is not needed if external clock device is connected. */
    of_property_read_u32(np, "quartz-load-femtofarads", &mut of_lc);

    ret = vbattb_clk_validate_load_capacitance(&mut reg_lc, of_lc);
    if ret != 0 { return ret; }

    vbclk = devm_kzalloc(dev, core::mem::size_of::<vbattb_clk>(), GFP_KERNEL) as *mut vbattb_clk;
    if vbclk.is_null() { return -ENOMEM; }

    clk_data = devm_kzalloc(dev, struct_size_clk_hw_onecell_data(num_clks), GFP_KERNEL)
        as *mut clk_hw_onecell_data;
    if clk_data.is_null() { return -ENOMEM; }
    (*clk_data).num = num_clks as u32;

    (*vbclk).base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR((*vbclk).base) { return PTR_ERR((*vbclk).base); }

    ret = devm_pm_runtime_enable(dev);
    if ret != 0 { return ret; }

    rstc = devm_reset_control_get_shared(dev, core::ptr::null());
    if IS_ERR(rstc) { return PTR_ERR(rstc); }

    ret = pm_runtime_resume_and_get(dev);
    if ret != 0 { return ret; }

    ret = reset_control_deassert(rstc);
    if ret != 0 {
        pm_runtime_put_sync(dev);
        return ret;
    }

    dev_set_drvdata(dev, rstc as *mut core::ffi::c_void);
    ret = devm_add_action_or_reset(dev, vbattb_clk_action, dev as *mut core::ffi::c_void);
    if ret != 0 { return ret; }

    spin_lock_init(&mut (*vbclk).lock);

    parent_data.fw_name = "rtx";
    hw = devm_clk_hw_register_gate_parent_data(dev, "xc", &parent_data, 0,
        (*vbclk).base.add(VBATTB_SOSCCR2), VBATTB_SOSCCR2_SOSTP2,
        CLK_GATE_SET_TO_DISABLE, &mut (*vbclk).lock);
    if IS_ERR(hw) { return PTR_ERR(hw); }
    (*clk_data).hws[VBATTB_XC] = hw;

    hw = devm_clk_hw_register_fixed_factor_fwname(dev, np, "xbyp", "rtx", 0, 1, 1);
    if IS_ERR(hw) { return PTR_ERR(hw); }
    (*clk_data).hws[VBATTB_XBYP] = hw;

    parent_hws[0] = (*clk_data).hws[VBATTB_XC];
    parent_hws[1] = (*clk_data).hws[VBATTB_XBYP];
    hw = devm_clk_hw_register_mux_parent_hws(dev, "mux", parent_hws.as_ptr(), 2, 0,
        (*vbclk).base.add(VBATTB_BKSCCR), VBATTB_BKSCCR_SOSEL, 1, 0, &mut (*vbclk).lock);
    if IS_ERR(hw) { return PTR_ERR(hw); }
    (*clk_data).hws[VBATTB_MUX] = hw;

    /* Set load capacitance before registering the VBATTCLK clock. */
    {
        let _guard = spin_lock_guard(&mut (*vbclk).lock);
        let reg = (*vbclk).base.add(VBATTB_XOSCCR) as *mut u32;
        let mut val = readl_relaxed(reg);
        val &= !VBATTB_XOSCCR_XSEL;
        val |= reg_lc;
        writel_relaxed(val, reg);
    }

    /* This feeds the RTC counter clock and it needs to stay on. */
    hw = devm_clk_hw_register_gate_parent_hw(dev, "vbattclk", hw, CLK_IS_CRITICAL,
        (*vbclk).base.add(VBATTB_XOSCCR), VBATTB_XOSCCR_OUTEN, 0,
        &mut (*vbclk).lock);
    if IS_ERR(hw) { return PTR_ERR(hw); }
    (*clk_data).hws[VBATTB_VBATTCLK] = hw;

    of_clk_add_hw_provider(np, of_clk_hw_onecell_get, clk_data)
}

#[repr(C)]
static mut vbattb_clk_match: [of_device_id; 2] = [
    of_device_id { compatible: "renesas,r9a08g045-vbattb" },
    of_device_id { compatible: core::ptr::null() },
];

static mut vbattb_clk_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: "renesas-vbattb-clk",
        of_match_table: unsafe { vbattb_clk_match.as_ptr() },
    },
    probe: Some(vbattb_clk_probe),
};

module_platform_driver!(vbattb_clk_driver);

module_description!("Renesas VBATTB Clock Driver");
module_author!("Claudiu Beznea <claudiu.beznea.uj@bp.renesas.com>");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
