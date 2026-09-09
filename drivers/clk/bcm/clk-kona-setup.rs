// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2013 Broadcom Corporation
 * Copyright 2013 Linaro Limited
 */

// Dependencies supplied by the surrounding kernel translation.

macro_rules! selector_clear_exists { ($sel:expr) => { (*$sel).width = 0 }; }
macro_rules! trigger_clear_exists { ($trig:expr) => { FLAG_CLEAR!($trig, TRIG, EXISTS) }; }

unsafe fn ccu_data_offsets_valid(ccu: *mut ccu_data) -> bool {
    let ccu_policy = &mut (*ccu).policy;
    let mut limit = (*ccu).range - core::mem::size_of::<u32>() as u32;
    limit &= !(core::mem::size_of::<u32>() as u32 - 1);
    if ccu_policy_exists(ccu_policy) {
        if ccu_policy.enable.offset > limit { pr_err!("{}: bad policy enable offset for {} ({} > {})\n", "ccu_data_offsets_valid", (*ccu).name, ccu_policy.enable.offset, limit); return false; }
        if ccu_policy.control.offset > limit { pr_err!("{}: bad policy control offset for {} ({} > {})\n", "ccu_data_offsets_valid", (*ccu).name, ccu_policy.control.offset, limit); return false; }
    }
    true
}

unsafe fn clk_requires_trigger(bcm_clk: *mut kona_clk) -> bool {
    if (*bcm_clk).type_ != bcm_clk_peri { return false; }
    let peri = (*bcm_clk).u.peri;
    let sel = &(*peri).sel;
    if sel.parent_count != 0 && selector_exists(sel) { return true; }
    let div = &(*peri).div;
    if !divider_exists(div) { return false; }
    if !divider_is_fixed(div) { return true; }
    let pre = &(*peri).pre_div;
    divider_exists(pre) && !divider_is_fixed(pre)
}

unsafe fn peri_clk_data_offsets_valid(bcm_clk: *mut kona_clk) -> bool {
    BUG_ON!((*bcm_clk).type_ != bcm_clk_peri);
    let peri = (*bcm_clk).u.peri;
    let name = (*bcm_clk).init_data.name;
    let mut limit = (*(*bcm_clk).ccu).range - core::mem::size_of::<u32>() as u32;
    limit &= !(core::mem::size_of::<u32>() as u32 - 1);
    let policy = &(*peri).policy;
    if policy_exists(policy) && policy.offset > limit { pr_err!("{}: bad policy offset for {} ({} > {})\n", "peri_clk_data_offsets_valid", name, policy.offset, limit); return false; }
    let gate = &(*peri).gate; let hyst = &(*peri).hyst;
    if gate_exists(gate) {
        if gate.offset > limit { pr_err!("{}: bad gate offset for {} ({} > {})\n", "peri_clk_data_offsets_valid", name, gate.offset, limit); return false; }
        if hyst_exists(hyst) && hyst.offset > limit { pr_err!("{}: bad hysteresis offset for {} ({} > {})\n", "peri_clk_data_offsets_valid", name, hyst.offset, limit); return false; }
    } else if hyst_exists(hyst) { pr_err!("{}: hysteresis but no gate for {}\n", "peri_clk_data_offsets_valid", name); return false; }
    let div = &(*peri).div;
    if divider_exists(div) && div.u.s.offset > limit { pr_err!("{}: bad divider offset for {} ({} > {})\n", "peri_clk_data_offsets_valid", name, div.u.s.offset, limit); return false; }
    let div = &(*peri).pre_div;
    if divider_exists(div) && div.u.s.offset > limit { pr_err!("{}: bad pre-divider offset for {} ({} > {})\n", "peri_clk_data_offsets_valid", name, div.u.s.offset, limit); return false; }
    let sel = &(*peri).sel;
    if selector_exists(sel) && sel.offset > limit { pr_err!("{}: bad selector offset for {} ({} > {})\n", "peri_clk_data_offsets_valid", name, sel.offset, limit); return false; }
    let trig = &(*peri).trig;
    if trigger_exists(trig) && trig.offset > limit { pr_err!("{}: bad trigger offset for {} ({} > {})\n", "peri_clk_data_offsets_valid", name, trig.offset, limit); return false; }
    let trig = &(*peri).pre_trig;
    if trigger_exists(trig) && trig.offset > limit { pr_err!("{}: bad pre-trigger offset for {} ({} > {})\n", "peri_clk_data_offsets_valid", name, trig.offset, limit); return false; }
    true
}

unsafe fn bit_posn_valid(bit_posn: u32, field_name: *const i8, clock_name: *const i8) -> bool {
    let limit = 8 * core::mem::size_of::<u32>() as u32 - 1;
    if bit_posn > limit { pr_err!("{}: bad {} bit for {} ({} > {})\n", "bit_posn_valid", field_name, clock_name, bit_posn, limit); return false; }
    true
}

unsafe fn bitfield_valid(shift: u32, width: u32, field_name: *const i8, clock_name: *const i8) -> bool {
    let limit = 8 * core::mem::size_of::<u32>() as u32;
    if width == 0 { pr_err!("{}: bad {} field width 0 for {}\n", "bitfield_valid", field_name, clock_name); return false; }
    if shift + width > limit { pr_err!("{}: bad {} for {} ({} + {} > {})\n", "bitfield_valid", field_name, clock_name, shift, width, limit); return false; }
    true
}

unsafe fn ccu_policy_valid(p: *mut ccu_policy, name: *const i8) -> bool {
    let e = &(*p).enable; let c = &(*p).control;
    bit_posn_valid(e.bit, c"policy enable".as_ptr() as *const i8, name) &&
    bit_posn_valid(c.go_bit, c"policy control GO".as_ptr() as *const i8, name) &&
    bit_posn_valid(c.atl_bit, c"policy control ATL".as_ptr() as *const i8, name) &&
    bit_posn_valid(c.ac_bit, c"policy control AC".as_ptr() as *const i8, name)
}

unsafe fn policy_valid(p: *mut bcm_clk_policy, name: *const i8) -> bool { bit_posn_valid((*p).bit, c"policy".as_ptr() as *const i8, name) }

unsafe fn gate_valid(g: *mut bcm_clk_gate, _field: *const i8, name: *const i8) -> bool {
    if !bit_posn_valid((*g).status_bit, c"gate status".as_ptr() as *const i8, name) { return false; }
    if gate_is_sw_controllable(g) {
        if !bit_posn_valid((*g).en_bit, c"gate enable".as_ptr() as *const i8, name) { return false; }
        if gate_is_hw_controllable(g) && !bit_posn_valid((*g).hw_sw_sel_bit, c"gate hw/sw select".as_ptr() as *const i8, name) { return false; }
    } else { BUG_ON!(!gate_is_hw_controllable(g)); }
    true
}

unsafe fn hyst_valid(h: *mut bcm_clk_hyst, name: *const i8) -> bool {
    bit_posn_valid((*h).en_bit, c"hysteresis enable".as_ptr() as *const i8, name) && bit_posn_valid((*h).val_bit, c"hysteresis value".as_ptr() as *const i8, name)
}

unsafe fn sel_valid(s: *mut bcm_clk_sel, field: *const i8, name: *const i8) -> bool {
    if !bitfield_valid((*s).shift, (*s).width, field, name) { return false; }
    if (*s).parent_count != 0 {
        let max_sel = *(*s).parent_sel.add((*s).parent_count as usize - 1);
        let limit = (1u32 << (*s).width) - 1;
        if max_sel > limit { pr_err!("{}: bad selector for {} ({} needs > {} bits)\n", "sel_valid", name, max_sel, (*s).width); return false; }
    } else { pr_warn!("{}: ignoring selector for {} (no parents)\n", "sel_valid", name); selector_clear_exists!(s); kfree((*s).parent_sel); (*s).parent_sel = core::ptr::null_mut(); }
    true
}

unsafe fn div_valid(d: *mut bcm_clk_div, field: *const i8, name: *const i8) -> bool {
    if divider_is_fixed(d) { if (*d).u.fixed == 0 { pr_err!("{}: bad {} fixed value 0 for {}\n", "div_valid", field, name); return false; } return true; }
    if !bitfield_valid((*d).u.s.shift, (*d).u.s.width, field, name) { return false; }
    if divider_has_fraction(d) && (*d).u.s.frac_width > (*d).u.s.width { pr_warn!("{}: bad {} fraction width for {} ({} > {})\n", "div_valid", field, name, (*d).u.s.frac_width, (*d).u.s.width); return false; }
    true
}

unsafe fn kona_dividers_valid(c: *mut kona_clk) -> bool {
    BUG_ON!((*c).type_ != bcm_clk_peri); let p = (*c).u.peri;
    if !divider_exists(&(*p).div) || !divider_exists(&(*p).pre_div) { return true; }
    if divider_is_fixed(&(*p).div) || divider_is_fixed(&(*p).pre_div) { return true; }
    (*p).div.u.s.frac_width + (*p).pre_div.u.s.frac_width <= 8 * core::mem::size_of::<u32>() as u32
}

unsafe fn trig_valid(t: *mut bcm_clk_trig, field: *const i8, name: *const i8) -> bool { bit_posn_valid((*t).bit, field, name) }

unsafe fn peri_clk_data_valid(c: *mut kona_clk) -> bool {
    BUG_ON!((*c).type_ != bcm_clk_peri); if !peri_clk_data_offsets_valid(c) { return false; }
    let p = (*c).u.peri; let name = (*c).init_data.name;
    if policy_exists(&(*p).policy) && !policy_valid(&mut (*p).policy, name) { return false; }
    if gate_exists(&(*p).gate) && !gate_valid(&mut (*p).gate, c"gate".as_ptr() as *const i8, name) { return false; }
    if hyst_exists(&(*p).hyst) && !hyst_valid(&mut (*p).hyst, name) { return false; }
    let s = &mut (*p).sel;
    if selector_exists(s) { if !sel_valid(s, c"selector".as_ptr() as *const i8, name) { return false; } } else if s.parent_count > 1 { pr_err!("{}: multiple parents but no selector for {}\n", "peri_clk_data_valid", name); return false; }
    let d = &mut (*p).div; let pd = &mut (*p).pre_div;
    if divider_exists(d) { if !div_valid(d, c"divider".as_ptr() as *const i8, name) { return false; } if divider_exists(pd) && !div_valid(pd, c"pre-divider".as_ptr() as *const i8, name) { return false; } } else if divider_exists(pd) { pr_err!("{}: pre-divider but no divider for {}\n", "peri_clk_data_valid", name); return false; }
    let t = &mut (*p).trig;
    if trigger_exists(t) {
        if !trig_valid(t, c"trigger".as_ptr() as *const i8, name) { return false; }
        if trigger_exists(&(*p).pre_trig) && !trig_valid(t, c"pre-trigger".as_ptr() as *const i8, name) { return false; }
        if !clk_requires_trigger(c) { pr_warn!("{}: ignoring trigger for {} (not needed)\n", "peri_clk_data_valid", name); trigger_clear_exists!(t); }
    } else if trigger_exists(&(*p).pre_trig) { pr_err!("{}: pre-trigger but no trigger for {}\n", "peri_clk_data_valid", name); return false; } else if clk_requires_trigger(c) { pr_err!("{}: required trigger missing for {}\n", "peri_clk_data_valid", name); return false; }
    kona_dividers_valid(c)
}

unsafe fn kona_clk_valid(c: *mut kona_clk) -> bool { match (*c).type_ { bcm_clk_peri => peri_clk_data_valid(c), _ => { pr_err!("{}: unrecognized clock type ({})\n", "kona_clk_valid", (*c).type_ as i32); false } } }

unsafe fn parent_process(clocks: *const *const i8, count: *mut u32, names: *mut *mut *const i8) -> *mut u32 {
    *count = 0; *names = core::ptr::null_mut(); if clocks.is_null() { return core::ptr::null_mut(); }
    let mut orig = 0u32; let mut bad = 0u32; while !(*clocks.add(orig as usize)).is_null() { if *clocks.add(orig as usize) == BAD_CLK_NAME { bad += 1; } orig += 1; }
    let parent_count = orig - bad; if parent_count == 0 { return core::ptr::null_mut(); }
    if parent_count > PARENT_COUNT_MAX { pr_err!("{}: too many parents ({} > {})\n", "parent_process", parent_count, PARENT_COUNT_MAX); return ERR_PTR!(-EINVAL); }
    let pn = kmalloc_array(parent_count as usize, core::mem::size_of::<*const i8>(), GFP_KERNEL); if pn.is_null() { return ERR_PTR!(-ENOMEM); }
    let ps = kmalloc_array(parent_count as usize, core::mem::size_of::<u32>(), GFP_KERNEL); if ps.is_null() { kfree(pn); return ERR_PTR!(-ENOMEM); }
    let mut j = 0u32; for i in 0..orig { let v = *clocks.add(i as usize); if v != BAD_CLK_NAME { *(pn as *mut *const i8).add(j as usize) = v; *ps.add(j as usize) = i; j += 1; } }
    *names = pn as *mut *const i8; *count = parent_count; ps
}

unsafe fn clk_sel_setup(clocks: *const *const i8, sel: *mut bcm_clk_sel, init: *mut clk_init_data) -> i32 {
    let mut count = 0; let mut names = core::ptr::null_mut(); let ps = parent_process(clocks, &mut count, &mut names);
    if IS_ERR!(ps) { let ret = PTR_ERR!(ps); pr_err!("{}: error processing parent clocks for {} ({})\n", "clk_sel_setup", (*init).name, ret); return ret; }
    (*init).parent_names = names; (*init).num_parents = count; (*sel).parent_count = count; (*sel).parent_sel = ps; 0
}

unsafe fn clk_sel_teardown(sel: *mut bcm_clk_sel, init: *mut clk_init_data) { kfree((*sel).parent_sel); (*sel).parent_sel = core::ptr::null_mut(); (*sel).parent_count = 0; (*init).num_parents = 0; kfree((*init).parent_names); (*init).parent_names = core::ptr::null_mut(); }
unsafe fn peri_clk_teardown(data: *mut peri_clk_data, init: *mut clk_init_data) { clk_sel_teardown(&mut (*data).sel, init); }
unsafe fn peri_clk_setup(data: *mut peri_clk_data, init: *mut clk_init_data) -> i32 { (*init).flags = CLK_IGNORE_UNUSED; clk_sel_setup((*data).clocks, &mut (*data).sel, init) }
unsafe fn bcm_clk_teardown(c: *mut kona_clk) { if (*c).type_ == bcm_clk_peri { peri_clk_teardown((*c).u.data, &mut (*c).init_data); } (*c).u.data = core::ptr::null_mut(); (*c).type_ = bcm_clk_none; }
unsafe fn kona_clk_teardown(hw: *mut clk_hw) { if hw.is_null() { return; } clk_hw_unregister(hw); let c = to_kona_clk(hw); bcm_clk_teardown(c); }

unsafe fn kona_clk_setup(c: *mut kona_clk) -> i32 {
    let init = &mut (*c).init_data; let ret = match (*c).type_ { bcm_clk_peri => peri_clk_setup((*c).u.data, init), _ => { pr_err!("{}: clock type {} invalid for {}\n", "kona_clk_setup", (*c).type_ as i32, init.name); return -EINVAL } };
    if ret != 0 { return ret; } if !kona_clk_valid(c) { pr_err!("{}: clock data invalid for {}\n", "kona_clk_setup", init.name); bcm_clk_teardown(c); return -EINVAL; }
    (*c).hw.init = init; let ret = clk_hw_register(core::ptr::null_mut(), &mut (*c).hw); if ret != 0 { pr_err!("{}: error registering clock {} ({})\n", "kona_clk_setup", init.name, ret); bcm_clk_teardown(c); } ret
}
unsafe fn ccu_clks_teardown(c: *mut ccu_data) { for i in 0..(*c).clk_num { kona_clk_teardown(&mut (*c).kona_clks.add(i as usize).hw); } }
unsafe fn kona_ccu_teardown(c: *mut ccu_data) { if (*c).base.is_null() { return; } of_clk_del_provider((*c).node); ccu_clks_teardown(c); of_node_put((*c).node); (*c).node = core::ptr::null_mut(); iounmap((*c).base); (*c).base = core::ptr::null_mut(); }
unsafe fn ccu_data_valid(c: *mut ccu_data) -> bool { if !ccu_data_offsets_valid(c) { return false; } ccu_policy_exists(&(*c).policy) && !ccu_policy_valid(&mut (*c).policy, (*c).name) == false }
unsafe fn of_clk_kona_onecell_get(spec: *mut of_phandle_args, data: *mut core::ffi::c_void) -> *mut clk_hw { let c = data as *mut ccu_data; let idx = (*spec).args[0]; if idx >= (*c).clk_num { pr_err!("{}: invalid index {}\n", "of_clk_kona_onecell_get", idx); return ERR_PTR!(-EINVAL); } &mut (*c).kona_clks.add(idx as usize).hw }

pub unsafe fn kona_dt_ccu_setup(c: *mut ccu_data, node: *mut device_node) {
    let mut res = core::mem::zeroed::<resource>(); let mut ret = of_address_to_resource(node, 0, &mut res); if ret != 0 { pr_err!("{}: no valid CCU registers found for %pOFn\n", "kona_dt_ccu_setup", node); kona_ccu_teardown(c); return; }
    let range = resource_size(&res); if range > U32_MAX as resource_size_t { pr_err!("{}: address range too large for %pOFn\n", "kona_dt_ccu_setup", node); kona_ccu_teardown(c); return; } (*c).range = range as u32;
    if !ccu_data_valid(c) { pr_err!("{}: ccu data not valid for %pOFn\n", "kona_dt_ccu_setup", node); kona_ccu_teardown(c); return; }
    (*c).base = ioremap(res.start, (*c).range as usize); if (*c).base.is_null() { pr_err!("{}: unable to map CCU registers for %pOFn\n", "kona_dt_ccu_setup", node); kona_ccu_teardown(c); return; }
    (*c).node = of_node_get(node); for i in 0..(*c).clk_num { if !(*c).kona_clks.add(i as usize).ccu.is_null() { kona_clk_setup(&mut *(*c).kona_clks.add(i as usize)); } }
    ret = of_clk_add_hw_provider(node, of_clk_kona_onecell_get, c as *mut _); if ret != 0 { pr_err!("{}: error adding ccu %pOFn as provider ({})\n", "kona_dt_ccu_setup", node, ret); kona_ccu_teardown(c); return; }
    if !kona_ccu_init(c) { pr_err!("Broadcom %pOFn initialization had errors\n", node); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
