/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * RapidIO interconnect services
 * (RapidIO Interconnect Specification, http://www.rapidio.org)
 *
 * Copyright 2005 MontaVista Software, Inc.
 * Matt Porter <mporter@kernel.crashing.org>
 */

pub const RIO_NO_HOPCOUNT: i32 = -1;
pub const RIO_INVALID_DESTID: u16 = 0xffff;
pub const RIO_MAX_MPORTS: usize = 8;
pub const RIO_MAX_MPORT_RESOURCES: usize = 16;
pub const RIO_MAX_DEV_RESOURCES: usize = 16;
pub const RIO_MAX_MPORT_NAME: usize = 40;
pub const RIO_GLOBAL_TABLE: u8 = 0xff;
pub const RIO_INVALID_ROUTE: u8 = 0xff;
pub const RIO_MAX_MBOX: usize = 4;
pub const RIO_MAX_MSG_SIZE: usize = 0x1000;
pub const RIO_SUCCESSFUL: u32 = 0x00;
pub const RIO_BAD_SIZE: u32 = 0x81;
pub const RIO_DOORBELL_RESOURCE: u32 = 0;
pub const RIO_INB_MBOX_RESOURCE: u32 = 1;
pub const RIO_OUTB_MBOX_RESOURCE: u32 = 2;
pub const RIO_PW_MSG_SIZE: usize = 64;
pub const RIO_CTAG_RESRVD: u32 = 0xfffe0000;
pub const RIO_CTAG_UDEVID: u32 = 0x0001ffff;

pub const fn rio_max_route_entries(size: bool) -> u32 { if size { 1 << 16 } else { 1 << 8 } }
pub const fn rio_any_destid(size: bool) -> u32 { if size { 0xffff } else { 0xff } }

extern "C" {
    pub static rio_bus_type: bus_type;
    pub static mut rio_mport_class: class;
}

pub struct rio_mport;
pub struct rio_dev;
pub union rio_pw_msg;

#[repr(C)]
pub struct rio_switch {
    pub node: list_head,
    pub route_table: *mut u8,
    pub port_ok: u32,
    pub ops: *mut rio_switch_ops,
    pub lock: spinlock_t,
    pub nextdev: [*mut rio_dev; 0],
}

#[repr(C)]
pub struct rio_switch_ops {
    pub owner: *mut module,
    pub add_entry: Option<unsafe extern "C" fn(*mut rio_mport, u16, u8, u16, u16, u8) -> i32>,
    pub get_entry: Option<unsafe extern "C" fn(*mut rio_mport, u16, u8, u16, u16, *mut u8) -> i32>,
    pub clr_table: Option<unsafe extern "C" fn(*mut rio_mport, u16, u8, u16) -> i32>,
    pub set_domain: Option<unsafe extern "C" fn(*mut rio_mport, u16, u8, u8) -> i32>,
    pub get_domain: Option<unsafe extern "C" fn(*mut rio_mport, u16, u8, *mut u8) -> i32>,
    pub em_init: Option<unsafe extern "C" fn(*mut rio_dev) -> i32>,
    pub em_handle: Option<unsafe extern "C" fn(*mut rio_dev, u8) -> i32>,
}

#[repr(C)]
pub enum rio_device_state { RIO_DEVICE_INITIALIZING, RIO_DEVICE_RUNNING, RIO_DEVICE_GONE, RIO_DEVICE_SHUTDOWN }

#[repr(C)]
pub struct rio_dev {
    pub global_list: list_head, pub net_list: list_head, pub net: *mut rio_net, pub do_enum: bool,
    pub did: u16, pub vid: u16, pub device_rev: u32, pub asm_did: u16, pub asm_vid: u16, pub asm_rev: u16,
    pub efptr: u16, pub pef: u32, pub swpinfo: u32, pub src_ops: u32, pub dst_ops: u32, pub comp_tag: u32,
    pub phys_efptr: u32, pub phys_rmap: u32, pub em_efptr: u32, pub dma_mask: u64,
    pub driver: *mut rio_driver, pub dev: device, pub riores: [resource; RIO_MAX_DEV_RESOURCES],
    pub pwcback: Option<unsafe extern "C" fn(*mut rio_dev, *mut rio_pw_msg, i32) -> i32>,
    pub destid: u16, pub hopcount: u8, pub prev: *mut rio_dev, pub state: atomic_t, pub rswitch: [rio_switch; 0],
}

/* Direct equivalents of the Linux container/list helper macros. */
#[macro_export] macro_rules! rio_dev_g { ($n:expr) => { list_entry!($n, rio_dev, global_list) }; }
#[macro_export] macro_rules! rio_dev_f { ($n:expr) => { list_entry!($n, rio_dev, net_list) }; }
#[macro_export] macro_rules! to_rio_dev { ($n:expr) => { container_of!($n, rio_dev, dev) }; }
#[macro_export] macro_rules! sw_to_rio_dev { ($n:expr) => { container_of!($n, rio_dev, rswitch[0]) }; }
#[macro_export] macro_rules! to_rio_mport { ($n:expr) => { container_of!($n, rio_mport, dev) }; }
#[macro_export] macro_rules! to_rio_net { ($n:expr) => { container_of!($n, rio_net, dev) }; }

pub const RIO_SCAN_ENUM_NO_WAIT: u32 = 0x00000001;

#[repr(C)] pub struct rio_msg { pub res: *mut resource, pub mcback: Option<unsafe extern "C" fn(*mut rio_mport, *mut core::ffi::c_void, i32, i32)> }
#[repr(C)] pub struct rio_dbell { pub node: list_head, pub res: *mut resource, pub dinb: Option<unsafe extern "C" fn(*mut rio_mport, *mut core::ffi::c_void, u16, u16, u16)>, pub dev_id: *mut core::ffi::c_void }

#[repr(C)]
pub struct rio_mport {
    pub dbells: list_head, pub pwrites: list_head, pub node: list_head, pub nnode: list_head, pub net: *mut rio_net,
    pub lock: mutex, pub iores: resource, pub riores: [resource; RIO_MAX_MPORT_RESOURCES],
    pub inb_msg: [rio_msg; RIO_MAX_MBOX], pub outb_msg: [rio_msg; RIO_MAX_MBOX], pub host_deviceid: i32,
    pub ops: *mut rio_ops, pub id: u8, pub index: u8, pub sys_size: u32, pub phys_efptr: u32, pub phys_rmap: u32,
    pub name: [u8; RIO_MAX_MPORT_NAME], pub dev: device, pub priv_: *mut core::ffi::c_void,
    #[cfg(CONFIG_RAPIDIO_DMA_ENGINE)] pub dma: dma_device,
    pub nscan: *mut rio_scan, pub state: atomic_t, pub pwe_refcnt: u32,
}

pub unsafe fn rio_mport_is_running(mport: *mut rio_mport) -> bool { atomic_read(&(*mport).state) == RIO_DEVICE_RUNNING as i32 }

#[repr(C)] pub struct rio_net { pub node: list_head, pub devices: list_head, pub switches: list_head, pub mports: list_head, pub hport: *mut rio_mport, pub id: u8, pub dev: device, pub enum_data: *mut core::ffi::c_void, pub release: Option<unsafe extern "C" fn(*mut rio_net)> }
#[repr(C)] pub enum rio_link_speed { RIO_LINK_DOWN=0, RIO_LINK_125=1, RIO_LINK_250=2, RIO_LINK_312=3, RIO_LINK_500=4, RIO_LINK_625=5 }
#[repr(C)] pub enum rio_link_width { RIO_LINK_1X=0, RIO_LINK_1XR=1, RIO_LINK_2X=3, RIO_LINK_4X=2, RIO_LINK_8X=4, RIO_LINK_16X=5 }
#[repr(C)] pub enum rio_mport_flags { RIO_MPORT_DMA=1<<0, RIO_MPORT_DMA_SG=1<<1, RIO_MPORT_IBSG=1<<2 }
#[repr(C)] pub struct rio_mport_attr { pub flags: i32, pub link_speed: i32, pub link_width: i32, pub dma_max_sge: i32, pub dma_max_size: i32, pub dma_align: i32 }

#[repr(C)] pub struct rio_ops {
    pub lcread: Option<unsafe extern "C" fn(*mut rio_mport,i32,u32,i32,*mut u32)->i32>, pub lcwrite: Option<unsafe extern "C" fn(*mut rio_mport,i32,u32,i32,u32)->i32>,
    pub cread: Option<unsafe extern "C" fn(*mut rio_mport,i32,u16,u8,u32,i32,*mut u32)->i32>, pub cwrite: Option<unsafe extern "C" fn(*mut rio_mport,i32,u16,u8,u32,i32,u32)->i32>,
    pub dsend: Option<unsafe extern "C" fn(*mut rio_mport,i32,u16,u16)->i32>, pub pwenable: Option<unsafe extern "C" fn(*mut rio_mport,i32)->i32>,
    pub open_outb_mbox: Option<unsafe extern "C" fn(*mut rio_mport,*mut core::ffi::c_void,i32,i32)->i32>, pub close_outb_mbox: Option<unsafe extern "C" fn(*mut rio_mport,i32)>,
    pub open_inb_mbox: Option<unsafe extern "C" fn(*mut rio_mport,*mut core::ffi::c_void,i32,i32)->i32>, pub close_inb_mbox: Option<unsafe extern "C" fn(*mut rio_mport,i32)>,
    pub add_outb_message: Option<unsafe extern "C" fn(*mut rio_mport,*mut rio_dev,i32,*mut core::ffi::c_void,usize)->i32>, pub add_inb_buffer: Option<unsafe extern "C" fn(*mut rio_mport,i32,*mut core::ffi::c_void)->i32>,
    pub get_inb_message: Option<unsafe extern "C" fn(*mut rio_mport,i32)->*mut core::ffi::c_void>, pub map_inb: Option<unsafe extern "C" fn(*mut rio_mport,dma_addr_t,u64,u64,u32)->i32>, pub unmap_inb: Option<unsafe extern "C" fn(*mut rio_mport,dma_addr_t)>,
    pub query_mport: Option<unsafe extern "C" fn(*mut rio_mport,*mut rio_mport_attr)->i32>, pub map_outb: Option<unsafe extern "C" fn(*mut rio_mport,u16,u64,u32,u32,*mut dma_addr_t)->i32>, pub unmap_outb: Option<unsafe extern "C" fn(*mut rio_mport,u16,u64)>,
}

pub const RIO_RESOURCE_MEM:u32=0x00000100; pub const RIO_RESOURCE_DOORBELL:u32=0x00000200; pub const RIO_RESOURCE_MAILBOX:u32=0x00000400; pub const RIO_RESOURCE_CACHEABLE:u32=0x00010000; pub const RIO_RESOURCE_PCI:u32=0x00020000; pub const RIO_RESOURCE_BUSY:u32=0x80000000;
#[repr(C)] pub struct rio_driver { pub node:list_head, pub name:*mut i8, pub id_table:*const rio_device_id, pub probe:Option<unsafe extern "C" fn(*mut rio_dev,*const rio_device_id)->i32>, pub remove:Option<unsafe extern "C" fn(*mut rio_dev)>, pub shutdown:Option<unsafe extern "C" fn(*mut rio_dev)>, pub suspend:Option<unsafe extern "C" fn(*mut rio_dev,u32)->i32>, pub resume:Option<unsafe extern "C" fn(*mut rio_dev)->i32>, pub enable_wake:Option<unsafe extern "C" fn(*mut rio_dev,u32,i32)->i32>, pub driver:device_driver }

#[repr(C)] pub union rio_pw_msg { pub em: rio_pw_msg_em, pub raw: [u32; RIO_PW_MSG_SIZE / 4] }
#[repr(C)] pub struct rio_pw_msg_em { pub comptag:u32, pub errdetect:u32, pub is_port:u32, pub ltlerrdet:u32, pub padding:[u32;12] }

#[cfg(CONFIG_RAPIDIO_DMA_ENGINE)]
#[repr(C)] pub enum rio_write_type { RDW_DEFAULT, RDW_ALL_NWRITE, RDW_ALL_NWRITE_R, RDW_LAST_NWRITE_R }
#[cfg(CONFIG_RAPIDIO_DMA_ENGINE)] #[repr(C)] pub struct rio_dma_ext { pub destid:u16, pub rio_addr:u64, pub rio_addr_u:u8, pub wr_type:rio_write_type }
#[cfg(CONFIG_RAPIDIO_DMA_ENGINE)] #[repr(C)] pub struct rio_dma_data { pub sg:*mut scatterlist, pub sg_len:u32, pub rio_addr:u64, pub rio_addr_u:u8, pub wr_type:rio_write_type }
#[cfg(CONFIG_RAPIDIO_DMA_ENGINE)] #[macro_export] macro_rules! dma_to_mport { ($ddev:expr) => { container_of!($ddev, rio_mport, dma) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
