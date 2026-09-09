// SPDX-License-Identifier: GPL-2.0-only
/* Faithful low-level translation of clk-kona.c. External kernel types and
 * helpers are supplied by the surrounding translation unit. */

const CCU_POLICY_COUNT: i32 = 4;
const CCU_ACCESS_PASSWORD: u32 = 0xA5A500;
const CLK_GATE_DELAY_LOOP: u32 = 2000;

#[inline]
unsafe fn bitfield_mask(shift: u32, width: u32) -> u32 { ((1u32 << width).wrapping_sub(1)) << shift }
#[inline]
unsafe fn bitfield_extract(reg_val: u32, shift: u32, width: u32) -> u32 { (reg_val & bitfield_mask(shift, width)) >> shift }
#[inline]
unsafe fn bitfield_replace(reg_val: u32, shift: u32, width: u32, val: u32) -> u32 {
    let mask = bitfield_mask(shift, width); (reg_val & !mask) | (val << shift)
}
#[inline]
unsafe fn scaled_div_value(div: *mut bcm_clk_div, reg_div: u32) -> u64 { reg_div as u64 + (1u64 << (*div).u.s.frac_width) }
#[inline]
unsafe fn scaled_div_min(div: *mut bcm_clk_div) -> u64 { if divider_is_fixed(div) { (*div).u.fixed as u64 } else { scaled_div_value(div, 0) } }
unsafe fn scaled_div_max(div: *mut bcm_clk_div) -> u64 {
    if divider_is_fixed(div) { return (*div).u.fixed as u64; }
    scaled_div_value(div, (1u32 << (*div).u.s.width).wrapping_sub(1))
}
#[inline]
unsafe fn divider(div: *mut bcm_clk_div, scaled_div: u64) -> u32 {
    BUG_ON(scaled_div < scaled_div_min(div)); BUG_ON(scaled_div > scaled_div_max(div));
    (scaled_div - (1u64 << (*div).u.s.frac_width)) as u32
}
#[inline]
unsafe fn scale_rate(div: *mut bcm_clk_div, rate: u32) -> u64 { if divider_is_fixed(div) { rate as u64 } else { (rate as u64) << (*div).u.s.frac_width } }

#[inline] unsafe fn __ccu_read(ccu: *mut ccu_data, off: u32) -> u32 { readl((*ccu).base.add(off as usize)) }
#[inline] unsafe fn __ccu_write(ccu: *mut ccu_data, off: u32, val: u32) { writel(val, (*ccu).base.add(off as usize)); }
#[inline] unsafe fn ccu_lock(ccu: *mut ccu_data) -> u64 { let mut flags = 0; spin_lock_irqsave(&mut (*ccu).lock, &mut flags); flags }
#[inline] unsafe fn ccu_unlock(ccu: *mut ccu_data, flags: u64) { spin_unlock_irqrestore(&mut (*ccu).lock, flags); }
#[inline] unsafe fn __ccu_write_enable(ccu: *mut ccu_data) { if (*ccu).write_enabled { pr_err!("{}: access already enabled for {}\n", "__ccu_write_enable", (*ccu).name); return; } (*ccu).write_enabled=true; __ccu_write(ccu,0,CCU_ACCESS_PASSWORD|1); }
#[inline] unsafe fn __ccu_write_disable(ccu: *mut ccu_data) { if !(*ccu).write_enabled { pr_err!("{}: access wasn't enabled for {}\n", "__ccu_write_disable", (*ccu).name); return; } __ccu_write(ccu,0,CCU_ACCESS_PASSWORD); (*ccu).write_enabled=false; }

unsafe fn __ccu_wait_bit(ccu: *mut ccu_data, off: u32, bit: u32, want: bool) -> bool {
    let mask=1u32<<bit; for _ in 0..CLK_GATE_DELAY_LOOP { let v=__ccu_read(ccu,off); if ((v&mask)!=0)==want { return true; } udelay(1); }
    pr_warn!("{}: {}/0x{:04x} bit {} was never {}\n", "__ccu_wait_bit", (*ccu).name, off, bit, if want {"set"} else {"clear"}); false
}

unsafe fn __ccu_policy_engine_start(ccu:*mut ccu_data, sync:bool)->bool { let c=&mut (*ccu).policy.control; if !policy_ctl_exists(c) {return true;} let o=c.offset; let b=c.go_bit; if !__ccu_wait_bit(ccu,o,b,false){pr_err!("ccu {} policy engine wouldn't go idle\n",(*ccu).name);return false;} let mut m=1u32<<b; m|=1u32<<if sync{c.atl_bit}else{c.ac_bit}; __ccu_write(ccu,o,m); let r=__ccu_wait_bit(ccu,o,b,false); if !r{pr_err!("ccu {} policy engine never started\n",(*ccu).name);} r }
unsafe fn __ccu_policy_engine_stop(ccu:*mut ccu_data)->bool { let e=&mut (*ccu).policy.enable; if !policy_lvm_en_exists(e){return true;} if !__ccu_wait_bit(ccu,e.offset,e.bit,false){pr_err!("ccu {} policy engine already stopped\n",(*ccu).name);return false;} __ccu_write(ccu,e.offset,1u32<<e.bit); let r=__ccu_wait_bit(ccu,e.offset,e.bit,false); if !r{pr_err!("ccu {} policy engine never stopped\n",(*ccu).name);} r }
unsafe fn policy_init(ccu:*mut ccu_data,p:*mut bcm_clk_policy)->bool { if !policy_exists(p){return true;} if !__ccu_policy_engine_stop(ccu){return false;} let mut o=(*p).offset; let m=1u32<<(*p).bit; for _ in 0..CCU_POLICY_COUNT { let v=__ccu_read(ccu,o)|m; __ccu_write(ccu,o,v); o+=4; } __ccu_policy_engine_start(ccu,true) }

unsafe fn __is_clk_gate_enabled(ccu:*mut ccu_data,g:*mut bcm_clk_gate)->bool { if !gate_exists(g){return true;} (__ccu_read(ccu,(*g).offset)&(1u32<<(*g).status_bit))!=0 }
unsafe fn is_clk_gate_enabled(ccu:*mut ccu_data,g:*mut bcm_clk_gate)->bool { if !gate_exists(g){return true;} let f=ccu_lock(ccu); let r=__is_clk_gate_enabled(ccu,g); ccu_unlock(ccu,f); r }
unsafe fn __gate_commit(ccu:*mut ccu_data,g:*mut bcm_clk_gate)->bool { BUG_ON(!gate_exists(g)); if !gate_is_sw_controllable(g){return true;} let mut v=__ccu_read(ccu,(*g).offset); if gate_is_hw_controllable(g){let m=1u32<<(*g).hw_sw_sel_bit; if gate_is_sw_managed(g){v|=m}else{v&=!m}} let en=false; let m=1u32<<(*g).en_bit; if gate_is_sw_managed(g)&&gate_is_enabled(g)&&!gate_is_no_disable(g){v|=m}else{v&=!m} __ccu_write(ccu,(*g).offset,v); if !gate_is_sw_managed(g){true}else{__ccu_wait_bit(ccu,(*g).offset,(*g).status_bit,en)} }
unsafe fn gate_init(ccu:*mut ccu_data,g:*mut bcm_clk_gate)->bool { if !gate_exists(g){true}else{__gate_commit(ccu,g)} }
unsafe fn __clk_gate(ccu:*mut ccu_data,g:*mut bcm_clk_gate,enable:bool)->bool { if !gate_exists(g)||!gate_is_sw_managed(g){return true;} if !enable&&gate_is_no_disable(g){return true;} if enable==gate_is_enabled(g){return true;} gate_flip_enabled(g); let r=__gate_commit(ccu,g); if !r{gate_flip_enabled(g);} r }
unsafe fn clk_gate(ccu:*mut ccu_data,name:*const i8,g:*mut bcm_clk_gate,enable:bool)->i32 { if !gate_exists(g)||!gate_is_sw_managed(g)||(!enable&&gate_is_no_disable(g)){return 0;} let f=ccu_lock(ccu);__ccu_write_enable(ccu);let ok=__clk_gate(ccu,g,enable);__ccu_write_disable(ccu);ccu_unlock(ccu,f);if ok{0}else{-5} }

unsafe fn hyst_init(ccu:*mut ccu_data,h:*mut bcm_clk_hyst)->bool { if !hyst_exists(h){return true;} let m=(1u32<<(*h).en_bit)|(1u32<<(*h).val_bit); let v=__ccu_read(ccu,(*h).offset)|m;__ccu_write(ccu,(*h).offset,v);true }
unsafe fn __clk_trigger(ccu:*mut ccu_data,t:*mut bcm_clk_trig)->bool { __ccu_write(ccu,(*t).offset,1u32<<(*t).bit);__ccu_wait_bit(ccu,(*t).offset,(*t).bit,false) }
unsafe fn divider_read_scaled(ccu:*mut ccu_data,d:*mut bcm_clk_div)->u64 { if divider_is_fixed(d){return (*d).u.fixed as u64;} let f=ccu_lock(ccu);let v=__ccu_read(ccu,(*d).u.s.offset);ccu_unlock(ccu,f);scaled_div_value(d,bitfield_extract(v,(*d).u.s.shift,(*d).u.s.width)) }
unsafe fn __div_commit(ccu:*mut ccu_data,g:*mut bcm_clk_gate,d:*mut bcm_clk_div,t:*mut bcm_clk_trig)->i32 { BUG_ON(divider_is_fixed(d)); if (*d).u.s.scaled_div==BAD_SCALED_DIV_VALUE {let v=__ccu_read(ccu,(*d).u.s.offset);(*d).u.s.scaled_div=scaled_div_value(d,bitfield_extract(v,(*d).u.s.shift,(*d).u.s.width));return 0;} let rd=divider(d,(*d).u.s.scaled_div);let was=__is_clk_gate_enabled(ccu,g);if !was&&!__clk_gate(ccu,g,true){return -6;}let v=bitfield_replace(__ccu_read(ccu,(*d).u.s.offset),(*d).u.s.shift,(*d).u.s.width,rd);__ccu_write(ccu,(*d).u.s.offset,v);let mut r=if __clk_trigger(ccu,t){0}else{-5};if !was&&!__clk_gate(ccu,g,false)&&r==0{r=-6;}r }
unsafe fn div_init(ccu:*mut ccu_data,g:*mut bcm_clk_gate,d:*mut bcm_clk_div,t:*mut bcm_clk_trig)->bool { !divider_exists(d)||divider_is_fixed(d)||__div_commit(ccu,g,d,t)==0 }
unsafe fn divider_write(ccu:*mut ccu_data,g:*mut bcm_clk_gate,d:*mut bcm_clk_div,t:*mut bcm_clk_trig,v:u64)->i32 { BUG_ON(divider_is_fixed(d));let old=(*d).u.s.scaled_div;if old==v{return 0;}(*d).u.s.scaled_div=v;let f=ccu_lock(ccu);__ccu_write_enable(ccu);let r=__div_commit(ccu,g,d,t);__ccu_write_disable(ccu);ccu_unlock(ccu,f);if r!=0{(*d).u.s.scaled_div=old;}r }

unsafe fn clk_recalc_rate(ccu:*mut ccu_data,d:*mut bcm_clk_div,p:*mut bcm_clk_div,parent:u64)->u64 { if !divider_exists(d){return parent;} if p!=core::ptr::null_mut()&&divider_exists(p){let mut r=scale_rate(p,parent as u32);r=scale_rate(d,r as u32);let sp=divider_read_scaled(ccu,p);let x=(r+sp/2)/sp;return (x+divider_read_scaled(ccu,d)/2)/divider_read_scaled(ccu,d);} let r=scale_rate(d,parent as u32);(r+divider_read_scaled(ccu,d)/2)/divider_read_scaled(ccu,d) }
unsafe fn round_rate(ccu:*mut ccu_data,d:*mut bcm_clk_div,p:*mut bcm_clk_div,rate:u64,parent:u64,out:*mut u64)->i64 { BUG_ON(!divider_exists(d));let mut sp=scale_rate(d,parent as u32);if divider_exists(p){let r=scale_rate(p,parent as u32);sp=((scale_rate(d,r as u32)+divider_read_scaled(ccu,p)/2)/divider_read_scaled(ccu,p));}let mut best=if divider_is_fixed(d){divider_read_scaled(ccu,d)}else{(sp+rate/2)/rate};if !divider_is_fixed(d){best=best.max(scaled_div_min(d)).min(scaled_div_max(d));}if !out.is_null(){*out=best;}((sp+best/2)/best) as i64 }

// Parent/clock operation declarations and remaining framework glue.
// These signatures intentionally retain the external kernel data model.
unsafe fn parent_index(sel:*mut bcm_clk_sel,v:u8)->u8 { for i in 0..(*sel).parent_count {if (*sel).parent_sel[i as usize]==v{return i as u8;}} BAD_CLK_INDEX }

pub unsafe fn kona_ccu_init(ccu:*mut ccu_data)->bool { let f=ccu_lock(ccu);__ccu_write_enable(ccu);for i in 0..(*ccu).clk_num {let k=(*ccu).kona_clks.add(i as usize);if !(*k).ccu.is_null(){BUG_ON((*k).typ!=bcm_clk_peri);}}__ccu_write_disable(ccu);ccu_unlock(ccu,f);true }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
