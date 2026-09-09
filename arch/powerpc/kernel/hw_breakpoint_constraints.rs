// SPDX-License-Identifier: GPL-2.0+
//
// C dependencies are supplied by the surrounding kernel translation unit.

unsafe fn dar_in_user_range(dar: ::core::ffi::c_ulong,
                            info: *const arch_hw_breakpoint) -> bool {
    (*info).address <= dar && dar.wrapping_sub((*info).address) < (*info).len
}

unsafe fn ea_user_range_overlaps(ea: ::core::ffi::c_ulong, size: ::core::ffi::c_int,
                                 info: *const arch_hw_breakpoint) -> bool {
    ea < (*info).address.wrapping_add((*info).len)
        && ea.wrapping_add(size as ::core::ffi::c_ulong) > (*info).address
}

unsafe fn dar_in_hw_range(dar: ::core::ffi::c_ulong,
                          info: *const arch_hw_breakpoint) -> bool {
    let hw_start_addr = ALIGN_DOWN((*info).address, HW_BREAKPOINT_SIZE);
    let hw_end_addr = ALIGN((*info).address.wrapping_add((*info).len), HW_BREAKPOINT_SIZE);

    hw_start_addr <= dar && hw_end_addr > dar
}

unsafe fn ea_hw_range_overlaps(ea: ::core::ffi::c_ulong, size: ::core::ffi::c_int,
                               info: *const arch_hw_breakpoint) -> bool {
    let mut align_size = HW_BREAKPOINT_SIZE;

    /*
     * On p10 predecessors, quadword is handle differently then
     * other instructions.
     */
    if !cpu_has_feature(CPU_FTR_ARCH_31) && size == 16 {
        align_size = HW_BREAKPOINT_SIZE_QUADWORD;
    }

    let hw_start_addr = ALIGN_DOWN((*info).address, align_size);
    let hw_end_addr = ALIGN((*info).address.wrapping_add((*info).len), align_size);

    ea < hw_end_addr && ea.wrapping_add(size as ::core::ffi::c_ulong) > hw_start_addr
}

/*
 * If hw has multiple DAWR registers, we also need to check all
 * dawrx constraint bits to confirm this is _really_ a valid event.
 * If type is UNKNOWN, but privilege level matches, consider it as a
 * positive match.
 */
unsafe fn check_dawrx_constraints(regs: *mut pt_regs, type_: ::core::ffi::c_int,
                                  info: *mut arch_hw_breakpoint) -> bool {
    if OP_IS_LOAD(type_) && ((*info).type_ & HW_BRK_TYPE_READ) == 0 {
        return false;
    }

    /*
     * The Cache Management instructions other than dcbz never
     * cause a match. i.e. if type is CACHEOP, the instruction
     * is dcbz, and dcbz is treated as Store.
     */
    if (OP_IS_STORE(type_) || type_ == CACHEOP) && ((*info).type_ & HW_BRK_TYPE_WRITE) == 0 {
        return false;
    }

    if is_kernel_addr((*regs).nip) && ((*info).type_ & HW_BRK_TYPE_KERNEL) == 0 {
        return false;
    }

    if user_mode(regs) && ((*info).type_ & HW_BRK_TYPE_USER) == 0 {
        return false;
    }

    true
}

/*
 * Return true if the event is valid wrt dawr configuration,
 * including extraneous exception. Otherwise return false.
 */
pub unsafe fn wp_check_constraints(regs: *mut pt_regs, instr: ppc_inst_t,
                                  ea: ::core::ffi::c_ulong, type_: ::core::ffi::c_int,
                                  size: ::core::ffi::c_int,
                                  info: *mut arch_hw_breakpoint) -> bool {
    let in_user_range = dar_in_user_range((*regs).dar, info);
    let dawrx_constraints;

    /*
     * 8xx supports only one breakpoint and thus we can
     * unconditionally return true.
     */
    if IS_ENABLED(CONFIG_PPC_8xx) {
        if !in_user_range {
            (*info).type_ |= HW_BRK_TYPE_EXTRANEOUS_IRQ;
        }
        return true;
    }

    if unlikely(ppc_inst_equal(instr, ppc_inst(0))) {
        if cpu_has_feature(CPU_FTR_ARCH_31) && !dar_in_hw_range((*regs).dar, info) {
            return false;
        }
        return true;
    }

    dawrx_constraints = check_dawrx_constraints(regs, type_, info);

    if type_ == UNKNOWN {
        if cpu_has_feature(CPU_FTR_ARCH_31) && !dar_in_hw_range((*regs).dar, info) {
            return false;
        }
        return dawrx_constraints;
    }

    if ea_user_range_overlaps(ea, size, info) {
        return dawrx_constraints;
    }

    if ea_hw_range_overlaps(ea, size, info) && dawrx_constraints {
        (*info).type_ |= HW_BRK_TYPE_EXTRANEOUS_IRQ;
        return true;
    }
    false
}

pub unsafe fn wp_get_instr_detail(regs: *mut pt_regs, instr: *mut ppc_inst_t,
                                  type_: *mut ::core::ffi::c_int,
                                  size: *mut ::core::ffi::c_int,
                                  ea: *mut ::core::ffi::c_ulong) {
    let mut op: instruction_op = ::core::mem::zeroed();
    let err: ::core::ffi::c_int;

    pagefault_disable();
    err = __get_user_instr(instr, (*regs).nip as *const ::core::ffi::c_void);
    pagefault_enable();

    if err != 0 {
        return;
    }

    analyse_instr(&mut op, regs, *instr);
    *type_ = GETTYPE(op.type_);
    *ea = op.ea;

    if ((*regs).msr & MSR_64BIT) == 0 {
        *ea &= 0xffffffffUL;
    }

    *size = GETSIZE(op.type_);
    if *type_ == CACHEOP {
        *size = l1_dcache_bytes();
        *ea &= !((*size - 1) as ::core::ffi::c_ulong);
    } else if *type_ == LOAD_VMX || *type_ == STORE_VMX {
        *ea &= !((*size - 1) as ::core::ffi::c_ulong);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
