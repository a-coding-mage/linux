// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2016 Chen-Yu Tsai. All rights reserved.
 */

// Linux kernel and CCU dependencies are supplied externally.

static fe0_clk: SunxiCcuGate = SUNXI_CCU_GATE!("fe0", "fe0-div", 0x00, BIT!(0), 0);
static fe1_clk: SunxiCcuGate = SUNXI_CCU_GATE!("fe1", "fe1-div", 0x00, BIT!(1), 0);
static fe2_clk: SunxiCcuGate = SUNXI_CCU_GATE!("fe2", "fe2-div", 0x00, BIT!(2), 0);
static iep_deu0_clk: SunxiCcuGate = SUNXI_CCU_GATE!("iep-deu0", "de", 0x00, BIT!(4), 0);
static iep_deu1_clk: SunxiCcuGate = SUNXI_CCU_GATE!("iep-deu1", "de", 0x00, BIT!(5), 0);
static be0_clk: SunxiCcuGate = SUNXI_CCU_GATE!("be0", "be0-div", 0x00, BIT!(8), 0);
static be1_clk: SunxiCcuGate = SUNXI_CCU_GATE!("be1", "be1-div", 0x00, BIT!(9), 0);
static be2_clk: SunxiCcuGate = SUNXI_CCU_GATE!("be2", "be2-div", 0x00, BIT!(10), 0);
static iep_drc0_clk: SunxiCcuGate = SUNXI_CCU_GATE!("iep-drc0", "de", 0x00, BIT!(12), 0);
static iep_drc1_clk: SunxiCcuGate = SUNXI_CCU_GATE!("iep-drc1", "de", 0x00, BIT!(13), 0);
static merge_clk: SunxiCcuGate = SUNXI_CCU_GATE!("merge", "de", 0x00, BIT!(20), 0);

static dram_fe0_clk: SunxiCcuGate = SUNXI_CCU_GATE!("dram-fe0", "sdram", 0x04, BIT!(0), 0);
static dram_fe1_clk: SunxiCcuGate = SUNXI_CCU_GATE!("dram-fe1", "sdram", 0x04, BIT!(1), 0);
static dram_fe2_clk: SunxiCcuGate = SUNXI_CCU_GATE!("dram-fe2", "sdram", 0x04, BIT!(2), 0);
static dram_deu0_clk: SunxiCcuGate = SUNXI_CCU_GATE!("dram-deu0", "sdram", 0x04, BIT!(4), 0);
static dram_deu1_clk: SunxiCcuGate = SUNXI_CCU_GATE!("dram-deu1", "sdram", 0x04, BIT!(5), 0);
static dram_be0_clk: SunxiCcuGate = SUNXI_CCU_GATE!("dram-be0", "sdram", 0x04, BIT!(8), 0);
static dram_be1_clk: SunxiCcuGate = SUNXI_CCU_GATE!("dram-be1", "sdram", 0x04, BIT!(9), 0);
static dram_be2_clk: SunxiCcuGate = SUNXI_CCU_GATE!("dram-be2", "sdram", 0x04, BIT!(10), 0);
static dram_drc0_clk: SunxiCcuGate = SUNXI_CCU_GATE!("dram-drc0", "sdram", 0x04, BIT!(12), 0);
static dram_drc1_clk: SunxiCcuGate = SUNXI_CCU_GATE!("dram-drc1", "sdram", 0x04, BIT!(13), 0);

static bus_fe0_clk: SunxiCcuGate = SUNXI_CCU_GATE!("bus-fe0", "bus-de", 0x08, BIT!(0), 0);
static bus_fe1_clk: SunxiCcuGate = SUNXI_CCU_GATE!("bus-fe1", "bus-de", 0x08, BIT!(1), 0);
static bus_fe2_clk: SunxiCcuGate = SUNXI_CCU_GATE!("bus-fe2", "bus-de", 0x08, BIT!(2), 0);
static bus_deu0_clk: SunxiCcuGate = SUNXI_CCU_GATE!("bus-deu0", "bus-de", 0x08, BIT!(4), 0);
static bus_deu1_clk: SunxiCcuGate = SUNXI_CCU_GATE!("bus-deu1", "bus-de", 0x08, BIT!(5), 0);
static bus_be0_clk: SunxiCcuGate = SUNXI_CCU_GATE!("bus-be0", "bus-de", 0x08, BIT!(8), 0);
static bus_be1_clk: SunxiCcuGate = SUNXI_CCU_GATE!("bus-be1", "bus-de", 0x08, BIT!(9), 0);
static bus_be2_clk: SunxiCcuGate = SUNXI_CCU_GATE!("bus-be2", "bus-de", 0x08, BIT!(10), 0);
static bus_drc0_clk: SunxiCcuGate = SUNXI_CCU_GATE!("bus-drc0", "bus-de", 0x08, BIT!(12), 0);
static bus_drc1_clk: SunxiCcuGate = SUNXI_CCU_GATE!("bus-drc1", "bus-de", 0x08, BIT!(13), 0);

static fe0_div_clk: SunxiCcuM = SUNXI_CCU_M!("fe0-div", "de", 0x20, 0, 4, 0);
static fe1_div_clk: SunxiCcuM = SUNXI_CCU_M!("fe1-div", "de", 0x20, 4, 4, 0);
static fe2_div_clk: SunxiCcuM = SUNXI_CCU_M!("fe2-div", "de", 0x20, 8, 4, 0);
static be0_div_clk: SunxiCcuM = SUNXI_CCU_M!("be0-div", "de", 0x20, 16, 4, 0);
static be1_div_clk: SunxiCcuM = SUNXI_CCU_M!("be1-div", "de", 0x20, 20, 4, 0);
static be2_div_clk: SunxiCcuM = SUNXI_CCU_M!("be2-div", "de", 0x20, 24, 4, 0);

static mut sun9i_a80_de_clks: [*const CcuCommon; 37] = [
    &fe0_clk.common, &fe1_clk.common, &fe2_clk.common,
    &iep_deu0_clk.common, &iep_deu1_clk.common,
    &be0_clk.common, &be1_clk.common, &be2_clk.common,
    &iep_drc0_clk.common, &iep_drc1_clk.common, &merge_clk.common,
    &dram_fe0_clk.common, &dram_fe1_clk.common, &dram_fe2_clk.common,
    &dram_deu0_clk.common, &dram_deu1_clk.common,
    &dram_be0_clk.common, &dram_be1_clk.common, &dram_be2_clk.common,
    &dram_drc0_clk.common, &dram_drc1_clk.common,
    &bus_fe0_clk.common, &bus_fe1_clk.common, &bus_fe2_clk.common,
    &bus_deu0_clk.common, &bus_deu1_clk.common,
    &bus_be0_clk.common, &bus_be1_clk.common, &bus_be2_clk.common,
    &bus_drc0_clk.common, &bus_drc1_clk.common,
    &fe0_div_clk.common, &fe1_div_clk.common, &fe2_div_clk.common,
    &be0_div_clk.common, &be1_div_clk.common, &be2_div_clk.common,
];

static mut sun9i_a80_de_hw_clks: ClkHwOnecellData = ClkHwOnecellData {
    hws: [
        [CLK_FE0] = &fe0_clk.common.hw, [CLK_FE1] = &fe1_clk.common.hw,
        [CLK_FE2] = &fe2_clk.common.hw, [CLK_IEP_DEU0] = &iep_deu0_clk.common.hw,
        [CLK_IEP_DEU1] = &iep_deu1_clk.common.hw, [CLK_BE0] = &be0_clk.common.hw,
        [CLK_BE1] = &be1_clk.common.hw, [CLK_BE2] = &be2_clk.common.hw,
        [CLK_IEP_DRC0] = &iep_drc0_clk.common.hw, [CLK_IEP_DRC1] = &iep_drc1_clk.common.hw,
        [CLK_MERGE] = &merge_clk.common.hw,
        [CLK_DRAM_FE0] = &dram_fe0_clk.common.hw, [CLK_DRAM_FE1] = &dram_fe1_clk.common.hw,
        [CLK_DRAM_FE2] = &dram_fe2_clk.common.hw, [CLK_DRAM_DEU0] = &dram_deu0_clk.common.hw,
        [CLK_DRAM_DEU1] = &dram_deu1_clk.common.hw, [CLK_DRAM_BE0] = &dram_be0_clk.common.hw,
        [CLK_DRAM_BE1] = &dram_be1_clk.common.hw, [CLK_DRAM_BE2] = &dram_be2_clk.common.hw,
        [CLK_DRAM_DRC0] = &dram_drc0_clk.common.hw, [CLK_DRAM_DRC1] = &dram_drc1_clk.common.hw,
        [CLK_BUS_FE0] = &bus_fe0_clk.common.hw, [CLK_BUS_FE1] = &bus_fe1_clk.common.hw,
        [CLK_BUS_FE2] = &bus_fe2_clk.common.hw, [CLK_BUS_DEU0] = &bus_deu0_clk.common.hw,
        [CLK_BUS_DEU1] = &bus_deu1_clk.common.hw, [CLK_BUS_BE0] = &bus_be0_clk.common.hw,
        [CLK_BUS_BE1] = &bus_be1_clk.common.hw, [CLK_BUS_BE2] = &bus_be2_clk.common.hw,
        [CLK_BUS_DRC0] = &bus_drc0_clk.common.hw, [CLK_BUS_DRC1] = &bus_drc1_clk.common.hw,
        [CLK_FE0_DIV] = &fe0_div_clk.common.hw, [CLK_FE1_DIV] = &fe1_div_clk.common.hw,
        [CLK_FE2_DIV] = &fe2_div_clk.common.hw, [CLK_BE0_DIV] = &be0_div_clk.common.hw,
        [CLK_BE1_DIV] = &be1_div_clk.common.hw, [CLK_BE2_DIV] = &be2_div_clk.common.hw,
    ],
    num: CLK_NUMBER,
};

static sun9i_a80_de_resets: [CcuResetMap; 11] = [
    [RST_FE0] = CcuResetMap { reg: 0x0c, bit: BIT!(0) },
    [RST_FE1] = CcuResetMap { reg: 0x0c, bit: BIT!(1) },
    [RST_FE2] = CcuResetMap { reg: 0x0c, bit: BIT!(2) },
    [RST_DEU0] = CcuResetMap { reg: 0x0c, bit: BIT!(4) },
    [RST_DEU1] = CcuResetMap { reg: 0x0c, bit: BIT!(5) },
    [RST_BE0] = CcuResetMap { reg: 0x0c, bit: BIT!(8) },
    [RST_BE1] = CcuResetMap { reg: 0x0c, bit: BIT!(9) },
    [RST_BE2] = CcuResetMap { reg: 0x0c, bit: BIT!(10) },
    [RST_DRC0] = CcuResetMap { reg: 0x0c, bit: BIT!(12) },
    [RST_DRC1] = CcuResetMap { reg: 0x0c, bit: BIT!(13) },
    [RST_MERGE] = CcuResetMap { reg: 0x0c, bit: BIT!(20) },
];

static sun9i_a80_de_clk_desc: SunxiCcuDesc = SunxiCcuDesc {
    ccu_clks: sun9i_a80_de_clks.as_ptr(), num_ccu_clks: sun9i_a80_de_clks.len(),
    hw_clks: &sun9i_a80_de_hw_clks, resets: sun9i_a80_de_resets.as_ptr(),
    num_resets: sun9i_a80_de_resets.len(),
};

unsafe fn sun9i_a80_de_clk_probe(pdev: *mut PlatformDevice) -> i32 {
    let mut bus_clk: *mut Clk;
    let mut rstc: *mut ResetControl;
    let reg: *mut core::ffi::c_void;
    let mut ret: i32;

    reg = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR!(reg) { return PTR_ERR!(reg); }
    bus_clk = devm_clk_get(&mut (*pdev).dev, "bus");
    if IS_ERR!(bus_clk) { return dev_err_probe(&mut (*pdev).dev, PTR_ERR!(bus_clk), "Couldn't get bus clk\n"); }
    rstc = devm_reset_control_get_exclusive(&mut (*pdev).dev, core::ptr::null());
    if IS_ERR!(rstc) { return dev_err_probe(&mut (*pdev).dev, PTR_ERR!(rstc), "Couldn't get reset control\n"); }
    ret = clk_prepare_enable(bus_clk);
    if ret != 0 { dev_err!(&mut (*pdev).dev, "Couldn't enable bus clk: %d\n", ret); return ret; }
    ret = reset_control_deassert(rstc);
    if ret != 0 { dev_err!(&mut (*pdev).dev, "Couldn't deassert reset control: %d\n", ret); goto!(err_disable_clk); }
    ret = devm_sunxi_ccu_probe(&mut (*pdev).dev, reg, &sun9i_a80_de_clk_desc);
    if ret != 0 { goto!(err_assert_reset); }
    return 0;
    err_assert_reset: reset_control_assert(rstc);
    err_disable_clk: clk_disable_unprepare(bus_clk); ret
}

static sun9i_a80_de_clk_ids: [OfDeviceId; 2] = [
    OfDeviceId { compatible: "allwinner,sun9i-a80-de-clks" }, OfDeviceId::default(),
];

static sun9i_a80_de_clk_driver: PlatformDriver = platform_driver! {
    probe: sun9i_a80_de_clk_probe,
    name: "sun9i-a80-de-clks", suppress_bind_attrs: true,
    of_match_table: sun9i_a80_de_clk_ids,
};

module_platform_driver!(sun9i_a80_de_clk_driver);
MODULE_IMPORT_NS!("SUNXI_CCU");
MODULE_DESCRIPTION!("Support for the Allwinner A80 Display Engine CCU");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
