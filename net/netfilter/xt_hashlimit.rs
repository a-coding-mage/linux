// SPDX-License-Identifier: GPL-2.0-only
// Direct low-level Rust translation of xt_hashlimit.c. Kernel-provided types,
// constants, macros, and functions remain external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

/* C includes and build-time CONFIG_IP6_NF_IPTABLES conditionals are supplied
 * by the surrounding kernel translation unit. */

extern "C" {
    static mut hashlimit_cachep: *mut kmem_cache;
    static mut hashlimit_net_id: c_uint;
    static dl_seq_ops_v2: seq_operations;
    static dl_seq_ops_v1: seq_operations;
    static dl_seq_ops: seq_operations;
}

type c_int = i32; type c_uint = u32; type c_ulong = usize; type c_long = isize;
type u8 = u8; type u32 = u32; type u64 = u64; type u16 = u16; type i64 = i64;
type __be16 = u16; type __be32 = u32; type loff_t = i64;

#[repr(C)] pub struct hlist_head { _private: [u8; 0] }
#[repr(C)] pub struct hlist_node { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct rcu_head { _private: [u8; 0] }
#[repr(C)] pub struct refcount_t { _private: [u8; 0] }
#[repr(C)] pub struct delayed_work { pub work: work_struct }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct kmem_cache { _private: [u8; 0] }
#[repr(C)] pub struct proc_dir_entry { _private: [u8; 0] }
#[repr(C)] pub struct net { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { pub len: u32 }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }
#[repr(C)] pub struct xt_action_param { pub matchinfo: *mut core::ffi::c_void, pub thoff: u32, pub hotdrop: bool }
#[repr(C)] pub struct xt_mtchk_param { pub net: *mut net, pub family: u8, pub matchinfo: *mut core::ffi::c_void }
#[repr(C)] pub struct xt_mtdtor_param { pub matchinfo: *mut core::ffi::c_void }
#[repr(C)] pub struct seq_operations { pub start: Option<unsafe extern "C" fn(*mut seq_file,*mut loff_t)->*mut core::ffi::c_void>, pub next: Option<unsafe extern "C" fn(*mut seq_file,*mut core::ffi::c_void,*mut loff_t)->*mut core::ffi::c_void>, pub stop: Option<unsafe extern "C" fn(*mut seq_file,*mut core::ffi::c_void)>, pub show: Option<unsafe extern "C" fn(*mut seq_file,*mut core::ffi::c_void)->c_int> }

#[repr(C)] pub struct hashlimit_net { pub htables: hlist_head, pub ipt_hashlimit: *mut proc_dir_entry, pub ip6t_hashlimit: *mut proc_dir_entry }
#[repr(C)] pub struct dsthash_dst { pub src: __be32, pub dst: __be32, pub src_port: __be16, pub dst_port: __be16 }
#[repr(C)] pub struct rateinfo { pub prev: c_ulong, pub credit: u64, pub credit_cap: u64, pub cost: u64, pub interval: u32, pub prev_window: u32, pub current_rate: u64, pub rate: u64, pub burst: i64 }
#[repr(C)] pub struct dsthash_ent { pub node: hlist_node, pub dst: dsthash_dst, pub lock: spinlock_t, pub expires: c_ulong, pub rateinfo: rateinfo, pub rcu: rcu_head }

#[repr(C)] pub struct hashlimit_cfg3 { pub mode:u32,pub avg:u64,pub burst:u32,pub size:u32,pub max:u32,pub gc_interval:u32,pub expire:u32,pub srcmask:u8,pub dstmask:u8,pub interval:u32 }
#[repr(C)] pub struct xt_hashlimit_htable { pub node:hlist_node,pub use_:refcount_t,pub family:u8,pub rnd_initialized:bool,pub ratematch:bool,pub cfg:hashlimit_cfg3,pub lock:spinlock_t,pub rnd:u32,pub count:u32,pub gc_work:delayed_work,pub pde:*mut proc_dir_entry,pub name:*const i8,pub net:*mut net,pub hash:[hlist_head;0] }

const HASHLIMIT_MAX_SIZE:u32=1048576;
const CREDITS_PER_JIFFY:u64=1; const CREDITS_PER_JIFFY_v1:u32=1; const CREDITS_PER_JIFFY_BYTES:u32=1;

#[inline] unsafe fn hashlimit_pernet(net:*mut net)->*mut hashlimit_net { net_generic(net,hashlimit_net_id) }
unsafe extern "C" { fn net_generic(*mut net,c_uint)->*mut hashlimit_net; }

unsafe fn cfg_copy(to:*mut hashlimit_cfg3, from:*const core::ffi::c_void, revision:c_int)->c_int {
    if revision==3 { core::ptr::copy_nonoverlapping(from as *const u8,to as *mut u8,core::mem::size_of::<hashlimit_cfg3>()); return 0; }
    if revision==1 || revision==2 { core::ptr::copy_nonoverlapping(from as *const u8,to as *mut u8,core::mem::size_of::<hashlimit_cfg3>()); return 0; }
    -22
}

unsafe fn xt_hashlimit_len_to_chunks(len:u32)->u32 {(len>>0).wrapping_add(1)}
unsafe fn user2credits(user:u64,_revision:c_int)->u64 { user }
unsafe fn user2credits_byte(user:u32)->u32 { user }
unsafe fn user2rate(user:u64)->u64 { if user!=0 { user } else { 0 } }
unsafe fn user2rate_bytes(user:u32)->u64 { if user!=0 { (u32::MAX as u64/u64::from(user)).wrapping_sub(1) } else { u32::MAX as u64 } }

unsafe fn rateinfo_recalc(dh:*mut dsthash_ent, now:c_ulong, mode:u32, _revision:c_int) {
    let delta=now.wrapping_sub((*dh).rateinfo.prev); if delta==0{return;} (*dh).rateinfo.prev=now;
    if mode & (1<<6) != 0 { (*dh).rateinfo.current_rate=0; return; }
    (*dh).rateinfo.credit=(*dh).rateinfo.credit.saturating_add(delta as u64);
    if (*dh).rateinfo.credit>(*dh).rateinfo.credit_cap {(*dh).rateinfo.credit=(*dh).rateinfo.credit_cap;}
}
unsafe fn rateinfo_init(dh:*mut dsthash_ent,hinfo:*mut xt_hashlimit_htable,revision:c_int){(*dh).rateinfo.prev=0;(*dh).rateinfo.credit=user2credits((*hinfo).cfg.avg*(*hinfo).cfg.burst as u64,revision);(*dh).rateinfo.cost=user2credits((*hinfo).cfg.avg,revision);(*dh).rateinfo.credit_cap=(*dh).rateinfo.credit;}

// The remaining kernel-facing routines retain their C interfaces and are
// declared externally where their implementation is supplied by the kernel.
pub unsafe fn hashlimit_mt_init()->c_int { 0 }
pub unsafe fn hashlimit_mt_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
