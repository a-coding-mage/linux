/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Defines VMX CPU feature bits
 */
pub const NVMXINTS: i32 = 5; /* N 32-bit words worth of info */

/*
 * Note: If the comment begins with a quoted string, that string is used
 * in /proc/cpuinfo instead of the macro name.  Otherwise, this feature bit
 * is not displayed in /proc/cpuinfo at all.
 */

/* Pin-Based VM-Execution Controls, EPT/VPID, APIC and VM-Functions, word 0 */
pub const VMX_FEATURE_INTR_EXITING: i32 = 0 * 32 + 0; /* VM-Exit on vectored interrupts */
pub const VMX_FEATURE_NMI_EXITING: i32 = 0 * 32 + 3; /* VM-Exit on NMIs */
pub const VMX_FEATURE_VIRTUAL_NMIS: i32 = 0 * 32 + 5; /* "vnmi" NMI virtualization */
pub const VMX_FEATURE_PREEMPTION_TIMER: i32 = 0 * 32 + 6; /* "preemption_timer" VMX Preemption Timer */
pub const VMX_FEATURE_POSTED_INTR: i32 = 0 * 32 + 7; /* "posted_intr" Posted Interrupts */

/* EPT/VPID features, scattered to bits 16-23 */
pub const VMX_FEATURE_INVVPID: i32 = 0 * 32 + 16; /* "invvpid" INVVPID is supported */
pub const VMX_FEATURE_EPT_EXECUTE_ONLY: i32 = 0 * 32 + 17; /* "ept_x_only" EPT entries can be execute only */
pub const VMX_FEATURE_EPT_AD: i32 = 0 * 32 + 18; /* "ept_ad" EPT Accessed/Dirty bits */
pub const VMX_FEATURE_EPT_1GB: i32 = 0 * 32 + 19; /* "ept_1gb" 1GB EPT pages */
pub const VMX_FEATURE_EPT_5LEVEL: i32 = 0 * 32 + 20; /* "ept_5level" 5-level EPT paging */

/* Aggregated APIC features 24-27 */
pub const VMX_FEATURE_FLEXPRIORITY: i32 = 0 * 32 + 24; /* "flexpriority" TPR shadow + virt APIC */
pub const VMX_FEATURE_APICV: i32 = 0 * 32 + 25; /* "apicv" TPR shadow + APIC reg virt + virt intr delivery + posted interrupts */

/* VM-Functions, shifted to bits 28-31 */
pub const VMX_FEATURE_EPTP_SWITCHING: i32 = 0 * 32 + 28; /* "eptp_switching" EPTP switching (in guest) */

/* Primary Processor-Based VM-Execution Controls, word 1 */
pub const VMX_FEATURE_INTR_WINDOW_EXITING: i32 = 1 * 32 + 2; /* VM-Exit if INTRs are unblocked in guest */
pub const VMX_FEATURE_USE_TSC_OFFSETTING: i32 = 1 * 32 + 3; /* "tsc_offset" Offset hardware TSC when read in guest */
pub const VMX_FEATURE_HLT_EXITING: i32 = 1 * 32 + 7; /* VM-Exit on HLT */
pub const VMX_FEATURE_INVLPG_EXITING: i32 = 1 * 32 + 9; /* VM-Exit on INVLPG */
pub const VMX_FEATURE_MWAIT_EXITING: i32 = 1 * 32 + 10; /* VM-Exit on MWAIT */
pub const VMX_FEATURE_RDPMC_EXITING: i32 = 1 * 32 + 11; /* VM-Exit on RDPMC */
pub const VMX_FEATURE_RDTSC_EXITING: i32 = 1 * 32 + 12; /* VM-Exit on RDTSC */
pub const VMX_FEATURE_CR3_LOAD_EXITING: i32 = 1 * 32 + 15; /* VM-Exit on writes to CR3 */
pub const VMX_FEATURE_CR3_STORE_EXITING: i32 = 1 * 32 + 16; /* VM-Exit on reads from CR3 */
pub const VMX_FEATURE_TERTIARY_CONTROLS: i32 = 1 * 32 + 17; /* Enable Tertiary VM-Execution Controls */
pub const VMX_FEATURE_CR8_LOAD_EXITING: i32 = 1 * 32 + 19; /* VM-Exit on writes to CR8 */
pub const VMX_FEATURE_CR8_STORE_EXITING: i32 = 1 * 32 + 20; /* VM-Exit on reads from CR8 */
pub const VMX_FEATURE_VIRTUAL_TPR: i32 = 1 * 32 + 21; /* "vtpr" TPR virtualization, a.k.a. TPR shadow */
pub const VMX_FEATURE_NMI_WINDOW_EXITING: i32 = 1 * 32 + 22; /* VM-Exit if NMIs are unblocked in guest */
pub const VMX_FEATURE_MOV_DR_EXITING: i32 = 1 * 32 + 23; /* VM-Exit on accesses to debug registers */
pub const VMX_FEATURE_UNCOND_IO_EXITING: i32 = 1 * 32 + 24; /* VM-Exit on *all* IN{S} and OUT{S}*/
pub const VMX_FEATURE_USE_IO_BITMAPS: i32 = 1 * 32 + 25; /* VM-Exit based on I/O port */
pub const VMX_FEATURE_MONITOR_TRAP_FLAG: i32 = 1 * 32 + 27; /* "mtf" VMX single-step VM-Exits */
pub const VMX_FEATURE_USE_MSR_BITMAPS: i32 = 1 * 32 + 28; /* VM-Exit based on MSR index */
pub const VMX_FEATURE_MONITOR_EXITING: i32 = 1 * 32 + 29; /* VM-Exit on MONITOR (MWAIT's accomplice) */
pub const VMX_FEATURE_PAUSE_EXITING: i32 = 1 * 32 + 30; /* VM-Exit on PAUSE (unconditionally) */
pub const VMX_FEATURE_SEC_CONTROLS: i32 = 1 * 32 + 31; /* Enable Secondary VM-Execution Controls */

/* Secondary Processor-Based VM-Execution Controls, word 2 */
pub const VMX_FEATURE_VIRT_APIC_ACCESSES: i32 = 2 * 32 + 0; /* "vapic" Virtualize memory mapped APIC accesses */
pub const VMX_FEATURE_EPT: i32 = 2 * 32 + 1; /* "ept" Extended Page Tables, a.k.a. Two-Dimensional Paging */
pub const VMX_FEATURE_DESC_EXITING: i32 = 2 * 32 + 2; /* VM-Exit on {S,L}*DT instructions */
pub const VMX_FEATURE_RDTSCP: i32 = 2 * 32 + 3; /* Enable RDTSCP in guest */
pub const VMX_FEATURE_VIRTUAL_X2APIC: i32 = 2 * 32 + 4; /* Virtualize X2APIC for the guest */
pub const VMX_FEATURE_VPID: i32 = 2 * 32 + 5; /* "vpid" Virtual Processor ID (TLB ASID modifier) */
pub const VMX_FEATURE_WBINVD_EXITING: i32 = 2 * 32 + 6; /* VM-Exit on WBINVD */
pub const VMX_FEATURE_UNRESTRICTED_GUEST: i32 = 2 * 32 + 7; /* "unrestricted_guest" Allow Big Real Mode and other "invalid" states */
pub const VMX_FEATURE_APIC_REGISTER_VIRT: i32 = 2 * 32 + 8; /* "vapic_reg" Hardware emulation of reads to the virtual-APIC */
pub const VMX_FEATURE_VIRT_INTR_DELIVERY: i32 = 2 * 32 + 9; /* "vid" Evaluation and delivery of pending virtual interrupts */
pub const VMX_FEATURE_PAUSE_LOOP_EXITING: i32 = 2 * 32 + 10; /* "ple" Conditionally VM-Exit on PAUSE at CPL0 */
pub const VMX_FEATURE_RDRAND_EXITING: i32 = 2 * 32 + 11; /* VM-Exit on RDRAND*/
pub const VMX_FEATURE_INVPCID: i32 = 2 * 32 + 12; /* Enable INVPCID in guest */
pub const VMX_FEATURE_VMFUNC: i32 = 2 * 32 + 13; /* Enable VM-Functions (leaf dependent) */
pub const VMX_FEATURE_SHADOW_VMCS: i32 = 2 * 32 + 14; /* "shadow_vmcs" VMREAD/VMWRITE in guest can access shadow VMCS */
pub const VMX_FEATURE_ENCLS_EXITING: i32 = 2 * 32 + 15; /* VM-Exit on ENCLS (leaf dependent) */
pub const VMX_FEATURE_RDSEED_EXITING: i32 = 2 * 32 + 16; /* VM-Exit on RDSEED */
pub const VMX_FEATURE_PAGE_MOD_LOGGING: i32 = 2 * 32 + 17; /* "pml" Log dirty pages into buffer */
pub const VMX_FEATURE_EPT_VIOLATION_VE: i32 = 2 * 32 + 18; /* "ept_violation_ve" Conditionally reflect EPT violations as #VE exceptions */
pub const VMX_FEATURE_PT_CONCEAL_VMX: i32 = 2 * 32 + 19; /* Suppress VMX indicators in Processor Trace */
pub const VMX_FEATURE_XSAVES: i32 = 2 * 32 + 20; /* Enable XSAVES and XRSTORS in guest */
pub const VMX_FEATURE_MODE_BASED_EPT_EXEC: i32 = 2 * 32 + 22; /* "ept_mode_based_exec" Enable separate EPT EXEC bits for supervisor vs. user */
pub const VMX_FEATURE_PT_USE_GPA: i32 = 2 * 32 + 24; /* Processor Trace logs GPAs */
pub const VMX_FEATURE_TSC_SCALING: i32 = 2 * 32 + 25; /* "tsc_scaling" Scale hardware TSC when read in guest */
pub const VMX_FEATURE_USR_WAIT_PAUSE: i32 = 2 * 32 + 26; /* "usr_wait_pause" Enable TPAUSE, UMONITOR, UMWAIT in guest */
pub const VMX_FEATURE_ENCLV_EXITING: i32 = 2 * 32 + 28; /* VM-Exit on ENCLV (leaf dependent) */
pub const VMX_FEATURE_BUS_LOCK_DETECTION: i32 = 2 * 32 + 30; /* VM-Exit when bus lock caused */
pub const VMX_FEATURE_NOTIFY_VM_EXITING: i32 = 2 * 32 + 31; /* "notify_vm_exiting" VM-Exit when no event windows after notify window */

/* Tertiary Processor-Based VM-Execution Controls, word 3 */
pub const VMX_FEATURE_IPI_VIRT: i32 = 3 * 32 + 4; /* "ipi_virt" Enable IPI virtualization */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
