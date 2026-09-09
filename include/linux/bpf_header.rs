/* SPDX-License-Identifier: GPL-2.0-only */
/* Translation of linux/bpf.h. External kernel types and functions are intentionally unresolved. */

pub type u8 = core::ffi::c_uchar;
pub type u16 = core::ffi::c_ushort;
pub type u32 = core::ffi::c_uint;
pub type u64 = core::ffi::c_ulonglong;
pub type s16 = core::ffi::c_short;
pub type s64 = core::ffi::c_longlong;
pub type ulong = core::ffi::c_ulong;
pub type size_t = usize;
pub type vm_fault_t = usize;
pub type __poll_t = u32;
pub type atomic64_t = i64;
pub type refcount_t = u32;
pub type bpf_callback_t = unsafe extern "C" fn(u64,u64,u64,u64,u64)->u64;
pub type btf_dtor_kfunc_t = unsafe extern "C" fn(*mut core::ffi::c_void);

#[repr(C)] pub struct bpf_verifier_env { _p: [u8;0] }
#[repr(C)] pub struct bpf_verifier_log { _p: [u8;0] }
#[repr(C)] pub struct perf_event { _p: [u8;0] }
#[repr(C)] pub struct bpf_prog { _p: [u8;0] }
#[repr(C)] pub struct bpf_prog_aux { _p: [u8;0] }
#[repr(C)] pub struct bpf_arena { _p: [u8;0] }
#[repr(C)] pub struct sock { _p: [u8;0] }
#[repr(C)] pub struct seq_file { _p: [u8;0] }
#[repr(C)] pub struct btf_type { _p: [u8;0] }
#[repr(C)] pub struct btf { _p: [u8;0] }
#[repr(C)] pub struct seq_operations { _p: [u8;0] }
#[repr(C)] pub struct bpf_iter_aux_info { _p: [u8;0] }
#[repr(C)] pub struct bpf_local_storage { _p: [u8;0] }
#[repr(C)] pub struct bpf_local_storage_map { _p: [u8;0] }
#[repr(C)] pub struct file { _p: [u8;0] }
#[repr(C)] pub struct module { _p: [u8;0] }
#[repr(C)] pub struct bpf_func_state { _p: [u8;0] }
#[repr(C)] pub struct net_device { _p: [u8;0] }
#[repr(C)] pub struct bpf_tramp_node { _p: [u8;0] }
#[repr(C)] pub struct bpf_tramp_run_ctx { _p: [u8;0] }
#[repr(C)] pub struct bpf_tramp_image { _p: [u8;0] }
#[repr(C)] pub struct rb_node { _p: [u8;0] }
#[repr(C)] pub struct list_head { _p: [u8;0] }
#[repr(C)] pub struct rcu_head { _p: [u8;0] }
#[repr(C)] pub struct work_struct { _p: [u8;0] }
#[repr(C)] pub struct hlist_node { _p: [u8;0] }
#[repr(C)] pub struct hlist_head { _p: [u8;0] }
#[repr(C)] pub struct latch_tree_node { _p: [u8;0] }
#[repr(C)] pub struct mutex { _p: [u8;0] }
#[repr(C)] pub struct bpf_insn { pub code:u8, pub dst_reg:u8, pub src_reg:u8, pub off:s16, pub imm:i32 }
#[repr(C)] pub union bpf_attr { _p: [u64; 0] }
#[repr(C)] pub struct bpf_spin_lock { _p:[u8;0] }
#[repr(C)] pub struct bpf_res_spin_lock { _p:[u8;0] }
#[repr(C)] pub struct bpf_timer { _p:[u8;0] }
#[repr(C)] pub struct bpf_wq { _p:[u8;0] }
#[repr(C)] pub struct bpf_list_head { _p:[u8;0] }
#[repr(C)] pub struct bpf_list_node { _p:[u8;0] }
#[repr(C)] pub struct bpf_rb_root { _p:[u8;0] }
#[repr(C)] pub struct bpf_rb_node { _p:[u8;0] }
#[repr(C)] pub struct bpf_refcount { _p:[u8;0] }
#[repr(C)] pub struct bpf_task_work { _p:[u8;0] }

pub type bpf_iter_init_seq_priv_t = unsafe extern "C" fn(*mut core::ffi::c_void,*mut bpf_iter_aux_info)->i32;
pub type bpf_iter_fini_seq_priv_t = unsafe extern "C" fn(*mut core::ffi::c_void);
pub type bpf_func_t = unsafe extern "C" fn(*const core::ffi::c_void,*const bpf_insn)->u32;
#[repr(C)] pub struct bpf_iter_seq_info { pub seq_ops:*const seq_operations, pub init_seq_private:Option<bpf_iter_init_seq_priv_t>, pub fini_seq_private:Option<bpf_iter_fini_seq_priv_t>, pub seq_priv_size:u32 }

#[repr(C)] pub struct btf_field_kptr { pub btf:*mut btf, pub module:*mut module, pub dtor:Option<btf_dtor_kfunc_t>, pub btf_id:u32 }
#[repr(C)] pub struct btf_field_graph_root { pub btf:*mut btf, pub value_btf_id:u32, pub node_offset:u32, pub value_rec:*mut btf_record }
#[repr(C)] pub union btf_field_union { pub kptr:core::mem::ManuallyDrop<btf_field_kptr>, pub graph_root:core::mem::ManuallyDrop<btf_field_graph_root> }
#[repr(C)] pub struct btf_field { pub offset:u32, pub size:u32, pub typ: btf_field_type, pub data:btf_field_union }
#[repr(C)] pub struct btf_record { pub cnt:u32, pub field_mask:u32, pub spin_lock_off:i32, pub res_spin_lock_off:i32, pub timer_off:i32, pub wq_off:i32, pub refcount_off:i32, pub task_work_off:i32, pub fields:[btf_field;0] }

#[repr(u32)] #[derive(Copy,Clone)] pub enum btf_field_type { BPF_SPIN_LOCK=1, BPF_TIMER=2, BPF_KPTR_UNREF=4, BPF_KPTR_REF=8, BPF_KPTR_PERCPU=16, BPF_LIST_HEAD=32, BPF_LIST_NODE=64, BPF_RB_ROOT=128, BPF_RB_NODE=256, BPF_REFCOUNT=512, BPF_WORKQUEUE=1024, BPF_UPTR=2048, BPF_RES_SPIN_LOCK=4096, BPF_TASK_WORK=8192 }
pub const BPF_KPTR:u32=4|8|16; pub const BPF_GRAPH_NODE:u32=256|64; pub const BPF_GRAPH_ROOT:u32=128|32;
pub const BTF_FIELDS_MAX:u32=11; pub const MAX_BPF_FUNC_ARGS:usize=12; pub const MAX_BPF_FUNC_REG_ARGS:usize=5;

#[repr(C)] pub struct bpf_map_ops { pub map_alloc_check:Option<unsafe extern "C" fn(*mut bpf_attr)->i32>, pub map_alloc:Option<unsafe extern "C" fn(*mut bpf_attr)->*mut bpf_map>, pub map_release:Option<unsafe extern "C" fn(*mut bpf_map,*mut file)>, pub map_free:Option<unsafe extern "C" fn(*mut bpf_map)>, pub map_get_next_key:Option<unsafe extern "C" fn(*mut bpf_map,*mut core::ffi::c_void,*mut core::ffi::c_void)->i32>, pub map_release_uref:Option<unsafe extern "C" fn(*mut bpf_map)>, pub map_lookup_elem:Option<unsafe extern "C" fn(*mut bpf_map,*mut core::ffi::c_void)->*mut core::ffi::c_void>, pub map_update_elem:Option<unsafe extern "C" fn(*mut bpf_map,*mut core::ffi::c_void,*mut core::ffi::c_void,u64)->i64>, pub map_delete_elem:Option<unsafe extern "C" fn(*mut bpf_map,*mut core::ffi::c_void)->i64>, pub map_push_elem:Option<unsafe extern "C" fn(*mut bpf_map,*mut core::ffi::c_void,u64)->i64>, pub map_pop_elem:Option<unsafe extern "C" fn(*mut bpf_map,*mut core::ffi::c_void)->i64>, pub map_peek_elem:Option<unsafe extern "C" fn(*mut bpf_map,*mut core::ffi::c_void)->i64>, pub map_lookup_percpu_elem:Option<unsafe extern "C" fn(*mut bpf_map,*mut core::ffi::c_void,u32)->*mut core::ffi::c_void>, pub map_get_hash:Option<unsafe extern "C" fn(*mut bpf_map)->i32>, pub map_seq_show_elem:Option<unsafe extern "C" fn(*mut bpf_map,*mut core::ffi::c_void,*mut seq_file)>, pub map_check_btf:Option<unsafe extern "C" fn(*mut bpf_map,*const btf,*const btf_type,*const btf_type)->i32>, pub map_poke_track:Option<unsafe extern "C" fn(*mut bpf_map,*mut bpf_prog_aux)->i32>, pub map_poke_untrack:Option<unsafe extern "C" fn(*mut bpf_map,*mut bpf_prog_aux)>, pub map_poke_run:Option<unsafe extern "C" fn(*mut bpf_map,u32,*mut bpf_prog,*mut bpf_prog)> }

#[repr(C)] pub struct bpf_map_owner { pub typ:u32, pub jited:bool, pub xdp_has_frags:bool, pub sleepable:bool, pub storage_cookie:[u64;3], pub attach_func_proto:*const btf_type, pub expected_attach_type:u32 }
#[repr(C)] pub struct bpf_map { pub sha:[u8;32], pub ops:*const bpf_map_ops, pub inner_map_meta:*mut bpf_map, pub map_type:u32, pub key_size:u32, pub value_size:u32, pub max_entries:u32, pub map_extra:u64, pub map_flags:u32, pub id:u32, pub record:*mut btf_record, pub numa_node:i32, pub btf_key_type_id:u32, pub btf_value_type_id:u32, pub btf_vmlinux_value_type_id:u32, pub btf:*mut btf, pub name:[u8;16], pub freeze_mutex:mutex, pub refcnt:atomic64_t, pub usercnt:atomic64_t, pub writecnt:atomic64_t, pub owner:*mut bpf_map_owner, pub bypass_spec_v1:bool, pub frozen:bool, pub free_after_mult_rcu_gp:bool, pub free_after_rcu_gp:bool, pub sleepable_refcnt:atomic64_t, pub elem_count:*mut s64, pub cookie:u64, pub excl_prog_sha:*mut u8 }

pub const BPF_BASE_TYPE_BITS:u32=8; pub const PTR_MAYBE_NULL:u32=1<<8; pub const MEM_RDONLY:u32=1<<9; pub const MEM_RINGBUF:u32=1<<10; pub const MEM_USER:u32=1<<11; pub const MEM_PERCPU:u32=1<<12; pub const OBJ_RELEASE:u32=1<<13; pub const PTR_UNTRUSTED:u32=1<<14; pub const MEM_UNINIT:u32=1<<15; pub const DYNPTR_TYPE_LOCAL:u32=1<<16; pub const DYNPTR_TYPE_RINGBUF:u32=1<<17; pub const MEM_FIXED_SIZE:u32=1<<18; pub const MEM_ALLOC:u32=1<<19; pub const PTR_TRUSTED:u32=1<<20; pub const MEM_RCU:u32=1<<21; pub const NON_OWN_REF:u32=1<<22; pub const DYNPTR_TYPE_SKB:u32=1<<23; pub const DYNPTR_TYPE_XDP:u32=1<<24; pub const MEM_ALIGNED:u32=1<<25; pub const MEM_WRITE:u32=1<<26; pub const DYNPTR_TYPE_SKB_META:u32=1<<27; pub const DYNPTR_TYPE_FILE:u32=1<<28;
pub const BPF_BASE_TYPE_LIMIT:u32=1<<BPF_BASE_TYPE_BITS;

#[repr(C)] pub struct bpf_func_proto { pub func:Option<unsafe extern "C" fn(u64,u64,u64,u64,u64)->u64>, pub gpl_only:bool, pub pkt_access:bool, pub might_sleep:bool, pub allow_fastcall:bool, pub ret_type:u32, pub arg_type:[u32;MAX_BPF_FUNC_ARGS], pub arg_btf_id:[*mut u32;MAX_BPF_FUNC_ARGS], pub arg_size:[size_t;MAX_BPF_FUNC_ARGS], pub ret_btf_id:*mut i32, pub allowed:Option<unsafe extern "C" fn(*const bpf_prog)->bool> }
#[repr(C)] pub struct bpf_context { _p:[u8;0] }
#[repr(C)] pub struct bpf_insn_access_aux { pub reg_type:u32, pub is_ldsx:bool, pub ctx_field_size:i32, pub btf:*mut btf, pub btf_id:u32, pub ref_id:u32, pub log:*mut bpf_verifier_log, pub is_retval:bool }
#[repr(C)] pub struct bpf_prog_ops { pub test_run:Option<unsafe extern "C" fn(*mut bpf_prog,*const bpf_attr,*mut bpf_attr)->i32> }
#[repr(C)] pub struct bpf_tramp_nodes { pub nodes:[*mut bpf_tramp_node;38], pub nr_nodes:i32 }
#[repr(C)] pub struct btf_func_model { pub ret_size:u8, pub ret_flags:u8, pub nr_args:u8, pub arg_size:[u8;12], pub arg_flags:[u8;12] }

extern "C" { pub static mut btf_idr:core::ffi::c_void; pub static mut btf_idr_lock:core::ffi::c_void; pub static bpf_map_offload_ops:bpf_map_ops; pub fn copy_map_value_locked(map:*mut bpf_map,dst:*mut core::ffi::c_void,src:*mut core::ffi::c_void,lock_src:bool); pub fn bpf_timer_cancel_and_free(timer:*mut core::ffi::c_void); pub fn bpf_wq_cancel_and_free(timer:*mut core::ffi::c_void); pub fn bpf_task_work_cancel_and_free(timer:*mut core::ffi::c_void); pub fn bpf_arena_get_kern_vm_start(arena:*mut bpf_arena)->u64; pub fn bpf_arena_get_user_vm_start(arena:*mut bpf_arena)->u64; pub fn bpf_arena_map_kern_vm_start(map:*mut bpf_map)->u64; pub fn bpf_prog_arena(prog:*mut bpf_prog)->*mut bpf_map; pub fn bpf_obj_name_cpy(dst:*mut u8,src:*const u8,size:u32)->i32; pub fn bpf_tramp_arena_base(m:*const btf_func_model,tnodes:*mut bpf_tramp_nodes,flags:u32)->u64; }

#[inline] pub unsafe fn btf_record_has_field(rec:*const btf_record, typ:u32)->bool { !rec.is_null() && (*rec).field_mask & typ != 0 }
#[inline] pub unsafe fn btf_field_is_nmi_safe(typ:btf_field_type)->bool { matches!(typ, btf_field_type::BPF_SPIN_LOCK|btf_field_type::BPF_RES_SPIN_LOCK|btf_field_type::BPF_TIMER|btf_field_type::BPF_WORKQUEUE|btf_field_type::BPF_TASK_WORK|btf_field_type::BPF_KPTR_UNREF|btf_field_type::BPF_REFCOUNT) }
#[inline] pub unsafe fn bpf_ctx_record_field_size(aux:*mut bpf_insn_access_aux,size:u32) { (*aux).ctx_field_size=size as i32; }
#[inline] pub unsafe fn bpf_is_ldimm64(insn:*const bpf_insn)->bool { (*insn).code == 0x18 }
#[inline] pub unsafe fn bpf_pseudo_func(insn:*const bpf_insn)->bool { bpf_is_ldimm64(insn) && (*insn).src_reg == 4 }
#[inline] pub unsafe fn bpf_trampoline_use_jmp(flags:u64)->bool { flags & (1<<1) != 0 && flags & (1<<2) == 0 }

#[repr(C)] pub struct bpf_offload_dev { _p:[u8;0] }
#[repr(C)] pub struct bpf_map_dev_ops { pub map_get_next_key:Option<unsafe extern "C" fn(*mut bpf_offloaded_map,*mut core::ffi::c_void,*mut core::ffi::c_void)->i32>, pub map_lookup_elem:Option<unsafe extern "C" fn(*mut bpf_offloaded_map,*mut core::ffi::c_void,*mut core::ffi::c_void)->i32>, pub map_update_elem:Option<unsafe extern "C" fn(*mut bpf_offloaded_map,*mut core::ffi::c_void,*mut core::ffi::c_void,u64)->i32>, pub map_delete_elem:Option<unsafe extern "C" fn(*mut bpf_offloaded_map,*mut core::ffi::c_void)->i32> }
#[repr(C)] pub struct bpf_offloaded_map { pub map:bpf_map, pub netdev:*mut net_device, pub dev_ops:*const bpf_map_dev_ops, pub dev_priv:*mut core::ffi::c_void, pub offloads:list_head }
#[repr(C)] pub struct bpf_prog_offload_ops { pub insn_hook:Option<unsafe extern "C" fn(*mut bpf_verifier_env,i32,i32)->i32>, pub finalize:Option<unsafe extern "C" fn(*mut bpf_verifier_env)->i32>, pub replace_insn:Option<unsafe extern "C" fn(*mut bpf_verifier_env,u32,*mut bpf_insn)->i32>, pub remove_insns:Option<unsafe extern "C" fn(*mut bpf_verifier_env,u32,u32)->i32>, pub prepare:Option<unsafe extern "C" fn(*mut bpf_prog)->i32>, pub translate:Option<unsafe extern "C" fn(*mut bpf_prog)->i32>, pub destroy:Option<unsafe extern "C" fn(*mut bpf_prog)> }
#[repr(C)] pub struct bpf_prog_offload { pub prog:*mut bpf_prog, pub netdev:*mut net_device, pub offdev:*mut bpf_offload_dev, pub dev_priv:*mut core::ffi::c_void, pub offloads:list_head, pub dev_state:bool, pub opt_failed:bool, pub jited_image:*mut core::ffi::c_void, pub jited_len:u32 }
#[repr(C)] pub struct bpf_ksym { pub start:ulong, pub end:ulong, pub name:[u8;128], pub lnode:list_head, pub tnode:latch_tree_node, pub prog:bool, pub fp_start:u32, pub fp_end:u32 }
#[repr(C)] pub struct bpf_tramp_image { pub image:*mut core::ffi::c_void, pub size:i32, pub ksym:bpf_ksym, pub ip_after_call:*mut core::ffi::c_void, pub ip_epilogue:*mut core::ffi::c_void, pub rcu:rcu_head }
#[repr(C)] pub struct bpf_trampoline { pub hlist_key:hlist_node, pub hlist_ip:hlist_node, pub fops:*mut core::ffi::c_void, pub refcnt:refcount_t, pub flags:u32, pub key:u64, pub ip:ulong, pub extension_prog:*mut bpf_prog, pub progs_hlist:[hlist_head;6], pub progs_cnt:[i32;6], pub cur_image:*mut bpf_tramp_image, pub old_image:*mut bpf_tramp_image, pub old_flags:u32 }
#[repr(C)] pub struct bpf_attach_target_info { pub fmodel:btf_func_model, pub tgt_addr:i64, pub tgt_mod:*mut module, pub tgt_name:*const u8, pub tgt_type:*const btf_type }
#[repr(C)] pub struct bpf_dispatcher_prog { pub prog:*mut bpf_prog, pub users:refcount_t }
#[repr(C)] pub struct bpf_dispatcher { pub mutex:mutex, pub func:*mut core::ffi::c_void, pub progs:[bpf_dispatcher_prog;48], pub num_progs:i32, pub image:*mut core::ffi::c_void, pub rw_image:*mut core::ffi::c_void, pub image_off:u32, pub ksym:bpf_ksym }
pub const BPF_TRAMP_F_RESTORE_REGS:u32=1; pub const BPF_TRAMP_F_CALL_ORIG:u32=2; pub const BPF_TRAMP_F_SKIP_FRAME:u32=4; pub const BPF_TRAMP_F_IP_ARG:u32=8; pub const BPF_TRAMP_F_RET_FENTRY_RET:u32=16; pub const BPF_TRAMP_F_ORIG_STACK:u32=32; pub const BPF_TRAMP_F_SHARE_IPMODIFY:u32=64; pub const BPF_TRAMP_F_TAIL_CALL_CTX:u32=128; pub const BPF_TRAMP_F_INDIRECT:u32=256; pub const BPF_MAX_TRAMP_LINKS:usize=38;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
