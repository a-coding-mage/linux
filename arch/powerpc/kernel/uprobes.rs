// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * User-space Probes (UProbes) for powerpc
 *
 * Copyright IBM Corporation, 2007-2012
 *
 * Adapted from the x86 port by Ananth N Mavinakayanahalli <ananth@in.ibm.com>
 */

// Dependencies supplied by the surrounding kernel translation.

const UPROBE_TRAP_NR: u32 = u32::MAX;

/**
 * is_trap_insn - check if the instruction is a trap variant
 * @insn: instruction to be checked.
 * Returns true if @insn is a trap variant.
 */
pub unsafe fn is_trap_insn(insn: *mut uprobe_opcode_t) -> bool {
    is_trap(*insn)
}

/**
 * arch_uprobe_analyze_insn
 * @mm: the probed address space.
 * @arch_uprobe: the probepoint information.
 * @addr: vaddr to probe.
 * Return 0 on success or a -ve number on error.
 */
pub unsafe fn arch_uprobe_analyze_insn(
    auprobe: *mut arch_uprobe,
    mm: *mut mm_struct,
    addr: c_ulong,
) -> c_int {
    if addr & 0x03 != 0 {
        return -EINVAL;
    }

    if cpu_has_feature(CPU_FTR_ARCH_31)
        && ppc_inst_prefixed(ppc_inst_read((*auprobe).insn))
        && addr & 0x3f == 60
    {
        pr_info_ratelimited!("Cannot register a uprobe on 64 byte unaligned prefixed instruction\n");
        return -EINVAL;
    }

    if !can_single_step(ppc_inst_val(ppc_inst_read((*auprobe).insn))) {
        pr_info_ratelimited!("Cannot register a uprobe on instructions that can't be single stepped\n");
        return -ENOTSUPP;
    }

    0
}

/*
 * arch_uprobe_pre_xol - prepare to execute out of line.
 * @auprobe: the probepoint information.
 * @regs: reflects the saved user state of current task.
 */
pub unsafe fn arch_uprobe_pre_xol(
    auprobe: *mut arch_uprobe,
    regs: *mut pt_regs,
) -> c_int {
    let autask: *mut arch_uprobe_task = &mut (*(*current).utask).autask;

    (*autask).saved_trap_nr = (*current).thread.trap_nr;
    (*current).thread.trap_nr = UPROBE_TRAP_NR as _;
    regs_set_return_ip(regs, (*(*current).utask).xol_vaddr);

    user_enable_single_step(current);
    0
}

/**
 * uprobe_get_swbp_addr - compute address of swbp given post-swbp regs
 * @regs: Reflects the saved state of the task after it has hit a breakpoint
 * instruction.
 * Return the address of the breakpoint instruction.
 */
pub unsafe fn uprobe_get_swbp_addr(regs: *mut pt_regs) -> c_ulong {
    instruction_pointer(regs)
}

/*
 * If xol insn itself traps and generates a signal (SIGILL/SIGSEGV/etc),
 * then detect the case where a singlestepped instruction jumps back to its
 * own address. It is assumed that anything like do_page_fault/do_trap/etc
 * sets thread.trap_nr != UINT_MAX.
 *
 * arch_uprobe_pre_xol/arch_uprobe_post_xol save/restore thread.trap_nr,
 * arch_uprobe_xol_was_trapped() simply checks that ->trap_nr is not equal
 * to UPROBE_TRAP_NR == UINT_MAX set by arch_uprobe_pre_xol().
 */
pub unsafe fn arch_uprobe_xol_was_trapped(t: *mut task_struct) -> bool {
    if (*t).thread.trap_nr != UPROBE_TRAP_NR as _ {
        return true;
    }

    false
}

/*
 * Called after single-stepping. To avoid the SMP problems that can
 * occur when we temporarily put back the original opcode to
 * single-step, we single-stepped a copy of the instruction.
 *
 * This function prepares to resume execution after the single-step.
 */
pub unsafe fn arch_uprobe_post_xol(
    auprobe: *mut arch_uprobe,
    regs: *mut pt_regs,
) -> c_int {
    let utask: *mut uprobe_task = (*current).utask;

    WARN_ON_ONCE!((*current).thread.trap_nr != UPROBE_TRAP_NR as _);

    (*current).thread.trap_nr = (*utask).autask.saved_trap_nr;

    /*
     * On powerpc, except for loads and stores, most instructions
     * including ones that alter code flow (branches, calls, returns)
     * are emulated in the kernel. We get here only if the emulation
     * support doesn't exist and have to fix-up the next instruction
     * to be executed.
     */
    regs_set_return_ip(
        regs,
        ppc_inst_next((*utask).vaddr as *mut c_void, (*auprobe).insn) as c_ulong,
    );

    user_disable_single_step(current);
    0
}

/* callback routine for handling exceptions. */
pub unsafe fn arch_uprobe_exception_notify(
    self_: *mut notifier_block,
    val: c_ulong,
    data: *mut c_void,
) -> c_int {
    let args: *mut die_args = data as *mut die_args;
    let regs: *mut pt_regs = (*args).regs;

    /* regs == NULL is a kernel bug */
    if WARN_ON!(regs.is_null()) {
        return NOTIFY_DONE;
    }

    /* We are only interested in userspace traps */
    if !user_mode(regs) {
        return NOTIFY_DONE;
    }

    match val {
        DIE_BPT => {
            if uprobe_pre_sstep_notifier(regs) {
                return NOTIFY_STOP;
            }
        }
        DIE_SSTEP => {
            if uprobe_post_sstep_notifier(regs) {
                return NOTIFY_STOP;
            }
        }
        _ => {}
    }
    NOTIFY_DONE
}

/*
 * This function gets called when XOL instruction either gets trapped or
 * the thread has a fatal signal, so reset the instruction pointer to its
 * probed address.
 */
pub unsafe fn arch_uprobe_abort_xol(
    auprobe: *mut arch_uprobe,
    regs: *mut pt_regs,
) {
    let utask: *mut uprobe_task = (*current).utask;

    (*current).thread.trap_nr = (*utask).autask.saved_trap_nr;
    instruction_pointer_set(regs, (*utask).vaddr);

    user_disable_single_step(current);
}

/*
 * See if the instruction can be emulated.
 * Returns true if instruction was emulated, false otherwise.
 */
pub unsafe fn arch_uprobe_skip_sstep(
    auprobe: *mut arch_uprobe,
    regs: *mut pt_regs,
) -> bool {
    let ret: c_int;

    /*
     * emulate_step() returns 1 if the insn was successfully emulated.
     * For all other cases, we need to single-step in hardware.
     */
    ret = emulate_step(regs, ppc_inst_read((*auprobe).insn));
    if ret > 0 {
        return true;
    }

    false
}

pub unsafe fn arch_uretprobe_hijack_return_addr(
    trampoline_vaddr: c_ulong,
    regs: *mut pt_regs,
) -> c_ulong {
    let orig_ret_vaddr: c_ulong;

    orig_ret_vaddr = (*regs).link;

    /* Replace the return addr with trampoline addr */
    (*regs).link = trampoline_vaddr;

    orig_ret_vaddr
}

pub unsafe fn arch_uretprobe_is_alive(
    ret: *mut return_instance,
    ctx: rp_check,
    regs: *mut pt_regs,
) -> bool {
    if ctx == RP_CHECK_CHAIN_CALL {
        (*regs).gpr[1] <= (*ret).stack
    } else {
        (*regs).gpr[1] < (*ret).stack
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
