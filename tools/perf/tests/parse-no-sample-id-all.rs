// Dependencies from the original C includes:
// linux/kernel.h, linux/types.h, stddef.h, tests.h, event.h, evlist.h,
// header.h, debug.h, util/sample.h.

use core::ffi::c_void;
use core::mem::size_of;

extern "C" {
    fn perf_event__process_attr(
        tool: *mut c_void,
        event: *mut perf_event,
        pevlist: *mut *mut evlist,
    ) -> i32;
    fn pr_debug(fmt: *const u8, ...);
    fn perf_sample__init(sample: *mut perf_sample, all: bool);
    fn evlist__parse_sample(
        evlist: *mut evlist,
        event: *mut perf_event,
        sample: *mut perf_sample,
    ) -> i32;
    fn perf_sample__exit(sample: *mut perf_sample);
    fn evlist__put(evlist: *mut evlist);
}

// External constants supplied by perf headers.
extern "C" {
    static PERF_RECORD_HEADER_ATTR: u32;
    static PERF_RECORD_USER_TYPE_START: u32;
    static PERF_RECORD_MMAP: u32;
}

// External types supplied by perf headers.
#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct test_suite {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_sample {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_event_attr {
    pub size: u32,
    // Remaining fields are supplied by the original perf headers.
}

#[repr(C)]
pub struct perf_event_header {
    pub type_: u32,
    pub misc: u16,
    pub size: u16,
}

#[repr(C)]
pub struct perf_record_mmap {
    pub header: perf_event_header,
    // Remaining fields are supplied by the original perf headers.
}

#[repr(C)]
pub union perf_event {
    pub header: perf_event_header,
    pub mmap: core::mem::ManuallyDrop<perf_record_mmap>,
}

unsafe fn process_event(pevlist: *mut *mut evlist, event: *mut perf_event) -> i32 {
    let mut sample: perf_sample = core::mem::zeroed();
    let ret: i32;

    if (*event).header.type_ == PERF_RECORD_HEADER_ATTR {
        if perf_event__process_attr(core::ptr::null_mut(), event, pevlist) != 0 {
            pr_debug(c"perf_event__process_attr failed\n".as_ptr() as *const u8);
            return -1;
        }
        return 0;
    }

    if (*event).header.type_ >= PERF_RECORD_USER_TYPE_START {
        return -1;
    }

    if (*pevlist).is_null() {
        return -1;
    }

    perf_sample__init(&mut sample, false);
    ret = evlist__parse_sample(*pevlist, event, &mut sample);
    perf_sample__exit(&mut sample);
    if ret != 0 {
        pr_debug(c"evlist__parse_sample failed\n".as_ptr() as *const u8);
        return -1;
    }

    0
}

unsafe fn process_events(events: *mut *mut perf_event, count: usize) -> i32 {
    let mut evlist: *mut evlist = core::ptr::null_mut();
    let mut err: i32 = 0;
    let mut i: usize;

    i = 0;
    while i < count && err == 0 {
        err = process_event(&mut evlist, *events.add(i));
        i += 1;
    }

    evlist__put(evlist);

    err
}

#[repr(C)]
struct test_attr_event {
    header: perf_event_header,
    attr: perf_event_attr,
    id: u64,
}

/**
 * test__parse_no_sample_id_all - test parsing with no sample_id_all bit set.
 *
 * This function tests parsing data produced on kernel's that do not support the
 * sample_id_all bit.  Without the sample_id_all bit, non-sample events (such as
 * mmap events) do not have an id sample appended, and consequently logic
 * designed to determine the id will not work.  That case happens when there is
 * more than one selected event, so this test processes three events: 2
 * attributes representing the selected events and one mmap event.
 *
 * Return: %0 on success, %-1 if the test fails.
 */
unsafe fn test__parse_no_sample_id_all(
    _test: *mut test_suite,
    _subtest: i32,
) -> i32 {
    let err: i32;

    let mut event1 = test_attr_event {
        header: perf_event_header {
            type_: PERF_RECORD_HEADER_ATTR,
            misc: 0,
            size: size_of::<test_attr_event>() as u16,
        },
        attr: perf_event_attr {
            size: size_of::<perf_event_attr>() as u32,
        },
        id: 1,
    };
    let mut event2 = test_attr_event {
        header: perf_event_header {
            type_: PERF_RECORD_HEADER_ATTR,
            misc: 0,
            size: size_of::<test_attr_event>() as u16,
        },
        attr: perf_event_attr {
            size: size_of::<perf_event_attr>() as u32,
        },
        id: 2,
    };
    let mut event3 = perf_record_mmap {
        header: perf_event_header {
            type_: PERF_RECORD_MMAP,
            misc: 0,
            size: size_of::<perf_record_mmap>() as u16,
        },
    };
    let mut events: [*mut perf_event; 3] = [
        &mut event1 as *mut test_attr_event as *mut perf_event,
        &mut event2 as *mut test_attr_event as *mut perf_event,
        &mut event3 as *mut perf_record_mmap as *mut perf_event,
    ];

    err = process_events(events.as_mut_ptr(), events.len());
    if err != 0 {
        return -1;
    }

    0
}

// DEFINE_SUITE("Parse with no sample_id_all bit set", parse_no_sample_id_all);
