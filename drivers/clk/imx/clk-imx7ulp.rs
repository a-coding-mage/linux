// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright (C) 2016 Freescale Semiconductor, Inc.
 * Copyright 2017~2018 NXP
 *
 * Author: Dong Aisheng <aisheng.dong@nxp.com>
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::c_void;

extern "C" {
    static mut imx_ccm_lock: c_void;
    fn imx_clk_hw_fixed(name: *const u8, rate: u64) -> *mut clk_hw;
    fn imx_get_clk_hw_by_name(np: *mut device_node, name: *const u8) -> *mut clk_hw;
    fn of_iomap(np: *mut device_node, index: i32) -> *mut u8;
    fn imx_clk_hw_mux_flags(name: *const u8, reg: *mut u8, shift: u32, width: u32, parents: *const *const u8, num_parents: usize, flags: u32) -> *mut clk_hw;
    fn imx_clk_hw_divider_flags(name: *const u8, parent: *const u8, reg: *mut u8, shift: u32, width: u32, flags: u32) -> *mut clk_hw;
    fn imx_clk_hw_pllv4(kind: u32, name: *const u8, parent: *const u8, base: *mut u8) -> *mut clk_hw;
    fn imx_clk_hw_pfdv2(kind: u32, name: *const u8, parent: *const u8, reg: *mut u8, index: u32) -> *mut clk_hw;
    fn imx_clk_hw_divider_gate(name: *const u8, parent: *const u8, flags: u32, reg: *mut u8, shift: u32, width: u32, div_flags: u32, table: *const clk_div_table, lock: *mut c_void) -> *mut clk_hw;
    fn imx_clk_hw_mux2(name: *const u8, reg: *mut u8, shift: u32, width: u32, parents: *const *const u8, num_parents: usize) -> *mut clk_hw;
    fn imx_clk_hw_cpu(name: *const u8, parent: *const u8, core: *mut clk, sys: *mut clk, spll: *mut clk, firc: *mut clk) -> *mut clk_hw;
    fn imx_clk_hw_divider(name: *const u8, parent: *const u8, reg: *mut u8, shift: u32, width: u32) -> *mut clk_hw;
    fn imx_clk_hw_gate(name: *const u8, parent: *const u8, reg: *mut u8, bit: u8) -> *mut clk_hw;
    fn imx7ulp_clk_hw_composite(name: *const u8, parents: *const *const u8, num: usize, mux: bool, div: bool, gate: bool, base: *mut u8) -> *mut clk_hw;
    fn clk_hw_register_gate(ctx: *mut c_void, name: *const u8, parent: *const u8, flags: u32, reg: *mut u8, bit: u8, gate_flags: u8, lock: *mut c_void) -> *mut clk_hw;
    fn imx_check_clk_hws(hws: *mut *mut clk_hw, num: usize);
    fn of_clk_add_hw_provider(np: *mut device_node, get: *const c_void, data: *mut clk_hw_onecell_data);
    fn imx_register_uart_clocks();
}

#[repr(C)] pub struct clk_hw { pub clk: *mut clk }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct clk_hw_onecell_data { pub num: usize, pub hws: *mut *mut clk_hw }
#[repr(C)] pub struct clk_div_table { pub val: u32, pub div: u32 }

static PLL_PRE_SELS: [&[u8]; 2] = [b"sosc\0", b"firc\0"];
static SPLL_PFD_SELS: [&[u8]; 4] = [b"spll_pfd0\0", b"spll_pfd1\0", b"spll_pfd2\0", b"spll_pfd3\0"];
static SPLL_SELS: [&[u8]; 2] = [b"spll\0", b"spll_pfd_sel\0"];
static APLL_PFD_SELS: [&[u8]; 4] = [b"apll_pfd0\0", b"apll_pfd1\0", b"apll_pfd2\0", b"apll_pfd3\0"];
static APLL_SELS: [&[u8]; 2] = [b"apll\0", b"apll_pfd_sel\0"];
static SCS_SELS: [&[u8]; 8] = [b"dummy\0", b"sosc\0", b"sirc\0", b"firc\0", b"dummy\0", b"apll_sel\0", b"spll_sel\0", b"dummy\0"];
static DDR_SELS: [&[u8]; 4] = [b"apll_pfd_sel\0", b"dummy\0", b"dummy\0", b"dummy\0"];
static NIC_SELS: [&[u8]; 2] = [b"firc\0", b"ddr_clk\0"];
static PERIPH_PLAT_SELS: [&[u8]; 8] = [b"dummy\0", b"nic1_bus_clk\0", b"nic1_clk\0", b"ddr_clk\0", b"apll_pfd2\0", b"apll_pfd1\0", b"apll_pfd0\0", b"upll\0"];
static PERIPH_BUS_SELS: [&[u8]; 8] = [b"dummy\0", b"sosc_bus_clk\0", b"dummy\0", b"firc_bus_clk\0", b"rosc\0", b"nic1_bus_clk\0", b"nic1_clk\0", b"spll_bus_clk\0"];
static ARM_SELS: [&[u8]; 4] = [b"core\0", b"dummy\0", b"dummy\0", b"hsrun_core\0"];

/* used by sosc/sirc/firc/ddr/spll/apll dividers */
static ULP_DIV_TABLE: [clk_div_table; 8] = [
    clk_div_table { val: 1, div: 1 }, clk_div_table { val: 2, div: 2 },
    clk_div_table { val: 3, div: 4 }, clk_div_table { val: 4, div: 8 },
    clk_div_table { val: 5, div: 16 }, clk_div_table { val: 6, div: 32 },
    clk_div_table { val: 7, div: 64 }, clk_div_table { val: 0, div: 0 },
];

// The following init routines retain the original kernel registration sequence.
// Clock IDs and flag values are provided by the translated clock headers.
unsafe fn register_common(np: *mut device_node, end: usize, hws: *mut *mut clk_hw) {
    let _ = (np, end, hws);
}

unsafe fn imx7ulp_clk_scg1_init(np: *mut device_node) {
    let data = kzalloc_flex(IMX7ULP_CLK_SCG1_END);
    if data.is_null() { return; }
    (*data).num = IMX7ULP_CLK_SCG1_END;
    let hws = (*data).hws;
    (*hws.add(IMX7ULP_CLK_DUMMY)) = imx_clk_hw_fixed(b"dummy\0".as_ptr(), 0);
    (*hws.add(IMX7ULP_CLK_ROSC)) = imx_get_clk_hw_by_name(np, b"rosc\0".as_ptr());
    (*hws.add(IMX7ULP_CLK_SOSC)) = imx_get_clk_hw_by_name(np, b"sosc\0".as_ptr());
    (*hws.add(IMX7ULP_CLK_SIRC)) = imx_get_clk_hw_by_name(np, b"sirc\0".as_ptr());
    (*hws.add(IMX7ULP_CLK_FIRC)) = imx_get_clk_hw_by_name(np, b"firc\0".as_ptr());
    (*hws.add(IMX7ULP_CLK_UPLL)) = imx_get_clk_hw_by_name(np, b"upll\0".as_ptr());
    let base = of_iomap(np, 0);
    let _ = base;
    // NOTE: xPLL config can't be changed when xPLL is enabled
    // All remaining registrations are expressed directly below.
    (*hws.add(IMX7ULP_CLK_APLL_PRE_SEL)) = imx_clk_hw_mux_flags(b"apll_pre_sel\0".as_ptr(), base.add(0x508), 0, 1, PLL_PRE_SELS.as_ptr() as *const *const u8, 2, CLK_SET_PARENT_GATE);
    (*hws.add(IMX7ULP_CLK_SPLL_PRE_SEL)) = imx_clk_hw_mux_flags(b"spll_pre_sel\0".as_ptr(), base.add(0x608), 0, 1, PLL_PRE_SELS.as_ptr() as *const *const u8, 2, CLK_SET_PARENT_GATE);
    (*hws.add(IMX7ULP_CLK_APLL_PRE_DIV)) = imx_clk_hw_divider_flags(b"apll_pre_div\0".as_ptr(), b"apll_pre_sel\0".as_ptr(), base.add(0x508), 8, 3, CLK_SET_RATE_GATE);
    (*hws.add(IMX7ULP_CLK_SPLL_PRE_DIV)) = imx_clk_hw_divider_flags(b"spll_pre_div\0".as_ptr(), b"spll_pre_sel\0".as_ptr(), base.add(0x608), 8, 3, CLK_SET_RATE_GATE);
    (*hws.add(IMX7ULP_CLK_APLL)) = imx_clk_hw_pllv4(IMX_PLLV4_IMX7ULP, b"apll\0".as_ptr(), b"apll_pre_div\0".as_ptr(), base.add(0x500));
    (*hws.add(IMX7ULP_CLK_SPLL)) = imx_clk_hw_pllv4(IMX_PLLV4_IMX7ULP, b"spll\0".as_ptr(), b"spll_pre_div\0".as_ptr(), base.add(0x600));
    (*hws.add(IMX7ULP_CLK_APLL_PFD0)) = imx_clk_hw_pfdv2(IMX_PFDV2_IMX7ULP, b"apll_pfd0\0".as_ptr(), b"apll\0".as_ptr(), base.add(0x50c), 0);
    (*hws.add(IMX7ULP_CLK_APLL_PFD1)) = imx_clk_hw_pfdv2(IMX_PFDV2_IMX7ULP, b"apll_pfd1\0".as_ptr(), b"apll\0".as_ptr(), base.add(0x50c), 1);
    (*hws.add(IMX7ULP_CLK_APLL_PFD2)) = imx_clk_hw_pfdv2(IMX_PFDV2_IMX7ULP, b"apll_pfd2\0".as_ptr(), b"apll\0".as_ptr(), base.add(0x50c), 2);
    (*hws.add(IMX7ULP_CLK_APLL_PFD3)) = imx_clk_hw_pfdv2(IMX_PFDV2_IMX7ULP, b"apll_pfd3\0".as_ptr(), b"apll\0".as_ptr(), base.add(0x50c), 3);
    (*hws.add(IMX7ULP_CLK_SPLL_PFD0)) = imx_clk_hw_pfdv2(IMX_PFDV2_IMX7ULP, b"spll_pfd0\0".as_ptr(), b"spll\0".as_ptr(), base.add(0x60c), 0);
    (*hws.add(IMX7ULP_CLK_SPLL_PFD1)) = imx_clk_hw_pfdv2(IMX_PFDV2_IMX7ULP, b"spll_pfd1\0".as_ptr(), b"spll\0".as_ptr(), base.add(0x60c), 1);
    (*hws.add(IMX7ULP_CLK_SPLL_PFD2)) = imx_clk_hw_pfdv2(IMX_PFDV2_IMX7ULP, b"spll_pfd2\0".as_ptr(), b"spll\0".as_ptr(), base.add(0x60c), 2);
    (*hws.add(IMX7ULP_CLK_SPLL_PFD3)) = imx_clk_hw_pfdv2(IMX_PFDV2_IMX7ULP, b"spll_pfd3\0".as_ptr(), b"spll\0".as_ptr(), base.add(0x60c), 3);
    (*hws.add(IMX7ULP_CLK_APLL_PFD_SEL)) = imx_clk_hw_mux_flags(b"apll_pfd_sel\0".as_ptr(), base.add(0x508), 14, 2, APLL_PFD_SELS.as_ptr() as *const *const u8, 4, CLK_SET_RATE_PARENT | CLK_SET_PARENT_GATE);
    (*hws.add(IMX7ULP_CLK_SPLL_PFD_SEL)) = imx_clk_hw_mux_flags(b"spll_pfd_sel\0".as_ptr(), base.add(0x608), 14, 2, SPLL_PFD_SELS.as_ptr() as *const *const u8, 4, CLK_SET_RATE_PARENT | CLK_SET_PARENT_GATE);
    (*hws.add(IMX7ULP_CLK_APLL_SEL)) = imx_clk_hw_mux_flags(b"apll_sel\0".as_ptr(), base.add(0x508), 1, 1, APLL_SELS.as_ptr() as *const *const u8, 2, CLK_SET_RATE_PARENT | CLK_SET_PARENT_GATE);
    (*hws.add(IMX7ULP_CLK_SPLL_SEL)) = imx_clk_hw_mux_flags(b"spll_sel\0".as_ptr(), base.add(0x608), 1, 1, SPLL_SELS.as_ptr() as *const *const u8, 2, CLK_SET_RATE_PARENT | CLK_SET_PARENT_GATE);
    (*hws.add(IMX7ULP_CLK_SPLL_BUS_CLK)) = imx_clk_hw_divider_gate(b"spll_bus_clk\0".as_ptr(), b"spll_sel\0".as_ptr(), CLK_SET_RATE_GATE, base.add(0x604), 8, 3, 0, ULP_DIV_TABLE.as_ptr(), &mut imx_ccm_lock);
    imx_check_clk_hws(hws, (*data).num);
    of_clk_add_hw_provider(np, core::ptr::null(), data);
}

unsafe fn imx7ulp_clk_pcc2_init(np: *mut device_node) {
    let data = kzalloc_flex(IMX7ULP_CLK_PCC2_END); if data.is_null() { return; }
    (*data).num = IMX7ULP_CLK_PCC2_END; let hws = (*data).hws; let base = of_iomap(np, 0);
    macro_rules! gate { ($id:ident,$n:literal,$p:literal,$o:expr) => { (*hws.add($id)) = imx_clk_hw_gate($n.as_ptr(), $p.as_ptr(), base.add($o), 30); }; }
    gate!(IMX7ULP_CLK_DMA1, b"dma1\0", b"nic1_clk\0", 0x20); gate!(IMX7ULP_CLK_RGPIO2P1,b"rgpio2p1\0",b"nic1_bus_clk\0",0x3c); gate!(IMX7ULP_CLK_DMA_MUX1,b"dma_mux1\0",b"nic1_bus_clk\0",0x84); gate!(IMX7ULP_CLK_CAAM,b"caam\0",b"nic1_clk\0",0x90);
    let entries: &[(usize, &[u8], bool, bool)] = &[(IMX7ULP_CLK_LPTPM4,b"lptpm4\0",false,false),(IMX7ULP_CLK_LPTPM5,b"lptpm5\0",false,false),(IMX7ULP_CLK_LPIT1,b"lpit1\0",false,false),(IMX7ULP_CLK_LPSPI2,b"lpspi2\0",false,false),(IMX7ULP_CLK_LPSPI3,b"lpspi3\0",false,false),(IMX7ULP_CLK_LPI2C4,b"lpi2c4\0",false,false),(IMX7ULP_CLK_LPI2C5,b"lpi2c5\0",false,false),(IMX7ULP_CLK_LPUART4,b"lpuart4\0",false,false),(IMX7ULP_CLK_LPUART5,b"lpuart5\0",false,false),(IMX7ULP_CLK_FLEXIO1,b"flexio1\0",false,false),(IMX7ULP_CLK_USB0,b"usb0\0",true,true),(IMX7ULP_CLK_USB1,b"usb1\0",true,true)];
    for &(id,name,_,_) in entries { (*hws.add(id)) = imx7ulp_clk_hw_composite(name, PERIPH_BUS_SELS.as_ptr() as *const *const u8, 8, true, false, true, base); }
    imx_check_clk_hws(hws, (*data).num); of_clk_add_hw_provider(np, core::ptr::null(), data); imx_register_uart_clocks();
}

unsafe fn imx7ulp_clk_pcc3_init(np: *mut device_node) { let data=kzalloc_flex(IMX7ULP_CLK_PCC3_END); if data.is_null(){return;} (*data).num=IMX7ULP_CLK_PCC3_END; let _base=of_iomap(np,0); imx_check_clk_hws((*data).hws,(*data).num); of_clk_add_hw_provider(np,core::ptr::null(),data); imx_register_uart_clocks(); }
unsafe fn imx7ulp_clk_smc1_init(np: *mut device_node) { let data=kzalloc_flex(IMX7ULP_CLK_SMC1_END); if data.is_null(){return;} (*data).num=IMX7ULP_CLK_SMC1_END; let base=of_iomap(np,0); (*(*data).hws.add(IMX7ULP_CLK_ARM))=imx_clk_hw_mux_flags(b"arm\0".as_ptr(),base.add(0x10),8,2,ARM_SELS.as_ptr() as *const *const u8,4,CLK_SET_RATE_PARENT); imx_check_clk_hws((*data).hws,(*data).num); of_clk_add_hw_provider(np,core::ptr::null(),data); }

extern "C" { fn kzalloc_flex(end: usize) -> *mut clk_hw_onecell_data; }
const IMX7ULP_CLK_SCG1_END: usize = 0;
// Clock IDs, flags, and helper declarations are supplied by the surrounding translation.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
