/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Freescale MPC85xx/MPC86xx RapidIO support
 *
 * Copyright 2009 Sysgo AG
 * Thomas Moll <thomas.moll@sysgo.com>
 * - fixed maintenance access routines, check for aligned access
 *
 * Copyright 2009 Integrated Device Technology, Inc.
 * Alex Bounine <alexandre.bounine@idt.com>
 * - Added Port-Write message handling
 * - Added Machine Check exception handling
 *
 * Copyright (C) 2007, 2008, 2010, 2011 Freescale Semiconductor, Inc.
 * Zhang Wei <wei.zhang@freescale.com>
 * Lian Minghuan-B31939 <Minghuan.Lian@freescale.com>
 * Liu Gang <Gang.Liu@freescale.com>
 *
 * Copyright 2005 MontaVista Software, Inc.
 * Matt Porter <mporter@kernel.crashing.org>
 */

/* Dependencies supplied by the surrounding translation unit. */

#[macro_export]
macro_rules! RIO_REGS_WIN {
    ($mport:expr) => {{
        unsafe { (*((*$mport).priv_ as *mut rio_priv)).regs_win }
    }};
}

pub const RIO_MAINT_WIN_SIZE: u32 = 0x400000;
pub const RIO_LTLEDCSR: u32 = 0x0608;

pub const DOORBELL_ROWAR_EN: u32 = 0x80000000;
pub const DOORBELL_ROWAR_TFLOWLV: u32 = 0x08000000; /* highest priority level */
pub const DOORBELL_ROWAR_PCI: u32 = 0x02000000; /* PCI window */
pub const DOORBELL_ROWAR_NREAD: u32 = 0x00040000; /* NREAD */
pub const DOORBELL_ROWAR_MAINTRD: u32 = 0x00070000; /* maintenance read */
pub const DOORBELL_ROWAR_RES: u32 = 0x00002000; /* wrtpy: reserved */
pub const DOORBELL_ROWAR_MAINTWD: u32 = 0x00007000;
pub const DOORBELL_ROWAR_SIZE: u32 = 0x0000000b; /* window size is 4k */

pub const RIO_ATMU_REGS_PORT1_OFFSET: u32 = 0x10c00;
pub const RIO_ATMU_REGS_PORT2_OFFSET: u32 = 0x10e00;
pub const RIO_S_DBELL_REGS_OFFSET: u32 = 0x13400;
pub const RIO_S_PW_REGS_OFFSET: u32 = 0x134e0;
pub const RIO_ATMU_REGS_DBELL_OFFSET: u32 = 0x10C40;
pub const RIO_INB_ATMU_REGS_PORT1_OFFSET: u32 = 0x10d60;
pub const RIO_INB_ATMU_REGS_PORT2_OFFSET: u32 = 0x10f60;

pub const MAX_MSG_UNIT_NUM: usize = 2;
pub const MAX_PORT_NUM: usize = 4;
pub const RIO_INB_ATMU_COUNT: usize = 4;

#[repr(C)]
pub struct rio_atmu_regs { pub rowtar: u32, pub rowtear: u32, pub rowbar: u32, pub pad1: u32, pub rowar: u32, pub pad2: [u32; 3] }

#[repr(C)]
pub struct rio_inb_atmu_regs { pub riwtar: u32, pub pad1: u32, pub riwbar: u32, pub pad2: u32, pub riwar: u32, pub pad3: [u32; 3] }

#[repr(C)]
pub struct rio_dbell_ring { pub virt: *mut core::ffi::c_void, pub phys: dma_addr_t }

#[repr(C)]
pub struct rio_port_write_msg { pub virt: *mut core::ffi::c_void, pub phys: dma_addr_t, pub msg_count: u32, pub err_count: u32, pub discard_count: u32 }

#[repr(C)]
pub struct fsl_rio_dbell { pub mport: [*mut rio_mport; MAX_PORT_NUM], pub dev: *mut device, pub dbell_regs: *mut rio_dbell_regs, pub dbell_ring: rio_dbell_ring, pub bellirq: i32 }

#[repr(C)]
pub struct fsl_rio_pw { pub mport: [*mut rio_mport; MAX_PORT_NUM], pub dev: *mut device, pub pw_regs: *mut rio_pw_regs, pub port_write_msg: rio_port_write_msg, pub pwirq: i32, pub pw_work: work_struct, pub pw_fifo: kfifo, pub pw_fifo_lock: spinlock_t }

#[repr(C)]
pub struct rio_priv { pub dev: *mut device, pub regs_win: *mut core::ffi::c_void, pub atmu_regs: *mut rio_atmu_regs, pub maint_atmu_regs: *mut rio_atmu_regs, pub inb_atmu_regs: *mut rio_inb_atmu_regs, pub maint_win: *mut core::ffi::c_void, pub rmm_handle: *mut core::ffi::c_void /* RapidIO message manager(unit) Handle */ }

extern "C" {
    pub static mut rio_regs_win: *mut core::ffi::c_void;
    pub static mut rmu_regs_win: *mut core::ffi::c_void;
    pub static mut rio_law_start: resource_size_t;
    pub static mut dbell: *mut fsl_rio_dbell;
    pub static mut pw: *mut fsl_rio_pw;

    pub fn fsl_rio_setup_rmu(mport: *mut rio_mport, node: *mut device_node) -> i32;
    pub fn fsl_rio_port_write_init(pw: *mut fsl_rio_pw) -> i32;
    pub fn fsl_rio_pw_enable(mport: *mut rio_mport, enable: i32) -> i32;
    pub fn fsl_rio_port_error_handler(offset: i32);
    pub fn fsl_rio_doorbell_init(dbell: *mut fsl_rio_dbell) -> i32;
    pub fn fsl_rio_doorbell_send(mport: *mut rio_mport, index: i32, destid: u16, data: u16) -> i32;
    pub fn fsl_add_outb_message(mport: *mut rio_mport, rdev: *mut rio_dev, mbox: i32, buffer: *mut core::ffi::c_void, len: usize) -> i32;
    pub fn fsl_open_outb_mbox(mport: *mut rio_mport, dev_id: *mut core::ffi::c_void, mbox: i32, entries: i32) -> i32;
    pub fn fsl_close_outb_mbox(mport: *mut rio_mport, mbox: i32);
    pub fn fsl_open_inb_mbox(mport: *mut rio_mport, dev_id: *mut core::ffi::c_void, mbox: i32, entries: i32) -> i32;
    pub fn fsl_close_inb_mbox(mport: *mut rio_mport, mbox: i32);
    pub fn fsl_add_inb_buffer(mport: *mut rio_mport, mbox: i32, buf: *mut core::ffi::c_void) -> i32;
    pub fn fsl_get_inb_message(mport: *mut rio_mport, mbox: i32) -> *mut core::ffi::c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
