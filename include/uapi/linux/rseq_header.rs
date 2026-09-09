/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */

/*
 * linux/rseq.h
 *
 * Restartable sequences system call API
 *
 * Copyright (c) 2015-2018 Mathieu Desnoyers <mathieu.desnoyers@efficios.com>
 */

// Dependency intent from <linux/types.h> and <asm/byteorder.h> is preserved
// through the Rust primitive integer types used below.

#[repr(i32)]
pub enum RseqCpuIdState {
    RSEQ_CPU_ID_UNINITIALIZED = -1,
    RSEQ_CPU_ID_REGISTRATION_FAILED = -2,
}

pub const RSEQ_FLAG_UNREGISTER: u32 = 1 << 0;
pub const RSEQ_FLAG_SLICE_EXT_DEFAULT_ON: u32 = 1 << 1;

/* Historical and unsupported bits */
pub const RSEQ_CS_FLAG_NO_RESTART_ON_PREEMPT_BIT: u32 = 0;
pub const RSEQ_CS_FLAG_NO_RESTART_ON_SIGNAL_BIT: u32 = 1;
pub const RSEQ_CS_FLAG_NO_RESTART_ON_MIGRATE_BIT: u32 = 2;
/* (3) Intentional gap to keep new bits separate */

/* User read only feature flags */
pub const RSEQ_CS_FLAG_SLICE_EXT_AVAILABLE_BIT: u32 = 4;
pub const RSEQ_CS_FLAG_SLICE_EXT_ENABLED_BIT: u32 = 5;

pub const RSEQ_CS_FLAG_NO_RESTART_ON_PREEMPT: u32 =
    1u32 << RSEQ_CS_FLAG_NO_RESTART_ON_PREEMPT_BIT;
pub const RSEQ_CS_FLAG_NO_RESTART_ON_SIGNAL: u32 =
    1u32 << RSEQ_CS_FLAG_NO_RESTART_ON_SIGNAL_BIT;
pub const RSEQ_CS_FLAG_NO_RESTART_ON_MIGRATE: u32 =
    1u32 << RSEQ_CS_FLAG_NO_RESTART_ON_MIGRATE_BIT;
pub const RSEQ_CS_FLAG_SLICE_EXT_AVAILABLE: u32 =
    1u32 << RSEQ_CS_FLAG_SLICE_EXT_AVAILABLE_BIT;
pub const RSEQ_CS_FLAG_SLICE_EXT_ENABLED: u32 =
    1u32 << RSEQ_CS_FLAG_SLICE_EXT_ENABLED_BIT;

/*
 * struct rseq_cs is aligned on 4 * 8 bytes to ensure it is always
 * contained within a single cache-line. It is usually declared as
 * link-time constant data.
 */
#[repr(C, align(32))]
pub struct RseqCs {
    /* Version of this structure. */
    pub version: u32,
    /* enum rseq_cs_flags */
    pub flags: u32,
    pub start_ip: u64,
    /* Offset from start_ip. */
    pub post_commit_offset: u64,
    pub abort_ip: u64,
}

/**
 * rseq_slice_ctrl - Time slice extension control structure
 * @all: Compound value
 * @request: Request for a time slice extension
 * @granted: Granted time slice extension
 *
 * @request is set by user space and can be cleared by user space or kernel
 * space.  @granted is set and cleared by the kernel and must only be read
 * by user space.
 */
#[repr(C)]
pub union RseqSliceCtrl {
    pub all: u32,
    pub bytes: RseqSliceCtrlBytes,
}

#[repr(C)]
pub struct RseqSliceCtrlBytes {
    pub request: u8,
    pub granted: u8,
    pub __reserved: u16,
}

/*
 * The original size and alignment of the allocation for struct rseq is
 * 32 bytes.
 *
 * The allocation size needs to be greater or equal to
 * max(getauxval(AT_RSEQ_FEATURE_SIZE), 32), and the allocation needs to
 * be aligned on max(getauxval(AT_RSEQ_ALIGN), 32).
 *
 * As an alternative, userspace is allowed to use both the original size
 * and alignment of 32 bytes for backward compatibility.
 *
 * A single active struct rseq registration per thread is allowed.
 */
#[repr(C, align(32))]
pub struct Rseq {
    /* Restartable sequences cpu_id_start field. */
    pub cpu_id_start: u32,
    /* Restartable sequences cpu_id field. */
    pub cpu_id: u32,
    /* Restartable sequences rseq_cs field. */
    pub rseq_cs: u64,
    /* Restartable sequences flags field. */
    pub flags: u32,
    /* Restartable sequences node_id field. */
    pub node_id: u32,
    /* Restartable sequences mm_cid field. */
    pub mm_cid: u32,
    /* Time slice extension control structure. CPU local updates from
     * kernel and user space. */
    pub slice_ctrl: RseqSliceCtrl,
    /* Reserved byte used to bump the rseq feature size from 32 to 33. */
    pub __reserved: u8,
    /* Flexible array member at end of structure, after last feature field. */
    pub end: [i8; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
