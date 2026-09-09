// SPDX-License-Identifier: GPL-2.0+
/*
 *  comedi/drivers/ni_routing/ni_device_routes/pxi-6733.c
 *  List of valid routes for specific NI boards.
 *
 *  The contents of this file are generated using the tools in
 *  comedi/drivers/ni_routing/tools
 */

// The declarations below are supplied by the NI routing headers.

macro_rules! route {
    ($dest:expr, [$($src:expr),* $(,)?]) => {
        ni_route_set {
            dest: $dest,
            src: [$($src,)* 0].as_ptr(),
        }
    };
}

pub static mut ni_pxi_6733_device_routes: ni_device_routes = ni_device_routes {
    device: b"pxi-6733\0".as_ptr() as *const i8,
    routes: [
        route!(NI_PFI(3), [NI_CtrSource(1)]),
        route!(NI_PFI(4), [NI_CtrGate(1)]),
        route!(NI_PFI(5), [NI_AO_SampleClock]),
        route!(NI_PFI(6), [NI_AO_StartTrigger]),
        route!(NI_PFI(8), [NI_CtrSource(0)]),
        route!(NI_PFI(9), [NI_CtrGate(0)]),
        route!(TRIGGER_LINE(0), [NI_CtrSource(0), NI_CtrGate(0), NI_CtrInternalOutput(0), NI_CtrOut(0), NI_AO_SampleClock, NI_AO_StartTrigger]),
        route!(TRIGGER_LINE(1), [NI_CtrSource(0), NI_CtrGate(0), NI_CtrInternalOutput(0), NI_CtrOut(0), NI_AO_SampleClock, NI_AO_StartTrigger]),
        route!(TRIGGER_LINE(2), [NI_CtrSource(0), NI_CtrGate(0), NI_CtrInternalOutput(0), NI_CtrOut(0), NI_AO_SampleClock, NI_AO_StartTrigger]),
        route!(TRIGGER_LINE(3), [NI_CtrSource(0), NI_CtrGate(0), NI_CtrInternalOutput(0), NI_CtrOut(0), NI_AO_SampleClock, NI_AO_StartTrigger]),
        route!(TRIGGER_LINE(4), [NI_CtrSource(0), NI_CtrGate(0), NI_CtrInternalOutput(0), NI_CtrOut(0), NI_AO_SampleClock, NI_AO_StartTrigger]),
        route!(TRIGGER_LINE(5), [NI_CtrSource(0), NI_CtrGate(0), NI_CtrInternalOutput(0), NI_CtrOut(0), NI_AO_SampleClock, NI_AO_StartTrigger]),
        route!(TRIGGER_LINE(7), [NI_20MHzTimebase]),
        route!(NI_CtrSource(0), [NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5), NI_PFI(6), NI_PFI(7), NI_PFI(8), NI_PFI(9), TRIGGER_LINE(0), TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3), TRIGGER_LINE(4), TRIGGER_LINE(5), TRIGGER_LINE(7), PXI_Star, NI_MasterTimebase, NI_20MHzTimebase, NI_100kHzTimebase]),
        route!(NI_CtrSource(1), [NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5), NI_PFI(6), NI_PFI(7), NI_PFI(8), NI_PFI(9), TRIGGER_LINE(0), TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3), TRIGGER_LINE(4), TRIGGER_LINE(5), TRIGGER_LINE(7), PXI_Star, NI_MasterTimebase, NI_20MHzTimebase, NI_100kHzTimebase]),
        route!(NI_CtrGate(0), [NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5), NI_PFI(6), NI_PFI(7), NI_PFI(8), NI_PFI(9), TRIGGER_LINE(0), TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3), TRIGGER_LINE(4), TRIGGER_LINE(5), NI_CtrInternalOutput(1), PXI_Star]),
        route!(NI_CtrGate(1), [NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5), NI_PFI(6), NI_PFI(7), NI_PFI(8), NI_PFI(9), TRIGGER_LINE(0), TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3), TRIGGER_LINE(4), TRIGGER_LINE(5), NI_CtrInternalOutput(0), PXI_Star]),
        route!(NI_CtrOut(0), [TRIGGER_LINE(0), TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3), TRIGGER_LINE(4), TRIGGER_LINE(5), NI_CtrInternalOutput(0), PXI_Star]),
        route!(NI_CtrOut(1), [NI_CtrInternalOutput(1)]),
        route!(PXI_Star, [NI_CtrSource(0), NI_CtrGate(0), NI_CtrInternalOutput(0), NI_CtrOut(0), NI_AO_SampleClock, NI_AO_StartTrigger]),
        route!(NI_AO_SampleClock, [NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5), NI_PFI(6), NI_PFI(7), NI_PFI(8), NI_PFI(9), TRIGGER_LINE(0), TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3), TRIGGER_LINE(4), TRIGGER_LINE(5), NI_CtrInternalOutput(1), PXI_Star, NI_AO_SampleClockTimebase]),
        route!(NI_AO_SampleClockTimebase, [NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5), NI_PFI(6), NI_PFI(7), NI_PFI(8), NI_PFI(9), TRIGGER_LINE(0), TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3), TRIGGER_LINE(4), TRIGGER_LINE(5), TRIGGER_LINE(7), PXI_Star, NI_MasterTimebase, NI_20MHzTimebase, NI_100kHzTimebase]),
        route!(NI_AO_StartTrigger, [NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5), NI_PFI(6), NI_PFI(7), NI_PFI(8), NI_PFI(9), TRIGGER_LINE(0), TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3), TRIGGER_LINE(4), TRIGGER_LINE(5), PXI_Star]),
        route!(NI_AO_PauseTrigger, [NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5), NI_PFI(6), NI_PFI(7), NI_PFI(8), NI_PFI(9), TRIGGER_LINE(0), TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3), TRIGGER_LINE(4), TRIGGER_LINE(5), PXI_Star]),
        route!(NI_DI_SampleClock, [TRIGGER_LINE(0), TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3), TRIGGER_LINE(4), TRIGGER_LINE(5), PXI_Star, NI_AO_SampleClock]),
        route!(NI_DO_SampleClock, [TRIGGER_LINE(0), TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3), TRIGGER_LINE(4), TRIGGER_LINE(5), PXI_Star, NI_AO_SampleClock]),
        route!(NI_MasterTimebase, [TRIGGER_LINE(7), NI_20MHzTimebase]),
        ni_route_set { dest: 0, src: core::ptr::null() },
    ],
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
