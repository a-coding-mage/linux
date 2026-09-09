// SPDX-License-Identifier: GPL-2.0+
/*
 *  comedi/drivers/ni_routing/ni_device_routes/pci-6221.c
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

// The original C file is generated using the tools in
// comedi/drivers/ni_routing/tools.

const PFI_ROUTES: &[i32] = &[
    TRIGGER_LINE(0), TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3),
    TRIGGER_LINE(4), TRIGGER_LINE(5), TRIGGER_LINE(6), TRIGGER_LINE(7),
    NI_CtrSource(0), NI_CtrSource(1), NI_CtrGate(0), NI_CtrGate(1),
    NI_CtrInternalOutput(0), NI_CtrInternalOutput(1), NI_AI_SampleClock,
    NI_AI_StartTrigger, NI_AI_ReferenceTrigger, NI_AI_ConvertClock,
    NI_AO_SampleClock, NI_AO_StartTrigger, NI_DI_SampleClock,
    NI_DO_SampleClock, NI_FrequencyOutput, NI_ChangeDetectionEvent, 0,
];

const TRIGGER_ROUTES: &[i32] = &[
    NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5),
    NI_CtrSource(0), NI_CtrSource(1), NI_CtrGate(0), NI_CtrGate(1),
    NI_CtrInternalOutput(0), NI_CtrInternalOutput(1), NI_AI_SampleClock,
    NI_AI_StartTrigger, NI_AI_ReferenceTrigger, NI_AI_ConvertClock,
    NI_AI_PauseTrigger, NI_AO_SampleClock, NI_AO_StartTrigger,
    NI_AO_PauseTrigger, NI_10MHzRefClock, NI_FrequencyOutput,
    NI_ChangeDetectionEvent, 0,
];

const COUNTER_SOURCE: &[i32] = &[
    NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5), NI_PFI(6),
    NI_PFI(7), NI_PFI(8), NI_PFI(9), NI_PFI(10), NI_PFI(11), NI_PFI(12),
    NI_PFI(13), NI_PFI(14), NI_PFI(15), TRIGGER_LINE(0), TRIGGER_LINE(1),
    TRIGGER_LINE(2), TRIGGER_LINE(3), TRIGGER_LINE(4), TRIGGER_LINE(5),
    TRIGGER_LINE(6), TRIGGER_LINE(7), NI_CtrGate(1), NI_20MHzTimebase,
    NI_80MHzTimebase, NI_100kHzTimebase, 0,
];

const COUNTER_GATE: &[i32] = &[
    NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5), NI_PFI(6),
    NI_PFI(7), NI_PFI(8), NI_PFI(9), NI_PFI(10), NI_PFI(11), NI_PFI(12),
    NI_PFI(13), NI_PFI(14), NI_PFI(15), TRIGGER_LINE(0), TRIGGER_LINE(1),
    TRIGGER_LINE(2), TRIGGER_LINE(3), TRIGGER_LINE(4), TRIGGER_LINE(5),
    TRIGGER_LINE(6), TRIGGER_LINE(7), NI_CtrSource(1), NI_CtrInternalOutput(1),
    NI_AI_StartTrigger, NI_AI_ReferenceTrigger, 0,
];

const DIGITAL_ROUTES: &[i32] = &[
    NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5),
    NI_PFI(6), NI_PFI(7), NI_PFI(8), NI_PFI(9), NI_PFI(10), NI_PFI(11),
    NI_PFI(12), NI_PFI(13), NI_PFI(14), NI_PFI(15), TRIGGER_LINE(0),
    TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3), TRIGGER_LINE(4),
    TRIGGER_LINE(5), TRIGGER_LINE(6), TRIGGER_LINE(7), NI_CtrInternalOutput(0),
    NI_CtrInternalOutput(1), NI_AI_SampleClock, NI_AI_ConvertClock,
    NI_AO_SampleClock, NI_FrequencyOutput, NI_ChangeDetectionEvent, 0,
];

macro_rules! route { ($dest:expr, $src:expr) => { ni_route_set { dest: $dest, src: $src } }; }

pub static mut ni_pci_6221_device_routes: ni_device_routes = ni_device_routes {
    device: "pci-6221",
    routes: &[
        route!(NI_PFI(0), PFI_ROUTES), route!(NI_PFI(1), PFI_ROUTES),
        route!(NI_PFI(2), PFI_ROUTES), route!(NI_PFI(3), PFI_ROUTES),
        route!(NI_PFI(4), PFI_ROUTES), route!(NI_PFI(5), PFI_ROUTES),
        route!(NI_PFI(6), PFI_ROUTES), route!(NI_PFI(7), PFI_ROUTES),
        route!(NI_PFI(8), PFI_ROUTES), route!(NI_PFI(9), PFI_ROUTES),
        route!(NI_PFI(10), PFI_ROUTES), route!(NI_PFI(11), PFI_ROUTES),
        route!(NI_PFI(12), PFI_ROUTES), route!(NI_PFI(13), PFI_ROUTES),
        route!(NI_PFI(14), PFI_ROUTES), route!(NI_PFI(15), PFI_ROUTES),
        route!(TRIGGER_LINE(0), TRIGGER_ROUTES), route!(TRIGGER_LINE(1), TRIGGER_ROUTES),
        route!(TRIGGER_LINE(2), TRIGGER_ROUTES), route!(TRIGGER_LINE(3), TRIGGER_ROUTES),
        route!(TRIGGER_LINE(4), TRIGGER_ROUTES), route!(TRIGGER_LINE(5), TRIGGER_ROUTES),
        route!(TRIGGER_LINE(6), TRIGGER_ROUTES), route!(TRIGGER_LINE(7), TRIGGER_ROUTES),
        route!(NI_CtrSource(0), COUNTER_SOURCE), route!(NI_CtrSource(1), COUNTER_GATE),
        route!(NI_CtrGate(0), COUNTER_GATE), route!(NI_CtrGate(1), COUNTER_GATE),
        route!(NI_CtrAux(0), COUNTER_GATE), route!(NI_CtrAux(1), COUNTER_GATE),
        route!(NI_CtrA(0), DIGITAL_ROUTES), route!(NI_CtrA(1), DIGITAL_ROUTES),
        route!(NI_CtrB(0), DIGITAL_ROUTES), route!(NI_CtrB(1), DIGITAL_ROUTES),
        route!(NI_CtrZ(0), DIGITAL_ROUTES), route!(NI_CtrZ(1), DIGITAL_ROUTES),
        route!(NI_CtrArmStartTrigger(0), DIGITAL_ROUTES),
        route!(NI_CtrArmStartTrigger(1), DIGITAL_ROUTES),
        route!(NI_AI_SampleClock, DIGITAL_ROUTES), route!(NI_AI_StartTrigger, DIGITAL_ROUTES),
        route!(NI_AI_ReferenceTrigger, DIGITAL_ROUTES), route!(NI_AI_ConvertClock, DIGITAL_ROUTES),
        route!(NI_AI_ConvertClockTimebase, &[NI_AI_SampleClockTimebase, NI_20MHzTimebase, 0]),
        route!(NI_AI_PauseTrigger, DIGITAL_ROUTES), route!(NI_AO_SampleClock, DIGITAL_ROUTES),
        route!(NI_AO_SampleClockTimebase, &[NI_20MHzTimebase, NI_100kHzTimebase, 0]),
        route!(NI_AO_StartTrigger, DIGITAL_ROUTES), route!(NI_AO_PauseTrigger, DIGITAL_ROUTES),
        route!(NI_DI_SampleClock, DIGITAL_ROUTES), route!(NI_DO_SampleClock, DIGITAL_ROUTES),
        route!(0, &[]),
    ],
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
