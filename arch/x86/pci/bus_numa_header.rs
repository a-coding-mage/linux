/* SPDX-License-Identifier: GPL-2.0 */
/*
 * sub bus (transparent) will use entres from 3 to store extra from
 * root, so need to make sure we have enough slot there.
 */

#[repr(C)]
pub struct pci_root_res {
    pub list: list_head,
    pub res: resource,
}

#[repr(C)]
pub struct pci_root_info {
    pub list: list_head,
    pub name: [i8; 12],
    pub resources: list_head,
    pub busn: resource,
    pub node: ::kernel::ffi::c_int,
    pub link: ::kernel::ffi::c_int,
}

extern "C" {
    pub static mut pci_root_infos: list_head;

    pub fn alloc_pci_root_info(
        bus_min: ::kernel::ffi::c_int,
        bus_max: ::kernel::ffi::c_int,
        node: ::kernel::ffi::c_int,
        link: ::kernel::ffi::c_int,
    ) -> *mut pci_root_info;

    pub fn update_res(
        info: *mut pci_root_info,
        start: resource_size_t,
        end: resource_size_t,
        flags: ::kernel::ffi::c_ulong,
        merge: ::kernel::ffi::c_int,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
