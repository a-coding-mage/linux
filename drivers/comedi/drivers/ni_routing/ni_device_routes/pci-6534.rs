// SPDX-License-Identifier: GPL-2.0+
/*
 *  comedi/drivers/ni_routing/ni_device_routes/pci-6534.c
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

use std::ffi::c_char;

// Dependency declarations supplied by the surrounding routing code.
// NI_PFI!(...), TRIGGER_LINE!(...), NI_20MHzTimebase, and
// NI_MasterTimebase are supplied by the surrounding routing code.

#[repr(C)]
pub struct ni_route_set {
    pub dest: i32,
    pub src: *const i32,
}

#[repr(C)]
pub struct ni_device_routes {
    pub device: *const c_char,
    pub routes: *const ni_route_set,
}

#[allow(non_upper_case_globals)]
pub static mut ni_pci_6534_device_routes: ni_device_routes = ni_device_routes {
    device: b"pci-6534\0".as_ptr() as *const c_char,
    routes: ROUTES.as_ptr(),
};

static PFI0_SOURCES: [i32; 8] = [TRIGGER_LINE!(0), TRIGGER_LINE!(1), TRIGGER_LINE!(2), TRIGGER_LINE!(3), TRIGGER_LINE!(4), TRIGGER_LINE!(5), TRIGGER_LINE!(6), 0];
static PFI1_SOURCES: [i32; 8] = PFI0_SOURCES;
static PFI2_SOURCES: [i32; 8] = PFI0_SOURCES;
static PFI3_SOURCES: [i32; 8] = PFI0_SOURCES;
static PFI4_SOURCES: [i32; 8] = PFI0_SOURCES;
static PFI5_SOURCES: [i32; 8] = PFI0_SOURCES;
static PFI6_SOURCES: [i32; 8] = PFI0_SOURCES;
static PFI7_SOURCES: [i32; 8] = PFI0_SOURCES;

macro_rules! trigger_sources {
    ($n:expr, [$($x:expr),* $(,)?]) => {
        [NI_PFI! (0), NI_PFI! (1), NI_PFI! (2), NI_PFI! (3), NI_PFI! (4), NI_PFI! (5), NI_PFI! (6),
         $(TRIGGER_LINE!($x),)* 0]
    };
}

static TRIGGER0_SOURCES: [i32; 14] = trigger_sources!(0, [1, 2, 3, 4, 5, 6]);
static TRIGGER1_SOURCES: [i32; 14] = trigger_sources!(1, [0, 2, 3, 4, 5, 6]);
static TRIGGER2_SOURCES: [i32; 14] = trigger_sources!(2, [0, 1, 3, 4, 5, 6]);
static TRIGGER3_SOURCES: [i32; 14] = trigger_sources!(3, [0, 1, 2, 4, 5, 6]);
static TRIGGER4_SOURCES: [i32; 14] = trigger_sources!(4, [0, 1, 2, 3, 5, 6]);
static TRIGGER5_SOURCES: [i32; 14] = trigger_sources!(5, [0, 1, 2, 3, 4, 6]);
static TRIGGER6_SOURCES: [i32; 14] = trigger_sources!(6, [0, 1, 2, 3, 4, 5]);
static TRIGGER7_SOURCES: [i32; 2] = [NI_20MHzTimebase, 0];
static MASTER_SOURCES: [i32; 3] = [TRIGGER_LINE!(7), NI_20MHzTimebase, 0];

static ROUTES: [ni_route_set; 18] = [
    ni_route_set { dest: NI_PFI!(0), src: PFI0_SOURCES.as_ptr() },
    ni_route_set { dest: NI_PFI!(1), src: PFI1_SOURCES.as_ptr() },
    ni_route_set { dest: NI_PFI!(2), src: PFI2_SOURCES.as_ptr() },
    ni_route_set { dest: NI_PFI!(3), src: PFI3_SOURCES.as_ptr() },
    ni_route_set { dest: NI_PFI!(4), src: PFI4_SOURCES.as_ptr() },
    ni_route_set { dest: NI_PFI!(5), src: PFI5_SOURCES.as_ptr() },
    ni_route_set { dest: NI_PFI!(6), src: PFI6_SOURCES.as_ptr() },
    ni_route_set { dest: NI_PFI!(7), src: PFI7_SOURCES.as_ptr() },
    ni_route_set { dest: TRIGGER_LINE!(0), src: TRIGGER0_SOURCES.as_ptr() },
    ni_route_set { dest: TRIGGER_LINE!(1), src: TRIGGER1_SOURCES.as_ptr() },
    ni_route_set { dest: TRIGGER_LINE!(2), src: TRIGGER2_SOURCES.as_ptr() },
    ni_route_set { dest: TRIGGER_LINE!(3), src: TRIGGER3_SOURCES.as_ptr() },
    ni_route_set { dest: TRIGGER_LINE!(4), src: TRIGGER4_SOURCES.as_ptr() },
    ni_route_set { dest: TRIGGER_LINE!(5), src: TRIGGER5_SOURCES.as_ptr() },
    ni_route_set { dest: TRIGGER_LINE!(6), src: TRIGGER6_SOURCES.as_ptr() },
    ni_route_set { dest: TRIGGER_LINE!(7), src: TRIGGER7_SOURCES.as_ptr() },
    ni_route_set { dest: NI_MasterTimebase, src: MASTER_SOURCES.as_ptr() },
    ni_route_set { dest: 0, src: std::ptr::null() },
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
