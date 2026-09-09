// SPDX-License-Identifier: GPL-2.0
/*
 * Intel Performance and Energy Bias Hint support.
 *
 * Copyright (C) 2019 Intel Corporation
 *
 * Author:
 *	Rafael J. Wysocki <rafael.j.wysocki@intel.com>
 */

/* Kernel dependencies supplied by other translation units. */

static mut SAVED_EPB: u8 = 0;

const EPB_MASK: u64 = 0x0f_u64;
const EPB_SAVED: u64 = 0x10_u64;
const MAX_EPB: u64 = EPB_MASK;

#[repr(usize)]
enum EnergyPerfValueIndex {
    EpbIndexPerformance,
    EpbIndexBalancePerformance,
    EpbIndexNormal,
    EpbIndexBalancePowersave,
    EpbIndexPowersave,
}

static mut ENERG_PERF_VALUES: [u8; 5] = [
    ENERGY_PERF_BIAS_PERFORMANCE,
    ENERGY_PERF_BIAS_BALANCE_PERFORMANCE,
    ENERGY_PERF_BIAS_NORMAL,
    ENERGY_PERF_BIAS_BALANCE_POWERSAVE,
    ENERGY_PERF_BIAS_POWERSAVE,
];

unsafe fn intel_epb_save(_data: *mut core::ffi::c_void) -> i32 {
    let mut epb: u64 = 0;

    rdmsrq(MSR_IA32_ENERGY_PERF_BIAS, &mut epb);
    /*
     * Ensure that saved_epb will always be nonzero after this write even if
     * the EPB value read from the MSR is 0.
     */
    SAVED_EPB = ((epb & EPB_MASK) | EPB_SAVED) as u8;

    0
}

unsafe fn intel_epb_restore(_data: *mut core::ffi::c_void) {
    let mut val = SAVED_EPB as u64;
    let mut epb: u64 = 0;

    rdmsrq(MSR_IA32_ENERGY_PERF_BIAS, &mut epb);
    if val != 0 {
        val &= EPB_MASK;
    } else {
        /*
         * Because intel_epb_save() has not run for the current CPU yet,
         * it is going online for the first time, so if its EPB value is
         * 0 ('performance') at this point, assume that it has not been
         * initialized by the platform firmware and set it to 6
         * ('normal').
         */
        val = epb & EPB_MASK;
        if val == ENERGY_PERF_BIAS_PERFORMANCE as u64 {
            val = ENERG_PERF_VALUES[EnergyPerfValueIndex::EpbIndexNormal as usize] as u64;
            pr_warn_once!("ENERGY_PERF_BIAS: Set to 'normal', was 'performance'\n");
        }
    }
    wrmsrq(MSR_IA32_ENERGY_PERF_BIAS, (epb & !EPB_MASK) | val);
}

static INTEL_EPB_SYSCORE_OPS: SyscoreOps = SyscoreOps {
    suspend: Some(intel_epb_save),
    resume: Some(intel_epb_restore),
};

static mut INTEL_EPB_SYSCORE: Syscore = Syscore {
    ops: &INTEL_EPB_SYSCORE_OPS,
};

static ENERGY_PERF_STRINGS: [&[u8]; 5] = [
    b"performance\0",
    b"balance-performance\0",
    b"normal\0",
    b"balance-power\0",
    b"power\0",
];

unsafe fn energy_perf_bias_show(
    dev: *mut Device,
    _attr: *mut DeviceAttribute,
    buf: *mut core::ffi::c_char,
) -> isize {
    let cpu = (*dev).id;
    let mut epb: u64 = 0;
    let ret = rdmsrq_on_cpu(cpu, MSR_IA32_ENERGY_PERF_BIAS, &mut epb);
    if ret < 0 {
        return ret as isize;
    }
    sprintf(buf, b"%llu\n\0".as_ptr() as *const core::ffi::c_char, epb) as isize
}

unsafe fn energy_perf_bias_store(
    dev: *mut Device,
    _attr: *mut DeviceAttribute,
    buf: *const core::ffi::c_char,
    count: usize,
) -> isize {
    let cpu = (*dev).id;
    let mut epb: u64 = 0;
    let val: u64;
    let ret = __sysfs_match_string(
        ENERGY_PERF_STRINGS.as_ptr(),
        ENERGY_PERF_STRINGS.len(),
        buf,
    );
    if ret >= 0 {
        val = ENERG_PERF_VALUES[ret as usize] as u64;
    } else {
        let mut parsed: u64 = 0;
        if kstrtou64(buf, 0, &mut parsed) != 0 || parsed > MAX_EPB {
            return -EINVAL as isize;
        }
        val = parsed;
    }

    let ret = rdmsrq_on_cpu(cpu, MSR_IA32_ENERGY_PERF_BIAS, &mut epb);
    if ret < 0 {
        return ret as isize;
    }
    let ret = wrmsrq_on_cpu(cpu, MSR_IA32_ENERGY_PERF_BIAS, (epb & !EPB_MASK) | val);
    if ret < 0 {
        return ret as isize;
    }
    count as isize
}

/* DEVICE_ATTR_RW(energy_perf_bias); */
static mut INTEL_EPB_ATTRS: [*mut Attribute; 2] = [
    &mut DEV_ATTR_ENERGY_PERF_BIAS.attr,
    core::ptr::null_mut(),
];

static INTEL_EPB_ATTR_GROUP: AttributeGroup = AttributeGroup {
    name: POWER_GROUP_NAME,
    attrs: unsafe { INTEL_EPB_ATTRS.as_ptr() },
};

unsafe fn intel_epb_online(cpu: u32) -> i32 {
    let cpu_dev = get_cpu_device(cpu);

    intel_epb_restore(core::ptr::null_mut());
    if !CPUHP_TASKS_FROZEN {
        sysfs_merge_group((*cpu_dev).kobj, &INTEL_EPB_ATTR_GROUP);
    }

    0
}

unsafe fn intel_epb_offline(cpu: u32) -> i32 {
    let cpu_dev = get_cpu_device(cpu);

    if !CPUHP_TASKS_FROZEN {
        sysfs_unmerge_group((*cpu_dev).kobj, &INTEL_EPB_ATTR_GROUP);
    }

    intel_epb_save(core::ptr::null_mut());
    0
}

static INTEL_EPB_NORMAL: [X86CpuId; 4] = [
    X86_MATCH_VFM(INTEL_ALDERLAKE_L, ENERGY_PERF_BIAS_NORMAL_POWERSAVE),
    X86_MATCH_VFM(INTEL_ATOM_GRACEMONT, ENERGY_PERF_BIAS_NORMAL_POWERSAVE),
    X86_MATCH_VFM(INTEL_RAPTORLAKE_P, ENERGY_PERF_BIAS_NORMAL_POWERSAVE),
    X86_CPU_ID_EMPTY,
];

unsafe fn intel_epb_init() -> i32 {
    let id = x86_match_cpu(INTEL_EPB_NORMAL.as_ptr());
    let ret: i32;

    if !boot_cpu_has(X86_FEATURE_EPB) {
        return -ENODEV;
    }

    if !id.is_null() {
        ENERG_PERF_VALUES[EnergyPerfValueIndex::EpbIndexNormal as usize] = (*id).driver_data as u8;
    }

    ret = cpuhp_setup_state(
        CPUHP_AP_X86_INTEL_EPB_ONLINE,
        b"x86/intel/epb:online\0".as_ptr() as *const core::ffi::c_char,
        Some(intel_epb_online),
        Some(intel_epb_offline),
    );
    if ret < 0 {
        cpuhp_remove_state(CPUHP_AP_X86_INTEL_EPB_ONLINE);
        return ret;
    }

    register_syscore(&mut INTEL_EPB_SYSCORE);
    0
}

/* late_initcall(intel_epb_init); */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
