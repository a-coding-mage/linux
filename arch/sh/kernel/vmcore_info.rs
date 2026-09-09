// SPDX-License-Identifier: GPL-2.0-only

// The declarations and macros below are supplied by the corresponding kernel
// dependencies.

pub unsafe fn arch_crash_save_vmcoreinfo() {
    #[cfg(CONFIG_NUMA)]
    {
        VMCOREINFO_SYMBOL!(node_data);
        VMCOREINFO_LENGTH!(node_data, MAX_NUMNODES);
    }

    #[cfg(CONFIG_X2TLB)]
    {
        VMCOREINFO_CONFIG!(X2TLB);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
