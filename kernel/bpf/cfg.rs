// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */

// Kernel headers and diagnostics.h supply the referenced types, constants, and helpers.

const DISCOVERED: i32 = 0x10;
const EXPLORED: i32 = 0x20;
const FALLTHROUGH: i32 = 1;
const BRANCH: i32 = 2;
const DONE_EXPLORING: i32 = 0;
const KEEP_EXPLORING: i32 = 1;

unsafe fn mark_subprog_changes_pkt_data(env: *mut bpf_verifier_env, off: i32) {
    let subprog = bpf_find_containing_subprog(env, off);
    (*subprog).changes_pkt_data = true;
}
unsafe fn mark_subprog_might_sleep(env: *mut bpf_verifier_env, off: i32) {
    let subprog = bpf_find_containing_subprog(env, off);
    (*subprog).might_sleep = true;
}
unsafe fn mark_subprog_might_throw(env: *mut bpf_verifier_env, off: i32) {
    let subprog = bpf_find_containing_subprog(env, off);
    (*subprog).might_throw = true;
}

unsafe fn merge_callee_effects(env: *mut bpf_verifier_env, t: i32, w: i32) {
    let caller = bpf_find_containing_subprog(env, t);
    let callee = bpf_find_containing_subprog(env, w);
    (*caller).changes_pkt_data |= (*callee).changes_pkt_data;
    (*caller).might_sleep |= (*callee).might_sleep;
    (*caller).might_throw |= (*callee).might_throw;
}

unsafe fn push_insn(t: i32, w: i32, e: i32, env: *mut bpf_verifier_env) -> i32 {
    let insn_stack = (*env).cfg.insn_stack;
    let insn_state = (*env).cfg.insn_state;
    if e == FALLTHROUGH && *insn_state.add(t as usize) >= (DISCOVERED | FALLTHROUGH) { return DONE_EXPLORING; }
    if e == BRANCH && *insn_state.add(t as usize) >= (DISCOVERED | BRANCH) { return DONE_EXPLORING; }
    if w < 0 || w >= (*(*env).prog).len {
        verbose_linfo(env, t, "%d: ");
        verbose(env, "jump out of range from insn %d to %d\n", t, w);
        bpf_diag_program_structure(env, t, "jump out of range", "Keep branch targets inside the program.", "Instruction %d jumps to instruction %d, but the program only contains instructions 0 through %d.", t, w, (*(*env).prog).len - 1);
        return -EINVAL;
    }
    if e == BRANCH { mark_prune_point(env, w); mark_jmp_point(env, w); }
    if *insn_state.add(w as usize) == 0 {
        *insn_state.add(t as usize) = DISCOVERED | e;
        *insn_state.add(w as usize) = DISCOVERED;
        if (*env).cfg.cur_stack >= (*(*env).prog).len { return -E2BIG; }
        *insn_stack.add((*env).cfg.cur_stack as usize) = w;
        (*env).cfg.cur_stack += 1;
        return KEEP_EXPLORING;
    } else if (*insn_state.add(w as usize) & 0xF0) == DISCOVERED {
        if (*env).bpf_capable { return DONE_EXPLORING; }
        verbose_linfo(env, t, "%d: "); verbose_linfo(env, w, "%d: ");
        verbose(env, "back-edge from insn %d to %d\n", t, w);
        bpf_diag_program_structure(env, t, "back-edge is not allowed", "Load with privileges that allow this back-edge, or rewrite the control flow so it does not branch backward.", "Instruction %d branches back to instruction %d. This program is being rejected without the privilege needed for this back-edge.", t, w);
        return -EINVAL;
    } else if *insn_state.add(w as usize) == EXPLORED {
        *insn_state.add(t as usize) = DISCOVERED | e;
    } else { verifier_bug(env, "insn state internal bug"); return -EFAULT; }
    DONE_EXPLORING
}

unsafe fn visit_func_call_insn(t: i32, insns: *mut bpf_insn, env: *mut bpf_verifier_env, visit_callee: bool) -> i32 {
    let insn_sz = if bpf_is_ldimm64(insns.add(t as usize)) { 2 } else { 1 };
    let mut ret = push_insn(t, t + insn_sz, FALLTHROUGH, env); if ret != 0 { return ret; }
    mark_prune_point(env, t + insn_sz); mark_jmp_point(env, t + insn_sz);
    if visit_callee { let w = t + (*insns.add(t as usize)).imm + 1; mark_prune_point(env, t); merge_callee_effects(env, t, w); ret = push_insn(t, w, BRANCH, env); }
    ret
}

pub unsafe fn bpf_iarray_realloc(old: *mut bpf_iarray, n_elem: usize) -> *mut bpf_iarray {
    let new_size = core::mem::size_of::<bpf_iarray>() + n_elem * core::mem::size_of::<u32>();
    let new = kvrealloc(old as *mut _, new_size, GFP_KERNEL_ACCOUNT) as *mut bpf_iarray;
    if new.is_null() { kvfree(old as *mut _); return core::ptr::null_mut(); }
    (*new).cnt = n_elem as u32; new
}

unsafe fn cmp_ptr_to_u32(a: *const core::ffi::c_void, b: *const core::ffi::c_void) -> i32 { (*(a as *const u32)).wrapping_sub(*(b as *const u32)) as i32 }
unsafe fn sort_insn_array_uniq(items: *mut u32, cnt: i32) -> i32 {
    sort(items as *mut _, cnt as usize, core::mem::size_of::<u32>(), cmp_ptr_to_u32, core::ptr::null_mut());
    let mut unique = 1; let mut i = 1;
    while i < cnt { if *items.add(i as usize) != *items.add((unique - 1) as usize) { *items.add(unique as usize) = *items.add(i as usize); unique += 1; } i += 1; }
    unique
}

pub unsafe fn bpf_copy_insn_array_uniq(map: *mut bpf_map, start: u32, end: u32, off: *mut u32) -> i32 {
    let mut i = start; while i <= end { let value = (*(*map).ops).map_lookup_elem(map, &i); if IS_ERR(value) { return PTR_ERR(value); } if value.is_null() { return -EINVAL; } *off.add((i-start) as usize) = (*value).xlated_off; i += 1; }
    sort_insn_array_uniq(off, (end - start + 1) as i32)
}

unsafe fn visit_insn(t: i32, env: *mut bpf_verifier_env) -> i32 {
    let insns = (*(*env).prog).insnsi; let insn = insns.add(t as usize); let mut ret; let insn_sz;
    if bpf_pseudo_func(insn) { return visit_func_call_insn(t, insns, env, true); }
    if BPF_CLASS((*insn).code) != BPF_JMP && BPF_CLASS((*insn).code) != BPF_JMP32 {
        if BPF_CLASS((*insn).code) == BPF_LD && (BPF_MODE((*insn).code) == BPF_ABS || BPF_MODE((*insn).code) == BPF_IND) { ret = visit_abnormal_return_insn(env, t); if ret != 0 { return ret; } }
        insn_sz = if bpf_is_ldimm64(insn) { 2 } else { 1 }; return push_insn(t, t + insn_sz, FALLTHROUGH, env);
    }
    match BPF_OP((*insn).code) {
        BPF_EXIT => DONE_EXPLORING,
        BPF_CALL => visit_func_call_insn(t, insns, env, (*insn).src_reg == BPF_PSEUDO_CALL),
        BPF_JA => { if BPF_SRC((*insn).code) == BPF_X { return visit_gotox_insn(t, env); } let off = if BPF_CLASS((*insn).code) == BPF_JMP { (*insn).off as i32 } else { (*insn).imm }; ret = push_insn(t, t + off + 1, FALLTHROUGH, env); if ret != 0 { return ret; } mark_prune_point(env, t+off+1); mark_jmp_point(env, t+off+1); ret },
        _ => { mark_prune_point(env, t); ret = push_insn(t, t+1, FALLTHROUGH, env); if ret != 0 { return ret; } push_insn(t, t + (*insn).off as i32 + 1, BRANCH, env) }
    }
}

pub unsafe fn bpf_check_cfg(env: *mut bpf_verifier_env) -> i32 {
    let n = (*(*env).prog).len; let state = kvzalloc_objs::<i32>(n, GFP_KERNEL_ACCOUNT); if state.is_null() { return -ENOMEM; }
    let stack = kvzalloc_objs::<i32>(n, GFP_KERNEL_ACCOUNT); if stack.is_null() { kvfree(state as *mut _); return -ENOMEM; }
    (*env).cfg.insn_state = state; (*env).cfg.insn_stack = stack; *state = DISCOVERED; *stack = 0; (*env).cfg.cur_stack = 1;
    while (*env).cfg.cur_stack > 0 { let t = *stack.add(((*env).cfg.cur_stack-1) as usize); let ret = visit_insn(t, env); match ret { DONE_EXPLORING => { *state.add(t as usize)=EXPLORED; (*env).cfg.cur_stack-=1; }, KEEP_EXPLORING => {}, _ => { kvfree(state as *mut _); kvfree(stack as *mut _); (*env).cfg.insn_state=core::ptr::null_mut(); (*env).cfg.insn_stack=core::ptr::null_mut(); return if ret>0 {-EFAULT} else {ret}; } } }
    for i in 0..n { if *state.add(i as usize) != EXPLORED { kvfree(state as *mut _); kvfree(stack as *mut _); return -EINVAL; } }
    kvfree(state as *mut _); kvfree(stack as *mut _); (*env).cfg.insn_state=core::ptr::null_mut(); (*env).cfg.insn_stack=core::ptr::null_mut(); 0
}

pub unsafe fn bpf_compute_postorder(env: *mut bpf_verifier_env) -> i32 {
    let n = (*(*env).prog).len as usize;
    let postorder = kvzalloc_objs::<i32>(n, GFP_KERNEL_ACCOUNT);
    let state = kvzalloc_objs::<i32>(n, GFP_KERNEL_ACCOUNT);
    let stack = kvzalloc_objs::<i32>(n, GFP_KERNEL_ACCOUNT);
    if postorder.is_null() || state.is_null() || stack.is_null() { kvfree(postorder as *mut _); kvfree(state as *mut _); kvfree(stack as *mut _); return -ENOMEM; }
    let mut cur = 0u32;
    for i in 0..(*env).subprog_cnt as usize {
        (*(*env).subprog_info.add(i)).postorder_start = cur;
        *stack = (*(*env).subprog_info.add(i)).start; let mut sz = 1usize;
        while sz != 0 { let top = *stack.add(sz-1); *state.add(top as usize) |= DISCOVERED;
            if *state.add(top as usize) & EXPLORED != 0 { *postorder.add(cur as usize)=top; cur+=1; sz-=1; continue; }
            let succ = bpf_insn_successors(env, top); for s in 0..(*succ).cnt as usize { let w=*(*succ).items.add(s); if *state.add(w as usize)==0 { *stack.add(sz)=w; sz+=1; *state.add(w as usize)|=DISCOVERED; } }
            *state.add(top as usize)|=EXPLORED;
        }
    }
    (*(*env).subprog_info.add((*env).subprog_cnt as usize)).postorder_start=cur; (*env).cfg.insn_postorder=postorder; (*env).cfg.cur_postorder=cur;
    kvfree(stack as *mut _); kvfree(state as *mut _); 0
}

pub unsafe fn bpf_compute_scc(env: *mut bpf_verifier_env) -> i32 {
    let n=(*(*env).prog).len as usize; let pre=kvzalloc_objs::<u32>(n,GFP_KERNEL_ACCOUNT); let low=kvzalloc_objs::<u32>(n,GFP_KERNEL_ACCOUNT); let stack=kvcalloc(n,core::mem::size_of::<u32>(),GFP_KERNEL_ACCOUNT) as *mut u32; if pre.is_null()||low.is_null()||stack.is_null(){return -ENOMEM;}
    let mut ps=0usize; let mut next=1u32; let mut sid=1u32;
    for root in 0..n { if *pre.add(root)!=0 {continue;} let mut dfs=[root as u32]; while let Some(&w)=dfs.last(){ if *pre.add(w as usize)==0 { *pre.add(w as usize)=next;*low.add(w as usize)=next;next+=1;*stack.add(ps)=w;ps+=1; } let succ=bpf_insn_successors(env,w); let mut pushed=false; for j in 0..(*succ).cnt as usize {let s=*(*succ).items.add(j);if *pre.add(s as usize)==0 {dfs.push(s);pushed=true;break;} *low.add(w as usize)=core::cmp::min(*low.add(w as usize),*low.add(s as usize));} if pushed{continue;} if *low.add(w as usize)==*pre.add(w as usize){loop{ps-=1;let t=*stack.add(ps);(*env).insn_aux_data.add(t as usize).scc=sid;if t==w{break;}}sid+=1;} dfs.pop(); }} (*env).scc_cnt=sid; kvfree(pre as *mut _);kvfree(low as *mut _);kvfree(stack as *mut _);0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
