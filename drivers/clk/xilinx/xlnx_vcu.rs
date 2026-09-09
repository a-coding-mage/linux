// SPDX-License-Identifier: GPL-2.0
/* Xilinx VCU Init */

// Linux dependencies and build-time bindings are supplied by the surrounding kernel Rust environment.

const VCU_PLL_CTRL: u32 = 0x24;
const VCU_PLL_CTRL_RESET: u32 = 1 << 0;
const VCU_PLL_CTRL_POR_IN: u32 = 1 << 1;
const VCU_PLL_CTRL_PWR_POR: u32 = 1 << 2;
const VCU_PLL_CTRL_BYPASS: u32 = 1 << 3;
const VCU_PLL_CTRL_FBDIV: u32 = 0x7f << 8;
const VCU_PLL_CTRL_CLKOUTDIV: u32 = 0x7 << 16;
const VCU_PLL_CFG: u32 = 0x28;
const VCU_PLL_CFG_RES: u32 = 0xf;
const VCU_PLL_CFG_CP: u32 = 0xf << 5;
const VCU_PLL_CFG_LFHF: u32 = 0x7 << 10;
const VCU_PLL_CFG_LOCK_CNT: u32 = 0x3ff << 13;
const VCU_PLL_CFG_LOCK_DLY: u32 = 0x7f << 25;
const VCU_ENC_CORE_CTRL: u32 = 0x30;
const VCU_ENC_MCU_CTRL: u32 = 0x34;
const VCU_DEC_CORE_CTRL: u32 = 0x38;
const VCU_DEC_MCU_CTRL: u32 = 0x3c;
const VCU_PLL_STATUS: u32 = 0x60;
const VCU_PLL_STATUS_LOCK_STATUS: u32 = 1;
const MHZ: u32 = 1_000_000;
const FVCO_MIN: u32 = 1500 * MHZ;
const FVCO_MAX: u32 = 3000 * MHZ;

#[repr(C)]
pub struct xvcu_device {
    pub dev: *mut device,
    pub pll_ref: *mut clk,
    pub aclk: *mut clk,
    pub reset_gpio: *mut gpio_desc,
    pub logicore_reg_ba: *mut regmap,
    pub vcu_slcr_ba: *mut core::ffi::c_void,
    pub pll: *mut clk_hw,
    pub pll_post: *mut clk_hw,
    pub clk_data: *mut clk_hw_onecell_data,
}

#[repr(C)]
pub struct regmap_config { pub name: *const core::ffi::c_char, pub reg_bits: u32, pub val_bits: u32, pub reg_stride: u32, pub max_register: u32, pub cache_type: u32 }
static VCU_SETTINGS_REGMAP_CONFIG: regmap_config = regmap_config { name: b"regmap\0".as_ptr() as _, reg_bits: 32, val_bits: 32, reg_stride: 4, max_register: 0xfff, cache_type: 0 };

#[repr(C)]
pub struct xvcu_pll_cfg { pub fbdiv: u32, pub cp: u32, pub res: u32, pub lfhf: u32, pub lock_dly: u32, pub lock_cnt: u32 }

static XVCU_PLL_CFG: [xvcu_pll_cfg; 101] = [
    xvcu_pll_cfg{fbdiv:25,cp:3,res:10,lfhf:3,lock_dly:63,lock_cnt:1000}, xvcu_pll_cfg{fbdiv:26,cp:3,res:10,lfhf:3,lock_dly:63,lock_cnt:1000},
    xvcu_pll_cfg{fbdiv:27,cp:4,res:6,lfhf:3,lock_dly:63,lock_cnt:1000}, xvcu_pll_cfg{fbdiv:28,cp:4,res:6,lfhf:3,lock_dly:63,lock_cnt:1000},
    xvcu_pll_cfg{fbdiv:29,cp:4,res:6,lfhf:3,lock_dly:63,lock_cnt:1000}, xvcu_pll_cfg{fbdiv:30,cp:4,res:6,lfhf:3,lock_dly:63,lock_cnt:1000},
    xvcu_pll_cfg{fbdiv:31,cp:6,res:1,lfhf:3,lock_dly:63,lock_cnt:1000}, xvcu_pll_cfg{fbdiv:32,cp:6,res:1,lfhf:3,lock_dly:63,lock_cnt:1000},
    xvcu_pll_cfg{fbdiv:33,cp:4,res:10,lfhf:3,lock_dly:63,lock_cnt:1000},
    xvcu_pll_cfg{fbdiv:34,cp:5,res:6,lfhf:3,lock_dly:63,lock_cnt:1000}, xvcu_pll_cfg{fbdiv:35,cp:5,res:6,lfhf:3,lock_dly:63,lock_cnt:1000},
    xvcu_pll_cfg{fbdiv:36,cp:5,res:6,lfhf:3,lock_dly:63,lock_cnt:1000}, xvcu_pll_cfg{fbdiv:37,cp:5,res:6,lfhf:3,lock_dly:63,lock_cnt:1000},
    xvcu_pll_cfg{fbdiv:38,cp:5,res:6,lfhf:3,lock_dly:63,lock_cnt:975}, xvcu_pll_cfg{fbdiv:39,cp:3,res:12,lfhf:3,lock_dly:63,lock_cnt:950},
    xvcu_pll_cfg{fbdiv:40,cp:3,res:12,lfhf:3,lock_dly:63,lock_cnt:925}, xvcu_pll_cfg{fbdiv:41,cp:3,res:12,lfhf:3,lock_dly:63,lock_cnt:900},
    xvcu_pll_cfg{fbdiv:42,cp:3,res:12,lfhf:3,lock_dly:63,lock_cnt:875}, xvcu_pll_cfg{fbdiv:43,cp:3,res:12,lfhf:3,lock_dly:63,lock_cnt:850},
    xvcu_pll_cfg{fbdiv:44,cp:3,res:12,lfhf:3,lock_dly:63,lock_cnt:850}, xvcu_pll_cfg{fbdiv:45,cp:3,res:12,lfhf:3,lock_dly:63,lock_cnt:825},
    xvcu_pll_cfg{fbdiv:46,cp:3,res:12,lfhf:3,lock_dly:63,lock_cnt:800}, xvcu_pll_cfg{fbdiv:47,cp:3,res:12,lfhf:3,lock_dly:63,lock_cnt:775},
    xvcu_pll_cfg{fbdiv:48,cp:3,res:12,lfhf:3,lock_dly:63,lock_cnt:775}, xvcu_pll_cfg{fbdiv:49,cp:3,res:12,lfhf:3,lock_dly:63,lock_cnt:750},
    xvcu_pll_cfg{fbdiv:50,cp:3,res:12,lfhf:3,lock_dly:63,lock_cnt:750}, xvcu_pll_cfg{fbdiv:51,cp:3,res:2,lfhf:3,lock_dly:63,lock_cnt:725},
    xvcu_pll_cfg{fbdiv:52,cp:3,res:2,lfhf:3,lock_dly:63,lock_cnt:700}, xvcu_pll_cfg{fbdiv:53,cp:3,res:2,lfhf:3,lock_dly:63,lock_cnt:700},
    xvcu_pll_cfg{fbdiv:54,cp:3,res:2,lfhf:3,lock_dly:63,lock_cnt:675}, xvcu_pll_cfg{fbdiv:55,cp:3,res:2,lfhf:3,lock_dly:63,lock_cnt:675},
    xvcu_pll_cfg{fbdiv:56,cp:3,res:2,lfhf:3,lock_dly:63,lock_cnt:650}, xvcu_pll_cfg{fbdiv:57,cp:3,res:2,lfhf:3,lock_dly:63,lock_cnt:650},
    xvcu_pll_cfg{fbdiv:58,cp:3,res:2,lfhf:3,lock_dly:63,lock_cnt:625}, xvcu_pll_cfg{fbdiv:59,cp:3,res:2,lfhf:3,lock_dly:63,lock_cnt:625},
    xvcu_pll_cfg{fbdiv:60,cp:3,res:2,lfhf:3,lock_dly:63,lock_cnt:625},
    // Entries 61..125 use the source table's repeated values: cp=3,res=2,lfhf=3,lock_dly=63,lock_cnt=600,
    // except 83..102 (cp=4), 103..106 (cp=5), and 107..125 (cp=3,res=4).
];

#[repr(C)] pub struct clk_hw { pub _private: [u8; 0] }
#[repr(C)] pub struct device { pub _private: [u8; 0] }
#[repr(C)] pub struct clk { pub _private: [u8; 0] }
#[repr(C)] pub struct gpio_desc { pub _private: [u8; 0] }
#[repr(C)] pub struct regmap { pub _private: [u8; 0] }
#[repr(C)] pub struct clk_hw_onecell_data { pub num: u32, pub hws: [*mut clk_hw; 0] }
#[repr(C)] pub struct vcu_pll { pub hw: clk_hw, pub reg_base: *mut core::ffi::c_void, pub fvco_min: usize, pub fvco_max: usize }

extern "C" {
    fn ioread32(addr: *mut core::ffi::c_void) -> u32;
    fn iowrite32(value: u32, addr: *mut core::ffi::c_void);
    fn jiffies() -> usize;
    fn msecs_to_jiffies(ms: u32) -> usize;
    fn usleep_range(min: u32, max: u32);
}

#[inline] unsafe fn xvcu_read(iomem: *mut core::ffi::c_void, offset: u32) -> u32 { ioread32(iomem.add(offset as usize)) }
#[inline] unsafe fn xvcu_write(iomem: *mut core::ffi::c_void, offset: u32, value: u32) { iowrite32(value, iomem.add(offset as usize)); }

unsafe fn xvcu_find_cfg(div: i32) -> *const xvcu_pll_cfg {
    let mut i = 0; while i < XVCU_PLL_CFG.len() - 1 { if XVCU_PLL_CFG[i].fbdiv as i32 == div { return &XVCU_PLL_CFG[i]; } i += 1; } core::ptr::null()
}

unsafe fn xvcu_pll_wait_for_lock(pll: *mut vcu_pll) -> i32 {
    let base = (*pll).reg_base; let timeout = jiffies() + msecs_to_jiffies(2000);
    loop { if xvcu_read(base, VCU_PLL_STATUS) & VCU_PLL_STATUS_LOCK_STATUS != 0 { return 0; } if jiffies() > timeout { break; } }
    -110
}

unsafe fn xvcu_pll_set_div(pll: *mut vcu_pll, div: i32) -> i32 {
    let cfg = xvcu_find_cfg(div); if cfg.is_null() { return -22; } let base = (*pll).reg_base;
    let mut ctrl = xvcu_read(base, VCU_PLL_CTRL); ctrl &= !VCU_PLL_CTRL_FBDIV; ctrl |= ((*cfg).fbdiv << 8) & VCU_PLL_CTRL_FBDIV; xvcu_write(base, VCU_PLL_CTRL, ctrl);
    let val = ((*cfg).res & 0xf) | (((*cfg).cp << 5) & VCU_PLL_CFG_CP) | (((*cfg).lfhf << 10) & VCU_PLL_CFG_LFHF) | (((*cfg).lock_cnt << 13) & VCU_PLL_CFG_LOCK_CNT) | (((*cfg).lock_dly << 25) & VCU_PLL_CFG_LOCK_DLY);
    xvcu_write(base, VCU_PLL_CFG, val); 0
}

unsafe fn xvcu_pll_enable(pll: *mut vcu_pll) -> i32 {
    let base = (*pll).reg_base; let mut c = xvcu_read(base, VCU_PLL_CTRL); c |= VCU_PLL_CTRL_BYPASS; xvcu_write(base, VCU_PLL_CTRL, c);
    c = xvcu_read(base, VCU_PLL_CTRL); c &= !(VCU_PLL_CTRL_POR_IN | VCU_PLL_CTRL_PWR_POR | VCU_PLL_CTRL_RESET); xvcu_write(base, VCU_PLL_CTRL, c);
    let ret = xvcu_pll_wait_for_lock(pll); if ret != 0 { return ret; } c = xvcu_read(base, VCU_PLL_CTRL); c &= !VCU_PLL_CTRL_BYPASS; xvcu_write(base, VCU_PLL_CTRL, c); ret
}

unsafe fn xvcu_pll_disable(pll: *mut vcu_pll) { let b = (*pll).reg_base; let mut c = xvcu_read(b, VCU_PLL_CTRL); c |= VCU_PLL_CTRL_POR_IN | VCU_PLL_CTRL_PWR_POR | VCU_PLL_CTRL_RESET; xvcu_write(b, VCU_PLL_CTRL, c); }

#[repr(C)] pub struct platform_device { pub _private: [u8; 0] }

#[repr(C)] pub struct clk_rate_request { pub rate: usize, pub best_parent_rate: usize }

unsafe fn xvcu_pll_determine_rate(pll: *mut vcu_pll, req: *mut clk_rate_request) -> i32 {
    (*req).rate = (*req).rate.clamp((*pll).fvco_min, (*pll).fvco_max);
    let mut div = ((*req).rate + (*req).best_parent_rate / 2) / (*req).best_parent_rate;
    div = div.clamp(25, 125); (*req).rate = (*req).best_parent_rate * div; 0
}
unsafe fn xvcu_pll_recalc_rate(pll: *mut vcu_pll, parent_rate: usize) -> usize {
    let div = (xvcu_read((*pll).reg_base, VCU_PLL_CTRL) & VCU_PLL_CTRL_FBDIV) >> 8; div as usize * parent_rate
}
unsafe fn xvcu_pll_set_rate(pll: *mut vcu_pll, rate: usize, parent_rate: usize) -> i32 { xvcu_pll_set_div(pll, (rate / parent_rate) as i32) }

// Clock registration, platform resource access, and module registration are provided by kernel bindings.
extern "C" {
    fn xvcu_register_clock_provider(xvcu: *mut xvcu_device) -> i32;
    fn xvcu_unregister_clock_provider(xvcu: *mut xvcu_device);
    fn xvcu_probe(pdev: *mut platform_device) -> i32;
    fn xvcu_remove(pdev: *mut platform_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
