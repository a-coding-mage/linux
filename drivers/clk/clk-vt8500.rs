// SPDX-License-Identifier: GPL-2.0-only
/* Clock implementation for VIA/Wondermedia SoC's */

// C dependencies are supplied by the surrounding kernel translation.

const LEGACY_PMC_BASE: usize = 0xD8130000;
const PLL_TYPE_VT8500: i32 = 0;
const PLL_TYPE_WM8650: i32 = 1;
const PLL_TYPE_WM8750: i32 = 2;
const PLL_TYPE_WM8850: i32 = 3;

#[repr(C)]
pub struct clk_device {
    pub hw: clk_hw,
    pub div_reg: *mut core::ffi::c_void,
    pub div_mask: u32,
    pub en_reg: *mut core::ffi::c_void,
    pub en_bit: i32,
    pub lock: *mut spinlock_t,
}

#[repr(C)]
pub struct clk_pll {
    pub hw: clk_hw,
    pub reg: *mut core::ffi::c_void,
    pub lock: *mut spinlock_t,
    pub r#type: i32,
}

static mut _lock: spinlock_t = spinlock_t::new();
static mut pmc_base: *mut core::ffi::c_void = core::ptr::null_mut();

unsafe fn vtwm_set_pmc_base() {
    let np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), c"via,vt8500-pmc".as_ptr());
    if !np.is_null() { pmc_base = of_iomap(np, 0); } else { pmc_base = ioremap(LEGACY_PMC_BASE, 0x1000); }
    of_node_put(np);
    if pmc_base.is_null() { pr_err(c"%s:of_iomap(pmc) failed\n", c"vtwm_set_pmc_base".as_ptr()); }
}

const VT8500_PMC_BUSY_MASK: u32 = 0x18;
unsafe fn vt8500_pmc_wait_busy() { while readl(pmc_base) & VT8500_PMC_BUSY_MASK != 0 { cpu_relax(); } }

unsafe fn vt8500_dclk_enable(hw: *mut clk_hw) -> i32 {
    let cdev = container_of_clk_device(hw); let mut flags: usize = 0;
    spin_lock_irqsave((*cdev).lock, &mut flags); let mut v = readl((*cdev).en_reg); v |= 1u32 << (*cdev).en_bit; writel(v, (*cdev).en_reg); spin_unlock_irqrestore((*cdev).lock, flags); 0
}
unsafe fn vt8500_dclk_disable(hw: *mut clk_hw) {
    let cdev = container_of_clk_device(hw); let mut flags: usize = 0;
    spin_lock_irqsave((*cdev).lock, &mut flags); let mut v = readl((*cdev).en_reg); v &= !(1u32 << (*cdev).en_bit); writel(v, (*cdev).en_reg); spin_unlock_irqrestore((*cdev).lock, flags);
}
unsafe fn vt8500_dclk_is_enabled(hw: *mut clk_hw) -> i32 { let cdev = container_of_clk_device(hw); if readl((*cdev).en_reg) & (1u32 << (*cdev).en_bit) != 0 { 1 } else { 0 } }
unsafe fn vt8500_dclk_recalc_rate(hw: *mut clk_hw, parent_rate: u64) -> u64 {
    let cdev = container_of_clk_device(hw); let mut div = readl((*cdev).div_reg) & (*cdev).div_mask;
    if (*cdev).div_mask == 0x3f && div & (1 << 5) != 0 { div = 64 * (div & 0x1f); }
    if div == 0 { div = (*cdev).div_mask + 1; } parent_rate / div as u64
}
unsafe fn vt8500_dclk_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    let cdev = container_of_clk_device(hw); if (*req).rate == 0 { return 0; }
    let mut divisor = (*req).best_parent_rate / (*req).rate; if (*req).rate * divisor < (*req).best_parent_rate { divisor += 1; }
    if (*cdev).div_mask == 0x3f && divisor > 31 { divisor = 64 * (divisor / 64 + 1); }
    (*req).rate = (*req).best_parent_rate / divisor; 0
}
unsafe fn vt8500_dclk_set_rate(hw: *mut clk_hw, rate: u64, parent_rate: u64) -> i32 {
    let cdev = container_of_clk_device(hw); if rate == 0 { return 0; } let mut divisor = parent_rate / rate; let mut flags = 0usize;
    if divisor == (*cdev).div_mask as u64 + 1 { divisor = 0; }
    if (*cdev).div_mask == 0x3f && divisor > 31 { divisor = 0x20 + divisor / 64; }
    if divisor > (*cdev).div_mask as u64 { pr_err(c"%s: invalid divisor for clock\n", c"vt8500_dclk_set_rate".as_ptr()); return -22; }
    spin_lock_irqsave((*cdev).lock, &mut flags); vt8500_pmc_wait_busy(); writel(divisor as u32, (*cdev).div_reg); vt8500_pmc_wait_busy(); spin_unlock_irqrestore((*cdev).lock, flags); 0
}

// PLL helper macros translated as functions to preserve their integer operations.
fn vt8500_pll_mul(x: u32) -> u32 { (x & 0x1f) << 1 }
fn vt8500_pll_div(x: u32) -> u32 { if x & 0x100 != 0 { 1 } else { 2 } }
fn wm8650_pll_mul(x: u32) -> u32 { x & 0x3ff }
fn wm8650_pll_div(x: u32) -> u32 { ((x >> 10) & 7) * (1 << ((x >> 13) & 3)) }
fn wm8750_pll_mul(x: u32) -> u32 { ((x >> 16) & 0xff) + 1 }
fn wm8750_pll_div(x: u32) -> u32 { (((x >> 8) & 1) + 1) * (1 << (x & 7)) }
fn wm8850_pll_mul(x: u32) -> u32 { (((x >> 16) & 0x7f) + 1) * 2 }
fn wm8850_pll_div(x: u32) -> u32 { (((x >> 8) & 1) + 1) * (1 << (x & 3)) }

unsafe fn vtwm_pll_recalc_rate(hw: *mut clk_hw, parent_rate: u64) -> u64 {
    let pll = container_of_clk_pll(hw); let v = readl((*pll).reg);
    match (*pll).r#type { PLL_TYPE_VT8500 => parent_rate * vt8500_pll_mul(v) as u64 / vt8500_pll_div(v) as u64, PLL_TYPE_WM8650 => parent_rate * wm8650_pll_mul(v) as u64 / wm8650_pll_div(v) as u64, PLL_TYPE_WM8750 => parent_rate * wm8750_pll_mul(v) as u64 / wm8750_pll_div(v) as u64, PLL_TYPE_WM8850 => parent_rate * wm8850_pll_mul(v) as u64 / wm8850_pll_div(v) as u64, _ => 0 }
}

unsafe fn vt8500_find_pll_bits(rate: u64, parent: u64, multiplier: *mut u32, prediv: *mut u32) -> i32 {
    if rate < parent * 4 || rate > parent * 62 { *multiplier = 0; *prediv = 1; return -22; }
    *prediv = if rate <= parent * 31 { 2 } else { 1 }; *multiplier = (rate / (parent / *prediv as u64)) as u32; 0
}
unsafe fn wm8650_find_pll_bits(rate: u64, parent: u64, multiplier: *mut u32, divisor1: *mut u32, divisor2: *mut u32) -> i32 {
    if parent == 0 || rate < 37_500_000 || rate > 600_000_000 { return -22; }
    *divisor2 = if rate <= 75_000_000 { 3 } else if rate <= 150_000_000 { 2 } else if rate <= 300_000_000 { 1 } else { 0 };
    let mut min_err = u64::MAX; for d in (3..=5).rev() { let o1 = rate * d * (1u64 << *divisor2); let err = o1 % parent; if err < min_err { *multiplier = (o1 / parent) as u32; *divisor1 = d as u32; min_err = err; if err == 0 { return 0; } } }
    if *multiplier < 3 || *multiplier > 1023 { -22 } else { 0 }
}
unsafe fn wm8750_find_pll_bits(rate: u64, parent: u64, filter: *mut u32, multiplier: *mut u32, divisor1: *mut u32, divisor2: *mut u32) -> i32 {
    let mut best = u64::MAX; for d1 in (0..=1).rev() { for d2 in (0..=7).rev() { for m in 0..=255u32 { let t = parent * (m as u64 + 1) / ((d1 + 1) as u64 * (1u64 << d2)); if t > rate { continue; } let e = rate - t; if e < best { best = e; *multiplier=m; *divisor1=d1; *divisor2=d2; } } } }
    if best == u64::MAX { return -22; } let f = (parent / 1_000_000) / (*divisor1 as u64 + 1); *filter = if f >= 166 {7} else if f >= 104 {6} else if f >= 65 {5} else if f >= 42 {4} else if f >= 26 {3} else if f >= 16 {2} else if f >= 10 {1} else {0}; 0
}
unsafe fn wm8850_find_pll_bits(rate: u64, parent: u64, multiplier: *mut u32, divisor1: *mut u32, divisor2: *mut u32) -> i32 {
    let mut best=u64::MAX; for d1 in (0..=1).rev() { for d2 in (0..=3).rev() { for m in 0..=127u32 { let t=parent*((m+1)*2) as u64/((d1+1) as u64*(1u64<<d2)); if t<=rate && rate-t<best { best=rate-t; *multiplier=m; *divisor1=d1; *divisor2=d2; } } } } if best==u64::MAX {-22} else {0}
}

unsafe fn vtwm_pll_set_rate(hw: *mut clk_hw, rate: u64, parent: u64) -> i32 {
    let pll=container_of_clk_pll(hw); let mut m=0; let mut d1=0; let mut d2=0; let mut f=0; let ret=match (*pll).r#type { PLL_TYPE_VT8500=>vt8500_find_pll_bits(rate,parent,&mut m,&mut d1), PLL_TYPE_WM8650=>wm8650_find_pll_bits(rate,parent,&mut m,&mut d1,&mut d2), PLL_TYPE_WM8750=>wm8750_find_pll_bits(rate,parent,&mut f,&mut m,&mut d1,&mut d2), PLL_TYPE_WM8850=>wm8850_find_pll_bits(rate,parent,&mut m,&mut d1,&mut d2), _=>-22 }; if ret!=0{return ret;} let v=match (*pll).r#type {0=>if d1==2 {0} else {0x100}|((m>>1)&0x1f),1=>(d2<<13)|(d1<<10)|(m&0x3ff),2=>(f<<24)|((m-1)<<16)|((d1-1)<<8)|d2,_=>(((m/2)-1)<<16)|((d1-1)<<8)|d2}; let mut flags=0; spin_lock_irqsave((*pll).lock,&mut flags); vt8500_pmc_wait_busy(); writel(v,(*pll).reg); vt8500_pmc_wait_busy(); spin_unlock_irqrestore((*pll).lock,flags); 0
}

unsafe fn vt8500_pll_init(node: *mut device_node) { vtwm_pll_clk_init(node, PLL_TYPE_VT8500); }
unsafe fn wm8650_pll_init(node: *mut device_node) { vtwm_pll_clk_init(node, PLL_TYPE_WM8650); }
unsafe fn wm8750_pll_init(node: *mut device_node) { vtwm_pll_clk_init(node, PLL_TYPE_WM8750); }
unsafe fn wm8850_pll_init(node: *mut device_node) { vtwm_pll_clk_init(node, PLL_TYPE_WM8850); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
