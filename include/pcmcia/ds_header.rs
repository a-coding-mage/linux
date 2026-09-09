/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * ds.h -- 16-bit PCMCIA core support
 *
 * The initial developer of the original code is David A. Hinds
 * <dahinds@users.sourceforge.net>.  Portions created by David A. Hinds
 * are Copyright (C) 1999 David A. Hinds.  All Rights Reserved.
 *
 * (C) 1999        David A. Hinds
 * (C) 2003 - 2008 Dominik Brodowski
 */

/* C header dependencies are supplied by the surrounding kernel translation. */

/* PCMCIA device drivers (16-bit cards only; 32-bit cards require CardBus). */
#[repr(C)] pub struct pcmcia_socket { _private: [u8; 0] }
#[repr(C)] pub struct config_t { _private: [u8; 0] }
#[repr(C)] pub struct net_device { _private: [u8; 0] }

#[repr(C)]
pub struct pcmcia_dynids {
    pub lock: mutex,
    pub list: list_head,
}

#[repr(C)]
pub struct pcmcia_driver {
    pub name: *const core::ffi::c_char,
    pub probe: Option<unsafe extern "C" fn(*mut pcmcia_device) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut pcmcia_device)>,
    pub suspend: Option<unsafe extern "C" fn(*mut pcmcia_device) -> i32>,
    pub resume: Option<unsafe extern "C" fn(*mut pcmcia_device) -> i32>,
    pub owner: *mut module,
    pub id_table: *const pcmcia_device_id,
    pub drv: device_driver,
    pub dynids: pcmcia_dynids,
}

extern "C" {
    pub fn pcmcia_register_driver(driver: *mut pcmcia_driver) -> i32;
    pub fn pcmcia_unregister_driver(driver: *mut pcmcia_driver);
}

/* module_pcmcia_driver(__pcmcia_driver) expands to module_driver registration. */

pub const PCMCIA_IOPORT_0: usize = 0;
pub const PCMCIA_IOPORT_1: usize = 1;
pub const PCMCIA_IOMEM_0: usize = 2;
pub const PCMCIA_IOMEM_1: usize = 3;
pub const PCMCIA_IOMEM_2: usize = 4;
pub const PCMCIA_IOMEM_3: usize = 5;
pub const PCMCIA_NUM_RESOURCES: usize = 6;

#[repr(C)]
pub struct pcmcia_device {
    pub socket: *mut pcmcia_socket,
    pub devname: *mut core::ffi::c_char,
    pub device_no: u8,
    pub func: u8,
    pub function_config: *mut config_t,
    pub socket_device_list: list_head,
    pub irq: u32,
    pub resource: [*mut resource; PCMCIA_NUM_RESOURCES],
    pub card_addr: resource_size_t,
    pub vpp: u32,
    pub config_flags: u32,
    pub config_base: u32,
    pub config_index: u32,
    pub config_regs: u32,
    pub io_lines: u32,
    /* C bit-fields are represented by their underlying storage unit. */
    pub suspended: u16,
    pub _irq: u16,
    pub _io: u16,
    pub _win: u16,
    pub _locked: u16,
    pub allow_func_id_match: u16,
    pub has_manf_id: u16,
    pub has_card_id: u16,
    pub has_func_id: u16,
    pub reserved: u16,
    pub func_id: u8,
    pub manf_id: u16,
    pub card_id: u16,
    pub prod_id: [*mut core::ffi::c_char; 4],
    pub dma_mask: u64,
    pub dev: device,
    pub priv_: *mut core::ffi::c_void,
    pub open: u32,
}

/* to_pcmcia_dev and to_pcmcia_drv use the kernel container_of macro. */

extern "C" {
    pub fn pcmcia_get_tuple(p_dev: *mut pcmcia_device, code: cisdata_t, buf: *mut *mut u8) -> usize;
    pub fn pcmcia_loop_tuple(p_dev: *mut pcmcia_device, code: cisdata_t,
        loop_tuple: Option<unsafe extern "C" fn(*mut pcmcia_device, *mut tuple_t, *mut core::ffi::c_void) -> i32>,
        priv_data: *mut core::ffi::c_void) -> i32;
    pub fn pcmcia_get_mac_from_cis(p_dev: *mut pcmcia_device, dev: *mut net_device) -> i32;
    pub fn pcmcia_parse_tuple(tuple: *mut tuple_t, parse: *mut cisparse_t) -> i32;
    pub fn pcmcia_loop_config(p_dev: *mut pcmcia_device,
        conf_check: Option<unsafe extern "C" fn(*mut pcmcia_device, *mut core::ffi::c_void) -> i32>,
        priv_data: *mut core::ffi::c_void) -> i32;
    pub fn pcmcia_dev_present(p_dev: *mut pcmcia_device) -> *mut pcmcia_device;
    pub fn pcmcia_reset_card(skt: *mut pcmcia_socket) -> i32;
    pub fn pcmcia_read_config_byte(p_dev: *mut pcmcia_device, where_: off_t, val: *mut u8) -> i32;
    pub fn pcmcia_write_config_byte(p_dev: *mut pcmcia_device, where_: off_t, val: u8) -> i32;
    pub fn pcmcia_request_io(p_dev: *mut pcmcia_device) -> i32;
    pub fn pcmcia_request_irq(p_dev: *mut pcmcia_device, handler: irq_handler_t) -> i32;
    pub fn pcmcia_enable_device(p_dev: *mut pcmcia_device) -> i32;
    pub fn pcmcia_request_window(p_dev: *mut pcmcia_device, res: *mut resource, speed: u32) -> i32;
    pub fn pcmcia_release_window(p_dev: *mut pcmcia_device, res: *mut resource) -> i32;
    pub fn pcmcia_map_mem_page(p_dev: *mut pcmcia_device, res: *mut resource, offset: u32) -> i32;
    pub fn pcmcia_fixup_vpp(p_dev: *mut pcmcia_device, new_vpp: u8) -> i32;
    pub fn pcmcia_fixup_iowidth(p_dev: *mut pcmcia_device) -> i32;
    pub fn pcmcia_disable_device(p_dev: *mut pcmcia_device);
}

pub const IO_DATA_PATH_WIDTH: u32 = 0x18;
pub const IO_DATA_PATH_WIDTH_8: u32 = 0x00;
pub const IO_DATA_PATH_WIDTH_16: u32 = 0x08;
pub const IO_DATA_PATH_WIDTH_AUTO: u32 = 0x10;
pub const WIN_MEMORY_TYPE_CM: u32 = 0x00;
pub const WIN_MEMORY_TYPE_AM: u32 = 0x20;
pub const WIN_DATA_WIDTH_8: u32 = 0x00;
pub const WIN_DATA_WIDTH_16: u32 = 0x02;
pub const WIN_ENABLE: u32 = 0x01;
pub const WIN_USE_WAIT: u32 = 0x40;
pub const WIN_FLAGS_MAP: u32 = 0x63;
pub const WIN_FLAGS_REQ: u32 = 0x1c;

pub const PRESENT_OPTION: u32 = 0x001;
pub const PRESENT_STATUS: u32 = 0x002;
pub const PRESENT_PIN_REPLACE: u32 = 0x004;
pub const PRESENT_COPY: u32 = 0x008;
pub const PRESENT_EXT_STATUS: u32 = 0x010;
pub const PRESENT_IOBASE_0: u32 = 0x020;
pub const PRESENT_IOBASE_1: u32 = 0x040;
pub const PRESENT_IOBASE_2: u32 = 0x080;
pub const PRESENT_IOBASE_3: u32 = 0x100;
pub const PRESENT_IOSIZE: u32 = 0x200;

pub const CONF_ENABLE_IRQ: u32 = 0x0001;
pub const CONF_ENABLE_SPKR: u32 = 0x0002;
pub const CONF_ENABLE_PULSE_IRQ: u32 = 0x0004;
pub const CONF_ENABLE_ESR: u32 = 0x0008;
pub const CONF_ENABLE_IOCARD: u32 = 0x0010;
pub const CONF_ENABLE_ZVCARD: u32 = 0x0020;
pub const CONF_AUTO_CHECK_VCC: u32 = 0x0100;
pub const CONF_AUTO_SET_VPP: u32 = 0x0200;
pub const CONF_AUTO_AUDIO: u32 = 0x0400;
pub const CONF_AUTO_SET_IO: u32 = 0x0800;
pub const CONF_AUTO_SET_IOMEM: u32 = 0x1000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
