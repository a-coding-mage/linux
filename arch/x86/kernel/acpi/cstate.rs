// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2005 Intel Corporation
 * 	Venkatesh Pallipadi <venkatesh.pallipadi@intel.com>
 * 	- Added _PDC for SMP C-states on Intel CPUs
 */

/* Kernel dependencies are supplied by the surrounding translation unit. */

/*
 * Initialize bm_flags based on the CPU cache properties
 * On SMP it depends on cache configuration
 * - When cache is not shared among all CPUs, we flush cache
 *   before entering C3.
 * - When cache is shared among all CPUs, we use bm_check
 *   mechanism as in UP case
 *
 * This routine is called only after all the CPUs are online
 */
pub unsafe fn acpi_processor_power_init_bm_check(
    flags: *mut acpi_processor_flags,
    cpu: c_uint,
) {
    let c = &cpu_data(cpu);

    (*flags).bm_check = 0;
    if num_online_cpus() == 1 {
        (*flags).bm_check = 1;
    } else if c.x86_vendor == X86_VENDOR_INTEL {
        /*
         * Today all MP CPUs that support C3 share cache.
         * And caches should not be flushed by software while
         * entering C3 type state.
         */
        (*flags).bm_check = 1;
    }

    /*
     * On all recent Intel platforms, ARB_DISABLE is a nop.
     * So, set bm_control to zero to indicate that ARB_DISABLE
     * is not required while entering C3 type state.
     */
    if c.x86_vendor == X86_VENDOR_INTEL
        && (c.x86 > 15
            || (c.x86_vfm >= INTEL_CORE2_MEROM && c.x86_vfm <= INTEL_FAM6_LAST))
    {
        (*flags).bm_control = 0;
    }

    if c.x86_vendor == X86_VENDOR_CENTAUR {
        if c.x86 > 6 || (c.x86 == 6 && c.x86_model == 0x0f && c.x86_stepping >= 0x0e) {
            /*
             * For all recent Centaur CPUs, the ucode will make sure that each
             * core can keep cache coherence with each other while entering C3
             * type state. So, set bm_check to 1 to indicate that the kernel
             * doesn't need to execute a cache flush operation (WBINVD) when
             * entering C3 type state.
             */
            (*flags).bm_check = 1;
            /*
             * For all recent Centaur platforms, ARB_DISABLE is a nop.
             * Set bm_control to zero to indicate that ARB_DISABLE is
             * not required while entering C3 type state.
             */
            (*flags).bm_control = 0;
        }
    }

    if c.x86_vendor == X86_VENDOR_ZHAOXIN {
        /*
         * All Zhaoxin CPUs that support C3 share cache.
         * And caches should not be flushed by software while
         * entering C3 type state.
         */
        (*flags).bm_check = 1;
        /*
         * On all recent Zhaoxin platforms, ARB_DISABLE is a nop.
         * So, set bm_control to zero to indicate that ARB_DISABLE
         * is not required while entering C3 type state.
         */
        (*flags).bm_control = 0;
    }
    if cpu_feature_enabled(X86_FEATURE_ZEN) {
        /*
         * For all AMD Zen or newer CPUs that support C3, caches
         * should not be flushed by software while entering C3
         * type state. Set bm->check to 1 so that kernel doesn't
         * need to execute cache flush operation.
         */
        (*flags).bm_check = 1;
        /*
         * In current AMD C state implementation ARB_DIS is no longer
         * used. So set bm_control to zero to indicate ARB_DIS is not
         * required while entering C3 type state.
         */
        (*flags).bm_control = 0;
    }
}

pub struct cstate_entry {
    pub states: [cstate_entry_state; ACPI_PROCESSOR_MAX_POWER],
}

#[repr(C)]
pub struct cstate_entry_state {
    pub eax: c_uint,
    pub ecx: c_uint,
}

static mut cpu_cstate_entry: *mut cstate_entry = core::ptr::null_mut();
static mut mwait_supported: [c_short; ACPI_PROCESSOR_MAX_POWER] =
    [0; ACPI_PROCESSOR_MAX_POWER];

const NATIVE_CSTATE_BEYOND_HALT: c_long = 2;

unsafe fn acpi_processor_ffh_cstate_probe_cpu(_cx: *mut c_void) -> c_long {
    let cx = _cx as *mut acpi_processor_cx;
    let mut eax: c_uint = 0;
    let mut ebx: c_uint = 0;
    let mut ecx: c_uint = 0;
    let mut edx: c_uint = 0;
    let mut edx_part: c_uint;
    let cstate_type: usize;
    let num_cstate_subtype: c_uint;

    cpuid(CPUID_LEAF_MWAIT, &mut eax, &mut ebx, &mut ecx, &mut edx);

    /* Check whether this particular cx_type (in CST) is supported or not */
    cstate_type = ((((*cx).address >> MWAIT_SUBSTATE_SIZE) & MWAIT_CSTATE_MASK) + 1)
        .wrapping_rem(MWAIT_CSTATE_MASK) as usize;
    edx_part = edx >> (cstate_type as c_uint * MWAIT_SUBSTATE_SIZE);
    num_cstate_subtype = edx_part & MWAIT_SUBSTATE_MASK;

    let mut retval: c_long = 0;
    /* If the HW does not support any sub-states in this C-state */
    if num_cstate_subtype == 0 {
        pr_warn!(FW_BUG "ACPI MWAIT C-state 0x%x not supported by HW (0x%x)\n", (*cx).address, edx_part);
        retval = -1;
        return retval;
    }

    /* mwait ecx extensions INTERRUPT_BREAK should be supported for C2/C3 */
    if (ecx & CPUID5_ECX_EXTENSIONS_SUPPORTED) == 0
        || (ecx & CPUID5_ECX_INTERRUPT_BREAK) == 0
    {
        return -1;
    }

    if mwait_supported[cstate_type] == 0 {
        mwait_supported[cstate_type] = 1;
        printk!(KERN_DEBUG "Monitor-Mwait will be used to enter C-%d state\n", (*cx).type_);
    }
    snprintf((*cx).desc.as_mut_ptr(), ACPI_CX_DESC_LEN, c"ACPI FFH MWAIT 0x%x", (*cx).address);
    retval
}

pub unsafe fn acpi_processor_ffh_cstate_probe(
    cpu: c_uint,
    cx: *mut acpi_processor_cx,
    reg: *mut acpi_power_register,
) -> c_int {
    let c = &cpu_data(cpu);
    if cpu_cstate_entry.is_null() || c.cpuid_level < CPUID_LEAF_MWAIT {
        return -1;
    }
    if (*reg).bit_offset != NATIVE_CSTATE_BEYOND_HALT {
        return -1;
    }

    let percpu_entry = per_cpu_ptr(cpu_cstate_entry, cpu);
    (*percpu_entry).states[(*cx).index].eax = 0;
    (*percpu_entry).states[(*cx).index].ecx = 0;

    /* Make sure we are running on right CPU */
    let retval = call_on_cpu(cpu, acpi_processor_ffh_cstate_probe_cpu, cx as *mut c_void, false);
    if retval == 0 {
        /* Use the hint in CST */
        (*percpu_entry).states[(*cx).index].eax = (*cx).address;
        (*percpu_entry).states[(*cx).index].ecx = MWAIT_ECX_INTERRUPT_BREAK;
    }

    /*
     * For _CST FFH on Intel, if GAS.access_size bit 1 is cleared,
     * then we should skip checking BM_STS for this C-state.
     * ref: "Intel Processor Vendor-Specific ACPI Interface Specification"
     */
    if c.x86_vendor == X86_VENDOR_INTEL && ((*reg).access_size & 0x2) == 0 {
        (*cx).bm_sts_skip = 1;
    }
    retval as c_int
}

pub unsafe fn acpi_processor_ffh_play_dead(cx: *mut acpi_processor_cx) -> ! {
    let cpu = smp_processor_id();
    let percpu_entry = per_cpu_ptr(cpu_cstate_entry, cpu);
    mwait_play_dead((*percpu_entry).states[(*cx).index].eax);
    core::hint::unreachable_unchecked()
}

pub unsafe fn acpi_processor_ffh_cstate_enter(cx: *mut acpi_processor_cx) {
    let cpu = smp_processor_id();
    let percpu_entry = per_cpu_ptr(cpu_cstate_entry, cpu);
    mwait_idle_with_hints(
        (*percpu_entry).states[(*cx).index].eax,
        (*percpu_entry).states[(*cx).index].ecx,
    );
}

unsafe fn ffh_cstate_init() -> c_int {
    let c = &boot_cpu_data;
    if c.x86_vendor != X86_VENDOR_INTEL
        && c.x86_vendor != X86_VENDOR_AMD
        && c.x86_vendor != X86_VENDOR_HYGON
    {
        return -1;
    }
    cpu_cstate_entry = alloc_percpu::<cstate_entry>();
    0
}

unsafe fn ffh_cstate_exit() {
    free_percpu(cpu_cstate_entry);
    cpu_cstate_entry = core::ptr::null_mut();
}

// arch_initcall(ffh_cstate_init);
// __exitcall(ffh_cstate_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
