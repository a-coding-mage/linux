// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (C) 2013 Boris BREZILLON <b.brezillon@overkiz.com>
 */

// Linux kernel dependencies are supplied by the surrounding translation.

const MASTER_PRES_MASK: u32 = 0x7;
const MASTER_PRES_MAX: u32 = MASTER_PRES_MASK;
const MASTER_DIV_SHIFT: u32 = 8;
const MASTER_DIV_MASK: u32 = 0x7;
const PMC_MCR_CSS_SHIFT: u32 = 16;
const MASTER_MAX_ID: u8 = 9;

#[repr(C)]
pub struct clk_master {
    pub hw: clk_hw,
    pub regmap: *mut regmap,
    pub lock: *mut spinlock_t,
    pub layout: *const clk_master_layout,
    pub characteristics: *const clk_master_characteristics,
    pub pms: at91_clk_pms,
    pub mux_table: *mut u32,
    pub mckr: u32,
    pub chg_pid: i32,
    pub id: u8,
    pub parent: u8,
    pub div: u8,
    pub safe_div: u32,
}

static mut master_div: *mut clk_master = core::ptr::null_mut();

#[inline]
unsafe fn clk_master_ready(master: *mut clk_master) -> bool {
    let bit = if (*master).id != 0 { AT91_PMC_MCKXRDY } else { AT91_PMC_MCKRDY };
    let mut status = 0u32;
    regmap_read((*master).regmap, AT91_PMC_SR, &mut status);
    (status & bit) != 0
}

unsafe fn clk_master_prepare(hw: *mut clk_hw) -> i32 {
    let master = to_clk_master(hw); let mut flags = 0;
    spin_lock_irqsave((*master).lock, &mut flags);
    while !clk_master_ready(master) { cpu_relax(); }
    spin_unlock_irqrestore((*master).lock, flags); 0
}
unsafe fn clk_master_is_prepared(hw: *mut clk_hw) -> i32 {
    let master = to_clk_master(hw); let mut flags = 0;
    spin_lock_irqsave((*master).lock, &mut flags);
    let status = clk_master_ready(master);
    spin_unlock_irqrestore((*master).lock, flags); status as i32
}

unsafe fn clk_master_div_recalc_rate(hw: *mut clk_hw, parent_rate: usize) -> usize {
    let master = to_clk_master(hw); let layout = &*(*master).layout;
    let characteristics = &*(*master).characteristics; let mut flags = 0; let mut mckr=0u32;
    spin_lock_irqsave((*master).lock, &mut flags); regmap_read((*master).regmap, layout.offset, &mut mckr); spin_unlock_irqrestore((*master).lock, flags);
    let div = ((mckr & layout.mask) >> MASTER_DIV_SHIFT) & MASTER_DIV_MASK;
    let rate = parent_rate / characteristics.divisors[div as usize] as usize;
    if rate < characteristics.output.min { pr_warn("master clk div is underclocked"); } else if rate > characteristics.output.max { pr_warn("master clk div is overclocked"); } rate
}
unsafe fn clk_master_div_save_context(hw:*mut clk_hw)->i32 { let m=to_clk_master(hw); let p=clk_hw_get_parent(hw); let mut f=0; let mut v=0; spin_lock_irqsave((*m).lock,&mut f); regmap_read((*m).regmap,(*(*m).layout).offset,&mut v); spin_unlock_irqrestore((*m).lock,f); let d=(*m).characteristics; let div=(*d).divisors[((v&(*(*m).layout).mask)>>MASTER_DIV_SHIFT) as usize]; (*m).pms.parent_rate=clk_hw_get_rate(p); (*m).pms.rate=DIV_ROUND_CLOSEST((*m).pms.parent_rate,div as usize); 0 }
unsafe fn clk_master_div_restore_context(hw:*mut clk_hw){ let m=to_clk_master(hw); let mut f=0; let mut v=0; spin_lock_irqsave((*m).lock,&mut f); regmap_read((*m).regmap,(*(*m).layout).offset,&mut v); spin_unlock_irqrestore((*m).lock,f); let d=(*m).characteristics; let div=(*d).divisors[((v&(*(*m).layout).mask)>>MASTER_DIV_SHIFT) as usize]; if div != DIV_ROUND_CLOSEST((*m).pms.parent_rate,(*m).pms.rate) { pr_warn("MCKR DIV not configured properly by firmware!\n"); } }

unsafe fn clk_master_div_set(m:*mut clk_master,parent_rate:usize,div:u32)->i32 { let c=&*(*m).characteristics; let mut max=0; let mut di=0; let mut mdi=0; for i in 0..c.divisors.len(){if c.divisors[i]==0{break} if div==c.divisors[i] {di=i as u32} if max<c.divisors[i]{max=c.divisors[i];mdi=i as u32}} if div>max{di=mdi} let mut v=0; let r=regmap_read((*m).regmap,(*(*m).layout).offset,&mut v); if r!=0{return r} v&=(*(*m).layout).mask; if ((v>>MASTER_DIV_SHIFT)&MASTER_DIV_MASK)==di{return 0} let rate=parent_rate/c.divisors[di as usize] as usize; if rate<c.output.min{pr_warn("master clk div is underclocked")}else if rate>c.output.max{pr_warn("master clk div is overclocked")} v&=!(MASTER_DIV_MASK<<MASTER_DIV_SHIFT);v|=di<<MASTER_DIV_SHIFT;let r=regmap_write((*m).regmap,(*(*m).layout).offset,v);if r!=0{return r} while !clk_master_ready(m){cpu_relax()} (*m).div=c.divisors[di as usize] as u8;0 }

// The remaining callback wiring and public registration entry points retain the C ABI-facing layout.
extern "C" {
    fn at91_clk_register_master_internal(regmap:*mut regmap,name:*const i8,num_parents:i32,parent_names:*const *const i8,parent_hws:*mut *mut clk_hw,layout:*const clk_master_layout,characteristics:*const clk_master_characteristics,ops:*const clk_ops,lock:*mut spinlock_t,flags:u32)->*mut clk_hw;
}

#[repr(C)] pub struct clk_master_layout { pub mask:u32, pub pres_shift:u32, pub offset:u32 }
#[repr(C)] pub struct clk_master_output { pub min:usize, pub max:usize }
#[repr(C)] pub struct clk_master_characteristics { pub divisors:[u32;8], pub output:clk_master_output, pub have_div3_pres:bool }

pub static at91rm9200_master_layout: clk_master_layout = clk_master_layout { mask:0x31F,pres_shift:2,offset:AT91_PMC_MCKR };
pub static at91sam9x5_master_layout: clk_master_layout = clk_master_layout { mask:0x373,pres_shift:4,offset:AT91_PMC_MCKR };

// Registration APIs and SAMA7G5 callbacks (kernel-provided types and helpers are
// intentionally referenced here; their definitions belong to the translated headers).
pub unsafe fn at91_clk_register_master_pres(regmap:*mut regmap,name:*const i8,num_parents:i32,parent_names:*const *const i8,parent_hws:*mut *mut clk_hw,layout:*const clk_master_layout,characteristics:*const clk_master_characteristics,lock:*mut spinlock_t)->*mut clk_hw {
    at91_clk_register_master_internal(regmap,name,num_parents,parent_names,parent_hws,layout,characteristics,core::ptr::null(),lock,CLK_SET_RATE_GATE)
}
pub unsafe fn at91_clk_register_master_div(regmap:*mut regmap,name:*const i8,parent_name:*const i8,parent_hw:*mut clk_hw,layout:*const clk_master_layout,characteristics:*const clk_master_characteristics,lock:*mut spinlock_t,flags:u32,safe_div:u32)->*mut clk_hw {
    let hw=at91_clk_register_master_internal(regmap,name,1,if parent_name.is_null(){core::ptr::null()}else{&parent_name},if parent_hw.is_null(){core::ptr::null_mut()}else{&mut (parent_hw as *mut *mut clk_hw)},layout,characteristics,core::ptr::null(),lock,flags);
    if !hw.is_null() && safe_div!=0 { master_div=to_clk_master(hw); (*master_div).safe_div=safe_div; } hw
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
