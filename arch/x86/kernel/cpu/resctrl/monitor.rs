// SPDX-License-Identifier: GPL-2.0-only
/* Resource Director Technology (RDT) monitoring code. */

// C dependencies are supplied by the surrounding kernel translation.

pub static mut rdt_mon_capable: bool = false;

#[repr(C)]
pub struct mbm_correction_factor_table { pub rmidthreshold: u32, pub cf: u64 }

const fn cf(v: f64) -> u64 { (1048576.0 * v + 0.5) as u64 }

static mut snc_nodes_per_l3_cache: i32 = 1;
static mbm_cf_table: [mbm_correction_factor_table; 28] = [
    mbm_correction_factor_table { rmidthreshold: 7, cf: cf(1.000000) },
    mbm_correction_factor_table { rmidthreshold: 15, cf: cf(1.000000) },
    mbm_correction_factor_table { rmidthreshold: 15, cf: cf(0.969650) },
    mbm_correction_factor_table { rmidthreshold: 31, cf: cf(1.000000) },
    mbm_correction_factor_table { rmidthreshold: 31, cf: cf(1.066667) },
    mbm_correction_factor_table { rmidthreshold: 31, cf: cf(0.969650) },
    mbm_correction_factor_table { rmidthreshold: 47, cf: cf(1.142857) },
    mbm_correction_factor_table { rmidthreshold: 63, cf: cf(1.000000) },
    mbm_correction_factor_table { rmidthreshold: 63, cf: cf(1.185115) },
    mbm_correction_factor_table { rmidthreshold: 63, cf: cf(1.066553) },
    mbm_correction_factor_table { rmidthreshold: 79, cf: cf(1.454545) },
    mbm_correction_factor_table { rmidthreshold: 95, cf: cf(1.000000) },
    mbm_correction_factor_table { rmidthreshold: 95, cf: cf(1.230769) },
    mbm_correction_factor_table { rmidthreshold: 95, cf: cf(1.142857) },
    mbm_correction_factor_table { rmidthreshold: 95, cf: cf(1.066667) },
    mbm_correction_factor_table { rmidthreshold: 127, cf: cf(1.000000) },
    mbm_correction_factor_table { rmidthreshold: 127, cf: cf(1.254863) },
    mbm_correction_factor_table { rmidthreshold: 127, cf: cf(1.185255) },
    mbm_correction_factor_table { rmidthreshold: 151, cf: cf(1.000000) },
    mbm_correction_factor_table { rmidthreshold: 127, cf: cf(1.066667) },
    mbm_correction_factor_table { rmidthreshold: 167, cf: cf(1.000000) },
    mbm_correction_factor_table { rmidthreshold: 159, cf: cf(1.454334) },
    mbm_correction_factor_table { rmidthreshold: 183, cf: cf(1.000000) },
    mbm_correction_factor_table { rmidthreshold: 127, cf: cf(0.969744) },
    mbm_correction_factor_table { rmidthreshold: 191, cf: cf(1.280246) },
    mbm_correction_factor_table { rmidthreshold: 191, cf: cf(1.230921) },
    mbm_correction_factor_table { rmidthreshold: 215, cf: cf(1.000000) },
    mbm_correction_factor_table { rmidthreshold: 191, cf: cf(1.143118) },
];
static mut mbm_cf_rmidthreshold: u32 = u32::MAX;
static mut mbm_cf: u64 = 0;

unsafe fn get_corrected_mbm_count(rmid: u32, mut val: u64) -> u64 {
    if rmid > mbm_cf_rmidthreshold { val = (val.wrapping_mul(mbm_cf)) >> 20; }
    val
}

unsafe fn logical_rmid_to_physical_rmid(cpu: i32, lrmid: i32) -> i32 {
    let r = &rdt_resources_all[RDT_RESOURCE_L3 as usize].r_resctrl;
    if snc_nodes_per_l3_cache == 1 { return lrmid; }
    lrmid + (cpu_to_node(cpu) % snc_nodes_per_l3_cache) * r.mon.num_rmid as i32
}

unsafe fn __rmid_read_phys(prmid: u32, eventid: resctrl_event_id, val: *mut u64) -> i32 {
    let mut msr_val = msr { l: eventid as u64, h: prmid };
    wrmsrq(MSR_IA32_QM_EVTSEL, msr_val.q);
    rdmsrq(MSR_IA32_QM_CTR, &mut msr_val.q);
    if msr_val.q & RMID_VAL_ERROR != 0 { return -EIO; }
    if msr_val.q & RMID_VAL_UNAVAIL != 0 { return -EINVAL; }
    *val = msr_val.q; 0
}

unsafe fn get_arch_mbm_state(hw_dom: *mut rdt_hw_l3_mon_domain, rmid: u32,
                             eventid: resctrl_event_id) -> *mut arch_mbm_state {
    if !resctrl_is_mbm_event(eventid) { return core::ptr::null_mut(); }
    let state = (*hw_dom).arch_mbm_states[MBM_STATE_IDX(eventid)];
    if state.is_null() { core::ptr::null_mut() } else { state.add(rmid as usize) }
}

pub unsafe fn resctrl_arch_reset_rmid(r: *mut rdt_resource, d: *mut rdt_l3_mon_domain,
    _unused: u32, rmid: u32, eventid: resctrl_event_id) {
    let hw_dom = resctrl_to_arch_mon_dom(d);
    let cpu = cpumask_any(&(*d).hdr.cpu_mask);
    let am = get_arch_mbm_state(hw_dom, rmid, eventid);
    if !am.is_null() { core::ptr::write_bytes(am, 0, 1); let prmid = logical_rmid_to_physical_rmid(cpu, rmid as i32); __rmid_read_phys(prmid as u32, eventid, &mut (*am).prev_msr); }
}

pub unsafe fn resctrl_arch_reset_rmid_all(r: *mut rdt_resource, d: *mut rdt_l3_mon_domain) {
    let hw_dom = resctrl_to_arch_mon_dom(d);
    let mut eventid: resctrl_event_id = core::mem::zeroed();
    for_each_mbm_event_id!(eventid) {
        if !resctrl_is_mon_event_enabled(eventid) { continue; }
        let idx = MBM_STATE_IDX(eventid);
        core::ptr::write_bytes((*hw_dom).arch_mbm_states[idx], 0, (*r).mon.num_rmid as usize);
    }
}

unsafe fn mbm_overflow_count(prev_msr: u64, cur_msr: u64, width: u32) -> u64 {
    let shift = 64 - width;
    ((cur_msr << shift).wrapping_sub(prev_msr << shift)) >> shift
}

unsafe fn get_corrected_val(r: *mut rdt_resource, d: *mut rdt_l3_mon_domain, rmid: u32,
    eventid: resctrl_event_id, msr_val: u64) -> u64 {
    let hw_dom = resctrl_to_arch_mon_dom(d); let hw_res = resctrl_to_arch_res(r);
    let am = get_arch_mbm_state(hw_dom, rmid, eventid);
    let chunks;
    if !am.is_null() { (*am).chunks = (*am).chunks.wrapping_add(mbm_overflow_count((*am).prev_msr, msr_val, (*hw_res).mbm_width)); chunks = get_corrected_mbm_count(rmid, (*am).chunks); (*am).prev_msr = msr_val; } else { chunks = msr_val; }
    chunks.wrapping_mul((*hw_res).mon_scale)
}

pub unsafe fn resctrl_arch_rmid_read(r: *mut rdt_resource, hdr: *mut rdt_domain_hdr, _unused: u32,
    rmid: u32, eventid: resctrl_event_id, arch_priv: *mut core::ffi::c_void, val: *mut u64,
    _ignored: *mut core::ffi::c_void) -> i32 {
    resctrl_arch_rmid_read_context_check!();
    if (*r).rid == RDT_RESOURCE_PERF_PKG { return intel_aet_read_event((*hdr).id, rmid, arch_priv, val); }
    if !domain_header_is_valid(hdr, RESCTRL_MON_DOMAIN, RDT_RESOURCE_L3) { return -EINVAL; }
    if cpumask_empty(&(*hdr).cpu_mask) { pr_warn_once!("Domain {} has no CPUs\n", (*hdr).id); return -EINVAL; }
    let d = container_of!(hdr, rdt_l3_mon_domain, hdr); let hw_dom = resctrl_to_arch_mon_dom(d);
    let cpu = cpumask_any(&(*hdr).cpu_mask); let prmid = logical_rmid_to_physical_rmid(cpu, rmid as i32);
    let mut msr_val = 0; let ret = __rmid_read_phys(prmid as u32, eventid, &mut msr_val);
    if ret == 0 { *val = get_corrected_val(r, d, rmid, eventid, msr_val); }
    else if ret == -EINVAL { let am = get_arch_mbm_state(hw_dom, rmid, eventid); if !am.is_null() { (*am).prev_msr = 0; } }
    ret
}

unsafe fn __cntr_id_read(cntr_id: u32, val: *mut u64) -> i32 {
    let mut msr_val = msr { l: ABMC_EXTENDED_EVT_ID | ABMC_EVT_ID, h: cntr_id };
    wrmsrq(MSR_IA32_QM_EVTSEL, msr_val.q); rdmsrq(MSR_IA32_QM_CTR, &mut msr_val.q);
    if msr_val.q & RMID_VAL_ERROR != 0 { return -EIO; } if msr_val.q & RMID_VAL_UNAVAIL != 0 { return -EINVAL; }
    *val = msr_val.q; 0
}

pub unsafe fn resctrl_arch_reset_cntr(_r: *mut rdt_resource, d: *mut rdt_l3_mon_domain, _unused: u32,
    rmid: u32, cntr_id: i32, eventid: resctrl_event_id) { let am = get_arch_mbm_state(resctrl_to_arch_mon_dom(d), rmid, eventid); if !am.is_null() { core::ptr::write_bytes(am, 0, 1); __cntr_id_read(cntr_id as u32, &mut (*am).prev_msr); } }

pub unsafe fn resctrl_arch_cntr_read(r: *mut rdt_resource, d: *mut rdt_l3_mon_domain, _unused: u32,
    rmid: u32, cntr_id: i32, eventid: resctrl_event_id, val: *mut u64) -> i32 { let mut msr_val = 0; let ret = __cntr_id_read(cntr_id as u32, &mut msr_val); if ret != 0 { return ret; } *val = get_corrected_val(r, d, rmid, eventid, msr_val); 0 }

pub unsafe fn arch_mon_domain_online(_r: *mut rdt_resource, _d: *mut rdt_l3_mon_domain) { if snc_nodes_per_l3_cache > 1 { msr_clear_bit(MSR_RMID_SNC_CONFIG, 0); } }

pub unsafe fn rdt_get_l3_mon_config(r: *mut rdt_resource) -> i32 {
    snc_nodes_per_l3_cache = snc_get_config();
    resctrl_rmid_realloc_limit = boot_cpu_data.x86_cache_size * 1024;
    let hw_res = resctrl_to_arch_res(r); (*hw_res).mon_scale = boot_cpu_data.x86_cache_occ_scale / snc_nodes_per_l3_cache as u64;
    (*r).mon.num_rmid = (boot_cpu_data.x86_cache_max_rmid + 1) / snc_nodes_per_l3_cache as u32; (*hw_res).mbm_width = MBM_CNTR_WIDTH_BASE;
    let mbm_offset = boot_cpu_data.x86_cache_mbm_width_offset; if mbm_offset > 0 && mbm_offset <= MBM_CNTR_WIDTH_OFFSET_MAX { (*hw_res).mbm_width += mbm_offset; } else if mbm_offset > MBM_CNTR_WIDTH_OFFSET_MAX { pr_warn!("Ignoring impossible MBM counter offset\n"); }
    let threshold = resctrl_rmid_realloc_limit / (*r).mon.num_rmid; resctrl_rmid_realloc_threshold = resctrl_arch_round_mon_val(threshold);
    if rdt_cpu_has(X86_FEATURE_BMEC) || rdt_cpu_has(X86_FEATURE_ABMC) { let mut eax=0; let mut ebx=0; let mut ecx=0; let mut edx=0; cpuid_count(0x80000020, 3, &mut eax, &mut ebx, &mut ecx, &mut edx); (*r).mon.mbm_cfg_mask = ecx & MAX_EVT_CONFIG_BITS; }
    (*r).mon_capable = true; 0
}

pub unsafe fn intel_rdt_mbm_apply_quirk() { let cf_index = (boot_cpu_data.x86_cache_max_rmid + 1) / 8 - 1; if cf_index as usize >= mbm_cf_table.len() { pr_info!("No MBM correction factor available\n"); return; } mbm_cf_rmidthreshold = mbm_cf_table[cf_index as usize].rmidthreshold; mbm_cf = mbm_cf_table[cf_index as usize].cf; }

unsafe fn snc_get_config() -> i32 {
    if boot_cpu_data.x86_vendor != X86_VENDOR_INTEL { return 1; }
    let mut ret = topology_num_nodes_per_package();
    if ret > 1 && !x86_match_cpu(snc_cpu_ids) { pr_warn!("CoD enabled system? Resctrl not supported\n"); return 1; }
    match ret { 1 => {}, 2..=4 | 6 => { pr_info!("Sub-NUMA Cluster mode detected with {} nodes per L3 cache\n", ret); rdt_resources_all[RDT_RESOURCE_L3 as usize].r_resctrl.mon_scope = RESCTRL_L3_NODE; }, _ => { pr_warn!("Ignore improbable SNC node count {}\n", ret); ret = 1; } } ret
}

static snc_cpu_ids: [x86_cpu_id; 7] = [
    X86_MATCH_VFM!(INTEL_ICELAKE_X, 0), X86_MATCH_VFM!(INTEL_SAPPHIRERAPIDS_X, 0),
    X86_MATCH_VFM!(INTEL_EMERALDRAPIDS_X, 0), X86_MATCH_VFM!(INTEL_GRANITERAPIDS_X, 0),
    X86_MATCH_VFM!(INTEL_ATOM_CRESTMONT_X, 0), X86_MATCH_VFM!(INTEL_ATOM_DARKMONT_X, 0),
    x86_cpu_id::default(),
];

unsafe fn resctrl_abmc_set_one_amd(arg: *mut core::ffi::c_void) { let enable = arg as *mut bool; if *enable { msr_set_bit(MSR_IA32_L3_QOS_EXT_CFG, ABMC_ENABLE_BIT); } else { msr_clear_bit(MSR_IA32_L3_QOS_EXT_CFG, ABMC_ENABLE_BIT); } }

unsafe fn _resctrl_abmc_enable(r: *mut rdt_resource, enable: bool) {
    lockdep_assert_cpus_held!();
    let mut d: *mut rdt_l3_mon_domain = core::ptr::null_mut();
    list_for_each_entry_rcu!(d, &(*r).mon_domains, hdr.list, lockdep_is_cpus_held!()) {
        on_each_cpu_mask(&(*d).hdr.cpu_mask, resctrl_abmc_set_one_amd, &enable as *const bool as *mut _, 1);
        resctrl_arch_reset_rmid_all(r, d);
    }
}

pub unsafe fn resctrl_arch_mbm_cntr_assign_set(r: *mut rdt_resource, enable: bool) -> i32 {
    let hw_res = resctrl_to_arch_res(r);
    if (*r).mon.mbm_cntr_assignable && (*hw_res).mbm_cntr_assign_enabled != enable { _resctrl_abmc_enable(r, enable); (*hw_res).mbm_cntr_assign_enabled = enable; } 0
}

pub unsafe fn resctrl_arch_mbm_cntr_assign_enabled(r: *mut rdt_resource) -> bool { resctrl_to_arch_res(r).as_ref().unwrap().mbm_cntr_assign_enabled }

unsafe fn resctrl_abmc_config_one_amd(info: *mut core::ffi::c_void) { let abmc_cfg = info as *mut l3_qos_abmc_cfg; wrmsrq(MSR_IA32_L3_QOS_ABMC_CFG, (*abmc_cfg).full); }

pub unsafe fn resctrl_arch_config_cntr(_r: *mut rdt_resource, d: *mut rdt_l3_mon_domain, evtid: resctrl_event_id,
    rmid: u32, _closid: u32, cntr_id: u32, assign: bool) {
    let hw_dom = resctrl_to_arch_mon_dom(d); let mut abmc_cfg: l3_qos_abmc_cfg = core::mem::zeroed();
    (*abmc_cfg.split_mut()).cfg_en = 1; (*abmc_cfg.split_mut()).cntr_en = if assign { 1 } else { 0 }; (*abmc_cfg.split_mut()).cntr_id = cntr_id; (*abmc_cfg.split_mut()).bw_src = rmid; if assign { (*abmc_cfg.split_mut()).bw_type = resctrl_get_mon_evt_cfg(evtid); }
    smp_call_function_any(&(*d).hdr.cpu_mask, resctrl_abmc_config_one_amd, &mut abmc_cfg as *mut _ as *mut _, 1);
    let am = get_arch_mbm_state(hw_dom, rmid, evtid); if !am.is_null() { core::ptr::write_bytes(am, 0, 1); }
}

pub unsafe fn resctrl_arch_mbm_cntr_assign_set_one(r: *mut rdt_resource) { let hw_res = resctrl_to_arch_res(r); resctrl_abmc_set_one_amd(&mut (*hw_res).mbm_cntr_assign_enabled as *mut bool as *mut _); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
