// SPDX-License-Identifier: GPL-2.0
//
// Socionext UniPhier AIO Compress Audio driver.
//
// Copyright (c) 2017-2018 Socionext Inc.

// Linux kernel includes: bitfield, circ_buf, dma-mapping, errno, kernel, module
// ALSA sound framework includes: core, pcm, soc

// #include "aio.h"

fn uniphier_aio_compr_prepare(
    component: *mut snd_soc_component,
    cstream: *mut snd_compr_stream,
) -> i32;

fn uniphier_aio_compr_hw_free(
    component: *mut snd_soc_component,
    cstream: *mut snd_compr_stream,
) -> i32;

fn uniphier_aio_comprdma_new(rtd: *mut snd_soc_pcm_runtime) -> i32 {
    unsafe {
        let compr = (*rtd).compr;
        let dev = (*(*compr).card).dev;
        let aio = uniphier_priv(snd_soc_rtd_to_cpu(rtd, 0));
        let sub = &mut (*aio).sub[(*compr).direction as usize];
        let size = AUD_RING_SIZE;
        let mut dma_dir = DMA_FROM_DEVICE;
        let ret: i32;

        ret = dma_set_mask_and_coherent(dev, DMA_BIT_MASK(33));
        if ret != 0 {
            return ret;
        }

        (*sub).compr_area = kzalloc(size, GFP_KERNEL) as *mut u8;
        if (*sub).compr_area.is_null() {
            return -ENOMEM;
        }

        if (*(*sub).swm).dir == PORT_DIR_OUTPUT {
            dma_dir = DMA_TO_DEVICE;
        }

        (*sub).compr_addr = dma_map_single(
            dev,
            (*sub).compr_area as *mut core::ffi::c_void,
            size,
            dma_dir,
        );
        if dma_mapping_error(dev, (*sub).compr_addr) {
            kfree((*sub).compr_area as *mut core::ffi::c_void);
            (*sub).compr_area = core::ptr::null_mut();

            return -ENOMEM;
        }

        (*sub).compr_bytes = size;

        return 0;
    }
}

fn uniphier_aio_comprdma_free(rtd: *mut snd_soc_pcm_runtime) -> i32 {
    unsafe {
        let compr = (*rtd).compr;
        let dev = (*(*compr).card).dev;
        let aio = uniphier_priv(snd_soc_rtd_to_cpu(rtd, 0));
        let sub = &mut (*aio).sub[(*compr).direction as usize];
        let mut dma_dir = DMA_FROM_DEVICE;

        if (*(*sub).swm).dir == PORT_DIR_OUTPUT {
            dma_dir = DMA_TO_DEVICE;
        }

        dma_unmap_single(dev, (*sub).compr_addr, (*sub).compr_bytes, dma_dir);
        kfree((*sub).compr_area as *mut core::ffi::c_void);
        (*sub).compr_area = core::ptr::null_mut();

        return 0;
    }
}

fn uniphier_aio_compr_open(
    component: *mut snd_soc_component,
    cstream: *mut snd_compr_stream,
) -> i32 {
    unsafe {
        let rtd = (*cstream).private_data as *mut snd_soc_pcm_runtime;
        let aio = uniphier_priv(snd_soc_rtd_to_cpu(rtd, 0));
        let sub = &mut (*aio).sub[(*cstream).direction as usize];
        let ret: i32;

        if !(*sub).cstream.is_null() {
            return -EBUSY;
        }

        (*sub).cstream = cstream;
        (*sub).pass_through = 1;
        (*sub).use_mmap = false;

        ret = uniphier_aio_comprdma_new(rtd);
        if ret != 0 {
            return ret;
        }

        ret = aio_init(sub);
        if ret != 0 {
            return ret;
        }

        return 0;
    }
}

fn uniphier_aio_compr_free(
    component: *mut snd_soc_component,
    cstream: *mut snd_compr_stream,
) -> i32 {
    unsafe {
        let rtd = (*cstream).private_data as *mut snd_soc_pcm_runtime;
        let aio = uniphier_priv(snd_soc_rtd_to_cpu(rtd, 0));
        let sub = &mut (*aio).sub[(*cstream).direction as usize];
        let ret: i32;

        ret = uniphier_aio_compr_hw_free(component, cstream);
        if ret != 0 {
            return ret;
        }
        ret = uniphier_aio_comprdma_free(rtd);
        if ret != 0 {
            return ret;
        }

        (*sub).cstream = core::ptr::null_mut();

        return 0;
    }
}

fn uniphier_aio_compr_get_params(
    component: *mut snd_soc_component,
    cstream: *mut snd_compr_stream,
    params: *mut snd_codec,
) -> i32 {
    unsafe {
        let rtd = (*cstream).private_data as *mut snd_soc_pcm_runtime;
        let aio = uniphier_priv(snd_soc_rtd_to_cpu(rtd, 0));
        let sub = &mut (*aio).sub[(*cstream).direction as usize];

        *params = (*sub).cparams.codec;

        return 0;
    }
}

fn uniphier_aio_compr_set_params(
    component: *mut snd_soc_component,
    cstream: *mut snd_compr_stream,
    params: *mut snd_compr_params,
) -> i32 {
    unsafe {
        let rtd = (*cstream).private_data as *mut snd_soc_pcm_runtime;
        let aio = uniphier_priv(snd_soc_rtd_to_cpu(rtd, 0));
        let sub = &mut (*aio).sub[(*cstream).direction as usize];
        let dev = &(*(*aio).chip).pdev.dev;

        if (*params).codec.id != SND_AUDIOCODEC_IEC61937 {
            dev_err(
                dev,
                b"Codec ID is not supported(%d)\n\0".as_ptr() as *const i8,
                (*params).codec.id,
            );
            return -EINVAL;
        }
        if (*params).codec.profile != SND_AUDIOPROFILE_IEC61937_SPDIF {
            dev_err(
                dev,
                b"Codec profile is not supported(%d)\n\0".as_ptr() as *const i8,
                (*params).codec.profile,
            );
            return -EINVAL;
        }

        // IEC frame type will be changed after received valid data
        (*sub).iec_pc = IEC61937_PC_AAC;

        (*sub).cparams = *params;
        (*sub).setting = 1;

        aio_port_reset(sub);
        aio_src_reset(sub);

        return uniphier_aio_compr_prepare(component, cstream);
    }
}

fn uniphier_aio_compr_hw_free(
    component: *mut snd_soc_component,
    cstream: *mut snd_compr_stream,
) -> i32 {
    unsafe {
        let rtd = (*cstream).private_data as *mut snd_soc_pcm_runtime;
        let aio = uniphier_priv(snd_soc_rtd_to_cpu(rtd, 0));
        let sub = &mut (*aio).sub[(*cstream).direction as usize];

        (*sub).setting = 0;

        return 0;
    }
}

fn uniphier_aio_compr_prepare(
    component: *mut snd_soc_component,
    cstream: *mut snd_compr_stream,
) -> i32 {
    unsafe {
        let rtd = (*cstream).private_data as *mut snd_soc_pcm_runtime;
        let runtime = (*cstream).runtime;
        let aio = uniphier_priv(snd_soc_rtd_to_cpu(rtd, 0));
        let sub = &mut (*aio).sub[(*cstream).direction as usize];
        let bytes = (*runtime).fragment_size;
        let ret: i32;

        ret = aiodma_ch_set_param(sub);
        if ret != 0 {
            return ret;
        }

        // scoped_guard(spinlock_irqsave, &sub->lock)
        let _guard = spinlock_irqsave_guard(&mut (*sub).lock);
        ret = aiodma_rb_set_buffer(
            sub,
            (*sub).compr_addr,
            (*sub).compr_addr + (*sub).compr_bytes,
            bytes,
        );
        drop(_guard);

        if ret != 0 {
            return ret;
        }

        ret = aio_port_set_param(sub, (*sub).pass_through, &(*sub).params);
        if ret != 0 {
            return ret;
        }
        ret = aio_oport_set_stream_type(sub, (*sub).iec_pc);
        if ret != 0 {
            return ret;
        }
        aio_port_set_enable(sub, 1);

        ret = aio_if_set_param(sub, (*sub).pass_through);
        if ret != 0 {
            return ret;
        }

        return 0;
    }
}

fn uniphier_aio_compr_trigger(
    component: *mut snd_soc_component,
    cstream: *mut snd_compr_stream,
    cmd: i32,
) -> i32 {
    unsafe {
        let rtd = (*cstream).private_data as *mut snd_soc_pcm_runtime;
        let runtime = (*cstream).runtime;
        let aio = uniphier_priv(snd_soc_rtd_to_cpu(rtd, 0));
        let sub = &mut (*aio).sub[(*cstream).direction as usize];
        let dev = &(*(*aio).chip).pdev.dev;
        let bytes = (*runtime).fragment_size;
        let ret = 0i32;

        let _guard = spinlock_irqsave_guard(&mut (*sub).lock);
        match cmd {
            SNDRV_PCM_TRIGGER_START => {
                aiodma_rb_sync(sub, (*sub).compr_addr, (*sub).compr_bytes, bytes);
                aiodma_ch_set_enable(sub, 1);
                (*sub).running = 1;
            }
            SNDRV_PCM_TRIGGER_STOP => {
                (*sub).running = 0;
                aiodma_ch_set_enable(sub, 0);
            }
            _ => {
                dev_warn(
                    dev,
                    b"Unknown trigger(%d)\n\0".as_ptr() as *const i8,
                    cmd,
                );
                return -EINVAL;
            }
        }
        drop(_guard);

        return ret;
    }
}

fn uniphier_aio_compr_pointer(
    component: *mut snd_soc_component,
    cstream: *mut snd_compr_stream,
    tstamp: *mut snd_compr_tstamp64,
) -> i32 {
    unsafe {
        let rtd = (*cstream).private_data as *mut snd_soc_pcm_runtime;
        let runtime = (*cstream).runtime;
        let aio = uniphier_priv(snd_soc_rtd_to_cpu(rtd, 0));
        let sub = &mut (*aio).sub[(*cstream).direction as usize];
        let bytes = (*runtime).fragment_size;
        let pos: u32;

        let _guard = spinlock_irqsave_guard(&mut (*sub).lock);

        aiodma_rb_sync(sub, (*sub).compr_addr, (*sub).compr_bytes, bytes);

        if (*(*sub).swm).dir == PORT_DIR_OUTPUT {
            pos = (*sub).rd_offs;
            // Size of AIO output format is double of IEC61937
            (*tstamp).copied_total = (*sub).rd_total / 2;
        } else {
            pos = (*sub).wr_offs;
            (*tstamp).copied_total = (*sub).rd_total;
        }
        (*tstamp).byte_offset = pos;
        drop(_guard);

        return 0;
    }
}

fn aio_compr_send_to_hw(sub: *mut uniphier_aio_sub, buf: *mut u8, dstsize: usize) -> i32 {
    unsafe {
        let srcbuf = buf as *const u32;
        let dstbuf = ((*sub).compr_area as *const u8).add((*sub).wr_offs as usize) as *mut u32;
        let mut src = 0usize;
        let mut dst = 0usize;
        let ret: i32;
        let mut frm: u32;
        let frm_a: u32;
        let frm_b: u32;
        let mut dstsize = dstsize;

        while dstsize > 0 {
            ret = get_user(&mut frm, srcbuf.add(src));
            if ret != 0 {
                return ret;
            }
            src = src.wrapping_add(1);

            frm_a = frm & 0xffffu32;
            frm_b = (frm >> 16) & 0xffffu32;

            let mut frm_a_mut = frm_a;
            if frm == IEC61937_HEADER_SIGN {
                frm_a_mut |= 0x01000000u32;

                // Next data is Pc and Pd
                (*sub).iec_header = true;
            } else {
                let pc = be16_to_cpu(frm_a as u16);

                if (*sub).iec_header && (*sub).iec_pc != pc {
                    // Force overwrite IEC frame type
                    (*sub).iec_pc = pc;
                    ret = aio_oport_set_stream_type(sub, pc);
                    if ret != 0 {
                        return ret;
                    }
                }
                (*sub).iec_header = false;
            }
            *dstbuf.add(dst) = frm_a_mut;
            dst = dst.wrapping_add(1);
            *dstbuf.add(dst) = frm_b;
            dst = dst.wrapping_add(1);

            dstsize = dstsize.wrapping_sub(core::mem::size_of::<u32>() * 2);
        }

        return 0;
    }
}

fn uniphier_aio_compr_copy(
    component: *mut snd_soc_component,
    cstream: *mut snd_compr_stream,
    buf: *mut u8,
    count: usize,
) -> i32 {
    unsafe {
        let rtd = (*cstream).private_data as *mut snd_soc_pcm_runtime;
        let runtime = (*cstream).runtime;
        let carddev = (*(*rtd).compr).card.dev;
        let aio = uniphier_priv(snd_soc_rtd_to_cpu(rtd, 0));
        let sub = &mut (*aio).sub[(*cstream).direction as usize];
        let cnt = {
            let space = aio_rb_space_to_end(sub) / 2;
            if count < space { count } else { space }
        };
        let bytes = (*runtime).fragment_size;
        let mut s: usize;
        let ret: i32;

        if cnt < core::mem::size_of::<u32>() {
            return 0;
        }

        if (*(*sub).swm).dir == PORT_DIR_OUTPUT {
            let dmapos = (*sub).compr_addr + (*sub).wr_offs as usize;

            // Size of AIO output format is double of IEC61937
            s = cnt * 2;

            dma_sync_single_for_cpu(carddev, dmapos, s, DMA_TO_DEVICE);
            ret = aio_compr_send_to_hw(sub, buf, s);
            dma_sync_single_for_device(carddev, dmapos, s, DMA_TO_DEVICE);
        } else {
            let dmapos = (*sub).compr_addr + (*sub).rd_offs as usize;

            s = cnt;

            dma_sync_single_for_cpu(carddev, dmapos, s, DMA_FROM_DEVICE);
            ret = copy_to_user(
                buf,
                ((*sub).compr_area as *const u8).add((*sub).rd_offs as usize),
                s,
            );
            dma_sync_single_for_device(carddev, dmapos, s, DMA_FROM_DEVICE);
        }
        if ret != 0 {
            return -EFAULT;
        }

        let _guard = spinlock_irqsave_guard(&mut (*sub).lock);

        (*sub).threshold = 2 * bytes;
        aiodma_rb_set_threshold(sub, (*sub).compr_bytes, 2 * bytes);

        if (*(*sub).swm).dir == PORT_DIR_OUTPUT {
            (*sub).wr_offs += s as u32;
            if (*sub).wr_offs >= (*sub).compr_bytes {
                (*sub).wr_offs -= (*sub).compr_bytes;
            }
        } else {
            (*sub).rd_offs += s as u32;
            if (*sub).rd_offs >= (*sub).compr_bytes {
                (*sub).rd_offs -= (*sub).compr_bytes;
            }
        }
        aiodma_rb_sync(sub, (*sub).compr_addr, (*sub).compr_bytes, bytes);
        drop(_guard);

        return cnt as i32;
    }
}

fn uniphier_aio_compr_get_caps(
    component: *mut snd_soc_component,
    cstream: *mut snd_compr_stream,
    caps: *mut snd_compr_caps,
) -> i32 {
    unsafe {
        (*caps).num_codecs = 1;
        (*caps).min_fragment_size = AUD_MIN_FRAGMENT_SIZE;
        (*caps).max_fragment_size = AUD_MAX_FRAGMENT_SIZE;
        (*caps).min_fragments = AUD_MIN_FRAGMENT;
        (*caps).max_fragments = AUD_MAX_FRAGMENT;
        (*caps).codecs[0] = SND_AUDIOCODEC_IEC61937;

        return 0;
    }
}

#[repr(C)]
struct SndComprCodecCaps {
    num_descriptors: i32,
    descriptor: [SndComprCodecDescriptor; 1],
}

#[repr(C)]
struct SndComprCodecDescriptor {
    max_ch: u32,
    num_sample_rates: u32,
    num_bitrates: u32,
    profiles: u32,
    modes: u32,
    formats: u32,
}

static CAPS_IEC: SndComprCodecCaps = SndComprCodecCaps {
    num_descriptors: 1,
    descriptor: [SndComprCodecDescriptor {
        max_ch: 8,
        num_sample_rates: 0,
        num_bitrates: 0,
        profiles: SND_AUDIOPROFILE_IEC61937_SPDIF,
        modes: SND_AUDIOMODE_IEC_AC3 | SND_AUDIOMODE_IEC_MPEG1 | SND_AUDIOMODE_IEC_MP3 | SND_AUDIOMODE_IEC_DTS,
        formats: 0,
    }],
};

fn uniphier_aio_compr_get_codec_caps(
    component: *mut snd_soc_component,
    stream: *mut snd_compr_stream,
    codec: *mut snd_compr_codec_caps,
) -> i32 {
    unsafe {
        if (*codec).codec == SND_AUDIOCODEC_IEC61937 {
            *(codec as *mut SndComprCodecCaps) = CAPS_IEC;
        } else {
            return -EINVAL;
        }

        return 0;
    }
}

#[repr(C)]
pub struct SndCompressOps {
    pub open: unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream) -> i32,
    pub free: unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream) -> i32,
    pub get_params: unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, *mut snd_codec) -> i32,
    pub set_params: unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, *mut snd_compr_params) -> i32,
    pub trigger: unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, i32) -> i32,
    pub pointer: unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, *mut snd_compr_tstamp64) -> i32,
    pub copy: unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, *mut u8, usize) -> i32,
    pub get_caps: unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, *mut snd_compr_caps) -> i32,
    pub get_codec_caps: unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, *mut snd_compr_codec_caps) -> i32,
}

#[no_mangle]
pub static uniphier_aio_compress_ops: SndCompressOps = SndCompressOps {
    open: uniphier_aio_compr_open as unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream) -> i32,
    free: uniphier_aio_compr_free as unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream) -> i32,
    get_params: uniphier_aio_compr_get_params as unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, *mut snd_codec) -> i32,
    set_params: uniphier_aio_compr_set_params as unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, *mut snd_compr_params) -> i32,
    trigger: uniphier_aio_compr_trigger as unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, i32) -> i32,
    pointer: uniphier_aio_compr_pointer as unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, *mut snd_compr_tstamp64) -> i32,
    copy: uniphier_aio_compr_copy as unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, *mut u8, usize) -> i32,
    get_caps: uniphier_aio_compr_get_caps as unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, *mut snd_compr_caps) -> i32,
    get_codec_caps: uniphier_aio_compr_get_codec_caps as unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, *mut snd_compr_codec_caps) -> i32,
};

// External dependencies from "aio.h" and Linux kernel headers
extern "C" {
    fn uniphier_priv(snd_soc_cpu_dai: *mut snd_soc_dai) -> *mut uniphier_aio;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, index: i32) -> *mut snd_soc_dai;
    fn dma_set_mask_and_coherent(dev: *mut device, mask: u64) -> i32;
    fn kzalloc(size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn dma_map_single(dev: *mut device, ptr: *mut core::ffi::c_void, size: usize, direction: i32) -> usize;
    fn dma_mapping_error(dev: *mut device, addr: usize) -> i32;
    fn dma_unmap_single(dev: *mut device, addr: usize, size: usize, direction: i32);
    fn aio_init(sub: *mut uniphier_aio_sub) -> i32;
    fn aiodma_ch_set_param(sub: *mut uniphier_aio_sub) -> i32;
    fn aiodma_rb_set_buffer(sub: *mut uniphier_aio_sub, start: usize, end: usize, size: i32) -> i32;
    fn aio_port_set_param(sub: *mut uniphier_aio_sub, pass_through: i32, params: *mut core::ffi::c_void) -> i32;
    fn aio_oport_set_stream_type(sub: *mut uniphier_aio_sub, pc: u16) -> i32;
    fn aio_port_set_enable(sub: *mut uniphier_aio_sub, enable: i32);
    fn aio_if_set_param(sub: *mut uniphier_aio_sub, pass_through: i32) -> i32;
    fn aio_port_reset(sub: *mut uniphier_aio_sub);
    fn aio_src_reset(sub: *mut uniphier_aio_sub);
    fn aiodma_rb_sync(sub: *mut uniphier_aio_sub, compr_addr: usize, compr_bytes: u32, bytes: i32);
    fn aiodma_ch_set_enable(sub: *mut uniphier_aio_sub, enable: i32);
    fn aio_rb_space_to_end(sub: *mut uniphier_aio_sub) -> usize;
    fn aiodma_rb_set_threshold(sub: *mut uniphier_aio_sub, compr_bytes: u32, threshold: i32);
    fn dma_sync_single_for_cpu(dev: *mut device, addr: usize, size: usize, direction: i32);
    fn dma_sync_single_for_device(dev: *mut device, addr: usize, size: usize, direction: i32);
    fn get_user(dst: *mut u32, src: *const u32) -> i32;
    fn copy_to_user(dst: *mut u8, src: *const u8, n: usize) -> i32;
    fn be16_to_cpu(x: u16) -> u16;
    fn dev_err(dev: *const device, format: *const i8, ...);
    fn dev_warn(dev: *const device, format: *const i8, ...);
    fn spinlock_irqsave_guard(lock: *mut core::ffi::c_void) -> SpinlockGuard;
}

pub struct SpinlockGuard {
    _private: core::marker::PhantomData<()>,
}

// Type stubs for external structures
#[repr(C)]
struct snd_soc_component;
#[repr(C)]
struct snd_compr_stream;
#[repr(C)]
struct snd_soc_pcm_runtime;
#[repr(C)]
struct snd_compr;
#[repr(C)]
struct device;
#[repr(C)]
struct uniphier_aio;
#[repr(C)]
struct uniphier_aio_sub;
#[repr(C)]
struct snd_codec;
#[repr(C)]
struct snd_compr_params;
#[repr(C)]
struct snd_compr_runtime;
#[repr(C)]
struct snd_compr_tstamp64;
#[repr(C)]
struct snd_compr_caps;
#[repr(C)]
struct snd_compr_codec_caps;
#[repr(C)]
struct snd_soc_dai;

// Constants (from kernel headers and aio.h)
const DMA_FROM_DEVICE: i32 = 2;
const DMA_TO_DEVICE: i32 = 1;
const GFP_KERNEL: u32 = 0xd0;
const PORT_DIR_OUTPUT: i32 = 1;
const SNDRV_PCM_TRIGGER_START: i32 = 0;
const SNDRV_PCM_TRIGGER_STOP: i32 = 5;
const SND_AUDIOCODEC_IEC61937: u32 = 4;
const SND_AUDIOPROFILE_IEC61937_SPDIF: u32 = 0;
const SND_AUDIOMODE_IEC_AC3: u32 = 0;
const SND_AUDIOMODE_IEC_MPEG1: u32 = 1;
const SND_AUDIOMODE_IEC_MP3: u32 = 2;
const SND_AUDIOMODE_IEC_DTS: u32 = 3;
const IEC61937_HEADER_SIGN: u32 = 0xf8724e1f;
const IEC61937_PC_AAC: u16 = 0x0408;
const ENOMEM: i32 = -12;
const EBUSY: i32 = -16;
const EINVAL: i32 = -22;
const EFAULT: i32 = -14;

fn DMA_BIT_MASK(bits: u32) -> u64 {
    if bits == 64 {
        !0u64
    } else {
        (1u64 << bits) - 1
    }
}

// Constants from aio driver (to be imported from aio.h)
const AUD_RING_SIZE: usize = 65536;
const AUD_MIN_FRAGMENT_SIZE: u32 = 4096;
const AUD_MAX_FRAGMENT_SIZE: u32 = 16384;
const AUD_MIN_FRAGMENT: u32 = 2;
const AUD_MAX_FRAGMENT: u32 = 8;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
