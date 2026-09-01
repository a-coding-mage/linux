// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Meta Platforms, Inc. and affiliates. */
// C dependencies: <test_progs.h>, "cgroup_helpers.h", "cgroup_preorder.skel.h"

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

type SocklenT = c_uint;
type __u8 = u8;

const SOL_IP: c_int = 0;
const IP_TOS: c_int = 1;
const BPF_F_ALLOW_MULTI: u32 = 1 << 1;
const BPF_F_PREORDER: u32 = 1 << 4;
const AF_INET: c_int = 2;
const SOCK_STREAM: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum bpf_attach_type {
    BPF_ATTACH_TYPE_UNSPEC = 0,
}

#[repr(C)]
pub struct bpf_prog_attach_opts {
    pub sz: usize,
    pub flags: u32,
}

#[repr(C)]
pub struct bpf_link_create_opts {
    pub sz: usize,
    pub flags: u32,
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cgroup_preorder_bss {
    pub result: [__u8; 4],
    pub idx: __u8,
}

#[repr(C)]
pub struct cgroup_preorder_progs {
    pub child: *mut bpf_program,
    pub child_2: *mut bpf_program,
    pub parent: *mut bpf_program,
    pub parent_2: *mut bpf_program,
}

#[repr(C)]
pub struct cgroup_preorder {
    pub progs: cgroup_preorder_progs,
    pub bss: *mut cgroup_preorder_bss,
}

unsafe extern "C" {
    fn cgroup_preorder__open_and_load() -> *mut cgroup_preorder;
    fn cgroup_preorder__destroy(skel: *mut cgroup_preorder);
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_program__expected_attach_type(prog: *mut bpf_program) -> bpf_attach_type;
    fn bpf_prog_attach_opts(
        prog_fd: c_int,
        target_fd: c_int,
        attach_type: bpf_attach_type,
        opts: *const bpf_prog_attach_opts,
    ) -> c_int;
    fn bpf_prog_detach2(prog_fd: c_int, target_fd: c_int, attach_type: bpf_attach_type)
        -> c_int;
    fn bpf_link_create(
        prog_fd: c_int,
        target_fd: c_int,
        attach_type: bpf_attach_type,
        opts: *const bpf_link_create_opts,
    ) -> c_int;
    fn bpf_link_update(link_fd: c_int, new_prog_fd: c_int, opts: *const c_void) -> c_int;
    fn test__join_cgroup(path: *const c_char) -> c_int;
    fn setsockopt(
        fd: c_int,
        level: c_int,
        optname: c_int,
        optval: *const c_void,
        optlen: SocklenT,
    ) -> c_int;
    fn getsockopt(
        fd: c_int,
        level: c_int,
        optname: c_int,
        optval: *mut c_void,
        optlen: *mut SocklenT,
    ) -> c_int;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_TRUE(condition: bool, name: *const c_char) -> bool;
    fn ASSERT_GE(a: c_int, b: c_int, name: *const c_char) -> bool;
}

unsafe fn run_getsockopt_test(
    cg_parent: c_int,
    cg_child: c_int,
    sock_fd: c_int,
    all_preorder: bool,
) -> c_int {
    let mut opts = bpf_prog_attach_opts {
        sz: core::mem::size_of::<bpf_prog_attach_opts>(),
        flags: 0,
    };
    let prog_c_atype: bpf_attach_type;
    let prog_c2_atype: bpf_attach_type;
    let prog_p_atype: bpf_attach_type;
    let prog_p2_atype: bpf_attach_type;
    let prog_c_fd: c_int;
    let prog_c2_fd: c_int;
    let prog_p_fd: c_int;
    let prog_p2_fd: c_int;
    let mut skel: *mut cgroup_preorder = ptr::null_mut();
    let mut prog: *mut bpf_program;
    let result: *mut __u8;
    let mut buf: __u8;
    let mut optlen: SocklenT;
    let mut err: c_int = 0;

    skel = cgroup_preorder__open_and_load();
    if !ASSERT_OK_PTR(skel.cast::<c_void>(), c"cgroup_preorder__open_and_load".as_ptr()) {
        return 0;
    }

    buf = 0x00;
    err = setsockopt(sock_fd, SOL_IP, IP_TOS, (&buf as *const __u8).cast::<c_void>(), 1);
    if !ASSERT_OK(err, c"setsockopt".as_ptr()) {
        cgroup_preorder__destroy(skel);
        return err;
    }

    opts.flags = BPF_F_ALLOW_MULTI;
    if all_preorder {
        opts.flags |= BPF_F_PREORDER;
    }
    prog = (*skel).progs.child;
    prog_c_fd = bpf_program__fd(prog);
    prog_c_atype = bpf_program__expected_attach_type(prog);
    err = bpf_prog_attach_opts(prog_c_fd, cg_child, prog_c_atype, &opts);
    if !ASSERT_OK(err, c"bpf_prog_attach_opts-child".as_ptr()) {
        cgroup_preorder__destroy(skel);
        return err;
    }

    opts.flags = BPF_F_ALLOW_MULTI | BPF_F_PREORDER;
    prog = (*skel).progs.child_2;
    prog_c2_fd = bpf_program__fd(prog);
    prog_c2_atype = bpf_program__expected_attach_type(prog);
    err = bpf_prog_attach_opts(prog_c2_fd, cg_child, prog_c2_atype, &opts);
    if !ASSERT_OK(err, c"bpf_prog_attach_opts-child_2".as_ptr()) {
        ASSERT_OK(
            bpf_prog_detach2(prog_c_fd, cg_child, prog_c_atype),
            c"bpf_prog_detach2-child".as_ptr(),
        );
        cgroup_preorder__destroy(skel);
        return err;
    }

    optlen = 1;
    err = getsockopt(
        sock_fd,
        SOL_IP,
        IP_TOS,
        (&mut buf as *mut __u8).cast::<c_void>(),
        &mut optlen,
    );
    if !ASSERT_OK(err, c"getsockopt".as_ptr()) {
        ASSERT_OK(
            bpf_prog_detach2(prog_c2_fd, cg_child, prog_c2_atype),
            c"bpf_prog_detach2-child_2".as_ptr(),
        );
        ASSERT_OK(
            bpf_prog_detach2(prog_c_fd, cg_child, prog_c_atype),
            c"bpf_prog_detach2-child".as_ptr(),
        );
        cgroup_preorder__destroy(skel);
        return err;
    }

    result = (*(*skel).bss).result.as_mut_ptr();
    if all_preorder {
        ASSERT_TRUE(*result.add(0) == 1 && *result.add(1) == 2, c"child only".as_ptr());
    } else {
        ASSERT_TRUE(*result.add(0) == 2 && *result.add(1) == 1, c"child only".as_ptr());
    }

    (*(*skel).bss).idx = 0;
    memset(result.cast::<c_void>(), 0, 4);

    opts.flags = BPF_F_ALLOW_MULTI;
    if all_preorder {
        opts.flags |= BPF_F_PREORDER;
    }
    prog = (*skel).progs.parent;
    prog_p_fd = bpf_program__fd(prog);
    prog_p_atype = bpf_program__expected_attach_type(prog);
    err = bpf_prog_attach_opts(prog_p_fd, cg_parent, prog_p_atype, &opts);
    if !ASSERT_OK(err, c"bpf_prog_attach_opts-parent".as_ptr()) {
        ASSERT_OK(
            bpf_prog_detach2(prog_c2_fd, cg_child, prog_c2_atype),
            c"bpf_prog_detach2-child_2".as_ptr(),
        );
        ASSERT_OK(
            bpf_prog_detach2(prog_c_fd, cg_child, prog_c_atype),
            c"bpf_prog_detach2-child".as_ptr(),
        );
        cgroup_preorder__destroy(skel);
        return err;
    }

    opts.flags = BPF_F_ALLOW_MULTI | BPF_F_PREORDER;
    prog = (*skel).progs.parent_2;
    prog_p2_fd = bpf_program__fd(prog);
    prog_p2_atype = bpf_program__expected_attach_type(prog);
    err = bpf_prog_attach_opts(prog_p2_fd, cg_parent, prog_p2_atype, &opts);
    if !ASSERT_OK(err, c"bpf_prog_attach_opts-parent_2".as_ptr()) {
        ASSERT_OK(
            bpf_prog_detach2(prog_p_fd, cg_parent, prog_p_atype),
            c"bpf_prog_detach2-parent".as_ptr(),
        );
        ASSERT_OK(
            bpf_prog_detach2(prog_c2_fd, cg_child, prog_c2_atype),
            c"bpf_prog_detach2-child_2".as_ptr(),
        );
        ASSERT_OK(
            bpf_prog_detach2(prog_c_fd, cg_child, prog_c_atype),
            c"bpf_prog_detach2-child".as_ptr(),
        );
        cgroup_preorder__destroy(skel);
        return err;
    }

    err = getsockopt(
        sock_fd,
        SOL_IP,
        IP_TOS,
        (&mut buf as *mut __u8).cast::<c_void>(),
        &mut optlen,
    );
    if ASSERT_OK(err, c"getsockopt".as_ptr()) {
        if all_preorder {
            ASSERT_TRUE(
                *result.add(0) == 3
                    && *result.add(1) == 4
                    && *result.add(2) == 1
                    && *result.add(3) == 2,
                c"parent and child".as_ptr(),
            );
        } else {
            ASSERT_TRUE(
                *result.add(0) == 4
                    && *result.add(1) == 2
                    && *result.add(2) == 1
                    && *result.add(3) == 3,
                c"parent and child".as_ptr(),
            );
        }
    }

    ASSERT_OK(
        bpf_prog_detach2(prog_p2_fd, cg_parent, prog_p2_atype),
        c"bpf_prog_detach2-parent_2".as_ptr(),
    );
    ASSERT_OK(
        bpf_prog_detach2(prog_p_fd, cg_parent, prog_p_atype),
        c"bpf_prog_detach2-parent".as_ptr(),
    );
    ASSERT_OK(
        bpf_prog_detach2(prog_c2_fd, cg_child, prog_c2_atype),
        c"bpf_prog_detach2-child_2".as_ptr(),
    );
    ASSERT_OK(
        bpf_prog_detach2(prog_c_fd, cg_child, prog_c_atype),
        c"bpf_prog_detach2-child".as_ptr(),
    );
    cgroup_preorder__destroy(skel);
    err
}

/*
 * Replacing a link's program (bpf_link_update) must target the correct slot in
 * the effective array even when a BPF_F_PREORDER program is attached to the
 * same cgroup. All programs here are attached to a single cgroup; "parent" is
 * reused only as a third distinct program.
 *
 * Attach child(1) normally and child_2(2) with BPF_F_PREORDER, so the effective
 * order is [2, 1]. Then replace child(1)'s program with parent(3): only the
 * non-preorder slot changes, giving [2, 3].
 */
unsafe fn run_link_replace_test(cgroup_fd: c_int, sock_fd: c_int) -> c_int {
    let mut create_opts = bpf_link_create_opts {
        sz: core::mem::size_of::<bpf_link_create_opts>(),
        flags: 0,
    };
    let mut err: c_int = 0;
    let mut normal_link: c_int = -1;
    let mut preorder_link: c_int = -1;
    let skel: *mut cgroup_preorder;
    let atype: bpf_attach_type;
    let result: *mut __u8;
    let mut buf: __u8 = 0x00;
    let mut optlen: SocklenT = 1;

    skel = cgroup_preorder__open_and_load();
    if !ASSERT_OK_PTR(skel.cast::<c_void>(), c"cgroup_preorder__open_and_load".as_ptr()) {
        return -1;
    }

    err = setsockopt(sock_fd, SOL_IP, IP_TOS, (&buf as *const __u8).cast::<c_void>(), 1);
    if !ASSERT_OK(err, c"setsockopt".as_ptr()) {
        cgroup_preorder__destroy(skel);
        return err;
    }

    atype = bpf_program__expected_attach_type((*skel).progs.child);

    create_opts.flags = 0;
    normal_link = bpf_link_create(
        bpf_program__fd((*skel).progs.child),
        cgroup_fd,
        atype,
        &create_opts,
    );
    if !ASSERT_GE(normal_link, 0, c"create_normal_link".as_ptr()) {
        err = normal_link;
        cgroup_preorder__destroy(skel);
        return err;
    }

    create_opts.flags = BPF_F_PREORDER;
    preorder_link = bpf_link_create(
        bpf_program__fd((*skel).progs.child_2),
        cgroup_fd,
        atype,
        &create_opts,
    );
    if !ASSERT_GE(preorder_link, 0, c"create_preorder_link".as_ptr()) {
        err = preorder_link;
        close(normal_link);
        cgroup_preorder__destroy(skel);
        return err;
    }

    result = (*(*skel).bss).result.as_mut_ptr();
    (*(*skel).bss).idx = 0;
    memset(result.cast::<c_void>(), 0, 4);

    err = getsockopt(
        sock_fd,
        SOL_IP,
        IP_TOS,
        (&mut buf as *mut __u8).cast::<c_void>(),
        &mut optlen,
    );
    if ASSERT_OK(err, c"getsockopt-before".as_ptr()) {
        ASSERT_TRUE(
            *result.add(0) == 2 && *result.add(1) == 1,
            c"order before update".as_ptr(),
        );

        /* Replace the normal link's program child(1) -> parent(3). */
        err = bpf_link_update(
            normal_link,
            bpf_program__fd((*skel).progs.parent),
            ptr::null(),
        );
        if ASSERT_OK(err, c"bpf_link_update".as_ptr()) {
            (*(*skel).bss).idx = 0;
            memset(result.cast::<c_void>(), 0, 4);

            err = getsockopt(
                sock_fd,
                SOL_IP,
                IP_TOS,
                (&mut buf as *mut __u8).cast::<c_void>(),
                &mut optlen,
            );
            if ASSERT_OK(err, c"getsockopt-after".as_ptr()) {
                ASSERT_TRUE(
                    *result.add(0) == 2 && *result.add(1) == 3,
                    c"order after update".as_ptr(),
                );
            }
        }
    }

    if preorder_link >= 0 {
        close(preorder_link);
    }
    close(normal_link);
    cgroup_preorder__destroy(skel);
    err
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_cgroup_preorder() {
    let mut cg_parent: c_int = -1;
    let mut cg_child: c_int = -1;
    let mut sock_fd: c_int = -1;

    cg_parent = test__join_cgroup(c"/parent".as_ptr());
    if !ASSERT_GE(cg_parent, 0, c"join_cgroup /parent".as_ptr()) {
        close(sock_fd);
        close(cg_child);
        close(cg_parent);
        return;
    }

    cg_child = test__join_cgroup(c"/parent/child".as_ptr());
    if !ASSERT_GE(cg_child, 0, c"join_cgroup /parent/child".as_ptr()) {
        close(sock_fd);
        close(cg_child);
        close(cg_parent);
        return;
    }

    sock_fd = socket(AF_INET, SOCK_STREAM, 0);
    if ASSERT_GE(sock_fd, 0, c"socket".as_ptr()) {
        ASSERT_OK(
            run_getsockopt_test(cg_parent, cg_child, sock_fd, false),
            c"getsockopt_test_1".as_ptr(),
        );
        ASSERT_OK(
            run_getsockopt_test(cg_parent, cg_child, sock_fd, true),
            c"getsockopt_test_2".as_ptr(),
        );
        ASSERT_OK(
            run_link_replace_test(cg_child, sock_fd),
            c"link_replace_test".as_ptr(),
        );
    }

    close(sock_fd);
    close(cg_child);
    close(cg_parent);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
