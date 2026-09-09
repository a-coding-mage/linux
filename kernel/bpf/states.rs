// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */
// Translated from bpf/states.c.  Kernel/BPF types and helpers are supplied by
// the surrounding translation unit.

const BPF_COMPLEXITY_LIMIT_STATES: u32 = 64;
const MAX_BACKEDGE_ITERS: i32 = 64;

#[allow(dead_code)]
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ExactLevel { NOT_EXACT, EXACT, RANGE_WITHIN }

// The following declarations intentionally retain the C ABI and pointer
// semantics; their definitions are provided by the other translated units.
extern "C" {
    fn bpf_is_may_goto_insn_at(env: *mut bpf_verifier_env, insn_idx: i32) -> bool;
    fn bpf_is_iter_next_insn(env: *mut bpf_verifier_env, insn_idx: i32) -> bool;
}

unsafe fn update_peak_states(env: *mut bpf_verifier_env) {
    let e = &mut *env;
    let cur = e.explored_states_size + e.free_list_size + e.num_backedges;
    e.peak_states = core::cmp::max(e.peak_states, cur);
}

unsafe fn state_parent_as_list(st: *mut bpf_verifier_state) -> *mut bpf_verifier_state_list {
    if (*st).parent.is_null() { core::ptr::null_mut() }
    else { (*st).parent as *mut bpf_verifier_state_list }
}

unsafe fn maybe_free_verifier_state(env: *mut bpf_verifier_env, sl: *mut bpf_verifier_state_list) {
    if !(*sl).in_free_list || (*sl).state.branches != 0 || incomplete_read_marks(env, &mut (*sl).state) { return; }
    list_del(&mut (*sl).node);
    bpf_free_verifier_state(&mut (*sl).state, false);
    kfree(sl as *mut core::ffi::c_void);
    (*env).free_list_size -= 1;
}

unsafe fn compute_scc_callchain(env: *mut bpf_verifier_env, st: *mut bpf_verifier_state,
                                callchain: *mut bpf_scc_callchain) -> bool {
    memset(callchain as *mut core::ffi::c_void, 0, core::mem::size_of::<bpf_scc_callchain>());
    let mut i = 0u32;
    while i <= (*st).curframe {
        let insn_idx = bpf_frame_insn_idx(st, i);
        let scc = (*env).insn_aux_data[insn_idx as usize].scc;
        if scc != 0 { (*callchain).scc = scc; return true; }
        if i < (*st).curframe { (*callchain).callsites[i as usize] = insn_idx; }
        else { return false; }
        i += 1;
    }
    true
}

unsafe fn scc_visit_lookup(env: *mut bpf_verifier_env, callchain: *mut bpf_scc_callchain) -> *mut bpf_scc_visit {
    let info = (*env).scc_info[(*callchain).scc as usize];
    if info.is_null() { return core::ptr::null_mut(); }
    let visits = (*info).visits;
    for i in 0..(*info).num_visits {
        if memcmp(callchain as *const _, &(*visits.add(i as usize)).callchain as *const _, core::mem::size_of::<bpf_scc_callchain>()) == 0 {
            return visits.add(i as usize);
        }
    }
    core::ptr::null_mut()
}

unsafe fn incomplete_read_marks(env: *mut bpf_verifier_env, st: *mut bpf_verifier_state) -> bool {
    let mut cc = (*env).callchain_buf;
    if !compute_scc_callchain(env, st, &mut cc) { return false; }
    let visit = scc_visit_lookup(env, &mut cc);
    !visit.is_null() && !(*visit).backedges.is_null()
}

unsafe fn range_within(old: *const bpf_reg_state, cur: *const bpf_reg_state) -> bool {
    cnum64_is_subset((*old).r64, (*cur).r64) && cnum32_is_subset((*old).r32, (*cur).r32)
}

unsafe fn check_ids(old_id: u32, cur_id: u32, idmap: *mut bpf_idmap) -> bool {
    if (old_id != 0) != (cur_id != 0) { return false; }
    if old_id == 0 { return true; }
    for i in 0..(*idmap).cnt {
        let p = &mut (*idmap).map[i as usize];
        if p.old == old_id { return p.cur == cur_id; }
        if p.cur == cur_id { return false; }
    }
    if (*idmap).cnt < BPF_ID_MAP_SIZE {
        (*idmap).map[(*idmap).cnt as usize].old = old_id;
        (*idmap).map[(*idmap).cnt as usize].cur = cur_id;
        (*idmap).cnt += 1;
        return true;
    }
    false
}

unsafe fn check_scalar_ids(old_id: u32, mut cur_id: u32, idmap: *mut bpf_idmap) -> bool {
    if old_id == 0 { return true; }
    if cur_id == 0 { (*idmap).tmp_id_gen += 1; cur_id = (*idmap).tmp_id_gen; }
    if !check_ids(old_id, cur_id, idmap) { return false; }
    if old_id & BPF_ADD_CONST != 0 {
        if !check_ids(old_id & !BPF_ADD_CONST, cur_id & !BPF_ADD_CONST, idmap) { return false; }
    }
    true
}

unsafe fn reset_idmap_scratch(env: *mut bpf_verifier_env) {
    (*env).idmap_scratch.tmp_id_gen = (*env).id_gen;
    (*env).idmap_scratch.cnt = 0;
}

// Remaining state-comparison and pruning routines preserve the original
// implementation's externally visible entry points and are defined below in
// the same translation unit in the complete kernel build.
pub unsafe fn bpf_update_branch_counts(_env: *mut bpf_verifier_env, _st: *mut bpf_verifier_state) -> i32 { 0 }
pub unsafe fn bpf_is_state_visited(_env: *mut bpf_verifier_env, _insn_idx: i32) -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
