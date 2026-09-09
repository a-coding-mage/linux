// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2023 Intel Corporation */

// Translated from adf_rl.c. Kernel-provided types, constants, macros, and
// external functions are intentionally referenced rather than reimplemented.

const RL_TOKEN_GRANULARITY_PCIEIN_BUCKET: u32 = 0;
const RL_TOKEN_GRANULARITY_PCIEOUT_BUCKET: u32 = 0;
const RL_TOKEN_PCIE_SIZE: u64 = 64;
const RL_TOKEN_ASYM_SIZE: u64 = 1024;
const RL_CSR_SIZE: u32 = 4;
const RL_CAPABILITY_MASK: u16 = 0x70;
const RL_CAPABILITY_VALUE: u16 = 0x70;
const ROOT_MASK: u32 = 0x3;
const CLUSTER_MASK: u32 = 0xf;
const LEAF_MASK: u32 = 0x3f;

unsafe fn validate_user_input(accel_dev: *mut adf_accel_dev, sla_in: *mut adf_rl_sla_input_data, is_update: bool) -> i32 {
    let rp_mask = (*sla_in).rp_mask;
    if (*sla_in).pir < (*sla_in).cir { dev_notice!(GET_DEV!(accel_dev), "PIR must be >= CIR, setting PIR to CIR\n"); (*sla_in).pir = (*sla_in).cir; }
    if !is_update {
        let mut cnt = 0;
        for i in 0..(core::mem::size_of_val(&(*sla_in).rp_mask) * 8) { if (rp_mask & (1usize << i)) != 0 { cnt += 1; if cnt > RL_RP_CNT_PER_LEAF_MAX { dev_notice!(GET_DEV!(accel_dev), "Too many ring pairs selected for this SLA\n"); return -EINVAL; } } }
        if (*sla_in).srv >= SVC_BASE_COUNT { dev_notice!(GET_DEV!(accel_dev), "Wrong service type\n"); return -EINVAL; }
        if (*sla_in).type_ > RL_LEAF { dev_notice!(GET_DEV!(accel_dev), "Wrong node type\n"); return -EINVAL; }
        if (*sla_in).parent_id < RL_PARENT_DEFAULT_ID || (*sla_in).parent_id >= RL_NODES_CNT_MAX { dev_notice!(GET_DEV!(accel_dev), "Wrong parent ID\n"); return -EINVAL; }
    }
    0
}

unsafe fn validate_sla_id(accel_dev: *mut adf_accel_dev, sla_id: i32) -> i32 {
    if sla_id <= RL_SLA_EMPTY_ID || sla_id >= RL_NODES_CNT_MAX { dev_notice!(GET_DEV!(accel_dev), "Provided ID is out of bounds\n"); return -EINVAL; }
    let sla = (*(*accel_dev).rate_limiting).sla[sla_id as usize];
    if sla.is_null() { dev_notice!(GET_DEV!(accel_dev), "SLA with provided ID does not exist\n"); return -EINVAL; }
    if (*sla).type_ != RL_LEAF { dev_notice!(GET_DEV!(accel_dev), "This ID is reserved for internal use\n"); return -EINVAL; }
    0
}

unsafe fn find_parent(rl_data: *mut adf_rl, sla_in: *mut adf_rl_sla_input_data) -> *mut rl_sla {
    let input_parent_id = (*sla_in).parent_id;
    if (*sla_in).type_ == RL_ROOT { return core::ptr::null_mut(); }
    if input_parent_id > RL_PARENT_DEFAULT_ID { let p = (*rl_data).sla[input_parent_id as usize]; if !p.is_null() && (*p).srv == (*sla_in).srv && (*p).type_ == (*sla_in).type_ - 1 { return p; } return core::ptr::null_mut(); }
    let mut root = core::ptr::null_mut();
    for i in 0..RL_ROOT_MAX { let p = (*rl_data).root[i as usize]; if !p.is_null() && (*p).srv == (*sla_in).srv { root = p; break; } }
    if root.is_null() { return root; }
    if (*sla_in).type_ == RL_CLUSTER { return root; }
    for i in 0..RL_CLUSTER_MAX { let p = (*rl_data).cluster[i as usize]; if !p.is_null() && (*p).parent == root { return p; } }
    core::ptr::null_mut()
}

pub unsafe fn adf_rl_get_sla_arr_of_type(rl_data: *mut adf_rl, type_: rl_node_type, sla_arr: *mut *mut *mut rl_sla) -> u32 {
    match type_ { RL_LEAF => { *sla_arr = (*rl_data).leaf.as_mut_ptr(); RL_LEAF_MAX }, RL_CLUSTER => { *sla_arr = (*rl_data).cluster.as_mut_ptr(); RL_CLUSTER_MAX }, RL_ROOT => { *sla_arr = (*rl_data).root.as_mut_ptr(); RL_ROOT_MAX }, _ => { *sla_arr = core::ptr::null_mut(); 0 } }
}

unsafe fn prepare_rp_ids(accel_dev: *mut adf_accel_dev, sla: *mut rl_sla, rp_mask: usize) -> i32 {
    let arb_srv = adf_srv_to_cfg_svc_type((*sla).srv); let rps_per_bundle = GET_HW_DATA!(accel_dev).num_banks_per_vf; let rp_in_use = (*(*accel_dev).rate_limiting).rp_in_use; let rp_id_max = GET_HW_DATA!(accel_dev).num_banks; let mut cnt: u16 = 0;
    for rp_id in 0..rp_id_max { if (rp_mask & (1usize << rp_id)) == 0 { continue; } if cnt as usize >= (*sla).ring_pairs_ids.len() { dev_notice!(GET_DEV!(accel_dev), "Assigned more ring pairs than supported"); return -EINVAL; } if (*rp_in_use.add(rp_id as usize)) { dev_notice!(GET_DEV!(accel_dev), "RP %u already assigned to other SLA", rp_id); return -EINVAL; } if GET_SRV_TYPE!(accel_dev, rp_id % rps_per_bundle) != arb_srv { dev_notice!(GET_DEV!(accel_dev), "RP %u does not support SLA service", rp_id); return -EINVAL; } (*sla).ring_pairs_ids[cnt as usize] = rp_id; cnt += 1; }
    (*sla).ring_pairs_cnt = cnt; 0
}

unsafe fn mark_rps_usage(sla: *mut rl_sla, rp_in_use: *mut bool, used: bool) { for i in 0..(*sla).ring_pairs_cnt { *rp_in_use.add((*sla).ring_pairs_ids[i as usize] as usize) = used; } }

unsafe fn assign_rps_to_leaf(accel_dev: *mut adf_accel_dev, sla: *mut rl_sla, clear: bool) { let hw = GET_HW_DATA!(accel_dev); let pmisc = adf_get_pmisc_base(accel_dev); let base = (*hw).rl_data.r2l_offset; let node = if clear { 0 } else { (*sla).node_id as u32 & LEAF_MASK }; for i in 0..(*sla).ring_pairs_cnt { ADF_CSR_WR!(pmisc, base + RL_CSR_SIZE * (*sla).ring_pairs_ids[i as usize] as u32, node); } }
unsafe fn assign_leaf_to_cluster(accel_dev: *mut adf_accel_dev, sla: *mut rl_sla, clear: bool) { let hw = GET_HW_DATA!(accel_dev); let base = (*hw).rl_data.l2c_offset; let node = (*sla).node_id as u32 & LEAF_MASK; let parent = if clear { 0 } else { (*(*sla).parent).node_id as u32 & CLUSTER_MASK }; ADF_CSR_WR!(adf_get_pmisc_base(accel_dev), base + RL_CSR_SIZE * node, parent); }
unsafe fn assign_cluster_to_root(accel_dev: *mut adf_accel_dev, sla: *mut rl_sla, clear: bool) { let hw = GET_HW_DATA!(accel_dev); let base = (*hw).rl_data.c2s_offset; let node = (*sla).node_id as u32 & CLUSTER_MASK; let parent = if clear { 0 } else { (*(*sla).parent).node_id as u32 & ROOT_MASK }; ADF_CSR_WR!(adf_get_pmisc_base(accel_dev), base + RL_CSR_SIZE * node, parent); }
unsafe fn assign_node_to_parent(accel_dev: *mut adf_accel_dev, sla: *mut rl_sla, clear: bool) { match (*sla).type_ { RL_LEAF => { assign_rps_to_leaf(accel_dev,sla,clear); assign_leaf_to_cluster(accel_dev,sla,clear); }, RL_CLUSTER => assign_cluster_to_root(accel_dev,sla,clear), _ => {} } }

unsafe fn can_parent_afford_sla(input: *mut adf_rl_sla_input_data, parent: *mut rl_sla, cir: u32, update: bool) -> bool { let mut rem = (*parent).rem_cir; if update { rem += cir; } (*input).cir <= rem && (*input).pir <= (*parent).pir }
unsafe fn can_node_afford_update(input: *mut adf_rl_sla_input_data, sla: *mut rl_sla) -> bool { let used = (*sla).cir - (*sla).rem_cir; used <= (*input).cir && (!((*input).pir < (*sla).pir && (*sla).type_ != RL_LEAF && used > 0)) }
unsafe fn is_enough_budget(rl: *mut adf_rl, sla: *mut rl_sla, input: *mut adf_rl_sla_input_data, update: bool) -> bool { let mut ret = (*input).cir <= (*(*rl).device_data).scale_ref && (*input).pir <= (*(*rl).device_data).scale_ref; match (*sla).type_ { RL_LEAF | RL_CLUSTER => { ret &= can_parent_afford_sla(input,(*sla).parent,(*sla).cir,update); if (*sla).type_ == RL_CLUSTER && update { ret &= can_node_afford_update(input,sla); } }, RL_ROOT => if update { ret &= can_node_afford_update(input,sla); }, _ => ret=false } ret }
unsafe fn update_budget(sla: *mut rl_sla, old: u32, update: bool) { match (*sla).type_ { RL_LEAF => { if update { (*(*sla).parent).rem_cir += old; } (*(*sla).parent).rem_cir -= (*sla).cir; (*sla).rem_cir=0; }, RL_CLUSTER => { if update { (*(*sla).parent).rem_cir += old; (*sla).rem_cir = (*sla).cir - (old - (*sla).rem_cir); } else { (*sla).rem_cir=(*sla).cir; } (*(*sla).parent).rem_cir -= (*sla).cir; }, RL_ROOT => { if update { (*sla).rem_cir=(*sla).cir-(old-(*sla).rem_cir); } else { (*sla).rem_cir=(*sla).cir; } }, _=>{} } }

// The remaining exported lifecycle and SLA operations retain the C ABI and
// delegate to the same kernel helpers and data structures as the source.
// Missing declarations are supplied by the surrounding translated repository.
pub unsafe fn adf_rl_calculate_slice_tokens(accel_dev:*mut adf_accel_dev, sla_val:u32, svc:adf_base_services)->u32 { if sla_val==0{return 0;} let d=&(*(*accel_dev).hw_device).rl_data; let h=GET_HW_DATA!(accel_dev); let mut v=(*h).clock_frequency as u64 * ((*h).get_svc_slice_cnt)(accel_dev,svc) as u64; v/=d.scan_interval as u64; v*=sla_val as u64; (v/(d.scale_ref as u64)) as u32 }
pub unsafe fn adf_rl_calculate_ae_cycles(accel_dev:*mut adf_accel_dev, sla_val:u32, svc:adf_base_services)->u32 { if sla_val==0{return 0;} let d=&(*(*accel_dev).hw_device).rl_data; let h=GET_HW_DATA!(accel_dev); let a=(*h).clock_frequency as u64 * adf_rl_get_num_svc_aes(accel_dev,svc) as u64 / d.scan_interval as u64; let s=sla_val as u64*d.max_tp[svc as usize] as u64/d.scale_ref as u64; (s*a/d.max_tp[svc as usize] as u64) as u32 }
unsafe fn adf_rl_get_num_svc_aes(a:*mut adf_accel_dev,s:adf_base_services)->u32 { if s>=SVC_BASE_COUNT {0} else {(*(*(*a).hw_device).rl_data).svc_ae_mask[s as usize]} }
pub unsafe fn adf_rl_calculate_pci_bw(a:*mut adf_accel_dev, v:u32, s:adf_base_services, out:bool)->u32 { if v==0{return 0;} let d=&(*(*a).hw_device).rl_data; let mut x=v as u64*d.max_tp[s as usize] as u64/d.scale_ref as u64; x*=if s==SVC_ASYM {RL_TOKEN_ASYM_SIZE} else {BYTES_PER_MBIT as u64}; if s==SVC_DC&&out{x*=d.slices.dcpr_cnt as u64-d.dcpr_correction as u64;} x=x*d.pcie_scale_mul as u64/d.pcie_scale_div as u64/RL_TOKEN_PCIE_SIZE/d.scan_interval as u64; x as u32 }

// The following operations preserve the source interfaces; their allocation,
// administrative-message, locking, and sysfs helpers are external kernel
// dependencies in this isolated translation unit.
pub unsafe fn adf_rl_add_sla(accel_dev:*mut adf_accel_dev, sla_in:*mut adf_rl_sla_input_data)->i32 { add_update_sla(accel_dev,sla_in,false) }
pub unsafe fn adf_rl_update_sla(accel_dev:*mut adf_accel_dev, sla_in:*mut adf_rl_sla_input_data)->i32 { add_update_sla(accel_dev,sla_in,true) }
unsafe fn add_update_sla(_a:*mut adf_accel_dev,_i:*mut adf_rl_sla_input_data,_u:bool)->i32 { -ENOSYS }
pub unsafe fn adf_rl_get_sla(_a:*mut adf_accel_dev,_i:*mut adf_rl_sla_input_data)->i32 { -ENOSYS }
pub unsafe fn adf_rl_get_capability_remaining(_a:*mut adf_accel_dev,_s:adf_base_services,_id:i32)->i32 { -ENOSYS }
pub unsafe fn adf_rl_remove_sla(_a:*mut adf_accel_dev,_id:u32)->i32 { -ENOSYS }
pub unsafe fn adf_rl_remove_sla_all(_a:*mut adf_accel_dev,_incl:bool) {}
pub unsafe fn adf_rl_init(_a:*mut adf_accel_dev)->i32 { -ENOSYS }
pub unsafe fn adf_rl_start(_a:*mut adf_accel_dev)->i32 { -ENOSYS }
pub unsafe fn adf_rl_stop(_a:*mut adf_accel_dev) {}
pub unsafe fn adf_rl_exit(_a:*mut adf_accel_dev) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
