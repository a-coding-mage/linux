// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */

// Dependencies from the original C includes:
// <vmlinux.h>, <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>,
// <bpf/bpf_core_read.h>, "bpf_misc.h", "xdp_metadata.h", "bpf_kfuncs.h".

extern "C" {
    fn bpf_task_acquire(p: *mut task_struct) -> *mut task_struct;
    fn bpf_task_release(p: *mut task_struct);
    fn bpf_get_current_task_btf() -> *mut task_struct;
    fn bpf_copy_from_user_task(
        dst: *mut ::core::ffi::c_void,
        size: usize,
        user_ptr: *const ::core::ffi::c_void,
        task: *mut ::core::ffi::c_void,
        flags: u64,
    ) -> ::core::ffi::c_int;
    fn bpf_rdonly_cast(p: u64, btf_id: u64) -> *mut ::core::ffi::c_void;
    fn bpf_core_cast_task_struct(p: u64) -> *mut task_struct;
    fn bpf_core_cast_bpf_verifier_env(p: u64) -> *mut bpf_verifier_env;
}

#[repr(C)]
pub struct task_struct {
    pub pid: ::core::ffi::c_int,
    pub tgid: ::core::ffi::c_int,
    pub prio: ::core::ffi::c_int,
}

#[repr(C)]
pub struct bpf_verifier_env {
    _unused: [u8; 0],
}

#[repr(C)]
pub enum bpf_attach_type {
    __Incomplete = 0,
}

// __weak; argument tags: task __arg_trusted __arg_nullable
pub unsafe extern "C" fn subprog_trusted_task_nullable(
    task: *mut task_struct,
) -> ::core::ffi::c_int {
    if task.is_null() {
        return 0;
    }
    (*task).pid + (*task).tgid
}

// __weak; argument tags: task __arg_trusted __arg_nullable
pub unsafe extern "C" fn subprog_trusted_task_nullable_extra_layer(
    task: *mut task_struct,
) -> ::core::ffi::c_int {
    subprog_trusted_task_nullable(task) + subprog_trusted_task_nullable(::core::ptr::null_mut())
}

// SEC("?tp_btf/task_newtask")
// __success __log_level(2)
// __msg("Validating subprog_trusted_task_nullable() func#1...")
// __msg(": R1=trusted_ptr_or_null_task_struct(")
pub unsafe extern "C" fn trusted_task_arg_nullable(
    ctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let t1: *mut task_struct = bpf_get_current_task_btf();
    let t2: *mut task_struct = bpf_task_acquire(t1);
    let mut res: ::core::ffi::c_int = 0;

    /* known NULL */
    res += subprog_trusted_task_nullable(::core::ptr::null_mut());

    /* known non-NULL */
    res += subprog_trusted_task_nullable(t1);
    res += subprog_trusted_task_nullable_extra_layer(t1);

    /* unknown if NULL or not */
    res += subprog_trusted_task_nullable(t2);
    res += subprog_trusted_task_nullable_extra_layer(t2);

    if !t2.is_null() {
        /* known non-NULL after explicit NULL check, just in case */
        res += subprog_trusted_task_nullable(t2);
        res += subprog_trusted_task_nullable_extra_layer(t2);

        bpf_task_release(t2);
    }

    let _ = ctx;
    res
}

// __weak; argument tags: task __arg_trusted
pub unsafe extern "C" fn subprog_trusted_task_nonnull(
    task: *mut task_struct,
) -> ::core::ffi::c_int {
    (*task).pid + (*task).tgid
}

// SEC("?kprobe")
// __failure __log_level(2)
// __msg("R1 type=scalar expected=ptr_, trusted_ptr_, rcu_ptr_")
// __msg("Caller passes invalid args into func#1 ('subprog_trusted_task_nonnull')")
pub unsafe extern "C" fn trusted_task_arg_nonnull_fail1(
    ctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let _ = ctx;
    subprog_trusted_task_nonnull(::core::ptr::null_mut())
}

// SEC("?tp_btf/task_newtask")
// __failure __log_level(2)
// __msg("R1 type=trusted_ptr_or_null_ expected=ptr_, trusted_ptr_, rcu_ptr_")
// __msg("Caller passes invalid args into func#1 ('subprog_trusted_task_nonnull')")
pub unsafe extern "C" fn trusted_task_arg_nonnull_fail2(
    ctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let t: *mut task_struct = bpf_get_current_task_btf();
    let nullable: *mut task_struct;
    let res: ::core::ffi::c_int;

    nullable = bpf_task_acquire(t);

    /* should fail, PTR_TO_BTF_ID_OR_NULL */
    res = subprog_trusted_task_nonnull(nullable);

    if !nullable.is_null() {
        bpf_task_release(nullable);
    }

    let _ = ctx;
    res
}

// SEC("?kprobe")
// __success __log_level(2)
// __msg("Validating subprog_trusted_task_nonnull() func#1...")
// __msg(": R1=trusted_ptr_task_struct(")
pub unsafe extern "C" fn trusted_task_arg_nonnull(
    ctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let t: *mut task_struct = bpf_get_current_task_btf();

    let _ = ctx;
    subprog_trusted_task_nonnull(t)
}

#[repr(C)]
pub struct task_struct___local {
    _unused: [u8; 0],
}

// Original C used __attribute__((preserve_access_index)).
// __weak; argument tags: task __arg_trusted __arg_nullable
pub unsafe extern "C" fn subprog_nullable_task_flavor(
    task: *mut task_struct___local,
) -> ::core::ffi::c_int {
    let mut buf: [::core::ffi::c_char; 16] = [0; 16];

    if task.is_null() {
        return 0;
    }

    bpf_copy_from_user_task(
        buf.as_mut_ptr() as *mut ::core::ffi::c_void,
        ::core::mem::size_of_val(&buf),
        ::core::ptr::null(),
        task as *mut ::core::ffi::c_void,
        0,
    )
}

// SEC("?uprobe.s")
// __success __log_level(2)
// __msg("Validating subprog_nullable_task_flavor() func#1...")
// __msg(": R1=trusted_ptr_or_null_task_struct(")
pub unsafe extern "C" fn flavor_ptr_nullable(ctx: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let t: *mut task_struct___local = bpf_get_current_task_btf() as *mut ::core::ffi::c_void
        as *mut task_struct___local;

    let _ = ctx;
    subprog_nullable_task_flavor(t)
}

// __weak; argument tags: task __arg_trusted
pub unsafe extern "C" fn subprog_nonnull_task_flavor(
    task: *mut task_struct___local,
) -> ::core::ffi::c_int {
    let mut buf: [::core::ffi::c_char; 16] = [0; 16];

    bpf_copy_from_user_task(
        buf.as_mut_ptr() as *mut ::core::ffi::c_void,
        ::core::mem::size_of_val(&buf),
        ::core::ptr::null(),
        task as *mut ::core::ffi::c_void,
        0,
    )
}

// SEC("?uprobe.s")
// __success __log_level(2)
// __msg("Validating subprog_nonnull_task_flavor() func#1...")
// __msg(": R1=trusted_ptr_task_struct(")
pub unsafe extern "C" fn flavor_ptr_nonnull(ctx: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let t: *mut task_struct = bpf_get_current_task_btf();

    let _ = ctx;
    subprog_nonnull_task_flavor(t as *mut ::core::ffi::c_void as *mut task_struct___local)
}

// __weak; argument tags: task __arg_trusted
pub unsafe extern "C" fn subprog_trusted_destroy(task: *mut task_struct) -> ::core::ffi::c_int {
    bpf_task_release(task); /* should be rejected */

    0
}

// SEC("?tp_btf/task_newtask")
// __failure __log_level(2)
// __msg("release kfunc bpf_task_release expects referenced PTR_TO_BTF_ID passed to R1")
// Original declaration used BPF_PROG(trusted_destroy_fail, struct task_struct *task, u64 clone_flags).
pub unsafe extern "C" fn trusted_destroy_fail(
    task: *mut task_struct,
    clone_flags: u64,
) -> ::core::ffi::c_int {
    let _ = clone_flags;
    subprog_trusted_destroy(task)
}

// __weak; argument tags: task __arg_trusted
pub unsafe extern "C" fn subprog_trusted_acq_rel(task: *mut task_struct) -> ::core::ffi::c_int {
    let owned: *mut task_struct;

    owned = bpf_task_acquire(task);
    if owned.is_null() {
        return 0;
    }

    bpf_task_release(owned); /* this one is OK, we acquired it locally */

    0
}

// SEC("?tp_btf/task_newtask")
// __success __log_level(2)
// Original declaration used BPF_PROG(trusted_acq_rel, struct task_struct *task, u64 clone_flags).
pub unsafe extern "C" fn trusted_acq_rel(
    task: *mut task_struct,
    clone_flags: u64,
) -> ::core::ffi::c_int {
    let _ = clone_flags;
    subprog_trusted_acq_rel(task)
}

// __weak; argument tags: task __arg_untrusted __arg_nullable
pub unsafe extern "C" fn subprog_untrusted_bad_tags(
    task: *mut task_struct,
) -> ::core::ffi::c_int {
    (*task).pid
}

// SEC("tp_btf/sys_enter")
// __failure
// __msg("arg#0 untrusted cannot be combined with any other tags")
pub unsafe extern "C" fn untrusted_bad_tags(
    ctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let _ = ctx;
    subprog_untrusted_bad_tags(::core::ptr::null_mut())
}

#[repr(C)]
pub struct local_type_wont_be_accepted {
    _unused: [u8; 0],
}

// __weak; argument tags: p __arg_untrusted
pub unsafe extern "C" fn subprog_untrusted_bad_type(
    p: *mut local_type_wont_be_accepted,
) -> ::core::ffi::c_int {
    let _ = p;
    0
}

// SEC("tp_btf/sys_enter")
// __failure
// __msg("arg#0 reference type('STRUCT local_type_wont_be_accepted') has no matches")
pub unsafe extern "C" fn untrusted_bad_type(
    ctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let _ = ctx;
    subprog_untrusted_bad_type(bpf_rdonly_cast(0, 0) as *mut local_type_wont_be_accepted)
}

// __weak; argument tags: task __arg_untrusted
pub unsafe extern "C" fn subprog_untrusted(
    task: *const task_struct,
) -> ::core::ffi::c_int {
    (*task).pid
}

// SEC("tp_btf/sys_enter")
// __success
// __log_level(2)
// __msg("r1 = {{.*}}; {{.*}}R1=trusted_ptr_task_struct()")
// __msg("Func#1 ('subprog_untrusted') is global and assumed valid.")
// __msg("Validating subprog_untrusted() func#1...")
// __msg(": R1=untrusted_ptr_task_struct")
pub unsafe extern "C" fn trusted_to_untrusted(
    ctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let _ = ctx;
    subprog_untrusted(bpf_get_current_task_btf())
}

#[no_mangle]
pub static mut mem: [::core::ffi::c_char; 16] = [0; 16];
#[no_mangle]
pub static mut offset: u32 = 0;

// SEC("tp_btf/sys_enter")
// __success
pub unsafe extern "C" fn anything_to_untrusted(
    ctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    /* untrusted to untrusted */
    subprog_untrusted(bpf_core_cast_task_struct(0));
    /* wrong type to untrusted */
    subprog_untrusted(bpf_core_cast_bpf_verifier_env(0) as *mut ::core::ffi::c_void as *const task_struct);
    /* map value to untrusted */
    subprog_untrusted(mem.as_mut_ptr() as *mut ::core::ffi::c_void as *const task_struct);
    /* scalar to untrusted */
    subprog_untrusted(::core::ptr::null());
    /* variable offset to untrusted (map) */
    subprog_untrusted((mem.as_mut_ptr() as *mut ::core::ffi::c_void).add(offset as usize) as *const task_struct);
    /* variable offset to untrusted (trusted) */
    subprog_untrusted((bpf_get_current_task_btf() as *mut ::core::ffi::c_void).add(offset as usize) as *const task_struct);
    let _ = ctx;
    0
}

// __weak; argument tags: task __arg_untrusted
pub unsafe extern "C" fn subprog_untrusted2(task: *mut task_struct) -> ::core::ffi::c_int {
    subprog_trusted_task_nullable(task)
}

// SEC("tp_btf/sys_enter")
// __failure
// __msg("R1 type=untrusted_ptr_ expected=ptr_, trusted_ptr_, rcu_ptr_")
// __msg("Caller passes invalid args into func#{{.*}} ('subprog_trusted_task_nullable')")
pub unsafe extern "C" fn untrusted_to_trusted(
    ctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let _ = ctx;
    subprog_untrusted2(bpf_get_current_task_btf())
}

// __weak; argument tags: p __arg_untrusted
pub unsafe extern "C" fn subprog_void_untrusted(
    p: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    *(p as *mut ::core::ffi::c_int)
}

// __weak; argument tags: p __arg_untrusted
pub unsafe extern "C" fn subprog_char_untrusted(
    p: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    *(p as *mut ::core::ffi::c_int)
}

// __weak; argument tags: p __arg_untrusted
pub unsafe extern "C" fn subprog_enum_untrusted(
    p: *mut bpf_attach_type,
) -> ::core::ffi::c_int {
    *(p as *mut ::core::ffi::c_int)
}

// SEC("tp_btf/sys_enter")
// __success
// __log_level(2)
// __msg("r1 = {{.*}}; {{.*}}R1=trusted_ptr_task_struct()")
// __msg("Func#1 ('subprog_void_untrusted') is global and assumed valid.")
// __msg("Validating subprog_void_untrusted() func#1...")
// __msg(": R1=rdonly_untrusted_mem(sz=0)")
pub unsafe extern "C" fn trusted_to_untrusted_mem(
    ctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let _ = ctx;
    subprog_void_untrusted(bpf_get_current_task_btf() as *mut ::core::ffi::c_void)
}

// __weak
pub unsafe extern "C" fn subprog_write_mem_arg(
    p: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if p.is_null() {
        return 0;
    }

    *p = 42;
    0
}

// SEC("?tp_btf/task_newtask")
// __failure
// __msg("only read is supported")
pub unsafe extern "C" fn trusted_btf_field_to_writable_mem(
    ctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let task: *mut task_struct = bpf_get_current_task_btf();

    let _ = ctx;
    subprog_write_mem_arg(::core::ptr::addr_of_mut!((*task).prio))
}

// SEC("tp_btf/sys_enter")
// __success
pub unsafe extern "C" fn anything_to_untrusted_mem(
    ctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    /* untrusted to untrusted mem */
    subprog_void_untrusted(bpf_core_cast_task_struct(0) as *mut ::core::ffi::c_void);
    /* map value to untrusted mem */
    subprog_void_untrusted(mem.as_mut_ptr() as *mut ::core::ffi::c_void);
    /* scalar to untrusted mem */
    subprog_void_untrusted(::core::ptr::null_mut());
    /* variable offset to untrusted mem (map) */
    subprog_void_untrusted((mem.as_mut_ptr() as *mut ::core::ffi::c_void).add(offset as usize));
    /* variable offset to untrusted mem (trusted) */
    subprog_void_untrusted((bpf_get_current_task_btf() as *mut ::core::ffi::c_void).add(offset as usize));
    /* variable offset to untrusted char/enum (map) */
    subprog_char_untrusted(mem.as_mut_ptr().add(offset as usize));
    subprog_enum_untrusted(
        (mem.as_mut_ptr() as *mut ::core::ffi::c_void).add(offset as usize) as *mut bpf_attach_type,
    );
    let _ = ctx;
    0
}

// char _license[] SEC("license") = "GPL";
#[no_mangle]
pub static mut _license: [::core::ffi::c_char; 4] = [b'G' as ::core::ffi::c_char, b'P' as ::core::ffi::c_char, b'L' as ::core::ffi::c_char, 0];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
