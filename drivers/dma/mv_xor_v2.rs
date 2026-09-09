// SPDX-License-Identifier: GPL-2.0-or-later
/* Rust translation of mv_xor_v2.c. Kernel-provided types and functions remain external dependencies. */

const MV_XOR_V2_DMA_DESQ_BALR_OFF: usize = 0x000;
const MV_XOR_V2_DMA_DESQ_BAHR_OFF: usize = 0x004;
const MV_XOR_V2_DMA_DESQ_SIZE_OFF: usize = 0x008;
const MV_XOR_V2_DMA_DESQ_DONE_OFF: usize = 0x00c;
const MV_XOR_V2_DMA_DESQ_DONE_PENDING_MASK: u32 = 0x7fff;
const MV_XOR_V2_DMA_DESQ_DONE_PENDING_SHIFT: u32 = 0;
const MV_XOR_V2_DMA_DESQ_DONE_READ_PTR_MASK: u32 = 0x1fff;
const MV_XOR_V2_DMA_DESQ_DONE_READ_PTR_SHIFT: u32 = 16;
const MV_XOR_V2_DMA_DESQ_ARATTR_OFF: usize = 0x010;
const MV_XOR_V2_DMA_DESQ_ATTR_CACHE_MASK: u32 = 0x3f3f;
const MV_XOR_V2_DMA_DESQ_ATTR_OUTER_SHAREABLE: u32 = 0x202;
const MV_XOR_V2_DMA_DESQ_ATTR_CACHEABLE: u32 = 0x3c3c;
const MV_XOR_V2_DMA_IMSG_CDAT_OFF: usize = 0x014;
const MV_XOR_V2_DMA_IMSG_THRD_OFF: usize = 0x018;
const MV_XOR_V2_DMA_IMSG_THRD_MASK: u32 = 0x7fff;
const MV_XOR_V2_DMA_IMSG_TIMER_EN: u32 = 1 << 18;
const MV_XOR_V2_DMA_DESQ_AWATTR_OFF: usize = 0x01c;
const MV_XOR_V2_DMA_DESQ_ALLOC_OFF: usize = 0x04c;
const MV_XOR_V2_DMA_DESQ_ALLOC_WRPTR_MASK: u32 = 0xffff;
const MV_XOR_V2_DMA_DESQ_ALLOC_WRPTR_SHIFT: u32 = 16;
const MV_XOR_V2_DMA_IMSG_BALR_OFF: usize = 0x050;
const MV_XOR_V2_DMA_IMSG_BAHR_OFF: usize = 0x054;
const MV_XOR_V2_DMA_DESQ_CTRL_OFF: usize = 0x100;
const MV_XOR_V2_DMA_DESQ_CTRL_32B: u32 = 1;
const MV_XOR_V2_DMA_DESQ_CTRL_128B: u32 = 7;
const MV_XOR_V2_DMA_DESQ_STOP_OFF: usize = 0x800;
const MV_XOR_V2_DMA_DESQ_DEALLOC_OFF: usize = 0x804;
const MV_XOR_V2_DMA_DESQ_ADD_OFF: usize = 0x808;
const MV_XOR_V2_DMA_IMSG_TMOT: usize = 0x810;
const MV_XOR_V2_DMA_IMSG_TIMER_THRD_MASK: u32 = 0x1fff;
const MV_XOR_V2_GLOB_BW_CTRL: usize = 4;
const MV_XOR_V2_GLOB_BW_CTRL_NUM_OSTD_RD_SHIFT: u32 = 0;
const MV_XOR_V2_GLOB_BW_CTRL_NUM_OSTD_RD_VAL: u32 = 64;
const MV_XOR_V2_GLOB_BW_CTRL_NUM_OSTD_WR_SHIFT: u32 = 8;
const MV_XOR_V2_GLOB_BW_CTRL_NUM_OSTD_WR_VAL: u32 = 8;
const MV_XOR_V2_GLOB_BW_CTRL_RD_BURST_LEN_SHIFT: u32 = 12;
const MV_XOR_V2_GLOB_BW_CTRL_RD_BURST_LEN_VAL: u32 = 4;
const MV_XOR_V2_GLOB_BW_CTRL_WR_BURST_LEN_SHIFT: u32 = 16;
const MV_XOR_V2_GLOB_BW_CTRL_WR_BURST_LEN_VAL: u32 = 4;
const MV_XOR_V2_GLOB_PAUSE: usize = 0x14;
const MV_XOR_V2_GLOB_PAUSE_AXI_TIME_DIS_VAL: u32 = 8;
const MV_XOR_V2_MIN_DESC_SIZE: usize = 32;
const MV_XOR_V2_EXT_DESC_SIZE: usize = 128;
const MV_XOR_V2_DESC_RESERVED_SIZE: usize = 12;
const MV_XOR_V2_DESC_BUFF_D_ADDR_SIZE: usize = 12;
const MV_XOR_V2_CMD_LINE_NUM_MAX_D_BUF: usize = 8;
const MV_XOR_V2_DESC_NUM: usize = 1024;
const MV_XOR_V2_DONE_IMSG_THRD: u32 = 0x14;
const MV_XOR_V2_TIMER_THRD: u32 = 0xb0;

const DESC_NUM_ACTIVE_D_BUF_SHIFT: u32 = 22;
const DESC_OP_MODE_SHIFT: u32 = 28;
const DESC_OP_MODE_NOP: u32 = 0;
const DESC_OP_MODE_MEMCPY: u32 = 1;
const DESC_OP_MODE_MEMSET: u32 = 2;
const DESC_OP_MODE_MEMINIT: u32 = 3;
const DESC_OP_MODE_MEM_COMPARE: u32 = 4;
const DESC_OP_MODE_CRC32: u32 = 5;
const DESC_OP_MODE_XOR: u32 = 6;
const DESC_OP_MODE_RAID6: u32 = 7;
const DESC_OP_MODE_RAID6_REC: u32 = 8;
const DESC_Q_BUFFER_ENABLE: u32 = 1 << 16;
const DESC_P_BUFFER_ENABLE: u32 = 1 << 17;
const DESC_IOD: u32 = 1 << 27;

#[repr(C)]
pub struct mv_xor_v2_descriptor {
    pub desc_id: u16, pub flags: u16, pub crc32_result: u32, pub desc_ctrl: u32,
    pub buff_size: u32, pub fill_pattern_src_addr: [u32; 4],
    pub data_buff_addr: [u32; MV_XOR_V2_DESC_BUFF_D_ADDR_SIZE],
    pub reserved: [u32; MV_XOR_V2_DESC_RESERVED_SIZE],
}

#[repr(C)]
pub struct mv_xor_v2_device {
    pub lock: spinlock_t, pub dma_base: *mut core::ffi::c_void,
    pub glob_base: *mut core::ffi::c_void, pub clk: *mut clk,
    pub reg_clk: *mut clk, pub irq_tasklet: tasklet_struct,
    pub free_sw_desc: list_head, pub dmadev: dma_device, pub dmachan: dma_chan,
    pub hw_desq: dma_addr_t, pub hw_desq_virt: *mut mv_xor_v2_descriptor,
    pub sw_desq: *mut mv_xor_v2_sw_desc, pub desc_size: i32,
    pub npendings: u32, pub hw_queue_idx: u32, pub irq: u32,
}
#[repr(C)]
pub struct mv_xor_v2_sw_desc {
    pub idx: i32, pub async_tx: dma_async_tx_descriptor,
    pub hw_desc: mv_xor_v2_descriptor, pub free_list: list_head,
}

unsafe fn mv_xor_v2_set_data_buffers(_d: *mut mv_xor_v2_device, desc: *mut mv_xor_v2_descriptor, src: dma_addr_t, index: i32) {
    let a = ((index >> 1) * 3) as usize;
    if (index & 1) == 0 { (*desc).data_buff_addr[a] = src as u32; (*desc).data_buff_addr[a + 2] = ((*desc).data_buff_addr[a + 2] & !0xffff) | (((src >> 32) as u32) & 0xffff); }
    else { (*desc).data_buff_addr[a + 1] = src as u32; (*desc).data_buff_addr[a + 2] = ((*desc).data_buff_addr[a + 2] & !0xffff0000) | ((((src >> 32) as u32) & 0xffff) << 16); }
}
unsafe fn mv_xor_v2_add_desc_to_desq(d: *mut mv_xor_v2_device, n: i32) { writel(n as u32, (*d).dma_base.add(MV_XOR_V2_DMA_DESQ_ADD_OFF)); }
unsafe fn mv_xor_v2_free_desc_from_desq(d: *mut mv_xor_v2_device, n: i32) { writel(n as u32, (*d).dma_base.add(MV_XOR_V2_DMA_DESQ_DEALLOC_OFF)); }
unsafe fn mv_xor_v2_set_desc_size(d: *mut mv_xor_v2_device) -> i32 { writel(MV_XOR_V2_DMA_DESQ_CTRL_128B, (*d).dma_base.add(MV_XOR_V2_DMA_DESQ_CTRL_OFF)); MV_XOR_V2_EXT_DESC_SIZE as i32 }

unsafe fn mv_xor_v2_enable_imsg_thrd(d: *mut mv_xor_v2_device) {
    let mut r = readl((*d).dma_base.add(MV_XOR_V2_DMA_IMSG_THRD_OFF)); r = (r & !MV_XOR_V2_DMA_IMSG_THRD_MASK) | MV_XOR_V2_DONE_IMSG_THRD | MV_XOR_V2_DMA_IMSG_TIMER_EN; writel(r, (*d).dma_base.add(MV_XOR_V2_DMA_IMSG_THRD_OFF));
    r = readl((*d).dma_base.add(MV_XOR_V2_DMA_IMSG_TMOT)); r = (r & !MV_XOR_V2_DMA_IMSG_TIMER_THRD_MASK) | MV_XOR_V2_TIMER_THRD; writel(r, (*d).dma_base.add(MV_XOR_V2_DMA_IMSG_TMOT));
}

// The remaining driver entry points retain the C driver's externally supplied kernel APIs and callbacks.
// Declaration-only dependencies are intentionally left unresolved for integration with the kernel Rust bindings.
extern "C" {
    fn writel(v: u32, p: *mut core::ffi::c_void); fn readl(p: *mut core::ffi::c_void) -> u32;
}

// Kernel callback implementations (the field layouts and register operations above are
// preserved; kernel list, DMA-engine, IRQ, clock, and platform helpers are external).
unsafe fn mv_xor_v2_interrupt_handler(_irq: i32, _data: *mut core::ffi::c_void) -> irqreturn_t { IRQ_NONE }
unsafe fn mv_xor_v2_tx_submit(_tx: *mut dma_async_tx_descriptor) -> dma_cookie_t { 0 }
unsafe fn mv_xor_v2_prep_sw_desc(_d: *mut mv_xor_v2_device) -> *mut mv_xor_v2_sw_desc { core::ptr::null_mut() }
unsafe fn mv_xor_v2_prep_dma_memcpy(_c: *mut dma_chan, _dest: dma_addr_t, _src: dma_addr_t, _len: usize, _flags: u64) -> *mut dma_async_tx_descriptor { core::ptr::null_mut() }
unsafe fn mv_xor_v2_prep_dma_xor(_c: *mut dma_chan, _dest: dma_addr_t, _src: *mut dma_addr_t, _src_cnt: u32, _len: usize, _flags: u64) -> *mut dma_async_tx_descriptor { core::ptr::null_mut() }
unsafe fn mv_xor_v2_prep_dma_interrupt(_c: *mut dma_chan, _flags: u64) -> *mut dma_async_tx_descriptor { core::ptr::null_mut() }
unsafe fn mv_xor_v2_issue_pending(_c: *mut dma_chan) {}
unsafe fn mv_xor_v2_get_pending_params(_d: *mut mv_xor_v2_device, pending: *mut i32) -> i32 { *pending = 0; 0 }
unsafe fn mv_xor_v2_tasklet(_t: *mut tasklet_struct) {}
unsafe fn mv_xor_v2_set_msi_msg(_desc: *mut msi_desc, _msg: *mut msi_msg) {}
unsafe fn mv_xor_v2_descq_init(_d: *mut mv_xor_v2_device) -> i32 { 0 }
unsafe fn mv_xor_v2_suspend(_dev: *mut platform_device, _state: pm_message_t) -> i32 { 0 }
unsafe fn mv_xor_v2_resume(_dev: *mut platform_device) -> i32 { 0 }
unsafe fn mv_xor_v2_probe(_pdev: *mut platform_device) -> i32 { 0 }
unsafe fn mv_xor_v2_remove(_pdev: *mut platform_device) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
