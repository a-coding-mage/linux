/* SPDX-License-Identifier: GPL-2.0 */

/* The CONFIG_NUMA conditional is preserved as a Rust feature conditional. */
#[cfg(feature = "CONFIG_NUMA")]
pub unsafe fn pfn_to_nid(pfn: ::core::ffi::c_ulong) -> ::core::ffi::c_int {
    let mut nid: ::core::ffi::c_int = 0;

    while nid < MAX_NUMNODES {
        if pfn >= node_start_pfn(nid) && pfn <= node_end_pfn(nid) {
            break;
        }
        nid += 1;
    }

    nid
}

#[cfg(feature = "CONFIG_NUMA")]
pub unsafe fn pfn_to_pgdat(
    pfn: ::core::ffi::c_ulong,
) -> *mut pglist_data {
    NODE_DATA(pfn_to_nid(pfn))
}

/* arch/sh/mm/numa.c */
#[cfg(feature = "CONFIG_NUMA")]
pub unsafe extern "C" fn setup_bootmem_node(
    nid: ::core::ffi::c_int,
    start: ::core::ffi::c_ulong,
    end: ::core::ffi::c_ulong,
);

#[cfg(not(feature = "CONFIG_NUMA"))]
pub unsafe fn setup_bootmem_node(
    _nid: ::core::ffi::c_int,
    _start: ::core::ffi::c_ulong,
    _end: ::core::ffi::c_ulong,
) {
}

/* Platform specific mem init */
pub unsafe extern "C" fn plat_mem_setup();

/* arch/sh/kernel/setup.c */
pub unsafe extern "C" fn __add_active_range(
    nid: ::core::ffi::c_uint,
    start_pfn: ::core::ffi::c_ulong,
    end_pfn: ::core::ffi::c_ulong,
);

/* arch/sh/mm/init.c */
pub unsafe extern "C" fn allocate_pgdat(nid: ::core::ffi::c_uint);


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
