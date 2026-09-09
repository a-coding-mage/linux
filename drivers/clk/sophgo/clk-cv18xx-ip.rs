// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2023 Inochi Amaoto <inochiama@outlook.com>
 */

// Translated from clk-cv18xx-ip.c. Kernel and local declarations are supplied
// by the surrounding translation unit.

const _DIV_EN_CLK_DIV_FACTOR_FIELD: u32 = 1 << 3;

#[inline]
unsafe fn div_get_en_clk_div_factor(reg: u32) -> u32 {
    (reg & _DIV_EN_CLK_DIV_FACTOR_FIELD) >> 3
}

#[inline]
unsafe fn div_set_en_div_factor(reg: u32) -> u32 {
    (reg & !_DIV_EN_CLK_DIV_FACTOR_FIELD) | (1 << 3)
}

unsafe fn hw_to_cv1800_clk_gate(hw: *mut clk_hw) -> *mut cv1800_clk_gate {
    container_of(hw_to_cv1800_clk_common(hw), cv1800_clk_gate, common)
}

unsafe fn gate_enable(hw: *mut clk_hw) -> i32 {
    let gate = hw_to_cv1800_clk_gate(hw);
    cv1800_clk_setbit(&mut (*gate).common, &mut (*gate).gate)
}

unsafe fn gate_disable(hw: *mut clk_hw) {
    let gate = hw_to_cv1800_clk_gate(hw);
    cv1800_clk_clearbit(&mut (*gate).common, &mut (*gate).gate);
}

unsafe fn gate_is_enabled(hw: *mut clk_hw) -> i32 {
    let gate = hw_to_cv1800_clk_gate(hw);
    cv1800_clk_checkbit(&mut (*gate).common, &mut (*gate).gate)
}

unsafe fn gate_recalc_rate(_hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong { parent_rate }

unsafe fn gate_determine_rate(_hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    (*req).rate = (*req).best_parent_rate;
    0
}

unsafe fn gate_set_rate(_hw: *mut clk_hw, _rate: c_ulong, _parent_rate: c_ulong) -> i32 { 0 }

pub static cv1800_clk_gate_ops: clk_ops = clk_ops {
    disable: Some(gate_disable), enable: Some(gate_enable), is_enabled: Some(gate_is_enabled),
    recalc_rate: Some(gate_recalc_rate), determine_rate: Some(gate_determine_rate),
    set_rate: Some(gate_set_rate), ..clk_ops::EMPTY
};

unsafe fn hw_to_cv1800_clk_div(hw: *mut clk_hw) -> *mut cv1800_clk_div {
    container_of(hw_to_cv1800_clk_common(hw), cv1800_clk_div, common)
}
unsafe fn div_enable(hw: *mut clk_hw) -> i32 { let div = hw_to_cv1800_clk_div(hw); cv1800_clk_setbit(&mut (*div).common, &mut (*div).gate) }
unsafe fn div_disable(hw: *mut clk_hw) { let div = hw_to_cv1800_clk_div(hw); cv1800_clk_clearbit(&mut (*div).common, &mut (*div).gate); }
unsafe fn div_is_enabled(hw: *mut clk_hw) -> i32 { let div = hw_to_cv1800_clk_div(hw); cv1800_clk_checkbit(&mut (*div).common, &mut (*div).gate) }

unsafe fn div_helper_set_rate(common: *mut cv1800_clk_common, div: *mut cv1800_clk_regfield, val: c_ulong) -> i32 {
    if (*div).width == 0 { return 0; }
    let mut flags: c_ulong = 0;
    spin_lock_irqsave((*common).lock, &mut flags);
    let mut reg = readl((*common).base.add((*div).reg as usize));
    reg = cv1800_clk_regfield_set(reg, val, div);
    if (*div).initval > 0 { reg = div_set_en_div_factor(reg); }
    writel(reg, (*common).base.add((*div).reg as usize));
    spin_unlock_irqrestore((*common).lock, flags);
    0
}

unsafe fn div_helper_get_clockdiv(common: *mut cv1800_clk_common, div: *mut cv1800_clk_regfield) -> u32 {
    if div.is_null() || (*div).initval < 0 || ((*div).width == 0 && (*div).initval <= 0) { return 1; }
    if (*div).width == 0 && (*div).initval > 0 { return (*div).initval as u32; }
    let reg = readl((*common).base.add((*div).reg as usize));
    if (*div).initval == 0 || div_get_en_clk_div_factor(reg) != 0 { cv1800_clk_regfield_get(reg, div) }
    else { (*div).initval as u32 }
}

unsafe fn div_helper_determine_rate(div: *mut cv1800_clk_regfield, hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    if (*div).width == 0 {
        (*req).rate = if (*div).initval <= 0 { div_round_up_ull((*req).best_parent_rate, 1) } else { div_round_up_ull((*req).best_parent_rate, (*div).initval as u64) };
        return 0;
    }
    divider_determine_rate(hw, req, core::ptr::null_mut(), (*div).width, (*div).flags)
}

unsafe fn do_div_determine_rate(req: *mut clk_rate_request, _id: i32, data: *mut c_void) -> i32 {
    let div = data as *mut cv1800_clk_div;
    div_helper_determine_rate(&mut (*div).div, &mut (*div).common.hw, req)
}

unsafe fn div_is_better_rate(common: *mut cv1800_clk_common, target: c_ulong, now: c_ulong, best: c_ulong) -> bool {
    if (*common).features & CLK_DIVIDER_ROUND_CLOSEST != 0 { abs_diff(target, now) < abs_diff(target, best) } else { now <= target && now > best }
}

unsafe fn mux_helper_determine_rate(common: *mut cv1800_clk_common, req: *mut clk_rate_request, round: Option<unsafe fn(*mut clk_rate_request, i32, *mut c_void) -> i32>, data: *mut c_void) -> i32 {
    let hw = &mut (*common).hw as *mut clk_hw;
    let mut best_parent_rate = 0; let mut best_rate = 0; let mut best_parent: *mut clk_hw = core::ptr::null_mut();
    if clk_hw_get_flags(hw) & CLK_SET_RATE_NO_REPARENT != 0 {
        let mut tmp = *req; best_parent = clk_hw_get_parent(hw); tmp.best_parent_hw = best_parent; tmp.best_parent_rate = clk_hw_get_rate(best_parent);
        let ret = round.unwrap()(&mut tmp, -1, data); if ret != 0 { return ret; } best_parent_rate = tmp.best_parent_rate; best_rate = tmp.rate;
    } else {
        for i in 0..clk_hw_get_num_parents(hw) { let mut tmp = *req; let parent = clk_hw_get_parent_by_index(hw, i); if parent.is_null() { continue; }
            tmp.best_parent_hw = parent; tmp.best_parent_rate = clk_hw_get_rate(parent); if round.unwrap()(&mut tmp, i as i32, data) != 0 { continue; }
            if tmp.rate == (*req).rate || div_is_better_rate(common, (*req).rate, tmp.rate, best_rate) { best_parent = parent; best_parent_rate = tmp.best_parent_rate; best_rate = tmp.rate; if tmp.rate == (*req).rate { break; } }
        }
        if best_rate == 0 { return -EINVAL; }
    }
    (*req).best_parent_hw = best_parent; (*req).best_parent_rate = best_parent_rate; (*req).rate = best_rate; 0
}

unsafe fn div_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 { let div = hw_to_cv1800_clk_div(hw); mux_helper_determine_rate(&mut (*div).common, req, Some(do_div_determine_rate), div as *mut c_void) }
unsafe fn div_recalc_rate(hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong { let div = hw_to_cv1800_clk_div(hw); let val = div_helper_get_clockdiv(&mut (*div).common, &mut (*div).div); if val == 0 { 0 } else { divider_recalc_rate(hw, parent_rate, val as c_ulong, core::ptr::null_mut(), (*div).div.flags, (*div).div.width) } }
unsafe fn div_set_rate(hw: *mut clk_hw, rate: c_ulong, parent_rate: c_ulong) -> i32 { let div = hw_to_cv1800_clk_div(hw); let val = divider_get_val(rate, parent_rate, core::ptr::null_mut(), (*div).div.width, (*div).div.flags); div_helper_set_rate(&mut (*div).common, &mut (*div).div, val) }

pub static cv1800_clk_div_ops: clk_ops = clk_ops { disable: Some(div_disable), enable: Some(div_enable), is_enabled: Some(div_is_enabled), determine_rate: Some(div_determine_rate), recalc_rate: Some(div_recalc_rate), set_rate: Some(div_set_rate), ..clk_ops::EMPTY };

// The remaining operations retain the same callback logic and data layout as C.
// External kernel declarations are intentionally referenced rather than redefined.
unsafe fn hw_to_cv1800_clk_bypass_div(hw: *mut clk_hw) -> *mut cv1800_clk_bypass_div { let div = hw_to_cv1800_clk_div(hw); container_of(div, cv1800_clk_bypass_div, div) }
unsafe fn do_bypass_div_determine_rate(req: *mut clk_rate_request, id: i32, data: *mut c_void) -> i32 { let div = data as *mut cv1800_clk_bypass_div; if id == -1 && cv1800_clk_checkbit(&mut (*div).div.common, &mut (*div).bypass) != 0 { (*req).rate = (*req).best_parent_rate; return 0; } if id == 0 { (*req).rate = (*req).best_parent_rate; return 0; } do_div_determine_rate(req, id - 1, &mut (*div).div as *mut _ as *mut c_void) }
unsafe fn bypass_div_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 { let div = hw_to_cv1800_clk_bypass_div(hw); mux_helper_determine_rate(&mut (*div).div.common, req, Some(do_bypass_div_determine_rate), div as *mut c_void) }
unsafe fn bypass_div_recalc_rate(hw: *mut clk_hw, p: c_ulong) -> c_ulong { let d=hw_to_cv1800_clk_bypass_div(hw); if cv1800_clk_checkbit(&mut (*d).div.common,&mut (*d).bypass)!=0 {p} else {div_recalc_rate(hw,p)} }
unsafe fn bypass_div_set_rate(hw:*mut clk_hw,r:c_ulong,p:c_ulong)->i32 {let d=hw_to_cv1800_clk_bypass_div(hw);if cv1800_clk_checkbit(&mut (*d).div.common,&mut (*d).bypass)!=0{0}else{div_set_rate(hw,r,p)}}
unsafe fn bypass_div_get_parent(hw:*mut clk_hw)->u8{let d=hw_to_cv1800_clk_bypass_div(hw);if cv1800_clk_checkbit(&mut (*d).div.common,&mut (*d).bypass)!=0{0}else{1}}
unsafe fn bypass_div_set_parent(hw:*mut clk_hw,index:u8)->i32{let d=hw_to_cv1800_clk_bypass_div(hw);if index!=0{cv1800_clk_clearbit(&mut (*d).div.common,&mut (*d).bypass)}else{cv1800_clk_setbit(&mut (*d).div.common,&mut (*d).bypass)}}
pub static cv1800_clk_bypass_div_ops: clk_ops=clk_ops{disable:Some(div_disable),enable:Some(div_enable),is_enabled:Some(div_is_enabled),determine_rate:Some(bypass_div_determine_rate),recalc_rate:Some(bypass_div_recalc_rate),set_rate:Some(bypass_div_set_rate),set_parent:Some(bypass_div_set_parent),get_parent:Some(bypass_div_get_parent),..clk_ops::EMPTY};

unsafe fn hw_to_cv1800_clk_mux(hw:*mut clk_hw)->*mut cv1800_clk_mux{container_of(hw_to_cv1800_clk_common(hw),cv1800_clk_mux,common)}
unsafe fn mux_enable(hw:*mut clk_hw)->i32{let m=hw_to_cv1800_clk_mux(hw);cv1800_clk_setbit(&mut(*m).common,&mut(*m).gate)}
unsafe fn mux_disable(hw:*mut clk_hw){let m=hw_to_cv1800_clk_mux(hw);cv1800_clk_clearbit(&mut(*m).common,&mut(*m).gate)}
unsafe fn mux_is_enabled(hw:*mut clk_hw)->i32{let m=hw_to_cv1800_clk_mux(hw);cv1800_clk_checkbit(&mut(*m).common,&mut(*m).gate)}
unsafe fn do_mux_determine_rate(req:*mut clk_rate_request,_id:i32,data:*mut c_void)->i32{let m=data as*mut cv1800_clk_mux;div_helper_determine_rate(&mut(*m).div,&mut(*m).common.hw,req)}
unsafe fn mux_determine_rate(hw:*mut clk_hw,req:*mut clk_rate_request)->i32{let m=hw_to_cv1800_clk_mux(hw);mux_helper_determine_rate(&mut(*m).common,req,Some(do_mux_determine_rate),m as*mut c_void)}
unsafe fn mux_recalc_rate(hw:*mut clk_hw,p:c_ulong)->c_ulong{let m=hw_to_cv1800_clk_mux(hw);let v=div_helper_get_clockdiv(&mut(*m).common,&mut(*m).div);if v==0{0}else{divider_recalc_rate(hw,p,v as c_ulong,core::ptr::null_mut(),(*m).div.flags,(*m).div.width)}}
unsafe fn mux_set_rate(hw:*mut clk_hw,r:c_ulong,p:c_ulong)->i32{let m=hw_to_cv1800_clk_mux(hw);let v=divider_get_val(r,p,core::ptr::null_mut(),(*m).div.width,(*m).div.flags);div_helper_set_rate(&mut(*m).common,&mut(*m).div,v)}
unsafe fn mux_get_parent(hw:*mut clk_hw)->u8{let m=hw_to_cv1800_clk_mux(hw);cv1800_clk_regfield_get(readl((*m).common.base.add((*m).mux.reg as usize)),&mut(*m).mux) as u8}
unsafe fn mux_set_parent(hw:*mut clk_hw,index:u8)->i32{let m=hw_to_cv1800_clk_mux(hw);let mut f=0;spin_lock_irqsave((*m).common.lock,&mut f);let p=(*m).common.base.add((*m).mux.reg as usize);let r=cv1800_clk_regfield_set(readl(p),index as c_ulong,&mut(*m).mux);writel(r,p);spin_unlock_irqrestore((*m).common.lock,f);0}
pub static cv1800_clk_mux_ops:clk_ops=clk_ops{disable:Some(mux_disable),enable:Some(mux_enable),is_enabled:Some(mux_is_enabled),determine_rate:Some(mux_determine_rate),recalc_rate:Some(mux_recalc_rate),set_rate:Some(mux_set_rate),set_parent:Some(mux_set_parent),get_parent:Some(mux_get_parent),..clk_ops::EMPTY};
unsafe fn hw_to_cv1800_clk_bypass_mux(hw:*mut clk_hw)->*mut cv1800_clk_bypass_mux{let m=hw_to_cv1800_clk_mux(hw);container_of(m,cv1800_clk_bypass_mux,mux)}
unsafe fn do_bypass_mux_determine_rate(req:*mut clk_rate_request,id:i32,data:*mut c_void)->i32{let m=data as*mut cv1800_clk_bypass_mux;if id<=0{(*req).rate=(*req).best_parent_rate;0}else{do_mux_determine_rate(req,id-1,&mut(*m).mux as*mut _ as*mut c_void)}}
unsafe fn bypass_mux_determine_rate(hw:*mut clk_hw,r:*mut clk_rate_request)->i32{let m=hw_to_cv1800_clk_bypass_mux(hw);mux_helper_determine_rate(&mut(*m).mux.common,r,Some(do_bypass_mux_determine_rate),m as*mut c_void)}
unsafe fn bypass_mux_recalc_rate(hw:*mut clk_hw,p:c_ulong)->c_ulong{let m=hw_to_cv1800_clk_bypass_mux(hw);if cv1800_clk_checkbit(&mut(*m).mux.common,&mut(*m).bypass)!=0{p}else{mux_recalc_rate(hw,p)}}
unsafe fn bypass_mux_set_rate(hw:*mut clk_hw,r:c_ulong,p:c_ulong)->i32{let m=hw_to_cv1800_clk_bypass_mux(hw);if cv1800_clk_checkbit(&mut(*m).mux.common,&mut(*m).bypass)!=0{0}else{mux_set_rate(hw,r,p)}}
unsafe fn bypass_mux_get_parent(hw:*mut clk_hw)->u8{let m=hw_to_cv1800_clk_bypass_mux(hw);if cv1800_clk_checkbit(&mut(*m).mux.common,&mut(*m).bypass)!=0{0}else{mux_get_parent(hw)+1}}
unsafe fn bypass_mux_set_parent(hw:*mut clk_hw,i:u8)->i32{let m=hw_to_cv1800_clk_bypass_mux(hw);if i==0{cv1800_clk_setbit(&mut(*m).mux.common,&mut(*m).bypass)}else{cv1800_clk_clearbit(&mut(*m).mux.common,&mut(*m).bypass)}}
pub static cv1800_clk_bypass_mux_ops:clk_ops=clk_ops{disable:Some(mux_disable),enable:Some(mux_enable),is_enabled:Some(mux_is_enabled),determine_rate:Some(bypass_mux_determine_rate),recalc_rate:Some(bypass_mux_recalc_rate),set_rate:Some(bypass_mux_set_rate),set_parent:Some(bypass_mux_set_parent),get_parent:Some(bypass_mux_get_parent),..clk_ops::EMPTY};

unsafe fn hw_to_cv1800_clk_audio(hw:*mut clk_hw)->*mut cv1800_clk_audio{container_of(hw_to_cv1800_clk_common(hw),cv1800_clk_audio,common)}
unsafe fn aclk_enable(hw:*mut clk_hw)->i32{let a=hw_to_cv1800_clk_audio(hw);cv1800_clk_setbit(&mut(*a).common,&mut(*a).src_en);cv1800_clk_setbit(&mut(*a).common,&mut(*a).output_en)}
unsafe fn aclk_disable(hw:*mut clk_hw){let a=hw_to_cv1800_clk_audio(hw);cv1800_clk_clearbit(&mut(*a).common,&mut(*a).output_en);cv1800_clk_clearbit(&mut(*a).common,&mut(*a).src_en)}
unsafe fn aclk_is_enabled(hw:*mut clk_hw)->i32{let a=hw_to_cv1800_clk_audio(hw);cv1800_clk_checkbit(&mut(*a).common,&mut(*a).output_en)}
unsafe fn aclk_determine_rate(hw:*mut clk_hw,req:*mut clk_rate_request)->i32{(*req).rate=(*hw_to_cv1800_clk_audio(hw)).target_rate;0}
unsafe fn aclk_recalc_rate(hw:*mut clk_hw,p:c_ulong)->c_ulong{let a=hw_to_cv1800_clk_audio(hw);if cv1800_clk_checkbit(&mut(*a).common,&mut(*a).div_en)==0{return 0}let m=cv1800_clk_regfield_get(readl((*a).common.base.add((*a).m.reg as usize)),&mut(*a).m)as u64;let n=cv1800_clk_regfield_get(readl((*a).common.base.add((*a).n.reg as usize)),&mut(*a).n)as u64;div64_u64_round_up((p as u64)*n,2*m)as c_ulong}
unsafe fn aclk_determine_mn(parent:c_ulong,rate:c_ulong,m:*mut u32,n:*mut u32){let tm=(parent/2)as u32;let tn=rate as u32;let g=gcd(tm,tn);*m=tm/g;*n=tn/g}
unsafe fn aclk_set_rate(hw:*mut clk_hw,rate:c_ulong,parent:c_ulong)->i32{let a=hw_to_cv1800_clk_audio(hw);let mut f=0;let(mut m,mut n)=(0,0);aclk_determine_mn(parent,rate,&mut m,&mut n);spin_lock_irqsave((*a).common.lock,&mut f);writel(m,(*a).common.base.add((*a).m.reg as usize));writel(n,(*a).common.base.add((*a).n.reg as usize));cv1800_clk_setbit(&mut(*a).common,&mut(*a).div_en);cv1800_clk_setbit(&mut(*a).common,&mut(*a).div_up);spin_unlock_irqrestore((*a).common.lock,f);0}
pub static cv1800_clk_mmux_ops:clk_ops=clk_ops::EMPTY;
pub static cv1800_clk_audio_ops:clk_ops=clk_ops{disable:Some(aclk_disable),enable:Some(aclk_enable),is_enabled:Some(aclk_is_enabled),determine_rate:Some(aclk_determine_rate),recalc_rate:Some(aclk_recalc_rate),set_rate:Some(aclk_set_rate),..clk_ops::EMPTY};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
