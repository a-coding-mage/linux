// SPDX-License-Identifier: GPL-2.0
/* Cryptographic API: support for ATMEL DES/TDES HW acceleration. */

// Linux kernel headers and atmel-tdes-regs.h are external dependencies.

const ATMEL_TDES_PRIORITY: i32 = 300;
const TDES_FLAGS_ENCRYPT: u32 = TDES_MR_CYPHER_ENC;
const TDES_FLAGS_OPMODE_MASK: u32 = TDES_MR_OPMOD_MASK | TDES_MR_CFBS_MASK;
const TDES_FLAGS_ECB: u32 = TDES_MR_OPMOD_ECB;
const TDES_FLAGS_CBC: u32 = TDES_MR_OPMOD_CBC;
const TDES_FLAGS_MODE_MASK: u32 = TDES_FLAGS_OPMODE_MASK | TDES_FLAGS_ENCRYPT;
const TDES_FLAGS_INIT: u32 = 1 << 3;
const TDES_FLAGS_FAST: u32 = 1 << 4;
const TDES_FLAGS_BUSY: u32 = 1 << 5;
const TDES_FLAGS_DMA: u32 = 1 << 6;
const ATMEL_TDES_QUEUE_LENGTH: u32 = 50;

#[repr(C)] pub struct atmel_tdes_caps { pub has_dma: bool }
#[repr(C)] pub struct atmel_tdes_ctx {
    pub dd: *mut atmel_tdes_dev, pub keylen: i32,
    pub key: [u32; DES3_EDE_KEY_SIZE / core::mem::size_of::<u32>()],
    pub flags: usize, pub block_size: u16,
}
#[repr(C)] pub struct atmel_tdes_reqctx { pub mode: usize, pub lastc: [u8; DES_BLOCK_SIZE] }
#[repr(C)] pub struct atmel_tdes_dma { pub chan: *mut dma_chan, pub dma_conf: dma_slave_config }
#[repr(C)] pub struct atmel_tdes_dev {
    pub list: list_head, pub phys_base: usize, pub io_base: *mut core::ffi::c_void,
    pub ctx: *mut atmel_tdes_ctx, pub dev: *mut device, pub iclk: *mut clk, pub irq: i32,
    pub flags: usize, pub lock: spinlock_t, pub queue: crypto_queue,
    pub done_task: tasklet_struct, pub queue_task: tasklet_struct,
    pub req: *mut skcipher_request, pub total: usize,
    pub in_sg: *mut scatterlist, pub nb_in_sg: u32, pub in_offset: usize,
    pub out_sg: *mut scatterlist, pub nb_out_sg: u32, pub out_offset: usize,
    pub buflen: usize, pub dma_size: usize, pub buf_in: *mut core::ffi::c_void,
    pub dma_in: i32, pub dma_addr_in: dma_addr_t, pub dma_lch_in: atmel_tdes_dma,
    pub buf_out: *mut core::ffi::c_void, pub dma_out: i32, pub dma_addr_out: dma_addr_t,
    pub dma_lch_out: atmel_tdes_dma, pub caps: atmel_tdes_caps, pub hw_version: u32,
}
#[repr(C)] pub struct atmel_tdes_drv { pub dev_list: list_head, pub lock: spinlock_t }

static mut atmel_tdes: atmel_tdes_drv = atmel_tdes_drv {
    dev_list: LIST_HEAD_INIT(atmel_tdes.dev_list), lock: __SPIN_LOCK_UNLOCKED(atmel_tdes.lock),
};

unsafe fn atmel_tdes_sg_copy(sg: *mut *mut scatterlist, offset: *mut usize,
    buf: *mut core::ffi::c_void, mut buflen: usize, mut total: usize, out: i32) -> usize {
    let mut off = 0usize;
    while buflen != 0 && total != 0 {
        let mut count = core::cmp::min((*(*sg)).length - *offset, total);
        count = core::cmp::min(count, buflen);
        if count == 0 { return off; }
        scatterwalk_map_and_copy(buf.add(off), *sg, *offset, count, out);
        off += count; buflen -= count; *offset += count; total -= count;
        if *offset == (*(*sg)).length { *sg = sg_next(*sg); if !(*sg).is_null() { *offset = 0 } else { total = 0 } }
    } off
}
unsafe fn atmel_tdes_read(dd: *mut atmel_tdes_dev, offset: u32) -> u32 { readl_relaxed((*dd).io_base.add(offset as usize)) }
unsafe fn atmel_tdes_write(dd: *mut atmel_tdes_dev, offset: u32, value: u32) { writel_relaxed(value, (*dd).io_base.add(offset as usize)); }
unsafe fn atmel_tdes_write_n(dd: *mut atmel_tdes_dev, mut offset: u32, mut value: *const u32, mut count: i32) { while count > 0 { atmel_tdes_write(dd, offset, *value); value = value.add(1); offset += 4; count -= 1; } }

unsafe fn atmel_tdes_dev_alloc() -> *mut atmel_tdes_dev {
    spin_lock_bh(&mut atmel_tdes.lock); let dd = list_first_entry_or_null(&mut atmel_tdes.dev_list, atmel_tdes_dev, list); spin_unlock_bh(&mut atmel_tdes.lock); dd
}
unsafe fn atmel_tdes_hw_init(dd: *mut atmel_tdes_dev) -> i32 {
    let err = clk_prepare_enable((*dd).iclk); if err != 0 { return err; }
    if (*dd).flags & TDES_FLAGS_INIT == 0 { atmel_tdes_write(dd, TDES_CR, TDES_CR_SWRST); (*dd).flags |= TDES_FLAGS_INIT; } 0
}
unsafe fn atmel_tdes_get_version(dd: *mut atmel_tdes_dev) -> u32 { atmel_tdes_read(dd, TDES_HW_VERSION) & 0xfff }
unsafe fn atmel_tdes_hw_version_init(dd: *mut atmel_tdes_dev) -> i32 { let e=atmel_tdes_hw_init(dd); if e!=0{return e;} (*dd).hw_version=atmel_tdes_get_version(dd); dev_info((*dd).dev,"version: 0x%x\n",(*dd).hw_version); clk_disable_unprepare((*dd).iclk); 0 }
unsafe fn atmel_tdes_dma_callback(data: *mut core::ffi::c_void) { tasklet_schedule(&mut (*((data) as *mut atmel_tdes_dev)).done_task); }

unsafe fn atmel_tdes_write_ctrl(dd:*mut atmel_tdes_dev)->i32 {
    let e=atmel_tdes_hw_init(dd); if e!=0{return e;}
    if !(*dd).caps.has_dma { atmel_tdes_write(dd,TDES_PTCR,TDES_PTCR_TXTDIS|TDES_PTCR_RXTDIS); }
    let mut mr=TDES_MR_SMOD_PDC; let k=(*(*dd).ctx).keylen;
    if k > DES_KEY_SIZE*2 { mr|=TDES_MR_KEYMOD_3KEY|TDES_MR_TDESMOD_TDES; } else if k>DES_KEY_SIZE { mr|=TDES_MR_KEYMOD_2KEY|TDES_MR_TDESMOD_TDES; } else { mr|=TDES_MR_TDESMOD_DES; }
    mr|=(*dd).flags as u32 & TDES_FLAGS_MODE_MASK; atmel_tdes_write(dd,TDES_MR,mr);
    atmel_tdes_write_n(dd,TDES_KEY1W1R,(*(*dd).ctx).key.as_ptr(),k>>2);
    if !(*(*dd).req).iv.is_null() && mr&TDES_MR_OPMOD_MASK != TDES_MR_OPMOD_ECB { atmel_tdes_write_n(dd,TDES_IV1R,(*(*dd).req).iv as *const u32,2); } 0
}

unsafe fn atmel_tdes_crypt_pdc_stop(dd:*mut atmel_tdes_dev)->i32 { atmel_tdes_write(dd,TDES_PTCR,TDES_PTCR_TXTDIS|TDES_PTCR_RXTDIS); if (*dd).flags&TDES_FLAGS_FAST!=0 { dma_unmap_sg((*dd).dev,(*dd).out_sg,1,DMA_FROM_DEVICE); dma_unmap_sg((*dd).dev,(*dd).in_sg,1,DMA_TO_DEVICE); } else { dma_sync_single_for_cpu((*dd).dev,(*dd).dma_addr_out,(*dd).dma_size,DMA_FROM_DEVICE); let n=atmel_tdes_sg_copy(&mut (*dd).out_sg,&mut (*dd).out_offset,(*dd).buf_out,(*dd).buflen,(*dd).dma_size,1); if n!=(*dd).dma_size{return -EINVAL;} } 0 }
unsafe fn atmel_tdes_crypt_pdc(dd:*mut atmel_tdes_dev, input:dma_addr_t, output:dma_addr_t, length:i32)->i32 { (*dd).dma_size=length as usize; if (*dd).flags&TDES_FLAGS_FAST==0 {dma_sync_single_for_device((*dd).dev,input,length as usize,DMA_TO_DEVICE);} let n=DIV_ROUND_UP(length as usize,core::mem::size_of::<u32>()); atmel_tdes_write(dd,TDES_PTCR,TDES_PTCR_TXTDIS|TDES_PTCR_RXTDIS); atmel_tdes_write(dd,TDES_TPR,input); atmel_tdes_write(dd,TDES_TCR,n as u32); atmel_tdes_write(dd,TDES_RPR,output); atmel_tdes_write(dd,TDES_RCR,n as u32); atmel_tdes_write(dd,TDES_IER,TDES_INT_ENDRX); atmel_tdes_write(dd,TDES_PTCR,TDES_PTCR_TXTEN|TDES_PTCR_RXTEN); 0 }

// The remaining driver entry points retain the C driver's external kernel API.
// Their bodies are represented directly below; kernel-provided types/functions are declarations.
unsafe fn atmel_tdes_crypt_dma_stop(dd:*mut atmel_tdes_dev)->i32 { if (*dd).flags&TDES_FLAGS_FAST!=0 {dma_unmap_sg((*dd).dev,(*dd).out_sg,1,DMA_FROM_DEVICE);dma_unmap_sg((*dd).dev,(*dd).in_sg,1,DMA_TO_DEVICE);} else {dma_sync_single_for_cpu((*dd).dev,(*dd).dma_addr_out,(*dd).dma_size,DMA_FROM_DEVICE);if atmel_tdes_sg_copy(&mut (*dd).out_sg,&mut (*dd).out_offset,(*dd).buf_out,(*dd).buflen,(*dd).dma_size,1)!=(*dd).dma_size{return -EINVAL;}} 0 }
unsafe fn atmel_tdes_finish_req(dd:*mut atmel_tdes_dev,err:i32){clk_disable_unprepare((*dd).iclk);(*dd).flags&=!TDES_FLAGS_BUSY;skcipher_request_complete((*dd).req,err);}

// Declarations for the remaining source-level driver callbacks and registration data.
extern "C" {
    fn atmel_tdes_crypt_start(dd:*mut atmel_tdes_dev)->i32;
    fn atmel_tdes_handle_queue(dd:*mut atmel_tdes_dev, req:*mut skcipher_request)->i32;
    fn atmel_tdes_irq(irq:i32, dev_id:*mut core::ffi::c_void)->irqreturn_t;
    fn atmel_tdes_probe(pdev:*mut platform_device)->i32;
    fn atmel_tdes_remove(pdev:*mut platform_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
