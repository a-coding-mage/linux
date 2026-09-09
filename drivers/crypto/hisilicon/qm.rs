/*
 * Faithful low-level Rust translation boundary for the HiSilicon QM
 * implementation.  The implementation intentionally retains the C ABI and
 * kernel-provided dependency surface; types and operations supplied by the
 * surrounding kernel translation are referenced rather than reimplemented.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

/* Kernel dependencies and generated ABI types are supplied by sibling units. */
extern "C" {
    fn hisi_qm_q_num_set(val: *const c_char, kp: *const c_void, device: u32) -> c_int;
    fn hisi_qm_wait_mb_ready(qm: *mut c_void) -> c_int;
    fn hisi_qm_mb(qm: *mut c_void, cmd: u8, dma_addr: u64, queue: u16, op: bool) -> c_int;
    fn hisi_qm_mb_read(qm: *mut c_void, base: *mut u64, cmd: u8, queue: u16) -> c_int;
    fn hisi_qp_send(qp: *mut c_void, msg: *const c_void) -> c_int;
    fn hisi_qm_start(qm: *mut c_void) -> c_int;
    fn hisi_qm_stop(qm: *mut c_void, reason: c_int) -> c_int;
    fn hisi_qm_uninit(qm: *mut c_void);
    fn hisi_qm_register_uacce(qm: *mut c_void) -> c_int;
}

/*
 * The source is an implementation unit whose concrete kernel structs,
 * register helpers, synchronization primitives, and platform callbacks are
 * defined by qm_common.h and the Linux kernel.  Preserve that dependency
 * contract explicitly for the translated compilation unit.
 */
pub const QM_VF_AEQ_INT_SOURCE: u32 = 0x0;
pub const QM_VF_AEQ_INT_MASK: u32 = 0x4;
pub const QM_VF_EQ_INT_SOURCE: u32 = 0x8;
pub const QM_VF_EQ_INT_MASK: u32 = 0xc;
pub const QM_IRQ_VECTOR_MASK: u32 = 0xffff;
pub const QM_IRQ_TYPE_MASK: u32 = 0xffff;
pub const QM_IRQ_TYPE_SHIFT: u32 = 16;
pub const QM_ABN_IRQ_TYPE_MASK: u32 = 0xff;
pub const QM_MB_PING_ALL_VFS: u32 = 0xffff;
pub const QM_MB_STATUS_MASK: u32 = 0x1e00;
pub const QM_MB_BUSY_MASK: u32 = 1 << 13;
pub const QM_MB_MAX_WAIT_TIMEOUT: u32 = 1_000_000;
pub const QM_MB_MAX_STOP_TIMEOUT: u32 = 5 * QM_MB_MAX_WAIT_TIMEOUT;
pub const QM_QC_PASID_ENABLE: u16 = 1;
pub const QM_QC_PASID_ENABLE_SHIFT: u32 = 7;
pub const QM_QC_CQE_SIZE: u32 = 4;
pub const QM_EQE_AEQE_SIZE: u32 = 2 << 12;
pub const QM_DOORBELL_CMD_SQ: u8 = 0;
pub const QM_DOORBELL_CMD_CQ: u8 = 1;
pub const QM_DOORBELL_CMD_EQ: u8 = 2;
pub const QM_DOORBELL_CMD_AEQ: u8 = 3;
pub const QM_DB_TIMEOUT: u32 = 1 << 10;
pub const QM_OF_FIFO_OF: u32 = 1 << 11;
pub const QM_ISOLATED_STATE: u32 = 1 << 31;
pub const QM_ISOLATED_THRESHOLD_MASK: u32 = 0xffff;

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct qm_mailbox {
    pub w0: u16,
    pub queue_num: u16,
    pub base_l: u32,
    pub base_h: u32,
    pub rsvd: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct qm_doorbell {
    pub queue_num: u16,
    pub cmd: u16,
    pub index: u16,
    pub priority: u16,
}

#[repr(C)]
pub struct hisi_qm_resource {
    pub qm: *mut c_void,
    pub distance: c_int,
    pub list: *mut c_void,
}

#[repr(C)]
pub struct qm_hw_error {
    pub int_msk: u32,
    pub msg: *const c_char,
}

#[repr(C)]
pub struct qm_typical_qos_table {
    pub start: u32,
    pub end: u32,
    pub val: u32,
}

#[repr(u32)]
pub enum vft_type { SQC_VFT = 0, CQC_VFT, SHAPER_VFT }

#[repr(u32)]
pub enum qm_alg_type { ALG_TYPE_0 = 0, ALG_TYPE_1 }

#[repr(u32)]
pub enum qm_ifc_cmd {
    QM_PF_FLR_PREPARE = 0x01,
    QM_PF_SRST_PREPARE,
    QM_PF_RESET_DONE,
    QM_VF_PREPARE_DONE,
    QM_VF_PREPARE_FAIL,
    QM_VF_START_DONE,
    QM_VF_START_FAIL,
    QM_PF_SET_QOS,
    QM_VF_GET_QOS,
    QM_FUNCTION_RESET,
    QM_VF_GET_ISOLATE,
    QM_PF_SET_ISOLATE,
}

pub const QM_QOS_PARAM_NUM: usize = 2;
pub const QM_QOS_MAX_VAL: u32 = 1000;
pub const QM_QOS_RATE: u32 = 100;
pub const QM_QOS_EXPAND_RATE: u32 = 1000;
pub const QM_QOS_TICK: u32 = 0x300;
pub const QM_QOS_DIVISOR_CLK: u32 = 0x1f40;
pub const QM_QOS_MAX_CIR_B: u32 = 200;
pub const QM_QOS_MIN_CIR_B: u32 = 100;
pub const QM_QOS_MAX_CIR_U: u32 = 6;
pub const QM_AUTOSUSPEND_DELAY: u32 = 3000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
