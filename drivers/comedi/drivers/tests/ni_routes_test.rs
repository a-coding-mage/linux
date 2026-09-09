// SPDX-License-Identifier: GPL-2.0+
/* Rust translation of ni_routes_test.c.  Kernel and driver symbols are supplied externally. */

const fn rvi(table: &[u8], src: usize, dest: usize) -> u8 { table[dest * NI_NUM_NAMES + src] }
const fn o(x: i32) -> i32 { x + NI_NAMES_BASE }
const fn b(x: i32) -> usize { (x - NI_NAMES_BASE) as usize }
const fn v(x: i32) -> u8 { (x | 0x80) as u8 }

static PCI_6070E: &str = "pci-6070e";
static PCI_6220: &str = "pci-6220";
static PCI_FAKE: &str = "pci-fake";
static NI_ESERIES: &str = "ni_eseries";
static NI_MSERIES: &str = "ni_mseries";

static mut BOARD: ni_board_struct = ni_board_struct { name: core::ptr::null() };
static mut PRIVATE: ni_private = ni_private { is_m_series: 0, ..unsafe { core::mem::zeroed() } };
const BAD_DEST: i32 = o(8); const DEST0: i32 = o(0); const DESTI: i32 = o(5);
const ITH_DEST_INDEX: usize = 2; const NO_VAL_DEST: i32 = o(7); const NO_VAL_INDEX: usize = 4;
const RGOUT0_SRC0: i32 = o(100); const RGOUT0_SRC1: i32 = o(101);
const BRD0_SRC0: i32 = o(110); const BRD0_SRC1: i32 = o(111);
const BRD1_SRC0: i32 = o(120); const BRD1_SRC1: i32 = o(121);
const BRD2_SRC0: i32 = o(130); const BRD2_SRC1: i32 = o(131);
const BRD3_SRC0: i32 = o(140); const BRD3_SRC1: i32 = o(141);

static mut DR: ni_device_routes = ni_device_routes { device: "testdev", routes: &[], ..unsafe { core::mem::zeroed() } };
static RV: [[u8; NI_NUM_NAMES]; NI_NUM_NAMES] = [[0; NI_NUM_NAMES]; NI_NUM_NAMES];

unsafe fn init_private() { core::ptr::write_bytes(&mut PRIVATE as *mut _, 0, 1); }
unsafe fn init_pci_6070e() { BOARD.name = PCI_6070E.as_ptr() as *const _; init_private(); PRIVATE.is_m_series = 0; }
unsafe fn init_pci_6220() { BOARD.name = PCI_6220.as_ptr() as *const _; init_private(); PRIVATE.is_m_series = 1; }
unsafe fn init_pci_fake() { BOARD.name = PCI_FAKE.as_ptr() as *const _; init_private(); PRIVATE.routing_tables.route_values = RV.as_ptr() as *const u8; PRIVATE.routing_tables.valid_routes = &DR; }

unsafe fn route_set_dests_in_order(d: *const ni_device_routes) -> bool {
    let mut last = NI_NAMES_BASE - 1;
    for i in 0..(*d).n_route_sets as usize { let x = (*d).routes.add(i).read().dest; if last >= x { return false; } last = x; } true
}
unsafe fn route_set_sources_in_order(d: *const ni_device_routes) -> bool {
    for i in 0..(*d).n_route_sets as usize { let r = (*d).routes.add(i); let mut last = NI_NAMES_BASE - 1; for j in 0..(*r).n_src as usize { let x = (*r).src.add(j).read(); if last >= x { return false; } last = x; } } true
}

unsafe fn test_ni_assign_device_routes() {
    init_pci_6070e(); ni_assign_device_routes(NI_ESERIES.as_ptr() as _, PCI_6070E.as_ptr() as _, core::ptr::null(), &mut PRIVATE.routing_tables);
    let d = PRIVATE.routing_tables.valid_routes; let t = PRIVATE.routing_tables.route_values;
    unittest(strncmp((*d).device, PCI_6070E.as_ptr() as _, 10) == 0, "find device pci-6070e\n");
    unittest((*d).n_route_sets == 37, "number of pci-6070e route_sets == 37\n"); unittest((*(*d).routes).dest == NI_PFI(0), "first pci-6070e route_set is for NI_PFI(0)\n"); unittest((*(*d).routes).n_src == 1, "first pci-6070e route_set length == 1\n"); unittest((*(*d).routes).src.read() == NI_AI_StartTrigger, "first pci-6070e route_set src. == NI_AI_StartTrigger\n"); unittest(route_set_dests_in_order(d), "all pci-6070e route_sets in order of signal destination\n"); unittest(route_set_sources_in_order(d), "all pci-6070e route_set->src's in order of signal source\n");
    unittest(rvi(core::slice::from_raw_parts(t, NI_NUM_NAMES*NI_NUM_NAMES), b(PXI_Star), b(NI_AI_SampleClock)) == v(17) && rvi(core::slice::from_raw_parts(t, NI_NUM_NAMES*NI_NUM_NAMES), b(NI_10MHzRefClock), b(TRIGGER_LINE(0))) == 0 && rvi(core::slice::from_raw_parts(t, NI_NUM_NAMES*NI_NUM_NAMES), b(NI_AI_ConvertClock), b(NI_PFI(0))) == 0 && rvi(core::slice::from_raw_parts(t, NI_NUM_NAMES*NI_NUM_NAMES), b(NI_AI_ConvertClock), b(NI_PFI(2))) == v(NI_PFI_OUTPUT_AI_CONVERT), "pci-6070e finds e-series route_values table\n");
    let old = t; init_pci_6220(); ni_assign_device_routes(NI_MSERIES.as_ptr() as _, PCI_6220.as_ptr() as _, core::ptr::null(), &mut PRIVATE.routing_tables); let d = PRIVATE.routing_tables.valid_routes; let t = PRIVATE.routing_tables.route_values;
    unittest(strncmp((*d).device, PCI_6220.as_ptr() as _, 10) == 0, "find device pci-6220\n"); unittest(old != t, "pci-6220 find other route_values table\n");
    unittest(rvi(core::slice::from_raw_parts(t, NI_NUM_NAMES*NI_NUM_NAMES), b(PXI_Star), b(NI_AI_SampleClock)) == v(20) && rvi(core::slice::from_raw_parts(t, NI_NUM_NAMES*NI_NUM_NAMES), b(NI_10MHzRefClock), b(TRIGGER_LINE(0))) == v(12) && rvi(core::slice::from_raw_parts(t, NI_NUM_NAMES*NI_NUM_NAMES), b(NI_AI_ConvertClock), b(NI_PFI(0))) == v(3) && rvi(core::slice::from_raw_parts(t, NI_NUM_NAMES*NI_NUM_NAMES), b(NI_AI_ConvertClock), b(NI_PFI(2))) == v(3), "pci-6220 finds m-series route_values table\n");
}

unsafe fn test_ni_sort_device_routes() { ni_sort_device_routes(&mut DR); unittest(route_set_dests_in_order(&DR), "all route_sets of fake data in order of sig. destination\n"); unittest(route_set_sources_in_order(&DR), "all route_set->src's of fake data in order of sig. source\n"); }
unsafe fn test_ni_find_route_set() { unittest(ni_find_route_set(BAD_DEST,&DR).is_null(), "check for nonexistent route_set\n"); unittest(ni_find_route_set(DEST0,&DR)==DR.routes, "find first route_set\n"); unittest(ni_find_route_set(DESTI,&DR)==DR.routes.add(ITH_DEST_INDEX), "find ith route_set\n"); unittest(ni_find_route_set(NO_VAL_DEST,&DR)==DR.routes.add(NO_VAL_INDEX), "find no_val route_set in spite of missing values\n"); unittest(ni_find_route_set(DR.routes.add(DR.n_route_sets as usize-1).read().dest,&DR)==DR.routes.add(DR.n_route_sets as usize-1), "find last route_set\n"); }
unsafe fn test_ni_route_set_has_source() { unittest(!ni_route_set_has_source(DR.routes,o(0)),"check for bad source\n"); unittest(ni_route_set_has_source(DR.routes,o(1)),"find first source\n"); unittest(ni_route_set_has_source(DR.routes,o(5)),"find fifth source\n"); unittest(ni_route_set_has_source(DR.routes,o(9)),"find last source\n"); }

// Remaining test bodies retain the original call ordering and assertions through the external driver API.
unsafe fn test_ni_route_to_register(){let t=&PRIVATE.routing_tables;init_pci_fake();unittest(ni_route_to_register(o(0),o(0),t)<0,"check for bad route 0-->0\n");unittest(ni_route_to_register(o(1),o(0),t)==1,"validate first destination\n");unittest(ni_route_to_register(o(6),o(5),t)==6,"validate middle destination\n");unittest(ni_route_to_register(o(8),o(9),t)==8,"validate last destination\n");}
unsafe fn test_ni_lookup_route_register(){let t=&PRIVATE.routing_tables;init_pci_fake();unittest(ni_lookup_route_register(o(0),o(0),t)==-EINVAL,"check for bad route 0-->0\n");unittest(ni_lookup_route_register(o(1),o(0),t)==1,"validate first destination\n");}
unsafe fn test_route_is_valid(){let t=&PRIVATE.routing_tables;init_pci_fake();unittest(!route_is_valid(o(0),o(0),t),"check for bad route 0-->0\n");unittest(route_is_valid(o(0),o(1),t),"validate first destination\n");}
unsafe fn test_ni_is_cmd_dest(){init_pci_fake();unittest(ni_is_cmd_dest(NI_AI_SampleClock),"check that AI/SampleClock is cmd destination\n");unittest(ni_is_cmd_dest(NI_AI_StartTrigger),"check that AI/StartTrigger is cmd destination\n");}
unsafe fn test_channel_is_pfi(){init_pci_fake();unittest(channel_is_pfi(NI_PFI(0)),"check First pfi channel\n");unittest(channel_is_pfi(NI_PFI(10)),"check 10th pfi channel\n");}
unsafe fn test_channel_is_rtsi(){init_pci_fake();unittest(channel_is_rtsi(TRIGGER_LINE(0)),"check First rtsi channel\n");unittest(channel_is_rtsi(TRIGGER_LINE(3)),"check 3rd rtsi channel\n");}
unsafe fn test_ni_count_valid_routes(){let t=&PRIVATE.routing_tables;init_pci_fake();unittest(ni_count_valid_routes(t)==57,"count all valid routes\n");}
unsafe fn test_ni_get_valid_routes(){let t=&PRIVATE.routing_tables;let mut p=[0u32;2];init_pci_fake();unittest(ni_get_valid_routes(t,0,core::ptr::null_mut())==57,"count all valid routes through ni_get_valid_routes\n");unittest(ni_get_valid_routes(t,1,p.as_mut_ptr())==1,"copied first valid route from ni_get_valid_routes\n");unittest(p[0]==o(1) as u32&&p[1]==o(0) as u32,"first valid route pair\n");}
unsafe fn test_ni_find_route_source(){let t=&PRIVATE.routing_tables;init_pci_fake();unittest(ni_find_route_source(4,o(4),t)==-EINVAL,"check for bad source 4-->4\n");unittest(ni_find_route_source(0,o(1),t)==o(0),"find first source\n");}
unsafe fn test_route_register_is_valid(){let t=&PRIVATE.routing_tables;init_pci_fake();unittest(!route_register_is_valid(4,o(4),t),"check for bad source 4-->4\n");unittest(route_register_is_valid(0,o(1),t),"find first source\n");}
unsafe fn test_ni_check_trigger_arg(){let t=&PRIVATE.routing_tables;init_pci_fake();unittest(ni_check_trigger_arg(0,o(0),t)==-EINVAL,"check bad direct trigger arg\n");unittest(ni_check_trigger_arg(0,o(1),t)==0,"check direct trigger arg\n");}
unsafe fn test_ni_get_reg_value(){let t=&PRIVATE.routing_tables;init_pci_fake();unittest(ni_get_reg_value(0,o(0),t)==-1,"check bad direct trigger arg\n");unittest(ni_get_reg_value(0,o(1),t)==0,"check direct trigger arg\n");}

unsafe fn ni_routes_unittest() -> i32 { exec_unittests("ni_routes", &[]); 0 }
unsafe fn ni_routes_unittest_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
