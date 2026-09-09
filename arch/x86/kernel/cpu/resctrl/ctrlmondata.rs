// SPDX-License-Identifier: GPL-2.0-only
/*
 * Resource Director Technology(RDT)
 * - Cache Allocation code.
 *
 * Copyright (C) 2016 Intel Corporation
 *
 * Authors:
 *    Fenghua Yu <fenghua.yu@intel.com>
 *    Tony Luck <tony.luck@intel.com>
 *
 * More information about RDT be found in the Intel (R) x86 Architecture
 * Software Developer Manual June 2016, volume 3, section 17.17.
 */

// pr_fmt(fmt) KBUILD_MODNAME ": " fmt

pub unsafe fn resctrl_arch_update_one(
    r: *mut rdt_resource,
    d: *mut rdt_ctrl_domain,
    closid: u32,
    t: resctrl_conf_type,
    cfg_val: u32,
) -> i32 {
    let hw_dom: *mut rdt_hw_ctrl_domain = resctrl_to_arch_ctrl_dom(d);
    let hw_res: *mut rdt_hw_resource = resctrl_to_arch_res(r);
    let idx: u32 = resctrl_get_config_index(closid, t);
    let mut msr_param: msr_param = core::mem::zeroed();

    if !cpumask_test_cpu(smp_processor_id(), &(*d).hdr.cpu_mask) {
        return -EINVAL;
    }

    (*hw_dom).ctrl_val[idx as usize] = cfg_val;

    msr_param.res = r;
    msr_param.dom = d;
    msr_param.low = idx;
    msr_param.high = idx + 1;
    ((*hw_res).msr_update)(&mut msr_param);

    0
}

pub unsafe fn resctrl_arch_update_domains(r: *mut rdt_resource, closid: u32) -> i32 {
    let mut cfg: *mut resctrl_staged_config;
    let mut hw_dom: *mut rdt_hw_ctrl_domain;
    let mut msr_param: msr_param = core::mem::zeroed();
    let mut d: *mut rdt_ctrl_domain;
    let mut t: resctrl_conf_type;
    let mut idx: u32;

    // Walking r->domains, ensure it can't race with cpuhp
    lockdep_assert_cpus_held();

    list_for_each_entry_rcu!(d, &(*r).ctrl_domains, hdr.list, lockdep_is_cpus_held());
    {
        hw_dom = resctrl_to_arch_ctrl_dom(d);
        msr_param.res = core::ptr::null_mut();
        t = 0;
        while t < CDP_NUM_TYPES {
            cfg = &mut (*hw_dom).d_resctrl.staged_config[t as usize];
            if !(*cfg).have_new_ctrl {
                t += 1;
                continue;
            }

            idx = resctrl_get_config_index(closid, t);
            if (*cfg).new_ctrl == (*hw_dom).ctrl_val[idx as usize] {
                t += 1;
                continue;
            }
            (*hw_dom).ctrl_val[idx as usize] = (*cfg).new_ctrl;

            if msr_param.res.is_null() {
                msr_param.low = idx;
                msr_param.high = msr_param.low + 1;
                msr_param.res = r;
                msr_param.dom = d;
            } else {
                msr_param.low = core::cmp::min(msr_param.low, idx);
                msr_param.high = core::cmp::max(msr_param.high, idx + 1);
            }
            t += 1;
        }
        if !msr_param.res.is_null() {
            smp_call_function_any(&(*d).hdr.cpu_mask, rdt_ctrl_update, &mut msr_param, 1);
        }
    }

    0
}

pub unsafe fn resctrl_arch_get_config(
    r: *mut rdt_resource,
    d: *mut rdt_ctrl_domain,
    closid: u32,
    type_: resctrl_conf_type,
) -> u32 {
    let hw_dom: *mut rdt_hw_ctrl_domain = resctrl_to_arch_ctrl_dom(d);
    let idx: u32 = resctrl_get_config_index(closid, type_);

    (*hw_dom).ctrl_val[idx as usize]
}

pub unsafe fn resctrl_arch_get_io_alloc_enabled(r: *mut rdt_resource) -> bool {
    (*resctrl_to_arch_res(r)).sdciae_enabled
}

unsafe fn resctrl_sdciae_set_one_amd(arg: *mut core::ffi::c_void) {
    let enable: *mut bool = arg as *mut bool;

    if *enable {
        msr_set_bit(MSR_IA32_L3_QOS_EXT_CFG, SDCIAE_ENABLE_BIT);
    } else {
        msr_clear_bit(MSR_IA32_L3_QOS_EXT_CFG, SDCIAE_ENABLE_BIT);
    }
}

unsafe fn _resctrl_sdciae_enable(r: *mut rdt_resource, enable: bool) {
    let mut d: *mut rdt_ctrl_domain;

    // Walking r->ctrl_domains, ensure it can't race with cpuhp
    lockdep_assert_cpus_held();

    // Update MSR_IA32_L3_QOS_EXT_CFG MSR on all the CPUs in all domains
    list_for_each_entry_rcu!(d, &(*r).ctrl_domains, hdr.list, lockdep_is_cpus_held());
    {
        on_each_cpu_mask(&(*d).hdr.cpu_mask, resctrl_sdciae_set_one_amd, &enable, 1);
    }
}

pub unsafe fn resctrl_arch_io_alloc_enable(r: *mut rdt_resource, enable: bool) -> i32 {
    let hw_res: *mut rdt_hw_resource = resctrl_to_arch_res(r);

    if (*hw_res).r_resctrl.cache.io_alloc_capable
        && (*hw_res).sdciae_enabled != enable
    {
        _resctrl_sdciae_enable(r, enable);
        (*hw_res).sdciae_enabled = enable;
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
