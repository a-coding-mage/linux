/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */

/*
 * rseq-abi.h
 *
 * Restartable sequences system call API
 *
 * Copyright (c) 2015-2022 Mathieu Desnoyers <mathieu.desnoyers@efficios.com>
 */

/* C dependencies removed from executable Rust:
 * #include <linux/types.h>
 * #include <asm/byteorder.h>
 */

pub const RSEQ_ABI_CPU_ID_UNINITIALIZED: i32 = -1;
pub const RSEQ_ABI_CPU_ID_REGISTRATION_FAILED: i32 = -2;

pub const RSEQ_ABI_FLAG_UNREGISTER: u32 = 1 << 0;

pub const RSEQ_ABI_CS_FLAG_NO_RESTART_ON_PREEMPT_BIT: u32 = 0;
pub const RSEQ_ABI_CS_FLAG_NO_RESTART_ON_SIGNAL_BIT: u32 = 1;
pub const RSEQ_ABI_CS_FLAG_NO_RESTART_ON_MIGRATE_BIT: u32 = 2;

pub const RSEQ_ABI_CS_FLAG_NO_RESTART_ON_PREEMPT: u32 =
    1u32 << RSEQ_ABI_CS_FLAG_NO_RESTART_ON_PREEMPT_BIT;
pub const RSEQ_ABI_CS_FLAG_NO_RESTART_ON_SIGNAL: u32 =
    1u32 << RSEQ_ABI_CS_FLAG_NO_RESTART_ON_SIGNAL_BIT;
pub const RSEQ_ABI_CS_FLAG_NO_RESTART_ON_MIGRATE: u32 =
    1u32 << RSEQ_ABI_CS_FLAG_NO_RESTART_ON_MIGRATE_BIT;

/*
 * struct rseq_abi_cs is aligned on 4 * 8 bytes to ensure it is always
 * contained within a single cache-line. It is usually declared as
 * link-time constant data.
 */
#[repr(C, align(32))]
#[derive(Copy, Clone)]
pub struct rseq_abi_cs {
    /* Version of this structure. */
    pub version: u32,
    /* enum rseq_abi_cs_flags */
    pub flags: u32,
    pub start_ip: u64,
    /* Offset from start_ip. */
    pub post_commit_offset: u64,
    pub abort_ip: u64,
}

/**
 * rseq_abi_slice_ctrl - Time slice extension control structure
 * @all:	Compound value
 * @request:	Request for a time slice extension
 * @granted:	Granted time slice extension
 *
 * @request is set by user space and can be cleared by user space or kernel
 * space.  @granted is set and cleared by the kernel and must only be read
 * by user space.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rseq_abi_slice_ctrl {
    pub u: rseq_abi_slice_ctrl_union,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union rseq_abi_slice_ctrl_union {
    pub all: u32,
    pub fields: rseq_abi_slice_ctrl_fields,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rseq_abi_slice_ctrl_fields {
    pub request: u8,
    pub granted: u8,
    pub __reserved: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union rseq_abi_rseq_cs {
    pub ptr64: u64,

    /*
     * The "arch" field provides architecture accessor for
     * the ptr field based on architecture pointer size and
     * endianness.
     */
    pub arch: rseq_abi_rseq_cs_arch,
}

#[cfg(target_pointer_width = "64")]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rseq_abi_rseq_cs_arch {
    pub ptr: u64,
}

#[cfg(all(not(target_pointer_width = "64"), target_endian = "big"))]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rseq_abi_rseq_cs_arch {
    pub padding: u32, /* Initialized to zero. */
    pub ptr: u32,
}

#[cfg(all(not(target_pointer_width = "64"), not(target_endian = "big")))]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rseq_abi_rseq_cs_arch {
    pub ptr: u32,
    pub padding: u32, /* Initialized to zero. */
}

/*
 * struct rseq_abi is aligned on 4 * 8 bytes to ensure it is always
 * contained within a single cache-line.
 *
 * A single struct rseq_abi per thread is allowed.
 */
#[repr(C, align(256))]
#[derive(Copy, Clone)]
pub struct rseq_abi {
    /*
     * Restartable sequences cpu_id_start field. Updated by the
     * kernel. Read by user-space with single-copy atomicity
     * semantics. This field should only be read by the thread which
     * registered this data structure. Aligned on 32-bit. Always
     * contains a value in the range of possible CPUs, although the
     * value may not be the actual current CPU (e.g. if rseq is not
     * initialized). This CPU number value should always be compared
     * against the value of the cpu_id field before performing a rseq
     * commit or returning a value read from a data structure indexed
     * using the cpu_id_start value.
     */
    pub cpu_id_start: u32,
    /*
     * Restartable sequences cpu_id field. Updated by the kernel.
     * Read by user-space with single-copy atomicity semantics. This
     * field should only be read by the thread which registered this
     * data structure. Aligned on 32-bit. Values
     * RSEQ_CPU_ID_UNINITIALIZED and RSEQ_CPU_ID_REGISTRATION_FAILED
     * have a special semantic: the former means "rseq uninitialized",
     * and latter means "rseq initialization failed". This value is
     * meant to be read within rseq critical sections and compared
     * with the cpu_id_start value previously read, before performing
     * the commit instruction, or read and compared with the
     * cpu_id_start value before returning a value loaded from a data
     * structure indexed using the cpu_id_start value.
     */
    pub cpu_id: u32,
    /*
     * Restartable sequences rseq_cs field.
     *
     * Contains NULL when no critical section is active for the current
     * thread, or holds a pointer to the currently active struct rseq_cs.
     *
     * Updated by user-space, which sets the address of the currently
     * active rseq_cs at the beginning of assembly instruction sequence
     * block, and set to NULL by the kernel when it restarts an assembly
     * instruction sequence block, as well as when the kernel detects that
     * it is preempting or delivering a signal outside of the range
     * targeted by the rseq_cs. Also needs to be set to NULL by user-space
     * before reclaiming memory that contains the targeted struct rseq_cs.
     *
     * Read and set by the kernel. Set by user-space with single-copy
     * atomicity semantics. This field should only be updated by the
     * thread which registered this data structure. Aligned on 64-bit.
     */
    pub rseq_cs: rseq_abi_rseq_cs,

    /*
     * Restartable sequences flags field.
     *
     * This field should only be updated by the thread which
     * registered this data structure. Read by the kernel.
     * Mainly used for single-stepping through rseq critical sections
     * with debuggers.
     *
     * - RSEQ_ABI_CS_FLAG_NO_RESTART_ON_PREEMPT
     *     Inhibit instruction sequence block restart on preemption
     *     for this thread.
     * - RSEQ_ABI_CS_FLAG_NO_RESTART_ON_SIGNAL
     *     Inhibit instruction sequence block restart on signal
     *     delivery for this thread.
     * - RSEQ_ABI_CS_FLAG_NO_RESTART_ON_MIGRATE
     *     Inhibit instruction sequence block restart on migration for
     *     this thread.
     */
    pub flags: u32,

    /*
     * Restartable sequences node_id field. Updated by the kernel. Read by
     * user-space with single-copy atomicity semantics. This field should
     * only be read by the thread which registered this data structure.
     * Aligned on 32-bit. Contains the current NUMA node ID.
     */
    pub node_id: u32,

    /*
     * Restartable sequences mm_cid field. Updated by the kernel. Read by
     * user-space with single-copy atomicity semantics. This field should
     * only be read by the thread which registered this data structure.
     * Aligned on 32-bit. Contains the current thread's concurrency ID
     * (allocated uniquely within a memory map).
     */
    pub mm_cid: u32,

    /*
     * Time slice extension control structure. CPU local updates from
     * kernel and user space.
     */
    pub slice_ctrl: rseq_abi_slice_ctrl,

    /*
     * Place holder to push the size above 32 bytes.
     */
    pub __reserved: u8,

    /*
     * Flexible array member at end of structure, after last feature field.
     */
    pub end: [::std::os::raw::c_char; 0],
}
