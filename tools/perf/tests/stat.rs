// SPDX-License-Identifier: GPL-2.0
// C dependencies: linux/compiler.h, event.h, tests.h, stat.h, counts.h,
// debug.h, util/synthetic-events.h

use core::ffi::{c_int, c_void};

type u64 = u64;

#[repr(C)]
pub struct perf_tool {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_sample {
    _private: [u8; 0],
}

#[repr(C)]
pub struct machine {
    _private: [u8; 0],
}

#[repr(C)]
pub struct test_suite {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_stat_config_term {
    pub tag: u64,
    pub val: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_stat_config {
    pub nr: u64,
    pub data: *mut perf_record_stat_config_term,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_stat {
    pub cpu: u64,
    pub thread: u64,
    pub id: u64,
    pub val: u64,
    pub ena: u64,
    pub run: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_record_stat_round {
    pub time: u64,
    pub type_: u64,
}

#[repr(C)]
pub union perf_event {
    pub stat_config: perf_record_stat_config,
    pub stat: perf_record_stat,
    pub stat_round: perf_record_stat_round,
}

#[repr(C)]
pub struct perf_stat_config {
    pub aggr_mode: u64,
    pub scale: u64,
    pub interval: u64,
}

#[repr(C)]
pub struct perf_counts_values {
    pub val: u64,
    pub ena: u64,
    pub run: u64,
}

#[repr(C)]
pub struct perf_cpu {
    pub cpu: c_int,
}

type perf_event__handler_t = Option<
    unsafe extern "C" fn(
        *const perf_tool,
        *mut perf_event,
        *mut perf_sample,
        *mut machine,
    ) -> c_int,
>;

extern "C" {
    static PERF_STAT_CONFIG_TERM__MAX: u64;
    static PERF_STAT_CONFIG_TERM__AGGR_MODE: u64;
    static PERF_STAT_CONFIG_TERM__SCALE: u64;
    static PERF_STAT_CONFIG_TERM__INTERVAL: u64;
    static AGGR_CORE: u64;
    static PERF_STAT_ROUND_TYPE__INTERVAL: u64;

    fn perf_event__read_stat_config(
        stat_config: *mut perf_stat_config,
        config: *mut perf_record_stat_config,
    );
    fn perf_event__synthesize_stat_config(
        tool: *const perf_tool,
        config: *mut perf_stat_config,
        process: perf_event__handler_t,
        machine: *mut machine,
    ) -> c_int;
    fn perf_event__synthesize_stat(
        tool: *const perf_tool,
        cpu: perf_cpu,
        thread: u64,
        id: u64,
        count: *mut perf_counts_values,
        process: perf_event__handler_t,
        machine: *mut machine,
    ) -> c_int;
    fn perf_event__synthesize_stat_round(
        tool: *const perf_tool,
        time: u64,
        type_: u64,
        process: perf_event__handler_t,
        machine: *mut machine,
    ) -> c_int;
}

macro_rules! TEST_ASSERT_VAL {
    ($msg:expr, $cond:expr) => {
        if !$cond {
            return -1;
        }
    };
}

unsafe fn has_term(config: *mut perf_record_stat_config, tag: u64, val: u64) -> bool {
    let mut i: u32 = 0;

    while (i as u64) < (*config).nr {
        if ((*(*config).data.add(i as usize)).tag == tag)
            && ((*(*config).data.add(i as usize)).val == val)
        {
            return true;
        }

        i = i.wrapping_add(1);
    }

    false
}

unsafe extern "C" fn process_stat_config_event(
    _tool: *const perf_tool,
    event: *mut perf_event,
    _sample: *mut perf_sample,
    _machine: *mut machine,
) -> c_int {
    let config: *mut perf_record_stat_config = &mut (*event).stat_config;
    let mut test_stat_config: perf_stat_config = core::mem::zeroed();

    macro_rules! HAS {
        ($term:ident, $val:expr) => {
            has_term(
                config,
                PERF_STAT_CONFIG_TERM__AGGR_MODE,
                $val as u64,
            )
        };
    }

    TEST_ASSERT_VAL!("wrong nr", (*config).nr == PERF_STAT_CONFIG_TERM__MAX);
    TEST_ASSERT_VAL!(
        "wrong aggr_mode",
        has_term(config, PERF_STAT_CONFIG_TERM__AGGR_MODE, AGGR_CORE)
    );
    TEST_ASSERT_VAL!(
        "wrong scale",
        has_term(config, PERF_STAT_CONFIG_TERM__SCALE, 1)
    );
    TEST_ASSERT_VAL!(
        "wrong interval",
        has_term(config, PERF_STAT_CONFIG_TERM__INTERVAL, 1)
    );

    perf_event__read_stat_config(&mut test_stat_config, config);

    TEST_ASSERT_VAL!("wrong aggr_mode", test_stat_config.aggr_mode == AGGR_CORE);
    TEST_ASSERT_VAL!("wrong scale", test_stat_config.scale == 1);
    TEST_ASSERT_VAL!("wrong interval", test_stat_config.interval == 1);
    0
}

unsafe extern "C" fn test__synthesize_stat_config(
    _test: *mut test_suite,
    _subtest: c_int,
) -> c_int {
    let mut test_stat_config = perf_stat_config {
        aggr_mode: AGGR_CORE,
        scale: 1,
        interval: 1,
    };

    TEST_ASSERT_VAL!(
        "failed to synthesize stat_config",
        perf_event__synthesize_stat_config(
            core::ptr::null(),
            &mut test_stat_config,
            Some(process_stat_config_event),
            core::ptr::null_mut(),
        ) == 0
    );

    0
}

unsafe extern "C" fn process_stat_event(
    _tool: *const perf_tool,
    event: *mut perf_event,
    _sample: *mut perf_sample,
    _machine: *mut machine,
) -> c_int {
    let st: *mut perf_record_stat = &mut (*event).stat;

    TEST_ASSERT_VAL!("wrong cpu", (*st).cpu == 1);
    TEST_ASSERT_VAL!("wrong thread", (*st).thread == 2);
    TEST_ASSERT_VAL!("wrong id", (*st).id == 3);
    TEST_ASSERT_VAL!("wrong val", (*st).val == 100);
    TEST_ASSERT_VAL!("wrong run", (*st).ena == 200);
    TEST_ASSERT_VAL!("wrong ena", (*st).run == 300);
    0
}

unsafe extern "C" fn test__synthesize_stat(
    _test: *mut test_suite,
    _subtest: c_int,
) -> c_int {
    let mut count: perf_counts_values = core::mem::zeroed();

    count.val = 100;
    count.ena = 200;
    count.run = 300;

    TEST_ASSERT_VAL!(
        "failed to synthesize stat_config",
        perf_event__synthesize_stat(
            core::ptr::null(),
            perf_cpu { cpu: 1 },
            2,
            3,
            &mut count,
            Some(process_stat_event),
            core::ptr::null_mut(),
        ) == 0
    );

    0
}

unsafe extern "C" fn process_stat_round_event(
    _tool: *const perf_tool,
    event: *mut perf_event,
    _sample: *mut perf_sample,
    _machine: *mut machine,
) -> c_int {
    let stat_round: *mut perf_record_stat_round = &mut (*event).stat_round;

    TEST_ASSERT_VAL!("wrong time", (*stat_round).time == 0xdeadbeef);
    TEST_ASSERT_VAL!(
        "wrong type",
        (*stat_round).type_ == PERF_STAT_ROUND_TYPE__INTERVAL
    );
    0
}

unsafe extern "C" fn test__synthesize_stat_round(
    _test: *mut test_suite,
    _subtest: c_int,
) -> c_int {
    TEST_ASSERT_VAL!(
        "failed to synthesize stat_config",
        perf_event__synthesize_stat_round(
            core::ptr::null(),
            0xdeadbeef,
            PERF_STAT_ROUND_TYPE__INTERVAL,
            Some(process_stat_round_event),
            core::ptr::null_mut(),
        ) == 0
    );

    0
}

// DEFINE_SUITE("Synthesize stat config", synthesize_stat_config);
// DEFINE_SUITE("Synthesize stat", synthesize_stat);
// DEFINE_SUITE("Synthesize stat round", synthesize_stat_round);


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
