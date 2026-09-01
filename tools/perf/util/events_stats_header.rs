/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies from the original C header:
// #include <stdio.h>
// #include <perf/event.h>
// #include <linux/types.h>
// #include "auxtrace.h"

// Opaque declaration corresponding to C's FILE from <stdio.h>.
#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

/*
 * The kernel collects the number of events it couldn't send in a stretch and
 * when possible sends this number in a PERF_RECORD_LOST event. The number of
 * such "chunks" of lost events is stored in .nr_events[PERF_EVENT_LOST] while
 * total_lost tells exactly how many events the kernel in fact lost, i.e. it is
 * the sum of all struct perf_record_lost.lost fields reported.
 *
 * The kernel discards mixed up samples and sends the number in a
 * PERF_RECORD_LOST_SAMPLES event. The number of lost-samples events is stored
 * in .nr_events[PERF_RECORD_LOST_SAMPLES] while total_lost_samples tells
 * exactly how many samples the kernel in fact dropped, i.e. it is the sum of
 * all struct perf_record_lost_samples.lost fields reported without setting the
 * misc field in the header.
 *
 * The BPF program can discard samples according to the filter expressions given
 * by the user.  This number is kept in a BPF map and dumped at the end of perf
 * record in a PERF_RECORD_LOST_SAMPLES event.  To differentiate it from other
 * lost samples, perf tools sets PERF_RECORD_MISC_LOST_SAMPLES_BPF flag in the
 * header.misc field.  The number of dropped-samples events is stored in
 * .nr_events[PERF_RECORD_LOST_SAMPLES] while total_dropped_samples tells
 * exactly how many samples the BPF program in fact dropped, i.e. it is the sum
 * of all struct perf_record_lost_samples.lost fields reported with the misc
 * field set in the header.
 *
 * The total_period is needed because by default auto-freq is used, so
 * multiplying nr_events[PERF_EVENT_SAMPLE] by a frequency isn't possible to get
 * the total number of low level events, it is necessary to sum all struct
 * perf_record_sample.period and stash the result in total_period.
 */
#[repr(C)]
pub struct events_stats {
    pub total_lost: u64,
    pub total_lost_samples: u64,
    pub total_dropped_samples: u64,
    pub total_aux_lost: u64,
    pub total_aux_partial: u64,
    pub total_aux_collision: u64,
    pub total_invalid_chains: u64,
    pub nr_events: [u32; PERF_RECORD_HEADER_MAX as usize],
    pub nr_lost_warned: u32,
    pub nr_unknown_events: u32,
    pub nr_invalid_chains: u32,
    pub nr_unknown_id: u32,
    pub nr_unprocessable_samples: u32,
    pub nr_auxtrace_errors: [u32; PERF_AUXTRACE_ERROR_MAX as usize],
    pub nr_proc_map_timeout: u32,
}

#[repr(C)]
pub struct hists_stats {
    pub total_period: u64,
    pub total_non_filtered_period: u64,
    pub total_latency: u64,
    pub total_non_filtered_latency: u64,
    pub nr_samples: u32,
    pub nr_non_filtered_samples: u32,
    pub nr_lost_samples: u32,
    pub nr_dropped_samples: u32,
}

unsafe extern "C" {
    pub fn events_stats__inc(stats: *mut events_stats, type_: u32);

    pub fn events_stats__fprintf(stats: *mut events_stats, fp: *mut FILE) -> usize;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
