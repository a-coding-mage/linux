// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (C) 2001 Ben. Herrenschmidt (benh@kernel.crashing.org)
 *
 *  Modifications for ppc64:
 *      Copyright (C) 2003 Dave Engebretsen <engebret@us.ibm.com>
 */

// C dependencies supplied by the surrounding kernel translation unit.

static mut THE_CPU_SPEC: cpu_spec = unsafe { core::mem::zeroed() };

pub static mut cur_cpu_spec: *mut cpu_spec = core::ptr::null_mut();

/* The platform string corresponding to the real PVR */
pub static mut powerpc_base_platform: *const core::ffi::c_char = core::ptr::null();

extern "C" {
    static mut cpu_specs: *mut cpu_spec;
    fn static_branch_disable(key: *mut static_key_true);
}

pub unsafe fn set_cur_cpu_spec(s: *const cpu_spec) {
    let t: *mut cpu_spec = &raw mut THE_CPU_SPEC;

    /* Use memcpy semantics, as in the original implementation. */
    core::ptr::copy_nonoverlapping(s, t, 1);

    cur_cpu_spec = &raw mut THE_CPU_SPEC;
}

unsafe fn setup_cpu_spec(offset: usize, s: *mut cpu_spec) -> *mut cpu_spec {
    let t: *mut cpu_spec = &raw mut THE_CPU_SPEC;
    let mut old: cpu_spec = core::mem::zeroed();

    core::ptr::copy_nonoverlapping(t, &mut old, 1);

    /* Copy everything, then do fixups. */
    core::ptr::copy_nonoverlapping(s, t, 1);

    /*
     * If we are overriding a previous value derived from the real
     * PVR with a new value obtained using a logical PVR value,
     * don't modify the performance monitor fields.
     */
    if old.num_pmcs != 0 && (*s).num_pmcs == 0 {
        (*t).num_pmcs = old.num_pmcs;
        (*t).pmc_type = old.pmc_type;

        /* Ensure that the fix for the PMAO bug is enabled on compatibility mode. */
        (*t).cpu_features |= old.cpu_features & CPU_FTR_PMAO_BUG;
    }

    /* CONFIG_PPC_KUAP && CONFIG_PPC32: set KUAP on at startup. */
    #[cfg(all(CONFIG_PPC_KUAP, CONFIG_PPC32))]
    {
        (*t).mmu_features |= MMU_FTR_KUAP;
    }

    cur_cpu_spec = &raw mut THE_CPU_SPEC;

    /* Set the base platform string once; real PVR is expected first. */
    if powerpc_base_platform.is_null() {
        powerpc_base_platform = (*t).platform;
    }

    /* CONFIG_PPC64 || CONFIG_BOOKE */
    #[cfg(any(CONFIG_PPC64, CONFIG_BOOKE))]
    if let Some(cpu_setup) = (*t).cpu_setup {
        cpu_setup(offset, t);
    }

    t
}

pub unsafe fn identify_cpu(offset: usize, pvr: u32) -> *mut cpu_spec {
    let mut s = cpu_specs;
    let count = ARRAY_SIZE_CPU_SPECS;

    // BUILD_BUG_ON(!ARRAY_SIZE(cpu_specs));
    for _i in 0..count {
        if (pvr as usize & (*s).pvr_mask) == (*s).pvr_value {
            return setup_cpu_spec(offset, s);
        }
        s = s.add(1);
    }

    BUG!();
    core::ptr::null_mut()
}

/*
 * Used by cpufeatures to get the name for CPUs with a PVR table.
 * If they don't have a PVR table, cpufeatures gets the name from
 * the CPU device-tree node.
 */
pub unsafe fn identify_cpu_name(pvr: u32) {
    let mut s = cpu_specs;
    let t: *mut cpu_spec = &raw mut THE_CPU_SPEC;

    for _i in 0..ARRAY_SIZE_CPU_SPECS {
        if (pvr as usize & (*s).pvr_mask) == (*s).pvr_value {
            (*t).cpu_name = (*s).cpu_name;
            return;
        }
        s = s.add(1);
    }
}

/* CONFIG_JUMP_LABEL_FEATURE_CHECKS */
#[cfg(CONFIG_JUMP_LABEL_FEATURE_CHECKS)]
pub static mut cpu_feature_keys: [static_key_true; NUM_CPU_FTR_KEYS] =
    [STATIC_KEY_TRUE_INIT; NUM_CPU_FTR_KEYS];

#[cfg(CONFIG_JUMP_LABEL_FEATURE_CHECKS)]
pub unsafe fn cpu_feature_keys_init() {
    for i in 0..NUM_CPU_FTR_KEYS {
        let f: usize = 1usize << i;
        if (*cur_cpu_spec).cpu_features & f == 0 {
            static_branch_disable(&raw mut cpu_feature_keys[i]);
        }
    }
}

#[cfg(CONFIG_JUMP_LABEL_FEATURE_CHECKS)]
pub static mut mmu_feature_keys: [static_key_true; NUM_MMU_FTR_KEYS] =
    [STATIC_KEY_TRUE_INIT; NUM_MMU_FTR_KEYS];

#[cfg(CONFIG_JUMP_LABEL_FEATURE_CHECKS)]
pub unsafe fn mmu_feature_keys_init() {
    for i in 0..NUM_MMU_FTR_KEYS {
        let f: usize = 1usize << i;
        if (*cur_cpu_spec).mmu_features & f == 0 {
            static_branch_disable(&raw mut mmu_feature_keys[i]);
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
