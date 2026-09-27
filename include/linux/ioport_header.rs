/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of linux/ioport.h. */

#[repr(C)]
pub struct resource {
    pub start: resource_size_t,
    pub end: resource_size_t,
    pub name: *const ::kernel::ffi::c_char,
    pub flags: ::kernel::ffi::c_ulong,
    pub desc: ::kernel::ffi::c_ulong,
    pub parent: *mut resource,
    pub sibling: *mut resource,
    pub child: *mut resource,
}

pub const IORESOURCE_BITS: ::kernel::ffi::c_ulong = 0x000000ff;
pub const IORESOURCE_TYPE_BITS: ::kernel::ffi::c_ulong = 0x00001f00;
pub const IORESOURCE_IO: ::kernel::ffi::c_ulong = 0x00000100;
pub const IORESOURCE_MEM: ::kernel::ffi::c_ulong = 0x00000200;
pub const IORESOURCE_REG: ::kernel::ffi::c_ulong = 0x00000300;
pub const IORESOURCE_IRQ: ::kernel::ffi::c_ulong = 0x00000400;
pub const IORESOURCE_DMA: ::kernel::ffi::c_ulong = 0x00000800;
pub const IORESOURCE_BUS: ::kernel::ffi::c_ulong = 0x00001000;
pub const IORESOURCE_PREFETCH: ::kernel::ffi::c_ulong = 0x00002000;
pub const IORESOURCE_READONLY: ::kernel::ffi::c_ulong = 0x00004000;
pub const IORESOURCE_CACHEABLE: ::kernel::ffi::c_ulong = 0x00008000;
pub const IORESOURCE_RANGELENGTH: ::kernel::ffi::c_ulong = 0x00010000;
pub const IORESOURCE_SHADOWABLE: ::kernel::ffi::c_ulong = 0x00020000;
pub const IORESOURCE_SIZEALIGN: ::kernel::ffi::c_ulong = 0x00040000;
pub const IORESOURCE_STARTALIGN: ::kernel::ffi::c_ulong = 0x00080000;
pub const IORESOURCE_MEM_64: ::kernel::ffi::c_ulong = 0x00100000;
pub const IORESOURCE_WINDOW: ::kernel::ffi::c_ulong = 0x00200000;
pub const IORESOURCE_MUXED: ::kernel::ffi::c_ulong = 0x00400000;
pub const IORESOURCE_EXT_TYPE_BITS: ::kernel::ffi::c_ulong = 0x01000000;
pub const IORESOURCE_SYSRAM: ::kernel::ffi::c_ulong = 0x01000000;
pub const IORESOURCE_SYSRAM_DRIVER_MANAGED: ::kernel::ffi::c_ulong = 0x02000000;
pub const IORESOURCE_SYSRAM_MERGEABLE: ::kernel::ffi::c_ulong = 0x04000000;
pub const IORESOURCE_EXCLUSIVE: ::kernel::ffi::c_ulong = 0x08000000;
pub const IORESOURCE_DISABLED: ::kernel::ffi::c_ulong = 0x10000000;
pub const IORESOURCE_UNSET: ::kernel::ffi::c_ulong = 0x20000000;
pub const IORESOURCE_AUTO: ::kernel::ffi::c_ulong = 0x40000000;
pub const IORESOURCE_BUSY: ::kernel::ffi::c_ulong = 0x80000000;
pub const IORESOURCE_SYSTEM_RAM: ::kernel::ffi::c_ulong = IORESOURCE_MEM | IORESOURCE_SYSRAM;

pub const IORESOURCE_IRQ_HIGHEDGE: ::kernel::ffi::c_ulong = 1 << 0;
pub const IORESOURCE_IRQ_LOWEDGE: ::kernel::ffi::c_ulong = 1 << 1;
pub const IORESOURCE_IRQ_HIGHLEVEL: ::kernel::ffi::c_ulong = 1 << 2;
pub const IORESOURCE_IRQ_LOWLEVEL: ::kernel::ffi::c_ulong = 1 << 3;
pub const IORESOURCE_IRQ_SHAREABLE: ::kernel::ffi::c_ulong = 1 << 4;
pub const IORESOURCE_IRQ_OPTIONAL: ::kernel::ffi::c_ulong = 1 << 5;
pub const IORESOURCE_IRQ_WAKECAPABLE: ::kernel::ffi::c_ulong = 1 << 6;
pub const IORESOURCE_DMA_TYPE_MASK: ::kernel::ffi::c_ulong = 3 << 0;
pub const IORESOURCE_DMA_8BIT: ::kernel::ffi::c_ulong = 0 << 0;
pub const IORESOURCE_DMA_8AND16BIT: ::kernel::ffi::c_ulong = 1 << 0;
pub const IORESOURCE_DMA_16BIT: ::kernel::ffi::c_ulong = 2 << 0;
pub const IORESOURCE_DMA_MASTER: ::kernel::ffi::c_ulong = 1 << 2;
pub const IORESOURCE_DMA_BYTE: ::kernel::ffi::c_ulong = 1 << 3;
pub const IORESOURCE_DMA_WORD: ::kernel::ffi::c_ulong = 1 << 4;
pub const IORESOURCE_DMA_SPEED_MASK: ::kernel::ffi::c_ulong = 3 << 6;
pub const IORESOURCE_DMA_COMPATIBLE: ::kernel::ffi::c_ulong = 0 << 6;
pub const IORESOURCE_DMA_TYPEA: ::kernel::ffi::c_ulong = 1 << 6;
pub const IORESOURCE_DMA_TYPEB: ::kernel::ffi::c_ulong = 2 << 6;
pub const IORESOURCE_DMA_TYPEF: ::kernel::ffi::c_ulong = 3 << 6;
pub const IORESOURCE_MEM_WRITEABLE: ::kernel::ffi::c_ulong = 1 << 0;
pub const IORESOURCE_MEM_CACHEABLE: ::kernel::ffi::c_ulong = 1 << 1;
pub const IORESOURCE_MEM_RANGELENGTH: ::kernel::ffi::c_ulong = 1 << 2;
pub const IORESOURCE_MEM_TYPE_MASK: ::kernel::ffi::c_ulong = 3 << 3;
pub const IORESOURCE_MEM_8BIT: ::kernel::ffi::c_ulong = 0 << 3;
pub const IORESOURCE_MEM_16BIT: ::kernel::ffi::c_ulong = 1 << 3;
pub const IORESOURCE_MEM_8AND16BIT: ::kernel::ffi::c_ulong = 2 << 3;
pub const IORESOURCE_MEM_32BIT: ::kernel::ffi::c_ulong = 3 << 3;
pub const IORESOURCE_MEM_SHADOWABLE: ::kernel::ffi::c_ulong = 1 << 5;
pub const IORESOURCE_MEM_EXPANSIONROM: ::kernel::ffi::c_ulong = 1 << 6;
pub const IORESOURCE_MEM_NONPOSTED: ::kernel::ffi::c_ulong = 1 << 7;
pub const IORESOURCE_IO_16BIT_ADDR: ::kernel::ffi::c_ulong = 1 << 0;
pub const IORESOURCE_IO_FIXED: ::kernel::ffi::c_ulong = 1 << 1;
pub const IORESOURCE_IO_SPARSE: ::kernel::ffi::c_ulong = 1 << 2;
pub const IORESOURCE_ROM_ENABLE: ::kernel::ffi::c_ulong = 1 << 0;
pub const IORESOURCE_ROM_SHADOW: ::kernel::ffi::c_ulong = 1 << 1;
pub const IORESOURCE_PCI_FIXED: ::kernel::ffi::c_ulong = 1 << 4;
pub const IORESOURCE_PCI_EA_BEI: ::kernel::ffi::c_ulong = 1 << 5;

pub const IORES_DESC_NONE: ::kernel::ffi::c_int = 0;
pub const IORES_DESC_CRASH_KERNEL: ::kernel::ffi::c_int = 1;
pub const IORES_DESC_ACPI_TABLES: ::kernel::ffi::c_int = 2;
pub const IORES_DESC_ACPI_NV_STORAGE: ::kernel::ffi::c_int = 3;
pub const IORES_DESC_PERSISTENT_MEMORY: ::kernel::ffi::c_int = 4;
pub const IORES_DESC_PERSISTENT_MEMORY_LEGACY: ::kernel::ffi::c_int = 5;
pub const IORES_DESC_DEVICE_PRIVATE_MEMORY: ::kernel::ffi::c_int = 6;
pub const IORES_DESC_RESERVED: ::kernel::ffi::c_int = 7;
pub const IORES_DESC_SOFT_RESERVED: ::kernel::ffi::c_int = 8;
pub const IORES_DESC_CXL: ::kernel::ffi::c_int = 9;
pub const IORES_MAP_SYSTEM_RAM: ::kernel::ffi::c_uint = 1 << 0;
pub const IORES_MAP_ENCRYPTED: ::kernel::ffi::c_uint = 1 << 1;

pub type resource_alignf = Option<unsafe extern "C" fn(*mut ::kernel::ffi::c_void, *const resource, *const resource, resource_size_t, resource_size_t) -> resource_size_t>;
#[repr(C)]
pub struct resource_constraint {
    pub min: resource_size_t, pub max: resource_size_t, pub align: resource_size_t,
    pub alignf: resource_alignf, pub alignf_data: *mut ::kernel::ffi::c_void,
}

pub const fn __define_res_named_desc(start: resource_size_t, size: resource_size_t, name: *const ::kernel::ffi::c_char, flags: ::kernel::ffi::c_ulong, desc: ::kernel::ffi::c_ulong) -> resource { resource { start, end: start + size - 1, name, flags, desc, parent: core::ptr::null_mut(), sibling: core::ptr::null_mut(), child: core::ptr::null_mut() } }
pub const fn __define_res0() -> resource { __define_res_named_desc(0, 0, core::ptr::null(), IORESOURCE_UNSET, IORES_DESC_NONE as _) }
pub const fn __define_res3(start: resource_size_t, size: resource_size_t, flags: ::kernel::ffi::c_ulong) -> resource { __define_res_named_desc(start, size, core::ptr::null(), flags, IORES_DESC_NONE as _) }

extern "C" {
    pub static mut ioport_resource: resource;
    pub static mut iomem_resource: resource;
    pub static mut soft_reserve_resource: resource;
    pub fn request_resource_conflict(root: *mut resource, new: *mut resource) -> *mut resource;
    pub fn request_resource(root: *mut resource, new: *mut resource) -> ::kernel::ffi::c_int;
    pub fn release_resource(new: *mut resource) -> ::kernel::ffi::c_int;
    pub fn release_child_resources(new: *mut resource);
    pub fn reserve_region_with_split(root: *mut resource, start: resource_size_t, end: resource_size_t, name: *const ::kernel::ffi::c_char);
    pub fn insert_resource_conflict(parent: *mut resource, new: *mut resource) -> *mut resource;
    pub fn insert_resource(parent: *mut resource, new: *mut resource) -> ::kernel::ffi::c_int;
    pub fn insert_resource_expand_to_fit(root: *mut resource, new: *mut resource);
    pub fn remove_resource(old: *mut resource) -> ::kernel::ffi::c_int;
    pub fn arch_remove_reservations(avail: *mut resource);
    pub fn allocate_resource(root: *mut resource, new: *mut resource, size: resource_size_t, min: resource_size_t, max: resource_size_t, align: resource_size_t, alignf: resource_alignf, alignf_data: *mut ::kernel::ffi::c_void) -> ::kernel::ffi::c_int;
    pub fn lookup_resource(root: *mut resource, start: resource_size_t) -> *mut resource;
    pub fn adjust_resource(res: *mut resource, start: resource_size_t, size: resource_size_t) -> ::kernel::ffi::c_int;
    pub fn resource_alignment(res: *const resource) -> resource_size_t;
    pub fn find_resource_space(root: *mut resource, new: *mut resource, size: resource_size_t, constraint: *mut resource_constraint) -> ::kernel::ffi::c_int;
    pub fn __request_region(parent: *mut resource, start: resource_size_t, n: resource_size_t, name: *const ::kernel::ffi::c_char, flags: ::kernel::ffi::c_int) -> *mut resource;
    pub fn __release_region(parent: *mut resource, start: resource_size_t, n: resource_size_t);
    #[cfg(CONFIG_MEMORY_HOTREMOVE)] pub fn release_mem_region_adjustable(start: resource_size_t, size: resource_size_t);
    #[cfg(CONFIG_MEMORY_HOTPLUG)] pub fn merge_system_ram_resource(res: *mut resource);
    pub fn iomem_map_sanity_check(addr: resource_size_t, size: ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_int;
    pub fn iomem_is_exclusive(addr: u64) -> bool;
    pub fn resource_is_exclusive(res: *mut resource, addr: u64, size: resource_size_t) -> bool;
    pub fn walk_system_ram_range(start_pfn: ::kernel::ffi::c_ulong, nr_pages: ::kernel::ffi::c_ulong, arg: *mut ::kernel::ffi::c_void, func: Option<unsafe extern "C" fn(::kernel::ffi::c_ulong, ::kernel::ffi::c_ulong, *mut ::kernel::ffi::c_void) -> ::kernel::ffi::c_int>) -> ::kernel::ffi::c_int;
    pub fn walk_mem_res(start: u64, end: u64, arg: *mut ::kernel::ffi::c_void, func: Option<unsafe extern "C" fn(*mut resource, *mut ::kernel::ffi::c_void) -> ::kernel::ffi::c_int>) -> ::kernel::ffi::c_int;
    pub fn walk_system_ram_res(start: u64, end: u64, arg: *mut ::kernel::ffi::c_void, func: Option<unsafe extern "C" fn(*mut resource, *mut ::kernel::ffi::c_void) -> ::kernel::ffi::c_int>) -> ::kernel::ffi::c_int;
    pub fn walk_system_ram_res_rev(start: u64, end: u64, arg: *mut ::kernel::ffi::c_void, func: Option<unsafe extern "C" fn(*mut resource, *mut ::kernel::ffi::c_void) -> ::kernel::ffi::c_int>) -> ::kernel::ffi::c_int;
    pub fn walk_iomem_res_desc(desc: ::kernel::ffi::c_ulong, flags: ::kernel::ffi::c_ulong, start: u64, end: u64, arg: *mut ::kernel::ffi::c_void, func: Option<unsafe extern "C" fn(*mut resource, *mut ::kernel::ffi::c_void) -> ::kernel::ffi::c_int>) -> ::kernel::ffi::c_int;
    pub fn walk_soft_reserve_res(start: u64, end: u64, arg: *mut ::kernel::ffi::c_void, func: Option<unsafe extern "C" fn(*mut resource, *mut ::kernel::ffi::c_void) -> ::kernel::ffi::c_int>) -> ::kernel::ffi::c_int;
    pub fn region_intersects_soft_reserve(start: resource_size_t, size: usize) -> ::kernel::ffi::c_int;
    pub fn devm_request_free_mem_region(dev: *mut device, base: *mut resource, size: ::kernel::ffi::c_ulong) -> *mut resource;
    pub fn request_free_mem_region(base: *mut resource, size: ::kernel::ffi::c_ulong, name: *const ::kernel::ffi::c_char) -> *mut resource;
    pub fn alloc_free_mem_region(base: *mut resource, size: ::kernel::ffi::c_ulong, align: ::kernel::ffi::c_ulong, name: *const ::kernel::ffi::c_char) -> *mut resource;
    pub fn devm_request_resource(dev: *mut device, root: *mut resource, new: *mut resource) -> ::kernel::ffi::c_int;
    pub fn devm_release_resource(dev: *mut device, new: *mut resource);
    pub fn __devm_request_region(dev: *mut device, parent: *mut resource, start: resource_size_t, n: resource_size_t, name: *const ::kernel::ffi::c_char) -> *mut resource;
    pub fn __devm_release_region(dev: *mut device, parent: *mut resource, start: resource_size_t, n: resource_size_t);
    pub fn iomem_get_mapping() -> *mut address_space;
}

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct address_space { _private: [u8; 0] }

#[macro_export] macro_rules! DEFINE_RES_NAMED_DESC { ($start:expr, $size:expr, $name:expr, $flags:expr, $desc:expr) => { $crate::__define_res_named_desc($start, $size, $name, $flags, $desc) }; }
#[macro_export] macro_rules! DEFINE_RES_NAMED { ($start:expr, $size:expr, $name:expr, $flags:expr) => { $crate::__define_res_named_desc($start, $size, $name, $flags, $crate::IORES_DESC_NONE as _) }; }
#[macro_export] macro_rules! DEFINE_RES_IO_NAMED { ($start:expr, $size:expr, $name:expr) => { $crate::DEFINE_RES_NAMED!($start, $size, $name, $crate::IORESOURCE_IO) }; }
#[macro_export] macro_rules! DEFINE_RES_MEM_NAMED { ($start:expr, $size:expr, $name:expr) => { $crate::DEFINE_RES_NAMED!($start, $size, $name, $crate::IORESOURCE_MEM) }; }
#[macro_export] macro_rules! DEFINE_RES_REG_NAMED { ($start:expr, $size:expr, $name:expr) => { $crate::DEFINE_RES_NAMED!($start, $size, $name, $crate::IORESOURCE_REG) }; }
#[macro_export] macro_rules! DEFINE_RES_IRQ_NAMED { ($irq:expr, $name:expr) => { $crate::DEFINE_RES_NAMED!($irq, 1, $name, $crate::IORESOURCE_IRQ) }; }
#[macro_export] macro_rules! DEFINE_RES_DMA_NAMED { ($dma:expr, $name:expr) => { $crate::DEFINE_RES_NAMED!($dma, 1, $name, $crate::IORESOURCE_DMA) }; }

#[inline] pub unsafe fn resource_set_size(res: *mut resource, size: resource_size_t) { (*res).end = (*res).start + size - 1; }
#[inline] pub unsafe fn resource_set_range(res: *mut resource, start: resource_size_t, size: resource_size_t) { (*res).start = start; resource_set_size(res, size); }
#[inline] pub unsafe fn resource_size(res: *const resource) -> resource_size_t { (*res).end - (*res).start + 1 }
#[inline] pub unsafe fn resource_type(res: *const resource) -> ::kernel::ffi::c_ulong { (*res).flags & IORESOURCE_TYPE_BITS }
#[inline] pub unsafe fn resource_ext_type(res: *const resource) -> ::kernel::ffi::c_ulong { (*res).flags & IORESOURCE_EXT_TYPE_BITS }
#[inline] pub unsafe fn __resource_contains_unbound(r1: *const resource, r2: *const resource) -> bool { resource_type(r1) == resource_type(r2) && (*r1).start <= (*r2).start && (*r1).end >= (*r2).end }
#[inline] pub unsafe fn resource_contains(r1: *const resource, r2: *const resource) -> bool { (*r1).flags & IORESOURCE_UNSET == 0 && (*r2).flags & IORESOURCE_UNSET == 0 && __resource_contains_unbound(r1, r2) }
#[inline] pub unsafe fn resource_overlaps(r1: *const resource, r2: *const resource) -> bool { (*r1).start <= (*r2).end && (*r1).end >= (*r2).start }
#[inline] pub unsafe fn resource_intersection(r1: *const resource, r2: *const resource, r: *mut resource) -> bool { if !resource_overlaps(r1,r2) { return false; } (*r).start = (*r1).start.max((*r2).start); (*r).end = (*r1).end.min((*r2).end); true }
#[inline] pub unsafe fn resource_union(r1: *const resource, r2: *const resource, r: *mut resource) -> bool { if !resource_overlaps(r1,r2) { return false; } (*r).start = (*r1).start.min((*r2).start); (*r).end = (*r1).end.max((*r2).end); true }
#[inline] pub unsafe fn resource_assigned(res: *const resource) -> bool { !(*res).parent.is_null() }

#[inline] pub unsafe fn irqresource_disabled(res: *mut resource, irq: u32) { (*res).start = irq as _; (*res).end = irq as _; (*res).flags |= IORESOURCE_IRQ | IORESOURCE_DISABLED | IORESOURCE_UNSET; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
