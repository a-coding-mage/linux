// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2020-2022 MaxLinear, Inc.
 * Copyright (C) 2020 Intel Corporation.
 * Zhu Yixin <yzhu@maxlinear.com>
 * Rahul Tanwar <rtanwar@maxlinear.com>
 */

const MAX_DDIV_REG: u32 = 8;
const MAX_DIVIDER_VAL: u32 = 64;

const fn gate_hw_reg_stat(reg: u32) -> u32 { reg + 0x0 }
const fn gate_hw_reg_en(reg: u32) -> u32 { reg + 0x4 }
const fn gate_hw_reg_dis(reg: u32) -> u32 { reg + 0x8 }

unsafe fn lgm_clk_register_fixed(ctx: *mut lgm_clk_provider, list: *const lgm_clk_branch) -> *mut clk_hw {
    if (*list).div_flags & CLOCK_FLAG_VAL_INIT != 0 {
        lgm_set_clk_val((*ctx).membase, (*list).div_off, (*list).div_shift, (*list).div_width, (*list).div_val);
    }
    clk_hw_register_fixed_rate(core::ptr::null_mut(), (*list).name, (*list).parent_data[0].name, (*list).flags, (*list).mux_flags)
}

unsafe fn lgm_clk_mux_get_parent(hw: *mut clk_hw) -> u8 {
    let mux = to_lgm_clk_mux(hw);
    let val = if (*mux).flags & MUX_CLK_SW != 0 { (*mux).reg } else { lgm_get_clk_val((*mux).membase, (*mux).reg, (*mux).shift, (*mux).width) };
    clk_mux_val_to_index(hw, core::ptr::null(), (*mux).flags, val)
}

unsafe fn lgm_clk_mux_set_parent(hw: *mut clk_hw, index: u8) -> i32 {
    let mux = to_lgm_clk_mux(hw);
    let val = clk_mux_index_to_val(core::ptr::null(), (*mux).flags, index);
    if (*mux).flags & MUX_CLK_SW != 0 { (*mux).reg = val; } else { lgm_set_clk_val((*mux).membase, (*mux).reg, (*mux).shift, (*mux).width, val); }
    0
}

unsafe fn lgm_clk_mux_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 { let mux = to_lgm_clk_mux(hw); clk_mux_determine_rate_flags(hw, req, (*mux).flags) }

static lgm_clk_mux_ops: clk_ops = clk_ops { get_parent: Some(lgm_clk_mux_get_parent), set_parent: Some(lgm_clk_mux_set_parent), determine_rate: Some(lgm_clk_mux_determine_rate), ..clk_ops::EMPTY };

unsafe fn lgm_clk_register_mux(ctx: *mut lgm_clk_provider, list: *const lgm_clk_branch) -> *mut clk_hw {
    let cflags = (*list).mux_flags; let dev = (*ctx).dev; let mut mux = devm_kzalloc(dev, core::mem::size_of::<lgm_clk_mux>(), GFP_KERNEL) as *mut lgm_clk_mux;
    if mux.is_null() { return ERR_PTR(-ENOMEM); }
    let mut init: clk_init_data = core::mem::zeroed(); init.name = (*list).name; init.ops = &lgm_clk_mux_ops; init.flags = (*list).flags; init.parent_data = (*list).parent_data; init.num_parents = (*list).num_parents;
    (*mux).membase = (*ctx).membase; (*mux).reg = (*list).mux_off; (*mux).shift = (*list).mux_shift; (*mux).width = (*list).mux_width; (*mux).flags = cflags; (*mux).hw.init = &mut init;
    let hw = &mut (*mux).hw; let ret = devm_clk_hw_register(dev, hw); if ret != 0 { return ERR_PTR(ret); }
    if cflags & CLOCK_FLAG_VAL_INIT != 0 { lgm_set_clk_val((*mux).membase, (*mux).reg, (*mux).shift, (*mux).width, (*list).mux_val); } hw
}

unsafe fn lgm_clk_divider_recalc_rate(hw: *mut clk_hw, parent_rate: u64) -> u64 { let d = to_lgm_clk_divider(hw); let val = lgm_get_clk_val((*d).membase, (*d).reg, (*d).shift, (*d).width); divider_recalc_rate(hw, parent_rate, val, (*d).table, (*d).flags, (*d).width) }
unsafe fn lgm_clk_divider_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 { let d=to_lgm_clk_divider(hw); divider_determine_rate(hw, req, (*d).table, (*d).width, (*d).flags) }
unsafe fn lgm_clk_divider_set_rate(hw: *mut clk_hw, rate: u64, prate: u64) -> i32 { let d=to_lgm_clk_divider(hw); let value=divider_get_val(rate,prate,(*d).table,(*d).width,(*d).flags); if value<0{return value;} lgm_set_clk_val((*d).membase,(*d).reg,(*d).shift,(*d).width,value as u32); 0 }
unsafe fn lgm_clk_divider_enable_disable(hw:*mut clk_hw, enable:i32)->i32 { let d=to_lgm_clk_divider(hw); if (*d).flags!=DIV_CLK_NO_MASK {lgm_set_clk_val((*d).membase,(*d).reg,(*d).shift_gate,(*d).width_gate,enable as u32);} 0 }
unsafe fn lgm_clk_divider_enable(hw:*mut clk_hw)->i32 {lgm_clk_divider_enable_disable(hw,1)}
unsafe fn lgm_clk_divider_disable(hw:*mut clk_hw){lgm_clk_divider_enable_disable(hw,0);}
static lgm_clk_divider_ops: clk_ops = clk_ops { recalc_rate: Some(lgm_clk_divider_recalc_rate), determine_rate: Some(lgm_clk_divider_determine_rate), set_rate: Some(lgm_clk_divider_set_rate), enable: Some(lgm_clk_divider_enable), disable: Some(lgm_clk_divider_disable), ..clk_ops::EMPTY };

unsafe fn lgm_clk_register_divider(ctx:*mut lgm_clk_provider,list:*const lgm_clk_branch)->*mut clk_hw { let d=devm_kzalloc((*ctx).dev,core::mem::size_of::<lgm_clk_divider>()) as *mut lgm_clk_divider; if d.is_null(){return ERR_PTR(-ENOMEM);} let mut init:clk_init_data=core::mem::zeroed(); init.name=(*list).name; init.ops=&lgm_clk_divider_ops; init.flags=(*list).flags; init.parent_data=(*list).parent_data; init.num_parents=1; (*d).membase=(*ctx).membase;(*d).reg=(*list).div_off;(*d).shift=(*list).div_shift;(*d).width=(*list).div_width;(*d).shift_gate=(*list).div_shift_gate;(*d).width_gate=(*list).div_width_gate;(*d).flags=(*list).div_flags;(*d).table=(*list).div_table;(*d).hw.init=&mut init; let hw=&mut (*d).hw;let ret=devm_clk_hw_register((*ctx).dev,hw);if ret!=0{return ERR_PTR(ret);}if (*list).div_flags&CLOCK_FLAG_VAL_INIT!=0{lgm_set_clk_val((*d).membase,(*d).reg,(*d).shift,(*d).width,(*list).div_val);}hw }

unsafe fn lgm_clk_register_fixed_factor(ctx:*mut lgm_clk_provider,list:*const lgm_clk_branch)->*mut clk_hw { let hw=clk_hw_register_fixed_factor((*ctx).dev,(*list).name,(*list).parent_data[0].name,(*list).flags,(*list).mult,(*list).div);if IS_ERR(hw){return ERR_CAST(hw);}if (*list).div_flags&CLOCK_FLAG_VAL_INIT!=0{lgm_set_clk_val((*ctx).membase,(*list).div_off,(*list).div_shift,(*list).div_width,(*list).div_val);}hw }

unsafe fn lgm_clk_gate_enable(hw:*mut clk_hw)->i32{let g=to_lgm_clk_gate(hw);lgm_set_clk_val((*g).membase,gate_hw_reg_en((*g).reg),(*g).shift,1,1);0}
unsafe fn lgm_clk_gate_disable(hw:*mut clk_hw){let g=to_lgm_clk_gate(hw);lgm_set_clk_val((*g).membase,gate_hw_reg_dis((*g).reg),(*g).shift,1,1);}
unsafe fn lgm_clk_gate_is_enabled(hw:*mut clk_hw)->i32{let g=to_lgm_clk_gate(hw);lgm_get_clk_val((*g).membase,gate_hw_reg_stat((*g).reg),(*g).shift,1) as i32}
static lgm_clk_gate_ops: clk_ops = clk_ops { enable: Some(lgm_clk_gate_enable), disable: Some(lgm_clk_gate_disable), is_enabled: Some(lgm_clk_gate_is_enabled), ..clk_ops::EMPTY };

unsafe fn lgm_clk_register_gate(ctx:*mut lgm_clk_provider,list:*const lgm_clk_branch)->*mut clk_hw { let g=devm_kzalloc((*ctx).dev,core::mem::size_of::<lgm_clk_gate>()) as *mut lgm_clk_gate;if g.is_null(){return ERR_PTR(-ENOMEM);}let mut init:clk_init_data=core::mem::zeroed();init.name=(*list).name;init.ops=&lgm_clk_gate_ops;init.flags=(*list).flags;init.parent_names=if (*list).parent_data[0].name.is_null(){core::ptr::null()}else{&(*list).parent_data[0].name};init.num_parents=if (*list).parent_data[0].name.is_null(){0}else{1};(*g).membase=(*ctx).membase;(*g).reg=(*list).gate_off;(*g).shift=(*list).gate_shift;(*g).flags=(*list).gate_flags;(*g).hw.init=&mut init;let hw=&mut (*g).hw;let ret=devm_clk_hw_register((*ctx).dev,hw);if ret!=0{return ERR_PTR(ret);}if (*list).gate_flags&CLOCK_FLAG_VAL_INIT!=0{lgm_set_clk_val((*g).membase,(*g).reg,(*g).shift,1,(*list).gate_val);}hw }

pub unsafe fn lgm_clk_register_branches(ctx:*mut lgm_clk_provider,mut list:*const lgm_clk_branch,nr_clk:u32)->i32 { for _ in 0..nr_clk { let hw=match (*list).type_ { CLK_TYPE_FIXED=>lgm_clk_register_fixed(ctx,list),CLK_TYPE_MUX=>lgm_clk_register_mux(ctx,list),CLK_TYPE_DIVIDER=>lgm_clk_register_divider(ctx,list),CLK_TYPE_FIXED_FACTOR=>lgm_clk_register_fixed_factor(ctx,list),CLK_TYPE_GATE=>if (*list).gate_flags&GATE_CLK_HW!=0{lgm_clk_register_gate(ctx,list)}else{core::ptr::null_mut()},_=>return -EINVAL};if IS_ERR(hw){return -EIO;}(*ctx).clk_data.hws[(*list).id as usize]=hw;list=list.add(1);}0 }

unsafe fn lgm_clk_ddiv_recalc_rate(hw:*mut clk_hw,parent_rate:u64)->u64{let d=to_lgm_clk_ddiv(hw);let a=lgm_get_clk_val((*d).membase,(*d).reg,(*d).shift0,(*d).width0)+1;let b=lgm_get_clk_val((*d).membase,(*d).reg,(*d).shift1,(*d).width1)+1;let e=lgm_get_clk_val((*d).membase,(*d).reg,(*d).shift2,(*d).width2);let mut r=parent_rate/a as u64/b as u64;if e!=0{r=r/(*d).div as u64*(*d).mult as u64;}r}
unsafe fn lgm_clk_ddiv_enable(hw:*mut clk_hw)->i32{let d=to_lgm_clk_ddiv(hw);lgm_set_clk_val((*d).membase,(*d).reg,(*d).shift_gate,(*d).width_gate,1);0}
unsafe fn lgm_clk_ddiv_disable(hw:*mut clk_hw){let d=to_lgm_clk_ddiv(hw);lgm_set_clk_val((*d).membase,(*d).reg,(*d).shift_gate,(*d).width_gate,0);}
unsafe fn lgm_clk_get_ddiv_val(mut div:u32,ddiv1:*mut u32,ddiv2:*mut u32)->i32{*ddiv1=1;*ddiv2=1;if div>MAX_DIVIDER_VAL{div=MAX_DIVIDER_VAL;}if div>1{let mut idx=2;while idx<=MAX_DDIV_REG{let temp=(div+idx-1)/idx;if div%idx==0&&temp<=MAX_DDIV_REG{*ddiv1=temp;*ddiv2=idx;return 0;}idx+=1;}return -EINVAL;}0}
unsafe fn lgm_clk_ddiv_set_rate(hw:*mut clk_hw,rate:u64,prate:u64)->i32{let d=to_lgm_clk_ddiv(hw);let mut div=(prate+rate/2)/rate;if lgm_get_clk_val((*d).membase,(*d).reg,(*d).shift2,1)!=0{div=((div/5)*2) as u64;}if div==0{return -EINVAL;}let(a,b)=(0u32,0u32);let mut x=a;let mut y=b;if lgm_clk_get_ddiv_val(div as u32,&mut x,&mut y)!=0{return -EINVAL;}lgm_set_clk_val((*d).membase,(*d).reg,(*d).shift0,(*d).width0,x-1);lgm_set_clk_val((*d).membase,(*d).reg,(*d).shift1,(*d).width1,y-1);0}
unsafe fn lgm_clk_ddiv_determine_rate(hw:*mut clk_hw,req:*mut clk_rate_request)->i32{let d=to_lgm_clk_ddiv(hw);let mut div=((*req).best_parent_rate+(*req).rate/2)/(*req).rate;if lgm_get_clk_val((*d).membase,(*d).reg,(*d).shift2,1)!=0{div=(div*2+2)/5;}if div==0{(*req).rate=(*req).best_parent_rate;return 0;}let mut a=0;let mut b=0;if lgm_clk_get_ddiv_val(div as u32,&mut a,&mut b)!=0&&lgm_clk_get_ddiv_val(div as u32+1,&mut a,&mut b)!=0{return -EINVAL;}let mut r=(*req).best_parent_rate/a as u64/b as u64;if lgm_get_clk_val((*d).membase,(*d).reg,(*d).shift2,1)!=0{r=(r*2+2)/5;}(*req).rate=r;0}
static lgm_clk_ddiv_ops: clk_ops = clk_ops { recalc_rate: Some(lgm_clk_ddiv_recalc_rate), enable: Some(lgm_clk_ddiv_enable), disable: Some(lgm_clk_ddiv_disable), set_rate: Some(lgm_clk_ddiv_set_rate), determine_rate: Some(lgm_clk_ddiv_determine_rate), ..clk_ops::EMPTY };

pub unsafe fn lgm_clk_register_ddiv(ctx:*mut lgm_clk_provider,list:*const lgm_clk_ddiv_data,nr_clk:u32)->i32{for _ in 0..nr_clk{let d=devm_kzalloc((*ctx).dev,core::mem::size_of::<lgm_clk_ddiv>()) as *mut lgm_clk_ddiv;if d.is_null(){return -ENOMEM;}(*d).membase=(*ctx).membase;(*d).reg=(*list).reg;(*d).shift0=(*list).shift0;(*d).width0=(*list).width0;(*d).shift1=(*list).shift1;(*d).width1=(*list).width1;(*d).shift_gate=(*list).shift_gate;(*d).width_gate=(*list).width_gate;(*d).shift2=(*list).ex_shift;(*d).width2=(*list).ex_width;(*d).flags=(*list).div_flags;(*d).mult=2;(*d).div=5;let mut init:clk_init_data=core::mem::zeroed();init.name=(*list).name;init.ops=&lgm_clk_ddiv_ops;init.flags=(*list).flags;init.parent_data=(*list).parent_data;init.num_parents=1;(*d).hw.init=&mut init;let ret=devm_clk_hw_register((*ctx).dev,&mut (*d).hw);if ret!=0{return ret;}(*ctx).clk_data.hws[(*list).id as usize]=&mut (*d).hw;list=list.add(1);}0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
