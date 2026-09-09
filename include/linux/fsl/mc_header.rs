/* SPDX-License-Identifier: GPL-2.0 */
/* Freescale Management Complex (MC) bus public interface */

// C includes are supplied by other translated headers.

pub const FSL_MC_VENDOR_FREESCALE: u32 = 0x1957;

#[repr(C)]
pub struct fsl_mc_driver {
    pub driver: device_driver,
    pub match_id_table: *const fsl_mc_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut fsl_mc_device) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut fsl_mc_device)>,
    pub shutdown: Option<unsafe extern "C" fn(*mut fsl_mc_device)>,
    pub suspend: Option<unsafe extern "C" fn(*mut fsl_mc_device, pm_message_t) -> i32>,
    pub resume: Option<unsafe extern "C" fn(*mut fsl_mc_device) -> i32>,
    pub driver_managed_dma: bool,
}

#[repr(i32)]
pub enum fsl_mc_pool_type {
    FSL_MC_POOL_DPMCP = 0x0,
    FSL_MC_POOL_DPBP,
    FSL_MC_POOL_DPCON,
    FSL_MC_POOL_IRQ,
    FSL_MC_NUM_POOL_TYPES,
}

#[repr(C)]
pub struct fsl_mc_resource {
    pub type_: fsl_mc_pool_type,
    pub id: i32,
    pub data: *mut core::ffi::c_void,
    pub parent_pool: *mut fsl_mc_resource_pool,
    pub node: list_head,
}

#[repr(C)]
pub struct fsl_mc_device_irq {
    pub virq: u32,
    pub mc_dev: *mut fsl_mc_device,
    pub dev_irq_index: u8,
    pub resource: fsl_mc_resource,
}

pub const FSL_MC_OBJ_STATE_OPEN: u32 = 0x00000001;
pub const FSL_MC_OBJ_STATE_PLUGGED: u32 = 0x00000002;
pub const FSL_MC_OBJ_FLAG_NO_MEM_SHAREABILITY: u16 = 0x0001;

#[repr(C)]
pub struct fsl_mc_obj_desc {
    pub type_: [i8; 16],
    pub id: i32,
    pub vendor: u16,
    pub ver_major: u16,
    pub ver_minor: u16,
    pub irq_count: u8,
    pub region_count: u8,
    pub state: u32,
    pub label: [i8; 16],
    pub flags: u16,
}

pub const FSL_MC_IS_DPRC: u16 = 0x0001;
pub const FSL_MC_REGION_CACHEABLE: u32 = 0x00000001;
pub const FSL_MC_REGION_SHAREABLE: u32 = 0x00000002;

#[repr(C)]
pub struct fsl_mc_device {
    pub dev: device,
    pub dma_mask: u64,
    pub flags: u16,
    pub icid: u32,
    pub mc_handle: u16,
    pub mc_io: *mut fsl_mc_io,
    pub obj_desc: fsl_mc_obj_desc,
    pub regions: *mut resource,
    pub irqs: *mut *mut fsl_mc_device_irq,
    pub resource: *mut fsl_mc_resource,
    pub consumer_link: *mut device_link,
}

#[repr(C)]
pub struct mc_cmd_header {
    pub src_id: u8,
    pub flags_hw: u8,
    pub status: u8,
    pub flags_sw: u8,
    pub token: u16,
    pub cmd_id: u16,
}

#[repr(u32)]
pub enum mc_cmd_status {
    MC_CMD_STATUS_OK = 0x0,
    MC_CMD_STATUS_READY = 0x1,
    MC_CMD_STATUS_AUTH_ERR = 0x3,
    MC_CMD_STATUS_NO_PRIVILEGE = 0x4,
    MC_CMD_STATUS_DMA_ERR = 0x5,
    MC_CMD_STATUS_CONFIG_ERR = 0x6,
    MC_CMD_STATUS_TIMEOUT = 0x7,
    MC_CMD_STATUS_NO_RESOURCE = 0x8,
    MC_CMD_STATUS_NO_MEMORY = 0x9,
    MC_CMD_STATUS_BUSY = 0xA,
    MC_CMD_STATUS_UNSUPPORTED_OP = 0xB,
    MC_CMD_STATUS_INVALID_STATE = 0xC,
}

pub const MC_CMD_FLAG_PRI: u32 = 0x80;
pub const MC_CMD_FLAG_INTR_DIS: u32 = 0x01;

#[inline]
pub unsafe fn mc_encode_cmd_header(cmd_id: u16, cmd_flags: u32, token: u16) -> u64 {
    let mut header: u64 = 0;
    let hdr = &mut *(&mut header as *mut u64 as *mut mc_cmd_header);
    hdr.cmd_id = cmd_id.to_le();
    hdr.token = token.to_le();
    hdr.status = MC_CMD_STATUS_READY as u8;
    if cmd_flags & MC_CMD_FLAG_PRI != 0 { hdr.flags_hw = MC_CMD_FLAG_PRI as u8; }
    if cmd_flags & MC_CMD_FLAG_INTR_DIS != 0 { hdr.flags_sw = MC_CMD_FLAG_INTR_DIS as u8; }
    header
}

#[inline]
pub unsafe fn mc_cmd_hdr_read_token(cmd: *mut fsl_mc_command) -> u16 {
    let hdr = &*((*cmd).header.as_ptr() as *const mc_cmd_header);
    u16::from_le(hdr.token)
}

#[repr(C)] pub struct mc_rsp_create { pub object_id: u32 }
#[repr(C)] pub struct mc_rsp_api_ver { pub major_ver: u16, pub minor_ver: u16 }

#[inline]
pub unsafe fn mc_cmd_read_object_id(cmd: *mut fsl_mc_command) -> u32 {
    u32::from_le(*( (*cmd).params.as_ptr() as *const u32))
}

#[inline]
pub unsafe fn mc_cmd_read_api_version(cmd: *mut fsl_mc_command, major_ver: *mut u16, minor_ver: *mut u16) {
    let rsp = &*((*cmd).params.as_ptr() as *const mc_rsp_api_ver);
    *major_ver = u16::from_le(rsp.major_ver);
    *minor_ver = u16::from_le(rsp.minor_ver);
}

pub const FSL_MC_IO_ATOMIC_CONTEXT_PORTAL: u16 = 0x0001;

#[repr(C)]
pub union fsl_mc_io_lock { pub mutex: mutex, pub spinlock: raw_spinlock_t }
#[repr(C)]
pub struct fsl_mc_io {
    pub dev: *mut device,
    pub flags: u16,
    pub portal_size: u32,
    pub portal_phys_addr: phys_addr_t,
    pub portal_virt_addr: *mut core::ffi::c_void,
    pub dpmcp_dev: *mut fsl_mc_device,
    pub lock: fsl_mc_io_lock,
}

extern "C" {
    pub fn mc_send_command(mc_io: *mut fsl_mc_io, cmd: *mut fsl_mc_command) -> i32;
    pub fn __fsl_mc_driver_register(fsl_mc_driver: *mut fsl_mc_driver, owner: *mut module) -> i32;
    pub fn fsl_mc_driver_unregister(driver: *mut fsl_mc_driver);
}

pub const FSL_MC_IRQ_POOL_MAX_TOTAL_IRQS: u32 = 256;
pub const DPRC_RESET_OPTION_NON_RECURSIVE: u32 = 0x00000001;
pub const DPCON_INVALID_DPIO_ID: i32 = -1;

#[repr(C)] pub struct fsl_mc_version { pub major: u32, pub minor: u32, pub revision: u32 }
#[repr(C)] pub struct dpbp_attr { pub id: i32, pub bpid: u16 }
#[repr(C)] pub struct dpcon_attr { pub id: i32, pub qbman_ch_id: u16, pub num_priorities: u8 }
#[repr(C)] pub struct dpcon_notification_cfg { pub dpio_id: i32, pub priority: u8, pub user_ctx: u64 }

extern "C" {
    pub fn fsl_mc_get_msi_id(dev: *mut device) -> u32;
    pub fn fsl_mc_portal_allocate(mc_dev: *mut fsl_mc_device, mc_io_flags: u16, new_mc_io: *mut *mut fsl_mc_io) -> i32;
    pub fn fsl_mc_portal_free(mc_io: *mut fsl_mc_io);
    pub fn fsl_mc_object_allocate(mc_dev: *mut fsl_mc_device, pool_type: fsl_mc_pool_type, new_mc_adev: *mut *mut fsl_mc_device) -> i32;
    pub fn fsl_mc_object_free(mc_adev: *mut fsl_mc_device);
    pub fn fsl_mc_allocate_irqs(mc_dev: *mut fsl_mc_device) -> i32;
    pub fn fsl_mc_free_irqs(mc_dev: *mut fsl_mc_device);
    pub fn fsl_mc_get_endpoint(mc_dev: *mut fsl_mc_device, if_id: u16) -> *mut fsl_mc_device;
    pub fn dprc_reset_container(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16, child_container_id: i32, options: u32) -> i32;
    pub fn dprc_scan_container(mc_bus_dev: *mut fsl_mc_device, alloc_interrupts: bool) -> i32;
    pub fn dprc_remove_devices(mc_bus_dev: *mut fsl_mc_device, obj_desc_array: *mut fsl_mc_obj_desc, num_child_objects_in_mc: i32);
    pub fn dprc_cleanup(mc_dev: *mut fsl_mc_device) -> i32;
    pub fn dprc_setup(mc_dev: *mut fsl_mc_device) -> i32;
    pub fn fsl_mc_populate_irq_pool(mc_bus_dev: *mut fsl_mc_device, irq_count: u32) -> i32;
    pub fn fsl_mc_cleanup_irq_pool(mc_bus_dev: *mut fsl_mc_device);
    pub fn dpbp_open(mc_io: *mut fsl_mc_io, cmd_flags: u32, dpbp_id: i32, token: *mut u16) -> i32;
    pub fn dpbp_close(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16) -> i32;
    pub fn dpbp_enable(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16) -> i32;
    pub fn dpbp_disable(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16) -> i32;
    pub fn dpbp_reset(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16) -> i32;
    pub fn dpbp_get_attributes(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16, attr: *mut dpbp_attr) -> i32;
    pub fn dpcon_open(mc_io: *mut fsl_mc_io, cmd_flags: u32, dpcon_id: i32, token: *mut u16) -> i32;
    pub fn dpcon_close(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16) -> i32;
    pub fn dpcon_enable(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16) -> i32;
    pub fn dpcon_disable(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16) -> i32;
    pub fn dpcon_reset(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16) -> i32;
    pub fn fsl_mc_obj_open(mc_io: *mut fsl_mc_io, cmd_flags: u32, obj_id: i32, obj_type: *mut i8, token: *mut u16) -> i32;
    pub fn fsl_mc_obj_close(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16) -> i32;
    pub fn fsl_mc_obj_reset(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16) -> i32;
    pub fn dpcon_get_attributes(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16, attr: *mut dpcon_attr) -> i32;
    pub fn dpcon_set_notification(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16, cfg: *mut dpcon_notification_cfg) -> i32;
    pub fn fsl_mc_get_version() -> *mut fsl_mc_version;
}

// Device-type objects and bus symbols are declared by the translated device-model headers.
extern "C" {
    pub static fsl_mc_bus_type: bus_type;
    pub static fsl_mc_bus_dprc_type: device_type;
    pub static fsl_mc_bus_dpni_type: device_type;
    pub static fsl_mc_bus_dpio_type: device_type;
    pub static fsl_mc_bus_dpsw_type: device_type;
    pub static fsl_mc_bus_dpbp_type: device_type;
    pub static fsl_mc_bus_dpcon_type: device_type;
    pub static fsl_mc_bus_dpmcp_type: device_type;
    pub static fsl_mc_bus_dpmac_type: device_type;
    pub static fsl_mc_bus_dprtc_type: device_type;
    pub static fsl_mc_bus_dpseci_type: device_type;
    pub static fsl_mc_bus_dpdmux_type: device_type;
    pub static fsl_mc_bus_dpdcei_type: device_type;
    pub static fsl_mc_bus_dpaiop_type: device_type;
    pub static fsl_mc_bus_dpci_type: device_type;
    pub static fsl_mc_bus_dpdmai_type: device_type;
}

#[inline] pub unsafe fn is_fsl_mc_bus_dprc(d: *const fsl_mc_device) -> bool { (*d).dev.type_ == &fsl_mc_bus_dprc_type }
#[inline] pub unsafe fn is_fsl_mc_bus_dpni(d: *const fsl_mc_device) -> bool { (*d).dev.type_ == &fsl_mc_bus_dpni_type }
#[inline] pub unsafe fn is_fsl_mc_bus_dpio(d: *const fsl_mc_device) -> bool { (*d).dev.type_ == &fsl_mc_bus_dpio_type }
#[inline] pub unsafe fn is_fsl_mc_bus_dpsw(d: *const fsl_mc_device) -> bool { (*d).dev.type_ == &fsl_mc_bus_dpsw_type }
#[inline] pub unsafe fn is_fsl_mc_bus_dpdmux(d: *const fsl_mc_device) -> bool { (*d).dev.type_ == &fsl_mc_bus_dpdmux_type }
#[inline] pub unsafe fn is_fsl_mc_bus_dpbp(d: *const fsl_mc_device) -> bool { (*d).dev.type_ == &fsl_mc_bus_dpbp_type }
#[inline] pub unsafe fn is_fsl_mc_bus_dpcon(d: *const fsl_mc_device) -> bool { (*d).dev.type_ == &fsl_mc_bus_dpcon_type }
#[inline] pub unsafe fn is_fsl_mc_bus_dpmcp(d: *const fsl_mc_device) -> bool { (*d).dev.type_ == &fsl_mc_bus_dpmcp_type }
#[inline] pub unsafe fn is_fsl_mc_bus_dpmac(d: *const fsl_mc_device) -> bool { (*d).dev.type_ == &fsl_mc_bus_dpmac_type }
#[inline] pub unsafe fn is_fsl_mc_bus_dprtc(d: *const fsl_mc_device) -> bool { (*d).dev.type_ == &fsl_mc_bus_dprtc_type }
#[inline] pub unsafe fn is_fsl_mc_bus_dpseci(d: *const fsl_mc_device) -> bool { (*d).dev.type_ == &fsl_mc_bus_dpseci_type }
#[inline] pub unsafe fn is_fsl_mc_bus_dpdcei(d: *const fsl_mc_device) -> bool { (*d).dev.type_ == &fsl_mc_bus_dpdcei_type }
#[inline] pub unsafe fn is_fsl_mc_bus_dpaiop(d: *const fsl_mc_device) -> bool { (*d).dev.type_ == &fsl_mc_bus_dpaiop_type }
#[inline] pub unsafe fn is_fsl_mc_bus_dpci(d: *const fsl_mc_device) -> bool { (*d).dev.type_ == &fsl_mc_bus_dpci_type }
#[inline] pub unsafe fn is_fsl_mc_bus_dpdmai(d: *const fsl_mc_device) -> bool { (*d).dev.type_ == &fsl_mc_bus_dpdmai_type }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
