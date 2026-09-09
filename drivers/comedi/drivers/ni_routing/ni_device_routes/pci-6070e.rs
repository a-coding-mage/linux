// SPDX-License-Identifier: GPL-2.0+
/*
 *  comedi/drivers/ni_routing/ni_device_routes/pci-6070e.c
 *  List of valid routes for specific NI boards.
 *
 *  The contents of this file are generated using the tools in
 *  comedi/drivers/ni_routing/tools
 */

// Dependencies supplied by the surrounding NI routing module.

macro_rules! sources {
    ($($x:expr),+ $(,)?) => { &[$($x,)+ 0] as &'static [i32] };
}

macro_rules! common_trigger_sources {
    () => { sources!(
        NI_CtrSource(0), NI_CtrGate(0), NI_CtrInternalOutput(0), NI_CtrOut(0),
        NI_AI_SampleClock, NI_AI_StartTrigger, NI_AI_ReferenceTrigger,
        NI_AI_ConvertClock, NI_AO_SampleClock, NI_AO_StartTrigger
    ) };
}

pub static mut ni_pci_6070e_device_routes: ni_device_routes = ni_device_routes {
    device: "pci-6070e",
    routes: &[
        ni_route_set { dest: NI_PFI(0), src: sources!(NI_AI_StartTrigger) },
        ni_route_set { dest: NI_PFI(1), src: sources!(NI_AI_ReferenceTrigger) },
        ni_route_set { dest: NI_PFI(2), src: sources!(NI_AI_ConvertClock) },
        ni_route_set { dest: NI_PFI(3), src: sources!(NI_CtrSource(1)) },
        ni_route_set { dest: NI_PFI(4), src: sources!(NI_CtrGate(1)) },
        ni_route_set { dest: NI_PFI(5), src: sources!(NI_AO_SampleClock) },
        ni_route_set { dest: NI_PFI(6), src: sources!(NI_AO_StartTrigger) },
        ni_route_set { dest: NI_PFI(7), src: sources!(NI_AI_SampleClock) },
        ni_route_set { dest: NI_PFI(8), src: sources!(NI_CtrSource(0)) },
        ni_route_set { dest: NI_PFI(9), src: sources!(NI_CtrGate(0)) },
        ni_route_set { dest: TRIGGER_LINE(0), src: common_trigger_sources!() },
        ni_route_set { dest: TRIGGER_LINE(1), src: common_trigger_sources!() },
        ni_route_set { dest: TRIGGER_LINE(2), src: common_trigger_sources!() },
        ni_route_set { dest: TRIGGER_LINE(3), src: common_trigger_sources!() },
        ni_route_set { dest: TRIGGER_LINE(4), src: common_trigger_sources!() },
        ni_route_set { dest: TRIGGER_LINE(5), src: common_trigger_sources!() },
        ni_route_set { dest: TRIGGER_LINE(6), src: common_trigger_sources!() },
        ni_route_set { dest: TRIGGER_LINE(7), src: sources!(NI_20MHzTimebase) },
        ni_route_set { dest: NI_CtrSource(0), src: sources!(
            NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5),
            NI_PFI(6), NI_PFI(7), NI_PFI(8), NI_PFI(9), TRIGGER_LINE(0),
            TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3), TRIGGER_LINE(4),
            TRIGGER_LINE(5), TRIGGER_LINE(6), TRIGGER_LINE(7), NI_MasterTimebase,
            NI_20MHzTimebase, NI_100kHzTimebase, NI_AnalogComparisonEvent) },
        ni_route_set { dest: NI_CtrSource(1), src: sources!(
            NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5),
            NI_PFI(6), NI_PFI(7), NI_PFI(8), NI_PFI(9), TRIGGER_LINE(0),
            TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3), TRIGGER_LINE(4),
            TRIGGER_LINE(5), TRIGGER_LINE(6), TRIGGER_LINE(7), NI_MasterTimebase,
            NI_20MHzTimebase, NI_100kHzTimebase, NI_AnalogComparisonEvent) },
        ni_route_set { dest: NI_CtrGate(0), src: sources!(
            NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5),
            NI_PFI(6), NI_PFI(7), NI_PFI(8), NI_PFI(9), TRIGGER_LINE(0),
            TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3), TRIGGER_LINE(4),
            TRIGGER_LINE(5), TRIGGER_LINE(6), TRIGGER_LINE(7), NI_CtrInternalOutput(1),
            NI_AI_StartTrigger, NI_AI_ReferenceTrigger, NI_AnalogComparisonEvent) },
        ni_route_set { dest: NI_CtrGate(1), src: sources!(
            NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5),
            NI_PFI(6), NI_PFI(7), NI_PFI(8), NI_PFI(9), TRIGGER_LINE(0),
            TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3), TRIGGER_LINE(4),
            TRIGGER_LINE(5), TRIGGER_LINE(6), TRIGGER_LINE(7), NI_CtrInternalOutput(0),
            NI_AI_StartTrigger, NI_AI_ReferenceTrigger, NI_AnalogComparisonEvent) },
        ni_route_set { dest: NI_CtrOut(0), src: sources!(
            TRIGGER_LINE(0), TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3),
            TRIGGER_LINE(4), TRIGGER_LINE(5), TRIGGER_LINE(6), NI_CtrInternalOutput(0)) },
        ni_route_set { dest: NI_CtrOut(1), src: sources!(NI_CtrInternalOutput(1)) },
        ni_route_set { dest: NI_AI_SampleClock, src: sources!(
            NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5),
            NI_PFI(6), NI_PFI(7), NI_PFI(8), NI_PFI(9), TRIGGER_LINE(0),
            TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3), TRIGGER_LINE(4),
            TRIGGER_LINE(5), TRIGGER_LINE(6), NI_CtrInternalOutput(0),
            NI_AI_SampleClockTimebase, NI_AnalogComparisonEvent) },
        ni_route_set { dest: NI_AI_SampleClockTimebase, src: sources!(
            NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5),
            NI_PFI(6), NI_PFI(7), NI_PFI(8), NI_PFI(9), TRIGGER_LINE(0),
            TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3), TRIGGER_LINE(4),
            TRIGGER_LINE(5), TRIGGER_LINE(6), TRIGGER_LINE(7), NI_MasterTimebase,
            NI_20MHzTimebase, NI_100kHzTimebase, NI_AnalogComparisonEvent) },
        ni_route_set { dest: NI_AI_StartTrigger, src: sources!(
            NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5),
            NI_PFI(6), NI_PFI(7), NI_PFI(8), NI_PFI(9), TRIGGER_LINE(0),
            TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3), TRIGGER_LINE(4),
            TRIGGER_LINE(5), TRIGGER_LINE(6), NI_CtrInternalOutput(0),
            NI_AnalogComparisonEvent) },
        ni_route_set { dest: NI_AI_ReferenceTrigger, src: sources!(
            NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5),
            NI_PFI(6), NI_PFI(7), NI_PFI(8), NI_PFI(9), TRIGGER_LINE(0),
            TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3), TRIGGER_LINE(4),
            TRIGGER_LINE(5), TRIGGER_LINE(6), NI_AnalogComparisonEvent) },
        ni_route_set { dest: NI_AI_ConvertClock, src: sources!(
            NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5),
            NI_PFI(6), NI_PFI(7), NI_PFI(8), NI_PFI(9), TRIGGER_LINE(0),
            TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3), TRIGGER_LINE(4),
            TRIGGER_LINE(5), TRIGGER_LINE(6), NI_CtrInternalOutput(0),
            NI_AI_ConvertClockTimebase, NI_AnalogComparisonEvent) },
        ni_route_set { dest: NI_AI_ConvertClockTimebase, src: sources!(
            TRIGGER_LINE(7), NI_AI_SampleClockTimebase, NI_MasterTimebase, NI_20MHzTimebase) },
        ni_route_set { dest: NI_AI_PauseTrigger, src: sources!(
            NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5),
            NI_PFI(6), NI_PFI(7), NI_PFI(8), NI_PFI(9), TRIGGER_LINE(0),
            TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3), TRIGGER_LINE(4),
            TRIGGER_LINE(5), TRIGGER_LINE(6), NI_AnalogComparisonEvent) },
        ni_route_set { dest: NI_AI_HoldComplete, src: sources!(NI_AI_HoldCompleteEvent) },
        ni_route_set { dest: NI_AO_SampleClock, src: sources!(
            NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5),
            NI_PFI(6), NI_PFI(7), NI_PFI(8), NI_PFI(9), TRIGGER_LINE(0),
            TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3), TRIGGER_LINE(4),
            TRIGGER_LINE(5), TRIGGER_LINE(6), NI_CtrInternalOutput(1),
            NI_AO_SampleClockTimebase, NI_AnalogComparisonEvent) },
        ni_route_set { dest: NI_AO_SampleClockTimebase, src: sources!(
            NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5),
            NI_PFI(6), NI_PFI(7), NI_PFI(8), NI_PFI(9), TRIGGER_LINE(0),
            TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3), TRIGGER_LINE(4),
            TRIGGER_LINE(5), TRIGGER_LINE(6), TRIGGER_LINE(7), NI_MasterTimebase,
            NI_20MHzTimebase, NI_100kHzTimebase, NI_AnalogComparisonEvent) },
        ni_route_set { dest: NI_AO_StartTrigger, src: sources!(
            NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5),
            NI_PFI(6), NI_PFI(7), NI_PFI(8), NI_PFI(9), TRIGGER_LINE(0),
            TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3), TRIGGER_LINE(4),
            TRIGGER_LINE(5), TRIGGER_LINE(6), NI_AI_StartTrigger,
            NI_AnalogComparisonEvent) },
        ni_route_set { dest: NI_AO_PauseTrigger, src: sources!(
            NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5),
            NI_PFI(6), NI_PFI(7), NI_PFI(8), NI_PFI(9), TRIGGER_LINE(0),
            TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3), TRIGGER_LINE(4),
            TRIGGER_LINE(5), TRIGGER_LINE(6), NI_AnalogComparisonEvent) },
        ni_route_set { dest: NI_MasterTimebase, src: sources!(TRIGGER_LINE(7), NI_20MHzTimebase) },
        ni_route_set { dest: 0, src: &[] },
    ],
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
