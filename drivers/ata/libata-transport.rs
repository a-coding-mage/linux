// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of libata-transport.c.  Kernel dependencies are supplied by
 * the surrounding translation unit. */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

extern "C" {
    fn ata_tlink_delete(link: *mut ata_link);
    fn ata_host_get(host: *mut c_void);
    fn ata_host_put(host: *mut c_void);
    fn ata_dev_classify(tf: *const ata_taskfile) -> c_uint;
    fn ata_port_dbg(ap: *mut ata_port, fmt: *const c_char, ...);
    fn ata_port_info(ap: *mut ata_port, fmt: *const c_char, ...);
    fn ata_acpi_bind_port(ap: *mut ata_port);
    fn ata_acpi_bind_dev(dev: *mut ata_device);
    fn ata_ering_map(ering: *mut c_void, cb: unsafe extern "C" fn(*mut ata_ering_entry,*mut c_void)->c_int, arg: *mut c_void);
    fn ata_id_has_trim(id: *const u16) -> bool;
    fn ata_fpdma_dsm_supported(dev: *mut ata_device) -> bool;
    fn sata_spd_string(spd: c_int) -> *const c_char;
    fn ata_scsi_error();
    fn ata_scsi_user_scan();
    fn transport_setup_device(dev: *mut device);
    fn transport_add_device(dev: *mut device) -> c_int;
    fn transport_remove_device(dev: *mut device);
    fn transport_configure_device(dev: *mut device);
    fn transport_destroy_device(dev: *mut device);
    fn device_initialize(dev: *mut device);
    fn device_add(dev: *mut device) -> c_int;
    fn device_del(dev: *mut device);
    fn put_device(dev: *mut device);
    fn device_enable_async_suspend(dev: *mut device);
    fn pm_runtime_set_active(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_forbid(dev: *mut device);
    fn dev_set_name(dev: *mut device, fmt: *const c_char, ...);
    fn transport_class_register(class: *mut transport_class) -> c_int;
    fn transport_class_unregister(class: *mut transport_class);
    fn transport_container_register(cont: *mut transport_container);
    fn transport_container_unregister(cont: *mut transport_container);
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
}

#[repr(C)] pub struct device { pub parent: *mut device, pub release: Option<unsafe extern "C" fn(*mut device)>, pub typ: *const device_type }
#[repr(C)] pub struct device_type { pub name: *const c_char }
#[repr(C)] pub struct device_attribute;
#[repr(C)] pub struct attribute;
#[repr(C)] pub struct attribute_group { pub attrs_const: *const *const attribute }
#[repr(C)] pub struct attribute_container;
#[repr(C)] pub struct transport_class { _x: [u8;0] }
#[repr(C)] pub struct transport_container { pub ac: attribute_container }
#[repr(C)] pub struct scsi_transport_template { _x: [u8;0] }
#[repr(C)] pub struct ata_taskfile;
#[repr(C)] pub struct ata_ering_entry { pub timestamp: u64, pub err_mask: u32 }
#[repr(C)] pub struct ata_port { pub tdev: device, pub link: ata_link, pub host: *mut c_void, pub flags: u32, pub print_id: c_int, pub nr_pmp_links: c_int, pub stats: ata_stats }
#[repr(C)] pub struct ata_stats { pub idle_irq: c_ulong }
#[repr(C)] pub struct ata_link { pub tdev: device, pub ap: *mut ata_port, pub pmp: u32, pub device: [ata_device; 2] }
#[repr(C)] pub struct ata_device { pub tdev: device, pub link: *mut ata_link, pub devno: u32, pub class: u32, pub pio_mode: u32, pub dma_mode: u32, pub xfer_mode: u32, pub spdn_cnt: c_int, pub id: [u16; 256], pub gscr: [u32; 32], pub quirks: u32, pub ering: c_void }

unsafe extern "C" fn ata_tport_release(dev: *mut device) { let ap = (dev as *mut ata_port); ata_host_put((*ap).host); }
unsafe extern "C" fn ata_tdev_release(_: *mut device) {}
unsafe extern "C" fn ata_tlink_release(_: *mut device) {}

static mut ATA_PORT_SAS_TYPE: device_type = device_type { name: b"ata_port\0".as_ptr() as *const c_char };
static mut ATA_PORT_CLASS: transport_class = transport_class { _x: [] };
static mut ATA_DEV_CLASS: transport_class = transport_class { _x: [] };
static mut ATA_LINK_CLASS: transport_class = transport_class { _x: [] };
pub static mut ata_scsi_transportt: scsi_transport_template = scsi_transport_template { _x: [] };
static mut ATA_LINK_ATTR_CONT: transport_container = transport_container { ac: attribute_container { _x: [] } };
static mut ATA_DEV_ATTR_CONT: transport_container = transport_container { ac: attribute_container { _x: [] } };

unsafe fn tdev_to_port(d: *mut device) -> *mut ata_port { d as *mut ata_port }
unsafe fn tdev_to_link(d: *mut device) -> *mut ata_link { d as *mut ata_link }
unsafe fn tdev_to_device(d: *mut device) -> *mut ata_device { d as *mut ata_device }

#[no_mangle] pub unsafe extern "C" fn ata_tport_delete(ap: *mut ata_port) {
    ata_tlink_delete(&mut (*ap).link); let dev=&mut (*ap).tdev; transport_remove_device(dev); device_del(dev); transport_destroy_device(dev); put_device(dev);
}

#[no_mangle] pub unsafe extern "C" fn ata_tport_add(parent:*mut device, ap:*mut ata_port)->c_int {
    let dev=&mut (*ap).tdev; device_initialize(dev); dev.parent=parent; dev.release=Some(ata_tport_release); dev_set_name(dev,b"ata%d\0".as_ptr() as _,(*ap).print_id); ata_host_get((*ap).host); transport_setup_device(dev); ata_acpi_bind_port(ap);
    let mut e=device_add(dev); if e!=0 { transport_destroy_device(dev); put_device(dev); return e; } device_enable_async_suspend(dev); pm_runtime_set_active(dev); pm_runtime_enable(dev); pm_runtime_forbid(dev); e=transport_add_device(dev); if e!=0 { device_del(dev); transport_destroy_device(dev); put_device(dev); return e; } transport_configure_device(dev); e=ata_tlink_add(&mut (*ap).link); if e!=0 { transport_remove_device(dev); device_del(dev); transport_destroy_device(dev); put_device(dev); } e
}

#[no_mangle] pub unsafe extern "C" fn ata_port_classify(ap:*mut ata_port, tf:*const ata_taskfile)->c_uint { let class=ata_dev_classify(tf); ata_port_info(ap,b"found unknown device (class %u)\n\0".as_ptr() as _,class); class }

unsafe fn ata_tdev_free(dev:*mut ata_device){ transport_destroy_device(&mut (*dev).tdev); put_device(&mut (*dev).tdev); }
unsafe fn ata_tdev_delete(d:*mut ata_device){ let dev=&mut (*d).tdev; transport_remove_device(dev); device_del(dev); ata_tdev_free(d); }
unsafe fn ata_tdev_add(d:*mut ata_device)->c_int { let dev=&mut (*d).tdev; let link=(*d).link; device_initialize(dev); dev.parent=&mut (*link).tdev; dev.release=Some(ata_tdev_release); dev_set_name(dev,b"dev%d.%d\0".as_ptr() as _,(*(*link).ap).print_id,(*d).devno); transport_setup_device(dev); ata_acpi_bind_dev(d); let e=device_add(dev); if e!=0 { ata_tdev_free(d); return e } let e=transport_add_device(dev); if e!=0 { device_del(dev); ata_tdev_free(d); return e } transport_configure_device(dev); 0 }

#[no_mangle] pub unsafe extern "C" fn ata_tlink_delete(link:*mut ata_link){ for i in 0..2 { ata_tdev_delete(&mut (*link).device[i]); } let d=&mut (*link).tdev; transport_remove_device(d); device_del(d); transport_destroy_device(d); put_device(d); }
#[no_mangle] pub unsafe extern "C" fn ata_tlink_add(link:*mut ata_link)->c_int { let d=&mut (*link).tdev; device_initialize(d); d.parent=&mut (*(*link).ap).tdev; d.release=Some(ata_tlink_release); dev_set_name(d,b"link%d.%d\0".as_ptr() as _,(*(*link).ap).print_id,(*link).pmp); transport_setup_device(d); let mut e=device_add(d); if e!=0 { transport_destroy_device(d); put_device(d); return e } e=transport_add_device(d); if e!=0 { device_del(d); transport_destroy_device(d); put_device(d); return e } transport_configure_device(d); for i in 0..2 { e=ata_tdev_add(&mut (*link).device[i]); if e!=0 { for j in 0..i { ata_tdev_delete(&mut (*link).device[j]); } transport_remove_device(d); device_del(d); transport_destroy_device(d); put_device(d); return e } } 0 }

#[no_mangle] pub unsafe extern "C" fn libata_transport_init()->c_int { let mut e=transport_class_register(&mut ATA_LINK_CLASS); if e!=0{return e} e=transport_class_register(&mut ATA_PORT_CLASS); if e!=0 {transport_class_unregister(&mut ATA_LINK_CLASS);return e} e=transport_class_register(&mut ATA_DEV_CLASS); if e!=0 {transport_class_unregister(&mut ATA_PORT_CLASS);transport_class_unregister(&mut ATA_LINK_CLASS);return e} transport_container_register(&mut ATA_LINK_ATTR_CONT); transport_container_register(&mut ATA_DEV_ATTR_CONT); 0 }
#[no_mangle] pub unsafe extern "C" fn libata_transport_exit(){ transport_container_unregister(&mut ATA_LINK_ATTR_CONT); transport_container_unregister(&mut ATA_DEV_ATTR_CONT); transport_class_unregister(&mut ATA_LINK_CLASS); transport_class_unregister(&mut ATA_PORT_CLASS); transport_class_unregister(&mut ATA_DEV_CLASS); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
