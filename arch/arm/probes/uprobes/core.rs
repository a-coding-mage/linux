// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2012 Rabin Vincent <rabin at rab.in>
 */

// Linux and architecture dependencies are supplied by the surrounding kernel translation.

const UPROBE_TRAP_NR: u32 = u32::MAX;

pub unsafe fn is_swbp_insn(insn: *mut uprobe_opcode_t) -> bool {
    (__mem_to_opcode_arm(*insn) & 0x0fffffff)
        == (UPROBE_SWBP_ARM_INSN & 0x0fffffff)
}

pub unsafe fn set_swbp(
    auprobe: *mut arch_uprobe,
    vma: *mut vm_area_struct,
    vaddr: c_ulong,
) -> c_int {
    uprobe_write_opcode(
        auprobe,
        vma,
        vaddr,
        __opcode_to_mem_arm((*auprobe).bpinsn),
        true,
    )
}

pub unsafe fn arch_uprobe_ignore(
    auprobe: *mut arch_uprobe,
    regs: *mut pt_regs,
) -> bool {
    if !((*auprobe).asi.insn_check_cc)((*regs).ARM_cpsr) {
        (*regs).ARM_pc += 4;
        return true;
    }

    false
}

pub unsafe fn arch_uprobe_skip_sstep(
    auprobe: *mut arch_uprobe,
    regs: *mut pt_regs,
) -> bool {
    let opcode: probes_opcode_t;

    if !(*auprobe).simulate {
        return false;
    }

    opcode = __mem_to_opcode_arm(*((*auprobe).insn as *mut c_uint));

    ((*auprobe).asi.insn_singlestep)(opcode, &mut (*auprobe).asi, regs);

    true
}

pub unsafe fn arch_uretprobe_hijack_return_addr(
    trampoline_vaddr: c_ulong,
    regs: *mut pt_regs,
) -> c_ulong {
    let orig_ret_vaddr: c_ulong;

    orig_ret_vaddr = (*regs).ARM_lr;
    /* Replace the return addr with trampoline addr */
    (*regs).ARM_lr = trampoline_vaddr;
    orig_ret_vaddr
}

pub unsafe fn arch_uprobe_analyze_insn(
    auprobe: *mut arch_uprobe,
    mm: *mut mm_struct,
    addr: c_ulong,
) -> c_int {
    let insn: c_uint;
    let bpinsn: c_uint;
    let ret: probes_insn;

    /* Thumb not yet support */
    if addr & 0x3 != 0 {
        return -EINVAL;
    }

    insn = __mem_to_opcode_arm(*((*auprobe).insn as *mut c_uint));
    (*auprobe).ixol[0] = __opcode_to_mem_arm(insn);
    (*auprobe).ixol[1] = __opcode_to_mem_arm(UPROBE_SS_ARM_INSN);

    ret = arm_probes_decode_insn(
        insn,
        &mut (*auprobe).asi,
        false,
        uprobes_probes_actions,
        core::ptr::null_mut(),
    );
    match ret {
        INSN_REJECTED => return -EINVAL,
        INSN_GOOD_NO_SLOT => {
            (*auprobe).simulate = true;
        }
        INSN_GOOD => {}
        _ => {}
    }

    bpinsn = UPROBE_SWBP_ARM_INSN & 0x0fffffff;
    let bpinsn = if insn >= 0xe0000000 {
        bpinsn | 0xe0000000 /* Unconditional instruction */
    } else {
        bpinsn | (insn & 0xf0000000) /* Copy condition from insn */
    };

    (*auprobe).bpinsn = bpinsn;

    0
}

pub unsafe fn arch_uprobe_copy_ixol(
    page: *mut page,
    vaddr: c_ulong,
    src: *mut c_void,
    len: c_ulong,
) {
    let xol_page_kaddr: *mut c_void = kmap_local_page(page);
    let dst = (xol_page_kaddr as *mut u8).add((vaddr & !PAGE_MASK) as usize) as *mut c_void;

    preempt_disable();

    /* Initialize the slot */
    core::ptr::copy_nonoverlapping(src as *const u8, dst as *mut u8, len as usize);

    /* flush caches (dcache/icache) */
    flush_uprobe_xol_access(page, vaddr, dst, len);

    preempt_enable();

    kunmap_local(xol_page_kaddr);
}

pub unsafe fn arch_uprobe_pre_xol(
    auprobe: *mut arch_uprobe,
    regs: *mut pt_regs,
) -> c_int {
    let utask: *mut uprobe_task = (*current).utask;

    if (*auprobe).prehandler.is_some() {
        ((*auprobe).prehandler.unwrap())(auprobe, &mut (*utask).autask, regs);
    }

    (*utask).autask.saved_trap_no = (*current).thread.trap_no;
    (*current).thread.trap_no = UPROBE_TRAP_NR;
    (*regs).ARM_pc = (*utask).xol_vaddr;

    0
}

pub unsafe fn arch_uprobe_post_xol(
    auprobe: *mut arch_uprobe,
    regs: *mut pt_regs,
) -> c_int {
    let utask: *mut uprobe_task = (*current).utask;

    WARN_ON_ONCE((*current).thread.trap_no != UPROBE_TRAP_NR);

    (*current).thread.trap_no = (*utask).autask.saved_trap_no;
    (*regs).ARM_pc = (*utask).vaddr + 4;

    if (*auprobe).posthandler.is_some() {
        ((*auprobe).posthandler.unwrap())(auprobe, &mut (*utask).autask, regs);
    }

    0
}

pub unsafe fn arch_uprobe_xol_was_trapped(t: *mut task_struct) -> bool {
    if (*t).thread.trap_no != UPROBE_TRAP_NR {
        return true;
    }

    false
}

pub unsafe fn arch_uprobe_abort_xol(
    _auprobe: *mut arch_uprobe,
    regs: *mut pt_regs,
) {
    let utask: *mut uprobe_task = (*current).utask;

    (*current).thread.trap_no = (*utask).autask.saved_trap_no;
    instruction_pointer_set(regs, (*utask).vaddr);
}

pub unsafe fn arch_uprobe_exception_notify(
    _self: *mut notifier_block,
    _val: c_ulong,
    _data: *mut c_void,
) -> c_int {
    NOTIFY_DONE
}

unsafe fn uprobe_trap_handler(regs: *mut pt_regs, mut instr: c_uint) -> c_int {
    let mut flags: c_ulong = 0;

    local_irq_save(&mut flags);
    instr &= 0x0fffffff;
    if instr == (UPROBE_SWBP_ARM_INSN & 0x0fffffff) {
        uprobe_pre_sstep_notifier(regs);
    } else if instr == (UPROBE_SS_ARM_INSN & 0x0fffffff) {
        uprobe_post_sstep_notifier(regs);
    }
    local_irq_restore(flags);

    0
}

pub unsafe fn uprobe_get_swbp_addr(regs: *mut pt_regs) -> c_ulong {
    instruction_pointer(regs)
}

static mut uprobes_arm_break_hook: undef_hook = undef_hook {
    instr_mask: 0x0fffffff,
    instr_val: UPROBE_SWBP_ARM_INSN & 0x0fffffff,
    cpsr_mask: PSR_T_BIT | MODE_MASK,
    cpsr_val: USR_MODE,
    fn_: Some(uprobe_trap_handler),
};

static mut uprobes_arm_ss_hook: undef_hook = undef_hook {
    instr_mask: 0x0fffffff,
    instr_val: UPROBE_SS_ARM_INSN & 0x0fffffff,
    cpsr_mask: PSR_T_BIT | MODE_MASK,
    cpsr_val: USR_MODE,
    fn_: Some(uprobe_trap_handler),
};

unsafe fn arch_uprobes_init() -> c_int {
    register_undef_hook(&mut uprobes_arm_break_hook);
    register_undef_hook(&mut uprobes_arm_ss_hook);

    0
}

// device_initcall(arch_uprobes_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
