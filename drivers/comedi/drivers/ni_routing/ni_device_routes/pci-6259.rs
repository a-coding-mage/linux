// SPDX-License-Identifier: GPL-2.0+
/*
 * comedi/drivers/ni_routing/ni_device_routes/pci-6259.rs
 * List of valid routes for the pci-6259 NI board.
 *
 * The contents of this file are generated using the tools in
 * comedi/drivers/ni_routing/tools.
 */

// Translated from the C implementation.  The routing symbols and structures
// below are supplied by the surrounding ni_device_routes implementation.

macro_rules! pfi_sources {
    () => { [
        NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5),
        NI_PFI(6), NI_PFI(7), NI_PFI(8), NI_PFI(9), NI_PFI(10), NI_PFI(11),
        NI_PFI(12), NI_PFI(13), NI_PFI(14), NI_PFI(15),
    ] };
}

macro_rules! trigger_sources {
    () => { [
        TRIGGER_LINE(0), TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3),
        TRIGGER_LINE(4), TRIGGER_LINE(5), TRIGGER_LINE(6), TRIGGER_LINE(7),
    ] };
}

macro_rules! pfi_dest_route {
    ($n:expr) => { ni_route_set {
        dest: NI_PFI($n),
        src: &[
            TRIGGER_LINE(0), TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3),
            TRIGGER_LINE(4), TRIGGER_LINE(5), TRIGGER_LINE(6), TRIGGER_LINE(7),
            NI_CtrSource(0), NI_CtrSource(1), NI_CtrGate(0), NI_CtrGate(1),
            NI_CtrInternalOutput(0), NI_CtrInternalOutput(1), NI_AI_SampleClock,
            NI_AI_StartTrigger, NI_AI_ReferenceTrigger, NI_AI_ConvertClock,
            NI_AO_SampleClock, NI_AO_StartTrigger, NI_DI_SampleClock,
            NI_DO_SampleClock, NI_FrequencyOutput, NI_ChangeDetectionEvent,
            NI_AnalogComparisonEvent, 0,
        ],
    } };
}

macro_rules! trigger_dest_route {
    ($n:expr) => { ni_route_set {
        dest: TRIGGER_LINE($n),
        src: &[
            NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5),
            NI_CtrSource(0), NI_CtrSource(1), NI_CtrGate(0), NI_CtrGate(1),
            NI_CtrInternalOutput(0), NI_CtrInternalOutput(1), NI_AI_SampleClock,
            NI_AI_StartTrigger, NI_AI_ReferenceTrigger, NI_AI_ConvertClock,
            NI_AI_PauseTrigger, NI_AO_SampleClock, NI_AO_StartTrigger,
            NI_AO_PauseTrigger, NI_10MHzRefClock, NI_FrequencyOutput,
            NI_ChangeDetectionEvent, NI_AnalogComparisonEvent, 0,
        ],
    } };
}

macro_rules! counter_route {
    ($dest:expr, $extra:expr) => { ni_route_set {
        dest: $dest,
        src: &[
            NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5),
            NI_PFI(6), NI_PFI(7), NI_PFI(8), NI_PFI(9), NI_PFI(10), NI_PFI(11),
            NI_PFI(12), NI_PFI(13), NI_PFI(14), NI_PFI(15),
            TRIGGER_LINE(0), TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3),
            TRIGGER_LINE(4), TRIGGER_LINE(5), TRIGGER_LINE(6), TRIGGER_LINE(7),
            $extra, NI_AnalogComparisonEvent, 0,
        ],
    } };
}

pub static mut ni_pci_6259_device_routes: ni_device_routes = ni_device_routes {
    device: "pci-6259",
    routes: &[
        pfi_dest_route!(0), pfi_dest_route!(1), pfi_dest_route!(2),
        pfi_dest_route!(3), pfi_dest_route!(4), pfi_dest_route!(5),
        pfi_dest_route!(6), pfi_dest_route!(7), pfi_dest_route!(8),
        pfi_dest_route!(9), pfi_dest_route!(10), pfi_dest_route!(11),
        pfi_dest_route!(12), pfi_dest_route!(13), pfi_dest_route!(14),
        pfi_dest_route!(15),
        trigger_dest_route!(0), trigger_dest_route!(1), trigger_dest_route!(2),
        trigger_dest_route!(3), trigger_dest_route!(4), trigger_dest_route!(5),
        trigger_dest_route!(6), trigger_dest_route!(7),
        counter_route!(NI_CtrSource(0), NI_CtrGate(1)),
        counter_route!(NI_CtrSource(1), NI_CtrGate(0)),
        counter_route!(NI_CtrGate(0), NI_CtrSource(1)),
        counter_route!(NI_CtrGate(1), NI_CtrSource(0)),
        counter_route!(NI_CtrAux(0), NI_CtrSource(1)),
        counter_route!(NI_CtrAux(1), NI_CtrSource(0)),
        counter_route!(NI_CtrA(0), NI_AnalogComparisonEvent),
        counter_route!(NI_CtrA(1), NI_AnalogComparisonEvent),
        counter_route!(NI_CtrB(0), NI_AnalogComparisonEvent),
        counter_route!(NI_CtrB(1), NI_AnalogComparisonEvent),
        counter_route!(NI_CtrZ(0), NI_AnalogComparisonEvent),
        counter_route!(NI_CtrZ(1), NI_AnalogComparisonEvent),
        counter_route!(NI_CtrArmStartTrigger(0), NI_CtrInternalOutput(1)),
        counter_route!(NI_CtrArmStartTrigger(1), NI_CtrInternalOutput(0)),
        counter_route!(NI_AI_SampleClock, NI_CtrInternalOutput(0)),
        counter_route!(NI_AI_SampleClockTimebase, NI_20MHzTimebase),
        counter_route!(NI_AI_StartTrigger, NI_CtrInternalOutput(0)),
        counter_route!(NI_AI_ReferenceTrigger, NI_AnalogComparisonEvent),
        counter_route!(NI_AI_ConvertClock, NI_CtrInternalOutput(0)),
        ni_route_set { dest: NI_AI_ConvertClockTimebase, src: &[NI_AI_SampleClockTimebase, NI_20MHzTimebase, 0] },
        counter_route!(NI_AI_PauseTrigger, NI_AnalogComparisonEvent),
        counter_route!(NI_AO_SampleClock, NI_CtrInternalOutput(0)),
        counter_route!(NI_AO_SampleClockTimebase, NI_20MHzTimebase),
        counter_route!(NI_AO_StartTrigger, NI_AI_StartTrigger),
        counter_route!(NI_AO_PauseTrigger, NI_AnalogComparisonEvent),
        counter_route!(NI_DI_SampleClock, NI_CtrInternalOutput(0)),
        counter_route!(NI_DO_SampleClock, NI_CtrInternalOutput(0)),
        ni_route_set { dest: 0, src: &[] },
    ],
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
