// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2025 Google LLC
 * Author: Vincent Donnefort <vdonnefort@google.com>
 */

// Dependencies supplied by nvhe/mm.h, nvhe/trace.h, and nvhe/define_events.h

pub unsafe fn __tracing_enable_event(id: u16, enable: bool) -> i32 {
    let event_id = &raw mut __hyp_event_ids_start.add(id as usize);
    let mut enabled: *mut atomic_t;

    if event_id >= __hyp_event_ids_end {
        return -(EINVAL as i32);
    }

    enabled = hyp_fixmap_map(__hyp_pa(&raw mut (*event_id).enabled));
    atomic_set(enabled, enable);
    hyp_fixmap_unmap();

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
