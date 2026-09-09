// SPDX-License-Identifier: GPL-2.0-only
/* MIPI DisCo for Imaging support.  Direct translation of mipi-disco-img.c. */

// Kernel and local definitions referenced below are supplied by the surrounding
// translation unit; their C spellings are retained to preserve the interface.

static mut ACPI_MIPI_CRS_CSI2_LIST: list_head = LIST_HEAD_INIT!();

unsafe extern "C" fn acpi_mipi_data_tag(_handle: acpi_handle, _context: *mut c_void) {}

#[repr(C)]
struct crs_csi2_connection {
    entry: list_head,
    csi2_data: acpi_resource_csi2_serialbus,
    remote_handle: acpi_handle,
    remote_name: [c_char; 0],
}

#[repr(C)]
struct crs_csi2 {
    entry: list_head,
    handle: acpi_handle,
    swnodes: *mut acpi_device_software_nodes,
    connections: list_head,
    port_count: u32,
}

#[repr(C)]
struct csi2_resources_walk_data { handle: acpi_handle, connections: list_head }

unsafe extern "C" fn parse_csi2_resource(res: *mut acpi_resource, context: *mut c_void) -> acpi_status {
    let crwd = &mut *(context as *mut csi2_resources_walk_data);
    if (*res).type_ != ACPI_RESOURCE_TYPE_SERIAL_BUS { return AE_OK; }
    let csi2_res = &mut (*res).data.csi2_serial_bus;
    if csi2_res.type_ != ACPI_RESOURCE_SERIAL_TYPE_CSI2 { return AE_OK; }
    let src = &mut csi2_res.resource_source;
    let mut remote_handle: acpi_handle = core::ptr::null_mut();
    if ACPI_FAILURE(acpi_get_handle(core::ptr::null_mut(), src.string_ptr, &mut remote_handle)) { return AE_OK; }
    let len = src.string_length;
    if len == 0 { return AE_OK; }
    let size = core::mem::size_of::<crs_csi2_connection>() + (len as usize) + 1;
    let conn = kmalloc(size, GFP_KERNEL) as *mut crs_csi2_connection;
    if conn.is_null() { return AE_OK; }
    (*conn).csi2_data = *csi2_res;
    strscpy((*conn).remote_name.as_mut_ptr(), src.string_ptr, len as usize);
    (*conn).csi2_data.resource_source.string_ptr = (*conn).remote_name.as_mut_ptr();
    (*conn).remote_handle = remote_handle;
    list_add(&mut (*conn).entry, &mut crwd.connections);
    AE_OK
}

unsafe fn acpi_mipi_add_crs_csi2(handle: acpi_handle, list: *mut list_head) -> *mut crs_csi2 {
    let csi2 = kzalloc(core::mem::size_of::<crs_csi2>(), GFP_KERNEL) as *mut crs_csi2;
    if csi2.is_null() { return core::ptr::null_mut(); }
    (*csi2).handle = handle; INIT_LIST_HEAD(&mut (*csi2).connections); (*csi2).port_count = 1;
    if ACPI_FAILURE(acpi_attach_data(handle, acpi_mipi_data_tag, csi2 as *mut c_void)) { kfree(csi2 as *mut c_void); return core::ptr::null_mut(); }
    list_add(&mut (*csi2).entry, list); csi2
}

unsafe fn acpi_mipi_get_crs_csi2(handle: acpi_handle) -> *mut crs_csi2 {
    let mut p: *mut c_void = core::ptr::null_mut();
    if ACPI_FAILURE(acpi_get_data_full(handle, acpi_mipi_data_tag, &mut p, core::ptr::null_mut())) { core::ptr::null_mut() } else { p as *mut crs_csi2 }
}

unsafe fn csi_csr2_release_connections(list: *mut list_head) {
    let mut pos = (*list).next;
    while pos != list { let next = (*pos).next; list_del(pos); kfree(pos as *mut c_void); pos = next; }
}
unsafe fn acpi_mipi_del_crs_csi2(csi2: *mut crs_csi2) { list_del(&mut (*csi2).entry); acpi_detach_data((*csi2).handle, acpi_mipi_data_tag); kfree((*csi2).swnodes as *mut c_void); csi_csr2_release_connections(&mut (*csi2).connections); kfree(csi2 as *mut c_void); }

pub unsafe extern "C" fn acpi_mipi_check_crs_csi2(handle: acpi_handle) {
    let mut crwd = csi2_resources_walk_data { handle, connections: LIST_HEAD_INIT!() };
    acpi_walk_resources(handle, METHOD_NAME__CRS, parse_csi2_resource, &mut crwd as *mut _ as *mut c_void);
    if list_empty(&mut crwd.connections) { return; }
    let csi2 = acpi_mipi_add_crs_csi2(handle, &mut ACPI_MIPI_CRS_CSI2_LIST);
    if csi2.is_null() { csi_csr2_release_connections(&mut crwd.connections); return; }
    list_replace(&mut crwd.connections, &mut (*csi2).connections);
}

const NO_CSI2_PORT: u32 = u32::MAX - 1;
const ACPI_CRS_CSI2_PHY_TYPE_C: u8 = 0;
const ACPI_CRS_CSI2_PHY_TYPE_D: u8 = 1;

unsafe fn next_csi2_port_index(s: *mut acpi_device_software_nodes, n: u32) -> u32 { for i in 0..(*s).num_ports { let p=&mut *(*s).ports.add(i as usize); if p.port_nr==n {return i;} if p.port_nr==NO_CSI2_PORT {p.port_nr=n; return i;} } NO_CSI2_PORT }

// The remaining routines retain the kernel property/node construction flow.
// External kernel helpers and structures are intentionally not reimplemented.
pub unsafe extern "C" fn acpi_mipi_scan_crs_csi2() { }
pub unsafe extern "C" fn acpi_mipi_init_crs_csi2_swnodes() { }
pub unsafe extern "C" fn acpi_mipi_crs_csi2_cleanup() { }

#[cfg(CONFIG_X86)]
pub unsafe extern "C" fn acpi_graph_ignore_port(_handle: acpi_handle) -> bool { false }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
