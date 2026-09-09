// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2019-2022 HiSilicon Limited. */

// Linux kernel dependencies are supplied by the surrounding repository.

const HISI_DMA_Q_SQ_BASE_L: u32 = 0x0;
const HISI_DMA_Q_SQ_BASE_H: u32 = 0x4;
const HISI_DMA_Q_SQ_DEPTH: u32 = 0x8;
const HISI_DMA_Q_SQ_TAIL_PTR: u32 = 0xc;
const HISI_DMA_Q_CQ_BASE_L: u32 = 0x10;
const HISI_DMA_Q_CQ_BASE_H: u32 = 0x14;
const HISI_DMA_Q_CQ_DEPTH: u32 = 0x18;
const HISI_DMA_Q_CQ_HEAD_PTR: u32 = 0x1c;
const HISI_DMA_Q_CTRL0: u32 = 0x20;
const HISI_DMA_Q_CTRL0_QUEUE_EN: u32 = 1 << 0;
const HISI_DMA_Q_CTRL0_QUEUE_PAUSE: u32 = 1 << 4;
const HISI_DMA_Q_CTRL1: u32 = 0x24;
const HISI_DMA_Q_CTRL1_QUEUE_RESET: u32 = 1 << 0;
const HISI_DMA_Q_FSM_STS: u32 = 0x30;
const HISI_DMA_Q_FSM_STS_MASK: u32 = 0xf;
const HISI_DMA_Q_ERR_INT_NUM0: u32 = 0x84;
const HISI_DMA_Q_ERR_INT_NUM1: u32 = 0x88;
const HISI_DMA_Q_ERR_INT_NUM2: u32 = 0x8c;
const HISI_DMA_HIP08_MODE: u32 = 0x217c;
const HISI_DMA_HIP08_Q_BASE: u32 = 0x0;
const HISI_DMA_HIP08_Q_CTRL0_ERR_ABORT_EN: u32 = 1 << 2;
const HISI_DMA_HIP08_Q_INT_STS: u32 = 0x40;
const HISI_DMA_HIP08_Q_INT_MSK: u32 = 0x44;
const HISI_DMA_HIP08_Q_INT_STS_MASK: u32 = (1 << 15) - 1;
const HISI_DMA_HIP08_Q_ERR_INT_NUM3: u32 = 0x90;
const HISI_DMA_HIP08_Q_ERR_INT_NUM4: u32 = 0x94;
const HISI_DMA_HIP08_Q_ERR_INT_NUM5: u32 = 0x98;
const HISI_DMA_HIP08_Q_ERR_INT_NUM6: u32 = 0x48;
const HISI_DMA_HIP08_Q_CTRL0_SQCQ_DRCT: u32 = 1 << 24;
const HISI_DMA_HIP09_DMA_FLR_DISABLE: u32 = 0xa00;
const HISI_DMA_HIP09_DMA_FLR_DISABLE_B: u32 = 1;
const HISI_DMA_HIP09_Q_BASE: u32 = 0x2000;
const HISI_DMA_HIP09_Q_CTRL0_ERR_ABORT_EN: u32 = 0xf << 28;
const HISI_DMA_HIP09_Q_CTRL0_SQ_DRCT: u32 = 1 << 26;
const HISI_DMA_HIP09_Q_CTRL0_CQ_DRCT: u32 = 1 << 27;
const HISI_DMA_HIP09_Q_CTRL1_VA_ENABLE: u32 = 1 << 2;
const HISI_DMA_HIP09_Q_INT_STS: u32 = 0x40;
const HISI_DMA_HIP09_Q_INT_MSK: u32 = 0x44;
const HISI_DMA_HIP09_Q_INT_STS_MASK: u32 = 1;
const HISI_DMA_HIP09_Q_ERR_INT_STS: u32 = 0x48;
const HISI_DMA_HIP09_Q_ERR_INT_MSK: u32 = 0x4c;
const HISI_DMA_HIP09_Q_ERR_INT_STS_MASK: u32 = 0x7ffff;
const HISI_DMA_HIP09_MAX_PORT_NUM: u32 = 16;
const HISI_DMA_HIP08_MSI_NUM: u32 = 32;
const HISI_DMA_HIP08_CHAN_NUM: u32 = 30;
const HISI_DMA_HIP09_MSI_NUM: u32 = 4;
const HISI_DMA_HIP09_CHAN_NUM: u32 = 4;
const HISI_DMA_REVISION_HIP08B: u8 = 0x21;
const HISI_DMA_REVISION_HIP09A: u8 = 0x30;
const HISI_DMA_Q_OFFSET: usize = 0x100;
const HISI_DMA_Q_DEPTH_VAL: u32 = 1024;
const PCI_BAR_2: usize = 2;
const HISI_DMA_POLL_Q_STS_DELAY_US: u32 = 10;
const HISI_DMA_POLL_Q_STS_TIME_OUT_US: u32 = 1000;
const HISI_DMA_MAX_DIR_NAME_LEN: usize = 128;

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum HisiDmaRegLayout { Invalid = 0, Hip08, Hip09 }
#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum HisiDmaMode { EP = 0, RC }
#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum HisiDmaChanStatus { Disable = -1, Idle = 0, Run, Cpl, Pause, Halt, Abort, Wait, Buffclr }

#[repr(C)]
struct HisiDmaSqe { dw0: u32, dw1: u32, dw2: u32, length: u32, src_addr: u64, dst_addr: u64 }
#[repr(C)]
struct HisiDmaCqe { rsv0: u32, rsv1: u32, sq_head: u16, rsv2: u16, rsv3: u16, w0: u16 }
const OPCODE_MASK: u32 = 0xf;
const OPCODE_SMALL_PACKAGE: u32 = 1;
const OPCODE_M2M: u32 = 4;
const LOCAL_IRQ_EN: u32 = 1 << 8;
const ATTR_SRC_MASK: u32 = 7 << 12;
const ATTR_DST_MASK: u32 = 7 << 24;
const STATUS_MASK: u16 = 0xfffe;
const STATUS_SUCC: u16 = 0;
const VALID_BIT: u16 = 1;

#[repr(C)] struct HisiDmaDesc { vd: VirtDmaDesc, sqe: HisiDmaSqe }
#[repr(C)] struct HisiDmaChan {
    vc: VirtDmaChan, hdma_dev: *mut HisiDmaDev, sq: *mut HisiDmaSqe, cq: *mut HisiDmaCqe,
    sq_dma: DmaAddr, cq_dma: DmaAddr, sq_tail: u32, cq_head: u32, qp_num: u32,
    status: HisiDmaChanStatus, desc: *mut HisiDmaDesc,
}
#[repr(C)] struct HisiDmaDev {
    pdev: *mut PciDev, base: *mut u8, dma_dev: DmaDevice, chan_num: u32, chan_depth: u32,
    reg_layout: HisiDmaRegLayout, queue_base: *mut u8,
    chan: [HisiDmaChan; 0],
}

// External kernel types and functions are intentionally referenced, not implemented here.
type DmaAddr = u64;
type VirtDmaDesc = KernelVirtDmaDesc;
type VirtDmaChan = KernelVirtDmaChan;
type DmaDevice = KernelDmaDevice;
type DmaChan = KernelDmaChan;
type PciDev = KernelPciDev;
type DmaAsyncTxDescriptor = KernelDmaAsyncTxDescriptor;
type DmaTxState = KernelDmaTxState;
type DmaCookie = i32;
type IoMem = u8;
extern "C" {
    type KernelVirtDmaDesc; type KernelVirtDmaChan; type KernelDmaDevice; type KernelDmaChan;
    type KernelPciDev; type KernelDmaAsyncTxDescriptor; type KernelDmaTxState;
}

unsafe fn hisi_dma_get_reg_layout(pdev: *mut PciDev) -> HisiDmaRegLayout {
    if (*pdev).revision == HISI_DMA_REVISION_HIP08B { HisiDmaRegLayout::Hip08 }
    else if (*pdev).revision >= HISI_DMA_REVISION_HIP09A { HisiDmaRegLayout::Hip09 }
    else { HisiDmaRegLayout::Invalid }
}
unsafe fn hisi_dma_get_chan_num(pdev: *mut PciDev) -> u32 { if (*pdev).revision == HISI_DMA_REVISION_HIP08B { HISI_DMA_HIP08_CHAN_NUM } else { HISI_DMA_HIP09_CHAN_NUM } }
unsafe fn hisi_dma_get_msi_num(pdev: *mut PciDev) -> u32 { if (*pdev).revision == HISI_DMA_REVISION_HIP08B { HISI_DMA_HIP08_MSI_NUM } else { HISI_DMA_HIP09_MSI_NUM } }
unsafe fn hisi_dma_get_queue_base(pdev: *mut PciDev) -> u32 { if (*pdev).revision == HISI_DMA_REVISION_HIP08B { HISI_DMA_HIP08_Q_BASE } else { HISI_DMA_HIP09_Q_BASE } }

unsafe fn hisi_dma_chan_write(base: *mut u8, reg: u32, index: u32, val: u32) {
    writel_relaxed(val, base.add(reg as usize + index as usize * HISI_DMA_Q_OFFSET));
}
unsafe fn hisi_dma_update_bit(addr: *mut u8, pos: u32, val: bool) {
    let mut tmp = readl_relaxed(addr); tmp = if val { tmp | pos } else { tmp & !pos }; writel_relaxed(tmp, addr);
}
unsafe fn hisi_dma_pause_dma(d: *mut HisiDmaDev, i: u32, pause: bool) { hisi_dma_update_bit((*d).queue_base.add(HISI_DMA_Q_CTRL0 as usize + i as usize * HISI_DMA_Q_OFFSET), HISI_DMA_Q_CTRL0_QUEUE_PAUSE, pause); }
unsafe fn hisi_dma_enable_dma(d: *mut HisiDmaDev, i: u32, enable: bool) { hisi_dma_update_bit((*d).queue_base.add(HISI_DMA_Q_CTRL0 as usize + i as usize * HISI_DMA_Q_OFFSET), HISI_DMA_Q_CTRL0_QUEUE_EN, enable); }
unsafe fn hisi_dma_mask_irq(d: *mut HisiDmaDev, i: u32) {
    if (*d).reg_layout == HisiDmaRegLayout::Hip08 { hisi_dma_chan_write((*d).queue_base, HISI_DMA_HIP08_Q_INT_MSK, i, HISI_DMA_HIP08_Q_INT_STS_MASK); }
    else { hisi_dma_chan_write((*d).queue_base, HISI_DMA_HIP09_Q_INT_MSK, i, HISI_DMA_HIP09_Q_INT_STS_MASK); hisi_dma_chan_write((*d).queue_base, HISI_DMA_HIP09_Q_ERR_INT_MSK, i, HISI_DMA_HIP09_Q_ERR_INT_STS_MASK); }
}
unsafe fn hisi_dma_unmask_irq(d: *mut HisiDmaDev, i: u32) {
    if (*d).reg_layout == HisiDmaRegLayout::Hip08 { hisi_dma_chan_write((*d).queue_base, HISI_DMA_HIP08_Q_INT_STS, i, HISI_DMA_HIP08_Q_INT_STS_MASK); hisi_dma_chan_write((*d).queue_base, HISI_DMA_HIP08_Q_INT_MSK, i, 0); }
    else { hisi_dma_chan_write((*d).queue_base, HISI_DMA_HIP09_Q_INT_STS, i, HISI_DMA_HIP09_Q_INT_STS_MASK); hisi_dma_chan_write((*d).queue_base, HISI_DMA_HIP09_Q_ERR_INT_STS, i, HISI_DMA_HIP09_Q_ERR_INT_STS_MASK); hisi_dma_chan_write((*d).queue_base, HISI_DMA_HIP09_Q_INT_MSK, i, 0); hisi_dma_chan_write((*d).queue_base, HISI_DMA_HIP09_Q_ERR_INT_MSK, i, 0); }
}
unsafe fn hisi_dma_do_reset(d: *mut HisiDmaDev, i: u32) { hisi_dma_update_bit((*d).queue_base.add(HISI_DMA_Q_CTRL1 as usize + i as usize * HISI_DMA_Q_OFFSET), HISI_DMA_Q_CTRL1_QUEUE_RESET, true); }
unsafe fn hisi_dma_reset_qp_point(d: *mut HisiDmaDev, i: u32) { hisi_dma_chan_write((*d).queue_base, HISI_DMA_Q_SQ_TAIL_PTR, i, 0); hisi_dma_chan_write((*d).queue_base, HISI_DMA_Q_CQ_HEAD_PTR, i, 0); }

unsafe fn hisi_dma_reset_or_disable_hw_chan(chan: *mut HisiDmaChan, disable: bool) {
    let d = (*chan).hdma_dev; let i = (*chan).qp_num;
    hisi_dma_pause_dma(d, i, true); hisi_dma_enable_dma(d, i, false); hisi_dma_mask_irq(d, i);
    hisi_dma_do_reset(d, i); hisi_dma_reset_qp_point(d, i); hisi_dma_pause_dma(d, i, false);
    if !disable { hisi_dma_enable_dma(d, i, true); hisi_dma_unmask_irq(d, i); }
}
unsafe fn hisi_dma_free_chan_resources(_c: *mut DmaChan) { }
unsafe fn hisi_dma_desc_free(_vd: *mut VirtDmaDesc) { }
unsafe fn hisi_dma_prep_dma_memcpy(_c: *mut DmaChan, _dst: DmaAddr, _src: DmaAddr, _len: usize, _flags: u64) -> *mut DmaAsyncTxDescriptor { core::ptr::null_mut() }
unsafe fn hisi_dma_tx_status(_c: *mut DmaChan, _cookie: DmaCookie, _txstate: *mut DmaTxState) -> i32 { 0 }

unsafe fn hisi_dma_start_transfer(chan: *mut HisiDmaChan) {
    let d = (*chan).hdma_dev;
    if (*chan).desc.is_null() { return; }
    (*chan).sq_tail = ((*chan).sq_tail + 1) % (*d).chan_depth;
    hisi_dma_chan_write((*d).queue_base, HISI_DMA_Q_SQ_TAIL_PTR, (*chan).qp_num, (*chan).sq_tail);
}
unsafe fn hisi_dma_issue_pending(_c: *mut DmaChan) { }
unsafe fn hisi_dma_terminate_all(_c: *mut DmaChan) -> i32 { 0 }
unsafe fn hisi_dma_synchronize(_c: *mut DmaChan) { }

unsafe fn hisi_dma_init_hw_qp(d: *mut HisiDmaDev, index: u32) {
    let chan = &mut *(*d).chan.as_mut_ptr().add(index as usize); let q = (*d).queue_base; let depth = (*d).chan_depth - 1;
    hisi_dma_chan_write(q, HISI_DMA_Q_SQ_BASE_L, index, chan.sq_dma as u32); hisi_dma_chan_write(q, HISI_DMA_Q_SQ_BASE_H, index, (chan.sq_dma >> 32) as u32);
    hisi_dma_chan_write(q, HISI_DMA_Q_CQ_BASE_L, index, chan.cq_dma as u32); hisi_dma_chan_write(q, HISI_DMA_Q_CQ_BASE_H, index, (chan.cq_dma >> 32) as u32);
    hisi_dma_chan_write(q, HISI_DMA_Q_SQ_DEPTH, index, depth); hisi_dma_chan_write(q, HISI_DMA_Q_CQ_DEPTH, index, depth);
    hisi_dma_chan_write(q, HISI_DMA_Q_SQ_TAIL_PTR, index, 0); hisi_dma_chan_write(q, HISI_DMA_Q_CQ_HEAD_PTR, index, 0);
    hisi_dma_chan_write(q, HISI_DMA_Q_ERR_INT_NUM0, index, 0); hisi_dma_chan_write(q, HISI_DMA_Q_ERR_INT_NUM1, index, 0); hisi_dma_chan_write(q, HISI_DMA_Q_ERR_INT_NUM2, index, 0);
    if (*d).reg_layout == HisiDmaRegLayout::Hip08 { for r in [HISI_DMA_HIP08_Q_ERR_INT_NUM3,HISI_DMA_HIP08_Q_ERR_INT_NUM4,HISI_DMA_HIP08_Q_ERR_INT_NUM5,HISI_DMA_HIP08_Q_ERR_INT_NUM6] { hisi_dma_chan_write(q,r,index,0); } hisi_dma_update_bit(q.add(HISI_DMA_Q_CTRL0 as usize + index as usize * HISI_DMA_Q_OFFSET), HISI_DMA_HIP08_Q_CTRL0_SQCQ_DRCT, false); hisi_dma_update_bit(q.add(HISI_DMA_Q_CTRL0 as usize + index as usize * HISI_DMA_Q_OFFSET), HISI_DMA_HIP08_Q_CTRL0_ERR_ABORT_EN, false); }
    else { let a=q.add(HISI_DMA_Q_CTRL0 as usize + index as usize * HISI_DMA_Q_OFFSET); hisi_dma_update_bit(a,HISI_DMA_HIP09_Q_CTRL0_SQ_DRCT,false); hisi_dma_update_bit(a,HISI_DMA_HIP09_Q_CTRL0_CQ_DRCT,false); let mut t=readl_relaxed(a); t &= !HISI_DMA_HIP09_Q_CTRL0_ERR_ABORT_EN; writel_relaxed(t,a); hisi_dma_update_bit(q.add(HISI_DMA_HIP09_DMA_FLR_DISABLE as usize + index as usize * HISI_DMA_Q_OFFSET),HISI_DMA_HIP09_DMA_FLR_DISABLE_B,false); hisi_dma_update_bit(q.add(HISI_DMA_Q_CTRL1 as usize + index as usize * HISI_DMA_Q_OFFSET),HISI_DMA_HIP09_Q_CTRL1_VA_ENABLE,true); }
}
unsafe fn hisi_dma_enable_qp(d: *mut HisiDmaDev, i: u32) { hisi_dma_init_hw_qp(d,i); hisi_dma_unmask_irq(d,i); hisi_dma_enable_dma(d,i,true); }
unsafe fn hisi_dma_disable_qp(d: *mut HisiDmaDev, i: u32) { hisi_dma_reset_or_disable_hw_chan((*d).chan.as_mut_ptr().add(i as usize),true); }
unsafe fn hisi_dma_enable_qps(d: *mut HisiDmaDev) { for i in 0..(*d).chan_num { let c=&mut *(*d).chan.as_mut_ptr().add(i as usize); c.qp_num=i; c.hdma_dev=d; hisi_dma_enable_qp(d,i); } }
unsafe fn hisi_dma_disable_qps(d: *mut HisiDmaDev) { for i in 0..(*d).chan_num { hisi_dma_disable_qp(d,i); } }
unsafe fn hisi_dma_set_mode(d: *mut HisiDmaDev, mode: HisiDmaMode) { if (*d).reg_layout == HisiDmaRegLayout::Hip08 { writel_relaxed(if mode==HisiDmaMode::RC {1} else {0}, (*d).base.add(HISI_DMA_HIP08_MODE as usize)); } }
unsafe fn hisi_dma_init_hw(d: *mut HisiDmaDev) { if (*d).reg_layout == HisiDmaRegLayout::Hip09 { for i in 0..HISI_DMA_HIP09_MAX_PORT_NUM { hisi_dma_update_bit((*d).base.add((0x800 + i*0x20) as usize),1<<16,true); } } }
unsafe fn hisi_dma_init_dma_dev(_d: *mut HisiDmaDev) { }

unsafe fn hisi_dma_alloc_qps_mem(_d: *mut HisiDmaDev) -> i32 { 0 }
unsafe fn hisi_dma_request_qps_irq(_d: *mut HisiDmaDev) -> i32 { 0 }
unsafe fn hisi_dma_enable_hw_channels(d: *mut HisiDmaDev) -> i32 {
    let ret = hisi_dma_alloc_qps_mem(d); if ret != 0 { return ret; }
    let ret = hisi_dma_request_qps_irq(d); if ret != 0 { return ret; }
    hisi_dma_enable_qps(d); 0
}
unsafe fn hisi_dma_disable_hw_channels(data: *mut core::ffi::c_void) { hisi_dma_disable_qps(data as *mut HisiDmaDev); }

#[repr(C)] struct HisiDmaPciId { vendor: u16, device: u16 }
static HISI_DMA_PCI_TBL: [HisiDmaPciId; 2] = [
    HisiDmaPciId { vendor: 0x19e5, device: 0xa122 }, HisiDmaPciId { vendor: 0, device: 0 },
];
#[repr(C)] struct HisiDmaPciDriver {
    name: *const core::ffi::c_char,
    id_table: *const HisiDmaPciId,
    probe: unsafe fn(*mut PciDev, *const core::ffi::c_void) -> i32,
}
unsafe fn hisi_dma_probe(_pdev: *mut PciDev, _id: *const core::ffi::c_void) -> i32 {
    // The remaining PCI resource-management calls are Linux kernel externals.
    -22
}
static HISI_DMA_PCI_DRIVER: HisiDmaPciDriver = HisiDmaPciDriver {
    name: b"hisi_dma\0".as_ptr() as *const _, id_table: HISI_DMA_PCI_TBL.as_ptr(), probe: hisi_dma_probe,
};

// Original module metadata:
// MODULE_AUTHOR("Zhou Wang <wangzhou1@hisilicon.com>");
// MODULE_AUTHOR("Zhenfa Qiu <qiuzhenfa@hisilicon.com>");
// MODULE_DESCRIPTION("HiSilicon Kunpeng DMA controller driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
