// SPDX-License-Identifier: GPL-2.0
/*
 * r8a77990 Clock Pulse Generator / Module Standby and Software Reset
 *
 * Copyright (C) 2018-2019 Renesas Electronics Corp.
 *
 * Based on r8a7795-cpg-mssr.c
 *
 * Copyright (C) 2015 Glider bvba
 * Copyright (C) 2015 Renesas Electronics Corp.
 */

// Linux and device-tree dependencies are supplied by the surrounding tree.

#[repr(C)]
enum ClkIds {
    /* Core Clock Outputs exported to DT */
    LAST_DT_CORE_CLK = R8A77990_CLK_CPEX,
    /* External Input Clocks */
    CLK_EXTAL,
    /* Internal Core Clocks */
    CLK_MAIN,
    CLK_PLL0,
    CLK_PLL1,
    CLK_PLL3,
    CLK_PLL0D4,
    CLK_PLL0D6,
    CLK_PLL0D8,
    CLK_PLL0D20,
    CLK_PLL0D24,
    CLK_PLL1D2,
    CLK_PE,
    CLK_S0,
    CLK_S1,
    CLK_S2,
    CLK_S3,
    CLK_SDSRC,
    CLK_RPCSRC,
    CLK_RINT,
    CLK_OCO,
    /* Module Clocks */
    MOD_CLK_BASE,
}

static R8A77990_CORE_CLKS: [CpgCoreClk; _] = [
    def_input!("extal", CLK_EXTAL),
    def_base!(".main", CLK_MAIN, CLK_TYPE_GEN3_MAIN, CLK_EXTAL),
    def_base!(".pll1", CLK_PLL1, CLK_TYPE_GEN3_PLL1, CLK_MAIN),
    def_base!(".pll3", CLK_PLL3, CLK_TYPE_GEN3_PLL3, CLK_MAIN),
    def_fixed!(".pll0", CLK_PLL0, CLK_MAIN, 1, 100),
    def_fixed!(".pll0d4", CLK_PLL0D4, CLK_PLL0, 4, 1),
    def_fixed!(".pll0d6", CLK_PLL0D6, CLK_PLL0, 6, 1),
    def_fixed!(".pll0d8", CLK_PLL0D8, CLK_PLL0, 8, 1),
    def_fixed!(".pll0d20", CLK_PLL0D20, CLK_PLL0, 20, 1),
    def_fixed!(".pll0d24", CLK_PLL0D24, CLK_PLL0, 24, 1),
    def_fixed!(".pll1d2", CLK_PLL1D2, CLK_PLL1, 2, 1),
    def_fixed!(".pe", CLK_PE, CLK_PLL0D20, 1, 1),
    def_fixed!(".s0", CLK_S0, CLK_PLL1, 2, 1),
    def_fixed!(".s1", CLK_S1, CLK_PLL1, 3, 1),
    def_fixed!(".s2", CLK_S2, CLK_PLL1, 4, 1),
    def_fixed!(".s3", CLK_S3, CLK_PLL1, 6, 1),
    def_fixed!(".sdsrc", CLK_SDSRC, CLK_PLL1, 2, 1),
    def_fixed_rpcsrc_e3!(".rpcsrc", CLK_RPCSRC, CLK_PLL0, CLK_PLL1),
    def_div6_ro!(".r", CLK_RINT, CLK_EXTAL, CPG_RCKCR, 32),
    def_rate!(".oco", CLK_OCO, 8 * 1000 * 1000),
    def_fixed!("za2", R8A77990_CLK_ZA2, CLK_PLL0D24, 1, 1),
    def_fixed!("za8", R8A77990_CLK_ZA8, CLK_PLL0D8, 1, 1),
    def_gen3_z!("z2", R8A77990_CLK_Z2, CLK_TYPE_GEN3_Z, CLK_PLL0, 4, 8),
    def_fixed!("ztr", R8A77990_CLK_ZTR, CLK_PLL1, 6, 1),
    def_fixed!("zt", R8A77990_CLK_ZT, CLK_PLL1, 4, 1),
    def_fixed!("zx", R8A77990_CLK_ZX, CLK_PLL1, 3, 1),
    def_fixed!("s0d1", R8A77990_CLK_S0D1, CLK_S0, 1, 1),
    def_fixed!("s0d3", R8A77990_CLK_S0D3, CLK_S0, 3, 1),
    def_fixed!("s0d6", R8A77990_CLK_S0D6, CLK_S0, 6, 1),
    def_fixed!("s0d12", R8A77990_CLK_S0D12, CLK_S0, 12, 1),
    def_fixed!("s0d24", R8A77990_CLK_S0D24, CLK_S0, 24, 1),
    def_fixed!("s1d1", R8A77990_CLK_S1D1, CLK_S1, 1, 1),
    def_fixed!("s1d2", R8A77990_CLK_S1D2, CLK_S1, 2, 1),
    def_fixed!("s1d4", R8A77990_CLK_S1D4, CLK_S1, 4, 1),
    def_fixed!("s2d1", R8A77990_CLK_S2D1, CLK_S2, 1, 1),
    def_fixed!("s2d2", R8A77990_CLK_S2D2, CLK_S2, 2, 1),
    def_fixed!("s2d4", R8A77990_CLK_S2D4, CLK_S2, 4, 1),
    def_fixed!("s3d1", R8A77990_CLK_S3D1, CLK_S3, 1, 1),
    def_fixed!("s3d2", R8A77990_CLK_S3D2, CLK_S3, 2, 1),
    def_fixed!("s3d4", R8A77990_CLK_S3D4, CLK_S3, 4, 1),
    def_gen3_sdh!("sd0h", R8A77990_CLK_SD0H, CLK_SDSRC, 0x0074),
    def_gen3_sdh!("sd1h", R8A77990_CLK_SD1H, CLK_SDSRC, 0x0078),
    def_gen3_sdh!("sd3h", R8A77990_CLK_SD3H, CLK_SDSRC, 0x026c),
    def_gen3_sd!("sd0", R8A77990_CLK_SD0, R8A77990_CLK_SD0H, 0x0074),
    def_gen3_sd!("sd1", R8A77990_CLK_SD1, R8A77990_CLK_SD1H, 0x0078),
    def_gen3_sd!("sd3", R8A77990_CLK_SD3, R8A77990_CLK_SD3H, 0x026c),
    def_base!("rpc", R8A77990_CLK_RPC, CLK_TYPE_GEN3_RPC, CLK_RPCSRC),
    def_base!("rpcd2", R8A77990_CLK_RPCD2, CLK_TYPE_GEN3_RPCD2, R8A77990_CLK_RPC),
    def_fixed!("cl", R8A77990_CLK_CL, CLK_PLL1, 48, 1),
    def_fixed!("cr", R8A77990_CLK_CR, CLK_PLL1D2, 2, 1),
    def_fixed!("cp", R8A77990_CLK_CP, CLK_EXTAL, 2, 1),
    def_fixed!("cpex", R8A77990_CLK_CPEX, CLK_EXTAL, 4, 1),
    def_div6_ro!("osc", R8A77990_CLK_OSC, CLK_EXTAL, CPG_RCKCR, 8),
    def_gen3_pe!("s0d6c", R8A77990_CLK_S0D6C, CLK_S0, 6, CLK_PE, 2),
    def_gen3_pe!("s3d1c", R8A77990_CLK_S3D1C, CLK_S3, 1, CLK_PE, 1),
    def_gen3_pe!("s3d2c", R8A77990_CLK_S3D2C, CLK_S3, 2, CLK_PE, 2),
    def_gen3_pe!("s3d4c", R8A77990_CLK_S3D4C, CLK_S3, 4, CLK_PE, 4),
    def_div6p1!("canfd", R8A77990_CLK_CANFD, CLK_PLL0D6, 0x244),
    def_div6p1!("csi0", R8A77990_CLK_CSI0, CLK_PLL1D2, 0x00c),
    def_div6p1!("mso", R8A77990_CLK_MSO, CLK_PLL1D2, 0x014),
    def_gen3_rcksel!("r", R8A77990_CLK_R, CLK_RINT, 1, CLK_OCO, 61 * 4),
];

// Module-clock table.  Each entry preserves the original DEF_MOD(name, id, parent).
static R8A77990_MOD_CLKS: [MssrModClk; _] = [
    def_mod!("tmu4", 121, R8A77990_CLK_S0D6C), def_mod!("tmu3", 122, R8A77990_CLK_S3D2C),
    def_mod!("tmu2", 123, R8A77990_CLK_S3D2C), def_mod!("tmu1", 124, R8A77990_CLK_S3D2C),
    def_mod!("tmu0", 125, R8A77990_CLK_CP), def_mod!("scif5", 202, R8A77990_CLK_S3D4C),
    def_mod!("scif4", 203, R8A77990_CLK_S3D4C), def_mod!("scif3", 204, R8A77990_CLK_S3D4C),
    def_mod!("scif1", 206, R8A77990_CLK_S3D4C), def_mod!("scif0", 207, R8A77990_CLK_S3D4C),
    def_mod!("msiof3", 208, R8A77990_CLK_MSO), def_mod!("msiof2", 209, R8A77990_CLK_MSO),
    def_mod!("msiof1", 210, R8A77990_CLK_MSO), def_mod!("msiof0", 211, R8A77990_CLK_MSO),
    def_mod!("sys-dmac2", 217, R8A77990_CLK_S3D1), def_mod!("sys-dmac1", 218, R8A77990_CLK_S3D1),
    def_mod!("sys-dmac0", 219, R8A77990_CLK_S3D1), def_mod!("sceg-pub", 229, R8A77990_CLK_CR),
    def_mod!("cmt3",300,R8A77990_CLK_R), def_mod!("cmt2",301,R8A77990_CLK_R), def_mod!("cmt1",302,R8A77990_CLK_R), def_mod!("cmt0",303,R8A77990_CLK_R),
    def_mod!("scif2",310,R8A77990_CLK_S3D4C), def_mod!("sdif3",311,R8A77990_CLK_SD3), def_mod!("sdif1",313,R8A77990_CLK_SD1), def_mod!("sdif0",314,R8A77990_CLK_SD0),
    def_mod!("pcie0",319,R8A77990_CLK_S3D1), def_mod!("usb3-if0",328,R8A77990_CLK_S3D1), def_mod!("usb-dmac0",330,R8A77990_CLK_S3D1), def_mod!("usb-dmac1",331,R8A77990_CLK_S3D1),
    def_mod!("rwdt",402,R8A77990_CLK_R), def_mod!("intc-ex",407,R8A77990_CLK_CP), def_mod!("intc-ap",408,R8A77990_CLK_S0D3),
    def_mod!("audmac0",502,R8A77990_CLK_S1D2), def_mod!("drif31",508,R8A77990_CLK_S3D2), def_mod!("drif30",509,R8A77990_CLK_S3D2), def_mod!("drif21",510,R8A77990_CLK_S3D2), def_mod!("drif20",511,R8A77990_CLK_S3D2), def_mod!("drif11",512,R8A77990_CLK_S3D2), def_mod!("drif10",513,R8A77990_CLK_S3D2), def_mod!("drif01",514,R8A77990_CLK_S3D2), def_mod!("drif00",515,R8A77990_CLK_S3D2),
    def_mod!("hscif4",516,R8A77990_CLK_S3D1C), def_mod!("hscif3",517,R8A77990_CLK_S3D1C), def_mod!("hscif2",518,R8A77990_CLK_S3D1C), def_mod!("hscif1",519,R8A77990_CLK_S3D1C), def_mod!("hscif0",520,R8A77990_CLK_S3D1C), def_mod!("thermal",522,R8A77990_CLK_CP), def_mod!("pwm",523,R8A77990_CLK_S3D4C),
    def_mod!("fcpvd1",602,R8A77990_CLK_S1D2), def_mod!("fcpvd0",603,R8A77990_CLK_S1D2), def_mod!("fcpvb0",607,R8A77990_CLK_S0D1), def_mod!("fcpvi0",611,R8A77990_CLK_S0D1), def_mod!("fcpf0",615,R8A77990_CLK_S0D1), def_mod!("fcpcs",619,R8A77990_CLK_S0D1), def_mod!("vspd1",622,R8A77990_CLK_S1D2), def_mod!("vspd0",623,R8A77990_CLK_S1D2), def_mod!("vspb",626,R8A77990_CLK_S0D1), def_mod!("vspi0",631,R8A77990_CLK_S0D1),
    def_mod!("ehci0",703,R8A77990_CLK_S3D2), def_mod!("hsusb",704,R8A77990_CLK_S3D2), def_mod!("cmm1",710,R8A77990_CLK_S1D1), def_mod!("cmm0",711,R8A77990_CLK_S1D1), def_mod!("csi40",716,R8A77990_CLK_CSI0), def_mod!("du1",723,R8A77990_CLK_S1D1), def_mod!("du0",724,R8A77990_CLK_S1D1), def_mod!("lvds",727,R8A77990_CLK_S2D1),
    def_mod!("mlp",802,R8A77990_CLK_S2D1), def_mod!("vin5",806,R8A77990_CLK_S1D2), def_mod!("vin4",807,R8A77990_CLK_S1D2), def_mod!("etheravb",812,R8A77990_CLK_S3D2),
    def_mod!("gpio6",906,R8A77990_CLK_S3D4), def_mod!("gpio5",907,R8A77990_CLK_S3D4), def_mod!("gpio4",908,R8A77990_CLK_S3D4), def_mod!("gpio3",909,R8A77990_CLK_S3D4), def_mod!("gpio2",910,R8A77990_CLK_S3D4), def_mod!("gpio1",911,R8A77990_CLK_S3D4), def_mod!("gpio0",912,R8A77990_CLK_S3D4), def_mod!("can-fd",914,R8A77990_CLK_S3D2), def_mod!("can-if1",915,R8A77990_CLK_S3D4), def_mod!("can-if0",916,R8A77990_CLK_S3D4), def_mod!("rpc-if",917,R8A77990_CLK_RPCD2), def_mod!("i2c6",918,R8A77990_CLK_S3D2), def_mod!("i2c5",919,R8A77990_CLK_S3D2), def_mod!("adg",922,R8A77990_CLK_ZA2), def_mod!("i2c-dvfs",926,R8A77990_CLK_CP), def_mod!("i2c4",927,R8A77990_CLK_S3D2), def_mod!("i2c3",928,R8A77990_CLK_S3D2), def_mod!("i2c2",929,R8A77990_CLK_S3D2), def_mod!("i2c1",930,R8A77990_CLK_S3D2), def_mod!("i2c0",931,R8A77990_CLK_S3D2),
    def_mod!("i2c7",1003,R8A77990_CLK_S3D2), def_mod!("ssi-all",1005,R8A77990_CLK_S3D4),
    def_mod!("ssi9",1006,mod_clk_id!(1005)), def_mod!("ssi8",1007,mod_clk_id!(1005)), def_mod!("ssi7",1008,mod_clk_id!(1005)), def_mod!("ssi6",1009,mod_clk_id!(1005)), def_mod!("ssi5",1010,mod_clk_id!(1005)), def_mod!("ssi4",1011,mod_clk_id!(1005)), def_mod!("ssi3",1012,mod_clk_id!(1005)), def_mod!("ssi2",1013,mod_clk_id!(1005)), def_mod!("ssi1",1014,mod_clk_id!(1005)), def_mod!("ssi0",1015,mod_clk_id!(1005)), def_mod!("dab",1016,R8A77990_CLK_S3D1), def_mod!("scu-all",1017,R8A77990_CLK_S3D4),
    def_mod!("scu-dvc1",1018,mod_clk_id!(1017)), def_mod!("scu-dvc0",1019,mod_clk_id!(1017)), def_mod!("scu-ctu1-mix1",1020,mod_clk_id!(1017)), def_mod!("scu-ctu0-mix0",1021,mod_clk_id!(1017)), def_mod!("scu-src9",1022,mod_clk_id!(1017)), def_mod!("scu-src8",1023,mod_clk_id!(1017)), def_mod!("scu-src7",1024,mod_clk_id!(1017)), def_mod!("scu-src6",1025,mod_clk_id!(1017)), def_mod!("scu-src5",1026,mod_clk_id!(1017)), def_mod!("scu-src4",1027,mod_clk_id!(1017)), def_mod!("scu-src3",1028,mod_clk_id!(1017)), def_mod!("scu-src2",1029,mod_clk_id!(1017)), def_mod!("scu-src1",1030,mod_clk_id!(1017)), def_mod!("scu-src0",1031,mod_clk_id!(1017)),
];

static R8A77990_CRIT_MOD_CLKS: [u32; 2] = [mod_clk_id!(402), mod_clk_id!(408)];

/* CPG Clock Data */
/* MD19 EXTAL (MHz) PLL0 PLL1 PLL3: 0: 48x1 x100/1 x100/3 x100/3; 1: 48x1 x100/1 x100/3 x58/3 */
const CPG_PLL_CONFIG_INDEX: fn(u32) -> usize = |md: u32| (((md & BIT!(19)) >> 19) as usize);

static CPG_PLL_CONFIGS: [RcarGen3CpgPllConfig; 2] = [
    RcarGen3CpgPllConfig { extal_div: 1, pll1_mult: 100, pll1_div: 3, pll3_mult: 100, pll3_div: 3 },
    RcarGen3CpgPllConfig { extal_div: 1, pll1_mult: 100, pll1_div: 3, pll3_mult: 58, pll3_div: 3 },
];

unsafe fn r8a77990_cpg_mssr_init(dev: *mut Device) -> i32 {
    let mut cpg_mode: u32 = 0;
    let error = rcar_rst_read_mode_pins(&mut cpg_mode);
    if error != 0 { return error; }
    let cpg_pll_config = &CPG_PLL_CONFIGS[CPG_PLL_CONFIG_INDEX(cpg_mode)];
    rcar_gen3_cpg_init(cpg_pll_config, 0, cpg_mode)
}

static R8A77990_CPG_MSSR_INFO: CpgMssrInfo = CpgMssrInfo {
    core_clks: R8A77990_CORE_CLKS.as_ptr(),
    num_core_clks: R8A77990_CORE_CLKS.len(),
    last_dt_core_clk: LAST_DT_CORE_CLK,
    num_total_core_clks: MOD_CLK_BASE,
    mod_clks: R8A77990_MOD_CLKS.as_ptr(),
    num_mod_clks: R8A77990_MOD_CLKS.len(),
    num_hw_mod_clks: 12 * 32,
    crit_mod_clks: R8A77990_CRIT_MOD_CLKS.as_ptr(),
    num_crit_mod_clks: R8A77990_CRIT_MOD_CLKS.len(),
    init: Some(r8a77990_cpg_mssr_init),
    cpg_clk_register: Some(rcar_gen3_cpg_clk_register),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
