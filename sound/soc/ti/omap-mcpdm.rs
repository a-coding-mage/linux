// SPDX-License-Identifier: GPL-2.0-only
/*
 * omap-mcpdm.c  --  OMAP ALSA SoC DAI driver using McPDM port
 *
 * Copyright (C) 2009 - 2011 Texas Instruments
 *
 * Author: Misael Lopez Cruz <misael.lopez@ti.com>
 * Contact: Jorge Eduardo Candelaria <x0107209@ti.com>
 *          Margarita Olaya <magi.olaya@ti.com>
 *          Peter Ujfalusi <peter.ujfalusi@ti.com>
 */

// Dependencies from Linux kernel and ALSA subsystems:
// linux/init.h, linux/module.h, linux/platform_device.h, linux/interrupt.h,
// linux/err.h, linux/io.h, linux/irq.h, linux/slab.h, linux/pm_runtime.h,
// sound/core.h, sound/pcm.h, sound/pcm_params.h, sound/soc.h,
// sound/dmaengine_pcm.h, omap-mcpdm.h, sdma-pcm.h

use core::ffi::c_void;
use core::ptr;

#[repr(C)]
pub struct McpdmLinkConfig {
    pub link_mask: u32,
    pub threshold: u32,
}

#[repr(C)]
pub struct OmapMcpdm {
    pub dev: *mut c_void,
    pub phys_base: usize,
    pub io_base: *mut c_void,
    pub irq: i32,
    pub pm_qos_req: *mut c_void,
    pub latency: [i32; 2],
    pub mutex: *mut c_void,
    pub config: [McpdmLinkConfig; 2],
    pub dn_rx_offset: u32,
    pub restart: bool,
    pub pm_active_count: i32,
    pub dma_data: [*mut c_void; 2],
}

// Stream DMA parameters

#[inline]
pub unsafe fn omap_mcpdm_write(mcpdm: *mut OmapMcpdm, reg: u16, val: u32) {
    let addr = (*mcpdm).io_base.add(reg as usize) as *mut u32;
    core::ptr::write_volatile(addr, val);
}

#[inline]
pub unsafe fn omap_mcpdm_read(mcpdm: *mut OmapMcpdm, reg: u16) -> u32 {
    let addr = (*mcpdm).io_base.add(reg as usize) as *const u32;
    core::ptr::read_volatile(addr)
}

#[cfg(debug_assertions)]
pub unsafe fn omap_mcpdm_reg_dump(mcpdm: *mut OmapMcpdm) {
    extern "C" {
        fn dev_dbg(dev: *const c_void, fmt: *const u8, ...);
    }

    dev_dbg((*mcpdm).dev as *const c_void, b"***********************\n".as_ptr());
    dev_dbg(
        (*mcpdm).dev as *const c_void,
        b"IRQSTATUS_RAW:  0x%04x\n".as_ptr(),
        omap_mcpdm_read(mcpdm, 0),
    );
    dev_dbg(
        (*mcpdm).dev as *const c_void,
        b"IRQSTATUS:  0x%04x\n".as_ptr(),
        omap_mcpdm_read(mcpdm, 2),
    );
    dev_dbg(
        (*mcpdm).dev as *const c_void,
        b"IRQENABLE_SET:  0x%04x\n".as_ptr(),
        omap_mcpdm_read(mcpdm, 4),
    );
    dev_dbg(
        (*mcpdm).dev as *const c_void,
        b"IRQENABLE_CLR:  0x%04x\n".as_ptr(),
        omap_mcpdm_read(mcpdm, 6),
    );
    dev_dbg(
        (*mcpdm).dev as *const c_void,
        b"IRQWAKE_EN: 0x%04x\n".as_ptr(),
        omap_mcpdm_read(mcpdm, 8),
    );
    dev_dbg(
        (*mcpdm).dev as *const c_void,
        b"DMAENABLE_SET: 0x%04x\n".as_ptr(),
        omap_mcpdm_read(mcpdm, 10),
    );
    dev_dbg(
        (*mcpdm).dev as *const c_void,
        b"DMAENABLE_CLR:  0x%04x\n".as_ptr(),
        omap_mcpdm_read(mcpdm, 12),
    );
    dev_dbg(
        (*mcpdm).dev as *const c_void,
        b"DMAWAKEEN:  0x%04x\n".as_ptr(),
        omap_mcpdm_read(mcpdm, 14),
    );
    dev_dbg(
        (*mcpdm).dev as *const c_void,
        b"CTRL:  0x%04x\n".as_ptr(),
        omap_mcpdm_read(mcpdm, 16),
    );
    dev_dbg(
        (*mcpdm).dev as *const c_void,
        b"DN_DATA:  0x%04x\n".as_ptr(),
        omap_mcpdm_read(mcpdm, 18),
    );
    dev_dbg(
        (*mcpdm).dev as *const c_void,
        b"UP_DATA: 0x%04x\n".as_ptr(),
        omap_mcpdm_read(mcpdm, 20),
    );
    dev_dbg(
        (*mcpdm).dev as *const c_void,
        b"FIFO_CTRL_DN: 0x%04x\n".as_ptr(),
        omap_mcpdm_read(mcpdm, 22),
    );
    dev_dbg(
        (*mcpdm).dev as *const c_void,
        b"FIFO_CTRL_UP:  0x%04x\n".as_ptr(),
        omap_mcpdm_read(mcpdm, 24),
    );
    dev_dbg((*mcpdm).dev as *const c_void, b"***********************\n".as_ptr());
}

#[cfg(not(debug_assertions))]
pub unsafe fn omap_mcpdm_reg_dump(_mcpdm: *mut OmapMcpdm) {}

pub unsafe fn omap_mcpdm_start(mcpdm: *mut OmapMcpdm) {
    let mut ctrl = omap_mcpdm_read(mcpdm, 16) as i32;
    let link_mask = (*mcpdm).config[0].link_mask | (*mcpdm).config[1].link_mask;

    ctrl |= (0x8 | 0x10);
    omap_mcpdm_write(mcpdm, 16, ctrl as u32);

    ctrl |= link_mask as i32;
    omap_mcpdm_write(mcpdm, 16, ctrl as u32);

    ctrl &= !(0x8 | 0x10);
    omap_mcpdm_write(mcpdm, 16, ctrl as u32);
}

pub unsafe fn omap_mcpdm_stop(mcpdm: *mut OmapMcpdm) {
    let mut ctrl = omap_mcpdm_read(mcpdm, 16) as i32;
    let link_mask = 0x3E;

    ctrl |= (0x8 | 0x10);
    omap_mcpdm_write(mcpdm, 16, ctrl as u32);

    ctrl &= !(link_mask);
    omap_mcpdm_write(mcpdm, 16, ctrl as u32);

    ctrl &= !(0x8 | 0x10);
    omap_mcpdm_write(mcpdm, 16, ctrl as u32);
}

#[inline]
pub unsafe fn omap_mcpdm_active(mcpdm: *mut OmapMcpdm) -> i32 {
    (omap_mcpdm_read(mcpdm, 16) & 0x1E) as i32
}

pub unsafe fn omap_mcpdm_open_streams(mcpdm: *mut OmapMcpdm) {
    let ctrl = omap_mcpdm_read(mcpdm, 16);

    omap_mcpdm_write(mcpdm, 16, ctrl | 0x04);

    omap_mcpdm_write(mcpdm, 4, 0x60 | 0x18);

    if (*mcpdm).dn_rx_offset != 0 {
        let mut dn_offset = (*mcpdm).dn_rx_offset;

        omap_mcpdm_write(mcpdm, 26, dn_offset);
        dn_offset |= 0x30;
        omap_mcpdm_write(mcpdm, 26, dn_offset);
    }

    omap_mcpdm_write(mcpdm, 22, (*mcpdm).config[0].threshold);
    omap_mcpdm_write(mcpdm, 24, (*mcpdm).config[1].threshold);

    omap_mcpdm_write(mcpdm, 10, 0x03);
}

pub unsafe fn omap_mcpdm_close_streams(mcpdm: *mut OmapMcpdm) {
    omap_mcpdm_write(mcpdm, 6, 0x60);

    omap_mcpdm_write(mcpdm, 12, 0x01);

    omap_mcpdm_write(mcpdm, 6, 0x18);

    omap_mcpdm_write(mcpdm, 12, 0x02);

    if (*mcpdm).dn_rx_offset != 0 {
        omap_mcpdm_write(mcpdm, 26, 0);
    }
}

pub unsafe extern "C" fn omap_mcpdm_irq_handler(irq: i32, dev_id: *mut c_void) -> i32 {
    let mcpdm = dev_id as *mut OmapMcpdm;
    let irq_status = omap_mcpdm_read(mcpdm, 2) as i32;

    omap_mcpdm_write(mcpdm, 2, irq_status as u32);

    if irq_status & 0x20 != 0 {
        extern "C" {
            fn dev_dbg(dev: *const c_void, fmt: *const u8, ...);
        }
        dev_dbg((*mcpdm).dev as *const c_void, b"DN (playback) FIFO Full\n".as_ptr());
    }

    if irq_status & 0x40 != 0 {
        extern "C" {
            fn dev_dbg(dev: *const c_void, fmt: *const u8, ...);
        }
        dev_dbg((*mcpdm).dev as *const c_void, b"DN (playback) FIFO Empty\n".as_ptr());
    }

    if irq_status & 0x01 != 0 {
        extern "C" {
            fn dev_dbg(dev: *const c_void, fmt: *const u8, ...);
        }
        dev_dbg((*mcpdm).dev as *const c_void, b"DN (playback) write request\n".as_ptr());
    }

    if irq_status & 0x02 != 0 {
        extern "C" {
            fn dev_dbg(dev: *const c_void, fmt: *const u8, ...);
        }
        dev_dbg((*mcpdm).dev as *const c_void, b"UP (capture) FIFO Full\n".as_ptr());
    }

    if irq_status & 0x04 != 0 {
        extern "C" {
            fn dev_dbg(dev: *const c_void, fmt: *const u8, ...);
        }
        dev_dbg((*mcpdm).dev as *const c_void, b"UP (capture) FIFO Empty\n".as_ptr());
    }

    if irq_status & 0x08 != 0 {
        extern "C" {
            fn dev_dbg(dev: *const c_void, fmt: *const u8, ...);
        }
        dev_dbg((*mcpdm).dev as *const c_void, b"UP (capture) write request\n".as_ptr());
    }

    1
}

pub unsafe fn omap_mcpdm_dai_startup(substream: *mut c_void, dai: *mut c_void) -> i32 {
    extern "C" {
        fn snd_soc_dai_get_drvdata(dai: *mut c_void) -> *mut c_void;
        fn snd_soc_dai_active(dai: *mut c_void) -> i32;
        fn mutex_lock(lock: *mut c_void);
        fn mutex_unlock(lock: *mut c_void);
    }

    let mcpdm = snd_soc_dai_get_drvdata(dai) as *mut OmapMcpdm;

    mutex_lock((*mcpdm).mutex);

    if snd_soc_dai_active(dai) == 0 {
        omap_mcpdm_open_streams(mcpdm);
    }

    mutex_unlock((*mcpdm).mutex);

    0
}

pub unsafe fn omap_mcpdm_dai_shutdown(substream: *mut c_void, dai: *mut c_void) {
    extern "C" {
        fn snd_soc_dai_get_drvdata(dai: *mut c_void) -> *mut c_void;
        fn snd_soc_dai_active(dai: *mut c_void) -> i32;
        fn snd_pcm_substream_get_stream(substream: *mut c_void) -> i32;
        fn mutex_lock(lock: *mut c_void);
        fn mutex_unlock(lock: *mut c_void);
        fn cpu_latency_qos_update_request(req: *mut c_void, value: i32);
        fn cpu_latency_qos_remove_request(req: *mut c_void);
    }

    let mcpdm = snd_soc_dai_get_drvdata(dai) as *mut OmapMcpdm;
    let tx = if snd_pcm_substream_get_stream(substream) == 0 { 1 } else { 0 };
    let stream1 = if tx != 0 { 0 } else { 1 };
    let stream2 = if tx != 0 { 1 } else { 0 };

    mutex_lock((*mcpdm).mutex);

    if snd_soc_dai_active(dai) == 0 {
        if omap_mcpdm_active(mcpdm) != 0 {
            omap_mcpdm_stop(mcpdm);
            omap_mcpdm_close_streams(mcpdm);
            (*mcpdm).config[0].link_mask = 0;
            (*mcpdm).config[1].link_mask = 0;
        }
    }

    if (*mcpdm).latency[stream2] != 0 {
        cpu_latency_qos_update_request(&mut (*mcpdm).pm_qos_req, (*mcpdm).latency[stream2]);
    } else if (*mcpdm).latency[stream1] != 0 {
        cpu_latency_qos_remove_request(&mut (*mcpdm).pm_qos_req);
    }

    (*mcpdm).latency[stream1] = 0;

    mutex_unlock((*mcpdm).mutex);
}

pub unsafe fn omap_mcpdm_dai_hw_params(
    substream: *mut c_void,
    params: *mut c_void,
    dai: *mut c_void,
) -> i32 {
    extern "C" {
        fn snd_soc_dai_get_drvdata(dai: *mut c_void) -> *mut c_void;
        fn snd_soc_dai_get_dma_data(dai: *mut c_void, substream: *mut c_void) -> *mut c_void;
        fn snd_pcm_substream_get_stream(substream: *mut c_void) -> i32;
        fn params_channels(params: *mut c_void) -> i32;
        fn params_rate(params: *mut c_void) -> i32;
    }

    let mcpdm = snd_soc_dai_get_drvdata(dai) as *mut OmapMcpdm;
    let stream = snd_pcm_substream_get_stream(substream);
    let dma_data = snd_soc_dai_get_dma_data(dai, substream) as *mut c_void;
    let channels = params_channels(params);
    let mut link_mask = 0;

    match channels {
        5 => {
            if stream == 1 {
                return -22;
            }
            link_mask |= 1 << 4;
        }
        4 => {
            if stream == 1 {
                return -22;
            }
            link_mask |= 1 << 3;
        }
        3 => {
            link_mask |= 1 << 2;
        }
        2 => {
            link_mask |= 1 << 1;
        }
        1 => {
            link_mask |= 1 << 0;
        }
        _ => {
            return -22;
        }
    }

    let threshold = (*mcpdm).config[stream as usize].threshold;

    if stream == 0 {
        link_mask <<= 3;

        if (*mcpdm).config[1].link_mask == 0 {
            (*mcpdm).config[1].link_mask = 0x3;
        }

        extern "C" {
            fn set_dma_maxburst(dma_data: *mut c_void, value: u32);
        }
        set_dma_maxburst(dma_data, (0x9E - threshold) * (channels as u32));
        let latency = threshold as i32;
        (*mcpdm).latency[stream as usize] = latency * 1000000 / params_rate(params);
    } else {
        if (*mcpdm).config[0].link_mask == 0 {
            (*mcpdm).config[0].link_mask = 0x18;
        }

        extern "C" {
            fn set_dma_maxburst(dma_data: *mut c_void, value: u32);
        }
        set_dma_maxburst(dma_data, threshold * (channels as u32));
        let latency = (0x9E - threshold) as i32;
        (*mcpdm).latency[stream as usize] = latency * 1000000 / params_rate(params);
    }

    if (*mcpdm).latency[stream as usize] == 0 {
        (*mcpdm).latency[stream as usize] = 10;
    }

    if (*mcpdm).config[stream as usize].link_mask != 0
        && (*mcpdm).config[stream as usize].link_mask != link_mask as u32
    {
        (*mcpdm).restart = true;
    }

    (*mcpdm).config[stream as usize].link_mask = link_mask as u32;

    0
}

pub unsafe fn omap_mcpdm_prepare(substream: *mut c_void, dai: *mut c_void) -> i32 {
    extern "C" {
        fn snd_soc_dai_get_drvdata(dai: *mut c_void) -> *mut c_void;
        fn snd_pcm_substream_get_stream(substream: *mut c_void) -> i32;
        fn cpu_latency_qos_request_active(req: *const c_void) -> i32;
        fn cpu_latency_qos_update_request(req: *mut c_void, value: i32);
        fn cpu_latency_qos_add_request(req: *mut c_void, value: i32);
    }

    let mcpdm = snd_soc_dai_get_drvdata(dai) as *mut OmapMcpdm;
    let tx = if snd_pcm_substream_get_stream(substream) == 0 { 1 } else { 0 };
    let stream1 = if tx != 0 { 0 } else { 1 };
    let stream2 = if tx != 0 { 1 } else { 0 };
    let mut latency = (*mcpdm).latency[stream2];

    if latency == 0 || (*mcpdm).latency[stream1] < latency {
        latency = (*mcpdm).latency[stream1];
    }

    if cpu_latency_qos_request_active(&(*mcpdm).pm_qos_req) != 0 {
        cpu_latency_qos_update_request(&mut (*mcpdm).pm_qos_req, latency);
    } else if latency != 0 {
        cpu_latency_qos_add_request(&mut (*mcpdm).pm_qos_req, latency);
    }

    if omap_mcpdm_active(mcpdm) == 0 {
        omap_mcpdm_start(mcpdm);
        omap_mcpdm_reg_dump(mcpdm);
    } else if (*mcpdm).restart {
        omap_mcpdm_stop(mcpdm);
        omap_mcpdm_start(mcpdm);
        (*mcpdm).restart = false;
        omap_mcpdm_reg_dump(mcpdm);
    }

    0
}

pub unsafe fn omap_mcpdm_probe(dai: *mut c_void) -> i32 {
    extern "C" {
        fn snd_soc_dai_get_drvdata(dai: *mut c_void) -> *mut c_void;
        fn pm_runtime_enable(dev: *mut c_void);
        fn pm_runtime_get_sync(dev: *mut c_void);
        fn pm_runtime_put_sync(dev: *mut c_void);
        fn pm_runtime_disable(dev: *mut c_void);
        fn request_irq(irq: i32, handler: *const c_void, flags: u32, name: *const u8, dev: *mut c_void) -> i32;
        fn dev_err(dev: *const c_void, fmt: *const u8, ...);
        fn snd_soc_dai_init_dma_data(dai: *mut c_void, playback: *mut c_void, capture: *mut c_void);
    }

    let mcpdm = snd_soc_dai_get_drvdata(dai) as *mut OmapMcpdm;
    let mut ret;

    pm_runtime_enable((*mcpdm).dev);

    pm_runtime_get_sync((*mcpdm).dev);
    omap_mcpdm_write(mcpdm, 16, 0x00);

    ret = request_irq(
        (*mcpdm).irq,
        omap_mcpdm_irq_handler as *const c_void,
        0,
        b"McPDM".as_ptr(),
        mcpdm as *mut c_void,
    );

    pm_runtime_put_sync((*mcpdm).dev);

    if ret != 0 {
        dev_err((*mcpdm).dev as *const c_void, b"Request for IRQ failed\n".as_ptr());
        pm_runtime_disable((*mcpdm).dev);
    }

    (*mcpdm).config[0].threshold = 2;
    (*mcpdm).config[1].threshold = 0x9E - 3;

    snd_soc_dai_init_dma_data(
        dai,
        (*mcpdm).dma_data[0],
        (*mcpdm).dma_data[1],
    );

    ret
}

pub unsafe fn omap_mcpdm_remove(dai: *mut c_void) -> i32 {
    extern "C" {
        fn snd_soc_dai_get_drvdata(dai: *mut c_void) -> *mut c_void;
        fn free_irq(irq: i32, dev: *mut c_void);
        fn pm_runtime_disable(dev: *mut c_void);
        fn cpu_latency_qos_request_active(req: *const c_void) -> i32;
        fn cpu_latency_qos_remove_request(req: *mut c_void);
    }

    let mcpdm = snd_soc_dai_get_drvdata(dai) as *mut OmapMcpdm;

    free_irq((*mcpdm).irq, mcpdm as *mut c_void);
    pm_runtime_disable((*mcpdm).dev);

    if cpu_latency_qos_request_active(&(*mcpdm).pm_qos_req) != 0 {
        cpu_latency_qos_remove_request(&mut (*mcpdm).pm_qos_req);
    }

    0
}

#[repr(C)]
pub struct SndSocDaiOps {
    pub probe: Option<unsafe extern "C" fn(*mut c_void) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut c_void) -> i32>,
    pub startup: Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> i32>,
    pub shutdown: Option<unsafe extern "C" fn(*mut c_void, *mut c_void)>,
    pub hw_params: Option<unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void) -> i32>,
    pub prepare: Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> i32>,
    pub probe_order: u32,
    pub remove_order: u32,
}

pub static OMAP_MCPDM_DAI_OPS: SndSocDaiOps = SndSocDaiOps {
    probe: Some(omap_mcpdm_probe),
    remove: Some(omap_mcpdm_remove),
    startup: Some(omap_mcpdm_dai_startup),
    shutdown: Some(omap_mcpdm_dai_shutdown),
    hw_params: Some(omap_mcpdm_dai_hw_params),
    prepare: Some(omap_mcpdm_prepare),
    probe_order: 2,
    remove_order: 0,
};

#[cfg(feature = "CONFIG_PM_SLEEP")]
pub unsafe fn omap_mcpdm_suspend(component: *mut c_void) -> i32 {
    extern "C" {
        fn snd_soc_component_get_drvdata(component: *mut c_void) -> *mut c_void;
        fn snd_soc_component_active(component: *mut c_void) -> i32;
        fn pm_runtime_active(dev: *mut c_void) -> i32;
        fn pm_runtime_put_sync(dev: *mut c_void);
    }

    let mcpdm = snd_soc_component_get_drvdata(component) as *mut OmapMcpdm;

    if snd_soc_component_active(component) != 0 {
        omap_mcpdm_stop(mcpdm);
        omap_mcpdm_close_streams(mcpdm);
    }

    (*mcpdm).pm_active_count = 0;
    while pm_runtime_active((*mcpdm).dev) != 0 {
        pm_runtime_put_sync((*mcpdm).dev);
        (*mcpdm).pm_active_count += 1;
    }

    0
}

#[cfg(feature = "CONFIG_PM_SLEEP")]
pub unsafe fn omap_mcpdm_resume(component: *mut c_void) -> i32 {
    extern "C" {
        fn snd_soc_component_get_drvdata(component: *mut c_void) -> *mut c_void;
        fn snd_soc_component_active(component: *mut c_void) -> i32;
        fn pm_runtime_get_sync(dev: *mut c_void);
    }

    let mcpdm = snd_soc_component_get_drvdata(component) as *mut OmapMcpdm;

    if (*mcpdm).pm_active_count != 0 {
        while (*mcpdm).pm_active_count > 0 {
            pm_runtime_get_sync((*mcpdm).dev);
            (*mcpdm).pm_active_count -= 1;
        }

        if snd_soc_component_active(component) != 0 {
            omap_mcpdm_open_streams(mcpdm);
            omap_mcpdm_start(mcpdm);
        }
    }

    0
}

#[cfg(not(feature = "CONFIG_PM_SLEEP"))]
pub unsafe fn omap_mcpdm_suspend(_component: *mut c_void) -> i32 {
    0
}

#[cfg(not(feature = "CONFIG_PM_SLEEP"))]
pub unsafe fn omap_mcpdm_resume(_component: *mut c_void) -> i32 {
    0
}

pub const OMAP_MCPDM_RATES: u32 = 0x00100000 | 0x00080000;
pub const OMAP_MCPDM_FORMATS: u32 = 0x00100000;

#[repr(C)]
pub struct SndSocDaiDriver {
    pub playback: SndSocDaiStream,
    pub capture: SndSocDaiStream,
    pub ops: *const SndSocDaiOps,
}

#[repr(C)]
pub struct SndSocDaiStream {
    pub channels_min: i32,
    pub channels_max: i32,
    pub rates: u32,
    pub formats: u32,
    pub sig_bits: i32,
}

pub static OMAP_MCPDM_DAI: SndSocDaiDriver = SndSocDaiDriver {
    playback: SndSocDaiStream {
        channels_min: 1,
        channels_max: 5,
        rates: OMAP_MCPDM_RATES,
        formats: OMAP_MCPDM_FORMATS,
        sig_bits: 24,
    },
    capture: SndSocDaiStream {
        channels_min: 1,
        channels_max: 3,
        rates: OMAP_MCPDM_RATES,
        formats: OMAP_MCPDM_FORMATS,
        sig_bits: 24,
    },
    ops: &OMAP_MCPDM_DAI_OPS,
};

#[repr(C)]
pub struct SndSocComponentDriver {
    pub name: *const u8,
    pub suspend: Option<unsafe extern "C" fn(*mut c_void) -> i32>,
    pub resume: Option<unsafe extern "C" fn(*mut c_void) -> i32>,
    pub legacy_dai_naming: i32,
}

pub static OMAP_MCPDM_COMPONENT: SndSocComponentDriver = SndSocComponentDriver {
    name: b"omap-mcpdm".as_ptr(),
    suspend: Some(omap_mcpdm_suspend),
    resume: Some(omap_mcpdm_resume),
    legacy_dai_naming: 1,
};

pub unsafe fn omap_mcpdm_configure_dn_offsets(rtd: *mut c_void, rx1: u8, rx2: u8) {
    extern "C" {
        fn snd_soc_rtd_to_cpu(rtd: *mut c_void, index: i32) -> *mut c_void;
        fn snd_soc_dai_get_drvdata(dai: *mut c_void) -> *mut c_void;
    }

    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let mcpdm = snd_soc_dai_get_drvdata(cpu_dai) as *mut OmapMcpdm;

    (*mcpdm).dn_rx_offset = (((rx1 as u32 & 0x3) << 2) | ((rx2 as u32 & 0x3) << 0));
}

pub unsafe fn asoc_mcpdm_probe(pdev: *mut c_void) -> i32 {
    extern "C" {
        fn devm_kzalloc(dev: *mut c_void, size: usize, flags: u32) -> *mut c_void;
        fn platform_set_drvdata(pdev: *mut c_void, data: *mut c_void);
        fn mutex_init(lock: *mut c_void);
        fn platform_get_resource_byname(pdev: *mut c_void, rtype: u32, name: *const u8) -> *mut c_void;
        fn devm_platform_ioremap_resource_byname(pdev: *mut c_void, name: *const u8) -> *mut c_void;
        fn platform_get_irq(pdev: *mut c_void, index: i32) -> i32;
        fn devm_snd_soc_register_component(
            dev: *mut c_void,
            component_driver: *const SndSocComponentDriver,
            dai_driver: *const SndSocDaiDriver,
            num_dai: i32,
        ) -> i32;
        fn sdma_pcm_platform_register(dev: *mut c_void, playback: *const u8, capture: *const u8) -> i32;
    }

    let mcpdm = devm_kzalloc(pdev as *mut c_void, core::mem::size_of::<OmapMcpdm>(), 0x100);
    if mcpdm.is_null() {
        return -12;
    }
    let mcpdm = mcpdm as *mut OmapMcpdm;

    platform_set_drvdata(pdev, mcpdm as *mut c_void);

    mutex_init(&mut (*mcpdm).mutex);

    let res = platform_get_resource_byname(pdev, 512, b"dma".as_ptr());
    if res.is_null() {
        return -12;
    }

    extern "C" {
        fn get_resource_start(res: *const c_void) -> usize;
    }

    let res_start = get_resource_start(res);
    (*mcpdm).dma_data[0] = (res_start + 20) as *mut c_void;
    (*mcpdm).dma_data[1] = (res_start + 24) as *mut c_void;

    extern "C" {
        fn set_filter_data(dma_data: *mut c_void, filter: *const u8);
    }

    set_filter_data((*mcpdm).dma_data[0], b"dn_link".as_ptr());
    set_filter_data((*mcpdm).dma_data[1], b"up_link".as_ptr());

    (*mcpdm).io_base = devm_platform_ioremap_resource_byname(pdev, b"mpu".as_ptr());
    if ((*mcpdm).io_base as i64) < 0 {
        return ((*mcpdm).io_base as i64) as i32;
    }

    (*mcpdm).irq = platform_get_irq(pdev, 0);
    if (*mcpdm).irq < 0 {
        return (*mcpdm).irq;
    }

    (*mcpdm).dev = pdev as *mut c_void;

    let ret = devm_snd_soc_register_component(
        pdev as *mut c_void,
        &OMAP_MCPDM_COMPONENT,
        &OMAP_MCPDM_DAI,
        1,
    );
    if ret != 0 {
        return ret;
    }

    sdma_pcm_platform_register(pdev as *mut c_void, b"dn_link".as_ptr(), b"up_link".as_ptr())
}

#[repr(C)]
pub struct OfDeviceId {
    pub compatible: *const u8,
}

pub static OMAP_MCPDM_OF_MATCH: [OfDeviceId; 2] = [
    OfDeviceId {
        compatible: b"ti,omap4-mcpdm\0".as_ptr(),
    },
    OfDeviceId {
        compatible: ptr::null(),
    },
];

#[repr(C)]
pub struct PlatformDriver {
    pub driver: PlatformDriverInfo,
    pub probe: Option<unsafe extern "C" fn(*mut c_void) -> i32>,
}

#[repr(C)]
pub struct PlatformDriverInfo {
    pub name: *const u8,
    pub of_match_table: *const OfDeviceId,
}

pub static ASOC_MCPDM_DRIVER: PlatformDriver = PlatformDriver {
    driver: PlatformDriverInfo {
        name: b"omap-mcpdm\0".as_ptr(),
        of_match_table: &OMAP_MCPDM_OF_MATCH[0],
    },
    probe: Some(asoc_mcpdm_probe),
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
