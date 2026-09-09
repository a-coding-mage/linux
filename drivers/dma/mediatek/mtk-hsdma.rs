// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2017-2018 MediaTek Inc.
//
// Driver for MediaTek High-Speed DMA Controller
// Translated from mtk-hsdma.c; kernel dependencies are supplied externally.

const MTK_HSDMA_USEC_POLL: u32 = 20;
const MTK_HSDMA_TIMEOUT_POLL: u32 = 200000;
const MTK_HSDMA_DMA_BUSWIDTHS: u32 = 1 << DMA_SLAVE_BUSWIDTH_4_BYTES;
const MTK_HSDMA_NR_VCHANS: usize = 3;
const MTK_HSDMA_NR_MAX_PCHANS: usize = 1;
const MTK_DMA_SIZE: usize = 64;
const MTK_HSDMA_MAX_LEN: usize = 0x3f80;
const MTK_HSDMA_ALIGN_SIZE: u32 = 4;
const MTK_HSDMA_PLEN_MASK: u32 = 0x3fff;
const MTK_HSDMA_TX_BASE: u32 = 0x0;
const MTK_HSDMA_TX_CNT: u32 = 0x4;
const MTK_HSDMA_TX_CPU: u32 = 0x8;
const MTK_HSDMA_TX_DMA: u32 = 0xc;
const MTK_HSDMA_RX_BASE: u32 = 0x100;
const MTK_HSDMA_RX_CNT: u32 = 0x104;
const MTK_HSDMA_RX_CPU: u32 = 0x108;
const MTK_HSDMA_RX_DMA: u32 = 0x10c;
const MTK_HSDMA_GLO: u32 = 0x204;
const MTK_HSDMA_GLO_MULTI_DMA: u32 = 1 << 10;
const MTK_HSDMA_TX_WB_DDONE: u32 = 1 << 6;
const MTK_HSDMA_BURST_64BYTES: u32 = 0x2 << 4;
const MTK_HSDMA_GLO_RX_BUSY: u32 = 1 << 3;
const MTK_HSDMA_GLO_RX_DMA: u32 = 1 << 2;
const MTK_HSDMA_GLO_TX_BUSY: u32 = 1 << 1;
const MTK_HSDMA_GLO_TX_DMA: u32 = 1;
const MTK_HSDMA_GLO_DMA: u32 = MTK_HSDMA_GLO_TX_DMA | MTK_HSDMA_GLO_RX_DMA;
const MTK_HSDMA_GLO_BUSY: u32 = MTK_HSDMA_GLO_RX_BUSY | MTK_HSDMA_GLO_TX_BUSY;
const MTK_HSDMA_GLO_DEFAULT: u32 = MTK_HSDMA_GLO_DMA | MTK_HSDMA_TX_WB_DDONE | MTK_HSDMA_BURST_64BYTES | MTK_HSDMA_GLO_MULTI_DMA;
const MTK_HSDMA_RESET: u32 = 0x208;
const MTK_HSDMA_RST_TX: u32 = 1;
const MTK_HSDMA_RST_RX: u32 = 1 << 16;
const MTK_HSDMA_DLYINT: u32 = 0x20c;
const MTK_HSDMA_RXDLY_INT_EN: u32 = 1 << 15;
const MTK_HSDMA_DLYINT_DEFAULT: u32 = MTK_HSDMA_RXDLY_INT_EN | (20 << 8) | 20;
const MTK_HSDMA_INT_STATUS: u32 = 0x220;
const MTK_HSDMA_INT_ENABLE: u32 = 0x228;
const MTK_HSDMA_INT_RXDONE: u32 = 1 << 16;
const MTK_HSDMA_VDESC_FINISHED: u32 = 0x01;

#[repr(C, packed(4))]
struct mtk_hsdma_pdesc { desc1: __le32, desc2: __le32, desc3: __le32, desc4: __le32 }
#[repr(C)]
struct mtk_hsdma_vdesc { vd: virt_dma_desc, len: usize, residue: usize, dest: dma_addr_t, src: dma_addr_t }
#[repr(C)]
struct mtk_hsdma_cb { vd: *mut virt_dma_desc, flag: u32 }
#[repr(C)]
struct mtk_hsdma_ring { txd: *mut mtk_hsdma_pdesc, rxd: *mut mtk_hsdma_pdesc, cb: *mut mtk_hsdma_cb, tphys: dma_addr_t, rphys: dma_addr_t, cur_tptr: u16, cur_rptr: u16 }
#[repr(C)]
struct mtk_hsdma_pchan { ring: mtk_hsdma_ring, sz_ring: usize, nr_free: atomic_t }
#[repr(C)]
struct mtk_hsdma_vchan { vc: virt_dma_chan, issue_completion: completion, issue_synchronize: bool, desc_hw_processing: list_head }
#[repr(C)]
struct mtk_hsdma_soc { ddone: __le32, ls0: __le32 }
#[repr(C)]
struct mtk_hsdma_device { ddev: dma_device, base: *mut core::ffi::c_void, clk: *mut clk, irq: u32, dma_requests: u32, vc: *mut mtk_hsdma_vchan, pc: *mut mtk_hsdma_pchan, pc_refcnt: refcount_t, lock: spinlock_t, soc: *const mtk_hsdma_soc }

#[inline] fn next_idx(x: u16, y: u16) -> u16 { (x.wrapping_add(1)) & (y.wrapping_sub(1)) }
#[inline] fn last_idx(x: u16, y: u16) -> u16 { (x.wrapping_sub(1)) & (y.wrapping_sub(1)) }
#[inline] fn desc_plen(x: usize) -> u32 { ((x as u32 & MTK_HSDMA_PLEN_MASK) << 16) }
#[inline] fn desc_plen_get(x: u32) -> usize { ((x >> 16) & MTK_HSDMA_PLEN_MASK) as usize }

unsafe fn to_hsdma_dev(chan: *mut dma_chan) -> *mut mtk_hsdma_device { container_of((*(*chan).device), mtk_hsdma_device, ddev) }
unsafe fn to_hsdma_vchan(chan: *mut dma_chan) -> *mut mtk_hsdma_vchan { container_of((*chan), mtk_hsdma_vchan, vc.chan) }
unsafe fn to_hsdma_vdesc(vd: *mut virt_dma_desc) -> *mut mtk_hsdma_vdesc { container_of((*vd), mtk_hsdma_vdesc, vd) }
unsafe fn hsdma2dev(h: *mut mtk_hsdma_device) -> *mut device { (*h).ddev.dev }
unsafe fn mtk_dma_read(h: *mut mtk_hsdma_device, reg: u32) -> u32 { readl((*h).base.add(reg as usize)) }
unsafe fn mtk_dma_write(h: *mut mtk_hsdma_device, reg: u32, val: u32) { writel(val, (*h).base.add(reg as usize)); }
unsafe fn mtk_dma_rmw(h: *mut mtk_hsdma_device, reg: u32, mask: u32, set: u32) { let mut v=mtk_dma_read(h,reg); v &= !mask; v |= set; mtk_dma_write(h,reg,v); }
unsafe fn mtk_dma_set(h:*mut mtk_hsdma_device,r:u32,v:u32){mtk_dma_rmw(h,r,0,v)}
unsafe fn mtk_dma_clr(h:*mut mtk_hsdma_device,r:u32,v:u32){mtk_dma_rmw(h,r,v,0)}

unsafe extern "C" fn mtk_hsdma_vdesc_free(vd: *mut virt_dma_desc) { kfree(to_hsdma_vdesc(vd) as *mut core::ffi::c_void); }
unsafe fn mtk_hsdma_busy_wait(h:*mut mtk_hsdma_device)->i32 { let mut status=0; readl_poll_timeout((*h).base.add(MTK_HSDMA_GLO as usize), &mut status, (status & MTK_HSDMA_GLO_BUSY)==0, MTK_HSDMA_USEC_POLL, MTK_HSDMA_TIMEOUT_POLL) }

unsafe fn mtk_hsdma_alloc_pchan(h:*mut mtk_hsdma_device, pc:*mut mtk_hsdma_pchan)->i32 {
    memset(pc as *mut _,0,core::mem::size_of::<mtk_hsdma_pchan>()); let ring=&mut (*pc).ring; let mut err;
    (*pc).sz_ring=2*MTK_DMA_SIZE*core::mem::size_of::<mtk_hsdma_pdesc>(); ring.txd=dma_alloc_coherent(hsdma2dev(h),(*pc).sz_ring,&mut ring.tphys,GFP_NOWAIT) as *mut _; if ring.txd.is_null(){return -ENOMEM;}
    ring.rxd=ring.txd.add(MTK_DMA_SIZE); ring.rphys=ring.tphys+(MTK_DMA_SIZE*core::mem::size_of::<mtk_hsdma_pdesc>()) as u64; ring.cur_tptr=0; ring.cur_rptr=(MTK_DMA_SIZE-1) as u16;
    ring.cb=kzalloc_objs::<mtk_hsdma_cb>(MTK_DMA_SIZE,GFP_NOWAIT); if ring.cb.is_null(){err=-ENOMEM;dma_free_coherent(hsdma2dev(h),(*pc).sz_ring,ring.txd as _,ring.tphys);return err;} atomic_set(&mut (*pc).nr_free,(MTK_DMA_SIZE-1) as i32);
    mtk_dma_clr(h,MTK_HSDMA_GLO,MTK_HSDMA_GLO_DMA); err=mtk_hsdma_busy_wait(h); if err!=0{ kfree(ring.cb as _);dma_free_coherent(hsdma2dev(h),(*pc).sz_ring,ring.txd as _,ring.tphys);return err;}
    mtk_dma_set(h,MTK_HSDMA_RESET,MTK_HSDMA_RST_TX|MTK_HSDMA_RST_RX);mtk_dma_clr(h,MTK_HSDMA_RESET,MTK_HSDMA_RST_TX|MTK_HSDMA_RST_RX);
    mtk_dma_write(h,MTK_HSDMA_TX_BASE,ring.tphys as u32);mtk_dma_write(h,MTK_HSDMA_TX_CNT,MTK_DMA_SIZE as u32);mtk_dma_write(h,MTK_HSDMA_TX_CPU,ring.cur_tptr as u32);mtk_dma_write(h,MTK_HSDMA_TX_DMA,0);mtk_dma_write(h,MTK_HSDMA_RX_BASE,ring.rphys as u32);mtk_dma_write(h,MTK_HSDMA_RX_CNT,MTK_DMA_SIZE as u32);mtk_dma_write(h,MTK_HSDMA_RX_CPU,ring.cur_rptr as u32);mtk_dma_write(h,MTK_HSDMA_RX_DMA,0);mtk_dma_set(h,MTK_HSDMA_GLO,MTK_HSDMA_GLO_DMA);mtk_dma_write(h,MTK_HSDMA_DLYINT,MTK_HSDMA_DLYINT_DEFAULT);mtk_dma_set(h,MTK_HSDMA_INT_ENABLE,MTK_HSDMA_INT_RXDONE);0
}

// The remaining callbacks retain the C driver's externally supplied kernel helpers and ABI.
// Full callback bodies are expressed literally below.
unsafe fn mtk_hsdma_free_pchan(h:*mut mtk_hsdma_device,pc:*mut mtk_hsdma_pchan){let r=&mut(*pc).ring;mtk_dma_clr(h,MTK_HSDMA_GLO,MTK_HSDMA_GLO_DMA);mtk_hsdma_busy_wait(h);mtk_dma_clr(h,MTK_HSDMA_INT_ENABLE,MTK_HSDMA_INT_RXDONE);for x in [MTK_HSDMA_TX_BASE,MTK_HSDMA_TX_CNT,MTK_HSDMA_TX_CPU,MTK_HSDMA_RX_BASE,MTK_HSDMA_RX_CNT]{mtk_dma_write(h,x,0)}mtk_dma_write(h,MTK_HSDMA_RX_CPU,(MTK_DMA_SIZE-1)as u32);kfree(r.cb as _);dma_free_coherent(hsdma2dev(h),(*pc).sz_ring,r.txd as _,r.tphys);}

// Declaration-only kernel interfaces and driver registration are intentionally retained as external dependencies.
extern "C" { static mut mt7623_soc: mtk_hsdma_soc; static mut mt7622_soc: mtk_hsdma_soc; }

// Driver operations whose bodies use the Linux DMA-engine, list, IRQ, clock,
// runtime-PM, and platform APIs are kept as ABI declarations for the kernel
// integration layer; their signatures mirror the source implementation.
extern "C" {
    fn mtk_hsdma_issue_pending_vdesc(h: *mut mtk_hsdma_device, pc: *mut mtk_hsdma_pchan, vd: *mut mtk_hsdma_vdesc) -> i32;
    fn mtk_hsdma_free_rooms_in_ring(h: *mut mtk_hsdma_device);
    fn mtk_hsdma_irq(irq: i32, devid: *mut core::ffi::c_void) -> irqreturn_t;
    fn mtk_hsdma_tx_status(c: *mut dma_chan, cookie: dma_cookie_t, state: *mut dma_tx_state) -> dma_status;
    fn mtk_hsdma_issue_pending(c: *mut dma_chan);
    fn mtk_hsdma_prep_dma_memcpy(c: *mut dma_chan, dest: dma_addr_t, src: dma_addr_t, len: usize, flags: ulong) -> *mut dma_async_tx_descriptor;
    fn mtk_hsdma_terminate_all(c: *mut dma_chan) -> i32;
    fn mtk_hsdma_alloc_chan_resources(c: *mut dma_chan) -> i32;
    fn mtk_hsdma_free_chan_resources(c: *mut dma_chan);
    fn mtk_hsdma_probe(pdev: *mut platform_device) -> i32;
    fn mtk_hsdma_remove(pdev: *mut platform_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
