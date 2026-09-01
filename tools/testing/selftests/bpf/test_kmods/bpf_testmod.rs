// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

// Translated from linux/bpf.h, linux/btf.h, linux/btf_ids.h,
// linux/delay.h, linux/error-injection.h, linux/init.h, linux/module.h,
// linux/percpu-defs.h, linux/sysfs.h, linux/tracepoint.h, linux/net.h,
// linux/socket.h, linux/nsproxy.h, linux/inet.h, linux/in.h, linux/in6.h,
// linux/un.h, linux/filter.h, linux/rcupdate_trace.h, net/sock.h,
// linux/namei.h, bpf_testmod.h, bpf_testmod_kfunc.h, and
// bpf_testmod-events.h dependencies as external kernel items.

pub type c_char = i8;
pub type c_void = core::ffi::c_void;
pub type ssize_t = isize;
pub type size_t = usize;
pub type loff_t = i64;
pub type s16 = i16;
pub type s64 = i64;
pub type u8 = u8;
pub type u16 = u16;
pub type u32 = u32;
pub type u64 = u64;
pub type __u64 = u64;

pub const CONNECT_TIMEOUT_SEC: i32 = 1;

pub const EINVAL: i32 = 22;
pub const ENOMEM: i32 = 12;
pub const EIO: i32 = 5;
pub const EBUSY: i32 = 16;
pub const EPERM: i32 = 1;
pub const EEXIST: i32 = 17;
pub const EACCES: i32 = 13;
pub const GFP_ATOMIC: u32 = 0;
pub const GFP_KERNEL: u32 = 0;
pub const AF_INET: i32 = 2;
pub const AF_INET6: i32 = 10;
pub const AF_UNIX: i32 = 1;
pub const SOCK_STREAM: i32 = 1;
pub const IPPROTO_TCP: i32 = 6;
pub const IPPROTO_UDP: i32 = 17;
pub const PF_UNIX: i32 = AF_UNIX;
pub const LOOKUP_FOLLOW: u32 = 1;
pub const ITER_SOURCE: u32 = 1;
pub const HZ: i32 = 100;
pub const BPF_PROG_TYPE_UNSPEC: i32 = 0;
pub const BPF_PROG_TYPE_SCHED_CLS: i32 = 3;
pub const BPF_PROG_TYPE_TRACING: i32 = 26;
pub const BPF_PROG_TYPE_SYSCALL: i32 = 31;
pub const BPF_PROG_TYPE_STRUCT_OPS: i32 = 27;

pub type func_proto_typedef = Option<unsafe extern "C" fn(i64) -> i32>;
pub type func_proto_typedef_nested1 = Option<unsafe extern "C" fn(func_proto_typedef) -> i32>;
pub type func_proto_typedef_nested2 = Option<unsafe extern "C" fn(func_proto_typedef_nested1) -> i32>;

#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct socket { pub sk: *mut sock }
#[repr(C)] pub struct sock { pub sk_sndtimeo: i64 }
#[repr(C)] pub struct sock_common { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff_head { _private: [u8; 0] }
#[repr(C)] pub struct vm_area_struct { _private: [u8; 0] }
#[repr(C)] pub struct task_struct { pub nsproxy: *mut nsproxy }
#[repr(C)] pub struct nsproxy { pub net_ns: *mut c_void }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct kobject { _private: [u8; 0] }
#[repr(C)] pub struct bin_attribute { pub attr: attribute, pub read: *const c_void, pub write: *const c_void }
#[repr(C)] pub struct attribute { pub name: *const c_char, pub mode: u16 }
#[repr(C)] pub struct bpf_dynptr { _private: [u8; 0] }
#[repr(C)] pub struct bpf_dynptr_kern { pub size: u32 }
#[repr(C)] pub struct bpf_timer { _private: [u8; 0] }
#[repr(C)] pub struct rcu_head { _private: [u8; 0] }
#[repr(C)] pub struct btf { _private: [u8; 0] }
#[repr(C)] pub struct btf_type { _private: [u8; 0] }
#[repr(C)] pub struct btf_member { pub offset: u32 }
#[repr(C)] pub struct btf_kfunc_id_set { pub owner: *mut c_void, pub set: *const c_void }
#[repr(C)] pub struct btf_id_dtor_kfunc { pub btf_id: i32, pub kfunc_btf_id: i32 }
#[repr(C)] pub struct bpf_link { _private: [u8; 0] }
#[repr(C)] pub struct bpf_prog { pub aux: *mut bpf_prog_aux, pub stats: *mut bpf_prog_stats, pub insnsi: *mut bpf_insn }
#[repr(C)] pub struct bpf_prog_aux { pub attach_func_name: *const c_char, pub name: [c_char; 16], pub priv_stack_requested: bool, pub recursion_detected: Option<unsafe extern "C" fn(*mut bpf_prog)> }
#[repr(C)] pub struct bpf_prog_stats { pub misses: u64 }
#[repr(C)] pub struct bpf_insn_access_aux { _private: [u8; 0] }
#[repr(C)] pub struct bpf_verifier_log { _private: [u8; 0] }
#[repr(C)] pub struct bpf_reg_state { _private: [u8; 0] }
#[repr(C)] pub struct bpf_insn { _private: [u8; 8] }
#[repr(C)] pub struct bpf_verifier_ops { pub get_func_proto: *const c_void, pub is_valid_access: *const c_void, pub btf_struct_access: *const c_void, pub gen_prologue: *const c_void, pub gen_epilogue: *const c_void }
#[repr(C)] pub struct bpf_struct_ops { pub verifier_ops: *const bpf_verifier_ops, pub init: *const c_void, pub init_member: *const c_void, pub reg: *const c_void, pub unreg: *const c_void, pub check_member: *const c_void, pub cfi_stubs: *const c_void, pub name: *const c_char, pub owner: *mut c_void }
#[repr(C)] pub struct hlist_head { _private: [u8; 0] }
#[repr(C)] pub struct hlist_node { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct path { pub dentry: *mut c_void }
#[repr(C)] pub struct uprobe { _private: [u8; 0] }
#[repr(C)] pub struct pt_regs { pub cx: u64, pub ax: u64, pub r11: u64 }
#[repr(C)] pub struct uprobe_consumer { pub handler: *const c_void, pub ret_handler: *const c_void }
#[repr(C)] pub struct tasklet_struct { _private: [u8; 0] }
#[repr(C)] pub struct irq_work { _private: [u8; 0] }
#[repr(C)] pub struct cgroup { _private: [u8; 0] }
#[repr(C)] pub struct __sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct sockaddr { _private: [u8; 0] }
#[repr(C)] pub struct sockaddr_unsized { _private: [u8; 0] }
#[repr(C)] pub struct msghdr { pub msg_name: *mut c_void, pub msg_namelen: i32, pub msg_iter: c_void }
#[repr(C)] pub struct kvec { pub iov_base: *mut c_void, pub iov_len: size_t }

#[repr(C)] pub struct bpf_iter_testmod_seq { pub value: s64, pub cnt: i32 }
#[repr(C)] pub struct prog_test_member { _private: [u8; 0] }
#[repr(C)] pub struct prog_test_member1 { _private: [u8; 0] }
#[repr(C)] pub struct prog_test_pass1 { pub x0: u64, pub x1: u64 }
#[repr(C)] pub struct prog_test_pass2 { _private: [u8; 0] }
#[repr(C)] pub struct prog_test_fail1 { _private: [u8; 0] }
#[repr(C)] pub struct prog_test_fail2 { _private: [u8; 0] }
#[repr(C)] pub struct prog_test_fail3 { _private: [u8; 0] }
#[repr(C)] pub struct prog_test_big_arg { pub a: u64, pub b: u64 }
#[repr(C)] pub struct refcount_t { pub refs: i32 }
#[repr(C)] pub struct prog_test_ref_kfunc { pub a: i32, pub b: i32, pub next: *mut prog_test_ref_kfunc, pub cnt: refcount_t }
#[repr(C)] pub struct bpf_testmod_ctx { pub usage: refcount_t, pub rcu: rcu_head }
#[repr(C)] pub struct bpf_testmod_arena_pair { pub a: u64, pub b: u64 }
#[repr(C)] pub struct st_ops_args { pub a: i32 }
#[repr(C)] pub struct init_sock_args { pub af: i32, pub type_: i32 }
#[repr(C)] pub struct addr_args { pub addr: [u8; 128], pub addrlen: i32 }
#[repr(C)] pub struct sendmsg_args { pub addr: addr_args, pub msg: [u8; 256], pub msglen: size_t }
#[repr(C)] pub struct bpf_testmod_test_read_ctx { pub buf: *mut c_char, pub off: loff_t, pub len: size_t }
#[repr(C)] pub struct bpf_testmod_test_write_ctx { pub buf: *mut c_char, pub off: loff_t, pub len: size_t }
#[repr(C)] pub struct bpf_testmod_test_writable_ctx { pub val: i32, pub early_ret: bool }

#[repr(C)]
pub struct bpf_testmod_ops {
    pub test_1: Option<unsafe extern "C" fn() -> i32>,
    pub test_2: Option<unsafe extern "C" fn(i32, i32)>,
    pub test_maybe_null: Option<unsafe extern "C" fn(i32, *mut task_struct) -> i32>,
    pub test_refcounted: Option<unsafe extern "C" fn(i32, *mut task_struct) -> i32>,
    pub test_refcounted_multi: Option<unsafe extern "C" fn(i32, *mut task_struct, *mut task_struct) -> i32>,
    pub test_return_ref_kptr: Option<unsafe extern "C" fn(i32, *mut task_struct, *mut cgroup) -> *mut task_struct>,
    pub data: i32,
    pub tramp_1: *mut c_void,
    pub tramp_40: *mut c_void,
}

#[repr(C)] pub struct bpf_testmod_ops2 { pub test_1: Option<unsafe extern "C" fn() -> i32> }
#[repr(C)]
pub struct bpf_testmod_ops3 {
    pub test_1: Option<unsafe extern "C" fn() -> i32>,
    pub test_2: Option<unsafe extern "C" fn() -> i32>,
    pub test_arena: Option<unsafe extern "C" fn(*mut u64) -> i32>,
    pub test_arena_nullable: Option<unsafe extern "C" fn(*mut u64) -> i32>,
    pub test_arena_stack: Option<unsafe extern "C" fn(u64, u64, u64, u64, u64, u64, u64, u64, *mut u64) -> i32>,
    pub test_arena_multislot: Option<unsafe extern "C" fn(bpf_testmod_arena_pair, *mut u64) -> i32>,
}
#[repr(C)]
pub struct bpf_testmod_st_ops {
    pub test_prologue: Option<unsafe extern "C" fn(*mut st_ops_args) -> i32>,
    pub test_epilogue: Option<unsafe extern "C" fn(*mut st_ops_args) -> i32>,
    pub test_pro_epilogue: Option<unsafe extern "C" fn(*mut st_ops_args) -> i32>,
}
#[repr(C)] pub struct bpf_testmod_multi_st_ops { pub test_1: Option<unsafe extern "C" fn(*mut st_ops_args) -> i32>, pub id: u32, pub node: hlist_node }

unsafe extern "C" {
    static mut THIS_MODULE: *mut c_void;
    static mut current: *mut task_struct;
    static mut kernel_kobj: *mut kobject;
    static bpf_base_func_proto: c_void;
    fn this_cpu_ptr(ptr: *mut c_void) -> *mut c_void;
    fn kzalloc(size: size_t, flags: u32) -> *mut c_void;
    fn kmalloc(size: size_t, flags: u32) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn refcount_set(r: *mut refcount_t, n: i32);
    fn refcount_inc(r: *mut refcount_t);
    fn refcount_dec_and_test(r: *mut refcount_t) -> bool;
    fn refcount_read(r: *const refcount_t) -> i32;
    fn call_rcu(head: *mut rcu_head, func: unsafe extern "C" fn(*mut rcu_head));
    fn call_rcu_tasks_trace(head: *mut rcu_head, func: unsafe extern "C" fn(*mut rcu_head));
    fn mutex_lock(m: *mut mutex);
    fn mutex_unlock(m: *mut mutex);
    fn mutex_init(m: *mut mutex);
    fn sock_create_kern(net: *mut c_void, family: i32, type_: i32, proto: i32, res: *mut *mut socket) -> i32;
    fn sock_release(sock: *mut socket);
    fn kernel_connect(sock: *mut socket, addr: *mut sockaddr_unsized, addrlen: i32, flags: i32) -> i32;
    fn kernel_bind(sock: *mut socket, addr: *mut sockaddr_unsized, addrlen: i32) -> i32;
    fn kernel_listen(sock: *mut socket, backlog: i32) -> i32;
    fn kernel_sendmsg(sock: *mut socket, msg: *mut msghdr, vec: *mut kvec, num: size_t, size: size_t) -> i32;
    fn sock_sendmsg(sock: *mut socket, msg: *mut msghdr) -> i32;
    fn kernel_getsockname(sock: *mut socket, addr: *mut sockaddr) -> i32;
    fn kernel_getpeername(sock: *mut socket, addr: *mut sockaddr) -> i32;
    fn iov_iter_kvec(iter: *mut c_void, direction: u32, kvec: *mut kvec, nr_segs: size_t, count: size_t);
    fn sysfs_create_bin_file(kobj: *mut kobject, attr: *mut bin_attribute) -> i32;
    fn sysfs_remove_bin_file(kobj: *mut kobject, attr: *mut bin_attribute);
    fn snprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> i32;
    fn strlen(s: *const c_char) -> size_t;
    fn kstrtoul(s: *const c_char, base: u32, res: *mut u64) -> i32;
    fn kern_path(name: *const c_char, flags: u32, path: *mut path) -> i32;
    fn path_put(path: *mut path);
    fn d_real_inode(dentry: *mut c_void) -> *mut c_void;
    fn uprobe_register(inode: *mut c_void, offset: u64, ref_ctr_offset: u64, consumer: *mut uprobe_consumer) -> *mut uprobe;
    fn uprobe_unregister_nosync(uprobe: *mut uprobe, consumer: *mut uprobe_consumer);
    fn uprobe_unregister_sync();
    fn bpf_tracing_btf_ctx_access(off: i32, size: i32, type_: i32, prog: *const bpf_prog, info: *mut bpf_insn_access_aux) -> bool;
    fn register_btf_kfunc_id_set(prog_type: i32, set: *const btf_kfunc_id_set) -> i32;
    fn register_btf_fmodret_id_set(set: *const btf_kfunc_id_set) -> i32;
    fn register_bpf_struct_ops(ops: *mut bpf_struct_ops, name: *mut c_void) -> i32;
    fn register_btf_id_dtor_kfuncs(dtors: *const btf_id_dtor_kfunc, cnt: size_t, owner: *mut c_void) -> i32;
    fn bpf_find_btf_id(name: *const c_char, kind: i32, btf: *mut *mut btf) -> i32;
    fn bpf_prog_get_assoc_struct_ops(aux: *mut bpf_prog_aux) -> *mut c_void;
    fn bpf_struct_ops_id(kdata: *mut c_void) -> u32;
    fn spin_lock_init(lock: *mut spinlock_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut u64);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: u64);
    fn hlist_add_head(node: *mut hlist_node, head: *mut hlist_head);
    fn hlist_del(node: *mut hlist_node);
    fn irq_work_queue(work: *mut irq_work);
    fn irq_work_sync(work: *mut irq_work);
    fn tasklet_schedule(t: *mut tasklet_struct);
    fn tasklet_kill(t: *mut tasklet_struct);
    fn msleep(ms: u32);
    fn strcmp(a: *const c_char, b: *const c_char) -> i32;
    fn strncmp(a: *const c_char, b: *const c_char, n: size_t) -> i32;
    fn printk(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn u64_stats_read(p: *const u64) -> u64;
    fn trace_bpf_testmod_fentry_test1_tp(a: i32);
    fn trace_bpf_testmod_fentry_test2_tp(a: i32, b: u64);
    fn trace_bpf_testmod_test_raw_tp_null_tp(arg: *mut c_void) -> i32;
    fn trace_bpf_testmod_test_read(task: *mut task_struct, ctx: *mut bpf_testmod_test_read_ctx);
    fn trace_bpf_testmod_test_nullable_bare_tp(arg: *mut c_void);
    fn trace_bpf_testmod_test_writable_bare_tp(ctx: *mut bpf_testmod_test_writable_ctx);
    fn trace_bpf_testmod_test_write_bare_tp(task: *mut task_struct, ctx: *mut bpf_testmod_test_write_ctx);
    fn bpf_fentry_test1(a: i32) -> i32;
}

#[repr(C)] pub struct bpf_testmod_struct_arg_1 { pub a: i32 }
#[repr(C)] pub struct bpf_testmod_struct_arg_2 { pub a: i64, pub b: i64 }
#[repr(C)] pub struct bpf_testmod_struct_arg_3 { pub a: i32, pub b: [i32; 0] }
#[repr(C)] pub struct bpf_testmod_struct_arg_4 { pub a: u64, pub b: i32 }
#[repr(C)] pub struct bpf_testmod_struct_arg_5 { pub a: c_char, pub b: i16, pub c: i32, pub d: i64 }
#[repr(C)] pub union bpf_testmod_union_arg_1 { pub a: c_char, pub b: i16, pub arg: core::mem::ManuallyDrop<bpf_testmod_struct_arg_1> }
#[repr(C)] pub union bpf_testmod_union_arg_2 { pub a: i32, pub b: i64, pub arg: core::mem::ManuallyDrop<bpf_testmod_struct_arg_2> }
#[repr(C)] pub struct bpf_testmod_btf_type_tag_1 { pub a: i32 }
#[repr(C)] pub struct bpf_testmod_btf_type_tag_2 { pub p: *mut bpf_testmod_btf_type_tag_1 }
#[repr(C)] pub struct bpf_testmod_btf_type_tag_3 { pub p: *mut bpf_testmod_btf_type_tag_1 }
#[repr(C)] pub struct bpf_kfunc_rcu_tasks_trace_data { pub rcu: rcu_head, pub done: *mut i32 }
#[repr(C)] pub struct testmod_uprobe { pub path: path, pub uprobe: *mut uprobe, pub consumer: uprobe_consumer }

#[unsafe(no_mangle)] pub static mut bpf_testmod_ksym_percpu: i32 = 123;
#[unsafe(no_mangle)] pub static mut bpf_testmod_test_struct_arg_result: i64 = 0;
static mut sock_lock: mutex = mutex { _private: [] };
static mut sock: *mut socket = core::ptr::null_mut();
static mut trusted_ptr: prog_test_member = prog_test_member { _private: [] };
static mut st_ops3: *mut bpf_testmod_ops3 = core::ptr::null_mut();
#[unsafe(no_mangle)] pub static mut bpf_testmod_fentry_ok: i32 = 0;
static mut testmod_uprobe_mutex: mutex = mutex { _private: [] };
static mut uprobe: testmod_uprobe = testmod_uprobe { path: path { dentry: core::ptr::null_mut() }, uprobe: core::ptr::null_mut(), consumer: uprobe_consumer { handler: uprobe_handler as *const c_void, ret_handler: uprobe_ret_handler as *const c_void } };
static mut st_ops_mutex: mutex = mutex { _private: [] };
static mut st_ops: *mut bpf_testmod_st_ops = core::ptr::null_mut();
static mut bpf_cgroup_from_id_id: i32 = 0;
static mut bpf_cgroup_release_id: i32 = 0;
#[unsafe(no_mangle)] pub static mut multi_st_ops_list: hlist_head = hlist_head { _private: [] };
static mut multi_st_ops_lock: spinlock_t = spinlock_t { _private: [] };

#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_testmod_test_struct_arg_1(a: bpf_testmod_struct_arg_2, b: i32, c: i32) -> i32 { unsafe { bpf_testmod_test_struct_arg_result = a.a + a.b + b as i64 + c as i64; bpf_testmod_test_struct_arg_result as i32 } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_testmod_test_struct_arg_2(a: i32, b: bpf_testmod_struct_arg_2, c: i32) -> i32 { unsafe { bpf_testmod_test_struct_arg_result = a as i64 + b.a + b.b + c as i64; bpf_testmod_test_struct_arg_result as i32 } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_testmod_test_struct_arg_3(a: i32, b: i32, c: bpf_testmod_struct_arg_2) -> i32 { unsafe { bpf_testmod_test_struct_arg_result = a as i64 + b as i64 + c.a + c.b; bpf_testmod_test_struct_arg_result as i32 } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_testmod_test_struct_arg_4(a: bpf_testmod_struct_arg_1, b: i32, c: i32, d: i32, e: bpf_testmod_struct_arg_2) -> i32 { unsafe { bpf_testmod_test_struct_arg_result = a.a as i64 + b as i64 + c as i64 + d as i64 + e.a + e.b; bpf_testmod_test_struct_arg_result as i32 } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_testmod_test_struct_arg_5() -> i32 { unsafe { bpf_testmod_test_struct_arg_result = 1; bpf_testmod_test_struct_arg_result as i32 } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_testmod_test_struct_arg_6(a: *mut bpf_testmod_struct_arg_3) -> i32 { unsafe { bpf_testmod_test_struct_arg_result = (*(*a).b.as_ptr()) as i64; bpf_testmod_test_struct_arg_result as i32 } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_testmod_test_struct_arg_7(a: u64, b: *mut c_void, c: i16, d: i32, e: *mut c_void, f: bpf_testmod_struct_arg_4) -> i32 { unsafe { bpf_testmod_test_struct_arg_result = a as i64 + b as i64 + c as i64 + d as i64 + e as i64 + f.a as i64 + f.b as i64; bpf_testmod_test_struct_arg_result as i32 } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_testmod_test_struct_arg_8(a: u64, b: *mut c_void, c: i16, d: i32, e: *mut c_void, f: bpf_testmod_struct_arg_4, g: i32) -> i32 { unsafe { bpf_testmod_test_struct_arg_result = a as i64 + b as i64 + c as i64 + d as i64 + e as i64 + f.a as i64 + f.b as i64 + g as i64; bpf_testmod_test_struct_arg_result as i32 } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_testmod_test_struct_arg_9(a: u64, b: *mut c_void, c: i16, d: i32, e: *mut c_void, f: c_char, g: i16, h: bpf_testmod_struct_arg_5, i: i64) -> i32 { unsafe { bpf_testmod_test_struct_arg_result = a as i64 + b as i64 + c as i64 + d as i64 + e as i64 + f as i64 + g as i64 + h.a as i64 + h.b as i64 + h.c as i64 + h.d + i; bpf_testmod_test_struct_arg_result as i32 } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_testmod_test_union_arg_1(a: bpf_testmod_union_arg_1, b: i32, c: i32) -> i32 { unsafe { bpf_testmod_test_struct_arg_result = a.arg.a as i64 + b as i64 + c as i64; bpf_testmod_test_struct_arg_result as i32 } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_testmod_test_union_arg_2(a: i32, b: bpf_testmod_union_arg_2) -> i32 { unsafe { bpf_testmod_test_struct_arg_result = a as i64 + b.arg.a + b.arg.b; bpf_testmod_test_struct_arg_result as i32 } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_testmod_test_arg_ptr_to_struct(a: *mut bpf_testmod_struct_arg_1) -> i32 { unsafe { bpf_testmod_test_struct_arg_result = (*a).a as i64; bpf_testmod_test_struct_arg_result as i32 } }

// #ifdef __SIZEOF_INT128__
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_testmod_test_int128_ret(a: i32) -> i128 { unsafe { bpf_testmod_test_struct_arg_result = a as i64; a as i128 } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_testmod_test_int128_arg(a: i128, b: i32, c: i64) -> i64 { unsafe { bpf_testmod_test_struct_arg_result = a as i64 + b as i64 + c; bpf_testmod_test_struct_arg_result } }
// #endif

#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_testmod_looooooooooooooooooooooooooooooong_name() {}
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_testmod_test_mod_kfunc(i: i32) { unsafe { *(this_cpu_ptr(&raw mut bpf_testmod_ksym_percpu as *mut c_void) as *mut i32) = i; } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_iter_testmod_seq_new(it: *mut bpf_iter_testmod_seq, value: s64, cnt: i32) -> i32 { unsafe { (*it).cnt = cnt; if cnt < 0 { return -EINVAL; } (*it).value = value; 0 } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_iter_testmod_seq_next(it: *mut bpf_iter_testmod_seq) -> *mut s64 { unsafe { if (*it).cnt <= 0 { return core::ptr::null_mut(); } (*it).cnt -= 1; &mut (*it).value } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_iter_testmod_seq_value(val: i32, it__iter: *mut bpf_iter_testmod_seq) -> s64 { unsafe { if (*it__iter).cnt < 0 { return 0; } val as s64 + (*it__iter).value } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_iter_testmod_seq_destroy(it: *mut bpf_iter_testmod_seq) { unsafe { (*it).cnt = 0; } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_common_test() {}
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_arena_arg_test(val__arena: *mut u64) -> u64 { unsafe { let old = *val__arena; *val__arena = old + 1; old } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_arena_cap_test(val__arena: *mut u64) -> u64 { val__arena as u64 }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_arena_cap_nullable_test(val__arena__nullable: *mut u64) -> u64 { val__arena__nullable as u64 }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_arena_args5_test(a__arena: *mut u64, b__arena: *mut u64, c__arena: *mut u64, d__arena: *mut u64, e__arena__nullable: *mut u64) -> u64 { unsafe { *a__arena + *b__arena + *c__arena + *d__arena + if !e__arena__nullable.is_null() { *e__arena__nullable } else { 0 } } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_arena_stack_arg_test(a: u64, b: u64, c: u64, d: u64, e: u64, f__arena: *mut u64) -> u64 { unsafe { a + b + c + d + e + *f__arena } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_arena_mixed_test(a__arena: *mut u64, b__arena__nullable: *mut u64) -> u64 { unsafe { *a__arena + if !b__arena__nullable.is_null() { *b__arena__nullable } else { 0 } } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_dynptr_test(_ptr: *mut bpf_dynptr, _ptr__nullable: *mut bpf_dynptr) {}
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_nested_acquire_nonzero_offset_test(_ptr: *mut sk_buff_head) -> *mut sk_buff { core::ptr::null_mut() }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_nested_acquire_zero_offset_test(_ptr: *mut sock_common) -> *mut sk_buff { core::ptr::null_mut() }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_nested_release_test(_ptr: *mut sk_buff) {}
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_trusted_vma_test(_ptr: *mut vm_area_struct) {}
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_trusted_task_test(_ptr: *mut task_struct) {}
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_trusted_num_test(_ptr: *mut i32) {}
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_rcu_task_test(_ptr: *mut task_struct) {}
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_ret_rcu_test() -> *mut task_struct { core::ptr::null_mut() }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_ret_rcu_test_nostruct(_rdonly_buf_size: i32) -> *mut i32 { core::ptr::null_mut() }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_get_default_trusted_ptr_test() -> *mut prog_test_member { &raw mut trusted_ptr }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_put_default_trusted_ptr_test(_trusted_ptr: *mut prog_test_member) { /* simulated argument-only put kfunc */ }

#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_testmod_ctx_create(err: *mut i32) -> *mut bpf_testmod_ctx { unsafe { let ctx = kzalloc(core::mem::size_of::<bpf_testmod_ctx>(), GFP_ATOMIC) as *mut bpf_testmod_ctx; if ctx.is_null() { *err = -ENOMEM; return core::ptr::null_mut(); } refcount_set(&mut (*ctx).usage, 1); ctx } }
unsafe extern "C" fn testmod_free_cb(head: *mut rcu_head) { unsafe { let ctx = (head as *mut u8).sub(core::mem::offset_of!(bpf_testmod_ctx, rcu)) as *mut bpf_testmod_ctx; kfree(ctx as *mut c_void); } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_testmod_ctx_release(ctx: *mut bpf_testmod_ctx) { unsafe { if ctx.is_null() { return; } if refcount_dec_and_test(&mut (*ctx).usage) { call_rcu(&mut (*ctx).rcu, testmod_free_cb); } } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_testmod_ctx_release_dtor(ctx: *mut c_void) { unsafe { bpf_testmod_ctx_release(ctx as *mut bpf_testmod_ctx); } }

unsafe extern "C" fn bpf_testmod_test_3() -> i32 { 0 }
unsafe extern "C" fn bpf_testmod_test_4() -> i32 { 0 }
unsafe extern "C" fn bpf_testmod_ops3__test_arena(_ptr__arena: *mut u64) -> i32 { 0 }
unsafe extern "C" fn bpf_testmod_ops3__test_arena_nullable(_ptr__arena__nullable: *mut u64) -> i32 { 0 }
unsafe extern "C" fn bpf_testmod_ops3__test_arena_stack(_a: u64, _b: u64, _c: u64, _d: u64, _e: u64, _f: u64, _g: u64, _h: u64, _ptr__arena: *mut u64) -> i32 { 0 }
unsafe extern "C" fn bpf_testmod_ops3__test_arena_multislot(_p: bpf_testmod_arena_pair, _ptr__arena: *mut u64) -> i32 { 0 }
static mut __bpf_testmod_ops3: bpf_testmod_ops3 = bpf_testmod_ops3 { test_1: Some(bpf_testmod_test_3), test_2: Some(bpf_testmod_test_4), test_arena: Some(bpf_testmod_ops3__test_arena), test_arena_nullable: Some(bpf_testmod_ops3__test_arena_nullable), test_arena_stack: Some(bpf_testmod_ops3__test_arena_stack), test_arena_multislot: Some(bpf_testmod_ops3__test_arena_multislot) };
unsafe extern "C" fn bpf_testmod_test_struct_ops3() { unsafe { if !st_ops3.is_null() { ((*st_ops3).test_1.unwrap())(); } } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_testmod_ops3_call_test_1() { unsafe { ((*st_ops3).test_1.unwrap())(); } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_testmod_ops3_call_test_2() { unsafe { ((*st_ops3).test_2.unwrap())(); } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_testmod_ops3_call_test_arena(ptr__arena: *mut u64) -> i32 { unsafe { ((*st_ops3).test_arena.unwrap())(ptr__arena) } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_testmod_ops3_call_test_arena_nullable(ptr__arena__nullable: *mut u64) -> i32 { unsafe { ((*st_ops3).test_arena_nullable.unwrap())(ptr__arena__nullable) } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_testmod_ops3_call_test_arena_stack(ptr__arena: *mut u64) -> i32 { unsafe { ((*st_ops3).test_arena_stack.unwrap())(1, 2, 3, 4, 5, 6, 7, 8, ptr__arena) } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_testmod_ops3_call_test_arena_multislot(ptr__arena: *mut u64) -> i32 { unsafe { let p = bpf_testmod_arena_pair { a: 11, b: 22 }; ((*st_ops3).test_arena_multislot.unwrap())(p, ptr__arena) } }

#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_testmod_test_btf_type_tag_user_1(arg: *mut bpf_testmod_btf_type_tag_1) -> i32 { unsafe { /* BTF_TYPE_EMIT typedefs */ (*arg).a } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_testmod_test_btf_type_tag_user_2(arg: *mut bpf_testmod_btf_type_tag_2) -> i32 { unsafe { (*(*arg).p).a } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_testmod_test_btf_type_tag_percpu_1(arg: *mut bpf_testmod_btf_type_tag_1) -> i32 { unsafe { (*arg).a } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_testmod_test_btf_type_tag_percpu_2(arg: *mut bpf_testmod_btf_type_tag_3) -> i32 { unsafe { (*(*arg).p).a } }

#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_testmod_loop_test(n: i32) -> i32 { let mut sum: i32 = 0; let mut i = 0; while i < n { unsafe { core::ptr::write_volatile(&mut sum, core::ptr::read_volatile(&sum).wrapping_add(i)); } i += 1; } unsafe { core::ptr::read_volatile(&sum) } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_testmod_return_ptr(arg: i32) -> *mut file { static mut f: file = file { _private: [] }; match arg { 1 => EINVAL as usize as *mut file, 2 => 0xcafe4a11usize as *mut file, 3 => (-EINVAL as isize) as *mut file, 4 => (1u64 << 60) as usize as *mut file, 5 => (!(1u64 << 30)) as usize as *mut file, 6 => &raw mut f, 7 => ((&raw mut f as usize) | 1) as *mut file, /* CONFIG_X86_64 case 8: VSYSCALL_ADDR */ _ => core::ptr::null_mut() } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_testmod_fentry_test1(a: i32) -> i32 { unsafe { trace_bpf_testmod_fentry_test1_tp(a); } a + 1 }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_testmod_fentry_test2(a: i32, b: u64) -> i32 { unsafe { trace_bpf_testmod_fentry_test2_tp(a, b); } (a as u64 + b) as i32 }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_testmod_fentry_test3(a: c_char, b: i32, c: u64) -> i32 { (a as u64 + b as u64 + c) as i32 }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_testmod_fentry_test7(a: u64, b: *mut c_void, c: i16, d: i32, e: *mut c_void, f: c_char, g: i32) -> i32 { (a as i64 + b as i64 + c as i64 + d as i64 + e as i64 + f as i64 + g as i64) as i32 }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_testmod_fentry_test11(a: u64, b: *mut c_void, c: i16, d: i32, e: *mut c_void, f: c_char, g: i32, h: u32, i: i64, j: __u64, k: u64) -> i32 { (a as i64 + b as i64 + c as i64 + d as i64 + e as i64 + f as i64 + g as i64 + h as i64 + i + j as i64 + k as i64) as i32 }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_testmod_stacktrace_test() { core::arch::asm!("", options(nomem, nostack, preserves_flags)); }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_testmod_stacktrace_test_3() { unsafe { bpf_testmod_stacktrace_test(); core::arch::asm!("", options(nomem, nostack, preserves_flags)); } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_testmod_stacktrace_test_2() { unsafe { bpf_testmod_stacktrace_test_3(); core::arch::asm!("", options(nomem, nostack, preserves_flags)); } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_testmod_stacktrace_test_1() { unsafe { bpf_testmod_stacktrace_test_2(); core::arch::asm!("", options(nomem, nostack, preserves_flags)); } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_testmod_trampoline_count_test() -> i32 { 0 }

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bpf_testmod_test_read(_file: *mut file, _kobj: *mut kobject, _bin_attr: *const bin_attribute, buf: *mut c_char, off: loff_t, len: size_t) -> ssize_t {
    unsafe {
        let mut ctx = bpf_testmod_test_read_ctx { buf, off, len };
        let struct_arg1 = bpf_testmod_struct_arg_1 { a: 10 };
        let struct_arg1_2 = bpf_testmod_struct_arg_1 { a: -1 };
        let struct_arg2 = bpf_testmod_struct_arg_2 { a: 2, b: 3 };
        let struct_arg4 = bpf_testmod_struct_arg_4 { a: 21, b: 22 };
        let struct_arg5 = bpf_testmod_struct_arg_5 { a: 23, b: 24, c: 25, d: 26 };
        let union_arg1 = bpf_testmod_union_arg_1 { arg: core::mem::ManuallyDrop::new(bpf_testmod_struct_arg_1 { a: 1 }) };
        let union_arg2 = bpf_testmod_union_arg_2 { arg: core::mem::ManuallyDrop::new(bpf_testmod_struct_arg_2 { a: 2, b: 3 }) };
        let mut i = 1;
        while !bpf_testmod_return_ptr(i).is_null() { i += 1; }
        let _ = bpf_testmod_test_struct_arg_1(struct_arg2, 1, 4);
        let struct_arg2 = bpf_testmod_struct_arg_2 { a: 2, b: 3 };
        let _ = bpf_testmod_test_struct_arg_2(1, struct_arg2, 4);
        let struct_arg2 = bpf_testmod_struct_arg_2 { a: 2, b: 3 };
        let _ = bpf_testmod_test_struct_arg_3(1, 4, struct_arg2);
        let struct_arg2 = bpf_testmod_struct_arg_2 { a: 2, b: 3 };
        let _ = bpf_testmod_test_struct_arg_4(struct_arg1, 1, 2, 3, struct_arg2);
        let _ = bpf_testmod_test_struct_arg_5();
        let _ = bpf_testmod_test_struct_arg_7(16, 17usize as *mut c_void, 18, 19, 20usize as *mut c_void, struct_arg4);
        let struct_arg4 = bpf_testmod_struct_arg_4 { a: 21, b: 22 };
        let _ = bpf_testmod_test_struct_arg_8(16, 17usize as *mut c_void, 18, 19, 20usize as *mut c_void, struct_arg4, 23);
        let _ = bpf_testmod_test_struct_arg_9(16, 17usize as *mut c_void, 18, 19, 20usize as *mut c_void, 21, 22, struct_arg5, 27);
        let _ = bpf_testmod_test_union_arg_1(union_arg1, 4, 5);
        let _ = bpf_testmod_test_union_arg_2(6, union_arg2);
        let _ = bpf_testmod_test_arg_ptr_to_struct(&struct_arg1_2 as *const _ as *mut _);
        let _ = bpf_testmod_test_int128_ret(i);
        let _ = bpf_testmod_test_int128_arg(1, 2, 3);
        let _ = trace_bpf_testmod_test_raw_tp_null_tp(core::ptr::null_mut());
        bpf_testmod_test_struct_ops3();
        let struct_arg3 = kmalloc(core::mem::size_of::<bpf_testmod_struct_arg_3>() + core::mem::size_of::<i32>(), GFP_KERNEL) as *mut bpf_testmod_struct_arg_3;
        if !struct_arg3.is_null() {
            *((struct_arg3 as *mut u8).add(core::mem::size_of::<i32>()) as *mut i32) = 1;
            let _ = bpf_testmod_test_struct_arg_6(struct_arg3);
            kfree(struct_arg3 as *mut c_void);
        }
        if bpf_testmod_loop_test(101) > 100 { trace_bpf_testmod_test_read(current, &mut ctx); }
        trace_bpf_testmod_test_nullable_bare_tp(core::ptr::null_mut());
        if len == 64 {
            let mut writable = bpf_testmod_test_writable_ctx { val: 1024, early_ret: false };
            trace_bpf_testmod_test_writable_bare_tp(&mut writable);
            if writable.early_ret { return snprintf(buf, len, b"%d\n\0".as_ptr() as *const c_char, writable.val) as ssize_t; }
        }
        if bpf_testmod_fentry_test1(1) != 2 || bpf_testmod_fentry_test2(2, 3) != 5 || bpf_testmod_fentry_test3(4, 5, 6) != 15 || bpf_testmod_fentry_test7(16, 17usize as *mut c_void, 18, 19, 20usize as *mut c_void, 21, 22) != 133 || bpf_testmod_fentry_test11(16, 17usize as *mut c_void, 18, 19, 20usize as *mut c_void, 21, 22, 23, 24, 25, 26) != 231 { return -EIO as ssize_t; }
        bpf_testmod_trampoline_count_test();
        bpf_testmod_stacktrace_test_1();
        bpf_testmod_fentry_ok = 1;
        -EIO as ssize_t
    }
}

#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_testmod_test_write(_file: *mut file, _kobj: *mut kobject, _bin_attr: *const bin_attribute, buf: *mut c_char, off: loff_t, len: size_t) -> ssize_t { unsafe { let mut ctx = bpf_testmod_test_write_ctx { buf, off, len }; trace_bpf_testmod_test_write_bare_tp(current, &mut ctx); -EIO as ssize_t } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_fentry_shadow_test(a: i32) -> i32 { a + 2 }

static mut bin_attr_bpf_testmod_file: bin_attribute = bin_attribute { attr: attribute { name: b"bpf_testmod\0".as_ptr() as *const c_char, mode: 0o666 }, read: bpf_testmod_test_read as *const c_void, write: bpf_testmod_test_write as *const c_void };

unsafe extern "C" fn uprobe_handler(_self: *mut uprobe_consumer, regs: *mut pt_regs, _data: *mut __u64) -> i32 { unsafe { (*regs).cx = 0x87654321feebdaed; 0 } }
unsafe extern "C" fn uprobe_ret_handler(_self: *mut uprobe_consumer, _func: u64, regs: *mut pt_regs, _data: *mut __u64) -> i32 { unsafe { (*regs).ax = 0x12345678deadbeef; (*regs).r11 = (-1i64) as u64; 0 } }
unsafe extern "C" fn testmod_register_uprobe(offset: loff_t) -> i32 { unsafe { let mut err = -EBUSY; if !uprobe.uprobe.is_null() { return -EBUSY; } mutex_lock(&raw mut testmod_uprobe_mutex); if !uprobe.uprobe.is_null() { mutex_unlock(&raw mut testmod_uprobe_mutex); return err; } err = kern_path(b"/proc/self/exe\0".as_ptr() as *const c_char, LOOKUP_FOLLOW, &mut uprobe.path); if err == 0 { uprobe.uprobe = uprobe_register(d_real_inode(uprobe.path.dentry), offset as u64, 0, &mut uprobe.consumer); if uprobe.uprobe as isize as isize < 0 { err = uprobe.uprobe as isize as i32; path_put(&mut uprobe.path); uprobe.uprobe = core::ptr::null_mut(); } } mutex_unlock(&raw mut testmod_uprobe_mutex); err } }
unsafe extern "C" fn testmod_unregister_uprobe() { unsafe { mutex_lock(&raw mut testmod_uprobe_mutex); if !uprobe.uprobe.is_null() { uprobe_unregister_nosync(uprobe.uprobe, &mut uprobe.consumer); uprobe_unregister_sync(); path_put(&mut uprobe.path); uprobe.uprobe = core::ptr::null_mut(); } mutex_unlock(&raw mut testmod_uprobe_mutex); } }
unsafe extern "C" fn bpf_testmod_uprobe_write(_file: *mut file, _kobj: *mut kobject, _bin_attr: *const bin_attribute, buf: *mut c_char, _off: loff_t, _len: size_t) -> ssize_t { unsafe { let mut offset: u64 = 0; let mut err = 0; if kstrtoul(buf, 0, &mut offset) != 0 { return -EINVAL as ssize_t; } if offset != 0 { err = testmod_register_uprobe(offset as loff_t); } else { testmod_unregister_uprobe(); } if err != 0 { err as ssize_t } else { strlen(buf) as ssize_t } } }
static mut bin_attr_bpf_testmod_uprobe_file: bin_attribute = bin_attribute { attr: attribute { name: b"bpf_testmod_uprobe\0".as_ptr() as *const c_char, mode: 0o666 }, read: core::ptr::null(), write: bpf_testmod_uprobe_write as *const c_void };
unsafe extern "C" fn register_bpf_testmod_uprobe() -> i32 { unsafe { sysfs_create_bin_file(kernel_kobj, &raw mut bin_attr_bpf_testmod_uprobe_file) } }
unsafe extern "C" fn unregister_bpf_testmod_uprobe() { unsafe { testmod_unregister_uprobe(); sysfs_remove_bin_file(kernel_kobj, &raw mut bin_attr_bpf_testmod_uprobe_file); } }

// BTF_KFUNCS_START/END, BTF_ID_FLAGS, BTF_ID_LIST, EXPORT_SYMBOL,
// ALLOW_ERROR_INJECTION, CFI_NOSEAL, module_init/module_exit and MODULE_*
// declarations are kernel macro metadata and are preserved here by comment.

static bpf_testmod_common_kfunc_set: btf_kfunc_id_set = btf_kfunc_id_set { owner: core::ptr::null_mut(), set: core::ptr::null() };
static bpf_testmod_kfunc_set: btf_kfunc_id_set = btf_kfunc_id_set { owner: core::ptr::null_mut(), set: core::ptr::null() };
static bpf_testmod_trampoline_count_fmodret_set: btf_kfunc_id_set = btf_kfunc_id_set { owner: core::ptr::null_mut(), set: core::ptr::null() };
static bpf_testmod_dtor_ids: [i32; 2] = [0, 0];

#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_call_test1(_sk: *mut sock, a: u32, b: u64, c: u32, d: u64) -> u64 { a as u64 + b + c as u64 + d }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_call_test2(_sk: *mut sock, a: u32, b: u32) -> i32 { (a + b) as i32 }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_call_test3(sk: *mut sock) -> *mut sock { sk }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_call_test4(a: i8, b: i16, c: i32, d: i64) -> i64 { let mut val: i64 = a as i64; if unsafe { core::ptr::read_volatile(&val) } >= 0 { return 1; } val = b as i64; if unsafe { core::ptr::read_volatile(&val) } >= 0 { return 2; } val = c as i64; if unsafe { core::ptr::read_volatile(&val) } >= 0 { return 3; } a as i64 + b as i64 + c as i64 + d }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_call_test5(a: u8, b: u16, c: u32) -> i32 { let mut val: i64 = a as i64; if unsafe { core::ptr::read_volatile(&val) } != a as u64 as i64 { return 1; } if unsafe { core::ptr::read_volatile(&val) } < 0 { return 2; } val = b as i64; if unsafe { core::ptr::read_volatile(&val) } != b as u64 as i64 { return 3; } if unsafe { core::ptr::read_volatile(&val) } < 0 { return 4; } val = c as i64; if unsafe { core::ptr::read_volatile(&val) } != c as u64 as i64 { return 5; } if unsafe { core::ptr::read_volatile(&val) } < 0 { return 6; } 0 }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_call_stack_arg(a: u64, b: u64, c: u64, d: u64, e: u64, f: u64, g: u64, h: u64, i: u64, j: u64) -> u64 { a + b + c + d + e + f + g + h + i + j }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_call_stack_arg_ptr(a: u64, b: u64, c: u64, d: u64, e: u64, f: u64, g: u64, h: u64, i: u64, p: *mut prog_test_pass1) -> u64 { unsafe { a + b + c + d + e + f + g + h + i + (*p).x0 + (*p).x1 } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_call_stack_arg_mix(a: u64, b: u64, c: u64, d: u64, e: u64, f: u64, g: u64, p: *mut prog_test_pass1, h: u64, q: *mut prog_test_pass1) -> u64 { unsafe { a + b + c + d + e + f + g + (*p).x0 + h + (*q).x1 } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_call_stack_arg_dynptr(a: u64, b: u64, c: u64, d: u64, e: u64, f: u64, g: u64, h: u64, i: u64, ptr: *mut bpf_dynptr) -> u64 { unsafe { let kern_ptr = ptr as *const bpf_dynptr_kern; a + b + c + d + e + f + g + h + i + ((*kern_ptr).size & 0xFFFFFF) as u64 } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_call_stack_arg_mem(a: u64, b: u64, c: u64, d: u64, e: u64, mem: *mut c_void, mem__sz: i32) -> u64 { unsafe { let p = mem as *const u8; let mut sum = a + b + c + d + e; let mut i = 0; while i < mem__sz { sum += *p.add(i as usize) as u64; i += 1; } sum } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_call_stack_arg_iter(a: u64, b: u64, c: u64, d: u64, e: u64, f: u64, g: u64, h: u64, i: u64, it__iter: *mut bpf_iter_testmod_seq) -> u64 { unsafe { a + b + c + d + e + f + g + h + i + (*it__iter).value as u64 } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_call_stack_arg_const_str(a: u64, b: u64, c: u64, d: u64, e: u64, f: u64, g: u64, h: u64, i: u64, _str__str: *const c_char) -> u64 { a + b + c + d + e + f + g + h + i }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_call_stack_arg_timer(a: u64, b: u64, c: u64, d: u64, e: u64, f: u64, g: u64, h: u64, i: u64, _timer: *mut bpf_timer) -> u64 { a + b + c + d + e + f + g + h + i }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_call_stack_arg_big(a: u64, b: u64, c: u64, d: u64, e: u64, s: prog_test_big_arg) -> u64 { a + b + c + d + e + s.a + s.b }

static mut prog_test_struct: prog_test_ref_kfunc = prog_test_ref_kfunc { a: 42, b: 108, next: core::ptr::null_mut(), cnt: refcount_t { refs: 1 } };
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_call_test_acquire(_scalar_ptr: *mut u64) -> *mut prog_test_ref_kfunc { unsafe { if prog_test_struct.next.is_null() { prog_test_struct.next = &raw mut prog_test_struct; } refcount_inc(&mut prog_test_struct.cnt); &raw mut prog_test_struct } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_call_test_offset(_p: *mut prog_test_ref_kfunc) { /* WARN_ON_ONCE(1) */ }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_call_memb_acquire() -> *mut prog_test_member { core::ptr::null_mut() }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_call_memb1_release(_p: *mut prog_test_member1) {}
unsafe extern "C" fn __bpf_kfunc_call_test_get_mem(p: *mut prog_test_ref_kfunc, size: i32) -> *mut i32 { if size as usize > 2 * core::mem::size_of::<i32>() { return core::ptr::null_mut(); } p as *mut i32 }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_call_test_get_rdwr_mem(p: *mut prog_test_ref_kfunc, rdwr_buf_size: i32) -> *mut i32 { __bpf_kfunc_call_test_get_mem(p, rdwr_buf_size) }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_call_test_get_rdonly_mem(p: *mut prog_test_ref_kfunc, rdonly_buf_size: i32) -> *mut i32 { __bpf_kfunc_call_test_get_mem(p, rdonly_buf_size) }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_call_test_acq_rdonly_mem(p: *mut prog_test_ref_kfunc, rdonly_buf_size: i32) -> *mut i32 { __bpf_kfunc_call_test_get_mem(p, rdonly_buf_size) }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_call_int_mem_release(_p: *mut i32) {}
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_call_test_pass_ctx(_skb: *mut __sk_buff) {}
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_call_test_pass1(_p: *mut prog_test_pass1) {}
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_call_test_pass2(_p: *mut prog_test_pass2) {}
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_call_test_fail1(_p: *mut prog_test_fail1) {}
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_call_test_fail2(_p: *mut prog_test_fail2) {}
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_call_test_fail3(_p: *mut prog_test_fail3) {}
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_call_test_mem_len_pass1(_mem: *mut c_void, _mem__sz: i32) {}
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_call_test_mem_len_fail1(_mem: *mut c_void, _len: i32) {}
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_call_test_mem_len_fail2(_mem: *mut u64, _len: i32) {}
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_call_test_ref(_p: *mut prog_test_ref_kfunc) { /* p != NULL, but p->cnt could be 0 */ }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_call_test_destructive() {}
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_call_test_static_unused_arg(arg: u32, _unused: u32) -> u32 { arg }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_call_test_sleepable() {}
unsafe extern "C" fn bpf_kfunc_rcu_tasks_trace_cb(rhp: *mut rcu_head) { unsafe { let data = rhp as *mut bpf_kfunc_rcu_tasks_trace_data; core::ptr::write_volatile((*data).done, 1); kfree(data as *mut c_void); } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_call_test_call_rcu_tasks_trace(done: *mut i32) -> i32 { unsafe { let data = kmalloc(core::mem::size_of::<bpf_kfunc_rcu_tasks_trace_data>(), GFP_ATOMIC) as *mut bpf_kfunc_rcu_tasks_trace_data; if data.is_null() { return -ENOMEM; } (*data).done = done; call_rcu_tasks_trace(&mut (*data).rcu, bpf_kfunc_rcu_tasks_trace_cb); 0 } }

#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_init_sock(args: *mut init_sock_args) -> i32 { unsafe { let mut err; mutex_lock(&raw mut sock_lock); if !sock.is_null() { pr_err(b"%s called without releasing old sock\0".as_ptr() as *const c_char, b"bpf_kfunc_init_sock\0".as_ptr()); err = -EPERM; mutex_unlock(&raw mut sock_lock); return err; } let proto = match (*args).af { AF_INET | AF_INET6 => if (*args).type_ == SOCK_STREAM { IPPROTO_TCP } else { IPPROTO_UDP }, AF_UNIX => PF_UNIX, _ => { pr_err(b"invalid address family %d\n\0".as_ptr() as *const c_char, (*args).af); mutex_unlock(&raw mut sock_lock); return -EINVAL; } }; err = sock_create_kern((*(*current).nsproxy).net_ns, (*args).af, (*args).type_, proto, &raw mut sock); if err == 0 { (*(*sock).sk).sk_sndtimeo = (CONNECT_TIMEOUT_SEC * HZ) as i64; } mutex_unlock(&raw mut sock_lock); err } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_close_sock() { unsafe { mutex_lock(&raw mut sock_lock); if !sock.is_null() { sock_release(sock); sock = core::ptr::null_mut(); } mutex_unlock(&raw mut sock_lock); } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_call_kernel_connect(args: *mut addr_args) -> i32 { unsafe { if (*args).addrlen as usize > core::mem::size_of_val(&(*args).addr) { return -EINVAL; } mutex_lock(&raw mut sock_lock); let err = if sock.is_null() { pr_err(b"%s called without initializing sock\0".as_ptr() as *const c_char, b"bpf_kfunc_call_kernel_connect\0".as_ptr()); -EPERM } else { kernel_connect(sock, &mut (*args).addr as *mut _ as *mut sockaddr_unsized, (*args).addrlen, 0) }; mutex_unlock(&raw mut sock_lock); err } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_call_kernel_bind(args: *mut addr_args) -> i32 { unsafe { if (*args).addrlen as usize > core::mem::size_of_val(&(*args).addr) { return -EINVAL; } mutex_lock(&raw mut sock_lock); let err = if sock.is_null() { pr_err(b"%s called without initializing sock\0".as_ptr() as *const c_char, b"bpf_kfunc_call_kernel_bind\0".as_ptr()); -EPERM } else { kernel_bind(sock, &mut (*args).addr as *mut _ as *mut sockaddr_unsized, (*args).addrlen) }; mutex_unlock(&raw mut sock_lock); err } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_call_kernel_listen() -> i32 { unsafe { mutex_lock(&raw mut sock_lock); let err = if sock.is_null() { pr_err(b"%s called without initializing sock\0".as_ptr() as *const c_char, b"bpf_kfunc_call_kernel_listen\0".as_ptr()); -EPERM } else { kernel_listen(sock, 128) }; mutex_unlock(&raw mut sock_lock); err } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_call_kernel_sendmsg(args: *mut sendmsg_args) -> i32 { unsafe { let mut msg = msghdr { msg_name: &mut (*args).addr.addr as *mut _ as *mut c_void, msg_namelen: (*args).addr.addrlen, msg_iter: core::mem::zeroed() }; let mut iov = kvec { iov_base: core::ptr::null_mut(), iov_len: 0 }; if (*args).addr.addrlen as usize > core::mem::size_of_val(&(*args).addr.addr) || (*args).msglen > core::mem::size_of_val(&(*args).msg) { return -EINVAL; } iov.iov_base = (*args).msg.as_mut_ptr() as *mut c_void; iov.iov_len = (*args).msglen; mutex_lock(&raw mut sock_lock); let err = if sock.is_null() { pr_err(b"%s called without initializing sock\0".as_ptr() as *const c_char, b"bpf_kfunc_call_kernel_sendmsg\0".as_ptr()); -EPERM } else { kernel_sendmsg(sock, &mut msg, &mut iov, 1, (*args).msglen) }; (*args).addr.addrlen = msg.msg_namelen; mutex_unlock(&raw mut sock_lock); err } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_call_sock_sendmsg(args: *mut sendmsg_args) -> i32 { unsafe { let mut msg = msghdr { msg_name: &mut (*args).addr.addr as *mut _ as *mut c_void, msg_namelen: (*args).addr.addrlen, msg_iter: core::mem::zeroed() }; let mut iov = kvec { iov_base: (*args).msg.as_mut_ptr() as *mut c_void, iov_len: (*args).msglen }; if (*args).addr.addrlen as usize > core::mem::size_of_val(&(*args).addr.addr) || (*args).msglen > core::mem::size_of_val(&(*args).msg) { return -EINVAL; } iov_iter_kvec(&mut msg.msg_iter, ITER_SOURCE, &mut iov, 1, (*args).msglen); mutex_lock(&raw mut sock_lock); let err = if sock.is_null() { pr_err(b"%s called without initializing sock\0".as_ptr() as *const c_char, b"bpf_kfunc_call_sock_sendmsg\0".as_ptr()); -EPERM } else { sock_sendmsg(sock, &mut msg) }; (*args).addr.addrlen = msg.msg_namelen; mutex_unlock(&raw mut sock_lock); err } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_call_kernel_getsockname(args: *mut addr_args) -> i32 { unsafe { mutex_lock(&raw mut sock_lock); let mut err = if sock.is_null() { pr_err(b"%s called without initializing sock\0".as_ptr() as *const c_char, b"bpf_kfunc_call_kernel_getsockname\0".as_ptr()); -EPERM } else { kernel_getsockname(sock, &mut (*args).addr as *mut _ as *mut sockaddr) }; if err >= 0 { (*args).addrlen = err; err = 0; } mutex_unlock(&raw mut sock_lock); err } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_call_kernel_getpeername(args: *mut addr_args) -> i32 { unsafe { mutex_lock(&raw mut sock_lock); let mut err = if sock.is_null() { pr_err(b"%s called without initializing sock\0".as_ptr() as *const c_char, b"bpf_kfunc_call_kernel_getpeername\0".as_ptr()); -EPERM } else { kernel_getpeername(sock, &mut (*args).addr as *mut _ as *mut sockaddr) }; if err >= 0 { (*args).addrlen = err; err = 0; } mutex_unlock(&raw mut sock_lock); err } }

#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_st_ops_test_prologue(args: *mut st_ops_args) -> i32 { unsafe { let mut ret = -1; mutex_lock(&raw mut st_ops_mutex); if !st_ops.is_null() && (*st_ops).test_prologue.is_some() { ret = ((*st_ops).test_prologue.unwrap())(args); } mutex_unlock(&raw mut st_ops_mutex); ret } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_st_ops_test_epilogue(args: *mut st_ops_args) -> i32 { unsafe { let mut ret = -1; mutex_lock(&raw mut st_ops_mutex); if !st_ops.is_null() && (*st_ops).test_epilogue.is_some() { ret = ((*st_ops).test_epilogue.unwrap())(args); } mutex_unlock(&raw mut st_ops_mutex); ret } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_st_ops_test_pro_epilogue(args: *mut st_ops_args) -> i32 { unsafe { let mut ret = -1; mutex_lock(&raw mut st_ops_mutex); if !st_ops.is_null() && (*st_ops).test_pro_epilogue.is_some() { ret = ((*st_ops).test_pro_epilogue.unwrap())(args); } mutex_unlock(&raw mut st_ops_mutex); ret } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_st_ops_inc10(args: *mut st_ops_args) -> i32 { unsafe { (*args).a += 10; (*args).a } }

#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_testmod_test_hardirq_fn() { core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst); }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_testmod_test_softirq_fn() { core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst); }
unsafe extern "C" fn ctx_check_tasklet_fn(_t: *mut tasklet_struct) { unsafe { bpf_testmod_test_softirq_fn(); } }
static mut ctx_check_tasklet: tasklet_struct = tasklet_struct { _private: [] };
unsafe extern "C" fn ctx_check_irq_fn(_work: *mut irq_work) { unsafe { bpf_testmod_test_hardirq_fn(); tasklet_schedule(&raw mut ctx_check_tasklet); } }
static mut ctx_check_irq: irq_work = irq_work { _private: [] };
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_trigger_ctx_check() { unsafe { irq_work_queue(&raw mut ctx_check_irq); } }

unsafe extern "C" fn bpf_testmod_ops_init(_btf: *mut btf) -> i32 { 0 }
unsafe extern "C" fn bpf_testmod_ops_is_valid_access(off: i32, size: i32, type_: i32, prog: *const bpf_prog, info: *mut bpf_insn_access_aux) -> bool { unsafe { bpf_tracing_btf_ctx_access(off, size, type_, prog, info) } }
unsafe extern "C" fn bpf_testmod_ops_init_member(_t: *const btf_type, member: *const btf_member, kdata: *mut c_void, udata: *const c_void) -> i32 { unsafe { if (*member).offset == (core::mem::offset_of!(bpf_testmod_ops, data) * 8) as u32 { (*(kdata as *mut bpf_testmod_ops)).data = (*(udata as *const bpf_testmod_ops)).data; return 1; } 0 } }
static bpf_testmod_verifier_ops: bpf_verifier_ops = bpf_verifier_ops { get_func_proto: unsafe { &bpf_base_func_proto }, is_valid_access: bpf_testmod_ops_is_valid_access as *const c_void, btf_struct_access: core::ptr::null(), gen_prologue: core::ptr::null(), gen_epilogue: core::ptr::null() };
static bpf_testmod_verifier_ops3: bpf_verifier_ops = bpf_verifier_ops { get_func_proto: core::ptr::null(), is_valid_access: bpf_testmod_ops_is_valid_access as *const c_void, btf_struct_access: core::ptr::null(), gen_prologue: core::ptr::null(), gen_epilogue: core::ptr::null() };
unsafe extern "C" fn bpf_dummy_reg(kdata: *mut c_void, _link: *mut bpf_link) -> i32 { unsafe { let ops = kdata as *mut bpf_testmod_ops; if let Some(f) = (*ops).test_1 { f(); } if let Some(f) = (*ops).test_2 { f(4, (*ops).data); } 0 } }
unsafe extern "C" fn bpf_dummy_unreg(_kdata: *mut c_void, _link: *mut bpf_link) {}
unsafe extern "C" fn bpf_testmod_test_1() -> i32 { 0 }
unsafe extern "C" fn bpf_testmod_test_2(_a: i32, _b: i32) {}
unsafe extern "C" fn bpf_testmod_tramp(_value: i32) -> i32 { 0 }
unsafe extern "C" fn bpf_testmod_ops__test_maybe_null(_dummy: i32, _task__nullable: *mut task_struct) -> i32 { 0 }
unsafe extern "C" fn bpf_testmod_ops__test_refcounted(_dummy: i32, _task__ref: *mut task_struct) -> i32 { 0 }
unsafe extern "C" fn bpf_testmod_ops__test_refcounted_multi(_dummy: i32, _task__nullable: *mut task_struct, _task__ref: *mut task_struct) -> i32 { 0 }
unsafe extern "C" fn bpf_testmod_ops__test_return_ref_kptr(_dummy: i32, _task__ref: *mut task_struct, _cgrp: *mut cgroup) -> *mut task_struct { core::ptr::null_mut() }
static mut __bpf_testmod_ops: bpf_testmod_ops = bpf_testmod_ops { test_1: Some(bpf_testmod_test_1), test_2: Some(bpf_testmod_test_2), test_maybe_null: Some(bpf_testmod_ops__test_maybe_null), test_refcounted: Some(bpf_testmod_ops__test_refcounted), test_refcounted_multi: Some(bpf_testmod_ops__test_refcounted_multi), test_return_ref_kptr: Some(bpf_testmod_ops__test_return_ref_kptr), data: 0, tramp_1: core::ptr::null_mut(), tramp_40: core::ptr::null_mut() };
#[unsafe(no_mangle)] pub static mut bpf_bpf_testmod_ops: bpf_struct_ops = bpf_struct_ops { verifier_ops: &bpf_testmod_verifier_ops, init: bpf_testmod_ops_init as *const c_void, init_member: bpf_testmod_ops_init_member as *const c_void, reg: bpf_dummy_reg as *const c_void, unreg: bpf_dummy_unreg as *const c_void, check_member: core::ptr::null(), cfi_stubs: &raw const __bpf_testmod_ops as *const c_void, name: b"bpf_testmod_ops\0".as_ptr() as *const c_char, owner: core::ptr::null_mut() };
unsafe extern "C" fn bpf_dummy_reg2(kdata: *mut c_void, _link: *mut bpf_link) -> i32 { unsafe { let ops = kdata as *mut bpf_testmod_ops2; ((*ops).test_1.unwrap())(); 0 } }
static mut __bpf_testmod_ops2: bpf_testmod_ops2 = bpf_testmod_ops2 { test_1: Some(bpf_testmod_test_1) };
#[unsafe(no_mangle)] pub static mut bpf_testmod_ops2: bpf_struct_ops = bpf_struct_ops { verifier_ops: &bpf_testmod_verifier_ops, init: bpf_testmod_ops_init as *const c_void, init_member: bpf_testmod_ops_init_member as *const c_void, reg: bpf_dummy_reg2 as *const c_void, unreg: bpf_dummy_unreg as *const c_void, check_member: core::ptr::null(), cfi_stubs: &raw const __bpf_testmod_ops2 as *const c_void, name: b"bpf_testmod_ops2\0".as_ptr() as *const c_char, owner: core::ptr::null_mut() };

unsafe extern "C" fn st_ops3_reg(kdata: *mut c_void, _link: *mut bpf_link) -> i32 { unsafe { let mut err = 0; mutex_lock(&raw mut st_ops_mutex); if !st_ops3.is_null() { pr_err(b"st_ops has already been registered\n\0".as_ptr() as *const c_char); err = -EEXIST; } else { st_ops3 = kdata as *mut bpf_testmod_ops3; } mutex_unlock(&raw mut st_ops_mutex); err } }
unsafe extern "C" fn st_ops3_unreg(_kdata: *mut c_void, _link: *mut bpf_link) { unsafe { mutex_lock(&raw mut st_ops_mutex); st_ops3 = core::ptr::null_mut(); mutex_unlock(&raw mut st_ops_mutex); } }
unsafe extern "C" fn test_1_recursion_detected(prog: *mut bpf_prog) { unsafe { let stats = this_cpu_ptr((*prog).stats as *mut c_void) as *mut bpf_prog_stats; printk(b"bpf_testmod: oh no, recursing into test_1, recursion_misses %llu\0".as_ptr() as *const c_char, u64_stats_read(&(*stats).misses)); } }
unsafe extern "C" fn st_ops3_check_member(_t: *const btf_type, _member: *const btf_member, prog: *const bpf_prog) -> i32 { unsafe { (*(*prog).aux).priv_stack_requested = true; (*(*prog).aux).recursion_detected = Some(test_1_recursion_detected); 0 } }
#[unsafe(no_mangle)] pub static mut bpf_testmod_ops3: bpf_struct_ops = bpf_struct_ops { verifier_ops: &bpf_testmod_verifier_ops3, init: bpf_testmod_ops_init as *const c_void, init_member: bpf_testmod_ops_init_member as *const c_void, reg: st_ops3_reg as *const c_void, unreg: st_ops3_unreg as *const c_void, check_member: st_ops3_check_member as *const c_void, cfi_stubs: &raw const __bpf_testmod_ops3 as *const c_void, name: b"bpf_testmod_ops3\0".as_ptr() as *const c_char, owner: core::ptr::null_mut() };

unsafe extern "C" fn bpf_test_mod_st_ops__test_prologue(_args: *mut st_ops_args) -> i32 { 0 }
unsafe extern "C" fn bpf_test_mod_st_ops__test_epilogue(_args: *mut st_ops_args) -> i32 { 0 }
unsafe extern "C" fn bpf_test_mod_st_ops__test_pro_epilogue(_args: *mut st_ops_args) -> i32 { 0 }
unsafe extern "C" fn st_ops_gen_prologue_with_kfunc(_insn_buf: *mut bpf_insn, _direct_write: bool, _prog: *const bpf_prog) -> i32 { /* emits BPF_MOV64_REG/BPF_CALL_KFUNC/... instruction sequence */ 13 }
unsafe extern "C" fn st_ops_gen_epilogue_with_kfunc(_insn_buf: *mut bpf_insn, _prog: *const bpf_prog, _ctx_stack_off: s16) -> i32 { /* emits BPF instruction sequence ending in BPF_EXIT */ 15 }
const KFUNC_PRO_EPI_PREFIX: &[u8] = b"test_kfunc_\0";
unsafe extern "C" fn st_ops_gen_prologue(insn_buf: *mut bpf_insn, direct_write: bool, prog: *const bpf_prog) -> i32 { unsafe { if strcmp((*(*prog).aux).attach_func_name, b"test_prologue\0".as_ptr() as *const c_char) != 0 && strcmp((*(*prog).aux).attach_func_name, b"test_pro_epilogue\0".as_ptr() as *const c_char) != 0 { return 0; } if strncmp((*(*prog).aux).name.as_ptr(), KFUNC_PRO_EPI_PREFIX.as_ptr() as *const c_char, KFUNC_PRO_EPI_PREFIX.len() - 1) == 0 { return st_ops_gen_prologue_with_kfunc(insn_buf, direct_write, prog); } 5 } }
unsafe extern "C" fn st_ops_gen_epilogue(insn_buf: *mut bpf_insn, prog: *const bpf_prog, ctx_stack_off: s16) -> i32 { unsafe { if strcmp((*(*prog).aux).attach_func_name, b"test_epilogue\0".as_ptr() as *const c_char) != 0 && strcmp((*(*prog).aux).attach_func_name, b"test_pro_epilogue\0".as_ptr() as *const c_char) != 0 { return 0; } if strncmp((*(*prog).aux).name.as_ptr(), KFUNC_PRO_EPI_PREFIX.as_ptr() as *const c_char, KFUNC_PRO_EPI_PREFIX.len() - 1) == 0 { return st_ops_gen_epilogue_with_kfunc(insn_buf, prog, ctx_stack_off); } 8 } }
unsafe extern "C" fn st_ops_btf_struct_access(_log: *mut bpf_verifier_log, _reg: *const bpf_reg_state, off: i32, size: i32) -> i32 { if off < 0 || (off + size) as usize > core::mem::size_of::<st_ops_args>() { return -EACCES; } 0 }
static st_ops_verifier_ops: bpf_verifier_ops = bpf_verifier_ops { get_func_proto: unsafe { &bpf_base_func_proto }, is_valid_access: bpf_testmod_ops_is_valid_access as *const c_void, btf_struct_access: st_ops_btf_struct_access as *const c_void, gen_prologue: st_ops_gen_prologue as *const c_void, gen_epilogue: st_ops_gen_epilogue as *const c_void };
static mut st_ops_cfi_stubs: bpf_testmod_st_ops = bpf_testmod_st_ops { test_prologue: Some(bpf_test_mod_st_ops__test_prologue), test_epilogue: Some(bpf_test_mod_st_ops__test_epilogue), test_pro_epilogue: Some(bpf_test_mod_st_ops__test_pro_epilogue) };
unsafe extern "C" fn st_ops_reg(kdata: *mut c_void, _link: *mut bpf_link) -> i32 { unsafe { let mut err = 0; mutex_lock(&raw mut st_ops_mutex); if !st_ops.is_null() { pr_err(b"st_ops has already been registered\n\0".as_ptr() as *const c_char); err = -EEXIST; } else { st_ops = kdata as *mut bpf_testmod_st_ops; } mutex_unlock(&raw mut st_ops_mutex); err } }
unsafe extern "C" fn st_ops_unreg(_kdata: *mut c_void, _link: *mut bpf_link) { unsafe { mutex_lock(&raw mut st_ops_mutex); st_ops = core::ptr::null_mut(); mutex_unlock(&raw mut st_ops_mutex); } }
unsafe extern "C" fn st_ops_init(_btf: *mut btf) -> i32 { unsafe { let mut kfunc_btf: *mut btf = core::ptr::null_mut(); bpf_cgroup_from_id_id = bpf_find_btf_id(b"bpf_cgroup_from_id\0".as_ptr() as *const c_char, 12, &mut kfunc_btf); bpf_cgroup_release_id = bpf_find_btf_id(b"bpf_cgroup_release\0".as_ptr() as *const c_char, 12, &mut kfunc_btf); if bpf_cgroup_from_id_id < 0 || bpf_cgroup_release_id < 0 { return -EINVAL; } 0 } }
unsafe extern "C" fn st_ops_init_member(_t: *const btf_type, _member: *const btf_member, _kdata: *mut c_void, _udata: *const c_void) -> i32 { 0 }
static mut testmod_st_ops: bpf_struct_ops = bpf_struct_ops { verifier_ops: &st_ops_verifier_ops, init: st_ops_init as *const c_void, init_member: st_ops_init_member as *const c_void, reg: st_ops_reg as *const c_void, unreg: st_ops_unreg as *const c_void, check_member: core::ptr::null(), cfi_stubs: &raw const st_ops_cfi_stubs as *const c_void, name: b"bpf_testmod_st_ops\0".as_ptr() as *const c_char, owner: core::ptr::null_mut() };

unsafe extern "C" fn multi_st_ops_init(_btf: *mut btf) -> i32 { unsafe { spin_lock_init(&raw mut multi_st_ops_lock); 0 } }
unsafe extern "C" fn multi_st_ops_init_member(_t: *const btf_type, _member: *const btf_member, _kdata: *mut c_void, _udata: *const c_void) -> i32 { 0 }
unsafe extern "C" fn multi_st_ops_find_nolock(_id: u32) -> *mut bpf_testmod_multi_st_ops { /* hlist_for_each_entry over multi_st_ops_list */ core::ptr::null_mut() }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_multi_st_ops_test_1(args: *mut st_ops_args, id: u32) -> i32 { unsafe { let mut flags = 0u64; let mut ret = -1; spin_lock_irqsave(&raw mut multi_st_ops_lock, &mut flags); let ops = multi_st_ops_find_nolock(id); if !ops.is_null() { ret = ((*ops).test_1.unwrap())(args); } spin_unlock_irqrestore(&raw mut multi_st_ops_lock, flags); ret } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_multi_st_ops_test_1_assoc(args: *mut st_ops_args, aux: *mut bpf_prog_aux) -> i32 { unsafe { let ops = bpf_prog_get_assoc_struct_ops(aux) as *mut bpf_testmod_multi_st_ops; if !ops.is_null() { ((*ops).test_1.unwrap())(args) } else { -1 } } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_implicit_arg(a: i32, aux: *mut bpf_prog_aux) -> i32 { if !aux.is_null() && a > 0 { a } else { -EINVAL } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_implicit_arg_legacy(a: i32, b: i32, aux: *mut bpf_prog_aux) -> i32 { if !aux.is_null() { a + b } else { -EINVAL } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn bpf_kfunc_implicit_arg_legacy_impl(a: i32, b: i32, aux: *mut bpf_prog_aux) -> i32 { unsafe { bpf_kfunc_implicit_arg_legacy(a, b, aux) } }
unsafe extern "C" fn multi_st_ops_reg(kdata: *mut c_void, _link: *mut bpf_link) -> i32 { unsafe { let ops = kdata as *mut bpf_testmod_multi_st_ops; let mut flags = 0u64; let mut err = 0; if (*ops).test_1.is_none() { return -EINVAL; } let id = bpf_struct_ops_id(kdata); spin_lock_irqsave(&raw mut multi_st_ops_lock, &mut flags); if !multi_st_ops_find_nolock(id).is_null() { pr_err(b"multi_st_ops(id:%d) has already been registered\n\0".as_ptr() as *const c_char, id); err = -EEXIST; } else { (*ops).id = id; hlist_add_head(&mut (*ops).node, &raw mut multi_st_ops_list); } spin_unlock_irqrestore(&raw mut multi_st_ops_lock, flags); err } }
unsafe extern "C" fn multi_st_ops_unreg(kdata: *mut c_void, _link: *mut bpf_link) { unsafe { let mut flags = 0u64; let id = bpf_struct_ops_id(kdata); spin_lock_irqsave(&raw mut multi_st_ops_lock, &mut flags); let ops = multi_st_ops_find_nolock(id); if !ops.is_null() { hlist_del(&mut (*ops).node); } spin_unlock_irqrestore(&raw mut multi_st_ops_lock, flags); } }
unsafe extern "C" fn bpf_testmod_multi_st_ops__test_1(_args: *mut st_ops_args) -> i32 { 0 }
static mut multi_st_ops_cfi_stubs: bpf_testmod_multi_st_ops = bpf_testmod_multi_st_ops { test_1: Some(bpf_testmod_multi_st_ops__test_1), id: 0, node: hlist_node { _private: [] } };
#[unsafe(no_mangle)] pub static mut testmod_multi_st_ops: bpf_struct_ops = bpf_struct_ops { verifier_ops: &bpf_testmod_verifier_ops, init: multi_st_ops_init as *const c_void, init_member: multi_st_ops_init_member as *const c_void, reg: multi_st_ops_reg as *const c_void, unreg: multi_st_ops_unreg as *const c_void, check_member: core::ptr::null(), cfi_stubs: &raw const multi_st_ops_cfi_stubs as *const c_void, name: b"bpf_testmod_multi_st_ops\0".as_ptr() as *const c_char, owner: core::ptr::null_mut() };

unsafe extern "C" fn bpf_testmod_init() -> i32 { unsafe { let bpf_testmod_dtors = [btf_id_dtor_kfunc { btf_id: bpf_testmod_dtor_ids[0], kfunc_btf_id: bpf_testmod_dtor_ids[1] }]; let mut ret = register_btf_kfunc_id_set(BPF_PROG_TYPE_UNSPEC, &bpf_testmod_common_kfunc_set); if ret == 0 { ret = register_btf_kfunc_id_set(BPF_PROG_TYPE_SCHED_CLS, &bpf_testmod_kfunc_set); } if ret == 0 { ret = register_btf_kfunc_id_set(BPF_PROG_TYPE_TRACING, &bpf_testmod_kfunc_set); } if ret == 0 { ret = register_btf_kfunc_id_set(BPF_PROG_TYPE_SYSCALL, &bpf_testmod_kfunc_set); } if ret == 0 { ret = register_btf_kfunc_id_set(BPF_PROG_TYPE_STRUCT_OPS, &bpf_testmod_kfunc_set); } if ret == 0 { ret = register_btf_fmodret_id_set(&bpf_testmod_trampoline_count_fmodret_set); } if ret == 0 { ret = register_bpf_struct_ops(&raw mut bpf_bpf_testmod_ops, core::ptr::null_mut()); } if ret == 0 { ret = register_bpf_struct_ops(&raw mut bpf_testmod_ops2, core::ptr::null_mut()); } if ret == 0 { ret = register_bpf_struct_ops(&raw mut bpf_testmod_ops3, core::ptr::null_mut()); } if ret == 0 { ret = register_bpf_struct_ops(&raw mut testmod_st_ops, core::ptr::null_mut()); } if ret == 0 { ret = register_bpf_struct_ops(&raw mut testmod_multi_st_ops, core::ptr::null_mut()); } if ret == 0 { ret = register_btf_id_dtor_kfuncs(bpf_testmod_dtors.as_ptr(), bpf_testmod_dtors.len(), THIS_MODULE); } if ret < 0 { return ret; } if bpf_fentry_test1(0) < 0 { return -EINVAL; } sock = core::ptr::null_mut(); mutex_init(&raw mut sock_lock); ret = sysfs_create_bin_file(kernel_kobj, &raw mut bin_attr_bpf_testmod_file); if ret < 0 { return ret; } ret = register_bpf_testmod_uprobe(); if ret < 0 { return ret; } let mut tramp = &raw mut __bpf_testmod_ops.tramp_1 as *mut *mut c_void; let end = &raw mut __bpf_testmod_ops.tramp_40 as *mut *mut c_void; while tramp <= end { *tramp = bpf_testmod_tramp as *mut c_void; tramp = tramp.add(1); } 0 } }
unsafe extern "C" fn bpf_testmod_exit() { unsafe { while refcount_read(&prog_test_struct.cnt) > 1 { msleep(20); } irq_work_sync(&raw mut ctx_check_irq); tasklet_kill(&raw mut ctx_check_tasklet); bpf_kfunc_close_sock(); sysfs_remove_bin_file(kernel_kobj, &raw mut bin_attr_bpf_testmod_file); unregister_bpf_testmod_uprobe(); } }

// module_init(bpf_testmod_init);
// module_exit(bpf_testmod_exit);
// MODULE_AUTHOR("Andrii Nakryiko");
// MODULE_DESCRIPTION("BPF selftests module");
// MODULE_LICENSE("Dual BSD/GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
