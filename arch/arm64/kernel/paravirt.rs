// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 * Copyright (C) 2013 Citrix Systems
 *
 * Author: Stefano Stabellini <stefano.stabellini@eu.citrix.com>
 */

// pr_fmt(fmt) = "arm-pv: " fmt

#[repr(C)]
pub struct pv_time_stolen_time_region {
    pub kaddr: *mut pvclock_vcpu_stolen_time,
}

// DEFINE_PER_CPU(struct pv_time_stolen_time_region, stolen_time_region)
static mut stolen_time_region: pv_time_stolen_time_region = pv_time_stolen_time_region {
    kaddr: core::ptr::null_mut(),
};

static mut steal_acc: bool = true;

// early_param("no-steal-acc", parse_no_stealacc)
unsafe extern "C" fn parse_no_stealacc(_arg: *mut core::ffi::c_char) -> i32 {
    steal_acc = false;
    0
}

/* return stolen time in ns by asking the hypervisor */
unsafe fn para_steal_clock(cpu: i32) -> u64 {
    let mut kaddr: *mut pvclock_vcpu_stolen_time = core::ptr::null_mut();
    let reg: *mut pv_time_stolen_time_region = per_cpu_ptr(&raw mut stolen_time_region, cpu);
    let mut ret: u64 = 0;

    /*
     * paravirt_steal_clock() may be called before the CPU
     * online notification callback runs. Until the callback
     * has run we just return zero.
     */
    rcu_read_lock();
    kaddr = rcu_dereference((*reg).kaddr);
    if kaddr.is_null() {
        rcu_read_unlock();
        return 0;
    }

    ret = le64_to_cpu(core::ptr::read_volatile(&(*kaddr).stolen_time));
    rcu_read_unlock();
    ret
}

unsafe extern "C" fn stolen_time_cpu_down_prepare(_cpu: u32) -> i32 {
    let mut kaddr: *mut pvclock_vcpu_stolen_time = core::ptr::null_mut();
    let reg: *mut pv_time_stolen_time_region = this_cpu_ptr(&raw mut stolen_time_region);

    if (*reg).kaddr.is_null() {
        return 0;
    }

    kaddr = rcu_replace_pointer(&mut (*reg).kaddr, core::ptr::null_mut(), true);
    synchronize_rcu();
    memunmap(kaddr as *mut core::ffi::c_void);

    0
}

unsafe extern "C" fn stolen_time_cpu_online(_cpu: u32) -> i32 {
    let mut kaddr: *mut pvclock_vcpu_stolen_time = core::ptr::null_mut();
    let reg: *mut pv_time_stolen_time_region = this_cpu_ptr(&raw mut stolen_time_region);
    let mut res = arm_smccc_res::default();

    arm_smccc_1_1_invoke(ARM_SMCCC_HV_PV_TIME_ST, &mut res);

    if res.a0 == SMCCC_RET_NOT_SUPPORTED {
        return -EINVAL;
    }

    kaddr = memremap(
        res.a0,
        core::mem::size_of::<pvclock_vcpu_stolen_time>(),
        MEMREMAP_WB,
    ) as *mut pvclock_vcpu_stolen_time;

    rcu_assign_pointer(&mut (*reg).kaddr, kaddr);

    if (*reg).kaddr.is_null() {
        pr_warn!("Failed to map stolen time data structure\n");
        return -ENOMEM;
    }

    if le32_to_cpu((*kaddr).revision) != 0 || le32_to_cpu((*kaddr).attributes) != 0 {
        pr_warn_once!("Unexpected revision or attributes in stolen time data\n");
        return -ENXIO;
    }

    0
}

unsafe fn pv_time_init_stolen_time() -> i32 {
    let ret = cpuhp_setup_state(
        CPUHP_AP_ONLINE_DYN,
        "hypervisor/arm/pvtime:online\0".as_ptr() as *const core::ffi::c_char,
        Some(stolen_time_cpu_online),
        Some(stolen_time_cpu_down_prepare),
    );
    if ret < 0 {
        return ret;
    }
    0
}

unsafe fn has_pv_steal_clock() -> bool {
    let mut res = arm_smccc_res::default();

    arm_smccc_1_1_invoke(ARM_SMCCC_ARCH_FEATURES_FUNC_ID, ARM_SMCCC_HV_PV_TIME_FEATURES, &mut res);

    if res.a0 != SMCCC_RET_SUCCESS {
        return false;
    }

    arm_smccc_1_1_invoke(ARM_SMCCC_HV_PV_TIME_FEATURES, ARM_SMCCC_HV_PV_TIME_ST, &mut res);

    res.a0 == SMCCC_RET_SUCCESS
}

pub unsafe fn pv_time_init() -> i32 {
    let ret: i32;

    if !has_pv_steal_clock() {
        return 0;
    }

    ret = pv_time_init_stolen_time();
    if ret != 0 {
        return ret;
    }

    static_call_update(pv_steal_clock, para_steal_clock);

    static_key_slow_inc(&paravirt_steal_enabled);
    if steal_acc {
        static_key_slow_inc(&paravirt_steal_rq_enabled);
    }

    pr_info!("using stolen time PV\n");

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
