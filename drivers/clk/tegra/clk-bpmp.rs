// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2016-2022 NVIDIA Corporation
 */

// Linux kernel and Tegra BPMP dependencies are supplied externally.

const TEGRA_BPMP_DUMP_CLOCK_INFO: u32 = 0;
const TEGRA_BPMP_CLK_HAS_MUX: u32 = 1 << 0;
const TEGRA_BPMP_CLK_HAS_SET_RATE: u32 = 1 << 1;
const TEGRA_BPMP_CLK_IS_ROOT: u32 = 1 << 2;

#[repr(C)]
struct tegra_bpmp_clk_info {
    id: u32,
    name: [::core::ffi::c_char; MRQ_CLK_NAME_MAXLEN],
    parents: [u32; MRQ_CLK_MAX_PARENTS],
    num_parents: u32,
    flags: ::core::ffi::c_ulong,
}

#[repr(C)]
struct tegra_bpmp_clk {
    hw: clk_hw,
    bpmp: *mut tegra_bpmp,
    id: u32,
    num_parents: u32,
    parents: *mut u32,
}

#[inline]
unsafe fn to_tegra_bpmp_clk(hw: *mut clk_hw) -> *mut tegra_bpmp_clk {
    container_of!(hw, tegra_bpmp_clk, hw)
}

#[repr(C)]
struct tegra_bpmp_clk_message {
    cmd: u32,
    id: u32,
    tx: tegra_bpmp_clk_message_tx,
    rx: tegra_bpmp_clk_message_rx,
}

#[repr(C)]
struct tegra_bpmp_clk_message_tx {
    data: *const ::core::ffi::c_void,
    size: usize,
}

#[repr(C)]
struct tegra_bpmp_clk_message_rx {
    data: *mut ::core::ffi::c_void,
    size: usize,
    ret: i32,
}

unsafe fn tegra_bpmp_clk_transfer(
    bpmp: *mut tegra_bpmp,
    clk: *const tegra_bpmp_clk_message,
) -> i32 {
    let mut request: mrq_clk_request = ::core::mem::zeroed();
    let mut msg: tegra_bpmp_message = ::core::mem::zeroed();
    request.cmd_and_id = ((*clk).cmd << 24) | (*clk).id;
    ::core::ptr::copy_nonoverlapping(
        (*clk).tx.data as *const u8,
        (&mut request as *mut _ as *mut u8).add(4),
        (*clk).tx.size,
    );
    msg.mrq = MRQ_CLK;
    msg.tx.data = &mut request as *mut _ as *mut ::core::ffi::c_void;
    msg.tx.size = ::core::mem::size_of::<mrq_clk_request>();
    msg.rx.data = (*clk).rx.data;
    msg.rx.size = (*clk).rx.size;
    let err = tegra_bpmp_transfer(bpmp, &mut msg);
    if err < 0 { return err; }
    if msg.rx.ret < 0 { return -EINVAL; }
    0
}

unsafe fn tegra_bpmp_clk_prepare(hw: *mut clk_hw) -> i32 {
    let clk = to_tegra_bpmp_clk(hw);
    let mut msg: tegra_bpmp_clk_message = ::core::mem::zeroed();
    msg.cmd = CMD_CLK_ENABLE;
    msg.id = (*clk).id;
    tegra_bpmp_clk_transfer((*clk).bpmp, &msg)
}

unsafe fn tegra_bpmp_clk_unprepare(hw: *mut clk_hw) {
    let clk = to_tegra_bpmp_clk(hw);
    let mut msg: tegra_bpmp_clk_message = ::core::mem::zeroed();
    msg.cmd = CMD_CLK_DISABLE;
    msg.id = (*clk).id;
    let err = tegra_bpmp_clk_transfer((*clk).bpmp, &msg);
    if err < 0 {
        dev_err!((*clk).bpmp, "failed to disable clock %s: %d\n", clk_hw_get_name(hw), err);
    }
}

unsafe fn tegra_bpmp_clk_is_prepared(hw: *mut clk_hw) -> i32 {
    let clk = to_tegra_bpmp_clk(hw);
    let mut response: cmd_clk_is_enabled_response = ::core::mem::zeroed();
    let mut msg: tegra_bpmp_clk_message = ::core::mem::zeroed();
    msg.cmd = CMD_CLK_IS_ENABLED;
    msg.id = (*clk).id;
    msg.rx.data = &mut response as *mut _ as *mut _;
    msg.rx.size = ::core::mem::size_of_val(&response);
    let err = tegra_bpmp_clk_transfer((*clk).bpmp, &msg);
    if err < 0 { return err; }
    response.state
}

unsafe fn tegra_bpmp_clk_recalc_rate(hw: *mut clk_hw, _parent_rate: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    let clk = to_tegra_bpmp_clk(hw);
    let mut response: cmd_clk_get_rate_response = ::core::mem::zeroed();
    let request: cmd_clk_get_rate_request = ::core::mem::zeroed();
    let mut msg: tegra_bpmp_clk_message = ::core::mem::zeroed();
    msg.cmd = CMD_CLK_GET_RATE;
    msg.id = (*clk).id;
    msg.tx.data = &request as *const _ as *const _;
    msg.tx.size = ::core::mem::size_of_val(&request);
    msg.rx.data = &mut response as *mut _ as *mut _;
    msg.rx.size = ::core::mem::size_of_val(&response);
    if tegra_bpmp_clk_transfer((*clk).bpmp, &msg) < 0 { return 0; }
    response.rate
}

unsafe fn tegra_bpmp_clk_determine_rate(hw: *mut clk_hw, rate_req: *mut clk_rate_request) -> i32 {
    let clk = to_tegra_bpmp_clk(hw);
    let mut response: cmd_clk_round_rate_response = ::core::mem::zeroed();
    let mut request: cmd_clk_round_rate_request = ::core::mem::zeroed();
    let mut msg: tegra_bpmp_clk_message = ::core::mem::zeroed();
    let rate = (*rate_req).rate.clamp((*rate_req).min_rate, (*rate_req).max_rate);
    request.rate = ::core::cmp::min(rate as u64, S64_MAX as u64);
    msg.cmd = CMD_CLK_ROUND_RATE;
    msg.id = (*clk).id;
    msg.tx.data = &request as *const _ as *const _;
    msg.tx.size = ::core::mem::size_of_val(&request);
    msg.rx.data = &mut response as *mut _ as *mut _;
    msg.rx.size = ::core::mem::size_of_val(&response);
    let err = tegra_bpmp_clk_transfer((*clk).bpmp, &msg);
    if err < 0 { return err; }
    (*rate_req).rate = response.rate as _;
    0
}

unsafe fn tegra_bpmp_clk_set_parent(hw: *mut clk_hw, index: u8) -> i32 {
    let clk = to_tegra_bpmp_clk(hw);
    let mut response: cmd_clk_set_parent_response = ::core::mem::zeroed();
    let mut request: cmd_clk_set_parent_request = ::core::mem::zeroed();
    let mut msg: tegra_bpmp_clk_message = ::core::mem::zeroed();
    request.parent_id = *(*clk).parents.add(index as usize);
    msg.cmd = CMD_CLK_SET_PARENT;
    msg.id = (*clk).id;
    msg.tx.data = &request as *const _ as *const _;
    msg.tx.size = ::core::mem::size_of_val(&request);
    msg.rx.data = &mut response as *mut _ as *mut _;
    msg.rx.size = ::core::mem::size_of_val(&response);
    tegra_bpmp_clk_transfer((*clk).bpmp, &msg)
}

unsafe fn tegra_bpmp_clk_get_parent(hw: *mut clk_hw) -> u8 {
    let clk = to_tegra_bpmp_clk(hw);
    let mut response: cmd_clk_get_parent_response = ::core::mem::zeroed();
    let mut msg: tegra_bpmp_clk_message = ::core::mem::zeroed();
    msg.cmd = CMD_CLK_GET_PARENT;
    msg.id = (*clk).id;
    msg.rx.data = &mut response as *mut _ as *mut _;
    msg.rx.size = ::core::mem::size_of_val(&response);
    if tegra_bpmp_clk_transfer((*clk).bpmp, &msg) < 0 {
        dev_err!((*clk).bpmp, "failed to get parent for %s\n", clk_hw_get_name(hw));
        return u8::MAX;
    }
    for i in 0..(*clk).num_parents as usize {
        if *(*clk).parents.add(i) == response.parent_id { return i as u8; }
    }
    u8::MAX
}

unsafe fn tegra_bpmp_clk_set_rate(hw: *mut clk_hw, rate: ::core::ffi::c_ulong, _parent_rate: ::core::ffi::c_ulong) -> i32 {
    let clk = to_tegra_bpmp_clk(hw);
    let mut response: cmd_clk_set_rate_response = ::core::mem::zeroed();
    let mut request: cmd_clk_set_rate_request = ::core::mem::zeroed();
    let mut msg: tegra_bpmp_clk_message = ::core::mem::zeroed();
    request.rate = ::core::cmp::min(rate as u64, S64_MAX as u64);
    msg.cmd = CMD_CLK_SET_RATE;
    msg.id = (*clk).id;
    msg.tx.data = &request as *const _ as *const _;
    msg.tx.size = ::core::mem::size_of_val(&request);
    msg.rx.data = &mut response as *mut _ as *mut _;
    msg.rx.size = ::core::mem::size_of_val(&response);
    tegra_bpmp_clk_transfer((*clk).bpmp, &msg)
}

const tegra_bpmp_clk_gate_ops: clk_ops = clk_ops { prepare: Some(tegra_bpmp_clk_prepare), unprepare: Some(tegra_bpmp_clk_unprepare), is_prepared: Some(tegra_bpmp_clk_is_prepared), recalc_rate: Some(tegra_bpmp_clk_recalc_rate), ..clk_ops::ZERO };
const tegra_bpmp_clk_mux_ops: clk_ops = clk_ops { prepare: Some(tegra_bpmp_clk_prepare), unprepare: Some(tegra_bpmp_clk_unprepare), is_prepared: Some(tegra_bpmp_clk_is_prepared), recalc_rate: Some(tegra_bpmp_clk_recalc_rate), determine_rate: Some(clk_hw_determine_rate_no_reparent), set_parent: Some(tegra_bpmp_clk_set_parent), get_parent: Some(tegra_bpmp_clk_get_parent), ..clk_ops::ZERO };
const tegra_bpmp_clk_rate_ops: clk_ops = clk_ops { prepare: Some(tegra_bpmp_clk_prepare), unprepare: Some(tegra_bpmp_clk_unprepare), is_prepared: Some(tegra_bpmp_clk_is_prepared), recalc_rate: Some(tegra_bpmp_clk_recalc_rate), determine_rate: Some(tegra_bpmp_clk_determine_rate), set_rate: Some(tegra_bpmp_clk_set_rate), ..clk_ops::ZERO };
const tegra_bpmp_clk_mux_rate_ops: clk_ops = clk_ops { prepare: Some(tegra_bpmp_clk_prepare), unprepare: Some(tegra_bpmp_clk_unprepare), is_prepared: Some(tegra_bpmp_clk_is_prepared), recalc_rate: Some(tegra_bpmp_clk_recalc_rate), determine_rate: Some(tegra_bpmp_clk_determine_rate), set_parent: Some(tegra_bpmp_clk_set_parent), get_parent: Some(tegra_bpmp_clk_get_parent), set_rate: Some(tegra_bpmp_clk_set_rate), ..clk_ops::ZERO };
const tegra_bpmp_clk_mux_read_only_ops: clk_ops = clk_ops { get_parent: Some(tegra_bpmp_clk_get_parent), recalc_rate: Some(tegra_bpmp_clk_recalc_rate), ..clk_ops::ZERO };
const tegra_bpmp_clk_read_only_ops: clk_ops = clk_ops { recalc_rate: Some(tegra_bpmp_clk_recalc_rate), ..clk_ops::ZERO };
const tegra_bpmp_clk_gate_mux_read_only_ops: clk_ops = clk_ops { prepare: Some(tegra_bpmp_clk_prepare), unprepare: Some(tegra_bpmp_clk_unprepare), is_prepared: Some(tegra_bpmp_clk_is_prepared), recalc_rate: Some(tegra_bpmp_clk_recalc_rate), get_parent: Some(tegra_bpmp_clk_get_parent), ..clk_ops::ZERO };

unsafe fn tegra_bpmp_clk_get_max_id(bpmp: *mut tegra_bpmp) -> i32 {
    let mut response: cmd_clk_get_max_clk_id_response = ::core::mem::zeroed();
    let mut msg: tegra_bpmp_clk_message = ::core::mem::zeroed();
    msg.cmd = CMD_CLK_GET_MAX_CLK_ID;
    msg.rx.data = &mut response as *mut _ as *mut _;
    msg.rx.size = ::core::mem::size_of_val(&response);
    let err = tegra_bpmp_clk_transfer(bpmp, &msg);
    if err < 0 { return err; }
    if response.max_id > i32::MAX as u32 { return -E2BIG; }
    response.max_id as i32
}

unsafe fn tegra_bpmp_clk_get_info(bpmp: *mut tegra_bpmp, id: u32, info: *mut tegra_bpmp_clk_info) -> i32 {
    let mut response: cmd_clk_get_all_info_response = ::core::mem::zeroed();
    let mut msg: tegra_bpmp_clk_message = ::core::mem::zeroed();
    msg.cmd = CMD_CLK_GET_ALL_INFO; msg.id = id;
    msg.rx.data = &mut response as *mut _ as *mut _;
    msg.rx.size = ::core::mem::size_of_val(&response);
    let err = tegra_bpmp_clk_transfer(bpmp, &msg); if err < 0 { return err; }
    if dev_to_node((*bpmp).dev) == NUMA_NO_NODE {
        strscpy!((*info).name.as_mut_ptr(), response.name.as_ptr(), (*info).name.len());
    } else {
        let n = snprintf!((*info).name.as_mut_ptr(), (*info).name.len(), "{}-{}", dev_to_node((*bpmp).dev), response.name);
        if n >= (*info).name.len() as i32 { return -E2BIG; }
    }
    (*info).num_parents = response.num_parents;
    for i in 0..(*info).num_parents as usize { (*info).parents[i] = response.parents[i]; }
    (*info).flags = response.flags;
    0
}

unsafe fn tegra_bpmp_clk_info_dump(bpmp: *mut tegra_bpmp, level: *const ::core::ffi::c_char, info: *const tegra_bpmp_clk_info) {
    let mut flags = [0 as ::core::ffi::c_char; 64];
    let mut prefix = "";
    if (*info).flags != 0 { seq_buf_printf!(&mut flags, "("); }
    if (*info).flags & TEGRA_BPMP_CLK_HAS_MUX as _ != 0 { seq_buf_printf!(&mut flags, "{}mux", prefix); prefix = ", "; }
    if (*info).flags & TEGRA_BPMP_CLK_HAS_SET_RATE as _ == 0 { seq_buf_printf!(&mut flags, "{}fixed", prefix); prefix = ", "; }
    if (*info).flags & TEGRA_BPMP_CLK_IS_ROOT as _ != 0 { seq_buf_printf!(&mut flags, "{}root", prefix); prefix = ", "; }
    if (*info).flags != 0 { seq_buf_printf!(&mut flags, ")"); }
    dev_printk!(level, (*bpmp).dev, "{:03}: {}\n", (*info).id, (*info).name);
    dev_printk!(level, (*bpmp).dev, "  flags: {:x} {}\n", (*info).flags, flags);
    dev_printk!(level, (*bpmp).dev, "  parents: {}\n", (*info).num_parents);
    for i in 0..(*info).num_parents as usize { dev_printk!(level, (*bpmp).dev, "    {:03}\n", (*info).parents[i]); }
}

unsafe fn tegra_bpmp_probe_clocks(bpmp: *mut tegra_bpmp, clocksp: *mut *mut tegra_bpmp_clk_info) -> i32 {
    let err = tegra_bpmp_clk_get_max_id(bpmp); if err < 0 { return err; }
    let max_id = err as u32;
    dev_dbg!((*bpmp).dev, "maximum clock ID: {}\n", max_id);
    let clocks = kzalloc_objs!(tegra_bpmp_clk_info, max_id as usize + 1);
    if clocks.is_null() { return -ENOMEM; }
    let mut count = 0u32; let mut holes = 0u32;
    for id in 0..=max_id {
        let info = clocks.add(count as usize);
        if tegra_bpmp_clk_get_info(bpmp, id, info) < 0 { continue; }
        if (*info).num_parents >= u8::MAX as u32 { dev_err!((*bpmp).dev, "clock {} has too many parents ({}, max: {})\n", id, (*info).num_parents, u8::MAX); continue; }
        if (*info).name[0] == 0 { holes += 1; continue; }
        (*info).id = id; count += 1;
        if TEGRA_BPMP_DUMP_CLOCK_INFO != 0 { tegra_bpmp_clk_info_dump(bpmp, KERN_DEBUG, info); }
    }
    dev_dbg!((*bpmp).dev, "holes: {}\n", holes); *clocksp = clocks; count as i32
}

unsafe fn tegra_bpmp_clk_id_to_index(clocks: *const tegra_bpmp_clk_info, num_clocks: u32, id: u32) -> u32 {
    for i in 0..num_clocks { if (*clocks.add(i as usize)).id == id { return i; } } u32::MAX
}

unsafe fn tegra_bpmp_clk_find(clocks: *const tegra_bpmp_clk_info, num_clocks: u32, id: u32) -> *const tegra_bpmp_clk_info {
    let i = tegra_bpmp_clk_id_to_index(clocks, num_clocks, id); if i < num_clocks { clocks.add(i as usize) } else { ::core::ptr::null() }
}

unsafe fn tegra_bpmp_clk_register(bpmp: *mut tegra_bpmp, info: *const tegra_bpmp_clk_info, clocks: *const tegra_bpmp_clk_info, num_clocks: u32) -> *mut tegra_bpmp_clk {
    let clk = devm_kzalloc!( (*bpmp).dev, ::core::mem::size_of::<tegra_bpmp_clk>(), GFP_KERNEL) as *mut tegra_bpmp_clk;
    if clk.is_null() { return ERR_PTR(-ENOMEM); }
    (*clk).id = (*info).id; (*clk).bpmp = bpmp;
    (*clk).parents = devm_kcalloc!((*bpmp).dev, (*info).num_parents as usize, ::core::mem::size_of::<u32>(), GFP_KERNEL) as *mut u32;
    if (*clk).parents.is_null() { return ERR_PTR(-ENOMEM); }
    (*clk).num_parents = (*info).num_parents;
    let mut init: clk_init_data = ::core::mem::zeroed(); init.name = (*info).name.as_ptr(); (*clk).hw.init = &mut init;
    if (*info).flags & BPMP_CLK_STATE_CHANGE_DENIED as _ != 0 {
        if (*info).flags & BPMP_CLK_RATE_PARENT_CHANGE_DENIED as _ == 0 { dev_warn!((*bpmp).dev, "Firmware bug! Inconsistent permission bits for clock %s. State and parent/rate changes disabled.\n", init.name); }
        init.ops = if (*info).flags & TEGRA_BPMP_CLK_HAS_MUX as _ != 0 { &tegra_bpmp_clk_mux_read_only_ops } else { &tegra_bpmp_clk_read_only_ops };
    } else if (*info).flags & BPMP_CLK_RATE_PARENT_CHANGE_DENIED as _ != 0 {
        init.ops = if (*info).flags & TEGRA_BPMP_CLK_HAS_MUX as _ != 0 { &tegra_bpmp_clk_gate_mux_read_only_ops } else { &tegra_bpmp_clk_gate_ops };
    } else if (*info).flags & TEGRA_BPMP_CLK_HAS_MUX as _ != 0 {
        init.ops = if (*info).flags & TEGRA_BPMP_CLK_HAS_SET_RATE as _ != 0 { &tegra_bpmp_clk_mux_rate_ops } else { &tegra_bpmp_clk_mux_ops };
    } else { init.ops = if (*info).flags & TEGRA_BPMP_CLK_HAS_SET_RATE as _ != 0 { &tegra_bpmp_clk_rate_ops } else { &tegra_bpmp_clk_gate_ops }; }
    init.num_parents = (*info).num_parents;
    let parents = kcalloc!((*info).num_parents as usize, ::core::mem::size_of::<*const ::core::ffi::c_char>(), GFP_KERNEL) as *mut *const ::core::ffi::c_char;
    if parents.is_null() { return ERR_PTR(-ENOMEM); }
    for i in 0..(*info).num_parents as usize {
        *(*clk).parents.add(i) = (*info).parents[i];
        let parent = tegra_bpmp_clk_find(clocks, num_clocks, (*info).parents[i]);
        if parent.is_null() { dev_err!((*bpmp).dev, "no parent {} found for {}\n", (*info).parents[i], (*info).id); continue; }
        *parents.add(i) = (*parent).name.as_ptr();
    }
    init.parent_names = parents;
    let err = devm_clk_hw_register((*bpmp).dev, &mut (*clk).hw);
    kfree!(parents);
    if err < 0 { return ERR_PTR(err); } clk
}

unsafe fn tegra_bpmp_register_clocks_one(bpmp: *mut tegra_bpmp, infos: *mut tegra_bpmp_clk_info, i: u32, count: u32) {
    if !(*bpmp).clocks.add(i as usize).read().is_null() { return; }
    let info = infos.add(i as usize);
    for j in 0..(*info).num_parents as usize { let p_i = tegra_bpmp_clk_id_to_index(infos, count, (*info).parents[j]); if p_i < count { tegra_bpmp_register_clocks_one(bpmp, infos, p_i, count); } }
    let clk = tegra_bpmp_clk_register(bpmp, info, infos, count);
    if IS_ERR(clk) { dev_err!((*bpmp).dev, "failed to register clock {}\n", (*info).id); }
    (*bpmp).clocks.add(i as usize).write(clk);
}

unsafe fn tegra_bpmp_register_clocks(bpmp: *mut tegra_bpmp, infos: *mut tegra_bpmp_clk_info, count: u32) -> i32 {
    (*bpmp).num_clocks = count;
    (*bpmp).clocks = devm_kcalloc!((*bpmp).dev, count as usize, ::core::mem::size_of::<*mut tegra_bpmp_clk>(), GFP_KERNEL) as *mut *mut tegra_bpmp_clk;
    if (*bpmp).clocks.is_null() { return -ENOMEM; }
    for i in 0..count { tegra_bpmp_register_clocks_one(bpmp, infos, i, count); } 0
}

unsafe fn tegra_bpmp_unregister_clocks(bpmp: *mut tegra_bpmp) {
    for i in 0..(*bpmp).num_clocks as usize { clk_hw_unregister(&mut (*(*bpmp).clocks.add(i)).hw); }
}

unsafe fn tegra_bpmp_clk_of_xlate(clkspec: *mut of_phandle_args, data: *mut ::core::ffi::c_void) -> *mut clk_hw {
    let bpmp = data as *mut tegra_bpmp; let id = (*clkspec).args[0];
    for i in 0..(*bpmp).num_clocks as usize { let clk = *(*bpmp).clocks.add(i); if !clk.is_null() && (*clk).id == id { return &mut (*clk).hw; } } ::core::ptr::null_mut()
}

pub unsafe fn tegra_bpmp_init_clocks(bpmp: *mut tegra_bpmp) -> i32 {
    let mut clocks = ::core::ptr::null_mut(); let err = tegra_bpmp_probe_clocks(bpmp, &mut clocks); if err < 0 { return err; }
    let count = err as u32; dev_dbg!((*bpmp).dev, "{} clocks probed\n", count);
    let mut err = tegra_bpmp_register_clocks(bpmp, clocks, count);
    if err >= 0 { err = of_clk_add_hw_provider((*bpmp).dev.of_node, tegra_bpmp_clk_of_xlate, bpmp as *mut _); if err < 0 { tegra_bpmp_unregister_clocks(bpmp); } }
    kfree!(clocks); err
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
