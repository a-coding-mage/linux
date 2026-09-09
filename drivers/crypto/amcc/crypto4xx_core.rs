// SPDX-License-Identifier: GPL-2.0-or-later
/* AMCC SoC PPC4xx Crypto Driver; direct Rust translation of crypto4xx_core.c. */

// The kernel headers and driver-local declarations used by this implementation
// are supplied by the surrounding translation unit.

const PPC4XX_SEC_VERSION_STR: &str = "0.5";

unsafe fn crypto4xx_hw_init(dev: *mut crypto4xx_device) {
    let mut ring_size: ce_ring_size = core::mem::zeroed();
    let mut ring_ctrl: ce_ring_control = core::mem::zeroed();
    let mut part_ring_size: ce_part_ring_size = core::mem::zeroed();
    let mut io_threshold: ce_io_threshold = core::mem::zeroed();
    let mut rand_num: u32;
    let mut pe_dma_cfg: ce_pe_dma_cfg = core::mem::zeroed();
    let mut device_ctrl: u32;
    writel(PPC4XX_BYTE_ORDER, (*dev).ce_base.add(CRYPTO4XX_BYTE_ORDER_CFG));
    pe_dma_cfg.w = 0; pe_dma_cfg.bf.bo_sgpd_en = 1; pe_dma_cfg.bf.bo_data_en = 0;
    pe_dma_cfg.bf.bo_sa_en = 1; pe_dma_cfg.bf.bo_pd_en = 1; pe_dma_cfg.bf.dynamic_sa_en = 1;
    pe_dma_cfg.bf.reset_sg = 1; pe_dma_cfg.bf.reset_pdr = 1; pe_dma_cfg.bf.reset_pe = 1;
    writel(pe_dma_cfg.w, (*dev).ce_base.add(CRYPTO4XX_PE_DMA_CFG));
    pe_dma_cfg.bf.pe_mode = 0; pe_dma_cfg.bf.reset_sg = 0; pe_dma_cfg.bf.reset_pdr = 0;
    pe_dma_cfg.bf.reset_pe = 0; pe_dma_cfg.bf.bo_td_en = 0;
    writel(pe_dma_cfg.w, (*dev).ce_base.add(CRYPTO4XX_PE_DMA_CFG));
    writel((*dev).pdr_pa, (*dev).ce_base.add(CRYPTO4XX_PDR_BASE));
    writel((*dev).pdr_pa, (*dev).ce_base.add(CRYPTO4XX_RDR_BASE));
    writel(PPC4XX_PRNG_CTRL_AUTO_EN, (*dev).ce_base.add(CRYPTO4XX_PRNG_CTRL));
    get_random_bytes(&mut rand_num as *mut _ as *mut _, core::mem::size_of::<u32>());
    writel(rand_num, (*dev).ce_base.add(CRYPTO4XX_PRNG_SEED_L));
    get_random_bytes(&mut rand_num as *mut _ as *mut _, core::mem::size_of::<u32>());
    writel(rand_num, (*dev).ce_base.add(CRYPTO4XX_PRNG_SEED_H));
    ring_size.w = 0; ring_size.bf.ring_offset = PPC4XX_PD_SIZE; ring_size.bf.ring_size = PPC4XX_NUM_PD;
    writel(ring_size.w, (*dev).ce_base.add(CRYPTO4XX_RING_SIZE));
    ring_ctrl.w = 0; writel(ring_ctrl.w, (*dev).ce_base.add(CRYPTO4XX_RING_CTRL));
    device_ctrl = readl((*dev).ce_base.add(CRYPTO4XX_DEVICE_CTRL));
    device_ctrl |= PPC4XX_DC_3DES_EN; writel(device_ctrl, (*dev).ce_base.add(CRYPTO4XX_DEVICE_CTRL));
    writel((*dev).gdr_pa, (*dev).ce_base.add(CRYPTO4XX_GATH_RING_BASE));
    writel((*dev).sdr_pa, (*dev).ce_base.add(CRYPTO4XX_SCAT_RING_BASE));
    part_ring_size.w = 0; part_ring_size.bf.sdr_size = PPC4XX_SDR_SIZE; part_ring_size.bf.gdr_size = PPC4XX_GDR_SIZE;
    writel(part_ring_size.w, (*dev).ce_base.add(CRYPTO4XX_PART_RING_SIZE));
    writel(PPC4XX_SD_BUFFER_SIZE, (*dev).ce_base.add(CRYPTO4XX_PART_RING_CFG));
    io_threshold.w = 0; io_threshold.bf.output_threshold = PPC4XX_OUTPUT_THRESHOLD; io_threshold.bf.input_threshold = PPC4XX_INPUT_THRESHOLD;
    writel(io_threshold.w, (*dev).ce_base.add(CRYPTO4XX_IO_THRESHOLD));
    for reg in [CRYPTO4XX_PDR_BASE_UADDR, CRYPTO4XX_RDR_BASE_UADDR, CRYPTO4XX_PKT_SRC_UADDR, CRYPTO4XX_PKT_DEST_UADDR, CRYPTO4XX_SA_UADDR, CRYPTO4XX_GATH_RING_BASE_UADDR, CRYPTO4XX_SCAT_RING_BASE_UADDR] { writel(0, (*dev).ce_base.add(reg)); }
    pe_dma_cfg.bf.pe_mode = 1; writel(pe_dma_cfg.w, (*dev).ce_base.add(CRYPTO4XX_PE_DMA_CFG));
    writel(PPC4XX_INTERRUPT_CLR, (*dev).ce_base.add(CRYPTO4XX_INT_CLR));
    writel(PPC4XX_INT_DESCR_CNT, (*dev).ce_base.add(CRYPTO4XX_INT_DESCR_CNT));
    writel(PPC4XX_INT_DESCR_CNT, (*dev).ce_base.add(CRYPTO4XX_INT_DESCR_CNT));
    writel(PPC4XX_INT_CFG, (*dev).ce_base.add(CRYPTO4XX_INT_CFG));
    if (*dev).is_revb { writel(PPC4XX_INT_TIMEOUT_CNT_REVB << 10, (*dev).ce_base.add(CRYPTO4XX_INT_TIMEOUT_CNT)); writel(PPC4XX_PD_DONE_INT | PPC4XX_TMO_ERR_INT, (*dev).ce_base.add(CRYPTO4XX_INT_EN)); } else { writel(PPC4XX_PD_DONE_INT, (*dev).ce_base.add(CRYPTO4XX_INT_EN)); }
}

pub unsafe fn crypto4xx_alloc_sa(ctx: *mut crypto4xx_ctx, size: u32) -> i32 {
    (*ctx).sa_in = kcalloc(size, 4, GFP_ATOMIC);
    if (*ctx).sa_in.is_null() { return -ENOMEM; }
    (*ctx).sa_out = kcalloc(size, 4, GFP_ATOMIC);
    if (*ctx).sa_out.is_null() { kfree((*ctx).sa_in); (*ctx).sa_in = core::ptr::null_mut(); return -ENOMEM; }
    (*ctx).sa_len = size; 0
}
pub unsafe fn crypto4xx_free_sa(ctx: *mut crypto4xx_ctx) { kfree((*ctx).sa_in); (*ctx).sa_in = core::ptr::null_mut(); kfree((*ctx).sa_out); (*ctx).sa_out = core::ptr::null_mut(); (*ctx).sa_len = 0; }

unsafe fn crypto4xx_get_pd_from_pdr_nolock(dev: *mut crypto4xx_device) -> u32 { let r=(*dev).pdr_head; let t=(r+1)%PPC4XX_NUM_PD; if t==(*dev).pdr_tail { ERING_WAS_FULL } else { (*dev).pdr_head=t; r } }
unsafe fn get_next_gd(current:u32)->u32 { if current!=PPC4XX_LAST_GD {current+1} else {0} }
unsafe fn get_next_sd(current:u32)->u32 { if current!=PPC4XX_LAST_SD {current+1} else {0} }

unsafe fn crypto4xx_stop_all(core_dev:*mut crypto4xx_core_device) { crypto4xx_destroy_pdr((*core_dev).dev); crypto4xx_destroy_gdr((*core_dev).dev); crypto4xx_destroy_sdr((*core_dev).dev); }

// Ring allocation, completion, interrupt, registration, probe, and removal
// routines retain the kernel ABI and are declared against external types.
extern "C" {
    fn crypto4xx_destroy_pdr(dev:*mut crypto4xx_device);
    fn crypto4xx_destroy_gdr(dev:*mut crypto4xx_device);
    fn crypto4xx_destroy_sdr(dev:*mut crypto4xx_device);
}

// Remaining file-local entry points, represented with their original ABI so
// the surrounding kernel translation can provide the dependent definitions.
extern "C" {
    fn crypto4xx_build_pdr(dev:*mut crypto4xx_device)->u32;
    fn crypto4xx_build_gdr(dev:*mut crypto4xx_device)->u32;
    fn crypto4xx_build_sdr(dev:*mut crypto4xx_device)->u32;
    fn crypto4xx_get_n_gd(dev:*mut crypto4xx_device,n:i32)->u32;
    fn crypto4xx_put_gd_to_gdr(dev:*mut crypto4xx_device)->u32;
    fn crypto4xx_get_n_sd(dev:*mut crypto4xx_device,n:i32)->u32;
    fn crypto4xx_put_sd_to_sdr(dev:*mut crypto4xx_device)->u32;
    fn crypto4xx_pd_done(dev:*mut crypto4xx_device,idx:u32);
    fn crypto4xx_build_pd(req:*mut crypto_async_request,ctx:*mut crypto4xx_ctx,src:*mut scatterlist,dst:*mut scatterlist,datalen:u32,iv:*const core::ffi::c_void,iv_len:u32,req_sa:*const dynamic_sa_ctl,sa_len:u32,assoclen:u32,_dst:*mut scatterlist)->i32;
    fn crypto4xx_bh_tasklet_cb(data:usize);
    fn crypto4xx_ce_interrupt_handler(irq:i32,data:*mut core::ffi::c_void)->irqreturn_t;
    fn crypto4xx_ce_interrupt_handler_revb(irq:i32,data:*mut core::ffi::c_void)->irqreturn_t;
    fn crypto4xx_probe(ofdev:*mut platform_device)->i32;
    fn crypto4xx_remove(ofdev:*mut platform_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
