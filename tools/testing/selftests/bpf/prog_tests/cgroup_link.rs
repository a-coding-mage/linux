// SPDX-License-Identifier: GPL-2.0

// Translated from:
// #include <test_progs.h>
// #include "cgroup_helpers.h"
// #include "testing_helpers.h"
// #include "test_cgroup_link.skel.h"

use core::ptr;

static mut duration: __u32 = 0;
const PING_CMD: *const i8 = b"ping -q -c1 -w1 127.0.0.1 > /dev/null\0".as_ptr() as *const i8;

static mut skel: *mut test_cgroup_link = ptr::null_mut();

pub unsafe fn ping_and_check(exp_calls: i32, exp_alt_calls: i32) -> i32 {
    (*(*skel).bss).calls = 0;
    (*(*skel).bss).alt_calls = 0;
    CHECK_FAIL!(system(PING_CMD));
    if CHECK!(
        (*(*skel).bss).calls != exp_calls,
        "call_cnt",
        "exp %d, got %d\n",
        exp_calls,
        (*(*skel).bss).calls
    ) {
        return -EINVAL;
    }
    if CHECK!(
        (*(*skel).bss).alt_calls != exp_alt_calls,
        "alt_call_cnt",
        "exp %d, got %d\n",
        exp_alt_calls,
        (*(*skel).bss).alt_calls
    ) {
        return -EINVAL;
    }
    0
}

pub unsafe fn serial_test_cgroup_link() {
    #[repr(C)]
    struct Cg {
        path: *const i8,
        fd: i32,
    }

    let mut cgs = [
        Cg {
            path: b"/cg1\0".as_ptr() as *const i8,
            fd: 0,
        },
        Cg {
            path: b"/cg1/cg2\0".as_ptr() as *const i8,
            fd: 0,
        },
        Cg {
            path: b"/cg1/cg2/cg3\0".as_ptr() as *const i8,
            fd: 0,
        },
        Cg {
            path: b"/cg1/cg2/cg3/cg4\0".as_ptr() as *const i8,
            fd: 0,
        },
    ];
    let last_cg: i32 = ARRAY_SIZE!(cgs) as i32 - 1;
    let cg_nr: i32 = ARRAY_SIZE!(cgs) as i32;
    let mut link_upd_opts = DECLARE_LIBBPF_OPTS!(bpf_link_update_opts);
    let mut links: [*mut bpf_link; ARRAY_SIZE!(cgs)] = [ptr::null_mut(); ARRAY_SIZE!(cgs)];
    let mut tmp_link: *mut bpf_link;
    let mut prog_ids: [__u32; ARRAY_SIZE!(cgs)] = [0; ARRAY_SIZE!(cgs)];
    let mut prog_cnt: __u32 = 0;
    let mut attach_flags: __u32 = 0;
    let mut prog_id: __u32;
    let mut info: bpf_link_info = core::mem::zeroed();
    let mut i: i32 = 0;
    let mut err: i32;
    let prog_fd: i32;
    let mut detach_legacy: bool = false;

    skel = test_cgroup_link__open_and_load();
    if CHECK!(
        skel.is_null(),
        "skel_open_load",
        "failed to open/load skeleton\n"
    ) {
        return;
    }
    prog_fd = bpf_program__fd((*(*skel).progs).egress);

    'cleanup: {
        err = setup_cgroup_environment();
        if CHECK!(err != 0, "cg_init", "failed: %d\n", err) {
            break 'cleanup;
        }

        i = 0;
        while i < cg_nr {
            cgs[i as usize].fd = create_and_get_cgroup(cgs[i as usize].path);
            if !ASSERT_GE!(cgs[i as usize].fd, 0, "cg_create") {
                break 'cleanup;
            }
            i += 1;
        }

        err = join_cgroup(cgs[last_cg as usize].path);
        if CHECK!(err != 0, "cg_join", "fail: %d\n", err) {
            break 'cleanup;
        }

        i = 0;
        while i < cg_nr {
            links[i as usize] =
                bpf_program__attach_cgroup((*(*skel).progs).egress, cgs[i as usize].fd);
            if !ASSERT_OK_PTR!(links[i as usize], "cg_attach") {
                break 'cleanup;
            }
            i += 1;
        }

        ping_and_check(cg_nr, 0);

        /* query the number of attached progs and attach flags in root cg */
        err = bpf_prog_query(
            cgs[0].fd,
            BPF_CGROUP_INET_EGRESS,
            0,
            &mut attach_flags,
            ptr::null_mut(),
            &mut prog_cnt,
        );
        CHECK_FAIL!(err);
        CHECK_FAIL!(attach_flags != BPF_F_ALLOW_MULTI);
        if CHECK!(
            prog_cnt != 1,
            "effect_cnt",
            "exp %d, got %d\n",
            1,
            prog_cnt
        ) {
            break 'cleanup;
        }

        /* query the number of effective progs in last cg */
        err = bpf_prog_query(
            cgs[last_cg as usize].fd,
            BPF_CGROUP_INET_EGRESS,
            BPF_F_QUERY_EFFECTIVE,
            ptr::null_mut(),
            ptr::null_mut(),
            &mut prog_cnt,
        );
        CHECK_FAIL!(err);
        if CHECK!(
            prog_cnt != cg_nr as __u32,
            "effect_cnt",
            "exp %d, got %d\n",
            cg_nr,
            prog_cnt
        ) {
            break 'cleanup;
        }

        /* query the effective prog IDs in last cg */
        err = bpf_prog_query(
            cgs[last_cg as usize].fd,
            BPF_CGROUP_INET_EGRESS,
            BPF_F_QUERY_EFFECTIVE,
            ptr::null_mut(),
            prog_ids.as_mut_ptr(),
            &mut prog_cnt,
        );
        CHECK_FAIL!(err);
        if CHECK!(
            prog_cnt != cg_nr as __u32,
            "effect_cnt",
            "exp %d, got %d\n",
            cg_nr,
            prog_cnt
        ) {
            break 'cleanup;
        }
        i = 1;
        while i < prog_cnt as i32 {
            CHECK!(
                prog_ids[(i - 1) as usize] != prog_ids[i as usize],
                "prog_id_check",
                "idx %d, prev id %d, cur id %d\n",
                i,
                prog_ids[(i - 1) as usize],
                prog_ids[i as usize]
            );
            i += 1;
        }

        /* detach bottom program and ping again */
        bpf_link__destroy(links[last_cg as usize]);
        links[last_cg as usize] = ptr::null_mut();

        ping_and_check(cg_nr - 1, 0);

        /* mix in with non link-based multi-attachments */
        err = bpf_prog_attach(
            prog_fd,
            cgs[last_cg as usize].fd,
            BPF_CGROUP_INET_EGRESS,
            BPF_F_ALLOW_MULTI,
        );
        if CHECK!(err != 0, "cg_attach_legacy", "errno=%d\n", errno) {
            break 'cleanup;
        }
        detach_legacy = true;

        links[last_cg as usize] =
            bpf_program__attach_cgroup((*(*skel).progs).egress, cgs[last_cg as usize].fd);
        if !ASSERT_OK_PTR!(links[last_cg as usize], "cg_attach") {
            break 'cleanup;
        }

        ping_and_check(cg_nr + 1, 0);

        /* detach link */
        bpf_link__destroy(links[last_cg as usize]);
        links[last_cg as usize] = ptr::null_mut();

        /* detach legacy */
        err = bpf_prog_detach2(prog_fd, cgs[last_cg as usize].fd, BPF_CGROUP_INET_EGRESS);
        if CHECK!(err != 0, "cg_detach_legacy", "errno=%d\n", errno) {
            break 'cleanup;
        }
        detach_legacy = false;

        /* attach legacy exclusive prog attachment */
        err = bpf_prog_attach(prog_fd, cgs[last_cg as usize].fd, BPF_CGROUP_INET_EGRESS, 0);
        if CHECK!(err != 0, "cg_attach_exclusive", "errno=%d\n", errno) {
            break 'cleanup;
        }
        detach_legacy = true;

        /* attempt to mix in with multi-attach bpf_link */
        tmp_link = bpf_program__attach_cgroup((*(*skel).progs).egress, cgs[last_cg as usize].fd);
        if !ASSERT_ERR_PTR!(tmp_link, "cg_attach_fail") {
            bpf_link__destroy(tmp_link);
            break 'cleanup;
        }

        ping_and_check(cg_nr, 0);

        /* detach */
        err = bpf_prog_detach2(prog_fd, cgs[last_cg as usize].fd, BPF_CGROUP_INET_EGRESS);
        if CHECK!(err != 0, "cg_detach_legacy", "errno=%d\n", errno) {
            break 'cleanup;
        }
        detach_legacy = false;

        ping_and_check(cg_nr - 1, 0);

        /* attach back link-based one */
        links[last_cg as usize] =
            bpf_program__attach_cgroup((*(*skel).progs).egress, cgs[last_cg as usize].fd);
        if !ASSERT_OK_PTR!(links[last_cg as usize], "cg_attach") {
            break 'cleanup;
        }

        ping_and_check(cg_nr, 0);

        /* check legacy exclusive prog can't be attached */
        err = bpf_prog_attach(prog_fd, cgs[last_cg as usize].fd, BPF_CGROUP_INET_EGRESS, 0);
        if CHECK!(!err, "cg_attach_exclusive", "unexpected success") {
            bpf_prog_detach2(prog_fd, cgs[last_cg as usize].fd, BPF_CGROUP_INET_EGRESS);
            break 'cleanup;
        }

        /* replace BPF programs inside their links for all but first link */
        i = 1;
        while i < cg_nr {
            err = bpf_link__update_program(links[i as usize], (*(*skel).progs).egress_alt);
            if CHECK!(err != 0, "prog_upd", "link #%d\n", i) {
                break 'cleanup;
            }
            i += 1;
        }

        ping_and_check(1, cg_nr - 1);

        /* Attempt program update with wrong expected BPF program */
        link_upd_opts.old_prog_fd = bpf_program__fd((*(*skel).progs).egress_alt);
        link_upd_opts.flags = BPF_F_REPLACE;
        err = bpf_link_update(
            bpf_link__fd(links[0]),
            bpf_program__fd((*(*skel).progs).egress_alt),
            &mut link_upd_opts,
        );
        if CHECK!(
            err == 0 || errno != EPERM,
            "prog_cmpxchg1",
            "unexpectedly succeeded, err %d, errno %d\n",
            err,
            -errno
        ) {
            break 'cleanup;
        }

        /* Compare-exchange single link program from egress to egress_alt */
        link_upd_opts.old_prog_fd = bpf_program__fd((*(*skel).progs).egress);
        link_upd_opts.flags = BPF_F_REPLACE;
        err = bpf_link_update(
            bpf_link__fd(links[0]),
            bpf_program__fd((*(*skel).progs).egress_alt),
            &mut link_upd_opts,
        );
        if CHECK!(err != 0, "prog_cmpxchg2", "errno %d\n", -errno) {
            break 'cleanup;
        }

        /* ping */
        ping_and_check(0, cg_nr);

        /* close cgroup FDs before detaching links */
        i = 0;
        while i < cg_nr {
            if cgs[i as usize].fd > 0 {
                close(cgs[i as usize].fd);
                cgs[i as usize].fd = -1;
            }
            i += 1;
        }

        /* BPF programs should still get called */
        ping_and_check(0, cg_nr);

        prog_id = link_info_prog_id(links[0], &mut info);
        CHECK!(prog_id == 0, "link_info", "failed\n");
        CHECK!(
            info.cgroup.cgroup_id == 0,
            "cgroup_id",
            "unexpected %llu\n",
            info.cgroup.cgroup_id
        );

        err = bpf_link__detach(links[0]);
        if CHECK!(err != 0, "link_detach", "failed %d\n", err) {
            break 'cleanup;
        }

        /* cgroup_id should be zero in link_info */
        prog_id = link_info_prog_id(links[0], &mut info);
        CHECK!(prog_id == 0, "link_info", "failed\n");
        CHECK!(
            info.cgroup.cgroup_id != 0,
            "cgroup_id",
            "unexpected %llu\n",
            info.cgroup.cgroup_id
        );

        /* First BPF program shouldn't be called anymore */
        ping_and_check(0, cg_nr - 1);

        /* leave cgroup and remove them, don't detach programs */
        cleanup_cgroup_environment();

        /* BPF programs should have been auto-detached */
        ping_and_check(0, 0);
    }

    if detach_legacy {
        bpf_prog_detach2(
            prog_fd,
            cgs[last_cg as usize].fd,
            BPF_CGROUP_INET_EGRESS,
        );
    }

    i = 0;
    while i < cg_nr {
        bpf_link__destroy(links[i as usize]);
        i += 1;
    }
    test_cgroup_link__destroy(skel);

    i = 0;
    while i < cg_nr {
        if cgs[i as usize].fd > 0 {
            close(cgs[i as usize].fd);
        }
        i += 1;
    }
    cleanup_cgroup_environment();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
