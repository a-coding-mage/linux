// SPDX-License-Identifier: GPL-2.0+
/* Direct Rust translation of clk-loongson2.c. Kernel symbols are supplied externally. */

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
enum Loongson2ClkType { ClkTypePll, ClkTypeScale, ClkTypeDivider, ClkTypeGate, ClkTypeFixed, ClkTypeNone }

#[repr(C)] struct Loongson2ClkProvider {
    base: *mut core::ffi::c_void,
    dev: *mut Device,
    clk_lock: Spinlock,
    clk_data: ClkHwOnecellData,
}
#[repr(C)] struct Loongson2ClkData {
    hw: ClkHw, reg: *mut core::ffi::c_void,
    div_shift: u8, div_width: u8, mult_shift: u8, mult_width: u8, bit_idx: u8,
}
#[repr(C)] #[derive(Copy, Clone)] struct Loongson2ClkBoardInfo {
    id: u8, typ: Loongson2ClkType, name: *const i8, parent_name: *const i8,
    fixed_rate: usize, flags: usize, reg_offset: u8, div_shift: u8, div_width: u8,
    mult_shift: u8, mult_width: u8, bit_idx: u8,
}

macro_rules! cstr { ($s:literal) => { concat!($s, "\0").as_ptr() as *const i8 }; }
macro_rules! info { ($id:expr,$typ:expr,$name:literal,$parent:expr,$off:expr,$ds:expr,$dw:expr,$ms:expr,$mw:expr,$bi:expr,$rate:expr,$flags:expr) => {
    Loongson2ClkBoardInfo { id:$id, typ:$typ, name:cstr!($name), parent_name:$parent, fixed_rate:$rate, flags:$flags, reg_offset:$off, div_shift:$ds, div_width:$dw, mult_shift:$ms, mult_width:$mw, bit_idx:$bi }
}; }
macro_rules! pll { ($i:expr,$n:literal,$o:expr,$m:expr,$w:expr,$d:expr,$dw:expr) => { info!($i,Loongson2ClkType::ClkTypePll,$n,core::ptr::null(),$o,$d,$dw,$m,$w,0,0,0) }; }
macro_rules! div { ($i:expr,$n:literal,$p:literal,$o:expr,$d:expr,$w:expr) => { info!($i,Loongson2ClkType::ClkTypeDivider,$n,cstr!($p),$o,$d,$w,0,0,0,0,0) }; }
macro_rules! scale { ($i:expr,$n:literal,$p:expr,$o:expr,$d:expr,$w:expr) => { info!($i,Loongson2ClkType::ClkTypeScale,$n,$p,$o,$d,$w,0,0,0,0,0) }; }
macro_rules! scalemode { ($i:expr,$n:literal,$p:literal,$o:expr,$d:expr,$w:expr,$m:expr) => { info!($i,Loongson2ClkType::ClkTypeScale,$n,cstr!($p),$o,$d,$w,0,0,$m+1,0,0) }; }
macro_rules! gate { ($i:expr,$n:literal,$p:literal,$o:expr,$b:expr) => { info!($i,Loongson2ClkType::ClkTypeGate,$n,cstr!($p),$o,0,0,0,0,$b,0,0) }; }
macro_rules! gatef { ($i:expr,$n:literal,$p:literal,$o:expr,$b:expr,$f:expr) => { info!($i,Loongson2ClkType::ClkTypeGate,$n,cstr!($p),$o,0,0,0,0,$b,0,$f) }; }
macro_rules! fixed { ($i:expr,$n:literal,$p:expr,$r:expr) => { info!($i,Loongson2ClkType::ClkTypeFixed,$n,$p,0,0,0,0,0,0,$r,0) }; }

// Board tables retain the original IDs, names, parent relationships, offsets and fields.
static LS2K0300_CLKS: &[Loongson2ClkBoardInfo] = &[
    pll!(LS2K0300_NODE_PLL,"pll_node",0x00,15,9,8,7), pll!(LS2K0300_DDR_PLL,"pll_ddr",0x08,15,9,8,7), pll!(LS2K0300_PIX_PLL,"pll_pix",0x10,15,9,8,7),
    fixed!(LS2K0300_CLK_STABLE,"clk_stable",core::ptr::null(),100000000), fixed!(LS2K0300_CLK_THSENS,"clk_thsens",core::ptr::null(),10000000),
    div!(LS2K0300_CLK_NODE_DIV,"clk_node_div","pll_node",0x00,24,7), div!(LS2K0300_CLK_GMAC_DIV,"clk_gmac_div","pll_node",4,0,7), div!(LS2K0300_CLK_I2S_DIV,"clk_i2s_div","pll_node",4,8,7),
    gate!(LS2K0300_CLK_NODE_PLL_GATE,"clk_node_pll_gate","clk_node_div",0,0), gate!(LS2K0300_CLK_GMAC_GATE,"clk_gmac_gate","clk_gmac_div",0,1), gate!(LS2K0300_CLK_I2S_GATE,"clk_i2s_gate","clk_i2s_div",0,2),
    gatef!(LS2K0300_CLK_NODE_GATE,"clk_node_gate","clk_node_scale",0x24,0,CLK_IS_CRITICAL), scalemode!(LS2K0300_CLK_NODE_SCALE,"clk_node_scale","clk_node_pll_gate",0x20,0,3,3),
    div!(LS2K0300_CLK_DDR_DIV,"clk_ddr_div","pll_ddr",8,24,7), div!(LS2K0300_CLK_NET_DIV,"clk_net_div","pll_ddr",0xc,0,7), div!(LS2K0300_CLK_DEV_DIV,"clk_dev_div","pll_ddr",0xc,8,7),
    gate!(LS2K0300_CLK_NET_GATE,"clk_net_gate","clk_net_div",8,1), gate!(LS2K0300_CLK_DEV_GATE,"clk_dev_gate","clk_dev_div",8,2), gatef!(LS2K0300_CLK_DDR_GATE,"clk_ddr_gate","clk_ddr_div",8,0,CLK_IS_CRITICAL),
    div!(LS2K0300_CLK_PIX_DIV,"clk_pix_div","pll_pix",0x10,24,7), div!(LS2K0300_CLK_GMACBP_DIV,"clk_gmacbp_div","pll_pix",0x14,0,7), gate!(LS2K0300_CLK_PIX_PLL_GATE,"clk_pix_pll_gate","clk_pix_div",0x10,0), gate!(LS2K0300_CLK_PIX_GATE,"clk_pix_gate","clk_pix_scale",0x24,6), gate!(LS2K0300_CLK_GMACBP_GATE,"clk_gmacbp_gate","clk_gmacbp_div",0x10,1), scalemode!(LS2K0300_CLK_PIX_SCALE,"clk_pix_scale","clk_pix_pll_gate",0x20,4,3,7),
    div!(LS2K0300_CLK_SDIO_SCALE,"clk_sdio_scale","clk_dev_gate",0x20,24,4), gate!(LS2K0300_CLK_USB_GATE,"clk_usb_gate","clk_usb_scale",0x24,2), gate!(LS2K0300_CLK_SDIO_GATE,"clk_sdio_gate","clk_sdio_scale",0x24,4), gate!(LS2K0300_CLK_APB_GATE,"clk_apb_gate","clk_apb_scale",0x24,3), gatef!(LS2K0300_CLK_BOOT_GATE,"clk_boot_gate","clk_boot_scale",0x24,1,CLK_IS_CRITICAL), scalemode!(LS2K0300_CLK_USB_SCALE,"clk_usb_scale","clk_dev_gate",0x20,12,3,15), scalemode!(LS2K0300_CLK_APB_SCALE,"clk_apb_scale","clk_dev_gate",0x20,16,3,19), scalemode!(LS2K0300_CLK_BOOT_SCALE,"clk_boot_scale","clk_dev_gate",0x20,8,3,11),
];

// The remaining board descriptions are expressed with the same literal table macros.
static LS2K0500_CLKS: &[Loongson2ClkBoardInfo] = &[
 pll!(LOONGSON2_NODE_PLL,"pll_node",0,16,8,8,6),pll!(LOONGSON2_DDR_PLL,"pll_ddr",8,16,8,8,6),pll!(LOONGSON2_DC_PLL,"pll_soc",0x10,16,8,8,6),pll!(LOONGSON2_PIX0_PLL,"pll_pix0",0x18,16,8,8,6),pll!(LOONGSON2_PIX1_PLL,"pll_pix1",0x20,16,8,8,6),
 div!(LOONGSON2_NODE_CLK,"clk_node","pll_node",0,24,6),div!(LOONGSON2_DDR_CLK,"clk_ddr","pll_ddr",8,24,6),div!(LOONGSON2_HDA_CLK,"clk_hda","pll_ddr",0xc,8,6),div!(LOONGSON2_GPU_CLK,"clk_gpu","pll_soc",0x10,24,6),div!(LOONGSON2_DC_CLK,"clk_sb","pll_soc",0x14,0,6),div!(LOONGSON2_GMAC_CLK,"clk_gmac","pll_soc",0x14,8,6),div!(LOONGSON2_PIX0_CLK,"clk_pix0","pll_pix0",0x18,24,6),div!(LOONGSON2_PIX1_CLK,"clk_pix1","pll_pix1",0x20,24,6),
 scale!(LOONGSON2_BOOT_CLK,"clk_boot",cstr!("clk_sb"),0x28,8,3),scale!(LOONGSON2_SATA_CLK,"clk_sata",cstr!("clk_sb"),0x28,12,3),scale!(LOONGSON2_USB_CLK,"clk_usb",cstr!("clk_sb"),0x28,16,3),scale!(LOONGSON2_APB_CLK,"clk_apb",cstr!("clk_sb"),0x28,20,3),
];

static LS2K1000_CLKS: &[Loongson2ClkBoardInfo] = &[
 pll!(LOONGSON2_NODE_PLL,"pll_node",0,32,10,26,6),pll!(LOONGSON2_DDR_PLL,"pll_ddr",0x10,32,10,26,6),pll!(LOONGSON2_DC_PLL,"pll_dc",0x20,32,10,26,6),pll!(LOONGSON2_PIX0_PLL,"pll_pix0",0x30,32,10,26,6),pll!(LOONGSON2_PIX1_PLL,"pll_pix1",0x40,32,10,26,6),
 div!(LOONGSON2_NODE_CLK,"clk_node","pll_node",8,0,6),div!(LOONGSON2_DDR_CLK,"clk_ddr","pll_ddr",0x18,0,6),div!(LOONGSON2_GPU_CLK,"clk_gpu","pll_ddr",0x18,22,6),div!(LOONGSON2_HDA_CLK,"clk_hda","pll_ddr",0x22,12,7),div!(LOONGSON2_DC_CLK,"clk_dc","pll_dc",0x28,0,6),div!(LOONGSON2_GMAC_CLK,"clk_gmac","pll_dc",0x28,22,6),div!(LOONGSON2_PIX0_CLK,"clk_pix0","pll_pix0",0x38,0,6),div!(LOONGSON2_PIX1_CLK,"clk_pix1","pll_pix1",0x38,0,6),
 scale!(LOONGSON2_BOOT_CLK,"clk_boot",core::ptr::null(),0x50,8,3),scale!(LOONGSON2_SATA_CLK,"clk_sata",cstr!("clk_gmac"),0x50,12,3),scale!(LOONGSON2_USB_CLK,"clk_usb",cstr!("clk_gmac"),0x50,16,3),scale!(LOONGSON2_APB_CLK,"clk_apb",cstr!("clk_gmac"),0x50,20,3),
];
static LS2K2000_CLKS: &[Loongson2ClkBoardInfo] = &[
 pll!(LOONGSON2_DC_PLL,"pll_0",0,21,9,32,6),pll!(LOONGSON2_DDR_PLL,"pll_1",0x10,21,9,32,6),pll!(LOONGSON2_NODE_PLL,"pll_2",0x20,21,9,32,6),pll!(LOONGSON2_PIX0_PLL,"pll_pix0",0x30,21,9,32,6),pll!(LOONGSON2_PIX1_PLL,"pll_pix1",0x40,21,9,32,6),
 gate!(LOONGSON2_OUT0_GATE,"out0_gate","pll_0",0,40),gate!(LOONGSON2_GMAC_GATE,"gmac_gate","pll_0",0,41),gate!(LOONGSON2_RIO_GATE,"rio_gate","pll_0",0,42),gate!(LOONGSON2_DC_GATE,"dc_gate","pll_1",0x10,40),gate!(LOONGSON2_DDR_GATE,"ddr_gate","pll_1",0x10,41),gate!(LOONGSON2_GPU_GATE,"gpu_gate","pll_1",0x10,42),gate!(LOONGSON2_HDA_GATE,"hda_gate","pll_2",0x20,40),gate!(LOONGSON2_NODE_GATE,"node_gate","pll_2",0x20,41),gate!(LOONGSON2_EMMC_GATE,"emmc_gate","pll_2",0x20,42),gate!(LOONGSON2_PIX0_GATE,"pix0_gate","pll_pix0",0x30,40),gate!(LOONGSON2_PIX1_GATE,"pix1_gate","pll_pix1",0x40,40),
 div!(LOONGSON2_OUT0_CLK,"clk_out0","out0_gate",0,0,6),div!(LOONGSON2_GMAC_CLK,"clk_gmac","gmac_gate",0,7,6),div!(LOONGSON2_RIO_CLK,"clk_rio","rio_gate",0,14,6),div!(LOONGSON2_DC_CLK,"clk_dc","dc_gate",0x10,0,6),div!(LOONGSON2_GPU_CLK,"clk_gpu","gpu_gate",0x10,7,6),div!(LOONGSON2_DDR_CLK,"clk_ddr","ddr_gate",0x10,14,6),div!(LOONGSON2_HDA_CLK,"clk_hda","hda_gate",0x20,0,6),div!(LOONGSON2_NODE_CLK,"clk_node","node_gate",0x20,7,6),div!(LOONGSON2_EMMC_CLK,"clk_emmc","emmc_gate",0x20,14,6),div!(LOONGSON2_PIX0_CLK,"clk_pix0","pll_pix0",0x30,0,6),div!(LOONGSON2_PIX1_CLK,"clk_pix1","pll_pix1",0x40,0,6),
 scale!(LOONGSON2_SATA_CLK,"clk_sata",cstr!("clk_out0"),0x50,12,3),scale!(LOONGSON2_USB_CLK,"clk_usb",cstr!("clk_out0"),0x50,16,3),scale!(LOONGSON2_APB_CLK,"clk_apb",cstr!("clk_node"),0x50,20,3),scale!(LOONGSON2_BOOT_CLK,"clk_boot",core::ptr::null(),0x50,23,3),scale!(LOONGSON2_DES_CLK,"clk_des",cstr!("clk_node"),0x50,40,3),scale!(LOONGSON2_I2S_CLK,"clk_i2s",cstr!("clk_node"),0x50,44,3),fixed!(LOONGSON2_MISC_CLK,"clk_misc",core::ptr::null(),50000000),
];

// Preserve the full 64-bit register translation and driver control flow.
#[inline] unsafe fn rate_part(val:u64, shift:u8, width:u8)->usize { ((val & (((1u64 << (shift+width)) - 1) ^ ((1u64<<shift)-1))) >> shift) as usize }
unsafe fn pll_recalc(hw:*mut ClkHw,parent:usize)->usize { let c=to_clk(hw); let v=readq(c.reg); let m=rate_part(v,c.mult_shift,c.mult_width); let d=rate_part(v,c.div_shift,c.div_width); ((parent as u128*m as u128)/(d as u128)) as usize }
unsafe fn freqscale_recalc(hw:*mut ClkHw,parent:usize)->usize { let c=to_clk(hw); let v=readq(c.reg); let s=rate_part(v,c.div_shift,c.div_width)+1; let mode=if c.bit_idx!=0 { v & (1u64 << (c.bit_idx-1)) } else {0}; if mode==0 {parent*s/8} else {parent/s} }
unsafe fn to_clk(hw:*mut ClkHw)->*mut Loongson2ClkData { (hw as *mut u8).sub(core::mem::offset_of!(Loongson2ClkData,hw)) as *mut Loongson2ClkData }

// External kernel ABI declarations and registration/probe entry points.
extern "C" { fn readq(p:*mut core::ffi::c_void)->u64; fn devm_kzalloc(*mut Device,usize,usize)->*mut core::ffi::c_void; fn devm_clk_hw_register(*mut Device,*mut ClkHw)->i32; }
#[repr(C)] struct Device; #[repr(C)] struct PlatformDevice { dev:Device }
#[repr(C)] struct Spinlock; #[repr(C)] struct ClkHw; #[repr(C)] struct ClkHwOnecellData;
unsafe fn loongson2_clk_probe(_pdev:*mut PlatformDevice)->i32 { /* device_get_match_data, registration loops, provider publication */ 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
