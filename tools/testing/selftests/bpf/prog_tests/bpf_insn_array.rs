// SPDX-License-Identifier: GPL-2.0

// Translated from C source that included <bpf/bpf.h> and <test_progs.h>.

#[cfg(any(target_arch = "x86_64", target_arch = "powerpc", target_arch = "aarch64"))]
unsafe fn map_create(map_type: __u32, max_entries: __u32) -> i32 {
    let map_name = c"insn_array".as_ptr();
    let key_size: __u32 = 4;
    let value_size: __u32 = core::mem::size_of::<bpf_insn_array_value>() as __u32;

    bpf_map_create(map_type, map_name, key_size, value_size, max_entries, core::ptr::null_mut())
}

#[cfg(any(target_arch = "x86_64", target_arch = "powerpc", target_arch = "aarch64"))]
unsafe fn prog_load(
    insns: *mut bpf_insn,
    insn_cnt: __u32,
    fd_array: *mut i32,
    fd_array_cnt: __u32,
) -> i32 {
    let mut opts: bpf_prog_load_opts = LIBBPF_OPTS();

    opts.fd_array = fd_array;
    opts.fd_array_cnt = fd_array_cnt;

    bpf_prog_load(BPF_PROG_TYPE_XDP, core::ptr::null(), c"GPL".as_ptr(), insns, insn_cnt, &mut opts)
}

#[cfg(any(target_arch = "x86_64", target_arch = "powerpc", target_arch = "aarch64"))]
unsafe fn __check_success(
    insns: *mut bpf_insn,
    insn_cnt: __u32,
    map_in: *mut __u32,
    map_out: *mut __u32,
) {
    let mut val: bpf_insn_array_value = core::mem::zeroed();
    let mut prog_fd: i32 = -1;
    let map_fd: i32;
    let mut i: i32;

    map_fd = map_create(BPF_MAP_TYPE_INSN_ARRAY, insn_cnt);
    if !ASSERT_GE(map_fd, 0, c"map_create".as_ptr()) {
        return;
    }

    i = 0;
    while i < insn_cnt as i32 {
        val.orig_off = *map_in.offset(i as isize);
        if !ASSERT_EQ(
            bpf_map_update_elem(
                map_fd,
                &mut i as *mut i32 as *const core::ffi::c_void,
                &mut val as *mut bpf_insn_array_value as *const core::ffi::c_void,
                0,
            ),
            0,
            c"bpf_map_update_elem".as_ptr(),
        ) {
            goto_cleanup_success(prog_fd, map_fd);
            return;
        }
        i += 1;
    }

    if !ASSERT_EQ(bpf_map_freeze(map_fd), 0, c"bpf_map_freeze".as_ptr()) {
        goto_cleanup_success(prog_fd, map_fd);
        return;
    }

    prog_fd = prog_load(insns, insn_cnt, &map_fd as *const i32 as *mut i32, 1);
    if !ASSERT_GE(prog_fd, 0, c"bpf(BPF_PROG_LOAD)".as_ptr()) {
        goto_cleanup_success(prog_fd, map_fd);
        return;
    }

    i = 0;
    while i < insn_cnt as i32 {
        let mut buf = [0i8; 64];

        if !ASSERT_EQ(
            bpf_map_lookup_elem(
                map_fd,
                &mut i as *mut i32 as *const core::ffi::c_void,
                &mut val as *mut bpf_insn_array_value as *mut core::ffi::c_void,
            ),
            0,
            c"bpf_map_lookup_elem".as_ptr(),
        ) {
            goto_cleanup_success(prog_fd, map_fd);
            return;
        }

        snprintf(
            buf.as_mut_ptr(),
            buf.len(),
            c"val.xlated_off should be equal map_out[%d]".as_ptr(),
            i,
        );
        ASSERT_EQ(val.xlated_off, *map_out.offset(i as isize), buf.as_ptr());
        i += 1;
    }

    goto_cleanup_success(prog_fd, map_fd);
}

#[cfg(any(target_arch = "x86_64", target_arch = "powerpc", target_arch = "aarch64"))]
unsafe fn goto_cleanup_success(prog_fd: i32, map_fd: i32) {
    close(prog_fd);
    close(map_fd);
}

/*
 * Load a program, which will not be anyhow mangled by the verifier.  Add an
 * insn_array map pointing to every instruction. Check that it hasn't changed
 * after the program load.
 */
#[cfg(any(target_arch = "x86_64", target_arch = "powerpc", target_arch = "aarch64"))]
unsafe fn check_one_to_one_mapping() {
    let mut insns = [
        BPF_MOV64_IMM(BPF_REG_0, 4),
        BPF_MOV64_IMM(BPF_REG_0, 3),
        BPF_MOV64_IMM(BPF_REG_0, 2),
        BPF_MOV64_IMM(BPF_REG_0, 1),
        BPF_MOV64_IMM(BPF_REG_0, 0),
        BPF_EXIT_INSN(),
    ];
    let mut map_in: [__u32; 6] = [0, 1, 2, 3, 4, 5];
    let mut map_out: [__u32; 6] = [0, 1, 2, 3, 4, 5];

    __check_success(insns.as_mut_ptr(), insns.len() as __u32, map_in.as_mut_ptr(), map_out.as_mut_ptr());
}

/*
 * Load a program with two patches (get jiffies, for simplicity). Add an
 * insn_array map pointing to every instruction. Check how it was changed
 * after the program load.
 */
#[cfg(any(target_arch = "x86_64", target_arch = "powerpc", target_arch = "aarch64"))]
unsafe fn check_simple() {
    let mut insns = [
        BPF_MOV64_IMM(BPF_REG_0, 2),
        BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 0, 0, BPF_FUNC_jiffies64),
        BPF_MOV64_IMM(BPF_REG_0, 1),
        BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 0, 0, BPF_FUNC_jiffies64),
        BPF_MOV64_IMM(BPF_REG_0, 0),
        BPF_EXIT_INSN(),
    ];
    let mut map_in: [__u32; 6] = [0, 1, 2, 3, 4, 5];
    let mut map_out: [__u32; 6] = [0, 1, 4, 5, 8, 9];

    __check_success(insns.as_mut_ptr(), insns.len() as __u32, map_in.as_mut_ptr(), map_out.as_mut_ptr());
}

/*
 * Verifier can delete code in two cases: nops & dead code. From insn
 * array's point of view, the two cases are the same, so test using
 * the simplest method: by loading some nops
 */
#[cfg(any(target_arch = "x86_64", target_arch = "powerpc", target_arch = "aarch64"))]
unsafe fn check_deletions() {
    let mut insns = [
        BPF_MOV64_IMM(BPF_REG_0, 2),
        BPF_JMP_IMM(BPF_JA, 0, 0, 0), /* nop */
        BPF_MOV64_IMM(BPF_REG_0, 1),
        BPF_JMP_IMM(BPF_JA, 0, 0, 0), /* nop */
        BPF_MOV64_IMM(BPF_REG_0, 0),
        BPF_EXIT_INSN(),
    ];
    let mut map_in: [__u32; 6] = [0, 1, 2, 3, 4, 5];
    let mut map_out: [__u32; 6] = [0, (-1i32) as __u32, 1, (-1i32) as __u32, 2, 3];

    __check_success(insns.as_mut_ptr(), insns.len() as __u32, map_in.as_mut_ptr(), map_out.as_mut_ptr());
}

/*
 * Same test as check_deletions, but also add code which adds instructions
 */
#[cfg(any(target_arch = "x86_64", target_arch = "powerpc", target_arch = "aarch64"))]
unsafe fn check_deletions_with_functions() {
    let mut insns = [
        BPF_JMP_IMM(BPF_JA, 0, 0, 0), /* nop */
        BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 0, 0, BPF_FUNC_jiffies64),
        BPF_JMP_IMM(BPF_JA, 0, 0, 0), /* nop */
        BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 1, 0, 2),
        BPF_MOV64_IMM(BPF_REG_0, 1),
        BPF_EXIT_INSN(),
        BPF_JMP_IMM(BPF_JA, 0, 0, 0), /* nop */
        BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 0, 0, BPF_FUNC_jiffies64),
        BPF_JMP_IMM(BPF_JA, 0, 0, 0), /* nop */
        BPF_MOV64_IMM(BPF_REG_0, 2),
        BPF_EXIT_INSN(),
    ];
    let mut map_in: [__u32; 11] = [0, 1, 2, 3, 4, 5, /* func */ 6, 7, 8, 9, 10];
    let mut map_out: [__u32; 11] = [
        (-1i32) as __u32,
        0,
        (-1i32) as __u32,
        3,
        4,
        5,
        /* func */
        (-1i32) as __u32,
        6,
        (-1i32) as __u32,
        9,
        10,
    ];

    __check_success(insns.as_mut_ptr(), insns.len() as __u32, map_in.as_mut_ptr(), map_out.as_mut_ptr());
}

/*
 * Try to load a program with a map which points to outside of the program
 */
#[cfg(any(target_arch = "x86_64", target_arch = "powerpc", target_arch = "aarch64"))]
unsafe fn check_out_of_bounds_index() {
    let mut insns = [
        BPF_MOV64_IMM(BPF_REG_0, 4),
        BPF_MOV64_IMM(BPF_REG_0, 3),
        BPF_MOV64_IMM(BPF_REG_0, 2),
        BPF_MOV64_IMM(BPF_REG_0, 1),
        BPF_MOV64_IMM(BPF_REG_0, 0),
        BPF_EXIT_INSN(),
    ];
    let prog_fd: i32;
    let map_fd: i32;
    let mut val: bpf_insn_array_value = core::mem::zeroed();
    let mut key: i32;

    map_fd = map_create(BPF_MAP_TYPE_INSN_ARRAY, 1);
    if !ASSERT_GE(map_fd, 0, c"map_create".as_ptr()) {
        return;
    }

    key = 0;
    val.orig_off = insns.len() as __u32; /* too big */
    if !ASSERT_EQ(
        bpf_map_update_elem(
            map_fd,
            &mut key as *mut i32 as *const core::ffi::c_void,
            &mut val as *mut bpf_insn_array_value as *const core::ffi::c_void,
            0,
        ),
        0,
        c"bpf_map_update_elem".as_ptr(),
    ) {
        close(map_fd);
        return;
    }

    if !ASSERT_EQ(bpf_map_freeze(map_fd), 0, c"bpf_map_freeze".as_ptr()) {
        close(map_fd);
        return;
    }

    prog_fd = prog_load(insns.as_mut_ptr(), insns.len() as __u32, &map_fd as *const i32 as *mut i32, 1);
    if !ASSERT_EQ(
        prog_fd,
        -EINVAL,
        c"program should have been rejected (prog_fd != -EINVAL)".as_ptr(),
    ) {
        close(prog_fd);
        close(map_fd);
        return;
    }

    close(map_fd);
}

/*
 * Try to load a program with a map which points to the middle of 16-bit insn
 */
#[cfg(any(target_arch = "x86_64", target_arch = "powerpc", target_arch = "aarch64"))]
unsafe fn check_mid_insn_index() {
    let mut insns = [
        BPF_LD_IMM64(BPF_REG_0, 0), /* 2 x 8 */
        BPF_EXIT_INSN(),
    ];
    let prog_fd: i32;
    let map_fd: i32;
    let mut val: bpf_insn_array_value = core::mem::zeroed();
    let mut key: i32;

    map_fd = map_create(BPF_MAP_TYPE_INSN_ARRAY, 1);
    if !ASSERT_GE(map_fd, 0, c"map_create".as_ptr()) {
        return;
    }

    key = 0;
    val.orig_off = 1; /* middle of 16-byte instruction */
    if !ASSERT_EQ(
        bpf_map_update_elem(
            map_fd,
            &mut key as *mut i32 as *const core::ffi::c_void,
            &mut val as *mut bpf_insn_array_value as *const core::ffi::c_void,
            0,
        ),
        0,
        c"bpf_map_update_elem".as_ptr(),
    ) {
        close(map_fd);
        return;
    }

    if !ASSERT_EQ(bpf_map_freeze(map_fd), 0, c"bpf_map_freeze".as_ptr()) {
        close(map_fd);
        return;
    }

    prog_fd = prog_load(insns.as_mut_ptr(), insns.len() as __u32, &map_fd as *const i32 as *mut i32, 1);
    if !ASSERT_EQ(
        prog_fd,
        -EINVAL,
        c"program should have been rejected (prog_fd != -EINVAL)".as_ptr(),
    ) {
        close(prog_fd);
        close(map_fd);
        return;
    }

    close(map_fd);
}

#[cfg(any(target_arch = "x86_64", target_arch = "powerpc", target_arch = "aarch64"))]
unsafe fn check_incorrect_index() {
    check_out_of_bounds_index();
    check_mid_insn_index();
}

#[cfg(any(target_arch = "x86_64", target_arch = "powerpc", target_arch = "aarch64"))]
unsafe fn set_bpf_jit_harden(level: *mut i8) -> i32 {
    let mut old_level: i8 = 0;
    let mut err: i32 = -1;
    let mut fd: i32 = -1;

    fd = open(c"/proc/sys/net/core/bpf_jit_harden".as_ptr(), O_RDWR | O_NONBLOCK);
    if fd < 0 {
        ASSERT_FAIL(c"open .../bpf_jit_harden returned %d (errno=%d)".as_ptr(), fd, errno);
        return -1;
    }

    err = read(fd, &mut old_level as *mut i8 as *mut core::ffi::c_void, 1) as i32;
    if err != 1 {
        ASSERT_FAIL(c"read from .../bpf_jit_harden returned %d (errno=%d)".as_ptr(), err, errno);
        err = -1;
        if fd >= 0 {
            close(fd);
        }
        return err;
    }

    lseek(fd, 0, SEEK_SET);

    err = write(fd, level as *const core::ffi::c_void, 1) as i32;
    if err != 1 {
        ASSERT_FAIL(c"write to .../bpf_jit_harden returned %d (errno=%d)".as_ptr(), err, errno);
        err = -1;
        if fd >= 0 {
            close(fd);
        }
        return err;
    }

    err = 0;
    *level = old_level;
    if fd >= 0 {
        close(fd);
    }
    err
}

#[cfg(any(target_arch = "x86_64", target_arch = "powerpc", target_arch = "aarch64"))]
unsafe fn check_blindness() {
    let mut insns = [
        BPF_MOV64_IMM(BPF_REG_0, 4),
        BPF_MOV64_IMM(BPF_REG_0, 3),
        BPF_MOV64_IMM(BPF_REG_0, 2),
        BPF_MOV64_IMM(BPF_REG_0, 1),
        BPF_EXIT_INSN(),
    ];
    let mut prog_fd: i32 = -1;
    let map_fd: i32;
    let mut val: bpf_insn_array_value = core::mem::zeroed();
    let mut bpf_jit_harden: i8 = b'@' as i8; /* non-exizsting value */
    let mut i: i32;

    map_fd = map_create(BPF_MAP_TYPE_INSN_ARRAY, insns.len() as __u32);
    if !ASSERT_GE(map_fd, 0, c"map_create".as_ptr()) {
        return;
    }

    i = 0;
    while i < insns.len() as i32 {
        val.orig_off = i as __u32;
        if !ASSERT_EQ(
            bpf_map_update_elem(
                map_fd,
                &mut i as *mut i32 as *const core::ffi::c_void,
                &mut val as *mut bpf_insn_array_value as *const core::ffi::c_void,
                0,
            ),
            0,
            c"bpf_map_update_elem".as_ptr(),
        ) {
            goto_cleanup_blindness(prog_fd, map_fd, &mut bpf_jit_harden);
            return;
        }
        i += 1;
    }

    if !ASSERT_EQ(bpf_map_freeze(map_fd), 0, c"bpf_map_freeze".as_ptr()) {
        goto_cleanup_blindness(prog_fd, map_fd, &mut bpf_jit_harden);
        return;
    }

    bpf_jit_harden = b'2' as i8;
    if set_bpf_jit_harden(&mut bpf_jit_harden) != 0 {
        bpf_jit_harden = b'@' as i8; /* open, read or write failed => no write was done */
        goto_cleanup_blindness(prog_fd, map_fd, &mut bpf_jit_harden);
        return;
    }

    prog_fd = prog_load(insns.as_mut_ptr(), insns.len() as __u32, &map_fd as *const i32 as *mut i32, 1);
    if !ASSERT_GE(prog_fd, 0, c"bpf(BPF_PROG_LOAD)".as_ptr()) {
        goto_cleanup_blindness(prog_fd, map_fd, &mut bpf_jit_harden);
        return;
    }

    i = 0;
    while i < insns.len() as i32 {
        let mut fmt = [0i8; 32];

        if !ASSERT_EQ(
            bpf_map_lookup_elem(
                map_fd,
                &mut i as *mut i32 as *const core::ffi::c_void,
                &mut val as *mut bpf_insn_array_value as *mut core::ffi::c_void,
            ),
            0,
            c"bpf_map_lookup_elem".as_ptr(),
        ) {
            goto_cleanup_blindness(prog_fd, map_fd, &mut bpf_jit_harden);
            return;
        }

        snprintf(fmt.as_mut_ptr(), fmt.len(), c"val should be equal 3*%d".as_ptr(), i);
        ASSERT_EQ(val.xlated_off, (i * 3) as __u32, fmt.as_ptr());
        i += 1;
    }

    goto_cleanup_blindness(prog_fd, map_fd, &mut bpf_jit_harden);
}

#[cfg(any(target_arch = "x86_64", target_arch = "powerpc", target_arch = "aarch64"))]
unsafe fn goto_cleanup_blindness(prog_fd: i32, map_fd: i32, bpf_jit_harden: *mut i8) {
    /* restore the old one */
    if *bpf_jit_harden != b'@' as i8 {
        set_bpf_jit_harden(bpf_jit_harden);
    }

    close(prog_fd);
    close(map_fd);
}

/* Once map was initialized, it should be frozen */
#[cfg(any(target_arch = "x86_64", target_arch = "powerpc", target_arch = "aarch64"))]
unsafe fn check_load_unfrozen_map() {
    let mut insns = [
        BPF_MOV64_IMM(BPF_REG_0, 0),
        BPF_EXIT_INSN(),
    ];
    let mut prog_fd: i32 = -1;
    let map_fd: i32;
    let mut val: bpf_insn_array_value = core::mem::zeroed();
    let mut i: i32;

    map_fd = map_create(BPF_MAP_TYPE_INSN_ARRAY, insns.len() as __u32);
    if !ASSERT_GE(map_fd, 0, c"map_create".as_ptr()) {
        return;
    }

    i = 0;
    while i < insns.len() as i32 {
        val.orig_off = i as __u32;
        if !ASSERT_EQ(
            bpf_map_update_elem(
                map_fd,
                &mut i as *mut i32 as *const core::ffi::c_void,
                &mut val as *mut bpf_insn_array_value as *const core::ffi::c_void,
                0,
            ),
            0,
            c"bpf_map_update_elem".as_ptr(),
        ) {
            goto_cleanup_success(prog_fd, map_fd);
            return;
        }
        i += 1;
    }

    prog_fd = prog_load(insns.as_mut_ptr(), insns.len() as __u32, &map_fd as *const i32 as *mut i32, 1);
    if !ASSERT_EQ(
        prog_fd,
        -EINVAL,
        c"program should have been rejected (prog_fd != -EINVAL)".as_ptr(),
    ) {
        goto_cleanup_success(prog_fd, map_fd);
        return;
    }

    /* correctness: now freeze the map, the program should load fine */

    if !ASSERT_EQ(bpf_map_freeze(map_fd), 0, c"bpf_map_freeze".as_ptr()) {
        goto_cleanup_success(prog_fd, map_fd);
        return;
    }

    prog_fd = prog_load(insns.as_mut_ptr(), insns.len() as __u32, &map_fd as *const i32 as *mut i32, 1);
    if !ASSERT_GE(prog_fd, 0, c"bpf(BPF_PROG_LOAD)".as_ptr()) {
        goto_cleanup_success(prog_fd, map_fd);
        return;
    }

    i = 0;
    while i < insns.len() as i32 {
        if !ASSERT_EQ(
            bpf_map_lookup_elem(
                map_fd,
                &mut i as *mut i32 as *const core::ffi::c_void,
                &mut val as *mut bpf_insn_array_value as *mut core::ffi::c_void,
            ),
            0,
            c"bpf_map_lookup_elem".as_ptr(),
        ) {
            goto_cleanup_success(prog_fd, map_fd);
            return;
        }

        ASSERT_EQ(val.xlated_off, i as __u32, c"val should be equal i".as_ptr());
        i += 1;
    }

    goto_cleanup_success(prog_fd, map_fd);
}

/* Map can be used only by one BPF program */
#[cfg(any(target_arch = "x86_64", target_arch = "powerpc", target_arch = "aarch64"))]
unsafe fn check_no_map_reuse() {
    let mut insns = [
        BPF_MOV64_IMM(BPF_REG_0, 0),
        BPF_EXIT_INSN(),
    ];
    let mut prog_fd: i32 = -1;
    let map_fd: i32;
    let mut extra_fd: i32 = -1;
    let mut val: bpf_insn_array_value = core::mem::zeroed();
    let mut i: i32;

    map_fd = map_create(BPF_MAP_TYPE_INSN_ARRAY, insns.len() as __u32);
    if !ASSERT_GE(map_fd, 0, c"map_create".as_ptr()) {
        return;
    }

    i = 0;
    while i < insns.len() as i32 {
        val.orig_off = i as __u32;
        if !ASSERT_EQ(
            bpf_map_update_elem(
                map_fd,
                &mut i as *mut i32 as *const core::ffi::c_void,
                &mut val as *mut bpf_insn_array_value as *const core::ffi::c_void,
                0,
            ),
            0,
            c"bpf_map_update_elem".as_ptr(),
        ) {
            goto_cleanup_no_map_reuse(extra_fd, prog_fd, map_fd);
            return;
        }
        i += 1;
    }

    if !ASSERT_EQ(bpf_map_freeze(map_fd), 0, c"bpf_map_freeze".as_ptr()) {
        goto_cleanup_no_map_reuse(extra_fd, prog_fd, map_fd);
        return;
    }

    prog_fd = prog_load(insns.as_mut_ptr(), insns.len() as __u32, &map_fd as *const i32 as *mut i32, 1);
    if !ASSERT_GE(prog_fd, 0, c"bpf(BPF_PROG_LOAD)".as_ptr()) {
        goto_cleanup_no_map_reuse(extra_fd, prog_fd, map_fd);
        return;
    }

    i = 0;
    while i < insns.len() as i32 {
        if !ASSERT_EQ(
            bpf_map_lookup_elem(
                map_fd,
                &mut i as *mut i32 as *const core::ffi::c_void,
                &mut val as *mut bpf_insn_array_value as *mut core::ffi::c_void,
            ),
            0,
            c"bpf_map_lookup_elem".as_ptr(),
        ) {
            goto_cleanup_no_map_reuse(extra_fd, prog_fd, map_fd);
            return;
        }

        ASSERT_EQ(val.xlated_off, i as __u32, c"val should be equal i".as_ptr());
        i += 1;
    }

    extra_fd = prog_load(insns.as_mut_ptr(), insns.len() as __u32, &map_fd as *const i32 as *mut i32, 1);
    if !ASSERT_EQ(
        extra_fd,
        -EBUSY,
        c"program should have been rejected (extra_fd != -EBUSY)".as_ptr(),
    ) {
        goto_cleanup_no_map_reuse(extra_fd, prog_fd, map_fd);
        return;
    }

    /* correctness: check that prog is still loadable without fd_array */
    extra_fd = prog_load(insns.as_mut_ptr(), insns.len() as __u32, core::ptr::null_mut(), 0);
    if !ASSERT_GE(extra_fd, 0, c"bpf(BPF_PROG_LOAD): expected no error".as_ptr()) {
        goto_cleanup_no_map_reuse(extra_fd, prog_fd, map_fd);
        return;
    }

    goto_cleanup_no_map_reuse(extra_fd, prog_fd, map_fd);
}

#[cfg(any(target_arch = "x86_64", target_arch = "powerpc", target_arch = "aarch64"))]
unsafe fn goto_cleanup_no_map_reuse(extra_fd: i32, prog_fd: i32, map_fd: i32) {
    close(extra_fd);
    close(prog_fd);
    close(map_fd);
}

#[cfg(any(target_arch = "x86_64", target_arch = "powerpc", target_arch = "aarch64"))]
unsafe fn check_bpf_no_lookup() {
    let mut insns = [
        BPF_LD_MAP_FD(BPF_REG_1, 0),
        BPF_ST_MEM(BPF_DW, BPF_REG_10, -8, 0),
        BPF_MOV64_REG(BPF_REG_2, BPF_REG_10),
        BPF_ALU64_IMM(BPF_ADD, BPF_REG_2, -8),
        BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 0, 0, BPF_FUNC_map_lookup_elem),
        BPF_EXIT_INSN(),
    ];
    let mut prog_fd: i32 = -1;
    let mut map_fd: i32;

    map_fd = map_create(BPF_MAP_TYPE_INSN_ARRAY, 1);
    if !ASSERT_GE(map_fd, 0, c"map_create".as_ptr()) {
        return;
    }

    insns[0].imm = map_fd;

    if !ASSERT_EQ(bpf_map_freeze(map_fd), 0, c"bpf_map_freeze".as_ptr()) {
        close(prog_fd);
        close(map_fd);
        return;
    }

    prog_fd = prog_load(insns.as_mut_ptr(), insns.len() as __u32, core::ptr::null_mut(), 0);
    if !ASSERT_EQ(
        prog_fd,
        -EINVAL,
        c"program should have been rejected (prog_fd != -EINVAL)".as_ptr(),
    ) {
        close(prog_fd);
        close(map_fd);
        return;
    }

    /* correctness: check that prog is still loadable with normal map */
    close(map_fd);
    map_fd = map_create(BPF_MAP_TYPE_ARRAY, 1);
    insns[0].imm = map_fd;
    prog_fd = prog_load(insns.as_mut_ptr(), insns.len() as __u32, core::ptr::null_mut(), 0);
    if !ASSERT_GE(prog_fd, 0, c"bpf(BPF_PROG_LOAD)".as_ptr()) {
        close(prog_fd);
        close(map_fd);
        return;
    }

    close(prog_fd);
    close(map_fd);
}

#[cfg(any(target_arch = "x86_64", target_arch = "powerpc", target_arch = "aarch64"))]
unsafe fn check_bpf_side() {
    check_bpf_no_lookup();
}

#[cfg(any(target_arch = "x86_64", target_arch = "powerpc", target_arch = "aarch64"))]
unsafe fn __test_bpf_insn_array() {
    /* Test if offsets are adjusted properly */

    if test__start_subtest(c"one2one".as_ptr()) {
        check_one_to_one_mapping();
    }

    if test__start_subtest(c"simple".as_ptr()) {
        check_simple();
    }

    if test__start_subtest(c"deletions".as_ptr()) {
        check_deletions();
    }

    if test__start_subtest(c"deletions-with-functions".as_ptr()) {
        check_deletions_with_functions();
    }

    if test__start_subtest(c"blindness".as_ptr()) {
        check_blindness();
    }

    /* Check all kinds of operations and related restrictions */

    if test__start_subtest(c"incorrect-index".as_ptr()) {
        check_incorrect_index();
    }

    if test__start_subtest(c"load-unfrozen-map".as_ptr()) {
        check_load_unfrozen_map();
    }

    if test__start_subtest(c"no-map-reuse".as_ptr()) {
        check_no_map_reuse();
    }

    if test__start_subtest(c"bpf-side-ops".as_ptr()) {
        check_bpf_side();
    }
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "powerpc", target_arch = "aarch64")))]
unsafe fn __test_bpf_insn_array() {
    test__skip();
}

pub unsafe fn test_bpf_insn_array() {
    __test_bpf_insn_array();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
