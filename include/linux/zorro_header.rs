/*
 *  linux/zorro.h -- Amiga AutoConfig (Zorro) Bus Definitions
 *
 *  Copyright (C) 1995--2003 Geert Uytterhoeven
 *
 *  This file is subject to the terms and conditions of the GNU General Public
 *  License.  See the file COPYING in the main directory of this archive
 *  for more details.
 */

/* Declarations from the C includes are supplied by the surrounding kernel. */

/* Zorro devices */
#[repr(C)]
pub struct zorro_dev {
    pub rom: ExpansionRom,
    pub id: zorro_id,
    pub dev: device,
    pub slotaddr: u16,
    pub slotsize: u16,
    pub name: [core::ffi::c_char; 64],
    pub resource: resource,
}

/* C: container_of(n, struct zorro_dev, dev). */
#[macro_export]
macro_rules! to_zorro_dev {
    ($n:expr) => { container_of!($n, zorro_dev, dev) };
}

/* Zorro device drivers */
#[repr(C)]
pub struct zorro_driver {
    pub node: list_head,
    pub name: *mut core::ffi::c_char,
    pub id_table: *const zorro_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut zorro_dev, *const zorro_device_id) -> core::ffi::c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut zorro_dev)>,
    pub driver: device_driver,
}

/* C: container_of_const(drv, struct zorro_driver, driver). */
#[macro_export]
macro_rules! to_zorro_driver {
    ($drv:expr) => { container_of_const!($drv, zorro_driver, driver) };
}

/* C iteration macro; zorro_autocon and zorro_num_autocon are external globals. */
#[macro_export]
macro_rules! zorro_for_each_dev {
    ($dev:ident) => {
        for $dev in core::slice::from_raw_parts_mut(
            zorro_autocon,
            zorro_num_autocon as usize,
        )
    };
}

/* New-style probing */
extern "C" {
    pub fn zorro_register_driver(driver: *mut zorro_driver) -> core::ffi::c_int;
    pub fn zorro_unregister_driver(driver: *mut zorro_driver);

    pub static mut zorro_num_autocon: u32;
    pub static mut zorro_autocon: *mut zorro_dev;
}

/* Minimal information about a Zorro device, passed from bootinfo.
 * Only available temporarily, i.e. until initmem has been freed!
 */
#[repr(C)]
pub struct zorro_dev_init {
    pub rom: ExpansionRom,
    pub slotaddr: u16,
    pub slotsize: u16,
    pub boardaddr: u32,
    pub boardsize: u32,
}

extern "C" {
    pub static mut zorro_autocon_init: [zorro_dev_init; ZORRO_NUM_AUTO as usize];
}

/* Zorro Functions */
extern "C" {
    pub fn zorro_find_device(id: zorro_id, from: *mut zorro_dev) -> *mut zorro_dev;
}

#[inline]
pub unsafe fn zorro_resource_start(z: *mut zorro_dev) -> resource_size_t {
    (*z).resource.start
}

#[inline]
pub unsafe fn zorro_resource_end(z: *mut zorro_dev) -> resource_size_t {
    (*z).resource.end
}

#[inline]
pub unsafe fn zorro_resource_len(z: *mut zorro_dev) -> resource_size_t {
    resource_size(&(*z).resource)
}

#[inline]
pub unsafe fn zorro_resource_flags(z: *mut zorro_dev) -> resource_flags_t {
    (*z).resource.flags
}

#[inline]
pub unsafe fn zorro_request_device(z: *mut zorro_dev, name: *const core::ffi::c_char) -> *mut resource {
    request_mem_region(zorro_resource_start(z), zorro_resource_len(z), name)
}

#[inline]
pub unsafe fn zorro_release_device(z: *mut zorro_dev) {
    release_mem_region(zorro_resource_start(z), zorro_resource_len(z));
}

/* Similar to the helpers above, these manipulate per-zorro_dev
 * driver-specific data.  They are really just a wrapper around the generic
 * device structure functions of these calls.
 */
#[inline]
pub unsafe fn zorro_get_drvdata(z: *mut zorro_dev) -> *mut core::ffi::c_void {
    dev_get_drvdata(&mut (*z).dev)
}

#[inline]
pub unsafe fn zorro_set_drvdata(z: *mut zorro_dev, data: *mut core::ffi::c_void) {
    dev_set_drvdata(&mut (*z).dev, data);
}

/* Bitmask indicating portions of available Zorro II RAM that are unused by
 * the system. Every bit represents a 64K chunk, for a maximum of 8MB.
 */
extern "C" {
    pub static mut zorro_unused_z2ram: [core::ffi::c_ulong; 2];
}

pub const Z2RAM_START: u32 = 0x00200000;
pub const Z2RAM_END: u32 = 0x00a00000;
pub const Z2RAM_SIZE: u32 = 0x00800000;
pub const Z2RAM_CHUNKSIZE: u32 = 0x00010000;
pub const Z2RAM_CHUNKMASK: u32 = 0x0000ffff;
pub const Z2RAM_CHUNKSHIFT: u32 = 16;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
