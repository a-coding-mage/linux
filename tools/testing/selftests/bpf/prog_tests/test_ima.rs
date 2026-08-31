// SPDX-License-Identifier: GPL-2.0

/*
 * Copyright (C) 2020 Google LLC.
 */

// C dependencies: stdio.h, stdlib.h, unistd.h, sys/wait.h, test_progs.h,
// linux/ring_buffer.h, and "ima.skel.h".

use core::ffi::{c_char, c_int, c_uint, c_void};

type size_t = usize;
type u32 = u32;
type u64 = u64;

const MAX_SAMPLES: usize = 4;

const NULL: *mut c_void = core::ptr::null_mut();

extern "C" {
    static mut errno: c_int;

    fn fork() -> c_int;
    fn getpid() -> c_int;
    fn execlp(file: *const c_char, arg: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn waitpid(pid: c_int, status: *mut c_int, options: c_int) -> c_int;
    fn mkdtemp(template: *mut c_char) -> *mut c_char;
    fn snprintf(str: *mut c_char, size: size_t, format: *const c_char, ...) -> c_int;
    fn system(command: *const c_char) -> c_int;

    fn ima__open_and_load() -> *mut ima;
    fn ima__attach(obj: *mut ima) -> c_int;
    fn ima__destroy(obj: *mut ima);

    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn ring_buffer__new(
        map_fd: c_int,
        sample_cb: Option<unsafe extern "C" fn(*mut c_void, *mut c_void, size_t) -> c_int>,
        ctx: *mut c_void,
        opts: *mut c_void,
    ) -> *mut ring_buffer;
    fn ring_buffer__consume(rb: *mut ring_buffer) -> c_int;
    fn ring_buffer__free(rb: *mut ring_buffer);

    fn CHECK(condition: bool, name: *const c_char, format: *const c_char, ...) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: u64, expected: u64, name: *const c_char) -> bool;
    fn ASSERT_NEQ(actual: u64, expected: u64, name: *const c_char) -> bool;
    fn ASSERT_GE(actual: c_int, expected: c_int, name: *const c_char) -> bool;
}

#[repr(C)]
struct ring_buffer {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
struct ima__bss {
    use_ima_file_hash: bool,
    enable_bprm_creds_for_exec: bool,
    enable_kernel_read_file: bool,
    test_deny: bool,
    monitored_pid: u32,
}

#[repr(C)]
struct ima__maps {
    ringbuf: *mut bpf_map,
}

#[repr(C)]
struct ima {
    maps: ima__maps,
    bss: *mut ima__bss,
}

const EINVAL: c_int = 22;
const ENOSPC: c_int = 28;

#[inline]
unsafe fn WEXITSTATUS(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

unsafe fn _run_measured_process(
    measured_dir: *const c_char,
    monitored_pid: *mut u32,
    cmd: *const c_char,
) -> c_int {
    let child_pid: c_int;
    let mut child_status: c_int = 0;

    child_pid = fork();
    if child_pid == 0 {
        *monitored_pid = getpid() as u32;
        execlp(
            b"./ima_setup.sh\0".as_ptr() as *const c_char,
            b"./ima_setup.sh\0".as_ptr() as *const c_char,
            cmd,
            measured_dir,
            core::ptr::null::<c_char>(),
        );
        exit(errno);
    } else if child_pid > 0 {
        waitpid(child_pid, &mut child_status, 0);
        return WEXITSTATUS(child_status);
    }

    -EINVAL
}

unsafe fn run_measured_process(measured_dir: *const c_char, monitored_pid: *mut u32) -> c_int {
    _run_measured_process(measured_dir, monitored_pid, b"run\0".as_ptr() as *const c_char)
}

static mut ima_hash_from_bpf: [u64; MAX_SAMPLES] = [0; MAX_SAMPLES];
static mut ima_hash_from_bpf_idx: c_int = 0;

unsafe extern "C" fn process_sample(
    _ctx: *mut c_void,
    data: *mut c_void,
    _len: size_t,
) -> c_int {
    if ima_hash_from_bpf_idx >= MAX_SAMPLES as c_int {
        return -ENOSPC;
    }

    ima_hash_from_bpf[ima_hash_from_bpf_idx as usize] = *(data as *mut u64);
    ima_hash_from_bpf_idx += 1;
    0
}

unsafe fn test_init(bss: *mut ima__bss) {
    ima_hash_from_bpf_idx = 0;

    (*bss).use_ima_file_hash = false;
    (*bss).enable_bprm_creds_for_exec = false;
    (*bss).enable_kernel_read_file = false;
    (*bss).test_deny = false;
}

#[no_mangle]
pub unsafe extern "C" fn test_test_ima() {
    let mut measured_dir_template = *b"/tmp/ima_measuredXXXXXX\0";
    let mut ringbuf: *mut ring_buffer = core::ptr::null_mut();
    let measured_dir: *const c_char;
    let mut bin_true_sample: u64 = 0;
    let mut cmd: [c_char; 256] = [0; 256];

    let mut err: c_int;
    let duration: c_int = 0;
    let mut fresh_digest_idx: c_int = 0;
    let mut skel: *mut ima = core::ptr::null_mut();

    skel = ima__open_and_load();
    if CHECK(
        skel.is_null(),
        b"skel_load\0".as_ptr() as *const c_char,
        b"skeleton failed\n\0".as_ptr() as *const c_char,
    ) {
        goto_close_prog(ringbuf, skel);
        return;
    }

    ringbuf = ring_buffer__new(
        bpf_map__fd((*skel).maps.ringbuf),
        Some(process_sample),
        NULL,
        NULL,
    );
    if !ASSERT_OK_PTR(ringbuf as *const c_void, b"ringbuf\0".as_ptr() as *const c_char) {
        goto_close_prog(ringbuf, skel);
        return;
    }

    err = ima__attach(skel);
    if CHECK(
        err != 0,
        b"attach\0".as_ptr() as *const c_char,
        b"attach failed: %d\n\0".as_ptr() as *const c_char,
        err,
    ) {
        goto_close_prog(ringbuf, skel);
        return;
    }

    measured_dir = mkdtemp(measured_dir_template.as_mut_ptr() as *mut c_char);
    if CHECK(
        measured_dir.is_null(),
        b"mkdtemp\0".as_ptr() as *const c_char,
        b"err %d\n\0".as_ptr() as *const c_char,
        errno,
    ) {
        goto_close_prog(ringbuf, skel);
        return;
    }

    snprintf(
        cmd.as_mut_ptr(),
        cmd.len(),
        b"./ima_setup.sh setup %s\0".as_ptr() as *const c_char,
        measured_dir,
    );
    err = system(cmd.as_ptr());
    if CHECK(
        err != 0,
        b"failed to run command\0".as_ptr() as *const c_char,
        b"%s, errno = %d\n\0".as_ptr() as *const c_char,
        cmd.as_ptr(),
        errno,
    ) {
        goto_close_clean(cmd.as_mut_ptr(), measured_dir, ringbuf, skel);
        return;
    }

    /*
     * Test #1
     * - Goal: obtain a sample with the bpf_ima_inode_hash() helper
     * - Expected result:  1 sample (/bin/true)
     */
    test_init((*skel).bss);
    err = run_measured_process(measured_dir, &mut (*(*skel).bss).monitored_pid);
    if CHECK(
        err != 0,
        b"run_measured_process #1\0".as_ptr() as *const c_char,
        b"err = %d\n\0".as_ptr() as *const c_char,
        err,
    ) {
        goto_close_clean(cmd.as_mut_ptr(), measured_dir, ringbuf, skel);
        return;
    }

    err = ring_buffer__consume(ringbuf);
    ASSERT_EQ(err as u64, 1, b"num_samples_or_err\0".as_ptr() as *const c_char);
    ASSERT_NEQ(ima_hash_from_bpf[0], 0, b"ima_hash\0".as_ptr() as *const c_char);

    /*
     * Test #2
     * - Goal: obtain samples with the bpf_ima_file_hash() helper
     * - Expected result: 2 samples (./ima_setup.sh, /bin/true)
     */
    test_init((*skel).bss);
    (*(*skel).bss).use_ima_file_hash = true;
    err = run_measured_process(measured_dir, &mut (*(*skel).bss).monitored_pid);
    if CHECK(
        err != 0,
        b"run_measured_process #2\0".as_ptr() as *const c_char,
        b"err = %d\n\0".as_ptr() as *const c_char,
        err,
    ) {
        goto_close_clean(cmd.as_mut_ptr(), measured_dir, ringbuf, skel);
        return;
    }

    err = ring_buffer__consume(ringbuf);
    ASSERT_EQ(err as u64, 2, b"num_samples_or_err\0".as_ptr() as *const c_char);
    ASSERT_NEQ(ima_hash_from_bpf[0], 0, b"ima_hash\0".as_ptr() as *const c_char);
    ASSERT_NEQ(ima_hash_from_bpf[1], 0, b"ima_hash\0".as_ptr() as *const c_char);
    bin_true_sample = ima_hash_from_bpf[1];

    /*
     * Test #3
     * - Goal: confirm that bpf_ima_inode_hash() returns a non-fresh digest
     * - Expected result:
     *   1 sample (/bin/true: fresh) if commit 62622dab0a28 applied
     *   2 samples (/bin/true: non-fresh, fresh) if commit 62622dab0a28 is
     *     not applied
     *
     * If commit 62622dab0a28 ("ima: return IMA digest value only when
     * IMA_COLLECTED flag is set") is applied, bpf_ima_inode_hash() refuses
     * to give a non-fresh digest, hence the correct result is 1 instead of
     * 2.
     */
    test_init((*skel).bss);

    err = _run_measured_process(
        measured_dir,
        &mut (*(*skel).bss).monitored_pid,
        b"modify-bin\0".as_ptr() as *const c_char,
    );
    if CHECK(
        err != 0,
        b"modify-bin #3\0".as_ptr() as *const c_char,
        b"err = %d\n\0".as_ptr() as *const c_char,
        err,
    ) {
        goto_close_clean(cmd.as_mut_ptr(), measured_dir, ringbuf, skel);
        return;
    }

    (*(*skel).bss).enable_bprm_creds_for_exec = true;
    err = run_measured_process(measured_dir, &mut (*(*skel).bss).monitored_pid);
    if CHECK(
        err != 0,
        b"run_measured_process #3\0".as_ptr() as *const c_char,
        b"err = %d\n\0".as_ptr() as *const c_char,
        err,
    ) {
        goto_close_clean(cmd.as_mut_ptr(), measured_dir, ringbuf, skel);
        return;
    }

    err = ring_buffer__consume(ringbuf);
    ASSERT_GE(err, 1, b"num_samples_or_err\0".as_ptr() as *const c_char);
    if err == 2 {
        ASSERT_NEQ(ima_hash_from_bpf[0], 0, b"ima_hash\0".as_ptr() as *const c_char);
        ASSERT_EQ(
            ima_hash_from_bpf[0],
            bin_true_sample,
            b"sample_equal_or_err\0".as_ptr() as *const c_char,
        );
        fresh_digest_idx = 1;
    }

    ASSERT_NEQ(
        ima_hash_from_bpf[fresh_digest_idx as usize],
        0,
        b"ima_hash\0".as_ptr() as *const c_char,
    );
    /* IMA refreshed the digest. */
    ASSERT_NEQ(
        ima_hash_from_bpf[fresh_digest_idx as usize],
        bin_true_sample,
        b"sample_equal_or_err\0".as_ptr() as *const c_char,
    );

    /*
     * Test #4
     * - Goal: verify that bpf_ima_file_hash() returns a fresh digest
     * - Expected result: 4 samples (./ima_setup.sh: fresh, fresh;
     *                               /bin/true: fresh, fresh)
     */
    test_init((*skel).bss);
    (*(*skel).bss).use_ima_file_hash = true;
    (*(*skel).bss).enable_bprm_creds_for_exec = true;
    err = run_measured_process(measured_dir, &mut (*(*skel).bss).monitored_pid);
    if CHECK(
        err != 0,
        b"run_measured_process #4\0".as_ptr() as *const c_char,
        b"err = %d\n\0".as_ptr() as *const c_char,
        err,
    ) {
        goto_close_clean(cmd.as_mut_ptr(), measured_dir, ringbuf, skel);
        return;
    }

    err = ring_buffer__consume(ringbuf);
    ASSERT_EQ(err as u64, 4, b"num_samples_or_err\0".as_ptr() as *const c_char);
    ASSERT_NEQ(ima_hash_from_bpf[0], 0, b"ima_hash\0".as_ptr() as *const c_char);
    ASSERT_NEQ(ima_hash_from_bpf[1], 0, b"ima_hash\0".as_ptr() as *const c_char);
    ASSERT_NEQ(ima_hash_from_bpf[2], 0, b"ima_hash\0".as_ptr() as *const c_char);
    ASSERT_NEQ(ima_hash_from_bpf[3], 0, b"ima_hash\0".as_ptr() as *const c_char);
    ASSERT_NEQ(
        ima_hash_from_bpf[2],
        bin_true_sample,
        b"sample_different_or_err\0".as_ptr() as *const c_char,
    );
    ASSERT_EQ(
        ima_hash_from_bpf[3],
        ima_hash_from_bpf[2],
        b"sample_equal_or_err\0".as_ptr() as *const c_char,
    );

    (*(*skel).bss).use_ima_file_hash = false;
    (*(*skel).bss).enable_bprm_creds_for_exec = false;
    err = _run_measured_process(
        measured_dir,
        &mut (*(*skel).bss).monitored_pid,
        b"restore-bin\0".as_ptr() as *const c_char,
    );
    if CHECK(
        err != 0,
        b"restore-bin #3\0".as_ptr() as *const c_char,
        b"err = %d\n\0".as_ptr() as *const c_char,
        err,
    ) {
        goto_close_clean(cmd.as_mut_ptr(), measured_dir, ringbuf, skel);
        return;
    }

    /*
     * Test #5
     * - Goal: obtain a sample from the kernel_read_file hook
     * - Expected result: 2 samples (./ima_setup.sh, policy_test)
     */
    test_init((*skel).bss);
    (*(*skel).bss).use_ima_file_hash = true;
    (*(*skel).bss).enable_kernel_read_file = true;
    err = _run_measured_process(
        measured_dir,
        &mut (*(*skel).bss).monitored_pid,
        b"load-policy\0".as_ptr() as *const c_char,
    );
    if CHECK(
        err != 0,
        b"run_measured_process #5\0".as_ptr() as *const c_char,
        b"err = %d\n\0".as_ptr() as *const c_char,
        err,
    ) {
        goto_close_clean(cmd.as_mut_ptr(), measured_dir, ringbuf, skel);
        return;
    }

    err = ring_buffer__consume(ringbuf);
    ASSERT_EQ(err as u64, 2, b"num_samples_or_err\0".as_ptr() as *const c_char);
    ASSERT_NEQ(ima_hash_from_bpf[0], 0, b"ima_hash\0".as_ptr() as *const c_char);
    ASSERT_NEQ(ima_hash_from_bpf[1], 0, b"ima_hash\0".as_ptr() as *const c_char);

    /*
     * Test #6
     * - Goal: ensure that the kernel_read_file hook denies an operation
     * - Expected result: 0 samples
     */
    test_init((*skel).bss);
    (*(*skel).bss).enable_kernel_read_file = true;
    (*(*skel).bss).test_deny = true;
    err = _run_measured_process(
        measured_dir,
        &mut (*(*skel).bss).monitored_pid,
        b"load-policy\0".as_ptr() as *const c_char,
    );
    if CHECK(
        err == 0,
        b"run_measured_process #6\0".as_ptr() as *const c_char,
        b"err = %d\n\0".as_ptr() as *const c_char,
        err,
    ) {
        goto_close_clean(cmd.as_mut_ptr(), measured_dir, ringbuf, skel);
        return;
    }

    err = ring_buffer__consume(ringbuf);
    ASSERT_EQ(err as u64, 0, b"num_samples_or_err\0".as_ptr() as *const c_char);

    goto_close_clean(cmd.as_mut_ptr(), measured_dir, ringbuf, skel);

    let _ = duration;
}

unsafe fn goto_close_clean(
    cmd: *mut c_char,
    measured_dir: *const c_char,
    ringbuf: *mut ring_buffer,
    skel: *mut ima,
) {
    let err: c_int;

    snprintf(
        cmd,
        256,
        b"./ima_setup.sh cleanup %s\0".as_ptr() as *const c_char,
        measured_dir,
    );
    err = system(cmd);
    CHECK(
        err != 0,
        b"failed to run command\0".as_ptr() as *const c_char,
        b"%s, errno = %d\n\0".as_ptr() as *const c_char,
        cmd,
        errno,
    );
    goto_close_prog(ringbuf, skel);
}

unsafe fn goto_close_prog(ringbuf: *mut ring_buffer, skel: *mut ima) {
    ring_buffer__free(ringbuf);
    ima__destroy(skel);
}
