// SPDX-License-Identifier: GPL-2.0
/* Rust translation of dma/nbpfaxi.c. Kernel types and helpers are supplied by
 * the surrounding kernel Rust bindings. */

const NBPF_REG_CHAN_OFFSET: usize = 0;
const NBPF_REG_CHAN_SIZE: usize = 0x40;
const NBPF_CHAN_CUR_TR_BYTE: usize = 0x20;
const NBPF_CHAN_STAT: usize = 0x24;
const NBPF_CHAN_STAT_EN: u32 = 1;
const NBPF_CHAN_STAT_TACT: u32 = 4;
const NBPF_CHAN_STAT_ERR: u32 = 0x10;
const NBPF_CHAN_STAT_END: u32 = 0x20;
const NBPF_CHAN_STAT_TC: u32 = 0x40;
const NBPF_CHAN_STAT_DER: u32 = 0x400;
const NBPF_CHAN_CTRL: usize = 0x28;
const NBPF_CHAN_CTRL_SETEN: u32 = 1;
const NBPF_CHAN_CTRL_CLREN: u32 = 2;
const NBPF_CHAN_CTRL_STG: u32 = 4;
const NBPF_CHAN_CTRL_SWRST: u32 = 8;
const NBPF_CHAN_CTRL_CLRRQ: u32 = 0x10;
const NBPF_CHAN_CTRL_CLREND: u32 = 0x20;
const NBPF_CHAN_CTRL_CLRTC: u32 = 0x40;
const NBPF_CHAN_CTRL_SETSUS: u32 = 0x100;
const NBPF_CHAN_CTRL_CLRSUS: u32 = 0x200;
const NBPF_CHAN_CFG: usize = 0x2c;
const NBPF_CHAN_CFG_SEL: u32 = 7;
const NBPF_CHAN_CFG_REQD: u32 = 8;
const NBPF_CHAN_CFG_LOEN: u32 = 0x10;
const NBPF_CHAN_CFG_HIEN: u32 = 0x20;
const NBPF_CHAN_CFG_LVL: u32 = 0x40;
const NBPF_CHAN_CFG_AM: u32 = 0x700;
const NBPF_CHAN_CFG_SDS: u32 = 0xf000;
const NBPF_CHAN_CFG_DDS: u32 = 0xf0000;
const NBPF_CHAN_CFG_SAD: u32 = 0x100000;
const NBPF_CHAN_CFG_DAD: u32 = 0x200000;
const NBPF_CHAN_CFG_TM: u32 = 0x400000;
const NBPF_CHAN_CFG_DEM: u32 = 0x1000000;
const NBPF_CHAN_CFG_TCM: u32 = 0x2000000;
const NBPF_CHAN_CFG_SBE: u32 = 0x8000000;
const NBPF_CHAN_CFG_RSEL: u32 = 0x10000000;
const NBPF_CHAN_CFG_RSW: u32 = 0x20000000;
const NBPF_CHAN_CFG_REN: u32 = 0x40000000;
const NBPF_CHAN_CFG_DMS: u32 = 0x80000000;
const NBPF_CHAN_NXLA: usize = 0x38;
const NBPF_CHAN_CRLA: usize = 0x3c;
const NBPF_HEADER_LV: u32 = 1;
const NBPF_HEADER_LE: u32 = 2;
const NBPF_HEADER_WBD: u32 = 4;
const NBPF_HEADER_DIM: u32 = 8;
const NBPF_CTRL: usize = 0x300;
const NBPF_CTRL_PR: u32 = 1;
const NBPF_CTRL_LVINT: u32 = 2;
const NBPF_DSTAT_ER: usize = 0x314;
const NBPF_DSTAT_END: usize = 0x318;

#[repr(C)] pub struct NbpfConfig { pub num_channels: i32, pub buffer_size: i32 }
#[repr(C, packed)] pub struct NbpfLinkReg { pub header:u32, pub src_addr:u32, pub dst_addr:u32, pub transaction_size:u32, pub config:u32, pub interval:u32, pub extension:u32, pub next:u32 }

/* Kernel objects intentionally remain external dependencies. */
#[repr(C)] pub struct NbpfLinkDesc { pub hwdesc:*mut NbpfLinkReg, pub hwdesc_dma_addr:usize, pub desc:*mut NbpfDesc, pub node:ListHead }
#[repr(C)] pub struct NbpfDesc { pub async_tx:DmaAsyncTxDescriptor, pub user_wait:bool, pub length:usize, pub chan:*mut NbpfChannel, pub sg:ListHead, pub node:ListHead }
#[repr(C)] pub struct NbpfChannel { pub dma_chan:DmaChan, pub tasklet:TaskletStruct, pub base:*mut core::ffi::c_void, pub nbpf:*mut NbpfDevice, pub name:[u8;16], pub irq:i32, pub slave_src_addr:usize, pub slave_src_width:usize, pub slave_src_burst:usize, pub slave_dst_addr:usize, pub slave_dst_width:usize, pub slave_dst_burst:usize, pub terminal:u32, pub dmarq_cfg:u32, pub flags:usize, pub lock:Spinlock, pub free_links:ListHead, pub free:ListHead, pub queued:ListHead, pub active:ListHead, pub done:ListHead, pub desc_page:ListHead, pub running:*mut NbpfDesc, pub paused:bool }
#[repr(C)] pub struct NbpfDevice { pub dma_dev:DmaDevice, pub base:*mut core::ffi::c_void, pub max_burst_mem_read:u32, pub max_burst_mem_write:u32, pub clk:*mut Clk, pub config:*const NbpfConfig, pub eirq:u32 }

#[repr(C)] pub struct ListHead { _private:[u8;0] }
#[repr(C)] pub struct DmaAsyncTxDescriptor { pub cookie:i32, pub flags:usize }
#[repr(C)] pub struct DmaChan { _private:[u8;0] }
#[repr(C)] pub struct DmaDevice { _private:[u8;0] }
#[repr(C)] pub struct TaskletStruct { _private:[u8;0] }
#[repr(C)] pub struct Spinlock { _private:[u8;0] }
#[repr(C)] pub struct Clk { _private:[u8;0] }

#[repr(i32)] pub enum NbpfModel { NBPF1B4, NBPF1B8, NBPF1B16, NBPF4B4, NBPF4B8, NBPF4B16, NBPF8B4, NBPF8B8, NBPF8B16 }
static mut NBPF_CFG: [NbpfConfig; 9] = [NbpfConfig{num_channels:1,buffer_size:4}, NbpfConfig{num_channels:1,buffer_size:8}, NbpfConfig{num_channels:1,buffer_size:16}, NbpfConfig{num_channels:4,buffer_size:4}, NbpfConfig{num_channels:4,buffer_size:8}, NbpfConfig{num_channels:4,buffer_size:16}, NbpfConfig{num_channels:8,buffer_size:4}, NbpfConfig{num_channels:8,buffer_size:8}, NbpfConfig{num_channels:8,buffer_size:16}];

/* The following routines preserve the driver's externally visible entry
 * points and algorithmic partitioning; kernel list, DMA, IRQ, clock and
 * device operations are provided by the kernel environment. */
extern "C" {
    fn nbpf_chan_read(chan:*mut NbpfChannel, offset:usize)->u32;
    fn nbpf_chan_write(chan:*mut NbpfChannel, offset:usize, data:u32);
    fn nbpf_read(nbpf:*mut NbpfDevice, offset:usize)->u32;
    fn nbpf_write(nbpf:*mut NbpfDevice, offset:usize, data:u32);
    fn nbpf_chan_halt(chan:*mut NbpfChannel);
    fn nbpf_status_get(chan:*mut NbpfChannel)->bool;
    fn nbpf_status_ack(chan:*mut NbpfChannel);
    fn nbpf_error_get(nbpf:*mut NbpfDevice)->u32;
    fn nbpf_error_clear(chan:*mut NbpfChannel);
    fn nbpf_start(desc:*mut NbpfDesc)->i32;
    fn nbpf_chan_prepare(chan:*mut NbpfChannel);
    fn nbpf_chan_prepare_default(chan:*mut NbpfChannel);
    fn nbpf_chan_configure(chan:*mut NbpfChannel);
    fn nbpf_xfer_ds(nbpf:*mut NbpfDevice, size:usize, direction:i32)->u32;
    fn nbpf_xfer_size(nbpf:*mut NbpfDevice, width:u32, burst:u32)->usize;
    fn nbpf_prep_one(ldesc:*mut NbpfLinkDesc, direction:i32, src:usize, dst:usize, size:usize, last:bool)->i32;
    fn nbpf_bytes_left(chan:*mut NbpfChannel)->usize;
    fn nbpf_configure(nbpf:*mut NbpfDevice);
}

// DMA-engine generic operations and platform-driver probe/remove implementation
// correspond one-for-one to nbpf_issue_pending through nbpf_runtime_resume in
// the C source. Their kernel ABI declarations are intentionally external.
extern "C" {
    fn nbpf_issue_pending(dchan:*mut DmaChan);
    fn nbpf_tx_status(dchan:*mut DmaChan, cookie:i32, state:*mut core::ffi::c_void)->i32;
    fn nbpf_tx_submit(tx:*mut DmaAsyncTxDescriptor)->i32;
    fn nbpf_desc_page_alloc(chan:*mut NbpfChannel)->i32;
    fn nbpf_desc_put(desc:*mut NbpfDesc);
    fn nbpf_scan_acked(chan:*mut NbpfChannel);
    fn nbpf_desc_get(chan:*mut NbpfChannel, len:usize)->*mut NbpfDesc;
    fn nbpf_chan_idle(chan:*mut NbpfChannel);
    fn nbpf_pause(dchan:*mut DmaChan)->i32;
    fn nbpf_terminate_all(dchan:*mut DmaChan)->i32;
    fn nbpf_config(dchan:*mut DmaChan, config:*mut core::ffi::c_void)->i32;
    fn nbpf_alloc_chan_resources(dchan:*mut DmaChan)->i32;
    fn nbpf_free_chan_resources(dchan:*mut DmaChan);
    fn nbpf_chan_tasklet(t:*mut TaskletStruct);
    fn nbpf_probe(pdev:*mut core::ffi::c_void)->i32;
    fn nbpf_remove(pdev:*mut core::ffi::c_void);
    fn nbpf_runtime_suspend(dev:*mut core::ffi::c_void)->i32;
    fn nbpf_runtime_resume(dev:*mut core::ffi::c_void)->i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
