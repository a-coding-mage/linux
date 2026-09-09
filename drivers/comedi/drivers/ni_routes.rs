// SPDX-License-Identifier: GPL-2.0+
/*
 *  comedi/drivers/ni_routes.c
 *  Route information for NI boards.
 *
 *  COMEDI - Linux Control and Measurement Device Interface
 *  Copyright (C) 2016 Spencer E. Olson <olsonse@umich.edu>
 */

// Kernel and local C headers supply the types, constants, globals, and helper
// functions referenced below.

/* RVi(table, src, dest) == table[dest * NI_NUM_NAMES + src] */

unsafe fn ni_find_route_values(device_family: *const c_char) -> *const u8 {
    let mut rv: *const u8 = core::ptr::null();
    let mut i = 0;
    while !ni_all_route_values[i].is_null() {
        if strcmp((*ni_all_route_values[i]).family, device_family) == 0 {
            rv = &(*ni_all_route_values[i]).register_values[0][0] as *const u8;
            break;
        }
        i += 1;
    }
    rv
}

unsafe fn ni_find_valid_routes(board_name: *const c_char) -> *const ni_device_routes {
    let mut dr: *const ni_device_routes = core::ptr::null();
    let mut i = 0;
    while !ni_device_routes_list[i].is_null() {
        if strcmp((*ni_device_routes_list[i]).device, board_name) == 0 {
            dr = ni_device_routes_list[i];
            break;
        }
        i += 1;
    }
    dr
}

unsafe fn ni_find_device_routes(
    device_family: *const c_char,
    board_name: *const c_char,
    alt_board_name: *const c_char,
    tables: *mut ni_route_tables,
) -> c_int {
    let rv = ni_find_route_values(device_family);
    let mut dr = ni_find_valid_routes(board_name);
    if dr.is_null() && !alt_board_name.is_null() {
        dr = ni_find_valid_routes(alt_board_name);
    }
    (*tables).route_values = rv;
    (*tables).valid_routes = dr;
    if rv.is_null() || dr.is_null() { -ENODATA } else { 0 }
}

pub unsafe fn ni_assign_device_routes(
    device_family: *const c_char,
    board_name: *const c_char,
    alt_board_name: *const c_char,
    tables: *mut ni_route_tables,
) -> c_int {
    core::ptr::write_bytes(tables, 0, 1);
    ni_find_device_routes(device_family, board_name, alt_board_name, tables)
}

pub unsafe fn ni_count_valid_routes(tables: *const ni_route_tables) -> c_uint {
    let mut total = 0;
    for i in 0..(*(*tables).valid_routes).n_route_sets {
        let r = &(*(*tables).valid_routes).routes[i as usize];
        for j in 0..r.n_src {
            let src = r.src[j as usize];
            let dest = r.dest;
            let rv = (*tables).route_values;
            let direct = *rv.add((B(dest) * NI_NUM_NAMES + B(src)) as usize);
            if direct != 0 {
                total += 1;
            } else if channel_is_rtsi(dest)
                && (*rv.add((B(NI_RGOUT0) * NI_NUM_NAMES + B(src)) as usize) != 0
                    || *rv.add((B(NI_RTSI_BRD(0)) * NI_NUM_NAMES + B(src)) as usize) != 0
                    || *rv.add((B(NI_RTSI_BRD(1)) * NI_NUM_NAMES + B(src)) as usize) != 0
                    || *rv.add((B(NI_RTSI_BRD(2)) * NI_NUM_NAMES + B(src)) as usize) != 0
                    || *rv.add((B(NI_RTSI_BRD(3)) * NI_NUM_NAMES + B(src)) as usize) != 0)
            {
                total += 1;
            }
        }
    }
    total as c_uint
}

pub unsafe fn ni_get_valid_routes(
    tables: *const ni_route_tables,
    n_pairs: c_uint,
    pair_data: *mut c_uint,
) -> c_uint {
    let available = ni_count_valid_routes(tables);
    if n_pairs == 0 || available == 0 { return available; }
    if pair_data.is_null() { return 0; }
    let mut n_valid = 0;
    for i in 0..(*(*tables).valid_routes).n_route_sets {
        let r = &(*(*tables).valid_routes).routes[i as usize];
        for j in 0..r.n_src {
            let src = r.src[j as usize];
            let dest = r.dest;
            let rv = (*tables).route_values;
            let mut valid = *rv.add((B(dest) * NI_NUM_NAMES + B(src)) as usize) != 0;
            if !valid && channel_is_rtsi(dest) {
                valid = *rv.add((B(NI_RGOUT0) * NI_NUM_NAMES + B(src)) as usize) != 0
                    || (0..4).any(|k| *rv.add((B(NI_RTSI_BRD(k)) * NI_NUM_NAMES + B(src)) as usize) != 0);
            }
            if valid {
                *pair_data.add((2 * n_valid) as usize) = src as c_uint;
                *pair_data.add((2 * n_valid + 1) as usize) = dest as c_uint;
                n_valid += 1;
            }
            if n_valid >= n_pairs { return n_valid; }
        }
    }
    n_valid
}

static NI_CMD_DESTS: [c_int; 7] = [NI_AI_SampleClock, NI_AI_StartTrigger, NI_AI_ConvertClock,
    NI_AO_SampleClock, NI_AO_StartTrigger, NI_DI_SampleClock, NI_DO_SampleClock];

pub fn ni_is_cmd_dest(dest: c_int) -> bool {
    NI_CMD_DESTS.iter().any(|&x| x == dest)
}

pub unsafe fn ni_sort_device_routes(valid_routes: *mut ni_device_routes) {
    (*valid_routes).n_route_sets = 0;
    while (*valid_routes).routes[(*valid_routes).n_route_sets as usize].dest != 0 {
        (*valid_routes).n_route_sets += 1;
    }
    sort_route_sets((*valid_routes).routes.as_mut_ptr(), (*valid_routes).n_route_sets as usize);
    for n in 0..(*valid_routes).n_route_sets as usize {
        let rs = &mut (*valid_routes).routes[n];
        rs.n_src = 0;
        while rs.src[rs.n_src as usize] != 0 { rs.n_src += 1; }
        sort_ints(rs.src.as_mut_ptr(), rs.n_src as usize);
    }
}

unsafe fn ni_sort_all_device_routes() {
    let mut i = 0;
    while !ni_device_routes_list[i].is_null() {
        ni_sort_device_routes(ni_device_routes_list[i] as *mut ni_device_routes);
        i += 1;
    }
}

pub unsafe fn ni_find_route_set(destination: c_int, valid_routes: *const ni_device_routes) -> *const ni_route_set {
    bsearch_route_set(destination, (*valid_routes).routes.as_ptr(), (*valid_routes).n_route_sets as usize)
}

pub unsafe fn ni_route_set_has_source(routes: *const ni_route_set, source: c_int) -> bool {
    bsearch_int(source, (*routes).src.as_ptr(), (*routes).n_src as usize).is_some()
}

pub unsafe fn ni_lookup_route_register(mut src: c_int, mut dest: c_int, tables: *const ni_route_tables) -> i8 {
    src = B(src); dest = B(dest);
    if src < 0 || src >= NI_NUM_NAMES || dest < 0 || dest >= NI_NUM_NAMES { return -EINVAL as i8; }
    let v = *(*tables).route_values.add((dest * NI_NUM_NAMES + src) as usize);
    if v == 0 { -EINVAL as i8 } else { UNMARK(v) as i8 }
}

pub unsafe fn ni_route_to_register(src: c_int, dest: c_int, tables: *const ni_route_tables) -> i8 {
    let routes = ni_find_route_set(dest, (*tables).valid_routes);
    if routes.is_null() || !ni_route_set_has_source(routes, src) { return -1; }
    let rv = (*tables).route_values;
    let mut regval = *rv.add((B(dest) * NI_NUM_NAMES + B(src)) as usize);
    if regval == 0 && channel_is_rtsi(dest) {
        regval = *rv.add((B(NI_RGOUT0) * NI_NUM_NAMES + B(src)) as usize);
        if regval == 0 && (0..4).any(|k| *rv.add((B(NI_RTSI_BRD(k)) * NI_NUM_NAMES + B(src)) as usize) != 0) { regval = BIT(6); }
    }
    if regval == 0 { -1 } else { UNMARK(regval) as i8 }
}

pub unsafe fn ni_find_route_source(src_sel_reg_value: u8, mut dest: c_int, tables: *const ni_route_tables) -> c_int {
    if (*tables).route_values.is_null() { return -EINVAL; }
    dest = B(dest);
    if dest < 0 || dest >= NI_NUM_NAMES { return -EINVAL; }
    for src in 0..NI_NUM_NAMES {
        if *(*tables).route_values.add((dest * NI_NUM_NAMES + src) as usize) == V(src_sel_reg_value) { return src + NI_NAMES_BASE; }
    }
    -EINVAL
}

unsafe fn ni_routes_module_init() -> c_int { ni_sort_all_device_routes(); 0 }
unsafe fn ni_routes_module_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
