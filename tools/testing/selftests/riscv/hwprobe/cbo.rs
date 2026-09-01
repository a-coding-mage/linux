// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2023 Ventana Micro Systems Inc.
 *
 * Run with 'taskset -c <cpu-list> cbo' to only execute hwprobe on a
 * subset of cpus, as well as only executing the tests on those cpus.
 */

use core::arch::asm;
use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

// C dependencies: <sched.h>, <signal.h>, <getopt.h>, "hwprobe.h", "kselftest.h".

const fn le32_bswap(x: u32) -> u32 {
    x.to_le().swap_bytes()
}

const fn MK_CBO(fn_: u64) -> u32 {
    le32_bswap(((fn_ as u32) << 20) | (10 << 15) | (2 << 12) | (0 << 7) | 15)
}

const fn MK_PREFETCH(fn_: u64) -> u32 {
    le32_bswap((0 << 25) | ((fn_ as u32) << 20) | (10 << 15) | (6 << 12) | (0 << 7) | 19)
}

#[repr(C, align(4096))]
struct AlignedMem([c_char; 4096]);

static mut mem: AlignedMem = AlignedMem([0xa5u8 as c_char; 4096]);

static mut got_fault: bool = false;

type __u64 = u64;

#[repr(C)]
struct riscv_hwprobe {
    key: i64,
    value: __u64,
}

#[repr(C)]
struct test_info {
    enabled: bool,
    nr_tests: u32,
    test_fn: unsafe extern "C" fn(*mut c_void),
}

extern "C" {
    static mut stderr: *mut c_void;
    static mut optarg: *mut c_char;
    static mut optind: c_int;

    static RISCV_HWPROBE_KEY_ZICBOP_BLOCK_SIZE: i64;
    static RISCV_HWPROBE_KEY_ZICBOM_BLOCK_SIZE: i64;
    static RISCV_HWPROBE_KEY_ZICBOZ_BLOCK_SIZE: i64;
    static RISCV_HWPROBE_KEY_IMA_EXT_0: i64;
    static RISCV_HWPROBE_EXT_ZICBOZ: __u64;
    static RISCV_HWPROBE_EXT_ZICBOM: __u64;
    static RISCV_HWPROBE_EXT_ZICBOP: __u64;

    fn riscv_hwprobe(
        pairs: *mut riscv_hwprobe,
        pair_count: usize,
        cpusetsize: usize,
        cpus: *mut c_ulong,
        flags: u32,
    ) -> c_long;

    fn sigaction(sig: c_int, act: *const libc::sigaction, oldact: *mut libc::sigaction) -> c_int;
    fn sched_getaffinity(pid: c_int, cpusetsize: usize, mask: *mut libc::cpu_set_t) -> c_int;
    fn getopt_long(
        argc: c_int,
        argv: *mut *mut c_char,
        optstring: *const c_char,
        longopts: *const libc::option,
        longindex: *mut c_int,
    ) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;

    fn CPU_COUNT(set: *const libc::cpu_set_t) -> c_int;
    fn CPU_ISSET(cpu: c_int, set: *const libc::cpu_set_t) -> c_int;
    fn CPU_ZERO(set: *mut libc::cpu_set_t);
    fn CPU_SET(cpu: c_int, set: *mut libc::cpu_set_t);

    fn ksft_print_msg(format: *const c_char, ...);
    fn ksft_test_result(condition: bool, name: *const c_char, ...);
    fn ksft_test_result_skip(name: *const c_char, ...);
    fn ksft_test_result_fail(name: *const c_char, ...);
    fn ksft_test_result_pass(name: *const c_char, ...);
    fn ksft_exit_fail_msg(format: *const c_char, ...) -> !;
    fn ksft_print_header();
    fn ksft_set_plan(plan: u32);
    fn ksft_finished() -> !;
}

unsafe extern "C" fn fault_handler(sig: c_int, _info: *mut libc::siginfo_t, context: *mut c_void) {
    let regs = &mut (*(context as *mut libc::ucontext_t)).uc_mcontext as *mut _ as *mut c_ulong;
    let insn = *(ptr::read(regs.add(0)) as *const u32);

    if sig == libc::SIGILL {
        assert!(insn == MK_CBO(ptr::read(regs.add(11))));
    }

    if sig == libc::SIGSEGV || sig == libc::SIGBUS {
        assert!(insn == MK_PREFETCH(ptr::read(regs.add(11))));
    }

    got_fault = true;
    *regs.add(0) = (*regs.add(0)).wrapping_add(4);
}

unsafe fn cbo_insn(base: *mut c_char, const FN: u64) {
    asm!(
        "mv\ta0, {0}",
        "li\ta1, {1}",
        ".4byte\t{2}",
        in(reg) base,
        const FN,
        const MK_CBO(FN),
        out("a0") _,
        out("a1") _,
        options(nostack)
    );
}

unsafe fn prefetch_insn(base: *mut c_char, const FN: u64) {
    asm!(
        "mv\ta0, {0}",
        "li\ta1, {1}",
        ".4byte\t{2}",
        in(reg) base,
        const FN,
        const MK_PREFETCH(FN),
        out("a0") _,
        out("a1") _,
        options(nostack)
    );
}

unsafe fn cbo_inval(base: *mut c_char) {
    cbo_insn::<0>(base);
}
unsafe fn cbo_clean(base: *mut c_char) {
    cbo_insn::<1>(base);
}
unsafe fn cbo_flush(base: *mut c_char) {
    cbo_insn::<2>(base);
}
unsafe fn cbo_zero(base: *mut c_char) {
    cbo_insn::<4>(base);
}
unsafe fn prefetch_i(base: *mut c_char) {
    prefetch_insn::<0>(base);
}
unsafe fn prefetch_r(base: *mut c_char) {
    prefetch_insn::<1>(base);
}
unsafe fn prefetch_w(base: *mut c_char) {
    prefetch_insn::<3>(base);
}

unsafe extern "C" fn test_no_cbo_inval(_arg: *mut c_void) {
    ksft_print_msg(c"Testing cbo.inval instruction remain privileged\n".as_ptr());
    got_fault = false;
    cbo_inval(mem.0.as_mut_ptr());
    ksft_test_result(got_fault, c"No cbo.inval\n".as_ptr());
}

unsafe extern "C" fn test_no_zicbom(_arg: *mut c_void) {
    ksft_print_msg(c"Testing Zicbom instructions remain privileged\n".as_ptr());

    got_fault = false;
    cbo_clean(mem.0.as_mut_ptr());
    ksft_test_result(got_fault, c"No cbo.clean\n".as_ptr());

    got_fault = false;
    cbo_flush(mem.0.as_mut_ptr());
    ksft_test_result(got_fault, c"No cbo.flush\n".as_ptr());
}

unsafe extern "C" fn test_no_zicboz(_arg: *mut c_void) {
    ksft_print_msg(c"No Zicboz, testing cbo.zero remains privileged\n".as_ptr());

    got_fault = false;
    cbo_zero(mem.0.as_mut_ptr());
    ksft_test_result(got_fault, c"No cbo.zero\n".as_ptr());
}

fn is_power_of_2(n: __u64) -> bool {
    n != 0 && (n & (n - 1)) == 0
}

unsafe extern "C" fn test_zicbop(arg: *mut c_void) {
    let mut pair = riscv_hwprobe {
        key: RISCV_HWPROBE_KEY_ZICBOP_BLOCK_SIZE,
        value: 0,
    };
    let mut act: libc::sigaction = core::mem::zeroed();
    act.sa_sigaction = fault_handler as usize;
    act.sa_flags = libc::SA_SIGINFO;
    let mut dfl: libc::sigaction = core::mem::zeroed();
    dfl.sa_sigaction = libc::SIG_DFL;
    let cpus = arg as *mut libc::cpu_set_t;
    let block_size: __u64;
    let mut rc: c_long;

    rc = sigaction(libc::SIGSEGV, &act, ptr::null_mut()) as c_long;
    assert!(rc == 0);
    rc = sigaction(libc::SIGBUS, &act, ptr::null_mut()) as c_long;
    assert!(rc == 0);

    rc = riscv_hwprobe(&mut pair, 1, size_of::<libc::cpu_set_t>(), cpus as *mut c_ulong, 0);
    block_size = pair.value;
    ksft_test_result(
        rc == 0
            && pair.key == RISCV_HWPROBE_KEY_ZICBOP_BLOCK_SIZE
            && is_power_of_2(block_size),
        c"Zicbop block size\n".as_ptr(),
    );
    ksft_print_msg(c"Zicbop block size: %llu\n".as_ptr(), block_size);

    got_fault = false;
    prefetch_i(mem.0.as_mut_ptr());
    prefetch_r(mem.0.as_mut_ptr());
    prefetch_w(mem.0.as_mut_ptr());
    ksft_test_result(!got_fault, c"Zicbop prefetch.* on valid address\n".as_ptr());

    got_fault = false;
    prefetch_i(ptr::null_mut());
    prefetch_r(ptr::null_mut());
    prefetch_w(ptr::null_mut());
    ksft_test_result(!got_fault, c"Zicbop prefetch.* on NULL\n".as_ptr());

    rc = sigaction(libc::SIGBUS, &dfl, ptr::null_mut()) as c_long;
    assert!(rc == 0);
    rc = sigaction(libc::SIGSEGV, &dfl, ptr::null_mut()) as c_long;
    assert!(rc == 0);
}

unsafe extern "C" fn test_zicbom(arg: *mut c_void) {
    let mut pair = riscv_hwprobe {
        key: RISCV_HWPROBE_KEY_ZICBOM_BLOCK_SIZE,
        value: 0,
    };
    let cpus = arg as *mut libc::cpu_set_t;
    let block_size: __u64;
    let rc: c_long;

    rc = riscv_hwprobe(&mut pair, 1, size_of::<libc::cpu_set_t>(), cpus as *mut c_ulong, 0);
    block_size = pair.value;
    ksft_test_result(
        rc == 0
            && pair.key == RISCV_HWPROBE_KEY_ZICBOM_BLOCK_SIZE
            && is_power_of_2(block_size),
        c"Zicbom block size\n".as_ptr(),
    );
    ksft_print_msg(c"Zicbom block size: %llu\n".as_ptr(), block_size);

    got_fault = false;
    cbo_clean(mem.0.as_mut_ptr().add(block_size as usize));
    ksft_test_result(!got_fault, c"cbo.clean\n".as_ptr());

    got_fault = false;
    cbo_flush(mem.0.as_mut_ptr().add(block_size as usize));
    ksft_test_result(!got_fault, c"cbo.flush\n".as_ptr());
}

unsafe extern "C" fn test_zicboz(arg: *mut c_void) {
    let mut pair = riscv_hwprobe {
        key: RISCV_HWPROBE_KEY_ZICBOZ_BLOCK_SIZE,
        value: 0,
    };
    let cpus = arg as *mut libc::cpu_set_t;
    let block_size: __u64;
    let mut i: c_int;
    let mut j: c_int;
    let rc: c_long;

    rc = riscv_hwprobe(&mut pair, 1, size_of::<libc::cpu_set_t>(), cpus as *mut c_ulong, 0);
    block_size = pair.value;
    ksft_test_result(
        rc == 0
            && pair.key == RISCV_HWPROBE_KEY_ZICBOZ_BLOCK_SIZE
            && is_power_of_2(block_size),
        c"Zicboz block size\n".as_ptr(),
    );
    ksft_print_msg(c"Zicboz block size: %llu\n".as_ptr(), block_size);

    got_fault = false;
    cbo_zero(mem.0.as_mut_ptr().add(block_size as usize));
    ksft_test_result(!got_fault, c"cbo.zero\n".as_ptr());

    if got_fault || !is_power_of_2(block_size) {
        ksft_test_result_skip(c"cbo.zero check\n".as_ptr());
        return;
    }

    assert!(block_size <= 1024);

    i = 0;
    while (i as __u64) < 4096 / block_size {
        if i % 2 != 0 {
            cbo_zero(mem.0.as_mut_ptr().add((i as __u64 * block_size) as usize));
        }
        i += 1;
    }

    i = 0;
    while (i as __u64) < 4096 / block_size {
        let expected: c_char = if i % 2 != 0 { 0x0 } else { 0xa5u8 as c_char };

        j = 0;
        while (j as __u64) < block_size {
            if mem.0[(i as __u64 * block_size + j as __u64) as usize] != expected {
                ksft_test_result_fail(c"cbo.zero check\n".as_ptr());
                ksft_print_msg(
                    c"cbo.zero check: mem[%llu] != 0x%x\n".as_ptr(),
                    i as __u64 * block_size + j as __u64,
                    expected as c_int,
                );
                return;
            }
            j += 1;
        }
        i += 1;
    }

    ksft_test_result_pass(c"cbo.zero check\n".as_ptr());
}

unsafe fn check_no_zicbo_cpus(cpus: *mut libc::cpu_set_t, cbo: __u64) {
    let mut pair = riscv_hwprobe {
        key: RISCV_HWPROBE_KEY_IMA_EXT_0,
        value: 0,
    };
    let mut one_cpu: libc::cpu_set_t = core::mem::zeroed();
    let mut i: c_int = 0;
    let mut c: c_int = 0;
    let mut rc: c_long;
    let cbostr: *const c_char;

    while {
        i += 1;
        i < CPU_COUNT(cpus)
    } {
        while CPU_ISSET(c, cpus) == 0 {
            c += 1;
        }

        CPU_ZERO(&mut one_cpu);
        CPU_SET(c, &mut one_cpu);

        rc = riscv_hwprobe(
            &mut pair,
            1,
            size_of::<libc::cpu_set_t>(),
            &mut one_cpu as *mut libc::cpu_set_t as *mut c_ulong,
            0,
        );
        assert!(rc == 0 && pair.key == RISCV_HWPROBE_KEY_IMA_EXT_0);

        if cbo == RISCV_HWPROBE_EXT_ZICBOZ {
            cbostr = c"Zicboz".as_ptr();
        } else if cbo == RISCV_HWPROBE_EXT_ZICBOM {
            cbostr = c"Zicbom".as_ptr();
        } else if cbo == RISCV_HWPROBE_EXT_ZICBOP {
            cbostr = c"Zicbop".as_ptr();
        } else {
            ksft_exit_fail_msg(c"Internal error: invalid cbo %llu\n".as_ptr(), cbo);
        }

        if pair.value & cbo != 0 {
            ksft_exit_fail_msg(
                c"%s is only present on a subset of harts.\nUse taskset to select a set of harts where %s\npresence (present or not) is consistent for each hart\n".as_ptr(),
                cbostr,
                cbostr,
            );
        }
        c += 1;
    }
}

const TEST_ZICBOZ: usize = 0;
const TEST_NO_ZICBOZ: usize = 1;
const TEST_ZICBOM: usize = 2;
const TEST_NO_ZICBOM: usize = 3;
const TEST_NO_CBO_INVAL: usize = 4;
const TEST_ZICBOP: usize = 5;

static mut tests: [test_info; 6] = [
    test_info {
        enabled: false,
        nr_tests: 3,
        test_fn: test_zicboz,
    },
    test_info {
        enabled: false,
        nr_tests: 1,
        test_fn: test_no_zicboz,
    },
    test_info {
        enabled: false,
        nr_tests: 3,
        test_fn: test_zicbom,
    },
    test_info {
        enabled: false,
        nr_tests: 2,
        test_fn: test_no_zicbom,
    },
    test_info {
        enabled: false,
        nr_tests: 1,
        test_fn: test_no_cbo_inval,
    },
    test_info {
        enabled: false,
        nr_tests: 3,
        test_fn: test_zicbop,
    },
];

static long_opts: [libc::option; 3] = [
    libc::option {
        name: c"zicbom-raises-sigill".as_ptr(),
        has_arg: libc::no_argument,
        flag: ptr::null_mut(),
        val: 'm' as c_int,
    },
    libc::option {
        name: c"zicboz-raises-sigill".as_ptr(),
        has_arg: libc::no_argument,
        flag: ptr::null_mut(),
        val: 'z' as c_int,
    },
    libc::option {
        name: ptr::null(),
        has_arg: 0,
        flag: ptr::null_mut(),
        val: 0,
    },
];

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut act: libc::sigaction = core::mem::zeroed();
    act.sa_sigaction = fault_handler as usize;
    act.sa_flags = libc::SA_SIGINFO;
    let mut pair: riscv_hwprobe = core::mem::zeroed();
    let mut plan: u32 = 0;
    let mut cpus: libc::cpu_set_t = core::mem::zeroed();
    let mut rc: c_long;
    let mut i: c_int;
    let mut opt: c_int;
    let mut long_index: c_int;

    long_index = 0;

    loop {
        opt = getopt_long(argc, argv, c"mz".as_ptr(), long_opts.as_ptr(), &mut long_index);
        if opt == -1 {
            break;
        }
        match opt {
            x if x == 'm' as c_int => {
                tests[TEST_NO_ZICBOM].enabled = true;
                tests[TEST_NO_CBO_INVAL].enabled = true;
                rc = sigaction(libc::SIGILL, &act, ptr::null_mut()) as c_long;
                assert!(rc == 0);
            }
            x if x == 'z' as c_int => {
                tests[TEST_NO_ZICBOZ].enabled = true;
                tests[TEST_NO_CBO_INVAL].enabled = true;
                rc = sigaction(libc::SIGILL, &act, ptr::null_mut()) as c_long;
                assert!(rc == 0);
            }
            x if x == '?' as c_int => {
                fprintf(
                    stderr,
                    c"Usage: %s [--zicbom-raises-sigill|-m] [--zicboz-raises-sigill|-z]\n".as_ptr(),
                    *argv.add(0),
                );
                exit(1);
            }
            _ => {}
        }
    }

    rc = sched_getaffinity(0, size_of::<libc::cpu_set_t>(), &mut cpus) as c_long;
    assert!(rc == 0);

    ksft_print_header();

    pair.key = RISCV_HWPROBE_KEY_IMA_EXT_0;
    rc = riscv_hwprobe(
        &mut pair,
        1,
        size_of::<libc::cpu_set_t>(),
        &mut cpus as *mut libc::cpu_set_t as *mut c_ulong,
        0,
    );
    if rc < 0 {
        ksft_exit_fail_msg(c"hwprobe() failed with %ld\n".as_ptr(), rc);
    }
    assert!(rc == 0 && pair.key == RISCV_HWPROBE_KEY_IMA_EXT_0);

    if pair.value & RISCV_HWPROBE_EXT_ZICBOZ != 0 {
        tests[TEST_ZICBOZ].enabled = true;
        tests[TEST_NO_ZICBOZ].enabled = false;
    } else {
        check_no_zicbo_cpus(&mut cpus, RISCV_HWPROBE_EXT_ZICBOZ);
    }

    if pair.value & RISCV_HWPROBE_EXT_ZICBOM != 0 {
        tests[TEST_ZICBOM].enabled = true;
        tests[TEST_NO_ZICBOM].enabled = false;
    } else {
        check_no_zicbo_cpus(&mut cpus, RISCV_HWPROBE_EXT_ZICBOM);
    }

    if pair.value & RISCV_HWPROBE_EXT_ZICBOP != 0 {
        tests[TEST_ZICBOP].enabled = true;
    } else {
        check_no_zicbo_cpus(&mut cpus, RISCV_HWPROBE_EXT_ZICBOP);
    }

    i = 0;
    while (i as usize) < tests.len() {
        plan += if tests[i as usize].enabled {
            tests[i as usize].nr_tests
        } else {
            0
        };
        i += 1;
    }

    if plan == 0 {
        ksft_print_msg(c"No tests enabled.\n".as_ptr());
    } else {
        ksft_set_plan(plan);
    }

    i = 0;
    while (i as usize) < tests.len() {
        if tests[i as usize].enabled {
            (tests[i as usize].test_fn)(&mut cpus as *mut libc::cpu_set_t as *mut c_void);
        }
        i += 1;
    }

    ksft_finished();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
