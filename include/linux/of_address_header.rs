/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation.

pub struct of_bus;

#[repr(C)]
pub struct of_pci_range_parser {
    pub node: *mut device_node,
    pub bus: *const of_bus,
    pub range: *const __be32,
    pub end: *const __be32,
    pub na: c_int,
    pub ns: c_int,
    pub pna: c_int,
    pub dma: bool,
}
pub type of_range_parser = of_pci_range_parser;

#[repr(C)]
pub union of_pci_range_address {
    pub pci_addr: u64,
    pub bus_addr: u64,
}

#[repr(C)]
pub struct of_pci_range {
    pub address: of_pci_range_address,
    pub cpu_addr: u64,
    pub parent_bus_addr: u64,
    pub size: u64,
    pub flags: u32,
}
pub type of_range = of_pci_range;

// for_each_of_pci_range(parser, range): for (; of_pci_range_parser_one(parser, range);)
// for_each_of_range is an alias for for_each_of_pci_range.

#[inline]
pub unsafe fn of_range_count(parser: *const of_range_parser) -> c_int {
    if parser.is_null()
        || (*parser).node.is_null()
        || (*parser).range.is_null()
        || (*parser).range == (*parser).end
    {
        return 0;
    }
    ((*parser).end.offset_from((*parser).range)
        / ((*parser).na + (*parser).pna + (*parser).ns) as isize) as c_int
}

/* Translate a DMA address from device space to CPU space */
extern "C" {
    pub fn of_translate_dma_address(dev: *mut device_node, in_addr: *const __be32) -> u64;
    pub fn of_translate_dma_region(
        dev: *mut device_node,
        addr: *const __be32,
        start: *mut phys_addr_t,
        length: *mut size_t,
    ) -> *const __be32;
}

// CONFIG_OF_ADDRESS declarations and !CONFIG_OF_ADDRESS inline fallbacks are
// preserved below as conditional Rust declarations.
#[cfg(feature = "CONFIG_OF_ADDRESS")]
extern "C" {
    pub fn of_translate_address(np: *mut device_node, addr: *const __be32) -> u64;
    pub fn of_address_to_resource(dev: *mut device_node, index: c_int, r: *mut resource) -> c_int;
    pub fn of_iomap(device: *mut device_node, index: c_int) -> *mut core::ffi::c_void;
    pub fn of_io_request_and_map(device: *mut device_node, index: c_int, name: *const c_char) -> *mut core::ffi::c_void;
    pub fn __of_get_address(dev: *mut device_node, index: c_int, bar_no: c_int, size: *mut u64, flags: *mut c_uint) -> *const __be32;
    pub fn of_property_read_reg(np: *mut device_node, idx: c_int, addr: *mut u64, size: *mut u64) -> c_int;
    pub fn of_pci_range_parser_init(parser: *mut of_pci_range_parser, node: *mut device_node) -> c_int;
    pub fn of_pci_dma_range_parser_init(parser: *mut of_pci_range_parser, node: *mut device_node) -> c_int;
    pub fn of_pci_range_parser_one(parser: *mut of_pci_range_parser, range: *mut of_pci_range) -> *mut of_pci_range;
    pub fn of_pci_address_to_resource(dev: *mut device_node, bar: c_int, r: *mut resource) -> c_int;
    pub fn of_pci_range_to_resource(range: *const of_pci_range, np: *const device_node, res: *mut resource) -> c_int;
    pub fn of_range_to_resource(np: *mut device_node, index: c_int, res: *mut resource) -> c_int;
    pub fn of_dma_is_coherent(np: *mut device_node) -> bool;
}

#[cfg(not(feature = "CONFIG_OF_ADDRESS"))]
#[inline]
pub unsafe fn of_translate_address(_np: *mut device_node, _addr: *const __be32) -> u64 { OF_BAD_ADDR }
#[cfg(not(feature = "CONFIG_OF_ADDRESS"))]
#[inline]
pub unsafe fn __of_get_address(_dev: *mut device_node, _index: c_int, _bar_no: c_int, _size: *mut u64, _flags: *mut c_uint) -> *const __be32 { core::ptr::null() }
#[cfg(not(feature = "CONFIG_OF_ADDRESS"))]
#[inline]
pub unsafe fn of_property_read_reg(_np: *mut device_node, _idx: c_int, _addr: *mut u64, _size: *mut u64) -> c_int { -ENOSYS }
#[cfg(not(feature = "CONFIG_OF_ADDRESS"))]
#[inline]
pub unsafe fn of_pci_range_parser_init(_parser: *mut of_pci_range_parser, _node: *mut device_node) -> c_int { -ENOSYS }
#[cfg(not(feature = "CONFIG_OF_ADDRESS"))]
#[inline]
pub unsafe fn of_pci_dma_range_parser_init(_parser: *mut of_pci_range_parser, _node: *mut device_node) -> c_int { -ENOSYS }
#[cfg(not(feature = "CONFIG_OF_ADDRESS"))]
#[inline]
pub unsafe fn of_pci_range_parser_one(_parser: *mut of_pci_range_parser, _range: *mut of_pci_range) -> *mut of_pci_range { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_OF_ADDRESS"))]
#[inline]
pub unsafe fn of_pci_address_to_resource(_dev: *mut device_node, _bar: c_int, _r: *mut resource) -> c_int { -ENOSYS }
#[cfg(not(feature = "CONFIG_OF_ADDRESS"))]
#[inline]
pub unsafe fn of_pci_range_to_resource(_range: *mut of_pci_range, _np: *mut device_node, _res: *mut resource) -> c_int { -ENOSYS }
#[cfg(not(feature = "CONFIG_OF_ADDRESS"))]
#[inline]
pub unsafe fn of_range_to_resource(_np: *mut device_node, _index: c_int, _res: *mut resource) -> c_int { -ENOSYS }
#[cfg(not(feature = "CONFIG_OF_ADDRESS"))]
#[inline]
pub unsafe fn of_dma_is_coherent(_np: *mut device_node) -> bool { false }

#[cfg(feature = "CONFIG_OF")]
extern "C" {
    pub fn of_address_to_resource(dev: *mut device_node, index: c_int, r: *mut resource) -> c_int;
    pub fn of_iomap(node: *mut device_node, index: c_int) -> *mut core::ffi::c_void;
}
#[cfg(not(feature = "CONFIG_OF"))]
#[inline]
pub unsafe fn of_address_to_resource(_dev: *mut device_node, _index: c_int, _r: *mut resource) -> c_int { -EINVAL }
#[cfg(not(feature = "CONFIG_OF"))]
#[inline]
pub unsafe fn of_iomap(_device: *mut device_node, _index: c_int) -> *mut core::ffi::c_void { core::ptr::null_mut() }

// of_range_parser_init is an alias for of_pci_range_parser_init.

#[inline]
pub unsafe fn of_get_address(dev: *mut device_node, index: c_int, size: *mut u64, flags: *mut c_uint) -> *const __be32 {
    __of_get_address(dev, index, -1, size, flags)
}

#[inline]
pub unsafe fn of_get_pci_address(dev: *mut device_node, bar_no: c_int, size: *mut u64, flags: *mut c_uint) -> *const __be32 {
    __of_get_address(dev, -1, bar_no, size, flags)
}

#[inline]
pub unsafe fn of_address_count(np: *mut device_node) -> c_int {
    let mut res: resource = core::mem::zeroed();
    let mut count: c_int = 0;
    while of_address_to_resource(np, count, &mut res) == 0 { count += 1; }
    count
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
