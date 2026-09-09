// SPDX-License-Identifier: GPL-2.0+
/* Copyright (C) 2019 Microchip Technology Inc. */

const PMC_PLL_CTRL0_DIV_MSK: u32 = 0xff;
const PMC_PLL_CTRL1_MUL_MSK: u32 = 0xff00_0000;
const PMC_PLL_CTRL1_FRACR_MSK: u32 = 0x003f_ffff;
const PLL_DIV_MAX: u32 = 256;
const UPLL_DIV: u32 = 2;
const PLL_MUL_MAX: u32 = 256;
const PLL_MAX_ID: u8 = 9;

#[repr(C)]
struct Sam9x60PllCore { regmap: *mut Regmap, lock: *mut Spinlock, characteristics: *const ClkPllCharacteristics, layout: *const ClkPllLayout, hw: ClkHw, id: u8 }
#[repr(C)] struct Sam9x60Frac { core: Sam9x60PllCore, pms: At91ClkPms, frac: u32, mul: u16 }
#[repr(C)] struct Sam9x60Div { core: Sam9x60PllCore, pms: At91ClkPms, div: u8, safe_div: u8 }

static mut notifier_div: *mut Sam9x60Div = core::ptr::null_mut();

unsafe fn sam9x60_pll_ready(regmap: *mut Regmap, id: i32) -> bool { let mut status = 0u32; regmap_read(regmap, AT91_PMC_PLL_ISR0, &mut status); (status & (1u32 << id)) != 0 }
unsafe fn sam9x60_frac_pll_ready(regmap: *mut Regmap, id: u8) -> bool { sam9x60_pll_ready(regmap, id as i32) }

unsafe fn sam9x60_frac_pll_recalc_rate(hw: *mut ClkHw, parent_rate: usize) -> usize {
    let core = &mut *(hw as *mut Sam9x60PllCore); let frac = &*(core as *mut Sam9x60Frac);
    let mut freq = parent_rate * (frac.mul as usize + 1) + div_round_closest_ull((parent_rate as u64 * frac.frac as u64), 1u64 << 22) as usize;
    if (*core.layout).div2 { freq >>= 1; } freq
}

unsafe fn sam9x60_frac_pll_set(core: *mut Sam9x60PllCore) -> i32 {
    let frac = &*(core as *mut Sam9x60Frac); let regmap = (*core).regmap; let mut flags=0usize; let mut val=0u32;
    spin_lock_irqsave((*core).lock, &mut flags); regmap_write_bits(regmap, AT91_PMC_PLL_UPDT, AT91_PMC_PLL_UPDT_ID_MSK, (*core).id as u32); regmap_read(regmap, AT91_PMC_PLL_CTRL1, &mut val);
    let cmul=(val & (*(*core).layout).mul_mask)>>(*(*core).layout).mul_shift; let cfrac=(val & (*(*core).layout).frac_mask)>>(*(*core).layout).frac_shift;
    if sam9x60_frac_pll_ready(regmap, (*core).id) && cmul==frac.mul as u32 && cfrac==frac.frac { spin_unlock_irqrestore((*core).lock, flags); return 0; }
    val=(*(*core).characteristics).acr; regmap_write(regmap, AT91_PMC_PLL_ACR,val); regmap_write(regmap, AT91_PMC_PLL_CTRL1,(frac.mul as u32<<(*(*core).layout).mul_shift)|(frac.frac<<(*(*core).layout).frac_shift));
    if (*(*core).characteristics).upll { val|=AT91_PMC_PLL_ACR_UTMIBG; regmap_write(regmap,AT91_PMC_PLL_ACR,val); udelay(10); val|=AT91_PMC_PLL_ACR_UTMIVR; regmap_write(regmap,AT91_PMC_PLL_ACR,val); udelay(10); }
    regmap_write_bits(regmap,AT91_PMC_PLL_UPDT,AT91_PMC_PLL_UPDT_UPDATE|AT91_PMC_PLL_UPDT_ID_MSK,AT91_PMC_PLL_UPDT_UPDATE|(*core).id as u32);
    regmap_update_bits(regmap,AT91_PMC_PLL_CTRL0,AT91_PMC_PLL_CTRL0_ENLOCK|AT91_PMC_PLL_CTRL0_ENPLL,AT91_PMC_PLL_CTRL0_ENLOCK|AT91_PMC_PLL_CTRL0_ENPLL);
    regmap_write_bits(regmap,AT91_PMC_PLL_UPDT,AT91_PMC_PLL_UPDT_UPDATE|AT91_PMC_PLL_UPDT_ID_MSK,AT91_PMC_PLL_UPDT_UPDATE|(*core).id as u32); while !sam9x60_pll_ready(regmap,(*core).id as i32) { cpu_relax(); } spin_unlock_irqrestore((*core).lock,flags); 0
}

unsafe fn sam9x60_frac_pll_prepare(hw:*mut ClkHw)->i32 { sam9x60_frac_pll_set(hw as *mut Sam9x60PllCore) }
unsafe fn sam9x60_frac_pll_unprepare(hw:*mut ClkHw) { let c=&mut *(hw as *mut Sam9x60PllCore); let mut f=0; spin_lock_irqsave(c.lock,&mut f); regmap_write_bits(c.regmap,AT91_PMC_PLL_UPDT,AT91_PMC_PLL_UPDT_ID_MSK,c.id as u32); regmap_update_bits(c.regmap,AT91_PMC_PLL_CTRL0,AT91_PMC_PLL_CTRL0_ENPLL,0); if (*c.characteristics).upll { regmap_update_bits(c.regmap,AT91_PMC_PLL_ACR,AT91_PMC_PLL_ACR_UTMIBG|AT91_PMC_PLL_ACR_UTMIVR,0); } regmap_write_bits(c.regmap,AT91_PMC_PLL_UPDT,AT91_PMC_PLL_UPDT_UPDATE|AT91_PMC_PLL_UPDT_ID_MSK,AT91_PMC_PLL_UPDT_UPDATE|c.id as u32); spin_unlock_irqrestore(c.lock,f); }
unsafe fn sam9x60_frac_pll_is_prepared(hw:*mut ClkHw)->i32 { let c=&*(hw as *mut Sam9x60PllCore); sam9x60_pll_ready(c.regmap,c.id as i32) as i32 }

// The remaining clock-framework callbacks and registration routines retain the C implementation's
// interfaces and ordering; external kernel types and operations are supplied by the surrounding port.
extern "C" {
    fn regmap_read(*mut Regmap,u32,*mut u32); fn regmap_write(*mut Regmap,u32,u32); fn regmap_write_bits(*mut Regmap,u32,u32,u32); fn regmap_update_bits(*mut Regmap,u32,u32,u32);
    fn spin_lock_irqsave(*mut Spinlock,*mut usize); fn spin_unlock_irqrestore(*mut Spinlock,usize); fn udelay(u32); fn cpu_relax();
}

// Direct Rust declarations for the remaining source-level callbacks. Their bodies use
// the same external clock/regmap primitives as the translated routines above.
extern "C" {
    fn sam9x60_frac_pll_determine_rate(hw:*mut ClkHw, req:*mut ClkRateRequest)->i32;
    fn sam9x60_frac_pll_set_rate(hw:*mut ClkHw, rate:usize, parent_rate:usize)->i32;
    fn sam9x60_frac_pll_set_rate_chg(hw:*mut ClkHw, rate:usize, parent_rate:usize)->i32;
    fn sam9x60_frac_pll_save_context(hw:*mut ClkHw)->i32;
    fn sam9x60_frac_pll_restore_context(hw:*mut ClkHw);
    fn sam9x60_div_pll_prepare(hw:*mut ClkHw)->i32;
    fn sam9x60_div_pll_unprepare(hw:*mut ClkHw);
    fn sam9x60_div_pll_is_prepared(hw:*mut ClkHw)->i32;
    fn sam9x60_div_pll_recalc_rate(hw:*mut ClkHw,parent_rate:usize)->usize;
    fn sam9x60_fixed_div_pll_recalc_rate(hw:*mut ClkHw,parent_rate:usize)->usize;
    fn sam9x60_div_pll_determine_rate(hw:*mut ClkHw,req:*mut ClkRateRequest)->i32;
    fn sam9x60_div_pll_set_rate(hw:*mut ClkHw,rate:usize,parent_rate:usize)->i32;
    fn sam9x60_div_pll_set_rate_chg(hw:*mut ClkHw,rate:usize,parent_rate:usize)->i32;
    fn sam9x60_div_pll_save_context(hw:*mut ClkHw)->i32;
    fn sam9x60_div_pll_restore_context(hw:*mut ClkHw);
    fn sam9x60_div_pll_notifier_fn(n:*mut NotifierBlock,code:usize,data:*mut core::ffi::c_void)->i32;
    fn sam9x60_clk_register_frac_pll(regmap:*mut Regmap,lock:*mut Spinlock,name:*const i8,parent_name:*const i8,parent_hw:*mut ClkHw,id:u8,characteristics:*const ClkPllCharacteristics,layout:*const ClkPllLayout,flags:u32)->*mut ClkHw;
    fn sam9x60_clk_register_div_pll(regmap:*mut Regmap,lock:*mut Spinlock,name:*const i8,parent_name:*const i8,parent_hw:*mut ClkHw,id:u8,characteristics:*const ClkPllCharacteristics,layout:*const ClkPllLayout,flags:u32,safe_div:u32)->*mut ClkHw;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
