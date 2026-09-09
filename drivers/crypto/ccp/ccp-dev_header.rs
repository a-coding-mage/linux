/* SPDX-License-Identifier: GPL-2.0-only */
/* AMD Cryptographic Coprocessor (CCP) driver header translation. */

// External Linux/project types referenced by this header are supplied by dependencies.

pub const MAX_CCP_NAME_LEN: usize = 16;
pub const MAX_DMAPOOL_NAME_LEN: usize = 32;
pub const MAX_HW_QUEUES: usize = 5;
pub const MAX_CMD_QLEN: usize = 100;
pub const TRNG_RETRIES: u32 = 10;
pub const CACHE_NONE: u32 = 0x00;
pub const CACHE_WB_NO_ALLOC: u32 = 0xb7;
pub const Q_MASK_REG: u32 = 0x000;
pub const TRNG_OUT_REG: u32 = 0x00c;
pub const IRQ_MASK_REG: u32 = 0x040;
pub const IRQ_STATUS_REG: u32 = 0x200;
pub const DEL_CMD_Q_JOB: u32 = 0x124;
pub const DEL_Q_ACTIVE: u32 = 0x00000200;
pub const DEL_Q_ID_SHIFT: u32 = 6;
pub const CMD_REQ0: u32 = 0x180;
pub const CMD_REQ_INCR: u32 = 0x04;
pub const CMD_Q_STATUS_BASE: u32 = 0x210;
pub const CMD_Q_INT_STATUS_BASE: u32 = 0x214;
pub const CMD_Q_STATUS_INCR: u32 = 0x20;
pub const CMD_Q_CACHE_BASE: u32 = 0x228;
pub const CMD_Q_CACHE_INC: u32 = 0x20;

#[inline] pub const fn CMD_Q_ERROR(qs: u32) -> u32 { qs & 0x0000003f }
#[inline] pub const fn CMD_Q_DEPTH(qs: u32) -> u32 { (qs >> 12) & 0x0000000f }

pub const CMD5_QUEUE_MASK_OFFSET: u32 = 0x00;
pub const CMD5_QUEUE_PRIO_OFFSET: u32 = 0x04;
pub const CMD5_REQID_CONFIG_OFFSET: u32 = 0x08;
pub const CMD5_CMD_TIMEOUT_OFFSET: u32 = 0x10;
pub const LSB_PUBLIC_MASK_LO_OFFSET: u32 = 0x18;
pub const LSB_PUBLIC_MASK_HI_OFFSET: u32 = 0x1C;
pub const LSB_PRIVATE_MASK_LO_OFFSET: u32 = 0x20;
pub const LSB_PRIVATE_MASK_HI_OFFSET: u32 = 0x24;
pub const CMD5_PSP_CCP_VERSION: u32 = 0x100;
pub const CMD5_Q_CONTROL_BASE: u32 = 0x0000;
pub const CMD5_Q_TAIL_LO_BASE: u32 = 0x0004;
pub const CMD5_Q_HEAD_LO_BASE: u32 = 0x0008;
pub const CMD5_Q_INT_ENABLE_BASE: u32 = 0x000C;
pub const CMD5_Q_INTERRUPT_STATUS_BASE: u32 = 0x0010;
pub const CMD5_Q_STATUS_BASE: u32 = 0x0100;
pub const CMD5_Q_INT_STATUS_BASE: u32 = 0x0104;
pub const CMD5_Q_DMA_STATUS_BASE: u32 = 0x0108;
pub const CMD5_Q_DMA_READ_STATUS_BASE: u32 = 0x010C;
pub const CMD5_Q_DMA_WRITE_STATUS_BASE: u32 = 0x0110;
pub const CMD5_Q_ABORT_BASE: u32 = 0x0114;
pub const CMD5_Q_AX_CACHE_BASE: u32 = 0x0118;
pub const CMD5_CONFIG_0_OFFSET: u32 = 0x6000;
pub const CMD5_TRNG_CTL_OFFSET: u32 = 0x6008;
pub const CMD5_AES_MASK_OFFSET: u32 = 0x6010;
pub const CMD5_CLK_GATE_CTL_OFFSET: u32 = 0x603C;
pub const CMD5_Q_STATUS_INCR: u32 = 0x1000;
pub const CMD5_Q_RUN: u32 = 0x1;
pub const CMD5_Q_HALT: u32 = 0x2;
pub const CMD5_Q_MEM_LOCATION: u32 = 0x4;
pub const CMD5_Q_SIZE: u32 = 0x1F;
pub const CMD5_Q_SHIFT: u32 = 3;
pub const COMMANDS_PER_QUEUE: u32 = 16;
pub const QUEUE_SIZE_VAL: u32 = ((32 - COMMANDS_PER_QUEUE.leading_zeros() - 1) - 2) & CMD5_Q_SIZE;
pub const Q_PTR_MASK: u32 = (2 << (QUEUE_SIZE_VAL + 5)) - 1;
pub const Q_SIZE: fn(u32) -> u32 = |n| COMMANDS_PER_QUEUE * n;
pub const INT_COMPLETION: u32 = 0x1;
pub const INT_ERROR: u32 = 0x2;
pub const INT_QUEUE_STOPPED: u32 = 0x4;
pub const INT_EMPTY_QUEUE: u32 = 0x8;
pub const SUPPORTED_INTERRUPTS: u32 = INT_COMPLETION | INT_ERROR;
pub const LSB_REGION_WIDTH: u32 = 5;
pub const MAX_LSB_CNT: usize = 8;
pub const LSB_SIZE: usize = 16;
pub const LSB_ITEM_SIZE: usize = 32;
pub const PLSB_MAP_SIZE: usize = LSB_SIZE;
pub const SLSB_MAP_SIZE: usize = MAX_LSB_CNT * LSB_SIZE;
#[inline] pub const fn LSB_ENTRY_NUMBER(addr: u32) -> u32 { addr / LSB_ITEM_SIZE as u32 }

pub const REQ0_WAIT_FOR_WRITE: u32 = 0x00000004;
pub const REQ0_INT_ON_COMPLETE: u32 = 0x00000002;
pub const REQ0_STOP_ON_COMPLETE: u32 = 0x00000001;
pub const REQ0_CMD_Q_SHIFT: u32 = 9;
pub const REQ0_JOBID_SHIFT: u32 = 3;
pub const REQ1_PROTECT_SHIFT: u32 = 27;
pub const REQ1_ENGINE_SHIFT: u32 = 23;
pub const REQ1_KEY_KSB_SHIFT: u32 = 2;
pub const REQ1_EOM: u32 = 0x2;
pub const REQ1_INIT: u32 = 0x1;
pub const REQ1_AES_TYPE_SHIFT: u32 = 21;
pub const REQ1_AES_MODE_SHIFT: u32 = 18;
pub const REQ1_AES_ACTION_SHIFT: u32 = 17;
pub const REQ1_AES_CFB_SIZE_SHIFT: u32 = 10;
pub const REQ1_XTS_AES_SIZE_SHIFT: u32 = 10;
pub const REQ1_SHA_TYPE_SHIFT: u32 = 21;
pub const REQ1_RSA_MOD_SIZE_SHIFT: u32 = 10;
pub const REQ1_PT_BW_SHIFT: u32 = 12;
pub const REQ1_PT_BS_SHIFT: u32 = 10;
pub const REQ1_ECC_AFFINE_CONVERT: u32 = 0x00200000;
pub const REQ1_ECC_FUNCTION_SHIFT: u32 = 18;
pub const REQ4_KSB_SHIFT: u32 = 18;
pub const REQ4_MEMTYPE_SHIFT: u32 = 16;
pub const REQ6_MEMTYPE_SHIFT: u32 = 16;
pub const KSB_START: u32 = 77;
pub const KSB_END: u32 = 127;
pub const KSB_COUNT: u32 = KSB_END - KSB_START + 1;
pub const CCP_SB_BITS: u32 = 256;
pub const CCP_JOBID_MASK: u32 = 0x0000003f;
pub const CCP_DMA_DFLT: u32 = 0;
pub const CCP_DMA_PRIV: u32 = 1;
pub const CCP_DMA_PUB: u32 = 2;
pub const CCP_DMAPOOL_MAX_SIZE: usize = 64;
pub const CCP_DMAPOOL_ALIGN: u32 = 1 << 5;
pub const CCP_REVERSE_BUF_SIZE: usize = 64;
pub const CCP_AES_KEY_SB_COUNT: u32 = 1;
pub const CCP_AES_CTX_SB_COUNT: u32 = 1;
pub const CCP_XTS_AES_KEY_SB_COUNT: u32 = 1;
pub const CCP5_XTS_AES_KEY_SB_COUNT: u32 = 2;
pub const CCP_XTS_AES_CTX_SB_COUNT: u32 = 1;
pub const CCP_DES3_KEY_SB_COUNT: u32 = 1;
pub const CCP_DES3_CTX_SB_COUNT: u32 = 1;
pub const CCP_SHA_SB_COUNT: u32 = 1;
pub const CCP_RSA_MAX_WIDTH: u32 = 4096;
pub const CCP5_RSA_MAX_WIDTH: u32 = 16384;
pub const CCP_PASSTHRU_BLOCKSIZE: u32 = 256;
pub const CCP_PASSTHRU_MASKSIZE: u32 = 32;
pub const CCP_PASSTHRU_SB_COUNT: u32 = 1;
pub const CCP_ECC_MODULUS_BYTES: u32 = 48; // 384-bits
pub const CCP_ECC_MAX_OPERANDS: u32 = 6;
pub const CCP_ECC_MAX_OUTPUTS: u32 = 3;
pub const CCP_ECC_SRC_BUF_SIZE: u32 = 448;
pub const CCP_ECC_DST_BUF_SIZE: u32 = 192;
pub const CCP_ECC_OPERAND_SIZE: u32 = 64;
pub const CCP_ECC_OUTPUT_SIZE: u32 = 64;
pub const CCP_ECC_RESULT_OFFSET: u32 = 60;
pub const CCP_ECC_RESULT_SUCCESS: u32 = 0x0001;
pub const CCP_SB_BYTES: u32 = 32;

pub struct ccp_op;
pub struct ccp_device;
pub struct ccp_cmd;
pub struct ccp_fns;

#[repr(C)] pub struct ccp_dma_cmd { pub entry: list_head, pub ccp_cmd: ccp_cmd }
#[repr(C)] pub struct ccp_dma_desc { pub entry: list_head, pub ccp: *mut ccp_device, pub pending: list_head, pub active: list_head, pub status: dma_status, pub tx_desc: dma_async_tx_descriptor, pub len: usize }
#[repr(C)] pub struct ccp_dma_chan { pub ccp: *mut ccp_device, pub lock: spinlock_t, pub created: list_head, pub pending: list_head, pub active: list_head, pub complete: list_head, pub cleanup_tasklet: tasklet_struct, pub status: dma_status, pub dma_chan: dma_chan }

#[repr(C)] pub struct ccp_cmd_queue {
    pub ccp: *mut ccp_device, pub id: u32, pub dma_pool: *mut dma_pool, pub qbase: *mut ccp5_desc,
    pub q_mutex: mutex, pub qidx: u32, pub qsize: u32, pub qbase_dma: dma_addr_t, pub qdma_tail: dma_addr_t,
    pub sb_key: u32, pub sb_ctx: u32, pub lsbmask: [u64; (MAX_LSB_CNT + 63) / 64], pub lsb: i32,
    pub lsbmap: [u64; (PLSB_MAP_SIZE + 63) / 64], pub kthread: *mut task_struct, pub active: u32, pub suspended: u32,
    pub free_slots: u32, pub int_ok: u32, pub int_err: u32, pub reg_control: *mut core::ffi::c_void, pub reg_tail_lo: *mut core::ffi::c_void,
    pub reg_head_lo: *mut core::ffi::c_void, pub reg_int_enable: *mut core::ffi::c_void, pub reg_interrupt_status: *mut core::ffi::c_void,
    pub reg_status: *mut core::ffi::c_void, pub reg_int_status: *mut core::ffi::c_void, pub reg_dma_status: *mut core::ffi::c_void,
    pub reg_dma_read_status: *mut core::ffi::c_void, pub reg_dma_write_status: *mut core::ffi::c_void, pub qcontrol: u32,
    pub int_status: u32, pub q_status: u32, pub q_int_status: u32, pub cmd_error: u32, pub int_queue: wait_queue_head_t, pub int_rcvd: u32,
    pub total_ops: usize, pub total_aes_ops: usize, pub total_xts_aes_ops: usize, pub total_3des_ops: usize, pub total_sha_ops: usize,
    pub total_rsa_ops: usize, pub total_pt_ops: usize, pub total_ecc_ops: usize,
}

#[repr(C)] pub struct ccp_device {
    pub entry: list_head, pub vdata: *mut ccp_vdata, pub ord: u32, pub name: [u8; MAX_CCP_NAME_LEN], pub rngname: [u8; MAX_CCP_NAME_LEN],
    pub dev: *mut device, pub sp: *mut sp_device, pub dev_specific: *mut core::ffi::c_void, pub qim: u32, pub irq: u32, pub use_tasklet: bool, pub irq_tasklet: tasklet_struct,
    pub req_mutex: mutex, pub io_regs: *mut core::ffi::c_void, pub cmd_lock: spinlock_t, pub cmd_count: u32, pub cmd: list_head, pub backlog: list_head,
    pub cmd_q: [ccp_cmd_queue; MAX_HW_QUEUES], pub cmd_q_count: u32, pub max_q_count: u32, pub hwrng: hwrng, pub hwrng_retries: u32,
    pub dma_dev: dma_device, pub ccp_dma_chan: *mut ccp_dma_chan, pub dma_cmd_cache: *mut kmem_cache, pub dma_desc_cache: *mut kmem_cache,
    pub current_id: atomic_t, pub sb_mutex: mutex, pub sb: [u64; (KSB_COUNT as usize + 63) / 64], pub sb_queue: wait_queue_head_t,
    pub sb_avail: u32, pub sb_count: u32, pub sb_start: u32, pub lsbmap: [u64; (SLSB_MAP_SIZE + 63) / 64], pub suspending: u32,
    pub suspend_queue: wait_queue_head_t, pub axcache: u32, pub total_interrupts: usize, pub debugfs_instance: *mut dentry,
}

#[repr(C)] pub enum ccp_memtype { CCP_MEMTYPE_SYSTEM = 0, CCP_MEMTYPE_SB, CCP_MEMTYPE_LOCAL, CCP_MEMTYPE__LAST }
pub const CCP_MEMTYPE_LSB: ccp_memtype = ccp_memtype::CCP_MEMTYPE_SB;
#[repr(C, packed)] pub struct ccp_dma_info { pub address: dma_addr_t, pub offset: u32, pub length: u32, pub dir: dma_data_direction }
#[repr(C)] pub struct ccp_dm_workarea { pub dev: *mut device, pub dma_pool: *mut dma_pool, pub address: *mut u8, pub dma: ccp_dma_info, pub length: u32 }
#[repr(C)] pub struct ccp_sg_workarea { pub sg: *mut scatterlist, pub nents: i32, pub sg_used: u32, pub dma_sg: *mut scatterlist, pub dma_sg_head: *mut scatterlist, pub dma_dev: *mut device, pub dma_count: u32, pub dma_dir: dma_data_direction, pub bytes_left: u64 }
#[repr(C)] pub struct ccp_data { pub sg_wa: ccp_sg_workarea, pub dm_wa: ccp_dm_workarea }
#[repr(C)] pub union ccp_mem_u { pub dma: ccp_dma_info, pub sb: u32 }
#[repr(C)] pub struct ccp_mem { pub r#type: ccp_memtype, pub u: ccp_mem_u }
#[repr(C)] pub struct ccp_aes_op { pub r#type: ccp_aes_type, pub mode: ccp_aes_mode, pub action: ccp_aes_action, pub size: u32 }
#[repr(C)] pub struct ccp_xts_aes_op { pub r#type: ccp_aes_type, pub action: ccp_aes_action, pub unit_size: ccp_xts_aes_unit_size }
#[repr(C)] pub struct ccp_des3_op { pub r#type: ccp_des3_type, pub mode: ccp_des3_mode, pub action: ccp_des3_action }
#[repr(C)] pub struct ccp_sha_op { pub r#type: ccp_sha_type, pub msg_bits: u64 }
#[repr(C)] pub struct ccp_rsa_op { pub mod_size: u32, pub input_len: u32 }
#[repr(C)] pub struct ccp_passthru_op { pub bit_mod: ccp_passthru_bitwise, pub byte_swap: ccp_passthru_byteswap }
#[repr(C)] pub struct ccp_ecc_op { pub function: ccp_ecc_function }
#[repr(C)] pub union ccp_op_u { pub aes: ccp_aes_op, pub xts: ccp_xts_aes_op, pub des3: ccp_des3_op, pub sha: ccp_sha_op, pub rsa: ccp_rsa_op, pub passthru: ccp_passthru_op, pub ecc: ccp_ecc_op }
#[repr(C)] pub struct ccp_op { pub cmd_q: *mut ccp_cmd_queue, pub jobid: u32, pub ioc: u32, pub soc: u32, pub sb_key: u32, pub sb_ctx: u32, pub init: u32, pub eom: u32, pub src: ccp_mem, pub dst: ccp_mem, pub exp: ccp_mem, pub u: ccp_op_u }

#[inline] pub unsafe fn ccp_addr_lo(info: *mut ccp_dma_info) -> u32 { ((*info).address.wrapping_add((*info).offset as _)) as u32 }
#[inline] pub unsafe fn ccp_addr_hi(info: *mut ccp_dma_info) -> u32 { ((((*info).address.wrapping_add((*info).offset as _)) >> 32) as u32) & 0x0000ffff }

#[repr(C)] pub struct dword0 { pub soc: u32, pub ioc: u32, pub rsvd1: u32, pub init: u32, pub eom: u32, pub function: u32, pub engine: u32, pub prot: u32, pub rsvd2: u32 }
#[repr(C)] pub struct dword3 { pub src_hi: u32, pub src_mem: u32, pub lsb_cxt_id: u32, pub rsvd1: u32, pub fixed: u32 }
#[repr(C)] pub union dword4 { pub dst_lo: u32, pub sha_len_lo: u32 }
#[repr(C)] pub struct dword5_fields { pub dst_hi: u32, pub dst_mem: u32, pub rsvd1: u32, pub fixed: u32 }
#[repr(C)] pub union dword5 { pub fields: dword5_fields, pub sha_len_hi: u32 }
#[repr(C)] pub struct dword7 { pub key_hi: u32, pub key_mem: u32, pub rsvd1: u32 }
#[repr(C)] pub struct ccp5_desc { pub dw0: dword0, pub length: u32, pub src_lo: u32, pub dw3: dword3, pub dw4: dword4, pub dw5: dword5, pub key_lo: u32, pub dw7: dword7 }

extern "C" {
    pub fn ccp_add_device(ccp: *mut ccp_device); pub fn ccp_del_device(ccp: *mut ccp_device); pub fn ccp_log_error(ccp: *mut ccp_device, error: u32);
    pub fn ccp_alloc_struct(sp: *mut sp_device) -> *mut ccp_device; pub fn ccp_queues_suspended(ccp: *mut ccp_device) -> bool; pub fn ccp_cmd_queue_thread(data: *mut core::ffi::c_void) -> i32;
    pub fn ccp_trng_read(rng: *mut hwrng, data: *mut core::ffi::c_void, max: usize, wait: bool) -> i32; pub fn ccp_run_cmd(cmd_q: *mut ccp_cmd_queue, cmd: *mut ccp_cmd) -> i32;
    pub fn ccp_register_rng(ccp: *mut ccp_device) -> i32; pub fn ccp_unregister_rng(ccp: *mut ccp_device); pub fn ccp_dmaengine_register(ccp: *mut ccp_device) -> i32; pub fn ccp_dmaengine_unregister(ccp: *mut ccp_device);
    pub fn ccp5_debugfs_setup(ccp: *mut ccp_device); pub fn ccp5_debugfs_destroy();
}

#[repr(C)] pub struct ccp_actions {
    pub aes: Option<unsafe extern "C" fn(*mut ccp_op) -> i32>, pub xts_aes: Option<unsafe extern "C" fn(*mut ccp_op) -> i32>, pub des3: Option<unsafe extern "C" fn(*mut ccp_op) -> i32>, pub sha: Option<unsafe extern "C" fn(*mut ccp_op) -> i32>, pub rsa: Option<unsafe extern "C" fn(*mut ccp_op) -> i32>, pub passthru: Option<unsafe extern "C" fn(*mut ccp_op) -> i32>, pub ecc: Option<unsafe extern "C" fn(*mut ccp_op) -> i32>,
    pub sballoc: Option<unsafe extern "C" fn(*mut ccp_cmd_queue, u32) -> u32>, pub sbfree: Option<unsafe extern "C" fn(*mut ccp_cmd_queue, u32, u32)>, pub get_free_slots: Option<unsafe extern "C" fn(*mut ccp_cmd_queue) -> u32>, pub init: Option<unsafe extern "C" fn(*mut ccp_device) -> i32>, pub destroy: Option<unsafe extern "C" fn(*mut ccp_device)>, pub irqhandler: Option<unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> irqreturn_t>
}

extern "C" {
    pub static ccpv3_platform: ccp_vdata; pub static ccpv3: ccp_vdata; pub static ccpv5a: ccp_vdata; pub static ccpv5b: ccp_vdata;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
