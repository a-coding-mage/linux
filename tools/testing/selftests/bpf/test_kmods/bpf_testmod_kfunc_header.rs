/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_int, c_long, c_schar, c_short, c_uchar, c_ulong, c_ushort, c_void};

pub type u32 = u32;
pub type __u8 = c_uchar;
pub type __u16 = c_ushort;
pub type __u32 = u32;
pub type __u64 = u64;

/* Original C header includes vmlinux.h and bpf/bpf_helpers.h outside __KERNEL__.
 * The following are dependency types supplied by those headers or kernel headers.
 */
#[repr(C)]
pub struct refcount_t {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct callback_head {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct __kernel_sockaddr_storage {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct sock {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct bpf_dynptr {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct bpf_iter_testmod_seq {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct bpf_timer {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct __sk_buff {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct sk_buff_head {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct sock_common {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct st_ops_args {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct vm_area_struct {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    _unused: [u8; 0],
}

/* Under __KERNEL__, the C header defines __ksym as empty and defines these
 * structs locally. Outside __KERNEL__, they are expected from vmlinux.h.
 */
#[repr(C)]
pub struct prog_test_member1 {
    pub a: c_int,
}

#[repr(C)]
pub struct prog_test_member {
    pub m: prog_test_member1,
    pub c: c_int,
}

#[repr(C)]
pub struct prog_test_ref_kfunc {
    pub a: c_int,
    pub b: c_int,
    pub memb: prog_test_member,
    pub next: *mut prog_test_ref_kfunc,
    pub cnt: refcount_t,
}

#[repr(C)]
pub struct prog_test_pass1_anon3 {
    pub x3: c_int,
}

#[repr(C)]
pub struct prog_test_pass1_anon2 {
    pub x2: c_int,
    pub anon: prog_test_pass1_anon3,
}

#[repr(C)]
pub struct prog_test_pass1_anon1 {
    pub x1: c_int,
    pub anon: prog_test_pass1_anon2,
}

#[repr(C)]
pub struct prog_test_pass1 {
    pub x0: c_int,
    pub anon: prog_test_pass1_anon1,
}

#[repr(C)]
pub struct prog_test_pass2_anon1 {
    pub arr2: [c_char; 4],
    pub arr3: [c_ulong; 8],
}

#[repr(C)]
pub struct prog_test_pass2 {
    pub len: c_int,
    pub arr1: [c_short; 4],
    pub x: prog_test_pass2_anon1,
}

#[repr(C)]
pub struct prog_test_big_arg {
    pub a: __u64,
    pub b: __u64,
}

#[repr(C)]
pub struct prog_test_fail1 {
    pub p: *mut c_void,
    pub x: c_int,
}

#[repr(C)]
pub struct prog_test_fail2 {
    pub x8: c_int,
    pub x: prog_test_pass1,
}

#[repr(C)]
pub struct prog_test_fail3 {
    pub len: c_int,
    pub arr1: [c_char; 2],
    pub arr2: [c_char; 0],
}

#[repr(C)]
pub struct init_sock_args {
    pub af: c_int,
    pub type_: c_int,
}

#[repr(C)]
pub struct addr_args {
    pub addr: [c_char; core::mem::size_of::<__kernel_sockaddr_storage>()],
    pub addrlen: c_int,
}

#[repr(C)]
pub struct sendmsg_args {
    pub addr: addr_args,
    pub msg: [c_char; 10],
    pub msglen: c_int,
}

#[repr(C)]
pub struct bpf_testmod_ctx {
    pub rcu: callback_head,
    pub usage: refcount_t,
}

unsafe extern "C" {
    pub fn bpf_kfunc_call_test_acquire(scalar_ptr: *mut c_ulong) -> *mut prog_test_ref_kfunc;
    pub fn bpf_kfunc_call_test_release(p: *mut prog_test_ref_kfunc);
    pub fn bpf_kfunc_call_test_ref(p: *mut prog_test_ref_kfunc);

    pub fn bpf_kfunc_call_test_mem_len_pass1(mem: *mut c_void, len: c_int);
    pub fn bpf_kfunc_arena_arg_test(val__arena: *mut __u64) -> __u64;
    pub fn bpf_kfunc_arena_cap_test(val__arena: *mut __u64) -> __u64;
    pub fn bpf_kfunc_arena_cap_nullable_test(val__arena__nullable: *mut __u64) -> __u64;
    pub fn bpf_kfunc_arena_args5_test(
        a__arena: *mut __u64,
        b__arena: *mut __u64,
        c__arena: *mut __u64,
        d__arena: *mut __u64,
        e__arena__nullable: *mut __u64,
    ) -> __u64;
    pub fn bpf_kfunc_arena_stack_arg_test(
        a: __u64,
        b: __u64,
        c: __u64,
        d: __u64,
        e: __u64,
        f__arena: *mut __u64,
    ) -> __u64;
    pub fn bpf_kfunc_arena_mixed_test(
        a__arena: *mut __u64,
        b__arena__nullable: *mut __u64,
    ) -> __u64;
    pub fn bpf_kfunc_call_test_get_rdwr_mem(
        p: *mut prog_test_ref_kfunc,
        rdwr_buf_size: c_int,
    ) -> *mut c_int;
    pub fn bpf_kfunc_call_test_get_rdonly_mem(
        p: *mut prog_test_ref_kfunc,
        rdonly_buf_size: c_int,
    ) -> *mut c_int;
    pub fn bpf_kfunc_call_test_acq_rdonly_mem(
        p: *mut prog_test_ref_kfunc,
        rdonly_buf_size: c_int,
    ) -> *mut c_int;
    pub fn bpf_kfunc_call_int_mem_release(p: *mut c_int);

    /* The bpf_kfunc_call_test_static_unused_arg is defined as static,
     * but bpf program compilation needs to see it as global symbol.
     * Original declaration is outside __KERNEL__ only.
     */
    pub fn bpf_kfunc_call_test_static_unused_arg(arg: u32, unused: u32) -> u32;

    pub fn bpf_testmod_test_mod_kfunc(i: c_int);
    pub fn bpf_testmod_ops3_call_test_arena(ptr__arena: *mut __u64) -> c_int;
    pub fn bpf_testmod_ops3_call_test_arena_nullable(ptr__arena__nullable: *mut __u64) -> c_int;
    pub fn bpf_testmod_ops3_call_test_arena_stack(ptr__arena: *mut __u64) -> c_int;
    pub fn bpf_testmod_ops3_call_test_arena_multislot(ptr__arena: *mut __u64) -> c_int;

    pub fn bpf_kfunc_call_test1(
        sk: *mut sock,
        a: __u32,
        b: __u64,
        c: __u32,
        d: __u64,
    ) -> __u64;
    pub fn bpf_kfunc_call_test2(sk: *mut sock, a: __u32, b: __u32) -> c_int;
    pub fn bpf_kfunc_call_test3(sk: *mut sock) -> *mut sock;
    pub fn bpf_kfunc_call_test4(a: c_schar, b: c_short, c: c_int, d: c_long) -> c_long;
    pub fn bpf_kfunc_call_test5(a: __u8, b: __u16, c: __u32) -> c_int;
    pub fn bpf_kfunc_call_stack_arg(
        a: __u64,
        b: __u64,
        c: __u64,
        d: __u64,
        e: __u64,
        f: __u64,
        g: __u64,
        h: __u64,
        i: __u64,
        j: __u64,
    ) -> __u64;
    pub fn bpf_kfunc_call_stack_arg_ptr(
        a: __u64,
        b: __u64,
        c: __u64,
        d: __u64,
        e: __u64,
        f: __u64,
        g: __u64,
        h: __u64,
        i: __u64,
        p: *mut prog_test_pass1,
    ) -> __u64;
    pub fn bpf_kfunc_call_stack_arg_mix(
        a: __u64,
        b: __u64,
        c: __u64,
        d: __u64,
        e: __u64,
        f: __u64,
        g: __u64,
        p: *mut prog_test_pass1,
        h: __u64,
        q: *mut prog_test_pass1,
    ) -> __u64;
    pub fn bpf_kfunc_call_stack_arg_dynptr(
        a: __u64,
        b: __u64,
        c: __u64,
        d: __u64,
        e: __u64,
        f: __u64,
        g: __u64,
        h: __u64,
        i: __u64,
        ptr: *mut bpf_dynptr,
    ) -> __u64;
    pub fn bpf_kfunc_call_stack_arg_mem(
        a: __u64,
        b: __u64,
        c: __u64,
        d: __u64,
        e: __u64,
        mem: *mut c_void,
        mem__sz: c_int,
    ) -> __u64;
    pub fn bpf_kfunc_call_stack_arg_iter(
        a: __u64,
        b: __u64,
        c: __u64,
        d: __u64,
        e: __u64,
        f: __u64,
        g: __u64,
        h: __u64,
        i: __u64,
        it__iter: *mut bpf_iter_testmod_seq,
    ) -> __u64;
    pub fn bpf_kfunc_call_stack_arg_const_str(
        a: __u64,
        b: __u64,
        c: __u64,
        d: __u64,
        e: __u64,
        f: __u64,
        g: __u64,
        h: __u64,
        i: __u64,
        str__str: *const c_char,
    ) -> __u64;
    pub fn bpf_kfunc_call_stack_arg_timer(
        a: __u64,
        b: __u64,
        c: __u64,
        d: __u64,
        e: __u64,
        f: __u64,
        g: __u64,
        h: __u64,
        i: __u64,
        timer: *mut bpf_timer,
    ) -> __u64;
    pub fn bpf_kfunc_call_stack_arg_big(
        a: __u64,
        b: __u64,
        c: __u64,
        d: __u64,
        e: __u64,
        s: prog_test_big_arg,
    ) -> __u64;

    pub fn bpf_kfunc_call_test_pass_ctx(skb: *mut __sk_buff);
    pub fn bpf_kfunc_call_test_pass1(p: *mut prog_test_pass1);
    pub fn bpf_kfunc_call_test_pass2(p: *mut prog_test_pass2);
    pub fn bpf_kfunc_call_test_mem_len_fail2(mem: *mut __u64, len: c_int);

    pub fn bpf_kfunc_call_test_destructive();
    pub fn bpf_kfunc_call_test_sleepable();
    pub fn bpf_kfunc_call_test_call_rcu_tasks_trace(done: *mut c_int) -> c_int;

    pub fn bpf_kfunc_call_test_offset(p: *mut prog_test_ref_kfunc);
    pub fn bpf_kfunc_call_memb_acquire() -> *mut prog_test_member;
    pub fn bpf_kfunc_call_memb1_release(p: *mut prog_test_member1);
    pub fn bpf_kfunc_call_test_fail1(p: *mut prog_test_fail1);
    pub fn bpf_kfunc_call_test_fail2(p: *mut prog_test_fail2);
    pub fn bpf_kfunc_call_test_fail3(p: *mut prog_test_fail3);
    pub fn bpf_kfunc_call_test_mem_len_fail1(mem: *mut c_void, len: c_int);

    pub fn bpf_kfunc_common_test();

    pub fn bpf_kfunc_init_sock(args: *mut init_sock_args) -> c_int;
    pub fn bpf_kfunc_close_sock();
    pub fn bpf_kfunc_call_kernel_connect(args: *mut addr_args) -> c_int;
    pub fn bpf_kfunc_call_kernel_bind(args: *mut addr_args) -> c_int;
    pub fn bpf_kfunc_call_kernel_listen() -> c_int;
    pub fn bpf_kfunc_call_kernel_sendmsg(args: *mut sendmsg_args) -> c_int;
    pub fn bpf_kfunc_call_sock_sendmsg(args: *mut sendmsg_args) -> c_int;
    pub fn bpf_kfunc_call_kernel_getsockname(args: *mut addr_args) -> c_int;
    pub fn bpf_kfunc_call_kernel_getpeername(args: *mut addr_args) -> c_int;

    pub fn bpf_kfunc_dynptr_test(ptr: *mut bpf_dynptr, ptr__nullable: *mut bpf_dynptr);

    pub fn bpf_testmod_ctx_create(err: *mut c_int) -> *mut bpf_testmod_ctx;
    pub fn bpf_testmod_ctx_release(ctx: *mut bpf_testmod_ctx);

    pub fn bpf_kfunc_nested_acquire_nonzero_offset_test(ptr: *mut sk_buff_head) -> *mut sk_buff;
    pub fn bpf_kfunc_nested_acquire_zero_offset_test(ptr: *mut sock_common) -> *mut sk_buff;
    pub fn bpf_kfunc_nested_release_test(ptr: *mut sk_buff);

    pub fn bpf_kfunc_st_ops_test_prologue(args: *mut st_ops_args) -> c_int;
    pub fn bpf_kfunc_st_ops_test_epilogue(args: *mut st_ops_args) -> c_int;
    pub fn bpf_kfunc_st_ops_test_pro_epilogue(args: *mut st_ops_args) -> c_int;
    pub fn bpf_kfunc_st_ops_inc10(args: *mut st_ops_args) -> c_int;

    pub fn bpf_kfunc_trusted_vma_test(ptr: *mut vm_area_struct);
    pub fn bpf_kfunc_trusted_task_test(ptr: *mut task_struct);
    pub fn bpf_kfunc_trusted_num_test(ptr: *mut c_int);
    pub fn bpf_kfunc_rcu_task_test(ptr: *mut task_struct);
    pub fn bpf_kfunc_ret_rcu_test() -> *mut task_struct;
    pub fn bpf_kfunc_ret_rcu_test_nostruct(rdonly_buf_size: c_int) -> *mut c_int;

    /* Original declarations are outside __KERNEL__ only and marked __weak. */
    pub fn bpf_kfunc_multi_st_ops_test_1(args: *mut st_ops_args, id: u32) -> c_int;
    pub fn bpf_kfunc_multi_st_ops_test_1_assoc(args: *mut st_ops_args) -> c_int;

    pub fn bpf_kfunc_get_default_trusted_ptr_test() -> *mut prog_test_member;
    pub fn bpf_kfunc_put_default_trusted_ptr_test(trusted_ptr: *mut prog_test_member);

    pub fn bpf_testmod_test_hardirq_fn();
    pub fn bpf_testmod_test_softirq_fn();
    pub fn bpf_kfunc_trigger_ctx_check();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
