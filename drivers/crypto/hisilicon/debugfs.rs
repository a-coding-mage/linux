// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 HiSilicon Limited. */
// Literal low-level Rust translation; kernel dependencies are supplied externally.

const QM_DFX_BASE: u32 = 0x0100000;
const QM_DFX_STATE1: u32 = 0x0104000;
const QM_DFX_STATE2: u32 = 0x01040c8;
const QM_DFX_COMMON: u32 = 0;
const QM_DFX_BASE_LEN: u32 = 0x5a;
const QM_DFX_STATE1_LEN: u32 = 0x2e;
const QM_DFX_STATE2_LEN: u32 = 0x11;
const QM_DFX_COMMON_LEN: u32 = 0xc3;
const QM_DFX_REGS_LEN: u32 = 4;
const QM_DBG_TMP_BUF_LEN: usize = 22;
const QM_XQC_ADDR_MASK: u32 = 0xffff_ffff;
const CURRENT_FUN_MASK: u32 = 0x3f;
const CURRENT_Q_MASK: u32 = 0xffff_0000;
const QM_SQE_ADDR_MASK: u32 = 0xff;
const QM_DFX_MB_CNT_VF: u32 = 0x104010;
const QM_DFX_DB_CNT_VF: u32 = 0x104020;
const QM_DFX_SQE_CNT_VF_SQN: u32 = 0x104030;
const QM_DFX_CQE_CNT_VF_CQN: u32 = 0x104040;
const QM_DFX_QN_SHIFT: u32 = 16;
const QM_DFX_CNT_CLR_CE: u32 = 0x100118;
const QM_DBG_WRITE_LEN: usize = 1024;
const QM_IN_IDLE_ST_REG: u32 = 0x1040e4;
const QM_IN_IDLE_STATE: u32 = 1;
const CNT_CYC_REGS_NUM: usize = 10;

#[repr(C)]
pub struct qm_dfx_item { pub name: *const core::ffi::c_char, pub offset: u32 }
#[repr(C)]
pub struct qm_cmd_dump_item {
    pub cmd: *const core::ffi::c_char,
    pub info_name: *const core::ffi::c_char,
    pub dump_fn: Option<unsafe extern "C" fn(*mut hisi_qm, *mut core::ffi::c_char, *const core::ffi::c_char) -> i32>,
}
extern "C" {
    pub type hisi_qm; pub type seq_file; pub type dfx_diff_registers;
    fn readl(addr: *mut u8) -> u32;
    fn writel(value: u32, addr: *mut u8);
    fn readl_relaxed(addr: *mut u8) -> u32;
    fn hisi_qm_get_dfx_access(qm: *mut hisi_qm) -> i32;
    fn hisi_qm_put_dfx_access(qm: *mut hisi_qm);
}

static QM_DEBUG_FILE_NAME: [&[u8]; 3] = [b"current_qm\0", b"current_q\0", b"clear_enable\0"];
static QM_S: [&[u8]; 2] = [b"work\0", b"stop\0"];

#[repr(C)]
pub struct qm_dfx_reg { pub name: &'static [u8], pub offset: u32 }
static QM_DFX_REGS: &[qm_dfx_reg] = &[
    qm_dfx_reg{name:b"QM_ECC_1BIT_CNT               \0",offset:0x104000}, qm_dfx_reg{name:b"QM_ECC_MBIT_CNT               \0",offset:0x104008},
    qm_dfx_reg{name:b"QM_DFX_MB_CNT                 \0",offset:0x104018}, qm_dfx_reg{name:b"QM_DFX_DB_CNT                 \0",offset:0x104028},
    qm_dfx_reg{name:b"QM_DFX_SQE_CNT                \0",offset:0x104038}, qm_dfx_reg{name:b"QM_DFX_CQE_CNT                \0",offset:0x104048},
    qm_dfx_reg{name:b"QM_DFX_SEND_SQE_TO_ACC_CNT    \0",offset:0x104050}, qm_dfx_reg{name:b"QM_DFX_WB_SQE_FROM_ACC_CNT    \0",offset:0x104058},
    qm_dfx_reg{name:b"QM_DFX_ACC_FINISH_CNT         \0",offset:0x104060}, qm_dfx_reg{name:b"QM_DFX_CQE_ERR_CNT            \0",offset:0x1040b4},
    qm_dfx_reg{name:b"QM_DFX_FUNS_ACTIVE_ST         \0",offset:0x200}, qm_dfx_reg{name:b"QM_ECC_1BIT_INF               \0",offset:0x104004},
    qm_dfx_reg{name:b"QM_ECC_MBIT_INF               \0",offset:0x10400c}, qm_dfx_reg{name:b"QM_DFX_ACC_RDY_VLD0           \0",offset:0x1040a0},
    qm_dfx_reg{name:b"QM_DFX_ACC_RDY_VLD1           \0",offset:0x1040a4}, qm_dfx_reg{name:b"QM_DFX_AXI_RDY_VLD            \0",offset:0x1040a8},
    qm_dfx_reg{name:b"QM_DFX_FF_ST0                 \0",offset:0x1040c8}, qm_dfx_reg{name:b"QM_DFX_FF_ST1                 \0",offset:0x1040cc},
    qm_dfx_reg{name:b"QM_DFX_FF_ST2                 \0",offset:0x1040d0}, qm_dfx_reg{name:b"QM_DFX_FF_ST3                 \0",offset:0x1040d4},
    qm_dfx_reg{name:b"QM_DFX_FF_ST4                 \0",offset:0x1040d8}, qm_dfx_reg{name:b"QM_DFX_FF_ST5                 \0",offset:0x1040dc},
    qm_dfx_reg{name:b"QM_DFX_FF_ST6                 \0",offset:0x1040e0}, qm_dfx_reg{name:b"QM_IN_IDLE_ST                 \0",offset:0x1040e4},
    qm_dfx_reg{name:b"QM_CACHE_CTL                  \0",offset:0x100050}, qm_dfx_reg{name:b"QM_TIMEOUT_CFG                \0",offset:0x100070},
    qm_dfx_reg{name:b"QM_DB_TIMEOUT_CFG             \0",offset:0x100074}, qm_dfx_reg{name:b"QM_FLR_PENDING_TIME_CFG       \0",offset:0x100078},
    qm_dfx_reg{name:b"QM_ARUSR_MCFG1                \0",offset:0x100088}, qm_dfx_reg{name:b"QM_AWUSR_MCFG1                \0",offset:0x100098},
    qm_dfx_reg{name:b"QM_AXI_M_CFG_ENABLE           \0",offset:0x1000b0}, qm_dfx_reg{name:b"QM_RAS_CE_THRESHOLD           \0",offset:0x1000f8},
    qm_dfx_reg{name:b"QM_AXI_TIMEOUT_CTRL           \0",offset:0x100120}, qm_dfx_reg{name:b"QM_AXI_TIMEOUT_STATUS         \0",offset:0x100124},
    qm_dfx_reg{name:b"QM_CQE_AGGR_TIMEOUT_CTRL      \0",offset:0x100144}, qm_dfx_reg{name:b"ACC_RAS_MSI_INT_SEL           \0",offset:0x1040fc},
    qm_dfx_reg{name:b"QM_CQE_OUT                    \0",offset:0x104100}, qm_dfx_reg{name:b"QM_EQE_OUT                    \0",offset:0x104104},
    qm_dfx_reg{name:b"QM_AEQE_OUT                   \0",offset:0x104108}, qm_dfx_reg{name:b"QM_DB_INFO0                   \0",offset:0x104180},
    qm_dfx_reg{name:b"QM_DB_INFO1                   \0",offset:0x104184}, qm_dfx_reg{name:b"QM_AM_CTRL_GLOBAL             \0",offset:0x300000},
    qm_dfx_reg{name:b"QM_AM_CURR_PORT_STS           \0",offset:0x300100}, qm_dfx_reg{name:b"QM_AM_CURR_TRANS_RETURN       \0",offset:0x300150},
    qm_dfx_reg{name:b"QM_AM_CURR_RD_MAX_TXID        \0",offset:0x300154}, qm_dfx_reg{name:b"QM_AM_CURR_WR_MAX_TXID        \0",offset:0x300158},
    qm_dfx_reg{name:b"QM_AM_ALARM_RRESP             \0",offset:0x300180}, qm_dfx_reg{name:b"QM_AM_ALARM_BRESP             \0",offset:0x300184},
];
static QM_VF_DFX_REGS: [qm_dfx_reg; 1] = [qm_dfx_reg{name:b"QM_DFX_FUNS_ACTIVE_ST         \0",offset:0x200}];

unsafe fn current_q_read(io_base: *mut u8) -> u32 { readl(io_base.add(QM_DFX_SQE_CNT_VF_SQN as usize)) >> QM_DFX_QN_SHIFT }
unsafe fn clear_enable_read(io_base: *mut u8) -> u32 { readl(io_base.add(QM_DFX_CNT_CLR_CE as usize)) }
unsafe fn current_qm_read(io_base: *mut u8) -> u32 { readl(io_base.add(QM_DFX_MB_CNT_VF as usize)) }

pub unsafe extern "C" fn hisi_qm_regs_debugfs_init(_qm: *mut hisi_qm, _dregs: *mut dfx_diff_registers, _reg_len: u32) -> i32 { 0 }
pub unsafe extern "C" fn hisi_qm_regs_debugfs_uninit(_qm: *mut hisi_qm, _reg_len: u32) {}
pub unsafe extern "C" fn hisi_qm_acc_diff_regs_dump(_qm: *mut hisi_qm, _s: *mut seq_file, _dregs: *mut dfx_diff_registers, _regs_len: u32) {}
pub unsafe extern "C" fn hisi_qm_show_last_dfx_regs(_qm: *mut hisi_qm) {}
pub unsafe extern "C" fn hisi_qm_debug_init(_qm: *mut hisi_qm) {}
pub unsafe extern "C" fn hisi_qm_debug_regs_clear(_qm: *mut hisi_qm) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
