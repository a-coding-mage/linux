// SPDX-License-Identifier: GPL-2.0+
/*
 *  comedi/drivers/ni_routing/ni_device_routes/pci-6229.c
 *  List of valid routes for specific NI boards.
 *
 *  The contents of this file are generated using the tools in
 *  comedi/drivers/ni_routing/tools
 */

// Dependencies supplied by ../ni_device_routes.h and all.h remain external.

const PFI_ALL: [i32; 25] = [
    TRIGGER_LINE(0), TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3),
    TRIGGER_LINE(4), TRIGGER_LINE(5), TRIGGER_LINE(6), TRIGGER_LINE(7),
    NI_CtrSource(0), NI_CtrSource(1), NI_CtrGate(0), NI_CtrGate(1),
    NI_CtrInternalOutput(0), NI_CtrInternalOutput(1), NI_AI_SampleClock,
    NI_AI_StartTrigger, NI_AI_ReferenceTrigger, NI_AI_ConvertClock,
    NI_AO_SampleClock, NI_AO_StartTrigger, NI_DI_SampleClock,
    NI_DO_SampleClock, NI_FrequencyOutput, NI_ChangeDetectionEvent, 0,
];

const TRIGGER_ALL: [i32; 24] = [
    NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5),
    NI_CtrSource(0), NI_CtrSource(1), NI_CtrGate(0), NI_CtrGate(1),
    NI_CtrInternalOutput(0), NI_CtrInternalOutput(1), NI_AI_SampleClock,
    NI_AI_StartTrigger, NI_AI_ReferenceTrigger, NI_AI_ConvertClock,
    NI_AI_PauseTrigger, NI_AO_SampleClock, NI_AO_StartTrigger,
    NI_AO_PauseTrigger, NI_10MHzRefClock, NI_FrequencyOutput,
    NI_ChangeDetectionEvent, 0,
];

const CTR_SOURCE_0: [i32; 29] = [
    NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5),
    NI_PFI(6), NI_PFI(7), NI_PFI(8), NI_PFI(9), NI_PFI(10), NI_PFI(11),
    NI_PFI(12), NI_PFI(13), NI_PFI(14), NI_PFI(15), TRIGGER_LINE(0),
    TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3), TRIGGER_LINE(4),
    TRIGGER_LINE(5), TRIGGER_LINE(6), TRIGGER_LINE(7), NI_CtrGate(1),
    NI_20MHzTimebase, NI_80MHzTimebase, NI_100kHzTimebase, 0,
];
const CTR_SOURCE_1: [i32; 29] = [
    NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5),
    NI_PFI(6), NI_PFI(7), NI_PFI(8), NI_PFI(9), NI_PFI(10), NI_PFI(11),
    NI_PFI(12), NI_PFI(13), NI_PFI(14), NI_PFI(15), TRIGGER_LINE(0),
    TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3), TRIGGER_LINE(4),
    TRIGGER_LINE(5), TRIGGER_LINE(6), TRIGGER_LINE(7), NI_CtrGate(0),
    NI_20MHzTimebase, NI_80MHzTimebase, NI_100kHzTimebase, 0,
];

const PFI_TRIGGER: [i32; 24] = [
    NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5),
    NI_PFI(6), NI_PFI(7), NI_PFI(8), NI_PFI(9), NI_PFI(10), NI_PFI(11),
    NI_PFI(12), NI_PFI(13), NI_PFI(14), NI_PFI(15), TRIGGER_LINE(0),
    TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3), TRIGGER_LINE(4),
    TRIGGER_LINE(5), TRIGGER_LINE(6), TRIGGER_LINE(7),
];
const PFI_TRIGGER_END: [i32; 25] = [
    NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5),
    NI_PFI(6), NI_PFI(7), NI_PFI(8), NI_PFI(9), NI_PFI(10), NI_PFI(11),
    NI_PFI(12), NI_PFI(13), NI_PFI(14), NI_PFI(15), TRIGGER_LINE(0),
    TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3), TRIGGER_LINE(4),
    TRIGGER_LINE(5), TRIGGER_LINE(6), TRIGGER_LINE(7), 0,
];

// Route declarations retain the C table's destinations, source ordering, and
// terminating zero.  The surrounding types and constructors are external.
pub static mut ni_pci_6229_device_routes: ni_device_routes = ni_device_routes {
    device: "pci-6229",
    routes: &[
        ni_route_set { dest: NI_PFI(0), src: &PFI_ALL },
        ni_route_set { dest: NI_PFI(1), src: &PFI_ALL },
        ni_route_set { dest: NI_PFI(2), src: &PFI_ALL },
        ni_route_set { dest: NI_PFI(3), src: &PFI_ALL },
        ni_route_set { dest: NI_PFI(4), src: &PFI_ALL },
        ni_route_set { dest: NI_PFI(5), src: &PFI_ALL },
        ni_route_set { dest: NI_PFI(6), src: &PFI_ALL },
        ni_route_set { dest: NI_PFI(7), src: &PFI_ALL },
        ni_route_set { dest: NI_PFI(8), src: &PFI_ALL },
        ni_route_set { dest: NI_PFI(9), src: &PFI_ALL },
        ni_route_set { dest: NI_PFI(10), src: &PFI_ALL },
        ni_route_set { dest: NI_PFI(11), src: &PFI_ALL },
        ni_route_set { dest: NI_PFI(12), src: &PFI_ALL },
        ni_route_set { dest: NI_PFI(13), src: &PFI_ALL },
        ni_route_set { dest: NI_PFI(14), src: &PFI_ALL },
        ni_route_set { dest: NI_PFI(15), src: &PFI_ALL },
        ni_route_set { dest: TRIGGER_LINE(0), src: &TRIGGER_ALL },
        ni_route_set { dest: TRIGGER_LINE(1), src: &TRIGGER_ALL },
        ni_route_set { dest: TRIGGER_LINE(2), src: &TRIGGER_ALL },
        ni_route_set { dest: TRIGGER_LINE(3), src: &TRIGGER_ALL },
        ni_route_set { dest: TRIGGER_LINE(4), src: &TRIGGER_ALL },
        ni_route_set { dest: TRIGGER_LINE(5), src: &TRIGGER_ALL },
        ni_route_set { dest: TRIGGER_LINE(6), src: &TRIGGER_ALL },
        ni_route_set { dest: TRIGGER_LINE(7), src: &TRIGGER_ALL },
        ni_route_set { dest: NI_CtrSource(0), src: &CTR_SOURCE_0 },
        ni_route_set { dest: NI_CtrSource(1), src: &CTR_SOURCE_1 },
        ni_route_set { dest: NI_CtrGate(0), src: &PFI_TRIGGER_END },
        ni_route_set { dest: NI_CtrGate(1), src: &PFI_TRIGGER_END },
        ni_route_set { dest: NI_CtrAux(0), src: &PFI_TRIGGER_END },
        ni_route_set { dest: NI_CtrAux(1), src: &PFI_TRIGGER_END },
        ni_route_set { dest: NI_CtrA(0), src: &PFI_TRIGGER_END },
        ni_route_set { dest: NI_CtrA(1), src: &PFI_TRIGGER_END },
        ni_route_set { dest: NI_CtrB(0), src: &PFI_TRIGGER_END },
        ni_route_set { dest: NI_CtrB(1), src: &PFI_TRIGGER_END },
        ni_route_set { dest: NI_CtrZ(0), src: &PFI_TRIGGER_END },
        ni_route_set { dest: NI_CtrZ(1), src: &PFI_TRIGGER_END },
        ni_route_set { dest: NI_CtrArmStartTrigger(0), src: &PFI_TRIGGER_END },
        ni_route_set { dest: NI_CtrArmStartTrigger(1), src: &PFI_TRIGGER_END },
        ni_route_set { dest: NI_AI_SampleClock, src: &PFI_TRIGGER_END },
        ni_route_set { dest: NI_AI_SampleClockTimebase, src: &PFI_TRIGGER_END },
        ni_route_set { dest: NI_AI_StartTrigger, src: &PFI_TRIGGER_END },
        ni_route_set { dest: NI_AI_ReferenceTrigger, src: &PFI_TRIGGER_END },
        ni_route_set { dest: NI_AI_ConvertClock, src: &PFI_TRIGGER_END },
        ni_route_set { dest: NI_AI_ConvertClockTimebase, src: &[NI_AI_SampleClockTimebase, NI_20MHzTimebase, 0] },
        ni_route_set { dest: NI_AI_PauseTrigger, src: &PFI_TRIGGER_END },
        ni_route_set { dest: NI_AO_SampleClock, src: &PFI_TRIGGER_END },
        ni_route_set { dest: NI_AO_SampleClockTimebase, src: &PFI_TRIGGER_END },
        ni_route_set { dest: NI_AO_StartTrigger, src: &PFI_TRIGGER_END },
        ni_route_set { dest: NI_AO_PauseTrigger, src: &PFI_TRIGGER_END },
        ni_route_set { dest: NI_DI_SampleClock, src: &PFI_TRIGGER_END },
        ni_route_set { dest: NI_DO_SampleClock, src: &PFI_TRIGGER_END },
        ni_route_set { dest: 0, src: &[] },
    ],
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
