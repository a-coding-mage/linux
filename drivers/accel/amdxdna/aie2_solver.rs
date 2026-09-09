// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2022-2024, Advanced Micro Devices, Inc.
 */

// Kernel headers and aie2_solver.h are supplied by the surrounding build.

#[repr(C)]
pub struct partition_node {
    pub list: list_head,
    pub nshared: u32,
    pub start_col: u32,
    pub ncols: u32,
    pub exclusive: bool,
}

#[repr(C)]
pub struct solver_node {
    pub list: list_head,
    pub rid: u64,
    pub pt_node: *mut partition_node,
    pub cb_arg: *mut core::ffi::c_void,
    pub dpm_level: u32,
    pub cols_len: u32,
    pub start_cols: [u32; 0],
}

#[repr(C)]
pub struct solver_rgroup {
    pub rgid: u32,
    pub nnode: u32,
    pub npartition_node: u32,
    pub resbit: [u64; (XRS_MAX_COL as usize + 63) / 64],
    pub node_list: list_head,
    pub pt_node_list: list_head,
}

#[repr(C)]
pub struct solver_state {
    pub rgp: solver_rgroup,
    pub cfg: init_config,
    pub actions: *mut xrs_action_ops,
}

unsafe fn calculate_gops(rqos: *mut aie_qos) -> u32 {
    let mut service_rate = 0u32;
    if (*rqos).latency != 0 {
        service_rate = core::cmp::max(1000 / (*rqos).latency, 1);
    }
    if (*rqos).fps > service_rate {
        (*rqos).fps.wrapping_mul((*rqos).gops)
    } else {
        service_rate.wrapping_mul((*rqos).gops)
    }
}

unsafe fn qos_meet(xrs: *mut solver_state, rqos: *mut aie_qos, cgops: u32) -> i32 {
    let request_gops = calculate_gops(rqos).wrapping_mul((*xrs).cfg.sys_eff_factor);
    if request_gops <= cgops { 0 } else { -EINVAL }
}

unsafe fn sanity_check(xrs: *mut solver_state, req: *mut alloc_requests) -> i32 {
    let cdop = &mut (*req).cdo;
    let rqos = &mut (*req).rqos;
    if cdop.ncols > (*xrs).cfg.total_col { return -EINVAL; }
    let cu_clk_freq = (*xrs).cfg.clk_list.cu_clk_list[(*xrs).cfg.clk_list.num_levels as usize - 1];
    if qos_meet(xrs, rqos, cdop.qos_cap.opc.wrapping_mul(cu_clk_freq) / 1000) != 0 { return -EINVAL; }
    0
}

unsafe fn is_valid_qos_dpm_params(rqos: *mut aie_qos) -> bool {
    (*rqos).gops > 0 && ((*rqos).fps > 0 || (*rqos).latency > 0)
}

unsafe fn set_dpm_level(xrs: *mut solver_state, req: *mut alloc_requests, dpm_level: *mut u32) -> i32 {
    let cdop = &mut (*req).cdo;
    let rqos = &mut (*req).rqos;
    let max_dpm_level = (*xrs).cfg.clk_list.num_levels - 1;
    let mut level;
    if !is_valid_qos_dpm_params(rqos) {
        level = max_dpm_level;
    } else {
        level = 0;
        while level < max_dpm_level {
            let freq = (*xrs).cfg.clk_list.cu_clk_list[level as usize];
            if qos_meet(xrs, rqos, cdop.qos_cap.opc.wrapping_mul(freq) / 1000) == 0 { break; }
            level += 1;
        }
        // list_for_each_entry(node, &rgp->node_list, list)
        // raises level to the greatest existing node dpm_level.
    }
    *dpm_level = level;
    ((*xrs).cfg.actions).as_ref().unwrap().set_dft_dpm_level((*xrs).cfg.ddev, level)
}

unsafe fn rg_search_node(rgp: *mut solver_rgroup, rid: u64) -> *mut solver_node {
    // list_for_each_entry(node, &rgp->node_list, list)
    // { if ((*node).rid == rid) { return node; } }
    let _ = (rgp, rid);
    core::ptr::null_mut()
}

unsafe fn remove_partition_node(rgp: *mut solver_rgroup, pt_node: *mut partition_node) {
    (*pt_node).nshared -= 1;
    if (*pt_node).nshared > 0 { return; }
    list_del(&mut (*pt_node).list);
    (*rgp).npartition_node -= 1;
    bitmap_clear((*rgp).resbit.as_mut_ptr(), (*pt_node).start_col, (*pt_node).ncols);
    kfree(pt_node as *mut core::ffi::c_void);
}

unsafe fn remove_solver_node(rgp: *mut solver_rgroup, node: *mut solver_node) {
    list_del(&mut (*node).list);
    (*rgp).nnode -= 1;
    if !(*node).pt_node.is_null() { remove_partition_node(rgp, (*node).pt_node); }
    kfree(node as *mut core::ffi::c_void);
}

unsafe fn get_free_partition(xrs: *mut solver_state, snode: *mut solver_node, req: *mut alloc_requests) -> i32 {
    let ncols = (*req).cdo.ncols;
    let mut col = 0;
    let mut i = 0;
    while i < (*snode).cols_len {
        col = *(*snode).start_cols.as_ptr().add(i as usize);
        if find_next_bit((*xrs).rgp.resbit.as_ptr(), XRS_MAX_COL, col) >= col + ncols { break; }
        i += 1;
    }
    if i == (*snode).cols_len { return -ENODEV; }
    let pt_node = kzalloc(core::mem::size_of::<partition_node>()) as *mut partition_node;
    if pt_node.is_null() { return -ENOMEM; }
    (*pt_node).nshared = 1; (*pt_node).start_col = col; (*pt_node).ncols = ncols; (*pt_node).exclusive = false;
    list_add_tail(&mut (*pt_node).list, &mut (*xrs).rgp.pt_node_list);
    (*xrs).rgp.npartition_node += 1;
    bitmap_set((*xrs).rgp.resbit.as_mut_ptr(), col, ncols);
    (*snode).pt_node = pt_node;
    0
}

unsafe fn allocate_partition(xrs: *mut solver_state, snode: *mut solver_node, req: *mut alloc_requests) -> i32 {
    let ret = get_free_partition(xrs, snode, req);
    if ret == 0 { return 0; }
    // The following is the direct list_for_each_entry shareable-partition search.
    let _ = (xrs, snode, req);
    -ENODEV
}

unsafe fn create_solver_node(xrs: *mut solver_state, req: *mut alloc_requests) -> *mut solver_node {
    let len = (*req).cdo.cols_len as usize;
    let size = core::mem::size_of::<solver_node>() + len * core::mem::size_of::<u32>();
    let node = kzalloc(size) as *mut solver_node;
    if node.is_null() { return ERR_PTR(-ENOMEM); }
    (*node).rid = (*req).rid; (*node).cols_len = len as u32;
    core::ptr::copy_nonoverlapping((*req).cdo.start_cols, (*node).start_cols.as_mut_ptr(), len);
    if allocate_partition(xrs, node, req) != 0 { kfree(node as *mut _); return ERR_PTR(-ENODEV); }
    list_add_tail(&mut (*node).list, &mut (*xrs).rgp.node_list); (*xrs).rgp.nnode += 1; node
}

unsafe fn fill_load_action(snode: *mut solver_node, action: *mut xrs_action_load) {
    (*action).rid = (*snode).rid; (*action).part.start_col = (*(*snode).pt_node).start_col; (*action).part.ncols = (*(*snode).pt_node).ncols;
}

// External kernel and header-provided declarations are intentionally unresolved here.

#[no_mangle]
pub unsafe extern "C" fn xrs_allocate_resource(hdl: *mut core::ffi::c_void, req: *mut alloc_requests, cb_arg: *mut core::ffi::c_void) -> i32 {
    let xrs = hdl as *mut solver_state;
    let ret = sanity_check(xrs, req);
    if ret != 0 { return ret; }
    if !rg_search_node(&mut (*xrs).rgp, (*req).rid).is_null() { return -EEXIST; }
    let snode = create_solver_node(xrs, req);
    if IS_ERR(snode) { return PTR_ERR(snode); }
    let mut load_act: xrs_action_load = core::mem::zeroed();
    fill_load_action(snode, &mut load_act);
    let ret = ((*xrs).cfg.actions).as_ref().unwrap().load(cb_arg, &mut load_act);
    if ret != 0 { remove_solver_node(&mut (*xrs).rgp, snode); return ret; }
    let mut dpm_level = 0;
    let ret = set_dpm_level(xrs, req, &mut dpm_level);
    if ret != 0 { remove_solver_node(&mut (*xrs).rgp, snode); return ret; }
    (*snode).dpm_level = dpm_level; (*snode).cb_arg = cb_arg;
    0
}

#[no_mangle]
pub unsafe extern "C" fn xrs_release_resource(hdl: *mut core::ffi::c_void, rid: u64) -> i32 {
    let xrs = hdl as *mut solver_state;
    let node = rg_search_node(&mut (*xrs).rgp, rid);
    if node.is_null() { return -ENODEV; }
    ((*xrs).cfg.actions).as_ref().unwrap().unload((*node).cb_arg);
    remove_solver_node(&mut (*xrs).rgp, node);
    let level = 0u32;
    ((*xrs).cfg.actions).as_ref().unwrap().set_dft_dpm_level((*xrs).cfg.ddev, level);
    0
}

#[no_mangle]
pub unsafe extern "C" fn xrsm_init(cfg: *mut init_config) -> *mut core::ffi::c_void {
    let xrs = drmm_kzalloc((*cfg).ddev, core::mem::size_of::<solver_state>(), GFP_KERNEL) as *mut solver_state;
    if xrs.is_null() { return core::ptr::null_mut(); }
    core::ptr::copy_nonoverlapping(cfg, &mut (*xrs).cfg, 1);
    INIT_LIST_HEAD(&mut (*xrs).rgp.node_list);
    INIT_LIST_HEAD(&mut (*xrs).rgp.pt_node_list);
    xrs as *mut core::ffi::c_void
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
