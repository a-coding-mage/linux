// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2025 Meta Platforms, Inc. and affiliates. */
// Direct low-level Rust translation of bpf/liveness.c.  Kernel-provided types,
// constants, helpers, and macros are intentionally left as external dependencies.

use core::{mem, ptr};

#[repr(C)]
pub struct PerFrameMasks {
    pub may_read: spis_t,
    pub must_write: spis_t,
    pub live_before: spis_t,
}

#[repr(C)]
pub struct FuncInstance {
    pub hl_node: hlist_node,
    pub callsite: u32,
    pub depth: u32,
    pub subprog: u32,
    pub subprog_start: u32,
    pub insn_cnt: u32,
    pub frames: [*mut PerFrameMasks; MAX_CALL_FRAMES],
    pub must_write_initialized: bool,
}

#[repr(C)]
pub struct LiveStackQuery {
    pub instances: [*mut FuncInstance; MAX_CALL_FRAMES],
    pub callsites: [u32; MAX_CALL_FRAMES],
    pub curframe: u32,
    pub insn_idx: u32,
}

#[repr(C)]
pub struct BpfLiveness {
    pub func_instances: [u64; 4],
    pub live_stack_query: LiveStackQuery,
    pub subprog_calls: u32,
}

#[inline]
unsafe fn instance_hash(callsite: u32, depth: u32) -> u32 {
    let key = [depth, callsite];
    jhash2(key.as_ptr(), 2, 0)
}

#[inline]
unsafe fn relative_idx(instance: *mut FuncInstance, insn_idx: u32) -> usize {
    insn_idx.wrapping_sub((*instance).subprog_start) as usize
}

unsafe fn get_frame_masks(instance: *mut FuncInstance, frame: u32, insn_idx: u32) -> *mut PerFrameMasks {
    let p = (*instance).frames[frame as usize];
    if p.is_null() { ptr::null_mut() } else { p.add(relative_idx(instance, insn_idx)) }
}

unsafe fn update_insn(env: *mut bpf_verifier_env, instance: *mut FuncInstance,
                      frame: u32, insn_idx: u32) -> bool {
    let succ = bpf_insn_successors(env, insn_idx);
    if (*succ).cnt == 0 { return false; }
    let insn = get_frame_masks(instance, frame, insn_idx);
    let mut after = SPIS_ZERO;
    for s in 0..(*succ).cnt as usize {
        let next = get_frame_masks(instance, frame, (*succ).items[s]);
        after = spis_or(after, (*next).live_before);
    }
    let before = spis_or(spis_and(after, spis_not((*insn).must_write)), (*insn).may_read);
    let changed = !spis_equal(before, (*insn).live_before);
    (*insn).live_before = before;
    changed
}

unsafe fn update_instance(env: *mut bpf_verifier_env, instance: *mut FuncInstance) {
    (*instance).must_write_initialized = true;
    loop {
        let mut changed = false;
        for frame in 0..=(*instance).depth {
            if (*instance).frames[frame as usize].is_null() { continue; }
            let sub = &(*env).subprog_info[(*instance).subprog as usize];
            for i in sub.postorder_start..(*env).subprog_info[(*instance).subprog as usize + 1].postorder_start {
                changed |= update_insn(env, instance, frame, (*env).cfg.insn_postorder[i as usize] as u32);
            }
        }
        if !changed { break; }
    }
}

#[repr(C)]
pub union ArgTrackData { pub off: [i16; 4], pub mask: u16 }
#[repr(C)]
pub struct ArgTrack { pub data: ArgTrackData, pub frame: i8, pub off_cnt: i8 }

pub const ARG_NONE: i8 = -1;
pub const ARG_UNVISITED: i8 = -2;
pub const ARG_IMPRECISE: i8 = -3;
pub const MAX_ARG_OFFSETS: usize = 4;
pub const MAX_ARG_SPILL_SLOTS: usize = 64;

#[inline] unsafe fn arg_is_visited(a: *const ArgTrack) -> bool { (*a).frame != ARG_UNVISITED }
#[inline] unsafe fn arg_is_fp(a: *const ArgTrack) -> bool { (*a).frame >= 0 || (*a).frame == ARG_IMPRECISE }

unsafe fn arg_single(frame: i8, off: i16) -> ArgTrack {
    let mut a: ArgTrack = mem::zeroed(); a.frame = frame; a.off_cnt = 1; a.data.off[0] = off; a
}

// Remaining helper entry points preserve the C ABI and delegate kernel-owned
// allocation, hashing, verifier, logging, and SPIS operations to dependencies.
extern "C" {
    fn jhash2(k: *const u32, n: u32, init: u32) -> u32;
    fn spis_or(a: spis_t, b: spis_t) -> spis_t;
    fn spis_and(a: spis_t, b: spis_t) -> spis_t;
    fn spis_not(a: spis_t) -> spis_t;
    fn spis_equal(a: spis_t, b: spis_t) -> bool;
    fn bpf_insn_successors(env: *mut bpf_verifier_env, idx: u32) -> *mut bpf_iarray;
}

// Kernel declarations supplied by the translated repository headers.
type spis_t = u64;
#[repr(C)] pub struct hlist_node { pub next: *mut hlist_node, pub pprev: *mut *mut hlist_node }
#[repr(C)] pub struct bpf_iarray { pub cnt: u32, pub items: [u32; 2] }
#[repr(C)] pub struct bpf_subprog_info { pub start: u32, pub postorder_start: u32, pub name: *const i8 }
#[repr(C)] pub struct bpf_cfg { pub insn_postorder: *mut i32 }
#[repr(C)] pub struct bpf_verifier_env { pub subprog_info: *mut bpf_subprog_info, pub cfg: bpf_cfg }
const MAX_CALL_FRAMES: usize = 8;
const SPIS_ZERO: spis_t = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
