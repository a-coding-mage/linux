// SPDX-License-Identifier: GPL-2.0+
/*
 *  comedi/drivers/ni_routing/ni_device_routes/pxi-6030e.c
 *  List of valid routes for specific NI boards.
 *
 *  COMEDI - Linux Control and Measurement Device Interface
 *  Copyright (C) 2016 Spencer E. Olson <olsonse@umich.edu>
 *
 *  This program is free software; you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation; either version 2 of the License, or
 *  (at your option) any later version.
 */

/* Generated using comedi/drivers/ni_routing/tools. */

// The route types and NI_* / TRIGGER_LINE symbols are supplied by the
// surrounding ni_device_routes implementation.

#[allow(non_upper_case_globals)]
static PFI_ROUTES: [[i32; 2]; 10] = [
    [NI_AI_StartTrigger, 0], [NI_AI_ReferenceTrigger, 0],
    [NI_AI_ConvertClock, 0], [NI_CtrSource(1), 0], [NI_CtrGate(1), 0],
    [NI_AO_SampleClock, 0], [NI_AO_StartTrigger, 0],
    [NI_AI_SampleClock, 0], [NI_CtrSource(0), 0], [NI_CtrGate(0), 0],
];

static TRIGGER_ROUTES: [i32; 11] = [
    NI_CtrSource(0), NI_CtrGate(0), NI_CtrInternalOutput(0), NI_CtrOut(0),
    NI_AI_SampleClock, NI_AI_StartTrigger, NI_AI_ReferenceTrigger,
    NI_AI_ConvertClock, NI_AO_SampleClock, NI_AO_StartTrigger, 0,
];
static TIMEBASE_20: [i32; 2] = [NI_20MHzTimebase, 0];
static CTR_SOURCE: [i32; 21] = [
    NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5),
    NI_PFI(6), NI_PFI(7), NI_PFI(8), NI_PFI(9), TRIGGER_LINE(0),
    TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3), TRIGGER_LINE(4),
    TRIGGER_LINE(5), TRIGGER_LINE(7), NI_MasterTimebase, NI_20MHzTimebase,
    NI_100kHzTimebase, NI_AnalogComparisonEvent,
];
static CTR_GATE_0: [i32; 20] = [
    NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5),
    NI_PFI(6), NI_PFI(7), NI_PFI(8), NI_PFI(9), TRIGGER_LINE(0),
    TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3), TRIGGER_LINE(4),
    TRIGGER_LINE(5), NI_CtrInternalOutput(1), NI_AI_StartTrigger,
    NI_AI_ReferenceTrigger, NI_AnalogComparisonEvent,
];
static CTR_GATE_1: [i32; 20] = [
    NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5),
    NI_PFI(6), NI_PFI(7), NI_PFI(8), NI_PFI(9), TRIGGER_LINE(0),
    TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3), TRIGGER_LINE(4),
    TRIGGER_LINE(5), NI_CtrInternalOutput(0), NI_AI_StartTrigger,
    NI_AI_ReferenceTrigger, NI_AnalogComparisonEvent,
];
static CTR_OUT_0: [i32; 8] = [
    TRIGGER_LINE(0), TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3),
    TRIGGER_LINE(4), TRIGGER_LINE(5), NI_CtrInternalOutput(0), 0,
];
static AI_SAMPLE: [i32; 20] = [
    NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5),
    NI_PFI(6), NI_PFI(7), NI_PFI(8), NI_PFI(9), TRIGGER_LINE(0),
    TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3), TRIGGER_LINE(4),
    TRIGGER_LINE(5), NI_CtrInternalOutput(0), NI_AI_SampleClockTimebase,
    NI_AnalogComparisonEvent, 0,
];
static SAMPLE_TIMEBASE: [i32; 22] = [
    NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5),
    NI_PFI(6), NI_PFI(7), NI_PFI(8), NI_PFI(9), TRIGGER_LINE(0),
    TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3), TRIGGER_LINE(4),
    TRIGGER_LINE(5), TRIGGER_LINE(7), NI_MasterTimebase, NI_20MHzTimebase,
    NI_100kHzTimebase, NI_AnalogComparisonEvent, 0,
];
static AI_START: [i32; 19] = [
    NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5),
    NI_PFI(6), NI_PFI(7), NI_PFI(8), NI_PFI(9), TRIGGER_LINE(0),
    TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3), TRIGGER_LINE(4),
    TRIGGER_LINE(5), NI_CtrInternalOutput(0), NI_AnalogComparisonEvent, 0,
];
static AI_REF: [i32; 18] = [
    NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5),
    NI_PFI(6), NI_PFI(7), NI_PFI(8), NI_PFI(9), TRIGGER_LINE(0),
    TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3), TRIGGER_LINE(4),
    TRIGGER_LINE(5), NI_AnalogComparisonEvent, 0,
];
static AI_CONVERT: [i32; 20] = [
    NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5),
    NI_PFI(6), NI_PFI(7), NI_PFI(8), NI_PFI(9), TRIGGER_LINE(0),
    TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3), TRIGGER_LINE(4),
    TRIGGER_LINE(5), NI_CtrInternalOutput(0), NI_AI_ConvertClockTimebase,
    NI_AnalogComparisonEvent, 0,
];
static AI_CONVERT_TIMEBASE: [i32; 5] = [
    TRIGGER_LINE(7), NI_AI_SampleClockTimebase, NI_MasterTimebase,
    NI_20MHzTimebase, 0,
];
static AI_PAUSE: [i32; 18] = [
    NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5),
    NI_PFI(6), NI_PFI(7), NI_PFI(8), NI_PFI(9), TRIGGER_LINE(0),
    TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3), TRIGGER_LINE(4),
    TRIGGER_LINE(5), NI_AnalogComparisonEvent, 0,
];
static AO_SAMPLE: [i32; 20] = [
    NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5),
    NI_PFI(6), NI_PFI(7), NI_PFI(8), NI_PFI(9), TRIGGER_LINE(0),
    TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3), TRIGGER_LINE(4),
    TRIGGER_LINE(5), NI_CtrInternalOutput(1), NI_AO_SampleClockTimebase,
    NI_AnalogComparisonEvent, 0,
];
static AO_START: [i32; 19] = [
    NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5),
    NI_PFI(6), NI_PFI(7), NI_PFI(8), NI_PFI(9), TRIGGER_LINE(0),
    TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3), TRIGGER_LINE(4),
    TRIGGER_LINE(5), NI_AI_StartTrigger, NI_AnalogComparisonEvent, 0,
];
static AO_PAUSE: [i32; 18] = AI_PAUSE;

// The following table is the direct translation of the C initializer.  Each
// source pointer refers to the corresponding static, NUL-terminated route.
#[allow(non_upper_case_globals)]
pub static mut ni_pxi_6030e_device_routes: ni_device_routes = ni_device_routes {
    device: "pxi-6030e\0".as_ptr() as *const _,
    routes: [
        ni_route_set { dest: NI_PFI(0), src: PFI_ROUTES[0].as_ptr() },
        ni_route_set { dest: NI_PFI(1), src: PFI_ROUTES[1].as_ptr() },
        ni_route_set { dest: NI_PFI(2), src: PFI_ROUTES[2].as_ptr() },
        ni_route_set { dest: NI_PFI(3), src: PFI_ROUTES[3].as_ptr() },
        ni_route_set { dest: NI_PFI(4), src: PFI_ROUTES[4].as_ptr() },
        ni_route_set { dest: NI_PFI(5), src: PFI_ROUTES[5].as_ptr() },
        ni_route_set { dest: NI_PFI(6), src: PFI_ROUTES[6].as_ptr() },
        ni_route_set { dest: NI_PFI(7), src: PFI_ROUTES[7].as_ptr() },
        ni_route_set { dest: NI_PFI(8), src: PFI_ROUTES[8].as_ptr() },
        ni_route_set { dest: NI_PFI(9), src: PFI_ROUTES[9].as_ptr() },
        ni_route_set { dest: TRIGGER_LINE(0), src: TRIGGER_ROUTES.as_ptr() },
        ni_route_set { dest: TRIGGER_LINE(1), src: TRIGGER_ROUTES.as_ptr() },
        ni_route_set { dest: TRIGGER_LINE(2), src: TRIGGER_ROUTES.as_ptr() },
        ni_route_set { dest: TRIGGER_LINE(3), src: TRIGGER_ROUTES.as_ptr() },
        ni_route_set { dest: TRIGGER_LINE(4), src: TRIGGER_ROUTES.as_ptr() },
        ni_route_set { dest: TRIGGER_LINE(5), src: TRIGGER_ROUTES.as_ptr() },
        ni_route_set { dest: TRIGGER_LINE(7), src: TIMEBASE_20.as_ptr() },
        ni_route_set { dest: NI_CtrSource(0), src: CTR_SOURCE.as_ptr() },
        ni_route_set { dest: NI_CtrSource(1), src: CTR_SOURCE.as_ptr() },
        ni_route_set { dest: NI_CtrGate(0), src: CTR_GATE_0.as_ptr() },
        ni_route_set { dest: NI_CtrGate(1), src: CTR_GATE_1.as_ptr() },
        ni_route_set { dest: NI_CtrOut(0), src: CTR_OUT_0.as_ptr() },
        ni_route_set { dest: NI_CtrOut(1), src: [NI_CtrInternalOutput(1), 0].as_ptr() },
        ni_route_set { dest: NI_AI_SampleClock, src: AI_SAMPLE.as_ptr() },
        ni_route_set { dest: NI_AI_SampleClockTimebase, src: SAMPLE_TIMEBASE.as_ptr() },
        ni_route_set { dest: NI_AI_StartTrigger, src: AI_START.as_ptr() },
        ni_route_set { dest: NI_AI_ReferenceTrigger, src: AI_REF.as_ptr() },
        ni_route_set { dest: NI_AI_ConvertClock, src: AI_CONVERT.as_ptr() },
        ni_route_set { dest: NI_AI_ConvertClockTimebase, src: AI_CONVERT_TIMEBASE.as_ptr() },
        ni_route_set { dest: NI_AI_PauseTrigger, src: AI_PAUSE.as_ptr() },
        ni_route_set { dest: NI_AI_HoldComplete, src: [NI_AI_HoldCompleteEvent, 0].as_ptr() },
        ni_route_set { dest: NI_AO_SampleClock, src: AO_SAMPLE.as_ptr() },
        ni_route_set { dest: NI_AO_SampleClockTimebase, src: SAMPLE_TIMEBASE.as_ptr() },
        ni_route_set { dest: NI_AO_StartTrigger, src: AO_START.as_ptr() },
        ni_route_set { dest: NI_AO_PauseTrigger, src: AO_PAUSE.as_ptr() },
        ni_route_set { dest: NI_MasterTimebase, src: [TRIGGER_LINE(7), NI_20MHzTimebase, 0].as_ptr() },
        ni_route_set { dest: 0, src: core::ptr::null() },
    ],
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
