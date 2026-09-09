// SPDX-License-Identifier: GPL-2.0-only
/*
 * TI Divider Clock
 *
 * Copyright (C) 2013 Texas Instruments, Inc.
 *
 * Tero Kristo <t-kristo@ti.com>
 */

// C dependencies supplied by the surrounding kernel translation.

unsafe fn _get_table_div(table: *const clk_div_table, val: c_uint) -> c_uint {
    let mut clkt = table;
    unsafe {
        while (*clkt).div != 0 {
            if (*clkt).val == val { return (*clkt).div; }
            clkt = clkt.add(1);
        }
    }
    0
}

unsafe fn _setup_mask(divider: *mut clk_omap_divider) {
    let mut max_val: u32;
    unsafe {
        if !(*divider).table.is_null() {
            max_val = 0;
            let mut clkt = (*divider).table;
            while (*clkt).div != 0 {
                if (*clkt).val > max_val { max_val = (*clkt).val; }
                clkt = clkt.add(1);
            }
        } else {
            max_val = (*divider).max as u32;
            if ((*divider).flags & CLK_DIVIDER_ONE_BASED) == 0 &&
               ((*divider).flags & CLK_DIVIDER_POWER_OF_TWO) == 0 { max_val -= 1; }
        }
        let mask: u16 = if ((*divider).flags & CLK_DIVIDER_POWER_OF_TWO) != 0 {
            (fls(max_val) - 1) as u16
        } else { max_val as u16 };
        (*divider).mask = ((1u32 << fls(mask as u32)) - 1) as u16;
    }
}

unsafe fn _get_div(divider: *mut clk_omap_divider, val: c_uint) -> c_uint {
    unsafe {
        if ((*divider).flags & CLK_DIVIDER_ONE_BASED) != 0 { return val; }
        if ((*divider).flags & CLK_DIVIDER_POWER_OF_TWO) != 0 { return 1 << val; }
        if !(*divider).table.is_null() { return _get_table_div((*divider).table, val); }
        val + 1
    }
}

unsafe fn _get_table_val(table: *const clk_div_table, div: c_uint) -> c_uint {
    let mut clkt = table;
    unsafe {
        while (*clkt).div != 0 {
            if (*clkt).div == div { return (*clkt).val; }
            clkt = clkt.add(1);
        }
    }
    0
}

unsafe fn _get_val(divider: *mut clk_omap_divider, div: u8) -> c_uint {
    unsafe {
        if ((*divider).flags & CLK_DIVIDER_ONE_BASED) != 0 { return div as c_uint; }
        if ((*divider).flags & CLK_DIVIDER_POWER_OF_TWO) != 0 { return __ffs(div as c_uint); }
        if !(*divider).table.is_null() { return _get_table_val((*divider).table, div as c_uint); }
        (div - 1) as c_uint
    }
}

unsafe fn ti_clk_divider_recalc_rate(hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong {
    unsafe {
        let divider = to_clk_omap_divider(hw);
        let mut val = ((*ti_clk_ll_ops).clk_readl(&mut (*divider).reg) >> (*divider).shift) & (*divider).mask as u32;
        let div = _get_div(divider, val);
        if div == 0 {
            WARN(((*divider).flags & CLK_DIVIDER_ALLOW_ZERO) == 0, "%s: Zero divisor and CLK_DIVIDER_ALLOW_ZERO not set\n", clk_hw_get_name(hw));
            return parent_rate;
        }
        DIV_ROUND_UP(parent_rate, div as c_ulong)
    }
}

// The reverse of DIV_ROUND_UP: The maximum number which divided by m is r.
macro_rules! MULT_ROUND_UP { ($r:expr, $m:expr) => { ($r) * ($m) + ($m) - 1 }; }

unsafe fn _is_valid_table_div(table: *const clk_div_table, div: c_uint) -> bool {
    let mut clkt = table;
    unsafe { while (*clkt).div != 0 { if (*clkt).div == div { return true; } clkt = clkt.add(1); } }
    false
}

unsafe fn _is_valid_div(divider: *mut clk_omap_divider, div: c_uint) -> bool {
    unsafe {
        if ((*divider).flags & CLK_DIVIDER_POWER_OF_TWO) != 0 { return is_power_of_2(div); }
        if !(*divider).table.is_null() { return _is_valid_table_div((*divider).table, div); }
        true
    }
}

unsafe fn _div_round_up(table: *const clk_div_table, parent_rate: c_ulong, rate: c_ulong) -> c_int {
    let mut up = INT_MAX;
    let div = DIV_ROUND_UP_ULL(parent_rate as u64, rate as u64) as c_int;
    let mut clkt = table;
    unsafe {
        while (*clkt).div != 0 {
            if (*clkt).div as c_int == div { return (*clkt).div as c_int; }
            if (*clkt).div as c_int >= div && ((*clkt).div as c_int - div) < (up - div) { up = (*clkt).div as c_int; }
            clkt = clkt.add(1);
        }
    }
    up
}

unsafe fn _div_round(table: *const clk_div_table, parent_rate: c_ulong, rate: c_ulong) -> c_int {
    if table.is_null() { DIV_ROUND_UP(parent_rate, rate) as c_int } else { _div_round_up(table, parent_rate, rate) }
}

unsafe fn ti_clk_divider_bestdiv(hw: *mut clk_hw, mut rate: c_ulong, best_parent_rate: *mut c_ulong) -> c_int {
    unsafe {
        let divider = to_clk_omap_divider(hw);
        if rate == 0 { rate = 1; }
        let mut maxdiv = (*divider).max as c_ulong;
        if (clk_hw_get_flags(hw) & CLK_SET_RATE_PARENT) == 0 {
            let parent_rate = *best_parent_rate;
            let mut bestdiv = _div_round((*divider).table, parent_rate, rate);
            if bestdiv == 0 { bestdiv = 1; }
            if bestdiv > maxdiv as c_int { bestdiv = maxdiv as c_int; }
            return bestdiv;
        }
        maxdiv = core::cmp::min(c_ulong::MAX / rate, maxdiv);
        let saved = *best_parent_rate;
        let mut bestdiv = 0; let mut best = 0;
        for i in 1..=maxdiv {
            if !_is_valid_div(divider, i as c_uint) { continue; }
            if rate * i == saved { *best_parent_rate = saved; return i as c_int; }
            let parent_rate = clk_hw_round_rate(clk_hw_get_parent(hw), MULT_ROUND_UP!(rate, i));
            let now = DIV_ROUND_UP(parent_rate, i);
            if now <= rate && now > best { bestdiv = i as c_int; best = now; *best_parent_rate = parent_rate; }
        }
        if bestdiv == 0 { bestdiv = (*divider).max as c_int; *best_parent_rate = clk_hw_round_rate(clk_hw_get_parent(hw), 1); }
        bestdiv
    }
}

unsafe fn ti_clk_divider_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> c_int {
    unsafe { let div = ti_clk_divider_bestdiv(hw, (*req).rate, &mut (*req).best_parent_rate); (*req).rate = DIV_ROUND_UP((*req).best_parent_rate, div as c_ulong); 0 }
}

unsafe fn ti_clk_divider_set_rate(hw: *mut clk_hw, rate: c_ulong, parent_rate: c_ulong) -> c_int {
    if hw.is_null() || rate == 0 { return -EINVAL; }
    unsafe {
        let divider = to_clk_omap_divider(hw);
        let mut div = DIV_ROUND_UP(parent_rate, rate) as u32;
        if div > (*divider).max as u32 { div = (*divider).max as u32; }
        if div < (*divider).min as u32 { div = (*divider).min as u32; }
        let value = _get_val(divider, div as u8);
        let mut val = (*ti_clk_ll_ops).clk_readl(&mut (*divider).reg);
        val &= !((*divider).mask as u32 << (*divider).shift); val |= value << (*divider).shift;
        (*ti_clk_ll_ops).clk_writel(val, &mut (*divider).reg);
        ti_clk_latch(&mut (*divider).reg, (*divider).latch); 0
    }
}

unsafe fn clk_divider_save_context(hw: *mut clk_hw) -> c_int { unsafe { let d=to_clk_omap_divider(hw); (*d).context=((*ti_clk_ll_ops).clk_readl(&mut (*d).reg)>>(*d).shift)&(*d).mask as u32; 0 } }
unsafe fn clk_divider_restore_context(hw: *mut clk_hw) { unsafe { let d=to_clk_omap_divider(hw); let mut v=(*ti_clk_ll_ops).clk_readl(&mut (*d).reg); v &= !((*d).mask as u32<<(*d).shift); v |= (*d).context<<(*d).shift; (*ti_clk_ll_ops).clk_writel(v,&mut (*d).reg); } }

pub static ti_clk_divider_ops: clk_ops = clk_ops { recalc_rate: Some(ti_clk_divider_recalc_rate), determine_rate: Some(ti_clk_divider_determine_rate), set_rate: Some(ti_clk_divider_set_rate), save_context: Some(clk_divider_save_context), restore_context: Some(clk_divider_restore_context) };

unsafe fn _register_divider(node: *mut device_node, flags: u32, div: *mut clk_omap_divider) -> *mut clk {
    unsafe {
        let mut init: clk_init_data = core::mem::zeroed();
        let parent_name = of_clk_get_parent_name(node, 0); let name = ti_dt_clk_name(node);
        init.name=name; init.ops=&ti_clk_divider_ops; init.flags=flags;
        init.parent_names=if !parent_name.is_null() { &parent_name } else { core::ptr::null() };
        init.num_parents=if !parent_name.is_null(){1}else{0}; (*div).hw.init=&init;
        of_ti_clk_register(node, &mut (*div).hw, name)
    }
}

pub unsafe fn ti_clk_parse_divider_data(div_table: *mut c_int, mut num_dividers: c_int, mut max_div: c_int, _flags: u8, divider: *mut clk_omap_divider) -> c_int {
    unsafe {
        if div_table.is_null() { (*divider).min=1; (*divider).max=max_div as u16; _setup_mask(divider); return 0; }
        let mut i=0; let mut valid=0;
        while num_dividers==0 || i<num_dividers { let v=*div_table.add(i as usize); if v==-1{break;} if v!=0{valid+=1;} i+=1; }
        num_dividers=i; let tmp=kzalloc_objs::<clk_div_table>((valid+1) as usize); if tmp.is_null(){return -ENOMEM;}
        valid=0; let mut min_div=0;
        for j in 0..num_dividers { let v=*div_table.add(j as usize); if v>0 { (*tmp.add(valid as usize)).div=v as u32; (*tmp.add(valid as usize)).val=j as u32; valid+=1; if v>max_div{max_div=v;} if min_div==0||v<min_div{min_div=v;} } }
        (*divider).min=min_div as u16; (*divider).max=max_div as u16; (*divider).table=tmp; _setup_mask(divider); 0
    }
}

unsafe fn ti_clk_get_div_table(node:*mut device_node, div:*mut clk_omap_divider)->c_int { unsafe { let mut n=0u32; let p=of_get_property(node,"ti,dividers",&mut n); if p.is_null(){return 0;} let count=n/4; let mut valid=0; for i in 0..count { let mut v=0; of_property_read_u32_index(node,"ti,dividers",i,&mut v); if v!=0{valid+=1;} } if valid==0{return -EINVAL;} let t=kzalloc_objs::<clk_div_table>((valid+1)as usize); if t.is_null(){return -ENOMEM;} let mut k=0; for i in 0..count {let mut v=0;of_property_read_u32_index(node,"ti,dividers",i,&mut v);if v!=0{(*t.add(k as usize)).div=v;(*t.add(k as usize)).val=i;k+=1;}}(*div).table=t;0 } }

unsafe fn _populate_divider_min_max(node:*mut device_node, d:*mut clk_omap_divider)->c_int { unsafe { let(mut min,mut max)=(0u32,0u32); if (*d).table.is_null(){if of_property_read_u32(node,"ti,min-div",&mut min)!=0{min=1;}if of_property_read_u32(node,"ti,max-div",&mut max)!=0{return -EINVAL;}}else{let mut c=(*d).table;while(*c).div!=0{let v=(*c).div;if v>max{max=v;}if min==0||v<min{min=v;}c=c.add(1);}}(*d).min=min as u16;(*d).max=max as u16;_setup_mask(d);0} }

unsafe fn ti_clk_divider_populate(node:*mut device_node,d:*mut clk_omap_divider,flags:*mut u32)->c_int { unsafe { let r=ti_clk_get_reg_addr(node,0,&mut(*d).reg);if r!=0{return r;}(*d).shift=(*d).reg.bit;let mut v=0;if of_property_read_u32(node,"ti,latch-bit",&mut v)==0{(*d).latch=v as c_int}else{(*d).latch=-EINVAL;}*flags=0;(*d).flags=0;if of_property_read_bool(node,"ti,index-starts-at-one"){(*d).flags|=CLK_DIVIDER_ONE_BASED;}if of_property_read_bool(node,"ti,index-power-of-two"){(*d).flags|=CLK_DIVIDER_POWER_OF_TWO;}if of_property_read_bool(node,"ti,set-rate-parent"){*flags|=CLK_SET_RATE_PARENT;}let r=ti_clk_get_div_table(node,d);if r!=0{return r;}_populate_divider_min_max(node,d)} }

unsafe fn of_ti_divider_clk_setup(node:*mut device_node){unsafe{let d=kzalloc_obj::<clk_omap_divider>();if d.is_null(){return;}let mut f=0;if ti_clk_divider_populate(node,d,&mut f)!=0{ kfree((*d).table);kfree(d);return;}let c=_register_divider(node,f,d);if !IS_ERR(c){of_clk_add_provider(node,of_clk_src_simple_get,c);of_ti_clk_autoidle_setup(node);return;}kfree((*d).table);kfree(d);}}
unsafe fn of_ti_composite_divider_clk_setup(node:*mut device_node){unsafe{let d=kzalloc_obj::<clk_omap_divider>();if d.is_null(){return;}let mut f=0;if ti_clk_divider_populate(node,d,&mut f)!=0{kfree((*d).table);kfree(d);return;}if ti_clk_add_component(node,&mut(*d).hw,CLK_COMPONENT_TYPE_DIVIDER)==0{return;}kfree((*d).table);kfree(d);}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
