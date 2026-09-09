// SPDX-License-Identifier: GPL-2.0
/*
 * Virtual I/O topology
 *
 * The Virtual I/O Translation Table (VIOT) describes the topology of
 * para-virtual IOMMUs and the endpoints they manage. The OS uses it to
 * initialize devices in the right order, preventing endpoints from issuing
 * DMA before their IOMMU is ready.
 */

use core::ffi::{c_char, c_int, c_void};

// Kernel and ACPI declarations supplied by other translation units.
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct fwnode_handle { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct pci_dev { pub dev: device, pub bus: *mut pci_bus }
#[repr(C)] pub struct pci_bus { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct acpi_device { pub fwnode: fwnode_handle }
#[repr(C)] pub struct resource { pub start: u64, pub end: u64, pub flags: u64 }
#[repr(C)] pub struct acpi_table_header { _private: [u8; 0] }
#[repr(C)] pub struct acpi_table_viot { pub header: acpi_table_header, pub node_offset: u16, pub node_count: u16 }
#[repr(C)] pub struct acpi_viot_header { pub type_: u8, pub length: u16 }
#[repr(C)] pub struct acpi_viot_virtio_iommu_pci { pub header: acpi_viot_header, pub segment: u16, pub bdf: u16 }
#[repr(C)] pub struct acpi_viot_virtio_iommu_mmio { pub header: acpi_viot_header, pub base_address: u64 }
#[repr(C)] pub struct acpi_viot_mmio { pub header: acpi_viot_header, pub endpoint: u32, pub output_node: u16, pub base_address: u64 }
#[repr(C)] pub struct acpi_viot_pci_range { pub header: acpi_viot_header, pub segment_start: u16, pub segment_end: u16, pub bdf_start: u16, pub bdf_end: u16, pub output_node: u16, pub endpoint_start: u32 }
pub type acpi_status = u32;

extern "C" {
    fn acpi_get_table(signature: *const c_char, instance: u32, out: *mut *mut acpi_table_header) -> acpi_status;
    fn acpi_put_table(header: *mut acpi_table_header);
    fn acpi_format_exception(status: acpi_status) -> *const c_char;
    fn acpi_alloc_fwnode_static() -> *mut fwnode_handle;
    fn acpi_resource_consumer(res: *mut resource) -> *mut acpi_device;
    fn pci_get_domain_bus_and_slot(segment: u16, bus: u16, devfn: u16) -> *mut pci_dev;
    fn pci_dev_put(dev: *mut pci_dev);
    fn pci_request_acs();
    fn dev_fwnode(dev: *mut device) -> *mut fwnode_handle;
    fn set_primary_fwnode(dev: *mut device, fwnode: *mut fwnode_handle);
    fn acpi_iommu_fwspec_init(dev: *mut device, epid: u32, fwnode: *mut fwnode_handle) -> c_int;
    fn device_match_fwnode(dev: *mut device, fwnode: *mut fwnode_handle) -> bool;
    fn dev_is_pci(dev: *mut device) -> bool;
    fn dev_is_platform(dev: *mut device) -> bool;
    fn to_pci_dev(dev: *mut device) -> *mut pci_dev;
    fn to_platform_device(dev: *mut device) -> *mut platform_device;
    fn pci_domain_nr(bus: *mut pci_bus) -> u32;
    fn pci_for_each_dma_alias(dev: *mut pci_dev, cb: unsafe extern "C" fn(*mut pci_dev, u16, *mut c_void) -> c_int, data: *mut c_void) -> c_int;
    fn platform_get_resource(dev: *mut platform_device, typ: u64, index: u32) -> *mut resource;
}

#[repr(C)] pub struct viot_iommu { pub offset: u32, pub fwnode: *mut fwnode_handle, pub list: list_head }
#[repr(C)] pub union viot_endpoint_range { pub pci: viot_pci_endpoint, pub address: u64 }
#[repr(C)] pub struct viot_pci_endpoint { pub segment_start: u16, pub segment_end: u16, pub bdf_start: u16, pub bdf_end: u16 }
#[repr(C)] pub struct viot_endpoint { pub range: viot_endpoint_range, pub endpoint_id: u32, pub viommu: *mut viot_iommu, pub list: list_head }

static mut viot: *mut acpi_table_viot = core::ptr::null_mut();
static mut viot_iommus: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };
static mut viot_pci_ranges: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };
static mut viot_mmio_endpoints: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };

unsafe fn viot_check_bounds(hdr: *const acpi_viot_header) -> c_int {
    let start = (viot as *mut u8).add(core::cmp::max(core::mem::size_of::<acpi_table_viot>(), (*viot).node_offset as usize)) as *mut acpi_viot_header;
    let end = (viot as *mut u8).add((*viot).header_size()) as *mut acpi_viot_header;
    let hdr_end = (hdr as *mut u8).add(core::mem::size_of::<acpi_viot_header>());
    if hdr < start || hdr_end > end { return -75; }
    if (*hdr).length as usize < core::mem::size_of::<acpi_viot_header>() { return -22; }
    0
}

unsafe fn viot_get_iommu(_offset: u32) -> *mut viot_iommu { core::ptr::null_mut() }

trait ViotHeaderLength { unsafe fn header_size(&self) -> usize; }
impl ViotHeaderLength for acpi_table_viot { unsafe fn header_size(&self) -> usize { core::mem::size_of::<acpi_table_viot>() } }

#[no_mangle]
pub unsafe extern "C" fn acpi_viot_early_init() {
    let mut hdr: *mut acpi_table_header = core::ptr::null_mut();
    if acpi_get_table(b"VIOT\0".as_ptr() as *const c_char, 0, &mut hdr) != 0 { return; }
    pci_request_acs();
    acpi_put_table(hdr);
}

#[no_mangle]
pub unsafe extern "C" fn acpi_viot_init() {
    let mut hdr: *mut acpi_table_header = core::ptr::null_mut();
    let status = acpi_get_table(b"VIOT\0".as_ptr() as *const c_char, 0, &mut hdr);
    if status != 0 { return; }
    viot = hdr as *mut acpi_table_viot;
    let mut node = (viot as *mut u8).add((*viot).node_offset as usize) as *mut acpi_viot_header;
    for _ in 0..(*viot).node_count {
        if viot_parse_node(node) != 0 { return; }
        node = (node as *mut u8).add((*node).length as usize) as *mut acpi_viot_header;
    }
    acpi_put_table(hdr);
}

unsafe fn viot_parse_node(hdr: *const acpi_viot_header) -> c_int {
    if viot_check_bounds(hdr) != 0 { return -22; }
    if (*hdr).type_ == 1 || (*hdr).type_ == 2 { return 0; }
    0
}

#[no_mangle]
pub unsafe extern "C" fn viot_iommu_configure(dev: *mut device) -> c_int {
    if dev_is_pci(dev) {
        return pci_for_each_dma_alias(to_pci_dev(dev), viot_pci_dev_iommu_init, dev as *mut c_void);
    } else if dev_is_platform(dev) {
        return viot_mmio_dev_iommu_init(to_platform_device(dev));
    }
    -19
}

unsafe extern "C" fn viot_dev_iommu_init(dev: *mut device, viommu: *mut viot_iommu, epid: u32) -> c_int {
    if viommu.is_null() { return -19; }
    if device_match_fwnode(dev, (*viommu).fwnode) { return -22; }
    acpi_iommu_fwspec_init(dev, epid, (*viommu).fwnode)
}

unsafe extern "C" fn viot_pci_dev_iommu_init(pdev: *mut pci_dev, dev_id: u16, data: *mut c_void) -> c_int {
    let domain_nr = pci_domain_nr((*pdev).bus);
    let _ = (domain_nr, dev_id, data);
    -19
}

unsafe fn viot_mmio_dev_iommu_init(pdev: *mut platform_device) -> c_int {
    let mem = platform_get_resource(pdev, 0x00000200, 0);
    if mem.is_null() { return -19; }
    let _ = (*mem).start;
    -19
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
