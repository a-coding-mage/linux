// SPDX-License-Identifier: GPL-2.0-only
// C dependencies: linux/module.h, linux/of.h, linux/platform_device.h, sound/soc.h

/*
 * The I2S interface consists of two ring buffers - one for RX and one for
 * TX.  A ring buffer has a producer index and a consumer index. Depending
 * on which way the data is flowing, either the software or the hardware
 * writes data and updates the producer index, and the other end reads data
 * and updates the consumer index.
 *
 * The pointer managed by software is updated using the .ack callback
 * (see chv3_dma_ack). This seems to be the only way to reliably obtain
 * the appl_ptr from within the driver and pass it to hardware.
 *
 * Because of the two pointer design, the ring buffer can never be full. With
 * capture this isn't a problem, because the hardware being the producer
 * will wait for the consumer index to move out of the way.  With playback,
 * however, this is problematic, because ALSA wants to fill up the buffer
 * completely when waiting for hardware. In the .ack callback, the driver
 * would have to wait for the consumer index to move out of the way by
 * busy-waiting, which would keep stalling the kernel for quite a long time.
 *
 * The workaround to this problem is to "lie" to ALSA that the hw_pointer
 * is one frame behind what it actually is (see chv3_dma_pointer). This
 * way, ALSA will not try to fill up the entire buffer, and all callbacks
 * are wait-free.
 */

const I2S_TX_ENABLE: i32 = 0x00;
const I2S_TX_BASE_ADDR: i32 = 0x04;
const I2S_TX_BUFFER_SIZE: i32 = 0x08;
const I2S_TX_PRODUCER_IDX: i32 = 0x0c;
const I2S_TX_CONSUMER_IDX: i32 = 0x10;
const I2S_RX_ENABLE: i32 = 0x14;
const I2S_RX_BASE_ADDR: i32 = 0x18;
const I2S_RX_BUFFER_SIZE: i32 = 0x1c;
const I2S_RX_PRODUCER_IDX: i32 = 0x20;
const I2S_RX_CONSUMER_IDX: i32 = 0x24;

const I2S_SOFT_RESET: i32 = 0x2c;
const I2S_SOFT_RESET_RX_BIT: u32 = 0x1;
const I2S_SOFT_RESET_TX_BIT: u32 = 0x2;

const I2S_RX_IRQ: i32 = 0x4c;
const I2S_RX_IRQ_CONST: i32 = 0x50;
const I2S_TX_IRQ: i32 = 0x54;
const I2S_TX_IRQ_CONST: i32 = 0x58;

const I2S_IRQ_MASK: i32 = 0x8;
const I2S_IRQ_CLR: i32 = 0xc;
const I2S_IRQ_RX_BIT: u32 = 0x1;
const I2S_IRQ_TX_BIT: u32 = 0x2;

const I2S_MAX_BUFFER_SIZE: u32 = 0x200000;

#[repr(C)]
struct chv3_i2s_dev {
    dev: *mut device,
    iobase: *mut core::ffi::c_void,
    iobase_irq: *mut core::ffi::c_void,
    rx_substream: *mut snd_pcm_substream,
    tx_substream: *mut snd_pcm_substream,
    tx_bytes_to_fetch: core::ffi::c_int,
}

static mut chv3_i2s_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"chv3-i2s".as_ptr(),
    capture: snd_soc_pcm_stream {
        channels_min: 1,
        channels_max: 128,
        rates: SNDRV_PCM_RATE_CONTINUOUS,
        rate_min: 8000,
        rate_max: 96000,
        formats: SNDRV_PCM_FMTBIT_S32_LE,
        ..unsafe { core::mem::zeroed() }
    },
    playback: snd_soc_pcm_stream {
        channels_min: 1,
        channels_max: 128,
        rates: SNDRV_PCM_RATE_CONTINUOUS,
        rate_min: 8000,
        rate_max: 96000,
        formats: SNDRV_PCM_FMTBIT_S32_LE,
        ..unsafe { core::mem::zeroed() }
    },
    ..unsafe { core::mem::zeroed() }
};

static chv3_dma_hw: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_BLOCK_TRANSFER,
    buffer_bytes_max: I2S_MAX_BUFFER_SIZE,
    period_bytes_min: 64,
    period_bytes_max: 8192,
    periods_min: 4,
    periods_max: 256,
    ..unsafe { core::mem::zeroed() }
};

#[inline]
unsafe fn chv3_i2s_wr(i2s: *mut chv3_i2s_dev, offset: core::ffi::c_int, val: u32) {
    unsafe {
        writel(val, (*i2s).iobase.byte_offset(offset as isize));
    }
}

#[inline]
unsafe fn chv3_i2s_rd(i2s: *mut chv3_i2s_dev, offset: core::ffi::c_int) -> u32 {
    unsafe { readl((*i2s).iobase.byte_offset(offset as isize)) }
}

unsafe extern "C" fn chv3_i2s_isr(
    irq: core::ffi::c_int,
    data: *mut core::ffi::c_void,
) -> irqreturn_t {
    let i2s: *mut chv3_i2s_dev = data as *mut chv3_i2s_dev;
    let reg: u32;

    unsafe {
        reg = readl((*i2s).iobase_irq.byte_offset(I2S_IRQ_CLR as isize));
        if reg == 0 {
            return IRQ_NONE;
        }

        if (reg & I2S_IRQ_RX_BIT) != 0 {
            snd_pcm_period_elapsed((*i2s).rx_substream);
        }

        if (reg & I2S_IRQ_TX_BIT) != 0 {
            snd_pcm_period_elapsed((*i2s).tx_substream);
        }

        writel(reg, (*i2s).iobase_irq.byte_offset(I2S_IRQ_CLR as isize));
    }

    IRQ_HANDLED
}

unsafe extern "C" fn chv3_dma_open(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> core::ffi::c_int {
    let rtd: *mut snd_soc_pcm_runtime = unsafe { snd_soc_substream_to_rtd(substream) };
    let i2s: *mut chv3_i2s_dev =
        unsafe { snd_soc_dai_get_drvdata(snd_soc_rtd_to_cpu(rtd, 0)) as *mut chv3_i2s_dev };
    let res: core::ffi::c_int;

    unsafe {
        snd_soc_set_runtime_hwparams(substream, &chv3_dma_hw);

        res = snd_pcm_hw_constraint_pow2(
            (*substream).runtime,
            0,
            SNDRV_PCM_HW_PARAM_BUFFER_BYTES,
        );
        if res != 0 {
            return res;
        }

        if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
            (*i2s).rx_substream = substream;
        } else {
            (*i2s).tx_substream = substream;
        }
    }

    0
}

unsafe extern "C" fn chv3_dma_close(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> core::ffi::c_int {
    let rtd: *mut snd_soc_pcm_runtime = unsafe { snd_soc_substream_to_rtd(substream) };
    let i2s: *mut chv3_i2s_dev =
        unsafe { snd_soc_dai_get_drvdata(snd_soc_rtd_to_cpu(rtd, 0)) as *mut chv3_i2s_dev };

    unsafe {
        if (*(*substream).pstr).stream == SNDRV_PCM_STREAM_CAPTURE {
            chv3_i2s_wr(i2s, I2S_RX_ENABLE, 0);
        } else {
            chv3_i2s_wr(i2s, I2S_TX_ENABLE, 0);
        }
    }

    0
}

unsafe extern "C" fn chv3_dma_pcm_new(
    component: *mut snd_soc_component,
    rtd: *mut snd_soc_pcm_runtime,
) -> core::ffi::c_int {
    let i2s: *mut chv3_i2s_dev =
        unsafe { snd_soc_dai_get_drvdata(snd_soc_rtd_to_cpu(rtd, 0)) as *mut chv3_i2s_dev };
    let mut substream: *mut snd_pcm_substream;
    let res: core::ffi::c_int;

    unsafe {
        substream = (*(*rtd).pcm).streams[SNDRV_PCM_STREAM_PLAYBACK as usize].substream;
        if !substream.is_null() {
            res = snd_dma_alloc_pages(
                SNDRV_DMA_TYPE_DEV,
                (*i2s).dev,
                I2S_MAX_BUFFER_SIZE,
                &mut (*substream).dma_buffer,
            );
            if res != 0 {
                return res;
            }
        }

        substream = (*(*rtd).pcm).streams[SNDRV_PCM_STREAM_CAPTURE as usize].substream;
        if !substream.is_null() {
            res = snd_dma_alloc_pages(
                SNDRV_DMA_TYPE_DEV,
                (*i2s).dev,
                I2S_MAX_BUFFER_SIZE,
                &mut (*substream).dma_buffer,
            );
            if res != 0 {
                return res;
            }
        }
    }

    0
}

unsafe extern "C" fn chv3_dma_hw_params(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> core::ffi::c_int {
    unsafe {
        snd_pcm_set_runtime_buffer(substream, &mut (*substream).dma_buffer);
    }
    0
}

unsafe extern "C" fn chv3_dma_prepare(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> core::ffi::c_int {
    let rtd: *mut snd_soc_pcm_runtime = unsafe { snd_soc_substream_to_rtd(substream) };
    let i2s: *mut chv3_i2s_dev =
        unsafe { snd_soc_dai_get_drvdata(snd_soc_rtd_to_cpu(rtd, 0)) as *mut chv3_i2s_dev };
    let buffer_bytes: core::ffi::c_uint;
    let period_bytes: core::ffi::c_uint;
    let period_size: core::ffi::c_uint;

    unsafe {
        buffer_bytes = snd_pcm_lib_buffer_bytes(substream);
        period_bytes = snd_pcm_lib_period_bytes(substream);
        period_size = (*(*substream).runtime).period_size as core::ffi::c_uint;

        if (*(*substream).pstr).stream == SNDRV_PCM_STREAM_CAPTURE {
            chv3_i2s_wr(i2s, I2S_SOFT_RESET, I2S_SOFT_RESET_RX_BIT);
            chv3_i2s_wr(i2s, I2S_RX_BASE_ADDR, (*substream).dma_buffer.addr as u32);
            chv3_i2s_wr(i2s, I2S_RX_BUFFER_SIZE, buffer_bytes);
            chv3_i2s_wr(i2s, I2S_RX_IRQ, (period_size << 8) | 1);
            chv3_i2s_wr(i2s, I2S_RX_ENABLE, 1);
        } else {
            chv3_i2s_wr(i2s, I2S_SOFT_RESET, I2S_SOFT_RESET_TX_BIT);
            chv3_i2s_wr(i2s, I2S_TX_BASE_ADDR, (*substream).dma_buffer.addr as u32);
            chv3_i2s_wr(i2s, I2S_TX_BUFFER_SIZE, buffer_bytes);
            chv3_i2s_wr(
                i2s,
                I2S_TX_IRQ,
                ((period_bytes / (*i2s).tx_bytes_to_fetch as u32) << 8) | 1,
            );
            chv3_i2s_wr(i2s, I2S_TX_ENABLE, 1);
        }
        writel(
            I2S_IRQ_RX_BIT | I2S_IRQ_TX_BIT,
            (*i2s).iobase_irq.byte_offset(I2S_IRQ_MASK as isize),
        );
    }

    0
}

unsafe extern "C" fn chv3_dma_pointer(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let rtd: *mut snd_soc_pcm_runtime = unsafe { snd_soc_substream_to_rtd(substream) };
    let i2s: *mut chv3_i2s_dev =
        unsafe { snd_soc_dai_get_drvdata(snd_soc_rtd_to_cpu(rtd, 0)) as *mut chv3_i2s_dev };
    let frame_bytes: u32;
    let buffer_bytes: u32;
    let mut idx_bytes: u32;

    unsafe {
        frame_bytes = (*(*substream).runtime).frame_bits * 8;
        buffer_bytes = snd_pcm_lib_buffer_bytes(substream);

        if (*(*substream).pstr).stream == SNDRV_PCM_STREAM_CAPTURE {
            idx_bytes = chv3_i2s_rd(i2s, I2S_RX_PRODUCER_IDX);
        } else {
            idx_bytes = chv3_i2s_rd(i2s, I2S_TX_CONSUMER_IDX);
            /* lag the pointer by one frame */
            idx_bytes = idx_bytes.wrapping_sub(frame_bytes) & (buffer_bytes - 1);
        }

        bytes_to_frames((*substream).runtime, idx_bytes)
    }
}

unsafe extern "C" fn chv3_dma_ack(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> core::ffi::c_int {
    let runtime: *mut snd_pcm_runtime = unsafe { (*substream).runtime };
    let rtd: *mut snd_soc_pcm_runtime = unsafe { snd_soc_substream_to_rtd(substream) };
    let i2s: *mut chv3_i2s_dev =
        unsafe { snd_soc_dai_get_drvdata(snd_soc_rtd_to_cpu(rtd, 0)) as *mut chv3_i2s_dev };
    let bytes: core::ffi::c_uint;
    let idx: core::ffi::c_uint;

    unsafe {
        bytes = frames_to_bytes(runtime, (*(*runtime).control).appl_ptr);
        idx = bytes & (snd_pcm_lib_buffer_bytes(substream) - 1);

        if (*(*substream).pstr).stream == SNDRV_PCM_STREAM_CAPTURE {
            chv3_i2s_wr(i2s, I2S_RX_CONSUMER_IDX, idx);
        } else {
            chv3_i2s_wr(i2s, I2S_TX_PRODUCER_IDX, idx);
        }
    }

    0
}

static chv3_i2s_comp: snd_soc_component_driver = snd_soc_component_driver {
    name: c"chv3-i2s-comp".as_ptr(),
    open: Some(chv3_dma_open),
    close: Some(chv3_dma_close),
    pcm_new: Some(chv3_dma_pcm_new),
    hw_params: Some(chv3_dma_hw_params),
    prepare: Some(chv3_dma_prepare),
    pointer: Some(chv3_dma_pointer),
    ack: Some(chv3_dma_ack),
    ..unsafe { core::mem::zeroed() }
};

unsafe extern "C" fn chv3_i2s_probe(pdev: *mut platform_device) -> core::ffi::c_int {
    let i2s: *mut chv3_i2s_dev;
    let mut res: core::ffi::c_int;
    let irq: core::ffi::c_int;

    unsafe {
        i2s = devm_kzalloc(
            &mut (*pdev).dev,
            core::mem::size_of::<chv3_i2s_dev>(),
            GFP_KERNEL,
        ) as *mut chv3_i2s_dev;
        if i2s.is_null() {
            return -ENOMEM;
        }

        (*i2s).iobase = devm_platform_ioremap_resource(pdev, 0);
        if IS_ERR((*i2s).iobase) {
            return PTR_ERR((*i2s).iobase) as core::ffi::c_int;
        }

        (*i2s).iobase_irq = devm_platform_ioremap_resource(pdev, 1);
        if IS_ERR((*i2s).iobase_irq) {
            return PTR_ERR((*i2s).iobase_irq) as core::ffi::c_int;
        }

        (*i2s).tx_bytes_to_fetch =
            ((chv3_i2s_rd(i2s, I2S_TX_IRQ_CONST) >> 8) & 0xffff) as core::ffi::c_int;

        (*i2s).dev = &mut (*pdev).dev;
        dev_set_drvdata(&mut (*pdev).dev, i2s as *mut core::ffi::c_void);

        irq = platform_get_irq(pdev, 0);
        if irq < 0 {
            return -ENXIO;
        }
        res = devm_request_irq(
            (*i2s).dev,
            irq,
            Some(chv3_i2s_isr),
            0,
            c"chv3-i2s".as_ptr(),
            i2s as *mut core::ffi::c_void,
        );
        if res != 0 {
            return res;
        }

        res = devm_snd_soc_register_component(
            &mut (*pdev).dev,
            &chv3_i2s_comp,
            &mut chv3_i2s_dai,
            1,
        );
        if res != 0 {
            dev_err(
                &mut (*pdev).dev,
                c"couldn't register component: %d\n".as_ptr(),
                res,
            );
            return res;
        }
    }

    0
}

static chv3_i2s_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c"google,chv3-i2s".as_ptr(),
        ..unsafe { core::mem::zeroed() }
    },
    unsafe { core::mem::zeroed() },
];
// MODULE_DEVICE_TABLE(of, chv3_i2s_of_match);

static mut chv3_i2s_driver: platform_driver = platform_driver {
    probe: Some(chv3_i2s_probe),
    driver: device_driver {
        name: c"chv3-i2s".as_ptr(),
        of_match_table: chv3_i2s_of_match.as_ptr(),
        ..unsafe { core::mem::zeroed() }
    },
    ..unsafe { core::mem::zeroed() }
};

// module_platform_driver(chv3_i2s_driver);

// MODULE_AUTHOR("Pawel Anikiel <pan@semihalf.com>");
// MODULE_DESCRIPTION("Chameleon v3 I2S interface");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
