/* SPDX-License-Identifier: GPL-2.0 */
/* Freescale Management Complex (MC) bus private declarations */

/* Dependencies supplied by the surrounding kernel translation. */

pub const DPMNG_CMD_BASE_VERSION: u32 = 1;
pub const DPMNG_CMD_ID_OFFSET: u32 = 4;
pub const DPMNG_CMDID_GET_VERSION: u32 = (0x831u32 << DPMNG_CMD_ID_OFFSET) | DPMNG_CMD_BASE_VERSION;

#[repr(C)]
pub struct dpmng_rsp_get_version { pub revision: __le32, pub version_major: __le32, pub version_minor: __le32 }

pub const DPMCP_MIN_VER_MAJOR: u32 = 3;
pub const DPMCP_MIN_VER_MINOR: u32 = 0;
pub const DPMCP_CMD_BASE_VERSION: u32 = 1;
pub const DPMCP_CMD_ID_OFFSET: u32 = 4;
pub const DPMCP_CMDID_CLOSE: u32 = (0x800u32 << DPMCP_CMD_ID_OFFSET) | DPMCP_CMD_BASE_VERSION;
pub const DPMCP_CMDID_RESET: u32 = (0x005u32 << DPMCP_CMD_ID_OFFSET) | DPMCP_CMD_BASE_VERSION;

#[repr(C)] pub struct dpmcp_cmd_open { pub dpmcp_id: __le32 }

pub const DPRC_MIN_VER_MAJOR: u32 = 6;
pub const DPRC_MIN_VER_MINOR: u32 = 0;
pub const DPRC_CMD_BASE_VERSION: u32 = 1;
pub const DPRC_CMD_2ND_VERSION: u32 = 2;
pub const DPRC_CMD_3RD_VERSION: u32 = 3;
pub const DPRC_CMD_ID_OFFSET: u32 = 4;
pub const DPRC_CMDID_CLOSE: u32 = (0x800 << 4) | 1;
pub const DPRC_CMDID_GET_API_VERSION: u32 = (0xa05 << 4) | 1;
pub const DPRC_CMDID_GET_ATTR: u32 = (0x004 << 4) | 1;
pub const DPRC_CMDID_RESET_CONT: u32 = (0x005 << 4) | 1;
pub const DPRC_CMDID_RESET_CONT_V2: u32 = (0x005 << 4) | 2;
pub const DPRC_CMDID_SET_IRQ: u32 = (0x010 << 4) | 1;
pub const DPRC_CMDID_SET_IRQ_ENABLE: u32 = (0x012 << 4) | 1;
pub const DPRC_CMDID_SET_IRQ_MASK: u32 = (0x014 << 4) | 1;
pub const DPRC_CMDID_GET_IRQ_STATUS: u32 = (0x016 << 4) | 1;
pub const DPRC_CMDID_CLEAR_IRQ_STATUS: u32 = (0x017 << 4) | 1;
pub const DPRC_CMDID_GET_CONT_ID: u32 = (0x830 << 4) | 1;
pub const DPRC_CMDID_GET_OBJ_COUNT: u32 = (0x159 << 4) | 1;
pub const DPRC_CMDID_GET_OBJ: u32 = (0x15A << 4) | 1;
pub const DPRC_CMDID_GET_OBJ_REG: u32 = (0x15E << 4) | 1;
pub const DPRC_CMDID_GET_OBJ_REG_V2: u32 = (0x15E << 4) | 2;
pub const DPRC_CMDID_GET_OBJ_REG_V3: u32 = (0x15E << 4) | 3;
pub const DPRC_CMDID_SET_OBJ_IRQ: u32 = (0x15F << 4) | 1;
pub const DPRC_CMDID_GET_CONNECTION: u32 = (0x16C << 4) | 1;

#[repr(C)] pub struct dprc_cmd_open { pub container_id: __le32 }
#[repr(C)] pub struct dprc_cmd_reset_container { pub child_container_id: __le32, pub options: __le32 }
#[repr(C)] pub struct dprc_cmd_set_irq { pub irq_val: __le32, pub irq_index: u8, pub pad: [u8; 3], pub irq_addr: __le64, pub irq_num: __le32 }
pub const DPRC_ENABLE: u32 = 0x1;
#[repr(C)] pub struct dprc_cmd_set_irq_enable { pub enable: u8, pub pad: [u8; 3], pub irq_index: u8 }
#[repr(C)] pub struct dprc_cmd_set_irq_mask { pub mask: __le32, pub irq_index: u8 }
#[repr(C)] pub struct dprc_cmd_get_irq_status { pub status: __le32, pub irq_index: u8 }
#[repr(C)] pub struct dprc_rsp_get_irq_status { pub status: __le32 }
#[repr(C)] pub struct dprc_cmd_clear_irq_status { pub status: __le32, pub irq_index: u8 }
#[repr(C)] pub struct dprc_rsp_get_attributes { pub container_id: __le32, pub icid: __le32, pub options: __le32, pub portal_id: __le32 }
#[repr(C)] pub struct dprc_rsp_get_obj_count { pub pad: __le32, pub obj_count: __le32 }
#[repr(C)] pub struct dprc_cmd_get_obj { pub obj_index: __le32 }
#[repr(C)] pub struct dprc_rsp_get_obj { pub pad0: __le32, pub id: __le32, pub vendor: __le16, pub irq_count: u8, pub region_count: u8, pub state: __le32, pub version_major: __le16, pub version_minor: __le16, pub flags: __le16, pub pad1: __le16, pub obj_type: [u8; 16], pub label: [u8; 16] }
#[repr(C)] pub struct dprc_cmd_get_obj_region { pub obj_id: __le32, pub pad0: __le16, pub region_index: u8, pub pad1: u8, pub pad2: [__le64; 2], pub obj_type: [u8; 16] }
#[repr(C)] pub struct dprc_rsp_get_obj_region { pub pad0: __le64, pub base_offset: __le64, pub size: __le32, pub region_type: u8, pub pad2: [u8; 3], pub flags: __le32, pub pad3: __le32, pub base_addr: __le64 }
#[repr(C)] pub struct dprc_cmd_set_obj_irq { pub irq_val: __le32, pub irq_index: u8, pub pad: [u8; 3], pub irq_addr: __le64, pub irq_num: __le32, pub obj_id: __le32, pub obj_type: [u8; 16] }
#[repr(C)] pub struct dprc_cmd_get_connection { pub ep1_id: __le32, pub ep1_interface_id: __le16, pub pad: [u8; 2], pub ep1_type: [u8; 16] }
#[repr(C)] pub struct dprc_rsp_get_connection { pub pad: [__le64; 3], pub ep2_id: __le32, pub ep2_interface_id: __le16, pub pad1: __le16, pub ep2_type: [u8; 16], pub state: __le32 }

pub const DPRC_IRQ_EVENT_OBJ_ADDED: u32 = 0x00000001;
pub const DPRC_IRQ_EVENT_OBJ_REMOVED: u32 = 0x00000002;
pub const DPRC_IRQ_EVENT_CONTAINER_DESTROYED: u32 = 0x00000010;
pub const DPRC_IRQ_EVENT_OBJ_DESTROYED: u32 = 0x00000020;
pub const DPRC_IRQ_EVENT_OBJ_CREATED: u32 = 0x00000040;

#[repr(C)] pub struct dprc_irq_cfg { pub paddr: phys_addr_t, pub val: u32, pub irq_num: i32 }
#[repr(C)] pub struct dprc_attributes { pub container_id: i32, pub icid: u32, pub portal_id: i32, pub options: u64 }
#[repr(C)] pub enum dprc_region_type { DPRC_REGION_TYPE_MC_PORTAL, DPRC_REGION_TYPE_QBMAN_PORTAL, DPRC_REGION_TYPE_QBMAN_MEM_BACKED_PORTAL }
#[repr(C)] pub struct dprc_region_desc { pub base_offset: u32, pub size: u32, pub flags: u32, pub region_type: dprc_region_type, pub base_address: u64 }
#[repr(C)] pub struct dprc_endpoint { pub type_: [std::ffi::c_char; 16], pub id: i32, pub if_id: u16 }

pub const DPBP_VER_MAJOR: u32 = 3; pub const DPBP_VER_MINOR: u32 = 2;
pub const DPBP_CMD_BASE_VERSION: u32 = 1; pub const DPBP_CMD_ID_OFFSET: u32 = 4;
pub const DPBP_CMDID_CLOSE: u32 = (0x800 << 4) | 1; pub const DPBP_CMDID_ENABLE: u32 = (0x002 << 4) | 1; pub const DPBP_CMDID_DISABLE: u32 = (0x003 << 4) | 1; pub const DPBP_CMDID_GET_ATTR: u32 = (0x004 << 4) | 1; pub const DPBP_CMDID_RESET: u32 = (0x005 << 4) | 1;
#[repr(C)] pub struct dpbp_cmd_open { pub dpbp_id: __le32 }
pub const DPBP_ENABLE: u32 = 0x1;
#[repr(C)] pub struct dpbp_rsp_get_attributes { pub pad: __le16, pub bpid: __le16, pub id: __le32, pub version_major: __le16, pub version_minor: __le16 }

pub const DPCON_VER_MAJOR: u32 = 3; pub const DPCON_VER_MINOR: u32 = 2;
pub const DPCON_CMD_BASE_VERSION: u32 = 1; pub const DPCON_CMD_ID_OFFSET: u32 = 4;
pub const DPCON_CMDID_CLOSE: u32 = (0x800 << 4) | 1; pub const DPCON_CMDID_ENABLE: u32 = (0x002 << 4) | 1; pub const DPCON_CMDID_DISABLE: u32 = (0x003 << 4) | 1; pub const DPCON_CMDID_GET_ATTR: u32 = (0x004 << 4) | 1; pub const DPCON_CMDID_RESET: u32 = (0x005 << 4) | 1; pub const DPCON_CMDID_SET_NOTIFICATION: u32 = (0x100 << 4) | 1;
#[repr(C)] pub struct dpcon_cmd_open { pub dpcon_id: __le32 }
pub const DPCON_ENABLE: u32 = 1;
#[repr(C)] pub struct dpcon_rsp_get_attr { pub id: __le32, pub qbman_ch_id: __le16, pub num_priorities: u8, pub pad: u8 }
#[repr(C)] pub struct dpcon_cmd_set_notification { pub dpio_id: __le32, pub priority: u8, pub pad: [u8; 3], pub user_ctx: __le64 }

pub const OBJ_CMD_BASE_VERSION: u32 = 1; pub const OBJ_CMD_ID_OFFSET: u32 = 4;
pub const DPRTC_CMDID_OPEN: u32 = (0x810 << 4) | 1; pub const DPNI_CMDID_OPEN: u32 = (0x801 << 4) | 1; pub const DPSW_CMDID_OPEN: u32 = (0x802 << 4) | 1; pub const DPIO_CMDID_OPEN: u32 = (0x803 << 4) | 1; pub const DPBP_CMDID_OPEN: u32 = (0x804 << 4) | 1; pub const DPRC_CMDID_OPEN: u32 = (0x805 << 4) | 1; pub const DPDMUX_CMDID_OPEN: u32 = (0x806 << 4) | 1; pub const DPCI_CMDID_OPEN: u32 = (0x807 << 4) | 1; pub const DPCON_CMDID_OPEN: u32 = (0x808 << 4) | 1; pub const DPSECI_CMDID_OPEN: u32 = (0x809 << 4) | 1; pub const DPAIOP_CMDID_OPEN: u32 = (0x80a << 4) | 1; pub const DPMCP_CMDID_OPEN: u32 = (0x80b << 4) | 1; pub const DPMAC_CMDID_OPEN: u32 = (0x80c << 4) | 1; pub const DPDCEI_CMDID_OPEN: u32 = (0x80d << 4) | 1; pub const DPDMAI_CMDID_OPEN: u32 = (0x80e << 4) | 1; pub const DPDBG_CMDID_OPEN: u32 = (0x80f << 4) | 1;
pub const OBJ_CMDID_CLOSE: u32 = (0x800 << 4) | 1; pub const OBJ_CMDID_RESET: u32 = (0x005 << 4) | 1;
#[repr(C)] pub struct fsl_mc_obj_cmd_open { pub obj_id: __le32 }

#[repr(C)] pub struct fsl_mc_resource_pool { pub type_: fsl_mc_pool_type, pub max_count: i32, pub free_count: i32, pub mutex: mutex, pub free_list: list_head, pub mc_bus: *mut fsl_mc_bus }
#[repr(C)] pub struct fsl_mc_uapi { pub misc: miscdevice, pub device: *mut device, pub mutex: mutex, pub local_instance_in_use: u32, pub static_mc_io: *mut fsl_mc_io }
#[repr(C)] pub struct fsl_mc_bus { pub mc_dev: fsl_mc_device, pub resource_pools: [fsl_mc_resource_pool; FSL_MC_NUM_POOL_TYPES], pub irq_resources: *mut fsl_mc_device_irq, pub scan_mutex: mutex, pub dprc_attr: dprc_attributes, pub uapi_misc: fsl_mc_uapi, pub irq_enabled: i32 }

extern "C" {
    pub fn dpmcp_open(mc_io: *mut fsl_mc_io, cmd_flags: u32, dpmcp_id: i32, token: *mut u16) -> i32;
    pub fn dpmcp_close(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16) -> i32;
    pub fn dprc_open(mc_io: *mut fsl_mc_io, cmd_flags: u32, container_id: i32, token: *mut u16) -> i32;
    pub fn dprc_close(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16) -> i32;
    pub fn dprc_set_irq(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16, irq_index: u8, irq_cfg: *mut dprc_irq_cfg) -> i32;
    pub fn dprc_set_irq_enable(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16, irq_index: u8, en: u8) -> i32;
    pub fn dprc_set_irq_mask(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16, irq_index: u8, mask: u32) -> i32;
    pub fn dprc_get_irq_status(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16, irq_index: u8, status: *mut u32) -> i32;
    pub fn dprc_clear_irq_status(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16, irq_index: u8, status: u32) -> i32;
    pub fn dprc_get_attributes(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16, attributes: *mut dprc_attributes) -> i32;
    pub fn dprc_get_obj_count(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16, obj_count: *mut i32) -> i32;
    pub fn dprc_get_obj(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16, obj_index: i32, obj_desc: *mut fsl_mc_obj_desc) -> i32;
    pub fn dprc_set_obj_irq(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16, obj_type: *mut std::ffi::c_char, obj_id: i32, irq_index: u8, irq_cfg: *mut dprc_irq_cfg) -> i32;
    pub fn dprc_get_obj_region(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16, obj_type: *mut std::ffi::c_char, obj_id: i32, region_index: u8, region_desc: *mut dprc_region_desc) -> i32;
    pub fn dprc_get_api_version(mc_io: *mut fsl_mc_io, cmd_flags: u32, major_ver: *mut u16, minor_ver: *mut u16) -> i32;
    pub fn dprc_get_container_id(mc_io: *mut fsl_mc_io, cmd_flags: u32, container_id: *mut i32) -> i32;
    pub fn dprc_get_connection(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16, endpoint1: *const dprc_endpoint, endpoint2: *mut dprc_endpoint, state: *mut i32) -> i32;
    pub fn fsl_mc_device_add(obj_desc: *mut fsl_mc_obj_desc, mc_io: *mut fsl_mc_io, parent_dev: *mut device, new_mc_dev: *mut *mut fsl_mc_device) -> i32;
    pub fn fsl_mc_device_remove(mc_dev: *mut fsl_mc_device);
    pub fn dprc_driver_init() -> i32;
    pub fn dprc_driver_exit();
    pub fn dprc_scan_objects(mc_bus_dev: *mut fsl_mc_device, alloc_interrupts: bool) -> i32;
    pub fn fsl_mc_allocator_driver_init() -> i32;
    pub fn fsl_mc_init_all_resource_pools(mc_bus_dev: *mut fsl_mc_device);
    pub fn fsl_mc_resource_allocate(mc_bus: *mut fsl_mc_bus, pool_type: fsl_mc_pool_type, new_resource: *mut *mut fsl_mc_resource) -> i32;
    pub fn fsl_mc_resource_free(resource: *mut fsl_mc_resource);
    pub fn fsl_mc_msi_domain_alloc_irqs(dev: *mut device, irq_count: c_uint) -> i32;
    pub fn fsl_mc_msi_domain_free_irqs(dev: *mut device);
    pub fn fsl_mc_get_msi_parent(dev: *mut device) -> *mut irq_domain;
    pub fn fsl_create_mc_io(dev: *mut device, mc_portal_phys_addr: phys_addr_t, mc_portal_size: u32, dpmcp_dev: *mut fsl_mc_device, flags: u32, new_mc_io: *mut *mut fsl_mc_io) -> i32;
    pub fn fsl_destroy_mc_io(mc_io: *mut fsl_mc_io);
    pub fn fsl_mc_is_root_dprc(dev: *mut device) -> bool;
    pub fn fsl_mc_get_root_dprc(dev: *mut device, root_dprc_dev: *mut *mut device);
    pub fn fsl_mc_device_lookup(obj_desc: *mut fsl_mc_obj_desc, mc_bus_dev: *mut fsl_mc_device) -> *mut fsl_mc_device;
    pub fn mc_cmd_hdr_read_cmdid(cmd: *mut fsl_mc_command) -> u16;
    pub fn fsl_mc_uapi_create_device_file(mc_bus: *mut fsl_mc_bus) -> i32;
    pub fn fsl_mc_uapi_remove_device_file(mc_bus: *mut fsl_mc_bus);
    pub fn disable_dprc_irq(mc_dev: *mut fsl_mc_device) -> i32;
    pub fn enable_dprc_irq(mc_dev: *mut fsl_mc_device) -> i32;
    pub fn get_dprc_irq_state(mc_dev: *mut fsl_mc_device) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
