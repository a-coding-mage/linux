// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2023 Ventana Micro Systems Inc.
 *
 * Test the RISCV_HWPROBE_WHICH_CPUS flag of hwprobe. Also provides a command
 * line interface to get the cpu list for arbitrary hwprobe pairs.
 */
// C source defined _GNU_SOURCE and included stdio, stdlib, string, sched,
// unistd, assert, "hwprobe.h", and "kselftest.h".

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

pub type __u64 = u64;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct riscv_hwprobe {
    pub key: i64,
    pub value: u64,
}

const CPU_SETSIZE: usize = 1024;
const __NCPUBITS: usize = 8 * core::mem::size_of::<c_ulong>();

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_set_t {
    pub __bits: [c_ulong; CPU_SETSIZE / __NCPUBITS],
}

// Constants supplied by hwprobe.h / system headers.
extern "C" {
    static RISCV_HWPROBE_WHICH_CPUS: c_ulong;
    static RISCV_HWPROBE_KEY_BASE_BEHAVIOR: i64;
    static RISCV_HWPROBE_BASE_BEHAVIOR_IMA: u64;
    static RISCV_HWPROBE_KEY_IMA_EXT_0: i64;
    static RISCV_HWPROBE_KEY_IMA_EXT_1: i64;
    static EINVAL: c_int;
    static _SC_NPROCESSORS_ONLN: c_int;
}

extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn sched_getaffinity(pid: c_int, cpusetsize: usize, mask: *mut cpu_set_t) -> c_int;
    fn sysconf(name: c_int) -> c_long;

    fn riscv_hwprobe(
        pairs: *mut riscv_hwprobe,
        pair_count: usize,
        cpusetsize: usize,
        cpus: *mut c_ulong,
        flags: c_ulong,
    ) -> c_long;

    fn ksft_print_header();
    fn ksft_set_plan(plan: c_int);
    fn ksft_test_result(condition: bool, format: *const c_char, ...);
    fn ksft_finished();
}

unsafe fn CPU_ZERO(cpus: *mut cpu_set_t) {
    (*cpus).__bits = [0; CPU_SETSIZE / __NCPUBITS];
}

unsafe fn CPU_ISSET(cpu: c_int, cpus: *const cpu_set_t) -> c_int {
    let cpu = cpu as usize;
    (((*cpus).__bits[cpu / __NCPUBITS] & (1 as c_ulong).wrapping_shl((cpu % __NCPUBITS) as u32)) != 0)
        as c_int
}

unsafe fn CPU_COUNT(cpus: *const cpu_set_t) -> c_int {
    let mut count: c_int = 0;
    let mut i = 0;

    while i < CPU_SETSIZE / __NCPUBITS {
        count += (*cpus).__bits[i].count_ones() as c_int;
        i += 1;
    }

    count
}

unsafe fn CPU_EQUAL(cpus1: *const cpu_set_t, cpus2: *const cpu_set_t) -> c_int {
    let mut i = 0;

    while i < CPU_SETSIZE / __NCPUBITS {
        if (*cpus1).__bits[i] != (*cpus2).__bits[i] {
            return 0;
        }
        i += 1;
    }

    1
}

unsafe fn help() {
    printf(
        b"\n\
which-cpus: [-h] [<key=value> [<key=value> ...]]\n\n\
   Without parameters, tests the RISCV_HWPROBE_WHICH_CPUS flag of hwprobe.\n\
   With parameters, where each parameter is a hwprobe pair written as\n\
   <key=value>, outputs the cpulist for cpus which all match the given set\n\
   of pairs.  'key' and 'value' should be in numeric form, e.g. 4=0x3b\n\0"
            .as_ptr() as *const c_char,
    );
}

unsafe fn print_cpulist(cpus: *mut cpu_set_t) {
    let mut start: c_int = 0;
    let mut end: c_int = 0;

    if CPU_COUNT(cpus) == 0 {
        printf(b"cpus: None\n\0".as_ptr() as *const c_char);
        return;
    }

    printf(b"cpus:\0".as_ptr() as *const c_char);
    let mut i: c_int = 0;
    let mut c: c_int = 0;
    while i < CPU_COUNT(cpus) {
        if start != end && CPU_ISSET(c, cpus) == 0 {
            printf(b"-%d\0".as_ptr() as *const c_char, end);
        }

        while CPU_ISSET(c, cpus) == 0 {
            c += 1;
        }

        if i != 0 && c == end + 1 {
            end = c;
            i += 1;
            c += 1;
            continue;
        }

        printf(
            b"%c%d\0".as_ptr() as *const c_char,
            if i == 0 { b' ' as c_int } else { b',' as c_int },
            c,
        );
        start = c;
        end = c;

        i += 1;
        c += 1;
    }
    if start != end {
        printf(b"-%d\0".as_ptr() as *const c_char, end);
    }
    printf(b"\n\0".as_ptr() as *const c_char);
}

unsafe fn do_which_cpus(argc: c_int, argv: *mut *mut c_char, cpus: *mut cpu_set_t) {
    let pairs: *mut riscv_hwprobe;
    let nr_pairs: c_int = argc - 1;
    let mut start: *mut c_char;
    let mut end: *mut c_char = core::ptr::null_mut();
    let rc: c_int;

    pairs = malloc((nr_pairs as usize) * core::mem::size_of::<riscv_hwprobe>()) as *mut riscv_hwprobe;
    assert!(!pairs.is_null());

    let mut i: c_int = 0;
    while i < nr_pairs {
        start = *argv.offset((i + 1) as isize);
        (*pairs.offset(i as isize)).key = strtol(start, &mut end, 0) as i64;
        assert!(end != start && *end == b'=' as c_char);
        start = end.offset(1);
        (*pairs.offset(i as isize)).value = strtoul(start, &mut end, 0) as u64;
        assert!(end != start && *end == b'\0' as c_char);
        i += 1;
    }

    rc = riscv_hwprobe(
        pairs,
        nr_pairs as usize,
        core::mem::size_of::<cpu_set_t>(),
        cpus as *mut c_ulong,
        RISCV_HWPROBE_WHICH_CPUS,
    ) as c_int;
    assert!(rc == 0);
    print_cpulist(cpus);
    free(pairs as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut pairs: [riscv_hwprobe; 3] = [
        riscv_hwprobe { key: 0, value: 0 },
        riscv_hwprobe { key: 0, value: 0 },
        riscv_hwprobe { key: 0, value: 0 },
    ];
    let mut cpus_aff = cpu_set_t {
        __bits: [0; CPU_SETSIZE / __NCPUBITS],
    };
    let mut cpus = cpu_set_t {
        __bits: [0; CPU_SETSIZE / __NCPUBITS],
    };
    let ext0_all: __u64;
    let ext1_all: __u64;
    let mut rc: c_long;

    rc = sched_getaffinity(0, core::mem::size_of::<cpu_set_t>(), &mut cpus_aff);
    assert!(rc == 0);

    if argc > 1 {
        if strcmp(*argv.offset(1), b"-h\0".as_ptr() as *const c_char) == 0 {
            help();
        } else {
            do_which_cpus(argc, argv, &mut cpus_aff);
        }
        return 0;
    }

    ksft_print_header();
    ksft_set_plan(7);

    pairs[0] = riscv_hwprobe {
        key: RISCV_HWPROBE_KEY_BASE_BEHAVIOR,
        value: 0,
    };
    rc = riscv_hwprobe(pairs.as_mut_ptr(), 1, 0, core::ptr::null_mut(), 0);
    assert!(
        rc == 0
            && pairs[0].key == RISCV_HWPROBE_KEY_BASE_BEHAVIOR
            && pairs[0].value == RISCV_HWPROBE_BASE_BEHAVIOR_IMA
    );

    pairs[0] = riscv_hwprobe {
        key: RISCV_HWPROBE_KEY_IMA_EXT_0,
        value: 0,
    };
    rc = riscv_hwprobe(pairs.as_mut_ptr(), 1, 0, core::ptr::null_mut(), 0);
    assert!(rc == 0 && pairs[0].key == RISCV_HWPROBE_KEY_IMA_EXT_0);
    ext0_all = pairs[0].value;

    pairs[0] = riscv_hwprobe {
        key: RISCV_HWPROBE_KEY_IMA_EXT_1,
        value: 0,
    };
    rc = riscv_hwprobe(pairs.as_mut_ptr(), 1, 0, core::ptr::null_mut(), 0);
    assert!(rc == 0 && pairs[0].key == RISCV_HWPROBE_KEY_IMA_EXT_1);
    ext1_all = pairs[0].value;

    pairs[0] = riscv_hwprobe {
        key: RISCV_HWPROBE_KEY_BASE_BEHAVIOR,
        value: RISCV_HWPROBE_BASE_BEHAVIOR_IMA,
    };
    CPU_ZERO(&mut cpus);
    rc = riscv_hwprobe(
        pairs.as_mut_ptr(),
        1,
        0,
        &mut cpus as *mut cpu_set_t as *mut c_ulong,
        RISCV_HWPROBE_WHICH_CPUS,
    );
    ksft_test_result(rc == -(EINVAL as c_long), b"no cpusetsize\n\0".as_ptr() as *const c_char);

    pairs[0] = riscv_hwprobe {
        key: RISCV_HWPROBE_KEY_BASE_BEHAVIOR,
        value: RISCV_HWPROBE_BASE_BEHAVIOR_IMA,
    };
    rc = riscv_hwprobe(
        pairs.as_mut_ptr(),
        1,
        core::mem::size_of::<cpu_set_t>(),
        core::ptr::null_mut(),
        RISCV_HWPROBE_WHICH_CPUS,
    );
    ksft_test_result(rc == -(EINVAL as c_long), b"NULL cpus\n\0".as_ptr() as *const c_char);

    pairs[0] = riscv_hwprobe {
        key: 0xbadc0de,
        value: 0,
    };
    CPU_ZERO(&mut cpus);
    rc = riscv_hwprobe(
        pairs.as_mut_ptr(),
        1,
        core::mem::size_of::<cpu_set_t>(),
        &mut cpus as *mut cpu_set_t as *mut c_ulong,
        RISCV_HWPROBE_WHICH_CPUS,
    );
    ksft_test_result(
        rc == 0 && CPU_COUNT(&cpus) == 0,
        b"unknown key\n\0".as_ptr() as *const c_char,
    );

    pairs[0] = riscv_hwprobe {
        key: RISCV_HWPROBE_KEY_BASE_BEHAVIOR,
        value: RISCV_HWPROBE_BASE_BEHAVIOR_IMA,
    };
    pairs[1] = riscv_hwprobe {
        key: RISCV_HWPROBE_KEY_BASE_BEHAVIOR,
        value: RISCV_HWPROBE_BASE_BEHAVIOR_IMA,
    };
    CPU_ZERO(&mut cpus);
    rc = riscv_hwprobe(
        pairs.as_mut_ptr(),
        2,
        core::mem::size_of::<cpu_set_t>(),
        &mut cpus as *mut cpu_set_t as *mut c_ulong,
        RISCV_HWPROBE_WHICH_CPUS,
    );
    ksft_test_result(rc == 0, b"duplicate keys\n\0".as_ptr() as *const c_char);

    pairs[0] = riscv_hwprobe {
        key: RISCV_HWPROBE_KEY_BASE_BEHAVIOR,
        value: RISCV_HWPROBE_BASE_BEHAVIOR_IMA,
    };
    pairs[1] = riscv_hwprobe {
        key: RISCV_HWPROBE_KEY_IMA_EXT_0,
        value: ext0_all,
    };
    pairs[2] = riscv_hwprobe {
        key: RISCV_HWPROBE_KEY_IMA_EXT_1,
        value: ext1_all,
    };
    CPU_ZERO(&mut cpus);
    rc = riscv_hwprobe(
        pairs.as_mut_ptr(),
        3,
        core::mem::size_of::<cpu_set_t>(),
        &mut cpus as *mut cpu_set_t as *mut c_ulong,
        RISCV_HWPROBE_WHICH_CPUS,
    );
    ksft_test_result(
        rc == 0 && CPU_COUNT(&cpus) as c_long == sysconf(_SC_NPROCESSORS_ONLN),
        b"set all cpus\n\0".as_ptr() as *const c_char,
    );

    pairs[0] = riscv_hwprobe {
        key: RISCV_HWPROBE_KEY_BASE_BEHAVIOR,
        value: RISCV_HWPROBE_BASE_BEHAVIOR_IMA,
    };
    pairs[1] = riscv_hwprobe {
        key: RISCV_HWPROBE_KEY_IMA_EXT_0,
        value: ext0_all,
    };
    pairs[2] = riscv_hwprobe {
        key: RISCV_HWPROBE_KEY_IMA_EXT_1,
        value: ext1_all,
    };
    memcpy(
        &mut cpus as *mut cpu_set_t as *mut c_void,
        &cpus_aff as *const cpu_set_t as *const c_void,
        core::mem::size_of::<cpu_set_t>(),
    );
    rc = riscv_hwprobe(
        pairs.as_mut_ptr(),
        3,
        core::mem::size_of::<cpu_set_t>(),
        &mut cpus as *mut cpu_set_t as *mut c_ulong,
        RISCV_HWPROBE_WHICH_CPUS,
    );
    ksft_test_result(
        rc == 0 && CPU_EQUAL(&cpus, &cpus_aff) != 0,
        b"set all affinity cpus\n\0".as_ptr() as *const c_char,
    );

    pairs[0] = riscv_hwprobe {
        key: RISCV_HWPROBE_KEY_BASE_BEHAVIOR,
        value: RISCV_HWPROBE_BASE_BEHAVIOR_IMA,
    };
    pairs[1] = riscv_hwprobe {
        key: RISCV_HWPROBE_KEY_IMA_EXT_0,
        value: !ext0_all,
    };
    pairs[2] = riscv_hwprobe {
        key: RISCV_HWPROBE_KEY_IMA_EXT_1,
        value: !ext1_all,
    };
    memcpy(
        &mut cpus as *mut cpu_set_t as *mut c_void,
        &cpus_aff as *const cpu_set_t as *const c_void,
        core::mem::size_of::<cpu_set_t>(),
    );
    rc = riscv_hwprobe(
        pairs.as_mut_ptr(),
        3,
        core::mem::size_of::<cpu_set_t>(),
        &mut cpus as *mut cpu_set_t as *mut c_ulong,
        RISCV_HWPROBE_WHICH_CPUS,
    );
    ksft_test_result(
        rc == 0 && CPU_COUNT(&cpus) == 0,
        b"clear all cpus\n\0".as_ptr() as *const c_char,
    );

    ksft_finished();
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
