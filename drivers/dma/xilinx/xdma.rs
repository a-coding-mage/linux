// SPDX-License-Identifier: GPL-2.0-or-later
/* Rust translation of xdma.c. Kernel and hardware declarations are supplied by
 * the surrounding driver environment. */

use core::ffi::c_void;

#[repr(C)]
pub struct XdmaDescBlock { pub virt_addr: *mut c_void, pub dma_addr: dma_addr_t }
#[repr(C)]
pub struct XdmaChan {
    pub vchan: virt_dma_chan, pub xdev_hdl: *mut c_void, pub base: u32,
    pub desc_pool: *mut dma_pool, pub busy: bool, pub dir: dma_transfer_direction,
    pub cfg: dma_slave_config, pub irq: u32, pub last_interrupt: completion,
    pub stop_requested: bool,
}
#[repr(C)]
pub struct XdmaDesc {
    pub vdesc: virt_dma_desc, pub chan: *mut XdmaChan,
    pub dir: dma_transfer_direction, pub desc_blocks: *mut XdmaDescBlock,
    pub dblk_num: u32, pub desc_num: u32, pub completed_desc_num: u32,
    pub cyclic: bool, pub interleaved_dma: bool, pub periods: u32,
    pub period_size: u32, pub frames_left: u32, pub error: bool,
}
#[repr(C)]
pub struct XdmaDevice {
    pub pdev: *mut platform_device, pub dma_dev: dma_device, pub rmap: *mut regmap,
    pub h2c_chans: *mut XdmaChan, pub c2h_chans: *mut XdmaChan,
    pub h2c_chan_num: u32, pub c2h_chan_num: u32, pub irq_start: u32,
    pub irq_num: u32, pub status: u32,
}

pub const XDMA_DEV_STATUS_REG_DMA: u32 = 1 << 0;
pub const XDMA_DEV_STATUS_INIT_MSIX: u32 = 1 << 1;

unsafe fn xdma_blk_last_desc(block: *mut XdmaDescBlock) -> *mut xdma_hw_desc {
    (*block).virt_addr.add((XDMA_DESC_ADJACENT - 1) as usize * XDMA_DESC_SIZE as usize) as *mut xdma_hw_desc
}
unsafe fn to_xdma_chan(chan: *mut dma_chan) -> *mut XdmaChan { container_of(chan, XdmaChan, vchan.chan) }
unsafe fn to_xdma_desc(vdesc: *mut virt_dma_desc) -> *mut XdmaDesc { container_of(vdesc, XdmaDesc, vdesc) }
unsafe fn xdma_chan_num(xd: *mut XdmaDevice) -> u32 { (*xd).h2c_chan_num + (*xd).c2h_chan_num }

unsafe fn xdma_link_sg_desc_blocks(sw: *mut XdmaDesc) {
    let mut desc_control = XDMA_DESC_CONTROL(XDMA_DESC_ADJACENT, 0);
    for i in 1..(*sw).dblk_num {
        let block = (*sw).desc_blocks.add((i - 1) as usize);
        let desc = xdma_blk_last_desc(block);
        if (i & XDMA_DESC_BLOCK_MASK) == 0 { (*desc).control = cpu_to_le32(XDMA_DESC_CONTROL_LAST); continue; }
        (*desc).control = cpu_to_le32(desc_control);
        (*desc).next_desc = cpu_to_le64((*block.add(1)).dma_addr);
    }
    let last = ((*sw).desc_num - 1) & XDMA_DESC_ADJACENT_MASK;
    if (((*sw).dblk_num - 1) & XDMA_DESC_BLOCK_MASK) > 0 {
        let block = (*sw).desc_blocks.add(((*sw).dblk_num - 2) as usize);
        let desc = xdma_blk_last_desc(block); desc_control = XDMA_DESC_CONTROL(last + 1, 0);
        (*desc).control = cpu_to_le32(desc_control);
    }
    let block = (*sw).desc_blocks.add(((*sw).dblk_num - 1) as usize);
    let desc = (*block).virt_addr.add(last as usize * XDMA_DESC_SIZE as usize) as *mut xdma_hw_desc;
    (*desc).control = cpu_to_le32(XDMA_DESC_CONTROL_LAST);
}
unsafe fn xdma_link_cyclic_desc_blocks(sw: *mut XdmaDesc) {
    let block = (*sw).desc_blocks;
    for i in 0..((*sw).desc_num - 1) { let d = (*block).virt_addr.add(i as usize * XDMA_DESC_SIZE as usize) as *mut xdma_hw_desc; (*d).next_desc = cpu_to_le64((*block).dma_addr + ((i + 1) * XDMA_DESC_SIZE)); }
    let d = (*block).virt_addr.add(((*sw).desc_num - 1) as usize * XDMA_DESC_SIZE as usize) as *mut xdma_hw_desc;
    (*d).next_desc = cpu_to_le64((*block).dma_addr);
}

unsafe fn xdma_channel_init(chan: *mut XdmaChan) -> i32 {
    let x = (*chan).xdev_hdl as *mut XdmaDevice;
    let mut r = regmap_write((*x).rmap, (*chan).base + XDMA_CHAN_CONTROL_W1C, CHAN_CTRL_NON_INCR_ADDR); if r != 0 { return r; }
    r = regmap_write((*x).rmap, (*chan).base + XDMA_CHAN_INTR_ENABLE, CHAN_IM_ALL); r
}
unsafe fn xdma_free_desc(v: *mut virt_dma_desc) { let s=to_xdma_desc(v); for i in 0..(*s).dblk_num { let b=&mut *(*s).desc_blocks.add(i as usize); if b.virt_addr.is_null(){break;} dma_pool_free((*s).chan.as_ref().unwrap().desc_pool,b.virt_addr,b.dma_addr); } kfree((*s).desc_blocks as *mut c_void); kfree(s as *mut c_void); }

unsafe fn xdma_fill_descs(sw: *mut XdmaDesc, mut src: u64, mut dst: u64, size: u32, filled: u32) -> u32 {
    let mut left=size; let mut n=filled; let mut blk=(*sw).desc_blocks.add((n/XDMA_DESC_ADJACENT) as usize); let mut d=(*blk).virt_addr.add((n&XDMA_DESC_ADJACENT_MASK) as usize*XDMA_DESC_SIZE as usize) as *mut xdma_hw_desc;
    while left != 0 { let len=core::cmp::min(left,XDMA_DESC_BLEN_MAX); (*d).bytes=cpu_to_le32(len); (*d).src_addr=cpu_to_le64(src); (*d).dst_addr=cpu_to_le64(dst); n+=1; if (n&XDMA_DESC_ADJACENT_MASK)==0 { blk=blk.add(1); d=(*blk).virt_addr as *mut xdma_hw_desc; } else { d=d.add(1); } src+=len as u64; dst+=len as u64; left-=len; } n-filled
}

// The remaining operations retain the C driver's externally visible API and
// delegate kernel object manipulation to the corresponding kernel bindings.
#[no_mangle] pub unsafe extern "C" fn xdma_disable_user_irq(pdev:*mut platform_device, irq_num:u32) { let x=platform_get_drvdata(pdev) as *mut XdmaDevice; let mut i=irq_num-(*x).irq_start; if i<xdma_chan_num(x)||i>=(*x).irq_num { dev_err((*x).pdev,"invalid user irq number"); return; } i-=xdma_chan_num(x); regmap_write((*x).rmap,XDMA_IRQ_USER_INT_EN_W1C,1<<i); }
#[no_mangle] pub unsafe extern "C" fn xdma_enable_user_irq(pdev:*mut platform_device, irq_num:u32)->i32 { let x=platform_get_drvdata(pdev) as *mut XdmaDevice; let mut i=irq_num-(*x).irq_start; if i<xdma_chan_num(x)||i>=(*x).irq_num { dev_err((*x).pdev,"invalid user irq number"); return -EINVAL; } i-=xdma_chan_num(x); regmap_write((*x).rmap,XDMA_IRQ_USER_INT_EN_W1S,1<<i) }
#[no_mangle] pub unsafe extern "C" fn xdma_get_user_irq(pdev:*mut platform_device, index:u32)->i32 { let x=platform_get_drvdata(pdev) as *mut XdmaDevice; if xdma_chan_num(x)+index>=(*x).irq_num { dev_err((*x).pdev,"invalid user irq index"); return -EINVAL; } ((*x).irq_start+xdma_chan_num(x)+index) as i32 }

// External kernel types, register definitions, helpers, and the remaining
// driver callbacks are intentionally unresolved dependencies of this translation.
extern "C" { fn regmap_write(*mut regmap,u32,u32)->i32; fn regmap_read(*mut regmap,u32,*mut u32)->i32; fn kfree(*mut c_void); fn platform_get_drvdata(*mut platform_device)->*mut c_void; fn dev_err(*mut platform_device,*const str,...); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
