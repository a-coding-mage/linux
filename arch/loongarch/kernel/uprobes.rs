// SPDX-License-Identifier: GPL-2.0-only
//
// Dependencies supplied by the surrounding kernel translation unit.

const UPROBE_TRAP_NR: ::core::ffi::c_ulong = u32::MAX as ::core::ffi::c_ulong;

pub unsafe fn arch_uprobe_analyze_insn(
    auprobe: *mut arch_uprobe,
    _mm: *mut mm_struct,
    addr: ::core::ffi::c_ulong,
) -> ::core::ffi::c_int {
    let mut idx: ::core::ffi::c_int;
    let mut insn: loongarch_instruction;

    if addr & 0x3 != 0 {
        return -EILSEQ;
    }

    idx = (::core::mem::size_of_val(&(*auprobe).insn) /
        ::core::mem::size_of_val(&(*auprobe).insn[0])) as ::core::ffi::c_int - 1;
    while idx >= 0 {
        insn.word = (*auprobe).insn[idx as usize];
        if insns_not_supported(insn) {
            return -EINVAL;
        }
        idx -= 1;
    }

    if insns_need_simulation(insn) {
        (*auprobe).ixol[0] = larch_insn_gen_nop();
        (*auprobe).simulate = true;
    } else {
        (*auprobe).ixol[0] = (*auprobe).insn[0];
        (*auprobe).simulate = false;
    }

    (*auprobe).ixol[1] = UPROBE_XOLBP_INSN;

    0
}

pub unsafe fn arch_uprobe_pre_xol(
    _auprobe: *mut arch_uprobe,
    regs: *mut pt_regs,
) -> ::core::ffi::c_int {
    let utask: *mut uprobe_task = (*current).utask;

    (*utask).autask.saved_trap_nr = (*current).thread.trap_nr;
    (*current).thread.trap_nr = UPROBE_TRAP_NR;
    instruction_pointer_set(regs, (*utask).xol_vaddr);

    0
}

pub unsafe fn arch_uprobe_post_xol(
    _auprobe: *mut arch_uprobe,
    regs: *mut pt_regs,
) -> ::core::ffi::c_int {
    let utask: *mut uprobe_task = (*current).utask;

    WARN_ON_ONCE((*current).thread.trap_nr != UPROBE_TRAP_NR);
    (*current).thread.trap_nr = (*utask).autask.saved_trap_nr;
    instruction_pointer_set(regs, (*utask).vaddr + LOONGARCH_INSN_SIZE);

    0
}

pub unsafe fn arch_uprobe_abort_xol(_auprobe: *mut arch_uprobe, regs: *mut pt_regs) {
    let utask: *mut uprobe_task = (*current).utask;

    (*current).thread.trap_nr = (*utask).autask.saved_trap_nr;
    instruction_pointer_set(regs, (*utask).vaddr);
}

pub unsafe fn arch_uprobe_xol_was_trapped(t: *mut task_struct) -> bool {
    if (*t).thread.trap_nr != UPROBE_TRAP_NR {
        return true;
    }

    false
}

pub unsafe fn arch_uprobe_skip_sstep(
    auprobe: *mut arch_uprobe,
    regs: *mut pt_regs,
) -> bool {
    let mut insn: loongarch_instruction;

    if !(*auprobe).simulate {
        return false;
    }

    insn.word = (*auprobe).insn[0];
    arch_simulate_insn(insn, regs);

    true
}

pub unsafe fn arch_uretprobe_hijack_return_addr(
    trampoline_vaddr: ::core::ffi::c_ulong,
    regs: *mut pt_regs,
) -> ::core::ffi::c_ulong {
    let ra = (*regs).regs[1];

    (*regs).regs[1] = trampoline_vaddr;

    ra
}

pub unsafe fn arch_uretprobe_is_alive(
    ret: *mut return_instance,
    ctx: rp_check,
    regs: *mut pt_regs,
) -> bool {
    if ctx == RP_CHECK_CHAIN_CALL {
        (*regs).regs[3] <= (*ret).stack
    } else {
        (*regs).regs[3] < (*ret).stack
    }
}

pub unsafe fn arch_uprobe_exception_notify(
    _self: *mut notifier_block,
    _val: ::core::ffi::c_ulong,
    _data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    NOTIFY_DONE
}

pub unsafe fn uprobe_breakpoint_handler(regs: *mut pt_regs) -> bool {
    if uprobe_pre_sstep_notifier(regs) {
        return true;
    }

    false
}

pub unsafe fn uprobe_singlestep_handler(regs: *mut pt_regs) -> bool {
    if uprobe_post_sstep_notifier(regs) {
        return true;
    }

    false
}

pub unsafe fn uprobe_get_swbp_addr(regs: *mut pt_regs) -> ::core::ffi::c_ulong {
    instruction_pointer(regs)
}

pub unsafe fn arch_uprobe_copy_ixol(
    page: *mut page,
    vaddr: ::core::ffi::c_ulong,
    src: *mut ::core::ffi::c_void,
    len: ::core::ffi::c_ulong,
) {
    let kaddr: *mut ::core::ffi::c_void = kmap_local_page(page);
    let dst = (kaddr as *mut u8).add((vaddr & !PAGE_MASK) as usize) as *mut ::core::ffi::c_void;

    memcpy(dst, src, len);
    flush_icache_range(dst as ::core::ffi::c_ulong, dst as ::core::ffi::c_ulong + len);
    kunmap_local(kaddr);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
