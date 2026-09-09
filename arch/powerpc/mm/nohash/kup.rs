// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * This file contains the routines for initializing kernel userspace protection
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

#[cfg(CONFIG_PPC_KUAP)]
pub unsafe fn setup_kuap(disabled: bool) {
    if disabled {
        if crate::smp_processor_id() == crate::boot_cpuid {
            (*crate::cur_cpu_spec).mmu_features &= !crate::MMU_FTR_KUAP;
        }
        return;
    }

    crate::pr_info("Activating Kernel Userspace Access Protection\n");

    crate::prevent_user_access(crate::KUAP_READ_WRITE);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
