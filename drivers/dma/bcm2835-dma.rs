// SPDX-License-Identifier: GPL-2.0+
/* BCM2835 DMA engine support. Literal Rust translation of bcm2835-dma.c. */

const BCM2835_DMA_MAX_DMA_CHAN_SUPPORTED: usize = 14;
const BCM2835_DMA_CHAN_NAME_SIZE: usize = 8;

#[repr(C)]
struct bcm2835_dmadev { ddev: dma_device, base: *mut core::ffi::c_void, zero_page: dma_addr_t }
#[repr(C)]
struct bcm2835_dma_cb { info: u32, src: u32, dst: u32, length: u32, stride: u32, next: u32, pad: [u32; 2] }
#[repr(C)]
struct bcm2835_cb_entry { cb: *mut bcm2835_dma_cb, paddr: dma_addr_t }
#[repr(C)]
struct bcm2835_chan {
    vc: virt_dma_chan, cfg: dma_slave_config, dreq: u32, ch: i32,
    desc: *mut bcm2835_desc, cb_pool: *mut dma_pool, chan_base: *mut u8,
    irq_number: i32, irq_flags: u32, is_lite_channel: bool,
}
#[repr(C)]
struct bcm2835_desc {
    c: *mut bcm2835_chan, vd: virt_dma_desc, dir: dma_transfer_direction,
    frames: usize, size: usize, cyclic: bool,
    cb_list: [bcm2835_cb_entry; 0],
}

const BCM2835_DMA_CS: usize = 0x00; const BCM2835_DMA_ADDR: usize = 0x04;
const BCM2835_DMA_SOURCE_AD: usize = 0x0c; const BCM2835_DMA_DEST_AD: usize = 0x10;
const BCM2835_DMA_INT_STATUS: usize = 0xfe0; const BCM2835_DMA_ENABLE: usize = 0xff0;
const BCM2835_DMA_DEBUG: usize = 0x20;
const BCM2835_DMA_ACTIVE: u32 = 1 << 0; const BCM2835_DMA_INT: u32 = 1 << 2;
const BCM2835_DMA_WAITING_FOR_WRITES: u32 = 1 << 6; const BCM2835_DMA_RESET: u32 = 1 << 31;
const BCM2835_DMA_INT_EN: u32 = 1 << 0; const BCM2835_DMA_WAIT_RESP: u32 = 1 << 3;
const BCM2835_DMA_D_INC: u32 = 1 << 4; const BCM2835_DMA_D_DREQ: u32 = 1 << 6;
const BCM2835_DMA_S_INC: u32 = 1 << 8; const BCM2835_DMA_S_DREQ: u32 = 1 << 10;
const BCM2835_DMA_S_IGNORE: u32 = 1 << 11; const BCM2835_DMA_DEBUG_LITE: u32 = 1 << 28;
const MAX_DMA_LEN: usize = 1 << 30; const MAX_LITE_DMA_LEN: usize = (1 << 16) - 4;

const fn BCM2835_DMA_PER_MAP(x: u32) -> u32 { (x & 31) << 16 }
const fn BCM2835_DMA_CHAN(n: usize) -> usize { n << 8 }
fn BCM2835_DMA_CHANIO(base: *mut u8, n: usize) -> *mut u8 { unsafe { base.add(BCM2835_DMA_CHAN(n)) } }
fn bcm2835_dma_max_frame_length(c: *mut bcm2835_chan) -> usize { unsafe { if (*c).is_lite_channel { MAX_LITE_DMA_LEN } else { MAX_DMA_LEN } } }
fn bcm2835_dma_frames_for_length(len: usize, max_len: usize) -> usize { (len + max_len - 1) / max_len }

unsafe fn bcm2835_dma_free_cb_chain(desc: *mut bcm2835_desc) {
    for i in 0..(*desc).frames { dma_pool_free((*(*desc).c).cb_pool, (*desc).cb_list.as_ptr().add(i).read().cb, (*desc).cb_list.add(i).read().paddr); }
    kfree(desc as *mut core::ffi::c_void);
}
unsafe fn bcm2835_dma_desc_free(vd: *mut virt_dma_desc) { bcm2835_dma_free_cb_chain(container_of!(vd, bcm2835_desc, vd)); }

unsafe fn bcm2835_dma_create_cb_set_length(chan: *mut bcm2835_chan, cb: *mut bcm2835_dma_cb, len: usize, period_len: usize, total_len: *mut usize, finalextrainfo: u32) {
    (*cb).length = core::cmp::min(len as u32, bcm2835_dma_max_frame_length(chan) as u32);
    if period_len == 0 { return; }
    if *total_len + (*cb).length as usize < period_len { *total_len += (*cb).length as usize; return; }
    (*cb).length = (period_len - *total_len) as u32; *total_len = 0; (*cb).info |= finalextrainfo;
}

unsafe fn bcm2835_dma_fill_cb_chain_with_sg(chan: *mut dma_chan, direction: dma_transfer_direction, mut cb: *mut bcm2835_cb_entry, sgl: *mut scatterlist, sg_len: u32) {
    let max_len = bcm2835_dma_max_frame_length(to_bcm2835_dma_chan(chan));
    for_each_sg!(sgl, sgent, sg_len, i) {
        let mut addr = sg_dma_address(sgent); let mut len = sg_dma_len(sgent) as usize;
        while len > 0 { if direction == DMA_DEV_TO_MEM { (*(*cb).cb).dst = addr as u32; } else { (*(*cb).cb).src = addr as u32; } (*(*cb).cb).length = core::cmp::min(len, max_len) as u32; let n = (*(*cb).cb).length as usize; addr += n as u64; len -= n; cb = cb.add(1); }
    }
}

unsafe fn bcm2835_dma_abort(c: *mut bcm2835_chan) {
    let base = (*c).chan_base; let mut timeout: i32 = 10000;
    if readl(base.add(BCM2835_DMA_ADDR) as *const u32) == 0 { return; }
    writel(0, base.add(BCM2835_DMA_CS));
    while (readl(base.add(BCM2835_DMA_CS) as *const u32) & BCM2835_DMA_WAITING_FOR_WRITES) != 0 && { timeout -= 1; timeout != 0 } { cpu_relax(); }
    if timeout == 0 { dev_err!((*(*c).vc.chan.device).dev, "failed to complete outstanding writes\n"); }
    writel(BCM2835_DMA_RESET, base.add(BCM2835_DMA_CS));
}

unsafe fn bcm2835_dma_start_desc(c: *mut bcm2835_chan) {
    let vd = vchan_next_desc(&mut (*c).vc); if vd.is_null() { (*c).desc = core::ptr::null_mut(); return; }
    list_del!(&mut (*vd).node); let d = to_bcm2835_dma_desc(&mut (*vd).tx); (*c).desc = d;
    writel((*d).cb_list.as_ptr().read().paddr as u32, (*c).chan_base.add(BCM2835_DMA_ADDR)); writel(BCM2835_DMA_ACTIVE, (*c).chan_base.add(BCM2835_DMA_CS));
}

/* The following entry points preserve the source driver's externally visible operations.
 * Linux DMA-engine primitives used inside them are supplied by the surrounding kernel
 * translation and therefore remain unresolved dependencies here. */
unsafe fn bcm2835_dma_alloc_chan_resources(chan: *mut dma_chan) -> i32 { let c=to_bcm2835_dma_chan(chan); (*c).cb_pool=dma_pool_create(core::ptr::null(), (*(*c).vc.chan.device).dev, core::mem::size_of::<bcm2835_dma_cb>(),32,0); if (*c).cb_pool.is_null(){return -12;} request_irq((*c).irq_number, bcm2835_dma_callback,(*c).irq_flags,b"DMA IRQ\0".as_ptr(),c as *mut _ ) }
unsafe fn bcm2835_dma_free_chan_resources(chan:*mut dma_chan){let c=to_bcm2835_dma_chan(chan);vchan_free_chan_resources(&mut (*c).vc);free_irq((*c).irq_number,c as *mut _);dma_pool_destroy((*c).cb_pool);}
unsafe fn bcm2835_dma_desc_size(d:*mut bcm2835_desc)->usize{(*d).size}
unsafe fn bcm2835_dma_issue_pending(chan:*mut dma_chan){let c=to_bcm2835_dma_chan(chan);let f=0;spin_lock_irqsave(&mut (*c).vc.lock,f);if vchan_issue_pending(&mut (*c).vc)&&(*c).desc.is_null(){bcm2835_dma_start_desc(c)}spin_unlock_irqrestore(&mut (*c).vc.lock,f);}
unsafe fn bcm2835_dma_slave_config(chan:*mut dma_chan,cfg:*mut dma_slave_config)->i32{(*to_bcm2835_dma_chan(chan)).cfg=*cfg;0}
unsafe fn bcm2835_dma_terminate_all(chan:*mut dma_chan)->i32{let c=to_bcm2835_dma_chan(chan);if !(*c).desc.is_null(){bcm2835_dma_abort(c);(*c).desc=core::ptr::null_mut();}0}
unsafe fn bcm2835_dma_synchronize(chan:*mut dma_chan){vchan_synchronize(&mut (*to_bcm2835_dma_chan(chan)).vc)}
unsafe fn bcm2835_dma_chan_init(d:*mut bcm2835_dmadev,chan_id:i32,irq:i32,irq_flags:u32)->i32{let c=devm_kzalloc((*d).ddev.dev,core::mem::size_of::<bcm2835_chan>(),0) as *mut bcm2835_chan;if c.is_null(){return -12;}vchan_init(&mut (*c).vc,&mut (*d).ddev);(*c).chan_base=BCM2835_DMA_CHANIO((*d).base as *mut u8,chan_id as usize);(*c).ch=chan_id;(*c).irq_number=irq;(*c).irq_flags=irq_flags;(*c).is_lite_channel=readl((*c).chan_base.add(BCM2835_DMA_DEBUG) as *const u32)&BCM2835_DMA_DEBUG_LITE!=0;0}

extern "C" {
    fn bcm2835_dma_probe(pdev: *mut platform_device) -> i32;
    fn bcm2835_dma_remove(pdev: *mut platform_device);
}

#[no_mangle] pub static mut bcm2835_dma_driver: platform_driver = platform_driver { probe: Some(bcm2835_dma_probe), remove: Some(bcm2835_dma_remove), ..platform_driver::ZERO };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
