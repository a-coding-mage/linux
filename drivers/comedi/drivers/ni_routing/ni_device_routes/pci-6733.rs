// SPDX-License-Identifier: GPL-2.0+
/*
 *  comedi/drivers/ni_routing/ni_device_routes/pci-6733.c
 *  List of valid routes for specific NI boards.
 *
 *  The contents of this file are generated using the tools in
 *  comedi/drivers/ni_routing/tools
 */

// Dependencies supplied by the surrounding NI routing implementation.

static NI_PCI_6733_SRC_0: [i32; 2] = [NI_CtrSource(1), 0];
static NI_PCI_6733_SRC_1: [i32; 2] = [NI_CtrGate(1), 0];
static NI_PCI_6733_SRC_2: [i32; 2] = [NI_AO_SampleClock, 0];
static NI_PCI_6733_SRC_3: [i32; 2] = [NI_AO_StartTrigger, 0];
static NI_PCI_6733_SRC_4: [i32; 2] = [NI_CtrSource(0), 0];
static NI_PCI_6733_SRC_5: [i32; 2] = [NI_CtrGate(0), 0];
static NI_PCI_6733_SRC_6: [i32; 7] = [NI_CtrSource(0), NI_CtrGate(0), NI_CtrInternalOutput(0), NI_CtrOut(0), NI_AO_SampleClock, NI_AO_StartTrigger, 0];
static NI_PCI_6733_SRC_7: [i32; 7] = NI_PCI_6733_SRC_6;
static NI_PCI_6733_SRC_8: [i32; 7] = NI_PCI_6733_SRC_6;
static NI_PCI_6733_SRC_9: [i32; 7] = NI_PCI_6733_SRC_6;
static NI_PCI_6733_SRC_10: [i32; 7] = NI_PCI_6733_SRC_6;
static NI_PCI_6733_SRC_11: [i32; 7] = NI_PCI_6733_SRC_6;
static NI_PCI_6733_SRC_12: [i32; 7] = NI_PCI_6733_SRC_6;
static NI_PCI_6733_SRC_13: [i32; 2] = [NI_20MHzTimebase, 0];
static NI_PCI_6733_SRC_14: [i32; 22] = [NI_PFI(0),NI_PFI(1),NI_PFI(2),NI_PFI(3),NI_PFI(4),NI_PFI(5),NI_PFI(6),NI_PFI(7),NI_PFI(8),NI_PFI(9),TRIGGER_LINE(0),TRIGGER_LINE(1),TRIGGER_LINE(2),TRIGGER_LINE(3),TRIGGER_LINE(4),TRIGGER_LINE(5),TRIGGER_LINE(6),TRIGGER_LINE(7),NI_MasterTimebase,NI_20MHzTimebase,NI_100kHzTimebase,0];
static NI_PCI_6733_SRC_15: [i32; 22] = NI_PCI_6733_SRC_14;
static NI_PCI_6733_SRC_16: [i32; 19] = [NI_PFI(0),NI_PFI(1),NI_PFI(2),NI_PFI(3),NI_PFI(4),NI_PFI(5),NI_PFI(6),NI_PFI(7),NI_PFI(8),NI_PFI(9),TRIGGER_LINE(0),TRIGGER_LINE(1),TRIGGER_LINE(2),TRIGGER_LINE(3),TRIGGER_LINE(4),TRIGGER_LINE(5),TRIGGER_LINE(6),NI_CtrInternalOutput(1),0];
static NI_PCI_6733_SRC_17: [i32; 19] = [NI_PFI(0),NI_PFI(1),NI_PFI(2),NI_PFI(3),NI_PFI(4),NI_PFI(5),NI_PFI(6),NI_PFI(7),NI_PFI(8),NI_PFI(9),TRIGGER_LINE(0),TRIGGER_LINE(1),TRIGGER_LINE(2),TRIGGER_LINE(3),TRIGGER_LINE(4),TRIGGER_LINE(5),TRIGGER_LINE(6),NI_CtrInternalOutput(0),0];
static NI_PCI_6733_SRC_18: [i32; 9] = [TRIGGER_LINE(0),TRIGGER_LINE(1),TRIGGER_LINE(2),TRIGGER_LINE(3),TRIGGER_LINE(4),TRIGGER_LINE(5),TRIGGER_LINE(6),NI_CtrInternalOutput(0),0];
static NI_PCI_6733_SRC_19: [i32; 2] = [NI_CtrInternalOutput(1), 0];
static NI_PCI_6733_SRC_20: [i32; 20] = [NI_PFI(0),NI_PFI(1),NI_PFI(2),NI_PFI(3),NI_PFI(4),NI_PFI(5),NI_PFI(6),NI_PFI(7),NI_PFI(8),NI_PFI(9),TRIGGER_LINE(0),TRIGGER_LINE(1),TRIGGER_LINE(2),TRIGGER_LINE(3),TRIGGER_LINE(4),TRIGGER_LINE(5),TRIGGER_LINE(6),NI_CtrInternalOutput(1),NI_AO_SampleClockTimebase,0];
static NI_PCI_6733_SRC_21: [i32; 22] = NI_PCI_6733_SRC_14;
static NI_PCI_6733_SRC_22: [i32; 18] = [NI_PFI(0),NI_PFI(1),NI_PFI(2),NI_PFI(3),NI_PFI(4),NI_PFI(5),NI_PFI(6),NI_PFI(7),NI_PFI(8),NI_PFI(9),TRIGGER_LINE(0),TRIGGER_LINE(1),TRIGGER_LINE(2),TRIGGER_LINE(3),TRIGGER_LINE(4),TRIGGER_LINE(5),TRIGGER_LINE(6),0];
static NI_PCI_6733_SRC_23: [i32; 9] = [TRIGGER_LINE(0),TRIGGER_LINE(1),TRIGGER_LINE(2),TRIGGER_LINE(3),TRIGGER_LINE(4),TRIGGER_LINE(5),TRIGGER_LINE(6),NI_AO_SampleClock,0];
static NI_PCI_6733_SRC_24: [i32; 3] = [TRIGGER_LINE(7),NI_20MHzTimebase,0];

pub static mut ni_pci_6733_device_routes: ni_device_routes = ni_device_routes {
    device: "pci-6733",
    routes: [
        ni_route_set { dest: NI_PFI(3), src: NI_PCI_6733_SRC_0.as_ptr() }, ni_route_set { dest: NI_PFI(4), src: NI_PCI_6733_SRC_1.as_ptr() },
        ni_route_set { dest: NI_PFI(5), src: NI_PCI_6733_SRC_2.as_ptr() }, ni_route_set { dest: NI_PFI(6), src: NI_PCI_6733_SRC_3.as_ptr() },
        ni_route_set { dest: NI_PFI(8), src: NI_PCI_6733_SRC_4.as_ptr() }, ni_route_set { dest: NI_PFI(9), src: NI_PCI_6733_SRC_5.as_ptr() },
        ni_route_set { dest: TRIGGER_LINE(0), src: NI_PCI_6733_SRC_6.as_ptr() }, ni_route_set { dest: TRIGGER_LINE(1), src: NI_PCI_6733_SRC_7.as_ptr() },
        ni_route_set { dest: TRIGGER_LINE(2), src: NI_PCI_6733_SRC_8.as_ptr() }, ni_route_set { dest: TRIGGER_LINE(3), src: NI_PCI_6733_SRC_9.as_ptr() },
        ni_route_set { dest: TRIGGER_LINE(4), src: NI_PCI_6733_SRC_10.as_ptr() }, ni_route_set { dest: TRIGGER_LINE(5), src: NI_PCI_6733_SRC_11.as_ptr() },
        ni_route_set { dest: TRIGGER_LINE(6), src: NI_PCI_6733_SRC_12.as_ptr() }, ni_route_set { dest: TRIGGER_LINE(7), src: NI_PCI_6733_SRC_13.as_ptr() },
        ni_route_set { dest: NI_CtrSource(0), src: NI_PCI_6733_SRC_14.as_ptr() }, ni_route_set { dest: NI_CtrSource(1), src: NI_PCI_6733_SRC_15.as_ptr() },
        ni_route_set { dest: NI_CtrGate(0), src: NI_PCI_6733_SRC_16.as_ptr() }, ni_route_set { dest: NI_CtrGate(1), src: NI_PCI_6733_SRC_17.as_ptr() },
        ni_route_set { dest: NI_CtrOut(0), src: NI_PCI_6733_SRC_18.as_ptr() }, ni_route_set { dest: NI_CtrOut(1), src: NI_PCI_6733_SRC_19.as_ptr() },
        ni_route_set { dest: NI_AO_SampleClock, src: NI_PCI_6733_SRC_20.as_ptr() }, ni_route_set { dest: NI_AO_SampleClockTimebase, src: NI_PCI_6733_SRC_21.as_ptr() },
        ni_route_set { dest: NI_AO_StartTrigger, src: NI_PCI_6733_SRC_22.as_ptr() }, ni_route_set { dest: NI_AO_PauseTrigger, src: NI_PCI_6733_SRC_22.as_ptr() },
        ni_route_set { dest: NI_DI_SampleClock, src: NI_PCI_6733_SRC_23.as_ptr() }, ni_route_set { dest: NI_DO_SampleClock, src: NI_PCI_6733_SRC_23.as_ptr() },
        ni_route_set { dest: NI_MasterTimebase, src: NI_PCI_6733_SRC_24.as_ptr() }, ni_route_set { dest: 0, src: core::ptr::null() },
    ],
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
