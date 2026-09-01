// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Google LLC */

/* Dependencies from the original C file:
 * linux/bpf.h, unistd.h, sys/syscall.h, test_progs.h, cgroup_helpers.h,
 * and cgroup_skb_direct_packet_access.skel.h.
 */

const OLD_QUERY_SIZE: usize = offsetofend!(bpf_attr, query.prog_cnt);
const FULL_QUERY_SIZE: usize = offsetofend!(bpf_attr, query.revision);

unsafe fn test_query_size_boundaries() {
    let mut skel: *mut cgroup_skb_direct_packet_access;
    let mut link: *mut bpf_link = core::ptr::null_mut();
    let mut attr: bpf_attr;
    let mut cg_fd: i32 = -1;
    let mut err: i32;

    skel = cgroup_skb_direct_packet_access__open_and_load();
    if !ASSERT_OK_PTR!(skel, "skel_load") {
        return;
    }

    cg_fd = test__join_cgroup(c"/attr_size_cg".as_ptr());
    if !ASSERT_GE!(cg_fd, 0, "join_cgroup") {
        goto_cleanup_query_size_boundaries(skel, link, cg_fd);
        return;
    }

    link = bpf_program__attach_cgroup((*skel).progs.direct_packet_access, cg_fd);
    if !ASSERT_OK_PTR!(link, "cg_attach") {
        goto_cleanup_query_size_boundaries(skel, link, cg_fd);
        return;
    }

    attr = core::mem::zeroed();
    attr.query.target_fd = cg_fd as _;
    attr.query.attach_type = BPF_CGROUP_INET_INGRESS;
    attr.query.revision = 0xdeadbeefdeadbeef_u64;

    err = syscall(__NR_bpf, BPF_PROG_QUERY, &mut attr, OLD_QUERY_SIZE) as i32;
    if ASSERT_OK!(err, "query_old_size") {
        ASSERT_EQ!(attr.query.prog_cnt, 1, "prog_cnt_written_old");
        ASSERT_EQ!(
            attr.query.revision,
            0xdeadbeefdeadbeef_u64,
            "revision_not_written_old"
        );
    }

    attr = core::mem::zeroed();
    attr.query.target_fd = cg_fd as _;
    attr.query.attach_type = BPF_CGROUP_INET_INGRESS;

    err = syscall(__NR_bpf, BPF_PROG_QUERY, &mut attr, FULL_QUERY_SIZE) as i32;
    if !ASSERT_OK!(err, "query_full_size") {
        goto_cleanup_query_size_boundaries(skel, link, cg_fd);
        return;
    }

    ASSERT_EQ!(attr.query.prog_cnt, 1, "prog_cnt_written");
    ASSERT_GT!(attr.query.revision, 0, "revision_written");

    goto_cleanup_query_size_boundaries(skel, link, cg_fd);
}

unsafe fn goto_cleanup_query_size_boundaries(
    skel: *mut cgroup_skb_direct_packet_access,
    link: *mut bpf_link,
    cg_fd: i32,
) {
    if !link.is_null() {
        bpf_link__destroy(link);
    }
    if cg_fd >= 0 {
        close(cg_fd);
    }
    cgroup_skb_direct_packet_access__destroy(skel);
}

#[repr(C)]
struct bpf_map_info_fake {
    info: [__u8; offsetofend!(bpf_map_info, hash_size)],
    pad: __u32,
}

unsafe fn test_map_info_tail_zero() {
    LIBBPF_OPTS!(bpf_map_create_opts, map_opts);
    let mut info = bpf_map_info_fake {
        info: [0; offsetofend!(bpf_map_info, hash_size)],
        pad: 1,
    };
    let mut map_fd: i32;
    let mut err: i32;
    let mut info_len: __u32;

    map_fd = bpf_map_create(
        BPF_MAP_TYPE_ARRAY,
        c"arr".as_ptr(),
        core::mem::size_of::<i32>() as u32,
        1,
        1,
        &mut map_opts,
    );
    if !ASSERT_GE!(map_fd, 0, "bpf_map_create") {
        return;
    }

    info_len = core::mem::size_of_val(&info) as __u32;
    err = bpf_obj_get_info_by_fd(
        map_fd,
        &mut info as *mut bpf_map_info_fake as *mut core::ffi::c_void,
        &mut info_len,
    );
    ASSERT_EQ!(err, -E2BIG, "bpf_obj_get_info_by_fd");

    close(map_fd);
}

#[repr(C)]
struct bpf_prog_info_fake {
    info: [__u8; offsetofend!(bpf_prog_info, attach_btf_id)],
    pad: __u32,
}

unsafe fn test_prog_info_tail_zero() {
    LIBBPF_OPTS!(bpf_prog_load_opts, prog_opts);
    let mut insns: [bpf_insn; 2] = [
        BPF_MOV64_IMM!(BPF_REG_0, 0),
        BPF_EXIT_INSN!(),
    ];
    let mut info = bpf_prog_info_fake {
        info: [0; offsetofend!(bpf_prog_info, attach_btf_id)],
        pad: 1,
    };
    let mut prog_fd: i32;
    let mut err: i32;
    let mut info_len: __u32;

    prog_fd = bpf_prog_load(
        BPF_PROG_TYPE_SOCKET_FILTER,
        c"test_prog".as_ptr(),
        c"GPL".as_ptr(),
        insns.as_mut_ptr(),
        ARRAY_SIZE!(&insns),
        &mut prog_opts,
    );
    if !ASSERT_GE!(prog_fd, 0, "bpf_prog_load") {
        return;
    }

    info_len = core::mem::size_of_val(&info) as __u32;
    err = bpf_obj_get_info_by_fd(
        prog_fd,
        &mut info as *mut bpf_prog_info_fake as *mut core::ffi::c_void,
        &mut info_len,
    );
    ASSERT_EQ!(err, -E2BIG, "bpf_obj_get_info_by_fd");

    close(prog_fd);
}

pub unsafe fn test_bpf_attr_size() {
    if test__start_subtest(c"query_size_boundaries".as_ptr()) {
        test_query_size_boundaries();
    }
    if test__start_subtest(c"map_info_tail_zero".as_ptr()) {
        test_map_info_tail_zero();
    }
    if test__start_subtest(c"prog_info_tail_zero".as_ptr()) {
        test_prog_info_tail_zero();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
