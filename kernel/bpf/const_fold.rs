// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */

// Dependency declarations and BPF constants/types are supplied by the surrounding verifier.

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum ConstArgState {
    ConstArgUnvisited,
    ConstArgUnknown,
    ConstArgConst,
    ConstArgMapPtr,
    ConstArgMapValue,
    ConstArgSubprog,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ConstArgInfo {
    state: ConstArgState,
    map_index: u32,
    val: u64,
}

unsafe fn ci_is_unvisited(ci: *const ConstArgInfo) -> bool {
    (*ci).state == ConstArgState::ConstArgUnvisited
}

unsafe fn ci_is_unknown(ci: *const ConstArgInfo) -> bool {
    (*ci).state == ConstArgState::ConstArgUnknown
}

unsafe fn ci_is_const(ci: *const ConstArgInfo) -> bool {
    (*ci).state == ConstArgState::ConstArgConst
}

unsafe fn ci_is_map_value(ci: *const ConstArgInfo) -> bool {
    (*ci).state == ConstArgState::ConstArgMapValue
}

unsafe fn const_reg_xfer(
    env: *mut bpf_verifier_env,
    ci_out: *mut ConstArgInfo,
    insn: *mut bpf_insn,
    insns: *mut bpf_insn,
    idx: i32,
) {
    let unknown = ConstArgInfo { state: ConstArgState::ConstArgUnknown, map_index: 0, val: 0 };
    let dst = ci_out.add((*insn).dst_reg as usize);
    let src = ci_out.add((*insn).src_reg as usize);
    let class = BPF_CLASS((*insn).code);
    let mode = BPF_MODE((*insn).code);
    let opcode = BPF_OP((*insn).code) | BPF_SRC((*insn).code);
    let mut r: i32;

    if is_stack_arg_st(insn) || is_stack_arg_stx(insn) { return; }
    if is_stack_arg_ldx(insn) { *dst = unknown; return; }

    match class {
        BPF_ALU | BPF_ALU64 => {
            match opcode {
                x if x == (BPF_MOV | BPF_K) => { (*dst).state = ConstArgState::ConstArgConst; (*dst).val = (*insn).imm as i64 as u64; }
                x if x == (BPF_MOV | BPF_X) => {
                    *dst = *src;
                    if (*insn).off != 0 {
                        if !ci_is_const(dst) { *dst = unknown; }
                        else { match (*insn).off { 8 => (*dst).val = (*dst).val as i8 as u64, 16 => (*dst).val = (*dst).val as i16 as u64, 32 => (*dst).val = (*dst).val as i32 as u64, _ => *dst = unknown } }
                    }
                }
                x if x == (BPF_ADD | BPF_K) => { if !ci_is_const(dst) && !ci_is_map_value(dst) { *dst = unknown; } else { (*dst).val = (*dst).val.wrapping_add((*insn).imm as i64 as u64); } }
                x if x == (BPF_SUB | BPF_K) => { if !ci_is_const(dst) && !ci_is_map_value(dst) { *dst = unknown; } else { (*dst).val = (*dst).val.wrapping_sub((*insn).imm as i64 as u64); } }
                x if x == (BPF_AND | BPF_K) => { if !ci_is_const(dst) { if (*insn).imm == 0 { (*dst).state = ConstArgState::ConstArgConst; (*dst).val = 0; } else { *dst = unknown; } } else { (*dst).val &= (*insn).imm as i64 as u64; } }
                x if x == (BPF_AND | BPF_X) => {
                    if ci_is_const(dst) && (*dst).val == 0 {} else if ci_is_const(src) && (*src).val == 0 { (*dst).state = ConstArgState::ConstArgConst; (*dst).val = 0; } else if !ci_is_const(dst) || !ci_is_const(src) { *dst = unknown; } else { (*dst).val &= (*src).val; }
                }
                _ => *dst = unknown,
            }
            if class == BPF_ALU { if ci_is_const(dst) { (*dst).val = (*dst).val as u32 as u64; } else if !ci_is_unknown(dst) { *dst = unknown; } }
        }
        BPF_LD => {
            if mode == BPF_ABS || mode == BPF_IND { for r in BPF_REG_0..=BPF_REG_5 { *ci_out.add(r as usize) = unknown; } return; }
            if mode == BPF_IMM && BPF_SIZE((*insn).code) == BPF_DW {
                if (*insn).src_reg == BPF_PSEUDO_FUNC { let subprog = bpf_find_subprog(env, idx + (*insn).imm + 1); if subprog >= 0 { (*dst).state = ConstArgState::ConstArgSubprog; (*dst).val = subprog as u64; } else { *dst = unknown; } }
                else if (*insn).src_reg == BPF_PSEUDO_MAP_VALUE || (*insn).src_reg == BPF_PSEUDO_MAP_IDX_VALUE { (*dst).state = ConstArgState::ConstArgMapValue; (*dst).map_index = (*(*env).insn_aux_data.add(idx as usize)).map_index; (*dst).val = (*(*env).insn_aux_data.add(idx as usize)).map_off; }
                else if (*insn).src_reg == BPF_PSEUDO_MAP_FD || (*insn).src_reg == BPF_PSEUDO_MAP_IDX { (*dst).state = ConstArgState::ConstArgMapPtr; (*dst).map_index = (*(*env).insn_aux_data.add(idx as usize)).map_index; }
                else if (*insn).src_reg == 0 { (*dst).state = ConstArgState::ConstArgConst; (*dst).val = (*insn).imm as u32 as u64 | (((*insns.add(idx as usize + 1)).imm as u32 as u64) << 32); }
                else { *dst = unknown; }
            }
        }
        BPF_LDX => { if !ci_is_map_value(src) { *dst = unknown; } else { let map = *(*env).used_maps.add((*src).map_index as usize); let size = bpf_size_to_bytes(BPF_SIZE((*insn).code)); let is_ldsx = mode == BPF_MEMSX; let off = (*src).val as i64 + (*insn).off as i64; let mut val = 0u64; if !bpf_map_is_rdonly(map) || (*map).ops.map_direct_value_addr.is_none() || off < 0 || off + size as i64 > (*map).value_size as i64 || bpf_map_direct_read(map, off as i32, size, &mut val, is_ldsx) != 0 { *dst = unknown; } else { (*dst).state = ConstArgState::ConstArgConst; (*dst).val = val; } } }
        BPF_JMP => { if opcode == BPF_CALL { for r in BPF_REG_0..=BPF_REG_5 { *ci_out.add(r as usize) = unknown; } } }
        BPF_STX => { r = bpf_atomic_load_reg(insn); if r >= 0 { *ci_out.add(r as usize) = unknown; } }
        _ => {}
    }
}

unsafe fn const_reg_join(ci_target: *mut ConstArgInfo, ci_out: *mut ConstArgInfo) -> bool {
    let mut changed = false;
    for r in 0..MAX_BPF_REG as usize { let old = ci_target.add(r); let new = ci_out.add(r); if ci_is_unvisited(old) && !ci_is_unvisited(new) { *old = *new; changed = true; } else if !ci_is_unknown(old) && !ci_is_unvisited(old) && ((*new).state != (*old).state || (*new).val != (*old).val || (*new).map_index != (*old).map_index) { (*old).state = ConstArgState::ConstArgUnknown; changed = true; } }
    changed
}

// The remaining verifier integration functions are declared by the surrounding translation unit.
extern "C" {
    fn bpf_compute_postorder(env: *mut bpf_verifier_env) -> i32;
}

pub unsafe fn bpf_compute_const_regs(env: *mut bpf_verifier_env) -> i32 {
    let unknown = ConstArgInfo { state: ConstArgState::ConstArgUnknown, map_index: 0, val: 0 };
    let insn_cnt = (*(*env).prog).len as usize;
    let insns = (*(*env).prog).insnsi;
    let ci_in = kvzalloc_objs::<[ConstArgInfo; MAX_BPF_REG as usize]>(insn_cnt, GFP_KERNEL_ACCOUNT);
    if ci_in.is_null() { return -ENOMEM; }
    for i in 0..(*env).subprog_cnt as usize { let start = (*env).subprog_info.add(i).read().start as usize; for r in 0..MAX_BPF_REG as usize { (*ci_in.add(start)).as_mut_ptr().add(r).write(unknown); } }
    loop {
        let mut changed = false;
        for pi in (0..(*env).cfg.cur_postorder as usize).rev() {
            let idx = *(*env).cfg.insn_postorder.add(pi) as usize;
            let ci = ci_in.add(idx);
            let mut ci_out = [unknown; MAX_BPF_REG as usize];
            for r in 0..MAX_BPF_REG as usize { ci_out[r] = (*ci).as_ptr().add(r).read(); }
            const_reg_xfer(env, ci_out.as_mut_ptr(), insns.add(idx), insns, idx as i32);
            let succ = bpf_insn_successors(env, idx as i32);
            for s in 0..(*succ).cnt as usize { changed |= const_reg_join(ci_in.add(*(*succ).items.add(s) as usize), ci_out.as_mut_ptr()); }
        }
        if !changed { break; }
    }
    for i in 0..insn_cnt { let aux = (*env).insn_aux_data.add(i); let mut mask=0u16; let mut map_mask=0u16; let mut subprog_mask=0u16; for r in BPF_REG_0 as usize..(*aux).const_reg_vals.len() { let c=(*ci_in.add(i)).as_ptr().add(r).read(); match c.state { ConstArgState::ConstArgConst if c.val == c.val as u32 as u64 => { mask |= 1<<r; (*aux).const_reg_vals[r]=c.val; }, ConstArgState::ConstArgMapPtr => { map_mask |= 1<<r; (*aux).const_reg_vals[r]=c.map_index as u64; }, ConstArgState::ConstArgSubprog => { subprog_mask |= 1<<r; (*aux).const_reg_vals[r]=c.val; }, _ => {} } } (*aux).const_reg_mask=mask; (*aux).const_reg_map_mask=map_mask; (*aux).const_reg_subprog_mask=subprog_mask; }
    kvfree(ci_in as *mut core::ffi::c_void); 0
}

unsafe fn eval_const_branch(opcode: u8, dst: u64, src: u64) -> i32 {
    match BPF_OP(opcode) { BPF_JEQ=> (dst==src) as i32, BPF_JNE=>(dst!=src) as i32, BPF_JGT=>(dst>src) as i32, BPF_JGE=>(dst>=src) as i32, BPF_JLT=>(dst<src) as i32, BPF_JLE=>(dst<=src) as i32, BPF_JSGT=>((dst as i64)>(src as i64)) as i32, BPF_JSGE=>((dst as i64)>=(src as i64)) as i32, BPF_JSLT=>((dst as i64)<(src as i64)) as i32, BPF_JSLE=>((dst as i64)<=(src as i64)) as i32, BPF_JSET=>((dst&src)!=0) as i32, _=>-1 }
}

pub unsafe fn bpf_prune_dead_branches(env: *mut bpf_verifier_env) -> i32 {
    let mut changed=false; let n=(*(*env).prog).len as usize; let insns=(*(*env).prog).insnsi;
    for i in 0..n { let aux=(*env).insn_aux_data.add(i); let insn=insns.add(i); if !bpf_insn_is_cond_jump((*insn).code) || bpf_is_may_goto_insn(insn) || ((*aux).const_reg_mask & (1<<(*insn).dst_reg))==0 { continue; } let dv=(*aux).const_reg_vals[(*insn).dst_reg as usize]; let sv=if BPF_SRC((*insn).code)==BPF_K { (*insn).imm as i64 as u64 } else { if ((*aux).const_reg_mask & (1<<(*insn).src_reg))==0 { continue; } (*aux).const_reg_vals[(*insn).src_reg as usize] }; let t=eval_const_branch((*insn).code,dv,sv); if t<0 { return -EFAULT; } (*insn)=BPF_JMP_A(if t!=0 {(*insn).off} else {0}); changed=true; }
    if !changed { return 0; } kvfree((*env).cfg.insn_postorder as *mut core::ffi::c_void); (*env).cfg.insn_postorder=core::ptr::null_mut(); bpf_compute_postorder(env)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
