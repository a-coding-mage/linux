/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  tifm.h - TI FlashMedia driver
 *
 *  Copyright (C) 2006 Alex Dubov <oakad@yahoo.com>
 */

/* Dependencies supplied by the Linux kernel translation environment. */

pub const FM_SET_INTERRUPT_ENABLE: u32 = 0x008;
pub const FM_CLEAR_INTERRUPT_ENABLE: u32 = 0x00c;
pub const FM_INTERRUPT_STATUS: u32 = 0x014;

pub const SOCK_CONTROL: u32 = 0x004;
pub const SOCK_PRESENT_STATE: u32 = 0x008;
pub const SOCK_DMA_ADDRESS: u32 = 0x00c;
pub const SOCK_DMA_CONTROL: u32 = 0x010;
pub const SOCK_DMA_FIFO_INT_ENABLE_SET: u32 = 0x014;
pub const SOCK_DMA_FIFO_INT_ENABLE_CLEAR: u32 = 0x018;
pub const SOCK_DMA_FIFO_STATUS: u32 = 0x020;
pub const SOCK_FIFO_CONTROL: u32 = 0x024;
pub const SOCK_FIFO_PAGE_SIZE: u32 = 0x028;
pub const SOCK_MMCSD_COMMAND: u32 = 0x104;
pub const SOCK_MMCSD_ARG_LOW: u32 = 0x108;
pub const SOCK_MMCSD_ARG_HIGH: u32 = 0x10c;
pub const SOCK_MMCSD_CONFIG: u32 = 0x110;
pub const SOCK_MMCSD_STATUS: u32 = 0x114;
pub const SOCK_MMCSD_INT_ENABLE: u32 = 0x118;
pub const SOCK_MMCSD_COMMAND_TO: u32 = 0x11c;
pub const SOCK_MMCSD_DATA_TO: u32 = 0x120;
pub const SOCK_MMCSD_DATA: u32 = 0x124;
pub const SOCK_MMCSD_BLOCK_LEN: u32 = 0x128;
pub const SOCK_MMCSD_NUM_BLOCKS: u32 = 0x12c;
pub const SOCK_MMCSD_BUFFER_CONFIG: u32 = 0x130;
pub const SOCK_MMCSD_SPI_CONFIG: u32 = 0x134;
pub const SOCK_MMCSD_SDIO_MODE_CONFIG: u32 = 0x138;
pub const SOCK_MMCSD_RESPONSE: u32 = 0x144;
pub const SOCK_MMCSD_SDIO_SR: u32 = 0x164;
pub const SOCK_MMCSD_SYSTEM_CONTROL: u32 = 0x168;
pub const SOCK_MMCSD_SYSTEM_STATUS: u32 = 0x16c;
pub const SOCK_MS_COMMAND: u32 = 0x184;
pub const SOCK_MS_DATA: u32 = 0x188;
pub const SOCK_MS_STATUS: u32 = 0x18c;
pub const SOCK_MS_SYSTEM: u32 = 0x190;
pub const SOCK_FIFO_ACCESS: u32 = 0x200;

pub const TIFM_CTRL_LED: u32 = 0x00000040;
pub const TIFM_CTRL_FAST_CLK: u32 = 0x00000100;
pub const TIFM_CTRL_POWER_MASK: u32 = 0x00000007;
pub const TIFM_SOCK_STATE_OCCUPIED: u32 = 0x00000008;
pub const TIFM_SOCK_STATE_POWERED: u32 = 0x00000080;
pub const TIFM_FIFO_ENABLE: u32 = 0x00000001;
pub const TIFM_FIFO_READY: u32 = 0x00000001;
pub const TIFM_FIFO_MORE: u32 = 0x00000008;
pub const TIFM_FIFO_INT_SETALL: u32 = 0x0000ffff;
pub const TIFM_FIFO_INTMASK: u32 = 0x00000005;
pub const TIFM_DMA_RESET: u32 = 0x00000002;
pub const TIFM_DMA_TX: u32 = 0x00008000;
pub const TIFM_DMA_EN: u32 = 0x00000001;
pub const TIFM_DMA_TSIZE: u32 = 0x0000007f;
pub const TIFM_TYPE_XD: u32 = 1;
pub const TIFM_TYPE_MS: u32 = 2;
pub const TIFM_TYPE_SD: u32 = 3;

#[repr(C)]
pub struct tifm_device_id {
    pub r#type: u8,
}

#[repr(C)]
pub struct tifm_dev {
    pub addr: *mut core::ffi::c_char,
    pub lock: spinlock_t,
    pub r#type: u8,
    pub socket_id: core::ffi::c_uint,
    pub card_event: Option<unsafe extern "C" fn(sock: *mut tifm_dev)>,
    pub data_event: Option<unsafe extern "C" fn(sock: *mut tifm_dev)>,
    pub dev: device,
}

#[repr(C)]
pub struct tifm_driver {
    pub id_table: *const tifm_device_id,
    pub probe: Option<unsafe extern "C" fn(dev: *mut tifm_dev) -> core::ffi::c_int>,
    pub remove: Option<unsafe extern "C" fn(dev: *mut tifm_dev)>,
    pub suspend: Option<unsafe extern "C" fn(dev: *mut tifm_dev, state: pm_message_t) -> core::ffi::c_int>,
    pub resume: Option<unsafe extern "C" fn(dev: *mut tifm_dev) -> core::ffi::c_int>,
    pub driver: device_driver,
}

#[repr(C)]
pub struct tifm_adapter {
    pub addr: *mut core::ffi::c_char,
    pub lock: spinlock_t,
    pub irq_status: core::ffi::c_uint,
    pub socket_change_set: core::ffi::c_uint,
    pub id: core::ffi::c_uint,
    pub num_sockets: core::ffi::c_uint,
    pub finish_me: *mut completion,
    pub media_switcher: work_struct,
    pub dev: device,
    pub eject: Option<unsafe extern "C" fn(fm: *mut tifm_adapter, sock: *mut tifm_dev)>,
    pub has_ms_pif: Option<unsafe extern "C" fn(fm: *mut tifm_adapter, sock: *mut tifm_dev) -> core::ffi::c_int>,
    pub sockets: [*mut tifm_dev; 0],
}

extern "C" {
    pub fn tifm_alloc_adapter(num_sockets: core::ffi::c_uint, dev: *mut device) -> *mut tifm_adapter;
    pub fn tifm_add_adapter(fm: *mut tifm_adapter) -> core::ffi::c_int;
    pub fn tifm_remove_adapter(fm: *mut tifm_adapter);
    pub fn tifm_free_adapter(fm: *mut tifm_adapter);
    pub fn tifm_free_device(dev: *mut device);
    pub fn tifm_alloc_device(fm: *mut tifm_adapter, id: core::ffi::c_uint, r#type: u8) -> *mut tifm_dev;
    pub fn tifm_register_driver(drv: *mut tifm_driver) -> core::ffi::c_int;
    pub fn tifm_unregister_driver(drv: *mut tifm_driver);
    pub fn tifm_eject(sock: *mut tifm_dev);
    pub fn tifm_has_ms_pif(sock: *mut tifm_dev) -> core::ffi::c_int;
    pub fn tifm_map_sg(sock: *mut tifm_dev, sg: *mut scatterlist, nents: core::ffi::c_int, direction: core::ffi::c_int) -> core::ffi::c_int;
    pub fn tifm_unmap_sg(sock: *mut tifm_dev, sg: *mut scatterlist, nents: core::ffi::c_int, direction: core::ffi::c_int);
    pub fn tifm_queue_work(work: *mut work_struct);
    pub fn dev_get_drvdata(dev: *mut device) -> *mut core::ffi::c_void;
    pub fn dev_set_drvdata(dev: *mut device, data: *mut core::ffi::c_void);
}

pub unsafe fn tifm_get_drvdata(dev: *mut tifm_dev) -> *mut core::ffi::c_void {
    dev_get_drvdata(&mut (*dev).dev)
}

pub unsafe fn tifm_set_drvdata(dev: *mut tifm_dev, data: *mut core::ffi::c_void) {
    dev_set_drvdata(&mut (*dev).dev, data)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
