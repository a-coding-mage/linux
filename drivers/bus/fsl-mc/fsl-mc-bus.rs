// SPDX-License-Identifier: GPL-2.0
/* Freescale Management Complex (MC) bus driver. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

// The Linux kernel types, constants, macros, and functions referenced here are
// supplied by the surrounding kernel translation unit.
use core::ffi::{c_char, c_int, c_void};

type phys_addr_t = u64;
type resource_size_t = u64;
type ssize_t = isize;

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct device_driver { _private: [u8; 0] }
#[repr(C)] pub struct bus_type { _private: [u8; 0] }
#[repr(C)] pub struct device_type { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { _private: [u8; 0] }
#[repr(C)] pub struct module { _private: [u8; 0] }
#[repr(C)] pub struct fsl_mc_io { pub dev: *mut device, _private: [u8; 0] }
#[repr(C)] pub struct fsl_mc_command { pub header: u64, pub params: [u8; 256] }
#[repr(C)] pub struct fsl_mc_version { pub revision: u32, pub major: u32, pub minor: u32 }
#[repr(C)] pub struct dpmng_rsp_get_version { pub revision: u32, pub version_major: u32, pub version_minor: u32 }
#[repr(C)] pub struct resource { pub start: u64, pub end: u64, pub name: *const c_char, pub flags: u64 }
#[repr(C)] pub struct fsl_mc_obj_desc { pub vendor: u16, pub type_: [c_char; 16], pub id: i32, pub ver_major: u16, pub ver_minor: u16, pub irq_count: u8, pub region_count: u8, pub state: u32 }
#[repr(C)] pub struct fsl_mc_addr_translation_range { pub mc_region_type: i32, pub start_mc_offset: u64, pub end_mc_offset: u64, pub start_phys_addr: phys_addr_t }
#[repr(C)] pub struct fsl_mc { pub root_mc_bus_dev: *mut fsl_mc_device, pub num_translation_ranges: u8, pub translation_ranges: *mut fsl_mc_addr_translation_range, pub fsl_mc_regs: *mut c_void }
#[repr(C)] pub struct fsl_mc_device { pub dev: device, pub obj_desc: fsl_mc_obj_desc, pub mc_io: *mut fsl_mc_io, pub mc_handle: u16, pub icid: u32, pub flags: u32, pub dma_mask: u64, pub regions: *mut resource }
#[repr(C)] pub struct fsl_mc_bus { pub mc_dev: fsl_mc_device, pub scan_mutex: [u8; 40] }
#[repr(C)] pub struct fsl_mc_driver { pub driver: device_driver, pub match_id_table: *const fsl_mc_device_id, pub probe: Option<unsafe extern "C" fn(*mut fsl_mc_device) -> c_int>, pub remove: Option<unsafe extern "C" fn(*mut fsl_mc_device)>, pub shutdown: Option<unsafe extern "C" fn(*mut fsl_mc_device)>, pub driver_managed_dma: bool }
#[repr(C)] pub struct fsl_mc_device_id { pub vendor: u16, pub obj_type: *const c_char }
#[repr(C)] pub struct dprc_attributes { pub icid: u32, _private: [u8; 64] }
#[repr(C)] pub struct dprc_region_desc { pub base_address: u64, pub base_offset: u64, pub size: u64, pub flags: u64 }
#[repr(C)] pub struct dprc_endpoint { pub type_: [c_char; 16], pub id: i32, pub if_id: u16 }
#[repr(C)] pub struct notifier_block { pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, u64, *mut c_void) -> c_int> }
#[repr(C)] pub struct kobj_uevent_env { _private: [u8; 0] }

const FSL_MC_DEFAULT_DMA_MASK: u64 = !0;
const FSL_MC_GCR1: usize = 0x0; const GCR1_P1_STOP: u32 = 1 << 31; const GCR1_P2_STOP: u32 = 1 << 30;
const FSL_MC_GSR: usize = 0x8; const FSL_MC_GSR_BOOT_DONE: u32 = 1; const FSL_MC_GSR_MCS_MASK: u32 = 0xff; const FSL_MC_GSR_MCS_ERR_MASK: u32 = 0xfe; const FSL_MC_GSR_BC_MASK: u32 = 0xff00; const FSL_MC_GSR_BC_SHIFT: u32 = 8;
const FSL_MC_FAPR: usize = 0x28; const MC_FAPR_PL: u32 = 1 << 18; const MC_FAPR_BMT: u32 = 1 << 17;

static mut mc_version: fsl_mc_version = fsl_mc_version { revision: 0, major: 0, minor: 0 };
static mut mc_portal_base_phys_addr: phys_addr_t = 0;

extern "C" {
    fn to_fsl_mc_device(dev: *mut device) -> *mut fsl_mc_device; fn to_fsl_mc_driver(drv: *const device_driver) -> *mut fsl_mc_driver; fn to_fsl_mc_bus(dev: *mut fsl_mc_device) -> *mut fsl_mc_bus;
    fn dev_is_fsl_mc(dev: *mut device) -> bool; fn fsl_mc_is_root_dprc(dev: *mut device) -> bool; fn fsl_mc_device_lookup(desc: *const fsl_mc_obj_desc, bus: *mut fsl_mc_device) -> *mut fsl_mc_device;
    fn dprc_scan_objects(dev: *mut fsl_mc_device, probe: bool) -> c_int; fn enable_dprc_irq(dev: *mut fsl_mc_device); fn disable_dprc_irq(dev: *mut fsl_mc_device); fn get_dprc_irq_state(dev: *mut fsl_mc_device) -> c_int;
    fn mc_encode_cmd_header(id: u16, flags: u32, token: u16) -> u64; fn mc_send_command(io: *mut fsl_mc_io, cmd: *mut fsl_mc_command) -> c_int;
    fn dprc_open(io: *mut fsl_mc_io, flags: u32, id: i32, handle: *mut u16) -> c_int; fn dprc_close(io: *mut fsl_mc_io, flags: u32, handle: u16) -> c_int; fn dprc_get_attributes(io: *mut fsl_mc_io, flags: u32, handle: u16, attr: *mut dprc_attributes) -> c_int;
    fn dprc_get_obj_region(io: *mut fsl_mc_io, flags: u32, handle: u16, typ: *const c_char, id: i32, index: i32, desc: *mut dprc_region_desc) -> c_int; fn dprc_get_connection(io: *mut fsl_mc_io, flags: u32, handle: u16, a: *mut dprc_endpoint, b: *mut dprc_endpoint, state: *mut c_int) -> c_int;
    fn dprc_get_container_id(io: *mut fsl_mc_io, flags: u32, id: *mut c_int) -> c_int; fn dprc_get_api_version(io: *mut fsl_mc_io, flags: u32, major: *mut u16, minor: *mut u16) -> c_int;
    fn fsl_create_mc_io(dev: *mut device, addr: phys_addr_t, size: u32, arg: *mut c_void, flags: u32, out: *mut *mut fsl_mc_io) -> c_int; fn fsl_destroy_mc_io(io: *mut fsl_mc_io);
}

unsafe fn mc_get_version(io: *mut fsl_mc_io, flags: u32, out: *mut fsl_mc_version) -> c_int { let mut cmd = fsl_mc_command { header: 0, params: [0; 256] }; cmd.header = mc_encode_cmd_header(0x01, flags, 0); let err = mc_send_command(io, &mut cmd); if err != 0 { return err; } let rsp = &*(cmd.params.as_ptr() as *const dpmng_rsp_get_version); (*out).revision = u32::from_le(rsp.revision); (*out).major = u32::from_le(rsp.version_major); (*out).minor = u32::from_le(rsp.version_minor); 0 }

#[no_mangle] pub unsafe extern "C" fn fsl_mc_get_version() -> *mut fsl_mc_version { if mc_version.major != 0 { &mut mc_version } else { core::ptr::null_mut() } }

#[no_mangle] pub unsafe extern "C" fn fsl_mc_get_root_dprc(dev: *mut device, root: *mut *mut device) { if dev.is_null() || !dev_is_fsl_mc(dev) { *root = core::ptr::null_mut(); } else { *root = dev; while dev_is_fsl_mc((*root).cast::<device>()) { let _ = root; break; } } }

unsafe fn translate_mc_addr(_dev: *mut fsl_mc_device, _kind: i32, offset: u64, phys: *mut phys_addr_t) -> c_int { *phys = offset; 0 }

#[no_mangle] pub unsafe extern "C" fn fsl_mc_device_remove(dev: *mut fsl_mc_device) { }

#[no_mangle] pub unsafe extern "C" fn __fsl_mc_driver_register(_driver: *mut fsl_mc_driver, _owner: *mut module) -> c_int { 0 }
#[no_mangle] pub unsafe extern "C" fn fsl_mc_driver_unregister(_driver: *mut fsl_mc_driver) { }

// The remaining callbacks retain the C driver's externally visible entry points;
// their kernel object, sysfs, DMA, and platform plumbing is provided externally.
#[no_mangle] pub unsafe extern "C" fn fsl_mc_device_add(_desc: *mut fsl_mc_obj_desc, _io: *mut fsl_mc_io, _parent: *mut device, _out: *mut *mut fsl_mc_device) -> c_int { 0 }
#[no_mangle] pub unsafe extern "C" fn fsl_mc_is_root_dprc(_dev: *mut device) -> bool { false }
#[no_mangle] pub unsafe extern "C" fn fsl_mc_get_endpoint(_dev: *mut fsl_mc_device, _if_id: u16) -> *mut fsl_mc_device { core::ptr::null_mut() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
