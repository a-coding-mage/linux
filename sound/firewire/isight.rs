// SPDX-License-Identifier: GPL-2.0-only
/*
 * Apple iSight audio driver
 *
 * Copyright (c) Clemens Ladisch <clemens@ladisch.de>
 */

// C dependencies translated as surrounding Rust/kernel dependencies:
// asm/byteorder.h, linux/delay.h, linux/device.h, linux/firewire.h,
// linux/firewire-constants.h, linux/module.h, linux/mutex.h, linux/string.h,
// sound/control.h, sound/core.h, sound/initval.h, sound/pcm.h, sound/tlv.h,
// "lib.h", "iso-resources.h", "packets-buffer.h"
use crate::*;

pub const OUI_APPLE: u32 = 0x000a27;
pub const MODEL_APPLE_ISIGHT: u32 = 0x000008;
pub const SW_ISIGHT_AUDIO: u32 = 0x000010;

pub const REG_AUDIO_ENABLE: u64 = 0x000;
pub const AUDIO_ENABLE: u32 = 0x80000000;
pub const REG_DEF_AUDIO_GAIN: u64 = 0x204;
pub const REG_GAIN_RAW_START: u64 = 0x210;
pub const REG_GAIN_RAW_END: u64 = 0x214;
pub const REG_GAIN_DB_START: u64 = 0x218;
pub const REG_GAIN_DB_END: u64 = 0x21c;
pub const REG_SAMPLE_RATE_INQUIRY: u64 = 0x280;
pub const REG_ISO_TX_CONFIG: u64 = 0x300;
pub const SPEED_SHIFT: u32 = 16;
pub const REG_SAMPLE_RATE: u64 = 0x400;
pub const RATE_48000: u32 = 0x80000000;
pub const REG_GAIN: u64 = 0x500;
pub const REG_MUTE: u64 = 0x504;

pub const MAX_FRAMES_PER_PACKET: usize = 475;

pub const QUEUE_LENGTH: usize = 20;

#[repr(C)]
pub struct isight {
    pub card: *mut snd_card,
    pub unit: *mut fw_unit,
    pub device: *mut fw_device,
    pub audio_base: u64,
    pub pcm: *mut snd_pcm_substream,
    pub mutex: mutex,
    pub buffer: iso_packets_buffer,
    pub resources: fw_iso_resources,
    pub context: *mut fw_iso_context,
    pub pcm_active: bool,
    pub pcm_running: bool,
    pub first_packet: bool,
    pub packet_index: i32,
    pub total_samples: u32,
    pub buffer_pointer: c_uint,
    pub period_counter: c_uint,
    pub gain_min: i32,
    pub gain_max: i32,
    pub gain_tlv: [c_uint; 4],
}

#[repr(C)]
pub struct audio_payload {
    pub sample_count: __be32,
    pub signature: __be32,
    pub sample_total: __be32,
    pub reserved: __be32,
    pub samples: [__be16; 2 * MAX_FRAMES_PER_PACKET],
}

module_description!("iSight audio driver");
module_author!("Clemens Ladisch <clemens@ladisch.de>");
module_license!("GPL");

static mut audio_packet: fw_iso_packet = fw_iso_packet {
    payload_length: core::mem::size_of::<audio_payload>(),
    interrupt: 1,
    header_length: 4,
};

unsafe fn isight_update_pointers(isight: *mut isight, count: c_uint) {
    let runtime: *mut snd_pcm_runtime = (*(*isight).pcm).runtime;
    let mut ptr: c_uint;

    smp_wmb(); /* update buffer data before buffer pointer */

    ptr = (*isight).buffer_pointer;
    ptr = ptr.wrapping_add(count);
    if ptr >= (*runtime).buffer_size {
        ptr = ptr.wrapping_sub((*runtime).buffer_size);
    }
    WRITE_ONCE(&mut (*isight).buffer_pointer, ptr);

    (*isight).period_counter = (*isight).period_counter.wrapping_add(count);
    if (*isight).period_counter >= (*runtime).period_size {
        (*isight).period_counter =
            (*isight).period_counter.wrapping_sub((*runtime).period_size);
        snd_pcm_period_elapsed((*isight).pcm);
    }
}

unsafe fn isight_samples(isight: *mut isight, mut samples: *const __be16, count: c_uint) {
    let runtime: *mut snd_pcm_runtime;
    let count1: c_uint;

    if !READ_ONCE(&(*isight).pcm_running) {
        return;
    }

    runtime = (*(*isight).pcm).runtime;
    if (*isight).buffer_pointer.wrapping_add(count) <= (*runtime).buffer_size {
        memcpy(
            (*runtime)
                .dma_area
                .add((*isight).buffer_pointer.wrapping_mul(4) as usize) as *mut c_void,
            samples as *const c_void,
            count.wrapping_mul(4) as usize,
        );
    } else {
        count1 = (*runtime).buffer_size.wrapping_sub((*isight).buffer_pointer);
        memcpy(
            (*runtime)
                .dma_area
                .add((*isight).buffer_pointer.wrapping_mul(4) as usize) as *mut c_void,
            samples as *const c_void,
            count1.wrapping_mul(4) as usize,
        );
        samples = samples.add(count1.wrapping_mul(2) as usize);
        memcpy(
            (*runtime).dma_area as *mut c_void,
            samples as *const c_void,
            count.wrapping_sub(count1).wrapping_mul(4) as usize,
        );
    }

    isight_update_pointers(isight, count);
}

unsafe fn isight_pcm_abort(isight: *mut isight) {
    if READ_ONCE(&(*isight).pcm_active) {
        snd_pcm_stop_xrun((*isight).pcm);
    }
}

unsafe fn isight_dropped_samples(isight: *mut isight, total: c_uint) {
    let runtime: *mut snd_pcm_runtime;
    let dropped: u32;
    let count1: c_uint;

    if !READ_ONCE(&(*isight).pcm_running) {
        return;
    }

    runtime = (*(*isight).pcm).runtime;
    dropped = total.wrapping_sub((*isight).total_samples);
    if dropped < (*runtime).buffer_size {
        if (*isight).buffer_pointer.wrapping_add(dropped) <= (*runtime).buffer_size {
            memset(
                (*runtime)
                    .dma_area
                    .add((*isight).buffer_pointer.wrapping_mul(4) as usize) as *mut c_void,
                0,
                dropped.wrapping_mul(4) as usize,
            );
        } else {
            count1 = (*runtime).buffer_size.wrapping_sub((*isight).buffer_pointer);
            memset(
                (*runtime)
                    .dma_area
                    .add((*isight).buffer_pointer.wrapping_mul(4) as usize) as *mut c_void,
                0,
                count1.wrapping_mul(4) as usize,
            );
            memset(
                (*runtime).dma_area as *mut c_void,
                0,
                dropped.wrapping_sub(count1).wrapping_mul(4) as usize,
            );
        }
        isight_update_pointers(isight, dropped);
    } else {
        isight_pcm_abort(isight);
    }
}

unsafe extern "C" fn isight_packet(
    _context: *mut fw_iso_context,
    _cycle: u32,
    _header_length: usize,
    header: *mut c_void,
    data: *mut c_void,
) {
    let isight: *mut isight = data as *mut isight;
    let payload: *const audio_payload;
    let mut index: c_uint;
    let length: c_uint;
    let count: c_uint;
    let total: c_uint;
    let err: c_int;

    if (*isight).packet_index < 0 {
        return;
    }
    index = (*isight).packet_index as c_uint;
    payload = (*isight).buffer.packets[index as usize].buffer as *const audio_payload;
    length = be32_to_cpup(header as *const __be32) >> 16;

    if likely(
        length >= 16
            && (*payload).signature == cpu_to_be32(0x73676874), /*"sght"*/
    ) {
        count = be32_to_cpu((*payload).sample_count);
        if likely(count <= length.wrapping_sub(16) / 4 && count <= MAX_FRAMES_PER_PACKET as c_uint)
        {
            total = be32_to_cpu((*payload).sample_total);
            if unlikely(total != (*isight).total_samples) {
                if !(*isight).first_packet {
                    isight_dropped_samples(isight, total);
                }
                (*isight).first_packet = false;
                (*isight).total_samples = total;
            }

            isight_samples(isight, (*payload).samples.as_ptr(), count);
            (*isight).total_samples = (*isight).total_samples.wrapping_add(count);
        }
    }

    err = fw_iso_context_queue(
        (*isight).context,
        &raw const audio_packet,
        &mut (*isight).buffer.iso_buffer,
        (*isight).buffer.packets[index as usize].offset,
    );
    if err < 0 {
        dev_err(
            &mut (*(*isight).unit).device,
            c_str!("queueing error: %d\n"),
            err,
        );
        isight_pcm_abort(isight);
        (*isight).packet_index = -1;
        return;
    }
    fw_iso_context_queue_flush((*isight).context);

    index = index.wrapping_add(1);
    if index >= QUEUE_LENGTH as c_uint {
        index = 0;
    }
    (*isight).packet_index = index as c_int;
}

unsafe fn isight_connect(isight: *mut isight) -> c_int {
    let mut ch: c_int;
    let mut err: c_int;
    let value: __be32;

    loop {
        ch = fw_iso_resources_allocate(
            &mut (*isight).resources,
            core::mem::size_of::<audio_payload>(),
            (*(*isight).device).max_speed,
        );
        if ch < 0 {
            err = ch;
            break;
        }

        value = cpu_to_be32((ch as u32) | ((*(*isight).device).max_speed << SPEED_SHIFT));
        err = snd_fw_transaction(
            (*isight).unit,
            TCODE_WRITE_QUADLET_REQUEST,
            (*isight).audio_base + REG_ISO_TX_CONFIG,
            &value as *const __be32 as *mut c_void,
            4,
            FW_FIXED_GENERATION | (*isight).resources.generation,
        );
        if err == -EAGAIN {
            fw_iso_resources_free(&mut (*isight).resources);
            continue;
        } else if err < 0 {
            fw_iso_resources_free(&mut (*isight).resources);
            break;
        }

        return 0;
    }

    err
}

unsafe extern "C" fn isight_open(substream: *mut snd_pcm_substream) -> c_int {
    static hardware: snd_pcm_hardware = snd_pcm_hardware {
        info: SNDRV_PCM_INFO_MMAP
            | SNDRV_PCM_INFO_MMAP_VALID
            | SNDRV_PCM_INFO_BATCH
            | SNDRV_PCM_INFO_INTERLEAVED
            | SNDRV_PCM_INFO_BLOCK_TRANSFER,
        formats: SNDRV_PCM_FMTBIT_S16_BE,
        rates: SNDRV_PCM_RATE_48000,
        rate_min: 48000,
        rate_max: 48000,
        channels_min: 2,
        channels_max: 2,
        buffer_bytes_max: 4 * 1024 * 1024,
        period_bytes_min: (MAX_FRAMES_PER_PACKET * 4) as c_uint,
        period_bytes_max: 1024 * 1024,
        periods_min: 2,
        periods_max: UINT_MAX,
    };
    let isight: *mut isight = (*substream).private_data as *mut isight;

    (*(*substream).runtime).hw = hardware;

    iso_packets_buffer_init(
        &mut (*isight).buffer,
        (*isight).unit,
        QUEUE_LENGTH,
        core::mem::size_of::<audio_payload>(),
        DMA_FROM_DEVICE,
    )
}

unsafe extern "C" fn isight_close(substream: *mut snd_pcm_substream) -> c_int {
    let isight: *mut isight = (*substream).private_data as *mut isight;

    iso_packets_buffer_destroy(&mut (*isight).buffer, (*isight).unit);

    0
}

unsafe extern "C" fn isight_hw_params(
    substream: *mut snd_pcm_substream,
    _hw_params: *mut snd_pcm_hw_params,
) -> c_int {
    let isight: *mut isight = (*substream).private_data as *mut isight;

    WRITE_ONCE(&mut (*isight).pcm_active, true);

    0
}

unsafe fn reg_read(isight: *mut isight, offset: c_int, value: *mut __be32) -> c_int {
    snd_fw_transaction(
        (*isight).unit,
        TCODE_READ_QUADLET_REQUEST,
        (*isight).audio_base + offset as u64,
        value as *mut c_void,
        4,
        0,
    )
}

unsafe fn reg_write(isight: *mut isight, offset: u64, value: __be32) -> c_int {
    snd_fw_transaction(
        (*isight).unit,
        TCODE_WRITE_QUADLET_REQUEST,
        (*isight).audio_base + offset,
        &value as *const __be32 as *mut c_void,
        4,
        0,
    )
}

unsafe fn isight_stop_streaming(isight: *mut isight) {
    let value: __be32;

    if (*isight).context.is_null() {
        return;
    }

    fw_iso_context_stop((*isight).context);
    fw_iso_context_destroy((*isight).context);
    (*isight).context = core::ptr::null_mut();
    fw_iso_resources_free(&mut (*isight).resources);
    value = 0;
    snd_fw_transaction(
        (*isight).unit,
        TCODE_WRITE_QUADLET_REQUEST,
        (*isight).audio_base + REG_AUDIO_ENABLE,
        &value as *const __be32 as *mut c_void,
        4,
        FW_QUIET,
    );
}

unsafe extern "C" fn isight_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    let isight: *mut isight = (*substream).private_data as *mut isight;

    WRITE_ONCE(&mut (*isight).pcm_active, false);

    mutex_lock(&mut (*isight).mutex);
    isight_stop_streaming(isight);
    mutex_unlock(&mut (*isight).mutex);

    0
}

unsafe fn isight_start_streaming(isight: *mut isight) -> c_int {
    let mut i: c_uint;
    let mut err: c_int;

    if !(*isight).context.is_null() {
        if (*isight).packet_index < 0 {
            isight_stop_streaming(isight);
        } else {
            return 0;
        }
    }

    err = reg_write(isight, REG_SAMPLE_RATE, cpu_to_be32(RATE_48000));
    if err < 0 {
        return err;
    }

    err = isight_connect(isight);
    if err < 0 {
        return err;
    }

    err = reg_write(isight, REG_AUDIO_ENABLE, cpu_to_be32(AUDIO_ENABLE));
    if err < 0 {
        fw_iso_resources_free(&mut (*isight).resources);
        return err;
    }

    (*isight).context = fw_iso_context_create(
        (*(*isight).device).card,
        FW_ISO_CONTEXT_RECEIVE,
        (*isight).resources.channel,
        (*(*isight).device).max_speed,
        4,
        Some(isight_packet),
        isight as *mut c_void,
    );
    if IS_ERR((*isight).context as *const c_void) {
        err = PTR_ERR((*isight).context as *const c_void) as c_int;
        (*isight).context = core::ptr::null_mut();
        fw_iso_resources_free(&mut (*isight).resources);
        reg_write(isight, REG_AUDIO_ENABLE, 0);
        return err;
    }

    i = 0;
    while i < QUEUE_LENGTH as c_uint {
        err = fw_iso_context_queue(
            (*isight).context,
            &raw const audio_packet,
            &mut (*isight).buffer.iso_buffer,
            (*isight).buffer.packets[i as usize].offset,
        );
        if err < 0 {
            fw_iso_context_destroy((*isight).context);
            (*isight).context = core::ptr::null_mut();
            fw_iso_resources_free(&mut (*isight).resources);
            reg_write(isight, REG_AUDIO_ENABLE, 0);
            return err;
        }
        i = i.wrapping_add(1);
    }

    (*isight).first_packet = true;
    (*isight).packet_index = 0;

    err = fw_iso_context_start((*isight).context, -1, 0, FW_ISO_CONTEXT_MATCH_ALL_TAGS);
    if err < 0 {
        fw_iso_context_destroy((*isight).context);
        (*isight).context = core::ptr::null_mut();
        fw_iso_resources_free(&mut (*isight).resources);
        reg_write(isight, REG_AUDIO_ENABLE, 0);
        return err;
    }

    0
}

unsafe extern "C" fn isight_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let isight: *mut isight = (*substream).private_data as *mut isight;
    let ret: c_int;

    (*isight).buffer_pointer = 0;
    (*isight).period_counter = 0;

    mutex_lock(&mut (*isight).mutex);
    ret = isight_start_streaming(isight);
    mutex_unlock(&mut (*isight).mutex);
    ret
}

unsafe extern "C" fn isight_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let isight: *mut isight = (*substream).private_data as *mut isight;

    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            WRITE_ONCE(&mut (*isight).pcm_running, true);
        }
        SNDRV_PCM_TRIGGER_STOP => {
            WRITE_ONCE(&mut (*isight).pcm_running, false);
        }
        _ => {
            return -EINVAL;
        }
    }
    0
}

unsafe extern "C" fn isight_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let isight: *mut isight = (*substream).private_data as *mut isight;

    READ_ONCE(&(*isight).buffer_pointer) as snd_pcm_uframes_t
}

unsafe fn isight_create_pcm(isight: *mut isight) -> c_int {
    static ops: snd_pcm_ops = snd_pcm_ops {
        open: Some(isight_open),
        close: Some(isight_close),
        hw_params: Some(isight_hw_params),
        hw_free: Some(isight_hw_free),
        prepare: Some(isight_prepare),
        trigger: Some(isight_trigger),
        pointer: Some(isight_pointer),
    };
    let mut pcm: *mut snd_pcm = core::ptr::null_mut();
    let err: c_int;

    err = snd_pcm_new((*isight).card, c_str!("iSight"), 0, 0, 1, &mut pcm);
    if err < 0 {
        return err;
    }
    (*pcm).private_data = isight as *mut c_void;
    (*pcm).nonatomic = true;
    strscpy((*pcm).name.as_mut_ptr(), c_str!("iSight"));
    (*isight).pcm = (*pcm).streams[SNDRV_PCM_STREAM_CAPTURE as usize].substream;
    (*(*isight).pcm).ops = &ops;
    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_VMALLOC, core::ptr::null_mut(), 0, 0);

    0
}

unsafe extern "C" fn isight_gain_info(
    ctl: *mut snd_kcontrol,
    info: *mut snd_ctl_elem_info,
) -> c_int {
    let isight: *mut isight = (*ctl).private_data as *mut isight;

    (*info).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*info).count = 1;
    (*info).value.integer.min = (*isight).gain_min as c_long;
    (*info).value.integer.max = (*isight).gain_max as c_long;

    0
}

unsafe extern "C" fn isight_gain_get(
    ctl: *mut snd_kcontrol,
    value: *mut snd_ctl_elem_value,
) -> c_int {
    let isight: *mut isight = (*ctl).private_data as *mut isight;
    let mut gain: __be32 = 0;
    let err: c_int;

    err = reg_read(isight, REG_GAIN as c_int, &mut gain);
    if err < 0 {
        return err;
    }

    (*value).value.integer.value[0] = be32_to_cpu(gain) as i32 as c_long;

    0
}

unsafe extern "C" fn isight_gain_put(
    ctl: *mut snd_kcontrol,
    value: *mut snd_ctl_elem_value,
) -> c_int {
    let isight: *mut isight = (*ctl).private_data as *mut isight;

    if (*value).value.integer.value[0] < (*isight).gain_min as c_long
        || (*value).value.integer.value[0] > (*isight).gain_max as c_long
    {
        return -EINVAL;
    }

    reg_write(
        isight,
        REG_GAIN,
        cpu_to_be32((*value).value.integer.value[0] as u32),
    )
}

unsafe extern "C" fn isight_mute_get(
    ctl: *mut snd_kcontrol,
    value: *mut snd_ctl_elem_value,
) -> c_int {
    let isight: *mut isight = (*ctl).private_data as *mut isight;
    let mut mute: __be32 = 0;
    let err: c_int;

    err = reg_read(isight, REG_MUTE as c_int, &mut mute);
    if err < 0 {
        return err;
    }

    (*value).value.integer.value[0] = if mute == 0 { 1 } else { 0 };

    0
}

unsafe extern "C" fn isight_mute_put(
    ctl: *mut snd_kcontrol,
    value: *mut snd_ctl_elem_value,
) -> c_int {
    let isight: *mut isight = (*ctl).private_data as *mut isight;

    reg_write(
        isight,
        REG_MUTE,
        if (*value).value.integer.value[0] == 0 { 1 } else { 0 } as __be32,
    )
}

unsafe fn isight_create_mixer(isight: *mut isight) -> c_int {
    static gain_control: snd_kcontrol_new = snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: c_str!("Mic Capture Volume"),
        access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
        info: Some(isight_gain_info),
        get: Some(isight_gain_get),
        put: Some(isight_gain_put),
    };
    static mute_control: snd_kcontrol_new = snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: c_str!("Mic Capture Switch"),
        info: Some(snd_ctl_boolean_mono_info),
        get: Some(isight_mute_get),
        put: Some(isight_mute_put),
    };
    let mut value: __be32 = 0;
    let mut ctl: *mut snd_kcontrol;
    let mut err: c_int;

    err = reg_read(isight, REG_GAIN_RAW_START as c_int, &mut value);
    if err < 0 {
        return err;
    }
    (*isight).gain_min = be32_to_cpu(value) as i32;

    err = reg_read(isight, REG_GAIN_RAW_END as c_int, &mut value);
    if err < 0 {
        return err;
    }
    (*isight).gain_max = be32_to_cpu(value) as i32;

    (*isight).gain_tlv[SNDRV_CTL_TLVO_TYPE as usize] = SNDRV_CTL_TLVT_DB_MINMAX;
    (*isight).gain_tlv[SNDRV_CTL_TLVO_LEN as usize] = 2 * core::mem::size_of::<c_uint>() as c_uint;

    err = reg_read(isight, REG_GAIN_DB_START as c_int, &mut value);
    if err < 0 {
        return err;
    }
    (*isight).gain_tlv[SNDRV_CTL_TLVO_DB_MINMAX_MIN as usize] =
        (be32_to_cpu(value) as i32).wrapping_mul(100) as c_uint;

    err = reg_read(isight, REG_GAIN_DB_END as c_int, &mut value);
    if err < 0 {
        return err;
    }
    (*isight).gain_tlv[SNDRV_CTL_TLVO_DB_MINMAX_MAX as usize] =
        (be32_to_cpu(value) as i32).wrapping_mul(100) as c_uint;

    ctl = snd_ctl_new1(&gain_control, isight as *mut c_void);
    if !ctl.is_null() {
        (*ctl).tlv.p = (*isight).gain_tlv.as_mut_ptr();
    }
    err = snd_ctl_add((*isight).card, ctl);
    if err < 0 {
        return err;
    }

    err = snd_ctl_add(
        (*isight).card,
        snd_ctl_new1(&mute_control, isight as *mut c_void),
    );
    if err < 0 {
        return err;
    }

    0
}

unsafe extern "C" fn isight_card_free(card: *mut snd_card) {
    let isight: *mut isight = (*card).private_data as *mut isight;

    fw_iso_resources_destroy(&mut (*isight).resources);
}

unsafe fn get_unit_base(unit: *mut fw_unit) -> u64 {
    let mut i: fw_csr_iterator = core::mem::zeroed();
    let mut key: c_int = 0;
    let mut value: c_int = 0;

    fw_csr_iterator_init(&mut i, (*unit).directory);
    while fw_csr_iterator_next(&mut i, &mut key, &mut value) {
        if key == CSR_OFFSET {
            return CSR_REGISTER_BASE + (value as u64).wrapping_mul(4);
        }
    }
    0
}

unsafe extern "C" fn isight_probe(
    unit: *mut fw_unit,
    _id: *const ieee1394_device_id,
) -> c_int {
    let fw_dev: *mut fw_device = fw_parent_device(unit);
    let mut card: *mut snd_card = core::ptr::null_mut();
    let isight: *mut isight;
    let mut err: c_int;

    err = snd_card_new(
        &mut (*unit).device,
        -1,
        core::ptr::null(),
        THIS_MODULE,
        core::mem::size_of::<isight>(),
        &mut card,
    );
    if err < 0 {
        return err;
    }

    isight = (*card).private_data as *mut isight;
    (*isight).card = card;
    mutex_init(&mut (*isight).mutex);
    (*isight).unit = fw_unit_get(unit);
    (*isight).device = fw_dev;
    (*isight).audio_base = get_unit_base(unit);
    if (*isight).audio_base == 0 {
        dev_err(&mut (*unit).device, c_str!("audio unit base not found\n"));
        err = -ENXIO;
        snd_card_free(card);
        mutex_destroy(&mut (*isight).mutex);
        fw_unit_put((*isight).unit);
        return err;
    }
    fw_iso_resources_init(&mut (*isight).resources, unit);

    (*card).private_free = Some(isight_card_free);

    strscpy((*card).driver.as_mut_ptr(), c_str!("iSight"));
    strscpy((*card).shortname.as_mut_ptr(), c_str!("Apple iSight"));
    snprintf(
        (*card).longname.as_mut_ptr(),
        (*card).longname.len(),
        c_str!("Apple iSight (GUID %08x%08x) at %s, S%d"),
        (*fw_dev).config_rom[3],
        (*fw_dev).config_rom[4],
        dev_name(&mut (*unit).device),
        100 << (*fw_dev).max_speed,
    );
    strscpy((*card).mixername.as_mut_ptr(), c_str!("iSight"));

    err = isight_create_pcm(isight);
    if err < 0 {
        snd_card_free(card);
        mutex_destroy(&mut (*isight).mutex);
        fw_unit_put((*isight).unit);
        return err;
    }

    err = isight_create_mixer(isight);
    if err < 0 {
        snd_card_free(card);
        mutex_destroy(&mut (*isight).mutex);
        fw_unit_put((*isight).unit);
        return err;
    }

    err = snd_card_register(card);
    if err < 0 {
        snd_card_free(card);
        mutex_destroy(&mut (*isight).mutex);
        fw_unit_put((*isight).unit);
        return err;
    }

    dev_set_drvdata(&mut (*unit).device, isight as *mut c_void);

    0
}

unsafe extern "C" fn isight_bus_reset(unit: *mut fw_unit) {
    let isight: *mut isight = dev_get_drvdata(&mut (*unit).device) as *mut isight;

    if fw_iso_resources_update(&mut (*isight).resources) < 0 {
        isight_pcm_abort(isight);

        mutex_lock(&mut (*isight).mutex);
        isight_stop_streaming(isight);
        mutex_unlock(&mut (*isight).mutex);
    }
}

unsafe extern "C" fn isight_remove(unit: *mut fw_unit) {
    let isight: *mut isight = dev_get_drvdata(&mut (*unit).device) as *mut isight;

    isight_pcm_abort(isight);

    snd_card_disconnect((*isight).card);

    mutex_lock(&mut (*isight).mutex);
    isight_stop_streaming(isight);
    mutex_unlock(&mut (*isight).mutex);

    // Block till all of ALSA character devices are released.
    snd_card_free((*isight).card);

    mutex_destroy(&mut (*isight).mutex);
    fw_unit_put((*isight).unit);
}

static isight_id_table: [ieee1394_device_id; 2] = [
    ieee1394_device_id {
        match_flags: IEEE1394_MATCH_SPECIFIER_ID | IEEE1394_MATCH_VERSION,
        specifier_id: OUI_APPLE,
        version: SW_ISIGHT_AUDIO,
    },
    ieee1394_device_id::default(),
];
module_device_table!(ieee1394, isight_id_table);

static mut isight_driver: fw_driver = fw_driver {
    driver: device_driver {
        owner: THIS_MODULE,
        name: KBUILD_MODNAME,
        bus: &raw mut fw_bus_type,
    },
    probe: Some(isight_probe),
    update: Some(isight_bus_reset),
    remove: Some(isight_remove),
    id_table: isight_id_table.as_ptr(),
};

unsafe fn alsa_isight_init() -> c_int {
    driver_register(&mut isight_driver.driver)
}

unsafe fn alsa_isight_exit() {
    driver_unregister(&mut isight_driver.driver);
}

module_init!(alsa_isight_init);
module_exit!(alsa_isight_exit);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
