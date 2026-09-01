// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/sound/soc/m8m/hi6210_i2s.c - I2S IP driver
 *
 * Copyright (C) 2015 Linaro, Ltd
 * Author: Andy Green <andy.green@linaro.org>
 *
 * This driver only deals with S2 interface (BT)
 */

/* Dependencies from Linux and "hi6210-i2s.h" are expected to be supplied by
 * the surrounding kernel Rust binding environment.
 */

#[repr(C)]
pub struct hi6210_i2s {
    pub dev: *mut device,
    pub rc: *mut reset_control,
    pub clk: [*mut clk; 8],
    pub clocks: core::ffi::c_int,
    pub dai: snd_soc_dai_driver,
    pub base: *mut core::ffi::c_void,
    pub sysctrl: *mut regmap,
    pub base_phys: phys_addr_t,
    pub dma_data: [snd_dmaengine_dai_dma_data; 2],
    pub clk_rate: core::ffi::c_int,
    pub lock: spinlock_t,
    pub rate: core::ffi::c_int,
    pub format: core::ffi::c_int,
    pub bits: u8,
    pub channels: u8,
    pub id: u8,
    pub channel_length: u8,
    pub use_: u8,
    /* C bitfields: u32 master:1; u32 status:1; */
    pub master: u32,
    pub status: u32,
}

pub const SC_PERIPH_CLKEN1: u32 = 0x210;
pub const SC_PERIPH_CLKDIS1: u32 = 0x214;

pub const SC_PERIPH_CLKEN3: u32 = 0x230;
pub const SC_PERIPH_CLKDIS3: u32 = 0x234;

pub const SC_PERIPH_CLKEN12: u32 = 0x270;
pub const SC_PERIPH_CLKDIS12: u32 = 0x274;

pub const SC_PERIPH_RSTEN1: u32 = 0x310;
pub const SC_PERIPH_RSTDIS1: u32 = 0x314;
pub const SC_PERIPH_RSTSTAT1: u32 = 0x318;

pub const SC_PERIPH_RSTEN2: u32 = 0x320;
pub const SC_PERIPH_RSTDIS2: u32 = 0x324;
pub const SC_PERIPH_RSTSTAT2: u32 = 0x328;

pub const SOC_PMCTRL_BBPPLLALIAS: u32 = 0x48;

pub const CLK_DACODEC: usize = 0;
pub const CLK_I2S_BASE: usize = 1;

#[inline]
unsafe fn hi6210_write_reg(i2s: *mut hi6210_i2s, reg: core::ffi::c_int, val: u32) {
    unsafe {
        writel(val, ((*i2s).base as *mut u8).add(reg as usize) as *mut core::ffi::c_void);
    }
}

#[inline]
unsafe fn hi6210_read_reg(i2s: *mut hi6210_i2s, reg: core::ffi::c_int) -> u32 {
    unsafe { readl(((*i2s).base as *mut u8).add(reg as usize) as *const core::ffi::c_void) }
}

unsafe fn hi6210_i2s_startup(
    substream: *mut snd_pcm_substream,
    cpu_dai: *mut snd_soc_dai,
) -> core::ffi::c_int {
    unsafe {
        let i2s = dev_get_drvdata((*cpu_dai).dev) as *mut hi6210_i2s;
        let mut ret: core::ffi::c_int;
        let mut n: core::ffi::c_int;
        let mut val: u32 = 0;

        /* deassert reset on ABB */
        regmap_read((*i2s).sysctrl, SC_PERIPH_RSTSTAT2, &mut val);
        if val & BIT(4) != 0 {
            regmap_write((*i2s).sysctrl, SC_PERIPH_RSTDIS2, BIT(4));
        }

        n = 0;
        while n < (*i2s).clocks {
            ret = clk_prepare_enable((*i2s).clk[n as usize]);
            if ret != 0 {
                goto_err_unprepare_clk(i2s, n);
                return ret;
            }
            n += 1;
        }

        ret = clk_set_rate((*i2s).clk[CLK_I2S_BASE], 49152000);
        if ret != 0 {
            dev_err(
                (*i2s).dev,
                c"%s: setting 49.152MHz base rate failed %d\n".as_ptr(),
                c"hi6210_i2s_startup".as_ptr(),
                ret,
            );
            goto_err_unprepare_clk(i2s, n);
            return ret;
        }

        /* enable clock before frequency division */
        regmap_write((*i2s).sysctrl, SC_PERIPH_CLKEN12, BIT(9));

        /* enable codec working clock / == "codec bus clock" */
        regmap_write((*i2s).sysctrl, SC_PERIPH_CLKEN1, BIT(5));

        /* deassert reset on codec / interface clock / working clock */
        regmap_write((*i2s).sysctrl, SC_PERIPH_RSTEN1, BIT(5));
        regmap_write((*i2s).sysctrl, SC_PERIPH_RSTDIS1, BIT(5));

        /* not interested in i2s irqs */
        val = hi6210_read_reg(i2s, HII2S_CODEC_IRQ_MASK);
        val |= 0x3f;
        hi6210_write_reg(i2s, HII2S_CODEC_IRQ_MASK, val);

        /* reset the stereo downlink fifo */
        val = hi6210_read_reg(i2s, HII2S_APB_AFIFO_CFG_1);
        val |= BIT(5) | BIT(4);
        hi6210_write_reg(i2s, HII2S_APB_AFIFO_CFG_1, val);

        val = hi6210_read_reg(i2s, HII2S_APB_AFIFO_CFG_1);
        val &= !(BIT(5) | BIT(4));
        hi6210_write_reg(i2s, HII2S_APB_AFIFO_CFG_1, val);

        val = hi6210_read_reg(i2s, HII2S_SW_RST_N);
        val &= !(HII2S_SW_RST_N__ST_DL_WORDLEN_MASK << HII2S_SW_RST_N__ST_DL_WORDLEN_SHIFT);
        val |= HII2S_BITS_16 << HII2S_SW_RST_N__ST_DL_WORDLEN_SHIFT;
        hi6210_write_reg(i2s, HII2S_SW_RST_N, val);

        val = hi6210_read_reg(i2s, HII2S_MISC_CFG);
        /* mux 11/12 = APB not i2s */
        val &= !HII2S_MISC_CFG__ST_DL_TEST_SEL;
        /* BT R ch  0 = mixer op of DACR ch */
        val &= !HII2S_MISC_CFG__S2_DOUT_RIGHT_SEL;
        val &= !HII2S_MISC_CFG__S2_DOUT_TEST_SEL;

        val |= HII2S_MISC_CFG__S2_DOUT_RIGHT_SEL;
        /* BT L ch = 1 = mux 7 = "mixer output of DACL */
        val |= HII2S_MISC_CFG__S2_DOUT_TEST_SEL;
        hi6210_write_reg(i2s, HII2S_MISC_CFG, val);

        val = hi6210_read_reg(i2s, HII2S_SW_RST_N);
        val |= HII2S_SW_RST_N__SW_RST_N;
        hi6210_write_reg(i2s, HII2S_SW_RST_N, val);

        let _ = substream;
        0
    }
}

unsafe fn goto_err_unprepare_clk(i2s: *mut hi6210_i2s, mut n: core::ffi::c_int) {
    unsafe {
        while n != 0 {
            n -= 1;
            clk_disable_unprepare((*i2s).clk[n as usize]);
        }
    }
}

unsafe fn hi6210_i2s_shutdown(
    substream: *mut snd_pcm_substream,
    cpu_dai: *mut snd_soc_dai,
) {
    unsafe {
        let i2s = dev_get_drvdata((*cpu_dai).dev) as *mut hi6210_i2s;
        let mut n: core::ffi::c_int = 0;

        while n < (*i2s).clocks {
            clk_disable_unprepare((*i2s).clk[n as usize]);
            n += 1;
        }

        regmap_write((*i2s).sysctrl, SC_PERIPH_RSTEN1, BIT(5));
        let _ = substream;
    }
}

unsafe fn hi6210_i2s_txctrl(cpu_dai: *mut snd_soc_dai, on: core::ffi::c_int) {
    unsafe {
        let i2s = dev_get_drvdata((*cpu_dai).dev) as *mut hi6210_i2s;
        let mut val: u32;

        let _guard = guard_spinlock(&mut (*i2s).lock);
        if on != 0 {
            /* enable S2 TX */
            val = hi6210_read_reg(i2s, HII2S_I2S_CFG);
            val |= HII2S_I2S_CFG__S2_IF_TX_EN;
            hi6210_write_reg(i2s, HII2S_I2S_CFG, val);
        } else {
            /* disable S2 TX */
            val = hi6210_read_reg(i2s, HII2S_I2S_CFG);
            val &= !HII2S_I2S_CFG__S2_IF_TX_EN;
            hi6210_write_reg(i2s, HII2S_I2S_CFG, val);
        }
    }
}

unsafe fn hi6210_i2s_rxctrl(cpu_dai: *mut snd_soc_dai, on: core::ffi::c_int) {
    unsafe {
        let i2s = dev_get_drvdata((*cpu_dai).dev) as *mut hi6210_i2s;
        let mut val: u32;

        let _guard = guard_spinlock(&mut (*i2s).lock);
        if on != 0 {
            val = hi6210_read_reg(i2s, HII2S_I2S_CFG);
            val |= HII2S_I2S_CFG__S2_IF_RX_EN;
            hi6210_write_reg(i2s, HII2S_I2S_CFG, val);
        } else {
            val = hi6210_read_reg(i2s, HII2S_I2S_CFG);
            val &= !HII2S_I2S_CFG__S2_IF_RX_EN;
            hi6210_write_reg(i2s, HII2S_I2S_CFG, val);
        }
    }
}

unsafe fn hi6210_i2s_set_fmt(
    cpu_dai: *mut snd_soc_dai,
    fmt: core::ffi::c_uint,
) -> core::ffi::c_int {
    unsafe {
        let i2s = dev_get_drvdata((*cpu_dai).dev) as *mut hi6210_i2s;

        /*
         * We don't actually set the hardware until the hw_params
         * call, but we need to validate the user input here.
         */
        match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
            SND_SOC_DAIFMT_BC_FC | SND_SOC_DAIFMT_BP_FP => {}
            _ => return -EINVAL,
        }

        match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
            SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_LEFT_J | SND_SOC_DAIFMT_RIGHT_J => {}
            _ => return -EINVAL,
        }

        (*i2s).format = fmt as core::ffi::c_int;
        (*i2s).master =
            ((fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) == SND_SOC_DAIFMT_BP_FP) as u32;

        0
    }
}

unsafe fn hi6210_i2s_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    cpu_dai: *mut snd_soc_dai,
) -> core::ffi::c_int {
    unsafe {
        let i2s = dev_get_drvdata((*cpu_dai).dev) as *mut hi6210_i2s;
        let mut bits: u32 = 0;
        let mut rate: u32 = 0;
        let mut signed_data: u32 = 0;
        let mut fmt: u32 = 0;
        let mut val: u32;
        let dma_data: *mut snd_dmaengine_dai_dma_data;

        match params_format(params) {
            SNDRV_PCM_FORMAT_U16_LE => {
                signed_data = HII2S_I2S_CFG__S2_CODEC_DATA_FORMAT;
                bits = HII2S_BITS_16;
            }
            SNDRV_PCM_FORMAT_S16_LE => {
                bits = HII2S_BITS_16;
            }
            SNDRV_PCM_FORMAT_U24_LE => {
                signed_data = HII2S_I2S_CFG__S2_CODEC_DATA_FORMAT;
                bits = HII2S_BITS_24;
            }
            SNDRV_PCM_FORMAT_S24_LE => {
                bits = HII2S_BITS_24;
            }
            _ => {
                dev_err((*cpu_dai).dev, c"Bad format\n".as_ptr());
                return -EINVAL;
            }
        }

        match params_rate(params) {
            8000 => rate = HII2S_FS_RATE_8KHZ,
            16000 => rate = HII2S_FS_RATE_16KHZ,
            32000 => rate = HII2S_FS_RATE_32KHZ,
            48000 => rate = HII2S_FS_RATE_48KHZ,
            96000 => rate = HII2S_FS_RATE_96KHZ,
            192000 => rate = HII2S_FS_RATE_192KHZ,
            _ => {
                dev_err((*cpu_dai).dev, c"Bad rate: %d\n".as_ptr(), params_rate(params));
                return -EINVAL;
            }
        }

        if params_channels(params) == 0 {
            dev_err((*cpu_dai).dev, c"Bad channels\n".as_ptr());
            return -EINVAL;
        }

        dma_data = snd_soc_dai_get_dma_data(cpu_dai, substream);

        match bits {
            HII2S_BITS_24 => {
                (*i2s).bits = 32;
                (*dma_data).addr_width = 3;
            }
            _ => {
                (*i2s).bits = 16;
                (*dma_data).addr_width = 2;
            }
        }
        (*i2s).rate = params_rate(params);
        (*i2s).channels = params_channels(params) as u8;
        (*i2s).channel_length = (*i2s).channels.wrapping_mul((*i2s).bits);

        val = hi6210_read_reg(i2s, HII2S_ST_DL_FIFO_TH_CFG);
        val &= !((HII2S_ST_DL_FIFO_TH_CFG__ST_DL_R_AEMPTY_MASK
            << HII2S_ST_DL_FIFO_TH_CFG__ST_DL_R_AEMPTY_SHIFT)
            | (HII2S_ST_DL_FIFO_TH_CFG__ST_DL_R_AFULL_MASK
                << HII2S_ST_DL_FIFO_TH_CFG__ST_DL_R_AFULL_SHIFT)
            | (HII2S_ST_DL_FIFO_TH_CFG__ST_DL_L_AEMPTY_MASK
                << HII2S_ST_DL_FIFO_TH_CFG__ST_DL_L_AEMPTY_SHIFT)
            | (HII2S_ST_DL_FIFO_TH_CFG__ST_DL_L_AFULL_MASK
                << HII2S_ST_DL_FIFO_TH_CFG__ST_DL_L_AFULL_SHIFT));
        val |= (16 << HII2S_ST_DL_FIFO_TH_CFG__ST_DL_R_AEMPTY_SHIFT)
            | (30 << HII2S_ST_DL_FIFO_TH_CFG__ST_DL_R_AFULL_SHIFT)
            | (16 << HII2S_ST_DL_FIFO_TH_CFG__ST_DL_L_AEMPTY_SHIFT)
            | (30 << HII2S_ST_DL_FIFO_TH_CFG__ST_DL_L_AFULL_SHIFT);
        hi6210_write_reg(i2s, HII2S_ST_DL_FIFO_TH_CFG, val);

        val = hi6210_read_reg(i2s, HII2S_IF_CLK_EN_CFG);
        val |= BIT(19)
            | BIT(18)
            | BIT(17)
            | HII2S_IF_CLK_EN_CFG__S2_IF_CLK_EN
            | HII2S_IF_CLK_EN_CFG__S2_OL_MIXER_EN
            | HII2S_IF_CLK_EN_CFG__S2_OL_SRC_EN
            | HII2S_IF_CLK_EN_CFG__ST_DL_R_EN
            | HII2S_IF_CLK_EN_CFG__ST_DL_L_EN;
        hi6210_write_reg(i2s, HII2S_IF_CLK_EN_CFG, val);

        val = hi6210_read_reg(i2s, HII2S_DIG_FILTER_CLK_EN_CFG);
        val &= !(HII2S_DIG_FILTER_CLK_EN_CFG__DACR_SDM_EN
            | HII2S_DIG_FILTER_CLK_EN_CFG__DACR_HBF2I_EN
            | HII2S_DIG_FILTER_CLK_EN_CFG__DACR_AGC_EN
            | HII2S_DIG_FILTER_CLK_EN_CFG__DACL_SDM_EN
            | HII2S_DIG_FILTER_CLK_EN_CFG__DACL_HBF2I_EN
            | HII2S_DIG_FILTER_CLK_EN_CFG__DACL_AGC_EN);
        val |= HII2S_DIG_FILTER_CLK_EN_CFG__DACR_MIXER_EN
            | HII2S_DIG_FILTER_CLK_EN_CFG__DACL_MIXER_EN;
        hi6210_write_reg(i2s, HII2S_DIG_FILTER_CLK_EN_CFG, val);

        val = hi6210_read_reg(i2s, HII2S_DIG_FILTER_MODULE_CFG);
        val &= !(HII2S_DIG_FILTER_MODULE_CFG__DACR_MIXER_IN2_MUTE
            | HII2S_DIG_FILTER_MODULE_CFG__DACL_MIXER_IN2_MUTE);
        hi6210_write_reg(i2s, HII2S_DIG_FILTER_MODULE_CFG, val);

        val = hi6210_read_reg(i2s, HII2S_MUX_TOP_MODULE_CFG);
        val &= !(HII2S_MUX_TOP_MODULE_CFG__S2_OL_MIXER_IN1_MUTE
            | HII2S_MUX_TOP_MODULE_CFG__S2_OL_MIXER_IN2_MUTE
            | HII2S_MUX_TOP_MODULE_CFG__VOICE_DLINK_MIXER_IN1_MUTE
            | HII2S_MUX_TOP_MODULE_CFG__VOICE_DLINK_MIXER_IN2_MUTE);
        hi6210_write_reg(i2s, HII2S_MUX_TOP_MODULE_CFG, val);

        match ((*i2s).format as u32) & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
            SND_SOC_DAIFMT_BC_FC => {
                (*i2s).master = 0;
                val = hi6210_read_reg(i2s, HII2S_I2S_CFG);
                val |= HII2S_I2S_CFG__S2_MST_SLV;
                hi6210_write_reg(i2s, HII2S_I2S_CFG, val);
            }
            SND_SOC_DAIFMT_BP_FP => {
                (*i2s).master = 1;
                val = hi6210_read_reg(i2s, HII2S_I2S_CFG);
                val &= !HII2S_I2S_CFG__S2_MST_SLV;
                hi6210_write_reg(i2s, HII2S_I2S_CFG, val);
            }
            _ => {
                WARN_ONCE(
                    1,
                    c"Invalid i2s->fmt CLOCK_PROVIDER_MASK. This shouldn't happen\n".as_ptr(),
                );
                return -EINVAL;
            }
        }

        match ((*i2s).format as u32) & SND_SOC_DAIFMT_FORMAT_MASK {
            SND_SOC_DAIFMT_I2S => fmt = HII2S_FORMAT_I2S,
            SND_SOC_DAIFMT_LEFT_J => fmt = HII2S_FORMAT_LEFT_JUST,
            SND_SOC_DAIFMT_RIGHT_J => fmt = HII2S_FORMAT_RIGHT_JUST,
            _ => {
                WARN_ONCE(
                    1,
                    c"Invalid i2s->fmt FORMAT_MASK. This shouldn't happen\n".as_ptr(),
                );
                return -EINVAL;
            }
        }

        val = hi6210_read_reg(i2s, HII2S_I2S_CFG);
        val &= !(HII2S_I2S_CFG__S2_FUNC_MODE_MASK << HII2S_I2S_CFG__S2_FUNC_MODE_SHIFT);
        val |= fmt << HII2S_I2S_CFG__S2_FUNC_MODE_SHIFT;
        hi6210_write_reg(i2s, HII2S_I2S_CFG, val);

        val = hi6210_read_reg(i2s, HII2S_CLK_SEL);
        val &= !(HII2S_CLK_SEL__I2S_BT_FM_SEL | HII2S_CLK_SEL__EXT_12_288MHZ_SEL);
        hi6210_write_reg(i2s, HII2S_CLK_SEL, val);

        (*dma_data).maxburst = 2;

        if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
            (*dma_data).addr = (*i2s).base_phys + HII2S_ST_DL_CHANNEL as phys_addr_t;
        } else {
            (*dma_data).addr = (*i2s).base_phys + HII2S_STEREO_UPLINK_CHANNEL as phys_addr_t;
        }

        match (*i2s).channels {
            1 => {
                val = hi6210_read_reg(i2s, HII2S_I2S_CFG);
                val |= HII2S_I2S_CFG__S2_FRAME_MODE;
                hi6210_write_reg(i2s, HII2S_I2S_CFG, val);
            }
            _ => {
                val = hi6210_read_reg(i2s, HII2S_I2S_CFG);
                val &= !HII2S_I2S_CFG__S2_FRAME_MODE;
                hi6210_write_reg(i2s, HII2S_I2S_CFG, val);
            }
        }

        /* clear loopback, set signed type and word length */
        val = hi6210_read_reg(i2s, HII2S_I2S_CFG);
        val &= !HII2S_I2S_CFG__S2_CODEC_DATA_FORMAT;
        val &= !(HII2S_I2S_CFG__S2_CODEC_IO_WORDLENGTH_MASK
            << HII2S_I2S_CFG__S2_CODEC_IO_WORDLENGTH_SHIFT);
        val &= !(HII2S_I2S_CFG__S2_DIRECT_LOOP_MASK << HII2S_I2S_CFG__S2_DIRECT_LOOP_SHIFT);
        val |= signed_data;
        val |= bits << HII2S_I2S_CFG__S2_CODEC_IO_WORDLENGTH_SHIFT;
        hi6210_write_reg(i2s, HII2S_I2S_CFG, val);

        if (*i2s).master == 0 {
            return 0;
        }

        /* set DAC and related units to correct rate */
        val = hi6210_read_reg(i2s, HII2S_FS_CFG);
        val &= !(HII2S_FS_CFG__FS_S2_MASK << HII2S_FS_CFG__FS_S2_SHIFT);
        val &= !(HII2S_FS_CFG__FS_DACLR_MASK << HII2S_FS_CFG__FS_DACLR_SHIFT);
        val &= !(HII2S_FS_CFG__FS_ST_DL_R_MASK << HII2S_FS_CFG__FS_ST_DL_R_SHIFT);
        val &= !(HII2S_FS_CFG__FS_ST_DL_L_MASK << HII2S_FS_CFG__FS_ST_DL_L_SHIFT);
        val |= rate << HII2S_FS_CFG__FS_S2_SHIFT;
        val |= rate << HII2S_FS_CFG__FS_DACLR_SHIFT;
        val |= rate << HII2S_FS_CFG__FS_ST_DL_R_SHIFT;
        val |= rate << HII2S_FS_CFG__FS_ST_DL_L_SHIFT;
        hi6210_write_reg(i2s, HII2S_FS_CFG, val);

        0
    }
}

unsafe fn hi6210_i2s_trigger(
    substream: *mut snd_pcm_substream,
    cmd: core::ffi::c_int,
    cpu_dai: *mut snd_soc_dai,
) -> core::ffi::c_int {
    unsafe {
        pr_debug(c"%s\n".as_ptr(), c"hi6210_i2s_trigger".as_ptr());
        match cmd {
            SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
                if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
                    hi6210_i2s_rxctrl(cpu_dai, 1);
                } else {
                    hi6210_i2s_txctrl(cpu_dai, 1);
                }
            }
            SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
                if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
                    hi6210_i2s_rxctrl(cpu_dai, 0);
                } else {
                    hi6210_i2s_txctrl(cpu_dai, 0);
                }
            }
            _ => {
                dev_err((*cpu_dai).dev, c"unknown cmd\n".as_ptr());
                return -EINVAL;
            }
        }
        0
    }
}

unsafe fn hi6210_i2s_dai_probe(dai: *mut snd_soc_dai) -> core::ffi::c_int {
    unsafe {
        let i2s = snd_soc_dai_get_drvdata(dai) as *mut hi6210_i2s;

        snd_soc_dai_init_dma_data(
            dai,
            &mut (*i2s).dma_data[SNDRV_PCM_STREAM_PLAYBACK as usize],
            &mut (*i2s).dma_data[SNDRV_PCM_STREAM_CAPTURE as usize],
        );

        0
    }
}

static hi6210_i2s_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(hi6210_i2s_dai_probe),
    trigger: Some(hi6210_i2s_trigger),
    hw_params: Some(hi6210_i2s_hw_params),
    set_fmt: Some(hi6210_i2s_set_fmt),
    startup: Some(hi6210_i2s_startup),
    shutdown: Some(hi6210_i2s_shutdown),
};

static hi6210_i2s_dai_init: snd_soc_dai_driver = snd_soc_dai_driver {
    playback: snd_soc_pcm_stream {
        channels_min: 2,
        channels_max: 2,
        formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_U16_LE,
        rates: SNDRV_PCM_RATE_48000,
        ..unsafe { core::mem::zeroed() }
    },
    capture: snd_soc_pcm_stream {
        channels_min: 2,
        channels_max: 2,
        formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_U16_LE,
        rates: SNDRV_PCM_RATE_48000,
        ..unsafe { core::mem::zeroed() }
    },
    ops: &hi6210_i2s_dai_ops,
    ..unsafe { core::mem::zeroed() }
};

static hi6210_i2s_i2s_comp: snd_soc_component_driver = snd_soc_component_driver {
    name: c"hi6210_i2s-i2s".as_ptr(),
    legacy_dai_naming: 1,
    ..unsafe { core::mem::zeroed() }
};

unsafe fn hi6210_i2s_probe(pdev: *mut platform_device) -> core::ffi::c_int {
    unsafe {
        let node: *mut device_node = (*pdev).dev.of_node;
        let dev: *mut device = &mut (*pdev).dev;
        let i2s: *mut hi6210_i2s;
        let mut res: *mut resource = core::ptr::null_mut();
        let mut ret: core::ffi::c_int;

        i2s = devm_kzalloc(dev, core::mem::size_of::<hi6210_i2s>(), GFP_KERNEL) as *mut hi6210_i2s;
        if i2s.is_null() {
            return -ENOMEM;
        }

        (*i2s).dev = dev;
        spin_lock_init(&mut (*i2s).lock);

        (*i2s).base = devm_platform_get_and_ioremap_resource(pdev, 0, &mut res);
        if IS_ERR((*i2s).base) {
            return PTR_ERR((*i2s).base);
        }

        (*i2s).base_phys = (*res).start as phys_addr_t;
        (*i2s).dai = hi6210_i2s_dai_init;

        dev_set_drvdata(dev, i2s as *mut core::ffi::c_void);

        (*i2s).sysctrl =
            syscon_regmap_lookup_by_phandle(node, c"hisilicon,sysctrl-syscon".as_ptr());
        if IS_ERR((*i2s).sysctrl as *mut core::ffi::c_void) {
            return PTR_ERR((*i2s).sysctrl as *mut core::ffi::c_void);
        }

        (*i2s).clk[CLK_DACODEC] = devm_clk_get(dev, c"dacodec".as_ptr());
        if IS_ERR((*i2s).clk[CLK_DACODEC] as *mut core::ffi::c_void) {
            return PTR_ERR((*i2s).clk[CLK_DACODEC] as *mut core::ffi::c_void);
        }
        (*i2s).clocks += 1;

        (*i2s).clk[CLK_I2S_BASE] = devm_clk_get(dev, c"i2s-base".as_ptr());
        if IS_ERR((*i2s).clk[CLK_I2S_BASE] as *mut core::ffi::c_void) {
            return PTR_ERR((*i2s).clk[CLK_I2S_BASE] as *mut core::ffi::c_void);
        }
        (*i2s).clocks += 1;

        ret = devm_snd_dmaengine_pcm_register(dev, core::ptr::null(), 0);
        if ret != 0 {
            return ret;
        }

        ret = devm_snd_soc_register_component(dev, &hi6210_i2s_i2s_comp, &mut (*i2s).dai, 1);
        ret
    }
}

static hi6210_i2s_dt_ids: [of_device_id; 2] = [
    of_device_id {
        compatible: c"hisilicon,hi6210-i2s".as_ptr(),
        ..unsafe { core::mem::zeroed() }
    },
    of_device_id {
        /* sentinel */
        ..unsafe { core::mem::zeroed() }
    },
];

/* MODULE_DEVICE_TABLE(of, hi6210_i2s_dt_ids); */

static mut hi6210_i2s_driver: platform_driver = platform_driver {
    probe: Some(hi6210_i2s_probe),
    driver: device_driver {
        name: c"hi6210_i2s".as_ptr(),
        of_match_table: hi6210_i2s_dt_ids.as_ptr(),
        ..unsafe { core::mem::zeroed() }
    },
    ..unsafe { core::mem::zeroed() }
};

/* module_platform_driver(hi6210_i2s_driver); */

/* MODULE_DESCRIPTION("Hisilicon HI6210 I2S driver"); */
/* MODULE_AUTHOR("Andy Green <andy.green@linaro.org>"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
