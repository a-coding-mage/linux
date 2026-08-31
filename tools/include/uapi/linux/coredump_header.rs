/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// C header dependency: <linux/types.h> provides __u32 and __u64.

/**
 * coredump_{req,ack} flags
 * @COREDUMP_KERNEL: kernel writes coredump
 * @COREDUMP_USERSPACE: userspace writes coredump
 * @COREDUMP_REJECT: don't generate coredump
 * @COREDUMP_WAIT: wait for coredump server
 */
pub const COREDUMP_KERNEL: __u64 = 1u64 << 0;
pub const COREDUMP_USERSPACE: __u64 = 1u64 << 1;
pub const COREDUMP_REJECT: __u64 = 1u64 << 2;
pub const COREDUMP_WAIT: __u64 = 1u64 << 3;

/**
 * struct coredump_req - message kernel sends to userspace
 * @size: size of struct coredump_req
 * @size_ack: known size of struct coredump_ack on this kernel
 * @mask: supported features
 *
 * When a coredump happens the kernel will connect to the coredump
 * socket and send a coredump request to the coredump server. The @size
 * member is set to the size of struct coredump_req and provides a hint
 * to userspace how much data can be read. Userspace may use MSG_PEEK to
 * peek the size of struct coredump_req and then choose to consume it in
 * one go. Userspace may also simply read a COREDUMP_ACK_SIZE_VER0
 * request. If the size the kernel sends is larger userspace simply
 * discards any remaining data.
 *
 * The coredump_req->mask member is set to the currently know features.
 * Userspace may only set coredump_ack->mask to the bits raised by the
 * kernel in coredump_req->mask.
 *
 * The coredump_req->size_ack member is set by the kernel to the size of
 * struct coredump_ack the kernel knows. Userspace may only send up to
 * coredump_req->size_ack bytes to the kernel and must set
 * coredump_ack->size accordingly.
 */
#[repr(C)]
pub struct coredump_req {
    pub size: __u32,
    pub size_ack: __u32,
    pub mask: __u64,
}

pub const COREDUMP_REQ_SIZE_VER0: __u32 = 16u32; /* size of first published struct */

/**
 * struct coredump_ack - message userspace sends to kernel
 * @size: size of the struct
 * @spare: unused
 * @mask: features kernel is supposed to use
 *
 * The @size member must be set to the size of struct coredump_ack. It
 * may never exceed what the kernel returned in coredump_req->size_ack
 * but it may of course be smaller (>= COREDUMP_ACK_SIZE_VER0 and <=
 * coredump_req->size_ack).
 *
 * The @mask member must be set to the features the coredump server
 * wants the kernel to use. Only bits the kernel returned in
 * coredump_req->mask may be set.
 */
#[repr(C)]
pub struct coredump_ack {
    pub size: __u32,
    pub spare: __u32,
    pub mask: __u64,
}

pub const COREDUMP_ACK_SIZE_VER0: __u32 = 16u32; /* size of first published struct */

/**
 * enum coredump_mark - Markers for the coredump socket
 *
 * The kernel will place a single byte on the coredump socket. The
 * markers notify userspace whether the coredump ack succeeded or
 * failed.
 *
 * @COREDUMP_MARK_MINSIZE: the provided coredump_ack size was too small
 * @COREDUMP_MARK_MAXSIZE: the provided coredump_ack size was too big
 * @COREDUMP_MARK_UNSUPPORTED: the provided coredump_ack mask was invalid
 * @COREDUMP_MARK_CONFLICTING: the provided coredump_ack mask has conflicting options
 * @COREDUMP_MARK_REQACK: the coredump request and ack was successful
 * @__COREDUMP_MARK_MAX: the maximum coredump mark value
 */
#[repr(u32)]
pub enum coredump_mark {
    COREDUMP_MARK_REQACK = 0u32,
    COREDUMP_MARK_MINSIZE = 1u32,
    COREDUMP_MARK_MAXSIZE = 2u32,
    COREDUMP_MARK_UNSUPPORTED = 3u32,
    COREDUMP_MARK_CONFLICTING = 4u32,
    __COREDUMP_MARK_MAX = 1u32 << 31,
}
