/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright(C) 2015 Linaro Limited. All rights reserved.
 * Author: Mathieu Poirier <mathieu.poirier@linaro.org>
 */

// Dependency intent: GENMASK_ULL is supplied by the Linux bits definitions.

pub const CORESIGHT_ETM_PMU_NAME: &str = "cs_etm";

/*
 * The legacy Trace ID system based on fixed calculation from the cpu
 * number. This has been replaced by drivers using a dynamic allocation
 * system - but need to retain the legacy algorithm for backward comparibility
 * in certain situations:-
 * a) new perf running on older systems that generate the legacy mapping
 * b) older tools that may not update at the same time as the kernel.
 */
#[inline]
pub const fn coresight_legacy_cpu_trace_id(cpu: u64) -> u64 {
    0x10u64.wrapping_add(cpu.wrapping_mul(2))
}

/*
 * Interpretation of the PERF_RECORD_AUX_OUTPUT_HW_ID payload.
 * Used to associate a CPU with the CoreSight Trace ID.
 * [07:00] - Trace ID - uses 8 bits to make value easy to read in file.
 * [39:08] - Sink ID - as reported in /sys/bus/event_source/devices/cs_etm/sinks/
 *	      Added in minor version 1.
 * [55:40] - Unused (SBZ)
 * [59:56] - Minor Version - previously existing fields are compatible with
 *	      all minor versions.
 * [63:60] - Major Version - previously existing fields mean different things
 *	      in new major versions.
 */
pub const CS_AUX_HW_ID_TRACE_ID_MASK: u64 = 0xff;
pub const CS_AUX_HW_ID_SINK_ID_MASK: u64 = 0xffffffffff00;

pub const CS_AUX_HW_ID_MINOR_VERSION_MASK: u64 = 0x0f00000000000000;
pub const CS_AUX_HW_ID_MAJOR_VERSION_MASK: u64 = 0xf000000000000000;

pub const CS_AUX_HW_ID_MAJOR_VERSION: u32 = 0;
pub const CS_AUX_HW_ID_MINOR_VERSION: u32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
