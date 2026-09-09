// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2013,2018,2020-2021 Intel Corporation

// External Linux/DMA and internal driver definitions are supplied by dependencies.

const DMA_CTL_CH: usize = |x: usize| 0x1000 + x * 4;
const DMA_SRC_ADDR_FILLIN: usize = |x: usize| 0x1100 + x * 4;
const DMA_DST_ADDR_FILLIN: usize = |x: usize| 0x1200 + x * 4;
const DMA_XBAR_SEL: usize = |x: usize| 0x1300 + x * 4;
const DMA_REGACCESS_CHID_CFG: usize = 0x1400;

const CTL_CH_TRANSFER_MODE_MASK: u32 = 0x3;
const CTL_CH_TRANSFER_MODE_S2S: u32 = 0;
const CTL_CH_TRANSFER_MODE_S2D: u32 = 1;
const CTL_CH_TRANSFER_MODE_D2S: u32 = 2;
const CTL_CH_TRANSFER_MODE_D2D: u32 = 3;
const CTL_CH_RD_RS_MASK: u32 = 0x18;
const CTL_CH_WR_RS_MASK: u32 = 0x60;
const CTL_CH_RD_NON_SNOOP_BIT: u32 = 1 << 8;
const CTL_CH_WR_NON_SNOOP_BIT: u32 = 1 << 9;

const XBAR_SEL_DEVID_MASK: u32 = 0xffff;
const XBAR_SEL_RX_TX_BIT: u32 = 1 << 16;
const XBAR_SEL_RX_TX_SHIFT: u32 = 16;

const REGACCESS_CHID_MASK: u32 = 0x7;

unsafe fn idma32_get_slave_devfn(dwc: *mut dw_dma_chan) -> u32 {
    let slave = (*dwc).chan.slave;
    if slave.is_null() || !dev_is_pci(slave) {
        return 0;
    }
    to_pci_dev(slave).as_ref().unwrap().devfn as u32
}

unsafe fn idma32_initialize_chan_xbar(dwc: *mut dw_dma_chan) {
    let dw = to_dw_dma((*dwc).chan.device);
    let misc = __dw_regs(dw);
    let mut cfghi: u32 = 0;
    let mut cfglo: u32 = 0;
    let (dst_id, src_id): (u8, u8);
    let mut value: u32;

    /* DMA Channel ID Configuration register must be programmed first */
    value = readl(misc.add(DMA_REGACCESS_CHID_CFG));
    value &= !REGACCESS_CHID_MASK;
    value |= (*dwc).chan.chan_id as u32;
    writel(value, misc.add(DMA_REGACCESS_CHID_CFG));

    /* Configure channel attributes */
    value = readl(misc.add(DMA_CTL_CH((*dwc).chan.chan_id as usize)));
    value &= !(CTL_CH_RD_NON_SNOOP_BIT | CTL_CH_WR_NON_SNOOP_BIT);
    value &= !(CTL_CH_RD_RS_MASK | CTL_CH_WR_RS_MASK);
    value &= !CTL_CH_TRANSFER_MODE_MASK;

    match (*dwc).direction {
        DMA_MEM_TO_DEV => {
            value |= CTL_CH_TRANSFER_MODE_D2S;
            value |= CTL_CH_WR_NON_SNOOP_BIT;
        }
        DMA_DEV_TO_MEM => {
            value |= CTL_CH_TRANSFER_MODE_S2D;
            value |= CTL_CH_RD_NON_SNOOP_BIT;
        }
        _ => {
            /* Memory-to-Memory and Device-to-Device are ignored for now. */
            return;
        }
    }

    writel(value, misc.add(DMA_CTL_CH((*dwc).chan.chan_id as usize)));

    /* Configure crossbar selection */
    value = readl(misc.add(DMA_XBAR_SEL((*dwc).chan.chan_id as usize)));
    /* DEVFN selection */
    value &= !XBAR_SEL_DEVID_MASK;
    value |= idma32_get_slave_devfn(dwc);

    match (*dwc).direction {
        DMA_MEM_TO_DEV => value |= XBAR_SEL_RX_TX_BIT,
        DMA_DEV_TO_MEM => value &= !XBAR_SEL_RX_TX_BIT,
        _ => return,
    }
    writel(value, misc.add(DMA_XBAR_SEL((*dwc).chan.chan_id as usize)));

    /* Configure DMA channel low and high registers */
    match (*dwc).direction {
        DMA_MEM_TO_DEV => { dst_id = (*dwc).chan.chan_id; src_id = (*dwc).dws.src_id; }
        DMA_DEV_TO_MEM => { dst_id = (*dwc).dws.dst_id; src_id = (*dwc).chan.chan_id; }
        _ => return,
    }

    /* Set default burst alignment */
    cfglo |= IDMA32C_CFGL_DST_BURST_ALIGN | IDMA32C_CFGL_SRC_BURST_ALIGN;
    /* Low 4 bits of the request lines */
    cfghi |= IDMA32C_CFGH_DST_PER((dst_id & 0xf) as u32);
    cfghi |= IDMA32C_CFGH_SRC_PER((src_id & 0xf) as u32);
    /* Request line extension (2 bits) */
    cfghi |= IDMA32C_CFGH_DST_PER_EXT(((dst_id >> 4) & 0x3) as u32);
    cfghi |= IDMA32C_CFGH_SRC_PER_EXT(((src_id >> 4) & 0x3) as u32);
    channel_writel(dwc, CFG_LO, cfglo);
    channel_writel(dwc, CFG_HI, cfghi);
}

unsafe fn idma32_initialize_chan_generic(dwc: *mut dw_dma_chan) {
    let mut cfghi: u32 = 0;
    let mut cfglo: u32 = 0;
    /* Set default burst alignment */
    cfglo |= IDMA32C_CFGL_DST_BURST_ALIGN | IDMA32C_CFGL_SRC_BURST_ALIGN;
    /* Low 4 bits of the request lines */
    cfghi |= IDMA32C_CFGH_DST_PER(((*dwc).dws.dst_id & 0xf) as u32);
    cfghi |= IDMA32C_CFGH_SRC_PER(((*dwc).dws.src_id & 0xf) as u32);
    /* Request line extension (2 bits) */
    cfghi |= IDMA32C_CFGH_DST_PER_EXT(((((*dwc).dws.dst_id) >> 4) & 0x3) as u32);
    cfghi |= IDMA32C_CFGH_SRC_PER_EXT(((((*dwc).dws.src_id) >> 4) & 0x3) as u32);
    channel_writel(dwc, CFG_LO, cfglo);
    channel_writel(dwc, CFG_HI, cfghi);
}

unsafe fn idma32_suspend_chan(dwc: *mut dw_dma_chan, drain: bool) {
    let mut cfglo = channel_readl(dwc, CFG_LO);
    if drain { cfglo |= IDMA32C_CFGL_CH_DRAIN; }
    channel_writel(dwc, CFG_LO, cfglo | DWC_CFGL_CH_SUSP);
}

unsafe fn idma32_resume_chan(dwc: *mut dw_dma_chan, drain: bool) {
    let mut cfglo = channel_readl(dwc, CFG_LO);
    if drain { cfglo &= !IDMA32C_CFGL_CH_DRAIN; }
    channel_writel(dwc, CFG_LO, cfglo & !DWC_CFGL_CH_SUSP);
}

unsafe fn idma32_bytes2block(dwc: *mut dw_dma_chan, bytes: usize, _width: u32, len: *mut usize) -> u32 {
    if bytes > (*dwc).block_size { *len = (*dwc).block_size; (*dwc).block_size as u32 }
    else { *len = bytes; bytes as u32 }
}

unsafe fn idma32_block2bytes(_dwc: *mut dw_dma_chan, block: u32, _width: u32) -> usize {
    IDMA32C_CTLH_BLOCK_TS(block) as usize
}

#[inline]
fn idma32_encode_maxburst(maxburst: u32) -> u8 {
    if maxburst > 1 { (31 - maxburst.leading_zeros()) as u8 } else { 0 }
}

unsafe fn idma32_prepare_ctllo(dwc: *mut dw_dma_chan) -> u32 {
    let sconfig = &(*dwc).dma_sconfig;
    let mut smsize = 0u8;
    let mut dmsize = 0u8;
    if (*dwc).direction == DMA_MEM_TO_DEV { dmsize = idma32_encode_maxburst(sconfig.dst_maxburst); }
    else if (*dwc).direction == DMA_DEV_TO_MEM { smsize = idma32_encode_maxburst(sconfig.src_maxburst); }
    DWC_CTLL_LLP_D_EN | DWC_CTLL_LLP_S_EN | DWC_CTLL_DST_MSIZE(dmsize) | DWC_CTLL_SRC_MSIZE(smsize)
}

unsafe fn idma32_set_device_name(dw: *mut dw_dma, id: i32) {
    snprintf((*dw).name.as_mut_ptr(), core::mem::size_of_val(&(*dw).name), "idma32:dmac%d\0", id);
}

/* Program FIFO size of channels. */
unsafe fn idma32_fifo_partition(dw: *mut dw_dma) {
    let value = IDMA32C_FP_PSIZE_CH0(64) | IDMA32C_FP_PSIZE_CH1(64) | IDMA32C_FP_UPDATE;
    let mut fifo_partition: u64 = 0;
    /* Fill FIFO_PARTITION low bits (Channels 0..1, 4..5) */
    fifo_partition |= value << 0;
    /* Fill FIFO_PARTITION high bits (Channels 2..3, 6..7) */
    fifo_partition |= value << 32;
    /* Program FIFO Partition registers - 64 bytes per channel */
    idma32_writeq(dw, FIFO_PARTITION1, fifo_partition);
    idma32_writeq(dw, FIFO_PARTITION0, fifo_partition);
}

unsafe fn idma32_disable(dw: *mut dw_dma) { do_dw_dma_off(dw); idma32_fifo_partition(dw); }
unsafe fn idma32_enable(dw: *mut dw_dma) { idma32_fifo_partition(dw); do_dw_dma_on(dw); }

pub unsafe fn idma32_dma_probe(chip: *mut dw_dma_chip) -> i32 {
    let dw = devm_kzalloc((*chip).dev, core::mem::size_of::<dw_dma>(), GFP_KERNEL);
    if dw.is_null() { return -12; }
    if (*(*chip).pdata).quirks & DW_DMA_QUIRK_XBAR_PRESENT != 0 { (*dw).initialize_chan = Some(idma32_initialize_chan_xbar); }
    else { (*dw).initialize_chan = Some(idma32_initialize_chan_generic); }
    (*dw).suspend_chan = Some(idma32_suspend_chan);
    (*dw).resume_chan = Some(idma32_resume_chan);
    (*dw).prepare_ctllo = Some(idma32_prepare_ctllo);
    (*dw).bytes2block = Some(idma32_bytes2block);
    (*dw).block2bytes = Some(idma32_block2bytes);
    (*dw).set_device_name = Some(idma32_set_device_name);
    (*dw).disable = Some(idma32_disable);
    (*dw).enable = Some(idma32_enable);
    (*chip).dw = dw;
    do_dma_probe(chip)
}

pub unsafe fn idma32_dma_remove(chip: *mut dw_dma_chip) -> i32 { do_dma_remove(chip) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
