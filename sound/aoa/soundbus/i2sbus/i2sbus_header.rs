/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * i2sbus driver -- private definitions
 *
 * Copyright 2006 Johannes Berg <johannes@sipsolutions.net>
 */

/* Dependencies from the original header:
 * linux/interrupt.h, linux/spinlock.h, linux/mutex.h, linux/completion.h,
 * sound/pcm.h, asm/pmac_feature.h, asm/dbdma.h, interface.h, ../soundbus.h.
 */

use core::ffi::c_void;

#[repr(C)]
pub struct i2sbus_control {
    pub list: list_head,
    pub macio: *mut macio_chip,
}

pub const MAX_DBDMA_COMMANDS: usize = 32;

#[repr(C)]
pub struct dbdma_command_mem {
    pub bus_addr: dma_addr_t,
    pub bus_cmd_start: dma_addr_t,
    pub cmds: *mut dbdma_cmd,
    pub space: *mut c_void,
    pub size: i32,
    /* C bitfields:
     * u32 running:1;
     * u32 stopping:1;
     */
    pub __bitfield_running_stopping: u32,
}

impl dbdma_command_mem {
    pub const RUNNING_MASK: u32 = 1u32 << 0;
    pub const STOPPING_MASK: u32 = 1u32 << 1;
}

#[repr(C)]
pub struct pcm_info {
    /* C bitfields:
     * u32 created:1; has this direction been created with alsa?
     * u32 active:1;  is this stream active?
     */
    pub __bitfield_created_active: u32,

    /* runtime information */
    pub substream: *mut snd_pcm_substream,
    pub current_period: i32,
    pub frame_count: u32,
    pub dbdma_ring: dbdma_command_mem,
    /* Original type: volatile struct dbdma_regs __iomem * */
    pub dbdma: *mut dbdma_regs,
    pub stop_completion: *mut completion,
}

impl pcm_info {
    pub const CREATED_MASK: u32 = 1u32 << 0;
    pub const ACTIVE_MASK: u32 = 1u32 << 1;
}

pub const aoa_resource_i2smmio: i32 = 0;
pub const aoa_resource_txdbdma: i32 = 1;
pub const aoa_resource_rxdbdma: i32 = 2;

#[repr(C)]
pub struct i2sbus_dev {
    pub sound: soundbus_dev,
    pub macio: *mut macio_dev,
    pub control: *mut i2sbus_control,
    /* Original type: volatile struct i2s_interface_regs __iomem * */
    pub intfregs: *mut i2s_interface_regs,

    pub resources: [resource; 3],
    pub allocated_resource: [*mut resource; 3],
    pub interrupts: [i32; 3],
    pub rnames: [[i8; 32]; 3],

    /* info about currently active substreams */
    pub out: pcm_info,
    pub in_: pcm_info,
    pub format: snd_pcm_format_t,
    pub rate: u32,

    /* list for a single controller */
    pub item: list_head,
    /* number of bus on controller */
    pub bus_number: i32,
    /* for use by control layer */
    pub enable: *mut pmf_function,
    pub cell_enable: *mut pmf_function,
    pub cell_disable: *mut pmf_function,
    pub clock_enable: *mut pmf_function,
    pub clock_disable: *mut pmf_function,

    /* locks */
    /* spinlock for low-level interrupt locking */
    pub low_lock: spinlock_t,
    /* mutex for high-level consistency */
    pub lock: mutex,
}

/* C macro:
 * #define soundbus_dev_to_i2sbus_dev(sdev) \
 *         container_of(sdev, struct i2sbus_dev, sound)
 */
pub unsafe fn soundbus_dev_to_i2sbus_dev(sdev: *mut soundbus_dev) -> *mut i2sbus_dev {
    (sdev as *mut u8).sub(core::mem::offset_of!(i2sbus_dev, sound)) as *mut i2sbus_dev
}

/* pcm specific functions */
unsafe extern "C" {
    pub fn i2sbus_attach_codec(
        dev: *mut soundbus_dev,
        card: *mut snd_card,
        ci: *mut codec_info,
        data: *mut c_void,
    ) -> i32;
    pub fn i2sbus_detach_codec(dev: *mut soundbus_dev, data: *mut c_void);
    pub fn i2sbus_tx_intr(irq: i32, devid: *mut c_void) -> irqreturn_t;
    pub fn i2sbus_rx_intr(irq: i32, devid: *mut c_void) -> irqreturn_t;

    pub fn i2sbus_wait_for_stop_both(i2sdev: *mut i2sbus_dev);
    pub fn i2sbus_pcm_prepare_both(i2sdev: *mut i2sbus_dev);

    /* control specific functions */
    pub fn i2sbus_control_init(dev: *mut macio_dev, c: *mut *mut i2sbus_control) -> i32;
    pub fn i2sbus_control_destroy(c: *mut i2sbus_control);
    pub fn i2sbus_control_add_dev(c: *mut i2sbus_control, i2sdev: *mut i2sbus_dev) -> i32;
    pub fn i2sbus_control_remove_dev(c: *mut i2sbus_control, i2sdev: *mut i2sbus_dev);
    pub fn i2sbus_control_enable(c: *mut i2sbus_control, i2sdev: *mut i2sbus_dev) -> i32;
    pub fn i2sbus_control_cell(
        c: *mut i2sbus_control,
        i2sdev: *mut i2sbus_dev,
        enable: i32,
    ) -> i32;
    pub fn i2sbus_control_clock(
        c: *mut i2sbus_control,
        i2sdev: *mut i2sbus_dev,
        enable: i32,
    ) -> i32;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
