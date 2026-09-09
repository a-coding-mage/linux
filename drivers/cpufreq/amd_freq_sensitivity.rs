// SPDX-License-Identifier: GPL-2.0-only
/*
 * amd_freq_sensitivity.c: AMD frequency sensitivity feedback powersave bias
 *                         for the ondemand governor.
 *
 * Copyright (C) 2013 Advanced Micro Devices, Inc.
 *
 * Author: Jacob Shin <jacob.shin@amd.com>
 */

// Linux and architecture dependencies are supplied by the surrounding kernel
// translation unit.

const MSR_AMD64_FREQ_SENSITIVITY_ACTUAL: u32 = 0xc0010080;
const MSR_AMD64_FREQ_SENSITIVITY_REFERENCE: u32 = 0xc0010081;
const CLASS_CODE_SHIFT: u32 = 56;
const POWERSAVE_BIAS_MAX: i32 = 1000;
const POWERSAVE_BIAS_DEF: u32 = 400;

#[repr(C)]
struct CpuDataT {
    actual: u64,
    reference: u64,
    freq_prev: u32,
}

// DEFINE_PER_CPU(struct cpu_data_t, cpu_data);
extern "C" {
    static mut cpu_data: CpuDataT;
}

unsafe fn amd_powersave_bias_target(
    policy: *mut cpufreq_policy,
    mut freq_next: u32,
    relation: u32,
) -> u32 {
    let mut sensitivity: i32;
    let mut d_actual: i64;
    let mut d_reference: i64;
    let mut actual = msr { q: 0 };
    let mut reference = msr { q: 0 };
    let data: *mut CpuDataT = &mut cpu_data;
    let policy_dbs = (*policy).governor_data;
    let od_data = (*policy_dbs).dbs_data;
    let od_tuners = (*od_data).tuners;

    if (*policy).freq_table.is_null() {
        return freq_next;
    }

    rdmsrq_on_cpu((*policy).cpu, MSR_AMD64_FREQ_SENSITIVITY_ACTUAL, &mut actual.q);
    rdmsrq_on_cpu((*policy).cpu, MSR_AMD64_FREQ_SENSITIVITY_REFERENCE, &mut reference.q);
    actual.h &= 0x00ffffff;
    reference.h &= 0x00ffffff;

    // counter wrapped around, so stay on current frequency
    if actual.q < (*data).actual || reference.q < (*data).reference {
        freq_next = (*policy).cur;
        return amd_powersave_bias_target_out(policy, freq_next, data, actual.q, reference.q);
    }

    d_actual = (actual.q - (*data).actual) as i64;
    d_reference = (reference.q - (*data).reference) as i64;

    // divide by 0, so stay on current frequency as well
    if d_reference == 0 {
        freq_next = (*policy).cur;
        return amd_powersave_bias_target_out(policy, freq_next, data, actual.q, reference.q);
    }

    sensitivity = POWERSAVE_BIAS_MAX
        - (POWERSAVE_BIAS_MAX * (d_reference - d_actual) / d_reference) as i32;
    sensitivity = sensitivity.clamp(0, POWERSAVE_BIAS_MAX);

    // this workload is not CPU bound, so choose a lower freq
    if sensitivity < (*od_tuners).powersave_bias {
        if (*data).freq_prev == (*policy).cur {
            freq_next = (*policy).cur;
        }

        if freq_next > (*policy).cur {
            freq_next = (*policy).cur;
        } else if freq_next < (*policy).cur {
            freq_next = (*policy).min;
        } else {
            let index = cpufreq_table_find_index_h(
                policy,
                (*policy).cur - 1,
                relation & CPUFREQ_RELATION_E,
            );
            freq_next = (*(*policy).freq_table.add(index as usize)).frequency;
        }

        (*data).freq_prev = freq_next;
    } else {
        (*data).freq_prev = 0;
    }

    amd_powersave_bias_target_out(policy, freq_next, data, actual.q, reference.q)
}

unsafe fn amd_powersave_bias_target_out(
    _policy: *mut cpufreq_policy,
    freq_next: u32,
    data: *mut CpuDataT,
    actual: u64,
    reference: u64,
) -> u32 {
    (*data).actual = actual;
    (*data).reference = reference;
    freq_next
}

unsafe extern "C" fn amd_freq_sensitivity_init() -> i32 {
    let mut val: u64 = 0;
    let mut pcidev: *mut pci_dev;
    let pci_vendor: u32;

    if boot_cpu_data.x86_vendor == X86_VENDOR_AMD {
        pci_vendor = PCI_VENDOR_ID_AMD;
    } else if boot_cpu_data.x86_vendor == X86_VENDOR_HYGON {
        pci_vendor = PCI_VENDOR_ID_HYGON;
    } else {
        return -ENODEV;
    }

    pcidev = pci_get_device(pci_vendor, PCI_DEVICE_ID_AMD_KERNCZ_SMBUS, core::ptr::null_mut());
    if pcidev.is_null() {
        if !boot_cpu_has(X86_FEATURE_PROC_FEEDBACK) {
            return -ENODEV;
        }
    } else {
        pci_dev_put(pcidev);
    }

    if rdmsrq_safe(MSR_AMD64_FREQ_SENSITIVITY_ACTUAL, &mut val) != 0 {
        return -ENODEV;
    }
    if (val >> CLASS_CODE_SHIFT) == 0 {
        return -ENODEV;
    }

    od_register_powersave_bias_handler(Some(amd_powersave_bias_target), POWERSAVE_BIAS_DEF);
    0
}

unsafe extern "C" fn amd_freq_sensitivity_exit() {
    od_unregister_powersave_bias_handler();
}

// late_initcall(amd_freq_sensitivity_init);
// module_exit(amd_freq_sensitivity_exit);

// X86_MATCH_FEATURE(X86_FEATURE_PROC_FEEDBACK, NULL), {}
// MODULE_DEVICE_TABLE(x86cpu, amd_freq_sensitivity_ids);
// MODULE_AUTHOR("Jacob Shin <jacob.shin@amd.com>");
// MODULE_DESCRIPTION("AMD frequency sensitivity feedback powersave bias for the ondemand governor.");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
