/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of linux/uio_driver.h. */

/* Dependencies supplied by the surrounding kernel translation. */

pub struct module;
pub struct uio_map;
pub struct uio_portio;
pub struct uio_info;
pub struct device;
pub struct fasync_struct;
pub struct wait_queue_head_t;
pub struct mutex;
pub struct kobject;
pub struct vm_area_desc;
pub struct inode;

#[repr(C)]
pub struct uio_mem {
    pub name: *const ::core::ffi::c_char,
    pub addr: phys_addr_t,
    pub dma_addr: dma_addr_t,
    pub offs: ::core::ffi::c_ulong,
    pub size: resource_size_t,
    pub memtype: ::core::ffi::c_int,
    pub internal_addr: *mut ::core::ffi::c_void,
    pub dma_device: *mut device,
    pub map: *mut uio_map,
}

pub const MAX_UIO_MAPS: usize = 5;

#[repr(C)]
pub struct uio_port {
    pub name: *const ::core::ffi::c_char,
    pub start: ::core::ffi::c_ulong,
    pub size: ::core::ffi::c_ulong,
    pub porttype: ::core::ffi::c_int,
    pub portio: *mut uio_portio,
}

pub const MAX_UIO_PORT_REGIONS: usize = 5;

#[repr(C)]
pub struct uio_device {
    pub owner: *mut module,
    pub dev: device,
    pub minor: ::core::ffi::c_int,
    pub event: atomic_t,
    pub async_queue: *mut fasync_struct,
    pub wait: wait_queue_head_t,
    pub info: *mut uio_info,
    pub info_lock: mutex,
    pub map_dir: *mut kobject,
    pub portio_dir: *mut kobject,
}

#[repr(C)]
pub struct uio_info {
    pub uio_dev: *mut uio_device,
    pub name: *const ::core::ffi::c_char,
    pub version: *const ::core::ffi::c_char,
    pub mem: [uio_mem; MAX_UIO_MAPS],
    pub port: [uio_port; MAX_UIO_PORT_REGIONS],
    pub irq: ::core::ffi::c_long,
    pub irq_flags: ::core::ffi::c_ulong,
    pub priv_: *mut ::core::ffi::c_void,
    pub handler: Option<unsafe extern "C" fn(irq: ::core::ffi::c_int, dev_info: *mut uio_info) -> irqreturn_t>,
    pub mmap_prepare: Option<unsafe extern "C" fn(info: *mut uio_info, desc: *mut vm_area_desc) -> ::core::ffi::c_int>,
    pub open: Option<unsafe extern "C" fn(info: *mut uio_info, inode: *mut inode) -> ::core::ffi::c_int>,
    pub release: Option<unsafe extern "C" fn(info: *mut uio_info, inode: *mut inode) -> ::core::ffi::c_int>,
    pub irqcontrol: Option<unsafe extern "C" fn(info: *mut uio_info, irq_on: s32) -> ::core::ffi::c_int>,
}

unsafe extern "C" {
    pub fn __uio_register_device(owner: *mut module, parent: *mut device, info: *mut uio_info) -> ::core::ffi::c_int;
    pub fn uio_unregister_device(info: *mut uio_info);
    pub fn uio_event_notify(info: *mut uio_info);
    pub fn __devm_uio_register_device(owner: *mut module, parent: *mut device, info: *mut uio_info) -> ::core::ffi::c_int;
}

/* These macros preserve the C header's THIS_MODULE-based wrappers. */
#[macro_export]
macro_rules! uio_register_device {
    ($parent:expr, $info:expr) => { unsafe { __uio_register_device(THIS_MODULE, $parent, $info) } };
}
#[macro_export]
macro_rules! devm_uio_register_device {
    ($parent:expr, $info:expr) => { unsafe { __devm_uio_register_device(THIS_MODULE, $parent, $info) } };
}

pub const UIO_IRQ_CUSTOM: ::core::ffi::c_int = -1;
pub const UIO_IRQ_NONE: ::core::ffi::c_int = 0;
pub const UIO_MEM_NONE: ::core::ffi::c_int = 0;
pub const UIO_MEM_PHYS: ::core::ffi::c_int = 1;
pub const UIO_MEM_LOGICAL: ::core::ffi::c_int = 2;
pub const UIO_MEM_VIRTUAL: ::core::ffi::c_int = 3;
pub const UIO_MEM_IOVA: ::core::ffi::c_int = 4;
pub const UIO_MEM_DMA_COHERENT: ::core::ffi::c_int = 5;
pub const UIO_PORT_NONE: ::core::ffi::c_int = 0;
pub const UIO_PORT_X86: ::core::ffi::c_int = 1;
pub const UIO_PORT_GPIO: ::core::ffi::c_int = 2;
pub const UIO_PORT_OTHER: ::core::ffi::c_int = 3;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
