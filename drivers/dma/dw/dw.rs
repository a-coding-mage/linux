// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2007-2008 Atmel Corporation
// Copyright (C) 2010-2011 ST Microelectronics
// Copyright (C) 2013,2018 Intel Corporation

// Dependencies are supplied by the surrounding DMA implementation.

unsafe fn dw_dma_initialize_chan(dwc: *mut dw_dma_chan) {
    let dw = to_dw_dma((*dwc).chan.device);
    let mut cfghi: u32 = if is_slave_direction((*dwc).direction) {
        0
    } else {
        DWC_CFGH_FIFO_MODE
    };
    let mut cfglo: u32 = DWC_CFGL_CH_PRIOR((*dwc).priority);
    let hs_polarity: bool = (*dwc).dws.hs_polarity;

    cfghi |= DWC_CFGH_DST_PER((*dwc).dws.dst_id);
    cfghi |= DWC_CFGH_SRC_PER((*dwc).dws.src_id);
    cfghi |= DWC_CFGH_PROTCTL((*dw).pdata.protctl);

    /* Set polarity of handshake interface */
    cfglo |= if hs_polarity {
        DWC_CFGL_HS_DST_POL | DWC_CFGL_HS_SRC_POL
    } else {
        0
    };

    channel_writel(dwc, CFG_LO, cfglo);
    channel_writel(dwc, CFG_HI, cfghi);
}

unsafe fn dw_dma_suspend_chan(dwc: *mut dw_dma_chan, _drain: bool) {
    let cfglo: u32 = channel_readl(dwc, CFG_LO);

    channel_writel(dwc, CFG_LO, cfglo | DWC_CFGL_CH_SUSP);
}

unsafe fn dw_dma_resume_chan(dwc: *mut dw_dma_chan, _drain: bool) {
    let cfglo: u32 = channel_readl(dwc, CFG_LO);

    channel_writel(dwc, CFG_LO, cfglo & !DWC_CFGL_CH_SUSP);
}

unsafe fn dw_dma_bytes2block(
    dwc: *mut dw_dma_chan,
    bytes: usize,
    width: u32,
    len: *mut usize,
) -> u32 {
    let block: u32;

    if (bytes >> width) > (*dwc).block_size {
        block = (*dwc).block_size;
        *len = (*dwc).block_size as usize << width;
    } else {
        block = (bytes >> width) as u32;
        *len = bytes;
    }

    block
}

unsafe fn dw_dma_block2bytes(_dwc: *mut dw_dma_chan, block: u32, width: u32) -> usize {
    (DWC_CTLH_BLOCK_TS(block) as usize) << width
}

#[inline]
fn dw_dma_encode_maxburst(maxburst: u32) -> u8 {
    /*
     * Fix burst size according to dw_dmac. We need to convert them as:
     * 1 -> 0, 4 -> 1, 8 -> 2, 16 -> 3.
     */
    if maxburst > 1 {
        fls(maxburst) as u8 - 2
    } else {
        0
    }
}

unsafe fn dw_dma_prepare_ctllo(dwc: *mut dw_dma_chan) -> u32 {
    let sconfig: *mut dma_slave_config = &mut (*dwc).dma_sconfig;
    let mut smsize: u8 = 0;
    let mut dmsize: u8 = 0;
    let sms: u8;
    let dms: u8;

    if (*dwc).direction == DMA_MEM_TO_DEV {
        sms = (*dwc).dws.m_master;
        dms = (*dwc).dws.p_master;
        dmsize = dw_dma_encode_maxburst((*sconfig).dst_maxburst);
    } else if (*dwc).direction == DMA_DEV_TO_MEM {
        sms = (*dwc).dws.p_master;
        dms = (*dwc).dws.m_master;
        smsize = dw_dma_encode_maxburst((*sconfig).src_maxburst);
    } else {
        /* DMA_MEM_TO_MEM */
        sms = (*dwc).dws.m_master;
        dms = (*dwc).dws.m_master;
    }

    DWC_CTLL_LLP_D_EN
        | DWC_CTLL_LLP_S_EN
        | DWC_CTLL_DST_MSIZE(dmsize)
        | DWC_CTLL_SRC_MSIZE(smsize)
        | DWC_CTLL_DMS(dms)
        | DWC_CTLL_SMS(sms)
}

unsafe fn dw_dma_set_device_name(dw: *mut dw_dma, id: i32) {
    snprintf(
        (*dw).name.as_mut_ptr(),
        core::mem::size_of_val(&(*dw).name),
        b"dw:dmac%d\0".as_ptr() as *const i8,
        id,
    );
}

unsafe fn dw_dma_disable(dw: *mut dw_dma) {
    do_dw_dma_off(dw);
}

unsafe fn dw_dma_enable(dw: *mut dw_dma) {
    do_dw_dma_on(dw);
}

unsafe fn dw_dma_probe(chip: *mut dw_dma_chip) -> i32 {
    let mut dw: *mut dw_dma;

    dw = devm_kzalloc((*chip).dev, core::mem::size_of::<dw_dma>(), GFP_KERNEL);
    if dw.is_null() {
        return -ENOMEM;
    }

    /* Channel operations */
    (*dw).initialize_chan = Some(dw_dma_initialize_chan);
    (*dw).suspend_chan = Some(dw_dma_suspend_chan);
    (*dw).resume_chan = Some(dw_dma_resume_chan);
    (*dw).prepare_ctllo = Some(dw_dma_prepare_ctllo);
    (*dw).bytes2block = Some(dw_dma_bytes2block);
    (*dw).block2bytes = Some(dw_dma_block2bytes);

    /* Device operations */
    (*dw).set_device_name = Some(dw_dma_set_device_name);
    (*dw).disable = Some(dw_dma_disable);
    (*dw).enable = Some(dw_dma_enable);

    (*chip).dw = dw;
    do_dma_probe(chip)
}

unsafe fn dw_dma_remove(chip: *mut dw_dma_chip) -> i32 {
    do_dma_remove(chip)
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
