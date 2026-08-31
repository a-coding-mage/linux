// SPDX-License-Identifier: GPL-2.0

// Translated from testing/selftests/bpf/prog_tests/cgroup_attach_multi.c.
// External constants, helper macros, types, and functions are supplied by the
// surrounding selftest/libbpf bindings.

const PING_CMD: *const ::std::os::raw::c_char = b"ping -q -c1 -w1 127.0.0.1 > /dev/null\0".as_ptr() as *const _;

static mut bpf_log_buf: [::std::os::raw::c_char; BPF_LOG_BUF_SIZE] = [0; BPF_LOG_BUF_SIZE];

static mut map_fd: ::std::os::raw::c_int = -1;

unsafe fn prog_load_cnt(verdict: ::std::os::raw::c_int, val: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
    let cgroup_storage_fd: ::std::os::raw::c_int;
    let percpu_cgroup_storage_fd: ::std::os::raw::c_int;

    if map_fd < 0 {
        map_fd = bpf_map_create(BPF_MAP_TYPE_ARRAY, ::std::ptr::null(), 4, 8, 1, ::std::ptr::null());
    }
    if map_fd < 0 {
        printf(
            b"failed to create map '%s'\n\0".as_ptr() as *const _,
            strerror(errno),
        );
        return -1;
    }

    cgroup_storage_fd = bpf_map_create(
        BPF_MAP_TYPE_CGROUP_STORAGE,
        ::std::ptr::null(),
        ::std::mem::size_of::<bpf_cgroup_storage_key>() as _,
        8,
        0,
        ::std::ptr::null(),
    );
    if cgroup_storage_fd < 0 {
        printf(
            b"failed to create map '%s'\n\0".as_ptr() as *const _,
            strerror(errno),
        );
        return -1;
    }

    percpu_cgroup_storage_fd = bpf_map_create(
        BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE,
        ::std::ptr::null(),
        ::std::mem::size_of::<bpf_cgroup_storage_key>() as _,
        8,
        0,
        ::std::ptr::null(),
    );
    if percpu_cgroup_storage_fd < 0 {
        printf(
            b"failed to create map '%s'\n\0".as_ptr() as *const _,
            strerror(errno),
        );
        return -1;
    }

    let mut prog: [bpf_insn; 23] = [
        BPF_MOV32_IMM(BPF_REG_0, 0),
        BPF_STX_MEM(BPF_W, BPF_REG_10, BPF_REG_0, -4), /* *(u32 *)(fp - 4) = r0 */
        BPF_MOV64_REG(BPF_REG_2, BPF_REG_10),
        BPF_ALU64_IMM(BPF_ADD, BPF_REG_2, -4), /* r2 = fp - 4 */
        BPF_LD_MAP_FD(BPF_REG_1, map_fd),
        BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 0, 0, BPF_FUNC_map_lookup_elem),
        BPF_JMP_IMM(BPF_JEQ, BPF_REG_0, 0, 2),
        BPF_MOV64_IMM(BPF_REG_1, val), /* r1 = 1 */
        BPF_ATOMIC_OP(BPF_DW, BPF_ADD, BPF_REG_0, BPF_REG_1, 0),

        BPF_LD_MAP_FD(BPF_REG_1, cgroup_storage_fd),
        BPF_MOV64_IMM(BPF_REG_2, 0),
        BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 0, 0, BPF_FUNC_get_local_storage),
        BPF_MOV64_IMM(BPF_REG_1, val),
        BPF_ATOMIC_OP(BPF_W, BPF_ADD, BPF_REG_0, BPF_REG_1, 0),

        BPF_LD_MAP_FD(BPF_REG_1, percpu_cgroup_storage_fd),
        BPF_MOV64_IMM(BPF_REG_2, 0),
        BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 0, 0, BPF_FUNC_get_local_storage),
        BPF_LDX_MEM(BPF_W, BPF_REG_3, BPF_REG_0, 0),
        BPF_ALU64_IMM(BPF_ADD, BPF_REG_3, 0x1),
        BPF_STX_MEM(BPF_W, BPF_REG_0, BPF_REG_3, 0),

        BPF_MOV64_IMM(BPF_REG_0, verdict), /* r0 = verdict */
        BPF_EXIT_INSN(),
    ];
    let insns_cnt: usize = prog.len();
    let ret: ::std::os::raw::c_int;

    ret = bpf_test_load_program(
        BPF_PROG_TYPE_CGROUP_SKB,
        prog.as_mut_ptr(),
        insns_cnt,
        b"GPL\0".as_ptr() as *const _,
        0,
        bpf_log_buf.as_mut_ptr(),
        BPF_LOG_BUF_SIZE,
    );

    close(cgroup_storage_fd);
    ret
}

pub unsafe fn serial_test_cgroup_attach_multi() {
    let mut prog_ids: [__u32; 4] = [0; 4];
    let mut prog_cnt: __u32 = 0;
    let mut attach_flags: __u32 = 0;
    let mut saved_prog_id: __u32;
    let mut cg1: ::std::os::raw::c_int = 0;
    let mut cg2: ::std::os::raw::c_int = 0;
    let mut cg3: ::std::os::raw::c_int = 0;
    let mut cg4: ::std::os::raw::c_int = 0;
    let mut cg5: ::std::os::raw::c_int = 0;
    let mut key: ::std::os::raw::c_int = 0;
    let mut attach_opts: bpf_prog_attach_opts = DECLARE_LIBBPF_OPTS::<bpf_prog_attach_opts>();
    let mut allow_prog: [::std::os::raw::c_int; 7] = [-1; 7];
    let mut value: ::std::os::raw::c_ulonglong = 0;
    let duration: __u32 = 0;
    let mut i: ::std::os::raw::c_int = 0;

    'err: loop {
        i = 0;
        while (i as usize) < allow_prog.len() {
            allow_prog[i as usize] = prog_load_cnt(1, 1 << i);
            if CHECK!(
                allow_prog[i as usize] < 0,
                "prog_load",
                "verifier output:\n%s\n-------\n",
                bpf_log_buf.as_ptr()
            ) {
                break 'err;
            }
            i += 1;
        }

        if CHECK_FAIL!(setup_cgroup_environment()) {
            break 'err;
        }

        cg1 = create_and_get_cgroup(b"/cg1\0".as_ptr() as *const _);
        if CHECK_FAIL!(cg1 < 0) {
            break 'err;
        }
        cg2 = create_and_get_cgroup(b"/cg1/cg2\0".as_ptr() as *const _);
        if CHECK_FAIL!(cg2 < 0) {
            break 'err;
        }
        cg3 = create_and_get_cgroup(b"/cg1/cg2/cg3\0".as_ptr() as *const _);
        if CHECK_FAIL!(cg3 < 0) {
            break 'err;
        }
        cg4 = create_and_get_cgroup(b"/cg1/cg2/cg3/cg4\0".as_ptr() as *const _);
        if CHECK_FAIL!(cg4 < 0) {
            break 'err;
        }
        cg5 = create_and_get_cgroup(b"/cg1/cg2/cg3/cg4/cg5\0".as_ptr() as *const _);
        if CHECK_FAIL!(cg5 < 0) {
            break 'err;
        }

        if CHECK_FAIL!(join_cgroup(b"/cg1/cg2/cg3/cg4/cg5\0".as_ptr() as *const _)) {
            break 'err;
        }

        if CHECK!(
            bpf_prog_attach(allow_prog[0], cg1, BPF_CGROUP_INET_EGRESS, BPF_F_ALLOW_MULTI),
            "prog0_attach_to_cg1_multi",
            "errno=%d\n",
            errno
        ) {
            break 'err;
        }

        if CHECK!(
            !bpf_prog_attach(allow_prog[0], cg1, BPF_CGROUP_INET_EGRESS, BPF_F_ALLOW_MULTI),
            "fail_same_prog_attach_to_cg1",
            "unexpected success\n"
        ) {
            break 'err;
        }

        if CHECK!(
            bpf_prog_attach(allow_prog[1], cg1, BPF_CGROUP_INET_EGRESS, BPF_F_ALLOW_MULTI),
            "prog1_attach_to_cg1_multi",
            "errno=%d\n",
            errno
        ) {
            break 'err;
        }

        if CHECK!(
            bpf_prog_attach(allow_prog[2], cg2, BPF_CGROUP_INET_EGRESS, BPF_F_ALLOW_OVERRIDE),
            "prog2_attach_to_cg2_override",
            "errno=%d\n",
            errno
        ) {
            break 'err;
        }

        if CHECK!(
            bpf_prog_attach(allow_prog[3], cg3, BPF_CGROUP_INET_EGRESS, BPF_F_ALLOW_MULTI),
            "prog3_attach_to_cg3_multi",
            "errno=%d\n",
            errno
        ) {
            break 'err;
        }

        if CHECK!(
            bpf_prog_attach(allow_prog[4], cg4, BPF_CGROUP_INET_EGRESS, BPF_F_ALLOW_OVERRIDE),
            "prog4_attach_to_cg4_override",
            "errno=%d\n",
            errno
        ) {
            break 'err;
        }

        if CHECK!(
            bpf_prog_attach(allow_prog[5], cg5, BPF_CGROUP_INET_EGRESS, 0),
            "prog5_attach_to_cg5_none",
            "errno=%d\n",
            errno
        ) {
            break 'err;
        }

        CHECK_FAIL!(system(PING_CMD));
        CHECK_FAIL!(bpf_map_lookup_elem(map_fd, &mut key as *mut _ as *const _, &mut value as *mut _ as *mut _));
        CHECK_FAIL!(value != 1 + 2 + 8 + 32);

        /* query the number of effective progs in cg5 */
        CHECK_FAIL!(bpf_prog_query(
            cg5,
            BPF_CGROUP_INET_EGRESS,
            BPF_F_QUERY_EFFECTIVE,
            ::std::ptr::null_mut(),
            ::std::ptr::null_mut(),
            &mut prog_cnt,
        ));
        CHECK_FAIL!(prog_cnt != 4);
        /* retrieve prog_ids of effective progs in cg5 */
        CHECK_FAIL!(bpf_prog_query(
            cg5,
            BPF_CGROUP_INET_EGRESS,
            BPF_F_QUERY_EFFECTIVE,
            &mut attach_flags,
            prog_ids.as_mut_ptr(),
            &mut prog_cnt,
        ));
        CHECK_FAIL!(prog_cnt != 4);
        CHECK_FAIL!(attach_flags != 0);
        saved_prog_id = prog_ids[0];
        /* check enospc handling */
        prog_ids[0] = 0;
        prog_cnt = 2;
        CHECK_FAIL!(bpf_prog_query(
            cg5,
            BPF_CGROUP_INET_EGRESS,
            BPF_F_QUERY_EFFECTIVE,
            &mut attach_flags,
            prog_ids.as_mut_ptr(),
            &mut prog_cnt,
        ) >= 0);
        CHECK_FAIL!(errno != ENOSPC);
        CHECK_FAIL!(prog_cnt != 4);
        /* check that prog_ids are returned even when buffer is too small */
        CHECK_FAIL!(prog_ids[0] != saved_prog_id);
        /* retrieve prog_id of single attached prog in cg5 */
        prog_ids[0] = 0;
        CHECK_FAIL!(bpf_prog_query(
            cg5,
            BPF_CGROUP_INET_EGRESS,
            0,
            ::std::ptr::null_mut(),
            prog_ids.as_mut_ptr(),
            &mut prog_cnt,
        ));
        CHECK_FAIL!(prog_cnt != 1);
        CHECK_FAIL!(prog_ids[0] != saved_prog_id);

        /* detach bottom program and ping again */
        if CHECK!(
            bpf_prog_detach2(-1, cg5, BPF_CGROUP_INET_EGRESS),
            "prog_detach_from_cg5",
            "errno=%d\n",
            errno
        ) {
            break 'err;
        }

        value = 0;
        CHECK_FAIL!(bpf_map_update_elem(map_fd, &mut key as *mut _ as *const _, &mut value as *mut _ as *const _, 0));
        CHECK_FAIL!(system(PING_CMD));
        CHECK_FAIL!(bpf_map_lookup_elem(map_fd, &mut key as *mut _ as *const _, &mut value as *mut _ as *mut _));
        CHECK_FAIL!(value != 1 + 2 + 8 + 16);

        /* test replace */

        attach_opts.flags = BPF_F_ALLOW_OVERRIDE | BPF_F_REPLACE;
        attach_opts.replace_prog_fd = allow_prog[0];
        if CHECK!(
            !bpf_prog_attach_opts(allow_prog[6], cg1, BPF_CGROUP_INET_EGRESS, &mut attach_opts),
            "fail_prog_replace_override",
            "unexpected success\n"
        ) {
            break 'err;
        }
        CHECK_FAIL!(errno != EINVAL);

        attach_opts.flags = BPF_F_REPLACE;
        if CHECK!(
            !bpf_prog_attach_opts(allow_prog[6], cg1, BPF_CGROUP_INET_EGRESS, &mut attach_opts),
            "fail_prog_replace_no_multi",
            "unexpected success\n"
        ) {
            break 'err;
        }
        CHECK_FAIL!(errno != EINVAL);

        attach_opts.flags = BPF_F_ALLOW_MULTI | BPF_F_REPLACE;
        attach_opts.replace_prog_fd = -1;
        if CHECK!(
            !bpf_prog_attach_opts(allow_prog[6], cg1, BPF_CGROUP_INET_EGRESS, &mut attach_opts),
            "fail_prog_replace_bad_fd",
            "unexpected success\n"
        ) {
            break 'err;
        }
        CHECK_FAIL!(errno != EBADF);

        /* replacing a program that is not attached to cgroup should fail  */
        attach_opts.replace_prog_fd = allow_prog[3];
        if CHECK!(
            !bpf_prog_attach_opts(allow_prog[6], cg1, BPF_CGROUP_INET_EGRESS, &mut attach_opts),
            "fail_prog_replace_no_ent",
            "unexpected success\n"
        ) {
            break 'err;
        }
        CHECK_FAIL!(errno != ENOENT);

        /* replace 1st from the top program */
        attach_opts.replace_prog_fd = allow_prog[0];
        if CHECK!(
            bpf_prog_attach_opts(allow_prog[6], cg1, BPF_CGROUP_INET_EGRESS, &mut attach_opts),
            "prog_replace",
            "errno=%d\n",
            errno
        ) {
            break 'err;
        }

        /* replace program with itself */
        attach_opts.replace_prog_fd = allow_prog[6];
        if CHECK!(
            bpf_prog_attach_opts(allow_prog[6], cg1, BPF_CGROUP_INET_EGRESS, &mut attach_opts),
            "prog_replace",
            "errno=%d\n",
            errno
        ) {
            break 'err;
        }

        value = 0;
        CHECK_FAIL!(bpf_map_update_elem(map_fd, &mut key as *mut _ as *const _, &mut value as *mut _ as *const _, 0));
        CHECK_FAIL!(system(PING_CMD));
        CHECK_FAIL!(bpf_map_lookup_elem(map_fd, &mut key as *mut _ as *const _, &mut value as *mut _ as *mut _));
        CHECK_FAIL!(value != 64 + 2 + 8 + 16);

        /* detach 3rd from bottom program and ping again */
        if CHECK!(
            !bpf_prog_detach2(0, cg3, BPF_CGROUP_INET_EGRESS),
            "fail_prog_detach_from_cg3",
            "unexpected success\n"
        ) {
            break 'err;
        }

        if CHECK!(
            bpf_prog_detach2(allow_prog[3], cg3, BPF_CGROUP_INET_EGRESS),
            "prog3_detach_from_cg3",
            "errno=%d\n",
            errno
        ) {
            break 'err;
        }

        value = 0;
        CHECK_FAIL!(bpf_map_update_elem(map_fd, &mut key as *mut _ as *const _, &mut value as *mut _ as *const _, 0));
        CHECK_FAIL!(system(PING_CMD));
        CHECK_FAIL!(bpf_map_lookup_elem(map_fd, &mut key as *mut _ as *const _, &mut value as *mut _ as *mut _));
        CHECK_FAIL!(value != 64 + 2 + 16);

        /* detach 2nd from bottom program and ping again */
        if CHECK!(
            bpf_prog_detach2(-1, cg4, BPF_CGROUP_INET_EGRESS),
            "prog_detach_from_cg4",
            "errno=%d\n",
            errno
        ) {
            break 'err;
        }

        value = 0;
        CHECK_FAIL!(bpf_map_update_elem(map_fd, &mut key as *mut _ as *const _, &mut value as *mut _ as *const _, 0));
        CHECK_FAIL!(system(PING_CMD));
        CHECK_FAIL!(bpf_map_lookup_elem(map_fd, &mut key as *mut _ as *const _, &mut value as *mut _ as *mut _));
        CHECK_FAIL!(value != 64 + 2 + 4);

        prog_cnt = 4;
        CHECK_FAIL!(bpf_prog_query(
            cg5,
            BPF_CGROUP_INET_EGRESS,
            BPF_F_QUERY_EFFECTIVE,
            &mut attach_flags,
            prog_ids.as_mut_ptr(),
            &mut prog_cnt,
        ));
        CHECK_FAIL!(prog_cnt != 3);
        CHECK_FAIL!(attach_flags != 0);
        CHECK_FAIL!(bpf_prog_query(
            cg5,
            BPF_CGROUP_INET_EGRESS,
            0,
            ::std::ptr::null_mut(),
            prog_ids.as_mut_ptr(),
            &mut prog_cnt,
        ));
        CHECK_FAIL!(prog_cnt != 0);

        break 'err;
    }

    i = 0;
    while (i as usize) < allow_prog.len() {
        if allow_prog[i as usize] >= 0 {
            close(allow_prog[i as usize]);
        }
        i += 1;
    }
    close(cg1);
    close(cg2);
    close(cg3);
    close(cg4);
    close(cg5);
    cleanup_cgroup_environment();

    let _ = duration;
}
