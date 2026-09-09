// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the surrounding kernel translation.

#[inline]
unsafe fn field_get(mask: u64, value: u64) -> u64 {
    (value & mask) >> mask.trailing_zeros()
}

pub unsafe fn s390_search_extables(addr: ::core::ffi::c_ulong) -> *const exception_table_entry {
    let fixup = search_exception_tables(addr);
    if !fixup.is_null() {
        return fixup;
    }
    let num = __stop_amode31_ex_table.offset_from(__start_amode31_ex_table) as usize;
    search_extable(__start_amode31_ex_table, num, addr)
}

unsafe fn ex_handler_fixup(ex: *const exception_table_entry, regs: *mut pt_regs) -> bool {
    (*regs).psw.addr = extable_fixup(ex);
    true
}

unsafe fn ex_handler_ua_fault(ex: *const exception_table_entry, regs: *mut pt_regs) -> bool {
    let reg_err = field_get(EX_DATA_REG_ERR, (*ex).data) as usize;

    (*regs).gprs[reg_err] = (-EFAULT) as _;
    (*regs).psw.addr = extable_fixup(ex);
    true
}

unsafe fn ex_handler_ua_load_reg(
    ex: *const exception_table_entry,
    pair: bool,
    regs: *mut pt_regs,
) -> bool {
    let reg_zero = field_get(EX_DATA_REG_ADDR, (*ex).data) as usize;
    let reg_err = field_get(EX_DATA_REG_ERR, (*ex).data) as usize;

    (*regs).gprs[reg_err] = (-EFAULT) as _;
    (*regs).gprs[reg_zero] = 0;
    if pair {
        (*regs).gprs[reg_zero + 1] = 0;
    }
    (*regs).psw.addr = extable_fixup(ex);
    true
}

unsafe fn ex_handler_zeropad(ex: *const exception_table_entry, regs: *mut pt_regs) -> bool {
    let reg_addr = field_get(EX_DATA_REG_ADDR, (*ex).data) as usize;
    let reg_data = field_get(EX_DATA_REG_ERR, (*ex).data) as usize;
    let addr = (*regs).gprs[reg_addr];
    let offset = addr & (::core::mem::size_of::<::core::ffi::c_ulong>() - 1);
    let addr = addr & !(::core::mem::size_of::<::core::ffi::c_ulong>() - 1);
    let mut data = *(addr as *const ::core::ffi::c_ulong);
    data <<= 8 * offset;
    (*regs).gprs[reg_data] = data;
    (*regs).psw.addr = extable_fixup(ex);
    true
}

unsafe fn ex_handler_fpc(ex: *const exception_table_entry, regs: *mut pt_regs) -> bool {
    fpu_sfpc(0);
    (*regs).psw.addr = extable_fixup(ex);
    true
}

#[repr(C, packed)]
struct insn_ssf {
    opc1: u64,
    r3: u64,
    opc2: u64,
    b1: u64,
    d1: u64,
    b2: u64,
    d2: u64,
}

unsafe fn ex_handler_ua_mvcos(
    ex: *const exception_table_entry,
    from: bool,
    regs: *mut pt_regs,
) -> bool {
    let mut uaddr: ::core::ffi::c_ulong;
    let mut remainder: ::core::ffi::c_ulong;
    let insn: *const insn_ssf;

    /*
     * If the faulting user space access crossed a page boundary retry by
     * limiting the access to the first page (adjust length accordingly).
     * Then the mvcos instruction will either complete with condition code
     * zero, or generate another fault where the user space access did not
     * cross a page boundary.
     * If the faulting user space access did not cross a page boundary set
     * length to zero and retry. In this case no user space access will
     * happen, and the mvcos instruction will complete with condition code
     * zero.
     * In both cases the instruction will complete with condition code
     * zero (copying finished), and the register which contains the
     * length, indicates the number of bytes copied.
     */
    (*regs).psw.addr = extable_fixup(ex);
    insn = (*regs).psw.addr as *const insn_ssf;
    if from {
        uaddr = (*regs).gprs[(*insn).b2 as usize] + (*insn).d2;
    } else {
        uaddr = (*regs).gprs[(*insn).b1 as usize] + (*insn).d1;
    }
    remainder = PAGE_SIZE - (uaddr & (PAGE_SIZE - 1));
    if (*regs).gprs[(*insn).r3 as usize] <= remainder {
        remainder = 0;
    }
    (*regs).gprs[(*insn).r3 as usize] = remainder;
    true
}

pub unsafe fn fixup_exception(regs: *mut pt_regs) -> bool {
    let ex = s390_search_extables(instruction_pointer(regs));
    if ex.is_null() {
        return false;
    }
    match (*ex).type_ {
        EX_TYPE_FIXUP => ex_handler_fixup(ex, regs),
        EX_TYPE_BPF => ex_handler_bpf(ex, regs),
        EX_TYPE_UA_FAULT => ex_handler_ua_fault(ex, regs),
        EX_TYPE_UA_LOAD_REG => ex_handler_ua_load_reg(ex, false, regs),
        EX_TYPE_UA_LOAD_REGPAIR => ex_handler_ua_load_reg(ex, true, regs),
        EX_TYPE_ZEROPAD => ex_handler_zeropad(ex, regs),
        EX_TYPE_FPC => ex_handler_fpc(ex, regs),
        EX_TYPE_UA_MVCOS_TO => ex_handler_ua_mvcos(ex, false, regs),
        EX_TYPE_UA_MVCOS_FROM => ex_handler_ua_mvcos(ex, true, regs),
        _ => panic!("invalid exception table entry"),
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
