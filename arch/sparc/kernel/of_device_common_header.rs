/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the C header. Include dependencies are supplied externally.

#[inline]
unsafe fn of_read_addr(mut cell: *const u32, mut size: i32) -> u64 {
    let mut r: u64 = 0;
    while size > 0 {
        size -= 1;
        r = (r << 32) | (*cell as u64);
        cell = cell.add(1);
    }
    r
}

extern "C" {
    fn of_bus_default_count_cells(
        dev: *mut device_node,
        addrc: *mut i32,
        sizec: *mut i32,
    );
    fn of_out_of_range(
        addr: *const u32,
        base: *const u32,
        size: *const u32,
        na: i32,
        ns: i32,
    ) -> i32;
    fn of_bus_default_map(
        addr: *mut u32,
        range: *const u32,
        na: i32,
        ns: i32,
        pna: i32,
    ) -> i32;
    fn of_bus_default_get_flags(addr: *const u32, flags: usize) -> usize;

    fn of_bus_sbus_match(np: *mut device_node) -> i32;
    fn of_bus_sbus_count_cells(
        child: *mut device_node,
        addrc: *mut i32,
        sizec: *mut i32,
    );
}

/* Max address size we deal with */
const OF_MAX_ADDR_CELLS: i32 = 4;

#[repr(C)]
struct of_bus {
    name: *const i8,
    addr_prop_name: *const i8,
    r#match: unsafe extern "C" fn(parent: *mut device_node) -> i32,
    count_cells: unsafe extern "C" fn(
        child: *mut device_node,
        addrc: *mut i32,
        sizec: *mut i32,
    ),
    map: unsafe extern "C" fn(
        addr: *mut u32,
        range: *const u32,
        na: i32,
        ns: i32,
        pna: i32,
    ) -> i32,
    get_flags: unsafe extern "C" fn(addr: *const u32, flags: usize) -> usize,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
