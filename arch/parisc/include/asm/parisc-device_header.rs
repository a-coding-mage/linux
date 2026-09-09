/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the Linux device and PA-RISC device-id headers. */

use ::core::ffi::{c_char, c_int, c_ulong, c_void};

#[repr(C)]
pub struct parisc_device {
    pub hpa: resource,                         /* Hard Physical Address */
    pub id: parisc_device_id,
    pub driver: *mut parisc_driver,            /* Driver for this device */
    pub name: [c_char; 80],                    /* The hardware description */
    pub irq: c_int,
    pub aux_irq: c_int,                        /* Some devices have a second IRQ */

    pub hw_path: c_char,                       /* The module number on this bus */
    pub num_addrs: c_ulong,                    /* some devices have additional address ranges. */
    pub addr: *mut c_ulong,                    /* which will be stored here */

    /* CONFIG_64BIT fields; retained as conditional-intent comments. */
    #[cfg(target_pointer_width = "64")]
    pub pcell_loc: c_ulong,                    /* Physical Cell location */
    #[cfg(target_pointer_width = "64")]
    pub mod_index: c_ulong,                    /* PAT specific - Misc Module info */
    #[cfg(target_pointer_width = "64")]
    pub mod_info: c_ulong,                     /* PAT specific - Misc Module info */
    #[cfg(target_pointer_width = "64")]
    pub pmod_loc: c_ulong,                     /* physical Module location */
    #[cfg(target_pointer_width = "64")]
    pub mod0: c_ulong,

    pub dma_mask: u64,                         /* DMA mask for I/O */
    pub dev: device,
}

#[repr(C)]
pub struct parisc_driver {
    pub next: *mut parisc_driver,
    pub name: *mut c_char,
    pub id_table: *const parisc_device_id,
    pub probe: Option<unsafe extern "C" fn(dev: *mut parisc_device) -> c_int>, /* New device discovered */
    pub remove: Option<unsafe extern "C" fn(dev: *mut parisc_device)>,
    pub drv: device_driver,
}

/* The included Linux headers provide these types. */
extern "C" {
    pub type resource;
    pub type parisc_device_id;
    pub type device;
    pub type device_driver;
    pub type bus_type;

    pub fn dev_name(dev: *const device) -> *const c_char;
    pub fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    pub fn dev_get_drvdata(dev: *const device) -> *mut c_void;
}

macro_rules! to_parisc_device {
    ($d:expr) => {{
        ($d as *mut u8).sub(::core::mem::offset_of!(parisc_device, dev)) as *mut parisc_device
    }};
}

macro_rules! to_parisc_driver {
    ($d:expr) => {{
        ($d as *mut u8).sub(::core::mem::offset_of!(parisc_driver, drv)) as *mut parisc_driver
    }};
}

macro_rules! parisc_parent {
    ($d:expr) => {{
        to_parisc_device!((*$d).dev.parent)
    }};
}

#[inline]
pub unsafe fn parisc_pathname(d: *mut parisc_device) -> *const c_char {
    dev_name(&(*d).dev)
}

#[inline]
pub unsafe fn parisc_set_drvdata(d: *mut parisc_device, p: *mut c_void) {
    dev_set_drvdata(&mut (*d).dev, p);
}

#[inline]
pub unsafe fn parisc_get_drvdata(d: *mut parisc_device) -> *mut c_void {
    dev_get_drvdata(&(*d).dev)
}

extern "C" {
    pub static parisc_bus_type: bus_type;
    pub fn iosapic_serial_irq(dev: *mut parisc_device) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
