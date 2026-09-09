// SPDX-License-Identifier: GPL-2.0+
/*
 *  comedi/drivers/ni_routing/ni_device_routes/pxi-6224.rs
 *  List of valid routes for specific NI boards.
 *
 *  This file is generated from the corresponding C routing table.
 */

// The symbols below are supplied by the ni_device_routes and all dependencies.

macro_rules! pfi_sources {
    () => { [
        TRIGGER_LINE(0), TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3),
        TRIGGER_LINE(4), TRIGGER_LINE(5), TRIGGER_LINE(6), TRIGGER_LINE(7),
        NI_CtrSource(0), NI_CtrSource(1), NI_CtrGate(0),
        NI_CtrInternalOutput(0), NI_CtrInternalOutput(1), NI_AI_SampleClock,
        NI_AI_StartTrigger, NI_AI_ReferenceTrigger, NI_AI_ConvertClock,
        NI_DI_SampleClock, NI_DO_SampleClock, NI_FrequencyOutput,
        NI_ChangeDetectionEvent, NI_AnalogComparisonEvent, 0,
    ] };
}

macro_rules! trigger_sources {
    () => { [
        NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5),
        NI_CtrSource(0), NI_CtrSource(1), NI_CtrGate(0),
        NI_CtrInternalOutput(0), NI_CtrInternalOutput(1), NI_AI_SampleClock,
        NI_AI_StartTrigger, NI_AI_ReferenceTrigger, NI_AI_ConvertClock,
        NI_10MHzRefClock, NI_FrequencyOutput, NI_ChangeDetectionEvent,
        NI_AnalogComparisonEvent, 0,
    ] };
}

macro_rules! counter_sources {
    ($($extra:expr),* $(,)?) => { [
        NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5),
        NI_PFI(6), NI_PFI(7), NI_PFI(8), NI_PFI(9), NI_PFI(10), NI_PFI(11),
        NI_PFI(12), NI_PFI(13), NI_PFI(14), NI_PFI(15),
        TRIGGER_LINE(0), TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3),
        TRIGGER_LINE(4), TRIGGER_LINE(5), TRIGGER_LINE(6), TRIGGER_LINE(7),
        $($extra,)* NI_AnalogComparisonEvent, 0,
    ] };
}

macro_rules! route { ($dest:expr, $src:expr) => { ni_route_set { dest: $dest, src: $src.as_ptr() } }; }

macro_rules! pfi_routes {
    () => { [
        route!(NI_PFI(0), pfi_sources!()), route!(NI_PFI(1), pfi_sources!()),
        route!(NI_PFI(2), pfi_sources!()), route!(NI_PFI(3), pfi_sources!()),
        route!(NI_PFI(4), pfi_sources!()), route!(NI_PFI(5), pfi_sources!()),
        route!(NI_PFI(6), pfi_sources!()), route!(NI_PFI(7), pfi_sources!()),
        route!(NI_PFI(8), pfi_sources!()), route!(NI_PFI(9), pfi_sources!()),
        route!(NI_PFI(10), pfi_sources!()), route!(NI_PFI(11), pfi_sources!()),
        route!(NI_PFI(12), pfi_sources!()), route!(NI_PFI(13), pfi_sources!()),
        route!(NI_PFI(14), pfi_sources!()), route!(NI_PFI(15), pfi_sources!()),
    ] };
}

pub static mut ni_pxi_6224_device_routes: ni_device_routes = ni_device_routes {
    device: "pxi-6224",
    routes: &[
        route!(NI_PFI(0), pfi_sources!()), route!(NI_PFI(1), pfi_sources!()),
        route!(NI_PFI(2), pfi_sources!()), route!(NI_PFI(3), pfi_sources!()),
        route!(NI_PFI(4), pfi_sources!()), route!(NI_PFI(5), pfi_sources!()),
        route!(NI_PFI(6), pfi_sources!()), route!(NI_PFI(7), pfi_sources!()),
        route!(NI_PFI(8), pfi_sources!()), route!(NI_PFI(9), pfi_sources!()),
        route!(NI_PFI(10), pfi_sources!()), route!(NI_PFI(11), pfi_sources!()),
        route!(NI_PFI(12), pfi_sources!()), route!(NI_PFI(13), pfi_sources!()),
        route!(NI_PFI(14), pfi_sources!()), route!(NI_PFI(15), pfi_sources!()),
        route!(TRIGGER_LINE(0), trigger_sources!()), route!(TRIGGER_LINE(1), trigger_sources!()),
        route!(TRIGGER_LINE(2), trigger_sources!()), route!(TRIGGER_LINE(3), trigger_sources!()),
        route!(TRIGGER_LINE(4), trigger_sources!()), route!(TRIGGER_LINE(5), trigger_sources!()),
        route!(TRIGGER_LINE(6), trigger_sources!()), route!(TRIGGER_LINE(7), trigger_sources!()),
        route!(NI_CtrSource(0), counter_sources!(PXI_Clk10, NI_20MHzTimebase, NI_80MHzTimebase, NI_100kHzTimebase)),
        route!(NI_CtrSource(1), counter_sources!(NI_CtrGate(0), PXI_Clk10, NI_20MHzTimebase, NI_80MHzTimebase, NI_100kHzTimebase)),
        route!(NI_CtrGate(0), counter_sources!(NI_CtrSource(1), NI_CtrInternalOutput(1), NI_AI_StartTrigger, NI_AI_ReferenceTrigger)),
        route!(NI_CtrGate(1), counter_sources!(NI_CtrSource(0), NI_CtrInternalOutput(0), NI_AI_StartTrigger, NI_AI_ReferenceTrigger)),
        route!(NI_CtrAux(0), counter_sources!(NI_CtrSource(1), NI_CtrGate(0), NI_CtrInternalOutput(1), NI_AI_StartTrigger, NI_AI_ReferenceTrigger)),
        route!(NI_CtrAux(1), counter_sources!(NI_CtrSource(0), NI_CtrGate(0), NI_CtrInternalOutput(0), NI_AI_StartTrigger, NI_AI_ReferenceTrigger)),
        route!(NI_CtrA(0), counter_sources!()), route!(NI_CtrA(1), counter_sources!()),
        route!(NI_CtrB(0), counter_sources!()), route!(NI_CtrB(1), counter_sources!()),
        route!(NI_CtrZ(0), counter_sources!()), route!(NI_CtrZ(1), counter_sources!()),
        route!(NI_CtrArmStartTrigger(0), counter_sources!(NI_CtrInternalOutput(1), NI_AI_StartTrigger, NI_AI_ReferenceTrigger)),
        route!(NI_CtrArmStartTrigger(1), counter_sources!(NI_CtrInternalOutput(0), NI_AI_StartTrigger, NI_AI_ReferenceTrigger)),
        route!(NI_AI_SampleClock, counter_sources!(NI_CtrInternalOutput(0), NI_CtrInternalOutput(1), NI_AI_SampleClockTimebase)),
        route!(NI_AI_SampleClockTimebase, counter_sources!(PXI_Clk10, NI_20MHzTimebase, NI_100kHzTimebase)),
        route!(NI_AI_StartTrigger, counter_sources!(NI_CtrInternalOutput(0), NI_CtrInternalOutput(1))),
        route!(NI_AI_ReferenceTrigger, counter_sources!()),
        route!(NI_AI_ConvertClock, counter_sources!(NI_CtrInternalOutput(0), NI_CtrInternalOutput(1), NI_AI_ConvertClockTimebase)),
        route!(NI_AI_ConvertClockTimebase, [NI_AI_SampleClockTimebase, NI_20MHzTimebase, 0]),
        route!(NI_AI_PauseTrigger, counter_sources!()),
        route!(NI_DI_SampleClock, counter_sources!(NI_CtrInternalOutput(0), NI_CtrInternalOutput(1), NI_AI_SampleClock, NI_AI_ConvertClock, NI_FrequencyOutput, NI_ChangeDetectionEvent)),
        route!(NI_DO_SampleClock, counter_sources!(NI_CtrInternalOutput(0), NI_CtrInternalOutput(1), NI_AI_SampleClock, NI_AI_ConvertClock, NI_FrequencyOutput, NI_ChangeDetectionEvent)),
        ni_route_set { dest: 0, src: core::ptr::null() },
    ],
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
