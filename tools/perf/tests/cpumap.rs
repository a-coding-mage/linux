// SPDX-License-Identifier: GPL-2.0
// Translated from perf/tests/cpumap.c.
// Original C dependencies: tests.h, cpumap.h, event.h,
// util/synthetic-events.h, linux/bitops.h, internal/cpumap.h, debug.h.

use core::ffi::{c_char, c_int, c_uint, c_void};

struct machine;

unsafe extern "C" fn process_event_mask(
    tool: *const perf_tool,
    event: *mut perf_event,
    sample: *mut perf_sample,
    machine: *mut machine,
) -> c_int {
    let _ = tool;
    let _ = sample;
    let _ = machine;
    let map_event: *mut perf_record_cpu_map = unsafe { &mut (*event).cpu_map };
    let data: *mut perf_record_cpu_map_data;
    let map: *mut perf_cpu_map;
    let long_size: c_uint;

    data = unsafe { &mut (*map_event).data };

    TEST_ASSERT_VAL!(c"wrong type".as_ptr(), unsafe {
        (*data).type_ == PERF_CPU_MAP__MASK
    });

    long_size = unsafe { (*data).mask32_data.long_size };

    TEST_ASSERT_VAL!(c"wrong long_size".as_ptr(), long_size == 4 || long_size == 8);

    TEST_ASSERT_VAL!(c"wrong nr".as_ptr(), unsafe {
        (*data).mask32_data.nr == 1
    });

    TEST_ASSERT_VAL!(c"wrong cpu".as_ptr(), unsafe {
        perf_record_cpu_map_data__test_bit(0, data)
    });
    TEST_ASSERT_VAL!(c"wrong cpu".as_ptr(), unsafe {
        !perf_record_cpu_map_data__test_bit(1, data)
    });
    for i in 2..=20 {
        TEST_ASSERT_VAL!(c"wrong cpu".as_ptr(), unsafe {
            perf_record_cpu_map_data__test_bit(i, data)
        });
    }

    map = unsafe { cpu_map__new_data(data) };
    TEST_ASSERT_VAL!(c"wrong nr".as_ptr(), unsafe { perf_cpu_map__nr(map) == 20 });

    TEST_ASSERT_VAL!(c"wrong cpu".as_ptr(), unsafe {
        perf_cpu_map__cpu(map, 0).cpu == 0
    });
    for i in 2..=20 {
        TEST_ASSERT_VAL!(c"wrong cpu".as_ptr(), unsafe {
            perf_cpu_map__cpu(map, i - 1).cpu == i
        });
    }

    unsafe { perf_cpu_map__put(map) };
    0
}

unsafe extern "C" fn process_event_cpus(
    tool: *const perf_tool,
    event: *mut perf_event,
    sample: *mut perf_sample,
    machine: *mut machine,
) -> c_int {
    let _ = tool;
    let _ = sample;
    let _ = machine;
    let map_event: *mut perf_record_cpu_map = unsafe { &mut (*event).cpu_map };
    let data: *mut perf_record_cpu_map_data;
    let map: *mut perf_cpu_map;

    data = unsafe { &mut (*map_event).data };

    TEST_ASSERT_VAL!(c"wrong type".as_ptr(), unsafe {
        (*data).type_ == PERF_CPU_MAP__CPUS
    });

    TEST_ASSERT_VAL!(c"wrong nr".as_ptr(), unsafe {
        (*data).cpus_data.nr == 2
    });
    TEST_ASSERT_VAL!(c"wrong cpu".as_ptr(), unsafe {
        (*data).cpus_data.cpu[0] == 1
    });
    TEST_ASSERT_VAL!(c"wrong cpu".as_ptr(), unsafe {
        (*data).cpus_data.cpu[1] == 256
    });

    map = unsafe { cpu_map__new_data(data) };
    TEST_ASSERT_VAL!(c"wrong nr".as_ptr(), unsafe { perf_cpu_map__nr(map) == 2 });
    TEST_ASSERT_VAL!(c"wrong cpu".as_ptr(), unsafe {
        perf_cpu_map__cpu(map, 0).cpu == 1
    });
    TEST_ASSERT_VAL!(c"wrong cpu".as_ptr(), unsafe {
        perf_cpu_map__cpu(map, 1).cpu == 256
    });
    TEST_ASSERT_VAL!(c"wrong refcnt".as_ptr(), unsafe {
        refcount_read(perf_cpu_map__refcnt(map)) == 1
    });
    unsafe { perf_cpu_map__put(map) };
    0
}

unsafe extern "C" fn process_event_range_cpus(
    tool: *const perf_tool,
    event: *mut perf_event,
    sample: *mut perf_sample,
    machine: *mut machine,
) -> c_int {
    let _ = tool;
    let _ = sample;
    let _ = machine;
    let map_event: *mut perf_record_cpu_map = unsafe { &mut (*event).cpu_map };
    let data: *mut perf_record_cpu_map_data;
    let map: *mut perf_cpu_map;

    data = unsafe { &mut (*map_event).data };

    TEST_ASSERT_VAL!(c"wrong type".as_ptr(), unsafe {
        (*data).type_ == PERF_CPU_MAP__RANGE_CPUS
    });

    TEST_ASSERT_VAL!(c"wrong any_cpu".as_ptr(), unsafe {
        (*data).range_cpu_data.any_cpu == 0
    });
    TEST_ASSERT_VAL!(c"wrong start_cpu".as_ptr(), unsafe {
        (*data).range_cpu_data.start_cpu == 1
    });
    TEST_ASSERT_VAL!(c"wrong end_cpu".as_ptr(), unsafe {
        (*data).range_cpu_data.end_cpu == 256
    });

    map = unsafe { cpu_map__new_data(data) };
    TEST_ASSERT_VAL!(c"wrong nr".as_ptr(), unsafe { perf_cpu_map__nr(map) == 256 });
    TEST_ASSERT_VAL!(c"wrong cpu".as_ptr(), unsafe {
        perf_cpu_map__cpu(map, 0).cpu == 1
    });
    TEST_ASSERT_VAL!(c"wrong cpu".as_ptr(), unsafe {
        perf_cpu_map__max(map).cpu == 256
    });
    TEST_ASSERT_VAL!(c"wrong refcnt".as_ptr(), unsafe {
        refcount_read(perf_cpu_map__refcnt(map)) == 1
    });
    unsafe { perf_cpu_map__put(map) };
    0
}

unsafe extern "C" fn test__cpu_map_synthesize(
    test: *mut test_suite,
    subtest: c_int,
) -> c_int {
    let _ = test;
    let _ = subtest;
    let mut cpus: *mut perf_cpu_map;

    /* This one is better stored in a mask. */
    cpus = unsafe { perf_cpu_map__new(c"0,2-20".as_ptr()) };

    TEST_ASSERT_VAL!(c"failed to synthesize map".as_ptr(), unsafe {
        !perf_event__synthesize_cpu_map(
            core::ptr::null_mut(),
            cpus,
            Some(process_event_mask),
            core::ptr::null_mut(),
        )
    });

    unsafe { perf_cpu_map__put(cpus) };

    /* This one is better stored in cpu values. */
    cpus = unsafe { perf_cpu_map__new(c"1,256".as_ptr()) };

    TEST_ASSERT_VAL!(c"failed to synthesize map".as_ptr(), unsafe {
        !perf_event__synthesize_cpu_map(
            core::ptr::null_mut(),
            cpus,
            Some(process_event_cpus),
            core::ptr::null_mut(),
        )
    });

    unsafe { perf_cpu_map__put(cpus) };

    /* This one is better stored as a range. */
    cpus = unsafe { perf_cpu_map__new(c"1-256".as_ptr()) };

    TEST_ASSERT_VAL!(c"failed to synthesize map".as_ptr(), unsafe {
        !perf_event__synthesize_cpu_map(
            core::ptr::null_mut(),
            cpus,
            Some(process_event_range_cpus),
            core::ptr::null_mut(),
        )
    });

    unsafe { perf_cpu_map__put(cpus) };
    0
}

unsafe extern "C" fn cpu_map_print(str_: *const c_char) -> c_int {
    let map: *mut perf_cpu_map = unsafe { perf_cpu_map__new(str_) };
    let mut buf = [0 as c_char; 100];

    if map.is_null() {
        return -1;
    }

    unsafe { cpu_map__snprint(map, buf.as_mut_ptr(), core::mem::size_of_val(&buf)) };
    unsafe { perf_cpu_map__put(map) };

    (unsafe { strcmp(buf.as_ptr(), str_) } == 0) as c_int
}

unsafe extern "C" fn test__cpu_map_print(test: *mut test_suite, subtest: c_int) -> c_int {
    let _ = test;
    let _ = subtest;
    TEST_ASSERT_VAL!(c"failed to convert map".as_ptr(), unsafe {
        cpu_map_print(c"1".as_ptr()) != 0
    });
    TEST_ASSERT_VAL!(c"failed to convert map".as_ptr(), unsafe {
        cpu_map_print(c"1,5".as_ptr()) != 0
    });
    TEST_ASSERT_VAL!(c"failed to convert map".as_ptr(), unsafe {
        cpu_map_print(c"1,3,5,7,9,11,13,15,17,19,21-40".as_ptr()) != 0
    });
    TEST_ASSERT_VAL!(c"failed to convert map".as_ptr(), unsafe {
        cpu_map_print(c"2-5".as_ptr()) != 0
    });
    TEST_ASSERT_VAL!(c"failed to convert map".as_ptr(), unsafe {
        cpu_map_print(c"1,3-6,8-10,24,35-37".as_ptr()) != 0
    });
    TEST_ASSERT_VAL!(c"failed to convert map".as_ptr(), unsafe {
        cpu_map_print(c"1,3-6,8-10,24,35-37".as_ptr()) != 0
    });
    TEST_ASSERT_VAL!(c"failed to convert map".as_ptr(), unsafe {
        cpu_map_print(c"1-10,12-20,22-30,32-40".as_ptr()) != 0
    });
    0
}

unsafe extern "C" fn __test__cpu_map_merge(
    lhs: *const c_char,
    rhs: *const c_char,
    nr: c_uint,
    expected: *const c_char,
) -> c_int {
    let mut a: *mut perf_cpu_map = unsafe { perf_cpu_map__new(lhs) };
    let b: *mut perf_cpu_map = unsafe { perf_cpu_map__new(rhs) };
    let mut buf = [0 as c_char; 100];

    unsafe { perf_cpu_map__merge(&mut a, b) };
    TEST_ASSERT_VAL!(c"failed to merge map: bad nr".as_ptr(), unsafe {
        perf_cpu_map__nr(a) == nr
    });
    unsafe { cpu_map__snprint(a, buf.as_mut_ptr(), core::mem::size_of_val(&buf)) };
    TEST_ASSERT_VAL!(c"failed to merge map: bad result".as_ptr(), unsafe {
        strcmp(buf.as_ptr(), expected) == 0
    });
    unsafe { perf_cpu_map__put(b) };

    /*
     * If 'b' is a superset of 'a', 'a' points to the same map with the
     * map 'b'. In this case, the owner 'b' has released the resource above
     * but 'a' still keeps the ownership, the reference counter should be 1.
     */
    TEST_ASSERT_VAL!(c"unexpected refcnt: bad result".as_ptr(), unsafe {
        refcount_read(perf_cpu_map__refcnt(a)) == 1
    });

    unsafe { perf_cpu_map__put(a) };
    0
}

unsafe extern "C" fn test__cpu_map_merge(test: *mut test_suite, subtest: c_int) -> c_int {
    let _ = test;
    let _ = subtest;
    let mut ret: c_int;

    ret = unsafe { __test__cpu_map_merge(c"4,2,1".as_ptr(), c"4,5,7".as_ptr(), 5, c"1-2,4-5,7".as_ptr()) };
    if ret != 0 {
        return ret;
    }
    ret = unsafe { __test__cpu_map_merge(c"1-8".as_ptr(), c"6-9".as_ptr(), 9, c"1-9".as_ptr()) };
    if ret != 0 {
        return ret;
    }
    ret = unsafe { __test__cpu_map_merge(c"1-8,12-20".as_ptr(), c"6-9,15".as_ptr(), 18, c"1-9,12-20".as_ptr()) };
    if ret != 0 {
        return ret;
    }
    ret = unsafe { __test__cpu_map_merge(c"4,2,1".as_ptr(), c"1".as_ptr(), 3, c"1-2,4".as_ptr()) };
    if ret != 0 {
        return ret;
    }
    ret = unsafe { __test__cpu_map_merge(c"1".as_ptr(), c"4,2,1".as_ptr(), 3, c"1-2,4".as_ptr()) };
    if ret != 0 {
        return ret;
    }
    ret = unsafe { __test__cpu_map_merge(c"1".as_ptr(), c"1".as_ptr(), 1, c"1".as_ptr()) };
    ret
}

unsafe extern "C" fn __test__cpu_map_intersect(
    lhs: *const c_char,
    rhs: *const c_char,
    nr: c_uint,
    expected: *const c_char,
) -> c_int {
    let a: *mut perf_cpu_map = unsafe { perf_cpu_map__new(lhs) };
    let b: *mut perf_cpu_map = unsafe { perf_cpu_map__new(rhs) };
    let c: *mut perf_cpu_map = unsafe { perf_cpu_map__intersect(a, b) };
    let mut buf = [0 as c_char; 100];

    TEST_ASSERT_EQUAL!(c"failed to intersect map: bad nr".as_ptr(), unsafe {
        perf_cpu_map__nr(c)
    }, nr);
    unsafe { cpu_map__snprint(c, buf.as_mut_ptr(), core::mem::size_of_val(&buf)) };
    TEST_ASSERT_VAL!(c"failed to intersect map: bad result".as_ptr(), unsafe {
        strcmp(buf.as_ptr(), expected) == 0
    });
    unsafe { perf_cpu_map__put(a) };
    unsafe { perf_cpu_map__put(b) };
    unsafe { perf_cpu_map__put(c) };
    0
}

unsafe extern "C" fn test__cpu_map_intersect(
    test: *mut test_suite,
    subtest: c_int,
) -> c_int {
    let _ = test;
    let _ = subtest;
    let mut ret: c_int;

    ret = unsafe { __test__cpu_map_intersect(c"4,2,1".as_ptr(), c"4,5,7".as_ptr(), 1, c"4".as_ptr()) };
    if ret != 0 {
        return ret;
    }
    ret = unsafe { __test__cpu_map_intersect(c"1-8".as_ptr(), c"6-9".as_ptr(), 3, c"6-8".as_ptr()) };
    if ret != 0 {
        return ret;
    }
    ret = unsafe { __test__cpu_map_intersect(c"1-8,12-20".as_ptr(), c"6-9,15".as_ptr(), 4, c"6-8,15".as_ptr()) };
    if ret != 0 {
        return ret;
    }
    ret = unsafe { __test__cpu_map_intersect(c"4,2,1".as_ptr(), c"1".as_ptr(), 1, c"1".as_ptr()) };
    if ret != 0 {
        return ret;
    }
    ret = unsafe { __test__cpu_map_intersect(c"1".as_ptr(), c"4,2,1".as_ptr(), 1, c"1".as_ptr()) };
    if ret != 0 {
        return ret;
    }
    ret = unsafe { __test__cpu_map_intersect(c"1".as_ptr(), c"1".as_ptr(), 1, c"1".as_ptr()) };
    ret
}

unsafe extern "C" fn test__cpu_map_equal(test: *mut test_suite, subtest: c_int) -> c_int {
    let _ = test;
    let _ = subtest;
    let any: *mut perf_cpu_map = unsafe { perf_cpu_map__new_any_cpu() };
    let one: *mut perf_cpu_map = unsafe { perf_cpu_map__new(c"1".as_ptr()) };
    let mut two: *mut perf_cpu_map = unsafe { perf_cpu_map__new(c"2".as_ptr()) };
    let empty: *mut perf_cpu_map = unsafe { perf_cpu_map__intersect(one, two) };
    let pair: *mut perf_cpu_map = unsafe { perf_cpu_map__new(c"1-2".as_ptr()) };
    let mut tmp: *mut perf_cpu_map;
    let mut maps: [*mut *mut perf_cpu_map; 5] = [
        &empty as *const _ as *mut _,
        &any as *const _ as *mut _,
        &one as *const _ as *mut _,
        &mut two,
        &pair as *const _ as *mut _,
    ];

    for i in 0..maps.len() {
        /* Maps equal themself. */
        TEST_ASSERT_VAL!(c"equal".as_ptr(), unsafe {
            perf_cpu_map__equal(*maps[i], *maps[i])
        });
        for j in 0..maps.len() {
            /* Maps dont't equal each other. */
            if i == j {
                continue;
            }
            TEST_ASSERT_VAL!(c"not equal".as_ptr(), unsafe {
                !perf_cpu_map__equal(*maps[i], *maps[j])
            });
        }
    }

    /* Maps equal made maps. */
    unsafe { perf_cpu_map__merge(&mut two, one) };
    TEST_ASSERT_VAL!(c"pair".as_ptr(), unsafe {
        perf_cpu_map__equal(pair, two)
    });

    tmp = unsafe { perf_cpu_map__intersect(pair, one) };
    TEST_ASSERT_VAL!(c"one".as_ptr(), unsafe {
        perf_cpu_map__equal(one, tmp)
    });
    unsafe { perf_cpu_map__put(tmp) };

    for i in 0..maps.len() {
        unsafe { perf_cpu_map__put(*maps[i]) };
    }

    TEST_OK
}

static mut tests__cpu_map: [test_case; 6] = [
    TEST_CASE!(c"Synthesize cpu map".as_ptr(), test__cpu_map_synthesize),
    TEST_CASE!(c"Print cpu map".as_ptr(), test__cpu_map_print),
    TEST_CASE!(c"Merge cpu map".as_ptr(), test__cpu_map_merge),
    TEST_CASE!(c"Intersect cpu map".as_ptr(), test__cpu_map_intersect),
    TEST_CASE!(c"Equal cpu map".as_ptr(), test__cpu_map_equal),
    test_case {
        name: core::ptr::null(),
    },
];

#[no_mangle]
pub static mut suite__cpu_map: test_suite = test_suite {
    desc: c"CPU map".as_ptr(),
    test_cases: unsafe { tests__cpu_map.as_mut_ptr() },
};
