// SPDX-License-Identifier: GPL-2.0

#[repr(C)]
pub struct vdso_exception_table_entry {
    pub insn: i32,
    pub fixup: i32,
}

pub unsafe fn fixup_vdso_exception(
    regs: *mut pt_regs,
    trapnr: i32,
    error_code: usize,
    fault_addr: usize,
) -> bool {
    let image = (*(*current()).mm).context.vdso_image;
    let mut extable: *const vdso_exception_table_entry;
    let nr_entries: usize;
    let mut i: usize;
    let base: usize;

    /*
     * Do not attempt to fixup #DB or #BP.  It's impossible to identify
     * whether or not a #DB/#BP originated from within an SGX enclave and
     * SGX enclaves are currently the only use case for vDSO fixup.
     */
    if trapnr == X86_TRAP_DB || trapnr == X86_TRAP_BP {
        return false;
    }

    if !(*(*current()).mm).context.vdso {
        return false;
    }

    base = (*(*current()).mm).context.vdso as usize + (*image).extable_base as usize;
    nr_entries = (*image).extable_len as usize
        / core::mem::size_of::<vdso_exception_table_entry>();
    extable = (*image).extable;

    i = 0;
    while i < nr_entries {
        if (*regs).ip == base + (*extable.add(i)).insn as usize {
            (*regs).ip = base + (*extable.add(i)).fixup as usize;
            (*regs).di = trapnr as usize;
            (*regs).si = error_code;
            (*regs).dx = fault_addr;
            return true;
        }
        i += 1;
    }

    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
