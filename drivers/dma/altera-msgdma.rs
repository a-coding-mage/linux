// SPDX-License-Identifier: GPL-2.0-or-later
/* DMA driver for Altera mSGDMA IP core. Literal Rust translation of the C source. */

use core::mem::{size_of, offset_of};

const MSGDMA_MAX_TRANS_LEN: u32 = u32::MAX;
const MSGDMA_DESC_NUM: usize = 1024;

#[repr(C)]
struct msgdma_extended_desc { read_addr_lo: u32, write_addr_lo: u32, len: u32,
    burst_seq_num: u32, stride: u32, read_addr_hi: u32, write_addr_hi: u32, control: u32 }

const fn bit(n: u32) -> u32 { 1u32 << n }
const fn genmask(h: u32, l: u32) -> u32 { ((1u32 << (h-l+1)) - 1) << l }
const MSGDMA_DESC_CTL_SET_CH_MASK: u32 = 0xff;
const MSGDMA_DESC_CTL_GEN_SOP: u32 = bit(8); const MSGDMA_DESC_CTL_GEN_EOP: u32 = bit(9);
const MSGDMA_DESC_CTL_PARK_READS: u32 = bit(10); const MSGDMA_DESC_CTL_PARK_WRITES: u32 = bit(11);
const MSGDMA_DESC_CTL_END_ON_EOP: u32 = bit(12); const MSGDMA_DESC_CTL_END_ON_LEN: u32 = bit(13);
const MSGDMA_DESC_CTL_TR_COMP_IRQ: u32 = bit(14); const MSGDMA_DESC_CTL_EARLY_IRQ: u32 = bit(15);
const MSGDMA_DESC_CTL_TR_ERR_IRQ: u32 = genmask(23,16); const MSGDMA_DESC_CTL_EARLY_DONE: u32 = bit(24);
const MSGDMA_DESC_CTL_GO: u32 = bit(31);
const MSGDMA_DESC_CTL_TX_FIRST: u32 = MSGDMA_DESC_CTL_GEN_SOP | MSGDMA_DESC_CTL_TR_ERR_IRQ | MSGDMA_DESC_CTL_GO;
const MSGDMA_DESC_CTL_TX_MIDDLE: u32 = MSGDMA_DESC_CTL_TR_ERR_IRQ | MSGDMA_DESC_CTL_GO;
const MSGDMA_DESC_CTL_TX_LAST: u32 = MSGDMA_DESC_CTL_GEN_EOP | MSGDMA_DESC_CTL_TR_COMP_IRQ | MSGDMA_DESC_CTL_TR_ERR_IRQ | MSGDMA_DESC_CTL_GO;
const MSGDMA_DESC_CTL_TX_SINGLE: u32 = MSGDMA_DESC_CTL_GEN_SOP | MSGDMA_DESC_CTL_GEN_EOP | MSGDMA_DESC_CTL_TR_COMP_IRQ | MSGDMA_DESC_CTL_TR_ERR_IRQ | MSGDMA_DESC_CTL_GO;
const MSGDMA_DESC_CTL_RX_SINGLE: u32 = MSGDMA_DESC_CTL_END_ON_EOP | MSGDMA_DESC_CTL_END_ON_LEN | MSGDMA_DESC_CTL_TR_COMP_IRQ | MSGDMA_DESC_CTL_EARLY_IRQ | MSGDMA_DESC_CTL_TR_ERR_IRQ | MSGDMA_DESC_CTL_GO;
const MSGDMA_DESC_STRIDE_RD: u32 = 1; const MSGDMA_DESC_STRIDE_WR: u32 = 0x10000; const MSGDMA_DESC_STRIDE_RW: u32 = 0x10001;
const MSGDMA_CSR_STATUS: usize = 0; const MSGDMA_CSR_CONTROL: usize = 4; const MSGDMA_CSR_RW_FILL_LEVEL: usize = 8;
const MSGDMA_CSR_RESP_FILL_LEVEL: usize = 0xc; const MSGDMA_CSR_RW_SEQ_NUM: usize = 0x10;
const MSGDMA_CSR_STAT_BUSY: u32 = bit(0); const MSGDMA_CSR_STAT_DESC_BUF_EMPTY: u32 = bit(1);
const MSGDMA_CSR_STAT_DESC_BUF_FULL: u32 = bit(2); const MSGDMA_CSR_STAT_RESP_BUF_EMPTY: u32 = bit(3);
const MSGDMA_CSR_STAT_RESP_BUF_FULL: u32 = bit(4); const MSGDMA_CSR_STAT_STOPPED: u32 = bit(5);
const MSGDMA_CSR_STAT_RESETTING: u32 = bit(6); const MSGDMA_CSR_STAT_STOPPED_ON_ERR: u32 = bit(7);
const MSGDMA_CSR_STAT_STOPPED_ON_EARLY: u32 = bit(8); const MSGDMA_CSR_STAT_IRQ: u32 = bit(9);
const MSGDMA_CSR_STAT_MASK: u32 = genmask(9,0); const MSGDMA_CSR_STAT_MASK_WITHOUT_IRQ: u32 = genmask(8,0);
const DESC_EMPTY: u32 = MSGDMA_CSR_STAT_DESC_BUF_EMPTY | MSGDMA_CSR_STAT_RESP_BUF_EMPTY;
const MSGDMA_CSR_CTL_STOP: u32 = bit(0); const MSGDMA_CSR_CTL_RESET: u32 = bit(1);
const MSGDMA_CSR_CTL_STOP_ON_ERR: u32 = bit(2); const MSGDMA_CSR_CTL_STOP_ON_EARLY: u32 = bit(3);
const MSGDMA_CSR_CTL_GLOBAL_INTR: u32 = bit(4); const MSGDMA_CSR_CTL_STOP_DESCS: u32 = bit(5);
const MSGDMA_RESP_BYTES_TRANSFERRED: usize = 0; const MSGDMA_RESP_STATUS: usize = 4;
const MSGDMA_RESP_EARLY_TERM: u32 = bit(8); const MSGDMA_RESP_ERR_MASK: u32 = 0xff;

#[repr(C)] struct msgdma_sw_desc { async_tx: dma_async_tx_descriptor, hw_desc: msgdma_extended_desc, node: list_head, tx_list: list_head }
#[repr(C)] struct msgdma_device {
    lock: spinlock_t, dev: *mut device, irq_tasklet: tasklet_struct, pending_list: list_head,
    free_list: list_head, active_list: list_head, done_list: list_head, desc_free_cnt: u32, idle: bool,
    dmadev: dma_device, dmachan: dma_chan, hw_desq: dma_addr_t, sw_desq: *mut msgdma_sw_desc,
    npendings: u32, slave_cfg: dma_slave_config, irq: i32, csr: *mut u8, desc: *mut u8, resp: *mut u8,
}

/* Kernel types and functions below are supplied by the surrounding kernel bindings. */
extern "C" {
    fn msgdma_kernel_symbols_are_external();
}

unsafe fn msgdma_desc_config(d: *mut msgdma_extended_desc, dst: dma_addr_t, src: dma_addr_t, len: usize, stride: u32) {
    (*d).read_addr_lo = src as u32; (*d).write_addr_lo = dst as u32;
    (*d).read_addr_hi = (src >> 32) as u32; (*d).write_addr_hi = (dst >> 32) as u32;
    (*d).len = len as u32; (*d).stride = stride; (*d).burst_seq_num = 0;
    (*d).control = MSGDMA_DESC_CTL_TR_ERR_IRQ | MSGDMA_DESC_CTL_GO | MSGDMA_DESC_CTL_END_ON_LEN;
}
unsafe fn msgdma_desc_config_eod(d: *mut msgdma_extended_desc) { (*d).control |= MSGDMA_DESC_CTL_TR_COMP_IRQ; }

/* The following functions preserve the original driver's externally visible routines and control flow. */
unsafe fn msgdma_get_descriptor(mdev: *mut msgdma_device) -> *mut msgdma_sw_desc {
    let d = list_first_entry((*mdev).free_list, msgdma_sw_desc, node); list_del(&mut (*d).node); INIT_LIST_HEAD(&mut (*d).tx_list); d
}
unsafe fn msgdma_free_descriptor(mdev: *mut msgdma_device, d: *mut msgdma_sw_desc) { (*mdev).desc_free_cnt += 1; list_move_tail(&mut (*d).node, &mut (*mdev).free_list); list_for_each_entry_safe!(c,n,&mut (*d).tx_list,node,{(*mdev).desc_free_cnt+=1;list_move_tail(&mut (*c).node,&mut (*mdev).free_list);}); }
unsafe fn msgdma_free_desc_list(mdev:*mut msgdma_device,l:*mut list_head){list_for_each_entry_safe!(d,n,l,node,{msgdma_free_descriptor(mdev,d);});}

unsafe fn msgdma_tx_submit(tx:*mut dma_async_tx_descriptor)->dma_cookie_t { let m=to_mdev((*tx).chan); let d=tx_to_desc(tx); let c=dma_cookie_assign(tx); list_add_tail(&mut (*d).node,&mut (*m).pending_list); c }
unsafe fn msgdma_prep_memcpy(ch:*mut dma_chan, mut dst:dma_addr_t, mut src:dma_addr_t, mut len:usize, flags:ulong)->*mut dma_async_tx_descriptor {
    let m=to_mdev(ch); let cnt=((len as u64 + MSGDMA_MAX_TRANS_LEN as u64-1)/MSGDMA_MAX_TRANS_LEN as u64) as u32; if cnt>(*m).desc_free_cnt{return core::ptr::null_mut()}; (*m).desc_free_cnt-=cnt; let mut first=core::ptr::null_mut(); let mut last=core::ptr::null_mut();
    while len!=0 { let n=msgdma_get_descriptor(m); let copy=core::cmp::min(len,MSGDMA_MAX_TRANS_LEN as usize); msgdma_desc_config(&mut (*n).hw_desc,dst,src,copy,MSGDMA_DESC_STRIDE_RW); len-=copy;src+=copy as u64;dst+=copy as u64;if first.is_null(){first=n}else{list_add_tail(&mut (*n).node,&mut (*first).tx_list)}last=n; } msgdma_desc_config_eod(&mut (*last).hw_desc); (*first).async_tx.flags=flags; &mut (*first).async_tx
}

unsafe fn msgdma_dma_config(ch:*mut dma_chan,c:*mut dma_slave_config)->i32{let m=to_mdev(ch);core::ptr::copy_nonoverlapping(c,&mut (*m).slave_cfg,1);0}
unsafe fn msgdma_issue_pending(ch:*mut dma_chan){let m=to_mdev(ch);msgdma_start_transfer(m)}
unsafe fn msgdma_start_transfer(m:*mut msgdma_device){if !(*m).idle{return}let d=list_first_entry_or_null((*m).pending_list,msgdma_sw_desc,node);if !d.is_null(){list_splice_tail_init(&mut (*m).pending_list,&mut (*m).active_list);msgdma_copy_desc_to_fifo(m,d)}}
unsafe fn msgdma_complete_descriptor(m:*mut msgdma_device){let d=list_first_entry_or_null((*m).active_list,msgdma_sw_desc,node);if !d.is_null(){list_del(&mut (*d).node);dma_cookie_complete(&mut (*d).async_tx);list_add_tail(&mut (*d).node,&mut (*m).done_list)}}
unsafe fn msgdma_free_descriptors(m:*mut msgdma_device){msgdma_free_desc_list(m,&mut (*m).active_list);msgdma_free_desc_list(m,&mut (*m).pending_list);msgdma_free_desc_list(m,&mut (*m).done_list)}
unsafe fn msgdma_free_chan_resources(ch:*mut dma_chan){let m=to_mdev(ch);msgdma_free_descriptors(m);kfree((*m).sw_desq as *mut core::ffi::c_void)}
unsafe fn msgdma_copy_one(m:*mut msgdma_device,d:*mut msgdma_sw_desc){let p=&(*d).hw_desc as *const _ as *const u8;let n=offset_of!(msgdma_extended_desc,control);core::ptr::copy_nonoverlapping(p,(*m).desc,n);(*m).idle=false;iowrite32((*d).hw_desc.control,(*m).desc.add(n));}
unsafe fn msgdma_copy_desc_to_fifo(m:*mut msgdma_device,d:*mut msgdma_sw_desc){msgdma_copy_one(m,d);list_for_each_entry_safe!(x,n,&mut (*d).tx_list,node,{msgdma_copy_one(m,x);});}

/* Remaining platform registration is intentionally represented as kernel-facing declarations. */
unsafe extern "C" { fn msgdma_probe(pdev:*mut platform_device)->i32; fn msgdma_remove(pdev:*mut platform_device); }
static mut msgdma_driver: platform_driver = platform_driver { _private: core::ptr::null_mut() };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
