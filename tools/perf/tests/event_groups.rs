// SPDX-License-Identifier: GPL-2.0
//
// C dependencies translated as external Rust declarations:
// string.h, unistd.h, stdio.h, linux/perf_event.h, tests.h, debug.h,
// pmu.h, pmus.h, header.h, ../perf-sys.h

use core::ffi::{c_char, c_int, c_ulong, c_void};

type __u64 = u64;

#[repr(C)]
pub struct perf_event_attr {
    pub type_: u32,
    pub size: u32,
    pub config: u64,
    pub disabled: u64,
}

#[repr(C)]
pub struct perf_pmu {
    pub name: *const c_char,
    pub type_: c_int,
}

#[repr(C)]
pub struct test_suite {
    _private: [u8; 0],
}

extern "C" {
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn sys_perf_event_open(
        attr: *mut perf_event_attr,
        pid: c_int,
        cpu: c_int,
        group_fd: c_int,
        flags: c_ulong,
    ) -> c_int;
    fn perf_pmus__scan(pmu: *mut perf_pmu) -> *mut perf_pmu;
    fn pr_debug(fmt: *const c_char, ...);
}

extern "C" {
    static TEST_SKIP: c_int;
    static TEST_OK: c_int;
    static TEST_FAIL: c_int;
}

/* hw: cycles,instructions sw: context-switch, uncore: [arch dependent] */
static mut types: [c_int; 3] = [0, 1, -1];
static mut configs: [c_ulong; 3] = [0, 3, 0];
static configs_hw: [c_ulong; 1] = [1];

const NR_UNCORE_PMUS: usize = 5;

/* Uncore pmus that support more than 3 counters */
#[repr(C)]
struct uncore_pmus {
    name: *const c_char,
    config: __u64,
}

static mut uncore_pmus: [uncore_pmus; NR_UNCORE_PMUS] = [
    uncore_pmus {
        name: b"amd_l3\0".as_ptr() as *const c_char,
        config: 0x0,
    },
    uncore_pmus {
        name: b"amd_df\0".as_ptr() as *const c_char,
        config: 0x0,
    },
    uncore_pmus {
        name: b"uncore_imc_0\0".as_ptr() as *const c_char,
        config: 0x1,
    }, /* Intel */
    uncore_pmus {
        name: b"core_imc\0".as_ptr() as *const c_char,
        config: 0x318,
    }, /* PowerPC: core_imc/CPM_STCX_FIN/ */
    uncore_pmus {
        name: b"hv_24x7\0".as_ptr() as *const c_char,
        config: 0x22000000003,
    }, /* PowerPC: hv_24x7/CPM_STCX_FIN/ */
];

unsafe fn event_open(type_: c_int, config: c_ulong, group_fd: c_int) -> c_int {
    let mut attr: perf_event_attr = core::mem::zeroed();

    memset(
        &mut attr as *mut perf_event_attr as *mut c_void,
        0,
        core::mem::size_of::<perf_event_attr>(),
    );
    attr.type_ = type_ as u32;
    attr.size = core::mem::size_of::<perf_event_attr>() as u32;
    attr.config = config as u64;
    /*
     * When creating an event group, typically the group leader is
     * initialized with disabled set to 1 and any child events are
     * initialized with disabled set to 0. Despite disabled being 0,
     * the child events will not start until the group leader is
     * enabled.
     */
    attr.disabled = if group_fd == -1 { 1 } else { 0 };

    sys_perf_event_open(&mut attr, -1, 0, group_fd, 0)
}

unsafe fn setup_uncore_event() -> c_int {
    let mut pmu: *mut perf_pmu = core::ptr::null_mut();
    let mut i: c_int;
    let fd: c_int;

    loop {
        pmu = perf_pmus__scan(pmu);
        if pmu.is_null() {
            break;
        }

        i = 0;
        while i < NR_UNCORE_PMUS as c_int {
            if strcmp(uncore_pmus[i as usize].name, (*pmu).name) == 0 {
                pr_debug(
                    b"Using %s for uncore pmu event\n\0".as_ptr() as *const c_char,
                    (*pmu).name,
                );
                types[2] = (*pmu).type_;
                configs[2] = uncore_pmus[i as usize].config as c_ulong;
                /*
                 * Check if the chosen uncore pmu event can be
                 * used in the test. For example, incase of accessing
                 * hv_24x7 pmu counters, partition should have
                 * additional permissions. If not, event open will
                 * fail. So check if the event open succeeds
                 * before proceeding.
                 */
                fd = event_open(types[2], configs[2], -1);
                if fd < 0 {
                    return -1;
                }
                close(fd);
                return 0;
            }
            i += 1;
        }
    }
    -1
}

unsafe fn run_test(i: c_int, j: c_int, k: c_int) -> c_int {
    let erroneous: c_int = if ((((1 as c_int) << i) | ((1 as c_int) << j) | ((1 as c_int) << k))
        & 5)
        == 5
    {
        1
    } else {
        0
    };
    let group_fd: c_int;
    let sibling_fd1: c_int;
    let sibling_fd2: c_int;

    group_fd = event_open(types[i as usize], configs[i as usize], -1);
    if group_fd == -1 {
        return -1;
    }

    sibling_fd1 = event_open(types[j as usize], configs[j as usize], group_fd);
    if sibling_fd1 == -1 {
        close(group_fd);
        return if erroneous != 0 { 0 } else { -1 };
    }

    /*
     * if all three events (leader and two sibling events)
     * are hardware events, use instructions as one of the
     * sibling event. There is event constraint in powerpc that
     * events using same counter cannot be programmed in a group.
     * Since PERF_COUNT_HW_INSTRUCTIONS is a generic hardware
     * event and present in all platforms, lets use that.
     */
    if i == 0 && j == 0 && k == 0 {
        sibling_fd2 = event_open(types[k as usize], configs_hw[k as usize], group_fd);
    } else {
        sibling_fd2 = event_open(types[k as usize], configs[k as usize], group_fd);
    }
    if sibling_fd2 == -1 {
        close(sibling_fd1);
        close(group_fd);
        return if erroneous != 0 { 0 } else { -1 };
    }

    close(sibling_fd2);
    close(sibling_fd1);
    close(group_fd);
    if erroneous != 0 {
        -1
    } else {
        0
    }
}

unsafe fn test__event_groups(_text: *mut test_suite, _subtest: c_int) -> c_int {
    let mut i: c_int;
    let mut j: c_int;
    let mut k: c_int;
    let mut ret: c_int;
    let r: c_int;

    ret = setup_uncore_event();
    if ret != 0 || types[2] == -1 {
        return TEST_SKIP;
    }

    ret = TEST_OK;
    i = 0;
    while i < 3 {
        j = 0;
        while j < 3 {
            k = 0;
            while k < 3 {
                r = run_test(i, j, k);
                if r != 0 {
                    ret = TEST_FAIL;
                }

                /*
                 * For all three events as HW events, second sibling
                 * event is picked from configs_hw. So print accordingly
                 */
                if i == 0 && j == 0 && k == 0 {
                    pr_debug(
                        b"0x%x 0x%lx, 0x%x 0x%lx, 0x%x 0x%lx: %s\n\0".as_ptr()
                            as *const c_char,
                        types[i as usize],
                        configs[i as usize],
                        types[j as usize],
                        configs[j as usize],
                        types[k as usize],
                        configs_hw[k as usize],
                        if r != 0 {
                            b"Fail\0".as_ptr() as *const c_char
                        } else {
                            b"Pass\0".as_ptr() as *const c_char
                        },
                    );
                } else {
                    pr_debug(
                        b"0x%x 0x%lx, 0x%x 0x%lx, 0x%x 0x%lx: %s\n\0".as_ptr()
                            as *const c_char,
                        types[i as usize],
                        configs[i as usize],
                        types[j as usize],
                        configs[j as usize],
                        types[k as usize],
                        configs[k as usize],
                        if r != 0 {
                            b"Fail\0".as_ptr() as *const c_char
                        } else {
                            b"Pass\0".as_ptr() as *const c_char
                        },
                    );
                }
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }
    ret
}

// DEFINE_SUITE("Event groups", event_groups);
