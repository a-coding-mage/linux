// SPDX-License-Identifier: GPL-2.0
// Translated from perf/arch/x86/util/evlist.c.
// C dependencies: ../../../util/evlist.h, ../../../util/evsel.h,
// topdown.h, evsel.h.

use core::mem::offset_of;
use core::ptr;
use std::os::raw::c_int;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct evsel_core {
    pub node: list_head,
    pub leader: *mut evsel,
    pub idx: c_int,
}

#[repr(C)]
pub struct evsel {
    pub core: evsel_core,
    pub retire_lat: bool,
}

unsafe extern "C" {
    fn topdown_sys_has_perf_metrics() -> bool;
    fn arch_evsel__must_be_in_group(evsel: *const evsel) -> bool;
    fn arch_is_topdown_slots(evsel: *const evsel) -> bool;
    fn arch_is_topdown_metrics(evsel: *const evsel) -> bool;
    fn topdown_insert_slots_event(
        list: *mut list_head,
        idx: c_int,
        metric_event: *mut evsel,
    ) -> c_int;
}

unsafe fn list_entry_evsel_core_node(ptr: *mut list_head) -> *mut evsel {
    (ptr as *mut u8)
        .sub(offset_of!(evsel, core) + offset_of!(evsel_core, node))
        as *mut evsel
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn arch_evlist__cmp(lhs: *const evsel, rhs: *const evsel) -> c_int {
    /*
     * Currently the following topdown events sequence are supported to
     * move and regroup correctly.
     *
     * a. all events in a group
     *    perf stat -e "{instructions,topdown-retiring,slots}" -C0 sleep 1
     *    WARNING: events were regrouped to match PMUs
     *     Performance counter stats for 'CPU(s) 0':
     *          15,066,240     slots
     *          1,899,760      instructions
     *          2,126,998      topdown-retiring
     * b. all events not in a group
     *    perf stat -e "instructions,topdown-retiring,slots" -C0 sleep 1
     *    WARNING: events were regrouped to match PMUs
     *     Performance counter stats for 'CPU(s) 0':
     *          2,045,561      instructions
     *          17,108,370     slots
     *          2,281,116      topdown-retiring
     * c. slots event in a group but topdown metrics events outside the group
     *    perf stat -e "{instructions,slots},topdown-retiring" -C0 sleep 1
     *    WARNING: events were regrouped to match PMUs
     *     Performance counter stats for 'CPU(s) 0':
     *         20,323,878      slots
     *          2,634,884      instructions
     *          3,028,656      topdown-retiring
     * d. slots event and topdown metrics events in two groups
     *    perf stat -e "{instructions,slots},{topdown-retiring}" -C0 sleep 1
     *    WARNING: events were regrouped to match PMUs
     *     Performance counter stats for 'CPU(s) 0':
     *         26,319,024      slots
     *          2,427,791      instructions
     *          2,683,508      topdown-retiring
     * e. slots event and metrics event are not in a group and not adjacent
     *    perf stat -e "{instructions,slots},cycles,topdown-retiring" -C0 sleep 1
     *    WARNING: events were regrouped to match PMUs
     *         68,433,522      slots
     *          8,856,102      topdown-retiring
     *          7,791,494      instructions
     *         11,469,513      cycles
     */
    if topdown_sys_has_perf_metrics()
        && (arch_evsel__must_be_in_group(lhs) || arch_evsel__must_be_in_group(rhs))
    {
        /* Ensure the topdown slots comes first. */
        if arch_is_topdown_slots(lhs) {
            return -1;
        }
        if arch_is_topdown_slots(rhs) {
            return 1;
        }

        /*
         * Move topdown metrics events forward only when topdown metrics
         * events are not in same group with previous slots event. If
         * topdown metrics events are already in same group with slots
         * event, do nothing.
         */
        if (*lhs).core.leader != (*rhs).core.leader {
            let lhs_topdown = arch_is_topdown_metrics(lhs);
            let rhs_topdown = arch_is_topdown_metrics(rhs);

            if lhs_topdown && !rhs_topdown {
                return -1;
            }
            if !lhs_topdown && rhs_topdown {
                return 1;
            }
        }
    }

    /* Retire latency event should not be group leader*/
    if (*lhs).retire_lat && !(*rhs).retire_lat {
        return 1;
    }
    if !(*lhs).retire_lat && (*rhs).retire_lat {
        return -1;
    }

    /* Default ordering by insertion index. */
    (*lhs).core.idx - (*rhs).core.idx
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn arch_evlist__add_required_events(list: *mut list_head) -> c_int {
    let mut metric_event: *mut evsel = ptr::null_mut();
    let mut idx: c_int = 0;

    if !topdown_sys_has_perf_metrics() {
        return 0;
    }

    let mut node = (*list).next;
    while node != list {
        let pos = list_entry_evsel_core_node(node);

        if arch_is_topdown_slots(pos) {
            /* Slots event already present, nothing to do. */
            return 0;
        }
        if metric_event.is_null() && arch_is_topdown_metrics(pos) {
            metric_event = pos;
        }
        idx += 1;

        node = (*node).next;
    }

    if metric_event.is_null() {
        /* No topdown metric events, nothing to do. */
        return 0;
    }
    topdown_insert_slots_event(list, idx + 1, metric_event)
}
