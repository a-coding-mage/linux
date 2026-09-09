/* SPDX-License-Identifier: GPL-2.0 */

/* Dependency supplied by the corresponding Linux llist definitions. */

pub const CSD_FLAG_LOCK: u32 = 0x01;

pub const IRQ_WORK_PENDING: u32 = 0x01;
pub const IRQ_WORK_BUSY: u32 = 0x02;
pub const IRQ_WORK_LAZY: u32 = 0x04; /* No IPI, wait for tick */
pub const IRQ_WORK_HARD_IRQ: u32 = 0x08; /* IRQ context on PREEMPT_RT */

pub const IRQ_WORK_CLAIMED: u32 = IRQ_WORK_PENDING | IRQ_WORK_BUSY;

pub const CSD_TYPE_ASYNC: u32 = 0x00;
pub const CSD_TYPE_SYNC: u32 = 0x10;
pub const CSD_TYPE_IRQ_WORK: u32 = 0x20;
pub const CSD_TYPE_TTWU: u32 = 0x30;

pub const CSD_FLAG_TYPE_MASK: u32 = 0xF0;

/*
 * struct __call_single_node is the primary type on
 * smp.c:call_single_queue.
 *
 * flush_smp_call_function_queue() only reads the type from
 * __call_single_node::u_flags as a regular load, the above
 * (anonymous) enum defines all the bits of this word.
 *
 * Other bits are not modified until the type is known.
 *
 * CSD_TYPE_SYNC/ASYNC:
 * struct {
 *     struct llist_node node;
 *     unsigned int flags;
 *     smp_call_func_t func;
 *     void *info;
 * };
 *
 * CSD_TYPE_IRQ_WORK:
 * struct {
 *     struct llist_node node;
 *     atomic_t flags;
 *     void (*func)(struct irq_work *);
 * };
 *
 * CSD_TYPE_TTWU:
 * struct {
 *     struct llist_node node;
 *     unsigned int flags;
 * };
 */

#[repr(C)]
pub union __call_single_node_flags {
    pub u_flags: u32,
    pub a_flags: std::mem::ManuallyDrop<atomic_t>,
}

#[repr(C)]
pub struct __call_single_node {
    pub llist: llist_node,
    pub flags: __call_single_node_flags,
    #[cfg(target_pointer_width = "64")]
    pub src: u16,
    #[cfg(target_pointer_width = "64")]
    pub dst: u16,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
