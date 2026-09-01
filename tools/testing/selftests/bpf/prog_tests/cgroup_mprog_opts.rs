// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Meta Platforms, Inc. and affiliates. */
/* Depends on test_progs.h, cgroup_helpers.h, and cgroup_mprog.skel.h. */

use core::ffi::c_void;
use core::mem;
use core::ptr;

type __u32 = u32;

unsafe extern "C" {
    static BPF_CGROUP_GETSOCKOPT: i32;
    static BPF_F_ALLOW_MULTI: __u32;
    static BPF_F_BEFORE: __u32;
    static BPF_F_AFTER: __u32;
    static BPF_F_LINK: __u32;
    static BPF_F_PREORDER: __u32;
    static BPF_F_ID: __u32;
    static BPF_F_REPLACE: __u32;
    static EINVAL: i32;
    static ENOENT: i32;

    fn bpf_prog_query(
        cg: i32,
        atype: i32,
        query_flags: i32,
        attach_flags: *mut __u32,
        prog_ids: *mut __u32,
        prog_cnt: *mut __u32,
    ) -> i32;
    fn bpf_prog_attach_opts(
        prog_fd: __u32,
        target_fd: i32,
        attach_type: i32,
        opts: *mut bpf_prog_attach_opts,
    ) -> i32;
    fn bpf_prog_detach_opts(
        prog_fd: __u32,
        target_fd: i32,
        attach_type: i32,
        opts: *mut bpf_prog_detach_opts,
    ) -> i32;
    fn bpf_prog_query_opts(cg: i32, atype: i32, opts: *mut bpf_prog_query_opts) -> i32;
    fn bpf_program__fd(prog: *mut bpf_program) -> i32;
    fn bpf_program__attach_cgroup_opts(
        prog: *mut bpf_program,
        cgroup_fd: i32,
        opts: *mut bpf_cgroup_opts,
    ) -> *mut bpf_link;
    fn bpf_link__fd(link: *mut bpf_link) -> i32;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn cgroup_mprog__open_and_load() -> *mut cgroup_mprog;
    fn cgroup_mprog__destroy(skel: *mut cgroup_mprog);
    fn test__join_cgroup(path: *const u8) -> i32;
    fn test__start_subtest(name: *const u8) -> bool;
    fn id_from_prog_fd(fd: __u32) -> __u32;
    fn id_from_link_fd(fd: i32) -> __u32;
    fn close(fd: i32) -> i32;
}

#[repr(C)]
struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
struct cgroup_mprog {
    progs: cgroup_mprog_progs,
}

#[repr(C)]
struct cgroup_mprog_progs {
    getsockopt_1: *mut bpf_program,
    getsockopt_2: *mut bpf_program,
    getsockopt_3: *mut bpf_program,
    getsockopt_4: *mut bpf_program,
}

#[repr(C)]
struct bpf_prog_attach_opts {
    flags: __u32,
    relative_fd: __u32,
    relative_id: __u32,
    replace_prog_fd: __u32,
    expected_revision: __u32,
}

#[repr(C)]
struct bpf_prog_detach_opts {
    expected_revision: __u32,
}

#[repr(C)]
struct bpf_prog_query_opts {
    prog_ids: *mut __u32,
    link_ids: *mut __u32,
    count: __u32,
    revision: __u32,
}

#[repr(C)]
struct bpf_cgroup_opts {
    flags: __u32,
    relative_fd: __u32,
    relative_id: __u32,
    expected_revision: __u32,
}

unsafe fn assert_mprog_count(cg: i32, atype: i32, expected: i32) {
    let mut count: __u32 = 0;
    let mut attach_flags: __u32 = 0;
    let err: i32;

    err = bpf_prog_query(cg, atype, 0, &mut attach_flags, ptr::null_mut(), &mut count);
    ASSERT_EQ!(count, expected as __u32, "count");
    ASSERT_EQ!(err, 0, "prog_query");
}

unsafe fn test_prog_attach_detach(atype: i32) {
    let mut opta: bpf_prog_attach_opts = mem::zeroed();
    let mut optd: bpf_prog_detach_opts = mem::zeroed();
    let mut optq: bpf_prog_query_opts = mem::zeroed();
    let fd1: __u32;
    let fd2: __u32;
    let fd3: __u32;
    let fd4: __u32;
    let id1: __u32;
    let id2: __u32;
    let id3: __u32;
    let id4: __u32;
    let skel: *mut cgroup_mprog;
    let mut prog_ids: [__u32; 10] = [0; 10];
    let cg: i32;
    let mut err: i32;
    let mut cleanup_from = 0;

    cg = test__join_cgroup(c"/prog_attach_detach".as_ptr() as *const u8);
    if !ASSERT_GE!(cg, 0, "join_cgroup /prog_attach_detach") {
        return;
    }

    skel = cgroup_mprog__open_and_load();
    if !ASSERT_OK_PTR!(skel, "skel_load") {
        cleanup_from = 0;
    } else {
        fd1 = bpf_program__fd((*skel).progs.getsockopt_1) as __u32;
        fd2 = bpf_program__fd((*skel).progs.getsockopt_2) as __u32;
        fd3 = bpf_program__fd((*skel).progs.getsockopt_3) as __u32;
        fd4 = bpf_program__fd((*skel).progs.getsockopt_4) as __u32;

        id1 = id_from_prog_fd(fd1);
        id2 = id_from_prog_fd(fd2);
        id3 = id_from_prog_fd(fd3);
        id4 = id_from_prog_fd(fd4);

        assert_mprog_count(cg, atype, 0);

        opta = bpf_prog_attach_opts {
            flags: BPF_F_ALLOW_MULTI | BPF_F_BEFORE | BPF_F_AFTER,
            expected_revision: 1,
            ..mem::zeroed()
        };

        /* ordering: [fd1] */
        err = bpf_prog_attach_opts(fd1, cg, atype, &mut opta);
        if !ASSERT_EQ!(err, 0, "prog_attach") {
            cleanup_from = 0;
        } else {
            assert_mprog_count(cg, atype, 1);

            opta = bpf_prog_attach_opts {
                flags: BPF_F_ALLOW_MULTI | BPF_F_BEFORE,
                expected_revision: 2,
                ..mem::zeroed()
            };

            /* ordering: [fd2, fd1] */
            err = bpf_prog_attach_opts(fd2, cg, atype, &mut opta);
            if !ASSERT_EQ!(err, 0, "prog_attach") {
                cleanup_from = 1;
            } else {
                assert_mprog_count(cg, atype, 2);

                opta = bpf_prog_attach_opts {
                    flags: BPF_F_ALLOW_MULTI | BPF_F_AFTER,
                    relative_fd: fd2,
                    expected_revision: 3,
                    ..mem::zeroed()
                };

                /* ordering: [fd2, fd3, fd1] */
                err = bpf_prog_attach_opts(fd3, cg, atype, &mut opta);
                if !ASSERT_EQ!(err, 0, "prog_attach") {
                    cleanup_from = 2;
                } else {
                    assert_mprog_count(cg, atype, 3);

                    opta = bpf_prog_attach_opts {
                        flags: BPF_F_ALLOW_MULTI,
                        expected_revision: 4,
                        ..mem::zeroed()
                    };

                    /* ordering: [fd2, fd3, fd1, fd4] */
                    err = bpf_prog_attach_opts(fd4, cg, atype, &mut opta);
                    if !ASSERT_EQ!(err, 0, "prog_attach") {
                        cleanup_from = 3;
                    } else {
                        assert_mprog_count(cg, atype, 4);

                        /* retrieve optq.prog_cnt */
                        err = bpf_prog_query_opts(cg, atype, &mut optq);
                        if !ASSERT_OK!(err, "prog_query") {
                            cleanup_from = 4;
                        } else {
                            /* optq.prog_cnt will be used in below query */
                            prog_ids = [0; 10];
                            optq.prog_ids = prog_ids.as_mut_ptr();
                            err = bpf_prog_query_opts(cg, atype, &mut optq);
                            if !ASSERT_OK!(err, "prog_query") {
                                cleanup_from = 4;
                            } else {
                                ASSERT_EQ!(optq.count, 4, "count");
                                ASSERT_EQ!(optq.revision, 5, "revision");
                                ASSERT_EQ!(*optq.prog_ids.add(0), id2, "prog_ids[0]");
                                ASSERT_EQ!(*optq.prog_ids.add(1), id3, "prog_ids[1]");
                                ASSERT_EQ!(*optq.prog_ids.add(2), id1, "prog_ids[2]");
                                ASSERT_EQ!(*optq.prog_ids.add(3), id4, "prog_ids[3]");
                                ASSERT_EQ!(*optq.prog_ids.add(4), 0, "prog_ids[4]");
                                ASSERT_EQ!(optq.link_ids, ptr::null_mut::<__u32>(), "link_ids");
                                cleanup_from = 4;
                            }
                        }
                    }
                }
            }
        }
    }

    if cleanup_from >= 4 {
        optd.expected_revision = 5;
        err = bpf_prog_detach_opts(fd4, cg, atype, &mut optd);
        ASSERT_OK!(err, "prog_detach");
        assert_mprog_count(cg, atype, 3);
    }
    if cleanup_from >= 3 {
        optd = mem::zeroed();
        err = bpf_prog_detach_opts(fd3, cg, atype, &mut optd);
        ASSERT_OK!(err, "prog_detach");
        assert_mprog_count(cg, atype, 2);

        /* Check revision after two detach operations */
        err = bpf_prog_query_opts(cg, atype, &mut optq);
        ASSERT_OK!(err, "prog_query");
        ASSERT_EQ!(optq.revision, 7, "revision");
    }
    if cleanup_from >= 2 {
        err = bpf_prog_detach_opts(fd2, cg, atype, &mut optd);
        ASSERT_OK!(err, "prog_detach");
        assert_mprog_count(cg, atype, 1);
    }
    if cleanup_from >= 1 {
        err = bpf_prog_detach_opts(fd1, cg, atype, &mut optd);
        ASSERT_OK!(err, "prog_detach");
        assert_mprog_count(cg, atype, 0);
    }
    cgroup_mprog__destroy(skel);
    close(cg);
}

unsafe fn test_link_attach_detach(atype: i32) {
    let mut opta: bpf_cgroup_opts = mem::zeroed();
    let mut optq: bpf_prog_query_opts = mem::zeroed();
    let mut link1: *mut bpf_link = ptr::null_mut();
    let mut link2: *mut bpf_link = ptr::null_mut();
    let mut link3: *mut bpf_link = ptr::null_mut();
    let mut link4: *mut bpf_link = ptr::null_mut();
    let fd1: __u32;
    let fd2: __u32;
    let fd3: __u32;
    let fd4: __u32;
    let id1: __u32;
    let id2: __u32;
    let id3: __u32;
    let id4: __u32;
    let skel: *mut cgroup_mprog;
    let mut prog_ids: [__u32; 10] = [0; 10];
    let cg: i32;
    let mut err: i32;
    let mut cleanup_from = 0;

    cg = test__join_cgroup(c"/link_attach_detach".as_ptr() as *const u8);
    if !ASSERT_GE!(cg, 0, "join_cgroup /link_attach_detach") {
        return;
    }

    skel = cgroup_mprog__open_and_load();
    if !ASSERT_OK_PTR!(skel, "skel_load") {
        cleanup_from = 0;
    } else {
        fd1 = bpf_program__fd((*skel).progs.getsockopt_1) as __u32;
        fd2 = bpf_program__fd((*skel).progs.getsockopt_2) as __u32;
        fd3 = bpf_program__fd((*skel).progs.getsockopt_3) as __u32;
        fd4 = bpf_program__fd((*skel).progs.getsockopt_4) as __u32;

        id1 = id_from_prog_fd(fd1);
        id2 = id_from_prog_fd(fd2);
        id3 = id_from_prog_fd(fd3);
        id4 = id_from_prog_fd(fd4);

        assert_mprog_count(cg, atype, 0);

        opta = bpf_cgroup_opts {
            expected_revision: 1,
            ..mem::zeroed()
        };

        /* ordering: [fd1] */
        link1 = bpf_program__attach_cgroup_opts((*skel).progs.getsockopt_1, cg, &mut opta);
        if !ASSERT_OK_PTR!(link1, "link_attach") {
            cleanup_from = 0;
        } else {
            assert_mprog_count(cg, atype, 1);

            opta = bpf_cgroup_opts {
                flags: BPF_F_BEFORE | BPF_F_LINK,
                relative_id: id_from_link_fd(bpf_link__fd(link1)),
                expected_revision: 2,
                ..mem::zeroed()
            };

            /* ordering: [fd2, fd1] */
            link2 = bpf_program__attach_cgroup_opts((*skel).progs.getsockopt_2, cg, &mut opta);
            if !ASSERT_OK_PTR!(link2, "link_attach") {
                cleanup_from = 1;
            } else {
                assert_mprog_count(cg, atype, 2);

                opta = bpf_cgroup_opts {
                    flags: BPF_F_AFTER | BPF_F_LINK,
                    relative_fd: bpf_link__fd(link2) as __u32,
                    expected_revision: 3,
                    ..mem::zeroed()
                };

                /* ordering: [fd2, fd3, fd1] */
                link3 = bpf_program__attach_cgroup_opts((*skel).progs.getsockopt_3, cg, &mut opta);
                if !ASSERT_OK_PTR!(link3, "link_attach") {
                    cleanup_from = 2;
                } else {
                    assert_mprog_count(cg, atype, 3);

                    opta = bpf_cgroup_opts {
                        expected_revision: 4,
                        ..mem::zeroed()
                    };

                    /* ordering: [fd2, fd3, fd1, fd4] */
                    link4 = bpf_program__attach_cgroup_opts((*skel).progs.getsockopt_4, cg, &mut opta);
                    if !ASSERT_OK_PTR!(link4, "link_attach") {
                        cleanup_from = 3;
                    } else {
                        assert_mprog_count(cg, atype, 4);

                        /* retrieve optq.prog_cnt */
                        err = bpf_prog_query_opts(cg, atype, &mut optq);
                        if !ASSERT_OK!(err, "prog_query") {
                            cleanup_from = 4;
                        } else {
                            /* optq.prog_cnt will be used in below query */
                            prog_ids = [0; 10];
                            optq.prog_ids = prog_ids.as_mut_ptr();
                            err = bpf_prog_query_opts(cg, atype, &mut optq);
                            if !ASSERT_OK!(err, "prog_query") {
                                cleanup_from = 4;
                            } else {
                                ASSERT_EQ!(optq.count, 4, "count");
                                ASSERT_EQ!(optq.revision, 5, "revision");
                                ASSERT_EQ!(*optq.prog_ids.add(0), id2, "prog_ids[0]");
                                ASSERT_EQ!(*optq.prog_ids.add(1), id3, "prog_ids[1]");
                                ASSERT_EQ!(*optq.prog_ids.add(2), id1, "prog_ids[2]");
                                ASSERT_EQ!(*optq.prog_ids.add(3), id4, "prog_ids[3]");
                                ASSERT_EQ!(*optq.prog_ids.add(4), 0, "prog_ids[4]");
                                ASSERT_EQ!(optq.link_ids, ptr::null_mut::<__u32>(), "link_ids");
                                cleanup_from = 4;
                            }
                        }
                    }
                }
            }
        }
    }

    if cleanup_from >= 4 {
        bpf_link__destroy(link4);
        assert_mprog_count(cg, atype, 3);
    }
    if cleanup_from >= 3 {
        bpf_link__destroy(link3);
        assert_mprog_count(cg, atype, 2);

        /* Check revision after two detach operations */
        err = bpf_prog_query_opts(cg, atype, &mut optq);
        ASSERT_OK!(err, "prog_query");
        ASSERT_EQ!(optq.revision, 7, "revision");
    }
    if cleanup_from >= 2 {
        bpf_link__destroy(link2);
        assert_mprog_count(cg, atype, 1);
    }
    if cleanup_from >= 1 {
        bpf_link__destroy(link1);
        assert_mprog_count(cg, atype, 0);
    }
    cgroup_mprog__destroy(skel);
    close(cg);
}

unsafe fn test_preorder_prog_attach_detach(atype: i32) {
    let mut opta: bpf_prog_attach_opts = mem::zeroed();
    let mut optd: bpf_prog_detach_opts = mem::zeroed();
    let fd1: __u32;
    let fd2: __u32;
    let fd3: __u32;
    let fd4: __u32;
    let skel: *mut cgroup_mprog;
    let cg: i32;
    let mut err: i32;
    let mut cleanup_from = 0;

    cg = test__join_cgroup(c"/preorder_prog_attach_detach".as_ptr() as *const u8);
    if !ASSERT_GE!(cg, 0, "join_cgroup /preorder_prog_attach_detach") {
        return;
    }

    skel = cgroup_mprog__open_and_load();
    if !ASSERT_OK_PTR!(skel, "skel_load") {
        cleanup_from = 0;
    } else {
        fd1 = bpf_program__fd((*skel).progs.getsockopt_1) as __u32;
        fd2 = bpf_program__fd((*skel).progs.getsockopt_2) as __u32;
        fd3 = bpf_program__fd((*skel).progs.getsockopt_3) as __u32;
        fd4 = bpf_program__fd((*skel).progs.getsockopt_4) as __u32;

        assert_mprog_count(cg, atype, 0);

        opta = bpf_prog_attach_opts {
            flags: BPF_F_ALLOW_MULTI,
            expected_revision: 1,
            ..mem::zeroed()
        };

        /* ordering: [fd1] */
        err = bpf_prog_attach_opts(fd1, cg, atype, &mut opta);
        if !ASSERT_EQ!(err, 0, "prog_attach") {
            cleanup_from = 0;
        } else {
            assert_mprog_count(cg, atype, 1);

            opta = bpf_prog_attach_opts {
                flags: BPF_F_ALLOW_MULTI | BPF_F_PREORDER,
                expected_revision: 2,
                ..mem::zeroed()
            };

            /* ordering: [fd1, fd2] */
            err = bpf_prog_attach_opts(fd2, cg, atype, &mut opta);
            if !ASSERT_EQ!(err, 0, "prog_attach") {
                cleanup_from = 1;
            } else {
                assert_mprog_count(cg, atype, 2);

                opta = bpf_prog_attach_opts {
                    flags: BPF_F_ALLOW_MULTI | BPF_F_AFTER,
                    relative_fd: fd2,
                    expected_revision: 3,
                    ..mem::zeroed()
                };

                err = bpf_prog_attach_opts(fd3, cg, atype, &mut opta);
                if !ASSERT_EQ!(err, -EINVAL, "prog_attach") {
                    cleanup_from = 2;
                } else {
                    assert_mprog_count(cg, atype, 2);

                    opta = bpf_prog_attach_opts {
                        flags: BPF_F_ALLOW_MULTI | BPF_F_AFTER | BPF_F_PREORDER,
                        relative_fd: fd2,
                        expected_revision: 3,
                        ..mem::zeroed()
                    };

                    /* ordering: [fd1, fd2, fd3] */
                    err = bpf_prog_attach_opts(fd3, cg, atype, &mut opta);
                    if !ASSERT_EQ!(err, 0, "prog_attach") {
                        cleanup_from = 2;
                    } else {
                        assert_mprog_count(cg, atype, 3);

                        opta = bpf_prog_attach_opts {
                            flags: BPF_F_ALLOW_MULTI,
                            expected_revision: 4,
                            ..mem::zeroed()
                        };

                        /* ordering: [fd2, fd3, fd1, fd4] */
                        err = bpf_prog_attach_opts(fd4, cg, atype, &mut opta);
                        if !ASSERT_EQ!(err, 0, "prog_attach") {
                            cleanup_from = 3;
                        } else {
                            assert_mprog_count(cg, atype, 4);

                            err = bpf_prog_detach_opts(fd4, cg, atype, &mut optd);
                            ASSERT_OK!(err, "prog_detach");
                            assert_mprog_count(cg, atype, 3);
                            cleanup_from = 3;
                        }
                    }
                }
            }
        }
    }

    if cleanup_from >= 3 {
        err = bpf_prog_detach_opts(fd3, cg, atype, &mut optd);
        ASSERT_OK!(err, "prog_detach");
        assert_mprog_count(cg, atype, 2);
    }
    if cleanup_from >= 2 {
        err = bpf_prog_detach_opts(fd2, cg, atype, &mut optd);
        ASSERT_OK!(err, "prog_detach");
        assert_mprog_count(cg, atype, 1);
    }
    if cleanup_from >= 1 {
        err = bpf_prog_detach_opts(fd1, cg, atype, &mut optd);
        ASSERT_OK!(err, "prog_detach");
        assert_mprog_count(cg, atype, 0);
    }
    cgroup_mprog__destroy(skel);
    close(cg);
}

unsafe fn test_preorder_link_attach_detach(atype: i32) {
    let mut opta: bpf_cgroup_opts = mem::zeroed();
    let mut link1: *mut bpf_link = ptr::null_mut();
    let mut link2: *mut bpf_link = ptr::null_mut();
    let mut link3: *mut bpf_link = ptr::null_mut();
    let mut link4: *mut bpf_link = ptr::null_mut();
    let skel: *mut cgroup_mprog;
    let fd2: __u32;
    let cg: i32;
    let mut cleanup_from = 0;

    cg = test__join_cgroup(c"/preorder_link_attach_detach".as_ptr() as *const u8);
    if !ASSERT_GE!(cg, 0, "join_cgroup /preorder_link_attach_detach") {
        return;
    }

    skel = cgroup_mprog__open_and_load();
    if !ASSERT_OK_PTR!(skel, "skel_load") {
        cleanup_from = 0;
    } else {
        fd2 = bpf_program__fd((*skel).progs.getsockopt_2) as __u32;

        assert_mprog_count(cg, atype, 0);

        opta = bpf_cgroup_opts {
            expected_revision: 1,
            ..mem::zeroed()
        };

        /* ordering: [fd1] */
        link1 = bpf_program__attach_cgroup_opts((*skel).progs.getsockopt_1, cg, &mut opta);
        if !ASSERT_OK_PTR!(link1, "link_attach") {
            cleanup_from = 0;
        } else {
            assert_mprog_count(cg, atype, 1);

            opta = bpf_cgroup_opts {
                flags: BPF_F_PREORDER,
                expected_revision: 2,
                ..mem::zeroed()
            };

            /* ordering: [fd1, fd2] */
            link2 = bpf_program__attach_cgroup_opts((*skel).progs.getsockopt_2, cg, &mut opta);
            if !ASSERT_OK_PTR!(link2, "link_attach") {
                cleanup_from = 1;
            } else {
                assert_mprog_count(cg, atype, 2);

                opta = bpf_cgroup_opts {
                    flags: BPF_F_AFTER,
                    relative_fd: fd2,
                    expected_revision: 3,
                    ..mem::zeroed()
                };

                link3 = bpf_program__attach_cgroup_opts((*skel).progs.getsockopt_3, cg, &mut opta);
                if !ASSERT_ERR_PTR!(link3, "link_attach") {
                    cleanup_from = 2;
                } else {
                    assert_mprog_count(cg, atype, 2);

                    opta = bpf_cgroup_opts {
                        flags: BPF_F_AFTER | BPF_F_PREORDER | BPF_F_LINK,
                        relative_fd: bpf_link__fd(link2) as __u32,
                        expected_revision: 3,
                        ..mem::zeroed()
                    };

                    /* ordering: [fd1, fd2, fd3] */
                    link3 = bpf_program__attach_cgroup_opts((*skel).progs.getsockopt_3, cg, &mut opta);
                    if !ASSERT_OK_PTR!(link3, "link_attach") {
                        cleanup_from = 2;
                    } else {
                        assert_mprog_count(cg, atype, 3);

                        opta = bpf_cgroup_opts {
                            expected_revision: 4,
                            ..mem::zeroed()
                        };

                        /* ordering: [fd2, fd3, fd1, fd4] */
                        link4 = bpf_program__attach_cgroup_opts((*skel).progs.getsockopt_4, cg, &mut opta);
                        if !ASSERT_OK_PTR!(link4, "prog_attach") {
                            cleanup_from = 3;
                        } else {
                            assert_mprog_count(cg, atype, 4);

                            bpf_link__destroy(link4);
                            assert_mprog_count(cg, atype, 3);
                            cleanup_from = 3;
                        }
                    }
                }
            }
        }
    }

    if cleanup_from >= 3 {
        bpf_link__destroy(link3);
        assert_mprog_count(cg, atype, 2);
    }
    if cleanup_from >= 2 {
        bpf_link__destroy(link2);
        assert_mprog_count(cg, atype, 1);
    }
    if cleanup_from >= 1 {
        bpf_link__destroy(link1);
        assert_mprog_count(cg, atype, 0);
    }
    cgroup_mprog__destroy(skel);
    close(cg);
}

unsafe fn test_invalid_attach_detach(atype: i32) {
    let mut opta: bpf_prog_attach_opts = mem::zeroed();
    let fd1: __u32;
    let fd2: __u32;
    let id2: __u32;
    let skel: *mut cgroup_mprog;
    let cg: i32;
    let mut err: i32;

    cg = test__join_cgroup(c"/invalid_attach_detach".as_ptr() as *const u8);
    if !ASSERT_GE!(cg, 0, "join_cgroup /invalid_attach_detach") {
        return;
    }

    skel = cgroup_mprog__open_and_load();
    if !ASSERT_OK_PTR!(skel, "skel_load") {
        cgroup_mprog__destroy(skel);
        close(cg);
        return;
    }

    fd1 = bpf_program__fd((*skel).progs.getsockopt_1) as __u32;
    fd2 = bpf_program__fd((*skel).progs.getsockopt_2) as __u32;

    id2 = id_from_prog_fd(fd2);

    assert_mprog_count(cg, atype, 0);

    opta = bpf_prog_attach_opts {
        flags: BPF_F_ALLOW_MULTI | BPF_F_BEFORE | BPF_F_AFTER,
        relative_id: id2,
        ..mem::zeroed()
    };

    err = bpf_prog_attach_opts(fd1, cg, atype, &mut opta);
    ASSERT_EQ!(err, -EINVAL, "prog_attach");
    assert_mprog_count(cg, atype, 0);

    opta = bpf_prog_attach_opts {
        flags: BPF_F_ALLOW_MULTI | BPF_F_BEFORE | BPF_F_ID,
        ..mem::zeroed()
    };

    err = bpf_prog_attach_opts(fd1, cg, atype, &mut opta);
    ASSERT_EQ!(err, -ENOENT, "prog_attach");
    assert_mprog_count(cg, atype, 0);

    opta = bpf_prog_attach_opts {
        flags: BPF_F_ALLOW_MULTI | BPF_F_AFTER | BPF_F_ID,
        ..mem::zeroed()
    };

    err = bpf_prog_attach_opts(fd1, cg, atype, &mut opta);
    ASSERT_EQ!(err, -ENOENT, "prog_attach");
    assert_mprog_count(cg, atype, 0);

    opta = bpf_prog_attach_opts {
        flags: BPF_F_ALLOW_MULTI | BPF_F_BEFORE | BPF_F_AFTER,
        relative_id: id2,
        ..mem::zeroed()
    };

    err = bpf_prog_attach_opts(fd1, cg, atype, &mut opta);
    ASSERT_EQ!(err, -EINVAL, "prog_attach");
    assert_mprog_count(cg, atype, 0);

    opta = bpf_prog_attach_opts {
        flags: BPF_F_ALLOW_MULTI | BPF_F_LINK,
        relative_id: id2,
        ..mem::zeroed()
    };

    err = bpf_prog_attach_opts(fd1, cg, atype, &mut opta);
    ASSERT_EQ!(err, -EINVAL, "prog_attach");
    assert_mprog_count(cg, atype, 0);

    opta = bpf_prog_attach_opts {
        flags: BPF_F_ALLOW_MULTI,
        relative_id: id2,
        ..mem::zeroed()
    };

    err = bpf_prog_attach_opts(fd1, cg, atype, &mut opta);
    ASSERT_EQ!(err, -EINVAL, "prog_attach");
    assert_mprog_count(cg, atype, 0);

    opta = bpf_prog_attach_opts {
        flags: BPF_F_ALLOW_MULTI | BPF_F_BEFORE,
        relative_fd: fd1,
        ..mem::zeroed()
    };

    err = bpf_prog_attach_opts(fd1, cg, atype, &mut opta);
    ASSERT_EQ!(err, -ENOENT, "prog_attach");
    assert_mprog_count(cg, atype, 0);

    opta = bpf_prog_attach_opts {
        flags: BPF_F_ALLOW_MULTI | BPF_F_AFTER,
        relative_fd: fd1,
        ..mem::zeroed()
    };

    err = bpf_prog_attach_opts(fd1, cg, atype, &mut opta);
    ASSERT_EQ!(err, -ENOENT, "prog_attach");
    assert_mprog_count(cg, atype, 0);

    opta = bpf_prog_attach_opts {
        flags: BPF_F_ALLOW_MULTI,
        ..mem::zeroed()
    };

    err = bpf_prog_attach_opts(fd1, cg, atype, &mut opta);
    if !ASSERT_EQ!(err, 0, "prog_attach") {
        cgroup_mprog__destroy(skel);
        close(cg);
        return;
    }
    assert_mprog_count(cg, atype, 1);

    opta = bpf_prog_attach_opts {
        flags: BPF_F_ALLOW_MULTI | BPF_F_AFTER,
        ..mem::zeroed()
    };

    err = bpf_prog_attach_opts(fd1, cg, atype, &mut opta);
    ASSERT_EQ!(err, -EINVAL, "prog_attach");
    assert_mprog_count(cg, atype, 1);

    opta = bpf_prog_attach_opts {
        flags: BPF_F_ALLOW_MULTI | BPF_F_REPLACE | BPF_F_AFTER,
        replace_prog_fd: fd1,
        ..mem::zeroed()
    };

    err = bpf_prog_attach_opts(fd1, cg, atype, &mut opta);
    ASSERT_EQ!(err, -EINVAL, "prog_attach");
    assert_mprog_count(cg, atype, 1);

    cgroup_mprog__destroy(skel);
    close(cg);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_cgroup_mprog_opts() {
    if test__start_subtest(c"prog_attach_detach".as_ptr() as *const u8) {
        test_prog_attach_detach(BPF_CGROUP_GETSOCKOPT);
    }
    if test__start_subtest(c"link_attach_detach".as_ptr() as *const u8) {
        test_link_attach_detach(BPF_CGROUP_GETSOCKOPT);
    }
    if test__start_subtest(c"preorder_prog_attach_detach".as_ptr() as *const u8) {
        test_preorder_prog_attach_detach(BPF_CGROUP_GETSOCKOPT);
    }
    if test__start_subtest(c"preorder_link_attach_detach".as_ptr() as *const u8) {
        test_preorder_link_attach_detach(BPF_CGROUP_GETSOCKOPT);
    }
    if test__start_subtest(c"invalid_attach_detach".as_ptr() as *const u8) {
        test_invalid_attach_detach(BPF_CGROUP_GETSOCKOPT);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
