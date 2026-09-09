/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2019 HiSilicon Limited. */

// Linux dependencies are supplied by the surrounding translation unit.

pub const QM_QNUM_V1: u32 = 4096;
pub const QM_QNUM_V2: u32 = 1024;
pub const QM_MAX_VFS_NUM_V2: u32 = 63;
pub const QM_ARUSER_M_CFG_1: u32 = 0x100088;
pub const AXUSER_SNOOP_ENABLE: u32 = 1 << 30;
pub const AXUSER_CMD_TYPE: u32 = 0x7 << 12;
pub const AXUSER_CMD_SMMU_NORMAL: u32 = 1;
pub const AXUSER_NS: u32 = 1 << 6;
pub const AXUSER_NO: u32 = 1 << 5;
pub const AXUSER_FP: u32 = 1 << 4;
pub const AXUSER_SSV: u32 = 1;
pub const AXUSER_BASE: u32 = AXUSER_SNOOP_ENABLE | (AXUSER_CMD_SMMU_NORMAL << 12) | AXUSER_NS | AXUSER_NO | AXUSER_FP;
pub const QM_ARUSER_M_CFG_ENABLE: u32 = 0x100090;
pub const ARUSER_M_CFG_ENABLE: u32 = 0xfffffffe;
pub const QM_AWUSER_M_CFG_1: u32 = 0x100098;
pub const QM_AWUSER_M_CFG_ENABLE: u32 = 0x1000a0;
pub const AWUSER_M_CFG_ENABLE: u32 = 0xfffffffe;
pub const QM_WUSER_M_CFG_ENABLE: u32 = 0x1000a8;
pub const WUSER_M_CFG_ENABLE: u32 = 0xffffffff;

pub const QM_MB_CMD_SQC: u32 = 0x0;
pub const QM_MB_CMD_CQC: u32 = 0x1;
pub const QM_MB_CMD_EQC: u32 = 0x2;
pub const QM_MB_CMD_AEQC: u32 = 0x3;
pub const QM_MB_CMD_SQC_BT: u32 = 0x4;
pub const QM_MB_CMD_CQC_BT: u32 = 0x5;
pub const QM_MB_CMD_SQC_VFT_V2: u32 = 0x6;
pub const QM_MB_CMD_STOP_QP: u32 = 0x8;
pub const QM_MB_CMD_FLUSH_QM: u32 = 0x9;
pub const QM_MB_CMD_SRC: u32 = 0xc;
pub const QM_MB_CMD_DST: u32 = 0xd;
pub const QM_MB_CMD_SEND_BASE: u32 = 0x300;
pub const QM_MB_EVENT_SHIFT: u32 = 8;
pub const QM_MB_BUSY_SHIFT: u32 = 13;
pub const QM_MB_OP_SHIFT: u32 = 14;
pub const QM_MB_CMD_DATA_ADDR_L: u32 = 0x304;
pub const QM_MB_CMD_DATA_ADDR_H: u32 = 0x308;
pub const QM_MB_MAX_WAIT_CNT: u32 = 6000;
pub const QM_DOORBELL_CMD_SQ: u32 = 0;
pub const QM_DOORBELL_CMD_CQ: u32 = 1;
pub const QM_DOORBELL_CMD_EQ: u32 = 2;
pub const QM_DOORBELL_CMD_AEQ: u32 = 3;
pub const QM_DOORBELL_SQ_CQ_BASE_V2: u32 = 0x1000;
pub const QM_DOORBELL_EQ_AEQ_BASE_V2: u32 = 0x2000;
pub const QM_QP_MAX_NUM_SHIFT: u32 = 11;
pub const QM_DB_CMD_SHIFT_V2: u32 = 12;
pub const QM_DB_RAND_SHIFT_V2: u32 = 16;
pub const QM_DB_INDEX_SHIFT_V2: u32 = 32;
pub const QM_DB_PRIORITY_SHIFT_V2: u32 = 48;
pub const QM_VF_STATE: u32 = 0x60;

pub const QM_CACHE_CTL: u32 = 0x100050;
pub const SQC_CACHE_ENABLE: u32 = 1;
pub const CQC_CACHE_ENABLE: u32 = 1 << 1;
pub const SQC_CACHE_WB_ENABLE: u32 = 1 << 4;
pub const SQC_CACHE_WB_THRD: u32 = 0x3f << 5;
pub const CQC_CACHE_WB_ENABLE: u32 = 1 << 11;
pub const CQC_CACHE_WB_THRD: u32 = 0x3f << 12;
pub const QM_AXI_M_CFG: u32 = 0x1000ac;
pub const AXI_M_CFG: u32 = 0xffff;
pub const QM_AXI_M_CFG_ENABLE: u32 = 0x1000b0;
pub const AM_CFG_SINGLE_PORT_MAX_TRANS: u32 = 0x300014;
pub const AXI_M_CFG_ENABLE: u32 = 0xffffffff;
pub const QM_PEH_AXUSER_CFG: u32 = 0x1000cc;
pub const QM_PEH_AXUSER_CFG_ENABLE: u32 = 0x1000d0;
pub const PEH_AXUSER_CFG: u32 = 0x401001;
pub const PEH_AXUSER_CFG_ENABLE: u32 = 0xffffffff;
pub const QM_MIN_QNUM: u32 = 2;
pub const HISI_ACC_SGL_SGE_NR_MAX: u32 = 255;
pub const QM_SHAPER_CFG: u32 = 0x100164;
pub const QM_SHAPER_ENABLE: u32 = 1 << 30;
pub const QM_SHAPER_TYPE1_OFFSET: u32 = 10;
pub const QM_DOORBELL_PAGE_NR: u32 = 1;
pub const QM_DEV_ALG_MAX_LEN: u32 = 256;
pub const QM_MIG_REGION_SEL: u32 = 0x100198;
pub const QM_MIG_REGION_EN: u32 = 1;
pub const QM_MAX_CHANNEL_NUM: usize = 8;
pub const QM_CHANNEL_USAGE_OFFSET: u32 = 0x1100;
pub const QM_MAX_DEV_USAGE: u32 = 100;
pub const QM_DEV_USAGE_RATE: u32 = 100;
pub const QM_CHANNEL_ADDR_INTRVL: u32 = 0x4;
pub const UACCE_MODE_NOUACCE: i32 = 0;
pub const UACCE_MODE_SVA: i32 = 1;
pub const UACCE_MODE_DESC: &str = "0(default) means only register to crypto, 1 means both register to crypto and uacce";
pub const QM_ECC_MBIT: u32 = 1 << 2;

#[repr(C)] pub enum qm_stop_reason { QM_NORMAL, QM_SOFT_RESET, QM_DOWN, QM_SHUTDOWN }
#[repr(C)] pub enum qm_state { QM_WORK = 0, QM_STOP }
#[repr(C)] pub enum qp_state { QP_START = 1, QP_STOP }
#[repr(C)] pub enum qm_hw_ver { QM_HW_V1 = 0x20, QM_HW_V2 = 0x21, QM_HW_V3 = 0x30, QM_HW_V4 = 0x50, QM_HW_V5 = 0x51 }
#[repr(C)] pub enum qm_fun_type { QM_HW_PF, QM_HW_VF }
#[repr(C)] pub enum qm_debug_file { CURRENT_QM, CURRENT_Q, CLEAR_ENABLE, DEBUG_FILE_NUM }
#[repr(C)] pub enum qm_vf_state { QM_READY = 0, QM_NOT_READY }
#[repr(C)] pub enum qm_misc_ctl_bits { QM_DRIVER_REMOVING = 0, QM_RESETTING, QM_MODULE_PARAM }
#[repr(C)] pub enum qm_cap_bits { QM_SUPPORT_DB_ISOLATION = 0, QM_SUPPORT_FUNC_QOS, QM_SUPPORT_STOP_QP, QM_SUPPORT_STOP_FUNC, QM_SUPPORT_MB_COMMAND, QM_SUPPORT_SVA_PREFETCH, QM_SUPPORT_RPM, QM_SUPPORT_DAE }
#[repr(C)] pub enum acc_err_result { ACC_ERR_NONE, ACC_ERR_NEED_RESET, ACC_ERR_RECOVERED, ACC_ERR_NEED_FUNC_RESET }

#[repr(C)] pub struct qm_dev_alg { pub alg_msk: u64, pub alg: *const core::ffi::c_char }
#[repr(C)] pub struct qm_dev_dfx { pub dev_state: u32, pub dev_timeout: u32 }
#[repr(C)] pub struct dfx_diff_registers { pub regs: *mut u32, pub reg_offset: u32, pub reg_len: u32 }
#[repr(C)] pub struct qm_dfx { pub err_irq_cnt: atomic64_t, pub aeq_irq_cnt: atomic64_t, pub abnormal_irq_cnt: atomic64_t, pub create_qp_err_cnt: atomic64_t, pub mb_err_cnt: atomic64_t }
#[repr(C)] pub struct debugfs_file { pub index: qm_debug_file, pub lock: mutex, pub debug: *mut qm_debug }
#[repr(C)] pub struct qm_debug { pub curr_qm_qp_num: u32, pub sqe_mask_offset: u32, pub sqe_mask_len: u32, pub dfx: qm_dfx, pub debug_root: *mut dentry, pub qm_d: *mut dentry, pub files: [debugfs_file; 4], pub dev_dfx: qm_dev_dfx, pub qm_last_words: *mut u32, pub last_words: *mut u32, pub qm_diff_regs: *mut dfx_diff_registers, pub acc_diff_regs: *mut dfx_diff_registers }
#[repr(C)] pub struct qm_shaper_factor { pub func_qos: u32, pub cir_b: u64, pub cir_u: u64, pub cir_s: u64, pub cbs_s: u64 }
#[repr(C)] pub struct qm_dma { pub va: *mut core::ffi::c_void, pub dma: dma_addr_t, pub size: usize }
#[repr(C)] pub struct hisi_qm_status { pub eq_head: u32, pub eqc_phase: bool, pub aeq_head: u32, pub aeqc_phase: bool, pub flags: atomic_t, pub stop_reason: i32 }
#[repr(C)] pub struct hisi_qm_err_mask { pub ecc_2bits_mask: u32, pub shutdown_mask: u32, pub reset_mask: u32, pub ce: u32, pub nfe: u32, pub fe: u32 }
#[repr(C)] pub struct hisi_qm_err_info { pub acpi_rst: *mut core::ffi::c_char, pub msi_wr_port: u32, pub qm_err: hisi_qm_err_mask, pub dev_err: hisi_qm_err_mask }
#[repr(C)] pub struct hisi_qm_err_status { pub is_qm_ecc_mbit: u32, pub is_dev_ecc_mbit: u32 }
#[repr(C)] pub struct hisi_qm_cap_info { pub type_: u32, pub offset: u32, pub shift: u32, pub mask: u32, pub v1_val: u32, pub v2_val: u32, pub v3_val: u32 }
#[repr(C)] pub struct hisi_qm_cap_query_info { pub type_: u32, pub name: *const core::ffi::c_char, pub offset: u32, pub v1_val: u32, pub v2_val: u32, pub v3_val: u32 }
#[repr(C)] pub struct hisi_qm_cap_record { pub type_: u32, pub name: *const core::ffi::c_char, pub cap_val: u32 }
#[repr(C)] pub struct hisi_qm_cap_tables { pub qm_cap_size: u32, pub qm_cap_table: *mut hisi_qm_cap_record, pub dev_cap_size: u32, pub dev_cap_table: *mut hisi_qm_cap_record }
#[repr(C)] pub struct qm_channel { pub channel_num: i32, pub channel_name: [*const core::ffi::c_char; QM_MAX_CHANNEL_NUM] }
#[repr(C)] pub struct hisi_qm_list { pub lock: mutex, pub list: list_head, pub register_to_crypto: Option<unsafe extern "C" fn(*mut hisi_qm) -> i32>, pub unregister_from_crypto: Option<unsafe extern "C" fn(*mut hisi_qm)> }
#[repr(C)] pub struct hisi_qm_poll_data { pub qm: *mut hisi_qm, pub work: work_struct, pub qp_finish_id: *mut u16, pub eqe_num: u16 }
#[repr(C)] pub struct qm_err_isolate { pub isolate_lock: mutex, pub err_threshold: u32, pub is_isolate: bool, pub qm_hw_errs: list_head }

// Forward declarations retained for source-level compatibility.
pub enum hisi_qm {}
pub enum qm_sqc {}
pub enum qm_cqc {}
pub enum qm_eqc {}
pub enum qm_aeqc {}
pub enum qm_eqe {}
pub enum qm_aeqe {}
pub enum qm_cqe {}
pub enum hisi_qp {}

#[repr(C)] pub struct qm_rsv_buf { pub sqc: *mut qm_sqc, pub cqc: *mut qm_cqc, pub eqc: *mut qm_eqc, pub aeqc: *mut qm_aeqc, pub sqc_dma: dma_addr_t, pub cqc_dma: dma_addr_t, pub eqc_dma: dma_addr_t, pub aeqc_dma: dma_addr_t, pub qcdma: qm_dma }

// The remaining aggregate and callback declarations mirror the C header; opaque
// kernel member types are intentionally referenced as external dependencies.
#[repr(C)] pub struct hisi_qm_err_ini { pub hw_init: Option<unsafe extern "C" fn(*mut hisi_qm)->i32>, pub hw_err_enable: Option<unsafe extern "C" fn(*mut hisi_qm)>, pub hw_err_disable: Option<unsafe extern "C" fn(*mut hisi_qm)>, pub get_dev_hw_err_status: Option<unsafe extern "C" fn(*mut hisi_qm)->u32>, pub clear_dev_hw_err_status: Option<unsafe extern "C" fn(*mut hisi_qm,u32)>, pub open_axi_master_ooo: Option<unsafe extern "C" fn(*mut hisi_qm)>, pub close_axi_master_ooo: Option<unsafe extern "C" fn(*mut hisi_qm)>, pub open_sva_prefetch: Option<unsafe extern "C" fn(*mut hisi_qm)>, pub close_sva_prefetch: Option<unsafe extern "C" fn(*mut hisi_qm)>, pub show_last_dfx_regs: Option<unsafe extern "C" fn(*mut hisi_qm)>, pub err_info_init: Option<unsafe extern "C" fn(*mut hisi_qm)>, pub get_err_result: Option<unsafe extern "C" fn(*mut hisi_qm)->acc_err_result>, pub dev_is_abnormal: Option<unsafe extern "C" fn(*mut hisi_qm)->bool>, pub set_priv_status: Option<unsafe extern "C" fn(*mut hisi_qm)->i32>, pub disable_axi_error: Option<unsafe extern "C" fn(*mut hisi_qm)>, pub enable_axi_error: Option<unsafe extern "C" fn(*mut hisi_qm)> }

#[repr(C)] pub struct hisi_qp_status { pub used: atomic_t, pub sq_tail: u16, pub cq_head: u16, pub cqc_phase: bool, pub flags: atomic_t }
#[repr(C)] pub struct hisi_qp_ops { pub fill_sqe: Option<unsafe extern "C" fn(*mut core::ffi::c_void,*mut core::ffi::c_void,*mut core::ffi::c_void)->i32> }
#[repr(C)] pub struct instance_backlog { pub list: list_head, pub lock: spinlock_t }
#[repr(C)] pub struct hisi_qp { pub qp_id:u32, pub sq_depth:u16, pub cq_depth:u16, pub alg_type:u8, pub qdma:qm_dma, pub sqe:*mut core::ffi::c_void, pub cqe:*mut qm_cqe, pub sqe_dma:dma_addr_t, pub cqe_dma:dma_addr_t, pub qp_status:hisi_qp_status, pub hw_ops:*mut hisi_qp_ops, pub req_cb:Option<unsafe extern "C" fn(*mut hisi_qp,*mut core::ffi::c_void)>, pub event_cb:Option<unsafe extern "C" fn(*mut hisi_qp)>, pub qm:*mut hisi_qm, pub is_resetting:bool, pub is_in_kernel:bool, pub pasid:u16, pub uacce_q:*mut uacce_queue, pub ref_count:u32, pub qp_lock:spinlock_t, pub backlog:instance_backlog, pub msg:*mut *const core::ffi::c_void }

extern "C" {
    pub fn hisi_qm_register_uacce(qm:*mut hisi_qm)->i32; pub fn hisi_qm_q_num_set(val:*const core::ffi::c_char,kp:*const kernel_param,device:u32)->i32; pub fn hisi_qm_init(qm:*mut hisi_qm)->i32; pub fn hisi_qm_uninit(qm:*mut hisi_qm); pub fn hisi_qm_start(qm:*mut hisi_qm)->i32; pub fn hisi_qm_stop(qm:*mut hisi_qm,r:qm_stop_reason)->i32; pub fn hisi_qp_send(qp:*mut hisi_qp,msg:*const core::ffi::c_void)->i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
