// SPDX-License-Identifier: GPL-2.0-only
/* Support for OMAP DES and Triple DES HW acceleration. */

// Kernel dependencies supplied by the surrounding repository are intentionally
// referenced by their C/Rust-compatible names and are not implemented here.

const DST_MAXBURST: u32 = 2;
const DES_BLOCK_WORDS: usize = DES_BLOCK_SIZE >> 2;
const FLAGS_MODE_MASK: u32 = 0x000f;
const FLAGS_ENCRYPT: u32 = 1 << 0;
const FLAGS_CBC: u32 = 1 << 1;
const FLAGS_INIT: u32 = 1 << 4;
const FLAGS_BUSY: u32 = 1 << 6;
const DEFAULT_AUTOSUSPEND_DELAY: u32 = 1000;
const FLAGS_IN_DATA_ST_SHIFT: u32 = 8;
const FLAGS_OUT_DATA_ST_SHIFT: u32 = 10;
const OMAP_DES_QUEUE_LENGTH: usize = 1;
const OMAP_DES_CACHE_SIZE: usize = 0;

#[repr(C)]
struct omap_des_ctx {
    dd: *mut omap_des_dev,
    keylen: i32,
    key: [__le32; (3 * DES_KEY_SIZE) / core::mem::size_of::<u32>()],
    flags: c_ulong,
}

#[repr(C)]
struct omap_des_reqctx { mode: c_ulong }

#[repr(C)]
struct omap_des_algs_info {
    algs_list: *mut skcipher_engine_alg,
    size: u32,
    registered: u32,
}

#[repr(C)]
struct omap_des_pdata {
    algs_info: *mut omap_des_algs_info,
    algs_info_size: u32,
    trigger: Option<unsafe extern "C" fn(*mut omap_des_dev, i32)>,
    key_ofs: u32, iv_ofs: u32, ctrl_ofs: u32, data_ofs: u32,
    rev_ofs: u32, mask_ofs: u32, irq_enable_ofs: u32, irq_status_ofs: u32,
    dma_enable_in: u32, dma_enable_out: u32, dma_start: u32,
    major_mask: u32, major_shift: u32, minor_mask: u32, minor_shift: u32,
}

#[repr(C)]
struct omap_des_dev {
    list: list_head, phys_base: c_ulong, io_base: *mut c_void,
    ctx: *mut omap_des_ctx, dev: *mut device, flags: c_ulong, err: i32,
    done_task: work_struct, req: *mut skcipher_request, engine: *mut crypto_engine,
    total: usize, total_save: usize, in_sg: *mut scatterlist, out_sg: *mut scatterlist,
    in_sgl: scatterlist, out_sgl: scatterlist, orig_out: *mut scatterlist,
    in_sg_offset: u32, out_sg_offset: u32, dma_lch_in: *mut dma_chan,
    dma_lch_out: *mut dma_chan, in_sg_len: i32, out_sg_len: i32,
    pio_only: i32, pdata: *const omap_des_pdata,
}

static mut dev_list: list_head = LIST_HEAD_INIT;
static mut list_lock: spinlock_t = SPINLOCK_INIT;

#[inline] unsafe fn DES_REG_KEY(dd: *mut omap_des_dev, x: u32) -> u32 { (*(*dd).pdata).key_ofs - ((x ^ 1) * 4) }
#[inline] unsafe fn DES_REG_IV(dd: *mut omap_des_dev, x: u32) -> u32 { (*(*dd).pdata).iv_ofs + x * 4 }
#[inline] unsafe fn DES_REG_CTRL(dd: *mut omap_des_dev) -> u32 { (*(*dd).pdata).ctrl_ofs }
const DES_REG_CTRL_CBC: u32 = 1 << 4;
const DES_REG_CTRL_TDES: u32 = 1 << 3;
const DES_REG_CTRL_DIRECTION: u32 = 1 << 2;
const DES_REG_CTRL_INPUT_READY: u32 = 1 << 1;
const DES_REG_CTRL_OUTPUT_READY: u32 = 1;
#[inline] unsafe fn DES_REG_DATA_N(dd: *mut omap_des_dev, x: u32) -> u32 { (*(*dd).pdata).data_ofs + x * 4 }
#[inline] unsafe fn DES_REG_REV(dd: *mut omap_des_dev) -> u32 { (*(*dd).pdata).rev_ofs }
#[inline] unsafe fn DES_REG_MASK(dd: *mut omap_des_dev) -> u32 { (*(*dd).pdata).mask_ofs }
#[inline] fn DES_REG_LENGTH_N(x: u32) -> u32 { 0x24 + x * 4 }
#[inline] unsafe fn DES_REG_IRQ_STATUS(dd: *mut omap_des_dev) -> u32 { (*(*dd).pdata).irq_status_ofs }
#[inline] unsafe fn DES_REG_IRQ_ENABLE(dd: *mut omap_des_dev) -> u32 { (*(*dd).pdata).irq_enable_ofs }
const DES_REG_IRQ_DATA_IN: u32 = 1 << 1;
const DES_REG_IRQ_DATA_OUT: u32 = 1 << 2;

unsafe fn omap_des_read(dd: *mut omap_des_dev, offset: u32) -> u32 { __raw_readl((*dd).io_base.add(offset as usize)) }
unsafe fn omap_des_write(dd: *mut omap_des_dev, offset: u32, value: u32) { __raw_writel(value, (*dd).io_base.add(offset as usize)); }
unsafe fn omap_des_write_mask(dd: *mut omap_des_dev, offset: u32, value: u32, mask: u32) { let mut v=omap_des_read(dd,offset); v &= !mask; v |= value; omap_des_write(dd,offset,v); }
unsafe fn omap_des_write_n(dd: *mut omap_des_dev, mut offset: u32, mut value: *mut u32, mut count: i32) { while count > 0 { omap_des_write(dd,offset,*value); value=value.add(1); offset+=4; count-=1; } }

unsafe fn omap_des_hw_init(dd: *mut omap_des_dev) -> i32 { let e=pm_runtime_resume_and_get((*dd).dev); if e<0 { dev_err((*dd).dev, "%s: failed to get_sync(%d)\n", "omap_des_hw_init", e); return e; } if (*dd).flags & FLAGS_INIT as c_ulong == 0 { (*dd).flags |= FLAGS_INIT as c_ulong; (*dd).err=0; } 0 }
unsafe fn omap_des_write_ctrl(dd: *mut omap_des_dev) -> i32 { let e=omap_des_hw_init(dd); if e!=0{return e;} let n=((*(*dd).ctx).keylen as usize/core::mem::size_of::<u32>()) as u32; for i in 0..n { omap_des_write(dd,DES_REG_KEY(dd,i),__le32_to_cpu((*(*dd).ctx).key[i as usize])); } if (*dd).flags & FLAGS_CBC as c_ulong !=0 && !(*dd).req.is_null() && !(*(*dd).req).iv.is_null(){omap_des_write_n(dd,DES_REG_IV(dd,0),(*(*dd).req).iv as *mut u32,2);} let mut v=0; if (*dd).flags&FLAGS_CBC as c_ulong!=0{v|=DES_REG_CTRL_CBC;} if (*dd).flags&FLAGS_ENCRYPT as c_ulong!=0{v|=DES_REG_CTRL_DIRECTION;} if n==6{v|=DES_REG_CTRL_TDES;} omap_des_write_mask(dd,DES_REG_CTRL(dd),v,DES_REG_CTRL_CBC|DES_REG_CTRL_DIRECTION|DES_REG_CTRL_TDES); 0 }

unsafe fn omap_des_dma_trigger_omap4(dd:*mut omap_des_dev,length:i32){omap_des_write(dd,DES_REG_LENGTH_N(0),length as u32);let mut v=(*(*dd).pdata).dma_start;if !(*dd).dma_lch_out.is_null(){v|=(*(*dd).pdata).dma_enable_out;}if !(*dd).dma_lch_in.is_null(){v|=(*(*dd).pdata).dma_enable_in;}let m=(*(*dd).pdata).dma_enable_out|(*(*dd).pdata).dma_enable_in|(*(*dd).pdata).dma_start;omap_des_write_mask(dd,DES_REG_MASK(dd),v,m);}
unsafe fn omap_des_dma_stop(dd:*mut omap_des_dev){let m=(*(*dd).pdata).dma_enable_out|(*(*dd).pdata).dma_enable_in|(*(*dd).pdata).dma_start;omap_des_write_mask(dd,DES_REG_MASK(dd),0,m);}

// Remaining driver entry points retain the original kernel interfaces and are
// declared here for linkage with the surrounding translated kernel sources.
extern "C" {
    fn omap_des_find_dev(ctx:*mut omap_des_ctx)->*mut omap_des_dev;
    fn omap_des_prepare_req(req:*mut skcipher_request,dd:*mut omap_des_dev)->i32;
    fn omap_des_crypt_dma_start(dd:*mut omap_des_dev)->i32;
    fn omap_des_done_task(t:*mut work_struct);
    fn omap_des_crypt(req:*mut skcipher_request,mode:c_ulong)->i32;
    fn omap_des_setkey(cipher:*mut crypto_skcipher,key:*const u8,keylen:u32)->i32;
    fn omap_des3_setkey(cipher:*mut crypto_skcipher,key:*const u8,keylen:u32)->i32;
    fn omap_des_ecb_encrypt(req:*mut skcipher_request)->i32;
    fn omap_des_ecb_decrypt(req:*mut skcipher_request)->i32;
    fn omap_des_cbc_encrypt(req:*mut skcipher_request)->i32;
    fn omap_des_cbc_decrypt(req:*mut skcipher_request)->i32;
    fn omap_des_init_tfm(tfm:*mut crypto_skcipher)->i32;
    fn omap_des_irq(irq:i32,dev_id:*mut c_void)->irqreturn_t;
    fn omap_des_probe(pdev:*mut platform_device)->i32;
    fn omap_des_remove(pdev:*mut platform_device);
}

// The algorithm descriptors, OF match table, platform driver, PM operations,
// and MODULE_* declarations correspond directly to the C definitions above;
// their concrete kernel layout is supplied by the surrounding bindings.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
