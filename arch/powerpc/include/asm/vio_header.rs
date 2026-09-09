/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * IBM PowerPC Virtual I/O Infrastructure Support.
 *
 * Copyright (c) 2003 IBM Corp.
 *  Dave Engebretsen engebret@us.ibm.com
 *  Santiago Leon santil@us.ibm.com
 */

/* C header guard: _ASM_POWERPC_VIO_H */
/* C kernel-only section: __KERNEL__ */

/* Dependencies supplied by the surrounding kernel translation unit. */

/*
 * Architecture-specific constants for drivers to
 * extract attributes of the device using vio_get_attribute()
 */
pub const VETH_MAC_ADDR: &str = "local-mac-address";
pub const VETH_MCAST_FILTER_SIZE: &str = "ibm,mac-address-filters";

/* End architecture-specific constants */

/* h_vio_signal(ua, mode) calls plpar_hcall_norets(H_VIO_SIGNAL, ua, mode). */
#[macro_export]
macro_rules! h_vio_signal {
    ($ua:expr, $mode:expr) => {
        plpar_hcall_norets(H_VIO_SIGNAL, $ua, $mode)
    };
}

pub const VIO_IRQ_DISABLE: ::core::primitive::u64 = 0;
pub const VIO_IRQ_ENABLE: ::core::primitive::u64 = 1;

/*
 * VIO CMO minimum entitlement for all devices and spare entitlement
 */
pub const VIO_CMO_MIN_ENT: ::core::primitive::u64 = 1562624;

extern "C" {
    pub static vio_bus_type: bus_type;
}

/*
 * vio_register_driver must be a macro so that KBUILD_MODNAME can be expanded.
 * The THIS_MODULE and KBUILD_MODNAME symbols are supplied by the build.
 */
#[macro_export]
macro_rules! vio_register_driver {
    ($driver:expr) => {
        __vio_register_driver($driver, THIS_MODULE, KBUILD_MODNAME)
    };
}

pub enum iommu_table {}

/*
 * Platform Facilities Option (PFO)-specific data
 */

/* Starting unit address for PFO devices on the VIO BUS */
pub const VIO_BASE_PFO_UA: ::core::primitive::u32 = 0x50000000;

/**
 * vio_pfo_op - PFO operation parameters
 *
 * @flags: h_call subfunctions and modifiers
 * @in: Input data block logical real address
 * @inlen: If non-negative, the length of the input data block.  If negative,
 *\tthe length of the input data descriptor list in bytes.
 * @out: Output data block logical real address
 * @outlen: If non-negative, the length of the input data block.  If negative,
 *\tthe length of the input data descriptor list in bytes.
 * @csbcpb: Logical real address of the 4k naturally-aligned storage block
 *\tcontaining the CSB & optional FC field specific CPB
 * @timeout: # of milliseconds to retry h_call, 0 for no timeout.
 * @hcall_err: pointer to return the h_call return value, else NULL
 */
#[repr(C)]
pub struct vio_pfo_op {
    pub flags: u64,
    pub r#in: i64,
    pub inlen: i64,
    pub out: i64,
    pub outlen: i64,
    pub csbcpb: u64,
    pub done: *mut ::core::ffi::c_void,
    pub handle: ::core::primitive::c_ulong,
    pub timeout: ::core::primitive::c_uint,
    pub hcall_err: ::core::primitive::c_long,
}

/* End PFO specific data */

#[repr(C)]
pub enum vio_dev_family {
    VDEVICE, /* The OF node is a child of /vdevice */
    PFO,     /* The OF node is a child of /ibm,platform-facilities */
}

/**
 * vio_dev - This structure is used to describe virtual I/O devices.
 *
 * @desired: set from return of driver's get_desired_dma() function
 * @entitled: bytes of IO data that has been reserved for this device.
 * @allocated: bytes of IO data currently in use by the device.
 * @allocs_failed: number of DMA failures due to insufficient entitlement.
 */
#[repr(C)]
pub struct vio_dev {
    pub name: *const ::core::ffi::c_char,
    pub r#type: *const ::core::ffi::c_char,
    pub unit_address: u32,
    pub resource_id: u32,
    pub irq: ::core::primitive::c_uint,
    pub cmo: vio_dev_cmo,
    pub family: vio_dev_family,
    pub dev: device,
}

#[repr(C)]
pub struct vio_dev_cmo {
    pub desired: usize,
    pub entitled: usize,
    pub allocated: usize,
    pub allocs_failed: atomic_t,
}

#[repr(C)]
pub struct vio_driver {
    pub name: *const ::core::ffi::c_char,
    pub id_table: *const vio_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut vio_dev, *const vio_device_id) -> ::core::primitive::c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut vio_dev)>,
    pub shutdown: Option<unsafe extern "C" fn(*mut vio_dev)>,
    /* A driver must have a get_desired_dma() function to
     * be loaded in a CMO environment if it uses DMA.
     */
    pub get_desired_dma: Option<unsafe extern "C" fn(*mut vio_dev) -> ::core::primitive::c_ulong>,
    pub pm: *const dev_pm_ops,
    pub driver: device_driver,
}

extern "C" {
    pub fn __vio_register_driver(
        drv: *mut vio_driver,
        owner: *mut module,
        mod_name: *const ::core::ffi::c_char,
    ) -> ::core::primitive::c_int;
    pub fn vio_unregister_driver(drv: *mut vio_driver);
    pub fn vio_cmo_entitlement_update(size: usize) -> ::core::primitive::c_int;
    pub fn vio_cmo_set_dev_desired(viodev: *mut vio_dev, desired: usize);
    pub fn vio_unregister_device(dev: *mut vio_dev);
    pub fn vio_h_cop_sync(vdev: *mut vio_dev, op: *mut vio_pfo_op) -> ::core::primitive::c_int;
}

pub enum device_node {}

extern "C" {
    pub fn vio_register_device_node(node_vdev: *mut device_node) -> *mut vio_dev;
    pub fn vio_get_attribute(
        vdev: *mut vio_dev,
        which: *mut ::core::ffi::c_char,
        length: *mut ::core::primitive::c_int,
    ) -> *const ::core::ffi::c_void;
}

/* CONFIG_PPC_PSERIES selects the external interrupt implementations. */
#[cfg(CONFIG_PPC_PSERIES)]
extern "C" {
    pub fn vio_find_node(vnode: *mut device_node) -> *mut vio_dev;
    pub fn vio_enable_interrupts(dev: *mut vio_dev) -> ::core::primitive::c_int;
    pub fn vio_disable_interrupts(dev: *mut vio_dev) -> ::core::primitive::c_int;
}

#[cfg(not(CONFIG_PPC_PSERIES))]
pub unsafe extern "C" fn vio_enable_interrupts(_dev: *mut vio_dev) -> ::core::primitive::c_int {
    0
}

/* C macros to_vio_driver and to_vio_dev use container_of_const. */
#[macro_export]
macro_rules! to_vio_driver {
    ($drv:expr) => {
        container_of_const!($drv, vio_driver, driver)
    };
}

#[macro_export]
macro_rules! to_vio_dev {
    ($dev:expr) => {
        container_of_const!($dev, vio_dev, dev)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
