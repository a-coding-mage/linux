// SPDX-License-Identifier: GPL-2.0-only
/*
 * Purna Chandra Mandal,<purna.mandal@microchip.com>
 * Copyright (C) 2015 Microchip Technology Inc.  All rights reserved.
 */
// Dependencies supplied by the kernel and clk-pic32 headers are intentionally external.

const OSC_FRCDIV_MASK: u32 = 0x07;
const OSC_FRCDIV_SHIFT: u32 = 24;
const PLL_ICLK_MASK: u32 = 0x01;
const PLL_ICLK_SHIFT: u32 = 7;

const fn declare_peripheral_clock(name: &'static str, reg: u32, flags: u32) -> pic32_periph_clk_data {
    pic32_periph_clk_data {
        ctrl_reg: reg,
        init_data: clk_init_data {
            name,
            parent_names: ["sys_clk"].as_ptr(),
            num_parents: 1,
            ops: unsafe { &pic32_pbclk_ops },
            flags,
        },
    }
}

const fn declare_refo_clock(id: &'static str, reg: u32, parent_names: &'static [&'static str], parent_map: &'static [u32]) -> pic32_ref_osc_data {
    pic32_ref_osc_data {
        ctrl_reg: reg,
        init_data: clk_init_data {
            name: id,
            parent_names: parent_names.as_ptr(),
            num_parents: 9,
            flags: CLK_SET_RATE_GATE | CLK_SET_PARENT_GATE,
            ops: unsafe { &pic32_roclk_ops },
        },
        parent_map: parent_map.as_ptr(),
    }
}

static REFO1_PARENTS: [&str; 9] = ["sys_clk", "pb1_clk", "posc_clk", "frc_clk", "lprc_clk", "sosc_clk", "sys_pll", "refi1_clk", "bfrc_clk"];
static REFO2_PARENTS: [&str; 9] = ["sys_clk", "pb1_clk", "posc_clk", "frc_clk", "lprc_clk", "sosc_clk", "sys_pll", "refi2_clk", "bfrc_clk"];
static REFO3_PARENTS: [&str; 9] = ["sys_clk", "pb1_clk", "posc_clk", "frc_clk", "lprc_clk", "sosc_clk", "sys_pll", "refi3_clk", "bfrc_clk"];
static REFO4_PARENTS: [&str; 9] = ["sys_clk", "pb1_clk", "posc_clk", "frc_clk", "lprc_clk", "sosc_clk", "sys_pll", "refi4_clk", "bfrc_clk"];
static REFO5_PARENTS: [&str; 9] = ["sys_clk", "pb1_clk", "posc_clk", "frc_clk", "lprc_clk", "sosc_clk", "sys_pll", "refi5_clk", "bfrc_clk"];
static REFO_PARENT_MAP: [u32; 9] = [0, 1, 2, 3, 4, 5, 7, 8, 9];

static REF_CLKS: [pic32_ref_osc_data; 5] = [
    declare_refo_clock("refo1_clk", 0x80, &REFO1_PARENTS, &REFO_PARENT_MAP),
    declare_refo_clock("refo2_clk", 0xa0, &REFO2_PARENTS, &REFO_PARENT_MAP),
    declare_refo_clock("refo3_clk", 0xc0, &REFO3_PARENTS, &REFO_PARENT_MAP),
    declare_refo_clock("refo4_clk", 0xe0, &REFO4_PARENTS, &REFO_PARENT_MAP),
    declare_refo_clock("refo5_clk", 0x100, &REFO5_PARENTS, &REFO_PARENT_MAP),
];

static PERIPH_CLOCKS: [pic32_periph_clk_data; 7] = [
    declare_peripheral_clock("pb1_clk", 0x140, 0),
    declare_peripheral_clock("pb2_clk", 0x150, CLK_IGNORE_UNUSED),
    declare_peripheral_clock("pb3_clk", 0x160, 0),
    declare_peripheral_clock("pb4_clk", 0x170, 0),
    declare_peripheral_clock("pb5_clk", 0x180, 0),
    declare_peripheral_clock("pb6_clk", 0x190, 0),
    declare_peripheral_clock("cpu_clk", 0x1a0, CLK_IGNORE_UNUSED),
];

static SYS_MUX_PARENTS: [&str; 6] = ["frcdiv_clk", "sys_pll", "posc_clk", "sosc_clk", "lprc_clk", "frcdiv_clk"];
static SYS_MUX_PARENT_MAP: [u32; 6] = [0, 1, 2, 4, 5, 7];
static SYS_MUX_CLK: pic32_sys_clk_data = pic32_sys_clk_data {
    slew_reg: 0x1c0,
    slew_div: 2, // step of div_4 -> div_2 -> no_div
    init_data: clk_init_data { name: "sys_clk", parent_names: SYS_MUX_PARENTS.as_ptr(), num_parents: 6, ops: unsafe { &pic32_sclk_ops } },
    parent_map: SYS_MUX_PARENT_MAP.as_ptr(),
};

static SYS_PLL_PARENTS: [&str; 1] = ["spll_mux_clk"];
static SYS_PLL: pic32_sys_pll_data = pic32_sys_pll_data {
    ctrl_reg: 0x020, status_reg: 0x1d0, lock_mask: BIT(7),
    init_data: clk_init_data { name: "sys_pll", parent_names: SYS_PLL_PARENTS.as_ptr(), num_parents: 1, ops: unsafe { &pic32_spll_ops } },
};

static SOSC_CLK: pic32_sec_osc_data = pic32_sec_osc_data {
    status_reg: 0x1d0, enable_mask: BIT(1), status_mask: BIT(4), fixed_rate: 32768,
    init_data: clk_init_data { name: "sosc_clk", parent_names: core::ptr::null(), ops: unsafe { &pic32_sosc_ops } },
};

static mut PIC32MZDA_CRITICAL_CLKS: [i32; 2] = [PB2CLK, PB7CLK];

#[repr(C)]
struct pic32mzda_clk_data {
    clks: [*mut clk; MAXCLKS],
    core: pic32_clk_common,
    onecell_data: clk_onecell_data,
    failsafe_notifier: notifier_block,
}

unsafe extern "C" {
    static pic32_pbclk_ops: clk_ops;
    static pic32_roclk_ops: clk_ops;
    static pic32_sclk_ops: clk_ops;
    static pic32_spll_ops: clk_ops;
    static pic32_sosc_ops: clk_ops;
}

unsafe extern "C" fn pic32_fscm_nmi(nb: *mut notifier_block, _action: c_ulong, _data: *mut c_void) -> c_int {
    let cd = container_of!(nb, pic32mzda_clk_data, failsafe_notifier);
    if readl((*cd).core.iobase) & BIT(2) != 0 {
        pr_alert!("pic32-clk: FSCM detected clk failure.\n");
    }
    // TODO: detect reason of failure and recover accordingly
    NOTIFY_OK
}

unsafe extern "C" fn pic32mzda_clk_probe(pdev: *mut platform_device) -> c_int {
    let pll_mux_parents: [*const c_char; 2] = [c"posc_clk".as_ptr(), c"frc_clk".as_ptr()];
    let np = (*(*pdev).dev).of_node;
    let cd = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<pic32mzda_clk_data>(), GFP_KERNEL) as *mut pic32mzda_clk_data;
    if cd.is_null() { return -ENOMEM; }
    let core = &mut (*cd).core;
    core.iobase = of_io_request_and_map(np, 0, of_node_full_name(np));
    if IS_ERR(core.iobase) { dev_err!(&(*pdev).dev, "pic32-clk: failed to map registers\n"); return PTR_ERR(core.iobase); }
    spin_lock_init(&mut core.reg_lock);
    core.dev = &mut (*pdev).dev;
    let clks = (*cd).clks.as_mut_ptr();
    *clks.add(POSCCLK as usize) = clk_register_fixed_rate(&mut (*pdev).dev, c"posc_clk".as_ptr(), core::ptr::null(), 0, 24000000);
    *clks.add(FRCCLK as usize) = clk_register_fixed_rate(&mut (*pdev).dev, c"frc_clk".as_ptr(), core::ptr::null(), 0, 8000000);
    *clks.add(BFRCCLK as usize) = clk_register_fixed_rate(&mut (*pdev).dev, c"bfrc_clk".as_ptr(), core::ptr::null(), 0, 8000000);
    *clks.add(LPRCCLK as usize) = clk_register_fixed_rate(&mut (*pdev).dev, c"lprc_clk".as_ptr(), core::ptr::null(), 0, 32000);
    *clks.add(UPLLCLK as usize) = clk_register_fixed_rate(&mut (*pdev).dev, c"usbphy_clk".as_ptr(), core::ptr::null(), 0, 24000000);
    if of_property_read_bool(np, c"microchip,pic32mzda-sosc".as_ptr()) { pr_info!("pic32-clk: dt requests SOSC.\n"); *clks.add(SOSCCLK as usize) = pic32_sosc_clk_register(&SOSC_CLK, core); }
    *clks.add(FRCDIVCLK as usize) = clk_register_divider(&mut (*pdev).dev, c"frcdiv_clk".as_ptr(), c"frc_clk".as_ptr(), 0, core.iobase, OSC_FRCDIV_SHIFT, OSC_FRCDIV_MASK, CLK_DIVIDER_POWER_OF_TWO, &mut core.reg_lock);
    let pll_mux_clk = clk_register_mux(&mut (*pdev).dev, c"spll_mux_clk".as_ptr(), pll_mux_parents.as_ptr(), 2, 0, core.iobase.add(0x020), PLL_ICLK_SHIFT, 1, 0, &mut core.reg_lock);
    if IS_ERR(pll_mux_clk) { pr_err!("spll_mux_clk: clk register failed\n"); }
    *clks.add(PLLCLK as usize) = pic32_spll_clk_register(&SYS_PLL, core);
    *clks.add(SCLK as usize) = pic32_sys_clk_register(&SYS_MUX_CLK, core);
    for i in 0..7 { *clks.add((PB1CLK + i) as usize) = pic32_periph_clk_register(&PERIPH_CLOCKS[i as usize], core); }
    for i in 0..5 { *clks.add((REF1CLK + i) as usize) = pic32_refo_clk_register(&REF_CLKS[i as usize], core); }
    for i in 0..MAXCLKS { if !IS_ERR(*clks.add(i)) { clk_register_clkdev(*clks.add(i), core::ptr::null(), __clk_get_name(*clks.add(i))); } }
    (*cd).onecell_data.clks = clks; (*cd).onecell_data.clk_num = MAXCLKS;
    let ret = of_clk_add_provider(np, of_clk_src_onecell_get, &mut (*cd).onecell_data);
    if ret != 0 { return ret; }
    for i in 0..2 { let clk = *clks.add(PIC32MZDA_CRITICAL_CLKS[i] as usize); if clk_prepare_enable(clk) != 0 { dev_err!(&(*pdev).dev, "clk_prepare_enable(%s) failed\n", __clk_get_name(clk)); } }
    (*cd).failsafe_notifier.notifier_call = Some(pic32_fscm_nmi);
    register_nmi_notifier(&mut (*cd).failsafe_notifier)
}

static PIC32MZDA_CLK_MATCH_TABLE: [of_device_id; 2] = [of_device_id { compatible: c"microchip,pic32mzda-clk".as_ptr() }, of_device_id { compatible: core::ptr::null() }];
static mut PIC32MZDA_CLK_DRIVER: platform_driver = platform_driver { probe: Some(pic32mzda_clk_probe), driver: device_driver { name: c"clk-pic32mzda".as_ptr(), of_match_table: PIC32MZDA_CLK_MATCH_TABLE.as_ptr() } };

unsafe extern "C" fn microchip_pic32mzda_clk_init() -> c_int { platform_driver_register(&mut PIC32MZDA_CLK_DRIVER) }
core_initcall!(microchip_pic32mzda_clk_init);
module_description!("Microchip PIC32MZDA Clock Driver");
module_license!("GPL v2");
module_alias!("platform:clk-pic32mzda");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
