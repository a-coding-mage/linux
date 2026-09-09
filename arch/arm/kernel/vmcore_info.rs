// SPDX-License-Identifier: GPL-2.0-only

// Dependency: <linux/vmcore_info.h>

pub unsafe fn arch_crash_save_vmcoreinfo() {
    // Preserves the source build-time condition: CONFIG_ARM_LPAE.
    #[cfg(feature = "CONFIG_ARM_LPAE")]
    {
        VMCOREINFO_CONFIG!(ARM_LPAE);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
