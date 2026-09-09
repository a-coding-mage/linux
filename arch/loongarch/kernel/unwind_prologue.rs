// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2022 Loongson Technology Corporation Limited
 */

extern "C" {
    static unwind_hint_ade: ::core::ffi::c_int;
    static unwind_hint_ale: ::core::ffi::c_int;
    static unwind_hint_bp: ::core::ffi::c_int;
    static unwind_hint_fpe: ::core::ffi::c_int;
    static unwind_hint_fpu: ::core::ffi::c_int;
    static unwind_hint_lsx: ::core::ffi::c_int;
    static unwind_hint_lasx: ::core::ffi::c_int;
    static unwind_hint_lbt: ::core::ffi::c_int;
    static unwind_hint_ri: ::core::ffi::c_int;
    static unwind_hint_watch: ::core::ffi::c_int;
}

#[inline]
unsafe fn scan_handlers(entry_offset: ::core::ffi::c_ulong) -> bool {
    if entry_offset >= EXCCODE_INT_START * VECSIZE {
        return false;
    }

    let idx = entry_offset / VECSIZE;
    let offset = entry_offset % VECSIZE;
    match idx {
        EXCCODE_ADE => offset == unwind_hint_ade as ::core::ffi::c_ulong,
        EXCCODE_ALE => offset == unwind_hint_ale as ::core::ffi::c_ulong,
        EXCCODE_BP => offset == unwind_hint_bp as ::core::ffi::c_ulong,
        EXCCODE_FPE => offset == unwind_hint_fpe as ::core::ffi::c_ulong,
        EXCCODE_FPDIS => offset == unwind_hint_fpu as ::core::ffi::c_ulong,
        EXCCODE_LSXDIS => offset == unwind_hint_lsx as ::core::ffi::c_ulong,
        EXCCODE_LASXDIS => offset == unwind_hint_lasx as ::core::ffi::c_ulong,
        EXCCODE_BTDIS => offset == unwind_hint_lbt as ::core::ffi::c_ulong,
        EXCCODE_INE => offset == unwind_hint_ri as ::core::ffi::c_ulong,
        EXCCODE_WATCH => offset == unwind_hint_watch as ::core::ffi::c_ulong,
        _ => false,
    }
}

#[inline]
unsafe fn fix_exception(pc: ::core::ffi::c_ulong) -> bool {
    // Preserved build-time condition: CONFIG_NUMA && !CONFIG_PREEMPT_RT.
    #[cfg(all(CONFIG_NUMA, not(CONFIG_PREEMPT_RT)))]
    {
        let mut cpu: ::core::ffi::c_int = 0;
        for_each_possible_cpu!(cpu, {
            if !pcpu_handlers[cpu as usize].is_null()
                && scan_handlers(pc - pcpu_handlers[cpu as usize] as ::core::ffi::c_ulong)
            {
                return true;
            }
        });
    }
    scan_handlers(pc - eentry)
}

#[inline]
unsafe fn fix_ftrace(pc: ::core::ffi::c_ulong) -> bool {
    // Preserved build-time condition: CONFIG_DYNAMIC_FTRACE.
    #[cfg(CONFIG_DYNAMIC_FTRACE)]
    {
        return pc == ftrace_call as ::core::ffi::c_ulong + LOONGARCH_INSN_SIZE;
    }
    #[cfg(not(CONFIG_DYNAMIC_FTRACE))]
    {
        false
    }
}

#[inline]
unsafe fn unwind_state_fixup(state: *mut unwind_state) -> bool {
    if !fix_exception((*state).pc) && !fix_ftrace((*state).pc) {
        return false;
    }
    (*state).reset = true;
    true
}

/*
 * LoongArch function prologue is like follows,
 *     [instructions not use stack var]
 *     addi.d sp, sp, -imm
 *     st.d   xx, sp, offset <- save callee saved regs and
 *     st.d   yy, sp, offset    save ra if function is nest.
 *     [others instructions]
 */
unsafe fn unwind_by_prologue(state: *mut unwind_state) -> bool {
    let mut frame_ra: ::core::ffi::c_long = -1;
    let mut frame_size: ::core::ffi::c_ulong = 0;
    let (mut size, mut offset, mut pc): (::core::ffi::c_ulong, ::core::ffi::c_ulong, ::core::ffi::c_ulong);
    let mut regs: *mut pt_regs;
    let info: *mut stack_info = &mut (*state).stack_info;
    let (mut ip, mut ip_end): (*mut loongarch_instruction, *mut loongarch_instruction);

    if (*state).sp >= (*info).end || (*state).sp < (*info).begin {
        return false;
    }

    if (*state).reset {
        regs = (*state).sp as *mut pt_regs;
        (*state).first = true;
        (*state).reset = false;
        (*state).pc = (*regs).csr_era;
        (*state).ra = (*regs).regs[1];
        (*state).sp = (*regs).regs[3];
        return true;
    }

    pc = (*state).pc - if (*state).first { 0 } else { LOONGARCH_INSN_SIZE };
    if !kallsyms_lookup_size_offset(pc, &mut size, &mut offset) {
        return false;
    }

    ip = (pc - offset) as *mut loongarch_instruction;
    ip_end = pc as *mut loongarch_instruction;
    while ip < ip_end {
        if is_stack_alloc_ins(ip) {
            frame_size = (1u64 << 12) - (*ip).reg2i12_format.immediate as ::core::ffi::c_ulong;
            ip = ip.add(1);
            break;
        }
        ip = ip.add(1);
    }

    if frame_size == 0 {
        if (*state).first {
            goto_first(state);
        }
        return false;
    }

    while ip < ip_end {
        if is_ra_save_ins(ip) {
            frame_ra = (*ip).reg2i12_format.immediate as ::core::ffi::c_long;
            break;
        }
        if is_branch_ins(ip) {
            break;
        }
        ip = ip.add(1);
    }

    if frame_ra < 0 {
        if (*state).first {
            (*state).sp += frame_size;
            goto_first(state);
        }
        return false;
    }

    (*state).pc = *((*state).sp + frame_ra as ::core::ffi::c_ulong) as *const ::core::ffi::c_ulong;
    (*state).sp += frame_size;
    goto_out(state);
    (*state).first = false;
    unwind_state_fixup(state) || __kernel_text_address((*state).pc)
}

#[inline(always)]
unsafe fn goto_first(state: *mut unwind_state) {
    (*state).pc = (*state).ra;
}

#[inline(always)]
unsafe fn goto_out(_state: *mut unwind_state) {}

unsafe fn next_frame(state: *mut unwind_state) -> bool {
    let mut pc: ::core::ffi::c_ulong;
    let mut regs: *mut pt_regs;
    let info: *mut stack_info = &mut (*state).stack_info;
    if unwind_done(state) { return false; }
    loop {
        if unwind_by_prologue(state) {
            (*state).pc = unwind_graph_addr(state, (*state).pc, (*state).sp);
            return true;
        }
        if (*info).type_ == STACK_TYPE_IRQ && (*info).end == (*state).sp {
            regs = (*info).next_sp as *mut pt_regs;
            pc = (*regs).csr_era;
            if user_mode(regs) || !__kernel_text_address(pc) { break; }
            (*state).first = true;
            (*state).pc = pc;
            (*state).ra = (*regs).regs[1];
            (*state).sp = (*regs).regs[3];
            get_stack_info((*state).sp, (*state).task, info);
            return true;
        }
        (*state).sp = (*info).next_sp;
        if !get_stack_info((*state).sp, (*state).task, info) { break; }
    }
    (*state).stack_info.type_ = STACK_TYPE_UNKNOWN;
    false
}

#[no_mangle]
pub unsafe extern "C" fn unwind_get_return_address(state: *mut unwind_state) -> ::core::ffi::c_ulong {
    __unwind_get_return_address(state)
}

#[no_mangle]
pub unsafe extern "C" fn unwind_start(state: *mut unwind_state, task: *mut task_struct, regs: *mut pt_regs) {
    __unwind_start(state, task, regs);
    (*state).type_ = UNWINDER_PROLOGUE;
    (*state).first = true;
    if !__kernel_text_address((*state).pc) {
        (*state).type_ = UNWINDER_GUESS;
        if !unwind_done(state) { unwind_next_frame(state); }
    }
}

#[no_mangle]
pub unsafe extern "C" fn unwind_next_frame(state: *mut unwind_state) -> bool {
    if (*state).type_ == UNWINDER_PROLOGUE { next_frame(state) } else { default_next_frame(state) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
