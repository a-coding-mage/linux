/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Rust translation of ss.h.
 * C includes and header guards are intentionally omitted; referenced kernel
 * types are supplied by the surrounding translation unit.
 */

/* Definitions for card status flags for GetStatus */
pub const SS_WRPROT: u32 = 0x0001;
pub const SS_CARDLOCK: u32 = 0x0002;
pub const SS_EJECTION: u32 = 0x0004;
pub const SS_INSERTION: u32 = 0x0008;
pub const SS_BATDEAD: u32 = 0x0010;
pub const SS_BATWARN: u32 = 0x0020;
pub const SS_READY: u32 = 0x0040;
pub const SS_DETECT: u32 = 0x0080;
pub const SS_POWERON: u32 = 0x0100;
pub const SS_GPI: u32 = 0x0200;
pub const SS_STSCHG: u32 = 0x0400;
pub const SS_CARDBUS: u32 = 0x0800;
pub const SS_3VCARD: u32 = 0x1000;
pub const SS_XVCARD: u32 = 0x2000;
pub const SS_PENDING: u32 = 0x4000;
pub const SS_ZVCARD: u32 = 0x8000;

/* InquireSocket capabilities */
pub const SS_CAP_PAGE_REGS: u32 = 0x0001;
pub const SS_CAP_VIRTUAL_BUS: u32 = 0x0002;
pub const SS_CAP_MEM_ALIGN: u32 = 0x0004;
pub const SS_CAP_STATIC_MAP: u32 = 0x0008;
pub const SS_CAP_PCCARD: u32 = 0x4000;
pub const SS_CAP_CARDBUS: u32 = 0x8000;

#[repr(C)]
pub struct socket_state_t {
    pub flags: u32,
    pub csc_mask: u32,
    pub Vcc: u8,
    pub Vpp: u8,
    pub io_irq: u8,
}

extern "C" {
    pub static mut dead_socket: socket_state_t;
}

/* Socket configuration flags */
pub const SS_PWR_AUTO: u32 = 0x0010;
pub const SS_IOCARD: u32 = 0x0020;
pub const SS_RESET: u32 = 0x0040;
pub const SS_DMA_MODE: u32 = 0x0080;
pub const SS_SPKR_ENA: u32 = 0x0100;
pub const SS_OUTPUT_ENA: u32 = 0x0200;

/* Flags for I/O port and memory windows */
pub const MAP_ACTIVE: u8 = 0x01;
pub const MAP_16BIT: u8 = 0x02;
pub const MAP_AUTOSZ: u8 = 0x04;
pub const MAP_0WS: u8 = 0x08;
pub const MAP_WRPROT: u8 = 0x10;
pub const MAP_ATTRIB: u8 = 0x20;
pub const MAP_USE_WAIT: u8 = 0x40;
pub const MAP_PREFETCH: u8 = 0x80;
pub const MAP_IOSPACE: u8 = 0x20;

/* power hook operations */
pub const HOOK_POWER_PRE: u8 = 0x01;
pub const HOOK_POWER_POST: u8 = 0x02;

#[repr(C)]
pub struct pccard_io_map {
    pub map: u8,
    pub flags: u8,
    pub speed: u16,
    pub start: phys_addr_t,
    pub stop: phys_addr_t,
}

#[repr(C)]
pub struct pccard_mem_map {
    pub map: u8,
    pub flags: u8,
    pub speed: u16,
    pub static_start: phys_addr_t,
    pub card_start: u32,
    pub res: *mut resource,
}

#[repr(C)]
pub struct io_window_t {
    pub InUse: u32,
    pub Config: u32,
    pub res: *mut resource,
}

pub const MAX_IO_WIN: usize = 2;
pub const MAX_WIN: usize = 4;

pub struct pcmcia_socket;
pub struct pccard_resource_ops;
pub struct config_t;
pub struct pcmcia_callback;
pub struct user_info_t;

#[repr(C)]
pub struct pccard_operations {
    pub init: Option<unsafe extern "C" fn(*mut pcmcia_socket) -> i32>,
    pub suspend: Option<unsafe extern "C" fn(*mut pcmcia_socket) -> i32>,
    pub get_status: Option<unsafe extern "C" fn(*mut pcmcia_socket, *mut u32) -> i32>,
    pub set_socket: Option<unsafe extern "C" fn(*mut pcmcia_socket, *mut socket_state_t) -> i32>,
    pub set_io_map: Option<unsafe extern "C" fn(*mut pcmcia_socket, *mut pccard_io_map) -> i32>,
    pub set_mem_map: Option<unsafe extern "C" fn(*mut pcmcia_socket, *mut pccard_mem_map) -> i32>,
}

#[repr(C)]
pub struct pcmcia_socket {
    pub owner: *mut module,
    pub socket: socket_state_t,
    pub state: u32,
    pub suspended_state: u32,
    pub functions: u16,
    pub lock_count: u16,
    pub cis_mem: pccard_mem_map,
    pub cis_virt: *mut core::ffi::c_void,
    pub io: [io_window_t; MAX_IO_WIN],
    pub win: [pccard_mem_map; MAX_WIN],
    pub cis_cache: list_head,
    pub fake_cis_len: usize,
    pub fake_cis: *mut u8,
    pub socket_list: list_head,
    pub socket_released: completion,
    pub sock: u32,
    pub features: u32,
    pub irq_mask: u32,
    pub map_size: u32,
    pub io_offset: u32,
    pub pci_irq: u32,
    pub cb_dev: *mut pci_dev,
    pub resource_setup_done: u8,
    pub ops: *mut pccard_operations,
    pub resource_ops: *mut pccard_resource_ops,
    pub resource_data: *mut core::ffi::c_void,
    pub zoom_video: Option<unsafe extern "C" fn(*mut pcmcia_socket, i32)>,
    pub power_hook: Option<unsafe extern "C" fn(*mut pcmcia_socket, i32) -> i32>,
    /* CONFIG_CARDBUS */
    #[cfg(CONFIG_CARDBUS)]
    pub tune_bridge: Option<unsafe extern "C" fn(*mut pcmcia_socket, *mut pci_bus)>,
    pub thread: *mut task_struct,
    pub thread_done: completion,
    pub thread_events: u32,
    pub sysfs_events: u32,
    pub skt_mutex: mutex,
    pub ops_mutex: mutex,
    pub thread_lock: spinlock_t,
    pub callback: *mut pcmcia_callback,
    /* CONFIG_PCMCIA || CONFIG_PCMCIA_MODULE */
    #[cfg(any(CONFIG_PCMCIA, CONFIG_PCMCIA_MODULE))]
    pub devices_list: list_head,
    #[cfg(any(CONFIG_PCMCIA, CONFIG_PCMCIA_MODULE))]
    pub device_count: u8,
    #[cfg(any(CONFIG_PCMCIA, CONFIG_PCMCIA_MODULE))]
    pub pcmcia_pfc: u8,
    #[cfg(any(CONFIG_PCMCIA, CONFIG_PCMCIA_MODULE))]
    pub present: atomic_t,
    #[cfg(any(CONFIG_PCMCIA, CONFIG_PCMCIA_MODULE))]
    pub pcmcia_irq: u32,
    pub dev: device,
    pub driver_data: *mut core::ffi::c_void,
    pub resume_status: i32,
}

extern "C" {
    pub static mut pccard_static_ops: pccard_resource_ops;
    #[cfg(any(CONFIG_PCMCIA, CONFIG_PCMCIA_MODULE))]
    pub static mut pccard_nonstatic_ops: pccard_resource_ops;
}

extern "C" {
    pub fn pcmcia_parse_events(socket: *mut pcmcia_socket, events: u32);
    pub fn pcmcia_register_socket(socket: *mut pcmcia_socket) -> i32;
    pub fn pcmcia_unregister_socket(socket: *mut pcmcia_socket);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
