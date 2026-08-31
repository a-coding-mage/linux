// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */
// Depends on test_progs.h and syscall.skel.h.

#[repr(C)]
struct args {
    log_buf: __u64,
    log_size: __u32,
    max_entries: ::std::os::raw::c_int,
    map_fd: ::std::os::raw::c_int,
    prog_fd: ::std::os::raw::c_int,
    btf_fd: ::std::os::raw::c_int,
}

unsafe fn test_syscall_load_prog() {
    static mut verifier_log: [::std::os::raw::c_char; 8192] = [0; 8192];
    let mut ctx: args = args {
        log_buf: unsafe { verifier_log.as_mut_ptr() as usize as __u64 },
        log_size: ::std::mem::size_of_val(unsafe { &verifier_log }) as __u32,
        max_entries: 1024,
        map_fd: 0,
        prog_fd: 0,
        btf_fd: 0,
    };
    let mut tattr: bpf_test_run_opts = unsafe { ::std::mem::zeroed() };
    tattr.ctx_in = &mut ctx as *mut args as *mut _;
    tattr.ctx_size_in = ::std::mem::size_of::<args>() as _;
    let mut skel: *mut syscall = ::std::ptr::null_mut();
    let mut key: __u64 = 12;
    let mut value: __u64 = 0;
    let mut err: ::std::os::raw::c_int;
    let prog_fd: ::std::os::raw::c_int;

    skel = syscall__open_and_load();
    if !ASSERT_OK_PTR(skel, c"skel_load".as_ptr()) {
        goto_cleanup_load_prog(skel, &mut ctx);
        return;
    }

    prog_fd = bpf_program__fd((*skel).progs.load_prog);
    err = bpf_prog_test_run_opts(prog_fd, &mut tattr);
    ASSERT_EQ(err, 0, c"err".as_ptr());
    ASSERT_EQ(tattr.retval, 1, c"retval".as_ptr());
    ASSERT_GT(ctx.map_fd, 0, c"ctx.map_fd".as_ptr());
    ASSERT_GT(ctx.prog_fd, 0, c"ctx.prog_fd".as_ptr());
    ASSERT_OK(
        memcmp(
            verifier_log.as_ptr() as *const _,
            c"processed".as_ptr() as *const _,
            ::std::mem::size_of_val(c"processed") - 1,
        ),
        c"verifier_log".as_ptr(),
    );

    err = bpf_map_lookup_elem(
        ctx.map_fd,
        &mut key as *mut __u64 as *const _,
        &mut value as *mut __u64 as *mut _,
    );
    ASSERT_EQ(err, 0, c"map_lookup".as_ptr());
    ASSERT_EQ(value, 34, c"map lookup value".as_ptr());

    goto_cleanup_load_prog(skel, &mut ctx);
}

unsafe fn goto_cleanup_load_prog(skel: *mut syscall, ctx: *mut args) {
    syscall__destroy(skel);
    if (*ctx).prog_fd > 0 {
        close((*ctx).prog_fd);
    }
    if (*ctx).map_fd > 0 {
        close((*ctx).map_fd);
    }
    if (*ctx).btf_fd > 0 {
        close((*ctx).btf_fd);
    }
}

unsafe fn test_syscall_update_outer_map() {
    let mut opts: bpf_test_run_opts = unsafe { ::std::mem::zeroed() };
    let mut skel: *mut syscall;
    let err: ::std::os::raw::c_int;
    let prog_fd: ::std::os::raw::c_int;

    skel = syscall__open_and_load();
    if !ASSERT_OK_PTR(skel, c"skel_load".as_ptr()) {
        goto_cleanup_update_outer_map(skel);
        return;
    }

    prog_fd = bpf_program__fd((*skel).progs.update_outer_map);
    err = bpf_prog_test_run_opts(prog_fd, &mut opts);
    ASSERT_EQ(err, 0, c"err".as_ptr());
    ASSERT_EQ(opts.retval, 1, c"retval".as_ptr());

    goto_cleanup_update_outer_map(skel);
}

unsafe fn goto_cleanup_update_outer_map(skel: *mut syscall) {
    syscall__destroy(skel);
}

pub unsafe extern "C" fn test_syscall() {
    if test__start_subtest(c"load_prog".as_ptr()) {
        test_syscall_load_prog();
    }
    if test__start_subtest(c"update_outer_map".as_ptr()) {
        test_syscall_update_outer_map();
    }
}
