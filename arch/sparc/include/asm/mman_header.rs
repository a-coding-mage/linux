/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the corresponding UAPI header.

#[macro_export]
macro_rules! arch_mmap_check {
    ($addr:expr, $len:expr, $flags:expr) => {
        sparc_mmap_check($addr, $len)
    };
}

unsafe extern "C" {
    fn sparc_mmap_check(addr: c_ulong, len: c_ulong) -> c_int;
}

// The following declarations and definitions are conditional on CONFIG_SPARC64.

#[cfg(CONFIG_SPARC64)]
unsafe fn ipi_set_tstate_mcde(arg: *mut c_void) {
    let mm: *mut mm_struct = arg as *mut mm_struct;

    /* Set TSTATE_MCDE for the task using address map that ADI has been
     * enabled on if the task is running. If not, it will be set
     * automatically at the next context switch
     */
    if (*current).mm == mm {
        let regs: *mut pt_regs;

        regs = task_pt_regs(current);
        (*regs).tstate |= TSTATE_MCDE;
    }
}

#[cfg(CONFIG_SPARC64)]
#[macro_export]
macro_rules! arch_calc_vm_prot_bits {
    ($prot:expr, $pkey:expr) => {
        sparc_calc_vm_prot_bits($prot)
    };
}

#[cfg(CONFIG_SPARC64)]
unsafe fn sparc_calc_vm_prot_bits(prot: c_ulong) -> vm_flags_t {
    if adi_capable() && (prot & PROT_ADI) != 0 {
        let regs: *mut pt_regs;

        if !(*(*current).mm).context.adi {
            regs = task_pt_regs(current);
            (*regs).tstate |= TSTATE_MCDE;
            (*(*current).mm).context.adi = true;
            on_each_cpu_mask(
                mm_cpumask((*current).mm),
                ipi_set_tstate_mcde,
                (*current).mm as *mut c_void,
                0,
            );
        }
        VM_SPARC_ADI
    } else {
        0
    }
}

#[cfg(CONFIG_SPARC64)]
#[macro_export]
macro_rules! arch_validate_prot {
    ($prot:expr, $addr:expr) => {
        sparc_validate_prot($prot, $addr)
    };
}

#[cfg(CONFIG_SPARC64)]
unsafe fn sparc_validate_prot(prot: c_ulong, addr: c_ulong) -> c_int {
    let _ = addr;
    if (prot & !(PROT_READ | PROT_WRITE | PROT_EXEC | PROT_SEM | PROT_ADI)) != 0 {
        return 0;
    }
    1
}

#[cfg(CONFIG_SPARC64)]
// arch_validate_flags() - Ensure combination of flags is valid for a VMA.
unsafe fn arch_validate_flags(vm_flags: vm_flags_t) -> bool {
    /* If ADI is being enabled on this VMA, check for ADI
     * capability on the platform and ensure VMA is suitable
     * for ADI
     */
    if (vm_flags & VM_SPARC_ADI) != 0 {
        if !adi_capable() {
            return false;
        }

        /* ADI can not be enabled on PFN mapped pages */
        if (vm_flags & (VM_PFNMAP | VM_MIXEDMAP)) != 0 {
            return false;
        }

        /* Mergeable pages can become unmergeable
         * if ADI is enabled on them even if they
         * have identical data on them. This can be
         * because ADI enabled pages with identical
         * data may still not have identical ADI
         * tags on them. Disallow ADI on mergeable
         * pages.
         */
        if (vm_flags & VM_MERGEABLE) != 0 {
            return false;
        }
    }
    true
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
