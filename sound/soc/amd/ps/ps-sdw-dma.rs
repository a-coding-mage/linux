// SPDX-License-Identifier: GPL-2.0-only
/*
 * AMD ALSA SoC common SoundWire DMA Driver for ACP6.3, ACP7.0 and ACP7.1
 * platforms.
 *
 * Copyright 2023, 2025 Advanced Micro Devices, Inc.
 */

const DRV_NAME: &[u8] = b"amd_ps_sdw_dma\0";

static mut acp63_sdw0_dma_reg: [sdw_dma_ring_buf_reg; ACP63_SDW0_DMA_MAX_STREAMS] = [
    sdw_dma_ring_buf_reg { reg_dma_size: ACP_AUDIO0_TX_DMA_SIZE, reg_fifo_addr: ACP_AUDIO0_TX_FIFOADDR, reg_fifo_size: ACP_AUDIO0_TX_FIFOSIZE, reg_ring_buf_size: ACP_AUDIO0_TX_RINGBUFSIZE, reg_ring_buf_addr: ACP_AUDIO0_TX_RINGBUFADDR, water_mark_size_reg: ACP_AUDIO0_TX_INTR_WATERMARK_SIZE, pos_low_reg: ACP_AUDIO0_TX_LINEARPOSITIONCNTR_LOW, pos_high_reg: ACP_AUDIO0_TX_LINEARPOSITIONCNTR_HIGH },
    sdw_dma_ring_buf_reg { reg_dma_size: ACP_AUDIO1_TX_DMA_SIZE, reg_fifo_addr: ACP_AUDIO1_TX_FIFOADDR, reg_fifo_size: ACP_AUDIO1_TX_FIFOSIZE, reg_ring_buf_size: ACP_AUDIO1_TX_RINGBUFSIZE, reg_ring_buf_addr: ACP_AUDIO1_TX_RINGBUFADDR, water_mark_size_reg: ACP_AUDIO1_TX_INTR_WATERMARK_SIZE, pos_low_reg: ACP_AUDIO1_TX_LINEARPOSITIONCNTR_LOW, pos_high_reg: ACP_AUDIO1_TX_LINEARPOSITIONCNTR_HIGH },
    sdw_dma_ring_buf_reg { reg_dma_size: ACP_AUDIO2_TX_DMA_SIZE, reg_fifo_addr: ACP_AUDIO2_TX_FIFOADDR, reg_fifo_size: ACP_AUDIO2_TX_FIFOSIZE, reg_ring_buf_size: ACP_AUDIO2_TX_RINGBUFSIZE, reg_ring_buf_addr: ACP_AUDIO2_TX_RINGBUFADDR, water_mark_size_reg: ACP_AUDIO2_TX_INTR_WATERMARK_SIZE, pos_low_reg: ACP_AUDIO2_TX_LINEARPOSITIONCNTR_LOW, pos_high_reg: ACP_AUDIO2_TX_LINEARPOSITIONCNTR_HIGH },
    sdw_dma_ring_buf_reg { reg_dma_size: ACP_AUDIO0_RX_DMA_SIZE, reg_fifo_addr: ACP_AUDIO0_RX_FIFOADDR, reg_fifo_size: ACP_AUDIO0_RX_FIFOSIZE, reg_ring_buf_size: ACP_AUDIO0_RX_RINGBUFSIZE, reg_ring_buf_addr: ACP_AUDIO0_RX_RINGBUFADDR, water_mark_size_reg: ACP_AUDIO0_RX_INTR_WATERMARK_SIZE, pos_low_reg: ACP_AUDIO0_RX_LINEARPOSITIONCNTR_LOW, pos_high_reg: ACP_AUDIO0_RX_LINEARPOSITIONCNTR_HIGH },
    sdw_dma_ring_buf_reg { reg_dma_size: ACP_AUDIO1_RX_DMA_SIZE, reg_fifo_addr: ACP_AUDIO1_RX_FIFOADDR, reg_fifo_size: ACP_AUDIO1_RX_FIFOSIZE, reg_ring_buf_size: ACP_AUDIO1_RX_RINGBUFSIZE, reg_ring_buf_addr: ACP_AUDIO1_RX_RINGBUFADDR, water_mark_size_reg: ACP_AUDIO1_RX_INTR_WATERMARK_SIZE, pos_low_reg: ACP_AUDIO1_RX_LINEARPOSITIONCNTR_LOW, pos_high_reg: ACP_AUDIO1_RX_LINEARPOSITIONCNTR_HIGH },
    sdw_dma_ring_buf_reg { reg_dma_size: ACP_AUDIO2_RX_DMA_SIZE, reg_fifo_addr: ACP_AUDIO2_RX_FIFOADDR, reg_fifo_size: ACP_AUDIO2_RX_FIFOSIZE, reg_ring_buf_size: ACP_AUDIO2_RX_RINGBUFSIZE, reg_ring_buf_addr: ACP_AUDIO2_RX_RINGBUFADDR, water_mark_size_reg: ACP_AUDIO2_RX_INTR_WATERMARK_SIZE, pos_low_reg: ACP_AUDIO2_RX_LINEARPOSITIONCNTR_LOW, pos_high_reg: ACP_AUDIO2_RX_LINEARPOSITIONCNTR_HIGH },
];

/*
 * SDW1 instance supports one TX stream and one RX stream.
 * For TX/RX streams DMA registers programming for SDW1 instance, it uses ACP_P1_AUDIO1 register
 * set as per hardware register documentation
 */
static mut acp63_sdw1_dma_reg: [sdw_dma_ring_buf_reg; ACP63_SDW1_DMA_MAX_STREAMS] = [
    sdw_dma_ring_buf_reg { reg_dma_size: ACP_P1_AUDIO1_TX_DMA_SIZE, reg_fifo_addr: ACP_P1_AUDIO1_TX_FIFOADDR, reg_fifo_size: ACP_P1_AUDIO1_TX_FIFOSIZE, reg_ring_buf_size: ACP_P1_AUDIO1_TX_RINGBUFSIZE, reg_ring_buf_addr: ACP_P1_AUDIO1_TX_RINGBUFADDR, water_mark_size_reg: ACP_P1_AUDIO1_TX_INTR_WATERMARK_SIZE, pos_low_reg: ACP_P1_AUDIO1_TX_LINEARPOSITIONCNTR_LOW, pos_high_reg: ACP_P1_AUDIO1_TX_LINEARPOSITIONCNTR_HIGH },
    sdw_dma_ring_buf_reg { reg_dma_size: ACP_P1_AUDIO1_RX_DMA_SIZE, reg_fifo_addr: ACP_P1_AUDIO1_RX_FIFOADDR, reg_fifo_size: ACP_P1_AUDIO1_RX_FIFOSIZE, reg_ring_buf_size: ACP_P1_AUDIO1_RX_RINGBUFSIZE, reg_ring_buf_addr: ACP_P1_AUDIO1_RX_RINGBUFADDR, water_mark_size_reg: ACP_P1_AUDIO1_RX_INTR_WATERMARK_SIZE, pos_low_reg: ACP_P1_AUDIO1_RX_LINEARPOSITIONCNTR_LOW, pos_high_reg: ACP_P1_AUDIO1_RX_LINEARPOSITIONCNTR_HIGH },
];

static mut acp63_sdw0_dma_enable_reg: [u32; ACP63_SDW0_DMA_MAX_STREAMS] = [
    ACP_SW0_AUDIO0_TX_EN,
    ACP_SW0_AUDIO1_TX_EN,
    ACP_SW0_AUDIO2_TX_EN,
    ACP_SW0_AUDIO0_RX_EN,
    ACP_SW0_AUDIO1_RX_EN,
    ACP_SW0_AUDIO2_RX_EN,
];

/*
 * SDW1 instance supports one TX stream and one RX stream.
 * For TX/RX streams DMA enable register programming for SDW1 instance,
 * it uses ACP_SW1_AUDIO1_TX_EN and ACP_SW1_AUDIO1_RX_EN registers
 * as per hardware register documentation.
 */
static mut acp63_sdw1_dma_enable_reg: [u32; ACP63_SDW1_DMA_MAX_STREAMS] = [
    ACP_SW1_AUDIO1_TX_EN,
    ACP_SW1_AUDIO1_RX_EN,
];

static mut acp70_sdw0_dma_reg: [sdw_dma_ring_buf_reg; ACP70_SDW0_DMA_MAX_STREAMS] = [
    sdw_dma_ring_buf_reg { reg_dma_size: ACP_AUDIO0_TX_DMA_SIZE, reg_fifo_addr: ACP_AUDIO0_TX_FIFOADDR, reg_fifo_size: ACP_AUDIO0_TX_FIFOSIZE, reg_ring_buf_size: ACP_AUDIO0_TX_RINGBUFSIZE, reg_ring_buf_addr: ACP_AUDIO0_TX_RINGBUFADDR, water_mark_size_reg: ACP_AUDIO0_TX_INTR_WATERMARK_SIZE, pos_low_reg: ACP_AUDIO0_TX_LINEARPOSITIONCNTR_LOW, pos_high_reg: ACP_AUDIO0_TX_LINEARPOSITIONCNTR_HIGH },
    sdw_dma_ring_buf_reg { reg_dma_size: ACP_AUDIO1_TX_DMA_SIZE, reg_fifo_addr: ACP_AUDIO1_TX_FIFOADDR, reg_fifo_size: ACP_AUDIO1_TX_FIFOSIZE, reg_ring_buf_size: ACP_AUDIO1_TX_RINGBUFSIZE, reg_ring_buf_addr: ACP_AUDIO1_TX_RINGBUFADDR, water_mark_size_reg: ACP_AUDIO1_TX_INTR_WATERMARK_SIZE, pos_low_reg: ACP_AUDIO1_TX_LINEARPOSITIONCNTR_LOW, pos_high_reg: ACP_AUDIO1_TX_LINEARPOSITIONCNTR_HIGH },
    sdw_dma_ring_buf_reg { reg_dma_size: ACP_AUDIO2_TX_DMA_SIZE, reg_fifo_addr: ACP_AUDIO2_TX_FIFOADDR, reg_fifo_size: ACP_AUDIO2_TX_FIFOSIZE, reg_ring_buf_size: ACP_AUDIO2_TX_RINGBUFSIZE, reg_ring_buf_addr: ACP_AUDIO2_TX_RINGBUFADDR, water_mark_size_reg: ACP_AUDIO2_TX_INTR_WATERMARK_SIZE, pos_low_reg: ACP_AUDIO2_TX_LINEARPOSITIONCNTR_LOW, pos_high_reg: ACP_AUDIO2_TX_LINEARPOSITIONCNTR_HIGH },
    sdw_dma_ring_buf_reg { reg_dma_size: ACP_AUDIO0_RX_DMA_SIZE, reg_fifo_addr: ACP_AUDIO0_RX_FIFOADDR, reg_fifo_size: ACP_AUDIO0_RX_FIFOSIZE, reg_ring_buf_size: ACP_AUDIO0_RX_RINGBUFSIZE, reg_ring_buf_addr: ACP_AUDIO0_RX_RINGBUFADDR, water_mark_size_reg: ACP_AUDIO0_RX_INTR_WATERMARK_SIZE, pos_low_reg: ACP_AUDIO0_RX_LINEARPOSITIONCNTR_LOW, pos_high_reg: ACP_AUDIO0_RX_LINEARPOSITIONCNTR_HIGH },
    sdw_dma_ring_buf_reg { reg_dma_size: ACP_AUDIO1_RX_DMA_SIZE, reg_fifo_addr: ACP_AUDIO1_RX_FIFOADDR, reg_fifo_size: ACP_AUDIO1_RX_FIFOSIZE, reg_ring_buf_size: ACP_AUDIO1_RX_RINGBUFSIZE, reg_ring_buf_addr: ACP_AUDIO1_RX_RINGBUFADDR, water_mark_size_reg: ACP_AUDIO1_RX_INTR_WATERMARK_SIZE, pos_low_reg: ACP_AUDIO1_RX_LINEARPOSITIONCNTR_LOW, pos_high_reg: ACP_AUDIO1_RX_LINEARPOSITIONCNTR_HIGH },
    sdw_dma_ring_buf_reg { reg_dma_size: ACP_AUDIO2_RX_DMA_SIZE, reg_fifo_addr: ACP_AUDIO2_RX_FIFOADDR, reg_fifo_size: ACP_AUDIO2_RX_FIFOSIZE, reg_ring_buf_size: ACP_AUDIO2_RX_RINGBUFSIZE, reg_ring_buf_addr: ACP_AUDIO2_RX_RINGBUFADDR, water_mark_size_reg: ACP_AUDIO2_RX_INTR_WATERMARK_SIZE, pos_low_reg: ACP_AUDIO2_RX_LINEARPOSITIONCNTR_LOW, pos_high_reg: ACP_AUDIO2_RX_LINEARPOSITIONCNTR_HIGH },
];

static mut acp70_sdw1_dma_reg: [sdw_dma_ring_buf_reg; ACP70_SDW1_DMA_MAX_STREAMS] = [
    sdw_dma_ring_buf_reg { reg_dma_size: ACP_P1_AUDIO0_TX_DMA_SIZE, reg_fifo_addr: ACP_P1_AUDIO0_TX_FIFOADDR, reg_fifo_size: ACP_P1_AUDIO0_TX_FIFOSIZE, reg_ring_buf_size: ACP_P1_AUDIO0_TX_RINGBUFSIZE, reg_ring_buf_addr: ACP_P1_AUDIO0_TX_RINGBUFADDR, water_mark_size_reg: ACP_P1_AUDIO0_TX_INTR_WATERMARK_SIZE, pos_low_reg: ACP_P1_AUDIO0_TX_LINEARPOSITIONCNTR_LOW, pos_high_reg: ACP_P1_AUDIO0_TX_LINEARPOSITIONCNTR_HIGH },
    sdw_dma_ring_buf_reg { reg_dma_size: ACP_P1_AUDIO1_TX_DMA_SIZE, reg_fifo_addr: ACP_P1_AUDIO1_TX_FIFOADDR, reg_fifo_size: ACP_P1_AUDIO1_TX_FIFOSIZE, reg_ring_buf_size: ACP_P1_AUDIO1_TX_RINGBUFSIZE, reg_ring_buf_addr: ACP_P1_AUDIO1_TX_RINGBUFADDR, water_mark_size_reg: ACP_P1_AUDIO1_TX_INTR_WATERMARK_SIZE, pos_low_reg: ACP_P1_AUDIO1_TX_LINEARPOSITIONCNTR_LOW, pos_high_reg: ACP_P1_AUDIO1_TX_LINEARPOSITIONCNTR_HIGH },
    sdw_dma_ring_buf_reg { reg_dma_size: ACP_P1_AUDIO2_TX_DMA_SIZE, reg_fifo_addr: ACP_P1_AUDIO2_TX_FIFOADDR, reg_fifo_size: ACP_P1_AUDIO2_TX_FIFOSIZE, reg_ring_buf_size: ACP_P1_AUDIO2_TX_RINGBUFSIZE, reg_ring_buf_addr: ACP_P1_AUDIO2_TX_RINGBUFADDR, water_mark_size_reg: ACP_P1_AUDIO2_TX_INTR_WATERMARK_SIZE, pos_low_reg: ACP_P1_AUDIO2_TX_LINEARPOSITIONCNTR_LOW, pos_high_reg: ACP_P1_AUDIO2_TX_LINEARPOSITIONCNTR_HIGH },
    sdw_dma_ring_buf_reg { reg_dma_size: ACP_P1_AUDIO0_RX_DMA_SIZE, reg_fifo_addr: ACP_P1_AUDIO0_RX_FIFOADDR, reg_fifo_size: ACP_P1_AUDIO0_RX_FIFOSIZE, reg_ring_buf_size: ACP_P1_AUDIO0_RX_RINGBUFSIZE, reg_ring_buf_addr: ACP_P1_AUDIO0_RX_RINGBUFADDR, water_mark_size_reg: ACP_P1_AUDIO0_RX_INTR_WATERMARK_SIZE, pos_low_reg: ACP_P1_AUDIO0_RX_LINEARPOSITIONCNTR_LOW, pos_high_reg: ACP_P1_AUDIO0_RX_LINEARPOSITIONCNTR_HIGH },
    sdw_dma_ring_buf_reg { reg_dma_size: ACP_P1_AUDIO1_RX_DMA_SIZE, reg_fifo_addr: ACP_P1_AUDIO1_RX_FIFOADDR, reg_fifo_size: ACP_P1_AUDIO1_RX_FIFOSIZE, reg_ring_buf_size: ACP_P1_AUDIO1_RX_RINGBUFSIZE, reg_ring_buf_addr: ACP_P1_AUDIO1_RX_RINGBUFADDR, water_mark_size_reg: ACP_P1_AUDIO1_RX_INTR_WATERMARK_SIZE, pos_low_reg: ACP_P1_AUDIO1_RX_LINEARPOSITIONCNTR_LOW, pos_high_reg: ACP_P1_AUDIO1_RX_LINEARPOSITIONCNTR_HIGH },
    sdw_dma_ring_buf_reg { reg_dma_size: ACP_P1_AUDIO2_RX_DMA_SIZE, reg_fifo_addr: ACP_P1_AUDIO2_RX_FIFOADDR, reg_fifo_size: ACP_P1_AUDIO2_RX_FIFOSIZE, reg_ring_buf_size: ACP_P1_AUDIO2_RX_RINGBUFSIZE, reg_ring_buf_addr: ACP_P1_AUDIO2_RX_RINGBUFADDR, water_mark_size_reg: ACP_P1_AUDIO2_RX_INTR_WATERMARK_SIZE, pos_low_reg: ACP_P1_AUDIO2_RX_LINEARPOSITIONCNTR_LOW, pos_high_reg: ACP_P1_AUDIO2_RX_LINEARPOSITIONCNTR_HIGH },
];

static mut acp70_sdw0_dma_enable_reg: [u32; ACP70_SDW0_DMA_MAX_STREAMS] = [
    ACP70_SW0_AUDIO0_TX_EN,
    ACP70_SW0_AUDIO1_TX_EN,
    ACP70_SW0_AUDIO2_TX_EN,
    ACP70_SW0_AUDIO0_RX_EN,
    ACP70_SW0_AUDIO1_RX_EN,
    ACP70_SW0_AUDIO2_RX_EN,
];

static mut acp70_sdw1_dma_enable_reg: [u32; ACP70_SDW1_DMA_MAX_STREAMS] = [
    ACP70_SW1_AUDIO0_TX_EN,
    ACP70_SW1_AUDIO1_TX_EN,
    ACP70_SW1_AUDIO2_TX_EN,
    ACP70_SW1_AUDIO0_RX_EN,
    ACP70_SW1_AUDIO1_RX_EN,
    ACP70_SW1_AUDIO2_RX_EN,
];

static acp63_sdw_hardware_playback: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_BLOCK_TRANSFER | SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_PAUSE | SNDRV_PCM_INFO_RESUME,
    formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE,
    channels_min: 2,
    channels_max: 2,
    rates: SNDRV_PCM_RATE_48000,
    rate_min: 48000,
    rate_max: 48000,
    buffer_bytes_max: SDW_PLAYBACK_MAX_NUM_PERIODS * SDW_PLAYBACK_MAX_PERIOD_SIZE,
    period_bytes_min: SDW_PLAYBACK_MIN_PERIOD_SIZE,
    period_bytes_max: SDW_PLAYBACK_MAX_PERIOD_SIZE,
    periods_min: SDW_PLAYBACK_MIN_NUM_PERIODS,
    periods_max: SDW_PLAYBACK_MAX_NUM_PERIODS,
};

static acp63_sdw_hardware_capture: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_BLOCK_TRANSFER | SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_PAUSE | SNDRV_PCM_INFO_RESUME,
    formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE,
    channels_min: 2,
    channels_max: 2,
    rates: SNDRV_PCM_RATE_48000,
    rate_min: 48000,
    rate_max: 48000,
    buffer_bytes_max: SDW_CAPTURE_MAX_NUM_PERIODS * SDW_CAPTURE_MAX_PERIOD_SIZE,
    period_bytes_min: SDW_CAPTURE_MIN_PERIOD_SIZE,
    period_bytes_max: SDW_CAPTURE_MAX_PERIOD_SIZE,
    periods_min: SDW_CAPTURE_MIN_NUM_PERIODS,
    periods_max: SDW_CAPTURE_MAX_NUM_PERIODS,
};

unsafe fn acp63_enable_disable_sdw_dma_interrupts(
    acp_base: *mut core::ffi::c_void,
    irq_mask: u32,
    irq_mask1: u32,
    enable: bool,
) {
    let mut ext_intr_cntl: u32;
    let mut ext_intr_cntl1: u32;

    if enable {
        ext_intr_cntl = readl(acp_base.add(ACP_EXTERNAL_INTR_CNTL as usize));
        ext_intr_cntl |= irq_mask;
        writel(ext_intr_cntl, acp_base.add(ACP_EXTERNAL_INTR_CNTL as usize));
        ext_intr_cntl1 = readl(acp_base.add(ACP_EXTERNAL_INTR_CNTL1 as usize));
        ext_intr_cntl1 |= irq_mask1;
        writel(ext_intr_cntl1, acp_base.add(ACP_EXTERNAL_INTR_CNTL1 as usize));
    } else {
        ext_intr_cntl = readl(acp_base.add(ACP_EXTERNAL_INTR_CNTL as usize));
        ext_intr_cntl &= !irq_mask;
        writel(ext_intr_cntl, acp_base.add(ACP_EXTERNAL_INTR_CNTL as usize));
        ext_intr_cntl1 = readl(acp_base.add(ACP_EXTERNAL_INTR_CNTL1 as usize));
        ext_intr_cntl1 &= !irq_mask1;
        writel(ext_intr_cntl1, acp_base.add(ACP_EXTERNAL_INTR_CNTL1 as usize));
    }
}

unsafe fn acp63_config_dma(
    stream: *mut acp_sdw_dma_stream,
    acp_base: *mut core::ffi::c_void,
    stream_id: u32,
) {
    let mut page_idx: u16;
    let mut low: u32;
    let mut high: u32;
    let mut val: u32;
    let sdw_dma_pte_offset: u32;
    let mut addr: dma_addr_t;

    addr = (*stream).dma_addr;
    sdw_dma_pte_offset = SDW_PTE_OFFSET((*stream).instance);
    val = sdw_dma_pte_offset + stream_id * ACP_SDW_PTE_OFFSET;

    /* Group Enable */
    writel(ACP_SDW_SRAM_PTE_OFFSET | BIT(31), acp_base.add(ACPAXI2AXI_ATU_BASE_ADDR_GRP_2 as usize));
    writel(PAGE_SIZE_4K_ENABLE, acp_base.add(ACPAXI2AXI_ATU_PAGE_SIZE_GRP_2 as usize));
    page_idx = 0;
    while page_idx < (*stream).num_pages {
        /* Load the low address of page int ACP SRAM through SRBM */
        low = lower_32_bits(addr);
        high = upper_32_bits(addr);

        writel(low, acp_base.add((ACP_SCRATCH_REG_0 + val) as usize));
        high |= BIT(31);
        writel(high, acp_base.add((ACP_SCRATCH_REG_0 + val + 4) as usize));
        val += 8;
        addr += PAGE_SIZE as dma_addr_t;
        page_idx += 1;
    }
    writel(0x1, acp_base.add(ACPAXI2AXI_ATU_CTRL as usize));
}

unsafe fn acp63_configure_sdw_ringbuffer(
    acp_base: *mut core::ffi::c_void,
    stream_id: u32,
    size: u32,
    manager_instance: u32,
    acp_rev: u32,
) -> i32 {
    let reg_dma_size: u32;
    let reg_fifo_addr: u32;
    let reg_fifo_size: u32;
    let reg_ring_buf_size: u32;
    let reg_ring_buf_addr: u32;
    let sdw_fifo_addr: u32;
    let sdw_fifo_offset: u32;
    let sdw_ring_buf_addr: u32;
    let sdw_ring_buf_size: u32;
    let sdw_mem_window_offset: u32;

    match acp_rev {
        ACP63_PCI_REV => {
            match manager_instance {
                ACP_SDW0 => {
                    reg_dma_size = acp63_sdw0_dma_reg[stream_id as usize].reg_dma_size;
                    reg_fifo_addr = acp63_sdw0_dma_reg[stream_id as usize].reg_fifo_addr;
                    reg_fifo_size = acp63_sdw0_dma_reg[stream_id as usize].reg_fifo_size;
                    reg_ring_buf_size = acp63_sdw0_dma_reg[stream_id as usize].reg_ring_buf_size;
                    reg_ring_buf_addr = acp63_sdw0_dma_reg[stream_id as usize].reg_ring_buf_addr;
                }
                ACP_SDW1 => {
                    reg_dma_size = acp63_sdw1_dma_reg[stream_id as usize].reg_dma_size;
                    reg_fifo_addr = acp63_sdw1_dma_reg[stream_id as usize].reg_fifo_addr;
                    reg_fifo_size = acp63_sdw1_dma_reg[stream_id as usize].reg_fifo_size;
                    reg_ring_buf_size = acp63_sdw1_dma_reg[stream_id as usize].reg_ring_buf_size;
                    reg_ring_buf_addr = acp63_sdw1_dma_reg[stream_id as usize].reg_ring_buf_addr;
                }
                _ => return -EINVAL,
            }
        }
        ACP70_PCI_REV | ACP71_PCI_REV | ACP72_PCI_REV => {
            match manager_instance {
                ACP_SDW0 => {
                    reg_dma_size = acp70_sdw0_dma_reg[stream_id as usize].reg_dma_size;
                    reg_fifo_addr = acp70_sdw0_dma_reg[stream_id as usize].reg_fifo_addr;
                    reg_fifo_size = acp70_sdw0_dma_reg[stream_id as usize].reg_fifo_size;
                    reg_ring_buf_size = acp70_sdw0_dma_reg[stream_id as usize].reg_ring_buf_size;
                    reg_ring_buf_addr = acp70_sdw0_dma_reg[stream_id as usize].reg_ring_buf_addr;
                }
                ACP_SDW1 => {
                    reg_dma_size = acp70_sdw1_dma_reg[stream_id as usize].reg_dma_size;
                    reg_fifo_addr = acp70_sdw1_dma_reg[stream_id as usize].reg_fifo_addr;
                    reg_fifo_size = acp70_sdw1_dma_reg[stream_id as usize].reg_fifo_size;
                    reg_ring_buf_size = acp70_sdw1_dma_reg[stream_id as usize].reg_ring_buf_size;
                    reg_ring_buf_addr = acp70_sdw1_dma_reg[stream_id as usize].reg_ring_buf_addr;
                }
                _ => return -EINVAL,
            }
        }
        _ => return -EINVAL,
    }
    sdw_fifo_offset = ACP_SDW_FIFO_OFFSET(manager_instance);
    sdw_mem_window_offset = SDW_MEM_WINDOW_START(manager_instance);
    sdw_fifo_addr = sdw_fifo_offset + stream_id * SDW_FIFO_OFFSET;
    sdw_ring_buf_addr = sdw_mem_window_offset + stream_id * ACP_SDW_RING_BUFF_ADDR_OFFSET;
    sdw_ring_buf_size = size;
    writel(sdw_ring_buf_size, acp_base.add(reg_ring_buf_size as usize));
    writel(sdw_ring_buf_addr, acp_base.add(reg_ring_buf_addr as usize));
    writel(sdw_fifo_addr, acp_base.add(reg_fifo_addr as usize));
    writel(SDW_DMA_SIZE, acp_base.add(reg_dma_size as usize));
    writel(SDW_FIFO_SIZE, acp_base.add(reg_fifo_size as usize));
    0
}

unsafe fn acp63_sdw_dma_open(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> i32 {
    let runtime: *mut snd_pcm_runtime;
    let stream: *mut acp_sdw_dma_stream;
    let cpu_dai: *mut snd_soc_dai;
    let amd_manager: *mut amd_sdw_manager;
    let prtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let mut ret: i32;

    runtime = (*substream).runtime;
    cpu_dai = snd_soc_rtd_to_cpu(prtd, 0);
    amd_manager = snd_soc_dai_get_drvdata(cpu_dai);
    stream = kzalloc_obj::<acp_sdw_dma_stream>();
    if stream.is_null() {
        return -ENOMEM;
    }

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        (*runtime).hw = acp63_sdw_hardware_playback;
    } else {
        (*runtime).hw = acp63_sdw_hardware_capture;
    }
    ret = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if ret < 0 {
        dev_err((*component).dev, b"set integer constraint failed\n\0".as_ptr());
        kfree(stream as *mut core::ffi::c_void);
        return ret;
    }

    (*stream).stream_id = (*cpu_dai).id;
    (*stream).instance = (*amd_manager).instance;
    (*runtime).private_data = stream as *mut core::ffi::c_void;
    ret
}

unsafe fn acp63_sdw_dma_hw_params(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> i32 {
    let stream: *mut acp_sdw_dma_stream;
    let sdw_data: *mut sdw_dma_dev_data;
    let period_bytes: u32;
    let water_mark_size_reg: u32;
    let mut irq_mask: u32;
    let mut ext_intr_ctrl: u32;
    let size: u64;
    let stream_id: u32;
    let acp_ext_intr_cntl_reg: u32;
    let ret: i32;

    sdw_data = dev_get_drvdata((*component).dev);
    stream = (*(*substream).runtime).private_data as *mut acp_sdw_dma_stream;
    if stream.is_null() {
        return -EINVAL;
    }
    stream_id = (*stream).stream_id;
    match (*sdw_data).acp_rev {
        ACP63_PCI_REV => {
            match (*stream).instance {
                ACP_SDW0 => {
                    (*sdw_data).acp63_sdw0_dma_stream[stream_id as usize] = substream;
                    water_mark_size_reg = acp63_sdw0_dma_reg[stream_id as usize].water_mark_size_reg;
                    acp_ext_intr_cntl_reg = ACP_EXTERNAL_INTR_CNTL;
                    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
                        irq_mask = BIT(ACP63_SDW0_DMA_TX_IRQ_MASK(stream_id));
                    } else {
                        irq_mask = BIT(ACP63_SDW0_DMA_RX_IRQ_MASK(stream_id));
                    }
                }
                ACP_SDW1 => {
                    (*sdw_data).acp63_sdw1_dma_stream[stream_id as usize] = substream;
                    acp_ext_intr_cntl_reg = ACP_EXTERNAL_INTR_CNTL1;
                    water_mark_size_reg = acp63_sdw1_dma_reg[stream_id as usize].water_mark_size_reg;
                    irq_mask = BIT(ACP63_SDW1_DMA_IRQ_MASK(stream_id));
                }
                _ => return -EINVAL,
            }
        }
        ACP70_PCI_REV | ACP71_PCI_REV | ACP72_PCI_REV => {
            match (*stream).instance {
                ACP_SDW0 => {
                    (*sdw_data).acp70_sdw0_dma_stream[stream_id as usize] = substream;
                    water_mark_size_reg = acp70_sdw0_dma_reg[stream_id as usize].water_mark_size_reg;
                    acp_ext_intr_cntl_reg = ACP_EXTERNAL_INTR_CNTL;
                    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
                        irq_mask = BIT(ACP70_SDW0_DMA_TX_IRQ_MASK(stream_id));
                    } else {
                        irq_mask = BIT(ACP70_SDW0_DMA_RX_IRQ_MASK(stream_id));
                    }
                }
                ACP_SDW1 => {
                    (*sdw_data).acp70_sdw1_dma_stream[stream_id as usize] = substream;
                    acp_ext_intr_cntl_reg = ACP_EXTERNAL_INTR_CNTL1;
                    water_mark_size_reg = acp70_sdw1_dma_reg[stream_id as usize].water_mark_size_reg;
                    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
                        irq_mask = BIT(ACP70_SDW1_DMA_TX_IRQ_MASK(stream_id));
                    } else {
                        irq_mask = BIT(ACP70_SDW1_DMA_RX_IRQ_MASK(stream_id));
                    }
                }
                _ => return -EINVAL,
            }
        }
        _ => return -EINVAL,
    }
    size = params_buffer_bytes(params) as u64;
    period_bytes = params_period_bytes(params);
    (*stream).dma_addr = (*(*substream).runtime).dma_addr;
    (*stream).num_pages = PAGE_ALIGN(size) >> PAGE_SHIFT;
    acp63_config_dma(stream, (*sdw_data).acp_base, stream_id);
    ret = acp63_configure_sdw_ringbuffer((*sdw_data).acp_base, stream_id, size as u32, (*stream).instance, (*sdw_data).acp_rev);
    if ret != 0 {
        dev_err((*component).dev, b"Invalid DMA channel\n\0".as_ptr());
        return -EINVAL;
    }
    ext_intr_ctrl = readl((*sdw_data).acp_base.add(acp_ext_intr_cntl_reg as usize));
    ext_intr_ctrl |= irq_mask;
    writel(ext_intr_ctrl, (*sdw_data).acp_base.add(acp_ext_intr_cntl_reg as usize));
    writel(period_bytes, (*sdw_data).acp_base.add(water_mark_size_reg as usize));
    0
}

unsafe fn acp63_sdw_get_byte_count(
    stream: *mut acp_sdw_dma_stream,
    acp_base: *mut core::ffi::c_void,
    acp_rev: u32,
) -> u64 {
    let mut byte_count: acp_sdw_dma_count = core::mem::zeroed();
    let pos_low_reg: u32;
    let pos_high_reg: u32;

    byte_count.bytescount = 0;
    match acp_rev {
        ACP63_PCI_REV => {
            match (*stream).instance {
                ACP_SDW0 => {
                    pos_low_reg = acp63_sdw0_dma_reg[(*stream).stream_id as usize].pos_low_reg;
                    pos_high_reg = acp63_sdw0_dma_reg[(*stream).stream_id as usize].pos_high_reg;
                }
                ACP_SDW1 => {
                    pos_low_reg = acp63_sdw1_dma_reg[(*stream).stream_id as usize].pos_low_reg;
                    pos_high_reg = acp63_sdw1_dma_reg[(*stream).stream_id as usize].pos_high_reg;
                }
                _ => return byte_count.bytescount,
            }
        }
        ACP70_PCI_REV | ACP71_PCI_REV | ACP72_PCI_REV => {
            match (*stream).instance {
                ACP_SDW0 => {
                    pos_low_reg = acp70_sdw0_dma_reg[(*stream).stream_id as usize].pos_low_reg;
                    pos_high_reg = acp70_sdw0_dma_reg[(*stream).stream_id as usize].pos_high_reg;
                }
                ACP_SDW1 => {
                    pos_low_reg = acp70_sdw1_dma_reg[(*stream).stream_id as usize].pos_low_reg;
                    pos_high_reg = acp70_sdw1_dma_reg[(*stream).stream_id as usize].pos_high_reg;
                }
                _ => return byte_count.bytescount,
            }
        }
        _ => return byte_count.bytescount,
    }
    if pos_low_reg != 0 {
        byte_count.bcount.high = readl(acp_base.add(pos_high_reg as usize));
        byte_count.bcount.low = readl(acp_base.add(pos_low_reg as usize));
    }
    byte_count.bytescount
}

unsafe fn acp63_sdw_dma_pointer(
    comp: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let sdw_data: *mut sdw_dma_dev_data;
    let stream: *mut acp_sdw_dma_stream;
    let pos: u32;
    let buffersize: u32;
    let mut bytescount: u64;

    sdw_data = dev_get_drvdata((*comp).dev);
    stream = (*(*substream).runtime).private_data as *mut acp_sdw_dma_stream;
    buffersize = frames_to_bytes((*substream).runtime, (*(*substream).runtime).buffer_size);
    bytescount = acp63_sdw_get_byte_count(stream, (*sdw_data).acp_base, (*sdw_data).acp_rev);
    if bytescount > (*stream).bytescount {
        bytescount -= (*stream).bytescount;
    }
    pos = do_div(&mut bytescount, buffersize);
    bytes_to_frames((*substream).runtime, pos)
}

unsafe fn acp63_sdw_dma_new(
    component: *mut snd_soc_component,
    rtd: *mut snd_soc_pcm_runtime,
) -> i32 {
    let parent: *mut device = (*(*component).dev).parent;

    snd_pcm_set_managed_buffer_all((*rtd).pcm, SNDRV_DMA_TYPE_DEV, parent, SDW_MIN_BUFFER, SDW_MAX_BUFFER);
    0
}

unsafe fn acp63_sdw_dma_close(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> i32 {
    let sdw_data: *mut sdw_dma_dev_data;
    let stream: *mut acp_sdw_dma_stream;

    sdw_data = dev_get_drvdata((*component).dev);
    stream = (*(*substream).runtime).private_data as *mut acp_sdw_dma_stream;
    if stream.is_null() {
        return -EINVAL;
    }
    match (*sdw_data).acp_rev {
        ACP63_PCI_REV => {
            match (*stream).instance {
                ACP_SDW0 => (*sdw_data).acp63_sdw0_dma_stream[(*stream).stream_id as usize] = core::ptr::null_mut(),
                ACP_SDW1 => (*sdw_data).acp63_sdw1_dma_stream[(*stream).stream_id as usize] = core::ptr::null_mut(),
                _ => return -EINVAL,
            }
        }
        ACP70_PCI_REV | ACP71_PCI_REV | ACP72_PCI_REV => {
            match (*stream).instance {
                ACP_SDW0 => (*sdw_data).acp70_sdw0_dma_stream[(*stream).stream_id as usize] = core::ptr::null_mut(),
                ACP_SDW1 => (*sdw_data).acp70_sdw1_dma_stream[(*stream).stream_id as usize] = core::ptr::null_mut(),
                _ => return -EINVAL,
            }
        }
        _ => return -EINVAL,
    }
    kfree(stream as *mut core::ffi::c_void);
    0
}

unsafe fn acp63_sdw_dma_enable(
    substream: *mut snd_pcm_substream,
    acp_base: *mut core::ffi::c_void,
    acp_rev: u32,
    sdw_dma_enable: bool,
) -> i32 {
    let stream: *mut acp_sdw_dma_stream;
    let stream_id: u32;
    let sdw_dma_en_reg: u32;
    let sdw_dma_en_stat_reg: u32;
    let mut sdw_dma_stat: u32 = 0;
    let dma_enable: u32;

    stream = (*(*substream).runtime).private_data as *mut acp_sdw_dma_stream;
    stream_id = (*stream).stream_id;
    match acp_rev {
        ACP63_PCI_REV => {
            match (*stream).instance {
                ACP_SDW0 => sdw_dma_en_reg = acp63_sdw0_dma_enable_reg[stream_id as usize],
                ACP_SDW1 => sdw_dma_en_reg = acp63_sdw1_dma_enable_reg[stream_id as usize],
                _ => return -EINVAL,
            }
        }
        ACP70_PCI_REV | ACP71_PCI_REV | ACP72_PCI_REV => {
            match (*stream).instance {
                ACP_SDW0 => sdw_dma_en_reg = acp70_sdw0_dma_enable_reg[stream_id as usize],
                ACP_SDW1 => sdw_dma_en_reg = acp70_sdw1_dma_enable_reg[stream_id as usize],
                _ => return -EINVAL,
            }
        }
        _ => return -EINVAL,
    }
    sdw_dma_en_stat_reg = sdw_dma_en_reg + 4;
    dma_enable = sdw_dma_enable as u32;
    writel(dma_enable, acp_base.add(sdw_dma_en_reg as usize));
    readl_poll_timeout(
        acp_base.add(sdw_dma_en_stat_reg as usize),
        &mut sdw_dma_stat,
        sdw_dma_stat == dma_enable,
        ACP_DELAY_US,
        ACP_COUNTER,
    )
}

unsafe fn acp63_sdw_dma_trigger(
    comp: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    cmd: i32,
) -> i32 {
    let sdw_data: *mut sdw_dma_dev_data;
    let ret: i32;

    sdw_data = dev_get_drvdata((*comp).dev);
    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_PAUSE_RELEASE | SNDRV_PCM_TRIGGER_RESUME => {
            ret = acp63_sdw_dma_enable(substream, (*sdw_data).acp_base, (*sdw_data).acp_rev, true);
        }
        SNDRV_PCM_TRIGGER_PAUSE_PUSH | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_STOP => {
            ret = acp63_sdw_dma_enable(substream, (*sdw_data).acp_base, (*sdw_data).acp_rev, false);
        }
        _ => ret = -EINVAL,
    }
    if ret != 0 {
        dev_err((*comp).dev, b"trigger %d failed: %d\0".as_ptr(), cmd, ret);
    }
    ret
}

static acp63_sdw_component: snd_soc_component_driver = snd_soc_component_driver {
    name: DRV_NAME.as_ptr(),
    open: Some(acp63_sdw_dma_open),
    close: Some(acp63_sdw_dma_close),
    hw_params: Some(acp63_sdw_dma_hw_params),
    trigger: Some(acp63_sdw_dma_trigger),
    pointer: Some(acp63_sdw_dma_pointer),
    pcm_new: Some(acp63_sdw_dma_new),
    use_dai_pcm_id: true,
};

unsafe fn acp63_sdw_platform_probe(pdev: *mut platform_device) -> i32 {
    let res: *mut resource;
    let sdw_data: *mut sdw_dma_dev_data;
    let acp_data: *mut acp63_dev_data;
    let parent: *mut device;
    let status: i32;

    parent = (*(*pdev).dev).parent;
    acp_data = dev_get_drvdata(parent);
    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if res.is_null() {
        dev_err(&mut (*pdev).dev, b"IORESOURCE_MEM FAILED\n\0".as_ptr());
        return -ENODEV;
    }

    sdw_data = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<sdw_dma_dev_data>(), GFP_KERNEL);
    if sdw_data.is_null() {
        return -ENOMEM;
    }

    (*sdw_data).acp_base = devm_ioremap(&mut (*pdev).dev, (*res).start, resource_size(res));
    if (*sdw_data).acp_base.is_null() {
        return -ENOMEM;
    }

    (*sdw_data).acp_lock = &mut (*acp_data).acp_lock;
    (*sdw_data).acp_rev = (*acp_data).acp_rev;
    dev_set_drvdata(&mut (*pdev).dev, sdw_data as *mut core::ffi::c_void);
    status = devm_snd_soc_register_component(&mut (*pdev).dev, &acp63_sdw_component, core::ptr::null_mut(), 0);
    if status != 0 {
        dev_err(&mut (*pdev).dev, b"Fail to register sdw dma component\n\0".as_ptr());
        return status;
    }
    pm_runtime_set_autosuspend_delay(&mut (*pdev).dev, ACP_SUSPEND_DELAY_MS);
    pm_runtime_use_autosuspend(&mut (*pdev).dev);
    pm_runtime_mark_last_busy(&mut (*pdev).dev);
    pm_runtime_set_active(&mut (*pdev).dev);
    pm_runtime_enable(&mut (*pdev).dev);
    0
}

unsafe fn acp63_sdw_platform_remove(pdev: *mut platform_device) {
    pm_runtime_disable(&mut (*pdev).dev);
}

unsafe fn acp63_restore_sdw_dma_config(sdw_data: *mut sdw_dma_dev_data) -> i32 {
    let mut stream: *mut acp_sdw_dma_stream;
    let mut substream: *mut snd_pcm_substream;
    let mut runtime: *mut snd_pcm_runtime;
    let mut period_bytes: u32;
    let mut buf_size: u32;
    let mut water_mark_size_reg: u32;
    let mut stream_count: u32;
    let irq_mask: u32;
    let irq_mask1: u32;
    let mut index: i32;
    let mut instance: i32;
    let mut ret: i32;

    irq_mask = ACP63_SDW_DMA_IRQ_MASK;
    irq_mask1 = ACP63_P1_SDW_DMA_IRQ_MASK;
    instance = 0;
    while instance < AMD_SDW_MAX_MANAGERS as i32 {
        if instance == ACP_SDW0 as i32 {
            stream_count = ACP63_SDW0_DMA_MAX_STREAMS as u32;
        } else {
            stream_count = ACP63_SDW1_DMA_MAX_STREAMS as u32;
        }

        index = 0;
        while index < stream_count as i32 {
            if instance == ACP_SDW0 as i32 {
                substream = (*sdw_data).acp63_sdw0_dma_stream[index as usize];
                water_mark_size_reg = acp63_sdw0_dma_reg[index as usize].water_mark_size_reg;
            } else {
                substream = (*sdw_data).acp63_sdw1_dma_stream[index as usize];
                water_mark_size_reg = acp63_sdw1_dma_reg[index as usize].water_mark_size_reg;
            }

            if !substream.is_null() && !(*substream).runtime.is_null() {
                runtime = (*substream).runtime;
                stream = (*runtime).private_data as *mut acp_sdw_dma_stream;
                period_bytes = frames_to_bytes(runtime, (*runtime).period_size);
                buf_size = frames_to_bytes(runtime, (*runtime).buffer_size);
                acp63_config_dma(stream, (*sdw_data).acp_base, index as u32);
                ret = acp63_configure_sdw_ringbuffer((*sdw_data).acp_base, index as u32, buf_size, instance as u32, ACP63_PCI_REV);
                if ret != 0 {
                    return ret;
                }
                writel(period_bytes, (*sdw_data).acp_base.add(water_mark_size_reg as usize));
            }
            index += 1;
        }
        instance += 1;
    }
    acp63_enable_disable_sdw_dma_interrupts((*sdw_data).acp_base, irq_mask, irq_mask1, true);
    0
}

unsafe fn acp70_restore_sdw_dma_config(sdw_data: *mut sdw_dma_dev_data) -> i32 {
    let mut stream: *mut acp_sdw_dma_stream;
    let mut substream: *mut snd_pcm_substream;
    let mut runtime: *mut snd_pcm_runtime;
    let mut period_bytes: u32;
    let mut buf_size: u32;
    let mut water_mark_size_reg: u32;
    let stream_count: u32;
    let irq_mask: u32;
    let irq_mask1: u32;
    let mut index: i32;
    let mut instance: i32;
    let mut ret: i32;

    irq_mask = ACP70_SDW_DMA_IRQ_MASK;
    irq_mask1 = ACP70_P1_SDW_DMA_IRQ_MASK;
    stream_count = ACP70_SDW0_DMA_MAX_STREAMS as u32;
    instance = 0;
    while instance < AMD_SDW_MAX_MANAGERS as i32 {
        index = 0;
        while index < stream_count as i32 {
            if instance == ACP_SDW0 as i32 {
                substream = (*sdw_data).acp70_sdw0_dma_stream[index as usize];
                water_mark_size_reg = acp70_sdw0_dma_reg[index as usize].water_mark_size_reg;
            } else {
                substream = (*sdw_data).acp70_sdw1_dma_stream[index as usize];
                water_mark_size_reg = acp70_sdw1_dma_reg[index as usize].water_mark_size_reg;
            }

            if !substream.is_null() && !(*substream).runtime.is_null() {
                runtime = (*substream).runtime;
                stream = (*runtime).private_data as *mut acp_sdw_dma_stream;
                period_bytes = frames_to_bytes(runtime, (*runtime).period_size);
                buf_size = frames_to_bytes(runtime, (*runtime).buffer_size);
                acp63_config_dma(stream, (*sdw_data).acp_base, index as u32);
                ret = acp63_configure_sdw_ringbuffer((*sdw_data).acp_base, index as u32, buf_size, instance as u32, (*sdw_data).acp_rev);
                if ret != 0 {
                    return ret;
                }
                writel(period_bytes, (*sdw_data).acp_base.add(water_mark_size_reg as usize));
            }
            index += 1;
        }
        instance += 1;
    }
    acp63_enable_disable_sdw_dma_interrupts((*sdw_data).acp_base, irq_mask, irq_mask1, true);
    0
}

unsafe fn acp63_sdw_pcm_resume(dev: *mut device) -> i32 {
    let sdw_data: *mut sdw_dma_dev_data;

    sdw_data = dev_get_drvdata(dev);
    if (*sdw_data).acp_rev == ACP63_PCI_REV {
        acp63_restore_sdw_dma_config(sdw_data)
    } else {
        acp70_restore_sdw_dma_config(sdw_data)
    }
}

static acp63_pm_ops: dev_pm_ops = dev_pm_ops {
    /* SYSTEM_SLEEP_PM_OPS(NULL, acp63_sdw_pcm_resume) */
    resume: Some(acp63_sdw_pcm_resume),
};

static mut acp63_sdw_dma_driver: platform_driver = platform_driver {
    probe: Some(acp63_sdw_platform_probe),
    remove: Some(acp63_sdw_platform_remove),
    driver: device_driver {
        name: b"amd_ps_sdw_dma\0".as_ptr(),
        pm: pm_ptr(&acp63_pm_ops),
    },
};

module_platform_driver!(acp63_sdw_dma_driver);

MODULE_AUTHOR!("Vijendar.Mukunda@amd.com");
MODULE_DESCRIPTION!("AMD common SDW DMA Driver for ACP6.3, ACP7.0 & ACP7.1 platforms");
MODULE_LICENSE!("GPL");
MODULE_ALIAS!("platform:amd_ps_sdw_dma");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
