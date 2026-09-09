// SPDX-License-Identifier: GPL-2.0
/* arch/sh/drivers/dma/dma-sh.c - SuperH On-chip DMAC Support */

// External kernel, architecture, and machine symbols are supplied by dependencies.

#[cfg(SH_DMAC_BASE1)]
const SH_DMAC_NR_MD_CH: usize = CONFIG_NR_ONCHIP_DMA_CHANNELS / 2;
#[cfg(not(SH_DMAC_BASE1))]
const SH_DMAC_NR_MD_CH: usize = CONFIG_NR_ONCHIP_DMA_CHANNELS;

const SH_DMAC_CH_SZ: usize = 0x10;
const RS_DUAL: u32 = DM_INC | SM_INC | RS_AUTO | TS_INDEX2VAL(XMIT_SZ_32BIT);

unsafe fn dma_find_base(chan: u32) -> usize {
    let mut base = SH_DMAC_BASE0 as usize;
    #[cfg(SH_DMAC_BASE1)]
    if chan as usize >= SH_DMAC_NR_MD_CH {
        base = SH_DMAC_BASE1 as usize;
    }
    base
}

unsafe fn dma_base_addr(chan: u32) -> usize {
    let mut base = dma_find_base(chan);
    let chan = (chan as usize % SH_DMAC_NR_MD_CH) * SH_DMAC_CH_SZ;
    if chan >= DMAOR as usize {
        base += SH_DMAC_CH_SZ;
    }
    base + chan
}

#[cfg(CONFIG_SH_DMA_IRQ_MULTI)]
#[inline]
unsafe fn get_dmte_irq(chan: u32) -> u32 {
    if chan >= 6 { DMTE6_IRQ } else { DMTE0_IRQ }
}

#[cfg(not(CONFIG_SH_DMA_IRQ_MULTI))]
static mut dmte_irq_map: [u32; 12] = [
    DMTE0_IRQ, DMTE0_IRQ + 1, DMTE0_IRQ + 2, DMTE0_IRQ + 3,
    DMTE4_IRQ, DMTE4_IRQ + 1,
    DMTE6_IRQ, DMTE6_IRQ + 1,
    DMTE8_IRQ, DMTE9_IRQ, DMTE10_IRQ, DMTE11_IRQ,
];

#[cfg(not(CONFIG_SH_DMA_IRQ_MULTI))]
#[inline]
unsafe fn get_dmte_irq(chan: u32) -> u32 { dmte_irq_map[chan as usize] }

static mut ts_shift: [u32; 8] = TS_SHIFT;

#[inline]
unsafe fn calc_xmit_shift(chan: *mut dma_channel) -> u32 {
    let chcr = __raw_readl(dma_base_addr((*chan).chan) + CHCR as usize);
    let cnt = ((chcr & CHCR_TS_LOW_MASK) >> CHCR_TS_LOW_SHIFT)
        | ((chcr & CHCR_TS_HIGH_MASK) >> CHCR_TS_HIGH_SHIFT);
    ts_shift[cnt as usize]
}

unsafe extern "C" fn dma_tei(_irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let chan = dev_id as *mut dma_channel;
    let mut chcr = __raw_readl(dma_base_addr((*chan).chan) + CHCR as usize);
    if chcr & CHCR_TE == 0 { return IRQ_NONE; }
    chcr &= !(CHCR_IE | CHCR_DE);
    __raw_writel(chcr, dma_base_addr((*chan).chan) + CHCR as usize);
    wake_up(&mut (*chan).wait_queue);
    IRQ_HANDLED
}

unsafe fn sh_dmac_request_dma(chan: *mut dma_channel) -> i32 {
    if unlikely((*chan).flags & DMA_TEI_CAPABLE == 0) { return 0; }
    request_irq(get_dmte_irq((*chan).chan), dma_tei, IRQF_SHARED, (*chan).dev_id, chan)
}

unsafe fn sh_dmac_free_dma(chan: *mut dma_channel) { free_irq(get_dmte_irq((*chan).chan), chan); }

unsafe fn sh_dmac_configure_channel(chan: *mut dma_channel, mut chcr: u32) -> i32 {
    if chcr == 0 { chcr = RS_DUAL | CHCR_IE; }
    if chcr & CHCR_IE != 0 {
        chcr &= !CHCR_IE;
        (*chan).flags |= DMA_TEI_CAPABLE;
    } else { (*chan).flags &= !DMA_TEI_CAPABLE; }
    __raw_writel(chcr, dma_base_addr((*chan).chan) + CHCR as usize);
    (*chan).flags |= DMA_CONFIGURED;
    0
}

unsafe fn sh_dmac_enable_dma(chan: *mut dma_channel) {
    let mut chcr = __raw_readl(dma_base_addr((*chan).chan) + CHCR as usize) | CHCR_DE;
    if (*chan).flags & DMA_TEI_CAPABLE != 0 { chcr |= CHCR_IE; }
    __raw_writel(chcr, dma_base_addr((*chan).chan) + CHCR as usize);
    if (*chan).flags & DMA_TEI_CAPABLE != 0 { enable_irq(get_dmte_irq((*chan).chan)); }
}

unsafe fn sh_dmac_disable_dma(chan: *mut dma_channel) {
    if (*chan).flags & DMA_TEI_CAPABLE != 0 { disable_irq(get_dmte_irq((*chan).chan)); }
    let chcr = __raw_readl(dma_base_addr((*chan).chan) + CHCR as usize) & !(CHCR_DE | CHCR_TE | CHCR_IE);
    __raw_writel(chcr, dma_base_addr((*chan).chan) + CHCR as usize);
}

unsafe fn sh_dmac_xfer_dma(chan: *mut dma_channel) -> i32 {
    if unlikely((*chan).flags & DMA_CONFIGURED == 0) { sh_dmac_configure_channel(chan, 0); }
    sh_dmac_disable_dma(chan);
    if (*chan).sar != 0 || (mach_is_dreamcast() && (*chan).chan == PVR2_CASCADE_CHAN) {
        __raw_writel((*chan).sar, dma_base_addr((*chan).chan) + SAR as usize);
    }
    if (*chan).dar != 0 || (mach_is_dreamcast() && (*chan).chan == PVR2_CASCADE_CHAN) {
        __raw_writel((*chan).dar, dma_base_addr((*chan).chan) + DAR as usize);
    }
    __raw_writel((*chan).count >> calc_xmit_shift(chan), dma_base_addr((*chan).chan) + TCR as usize);
    sh_dmac_enable_dma(chan);
    0
}

unsafe fn sh_dmac_get_dma_residue(chan: *mut dma_channel) -> u32 {
    if __raw_readl(dma_base_addr((*chan).chan) + CHCR as usize) & CHCR_DE == 0 { return 0; }
    __raw_readl(dma_base_addr((*chan).chan) + TCR as usize) << calc_xmit_shift(chan)
}

#[cfg(any(CONFIG_CPU_SUBTYPE_SH7723, CONFIG_CPU_SUBTYPE_SH7724, CONFIG_CPU_SUBTYPE_SH7780, CONFIG_CPU_SUBTYPE_SH7785))]
const NR_DMAOR: usize = 2;
#[cfg(not(any(CONFIG_CPU_SUBTYPE_SH7723, CONFIG_CPU_SUBTYPE_SH7724, CONFIG_CPU_SUBTYPE_SH7780, CONFIG_CPU_SUBTYPE_SH7785)))]
const NR_DMAOR: usize = 1;

unsafe fn dmaor_read_reg(n: usize) -> u16 { __raw_readw(dma_find_base((n * SH_DMAC_NR_MD_CH) as u32) + DMAOR as usize) }
unsafe fn dmaor_write_reg(n: usize, data: u16) { __raw_writew(data, dma_find_base((n * SH_DMAC_NR_MD_CH) as u32) + DMAOR as usize); }

#[inline]
unsafe fn dmaor_reset(no: usize) -> i32 {
    let mut dmaor = dmaor_read_reg(no);
    dmaor &= !(DMAOR_NMIF | DMAOR_AE); dmaor_write_reg(no, dmaor);
    dmaor |= DMAOR_INIT; dmaor_write_reg(no, dmaor);
    if dmaor_read_reg(no) & (DMAOR_AE | DMAOR_NMIF) != 0 { printk(KERN_ERR, "dma-sh: Can't initialize DMAOR.\n"); return -EINVAL; }
    0
}

#[cfg(CONFIG_CPU_SH4)]
const NR_DMAE: usize = 1;
#[cfg(not(CONFIG_CPU_SH4))]
const NR_DMAE: usize = 0;

#[cfg(CONFIG_CPU_SH4)]
static dmae_name: [&str; 2] = ["DMAC Address Error0", "DMAC Address Error1"];

#[cfg(CONFIG_CPU_SH4)]
unsafe fn get_dma_error_irq(n: i32) -> u32 { get_dmte_irq((n * 6) as u32) }

#[cfg(CONFIG_CPU_SH4)]
unsafe extern "C" fn dma_err(irq: i32, _dummy: *mut core::ffi::c_void) -> irqreturn_t {
    for i in 0..NR_DMAOR { dmaor_reset(i); }
    disable_irq(irq as u32);
    IRQ_HANDLED
}

#[cfg(CONFIG_CPU_SH4)]
unsafe fn dmae_irq_init() -> i32 {
    for n in 0..NR_DMAE {
        let i = request_irq(get_dma_error_irq(n as i32), dma_err, IRQF_SHARED,
                            dmae_name[n], dmae_name[n].as_ptr() as *mut core::ffi::c_void);
        if unlikely(i < 0) { printk(KERN_ERR, "%s request_irq fail\n", dmae_name[n]); return i; }
    }
    0
}

#[cfg(CONFIG_CPU_SH4)]
unsafe fn dmae_irq_free() { for n in 0..NR_DMAE { free_irq(get_dma_error_irq(n as i32), core::ptr::null_mut()); } }

#[cfg(not(CONFIG_CPU_SH4))]
unsafe fn dmae_irq_init() -> i32 { 0 }
#[cfg(not(CONFIG_CPU_SH4))]
unsafe fn dmae_irq_free() {}

static mut sh_dmac_ops: dma_ops = dma_ops {
    request: Some(sh_dmac_request_dma), free: Some(sh_dmac_free_dma),
    get_residue: Some(sh_dmac_get_dma_residue), xfer: Some(sh_dmac_xfer_dma),
    configure: Some(sh_dmac_configure_channel),
};

static mut sh_dmac_info: dma_info = dma_info {
    name: "sh_dmac", nr_channels: CONFIG_NR_ONCHIP_DMA_CHANNELS,
    ops: &mut sh_dmac_ops, flags: DMAC_CHANNELS_TEI_CAPABLE,
};

unsafe extern "C" fn sh_dmac_init() -> i32 {
    let info = &mut sh_dmac_info;
    let rc = dmae_irq_init(); if unlikely(rc != 0) { return rc; }
    for i in 0..NR_DMAOR { let rc = dmaor_reset(i); if unlikely(rc != 0) { return rc; } }
    register_dmac(info)
}

unsafe extern "C" fn sh_dmac_exit() { dmae_irq_free(); unregister_dmac(&mut sh_dmac_info); }

// subsys_initcall(sh_dmac_init); module_exit(sh_dmac_exit);
// MODULE_AUTHOR("Takashi YOSHII, Paul Mundt, Andriy Skulysh");
// MODULE_DESCRIPTION("SuperH On-Chip DMAC Support");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
