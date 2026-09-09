/* SPDX-License-Identifier: GPL-2.0 */

// C header dependencies supplied by the surrounding kernel translation.
// #include <linux/damon.h>
// #include <linux/types.h>
// #include <linux/tracepoint.h>

// The following trace-event declarations are retained as a token-level Rust
// trace_event! interface.  The macro and referenced kernel types are supplied
// by the surrounding translation unit.

trace_event! {
    damos_stat_after_apply_interval,
    proto(unsigned int context_idx, unsigned int scheme_idx, struct damos_stat *stat),
    args(context_idx, scheme_idx, stat),
    entry {
        field(unsigned int, context_idx);
        field(unsigned int, scheme_idx);
        field(unsigned long, nr_tried);
        field(unsigned long, sz_tried);
        field(unsigned long, nr_applied);
        field(unsigned long, sz_applied);
        field(unsigned long, sz_ops_filter_passed);
        field(unsigned long, qt_exceeds);
        field(unsigned long, nr_snapshots);
    },
    fast_assign {
        __entry->context_idx = context_idx;
        __entry->scheme_idx = scheme_idx;
        __entry->nr_tried = stat->nr_tried;
        __entry->sz_tried = stat->sz_tried;
        __entry->nr_applied = stat->nr_applied;
        __entry->sz_applied = stat->sz_applied;
        __entry->sz_ops_filter_passed = stat->sz_ops_filter_passed;
        __entry->qt_exceeds = stat->qt_exceeds;
        __entry->nr_snapshots = stat->nr_snapshots;
    },
    printk("ctx_idx=%u scheme_idx=%u nr_tried=%lu sz_tried=%lu nr_applied=%lu sz_applied=%lu sz_ops_filter_passed=%lu qt_exceeds=%lu nr_snapshots=%lu",
        __entry->context_idx, __entry->scheme_idx, __entry->nr_tried,
        __entry->sz_tried, __entry->nr_applied, __entry->sz_applied,
        __entry->sz_ops_filter_passed, __entry->qt_exceeds,
        __entry->nr_snapshots)
}

trace_event! {
    damos_esz,
    proto(unsigned int context_idx, unsigned int scheme_idx, unsigned long esz),
    args(context_idx, scheme_idx, esz),
    entry {
        field(unsigned int, context_idx);
        field(unsigned int, scheme_idx);
        field(unsigned long, esz);
    },
    fast_assign {
        __entry->context_idx = context_idx;
        __entry->scheme_idx = scheme_idx;
        __entry->esz = esz;
    },
    printk("ctx_idx=%u scheme_idx=%u esz=%lu", __entry->context_idx,
        __entry->scheme_idx, __entry->esz)
}

trace_event! {
    condition damos_before_apply,
    proto(unsigned int context_idx, unsigned int scheme_idx,
        unsigned int target_idx, struct damon_region *r,
        unsigned int nr_accesses, unsigned int nr_regions, bool do_trace),
    args(context_idx, scheme_idx, target_idx, r, nr_accesses, nr_regions, do_trace),
    condition(do_trace),
    entry {
        field(unsigned int, context_idx);
        field(unsigned int, scheme_idx);
        field(unsigned long, target_idx);
        field(unsigned long, start);
        field(unsigned long, end);
        field(unsigned int, nr_accesses);
        field(unsigned int, age);
        field(unsigned int, nr_regions);
    },
    fast_assign {
        __entry->context_idx = context_idx;
        __entry->scheme_idx = scheme_idx;
        __entry->target_idx = target_idx;
        __entry->start = r->ar.start;
        __entry->end = r->ar.end;
        __entry->nr_accesses = nr_accesses;
        __entry->age = r->age;
        __entry->nr_regions = nr_regions;
    },
    printk("ctx_idx=%u scheme_idx=%u target_idx=%lu nr_regions=%u %lu-%lu: %u %u",
        __entry->context_idx, __entry->scheme_idx, __entry->target_idx,
        __entry->nr_regions, __entry->start, __entry->end,
        __entry->nr_accesses, __entry->age)
}

trace_event! {
    damon_monitor_intervals_tune,
    proto(unsigned long sample_us),
    args(sample_us),
    entry { field(unsigned long, sample_us); },
    fast_assign { __entry->sample_us = sample_us; },
    printk("sample_us=%lu", __entry->sample_us)
}

trace_event! {
    condition damon_region_aggregated,
    proto(unsigned int target_id, struct damon_region *r,
        unsigned int nr_regions, unsigned int nr_probes),
    args(target_id, r, nr_regions, nr_probes),
    condition(nr_probes > 0),
    entry {
        field(unsigned long, target_id);
        field(unsigned long, start);
        field(unsigned long, end);
        field(unsigned int, nr_regions);
        field(unsigned int, nr_accesses);
        field(unsigned int, age);
        dynamic_array(unsigned char, probe_hits, nr_probes);
    },
    fast_assign {
        __entry->target_id = target_id;
        __entry->start = r->ar.start;
        __entry->end = r->ar.end;
        __entry->nr_regions = nr_regions;
        __entry->nr_accesses = r->nr_accesses;
        __entry->age = r->age;
        memcpy(__get_dynamic_array(probe_hits), r->probe_hits,
            sizeof(*r->probe_hits) * nr_probes);
    },
    printk("target_id=%lu nr_regions=%u %lu-%lu: %u %u probe_hits=%s",
        __entry->target_id, __entry->nr_regions, __entry->start,
        __entry->end, __entry->nr_accesses, __entry->age,
        __print_hex(__get_dynamic_array(probe_hits),
            __get_dynamic_array_len(probe_hits)))
}

trace_event! {
    damon_aggregated,
    proto(unsigned int target_id, struct damon_region *r, unsigned int nr_regions),
    args(target_id, r, nr_regions),
    entry {
        field(unsigned long, target_id);
        field(unsigned int, nr_regions);
        field(unsigned long, start);
        field(unsigned long, end);
        field(unsigned int, nr_accesses);
        field(unsigned int, age);
    },
    fast_assign {
        __entry->target_id = target_id;
        __entry->nr_regions = nr_regions;
        __entry->start = r->ar.start;
        __entry->end = r->ar.end;
        __entry->nr_accesses = r->nr_accesses;
        __entry->age = r->age;
    },
    printk("target_id=%lu nr_regions=%u %lu-%lu: %u %u",
        __entry->target_id, __entry->nr_regions, __entry->start,
        __entry->end, __entry->nr_accesses, __entry->age)
}

// #include <trace/define_trace.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
