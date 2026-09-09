/* SPDX-License-Identifier: GPL-2.0-only */
/* Faithful Rust translation of linux/bpf_verifier.h. */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

use core::ffi::{c_char, c_int, c_void};

pub type s8 = i8; pub type u8 = u8; pub type s16 = i16; pub type u16 = u16;
pub type s32 = i32; pub type u32 = u32; pub type s64 = i64; pub type u64 = u64;
pub type ssize_t = isize; pub type size_t = usize;

/* Dependencies supplied by the surrounding kernel translation. */
extern "C" { }
pub enum bpf_map {} pub enum btf {} pub enum bpf_prog {} pub enum module {}
pub enum bpf_verifier_ops {} pub enum bpf_verifier_stack_elem {} pub enum bpf_diag {}
pub enum bpf_liveness {} pub enum bpf_func_proto {} pub enum btf_type {}
pub enum btf_field {} pub enum btf_struct_meta {} pub enum btf_mod_pair {}
pub enum bpf_line_info {} pub enum bpf_common_attr {} pub enum bpf_attach_target_info {}
pub enum bpf_kfunc_btf_tab {} pub enum list_head {} pub enum arg_track {}
pub enum bpf_dynptr_kern {}
#[repr(C)] pub struct bpf_insn { pub code:u8, pub dst_reg:u8, pub src_reg:u8, pub off:s16, pub imm:s32 }
#[repr(C)] pub struct tnum { pub value:u64, pub mask:u64 }
#[repr(C)] pub struct cnum64 { pub min:u64, pub max:u64 }
#[repr(C)] pub struct cnum32 { pub min:u32, pub max:u32 }
#[repr(C)] pub struct bpfptr_t(pub u64);
pub type va_list = *mut c_void;

pub const BPF_MAX_VAR_OFF:u32=1<<29; pub const BPF_MAX_VAR_SIZ:u32=1<<29;
pub const TMP_STR_BUF_LEN:usize=320; pub const INSN_BUF_SIZE:usize=32;
pub const ITER_PREFIX:&[u8]=b"bpf_iter_\0";
pub const BPF_ADD_CONST64:u32=1<<31; pub const BPF_ADD_CONST32:u32=1<<30;
pub const BPF_ADD_CONST:u32=BPF_ADD_CONST64|BPF_ADD_CONST32;

#[repr(C)] #[derive(Copy,Clone)] pub union bpf_reg_state_union {
 pub range:c_int, pub map: bpf_reg_state_map, pub btf:bpf_reg_state_btf,
 pub mem_size:u32, pub dynptr:bpf_reg_state_dynptr, pub iter:bpf_reg_state_iter,
 pub irq:bpf_reg_state_irq, pub raw:bpf_reg_state_raw, pub subprogno:u32,
}
#[repr(C)] #[derive(Copy,Clone)] pub struct bpf_reg_state_map { pub map_ptr:*mut bpf_map, pub map_uid:u32 }
#[repr(C)] #[derive(Copy,Clone)] pub struct bpf_reg_state_btf { pub btf:*mut btf, pub btf_id:u32 }
#[repr(C)] #[derive(Copy,Clone)] pub struct bpf_reg_state_dynptr { pub type_:u32, pub first_slot:bool }
#[repr(C)] #[derive(Copy,Clone)] pub struct bpf_reg_state_iter { pub btf:*mut btf, pub btf_id:u32, pub state:u32, pub depth:i32 }
#[repr(C)] #[derive(Copy,Clone)] pub struct bpf_reg_state_irq { pub kfunc_class:u32 }
#[repr(C)] #[derive(Copy,Clone)] pub struct bpf_reg_state_raw { pub raw1:usize, pub raw2:usize }
#[repr(C)] pub struct bpf_reg_state { pub type_:u32, pub delta:s32, pub data:bpf_reg_state_union, pub var_off:tnum, pub r64:cnum64, pub r32:cnum32, pub id:u32, pub parent_id:u32, pub frameno:u32, pub precise:bool }

#[repr(u32)] pub enum bpf_iter_state { BPF_ITER_STATE_INVALID, BPF_ITER_STATE_ACTIVE, BPF_ITER_STATE_DRAINED }
#[repr(u32)] pub enum bpf_stack_slot_type { STACK_INVALID, STACK_SPILL, STACK_MISC, STACK_ZERO, STACK_DYNPTR, STACK_ITER, STACK_IRQ_FLAG, STACK_POISON }
pub const BPF_REG_SIZE:usize=8; pub const BPF_HALF_REG_SIZE:usize=4; pub const STACK_SLOT_SZ:usize=4;
pub const STACK_SLOTS:usize=128;
#[repr(C)] #[derive(Copy,Clone)] pub struct spis_t { pub v:[u64;2] }
pub const MAX_CALL_FRAMES:usize=16; pub const MAX_BPF_REG:usize=11; pub const MAX_BPF_STACK:usize=512;
pub const MAX_BPF_FUNC_ARGS:usize=5; pub const MAX_BPF_FUNC_REG_ARGS:usize=5;
pub const MAX_STACK_ARG_SLOTS:usize=MAX_BPF_FUNC_ARGS-MAX_BPF_FUNC_REG_ARGS;
pub const BPF_ID_MAP_SIZE:usize=(MAX_BPF_REG+MAX_BPF_STACK/BPF_REG_SIZE+MAX_STACK_ARG_SLOTS)*MAX_CALL_FRAMES;

#[repr(C)] pub struct bpf_stack_state { pub spilled_ptr:bpf_reg_state, pub slot_type:[u8;8] }
#[repr(C)] pub union bpf_reference_union { pub parent_id:c_int, pub ptr:*mut c_void }
#[repr(C)] pub struct bpf_reference_state { pub type_:u32, pub id:c_int, pub insn_idx:c_int, pub data:bpf_reference_union }
#[repr(C)] pub struct bpf_retval_range { pub minval:s32, pub maxval:s32, pub return_32bit:bool }
#[repr(C)] pub struct bpf_func_state { pub regs:[bpf_reg_state;MAX_BPF_REG], pub callsite:c_int, pub frameno:u32, pub diag_frame_id:u32, pub subprogno:u32, pub async_entry_cnt:u32, pub callback_ret_range:bpf_retval_range, pub in_callback_fn:bool, pub in_async_callback_fn:bool, pub in_exception_callback_fn:bool, pub no_stack_arg_load:bool, pub callback_depth:u32, pub insns_subtotal:u32, pub stack:*mut bpf_stack_state, pub allocated_stack:c_int, pub out_stack_arg_cnt:u16, pub stack_arg_regs:*mut bpf_reg_state }
#[repr(C)] pub struct bpf_jmp_history_entry { pub idx:u32, pub frame:u32, pub spi:u32, pub prev_idx:u32, pub flags:u32, pub linked_regs:u64 }
#[repr(C)] pub struct bpf_verifier_state { pub frame:[*mut bpf_func_state;MAX_CALL_FRAMES], pub parent:*mut bpf_verifier_state, pub refs:*mut bpf_reference_state, pub branches:u32, pub insn_idx:u32, pub curframe:u32, pub acquired_refs:u32, pub active_locks:u32, pub active_preempt_locks:u32, pub active_irq_id:u32, pub active_lock_id:u32, pub active_lock_ptr:*mut c_void, pub active_rcu_locks:u32, pub speculative:bool, pub in_sleepable:bool, pub first_insn_idx:u32, pub last_insn_idx:u32, pub equal_state:*mut bpf_verifier_state, pub jmp_history:*mut bpf_jmp_history_entry, pub jmp_history_cnt:u32, pub dfs_depth:u32, pub callback_unroll_depth:u32, pub may_goto_depth:u32 }

extern "C" {
 pub fn cnum64_smin(x:cnum64)->s64; pub fn cnum64_smax(x:cnum64)->s64; pub fn cnum64_umin(x:cnum64)->u64; pub fn cnum64_umax(x:cnum64)->u64;
 pub fn cnum32_smin(x:cnum32)->s32; pub fn cnum32_smax(x:cnum32)->s32; pub fn cnum32_umin(x:cnum32)->u32; pub fn cnum32_umax(x:cnum32)->u32;
 pub fn cnum32_from_srange(a:s32,b:s32)->cnum32; pub fn cnum32_from_urange(a:u32,b:u32)->cnum32; pub fn cnum64_from_srange(a:s64,b:s64)->cnum64; pub fn cnum64_from_urange(a:u64,b:u64)->cnum64;
}
#[inline] pub unsafe fn reg_smin(r:*const bpf_reg_state)->s64 { cnum64_smin((*r).r64) }
#[inline] pub unsafe fn reg_smax(r:*const bpf_reg_state)->s64 { cnum64_smax((*r).r64) }
#[inline] pub unsafe fn reg_umin(r:*const bpf_reg_state)->u64 { cnum64_umin((*r).r64) }
#[inline] pub unsafe fn reg_umax(r:*const bpf_reg_state)->u64 { cnum64_umax((*r).r64) }
#[inline] pub unsafe fn reg_s32_min(r:*const bpf_reg_state)->s32 { cnum32_smin((*r).r32) }
#[inline] pub unsafe fn reg_s32_max(r:*const bpf_reg_state)->s32 { cnum32_smax((*r).r32) }
#[inline] pub unsafe fn reg_u32_min(r:*const bpf_reg_state)->u32 { cnum32_umin((*r).r32) }
#[inline] pub unsafe fn reg_u32_max(r:*const bpf_reg_state)->u32 { cnum32_umax((*r).r32) }
#[inline] pub unsafe fn reg_set_srange32(r:*mut bpf_reg_state,a:s32,b:s32){(*r).r32=cnum32_from_srange(a,b)}
#[inline] pub unsafe fn reg_set_urange32(r:*mut bpf_reg_state,a:u32,b:u32){(*r).r32=cnum32_from_urange(a,b)}
#[inline] pub unsafe fn reg_set_srange64(r:*mut bpf_reg_state,a:s64,b:s64){(*r).r64=cnum64_from_srange(a,b)}
#[inline] pub unsafe fn reg_set_urange64(r:*mut bpf_reg_state,a:u64,b:u64){(*r).r64=cnum64_from_urange(a,b)}

/* Remaining declarations retain the header's external interface. */
#[repr(C)] pub struct bpf_subprog_arg_info { pub arg_type:u32, pub data:u32 }
#[repr(C)] pub struct bpf_subprog_info { pub name:*const c_char, pub start:u32, pub linfo_idx:u32, pub postorder_start:u32, pub exit_idx:u32, pub stack_depth:u16, pub stack_extra:u16, pub insns_total:u32, pub insns_self:u32, pub fastcall_stack_off:s16, pub flags:u32, pub arg_cnt:u8, pub priv_stack_mode:u32, pub args:[bpf_subprog_arg_info;MAX_BPF_FUNC_ARGS], pub stack_arg_cnt:u16, pub max_out_stack_arg_cnt:u16 }
#[repr(C)] pub struct bpf_verifier_log { pub start_pos:u64, pub end_pos:u64, pub ubuf:*mut c_char, pub level:u32, pub len_total:u32, pub len_max:u32, pub kbuf:[c_char;1024] }
#[repr(C)] pub struct bpf_map_desc { pub ptr:*mut bpf_map, pub uid:c_int }
#[repr(C)] pub struct bpf_dynptr_desc { pub type_:u32, pub id:u32, pub parent_id:u32 }
#[repr(C)] pub struct ref_obj_desc { pub id:u32, pub parent_id:u32, pub cnt:u8 }
#[repr(C)] pub struct arg_raw_mem_desc { pub regno:u8, pub size:c_int }
#[repr(C)] pub struct ret_mem_desc { pub size:u32, pub found:bool }
#[repr(C)] pub struct arg_constant_desc { pub value:u64, pub found:bool }
#[repr(C)] pub struct bpf_kfunc_desc { pub func_model:[u8;64], pub proto:[u8;64], pub func_id:u32, pub imm:s32, pub offset:u16, pub addr:usize }
#[repr(C)] pub struct bpf_kfunc_desc_tab { pub nr_descs:u32, pub descs:[bpf_kfunc_desc;0] }

/* C inline helpers and exported functions not representable without external kernel types. */
extern "C" {
 pub fn bpf_log_attr_init(log:*mut c_void, log_buf:u64, log_size:u32, log_level:u32, off:u32, uattr:bpfptr_t, common:*mut bpf_common_attr, uattr_common:bpfptr_t, size_common:u32)->c_int;
 pub fn bpf_verifier_log_write(env:*mut c_void, fmt:*const c_char, ...);
 pub fn bpf_prog_offload_verifier_prep(prog:*mut bpf_prog)->c_int;
 pub fn bpf_prog_offload_verify_insn(env:*mut c_void, insn_idx:c_int, prev_insn_idx:c_int)->c_int;
 pub fn bpf_prog_offload_finalize(env:*mut c_void)->c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
