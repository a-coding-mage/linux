// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the corresponding Linux headers are referenced
// through the VMCOREINFO macros below.

pub unsafe extern "C" fn arch_crash_save_vmcoreinfo() {
    // Equivalent of CONFIG_NUMA.
    #[cfg(CONFIG_NUMA)]
    {
        VMCOREINFO_SYMBOL!(node_data);
        VMCOREINFO_LENGTH!(node_data, MAX_NUMNODES);
    }

    // Equivalent of CONFIG_X86_PAE.
    #[cfg(CONFIG_X86_PAE)]
    {
        VMCOREINFO_CONFIG!(X86_PAE);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
