// SPDX-License-Identifier: GPL-2.0+
/*
 * comedi/drivers/ni_routing/ni_device_routes/pxi-6251.rs
 * List of valid routes for the NI PXI-6251 board.
 *
 * The contents of this file are generated using the tools in
 * comedi/drivers/ni_routing/tools.
 */

// C dependencies: ../ni_device_routes.h and all.h.

/* The C source uses compound literals for each route's source list.  These
 * constants retain the same lists and ordering for the Rust representation. */
const PFI_DEST_SOURCES: &[i32] = &[
    TRIGGER_LINE(0), TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3),
    TRIGGER_LINE(4), TRIGGER_LINE(5), TRIGGER_LINE(6), TRIGGER_LINE(7),
    NI_CtrSource(0), NI_CtrSource(1), NI_CtrGate(0),
    NI_CtrInternalOutput(0), NI_CtrInternalOutput(1), PXI_Star,
    NI_AI_SampleClock, NI_AI_StartTrigger, NI_AI_ReferenceTrigger,
    NI_AI_ConvertClock, NI_AO_SampleClock, NI_AO_StartTrigger,
    NI_DI_SampleClock, NI_DO_SampleClock, NI_FrequencyOutput,
    NI_ChangeDetectionEvent, NI_AnalogComparisonEvent, 0,
];

const TRIGGER_DEST_SOURCES: &[i32] = &[
    NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5),
    NI_CtrSource(0), NI_CtrSource(1), NI_CtrGate(0),
    NI_CtrInternalOutput(0), NI_CtrInternalOutput(1), NI_AI_SampleClock,
    NI_AI_StartTrigger, NI_AI_ReferenceTrigger, NI_AI_ConvertClock,
    NI_AO_SampleClock, NI_AO_StartTrigger, NI_10MHzRefClock,
    NI_FrequencyOutput, NI_ChangeDetectionEvent, NI_AnalogComparisonEvent, 0,
];

const GENERAL_SOURCES: &[i32] = &[
    NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5),
    NI_PFI(6), NI_PFI(7), NI_PFI(8), NI_PFI(9), NI_PFI(10), NI_PFI(11),
    NI_PFI(12), NI_PFI(13), NI_PFI(14), NI_PFI(15), TRIGGER_LINE(0),
    TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3), TRIGGER_LINE(4),
    TRIGGER_LINE(5), TRIGGER_LINE(6), TRIGGER_LINE(7), PXI_Star,
    NI_AnalogComparisonEvent, 0,
];

macro_rules! route {
    ($dest:expr, $src:expr) => { ni_route_set { dest: $dest, src: $src } };
}

pub static mut ni_pxi_6251_device_routes: ni_device_routes = ni_device_routes {
    device: "pxi-6251",
    routes: &[
        route!(NI_PFI(0), PFI_DEST_SOURCES), route!(NI_PFI(1), PFI_DEST_SOURCES),
        route!(NI_PFI(2), PFI_DEST_SOURCES), route!(NI_PFI(3), PFI_DEST_SOURCES),
        route!(NI_PFI(4), PFI_DEST_SOURCES), route!(NI_PFI(5), PFI_DEST_SOURCES),
        route!(NI_PFI(6), PFI_DEST_SOURCES), route!(NI_PFI(7), PFI_DEST_SOURCES),
        route!(NI_PFI(8), PFI_DEST_SOURCES), route!(NI_PFI(9), PFI_DEST_SOURCES),
        route!(NI_PFI(10), PFI_DEST_SOURCES), route!(NI_PFI(11), PFI_DEST_SOURCES),
        route!(NI_PFI(12), PFI_DEST_SOURCES), route!(NI_PFI(13), PFI_DEST_SOURCES),
        route!(NI_PFI(14), PFI_DEST_SOURCES), route!(NI_PFI(15), PFI_DEST_SOURCES),
        route!(TRIGGER_LINE(0), TRIGGER_DEST_SOURCES), route!(TRIGGER_LINE(1), TRIGGER_DEST_SOURCES),
        route!(TRIGGER_LINE(2), TRIGGER_DEST_SOURCES), route!(TRIGGER_LINE(3), TRIGGER_DEST_SOURCES),
        route!(TRIGGER_LINE(4), TRIGGER_DEST_SOURCES), route!(TRIGGER_LINE(5), TRIGGER_DEST_SOURCES),
        route!(TRIGGER_LINE(6), TRIGGER_DEST_SOURCES), route!(TRIGGER_LINE(7), TRIGGER_DEST_SOURCES),
        route!(NI_CtrSource(0), GENERAL_SOURCES), route!(NI_CtrSource(1), GENERAL_SOURCES),
        route!(NI_CtrGate(0), GENERAL_SOURCES), route!(NI_CtrGate(1), GENERAL_SOURCES),
        route!(NI_CtrAux(0), GENERAL_SOURCES), route!(NI_CtrAux(1), GENERAL_SOURCES),
        route!(NI_CtrA(0), GENERAL_SOURCES), route!(NI_CtrA(1), GENERAL_SOURCES),
        route!(NI_CtrB(0), GENERAL_SOURCES), route!(NI_CtrB(1), GENERAL_SOURCES),
        route!(NI_CtrZ(0), GENERAL_SOURCES), route!(NI_CtrZ(1), GENERAL_SOURCES),
        route!(NI_CtrArmStartTrigger(0), GENERAL_SOURCES),
        route!(NI_CtrArmStartTrigger(1), GENERAL_SOURCES),
        route!(NI_AI_SampleClock, GENERAL_SOURCES), route!(NI_AI_SampleClockTimebase, GENERAL_SOURCES),
        route!(NI_AI_StartTrigger, GENERAL_SOURCES), route!(NI_AI_ReferenceTrigger, GENERAL_SOURCES),
        route!(NI_AI_ConvertClock, GENERAL_SOURCES), route!(NI_AI_ConvertClockTimebase, GENERAL_SOURCES),
        route!(NI_AI_PauseTrigger, GENERAL_SOURCES), route!(NI_AO_SampleClock, GENERAL_SOURCES),
        route!(NI_AO_SampleClockTimebase, GENERAL_SOURCES), route!(NI_AO_StartTrigger, GENERAL_SOURCES),
        route!(NI_AO_PauseTrigger, GENERAL_SOURCES), route!(NI_DI_SampleClock, GENERAL_SOURCES),
        route!(NI_DO_SampleClock, GENERAL_SOURCES),
        ni_route_set { dest: 0, src: &[] }, // Termination of list
    ],
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
