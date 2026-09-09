// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the surrounding kernel translation unit.

pub const UPROBE_TRAP_NR: c_uint = UINT_MAX;

pub unsafe fn is_swbp_insn(insn: *mut uprobe_opcode_t) -> bool {
    #[cfg(CONFIG_RISCV_ISA_C)]
    {
        return (*insn & 0xffff) == UPROBE_SWBP_INSN;
    }
    #[cfg(not(CONFIG_RISCV_ISA_C))]
    {
        return *insn == UPROBE_SWBP_INSN;
    }
}

pub unsafe fn is_trap_insn(insn: *mut uprobe_opcode_t) -> bool {
    riscv_insn_is_ebreak(*insn) || riscv_insn_is_c_ebreak(*insn)
}

pub unsafe fn uprobe_get_swbp_addr(regs: *mut pt_regs) -> c_ulong {
    instruction_pointer(regs)
}

pub unsafe fn arch_uprobe_analyze_insn(
    auprobe: *mut arch_uprobe,
    _mm: *mut mm_struct,
    _addr: c_ulong,
) -> c_int {
    let opcode: probe_opcode_t = *(std::ptr::addr_of!((*auprobe).insn[0]) as *const probe_opcode_t);

    (*auprobe).insn_size = GET_INSN_LENGTH(opcode);

    match riscv_probe_decode_insn(&opcode, &mut (*auprobe).api) {
        INSN_REJECTED => return -EINVAL,
        INSN_GOOD_NO_SLOT => (*auprobe).simulate = true,
        INSN_GOOD => (*auprobe).simulate = false,
        _ => return -EINVAL,
    }

    0
}

pub unsafe fn arch_uprobe_pre_xol(
    _auprobe: *mut arch_uprobe,
    regs: *mut pt_regs,
) -> c_int {
    let utask: *mut uprobe_task = (*current).utask;

    (*utask).autask.saved_cause = (*current).thread.bad_cause;
    (*current).thread.bad_cause = UPROBE_TRAP_NR;

    instruction_pointer_set(regs, (*utask).xol_vaddr);

    0
}

pub unsafe fn arch_uprobe_post_xol(
    auprobe: *mut arch_uprobe,
    regs: *mut pt_regs,
) -> c_int {
    let utask: *mut uprobe_task = (*current).utask;

    WARN_ON_ONCE((*current).thread.bad_cause != UPROBE_TRAP_NR);
    (*current).thread.bad_cause = (*utask).autask.saved_cause;

    instruction_pointer_set(regs, (*utask).vaddr + (*auprobe).insn_size);

    0
}

pub unsafe fn arch_uprobe_xol_was_trapped(t: *mut task_struct) -> bool {
    if (*t).thread.bad_cause != UPROBE_TRAP_NR {
        return true;
    }

    false
}

pub unsafe fn arch_uprobe_skip_sstep(
    auprobe: *mut arch_uprobe,
    regs: *mut pt_regs,
) -> bool {
    let insn: probe_opcode_t;
    let addr: c_ulong;

    if !(*auprobe).simulate {
        return false;
    }

    insn = *(std::ptr::addr_of!((*auprobe).insn[0]) as *const probe_opcode_t);
    addr = instruction_pointer(regs);

    if let Some(handler) = (*auprobe).api.handler {
        handler(insn, addr, regs);
    }

    true
}

pub unsafe fn arch_uprobe_abort_xol(
    _auprobe: *mut arch_uprobe,
    regs: *mut pt_regs,
) {
    let utask: *mut uprobe_task = (*current).utask;

    (*current).thread.bad_cause = (*utask).autask.saved_cause;
    /*
     * Task has received a fatal signal, so reset back to probed
     * address.
     */
    instruction_pointer_set(regs, (*utask).vaddr);
}

pub unsafe fn arch_uretprobe_is_alive(
    ret: *mut return_instance,
    ctx: rp_check,
    regs: *mut pt_regs,
) -> bool {
    if ctx == RP_CHECK_CHAIN_CALL {
        (*regs).sp <= (*ret).stack
    } else {
        (*regs).sp < (*ret).stack
    }
}

pub unsafe fn arch_uretprobe_hijack_return_addr(
    trampoline_vaddr: c_ulong,
    regs: *mut pt_regs,
) -> c_ulong {
    let ra: c_ulong = (*regs).ra;

    (*regs).ra = trampoline_vaddr;

    ra
}

pub unsafe fn arch_uprobe_exception_notify(
    _self: *mut notifier_block,
    _val: c_ulong,
    _data: *mut c_void,
) -> c_int {
    NOTIFY_DONE
}

pub unsafe fn uprobe_breakpoint_handler(regs: *mut pt_regs) -> bool {
    if uprobe_pre_sstep_notifier(regs) {
        return true;
    }

    false
}

pub unsafe fn uprobe_single_step_handler(regs: *mut pt_regs) -> bool {
    if uprobe_post_sstep_notifier(regs) {
        return true;
    }

    false
}

pub unsafe fn arch_uprobe_copy_ixol(
    page: *mut page,
    vaddr: c_ulong,
    src: *mut c_void,
    len: c_ulong,
) {
    /* Initialize the slot */
    let kaddr: *mut c_void = kmap_local_page(page);
    let mut dst: *mut u8 = (kaddr as *mut u8).add((vaddr & !PAGE_MASK) as usize);
    let start: c_ulong = dst as c_ulong;

    memcpy(dst as *mut c_void, src, len);

    /* Add ebreak behind opcode to simulate singlestep */
    if vaddr != 0 {
        dst = dst.add(GET_INSN_LENGTH(*(src as *const probe_opcode_t)) as usize);
        *(dst as *mut uprobe_opcode_t) = __BUG_INSN_32;
    }

    flush_icache_range(start, start + len);
    kunmap_local(kaddr);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
