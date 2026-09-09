// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2017 MediaTek Inc.
 * Author: Chen Zhong <chen.zhong@mediatek.com>
 *         Sean Wang <sean.wang@mediatek.com>
 */

// Linux clock-provider, platform-device, clk-cpumux, clk-gate, clk-mtk,
// dt-bindings/clock/mt7622-clk, and linux/clk dependencies are supplied by
// the surrounding translation unit.

macro_rules! GATE_TOP0 { ($id:expr, $name:expr, $parent:expr, $shift:expr) => { GATE_MTK!($id, $name, $parent, &top0_cg_regs, $shift, &mtk_clk_gate_ops_no_setclr) }; }
macro_rules! GATE_TOP1 { ($id:expr, $name:expr, $parent:expr, $shift:expr) => { GATE_MTK!($id, $name, $parent, &top1_cg_regs, $shift, &mtk_clk_gate_ops_no_setclr) }; }
macro_rules! GATE_PERI0 { ($id:expr, $name:expr, $parent:expr, $shift:expr) => { GATE_MTK!($id, $name, $parent, &peri0_cg_regs, $shift, &mtk_clk_gate_ops_setclr) }; }
macro_rules! GATE_PERI0_AO { ($id:expr, $name:expr, $parent:expr, $shift:expr) => { GATE_MTK_FLAGS!($id, $name, $parent, &peri0_cg_regs, $shift, &mtk_clk_gate_ops_setclr, CLK_IS_CRITICAL) }; }
macro_rules! GATE_PERI1 { ($id:expr, $name:expr, $parent:expr, $shift:expr) => { GATE_MTK!($id, $name, $parent, &peri1_cg_regs, $shift, &mtk_clk_gate_ops_setclr) }; }

static DEFINE_SPINLOCK!(mt7622_clk_lock);

static const axi_parents: [&str; 7] = ["clkxtal", "syspll1_d2", "syspll_d5", "syspll1_d4", "univpll_d5", "univpll2_d2", "univpll_d7"];
static const mem_parents: [&str; 2] = ["clkxtal", "dmpll_ck"];
static const ddrphycfg_parents: [&str; 2] = ["clkxtal", "syspll1_d8"];
static const eth_parents: [&str; 7] = ["clkxtal", "syspll1_d2", "univpll1_d2", "syspll1_d4", "univpll_d5", "clk_null", "univpll_d7"];
static const pwm_parents: [&str; 2] = ["clkxtal", "univpll2_d4"];
static const f10m_ref_parents: [&str; 2] = ["clkxtal", "syspll4_d16"];
static const nfi_infra_parents: [&str; 15] = ["clkxtal","clkxtal","clkxtal","clkxtal","clkxtal","clkxtal","clkxtal","clkxtal","univpll2_d8","syspll1_d8","univpll1_d8","syspll4_d2","univpll2_d4","univpll3_d2","syspll1_d4"];
static const flash_parents: [&str; 8] = ["clkxtal","univpll_d80_d4","syspll2_d8","syspll3_d4","univpll3_d4","univpll1_d8","syspll2_d4","univpll2_d4"];
static const uart_parents: [&str; 2] = ["clkxtal", "univpll2_d8"];
static const spi0_parents: [&str; 8] = ["clkxtal","syspll3_d2","clkxtal","syspll2_d4","syspll4_d2","univpll2_d4","univpll1_d8","clkxtal"];
static const spi1_parents: [&str; 8] = ["clkxtal","syspll3_d2","clkxtal","syspll4_d4","syspll4_d2","univpll2_d4","univpll1_d8","clkxtal"];
static const msdc30_0_parents: [&str; 3] = ["clkxtal", "univpll2_d16", "univ48m"];
static const a1sys_hp_parents: [&str; 4] = ["clkxtal", "aud1pll_ck", "aud2pll_ck", "clkxtal"];
static const intdir_parents: [&str; 4] = ["clkxtal", "syspll_d2", "univpll_d2", "sgmiipll_ck"];
static const aud_intbus_parents: [&str; 4] = ["clkxtal", "syspll1_d4", "syspll4_d2", "syspll3_d2"];
static const pmicspi_parents: [&str; 6] = ["clkxtal", "clk_null", "clk_null", "clk_null", "clk_null", "univpll2_d16"];
static const atb_parents: [&str; 3] = ["clkxtal", "syspll1_d2", "syspll_d5"];
static const audio_parents: [&str; 4] = ["clkxtal", "syspll3_d4", "syspll4_d4", "univpll1_d16"];
static const usb20_parents: [&str; 4] = ["clkxtal", "univpll3_d4", "syspll1_d8", "clkxtal"];
static const aud1_parents: [&str; 2] = ["clkxtal", "aud1pll_ck"];
static const aud2_parents: [&str; 2] = ["clkxtal", "aud2pll_ck"];
static const asm_l_parents: [&str; 4] = ["clkxtal", "syspll_d5", "univpll2_d2", "univpll2_d4"];
static const apll1_ck_parents: [&str; 2] = ["aud1_sel", "aud2_sel"];
static const peribus_ck_parents: [&str; 2] = ["syspll1_d8", "syspll1_d4"];

static const top0_cg_regs: struct_mtk_gate_regs = struct_mtk_gate_regs { set_ofs: 0x120, clr_ofs: 0x120, sta_ofs: 0x120 };
static const top1_cg_regs: struct_mtk_gate_regs = struct_mtk_gate_regs { set_ofs: 0x128, clr_ofs: 0x128, sta_ofs: 0x128 };
static const peri0_cg_regs: struct_mtk_gate_regs = struct_mtk_gate_regs { set_ofs: 0x8, clr_ofs: 0x10, sta_ofs: 0x18 };
static const peri1_cg_regs: struct_mtk_gate_regs = struct_mtk_gate_regs { set_ofs: 0xC, clr_ofs: 0x14, sta_ofs: 0x1C };

// The following descriptor tables retain the kernel's declarative clock data.
static const top_fixed_clks: [struct_mtk_fixed_clk; 10] = [
    FIXED_CLK!(CLK_TOP_TO_U2_PHY, "to_u2_phy", "clkxtal", 31250000), FIXED_CLK!(CLK_TOP_TO_U2_PHY_1P, "to_u2_phy_1p", "clkxtal", 31250000),
    FIXED_CLK!(CLK_TOP_PCIE0_PIPE_EN, "pcie0_pipe_en", "clkxtal", 125000000), FIXED_CLK!(CLK_TOP_PCIE1_PIPE_EN, "pcie1_pipe_en", "clkxtal", 125000000),
    FIXED_CLK!(CLK_TOP_SSUSB_TX250M, "ssusb_tx250m", "clkxtal", 250000000), FIXED_CLK!(CLK_TOP_SSUSB_EQ_RX250M, "ssusb_eq_rx250m", "clkxtal", 250000000),
    FIXED_CLK!(CLK_TOP_SSUSB_CDR_REF, "ssusb_cdr_ref", "clkxtal", 33333333), FIXED_CLK!(CLK_TOP_SSUSB_CDR_FB, "ssusb_cdr_fb", "clkxtal", 50000000),
    FIXED_CLK!(CLK_TOP_SATA_ASIC, "sata_asic", "clkxtal", 50000000), FIXED_CLK!(CLK_TOP_SATA_RBC, "sata_rbc", "clkxtal", 50000000),
];

// Factor, gate, divider, and composite entries are kept in their original
// order and use the corresponding supplied MTK table constructors.
static const top_divs: [_; 43] = [
    FACTOR!(CLK_TOP_TO_USB3_SYS,"to_usb3_sys","eth1pll",1,4), FACTOR!(CLK_TOP_P1_1MHZ,"p1_1mhz","eth1pll",1,500), FACTOR!(CLK_TOP_4MHZ,"free_run_4mhz","eth1pll",1,125), FACTOR!(CLK_TOP_P0_1MHZ,"p0_1mhz","eth1pll",1,500), FACTOR!(CLK_TOP_TXCLK_SRC_PRE,"txclk_src_pre","sgmiipll_d2",1,1), FACTOR!(CLK_TOP_RTC,"rtc","clkxtal",1,1024), FACTOR!(CLK_TOP_MEMPLL,"mempll","clkxtal",32,1), FACTOR!(CLK_TOP_DMPLL,"dmpll_ck","mempll",1,1),
    FACTOR!(CLK_TOP_SYSPLL_D2,"syspll_d2","mainpll",1,2), FACTOR!(CLK_TOP_SYSPLL1_D2,"syspll1_d2","mainpll",1,4), FACTOR!(CLK_TOP_SYSPLL1_D4,"syspll1_d4","mainpll",1,8), FACTOR!(CLK_TOP_SYSPLL1_D8,"syspll1_d8","mainpll",1,16), FACTOR!(CLK_TOP_SYSPLL2_D4,"syspll2_d4","mainpll",1,12), FACTOR!(CLK_TOP_SYSPLL2_D8,"syspll2_d8","mainpll",1,24), FACTOR!(CLK_TOP_SYSPLL_D5,"syspll_d5","mainpll",1,5), FACTOR!(CLK_TOP_SYSPLL3_D2,"syspll3_d2","mainpll",1,10), FACTOR!(CLK_TOP_SYSPLL3_D4,"syspll3_d4","mainpll",1,20), FACTOR!(CLK_TOP_SYSPLL4_D2,"syspll4_d2","mainpll",1,14), FACTOR!(CLK_TOP_SYSPLL4_D4,"syspll4_d4","mainpll",1,28), FACTOR!(CLK_TOP_SYSPLL4_D16,"syspll4_d16","mainpll",1,112),
    FACTOR!(CLK_TOP_UNIVPLL,"univpll","univ2pll",1,2), FACTOR!(CLK_TOP_UNIVPLL_D2,"univpll_d2","univpll",1,2), FACTOR!(CLK_TOP_UNIVPLL1_D2,"univpll1_d2","univpll",1,4), FACTOR!(CLK_TOP_UNIVPLL1_D4,"univpll1_d4","univpll",1,8), FACTOR!(CLK_TOP_UNIVPLL1_D8,"univpll1_d8","univpll",1,16), FACTOR!(CLK_TOP_UNIVPLL1_D16,"univpll1_d16","univpll",1,32), FACTOR!(CLK_TOP_UNIVPLL2_D2,"univpll2_d2","univpll",1,6), FACTOR!(CLK_TOP_UNIVPLL2_D4,"univpll2_d4","univpll",1,12), FACTOR!(CLK_TOP_UNIVPLL2_D8,"univpll2_d8","univpll",1,24), FACTOR!(CLK_TOP_UNIVPLL2_D16,"univpll2_d16","univpll",1,48), FACTOR!(CLK_TOP_UNIVPLL_D5,"univpll_d5","univpll",1,5), FACTOR!(CLK_TOP_UNIVPLL3_D2,"univpll3_d2","univpll",1,10), FACTOR!(CLK_TOP_UNIVPLL3_D4,"univpll3_d4","univpll",1,20), FACTOR!(CLK_TOP_UNIVPLL3_D16,"univpll3_d16","univpll",1,80), FACTOR!(CLK_TOP_UNIVPLL_D7,"univpll_d7","univpll",1,7), FACTOR!(CLK_TOP_UNIVPLL_D80_D4,"univpll_d80_d4","univpll",1,320), FACTOR!(CLK_TOP_UNIV48M,"univ48m","univpll",1,25), FACTOR!(CLK_TOP_SGMIIPLL,"sgmiipll_ck","sgmipll",1,1), FACTOR!(CLK_TOP_SGMIIPLL_D2,"sgmiipll_d2","sgmipll",1,2), FACTOR!(CLK_TOP_AUD1PLL,"aud1pll_ck","aud1pll",1,1), FACTOR!(CLK_TOP_AUD2PLL,"aud2pll_ck","aud2pll",1,1), FACTOR!(CLK_TOP_AUD_I2S2_MCK,"aud_i2s2_mck","i2s2_mck_sel",1,2), FACTOR!(CLK_TOP_TO_USB3_REF,"to_usb3_ref","univpll2_d4",1,4), FACTOR!(CLK_TOP_PCIE1_MAC_EN,"pcie1_mac_en","univpll1_d4",1,1), FACTOR!(CLK_TOP_PCIE0_MAC_EN,"pcie0_mac_en","univpll1_d4",1,1), FACTOR!(CLK_TOP_ETH_500M,"eth_500m","eth1pll",1,1),
];

// Remaining C table declarations are represented directly by the supplied
// constructors; no entries or ordering are intentionally omitted.
static top_clks: [_; 8] = [GATE_TOP0!(CLK_TOP_APLL1_DIV_PD,"apll1_ck_div_pd","apll1_ck_div",0),GATE_TOP0!(CLK_TOP_APLL2_DIV_PD,"apll2_ck_div_pd","apll2_ck_div",1),GATE_TOP0!(CLK_TOP_I2S0_MCK_DIV_PD,"i2s0_mck_div_pd","i2s0_mck_div",2),GATE_TOP0!(CLK_TOP_I2S1_MCK_DIV_PD,"i2s1_mck_div_pd","i2s1_mck_div",3),GATE_TOP0!(CLK_TOP_I2S2_MCK_DIV_PD,"i2s2_mck_div_pd","i2s2_mck_div",4),GATE_TOP0!(CLK_TOP_I2S3_MCK_DIV_PD,"i2s3_mck_div_pd","i2s3_mck_div",5),GATE_TOP1!(CLK_TOP_A1SYS_HP_DIV_PD,"a1sys_div_pd","a1sys_div",0),GATE_TOP1!(CLK_TOP_A2SYS_HP_DIV_PD,"a2sys_div_pd","a2sys_div",16)];
static top_adj_divs: [_; 8] = [DIV_ADJ!(CLK_TOP_APLL1_DIV,"apll1_ck_div","apll1_ck_sel",0x120,24,3),DIV_ADJ!(CLK_TOP_APLL2_DIV,"apll2_ck_div","apll2_ck_sel",0x120,28,3),DIV_ADJ!(CLK_TOP_I2S0_MCK_DIV,"i2s0_mck_div","i2s0_mck_sel",0x124,0,7),DIV_ADJ!(CLK_TOP_I2S1_MCK_DIV,"i2s1_mck_div","i2s1_mck_sel",0x124,8,7),DIV_ADJ!(CLK_TOP_I2S2_MCK_DIV,"i2s2_mck_div","aud_i2s2_mck",0x124,16,7),DIV_ADJ!(CLK_TOP_I2S3_MCK_DIV,"i2s3_mck_div","i2s3_mck_sel",0x124,24,7),DIV_ADJ!(CLK_TOP_A1SYS_HP_DIV,"a1sys_div","a1sys_hp_sel",0x128,8,7),DIV_ADJ!(CLK_TOP_A2SYS_HP_DIV,"a2sys_div","a2sys_hp_sel",0x128,24,7)];
static peri_clks: [_; 25] = [GATE_PERI0!(CLK_PERI_THERM_PD,"peri_therm_pd","axi_sel",1),GATE_PERI0!(CLK_PERI_PWM1_PD,"peri_pwm1_pd","clkxtal",2),GATE_PERI0!(CLK_PERI_PWM2_PD,"peri_pwm2_pd","clkxtal",3),GATE_PERI0!(CLK_PERI_PWM3_PD,"peri_pwm3_pd","clkxtal",4),GATE_PERI0!(CLK_PERI_PWM4_PD,"peri_pwm4_pd","clkxtal",5),GATE_PERI0!(CLK_PERI_PWM5_PD,"peri_pwm5_pd","clkxtal",6),GATE_PERI0!(CLK_PERI_PWM6_PD,"peri_pwm6_pd","clkxtal",7),GATE_PERI0!(CLK_PERI_PWM7_PD,"peri_pwm7_pd","clkxtal",8),GATE_PERI0!(CLK_PERI_PWM_PD,"peri_pwm_pd","clkxtal",9),GATE_PERI0!(CLK_PERI_AP_DMA_PD,"peri_ap_dma_pd","axi_sel",12),GATE_PERI0!(CLK_PERI_MSDC30_0_PD,"peri_msdc30_0","msdc30_0_sel",13),GATE_PERI0!(CLK_PERI_MSDC30_1_PD,"peri_msdc30_1","msdc30_1_sel",14),GATE_PERI0_AO!(CLK_PERI_UART0_PD,"peri_uart0_pd","axi_sel",17),GATE_PERI0!(CLK_PERI_UART1_PD,"peri_uart1_pd","axi_sel",18),GATE_PERI0!(CLK_PERI_UART2_PD,"peri_uart2_pd","axi_sel",19),GATE_PERI0!(CLK_PERI_UART3_PD,"peri_uart3_pd","axi_sel",20),GATE_PERI0!(CLK_PERI_UART4_PD,"peri_uart4_pd","axi_sel",21),GATE_PERI0!(CLK_PERI_BTIF_PD,"peri_btif_pd","axi_sel",22),GATE_PERI0!(CLK_PERI_I2C0_PD,"peri_i2c0_pd","axi_sel",23),GATE_PERI0!(CLK_PERI_I2C1_PD,"peri_i2c1_pd","axi_sel",24),GATE_PERI0!(CLK_PERI_I2C2_PD,"peri_i2c2_pd","axi_sel",25),GATE_PERI0!(CLK_PERI_SPI1_PD,"peri_spi1_pd","spi1_sel",26),GATE_PERI0!(CLK_PERI_AUXADC_PD,"peri_auxadc_pd","clkxtal",27),GATE_PERI0!(CLK_PERI_SPI0_PD,"peri_spi0_pd","spi0_sel",28),GATE_PERI0!(CLK_PERI_SNFI_PD,"peri_snfi_pd","nfi_infra_sel",29),GATE_PERI0!(CLK_PERI_NFI_PD,"peri_nfi_pd","axi_sel",30),GATE_PERI0!(CLK_PERI_NFIECC_PD,"peri_nfiecc_pd","axi_sel",31),GATE_PERI1!(CLK_PERI_FLASH_PD,"peri_flash_pd","flash_sel",1),GATE_PERI1!(CLK_PERI_IRTX_PD,"peri_irtx_pd","irtx_sel",2)];

static top_muxes: [_; 31] = [
    MUX_GATE_FLAGS!(CLK_TOP_AXI_SEL,"axi_sel",axi_parents,0x040,0,3,7,CLK_IS_CRITICAL), MUX_GATE_FLAGS!(CLK_TOP_MEM_SEL,"mem_sel",mem_parents,0x040,8,1,15,CLK_IS_CRITICAL), MUX_GATE_FLAGS!(CLK_TOP_DDRPHYCFG_SEL,"ddrphycfg_sel",ddrphycfg_parents,0x040,16,1,23,CLK_IS_CRITICAL), MUX_GATE!(CLK_TOP_ETH_SEL,"eth_sel",eth_parents,0x040,24,3,31),
    MUX_GATE!(CLK_TOP_PWM_SEL,"pwm_sel",pwm_parents,0x050,0,2,7), MUX_GATE!(CLK_TOP_F10M_REF_SEL,"f10m_ref_sel",f10m_ref_parents,0x050,8,1,15), MUX_GATE!(CLK_TOP_NFI_INFRA_SEL,"nfi_infra_sel",nfi_infra_parents,0x050,16,4,23), MUX_GATE!(CLK_TOP_FLASH_SEL,"flash_sel",flash_parents,0x050,24,3,31),
    MUX_GATE!(CLK_TOP_UART_SEL,"uart_sel",uart_parents,0x060,0,1,7), MUX_GATE!(CLK_TOP_SPI0_SEL,"spi0_sel",spi0_parents,0x060,8,3,15), MUX_GATE!(CLK_TOP_SPI1_SEL,"spi1_sel",spi1_parents,0x060,16,3,23), MUX_GATE!(CLK_TOP_MSDC50_0_SEL,"msdc50_0_sel",uart_parents,0x060,24,3,31),
    MUX_GATE!(CLK_TOP_MSDC30_0_SEL,"msdc30_0_sel",msdc30_0_parents,0x070,0,3,7), MUX_GATE!(CLK_TOP_MSDC30_1_SEL,"msdc30_1_sel",msdc30_0_parents,0x070,8,3,15), MUX_GATE!(CLK_TOP_A1SYS_HP_SEL,"a1sys_hp_sel",a1sys_hp_parents,0x070,16,2,23), MUX_GATE!(CLK_TOP_A2SYS_HP_SEL,"a2sys_hp_sel",a1sys_hp_parents,0x070,24,2,31),
    MUX_GATE!(CLK_TOP_INTDIR_SEL,"intdir_sel",intdir_parents,0x080,0,2,7), MUX_GATE!(CLK_TOP_AUD_INTBUS_SEL,"aud_intbus_sel",aud_intbus_parents,0x080,8,2,15), MUX_GATE!(CLK_TOP_PMICSPI_SEL,"pmicspi_sel",pmicspi_parents,0x080,16,3,23), MUX_GATE!(CLK_TOP_SCP_SEL,"scp_sel",ddrphycfg_parents,0x080,24,2,31),
    MUX_GATE!(CLK_TOP_ATB_SEL,"atb_sel",atb_parents,0x090,0,2,7), MUX_GATE!(CLK_TOP_HIF_SEL,"hif_sel",eth_parents,0x090,8,3,15), MUX_GATE!(CLK_TOP_AUDIO_SEL,"audio_sel",audio_parents,0x090,16,2,23), MUX_GATE!(CLK_TOP_U2_SEL,"usb20_sel",usb20_parents,0x090,24,2,31),
    MUX_GATE!(CLK_TOP_AUD1_SEL,"aud1_sel",aud1_parents,0x0A0,0,1,7), MUX_GATE!(CLK_TOP_AUD2_SEL,"aud2_sel",aud2_parents,0x0A0,8,1,15), MUX_GATE!(CLK_TOP_IRRX_SEL,"irrx_sel",f10m_ref_parents,0x0A0,16,1,23), MUX_GATE!(CLK_TOP_IRTX_SEL,"irtx_sel",f10m_ref_parents,0x0A0,24,1,31),
    MUX_GATE!(CLK_TOP_ASM_L_SEL,"asm_l_sel",asm_l_parents,0x0B0,0,2,7), MUX_GATE!(CLK_TOP_ASM_M_SEL,"asm_m_sel",asm_l_parents,0x0B0,8,2,15), MUX_GATE!(CLK_TOP_ASM_H_SEL,"asm_h_sel",asm_l_parents,0x0B0,16,2,23),
    MUX!(CLK_TOP_APLL1_SEL,"apll1_ck_sel",apll1_ck_parents,0x120,6,1), MUX!(CLK_TOP_APLL2_SEL,"apll2_ck_sel",apll1_ck_parents,0x120,7,1), MUX!(CLK_TOP_I2S0_MCK_SEL,"i2s0_mck_sel",apll1_ck_parents,0x120,8,1), MUX!(CLK_TOP_I2S1_MCK_SEL,"i2s1_mck_sel",apll1_ck_parents,0x120,9,1), MUX!(CLK_TOP_I2S2_MCK_SEL,"i2s2_mck_sel",apll1_ck_parents,0x120,10,1), MUX!(CLK_TOP_I2S3_MCK_SEL,"i2s3_mck_sel",apll1_ck_parents,0x120,11,1),
];
static peri_muxes: [_; 1] = [MUX!(CLK_PERIBUS_SEL,"peribus_ck_sel",peribus_ck_parents,0x05C,0,1)];

static const topck_desc: struct_mtk_clk_desc = struct_mtk_clk_desc { clks: top_clks.as_ptr(), num_clks: top_clks.len(), fixed_clks: top_fixed_clks.as_ptr(), num_fixed_clks: top_fixed_clks.len(), factor_clks: top_divs.as_ptr(), num_factor_clks: top_divs.len(), composite_clks: top_muxes.as_ptr(), num_composite_clks: top_muxes.len(), divider_clks: top_adj_divs.as_ptr(), num_divider_clks: top_adj_divs.len(), clk_lock: &mt7622_clk_lock };
static const peri_desc: struct_mtk_clk_desc = struct_mtk_clk_desc { clks: peri_clks.as_ptr(), num_clks: peri_clks.len(), composite_clks: peri_muxes.as_ptr(), num_composite_clks: peri_muxes.len(), rst_desc: &clk_rst_desc, clk_lock: &mt7622_clk_lock };

static pericfg_rst_ofs: [u16; 2] = [0x0, 0x4];
static const clk_rst_desc: struct_mtk_clk_rst_desc = struct_mtk_clk_rst_desc { version: MTK_RST_SIMPLE, rst_bank_ofs: pericfg_rst_ofs.as_ptr(), rst_bank_nr: 2 };

// Device matching and driver registration retain the original externally
// visible interfaces and callbacks.
static of_match_clk_mt7622: [_; 3] = [
    OF_DEVICE_ID { compatible: "mediatek,mt7622-topckgen", data: &topck_desc },
    OF_DEVICE_ID { compatible: "mediatek,mt7622-pericfg", data: &peri_desc },
    OF_DEVICE_ID { compatible: core::ptr::null(), data: core::ptr::null() },
];
static mut clk_mt7622_drv: platform_driver = platform_driver { driver: driver { name: "clk-mt7622", of_match_table: of_match_clk_mt7622.as_ptr() }, probe: mtk_clk_simple_probe, remove: mtk_clk_simple_remove };
module_platform_driver!(clk_mt7622_drv);
MODULE_DESCRIPTION!("MediaTek MT7622 clocks driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
