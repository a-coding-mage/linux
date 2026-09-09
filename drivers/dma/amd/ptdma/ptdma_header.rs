/* SPDX-License-Identifier: GPL-2.0-only */
/* AMD Passthru DMA device driver -- Rust translation of ptdma.h */

/* Linux dependencies are supplied by the surrounding translation unit. */

pub const MAX_PT_NAME_LEN: usize = 16;
pub const MAX_DMAPOOL_NAME_LEN: usize = 32;
pub const MAX_HW_QUEUES: u32 = 1;
pub const MAX_CMD_QLEN: u32 = 100;
pub const PT_ENGINE_PASSTHRU: u32 = 5;

pub const IRQ_MASK_REG: u32 = 0x040;
pub const IRQ_STATUS_REG: u32 = 0x200;
pub const CMD_QUEUE_PRIO_OFFSET: u32 = 0x00;
pub const CMD_REQID_CONFIG_OFFSET: u32 = 0x04;
pub const CMD_TIMEOUT_OFFSET: u32 = 0x08;
pub const CMD_PT_VERSION: u32 = 0x10;
pub const CMD_Q_CONTROL_BASE: u32 = 0x0000;
pub const CMD_Q_TAIL_LO_BASE: u32 = 0x0004;
pub const CMD_Q_HEAD_LO_BASE: u32 = 0x0008;
pub const CMD_Q_INT_ENABLE_BASE: u32 = 0x000C;
pub const CMD_Q_INTERRUPT_STATUS_BASE: u32 = 0x0010;
pub const CMD_Q_STATUS_BASE: u32 = 0x0100;
pub const CMD_Q_INT_STATUS_BASE: u32 = 0x0104;
pub const CMD_Q_DMA_STATUS_BASE: u32 = 0x0108;
pub const CMD_Q_DMA_READ_STATUS_BASE: u32 = 0x010C;
pub const CMD_Q_DMA_WRITE_STATUS_BASE: u32 = 0x0110;
pub const CMD_Q_ABORT_BASE: u32 = 0x0114;
pub const CMD_Q_AX_CACHE_BASE: u32 = 0x0118;
pub const CMD_CONFIG_OFFSET: u32 = 0x1120;
pub const CMD_CLK_GATE_CTL_OFFSET: u32 = 0x6004;
pub const CMD_DESC_DW0_VAL: u32 = 0x500012;
pub const CMD_Q_STATUS_INCR: u32 = 0x1000;
pub const CMD_CONFIG_REQID: u32 = 0;
pub const CMD_TIMEOUT_DISABLE: u32 = 0;
pub const CMD_CLK_DYN_GATING_DIS: u32 = 0;
pub const CMD_CLK_SW_GATE_MODE: u32 = 0;
pub const CMD_CLK_GATE_CTL: u32 = 0;
pub const CMD_QUEUE_PRIO: u32 = 0x6;
pub const CMD_CONFIG_VHB_EN: u32 = 1;
pub const CMD_CLK_DYN_GATING_EN: u32 = 1;
pub const CMD_CLK_HW_GATE_MODE: u32 = 1;
pub const CMD_CLK_GATE_ON_DELAY: u32 = 1 << 12;
pub const CMD_CLK_GATE_OFF_DELAY: u32 = 1 << 12;
pub const CMD_CLK_GATE_CONFIG: u32 = CMD_CLK_GATE_HW_MODE_FIX;
const CMD_CLK_GATE_HW_MODE_FIX: u32 = CMD_CLK_HW_GATE_MODE | CMD_CLK_GATE_ON_DELAY |
    CMD_CLK_DYN_GATING_EN | CMD_CLK_GATE_OFF_DELAY;
pub const CMD_Q_LEN: u32 = 32;
pub const CMD_Q_RUN: u32 = 1;
pub const CMD_Q_HALT: u32 = 1 << 1;
pub const CMD_Q_MEM_LOCATION: u32 = 1 << 2;
pub const CMD_Q_SIZE_MASK: u32 = 0x1f;
pub const CMD_Q_SIZE: u32 = 0xf8;
pub const CMD_Q_SHIFT: u32 = 0x3;
pub const QUEUE_SIZE_VAL: u32 = 3;
pub const Q_PTR_MASK: u32 = (2 << (QUEUE_SIZE_VAL + 5)) - 1;
pub const INT_COMPLETION: u32 = 1;
pub const INT_ERROR: u32 = 1 << 1;
pub const INT_QUEUE_STOPPED: u32 = 1 << 2;
pub const INT_EMPTY_QUEUE: u32 = 1 << 3;
pub const SUPPORTED_INTERRUPTS: u32 = INT_COMPLETION | INT_ERROR;
pub const LSB_START: u32 = 0;
pub const LSB_END: u32 = 127;
pub const LSB_COUNT: u32 = LSB_END - LSB_START + 1;
pub const PT_DMAPOOL_MAX_SIZE: u32 = 64;
pub const PT_DMAPOOL_ALIGN: u32 = 1 << 5;
pub const PT_PASSTHRU_BLOCKSIZE: u32 = 512;

#[repr(C)] pub struct pt_device;
#[repr(C)] pub struct pt_cmd;
#[repr(C)] pub struct pt_msix;
#[repr(C)] pub struct pt_dev_vdata;
#[repr(C)] pub struct device;
#[repr(C)] pub struct dma_pool;
#[repr(C)] pub struct dma_device;
#[repr(C)] pub struct kmem_cache;
#[repr(C)] pub struct completion;
#[repr(C)] pub struct work_struct;
#[repr(C)] pub struct list_head;
#[repr(C)] pub struct spinlock_t;
#[repr(C)] pub struct wait_queue_head_t;
#[repr(C)] pub struct virt_dma_desc;
#[repr(C)] pub struct virt_dma_chan;

pub type dma_addr_t = u64;
pub type __le32 = u32;
pub type dma_status = u32;

#[repr(C)] pub struct pt_tasklet_data { pub completion: completion, pub cmd: *mut pt_cmd }
#[repr(C)] pub struct pt_passthru_engine { pub mask: dma_addr_t, pub mask_len: u32, pub src_dma: dma_addr_t, pub dst_dma: dma_addr_t, pub src_len: u64 }
#[repr(C)] pub struct pt_cmd { pub entry: list_head, pub work: work_struct, pub pt: *mut pt_device, pub ret: i32, pub engine: u32, pub engine_error: u32, pub passthru: pt_passthru_engine, pub pt_cmd_callback: Option<unsafe extern "C" fn(*mut core::ffi::c_void, i32)>, pub data: *mut core::ffi::c_void }
#[repr(C)] pub struct pt_dma_desc { pub vd: virt_dma_desc, pub pt: *mut pt_device, pub status: dma_status, pub len: usize, pub issued_to_hw: bool, pub pt_cmd: pt_cmd }
#[repr(C)] pub struct pt_dma_chan { pub vc: virt_dma_chan, pub pt: *mut pt_device, pub id: u32 }
#[repr(C)] pub struct pt_cmd_queue { pub pt: *mut pt_device, pub dma_pool: *mut dma_pool, pub qbase: *mut ptdma_desc, pub q_lock: spinlock_t, pub qidx: u32, pub qsize: u32, pub qbase_dma: dma_addr_t, pub qdma_tail: dma_addr_t, pub active: u32, pub suspended: u32, pub int_en: bool, pub reg_control: *mut core::ffi::c_void, pub qcontrol: u32, pub int_status: u32, pub q_status: u32, pub q_int_status: u32, pub cmd_error: u32, pub total_pt_ops: usize }
#[repr(C)] pub struct pt_device { pub entry: list_head, pub ord: u32, pub name: [u8; MAX_PT_NAME_LEN], pub dev: *mut device, pub pt_msix: *mut pt_msix, pub dev_vdata: *mut pt_dev_vdata, pub pt_irq: u32, pub io_regs: *mut core::ffi::c_void, pub cmd_lock: spinlock_t, pub cmd_count: u32, pub cmd: list_head, pub cmd_q: pt_cmd_queue, pub dma_dev: dma_device, pub pt_dma_chan: *mut pt_dma_chan, pub dma_desc_cache: *mut kmem_cache, pub lsb_queue: wait_queue_head_t, pub total_interrupts: usize, pub tdata: pt_tasklet_data, pub ver: i32 }
#[repr(transparent)] pub struct dword3(pub u32);
#[repr(transparent)] pub struct dword5(pub u32);
#[repr(C)] pub struct ptdma_desc { pub dw0: u32, pub length: u32, pub src_lo: u32, pub dw3: dword3, pub dst_lo: u32, pub dw5: dword5, pub rsvd1: __le32, pub rsvd2: __le32 }
#[repr(C)] pub struct pt_dev_vdata { pub bar: u32 }

extern "C" { pub fn pt_dmaengine_register(pt: *mut pt_device) -> i32; pub fn pt_dmaengine_unregister(pt: *mut pt_device); pub fn ptdma_debugfs_setup(pt: *mut pt_device); pub fn pt_core_init(pt: *mut pt_device) -> i32; pub fn pt_core_destroy(pt: *mut pt_device); pub fn pt_core_perform_passthru(cmd_q: *mut pt_cmd_queue, pt_engine: *mut pt_passthru_engine) -> i32; pub fn pt_check_status_trans(pt: *mut pt_device, cmd_q: *mut pt_cmd_queue); pub fn pt_start_queue(cmd_q: *mut pt_cmd_queue); pub fn pt_stop_queue(cmd_q: *mut pt_cmd_queue); pub fn iowrite32(value: u32, addr: *mut core::ffi::c_void); }

#[inline] pub unsafe fn pt_core_disable_queue_interrupts(pt: *mut pt_device) { iowrite32(0, (*pt).cmd_q.reg_control.add(0x000C)); }
#[inline] pub unsafe fn pt_core_enable_queue_interrupts(pt: *mut pt_device) { iowrite32(SUPPORTED_INTERRUPTS, (*pt).cmd_q.reg_control.add(0x000C)); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
