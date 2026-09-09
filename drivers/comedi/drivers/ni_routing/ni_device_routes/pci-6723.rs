// SPDX-License-Identifier: GPL-2.0+
/*
 *  comedi/drivers/ni_routing/ni_device_routes/pci-6723.c
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

// The contents of this file are generated using the tools in
// comedi/drivers/ni_routing/tools.
// Please use those tools to help maintain the contents of this file.

// Dependencies supplied by the surrounding ni_device_routes module.

pub static mut ni_pci_6723_device_routes: ni_device_routes = ni_device_routes {
    device: "pci-6723",
    routes: &[
        ni_route_set { dest: NI_PFI(3), src: &[NI_CtrSource(1), 0] },
        ni_route_set { dest: NI_PFI(4), src: &[NI_CtrGate(1), 0] },
        ni_route_set { dest: NI_PFI(5), src: &[NI_AO_SampleClock, 0] },
        ni_route_set { dest: NI_PFI(6), src: &[NI_AO_StartTrigger, 0] },
        ni_route_set { dest: NI_PFI(8), src: &[NI_CtrSource(0), 0] },
        ni_route_set { dest: NI_PFI(9), src: &[NI_CtrGate(0), 0] },
        ni_route_set { dest: TRIGGER_LINE(0), src: &[NI_CtrSource(0), NI_CtrGate(0), NI_CtrInternalOutput(0), NI_CtrOut(0), NI_AO_SampleClock, NI_AO_StartTrigger, 0] },
        ni_route_set { dest: TRIGGER_LINE(1), src: &[NI_CtrSource(0), NI_CtrGate(0), NI_CtrInternalOutput(0), NI_CtrOut(0), NI_AO_SampleClock, NI_AO_StartTrigger, 0] },
        ni_route_set { dest: TRIGGER_LINE(2), src: &[NI_CtrSource(0), NI_CtrGate(0), NI_CtrInternalOutput(0), NI_CtrOut(0), NI_AO_SampleClock, NI_AO_StartTrigger, 0] },
        ni_route_set { dest: TRIGGER_LINE(3), src: &[NI_CtrSource(0), NI_CtrGate(0), NI_CtrInternalOutput(0), NI_CtrOut(0), NI_AO_SampleClock, NI_AO_StartTrigger, 0] },
        ni_route_set { dest: TRIGGER_LINE(4), src: &[NI_CtrSource(0), NI_CtrGate(0), NI_CtrInternalOutput(0), NI_CtrOut(0), NI_AO_SampleClock, NI_AO_StartTrigger, 0] },
        ni_route_set { dest: TRIGGER_LINE(5), src: &[NI_CtrSource(0), NI_CtrGate(0), NI_CtrInternalOutput(0), NI_CtrOut(0), NI_AO_SampleClock, NI_AO_StartTrigger, 0] },
        ni_route_set { dest: TRIGGER_LINE(6), src: &[NI_CtrSource(0), NI_CtrGate(0), NI_CtrInternalOutput(0), NI_CtrOut(0), NI_AO_SampleClock, NI_AO_StartTrigger, 0] },
        ni_route_set { dest: TRIGGER_LINE(7), src: &[NI_20MHzTimebase, 0] },
        ni_route_set { dest: NI_CtrSource(0), src: &[NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5), NI_PFI(6), NI_PFI(7), NI_PFI(8), NI_PFI(9), TRIGGER_LINE(0), TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3), TRIGGER_LINE(4), TRIGGER_LINE(5), TRIGGER_LINE(6), TRIGGER_LINE(7), NI_MasterTimebase, NI_20MHzTimebase, NI_100kHzTimebase, 0] },
        ni_route_set { dest: NI_CtrSource(1), src: &[NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5), NI_PFI(6), NI_PFI(7), NI_PFI(8), NI_PFI(9), TRIGGER_LINE(0), TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3), TRIGGER_LINE(4), TRIGGER_LINE(5), TRIGGER_LINE(6), TRIGGER_LINE(7), NI_MasterTimebase, NI_20MHzTimebase, NI_100kHzTimebase, 0] },
        ni_route_set { dest: NI_CtrGate(0), src: &[NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5), NI_PFI(6), NI_PFI(7), NI_PFI(8), NI_PFI(9), TRIGGER_LINE(0), TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3), TRIGGER_LINE(4), TRIGGER_LINE(5), TRIGGER_LINE(6), NI_CtrInternalOutput(1), 0] },
        ni_route_set { dest: NI_CtrGate(1), src: &[NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5), NI_PFI(6), NI_PFI(7), NI_PFI(8), NI_PFI(9), TRIGGER_LINE(0), TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3), TRIGGER_LINE(4), TRIGGER_LINE(5), TRIGGER_LINE(6), NI_CtrInternalOutput(0), 0] },
        ni_route_set { dest: NI_CtrOut(0), src: &[TRIGGER_LINE(0), TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3), TRIGGER_LINE(4), TRIGGER_LINE(5), TRIGGER_LINE(6), NI_CtrInternalOutput(0), 0] },
        ni_route_set { dest: NI_CtrOut(1), src: &[NI_CtrInternalOutput(1), 0] },
        ni_route_set { dest: NI_AO_SampleClock, src: &[NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5), NI_PFI(6), NI_PFI(7), NI_PFI(8), NI_PFI(9), TRIGGER_LINE(0), TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3), TRIGGER_LINE(4), TRIGGER_LINE(5), TRIGGER_LINE(6), NI_CtrInternalOutput(1), NI_AO_SampleClockTimebase, 0] },
        ni_route_set { dest: NI_AO_SampleClockTimebase, src: &[NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5), NI_PFI(6), NI_PFI(7), NI_PFI(8), NI_PFI(9), TRIGGER_LINE(0), TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3), TRIGGER_LINE(4), TRIGGER_LINE(5), TRIGGER_LINE(6), TRIGGER_LINE(7), NI_MasterTimebase, NI_20MHzTimebase, NI_100kHzTimebase, 0] },
        ni_route_set { dest: NI_AO_StartTrigger, src: &[NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5), NI_PFI(6), NI_PFI(7), NI_PFI(8), NI_PFI(9), TRIGGER_LINE(0), TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3), TRIGGER_LINE(4), TRIGGER_LINE(5), TRIGGER_LINE(6), 0] },
        ni_route_set { dest: NI_AO_PauseTrigger, src: &[NI_PFI(0), NI_PFI(1), NI_PFI(2), NI_PFI(3), NI_PFI(4), NI_PFI(5), NI_PFI(6), NI_PFI(7), NI_PFI(8), NI_PFI(9), TRIGGER_LINE(0), TRIGGER_LINE(1), TRIGGER_LINE(2), TRIGGER_LINE(3), TRIGGER_LINE(4), TRIGGER_LINE(5), TRIGGER_LINE(6), 0] },
        ni_route_set { dest: NI_MasterTimebase, src: &[TRIGGER_LINE(7), NI_20MHzTimebase, 0] },
        ni_route_set { dest: 0, src: &[] },
    ],
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
