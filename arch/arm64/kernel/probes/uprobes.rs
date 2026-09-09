// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2014-2016 Pratyush Anand <panand@redhat.com>
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left external, corresponding to the original C includes.

const UPROBE_INV_FAULT_CODE: c_uint = u32::MAX;

pub unsafe fn arch_uprobe_copy_ixol(
    page: *mut page,
    vaddr: c_ulong,
    src: *const c_void,
    len: c_ulong,
) {
    let xol_page_kaddr: *mut c_void = kmap_local_page(page);
    let dst = (xol_page_kaddr as *mut u8).add((vaddr & !PAGE_MASK) as usize) as *mut c_void;

    /*
     * Initial cache maintenance of the xol page done via set_pte_at().
     * Subsequent CMOs only needed if the xol slot changes.
     */
    if memcmp(dst, src, len as usize) == 0 {
        kunmap_local(xol_page_kaddr);
        return;
    }

    /* Initialize the slot */
    memcpy(dst, src, len as usize);

    /* flush caches (dcache/icache) */
    sync_icache_aliases(dst as c_ulong, dst as c_ulong + len);

    kunmap_local(xol_page_kaddr);
}

pub unsafe fn uprobe_get_swbp_addr(regs: *mut pt_regs) -> c_ulong {
    instruction_pointer(regs)
}

pub unsafe fn arch_uprobe_analyze_insn(
    auprobe: *mut arch_uprobe,
    mm: *mut mm_struct,
    addr: c_ulong,
) -> c_int {
    let insn: u32;

    /* TODO: Currently we do not support AARCH32 instruction probing */
    if (*mm).context.flags & MMCF_AARCH32 != 0 {
        return -EOPNOTSUPP;
    } else if addr % AARCH64_INSN_SIZE != 0 {
        return -EINVAL;
    }

    insn = le32_to_cpu((*auprobe).insn);

    match arm_probe_decode_insn(insn, &mut (*auprobe).api) {
        INSN_REJECTED => return -EINVAL,
        INSN_GOOD_NO_SLOT => (*auprobe).simulate = true,
        _ => {}
    }

    0
}

pub unsafe fn arch_uprobe_pre_xol(_auprobe: *mut arch_uprobe, regs: *mut pt_regs) -> c_int {
    let utask: *mut uprobe_task = (*current).utask;

    /* Initialize with an invalid fault code to detect if ol insn trapped */
    (*current).thread.fault_code = UPROBE_INV_FAULT_CODE;

    /* Instruction points to execute ol */
    instruction_pointer_set(regs, (*utask).xol_vaddr);

    user_enable_single_step(current);

    0
}

pub unsafe fn arch_uprobe_post_xol(_auprobe: *mut arch_uprobe, regs: *mut pt_regs) -> c_int {
    let utask: *mut uprobe_task = (*current).utask;

    WARN_ON_ONCE((*current).thread.fault_code != UPROBE_INV_FAULT_CODE);

    /* Instruction points to execute next to breakpoint address */
    instruction_pointer_set(regs, (*utask).vaddr + 4);

    user_disable_single_step(current);

    0
}

pub unsafe fn arch_uprobe_xol_was_trapped(t: *mut task_struct) -> bool {
    /*
     * Between arch_uprobe_pre_xol and arch_uprobe_post_xol, if an xol
     * insn itself is trapped, then detect the case with the help of
     * invalid fault code which is being set in arch_uprobe_pre_xol
     */
    (*t).thread.fault_code != UPROBE_INV_FAULT_CODE
}

pub unsafe fn arch_uprobe_skip_sstep(_auprobe: *mut arch_uprobe, regs: *mut pt_regs) -> bool {
    let auprobe = _auprobe;
    if !(*auprobe).simulate {
        return false;
    }

    let insn = le32_to_cpu((*auprobe).insn);
    let addr = instruction_pointer(regs);

    if let Some(handler) = (*auprobe).api.handler {
        handler(insn, addr, regs);
    }

    true
}

pub unsafe fn arch_uprobe_abort_xol(_auprobe: *mut arch_uprobe, regs: *mut pt_regs) {
    let utask: *mut uprobe_task = (*current).utask;

    /*
     * Task has received a fatal signal, so reset back to probed
     * address.
     */
    instruction_pointer_set(regs, (*utask).vaddr);

    user_disable_single_step(current);
}

pub unsafe fn arch_uretprobe_is_alive(
    ret: *mut return_instance,
    ctx: rp_check,
    regs: *mut pt_regs,
) -> bool {
    /*
     * If a simple branch instruction (B) was called for retprobed
     * assembly label then return true even when regs->sp and ret->stack
     * are same. It will ensure that cleanup and reporting of return
     * instances corresponding to callee label is done when
     * handle_trampoline for called function is executed.
     */
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
    let mut orig_ret_vaddr = procedure_link_pointer(regs);
    let mut gcs_ret_vaddr: c_ulong;
    let mut err: c_int = 0;
    let gcspr: u64;

    if task_gcs_el0_enabled(current) {
        gcspr = read_sysreg_s(SYS_GCSPR_EL0);
        gcs_ret_vaddr = get_user_gcs(gcspr as *mut c_ulong, &mut err);
        if err != 0 {
            force_sig(SIGSEGV);
            return orig_ret_vaddr;
        }

        /*
         * If the LR and GCS return addr don't match, then some kind of PAC
         * signing or control flow occurred since entering the probed function.
         * Likely because the user is attempting to retprobe on an instruction
         * that isn't a function boundary or inside a leaf function. Explicitly
         * abort this retprobe because it will generate a GCS exception.
         */
        if gcs_ret_vaddr != orig_ret_vaddr {
            orig_ret_vaddr = c_ulong::MAX;
            return orig_ret_vaddr;
        }

        put_user_gcs(trampoline_vaddr, gcspr as *mut c_ulong, &mut err);
        if err != 0 {
            force_sig(SIGSEGV);
            return orig_ret_vaddr;
        }
    }

    /* Replace the return addr with trampoline addr */
    procedure_link_pointer_set(regs, trampoline_vaddr);

    orig_ret_vaddr
}

pub unsafe fn arch_uprobe_exception_notify(
    _self: *mut notifier_block,
    _val: c_ulong,
    _data: *mut c_void,
) -> c_int {
    NOTIFY_DONE
}

pub unsafe fn uprobe_brk_handler(_regs: *mut pt_regs, _esr: c_ulong) -> c_int {
    if uprobe_pre_sstep_notifier(_regs) != 0 {
        return DBG_HOOK_HANDLED;
    }

    DBG_HOOK_ERROR
}

pub unsafe fn uprobe_single_step_handler(regs: *mut pt_regs, _esr: c_ulong) -> c_int {
    let utask: *mut uprobe_task = (*current).utask;

    WARN_ON(utask != core::ptr::null_mut() && instruction_pointer(regs) != (*utask).xol_vaddr + 4);
    if uprobe_post_sstep_notifier(regs) != 0 {
        return DBG_HOOK_HANDLED;
    }

    DBG_HOOK_ERROR
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
