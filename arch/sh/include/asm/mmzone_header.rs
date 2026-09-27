/* SPDX-License-Identifier: GPL-2.0 */

/* The CONFIG_NUMA conditional is preserved as a Rust feature conditional. */
#[cfg(CONFIG_NUMA)]
pub unsafe fn pfn_to_nid(pfn: ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_int {
    let mut nid: ::kernel::ffi::c_int = 0;

    while nid < MAX_NUMNODES {
        if pfn >= node_start_pfn(nid) && pfn <= node_end_pfn(nid) {
            break;
        }
        nid += 1;
    }

    nid
}

#[cfg(CONFIG_NUMA)]
pub unsafe fn pfn_to_pgdat(
    pfn: ::kernel::ffi::c_ulong,
) -> *mut pglist_data {
    NODE_DATA(pfn_to_nid(pfn))
}

/* arch/sh/mm/numa.c */
#[cfg(CONFIG_NUMA)]
pub unsafe extern "C" fn setup_bootmem_node(
    nid: ::kernel::ffi::c_int,
    start: ::kernel::ffi::c_ulong,
    end: ::kernel::ffi::c_ulong,
);

#[cfg(not(CONFIG_NUMA))]
pub unsafe fn setup_bootmem_node(
    _nid: ::kernel::ffi::c_int,
    _start: ::kernel::ffi::c_ulong,
    _end: ::kernel::ffi::c_ulong,
) {
}

/* Platform specific mem init */
pub unsafe extern "C" fn plat_mem_setup();

/* arch/sh/kernel/setup.c */
pub unsafe extern "C" fn __add_active_range(
    nid: ::kernel::ffi::c_uint,
    start_pfn: ::kernel::ffi::c_ulong,
    end_pfn: ::kernel::ffi::c_ulong,
);

/* arch/sh/mm/init.c */
pub unsafe extern "C" fn allocate_pgdat(nid: ::kernel::ffi::c_uint);


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
