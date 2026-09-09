// SPDX-License-Identifier: GPL-2.0-only
/*
 * User interface for Resource Allocation in Resource Director Technology(RDT)
 *
 * Copyright (C) 2016 Intel Corporation
 *
 * Author: Fenghua Yu <fenghua.yu@intel.com>
 *
 * More information about RDT be found in the Intel (R) x86 Architecture
 * Software Developer Manual.
 */

// C includes and build-time configuration dependencies are supplied externally.

extern "C" {
    static mut rdt_enable_key: StaticKeyFalse;
    static mut rdt_mon_enable_key: StaticKeyFalse;
    static mut rdt_alloc_enable_key: StaticKeyFalse;
}

pub unsafe extern "C" fn resctrl_arch_sync_cpu_closid_rmid(info: *mut core::ffi::c_void) {
    let r = info as *mut resctrl_cpu_defaults;

    if !r.is_null() {
        this_cpu_write_default_closid((*r).closid);
        this_cpu_write_default_rmid((*r).rmid);
    }

    resctrl_arch_sched_in(current);
}

const INVALID_CONFIG_INDEX: u32 = u32::MAX;

/// mon_event_config_index_get - get the hardware index for the configurable event
unsafe fn mon_event_config_index_get(evtid: u32) -> u32 {
    match evtid {
        QOS_L3_MBM_TOTAL_EVENT_ID => 0,
        QOS_L3_MBM_LOCAL_EVENT_ID => 1,
        _ => INVALID_CONFIG_INDEX,
    }
}

pub unsafe extern "C" fn resctrl_arch_mon_event_config_read(_config_info: *mut core::ffi::c_void) {
    let config_info = _config_info as *mut resctrl_mon_config_info;
    let index = mon_event_config_index_get((*config_info).evtid);
    if index == INVALID_CONFIG_INDEX {
        pr_warn_once!("Invalid event id %d\n", (*config_info).evtid);
        return;
    }
    let msrval = rdmsrq(MSR_IA32_EVT_CFG_BASE + index as u64);
    (*config_info).mon_config = msrval & MAX_EVT_CONFIG_BITS;
}

pub unsafe extern "C" fn resctrl_arch_mon_event_config_write(_config_info: *mut core::ffi::c_void) {
    let config_info = _config_info as *mut resctrl_mon_config_info;
    let index = mon_event_config_index_get((*config_info).evtid);
    if index == INVALID_CONFIG_INDEX {
        pr_warn_once!("Invalid event id %d\n", (*config_info).evtid);
        return;
    }
    wrmsrq(MSR_IA32_EVT_CFG_BASE + index as u64, (*config_info).mon_config);
}

unsafe extern "C" fn l3_qos_cfg_update(arg: *mut core::ffi::c_void) {
    let enable = arg as *mut bool;
    wrmsrq(MSR_IA32_L3_QOS_CFG, if *enable { L3_QOS_CDP_ENABLE } else { 0 });
}

unsafe extern "C" fn l2_qos_cfg_update(arg: *mut core::ffi::c_void) {
    let enable = arg as *mut bool;
    wrmsrq(MSR_IA32_L2_QOS_CFG, if *enable { L2_QOS_CDP_ENABLE } else { 0 });
}

unsafe fn set_cache_qos_cfg(level: i32, enable: bool) -> i32 {
    let update: unsafe extern "C" fn(*mut core::ffi::c_void) = if level == RDT_RESOURCE_L3 {
        l3_qos_cfg_update
    } else if level == RDT_RESOURCE_L2 {
        l2_qos_cfg_update
    } else {
        return -EINVAL;
    };
    let mut cpu_mask: cpumask_var_t = core::ptr::null_mut();
    if !zalloc_cpumask_var(&mut cpu_mask, GFP_KERNEL) { return -ENOMEM; }

    let r_l = &mut rdt_resources_all[level as usize].r_resctrl;
    let mut d: *mut rdt_ctrl_domain = core::ptr::null_mut();
    list_for_each_entry_rcu!(d, &r_l.ctrl_domains, hdr.list, lockdep_is_cpus_held(), {
        if r_l.cache.arch_has_per_cpu_cfg {
            for_each_cpu!(cpu, &(*d).hdr.cpu_mask, { cpumask_set_cpu(cpu, cpu_mask); });
        } else {
            cpumask_set_cpu(cpumask_any(&(*d).hdr.cpu_mask), cpu_mask);
        }
    });
    on_each_cpu_mask(cpu_mask, update, &enable as *const bool as *mut _, 1);
    free_cpumask_var(cpu_mask);
    0
}

pub unsafe extern "C" fn rdt_domain_reconfigure_cdp(r: *mut rdt_resource) {
    let hw_res = resctrl_to_arch_res(r);
    if !(*r).cdp_capable { return; }
    if (*r).rid == RDT_RESOURCE_L2 { l2_qos_cfg_update(&mut (*hw_res).cdp_enabled as *mut _ as *mut _); }
    if (*r).rid == RDT_RESOURCE_L3 { l3_qos_cfg_update(&mut (*hw_res).cdp_enabled as *mut _ as *mut _); }
}

unsafe fn cdp_enable(level: i32) -> i32 {
    let r_l = &mut rdt_resources_all[level as usize].r_resctrl;
    if !r_l.alloc_capable { return -EINVAL; }
    let ret = set_cache_qos_cfg(level, true);
    if ret == 0 { rdt_resources_all[level as usize].cdp_enabled = true; }
    ret
}

unsafe fn cdp_disable(level: i32) {
    let r_hw = &mut rdt_resources_all[level as usize];
    if r_hw.cdp_enabled {
        set_cache_qos_cfg(level, false);
        r_hw.cdp_enabled = false;
    }
}

pub unsafe extern "C" fn resctrl_arch_set_cdp_enabled(l: resctrl_res_level, enable: bool) -> i32 {
    let hw_res = &mut rdt_resources_all[l as usize];
    if !hw_res.r_resctrl.cdp_capable { return -EINVAL; }
    if enable { return cdp_enable(l as i32); }
    cdp_disable(l as i32);
    0
}

pub unsafe extern "C" fn resctrl_arch_get_cdp_enabled(l: resctrl_res_level) -> bool {
    rdt_resources_all[l as usize].cdp_enabled
}

pub unsafe extern "C" fn resctrl_arch_reset_all_ctrls(r: *mut rdt_resource) {
    let hw_res = resctrl_to_arch_res(r);
    let mut msr_param: msr_param = core::mem::zeroed();
    let mut d: *mut rdt_ctrl_domain = core::ptr::null_mut();
    lockdep_assert_cpus_held!();
    msr_param.res = r;
    msr_param.low = 0;
    msr_param.high = (*hw_res).num_closid;

    list_for_each_entry_rcu!(d, &(*r).ctrl_domains, hdr.list, lockdep_is_cpus_held(), {
        let hw_dom = resctrl_to_arch_ctrl_dom(d);
        for i in 0..(*hw_res).num_closid as usize {
            (*hw_dom).ctrl_val[i] = resctrl_get_default_ctrl(r);
        }
        msr_param.dom = d;
        smp_call_function_any(&(*d).hdr.cpu_mask, rdt_ctrl_update, &mut msr_param, 1);
    });
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
