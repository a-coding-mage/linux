// SPDX-License-Identifier: GPL-2.0-or-later
/* Rust translation of clk.c; kernel dependencies are supplied externally. */

unsafe fn rockchip_clk_register_branch(name: *const i8, parents: *const *const i8, np: u8,
    base: *mut u8, mo: i32, ms: u8, mw: u8, mf: u8, mt: *mut u32,
    doff: i32, ds: u8, dw: u8, df: u8, dt: *mut clk_div_table,
    goff: i32, gs: u8, gf: u8, flags: c_ulong, lock: *mut spinlock_t) -> *mut clk {
    let mut mux: *mut clk_mux = core::ptr::null_mut();
    let mut gate: *mut clk_gate = core::ptr::null_mut();
    let mut div: *mut clk_divider = core::ptr::null_mut();
    let mut mux_ops: *const clk_ops = core::ptr::null();
    let mut div_ops: *const clk_ops = core::ptr::null();
    let mut gate_ops: *const clk_ops = core::ptr::null();
    let mut ret: i32;
    if np > 1 {
        mux = kzalloc_obj(); if mux.is_null() { return ERR_PTR(-ENOMEM); }
        (*mux).reg = base.add(mo as usize); (*mux).shift = ms;
        (*mux).mask = (1u32 << mw) - 1; (*mux).flags = mf; (*mux).table = mt; (*mux).lock = lock;
        mux_ops = if mf & CLK_MUX_READ_ONLY != 0 { &clk_mux_ro_ops } else { &clk_mux_ops };
    }
    if goff >= 0 {
        gate = kzalloc_obj(); if gate.is_null() { ret = -ENOMEM; if !mux.is_null(){kfree(mux)}; return ERR_PTR(ret); }
        (*gate).flags = gf; (*gate).reg = base.add(goff as usize); (*gate).bit_idx = gs; (*gate).lock = lock;
        gate_ops = &clk_gate_ops;
    }
    if dw > 0 {
        div = kzalloc_obj(); if div.is_null() { ret = -ENOMEM; if !gate.is_null(){kfree(gate)}; if !mux.is_null(){kfree(mux)}; return ERR_PTR(ret); }
        (*div).flags = df; (*div).reg = base.add(if doff != 0 { doff } else { mo } as usize);
        (*div).shift = ds; (*div).width = dw; (*div).lock = lock; (*div).table = dt;
        div_ops = if df & CLK_DIVIDER_READ_ONLY != 0 { &clk_divider_ro_ops } else { &clk_divider_ops };
    }
    let hw = clk_hw_register_composite(core::ptr::null_mut(), name, parents, np,
        if mux.is_null(){core::ptr::null_mut()}else{&mut (*mux).hw}, mux_ops,
        if div.is_null(){core::ptr::null_mut()}else{&mut (*div).hw}, div_ops,
        if gate.is_null(){core::ptr::null_mut()}else{&mut (*gate).hw}, gate_ops, flags);
    if IS_ERR(hw) { if !div.is_null(){kfree(div)}; if !gate.is_null(){kfree(gate)}; return ERR_CAST(hw); }
    (*hw).clk
}

#[repr(C)]
struct rockchip_clk_frac { clk_nb: notifier_block, div: clk_fractional_divider, gate: clk_gate,
    mux: clk_mux, mux_ops: *const clk_ops, mux_frac_idx: i32, rate_change_remuxed: bool, rate_change_idx: i32 }

unsafe fn rockchip_clk_frac_notifier_cb(nb: *mut notifier_block, event: c_ulong, data: *mut c_void) -> i32 {
    let n = &*(data as *mut clk_notifier_data); let f = container_of!(nb, rockchip_clk_frac, clk_nb);
    let m = &mut (*f).mux;
    pr_debug!("{}: event {}, old_rate {}, new_rate: {}\n", __func__, event, n.old_rate, n.new_rate);
    if event == PRE_RATE_CHANGE { (*f).rate_change_idx = ((*(*f).mux_ops).get_parent)(&mut m.hw);
        if (*f).rate_change_idx != (*f).mux_frac_idx { ((*(*f).mux_ops).set_parent)(&mut m.hw, (*f).mux_frac_idx); (*f).rate_change_remuxed=true; }
    } else if event == POST_RATE_CHANGE && (*f).rate_change_remuxed { ((*(*f).mux_ops).set_parent)(&mut m.hw, (*f).rate_change_idx); (*f).rate_change_remuxed=false; }
    notifier_from_errno(0)
}

unsafe fn rockchip_fractional_approximation(hw: *mut clk_hw, rate: c_ulong, parent_rate: *mut c_ulong, m: *mut c_ulong, n: *mut c_ulong) {
    let fd = to_clk_fd(hw); let p_rate = clk_hw_get_rate(clk_hw_get_parent(hw));
    if rate * 20 > p_rate && p_rate % rate != 0 { *parent_rate = clk_hw_get_rate(clk_hw_get_parent(clk_hw_get_parent(hw))); }
    (*fd).flags |= CLK_FRAC_DIVIDER_POWER_OF_TWO_PS;
    clk_fractional_divider_general_approximation(hw, rate, parent_rate, m, n);
}

unsafe fn rockchip_clk_register_frac_branch(ctx: *mut rockchip_clk_provider, name:*const i8, parents:*const *const i8, np:u8, base:*mut u8, mo:i32, df:u8, goff:i32, gs:u8, gf:u8, flags:c_ulong, child:*mut rockchip_clk_branch, lock:*mut spinlock_t)->*mut clk {
    if mo < 0 || (!child.is_null() && (*child).branch_type != branch_mux) { return ERR_PTR(-EINVAL); }
    let f: *mut rockchip_clk_frac = kzalloc_obj(); if f.is_null(){return ERR_PTR(-ENOMEM)};
    let gate = if goff >= 0 { &mut (*f).gate } else { core::ptr::null_mut() };
    if !gate.is_null(){(*gate).flags=gf;(*gate).reg=base.add(goff as usize);(*gate).bit_idx=gs;(*gate).lock=lock;}
    (*f).div.flags=df; (*f).div.reg=base.add(mo as usize); (*f).div.mshift=16; (*f).div.mwidth=16; (*f).div.nshift=0; (*f).div.nwidth=16; (*f).div.lock=lock; (*f).div.approximation=rockchip_fractional_approximation;
    let hw=clk_hw_register_composite(core::ptr::null_mut(),name,parents,np,core::ptr::null_mut(),core::ptr::null(),&mut (*f).div.hw,&clk_fractional_divider_ops,gate.map_or(core::ptr::null_mut(),|g|&mut (*g).hw),if gate.is_null(){core::ptr::null()}else{&clk_gate_ops},flags|CLK_SET_RATE_UNGATE);
    if IS_ERR(hw){kfree(f);return ERR_CAST(hw)}
    if !child.is_null(){ let mux=&mut (*f).mux; (*f).mux_frac_idx=match_string((*child).parent_names,(*child).num_parents,name); (*f).mux_ops=&clk_mux_ops; (*f).clk_nb.notifier_call=rockchip_clk_frac_notifier_cb; mux.reg=base.add((*child).muxdiv_offset as usize);mux.shift=(*child).mux_shift;mux.mask=(1u32<<(*child).mux_width)-1;mux.flags=(*child).mux_flags;mux.table=(*child).mux_table;mux.lock=lock; let mc=clk_register(core::ptr::null_mut(),mux); if IS_ERR(mc){kfree(f);return mc} rockchip_clk_set_lookup(ctx,mc,(*child).id); if (*f).mux_frac_idx>=0 { clk_notifier_register((*hw).clk,&mut (*f).clk_nb); } }
    (*hw).clk
}

unsafe fn rockchip_clk_register_factor_branch(name:*const i8, parents:*const *const i8,np:u8,base:*mut u8,mult:u32,divv:u32,goff:i32,gs:u8,gf:u8,flags:c_ulong,lock:*mut spinlock_t)->*mut clk {
    if goff==0{return clk_register_fixed_factor(core::ptr::null_mut(),name,*parents,flags,mult,divv)}
    let gate: *mut clk_gate=kzalloc_obj(); if gate.is_null(){return ERR_PTR(-ENOMEM)};(*gate).flags=gf;(*gate).reg=base.add(goff as usize);(*gate).bit_idx=gs;(*gate).lock=lock;
    let fix: *mut clk_fixed_factor=kzalloc_obj();if fix.is_null(){kfree(gate);return ERR_PTR(-ENOMEM)};(*fix).mult=mult;(*fix).div=divv;
    let hw=clk_hw_register_composite(core::ptr::null_mut(),name,parents,np,core::ptr::null_mut(),core::ptr::null(),&mut (*fix).hw,&clk_fixed_factor_ops,&mut (*gate).hw,&clk_gate_ops,flags);if IS_ERR(hw){kfree(fix);kfree(gate);return ERR_CAST(hw)}(*hw).clk
}

unsafe fn rockchip_clk_init_base(np:*mut device_node,base:*mut u8,nr:c_ulong,late:bool)->*mut rockchip_clk_provider { let c: *mut rockchip_clk_provider=kzalloc_obj();if c.is_null(){return ERR_PTR(-ENOMEM)};let table: *mut *mut clk=kzalloc_objs(nr);if table.is_null(){kfree(c);return ERR_PTR(-ENOMEM)};let d=ERR_PTR(if late{-EPROBE_DEFER}else{-ENOENT});for i in 0..nr as usize{*table.add(i)=d;}(*c).reg_base=base;(*c).clk_data.clks=table;(*c).clk_data.clk_num=nr;(*c).cru_node=np;spin_lock_init(&mut (*c).lock);hash_init!((*c).aux_grf_table);(*c).grf=syscon_regmap_lookup_by_phandle(np,b"rockchip,grf\0".as_ptr() as *const i8);c }

pub unsafe fn rockchip_clk_init(np:*mut device_node,base:*mut u8,nr:c_ulong)->*mut rockchip_clk_provider{rockchip_clk_init_base(np,base,nr,false)}
pub unsafe fn rockchip_clk_init_early(np:*mut device_node,base:*mut u8,nr:c_ulong)->*mut rockchip_clk_provider{rockchip_clk_init_base(np,base,nr,true)}
pub unsafe fn rockchip_clk_finalize(ctx:*mut rockchip_clk_provider){for i in 0..(*ctx).clk_data.clk_num as usize{if (*(*ctx).clk_data.clks.add(i))==ERR_PTR(-EPROBE_DEFER){*(*ctx).clk_data.clks.add(i)=ERR_PTR(-ENOENT)}}}
pub unsafe fn rockchip_clk_of_add_provider(np:*mut device_node,ctx:*mut rockchip_clk_provider){if of_clk_add_provider(np,of_clk_src_onecell_get,&mut (*ctx).clk_data)!=0{pr_err!("{}: could not register clk provider\n",__func__);}}
pub unsafe fn rockchip_clk_add_grf(ctx:*mut rockchip_clk_provider,grf:*mut regmap,ty:rockchip_grf_type)->i32{let a:*mut rockchip_aux_grf= kzalloc_obj();if a.is_null(){return -ENOMEM}(*a).grf=grf;(*a).type_=ty;hash_add!((*ctx).aux_grf_table,&mut (*a).node,ty);0}

pub unsafe fn rockchip_clk_register_plls(ctx:*mut rockchip_clk_provider,list:*mut rockchip_pll_clock,nr:u32,grf_lock:i32){for _ in 0..nr{let c=rockchip_clk_register_pll(ctx,(*list).type_,(*list).name,(*list).parent_names,(*list).num_parents,(*list).con_offset,grf_lock,(*list).lock_shift,(*list).mode_offset,(*list).mode_shift,(*list).rate_table,(*list).flags,(*list).pll_flags);if !IS_ERR(c){rockchip_clk_set_lookup(ctx,c,(*list).id)}else{pr_err!("{}: failed to register clock {}\n",__func__,(*list).name)}list=list.add(1)}}
pub unsafe fn rockchip_clk_find_max_clk_id(mut list:*mut rockchip_clk_branch,nr:u32)->c_ulong{let mut max=0;for _ in 0..nr{if (*list).id>max{max=(*list).id}if !(*list).child.is_null()&&(*(*list).child).id>max{max=(*(*list).child).id}list=list.add(1)}max}
pub unsafe fn rockchip_clk_register_armclk(ctx:*mut rockchip_clk_provider,id:u32,name:*const i8,parents:*const *const i8,np:u8,rd:*const rockchip_cpuclk_reg_data,rates:*const rockchip_cpuclk_rate_table,n:i32){let c=rockchip_clk_register_cpuclk(name,parents,np,rd,rates,n,(*ctx).reg_base,&mut (*ctx).lock);if !IS_ERR(c){rockchip_clk_set_lookup(ctx,c,id)}else{pr_err!("{}: failed to register clock {}\n",__func__,name)}}
pub unsafe fn rockchip_clk_register_armclk_multi_pll(ctx:*mut rockchip_clk_provider,list:*mut rockchip_clk_branch,rates:*const rockchip_cpuclk_rate_table,n:i32){let c=rockchip_clk_register_cpuclk_multi_pll((*list).name,(*list).parent_names,(*list).num_parents,(*ctx).reg_base,(*list).muxdiv_offset,(*list).mux_shift,(*list).mux_width,(*list).mux_flags,(*list).div_offset,(*list).div_shift,(*list).div_width,(*list).div_flags,(*list).flags,&mut (*ctx).lock,rates,n);if !IS_ERR(c){rockchip_clk_set_lookup(ctx,c,(*list).id)}}
pub unsafe fn rockchip_clk_register_branches(ctx:*mut rockchip_clk_provider,mut list:*mut rockchip_clk_branch,n:u32){for _ in 0..n{let l=&mut *list;let mut c:*mut clk=core::ptr::null_mut();match l.branch_type{branch_mux=>{c=if !l.mux_table.is_null(){clk_register_mux_table(core::ptr::null_mut(),l.name,l.parent_names,l.num_parents,l.flags,(*ctx).reg_base.add(l.muxdiv_offset as usize),l.mux_shift,l.mux_width,l.mux_flags,l.mux_table,&mut (*ctx).lock)}else{clk_register_mux(core::ptr::null_mut(),l.name,l.parent_names,l.num_parents,l.flags,(*ctx).reg_base.add(l.muxdiv_offset as usize),l.mux_shift,l.mux_width,l.mux_flags,&mut (*ctx).lock)}}branch_divider=>{c=clk_register_divider(core::ptr::null_mut(),l.name,*l.parent_names,l.flags,(*ctx).reg_base.add(l.muxdiv_offset as usize),l.div_shift,l.div_width,l.div_flags,&mut (*ctx).lock)}branch_fraction_divider=>c=rockchip_clk_register_frac_branch(ctx,l.name,l.parent_names,l.num_parents,(*ctx).reg_base,l.muxdiv_offset,l.div_flags,l.gate_offset,l.gate_shift,l.gate_flags,l.flags,l.child,&mut (*ctx).lock),branch_factor=>c=rockchip_clk_register_factor_branch(l.name,l.parent_names,l.num_parents,(*ctx).reg_base,l.div_shift,l.div_width,l.gate_offset,l.gate_shift,l.gate_flags,l.flags,&mut (*ctx).lock),branch_gate=>c=clk_register_gate(core::ptr::null_mut(),l.name,*l.parent_names,l.flags|CLK_SET_RATE_PARENT,(*ctx).reg_base.add(l.gate_offset as usize),l.gate_shift,l.gate_flags,&mut (*ctx).lock),branch_composite=>c=rockchip_clk_register_branch(l.name,l.parent_names,l.num_parents,(*ctx).reg_base,l.muxdiv_offset,l.mux_shift,l.mux_width,l.mux_flags,l.mux_table,l.div_offset,l.div_shift,l.div_width,l.div_flags,l.div_table,l.gate_offset,l.gate_shift,l.gate_flags,l.flags,&mut (*ctx).lock),branch_linked_gate=>{},_=>{pr_err!("{}: unknown clock type {}\n",__func__,l.branch_type)}}if !c.is_null()&&!IS_ERR(c){rockchip_clk_set_lookup(ctx,c,l.id)}list=list.add(1)}}
unsafe fn rockchip_clk_register_gate_link(dev:*mut device,ctx:*mut rockchip_clk_provider,br:*mut rockchip_clk_branch)->*mut platform_device{let mut p:platform_device_info=core::mem::zeroed();p.parent=dev;p.name=b"rockchip-gate-link-clk\0".as_ptr() as *const i8;p.id=(*br).id;p.fwnode=dev_fwnode(dev);p.of_node_reused=true;p.data=core::ptr::null_mut();p.size_data=0;platform_device_register_full(&p)}
pub unsafe fn rockchip_clk_register_late_branches(dev:*mut device,ctx:*mut rockchip_clk_provider,mut list:*mut rockchip_clk_branch,n:u32){for _ in 0..n{if (*list).branch_type==branch_linked_gate{rockchip_clk_register_gate_link(dev,ctx,list);}else{dev_err!(dev,"unknown clock type {}\n",(*list).branch_type)}list=list.add(1)}}
pub unsafe fn rockchip_clk_protect_critical(clocks:*const *const i8,n:i32){for i in 0..n as usize{clk_prepare_enable(__clk_lookup(*clocks.add(i)));}}

static mut rst_base:*mut u8=core::ptr::null_mut(); static mut reg_restart:u32=0; static mut cb_restart:Option<unsafe extern "C" fn()>=None;
unsafe fn rockchip_restart_notify(_: *mut notifier_block,_:c_ulong,_:*mut c_void)->i32{if let Some(cb)=cb_restart{cb()} writel(0xfdb9,rst_base.add(reg_restart as usize));NOTIFY_DONE}
static mut rockchip_restart_handler:notifier_block=notifier_block{notifier_call:rockchip_restart_notify,priority:128};
pub unsafe fn rockchip_register_restart_notifier(ctx:*mut rockchip_clk_provider,reg:u32,cb:Option<unsafe extern "C" fn()>){rst_base=(*ctx).reg_base;reg_restart=reg;cb_restart=cb;let r=register_restart_handler(&mut rockchip_restart_handler);if r!=0{pr_err!("{}: cannot register restart handler, {}\n",__func__,r)}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
