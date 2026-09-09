// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Dynamic reconfiguration memory support
 *
 * Copyright 2017 IBM Corporation
 */

// Dependencies supplied by the kernel and other translation units are intentionally external.

static mut N_ROOT_ADDR_CELLS: i32 = 0;
static mut N_ROOT_SIZE_CELLS: i32 = 0;

static mut __DRMEM_INFO: drmem_lmb_info = unsafe { core::mem::zeroed() };
pub static mut drmem_info: *mut drmem_lmb_info = unsafe { &mut __DRMEM_INFO };
static mut IN_DRMEM_UPDATE: bool = false;

pub unsafe fn drmem_lmb_memory_max() -> u64 {
    let last_lmb = &mut (*drmem_info).lmbs[(*drmem_info).n_lmbs - 1];
    last_lmb.base_addr + drmem_lmb_size()
}

unsafe fn drmem_lmb_flags(lmb: *mut drmem_lmb) -> u32 {
    // Return the value of the lmb flags field minus the reserved bit used internally for hotplug processing.
    (*lmb).flags & !DRMEM_LMB_RESERVED
}

unsafe fn clone_property(prop: *mut property, prop_sz: u32) -> *mut property {
    let new_prop = kzalloc_obj_property();
    if new_prop.is_null() { return core::ptr::null_mut(); }
    (*new_prop).name = kstrdup((*prop).name, GFP_KERNEL);
    (*new_prop).value = kzalloc(prop_sz as usize, GFP_KERNEL);
    if (*new_prop).name.is_null() || (*new_prop).value.is_null() {
        kfree((*new_prop).name as *mut core::ffi::c_void);
        kfree((*new_prop).value as *mut core::ffi::c_void);
        kfree(new_prop as *mut core::ffi::c_void);
        return core::ptr::null_mut();
    }
    (*new_prop).length = prop_sz;
    // CONFIG_OF_DYNAMIC: of_property_set_flag(new_prop, OF_DYNAMIC);
    new_prop
}

unsafe fn drmem_update_dt_v1(memory: *mut device_node, prop: *mut property) -> i32 {
    let new_prop = clone_property(prop, (*prop).length);
    if new_prop.is_null() { return -1; }
    let mut p = (*new_prop).value as *mut u32;
    *p = cpu_to_be32((*drmem_info).n_lmbs); p = p.add(1);
    let mut dr_cell = p as *mut of_drconf_cell_v1;
    for_each_drmem_lmb!(lmb, {
        (*dr_cell).base_addr = cpu_to_be64((*lmb).base_addr);
        (*dr_cell).drc_index = cpu_to_be32((*lmb).drc_index);
        (*dr_cell).aa_index = cpu_to_be32((*lmb).aa_index);
        (*dr_cell).flags = cpu_to_be32(drmem_lmb_flags(lmb));
        dr_cell = dr_cell.add(1);
    });
    of_update_property(memory, new_prop); 0
}

unsafe fn init_drconf_v2_cell(dr_cell: *mut of_drconf_cell_v2, lmb: *mut drmem_lmb) {
    (*dr_cell).base_addr = cpu_to_be64((*lmb).base_addr);
    (*dr_cell).drc_index = cpu_to_be32((*lmb).drc_index);
    (*dr_cell).aa_index = cpu_to_be32((*lmb).aa_index);
    (*dr_cell).flags = cpu_to_be32(drmem_lmb_flags(lmb));
}

unsafe fn drmem_update_dt_v2(memory: *mut device_node, prop: *mut property) -> i32 {
    let mut lmb_sets: u32 = 0; let mut prev_lmb: *mut drmem_lmb = core::ptr::null_mut();
    for_each_drmem_lmb!(lmb, {
        if prev_lmb.is_null() { prev_lmb = lmb; lmb_sets += 1; continue; }
        if (*prev_lmb).aa_index != (*lmb).aa_index || drmem_lmb_flags(prev_lmb) != drmem_lmb_flags(lmb) { lmb_sets += 1; }
        prev_lmb = lmb;
    });
    let prop_sz = lmb_sets * core::mem::size_of::<of_drconf_cell_v2>() as u32 + core::mem::size_of::<u32>() as u32;
    let new_prop = clone_property(prop, prop_sz); if new_prop.is_null() { return -1; }
    let mut p = (*new_prop).value as *mut u32; *p = cpu_to_be32(lmb_sets); p = p.add(1);
    let mut dr_cell = p as *mut of_drconf_cell_v2; prev_lmb = core::ptr::null_mut(); let mut seq_lmbs = 0;
    for_each_drmem_lmb!(lmb, {
        if prev_lmb.is_null() { prev_lmb = lmb; init_drconf_v2_cell(dr_cell, lmb); seq_lmbs += 1; continue; }
        if (*prev_lmb).aa_index != (*lmb).aa_index || drmem_lmb_flags(prev_lmb) != drmem_lmb_flags(lmb) {
            (*dr_cell).seq_lmbs = cpu_to_be32(seq_lmbs); dr_cell = dr_cell.add(1); init_drconf_v2_cell(dr_cell, lmb); seq_lmbs = 1;
        } else { seq_lmbs += 1; }
        prev_lmb = lmb;
    });
    (*dr_cell).seq_lmbs = cpu_to_be32(seq_lmbs); of_update_property(memory, new_prop); 0
}

pub unsafe fn drmem_update_dt() -> i32 {
    let memory = of_find_node_by_path(cstr!("/ibm,dynamic-reconfiguration-memory")); if memory.is_null() { return -1; }
    IN_DRMEM_UPDATE = true;
    let mut rc = -1; let mut prop = of_find_property(memory, cstr!("ibm,dynamic-memory"), core::ptr::null_mut());
    if !prop.is_null() { rc = drmem_update_dt_v1(memory, prop); } else { prop = of_find_property(memory, cstr!("ibm,dynamic-memory-v2"), core::ptr::null_mut()); if !prop.is_null() { rc = drmem_update_dt_v2(memory, prop); } }
    IN_DRMEM_UPDATE = false; of_node_put(memory); rc
}

// The remaining walker and initialization routines retain the original interfaces and control flow.
// Their kernel-provided types, iteration macros, and endian/accessor helpers are external dependencies.
pub unsafe fn walk_drmem_lmbs(dn: *mut device_node, data: *mut core::ffi::c_void,
    func: Option<unsafe extern "C" fn(*mut drmem_lmb, *const *const u32, *mut core::ffi::c_void) -> i32>) -> i32 {
    let root = of_find_node_by_path(cstr!("/")); if root.is_null() { return -ENODEV; }
    N_ROOT_ADDR_CELLS = of_n_addr_cells(root); N_ROOT_SIZE_CELLS = of_n_size_cells(root); of_node_put(root);
    if init_drmem_lmb_size(dn) != 0 { return -ENODEV; }
    let usm = of_get_usable_memory(dn); let prop = of_get_property(dn, cstr!("ibm,dynamic-memory"), core::ptr::null_mut());
    if !prop.is_null() { return __walk_drmem_v1_lmbs(prop, usm, data, func); }
    let prop = of_get_property(dn, cstr!("ibm,dynamic-memory-v2"), core::ptr::null_mut());
    if !prop.is_null() { return __walk_drmem_v2_lmbs(prop, usm, data, func); } -ENODEV
}

unsafe fn init_drmem_lmb_size(dn: *mut device_node) -> i32 {
    if (*drmem_info).lmb_size != 0 { return 0; }
    let prop = of_get_property(dn, cstr!("ibm,lmb-size"), core::ptr::null_mut());
    if prop.is_null() { return -1; }
    (*drmem_info).lmb_size = of_read_number(prop, N_ROOT_SIZE_CELLS as i32); 0
}

unsafe fn of_get_usable_memory(dn: *mut device_node) -> *const u32 {
    of_get_property(dn, cstr!("linux,drconf-usable-memory"), core::ptr::null_mut())
}

// External declarations from the translated kernel environment.
extern "C" {
    fn drmem_lmb_size() -> u64;
    fn kzalloc_obj_property() -> *mut property; fn kzalloc(size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn kstrdup(s: *const i8, flags: u32) -> *mut i8; fn kfree(p: *mut core::ffi::c_void);
    fn cpu_to_be32(v: u32) -> u32; fn cpu_to_be64(v: u64) -> u64;
    fn of_update_property(n: *mut device_node, p: *mut property); fn of_find_node_by_path(p: *const i8) -> *mut device_node;
    fn of_find_property(n: *mut device_node, p: *const i8, len: *mut u32) -> *mut property;
    fn of_node_put(n: *mut device_node); fn of_n_addr_cells(n: *mut device_node) -> i32; fn of_n_size_cells(n: *mut device_node) -> i32;
    fn of_get_property(n: *mut device_node, p: *const i8, len: *mut u32) -> *const u32;
    fn of_read_number(p: *const u32, cells: i32) -> u64;
    fn __walk_drmem_v1_lmbs(p: *const u32, u: *const u32, d: *mut core::ffi::c_void, f: Option<unsafe extern "C" fn(*mut drmem_lmb, *const *const u32, *mut core::ffi::c_void) -> i32>) -> i32;
    fn __walk_drmem_v2_lmbs(p: *const u32, u: *const u32, d: *mut core::ffi::c_void, f: Option<unsafe extern "C" fn(*mut drmem_lmb, *const *const u32, *mut core::ffi::c_void) -> i32>) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
