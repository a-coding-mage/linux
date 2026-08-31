// SPDX-License-Identifier: GPL-2.0
// C dependencies: pthread.h, sched.h, sys/socket.h, test_progs.h,
// bpf/libbpf_internal.h, test_perf_branches.skel.h

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::mem::size_of;

#[repr(C)]
pub struct test_perf_branches {
    pub bss: *mut test_perf_branches_bss,
    pub progs: test_perf_branches_progs,
}

#[repr(C)]
pub struct test_perf_branches_bss {
    pub written_global_out: c_int,
    pub required_size_out: c_int,
    pub written_stack_out: c_int,
    pub run_cnt: c_int,
    pub valid: c_int,
}

#[repr(C)]
pub struct test_perf_branches_progs {
    pub perf_branches: *mut bpf_program,
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_branch_entry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_event_attr {
    pub type_: c_int,
    pub size: c_int,
    pub config: c_ulong,
    pub sample_period_or_freq: c_ulong,
    pub sample_type: c_ulong,
    pub read_format: c_ulong,
    pub flags: c_ulong,
    pub wakeup_events_or_watermark: c_int,
    pub bp_type: c_int,
    pub bp_addr_or_config1: c_ulong,
    pub bp_len_or_config2: c_ulong,
    pub branch_sample_type: c_ulong,
}

#[repr(C)]
pub struct cpu_set_t {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut errno: c_int;

    fn test_perf_branches__open_and_load() -> *mut test_perf_branches;
    fn test_perf_branches__detach(skel: *mut test_perf_branches);
    fn test_perf_branches__destroy(skel: *mut test_perf_branches);
    fn bpf_program__attach_perf_event(
        prog: *mut bpf_program,
        perf_fd: c_int,
    ) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link);

    fn CPU_ZERO(set: *mut cpu_set_t);
    fn CPU_SET(cpu: c_int, set: *mut cpu_set_t);
    fn pthread_self() -> c_ulong;
    fn pthread_setaffinity_np(thread: c_ulong, cpusetsize: usize, cpuset: *const cpu_set_t)
        -> c_int;

    fn syscall(num: c_long, ...) -> c_long;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn test__skip();
    fn test__start_subtest(name: *const c_char) -> bool;
}

const EINVAL: c_int = 22;
const ENOENT: c_int = 2;
const EOPNOTSUPP: c_int = 95;

const PERF_TYPE_HARDWARE: c_int = 0;
const PERF_TYPE_SOFTWARE: c_int = 1;
const PERF_COUNT_HW_CPU_CYCLES: c_ulong = 0;
const PERF_COUNT_SW_CPU_CLOCK: c_ulong = 0;
const PERF_SAMPLE_BRANCH_STACK: c_ulong = 1 << 11;
const PERF_SAMPLE_BRANCH_USER: c_ulong = 1 << 0;
const PERF_SAMPLE_BRANCH_ANY: c_ulong = 1 << 3;
const PERF_FLAG_FD_CLOEXEC: c_ulong = 1 << 3;
const __NR_perf_event_open: c_long = 298;

unsafe fn check_good_sample(skel: *mut test_perf_branches) {
    let written_global: c_int = (*(*skel).bss).written_global_out;
    let required_size: c_int = (*(*skel).bss).required_size_out;
    let written_stack: c_int = (*(*skel).bss).written_stack_out;
    let pbe_size: c_int = size_of::<perf_branch_entry>() as c_int;
    let duration: c_int = 0;
    let _ = duration;

    if CHECK!(
        (*(*skel).bss).run_cnt == 0,
        "invalid run_cnt",
        "checked sample validity before prog run"
    ) {
        return;
    }

    if CHECK!(
        (*(*skel).bss).valid == 0,
        "output not valid",
        "no valid sample from prog"
    ) {
        return;
    }

    /*
     * It's hard to validate the contents of the branch entries b/c it
     * would require some kind of disassembler and also encoding the
     * valid jump instructions for supported architectures. So just check
     * the easy stuff for now.
     */
    CHECK!(
        required_size <= 0,
        "read_branches_size",
        "err %d\n",
        required_size
    );
    CHECK!(
        written_stack < 0,
        "read_branches_stack",
        "err %d\n",
        written_stack
    );
    CHECK!(
        written_stack % pbe_size != 0,
        "read_branches_stack",
        "stack bytes written=%d not multiple of struct size=%d\n",
        written_stack,
        pbe_size
    );
    CHECK!(
        written_global < 0,
        "read_branches_global",
        "err %d\n",
        written_global
    );
    CHECK!(
        written_global % pbe_size != 0,
        "read_branches_global",
        "global bytes written=%d not multiple of struct size=%d\n",
        written_global,
        pbe_size
    );
    CHECK!(
        written_global < written_stack,
        "read_branches_size",
        "written_global=%d < written_stack=%d\n",
        written_global,
        written_stack
    );
}

unsafe fn check_bad_sample(skel: *mut test_perf_branches) {
    let written_global: c_int = (*(*skel).bss).written_global_out;
    let required_size: c_int = (*(*skel).bss).required_size_out;
    let written_stack: c_int = (*(*skel).bss).written_stack_out;
    let duration: c_int = 0;
    let _ = duration;

    if CHECK!(
        (*(*skel).bss).run_cnt == 0,
        "invalid run_cnt",
        "checked sample validity before prog run"
    ) {
        return;
    }

    if CHECK!(
        (*(*skel).bss).valid == 0,
        "output not valid",
        "no valid sample from prog"
    ) {
        return;
    }

    CHECK!(
        required_size != -EINVAL && required_size != -ENOENT,
        "read_branches_size",
        "err %d\n",
        required_size
    );
    CHECK!(
        written_stack != -EINVAL && written_stack != -ENOENT,
        "read_branches_stack",
        "written %d\n",
        written_stack
    );
    CHECK!(
        written_global != -EINVAL && written_global != -ENOENT,
        "read_branches_global",
        "written %d\n",
        written_global
    );
}

unsafe fn test_perf_branches_common(
    perf_fd: c_int,
    cb: unsafe fn(*mut test_perf_branches),
) {
    let skel: *mut test_perf_branches;
    let mut err: c_int;
    let mut i: c_int;
    let duration: c_int = 0;
    let mut detached: bool = false;
    let link: *mut bpf_link;
    let mut j: c_int = 0;
    let mut cpu_set: cpu_set_t = core::mem::zeroed();
    let _ = duration;

    skel = test_perf_branches__open_and_load();
    if CHECK!(
        skel.is_null(),
        "test_perf_branches_load",
        "perf_branches skeleton failed\n"
    ) {
        return;
    }

    /* attach perf_event */
    link = bpf_program__attach_perf_event((*skel).progs.perf_branches, perf_fd);
    if !ASSERT_OK_PTR!(link, "attach_perf_event") {
        test_perf_branches__destroy(skel);
        return;
    }

    /* generate some branches on cpu 0 */
    CPU_ZERO(&mut cpu_set);
    CPU_SET(0, &mut cpu_set);
    err = pthread_setaffinity_np(pthread_self(), size_of::<cpu_set_t>(), &cpu_set);
    if CHECK!(err != 0, "set_affinity", "cpu #0, err %d\n", err) {
        bpf_link__destroy(link);
        test_perf_branches__detach(skel);
        test_perf_branches__destroy(skel);
        return;
    }

    /* Spin the loop for a while by using a high iteration count, and by
     * checking whether the specific run count marker has been explicitly
     * incremented at least once by the backing perf_event BPF program.
     */
    i = 0;
    while i < 100000000
        && core::ptr::read_volatile(&(*(*skel).bss).run_cnt as *const c_int) == 0
    {
        j = j.wrapping_add(1);
        i += 1;
    }
    let _ = core::ptr::read_volatile(&j);

    test_perf_branches__detach(skel);
    detached = true;

    cb(skel);

    bpf_link__destroy(link);
    if !detached {
        test_perf_branches__detach(skel);
    }
    test_perf_branches__destroy(skel);
}

unsafe fn test_perf_branches_hw() {
    let mut attr: perf_event_attr = core::mem::zeroed();
    let duration: c_int = 0;
    let pfd: c_int;
    let _ = duration;

    /* create perf event */
    attr.size = size_of::<perf_event_attr>() as c_int;
    attr.type_ = PERF_TYPE_HARDWARE;
    attr.config = PERF_COUNT_HW_CPU_CYCLES;
    attr.flags |= 1 << 10;
    attr.sample_period_or_freq = 1000;
    attr.sample_type = PERF_SAMPLE_BRANCH_STACK;
    attr.branch_sample_type = PERF_SAMPLE_BRANCH_USER | PERF_SAMPLE_BRANCH_ANY;
    pfd = syscall(
        __NR_perf_event_open,
        &mut attr as *mut perf_event_attr,
        -1,
        0,
        -1,
        PERF_FLAG_FD_CLOEXEC,
    ) as c_int;

    /*
     * Some setups don't support LBR (virtual machines, !x86, AMD Milan Zen
     * 3 which only supports BRS), so skip test in this case.
     */
    if pfd < 0 {
        if errno == ENOENT || errno == EOPNOTSUPP || errno == EINVAL {
            printf(
                c"%s:SKIP:no PERF_SAMPLE_BRANCH_STACK\n".as_ptr(),
                c"test_perf_branches_hw".as_ptr(),
            );
            test__skip();
            return;
        }
        if CHECK!(
            pfd < 0,
            "perf_event_open",
            "err %d errno %d\n",
            pfd,
            errno
        ) {
            return;
        }
    }

    test_perf_branches_common(pfd, check_good_sample);

    close(pfd);
}

/*
 * Tests negative case -- run bpf_read_branch_records() on improperly configured
 * perf event.
 */
unsafe fn test_perf_branches_no_hw() {
    let mut attr: perf_event_attr = core::mem::zeroed();
    let duration: c_int = 0;
    let pfd: c_int;
    let _ = duration;

    /* create perf event */
    attr.size = size_of::<perf_event_attr>() as c_int;
    attr.type_ = PERF_TYPE_SOFTWARE;
    attr.config = PERF_COUNT_SW_CPU_CLOCK;
    attr.flags |= 1 << 10;
    attr.sample_period_or_freq = 1000;
    pfd = syscall(
        __NR_perf_event_open,
        &mut attr as *mut perf_event_attr,
        -1,
        0,
        -1,
        PERF_FLAG_FD_CLOEXEC,
    ) as c_int;
    if CHECK!(pfd < 0, "perf_event_open", "err %d\n", pfd) {
        return;
    }

    test_perf_branches_common(pfd, check_bad_sample);

    close(pfd);
}

#[no_mangle]
pub unsafe extern "C" fn test_perf_branches() {
    if test__start_subtest(c"perf_branches_hw".as_ptr()) {
        test_perf_branches_hw();
    }
    if test__start_subtest(c"perf_branches_no_hw".as_ptr()) {
        test_perf_branches_no_hw();
    }
}
