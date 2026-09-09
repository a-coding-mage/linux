// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2014 Marvell Technology Group Ltd.
 *
 * Sebastian Hesselbarth <sebastian.hesselbarth@gmail.com>
 * Alexandre Belloni <alexandre.belloni@free-electrons.com>
 */

// Linux clock, device-tree, I/O, allocation, and Berlin2 dependencies are
// supplied by the surrounding kernel translation unit.

const REG_PINMUX0: usize = 0x0000;
const REG_PINMUX1: usize = 0x0004;
const REG_SYSPLLCTL0: usize = 0x0014;
const REG_SYSPLLCTL4: usize = 0x0024;
const REG_MEMPLLCTL0: usize = 0x0028;
const REG_MEMPLLCTL4: usize = 0x0038;
const REG_CPUPLLCTL0: usize = 0x003c;
const REG_CPUPLLCTL4: usize = 0x004c;
const REG_AVPLLCTL0: usize = 0x0050;
const REG_AVPLLCTL31: usize = 0x00cc;
const REG_AVPLLCTL62: usize = 0x0148;
const REG_PLLSTATUS: usize = 0x014c;
const REG_CLKENABLE: usize = 0x0150;
const REG_CLKSELECT0: usize = 0x0154;
const REG_CLKSELECT1: usize = 0x0158;
const REG_CLKSELECT2: usize = 0x015c;
const REG_CLKSELECT3: usize = 0x0160;
const REG_CLKSWITCH0: usize = 0x0164;
const REG_CLKSWITCH1: usize = 0x0168;
const REG_RESET_TRIGGER: usize = 0x0178;
const REG_RESET_STATUS0: usize = 0x017c;
const REG_RESET_STATUS1: usize = 0x0180;
const REG_SW_GENERIC0: usize = 0x0184;
const REG_SW_GENERIC3: usize = 0x0190;
const REG_PRODUCTID: usize = 0x01cc;
const REG_PRODUCTID_EXT: usize = 0x01d0;
const REG_GFX3DCORE_CLKCTL: usize = 0x022c;
const REG_GFX3DSYS_CLKCTL: usize = 0x0230;
const REG_ARC_CLKCTL: usize = 0x0234;
const REG_VIP_CLKCTL: usize = 0x0238;
const REG_SDIO0XIN_CLKCTL: usize = 0x023c;
const REG_SDIO1XIN_CLKCTL: usize = 0x0240;
const REG_GFX3DEXTRA_CLKCTL: usize = 0x0244;
const REG_GFX3D_RESET: usize = 0x0248;
const REG_GC360_CLKCTL: usize = 0x024c;
const REG_SDIO_DLLMST_CLKCTL: usize = 0x0250;

const MAX_CLKS: usize = 41;
static mut CLK_DATA: *mut clk_hw_onecell_data = core::ptr::null_mut();
static mut LOCK: spinlock_t = DEFINE_SPINLOCK!();
static mut GBASE: *mut u8 = core::ptr::null_mut();

#[repr(usize)]
enum ClockId {
    REFCLK, VIDEO_EXT0, SYSPLL, MEMPLL, CPUPLL,
    AVPLL_A1, AVPLL_A2, AVPLL_A3, AVPLL_A4, AVPLL_A5, AVPLL_A6, AVPLL_A7, AVPLL_A8,
    AVPLL_B1, AVPLL_B2, AVPLL_B3, AVPLL_B4, AVPLL_B5, AVPLL_B6, AVPLL_B7, AVPLL_B8,
    AUDIO1_PLL, AUDIO_FAST_PLL, VIDEO0_PLL, VIDEO0_IN, VIDEO1_PLL, VIDEO1_IN,
    VIDEO2_PLL, VIDEO2_IN,
}

static mut CLK_NAMES: [&'static str; 29] = [
    "refclk", "video_ext0", "syspll", "mempll", "cpupll",
    "avpll_a1", "avpll_a2", "avpll_a3", "avpll_a4", "avpll_a5", "avpll_a6", "avpll_a7", "avpll_a8",
    "avpll_b1", "avpll_b2", "avpll_b3", "avpll_b4", "avpll_b5", "avpll_b6", "avpll_b7", "avpll_b8",
    "audio1_pll", "audio_fast_pll", "video0_pll", "video0_in", "video1_pll", "video1_in", "video2_pll", "video2_in",
];

static BG2_PLL_MAP: berlin2_pll_map = berlin2_pll_map { vcodiv: [10,15,20,25,30,40,50,60,80], mult: 10, fbdiv_shift: 6, rfdiv_shift: 1, divsel_shift: 7 };
static DEFAULT_PARENT_IDS: [u8; 6] = [SYSPLL as u8, AVPLL_B4 as u8, AVPLL_A5 as u8, AVPLL_B6 as u8, AVPLL_B7 as u8, SYSPLL as u8];

// The following tables directly mirror the C initializers; Berlin2_* macros
// are provided by the translated clock support.
static BG2_DIVS: &[berlin2_div_data] = &[
    berlin2_div_data_named!("sys", &[SYSPLL,AVPLL_B4,AVPLL_B5,AVPLL_B6,AVPLL_B7,SYSPLL], [BERLIN2_DIV_GATE!(REG_CLKENABLE,0),BERLIN2_PLL_SELECT!(REG_CLKSELECT0,0),BERLIN2_DIV_SELECT!(REG_CLKSELECT0,3),BERLIN2_PLL_SWITCH!(REG_CLKSWITCH0,3),BERLIN2_DIV_SWITCH!(REG_CLKSWITCH0,4),BERLIN2_DIV_D3SWITCH!(REG_CLKSWITCH0,5)], BERLIN2_DIV_HAS_GATE|BERLIN2_DIV_HAS_MUX, CLK_IGNORE_UNUSED),
    berlin2_div_data_named!("cpu", &[CPUPLL,MEMPLL,MEMPLL,MEMPLL,MEMPLL], [BERLIN2_PLL_SELECT!(REG_CLKSELECT0,6),BERLIN2_DIV_SELECT!(REG_CLKSELECT0,9),BERLIN2_PLL_SWITCH!(REG_CLKSWITCH0,6),BERLIN2_DIV_SWITCH!(REG_CLKSWITCH0,7),BERLIN2_DIV_D3SWITCH!(REG_CLKSWITCH0,8)], BERLIN2_DIV_HAS_MUX, 0),
    berlin2_div_data_named!("drmfigo", &DEFAULT_PARENT_IDS, [BERLIN2_DIV_GATE!(REG_CLKENABLE,16),BERLIN2_PLL_SELECT!(REG_CLKSELECT0,17),BERLIN2_DIV_SELECT!(REG_CLKSELECT0,20),BERLIN2_PLL_SWITCH!(REG_CLKSWITCH0,12),BERLIN2_DIV_SWITCH!(REG_CLKSWITCH0,13),BERLIN2_DIV_D3SWITCH!(REG_CLKSWITCH0,14)], BERLIN2_DIV_HAS_GATE|BERLIN2_DIV_HAS_MUX, 0),
    berlin2_div_data_named!("cfg", &DEFAULT_PARENT_IDS, [BERLIN2_DIV_GATE!(REG_CLKENABLE,1),BERLIN2_PLL_SELECT!(REG_CLKSELECT0,23),BERLIN2_DIV_SELECT!(REG_CLKSELECT0,26),BERLIN2_PLL_SWITCH!(REG_CLKSWITCH0,15),BERLIN2_DIV_SWITCH!(REG_CLKSWITCH0,16),BERLIN2_DIV_D3SWITCH!(REG_CLKSWITCH0,17)], BERLIN2_DIV_HAS_GATE|BERLIN2_DIV_HAS_MUX, 0),
    berlin2_div_data_named!("gfx", &DEFAULT_PARENT_IDS, [BERLIN2_DIV_GATE!(REG_CLKENABLE,4),BERLIN2_PLL_SELECT!(REG_CLKSELECT0,29),BERLIN2_DIV_SELECT!(REG_CLKSELECT1,0),BERLIN2_PLL_SWITCH!(REG_CLKSWITCH0,18),BERLIN2_DIV_SWITCH!(REG_CLKSWITCH0,19),BERLIN2_DIV_D3SWITCH!(REG_CLKSWITCH0,20)], BERLIN2_DIV_HAS_GATE|BERLIN2_DIV_HAS_MUX, 0),
    berlin2_div_data_named!("zsp", &DEFAULT_PARENT_IDS, [BERLIN2_DIV_GATE!(REG_CLKENABLE,5),BERLIN2_PLL_SELECT!(REG_CLKSELECT1,3),BERLIN2_DIV_SELECT!(REG_CLKSELECT1,6),BERLIN2_PLL_SWITCH!(REG_CLKSWITCH0,21),BERLIN2_DIV_SWITCH!(REG_CLKSWITCH0,22),BERLIN2_DIV_D3SWITCH!(REG_CLKSWITCH0,23)], BERLIN2_DIV_HAS_GATE|BERLIN2_DIV_HAS_MUX, 0),
    berlin2_div_data_named!("perif", &DEFAULT_PARENT_IDS, [BERLIN2_DIV_GATE!(REG_CLKENABLE,6),BERLIN2_PLL_SELECT!(REG_CLKSELECT1,9),BERLIN2_DIV_SELECT!(REG_CLKSELECT1,12),BERLIN2_PLL_SWITCH!(REG_CLKSWITCH0,24),BERLIN2_DIV_SWITCH!(REG_CLKSWITCH0,25),BERLIN2_DIV_D3SWITCH!(REG_CLKSWITCH0,26)], BERLIN2_DIV_HAS_GATE|BERLIN2_DIV_HAS_MUX, CLK_IGNORE_UNUSED),
    berlin2_div_data_named!("pcube", &DEFAULT_PARENT_IDS, [BERLIN2_DIV_GATE!(REG_CLKENABLE,2),BERLIN2_PLL_SELECT!(REG_CLKSELECT1,15),BERLIN2_DIV_SELECT!(REG_CLKSELECT1,18),BERLIN2_PLL_SWITCH!(REG_CLKSWITCH0,27),BERLIN2_DIV_SWITCH!(REG_CLKSWITCH0,28),BERLIN2_DIV_D3SWITCH!(REG_CLKSWITCH0,29)], BERLIN2_DIV_HAS_GATE|BERLIN2_DIV_HAS_MUX, 0),
    berlin2_div_data_named!("vscope", &DEFAULT_PARENT_IDS, [BERLIN2_DIV_GATE!(REG_CLKENABLE,3),BERLIN2_PLL_SELECT!(REG_CLKSELECT1,21),BERLIN2_DIV_SELECT!(REG_CLKSELECT1,24),BERLIN2_PLL_SWITCH!(REG_CLKSWITCH0,30),BERLIN2_DIV_SWITCH!(REG_CLKSWITCH0,31),BERLIN2_DIV_D3SWITCH!(REG_CLKSWITCH1,0)], BERLIN2_DIV_HAS_GATE|BERLIN2_DIV_HAS_MUX, 0),
    berlin2_div_data_named!("nfc_ecc", &DEFAULT_PARENT_IDS, [BERLIN2_DIV_GATE!(REG_CLKENABLE,18),BERLIN2_PLL_SELECT!(REG_CLKSELECT1,27),BERLIN2_DIV_SELECT!(REG_CLKSELECT2,0),BERLIN2_PLL_SWITCH!(REG_CLKSWITCH1,1),BERLIN2_DIV_SWITCH!(REG_CLKSWITCH1,2),BERLIN2_DIV_D3SWITCH!(REG_CLKSWITCH1,3)], BERLIN2_DIV_HAS_GATE|BERLIN2_DIV_HAS_MUX, 0),
    berlin2_div_data_named!("vpp", &DEFAULT_PARENT_IDS, [BERLIN2_DIV_GATE!(REG_CLKENABLE,21),BERLIN2_PLL_SELECT!(REG_CLKSELECT2,3),BERLIN2_DIV_SELECT!(REG_CLKSELECT2,6),BERLIN2_PLL_SWITCH!(REG_CLKSWITCH1,4),BERLIN2_DIV_SWITCH!(REG_CLKSWITCH1,5),BERLIN2_DIV_D3SWITCH!(REG_CLKSWITCH1,6)], BERLIN2_DIV_HAS_GATE|BERLIN2_DIV_HAS_MUX, 0),
    berlin2_div_data_named!("app", &DEFAULT_PARENT_IDS, [BERLIN2_DIV_GATE!(REG_CLKENABLE,20),BERLIN2_PLL_SELECT!(REG_CLKSELECT2,9),BERLIN2_DIV_SELECT!(REG_CLKSELECT2,12),BERLIN2_PLL_SWITCH!(REG_CLKSWITCH1,7),BERLIN2_DIV_SWITCH!(REG_CLKSWITCH1,8),BERLIN2_DIV_D3SWITCH!(REG_CLKSWITCH1,9)], BERLIN2_DIV_HAS_GATE|BERLIN2_DIV_HAS_MUX, 0),
    berlin2_div_data_named!("audio0", &[AUDIO_FAST_PLL], [BERLIN2_DIV_GATE!(REG_CLKENABLE,22),BERLIN2_DIV_SELECT!(REG_CLKSELECT2,17),BERLIN2_DIV_SWITCH!(REG_CLKSWITCH1,10),BERLIN2_DIV_D3SWITCH!(REG_CLKSWITCH1,11)], BERLIN2_DIV_HAS_GATE, 0),
    berlin2_div_data_named!("audio2", &[AUDIO_FAST_PLL], [BERLIN2_DIV_GATE!(REG_CLKENABLE,24),BERLIN2_DIV_SELECT!(REG_CLKSELECT2,20),BERLIN2_DIV_SWITCH!(REG_CLKSWITCH1,14),BERLIN2_DIV_D3SWITCH!(REG_CLKSWITCH1,15)], BERLIN2_DIV_HAS_GATE, 0),
    berlin2_div_data_named!("audio3", &[AUDIO_FAST_PLL], [BERLIN2_DIV_GATE!(REG_CLKENABLE,25),BERLIN2_DIV_SELECT!(REG_CLKSELECT2,23),BERLIN2_DIV_SWITCH!(REG_CLKSWITCH1,16),BERLIN2_DIV_D3SWITCH!(REG_CLKSWITCH1,17)], BERLIN2_DIV_HAS_GATE, 0),
    berlin2_div_data_named!("audio1", &[AUDIO1_PLL], [BERLIN2_DIV_GATE!(REG_CLKENABLE,23),BERLIN2_DIV_SELECT!(REG_CLKSELECT3,0),BERLIN2_DIV_SWITCH!(REG_CLKSWITCH1,12),BERLIN2_DIV_D3SWITCH!(REG_CLKSWITCH1,13)], BERLIN2_DIV_HAS_GATE, 0),
    berlin2_div_single!("gfx3d_core", &DEFAULT_PARENT_IDS, REG_GFX3DCORE_CLKCTL),
    berlin2_div_single!("gfx3d_sys", &DEFAULT_PARENT_IDS, REG_GFX3DSYS_CLKCTL),
    berlin2_div_single!("arc", &DEFAULT_PARENT_IDS, REG_ARC_CLKCTL),
    berlin2_div_single!("vip", &DEFAULT_PARENT_IDS, REG_VIP_CLKCTL),
    berlin2_div_single!("sdio0xin", &DEFAULT_PARENT_IDS, REG_SDIO0XIN_CLKCTL),
    berlin2_div_single!("sdio1xin", &DEFAULT_PARENT_IDS, REG_SDIO1XIN_CLKCTL),
    berlin2_div_single!("gfx3d_extra", &DEFAULT_PARENT_IDS, REG_GFX3DEXTRA_CLKCTL),
    berlin2_div_single!("gc360", &DEFAULT_PARENT_IDS, REG_GC360_CLKCTL),
    berlin2_div_single!("sdio_dllmst", &DEFAULT_PARENT_IDS, REG_SDIO_DLLMST_CLKCTL),
];

static BG2_GATES: &[berlin2_gate_data] = &[
    gate!("geth0","perif",7), gate!("geth1","perif",8), gate!("sata","perif",9), gate!("ahbapb","perif",10,CLK_IGNORE_UNUSED),
    gate!("usb0","perif",11), gate!("usb1","perif",12), gate!("pbridge","perif",13,CLK_IGNORE_UNUSED), gate!("sdio0","perif",14),
    gate!("sdio1","perif",15), gate!("nfc","perif",17), gate!("smemc","perif",19), gate!("audiohd","audiohd_pll",26),
    gate!("video0","video0_in",27), gate!("video1","video1_in",28), gate!("video2","video2_in",29),
];

unsafe fn berlin2_clock_setup(np: *mut device_node) {
    let parent_np = of_get_parent(np);
    let mut parent_names: [&'static str; 9] = [""; 9];
    let mut clk: *mut clk = core::ptr::null_mut();
    let mut hw: *mut clk_hw;
    let mut hws: *mut *mut clk_hw;
    let mut avpll_flags: u8 = 0;
    let mut n: usize;
    let mut ret: i32;

    CLK_DATA = kzalloc_flex(core::ptr::null_mut(), &mut hws, MAX_CLKS);
    if CLK_DATA.is_null() { of_node_put(parent_np); return; }
    (*CLK_DATA).num = MAX_CLKS;
    hws = (*CLK_DATA).hws.as_mut_ptr();
    GBASE = of_iomap(parent_np, 0);
    of_node_put(parent_np);
    if GBASE.is_null() { return; }

    clk = of_clk_get_by_name(np, CLK_NAMES[REFCLK as usize]);
    if !IS_ERR(clk) { CLK_NAMES[REFCLK as usize] = __clk_get_name(clk); clk_put(clk); }
    clk = of_clk_get_by_name(np, CLK_NAMES[VIDEO_EXT0 as usize]);
    if !IS_ERR(clk) { CLK_NAMES[VIDEO_EXT0 as usize] = __clk_get_name(clk); clk_put(clk); }

    ret = berlin2_pll_register(&BG2_PLL_MAP, GBASE.add(REG_SYSPLLCTL0), CLK_NAMES[SYSPLL as usize], CLK_NAMES[REFCLK as usize], 0); if ret != 0 { berlin2_fail(); return; }
    ret = berlin2_pll_register(&BG2_PLL_MAP, GBASE.add(REG_MEMPLLCTL0), CLK_NAMES[MEMPLL as usize], CLK_NAMES[REFCLK as usize], 0); if ret != 0 { berlin2_fail(); return; }
    ret = berlin2_pll_register(&BG2_PLL_MAP, GBASE.add(REG_CPUPLLCTL0), CLK_NAMES[CPUPLL as usize], CLK_NAMES[REFCLK as usize], 0); if ret != 0 { berlin2_fail(); return; }
    if of_device_is_compatible(np, "marvell,berlin2-global-register") { avpll_flags |= BERLIN2_AVPLL_SCRAMBLE_QUIRK; }
    ret = berlin2_avpll_vco_register(GBASE.add(REG_AVPLLCTL0), "avpll_vcoA", CLK_NAMES[REFCLK as usize], avpll_flags, 0); if ret != 0 { berlin2_fail(); return; }
    n = 0; while n < 8 { ret = berlin2_avpll_channel_register(GBASE.add(REG_AVPLLCTL0), CLK_NAMES[AVPLL_A1 as usize+n], n as i32, "avpll_vcoA", avpll_flags, 0); if ret != 0 { berlin2_fail(); return; } n += 1; }
    ret = berlin2_avpll_vco_register(GBASE.add(REG_AVPLLCTL31), "avpll_vcoB", CLK_NAMES[REFCLK as usize], BERLIN2_AVPLL_BIT_QUIRK | avpll_flags, 0); if ret != 0 { berlin2_fail(); return; }
    n = 0; while n < 8 { ret = berlin2_avpll_channel_register(GBASE.add(REG_AVPLLCTL31), CLK_NAMES[AVPLL_B1 as usize+n], n as i32, "avpll_vcoB", BERLIN2_AVPLL_BIT_QUIRK | avpll_flags, 0); if ret != 0 { berlin2_fail(); return; } n += 1; }

    // Reference bypasses, audio/video muxes, divider cells, gate cells, and
    // leaf-clock validation retain the same ordered registration as C.
    berlin2_register_muxes_and_cells(np, &mut parent_names, hws);
}

unsafe fn berlin2_fail() { iounmap(GBASE); }

CLK_OF_DECLARE!(berlin2_clk, "marvell,berlin2-clk", berlin2_clock_setup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
