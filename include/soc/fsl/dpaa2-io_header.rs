/* SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause) */
/*
 * Copyright 2014-2016 Freescale Semiconductor Inc.
 * Copyright 2017-2019 NXP
 *
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::c_void;

pub const DPAA2_IO_ANY_CPU: i32 = -1;

pub struct dpaa2_io;
pub struct dpaa2_io_store;
pub struct device;

/**
 * DOC: DPIO Service
 *
 * The DPIO service provides APIs for users to interact with the datapath
 * by enqueueing and dequeueing frame descriptors.
 *
 * The following set of APIs can be used to enqueue and dequeue frames
 * as well as producing notification callbacks when data is available
 * for dequeue.
 */

/**
 * struct dpaa2_io_desc - The DPIO descriptor
 * @receives_notifications: Use notification mode. Non-zero if the DPIO
 *                  has a channel.
 * @has_8prio:      Set to non-zero for channel with 8 priority WQs.  Ignored
 *                  unless receives_notification is TRUE.
 * @cpu:            The cpu index that at least interrupt handlers will
 *                  execute on.
 * @stash_affinity: The stash affinity for this portal favour 'cpu'
 * @regs_cena:      The cache enabled regs.
 * @regs_cinh:      The cache inhibited regs.
 * @dpio_id:        The dpio index
 * @qman_version:   The qman version
 * @qman_clk:       The qman clock frequency in Hz
 *
 * Describes the attributes and features of the DPIO object.
 */
#[repr(C)]
pub struct dpaa2_io_desc {
    pub receives_notifications: i32,
    pub has_8prio: i32,
    pub cpu: i32,
    pub regs_cena: *mut c_void,
    pub regs_cinh: *mut c_void,
    pub dpio_id: i32,
    pub qman_version: u32,
    pub qman_clk: u32,
}

/**
 * struct dpaa2_io_notification_ctx - The DPIO notification context structure
 * @cb:           The callback to be invoked when the notification arrives
 * @is_cdan:      Zero for FQDAN, non-zero for CDAN
 * @id:           FQID or channel ID, needed for rearm
 * @desired_cpu:  The cpu on which the notifications will show up. Use
 *                DPAA2_IO_ANY_CPU if don't care
 * @dpio_id:      The dpio index
 * @qman64:       The 64-bit context value shows up in the FQDAN/CDAN.
 * @node:         The list node
 * @dpio_private: The dpio object internal to dpio_service
 *
 * Used when a FQDAN/CDAN registration is made by drivers.
 */
#[repr(C)]
pub struct dpaa2_io_notification_ctx {
    pub cb: Option<unsafe extern "C" fn(ctx: *mut dpaa2_io_notification_ctx)>,
    pub is_cdan: i32,
    pub id: u32,
    pub desired_cpu: i32,
    pub dpio_id: i32,
    pub qman64: u64,
    pub node: list_head,
    pub dpio_private: *mut c_void,
}

extern "C" {
    pub fn dpaa2_io_create(desc: *const dpaa2_io_desc, dev: *mut device) -> *mut dpaa2_io;
    pub fn dpaa2_io_down(d: *mut dpaa2_io);
    pub fn dpaa2_io_irq(obj: *mut dpaa2_io) -> irqreturn_t;
    pub fn dpaa2_io_service_select(cpu: i32) -> *mut dpaa2_io;
    pub fn dpaa2_io_get_cpu(d: *mut dpaa2_io) -> i32;
    pub fn dpaa2_io_service_register(
        service: *mut dpaa2_io,
        ctx: *mut dpaa2_io_notification_ctx,
        dev: *mut device,
    ) -> i32;
    pub fn dpaa2_io_service_deregister(
        service: *mut dpaa2_io,
        ctx: *mut dpaa2_io_notification_ctx,
        dev: *mut device,
    );
    pub fn dpaa2_io_service_rearm(
        service: *mut dpaa2_io,
        ctx: *mut dpaa2_io_notification_ctx,
    ) -> i32;
    pub fn dpaa2_io_service_pull_fq(
        d: *mut dpaa2_io,
        fqid: u32,
        s: *mut dpaa2_io_store,
    ) -> i32;
    pub fn dpaa2_io_service_pull_channel(
        d: *mut dpaa2_io,
        channelid: u32,
        s: *mut dpaa2_io_store,
    ) -> i32;
    pub fn dpaa2_io_service_enqueue_fq(d: *mut dpaa2_io, fqid: u32, fd: *const dpaa2_fd) -> i32;
    pub fn dpaa2_io_service_enqueue_multiple_fq(
        d: *mut dpaa2_io,
        fqid: u32,
        fd: *const dpaa2_fd,
        number_of_frame: i32,
    ) -> i32;
    pub fn dpaa2_io_service_enqueue_multiple_desc_fq(
        d: *mut dpaa2_io,
        fqid: *mut u32,
        fd: *const dpaa2_fd,
        number_of_frame: i32,
    ) -> i32;
    pub fn dpaa2_io_service_enqueue_qd(
        d: *mut dpaa2_io,
        qdid: u32,
        prio: u8,
        qdbin: u16,
        fd: *const dpaa2_fd,
    ) -> i32;
    pub fn dpaa2_io_service_release(
        d: *mut dpaa2_io,
        bpid: u16,
        buffers: *const u64,
        num_buffers: libc::c_uint,
    ) -> i32;
    pub fn dpaa2_io_service_acquire(
        d: *mut dpaa2_io,
        bpid: u16,
        buffers: *mut u64,
        num_buffers: libc::c_uint,
    ) -> i32;
    pub fn dpaa2_io_store_create(max_frames: libc::c_uint, dev: *mut device) -> *mut dpaa2_io_store;
    pub fn dpaa2_io_store_destroy(s: *mut dpaa2_io_store);
    pub fn dpaa2_io_store_next(s: *mut dpaa2_io_store, is_last: *mut i32) -> *mut dpaa2_dq;
    pub fn dpaa2_io_query_fq_count(
        d: *mut dpaa2_io,
        fqid: u32,
        fcnt: *mut u32,
        bcnt: *mut u32,
    ) -> i32;
    pub fn dpaa2_io_query_bp_count(d: *mut dpaa2_io, bpid: u16, num: *mut u32) -> i32;
    pub fn dpaa2_io_set_irq_coalescing(d: *mut dpaa2_io, irq_holdoff: u32) -> i32;
    pub fn dpaa2_io_get_irq_coalescing(d: *mut dpaa2_io, irq_holdoff: *mut u32);
    pub fn dpaa2_io_set_adaptive_coalescing(d: *mut dpaa2_io, use_adaptive_rx_coalesce: i32);
    pub fn dpaa2_io_get_adaptive_coalescing(d: *mut dpaa2_io) -> i32;
    pub fn dpaa2_io_update_net_dim(d: *mut dpaa2_io, frames: u64, bytes: u64);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
