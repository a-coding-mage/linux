// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (C) 2013 Boris BREZILLON <b.brezillon@overkiz.com>
 */

// Dependencies supplied by the surrounding kernel translation are intentionally external.

const SLOW_CLOCK_FREQ: u32 = 32768;
const MAINF_DIV: u32 = 16;
const MAINFRDY_TIMEOUT: u32 = ((MAINF_DIV + 1) * USEC_PER_SEC) / SLOW_CLOCK_FREQ;
const MAINF_LOOP_MIN_WAIT: u32 = USEC_PER_SEC / SLOW_CLOCK_FREQ;
const MAINF_LOOP_MAX_WAIT: u32 = MAINFRDY_TIMEOUT;
const MOR_KEY_MASK: u32 = 0xff << 16;

#[repr(C)]
struct clk_main_osc {
    hw: clk_hw,
    regmap: *mut regmap,
    pms: at91_clk_pms,
}

#[repr(C)]
struct clk_main_rc_osc {
    hw: clk_hw,
    regmap: *mut regmap,
    frequency: c_ulong,
    accuracy: c_ulong,
    pms: at91_clk_pms,
}

#[repr(C)]
struct clk_rm9200_main {
    hw: clk_hw,
    regmap: *mut regmap,
}

#[repr(C)]
struct clk_sam9x5_main {
    hw: clk_hw,
    regmap: *mut regmap,
    pms: at91_clk_pms,
    parent: u8,
}

#[inline]
unsafe fn clk_main_osc_ready(regmap: *mut regmap) -> bool {
    let mut status: c_uint = 0;
    regmap_read(regmap, AT91_PMC_SR, &mut status);
    (status & AT91_PMC_MOSCS) != 0
}

unsafe fn clk_main_osc_prepare(hw: *mut clk_hw) -> c_int {
    let osc = &mut *(hw as *mut clk_main_osc);
    let regmap = osc.regmap;
    let mut tmp: u32 = 0;
    regmap_read(regmap, AT91_CKGR_MOR, &mut tmp);
    tmp &= !MOR_KEY_MASK;
    if (tmp & AT91_PMC_OSCBYPASS) != 0 { return 0; }
    if (tmp & AT91_PMC_MOSCEN) == 0 {
        tmp |= AT91_PMC_MOSCEN | AT91_PMC_KEY;
        regmap_write(regmap, AT91_CKGR_MOR, tmp);
    }
    while !clk_main_osc_ready(regmap) { cpu_relax(); }
    0
}

unsafe fn clk_main_osc_unprepare(hw: *mut clk_hw) {
    let osc = &mut *(hw as *mut clk_main_osc);
    let mut tmp: u32 = 0;
    regmap_read(osc.regmap, AT91_CKGR_MOR, &mut tmp);
    if (tmp & AT91_PMC_OSCBYPASS) != 0 || (tmp & AT91_PMC_MOSCEN) == 0 { return; }
    tmp &= !(AT91_PMC_KEY | AT91_PMC_MOSCEN);
    regmap_write(osc.regmap, AT91_CKGR_MOR, tmp | AT91_PMC_KEY);
}

unsafe fn clk_main_osc_is_prepared(hw: *mut clk_hw) -> c_int {
    let osc = &mut *(hw as *mut clk_main_osc);
    let mut tmp: u32 = 0; let mut status: u32 = 0;
    regmap_read(osc.regmap, AT91_CKGR_MOR, &mut tmp);
    if (tmp & AT91_PMC_OSCBYPASS) != 0 { return 1; }
    regmap_read(osc.regmap, AT91_PMC_SR, &mut status);
    if (status & AT91_PMC_MOSCS) != 0 && clk_main_parent_select(tmp) != 0 { 1 } else { 0 }
}

unsafe fn clk_main_osc_save_context(hw: *mut clk_hw) -> c_int {
    let osc = &mut *(hw as *mut clk_main_osc); osc.pms.status = clk_main_osc_is_prepared(hw); 0
}
unsafe fn clk_main_osc_restore_context(hw: *mut clk_hw) {
    let osc = &mut *(hw as *mut clk_main_osc); if osc.pms.status != 0 { clk_main_osc_prepare(hw); }
}

static main_osc_ops: clk_ops = clk_ops {
    prepare: Some(clk_main_osc_prepare), unprepare: Some(clk_main_osc_unprepare),
    is_prepared: Some(clk_main_osc_is_prepared), save_context: Some(clk_main_osc_save_context),
    restore_context: Some(clk_main_osc_restore_context), ..unsafe { core::mem::zeroed() }
};

#[inline]
unsafe fn clk_main_parent_select(s: u32) -> u8 { if (s & (AT91_PMC_MOSCEN | AT91_PMC_OSCBYPASS)) != 0 { 1 } else { 0 } }

unsafe fn at91_clk_register_main_osc(regmap: *mut regmap, name: *const c_char, parent_name: *const c_char, parent_data: *mut clk_parent_data, bypass: bool) -> *mut clk_hw {
    if name.is_null() || (parent_name.is_null() && parent_data.is_null()) { return ERR_PTR(-EINVAL); }
    let osc = kzalloc_obj::<clk_main_osc>(); if osc.is_null() { return ERR_PTR(-ENOMEM); }
    (*osc).hw.init = &mut clk_init_data { name, ops: &main_osc_ops, parent_data, parent_names: if parent_data.is_null() { &parent_name } else { core::ptr::null() }, num_parents: 1, flags: CLK_IGNORE_UNUSED, ..core::mem::zeroed() };
    (*osc).regmap = regmap;
    if bypass { regmap_update_bits(regmap, AT91_CKGR_MOR, MOR_KEY_MASK | AT91_PMC_OSCBYPASS, AT91_PMC_OSCBYPASS | AT91_PMC_KEY); }
    let hw = &mut (*osc).hw; let ret = clk_hw_register(core::ptr::null_mut(), hw);
    if ret != 0 { kfree(osc as *mut _); return ERR_PTR(ret); } hw
}

unsafe fn clk_main_rc_osc_ready(regmap: *mut regmap) -> bool { let mut s=0; regmap_read(regmap,AT91_PMC_SR,&mut s); (s & AT91_PMC_MOSCRCS)!=0 }
unsafe fn clk_main_rc_osc_prepare(hw:*mut clk_hw)->c_int { let o=&mut*(hw as *mut clk_main_rc_osc); let mut m=0; regmap_read(o.regmap,AT91_CKGR_MOR,&mut m); if m&AT91_PMC_MOSCRCEN==0 { regmap_update_bits(o.regmap,AT91_CKGR_MOR,MOR_KEY_MASK|AT91_PMC_MOSCRCEN,AT91_PMC_MOSCRCEN|AT91_PMC_KEY); } while !clk_main_rc_osc_ready(o.regmap){cpu_relax();} 0 }
unsafe fn clk_main_rc_osc_unprepare(hw:*mut clk_hw){let o=&mut*(hw as *mut clk_main_rc_osc);let mut m=0;regmap_read(o.regmap,AT91_CKGR_MOR,&mut m);if m&AT91_PMC_MOSCRCEN!=0{regmap_update_bits(o.regmap,AT91_CKGR_MOR,MOR_KEY_MASK|AT91_PMC_MOSCRCEN,AT91_PMC_KEY);}}
unsafe fn clk_main_rc_osc_is_prepared(hw:*mut clk_hw)->c_int{let o=&mut*(hw as *mut clk_main_rc_osc);let(mut m,mut s)=(0,0);regmap_read(o.regmap,AT91_CKGR_MOR,&mut m);regmap_read(o.regmap,AT91_PMC_SR,&mut s);if m&AT91_PMC_MOSCRCEN!=0&&s&AT91_PMC_MOSCRCS!=0{1}else{0}}
unsafe fn clk_main_rc_osc_recalc_rate(hw:*mut clk_hw,_:c_ulong)->c_ulong{(*(hw as *mut clk_main_rc_osc)).frequency}
unsafe fn clk_main_rc_osc_recalc_accuracy(hw:*mut clk_hw,_:c_ulong)->c_ulong{(*(hw as *mut clk_main_rc_osc)).accuracy}
unsafe fn clk_main_rc_osc_save_context(hw:*mut clk_hw)->c_int{let o=&mut*(hw as *mut clk_main_rc_osc);o.pms.status=clk_main_rc_osc_is_prepared(hw);0}
unsafe fn clk_main_rc_osc_restore_context(hw:*mut clk_hw){let o=&mut*(hw as *mut clk_main_rc_osc);if o.pms.status!=0{clk_main_rc_osc_prepare(hw);}}

unsafe fn at91_clk_register_main_rc_osc(regmap:*mut regmap,name:*const c_char,frequency:u32,accuracy:u32)->*mut clk_hw{if name.is_null()||frequency==0{return ERR_PTR(-EINVAL)}let o=kzalloc_obj::<clk_main_rc_osc>();if o.is_null(){return ERR_PTR(-ENOMEM)}(*o).regmap=regmap;(*o).frequency=frequency as c_ulong;(*o).accuracy=accuracy as c_ulong;let mut init:clk_init_data=core::mem::zeroed();init.name=name;init.num_parents=0;init.flags=CLK_IGNORE_UNUSED;(*o).hw.init=&mut init;let r=clk_hw_register(core::ptr::null_mut(),&mut(*o).hw);if r!=0{kfree(o as *mut _);ERR_PTR(r)}else{&mut(*o).hw}}

unsafe fn clk_main_probe_frequency(regmap:*mut regmap)->c_int{let timeout=jiffies()+usecs_to_jiffies(MAINFRDY_TIMEOUT);loop{let prep=jiffies();let mut mcfr=0;regmap_read(regmap,AT91_CKGR_MCFR,&mut mcfr);if mcfr&AT91_PMC_MAINRDY!=0{return 0}if system_state<SYSTEM_RUNNING{udelay(MAINF_LOOP_MIN_WAIT)}else{usleep_range(MAINF_LOOP_MIN_WAIT,MAINF_LOOP_MAX_WAIT)}if !time_before(prep,timeout){break}}-ETIMEDOUT}
unsafe fn clk_main_recalc_rate(regmap:*mut regmap,parent_rate:c_ulong)->c_ulong{if parent_rate!=0{return parent_rate}pr_warn!("Main crystal frequency not set, using approximate value\n");let mut m=0;regmap_read(regmap,AT91_CKGR_MCFR,&mut m);if m&AT91_PMC_MAINRDY==0{0}else{((m&AT91_PMC_MAINF)*SLOW_CLOCK_FREQ/MAINF_DIV) as c_ulong}}

unsafe fn clk_rm9200_main_prepare(hw:*mut clk_hw)->c_int{clk_main_probe_frequency((*(hw as *mut clk_rm9200_main)).regmap)}
unsafe fn clk_rm9200_main_is_prepared(hw:*mut clk_hw)->c_int{let mut s=0;regmap_read((*(hw as *mut clk_rm9200_main)).regmap,AT91_CKGR_MCFR,&mut s);if s&AT91_PMC_MAINRDY!=0{1}else{0}}
unsafe fn clk_rm9200_main_recalc_rate(hw:*mut clk_hw,p:c_ulong)->c_ulong{clk_main_recalc_rate((*(hw as *mut clk_rm9200_main)).regmap,p)}
unsafe fn at91_clk_register_rm9200_main(regmap:*mut regmap,name:*const c_char,parent_name:*const c_char,parent_hw:*mut clk_hw)->*mut clk_hw{if name.is_null()||(parent_name.is_null()&&parent_hw.is_null()){return ERR_PTR(-EINVAL)}let o=kzalloc_obj::<clk_rm9200_main>();if o.is_null(){return ERR_PTR(-ENOMEM)}(*o).regmap=regmap;let mut i:clk_init_data=core::mem::zeroed();i.name=name;i.num_parents=1;i.parent_names=if parent_hw.is_null(){&parent_name} else {core::ptr::null()};i.parent_hws=if parent_hw.is_null(){core::ptr::null()}else{&parent_hw};(*o).hw.init=&mut i;let r=clk_hw_register(core::ptr::null_mut(),&mut(*o).hw);if r!=0{kfree(o as *mut _);ERR_PTR(r)}else{&mut(*o).hw}}

unsafe fn clk_sam9x5_main_ready(r:*mut regmap)->bool{let mut s=0;regmap_read(r,AT91_PMC_SR,&mut s);s&AT91_PMC_MOSCSELS!=0}
unsafe fn clk_sam9x5_main_prepare(hw:*mut clk_hw)->c_int{let o=&mut*(hw as *mut clk_sam9x5_main);while !clk_sam9x5_main_ready(o.regmap){cpu_relax();}clk_main_probe_frequency(o.regmap)}
unsafe fn clk_sam9x5_main_is_prepared(hw:*mut clk_hw)->c_int{if clk_sam9x5_main_ready((*(hw as *mut clk_sam9x5_main)).regmap){1}else{0}}
unsafe fn clk_sam9x5_main_recalc_rate(hw:*mut clk_hw,p:c_ulong)->c_ulong{let o=&*(hw as *mut clk_sam9x5_main);clk_main_recalc_rate(o.regmap,p)}
unsafe fn clk_sam9x5_main_set_parent(hw:*mut clk_hw,index:u8)->c_int{let o=&mut*(hw as *mut clk_sam9x5_main);if index>1{return -EINVAL}let mut t=0;regmap_read(o.regmap,AT91_CKGR_MOR,&mut t);if index!=0&&t&AT91_PMC_MOSCSEL==0{t=AT91_PMC_MOSCSEL}else if index==0&&t&AT91_PMC_MOSCSEL!=0{t=0}else{return 0}regmap_update_bits(o.regmap,AT91_CKGR_MOR,AT91_PMC_MOSCSEL|MOR_KEY_MASK,t|AT91_PMC_KEY);while !clk_sam9x5_main_ready(o.regmap){cpu_relax();}0}
unsafe fn clk_sam9x5_main_get_parent(hw:*mut clk_hw)->u8{let o=&*(hw as *mut clk_sam9x5_main);let mut s=0;regmap_read(o.regmap,AT91_CKGR_MOR,&mut s);clk_main_parent_select(s)}
unsafe fn clk_sam9x5_main_save_context(hw:*mut clk_hw)->c_int{let o=&mut*(hw as *mut clk_sam9x5_main);o.pms.status=clk_main_rc_osc_is_prepared(&mut o.hw);o.pms.parent=clk_sam9x5_main_get_parent(&mut o.hw) as _;0}
unsafe fn clk_sam9x5_main_restore_context(hw:*mut clk_hw){let o=&mut*(hw as *mut clk_sam9x5_main);if clk_sam9x5_main_set_parent(hw,o.pms.parent as u8)!=0{return}if o.pms.status!=0{clk_sam9x5_main_prepare(hw);}}

unsafe fn at91_clk_register_sam9x5_main(regmap:*mut regmap,name:*const c_char,parent_names:*const *const c_char,parent_hws:*mut *mut clk_hw,num_parents:c_int)->*mut clk_hw{if name.is_null()||(parent_hws.is_null()&&parent_names.is_null())||num_parents==0{return ERR_PTR(-EINVAL)}let o=kzalloc_obj::<clk_sam9x5_main>();if o.is_null(){return ERR_PTR(-ENOMEM)}(*o).regmap=regmap;let mut s=0;regmap_read(regmap,AT91_CKGR_MOR,&mut s);(*o).parent=clk_main_parent_select(s);let mut i:clk_init_data=core::mem::zeroed();i.name=name;i.parent_names=parent_names;i.parent_hws=parent_hws as *const *const clk_hw;i.num_parents=num_parents;i.flags=CLK_SET_PARENT_GATE;(*o).hw.init=&mut i;let r=clk_hw_register(core::ptr::null_mut(),&mut(*o).hw);if r!=0{kfree(o as *mut _);ERR_PTR(r)}else{&mut(*o).hw}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
