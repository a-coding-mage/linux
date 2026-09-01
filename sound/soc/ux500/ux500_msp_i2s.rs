// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) ST-Ericsson SA 2012
 *
 * Author: Ola Lilja <ola.o.lilja@stericsson.com>,
 *         Roger Nilsson <roger.xr.nilsson@stericsson.com>,
 *         Sandeep Kaushik <sandeep.kaushik@st.com>
 *         for ST-Ericsson.
 */

// External dependencies from linux/soc headers and ux500_msp_i2s.h
extern "C" {
    // Types from ux500_msp_i2s.h
    pub type ux500_msp;
    pub type ux500_msp_config;
    pub type msp_protdesc;
    pub type msp_multichannel_config;
    pub type msp_data_size;
    pub type platform_device;
    pub type resource;
    pub type device;

    // Linux kernel functions
    fn readl(addr: *const u32) -> u32;
    fn writel(val: u32, addr: *mut u32);
    fn udelay(usecs: u32);
    fn in_interrupt() -> i32;

    // Platform device functions
    fn platform_get_resource(
        dev: *mut platform_device,
        typ: u32,
        num: u32,
    ) -> *mut resource;

    // Device resource management
    fn devm_kzalloc(dev: *mut device, size: usize, gfp_flags: u32) -> *mut u8;
    fn devm_ioremap(dev: *mut device, offset: usize, size: usize) -> *mut u32;

    // Logging functions
    fn dev_err(dev: *const device, fmt: *const u8, ...);
    fn dev_warn(dev: *const device, fmt: *const u8, ...);
    fn dev_dbg(dev: *const device, fmt: *const u8, ...);
}

// Constants and macros from headers
const IORESOURCE_MEM: u32 = 0;
const GFP_KERNEL: u32 = 0;

// Resource constants (would be defined in header)
const MSP_DR: usize = 0;
const MSP_GCR: usize = 0;
const MSP_TCF: usize = 0;
const MSP_RCF: usize = 0;
const MSP_DMACR: usize = 0;
const MSP_IODLY: usize = 0;
const MSP_FLR: usize = 0;
const MSP_SRG: usize = 0;
const MSP_MCR: usize = 0;
const MSP_RCM: usize = 0;
const MSP_RCV: usize = 0;
const MSP_ITCR: usize = 0;
const MSP_TSTDR: usize = 0;
const MSP_IMSC: usize = 0;
const MSP_TCE0: usize = 0;
const MSP_TCE1: usize = 0;
const MSP_TCE2: usize = 0;
const MSP_TCE3: usize = 0;
const MSP_RCE0: usize = 0;
const MSP_RCE1: usize = 0;
const MSP_RCE2: usize = 0;
const MSP_RCE3: usize = 0;

// Bit masks and constants (would be defined in header)
const TX_CLK_POL_RISING: u32 = 0;
const RX_CLK_POL_RISING: u32 = 0;
const SRG_ENABLE: u32 = 0;
const RX_ENABLE: u32 = 0;
const TX_ENABLE: u32 = 0;
const FRAME_GEN_ENABLE: u32 = 0;
const RX_FIFO_EMPTY: u32 = 0;
const TX_FIFO_EMPTY: u32 = 0;
const RX_DMA_ENABLE: u32 = 0;
const TX_DMA_ENABLE: u32 = 0;
const RX_SERVICE_INT: u32 = 0;
const RX_OVERRUN_ERROR_INT: u32 = 0;
const TX_SERVICE_INT: u32 = 0;
const TX_UNDERRUN_ERR_INT: u32 = 0;
const LOOPBACK_MASK: u32 = 0;
const RX_CLK_SEL_MASK: u32 = 0;
const TX_CLK_SEL_MASK: u32 = 0;
const RX_FSYNC_MASK: u32 = 0;
const TX_FSYNC_MASK: u32 = 0;
const RX_SYNC_SEL_MASK: u32 = 0;
const TX_SYNC_SEL_MASK: u32 = 0;
const RX_FIFO_ENABLE_MASK: u32 = 0;
const TX_FIFO_ENABLE_MASK: u32 = 0;
const SRG_CLK_SEL_MASK: u32 = 0;
const TX_EXTRA_DELAY_MASK: u32 = 0;
const SCK_DIV_MASK: u32 = 0;
const MSP_ITCR_ITEN: u32 = 0;
const MSP_ITCR_TESTFIFO: u32 = 0;
const TMCEN_BIT: u32 = 0;
const RMCEN_BIT: u32 = 0;
const RCMPM_BIT: u32 = 0;

// Macro functions (simplified as inline functions - full implementations in header)
fn MSP_P2_ENABLE_BIT(val: u32) -> u32 { val }
fn MSP_P2_START_MODE_BIT(val: u32) -> u32 { val }
fn MSP_P1_FRAME_LEN_BITS(val: u32) -> u32 { val }
fn MSP_P2_FRAME_LEN_BITS(val: u32) -> u32 { val }
fn MSP_P1_ELEM_LEN_BITS(val: u32) -> u32 { val }
fn MSP_P2_ELEM_LEN_BITS(val: u32) -> u32 { val }
fn MSP_DATA_DELAY_BITS(val: u32) -> u32 { val }
fn MSP_SET_ENDIANNES_BIT(val: u32) -> u32 { val }
fn MSP_FSYNC_POL(val: u32) -> u32 { val }
fn MSP_DATA_WORD_SWAP(val: u32) -> u32 { val }
fn MSP_SET_COMPANDING_MODE(val: u32) -> u32 { val }
fn MSP_SET_FSYNC_IGNORE(val: u32) -> u32 { val }
fn MSP_TX_CLKPOL_BIT(val: u32) -> u32 { val }
fn MSP_RX_CLKPOL_BIT(val: u32) -> u32 { val }
fn FRAME_WIDTH_BITS(val: u32) -> u32 { val }
fn FRAME_PERIOD_BITS(val: u32) -> u32 { val }

// Protocol descriptor constants from header
const MSP_SINGLE_PHASE: u32 = 0;
const MSP_DUAL_PHASE: u32 = 1;
const MSP_PHASE2_START_MODE_IMEDIATE: u32 = 0;
const MSP_PHASE2_START_MODE_FSYNC: u32 = 1;
const MSP_BTF_MS_BIT_FIRST: u32 = 0;
const MSP_FRAME_LEN_1: u32 = 0;
const MSP_ELEM_LEN_32: u32 = 0;
const MSP_ELEM_LEN_16: u32 = 0;
const MSP_ELEM_LEN_8: u32 = 0;
const MSP_DELAY_0: u32 = 0;
const MSP_DELAY_1: u32 = 1;
const MSP_RISING_EDGE: u32 = 0;
const MSP_FALLING_EDGE: u32 = 1;
const MSP_FSYNC_POL_ACT_LO: u32 = 0;
const MSP_FSYNC_POL_ACT_HI: u32 = 1;
const MSP_SWAP_NONE: u32 = 0;
const MSP_COMPRESS_MODE_LINEAR: u32 = 0;
const MSP_EXPAND_MODE_LINEAR: u32 = 0;
const MSP_FSYNC_IGNORE: u32 = 0;

// Protocol constants
const MSP_I2S_PROTOCOL: u32 = 0;
const MSP_PCM_PROTOCOL: u32 = 1;
const MSP_PCM_COMPAND_PROTOCOL: u32 = 2;
const MSP_INVALID_PROTOCOL: u32 = 3;

// Direction constants
const MSP_DIR_TX: u32 = 1;
const MSP_DIR_RX: u32 = 2;

// MSP state constants
const MSP_STATE_IDLE: u32 = 0;
const MSP_STATE_CONFIGURED: u32 = 1;

// Data size constants
const MSP_DATA_BITS_DEFAULT: u32 = 0;
const MSP_DATA_BITS_32: u32 = 32;

// Trigger command constants
const SNDRV_PCM_TRIGGER_START: i32 = 0;
const SNDRV_PCM_TRIGGER_RESUME: i32 = 2;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: i32 = 3;
const SNDRV_PCM_TRIGGER_STOP: i32 = 5;
const SNDRV_PCM_TRIGGER_SUSPEND: i32 = 6;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: i32 = 1;
const SNDRV_PCM_STREAM_PLAYBACK: i32 = 0;

/* Protocol descriptors */
static PROT_DESCS: [msp_protdesc_init; 3] = [
    // I2S
    msp_protdesc_init {
        tx_phase_mode: MSP_SINGLE_PHASE,
        rx_phase_mode: MSP_SINGLE_PHASE,
        tx_phase2_start_mode: MSP_PHASE2_START_MODE_IMEDIATE,
        rx_phase2_start_mode: MSP_PHASE2_START_MODE_IMEDIATE,
        tx_bit_order: MSP_BTF_MS_BIT_FIRST,
        rx_bit_order: MSP_BTF_MS_BIT_FIRST,
        tx_frame_len_1: MSP_FRAME_LEN_1,
        rx_frame_len_1: MSP_FRAME_LEN_1,
        tx_frame_len_2: MSP_FRAME_LEN_1,
        rx_frame_len_2: MSP_FRAME_LEN_1,
        tx_elem_len_1: MSP_ELEM_LEN_32,
        rx_elem_len_1: MSP_ELEM_LEN_32,
        tx_elem_len_2: MSP_ELEM_LEN_32,
        rx_elem_len_2: MSP_ELEM_LEN_32,
        tx_data_delay: MSP_DELAY_1,
        rx_data_delay: MSP_DELAY_1,
        tx_clk_pol: MSP_RISING_EDGE,
        rx_clk_pol: MSP_FALLING_EDGE,
        tx_fsync_pol: MSP_FSYNC_POL_ACT_LO,
        rx_fsync_pol: MSP_FSYNC_POL_ACT_LO,
        tx_half_word_swap: MSP_SWAP_NONE,
        rx_half_word_swap: MSP_SWAP_NONE,
        compression_mode: MSP_COMPRESS_MODE_LINEAR,
        expansion_mode: MSP_EXPAND_MODE_LINEAR,
        frame_sync_ignore: MSP_FSYNC_IGNORE,
        frame_width: 31,
        frame_period: 15,
        clocks_per_frame: 32,
    },
    // PCM
    msp_protdesc_init {
        tx_phase_mode: MSP_DUAL_PHASE,
        rx_phase_mode: MSP_DUAL_PHASE,
        tx_phase2_start_mode: MSP_PHASE2_START_MODE_FSYNC,
        rx_phase2_start_mode: MSP_PHASE2_START_MODE_FSYNC,
        tx_bit_order: MSP_BTF_MS_BIT_FIRST,
        rx_bit_order: MSP_BTF_MS_BIT_FIRST,
        tx_frame_len_1: MSP_FRAME_LEN_1,
        rx_frame_len_1: MSP_FRAME_LEN_1,
        tx_frame_len_2: MSP_FRAME_LEN_1,
        rx_frame_len_2: MSP_FRAME_LEN_1,
        tx_elem_len_1: MSP_ELEM_LEN_16,
        rx_elem_len_1: MSP_ELEM_LEN_16,
        tx_elem_len_2: MSP_ELEM_LEN_16,
        rx_elem_len_2: MSP_ELEM_LEN_16,
        tx_data_delay: MSP_DELAY_0,
        rx_data_delay: MSP_DELAY_0,
        tx_clk_pol: MSP_RISING_EDGE,
        rx_clk_pol: MSP_FALLING_EDGE,
        tx_fsync_pol: MSP_FSYNC_POL_ACT_HI,
        rx_fsync_pol: MSP_FSYNC_POL_ACT_HI,
        tx_half_word_swap: MSP_SWAP_NONE,
        rx_half_word_swap: MSP_SWAP_NONE,
        compression_mode: MSP_COMPRESS_MODE_LINEAR,
        expansion_mode: MSP_EXPAND_MODE_LINEAR,
        frame_sync_ignore: MSP_FSYNC_IGNORE,
        frame_width: 255,
        frame_period: 0,
        clocks_per_frame: 256,
    },
    // Companded PCM
    msp_protdesc_init {
        tx_phase_mode: MSP_SINGLE_PHASE,
        rx_phase_mode: MSP_SINGLE_PHASE,
        tx_phase2_start_mode: MSP_PHASE2_START_MODE_FSYNC,
        rx_phase2_start_mode: MSP_PHASE2_START_MODE_FSYNC,
        tx_bit_order: MSP_BTF_MS_BIT_FIRST,
        rx_bit_order: MSP_BTF_MS_BIT_FIRST,
        tx_frame_len_1: MSP_FRAME_LEN_1,
        rx_frame_len_1: MSP_FRAME_LEN_1,
        tx_frame_len_2: MSP_FRAME_LEN_1,
        rx_frame_len_2: MSP_FRAME_LEN_1,
        tx_elem_len_1: MSP_ELEM_LEN_8,
        rx_elem_len_1: MSP_ELEM_LEN_8,
        tx_elem_len_2: MSP_ELEM_LEN_8,
        rx_elem_len_2: MSP_ELEM_LEN_8,
        tx_data_delay: MSP_DELAY_0,
        rx_data_delay: MSP_DELAY_0,
        tx_clk_pol: MSP_RISING_EDGE,
        rx_clk_pol: MSP_RISING_EDGE,
        tx_fsync_pol: MSP_FSYNC_POL_ACT_HI,
        rx_fsync_pol: MSP_FSYNC_POL_ACT_HI,
        tx_half_word_swap: MSP_SWAP_NONE,
        rx_half_word_swap: MSP_SWAP_NONE,
        compression_mode: MSP_COMPRESS_MODE_LINEAR,
        expansion_mode: MSP_EXPAND_MODE_LINEAR,
        frame_sync_ignore: MSP_FSYNC_IGNORE,
        frame_width: 255,
        frame_period: 0,
        clocks_per_frame: 256,
    },
];

// Protocol descriptor initialization structure (mimic the C array)
#[repr(C)]
pub struct msp_protdesc_init {
    tx_phase_mode: u32,
    rx_phase_mode: u32,
    tx_phase2_start_mode: u32,
    rx_phase2_start_mode: u32,
    tx_bit_order: u32,
    rx_bit_order: u32,
    tx_frame_len_1: u32,
    rx_frame_len_1: u32,
    tx_frame_len_2: u32,
    rx_frame_len_2: u32,
    tx_elem_len_1: u32,
    rx_elem_len_1: u32,
    tx_elem_len_2: u32,
    rx_elem_len_2: u32,
    tx_data_delay: u32,
    rx_data_delay: u32,
    tx_clk_pol: u32,
    rx_clk_pol: u32,
    tx_fsync_pol: u32,
    rx_fsync_pol: u32,
    tx_half_word_swap: u32,
    rx_half_word_swap: u32,
    compression_mode: u32,
    expansion_mode: u32,
    frame_sync_ignore: u32,
    frame_width: u32,
    frame_period: u32,
    clocks_per_frame: u32,
}

unsafe fn set_prot_desc_tx(
    msp: *mut ux500_msp,
    protdesc: *const msp_protdesc,
    data_size: u32,
) {
    let mut temp_reg: u32 = 0;

    temp_reg |= MSP_P2_ENABLE_BIT((*protdesc).tx_phase_mode);
    temp_reg |= MSP_P2_START_MODE_BIT((*protdesc).tx_phase2_start_mode);
    temp_reg |= MSP_P1_FRAME_LEN_BITS((*protdesc).tx_frame_len_1);
    temp_reg |= MSP_P2_FRAME_LEN_BITS((*protdesc).tx_frame_len_2);
    if (*msp).def_elem_len != 0 {
        temp_reg |= MSP_P1_ELEM_LEN_BITS((*protdesc).tx_elem_len_1);
        temp_reg |= MSP_P2_ELEM_LEN_BITS((*protdesc).tx_elem_len_2);
    } else {
        temp_reg |= MSP_P1_ELEM_LEN_BITS(data_size);
        temp_reg |= MSP_P2_ELEM_LEN_BITS(data_size);
    }
    temp_reg |= MSP_DATA_DELAY_BITS((*protdesc).tx_data_delay);
    temp_reg |= MSP_SET_ENDIANNES_BIT((*protdesc).tx_byte_order);
    temp_reg |= MSP_FSYNC_POL((*protdesc).tx_fsync_pol);
    temp_reg |= MSP_DATA_WORD_SWAP((*protdesc).tx_half_word_swap);
    temp_reg |= MSP_SET_COMPANDING_MODE((*protdesc).compression_mode);
    temp_reg |= MSP_SET_FSYNC_IGNORE((*protdesc).frame_sync_ignore);

    writel(temp_reg, (*msp).registers.add(MSP_TCF));
}

unsafe fn set_prot_desc_rx(
    msp: *mut ux500_msp,
    protdesc: *const msp_protdesc,
    data_size: u32,
) {
    let mut temp_reg: u32 = 0;

    temp_reg |= MSP_P2_ENABLE_BIT((*protdesc).rx_phase_mode);
    temp_reg |= MSP_P2_START_MODE_BIT((*protdesc).rx_phase2_start_mode);
    temp_reg |= MSP_P1_FRAME_LEN_BITS((*protdesc).rx_frame_len_1);
    temp_reg |= MSP_P2_FRAME_LEN_BITS((*protdesc).rx_frame_len_2);
    if (*msp).def_elem_len != 0 {
        temp_reg |= MSP_P1_ELEM_LEN_BITS((*protdesc).rx_elem_len_1);
        temp_reg |= MSP_P2_ELEM_LEN_BITS((*protdesc).rx_elem_len_2);
    } else {
        temp_reg |= MSP_P1_ELEM_LEN_BITS(data_size);
        temp_reg |= MSP_P2_ELEM_LEN_BITS(data_size);
    }

    temp_reg |= MSP_DATA_DELAY_BITS((*protdesc).rx_data_delay);
    temp_reg |= MSP_SET_ENDIANNES_BIT((*protdesc).rx_byte_order);
    temp_reg |= MSP_FSYNC_POL((*protdesc).rx_fsync_pol);
    temp_reg |= MSP_DATA_WORD_SWAP((*protdesc).rx_half_word_swap);
    temp_reg |= MSP_SET_COMPANDING_MODE((*protdesc).expansion_mode);
    temp_reg |= MSP_SET_FSYNC_IGNORE((*protdesc).frame_sync_ignore);

    writel(temp_reg, (*msp).registers.add(MSP_RCF));
}

unsafe fn configure_protocol(msp: *mut ux500_msp, config: *const ux500_msp_config) -> i32 {
    let protdesc: *const msp_protdesc;
    let data_size: u32;
    let mut temp_reg: u32 = 0;

    data_size = (*config).data_size;
    (*msp).def_elem_len = (*config).def_elem_len;
    if (*config).default_protdesc == 1 {
        if (*config).protocol >= MSP_INVALID_PROTOCOL {
            dev_err(
                (*msp).dev,
                b"%s: ERROR: Invalid protocol!\n\0".as_ptr(),
                b"configure_protocol\0".as_ptr(),
            );
            return -22; // -EINVAL
        }
        protdesc = &PROT_DESCS[(*config).protocol as usize] as *const _ as *const _;
    } else {
        protdesc = &(*config).protdesc;
    }

    if data_size < MSP_DATA_BITS_DEFAULT || data_size > MSP_DATA_BITS_32 {
        dev_err(
            (*msp).dev,
            b"%s: ERROR: Invalid data-size requested (data_size = %d)!\n\0".as_ptr(),
            b"configure_protocol\0".as_ptr(),
            data_size,
        );
        return -22; // -EINVAL
    }

    if (*config).direction & MSP_DIR_TX != 0 {
        set_prot_desc_tx(msp, protdesc, data_size);
    }
    if (*config).direction & MSP_DIR_RX != 0 {
        set_prot_desc_rx(msp, protdesc, data_size);
    }

    // The code below should not be separated.
    temp_reg = readl((*msp).registers.add(MSP_GCR)) & !TX_CLK_POL_RISING;
    temp_reg |= MSP_TX_CLKPOL_BIT(!(*protdesc).tx_clk_pol);
    writel(temp_reg, (*msp).registers.add(MSP_GCR));
    temp_reg = readl((*msp).registers.add(MSP_GCR)) & !RX_CLK_POL_RISING;
    temp_reg |= MSP_RX_CLKPOL_BIT((*protdesc).rx_clk_pol);
    writel(temp_reg, (*msp).registers.add(MSP_GCR));

    0
}

unsafe fn setup_bitclk(msp: *mut ux500_msp, config: *const ux500_msp_config) -> i32 {
    let reg_val_GCR: u32;
    let frame_per: u32;
    let sck_div: u32;
    let frame_width: u32;
    let mut temp_reg: u32 = 0;
    let protdesc: *const msp_protdesc;

    reg_val_GCR = readl((*msp).registers.add(MSP_GCR));
    writel(reg_val_GCR & !SRG_ENABLE, (*msp).registers.add(MSP_GCR));

    if (*config).default_protdesc != 0 {
        protdesc = &PROT_DESCS[(*config).protocol as usize] as *const _ as *const _;
    } else {
        protdesc = &(*config).protdesc;
    }

    match (*config).protocol {
        MSP_PCM_PROTOCOL | MSP_PCM_COMPAND_PROTOCOL => {
            frame_width = (*protdesc).frame_width;
            sck_div = (*config).f_inputclk / ((*config).frame_freq * (*protdesc).clocks_per_frame);
            frame_per = (*protdesc).frame_period;
        }
        MSP_I2S_PROTOCOL => {
            frame_width = (*protdesc).frame_width;
            sck_div = (*config).f_inputclk / ((*config).frame_freq * (*protdesc).clocks_per_frame);
            frame_per = (*protdesc).frame_period;
        }
        _ => {
            dev_err(
                (*msp).dev,
                b"%s: ERROR: Unknown protocol (%d)!\n\0".as_ptr(),
                b"setup_bitclk\0".as_ptr(),
                (*config).protocol,
            );
            return -22; // -EINVAL
        }
    }

    temp_reg = (sck_div.wrapping_sub(1)) & SCK_DIV_MASK;
    temp_reg |= FRAME_WIDTH_BITS(frame_width);
    temp_reg |= FRAME_PERIOD_BITS(frame_per);
    writel(temp_reg, (*msp).registers.add(MSP_SRG));

    (*msp).f_bitclk = (*config).f_inputclk / (sck_div + 1);

    // Enable bit-clock
    udelay(100);
    let reg_val_GCR = readl((*msp).registers.add(MSP_GCR));
    writel(reg_val_GCR | SRG_ENABLE, (*msp).registers.add(MSP_GCR));
    udelay(100);

    0
}

unsafe fn configure_multichannel(
    msp: *mut ux500_msp,
    config: *const ux500_msp_config,
) -> i32 {
    let protdesc: *const msp_protdesc;
    let mcfg: *const msp_multichannel_config;
    let reg_val_MCR: u32;

    if (*config).default_protdesc == 1 {
        if (*config).protocol >= MSP_INVALID_PROTOCOL {
            dev_err(
                (*msp).dev,
                b"%s: ERROR: Invalid protocol (%d)!\n\0".as_ptr(),
                b"configure_multichannel\0".as_ptr(),
                (*config).protocol,
            );
            return -22; // -EINVAL
        }
        protdesc = &PROT_DESCS[(*config).protocol as usize] as *const _ as *const _;
    } else {
        protdesc = &(*config).protdesc;
    }

    mcfg = &(*config).multichannel_config;
    if (*mcfg).tx_multichannel_enable != 0 {
        if (*protdesc).tx_phase_mode == MSP_SINGLE_PHASE {
            reg_val_MCR = readl((*msp).registers.add(MSP_MCR));
            writel(
                reg_val_MCR | (if (*mcfg).tx_multichannel_enable != 0 { 1 << TMCEN_BIT } else { 0 }),
                (*msp).registers.add(MSP_MCR),
            );
            writel((*mcfg).tx_channel_0_enable, (*msp).registers.add(MSP_TCE0));
            writel((*mcfg).tx_channel_1_enable, (*msp).registers.add(MSP_TCE1));
            writel((*mcfg).tx_channel_2_enable, (*msp).registers.add(MSP_TCE2));
            writel((*mcfg).tx_channel_3_enable, (*msp).registers.add(MSP_TCE3));
        } else {
            dev_err(
                (*msp).dev,
                b"%s: ERROR: Only single-phase supported (TX-mode: %d)!\n\0".as_ptr(),
                b"configure_multichannel\0".as_ptr(),
                (*protdesc).tx_phase_mode,
            );
            return -22; // -EINVAL
        }
    }
    if (*mcfg).rx_multichannel_enable != 0 {
        if (*protdesc).rx_phase_mode == MSP_SINGLE_PHASE {
            reg_val_MCR = readl((*msp).registers.add(MSP_MCR));
            writel(
                reg_val_MCR | (if (*mcfg).rx_multichannel_enable != 0 { 1 << RMCEN_BIT } else { 0 }),
                (*msp).registers.add(MSP_MCR),
            );
            writel((*mcfg).rx_channel_0_enable, (*msp).registers.add(MSP_RCE0));
            writel((*mcfg).rx_channel_1_enable, (*msp).registers.add(MSP_RCE1));
            writel((*mcfg).rx_channel_2_enable, (*msp).registers.add(MSP_RCE2));
            writel((*mcfg).rx_channel_3_enable, (*msp).registers.add(MSP_RCE3));
        } else {
            dev_err(
                (*msp).dev,
                b"%s: ERROR: Only single-phase supported (RX-mode: %d)!\n\0".as_ptr(),
                b"configure_multichannel\0".as_ptr(),
                (*protdesc).rx_phase_mode,
            );
            return -22; // -EINVAL
        }
        if (*mcfg).rx_comparison_enable_mode != 0 {
            reg_val_MCR = readl((*msp).registers.add(MSP_MCR));
            writel(
                reg_val_MCR | ((*mcfg).rx_comparison_enable_mode << RCMPM_BIT),
                (*msp).registers.add(MSP_MCR),
            );

            writel((*mcfg).comparison_mask, (*msp).registers.add(MSP_RCM));
            writel((*mcfg).comparison_value, (*msp).registers.add(MSP_RCV));
        }
    }

    0
}

unsafe fn enable_msp(msp: *mut ux500_msp, config: *const ux500_msp_config) -> i32 {
    let mut status: i32 = 0;
    let reg_val_DMACR: u32;
    let reg_val_GCR: u32;

    // Configure msp with protocol dependent settings
    configure_protocol(msp, config);
    setup_bitclk(msp, config);
    if (*config).multichannel_configured == 1 {
        status = configure_multichannel(msp, config);
        if status != 0 {
            dev_warn(
                (*msp).dev,
                b"%s: WARN: configure_multichannel failed (%d)!\n\0".as_ptr(),
                b"enable_msp\0".as_ptr(),
                status,
            );
        }
    }

    reg_val_DMACR = readl((*msp).registers.add(MSP_DMACR));
    let mut new_dmacr = reg_val_DMACR;
    if (*config).direction & MSP_DIR_RX != 0 {
        new_dmacr |= RX_DMA_ENABLE;
    }
    if (*config).direction & MSP_DIR_TX != 0 {
        new_dmacr |= TX_DMA_ENABLE;
    }
    writel(new_dmacr, (*msp).registers.add(MSP_DMACR));

    writel((*config).iodelay, (*msp).registers.add(MSP_IODLY));

    // Enable frame generation logic
    reg_val_GCR = readl((*msp).registers.add(MSP_GCR));
    writel(reg_val_GCR | FRAME_GEN_ENABLE, (*msp).registers.add(MSP_GCR));

    status
}

unsafe fn flush_fifo_rx(msp: *mut ux500_msp) {
    let reg_val_GCR: u32;
    let mut reg_val_FLR: u32;
    let mut limit: u32 = 32;

    reg_val_GCR = readl((*msp).registers.add(MSP_GCR));
    writel(reg_val_GCR | RX_ENABLE, (*msp).registers.add(MSP_GCR));

    reg_val_FLR = readl((*msp).registers.add(MSP_FLR));
    while (reg_val_FLR & RX_FIFO_EMPTY) == 0 && limit != 0 {
        readl((*msp).registers.add(MSP_DR));
        reg_val_FLR = readl((*msp).registers.add(MSP_FLR));
        limit -= 1;
    }

    writel(reg_val_GCR, (*msp).registers.add(MSP_GCR));
}

unsafe fn flush_fifo_tx(msp: *mut ux500_msp) {
    let reg_val_GCR: u32;
    let mut reg_val_FLR: u32;
    let mut limit: u32 = 32;

    reg_val_GCR = readl((*msp).registers.add(MSP_GCR));
    writel(reg_val_GCR | TX_ENABLE, (*msp).registers.add(MSP_GCR));
    writel(
        MSP_ITCR_ITEN | MSP_ITCR_TESTFIFO,
        (*msp).registers.add(MSP_ITCR),
    );

    reg_val_FLR = readl((*msp).registers.add(MSP_FLR));
    while (reg_val_FLR & TX_FIFO_EMPTY) == 0 && limit != 0 {
        readl((*msp).registers.add(MSP_TSTDR));
        reg_val_FLR = readl((*msp).registers.add(MSP_FLR));
        limit -= 1;
    }
    writel(0x0, (*msp).registers.add(MSP_ITCR));
    writel(reg_val_GCR, (*msp).registers.add(MSP_GCR));
}

#[no_mangle]
pub unsafe extern "C" fn ux500_msp_i2s_open(
    msp: *mut ux500_msp,
    config: *const ux500_msp_config,
) -> i32 {
    let mut old_reg: u32;
    let mut new_reg: u32;
    let mask: u32;
    let res: i32;
    let tx_sel: u32;
    let rx_sel: u32;
    let tx_busy: u32;
    let rx_busy: u32;

    if in_interrupt() != 0 {
        dev_err(
            (*msp).dev,
            b"%s: ERROR: Open called in interrupt context!\n\0".as_ptr(),
            b"ux500_msp_i2s_open\0".as_ptr(),
        );
        return -1;
    }

    tx_sel = if ((*config).direction & MSP_DIR_TX) > 0 { 1 } else { 0 };
    rx_sel = if ((*config).direction & MSP_DIR_RX) > 0 { 1 } else { 0 };
    if tx_sel == 0 && rx_sel == 0 {
        dev_err(
            (*msp).dev,
            b"%s: Error: No direction selected!\n\0".as_ptr(),
            b"ux500_msp_i2s_open\0".as_ptr(),
        );
        return -22; // -EINVAL
    }

    tx_busy = if ((*msp).dir_busy & MSP_DIR_TX) > 0 { 1 } else { 0 };
    rx_busy = if ((*msp).dir_busy & MSP_DIR_RX) > 0 { 1 } else { 0 };
    if tx_busy != 0 && tx_sel != 0 {
        dev_err(
            (*msp).dev,
            b"%s: Error: TX is in use!\n\0".as_ptr(),
            b"ux500_msp_i2s_open\0".as_ptr(),
        );
        return -16; // -EBUSY
    }
    if rx_busy != 0 && rx_sel != 0 {
        dev_err(
            (*msp).dev,
            b"%s: Error: RX is in use!\n\0".as_ptr(),
            b"ux500_msp_i2s_open\0".as_ptr(),
        );
        return -16; // -EBUSY
    }

    (*msp).dir_busy |= (if tx_sel != 0 { MSP_DIR_TX } else { 0 }) | (if rx_sel != 0 { MSP_DIR_RX } else { 0 });

    // First do the global config register
    mask = RX_CLK_SEL_MASK
        | TX_CLK_SEL_MASK
        | RX_FSYNC_MASK
        | TX_FSYNC_MASK
        | RX_SYNC_SEL_MASK
        | TX_SYNC_SEL_MASK
        | RX_FIFO_ENABLE_MASK
        | TX_FIFO_ENABLE_MASK
        | SRG_CLK_SEL_MASK
        | LOOPBACK_MASK
        | TX_EXTRA_DELAY_MASK;

    new_reg = (*config).tx_clk_sel
        | (*config).rx_clk_sel
        | (*config).rx_fsync_pol
        | (*config).tx_fsync_pol
        | (*config).rx_fsync_sel
        | (*config).tx_fsync_sel
        | (*config).rx_fifo_config
        | (*config).tx_fifo_config
        | (*config).srg_clk_sel
        | (*config).loopback_enable
        | (*config).tx_data_enable;

    old_reg = readl((*msp).registers.add(MSP_GCR));
    old_reg &= !mask;
    new_reg |= old_reg;
    writel(new_reg, (*msp).registers.add(MSP_GCR));

    res = enable_msp(msp, config);
    if res < 0 {
        dev_err(
            (*msp).dev,
            b"%s: ERROR: enable_msp failed (%d)!\n\0".as_ptr(),
            b"ux500_msp_i2s_open\0".as_ptr(),
            res,
        );
        return -16; // -EBUSY
    }
    if ((*config).loopback_enable & 0x80) != 0 {
        (*msp).loopback_enable = 1;
    }

    // Flush FIFOs
    flush_fifo_tx(msp);
    flush_fifo_rx(msp);

    (*msp).msp_state = MSP_STATE_CONFIGURED;
    0
}

unsafe fn disable_msp_rx(msp: *mut ux500_msp) {
    let reg_val_GCR: u32;
    let reg_val_DMACR: u32;
    let reg_val_IMSC: u32;

    reg_val_GCR = readl((*msp).registers.add(MSP_GCR));
    writel(reg_val_GCR & !RX_ENABLE, (*msp).registers.add(MSP_GCR));
    reg_val_DMACR = readl((*msp).registers.add(MSP_DMACR));
    writel(reg_val_DMACR & !RX_DMA_ENABLE, (*msp).registers.add(MSP_DMACR));
    reg_val_IMSC = readl((*msp).registers.add(MSP_IMSC));
    writel(
        reg_val_IMSC & !(RX_SERVICE_INT | RX_OVERRUN_ERROR_INT),
        (*msp).registers.add(MSP_IMSC),
    );

    (*msp).dir_busy &= !MSP_DIR_RX;
}

unsafe fn disable_msp_tx(msp: *mut ux500_msp) {
    let reg_val_GCR: u32;
    let reg_val_DMACR: u32;
    let reg_val_IMSC: u32;

    reg_val_GCR = readl((*msp).registers.add(MSP_GCR));
    writel(reg_val_GCR & !TX_ENABLE, (*msp).registers.add(MSP_GCR));
    reg_val_DMACR = readl((*msp).registers.add(MSP_DMACR));
    writel(reg_val_DMACR & !TX_DMA_ENABLE, (*msp).registers.add(MSP_DMACR));
    reg_val_IMSC = readl((*msp).registers.add(MSP_IMSC));
    writel(
        reg_val_IMSC & !(TX_SERVICE_INT | TX_UNDERRUN_ERR_INT),
        (*msp).registers.add(MSP_IMSC),
    );

    (*msp).dir_busy &= !MSP_DIR_TX;
}

unsafe fn disable_msp(msp: *mut ux500_msp, dir: u32) -> i32 {
    let reg_val_GCR: u32;
    let disable_tx: u32;
    let disable_rx: u32;

    reg_val_GCR = readl((*msp).registers.add(MSP_GCR));
    disable_tx = dir & MSP_DIR_TX;
    disable_rx = dir & MSP_DIR_TX;
    if disable_tx != 0 && disable_rx != 0 {
        reg_val_GCR = readl((*msp).registers.add(MSP_GCR));
        writel(reg_val_GCR | LOOPBACK_MASK, (*msp).registers.add(MSP_GCR));

        // Flush TX-FIFO
        flush_fifo_tx(msp);

        // Disable TX-channel
        writel(
            readl((*msp).registers.add(MSP_GCR)) & !TX_ENABLE,
            (*msp).registers.add(MSP_GCR),
        );

        // Flush RX-FIFO
        flush_fifo_rx(msp);

        // Disable Loopback and Receive channel
        writel(
            readl((*msp).registers.add(MSP_GCR)) & !(RX_ENABLE | LOOPBACK_MASK),
            (*msp).registers.add(MSP_GCR),
        );

        disable_msp_tx(msp);
        disable_msp_rx(msp);
    } else if disable_tx != 0 {
        disable_msp_tx(msp);
    } else if disable_rx != 0 {
        disable_msp_rx(msp);
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn ux500_msp_i2s_trigger(
    msp: *mut ux500_msp,
    cmd: i32,
    direction: i32,
) -> i32 {
    let reg_val_GCR: u32;
    let enable_bit: u32;

    if (*msp).msp_state == MSP_STATE_IDLE {
        dev_err(
            (*msp).dev,
            b"%s: ERROR: MSP is not configured!\n\0".as_ptr(),
            b"ux500_msp_i2s_trigger\0".as_ptr(),
        );
        return -22; // -EINVAL
    }

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            enable_bit = if direction == SNDRV_PCM_STREAM_PLAYBACK {
                TX_ENABLE
            } else {
                RX_ENABLE
            };
            reg_val_GCR = readl((*msp).registers.add(MSP_GCR));
            writel(reg_val_GCR | enable_bit, (*msp).registers.add(MSP_GCR));
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            if direction == SNDRV_PCM_STREAM_PLAYBACK {
                disable_msp_tx(msp);
            } else {
                disable_msp_rx(msp);
            }
        }
        _ => {
            return -22; // -EINVAL
        }
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn ux500_msp_i2s_close(msp: *mut ux500_msp, dir: u32) -> i32 {
    let mut status: i32 = 0;

    dev_dbg(
        (*msp).dev,
        b"%s: Enter (dir = 0x%01x).\n\0".as_ptr(),
        b"ux500_msp_i2s_close\0".as_ptr(),
        dir,
    );

    status = disable_msp(msp, dir);
    if (*msp).dir_busy == 0 {
        // disable sample rate and frame generators
        (*msp).msp_state = MSP_STATE_IDLE;
        writel(
            readl((*msp).registers.add(MSP_GCR)) & !(FRAME_GEN_ENABLE | SRG_ENABLE),
            (*msp).registers.add(MSP_GCR),
        );

        writel(0, (*msp).registers.add(MSP_GCR));
        writel(0, (*msp).registers.add(MSP_TCF));
        writel(0, (*msp).registers.add(MSP_RCF));
        writel(0, (*msp).registers.add(MSP_DMACR));
        writel(0, (*msp).registers.add(MSP_SRG));
        writel(0, (*msp).registers.add(MSP_MCR));
        writel(0, (*msp).registers.add(MSP_RCM));
        writel(0, (*msp).registers.add(MSP_RCV));
        writel(0, (*msp).registers.add(MSP_TCE0));
        writel(0, (*msp).registers.add(MSP_TCE1));
        writel(0, (*msp).registers.add(MSP_TCE2));
        writel(0, (*msp).registers.add(MSP_TCE3));
        writel(0, (*msp).registers.add(MSP_RCE0));
        writel(0, (*msp).registers.add(MSP_RCE1));
        writel(0, (*msp).registers.add(MSP_RCE2));
        writel(0, (*msp).registers.add(MSP_RCE3));
    }

    status
}

#[no_mangle]
pub unsafe extern "C" fn ux500_msp_i2s_init_msp(
    pdev: *mut platform_device,
    msp_p: *mut *mut ux500_msp,
) -> i32 {
    let mut res: *mut resource = std::ptr::null_mut();
    let msp: *mut ux500_msp;

    *msp_p = devm_kzalloc(&(*pdev).dev, std::mem::size_of::<ux500_msp>(), GFP_KERNEL)
        as *mut ux500_msp;
    msp = *msp_p;
    if msp.is_null() {
        return -12; // -ENOMEM
    }

    (*msp).dev = &(*pdev).dev;

    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if res.is_null() {
        dev_err(
            &(*pdev).dev,
            b"%s: ERROR: Unable to get resource!\n\0".as_ptr(),
            b"ux500_msp_i2s_init_msp\0".as_ptr(),
        );
        return -12; // -ENOMEM
    }

    (*msp).tx_rx_addr = (*res).start + MSP_DR;
    (*msp).registers = devm_ioremap(&(*pdev).dev, (*res).start, (*res).end - (*res).start + 1);
    if (*msp).registers.is_null() {
        dev_err(
            &(*pdev).dev,
            b"%s: ERROR: ioremap failed!\n\0".as_ptr(),
            b"ux500_msp_i2s_init_msp\0".as_ptr(),
        );
        return -12; // -ENOMEM
    }

    (*msp).msp_state = MSP_STATE_IDLE;
    (*msp).loopback_enable = 0;

    0
}

#[no_mangle]
pub unsafe extern "C" fn ux500_msp_i2s_cleanup_msp(pdev: *mut platform_device, msp: *mut ux500_msp) {
    dev_dbg(
        (*msp).dev,
        b"%s: Enter (id = %d).\n\0".as_ptr(),
        b"ux500_msp_i2s_cleanup_msp\0".as_ptr(),
        (*msp).id,
    );
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
