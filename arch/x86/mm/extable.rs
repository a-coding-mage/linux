// SPDX-License-Identifier: GPL-2.0-only
// Dependencies supplied by the surrounding kernel translation.

unsafe fn pt_regs_nr(regs: *mut pt_regs, nr: i32) -> *mut c_ulong {
    let reg_offset = pt_regs_offset(regs, nr);
    static mut DUMMY: c_ulong = 0;
    if warn_on_once(reg_offset < 0) { return core::ptr::addr_of_mut!(DUMMY); }
    (regs as *mut u8).offset(reg_offset as isize) as *mut c_ulong
}

unsafe fn ex_fixup_addr(x: *const exception_table_entry) -> c_ulong {
    (&(*x).fixup as *const _ as c_ulong).wrapping_add((*x).fixup as c_ulong)
}

unsafe fn ex_handler_default(e: *const exception_table_entry, regs: *mut pt_regs) -> bool {
    if (*e).data & EX_FLAG_CLEAR_AX != 0 { (*regs).ax = 0; }
    if (*e).data & EX_FLAG_CLEAR_DX != 0 { (*regs).dx = 0; }
    (*regs).ip = ex_fixup_addr(e); true
}

unsafe fn ex_handler_zeropad(e: *const exception_table_entry, regs: *mut pt_regs, fault_addr: c_ulong) -> bool {
    let mut insn = core::mem::MaybeUninit::<insn>::uninit();
    let mask = core::mem::size_of::<c_long>() as c_ulong - 1;
    let next_ip = ex_fixup_addr(e);
    let len = next_ip.wrapping_sub((*regs).ip);
    if len > MAX_INSN_SIZE as c_ulong { return false; }
    if insn_decode(insn.as_mut_ptr(), (*regs).ip as *mut _, len, INSN_MODE_KERN) != 0 { return false; }
    let insn = insn.assume_init();
    if insn.length as c_ulong != len || insn.opcode.bytes[0] != 0x8b || insn.opnd_bytes as usize != core::mem::size_of::<c_long>() { return false; }
    let mut addr = insn_get_addr_ref(&insn, regs);
    if addr == !0 as c_ulong { return false; }
    let offset = addr & mask; addr &= !mask;
    if fault_addr != addr.wrapping_add(core::mem::size_of::<c_long>() as c_ulong) { return false; }
    let reg = insn_get_modrm_reg_ptr(&insn, regs);
    if reg.is_null() { return false; }
    *reg = *((addr as *const c_ulong)) >> (offset * 8);
    ex_handler_default(e, regs)
}

unsafe fn ex_handler_fault(fixup: *const exception_table_entry, regs: *mut pt_regs, trapnr: i32) -> bool { (*regs).ax = trapnr as _; ex_handler_default(fixup, regs) }
unsafe fn ex_handler_sgx(fixup: *const exception_table_entry, regs: *mut pt_regs, trapnr: i32) -> bool { (*regs).ax = trapnr as c_ulong | SGX_ENCLS_FAULT_FLAG; ex_handler_default(fixup, regs) }

unsafe fn ex_handler_fprestore(fixup: *const exception_table_entry, regs: *mut pt_regs) -> bool {
    warn_once(true, "Bad FPU state detected at %pB, reinitializing FPU registers.", instruction_pointer(regs));
    fpu_reset_from_exception_fixup(); ex_handler_default(fixup, regs)
}

unsafe fn gp_fault_address_ok(mut fault_address: c_ulong) -> bool {
    // CONFIG_X86_64 conditional: retain the x86-64 user-address checks.
    if valid_user_address(fault_address) { return true; }
    fault_address = fault_address.wrapping_sub(PAGE_SIZE as c_ulong);
    if valid_user_address(fault_address) { return true; }
    false
}

unsafe fn ex_handler_uaccess(fixup: *const exception_table_entry, regs: *mut pt_regs, trapnr: i32, fault_address: c_ulong) -> bool {
    warn_once(trapnr == X86_TRAP_GP && !gp_fault_address_ok(fault_address), "General protection fault in user access. Non-canonical address?");
    ex_handler_default(fixup, regs)
}

unsafe fn ex_handler_msr(fixup: *const exception_table_entry, regs: *mut pt_regs, wrmsr: bool, safe: bool, reg: i32) -> bool {
    if !safe && wrmsr { pr_warn("unchecked MSR access error: WRMSR to 0x%x (tried to write 0x%08x%08x) at rIP: 0x%lx (%pS)\n", (*regs).cx as u32, (*regs).dx as u32, (*regs).ax as u32, (*regs).ip, (*regs).ip); show_stack_regs(regs); }
    if !safe && !wrmsr { pr_warn("unchecked MSR access error: RDMSR from 0x%x at rIP: 0x%lx (%pS)\n", (*regs).cx as u32, (*regs).ip, (*regs).ip); show_stack_regs(regs); }
    if !wrmsr { (*regs).ax = 0; (*regs).dx = 0; }
    if safe { *pt_regs_nr(regs, reg) = (-EIO) as c_ulong; }
    ex_handler_default(fixup, regs)
}

unsafe fn ex_handler_clear_fs(fixup: *const exception_table_entry, regs: *mut pt_regs) -> bool {
    if cpu_feature_enabled(X86_BUG_NULL_SEG) { asm!("mov %fs, {0}", in(reg) __USER_DS); }
    asm!("mov %fs, {0}", in(reg) 0u64); ex_handler_default(fixup, regs)
}
unsafe fn ex_handler_imm_reg(fixup: *const exception_table_entry, regs: *mut pt_regs, reg: i32, imm: i32) -> bool { *pt_regs_nr(regs, reg) = imm as c_long as c_ulong; ex_handler_default(fixup, regs) }
unsafe fn ex_handler_ucopy_len(fixup: *const exception_table_entry, regs: *mut pt_regs, trapnr: i32, fault_address: c_ulong, reg: i32, imm: i32) -> bool { (*regs).cx = (imm as c_ulong).wrapping_mul((*regs).cx).wrapping_add(*pt_regs_nr(regs, reg)); ex_handler_uaccess(fixup, regs, trapnr, fault_address) }

pub unsafe fn ex_get_fixup_type(ip: c_ulong) -> i32 { let e = search_exception_tables(ip); if e.is_null() { EX_TYPE_NONE } else { field_get(EX_DATA_TYPE_MASK, (*e).data) } }

pub unsafe fn fixup_exception(regs: *mut pt_regs, trapnr: i32, error_code: c_ulong, fault_addr: c_ulong) -> i32 {
    let e = search_exception_tables((*regs).ip); if e.is_null() { return 0; }
    let typ = field_get(EX_DATA_TYPE_MASK, (*e).data); let reg = field_get(EX_DATA_REG_MASK, (*e).data); let imm = field_get_signed(EX_DATA_IMM_MASK, (*e).data);
    match typ {
        EX_TYPE_DEFAULT | EX_TYPE_DEFAULT_MCE_SAFE => ex_handler_default(e, regs),
        EX_TYPE_FAULT | EX_TYPE_FAULT_MCE_SAFE => ex_handler_fault(e, regs, trapnr),
        EX_TYPE_UACCESS => ex_handler_uaccess(e, regs, trapnr, fault_addr),
        EX_TYPE_CLEAR_FS => ex_handler_clear_fs(e, regs), EX_TYPE_FPU_RESTORE => ex_handler_fprestore(e, regs),
        EX_TYPE_BPF => ex_handler_bpf(e, regs), EX_TYPE_WRMSR => ex_handler_msr(e, regs, true, false, reg), EX_TYPE_RDMSR => ex_handler_msr(e, regs, false, false, reg),
        EX_TYPE_WRMSR_SAFE => ex_handler_msr(e, regs, true, true, reg), EX_TYPE_RDMSR_SAFE => ex_handler_msr(e, regs, false, true, reg),
        EX_TYPE_WRMSR_IN_MCE => { ex_handler_msr_mce(regs, true); true }, EX_TYPE_RDMSR_IN_MCE => { ex_handler_msr_mce(regs, false); true },
        EX_TYPE_POP_REG => { (*regs).sp = (*regs).sp.wrapping_add(core::mem::size_of::<c_long>() as c_ulong); ex_handler_imm_reg(e, regs, reg, imm) },
        EX_TYPE_IMM_REG => ex_handler_imm_reg(e, regs, reg, imm), EX_TYPE_FAULT_SGX => ex_handler_sgx(e, regs, trapnr),
        EX_TYPE_UCOPY_LEN => ex_handler_ucopy_len(e, regs, trapnr, fault_addr, reg, imm), EX_TYPE_ZEROPAD => ex_handler_zeropad(e, regs, fault_addr),
        _ => { bug!(); false }
    } as i32
}

extern "C" { static mut early_recursion_flag: u32; }
pub unsafe fn early_fixup_exception(regs: *mut pt_regs, trapnr: i32) {
    if trapnr == X86_TRAP_NMI { return; } if early_recursion_flag > 2 { loop { halt(); } }
    if !xen_pv_domain() && (*regs).cs != __KERNEL_CS { early_printk("PANIC: early exception 0x%02x IP %lx:%lx error %lx cr2 0x%lx\n", trapnr, (*regs).cs, (*regs).ip, (*regs).orig_ax, read_cr2()); show_regs(regs); loop { halt(); } }
    if fixup_exception(regs, trapnr, (*regs).orig_ax, 0) != 0 { return; }
    if trapnr == X86_TRAP_UD && handle_bug(regs) { return; }
    early_printk("PANIC: early exception 0x%02x IP %lx:%lx error %lx cr2 0x%lx\n", trapnr, (*regs).cs, (*regs).ip, (*regs).orig_ax, read_cr2()); show_regs(regs); loop { halt(); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
