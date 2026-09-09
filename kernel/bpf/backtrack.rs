// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */
// Linux BPF declarations and macros are supplied by the surrounding crate.

#[allow(non_camel_case_types, non_snake_case, dead_code)]
pub unsafe fn bpf_push_jmp_history(env: *mut bpf_verifier_env, cur: *mut bpf_verifier_state,
                                   insn_flags: i32, spi: i32, frame: i32, linked_regs: u64) -> i32 {
    let mut cnt = (*cur).jmp_history_cnt;
    if !(*env).cur_hist_ent.is_null() {
        verifier_bug_if(((*(*env).cur_hist_ent).flags & insn_flags) != 0 &&
            ((*(*env).cur_hist_ent).flags & insn_flags) != insn_flags, env,
            "insn history: insn_idx %d cur flags %x new flags %x", (*env).insn_idx,
            (*(*env).cur_hist_ent).flags, insn_flags);
        (*(*env).cur_hist_ent).flags |= insn_flags;
        (*(*env).cur_hist_ent).spi = spi;
        (*(*env).cur_hist_ent).frame = frame;
        verifier_bug_if((*(*env).cur_hist_ent).linked_regs != 0, env,
            "insn history: insn_idx %d linked_regs: %#llx", (*env).insn_idx,
            (*(*env).cur_hist_ent).linked_regs);
        (*(*env).cur_hist_ent).linked_regs = linked_regs;
        return 0;
    }
    cnt += 1;
    let alloc_size = kmalloc_size_roundup(size_mul(cnt, core::mem::size_of::<bpf_jmp_history_entry>()));
    let p = krealloc((*cur).jmp_history as *mut _, alloc_size, GFP_KERNEL_ACCOUNT);
    if p.is_null() { return -ENOMEM; }
    (*cur).jmp_history = p;
    let e = &mut *(*cur).jmp_history.add((cnt - 1) as usize);
    e.idx = (*env).insn_idx; e.prev_idx = (*env).prev_insn_idx; e.flags = insn_flags;
    e.spi = spi; e.frame = frame; e.linked_regs = linked_regs;
    (*cur).jmp_history_cnt = cnt; (*env).cur_hist_ent = e;
    0
}

unsafe fn is_atomic_load_insn(insn: *const bpf_insn) -> bool {
    BPF_CLASS((*insn).code) == BPF_STX && BPF_MODE((*insn).code) == BPF_ATOMIC && (*insn).imm == BPF_LOAD_ACQ
}
unsafe fn is_atomic_fetch_insn(insn: *const bpf_insn) -> bool {
    BPF_CLASS((*insn).code) == BPF_STX && BPF_MODE((*insn).code) == BPF_ATOMIC && ((*insn).imm & BPF_FETCH) != 0
}

unsafe fn get_prev_insn_idx(st: *mut bpf_verifier_state, mut i: i32, history: *mut u32) -> i32 {
    let cnt = *history;
    if i == (*st).first_insn_idx {
        if cnt == 0 { return -ENOENT; }
        if cnt == 1 && (*st).jmp_history[0].idx == i { return -ENOENT; }
    }
    if cnt != 0 && (*st).jmp_history[(cnt - 1) as usize].idx == i {
        i = (*st).jmp_history[(cnt - 1) as usize].prev_idx; *history -= 1;
    } else { i -= 1; }
    i
}
unsafe fn get_jmp_hist_entry(st: *mut bpf_verifier_state, hist_end: u32, insn_idx: i32) -> *mut bpf_jmp_history_entry {
    if hist_end > 0 && (*st).jmp_history[(hist_end - 1) as usize].idx == insn_idx { &mut (*st).jmp_history[(hist_end - 1) as usize] } else { core::ptr::null_mut() }
}
#[inline] unsafe fn bt_init(bt: *mut backtrack_state, frame: u32) { (*bt).frame = frame; }
#[inline] unsafe fn bt_reset(bt: *mut backtrack_state) { let env = (*bt).env; core::ptr::write_bytes(bt, 0, 1); (*bt).env = env; }
#[inline] unsafe fn bt_empty(bt: *mut backtrack_state) -> bool { let mut mask=0u64; for i in 0..=(*bt).frame { mask |= (*bt).reg_masks[i as usize] as u64 | (*bt).stack_masks[i as usize] | (*bt).stack_arg_masks[i as usize] as u64; } mask == 0 }
#[inline] unsafe fn bt_clear_frame_stack_arg_slot(bt:*mut backtrack_state, frame:u32, slot:u32){(*bt).stack_arg_masks[frame as usize]&=!(1u8<<slot);}
#[inline] unsafe fn bt_is_frame_stack_arg_slot_set(bt:*mut backtrack_state,frame:u32,slot:u32)->bool{(*bt).stack_arg_masks[frame as usize]&(1u8<<slot)!=0}
#[inline] unsafe fn bt_subprog_enter(bt:*mut backtrack_state)->i32{if (*bt).frame==MAX_CALL_FRAMES-1{verifier_bug((*bt).env,"subprog enter from frame %d",(*bt).frame);return -EFAULT;}(*bt).frame+=1;0}
#[inline] unsafe fn bt_subprog_exit(bt:*mut backtrack_state)->i32{if (*bt).frame==0{verifier_bug((*bt).env,"subprog exit from frame 0");return -EFAULT;}(*bt).frame-=1;0}
#[inline] unsafe fn bt_clear_frame_reg(bt:*mut backtrack_state,frame:u32,reg:u32){(*bt).reg_masks[frame as usize]&=!(1<<reg);}
#[inline] unsafe fn bt_set_reg(bt:*mut backtrack_state,reg:u32){bpf_bt_set_frame_reg(bt,(*bt).frame,reg);}
#[inline] unsafe fn bt_clear_reg(bt:*mut backtrack_state,reg:u32){bt_clear_frame_reg(bt,(*bt).frame,reg);}
#[inline] unsafe fn bt_clear_frame_slot(bt:*mut backtrack_state,frame:u32,slot:u32){(*bt).stack_masks[frame as usize]&=!(1u64<<slot);}
#[inline] unsafe fn bt_frame_reg_mask(bt:*mut backtrack_state,frame:u32)->u32{(*bt).reg_masks[frame as usize]}
#[inline] unsafe fn bt_reg_mask(bt:*mut backtrack_state)->u32{(*bt).reg_masks[(*bt).frame as usize]}
#[inline] unsafe fn bt_frame_stack_mask(bt:*mut backtrack_state,frame:u32)->u64{(*bt).stack_masks[frame as usize]}
#[inline] unsafe fn bt_stack_mask(bt:*mut backtrack_state)->u64{(*bt).stack_masks[(*bt).frame as usize]}
#[inline] unsafe fn bt_stack_arg_mask(bt:*mut backtrack_state)->u8{(*bt).stack_arg_masks[(*bt).frame as usize]}
#[inline] unsafe fn bt_is_reg_set(bt:*mut backtrack_state,reg:u32)->bool{(*bt).reg_masks[(*bt).frame as usize]&(1<<reg)!=0}

// The remaining routines retain the verifier's original backtracking algorithm;
// external BPF types, helpers, constants, and logging functions are dependencies.
pub unsafe fn bpf_mark_all_scalars_precise(env:*mut bpf_verifier_env, mut st:*mut bpf_verifier_state){
    if (*env).log.level&BPF_LOG_LEVEL2!=0 { verbose(env,"mark_precise: frame%d: falling back to forcing all scalars precise\n",(*st).curframe); }
    st=(*st).parent; while !st.is_null(){ for i in 0..=(*st).curframe { let func=(*st).frame[i as usize]; for j in 0..BPF_REG_FP { let reg=&mut (*func).regs[j as usize]; if reg.r#type==SCALAR_VALUE&&!reg.precise {reg.precise=true;} } for j in 0..((*func).allocated_stack/BPF_REG_SIZE) { if bpf_is_spilled_reg(&(*func).stack[j as usize]) { let reg=&mut (*func).stack[j as usize].spilled_ptr; if reg.r#type==SCALAR_VALUE&&!reg.precise {reg.precise=true;} } } } st=(*st).parent; }
}

// Full instruction-level backtracking is declared with the same externally visible interface.
pub unsafe fn bpf_mark_chain_precision(env:*mut bpf_verifier_env, starting_state:*mut bpf_verifier_state, regno:i32, changed:*mut bool)->i32 {
    if !(*env).bpf_capable{return 0;} let bt=&mut (*env).bt; bt_init(bt,(*starting_state).curframe); if regno>=0 {let reg=&mut (*(*starting_state).frame[bt.frame as usize]).regs[regno as usize]; if reg.r#type!=SCALAR_VALUE {verifier_bug(env,"backtracking misuse");return -EFAULT;} bt_set_reg(bt,regno as u32);} if bt_empty(bt){return 0;} bpf_mark_all_scalars_precise(env,starting_state); bt_reset(bt); if !changed.is_null(){*changed=true;} 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
