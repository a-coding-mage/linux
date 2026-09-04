// SPDX-License-Identifier: GPL-2.0+
/*
 * virtio-snd: Virtio sound device
 * Copyright (C) 2021 OpenSynergy GmbH
 */

// Requires: linux/virtio_config.h (external)
// Requires: virtio_card.h (external)

use std::os::raw::c_void;

extern "C" {
    type virtio_device;
    type virtio_snd;
    type virtio_snd_config;
    type virtio_snd_chmap_info;
    type virtio_pcm;
    type virtio_pcm_stream;
    type snd_pcm;
    type snd_pcm_chmap_elem;

    // Constants from external headers
    static SNDRV_CHMAP_UNKNOWN: u8;
    static SNDRV_CHMAP_NA: u8;
    static SNDRV_CHMAP_MONO: u8;
    static SNDRV_CHMAP_FL: u8;
    static SNDRV_CHMAP_FR: u8;
    static SNDRV_CHMAP_RL: u8;
    static SNDRV_CHMAP_RR: u8;
    static SNDRV_CHMAP_FC: u8;
    static SNDRV_CHMAP_LFE: u8;
    static SNDRV_CHMAP_SL: u8;
    static SNDRV_CHMAP_SR: u8;
    static SNDRV_CHMAP_RC: u8;
    static SNDRV_CHMAP_FLC: u8;
    static SNDRV_CHMAP_FRC: u8;
    static SNDRV_CHMAP_RLC: u8;
    static SNDRV_CHMAP_RRC: u8;
    static SNDRV_CHMAP_FLW: u8;
    static SNDRV_CHMAP_FRW: u8;
    static SNDRV_CHMAP_FLH: u8;
    static SNDRV_CHMAP_FCH: u8;
    static SNDRV_CHMAP_FRH: u8;
    static SNDRV_CHMAP_TC: u8;
    static SNDRV_CHMAP_TFL: u8;
    static SNDRV_CHMAP_TFR: u8;
    static SNDRV_CHMAP_TFC: u8;
    static SNDRV_CHMAP_TRL: u8;
    static SNDRV_CHMAP_TRR: u8;
    static SNDRV_CHMAP_TRC: u8;
    static SNDRV_CHMAP_TFLC: u8;
    static SNDRV_CHMAP_TFRC: u8;
    static SNDRV_CHMAP_TSL: u8;
    static SNDRV_CHMAP_TSR: u8;
    static SNDRV_CHMAP_LLFE: u8;
    static SNDRV_CHMAP_RLFE: u8;
    static SNDRV_CHMAP_BC: u8;
    static SNDRV_CHMAP_BLC: u8;
    static SNDRV_CHMAP_BRC: u8;

    // External functions
    fn virtio_cread_le(
        vdev: *const virtio_device,
        config_type: *const c_void,
        member: *const c_void,
        buf: *mut c_void,
    );

    fn devm_kcalloc(
        dev: *const c_void,
        n: u32,
        size: usize,
        flags: u32,
    ) -> *mut c_void;

    fn virtsnd_ctl_query_info(
        snd: *mut virtio_snd,
        command: u32,
        start_id: u32,
        count: u32,
        size: usize,
        info: *mut c_void,
    ) -> i32;

    fn virtsnd_pcm_find_or_create(
        snd: *mut virtio_snd,
        nid: u32,
    ) -> *mut virtio_pcm;

    fn virtsnd_pcm_find(
        snd: *mut virtio_snd,
        nid: u32,
    ) -> *mut virtio_pcm;

    fn dev_err(dev: *const c_void, fmt: *const u8, ...);

    fn snd_pcm_add_chmap_ctls(
        pcm: *mut snd_pcm,
        direction: i32,
        chmaps: *mut snd_pcm_chmap_elem,
        max_channels: i32,
        null_entry: i32,
        null_ptr: *const c_void,
    ) -> i32;
}

// VirtIO->ALSA channel position map
static G_V2A_POSITION_MAP: &[u8] = &[
    SNDRV_CHMAP_UNKNOWN,
    SNDRV_CHMAP_NA,
    SNDRV_CHMAP_MONO,
    SNDRV_CHMAP_FL,
    SNDRV_CHMAP_FR,
    SNDRV_CHMAP_RL,
    SNDRV_CHMAP_RR,
    SNDRV_CHMAP_FC,
    SNDRV_CHMAP_LFE,
    SNDRV_CHMAP_SL,
    SNDRV_CHMAP_SR,
    SNDRV_CHMAP_RC,
    SNDRV_CHMAP_FLC,
    SNDRV_CHMAP_FRC,
    SNDRV_CHMAP_RLC,
    SNDRV_CHMAP_RRC,
    SNDRV_CHMAP_FLW,
    SNDRV_CHMAP_FRW,
    SNDRV_CHMAP_FLH,
    SNDRV_CHMAP_FCH,
    SNDRV_CHMAP_FRH,
    SNDRV_CHMAP_TC,
    SNDRV_CHMAP_TFL,
    SNDRV_CHMAP_TFR,
    SNDRV_CHMAP_TFC,
    SNDRV_CHMAP_TRL,
    SNDRV_CHMAP_TRR,
    SNDRV_CHMAP_TRC,
    SNDRV_CHMAP_TFLC,
    SNDRV_CHMAP_TFRC,
    SNDRV_CHMAP_TSL,
    SNDRV_CHMAP_TSR,
    SNDRV_CHMAP_LLFE,
    SNDRV_CHMAP_RLFE,
    SNDRV_CHMAP_BC,
    SNDRV_CHMAP_BLC,
    SNDRV_CHMAP_BRC,
];

#[inline]
unsafe fn is_err<T>(ptr: *const T) -> bool {
    (ptr as usize) >= (-4096isize as usize)
}

#[inline]
unsafe fn ptr_err<T>(ptr: *const T) -> i32 {
    -(ptr as usize as i32)
}

/// Parse the channel map configuration.
///
/// This function is called during initial device initialization.
///
/// Context: Any context that permits to sleep.
/// Return: 0 on success, -errno on failure.
pub unsafe extern "C" fn virtsnd_chmap_parse_cfg(snd: *mut virtio_snd) -> i32 {
    let vdev = (*snd).vdev;
    let mut i: u32;
    let rc: i32;

    virtio_cread_le(vdev, std::ptr::null(), std::ptr::null(), &mut (*snd).nchmaps as *mut _ as *mut c_void);
    if (*snd).nchmaps == 0 {
        return 0;
    }

    (*snd).chmaps = devm_kcalloc(
        &(*vdev).dev as *const _ as *const c_void,
        (*snd).nchmaps,
        std::mem::size_of::<virtio_snd_chmap_info>(),
        0,
    ) as *mut _;
    if (*snd).chmaps.is_null() {
        return -12;
    }

    rc = virtsnd_ctl_query_info(
        snd,
        0,
        0,
        (*snd).nchmaps,
        std::mem::size_of::<virtio_snd_chmap_info>(),
        (*snd).chmaps as *mut c_void,
    );
    if rc != 0 {
        return rc;
    }

    i = 0;
    while i < (*snd).nchmaps {
        let info = &*(*snd).chmaps.add(i as usize);
        let nid = u32::from_le((*info).hdr.hda_fn_nid);
        let vpcm: *mut virtio_pcm;
        let vs: *mut virtio_pcm_stream;

        vpcm = virtsnd_pcm_find_or_create(snd, nid);
        if is_err(vpcm as *const c_void) {
            return ptr_err(vpcm as *const c_void);
        }

        match (*info).direction {
            0 => {
                vs = &mut (*vpcm).streams[0];
            }
            1 => {
                vs = &mut (*vpcm).streams[1];
            }
            _ => {
                dev_err(
                    &(*vdev).dev as *const _ as *const c_void,
                    b"chmap #%u: unknown direction (%u)\n\0" as *const u8,
                    i,
                    (*info).direction,
                );
                return -22;
            }
        }

        (*vs).nchmaps += 1;
        i += 1;
    }

    0
}

/// Create an ALSA control for channel maps.
///
/// Context: Any context.
/// Return: 0 on success, -errno on failure.
unsafe fn virtsnd_chmap_add_ctls(
    pcm: *mut snd_pcm,
    direction: i32,
    vs: *mut virtio_pcm_stream,
) -> i32 {
    let mut i: u32 = 0;
    let mut max_channels: i32 = 0;

    while i < (*vs).nchmaps {
        if max_channels < (*(*vs).chmaps.add(i as usize)).channels as i32 {
            max_channels = (*(*vs).chmaps.add(i as usize)).channels as i32;
        }
        i += 1;
    }

    snd_pcm_add_chmap_ctls(
        pcm,
        direction,
        (*vs).chmaps,
        max_channels,
        0,
        std::ptr::null(),
    )
}

/// Build ALSA controls for channel maps.
///
/// Context: Any context.
/// Return: 0 on success, -errno on failure.
pub unsafe extern "C" fn virtsnd_chmap_build_devs(snd: *mut virtio_snd) -> i32 {
    let vdev = (*snd).vdev;
    let mut rc: i32;
    let mut i: u32;
    let mut ch: u32;

    // TODO: Allocate channel map elements per each PCM device/stream.
    // Requires list_for_each_entry iteration over snd->pcm_list

    i = 0;
    while i < (*snd).nchmaps {
        let info = &*(*snd).chmaps.add(i as usize);
        let mut channels = (*info).channels as u32;
        let nid = u32::from_le((*info).hdr.hda_fn_nid);
        let vpcm: *mut virtio_pcm;
        let vs: *mut virtio_pcm_stream;
        let chmap: *mut snd_pcm_chmap_elem;

        vpcm = virtsnd_pcm_find(snd, nid);
        if is_err(vpcm as *const c_void) {
            return ptr_err(vpcm as *const c_void);
        }

        if (*info).direction == 0 {
            vs = &mut (*vpcm).streams[0];
        } else {
            vs = &mut (*vpcm).streams[1];
        }

        let nchmaps_idx = (*vs).nchmaps as usize;
        chmap = &mut *(*vs).chmaps.add(nchmaps_idx);
        (*vs).nchmaps = (*vs).nchmaps.wrapping_add(1);

        if channels > std::mem::size_of_val(&(*chmap).map) / std::mem::size_of_val(&(*chmap).map[0]) as u32 {
            channels = std::mem::size_of_val(&(*chmap).map) / std::mem::size_of_val(&(*chmap).map[0]) as u32;
        }

        (*chmap).channels = channels;

        ch = 0;
        while ch < channels {
            let position = *(*info).positions.add(ch as usize);

            if (position as usize) >= G_V2A_POSITION_MAP.len() {
                return -22;
            }

            *(*chmap).map.add(ch as usize) = G_V2A_POSITION_MAP[position as usize];
            ch += 1;
        }

        i += 1;
    }

    // TODO: Create an ALSA control per each PCM device/stream.
    // Requires list_for_each_entry iteration over snd->pcm_list

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
