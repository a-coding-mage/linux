// SPDX-License-Identifier: GPL-2.0
//
// Dependencies corresponding to the C includes are supplied by other files.

/**
 * x86_match_cpu - match current CPU against an array of x86_cpu_ids
 * @match: Pointer to array of x86_cpu_ids. Last entry terminated with
 *         {}.
 *
 * Return the entry if the current CPU matches the entries in the
 * passed x86_cpu_id match table. Otherwise NULL. The match table
 * contains vendor (X86_VENDOR_*), family, model and feature bits or
 * respective wildcard entries.
 *
 * A typical table entry would be to match a specific CPU
 *
 * X86_MATCH_VFM_FEATURE(INTEL_BROADWELL, X86_FEATURE_ANY, NULL);
 *
 * Fields can be wildcarded with %X86_VENDOR_ANY, %X86_FAMILY_ANY,
 * %X86_MODEL_ANY, %X86_FEATURE_ANY (except for vendor)
 *
 * asm/cpu_device_id.h contains a set of useful macros which are shortcuts
 * for various common selections. The above can be shortened to:
 *
 * X86_MATCH_VFM(INTEL_BROADWELL, NULL);
 *
 * Arrays used to match for this should also be declared using
 * MODULE_DEVICE_TABLE(x86cpu, ...)
 *
 * This always matches against the boot cpu, assuming models and features are
 * consistent over all CPUs.
 */
pub unsafe fn x86_match_cpu(mut match_: *const x86_cpu_id) -> *const x86_cpu_id {
    let mut m: *const x86_cpu_id = match_;
    let c: *mut cpuinfo_x86 = &raw mut boot_cpu_data;

    while (*m).flags & X86_CPU_ID_FLAG_ENTRY_VALID != 0 {
        if (*m).vendor != X86_VENDOR_ANY && (*c).x86_vendor != (*m).vendor {
            m = m.add(1);
            continue;
        }
        if (*m).family != X86_FAMILY_ANY && (*c).x86 != (*m).family {
            m = m.add(1);
            continue;
        }
        if (*m).model != X86_MODEL_ANY && (*c).x86_model != (*m).model {
            m = m.add(1);
            continue;
        }
        if (*m).steppings != X86_STEPPING_ANY
            && (bit((*c).x86_stepping) & (*m).steppings) == 0
        {
            m = m.add(1);
            continue;
        }
        if (*m).platform_mask != X86_PLATFORM_ANY
            && (bit((*c).intel_platform_id) & (*m).platform_mask) == 0
        {
            m = m.add(1);
            continue;
        }
        if (*m).feature != X86_FEATURE_ANY && !cpu_has(c, (*m).feature) {
            m = m.add(1);
            continue;
        }
        if (*m).type != X86_CPU_TYPE_ANY && (*c).topo.cpu_type != (*m).type {
            m = m.add(1);
            continue;
        }
        return m;
    }
    core::ptr::null()
}

pub unsafe fn x86_match_min_microcode_rev(table: *const x86_cpu_id) -> bool {
    let res: *const x86_cpu_id = x86_match_cpu(table);

    if res.is_null() || (*res).driver_data > boot_cpu_data.microcode {
        return false;
    }

    true
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
