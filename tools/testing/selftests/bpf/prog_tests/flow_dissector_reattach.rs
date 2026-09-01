// SPDX-License-Identifier: GPL-2.0
/*
 * Tests for attaching, detaching, and replacing flow_dissector BPF program.
 */

// C dependencies originally included:
// errno.h, fcntl.h, sched.h, stdbool.h, sys/stat.h, unistd.h,
// linux/bpf.h, bpf/bpf.h, and "test_progs.h".

use core::ffi::{c_char, c_int, c_uint, c_void};

type __u32 = u32;

#[repr(C)]
pub struct bpf_insn {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_prog_info {
    pub id: __u32,
}

#[repr(C)]
pub struct bpf_link_info_netns {
    pub netns_ino: u64,
    pub attach_type: __u32,
}

#[repr(C)]
pub struct bpf_link_info {
    pub type_: __u32,
    pub id: __u32,
    pub prog_id: __u32,
    pub netns: bpf_link_info_netns,
}

#[repr(C)]
pub struct stat {
    pub st_ino: u64,
}

#[repr(C)]
pub struct bpf_link_create_opts {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link_update_opts {
    pub flags: __u32,
    pub old_prog_fd: c_int,
}

type bpf_prog_type = c_uint;

unsafe extern "C" {
    static mut errno: c_int;

    static BPF_FLOW_DISSECTOR: c_uint;
    static BPF_LINK_TYPE_NETNS: c_uint;
    static BPF_PROG_TYPE_FLOW_DISSECTOR: bpf_prog_type;
    static BPF_PROG_TYPE_SOCKET_FILTER: bpf_prog_type;
    static BPF_REG_0: c_uint;
    static BPF_OK: c_int;
    static BPF_F_REPLACE: __u32;
    static BPF_F_ALLOW_MULTI: __u32;
    static CLONE_NEWNET: c_int;
    static O_RDONLY: c_int;
    static EINVAL: c_int;
    static E2BIG: c_int;
    static EEXIST: c_int;
    static EPERM: c_int;
    static EBADF: c_int;
    static ENOLINK: c_int;

    fn BPF_MOV64_IMM(dst: c_uint, imm: c_int) -> bpf_insn;
    fn BPF_EXIT_INSN() -> bpf_insn;
    fn CHECK_FAIL(cond: bool) -> bool;
    fn bpf_prog_query(
        target_fd: c_int,
        type_: c_uint,
        query_flags: __u32,
        attach_flags: *mut __u32,
        prog_ids: *mut __u32,
        prog_cnt: *mut __u32,
    ) -> c_int;
    fn bpf_test_load_program(
        type_: bpf_prog_type,
        insns: *const bpf_insn,
        insn_cnt: usize,
        license: *const c_char,
        kern_version: __u32,
        log_buf: *mut c_char,
        log_buf_sz: usize,
    ) -> c_int;
    fn bpf_prog_get_info_by_fd(
        prog_fd: c_int,
        info: *mut bpf_prog_info,
        info_len: *mut __u32,
    ) -> c_int;
    fn bpf_prog_attach(
        prog_fd: c_int,
        target_fd: c_int,
        type_: c_uint,
        flags: c_uint,
    ) -> c_int;
    fn bpf_prog_detach2(prog_fd: c_int, target_fd: c_int, type_: c_uint) -> c_int;
    fn bpf_link_create(
        prog_fd: c_int,
        target_fd: c_int,
        attach_type: c_uint,
        opts: *const bpf_link_create_opts,
    ) -> c_int;
    fn bpf_link_update(
        link_fd: c_int,
        new_prog_fd: c_int,
        opts: *const bpf_link_update_opts,
    ) -> c_int;
    fn bpf_link_get_info_by_fd(
        link_fd: c_int,
        info: *mut bpf_link_info,
        info_len: *mut __u32,
    ) -> c_int;
    fn unshare(flags: c_int) -> c_int;
    fn setns(fd: c_int, nstype: c_int) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn fstat(fd: c_int, statbuf: *mut stat) -> c_int;
    fn perror(s: *const c_char);
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn test__start_subtest(name: *const c_char) -> bool;
    fn test__skip();
}

static mut init_net: c_int = -1;

unsafe fn query_attached_prog_id(netns: c_int) -> __u32 {
    let mut prog_ids: [__u32; 1] = [0; 1];
    let mut prog_cnt: __u32 = prog_ids.len() as __u32;
    let err: c_int;

    err = bpf_prog_query(
        netns,
        BPF_FLOW_DISSECTOR,
        0,
        core::ptr::null_mut(),
        prog_ids.as_mut_ptr(),
        &mut prog_cnt,
    );
    if CHECK_FAIL(err != 0) {
        perror(c"bpf_prog_query".as_ptr());
        return 0;
    }

    if prog_cnt == 1 {
        prog_ids[0]
    } else {
        0
    }
}

unsafe fn prog_is_attached(netns: c_int) -> bool {
    query_attached_prog_id(netns) > 0
}

unsafe fn load_prog(type_: bpf_prog_type) -> c_int {
    let prog: [bpf_insn; 2] = [
        BPF_MOV64_IMM(BPF_REG_0, BPF_OK),
        BPF_EXIT_INSN(),
    ];
    let fd: c_int;

    fd = bpf_test_load_program(
        type_,
        prog.as_ptr(),
        prog.len(),
        c"GPL".as_ptr(),
        0,
        core::ptr::null_mut(),
        0,
    );
    if CHECK_FAIL(fd < 0) {
        perror(c"bpf_test_load_program".as_ptr());
    }

    fd
}

unsafe fn query_prog_id(prog: c_int) -> __u32 {
    let mut info: bpf_prog_info = core::mem::zeroed();
    let mut info_len: __u32 = core::mem::size_of::<bpf_prog_info>() as __u32;
    let err: c_int;

    err = bpf_prog_get_info_by_fd(prog, &mut info, &mut info_len);
    if CHECK_FAIL(err != 0 || info_len as usize != core::mem::size_of::<bpf_prog_info>()) {
        perror(c"bpf_prog_get_info_by_fd".as_ptr());
        return 0;
    }

    info.id
}

unsafe fn unshare_net(old_net: c_int) -> c_int {
    let err: c_int;
    let new_net: c_int;

    err = unshare(CLONE_NEWNET);
    if CHECK_FAIL(err != 0) {
        perror(c"unshare(CLONE_NEWNET)".as_ptr());
        return -1;
    }
    new_net = open(c"/proc/self/ns/net".as_ptr(), O_RDONLY);
    if CHECK_FAIL(new_net < 0) {
        perror(c"open(/proc/self/ns/net)".as_ptr());
        setns(old_net, CLONE_NEWNET);
        return -1;
    }
    new_net
}

unsafe fn test_prog_attach_prog_attach(netns: c_int, prog1: c_int, prog2: c_int) {
    let mut err: c_int;

    err = bpf_prog_attach(prog1, 0, BPF_FLOW_DISSECTOR, 0);
    if CHECK_FAIL(err != 0) {
        perror(c"bpf_prog_attach(prog1)".as_ptr());
        return;
    }
    CHECK_FAIL(query_attached_prog_id(netns) != query_prog_id(prog1));

    /* Expect success when attaching a different program */
    err = bpf_prog_attach(prog2, 0, BPF_FLOW_DISSECTOR, 0);
    if CHECK_FAIL(err != 0) {
        perror(c"bpf_prog_attach(prog2) #1".as_ptr());
    } else {
        CHECK_FAIL(query_attached_prog_id(netns) != query_prog_id(prog2));

        /* Expect failure when attaching the same program twice */
        err = bpf_prog_attach(prog2, 0, BPF_FLOW_DISSECTOR, 0);
        if CHECK_FAIL(err == 0 || errno != EINVAL) {
            perror(c"bpf_prog_attach(prog2) #2".as_ptr());
        }
        CHECK_FAIL(query_attached_prog_id(netns) != query_prog_id(prog2));
    }

    err = bpf_prog_detach2(prog2, 0, BPF_FLOW_DISSECTOR);
    if CHECK_FAIL(err != 0) {
        perror(c"bpf_prog_detach".as_ptr());
    }
    CHECK_FAIL(prog_is_attached(netns));
}

unsafe fn test_link_create_link_create(netns: c_int, prog1: c_int, prog2: c_int) {
    let opts: bpf_link_create_opts = core::mem::zeroed();
    let link1: c_int;
    let link2: c_int;

    link1 = bpf_link_create(prog1, netns, BPF_FLOW_DISSECTOR, &opts);
    if CHECK_FAIL(link1 < 0) {
        perror(c"bpf_link_create(prog1)".as_ptr());
        return;
    }
    CHECK_FAIL(query_attached_prog_id(netns) != query_prog_id(prog1));

    /* Expect failure creating link when another link exists */
    errno = 0;
    link2 = bpf_link_create(prog2, netns, BPF_FLOW_DISSECTOR, &opts);
    if CHECK_FAIL(link2 >= 0 || errno != E2BIG) {
        perror(c"bpf_prog_attach(prog2) expected E2BIG".as_ptr());
    }
    if link2 >= 0 {
        close(link2);
    }
    CHECK_FAIL(query_attached_prog_id(netns) != query_prog_id(prog1));

    close(link1);
    CHECK_FAIL(prog_is_attached(netns));
}

unsafe fn test_prog_attach_link_create(netns: c_int, prog1: c_int, prog2: c_int) {
    let opts: bpf_link_create_opts = core::mem::zeroed();
    let mut err: c_int;
    let link: c_int;

    err = bpf_prog_attach(prog1, 0, BPF_FLOW_DISSECTOR, 0);
    if CHECK_FAIL(err != 0) {
        perror(c"bpf_prog_attach(prog1)".as_ptr());
        return;
    }
    CHECK_FAIL(query_attached_prog_id(netns) != query_prog_id(prog1));

    /* Expect failure creating link when prog attached */
    errno = 0;
    link = bpf_link_create(prog2, netns, BPF_FLOW_DISSECTOR, &opts);
    if CHECK_FAIL(link >= 0 || errno != EEXIST) {
        perror(c"bpf_link_create(prog2) expected EEXIST".as_ptr());
    }
    if link >= 0 {
        close(link);
    }
    CHECK_FAIL(query_attached_prog_id(netns) != query_prog_id(prog1));

    err = bpf_prog_detach2(prog1, 0, BPF_FLOW_DISSECTOR);
    if CHECK_FAIL(err != 0) {
        perror(c"bpf_prog_detach".as_ptr());
    }
    CHECK_FAIL(prog_is_attached(netns));
}

unsafe fn test_link_create_prog_attach(netns: c_int, prog1: c_int, prog2: c_int) {
    let opts: bpf_link_create_opts = core::mem::zeroed();
    let mut err: c_int;
    let link: c_int;

    link = bpf_link_create(prog1, netns, BPF_FLOW_DISSECTOR, &opts);
    if CHECK_FAIL(link < 0) {
        perror(c"bpf_link_create(prog1)".as_ptr());
        return;
    }
    CHECK_FAIL(query_attached_prog_id(netns) != query_prog_id(prog1));

    /* Expect failure attaching prog when link exists */
    errno = 0;
    err = bpf_prog_attach(prog2, 0, BPF_FLOW_DISSECTOR, 0);
    if CHECK_FAIL(err == 0 || errno != EEXIST) {
        perror(c"bpf_prog_attach(prog2) expected EEXIST".as_ptr());
    }
    CHECK_FAIL(query_attached_prog_id(netns) != query_prog_id(prog1));

    close(link);
    CHECK_FAIL(prog_is_attached(netns));
}

unsafe fn test_link_create_prog_detach(netns: c_int, prog1: c_int, _prog2: c_int) {
    let opts: bpf_link_create_opts = core::mem::zeroed();
    let mut err: c_int;
    let link: c_int;

    link = bpf_link_create(prog1, netns, BPF_FLOW_DISSECTOR, &opts);
    if CHECK_FAIL(link < 0) {
        perror(c"bpf_link_create(prog1)".as_ptr());
        return;
    }
    CHECK_FAIL(query_attached_prog_id(netns) != query_prog_id(prog1));

    /* Expect failure detaching prog when link exists */
    errno = 0;
    err = bpf_prog_detach2(prog1, 0, BPF_FLOW_DISSECTOR);
    if CHECK_FAIL(err == 0 || errno != EINVAL) {
        perror(c"bpf_prog_detach expected EINVAL".as_ptr());
    }
    CHECK_FAIL(query_attached_prog_id(netns) != query_prog_id(prog1));

    close(link);
    CHECK_FAIL(prog_is_attached(netns));
}

unsafe fn test_prog_attach_detach_query(netns: c_int, prog1: c_int, _prog2: c_int) {
    let mut err: c_int;

    err = bpf_prog_attach(prog1, 0, BPF_FLOW_DISSECTOR, 0);
    if CHECK_FAIL(err != 0) {
        perror(c"bpf_prog_attach(prog1)".as_ptr());
        return;
    }
    CHECK_FAIL(query_attached_prog_id(netns) != query_prog_id(prog1));

    err = bpf_prog_detach2(prog1, 0, BPF_FLOW_DISSECTOR);
    if CHECK_FAIL(err != 0) {
        perror(c"bpf_prog_detach".as_ptr());
        return;
    }

    /* Expect no prog attached after successful detach */
    CHECK_FAIL(prog_is_attached(netns));
}

unsafe fn test_link_create_close_query(netns: c_int, prog1: c_int, _prog2: c_int) {
    let opts: bpf_link_create_opts = core::mem::zeroed();
    let link: c_int;

    link = bpf_link_create(prog1, netns, BPF_FLOW_DISSECTOR, &opts);
    if CHECK_FAIL(link < 0) {
        perror(c"bpf_link_create(prog1)".as_ptr());
        return;
    }
    CHECK_FAIL(query_attached_prog_id(netns) != query_prog_id(prog1));

    close(link);
    /* Expect no prog attached after closing last link FD */
    CHECK_FAIL(prog_is_attached(netns));
}

unsafe fn test_link_update_no_old_prog(netns: c_int, prog1: c_int, prog2: c_int) {
    let create_opts: bpf_link_create_opts = core::mem::zeroed();
    let mut update_opts: bpf_link_update_opts = core::mem::zeroed();
    let mut err: c_int;
    let link: c_int;

    link = bpf_link_create(prog1, netns, BPF_FLOW_DISSECTOR, &create_opts);
    if CHECK_FAIL(link < 0) {
        perror(c"bpf_link_create(prog1)".as_ptr());
        return;
    }
    CHECK_FAIL(query_attached_prog_id(netns) != query_prog_id(prog1));

    /* Expect success replacing the prog when old prog not specified */
    update_opts.flags = 0;
    update_opts.old_prog_fd = 0;
    err = bpf_link_update(link, prog2, &update_opts);
    if CHECK_FAIL(err != 0) {
        perror(c"bpf_link_update".as_ptr());
    }
    CHECK_FAIL(query_attached_prog_id(netns) != query_prog_id(prog2));

    close(link);
    CHECK_FAIL(prog_is_attached(netns));
}

unsafe fn test_link_update_replace_old_prog(netns: c_int, prog1: c_int, prog2: c_int) {
    let create_opts: bpf_link_create_opts = core::mem::zeroed();
    let mut update_opts: bpf_link_update_opts = core::mem::zeroed();
    let mut err: c_int;
    let link: c_int;

    link = bpf_link_create(prog1, netns, BPF_FLOW_DISSECTOR, &create_opts);
    if CHECK_FAIL(link < 0) {
        perror(c"bpf_link_create(prog1)".as_ptr());
        return;
    }
    CHECK_FAIL(query_attached_prog_id(netns) != query_prog_id(prog1));

    /* Expect success F_REPLACE and old prog specified to succeed */
    update_opts.flags = BPF_F_REPLACE;
    update_opts.old_prog_fd = prog1;
    err = bpf_link_update(link, prog2, &update_opts);
    if CHECK_FAIL(err != 0) {
        perror(c"bpf_link_update".as_ptr());
    }
    CHECK_FAIL(query_attached_prog_id(netns) != query_prog_id(prog2));

    close(link);
    CHECK_FAIL(prog_is_attached(netns));
}

unsafe fn test_link_update_same_prog(netns: c_int, prog1: c_int, _prog2: c_int) {
    let create_opts: bpf_link_create_opts = core::mem::zeroed();
    let mut update_opts: bpf_link_update_opts = core::mem::zeroed();
    let mut err: c_int;
    let link: c_int;

    link = bpf_link_create(prog1, netns, BPF_FLOW_DISSECTOR, &create_opts);
    if CHECK_FAIL(link < 0) {
        perror(c"bpf_link_create(prog1)".as_ptr());
        return;
    }
    CHECK_FAIL(query_attached_prog_id(netns) != query_prog_id(prog1));

    /* Expect success updating the prog with the same one */
    update_opts.flags = 0;
    update_opts.old_prog_fd = 0;
    err = bpf_link_update(link, prog1, &update_opts);
    if CHECK_FAIL(err != 0) {
        perror(c"bpf_link_update".as_ptr());
    }
    CHECK_FAIL(query_attached_prog_id(netns) != query_prog_id(prog1));

    close(link);
    CHECK_FAIL(prog_is_attached(netns));
}

unsafe fn test_link_update_invalid_opts(netns: c_int, prog1: c_int, prog2: c_int) {
    let create_opts: bpf_link_create_opts = core::mem::zeroed();
    let mut update_opts: bpf_link_update_opts = core::mem::zeroed();
    let mut err: c_int;
    let link: c_int;

    link = bpf_link_create(prog1, netns, BPF_FLOW_DISSECTOR, &create_opts);
    if CHECK_FAIL(link < 0) {
        perror(c"bpf_link_create(prog1)".as_ptr());
        return;
    }
    CHECK_FAIL(query_attached_prog_id(netns) != query_prog_id(prog1));

    'out_close: loop {
        /* Expect update to fail w/ old prog FD but w/o F_REPLACE*/
        errno = 0;
        update_opts.flags = 0;
        update_opts.old_prog_fd = prog1;
        err = bpf_link_update(link, prog2, &update_opts);
        if CHECK_FAIL(err == 0 || errno != EINVAL) {
            perror(c"bpf_link_update expected EINVAL".as_ptr());
            break 'out_close;
        }
        CHECK_FAIL(query_attached_prog_id(netns) != query_prog_id(prog1));

        /* Expect update to fail on old prog FD mismatch */
        errno = 0;
        update_opts.flags = BPF_F_REPLACE;
        update_opts.old_prog_fd = prog2;
        err = bpf_link_update(link, prog2, &update_opts);
        if CHECK_FAIL(err == 0 || errno != EPERM) {
            perror(c"bpf_link_update expected EPERM".as_ptr());
            break 'out_close;
        }
        CHECK_FAIL(query_attached_prog_id(netns) != query_prog_id(prog1));

        /* Expect update to fail for invalid old prog FD */
        errno = 0;
        update_opts.flags = BPF_F_REPLACE;
        update_opts.old_prog_fd = -1;
        err = bpf_link_update(link, prog2, &update_opts);
        if CHECK_FAIL(err == 0 || errno != EBADF) {
            perror(c"bpf_link_update expected EBADF".as_ptr());
            break 'out_close;
        }
        CHECK_FAIL(query_attached_prog_id(netns) != query_prog_id(prog1));

        /* Expect update to fail with invalid flags */
        errno = 0;
        update_opts.flags = BPF_F_ALLOW_MULTI;
        update_opts.old_prog_fd = 0;
        err = bpf_link_update(link, prog2, &update_opts);
        if CHECK_FAIL(err == 0 || errno != EINVAL) {
            perror(c"bpf_link_update expected EINVAL".as_ptr());
        }
        CHECK_FAIL(query_attached_prog_id(netns) != query_prog_id(prog1));
        break 'out_close;
    }

    close(link);
    CHECK_FAIL(prog_is_attached(netns));
}

unsafe fn test_link_update_invalid_prog(netns: c_int, prog1: c_int, _prog2: c_int) {
    let create_opts: bpf_link_create_opts = core::mem::zeroed();
    let mut update_opts: bpf_link_update_opts = core::mem::zeroed();
    let mut err: c_int;
    let link: c_int;
    let prog3: c_int;

    link = bpf_link_create(prog1, netns, BPF_FLOW_DISSECTOR, &create_opts);
    if CHECK_FAIL(link < 0) {
        perror(c"bpf_link_create(prog1)".as_ptr());
        return;
    }
    CHECK_FAIL(query_attached_prog_id(netns) != query_prog_id(prog1));

    'out_close_link: loop {
        /* Expect failure when new prog FD is not valid */
        errno = 0;
        update_opts.flags = 0;
        update_opts.old_prog_fd = 0;
        err = bpf_link_update(link, -1, &update_opts);
        if CHECK_FAIL(err == 0 || errno != EBADF) {
            perror(c"bpf_link_update expected EINVAL".as_ptr());
            break 'out_close_link;
        }
        CHECK_FAIL(query_attached_prog_id(netns) != query_prog_id(prog1));

        prog3 = load_prog(BPF_PROG_TYPE_SOCKET_FILTER);
        if prog3 < 0 {
            break 'out_close_link;
        }

        /* Expect failure when new prog FD type doesn't match */
        errno = 0;
        update_opts.flags = 0;
        update_opts.old_prog_fd = 0;
        err = bpf_link_update(link, prog3, &update_opts);
        if CHECK_FAIL(err == 0 || errno != EINVAL) {
            perror(c"bpf_link_update expected EINVAL".as_ptr());
        }
        CHECK_FAIL(query_attached_prog_id(netns) != query_prog_id(prog1));

        close(prog3);
        break 'out_close_link;
    }

    close(link);
    CHECK_FAIL(prog_is_attached(netns));
}

unsafe fn test_link_update_netns_gone(netns_arg: c_int, prog1: c_int, prog2: c_int) {
    let create_opts: bpf_link_create_opts = core::mem::zeroed();
    let mut update_opts: bpf_link_update_opts = core::mem::zeroed();
    let mut err: c_int;
    let link: c_int;
    let old_net: c_int;
    let mut netns = netns_arg;

    old_net = netns;
    netns = unshare_net(old_net);
    if netns < 0 {
        return;
    }

    link = bpf_link_create(prog1, netns, BPF_FLOW_DISSECTOR, &create_opts);
    if CHECK_FAIL(link < 0) {
        perror(c"bpf_link_create(prog1)".as_ptr());
        return;
    }
    CHECK_FAIL(query_attached_prog_id(netns) != query_prog_id(prog1));

    close(netns);
    err = setns(old_net, CLONE_NEWNET);
    if CHECK_FAIL(err != 0) {
        perror(c"setns(CLONE_NEWNET)".as_ptr());
        close(link);
        return;
    }

    /* Expect failure when netns destroyed */
    errno = 0;
    update_opts.flags = 0;
    update_opts.old_prog_fd = 0;
    err = bpf_link_update(link, prog2, &update_opts);
    if CHECK_FAIL(err == 0 || errno != ENOLINK) {
        perror(c"bpf_link_update".as_ptr());
    }

    close(link);
}

unsafe fn test_link_get_info(netns_arg: c_int, prog1: c_int, prog2: c_int) {
    let create_opts: bpf_link_create_opts = core::mem::zeroed();
    let mut update_opts: bpf_link_update_opts = core::mem::zeroed();
    let mut info: bpf_link_info = core::mem::zeroed();
    let mut netns_stat: stat = core::mem::zeroed();
    let mut info_len: __u32;
    let link_id: __u32;
    let mut err: c_int;
    let link: c_int;
    let mut old_net: c_int;
    let mut netns = netns_arg;

    old_net = netns;
    netns = unshare_net(old_net);
    if netns < 0 {
        return;
    }

    'out_resetns: loop {
        err = fstat(netns, &mut netns_stat);
        if CHECK_FAIL(err != 0) {
            perror(c"stat(netns)".as_ptr());
            break 'out_resetns;
        }

        link = bpf_link_create(prog1, netns, BPF_FLOW_DISSECTOR, &create_opts);
        if CHECK_FAIL(link < 0) {
            perror(c"bpf_link_create(prog1)".as_ptr());
            break 'out_resetns;
        }

        'out_unlink: loop {
            info_len = core::mem::size_of::<bpf_link_info>() as __u32;
            err = bpf_link_get_info_by_fd(link, &mut info, &mut info_len);
            if CHECK_FAIL(err != 0) {
                perror(c"bpf_obj_get_info".as_ptr());
                break 'out_unlink;
            }
            CHECK_FAIL(info_len as usize != core::mem::size_of::<bpf_link_info>());

            /* Expect link info to be sane and match prog and netns details */
            CHECK_FAIL(info.type_ != BPF_LINK_TYPE_NETNS);
            CHECK_FAIL(info.id == 0);
            CHECK_FAIL(info.prog_id != query_prog_id(prog1));
            CHECK_FAIL(info.netns.netns_ino != netns_stat.st_ino);
            CHECK_FAIL(info.netns.attach_type != BPF_FLOW_DISSECTOR);

            update_opts.flags = 0;
            update_opts.old_prog_fd = 0;
            err = bpf_link_update(link, prog2, &update_opts);
            if CHECK_FAIL(err != 0) {
                perror(c"bpf_link_update(prog2)".as_ptr());
                break 'out_unlink;
            }

            link_id = info.id;
            info_len = core::mem::size_of::<bpf_link_info>() as __u32;
            err = bpf_link_get_info_by_fd(link, &mut info, &mut info_len);
            if CHECK_FAIL(err != 0) {
                perror(c"bpf_obj_get_info".as_ptr());
                break 'out_unlink;
            }
            CHECK_FAIL(info_len as usize != core::mem::size_of::<bpf_link_info>());

            /* Expect no info change after update except in prog id */
            CHECK_FAIL(info.type_ != BPF_LINK_TYPE_NETNS);
            CHECK_FAIL(info.id != link_id);
            CHECK_FAIL(info.prog_id != query_prog_id(prog2));
            CHECK_FAIL(info.netns.netns_ino != netns_stat.st_ino);
            CHECK_FAIL(info.netns.attach_type != BPF_FLOW_DISSECTOR);

            /* Leave netns link is attached to and close last FD to it */
            err = setns(old_net, CLONE_NEWNET);
            if CHECK_FAIL(err != 0) {
                perror(c"setns(NEWNET)".as_ptr());
                break 'out_unlink;
            }
            close(netns);
            old_net = -1;
            netns = -1;

            info_len = core::mem::size_of::<bpf_link_info>() as __u32;
            err = bpf_link_get_info_by_fd(link, &mut info, &mut info_len);
            if CHECK_FAIL(err != 0) {
                perror(c"bpf_obj_get_info".as_ptr());
                break 'out_unlink;
            }
            CHECK_FAIL(info_len as usize != core::mem::size_of::<bpf_link_info>());

            /* Expect netns_ino to change to 0 */
            CHECK_FAIL(info.type_ != BPF_LINK_TYPE_NETNS);
            CHECK_FAIL(info.id != link_id);
            CHECK_FAIL(info.prog_id != query_prog_id(prog2));
            CHECK_FAIL(info.netns.netns_ino != 0);
            CHECK_FAIL(info.netns.attach_type != BPF_FLOW_DISSECTOR);
            break 'out_unlink;
        }

        close(link);
        break 'out_resetns;
    }

    if old_net != -1 {
        setns(old_net, CLONE_NEWNET);
    }
    if netns != -1 {
        close(netns);
    }
}

#[repr(C)]
struct test {
    test_name: *const c_char,
    test_func: unsafe fn(c_int, c_int, c_int),
}

unsafe fn run_tests(netns: c_int) {
    let tests: [test; 14] = [
        test {
            test_name: c"prog attach, prog attach".as_ptr(),
            test_func: test_prog_attach_prog_attach,
        },
        test {
            test_name: c"link create, link create".as_ptr(),
            test_func: test_link_create_link_create,
        },
        test {
            test_name: c"prog attach, link create".as_ptr(),
            test_func: test_prog_attach_link_create,
        },
        test {
            test_name: c"link create, prog attach".as_ptr(),
            test_func: test_link_create_prog_attach,
        },
        test {
            test_name: c"link create, prog detach".as_ptr(),
            test_func: test_link_create_prog_detach,
        },
        test {
            test_name: c"prog attach, detach, query".as_ptr(),
            test_func: test_prog_attach_detach_query,
        },
        test {
            test_name: c"link create, close, query".as_ptr(),
            test_func: test_link_create_close_query,
        },
        test {
            test_name: c"link update no old prog".as_ptr(),
            test_func: test_link_update_no_old_prog,
        },
        test {
            test_name: c"link update with replace old prog".as_ptr(),
            test_func: test_link_update_replace_old_prog,
        },
        test {
            test_name: c"link update with same prog".as_ptr(),
            test_func: test_link_update_same_prog,
        },
        test {
            test_name: c"link update invalid opts".as_ptr(),
            test_func: test_link_update_invalid_opts,
        },
        test {
            test_name: c"link update invalid prog".as_ptr(),
            test_func: test_link_update_invalid_prog,
        },
        test {
            test_name: c"link update netns gone".as_ptr(),
            test_func: test_link_update_netns_gone,
        },
        test {
            test_name: c"link get info".as_ptr(),
            test_func: test_link_get_info,
        },
    ];
    let mut progs: [c_int; 2] = [-1, -1];
    let mut test_name: [c_char; 80] = [0; 80];
    let mut i: usize;

    'out_close: loop {
        i = 0;
        while i < progs.len() {
            progs[i] = load_prog(BPF_PROG_TYPE_FLOW_DISSECTOR);
            if progs[i] < 0 {
                break 'out_close;
            }
            i += 1;
        }

        i = 0;
        while i < tests.len() {
            snprintf(
                test_name.as_mut_ptr(),
                test_name.len(),
                c"flow dissector %s%s".as_ptr(),
                tests[i].test_name,
                if netns == init_net {
                    c" (init_net)".as_ptr()
                } else {
                    c"".as_ptr()
                },
            );
            if test__start_subtest(test_name.as_ptr()) {
                (tests[i].test_func)(netns, progs[0], progs[1]);
            }
            i += 1;
        }
        break 'out_close;
    }

    i = 0;
    while i < progs.len() {
        if progs[i] >= 0 {
            CHECK_FAIL(close(progs[i]) != 0);
        }
        i += 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn serial_test_flow_dissector_reattach() {
    let mut err: c_int;
    let new_net: c_int;
    let saved_net: c_int;

    saved_net = open(c"/proc/self/ns/net".as_ptr(), O_RDONLY);
    if CHECK_FAIL(saved_net < 0) {
        perror(c"open(/proc/self/ns/net".as_ptr());
        return;
    }

    init_net = open(c"/proc/1/ns/net".as_ptr(), O_RDONLY);
    if CHECK_FAIL(init_net < 0) {
        perror(c"open(/proc/1/ns/net)".as_ptr());
    } else {
        'out_setns: loop {
            err = setns(init_net, CLONE_NEWNET);
            if CHECK_FAIL(err != 0) {
                perror(c"setns(/proc/1/ns/net)".as_ptr());
                break 'out_setns;
            }

            if prog_is_attached(init_net) {
                test__skip();
                printf(c"Can't test with flow dissector attached to init_net\n".as_ptr());
                break 'out_setns;
            }

            /* First run tests in root network namespace */
            run_tests(init_net);

            /* Then repeat tests in a non-root namespace */
            new_net = unshare_net(init_net);
            if new_net < 0 {
                break 'out_setns;
            }
            run_tests(new_net);
            close(new_net);
            break 'out_setns;
        }

        /* Move back to netns we started in. */
        err = setns(saved_net, CLONE_NEWNET);
        if CHECK_FAIL(err != 0) {
            perror(c"setns(/proc/self/ns/net)".as_ptr());
        }
    }

    close(init_net);
    close(saved_net);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
