// SPDX-License-Identifier: GPL-2.0-only
/*
 * fireworks_proc.c - a part of driver for Fireworks based devices
 *
 * Copyright (c) 2009-2010 Clemens Ladisch
 * Copyright (c) 2013-2014 Takashi Sakamoto
 */

// Translated from ./fireworks.h dependencies.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;
type u32 = c_uint;

const GFP_KERNEL: c_uint = 0;
const S_IFDIR: c_uint = 0o040000;

#[repr(C)]
pub struct snd_card {
    pub proc_root: *mut snd_info_entry,
}

#[repr(C)]
pub struct snd_info_entry {
    pub private_data: *mut c_void,
    pub mode: c_uint,
}

#[repr(C)]
pub struct snd_info_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_efw_phys_grp {
    pub type_: c_uint,
    pub count: c_uint,
}

#[repr(C)]
pub struct snd_efw {
    pub card: *mut snd_card,
    pub phys_in: c_uint,
    pub phys_out: c_uint,
    pub phys_in_grp_count: c_uint,
    pub phys_out_grp_count: c_uint,
    pub phys_in_grps: *mut snd_efw_phys_grp,
    pub phys_out_grps: *mut snd_efw_phys_grp,
    pub pull_ptr: *mut u8,
    pub push_ptr: *mut u8,
}

#[repr(C)]
pub struct snd_efw_hwinfo {
    pub guid_hi: c_uint,
    pub guid_lo: c_uint,
    pub type_: c_uint,
    pub version: c_uint,
    pub vendor_name: *const c_char,
    pub model_name: *const c_char,
    pub dsp_version: c_uint,
    pub arm_version: c_uint,
    pub fpga_version: c_uint,
    pub flags: c_uint,
    pub max_sample_rate: c_uint,
    pub min_sample_rate: c_uint,
    pub supported_clocks: c_uint,
    pub phys_out: c_uint,
    pub phys_in: c_uint,
    pub phys_in_grp_count: c_uint,
    pub phys_out_grp_count: c_uint,
    pub phys_in_grps: *mut snd_efw_phys_grp,
    pub phys_out_grps: *mut snd_efw_phys_grp,
    pub amdtp_rx_pcm_channels: c_uint,
    pub amdtp_tx_pcm_channels: c_uint,
    pub amdtp_rx_pcm_channels_2x: c_uint,
    pub amdtp_tx_pcm_channels_2x: c_uint,
    pub amdtp_rx_pcm_channels_4x: c_uint,
    pub amdtp_tx_pcm_channels_4x: c_uint,
    pub midi_out_ports: c_uint,
    pub midi_in_ports: c_uint,
    pub mixer_playback_channels: c_uint,
    pub mixer_capture_channels: c_uint,
}

#[repr(C)]
pub struct snd_efw_phys_meters {
    pub out_meters: c_uint,
    pub in_meters: c_uint,
    pub values: [u32; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum snd_efw_clock_source {
    _Unknown = 0,
}

unsafe extern "C" {
    static snd_efw_resp_buf_size: c_uint;

    fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);

    fn snd_efw_command_get_hwinfo(efw: *mut snd_efw, hwinfo: *mut snd_efw_hwinfo) -> c_int;
    fn snd_efw_command_get_clock_source(
        efw: *mut snd_efw,
        clock_source: *mut snd_efw_clock_source,
    ) -> c_int;
    fn snd_efw_command_get_sampling_rate(efw: *mut snd_efw, sampling_rate: *mut c_uint)
        -> c_int;
    fn snd_efw_command_get_phys_meters(
        efw: *mut snd_efw,
        meters: *mut snd_efw_phys_meters,
        size: c_uint,
    ) -> c_int;

    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...);
    fn snd_info_create_card_entry(
        card: *mut snd_card,
        name: *const c_char,
        root: *mut snd_info_entry,
    ) -> *mut snd_info_entry;
    fn snd_info_set_text_ops(
        entry: *mut snd_info_entry,
        private_data: *mut c_void,
        op: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
    );
}

#[inline]
unsafe fn kmalloc_obj<T>() -> *mut T {
    unsafe { kmalloc(size_of::<T>(), GFP_KERNEL) as *mut T }
}

#[inline]
unsafe fn get_phys_name(grp: *mut snd_efw_phys_grp, input: bool_) -> *const c_char {
    static CH_TYPE: [*const c_char; 10] = [
        b"Analog\0".as_ptr() as *const c_char,
        b"S/PDIF\0".as_ptr() as *const c_char,
        b"ADAT\0".as_ptr() as *const c_char,
        b"S/PDIF or ADAT\0".as_ptr() as *const c_char,
        b"Mirroring\0".as_ptr() as *const c_char,
        b"Headphones\0".as_ptr() as *const c_char,
        b"I2S\0".as_ptr() as *const c_char,
        b"Guitar\0".as_ptr() as *const c_char,
        b"Pirzo Guitar\0".as_ptr() as *const c_char,
        b"Guitar String\0".as_ptr() as *const c_char,
    ];

    if unsafe { (*grp).type_ } < CH_TYPE.len() as c_uint {
        CH_TYPE[unsafe { (*grp).type_ } as usize]
    } else if input {
        b"Input\0".as_ptr() as *const c_char
    } else {
        b"Output\0".as_ptr() as *const c_char
    }
}

unsafe extern "C" fn proc_read_hwinfo(
    entry: *mut snd_info_entry,
    buffer: *mut snd_info_buffer,
) {
    let efw = unsafe { (*entry).private_data as *mut snd_efw };
    let mut i: c_uint;
    let hwinfo: *mut snd_efw_hwinfo;

    hwinfo = unsafe { kmalloc_obj::<snd_efw_hwinfo>() };
    if hwinfo == ptr::null_mut() {
        return;
    }

    if unsafe { snd_efw_command_get_hwinfo(efw, hwinfo) } < 0 {
        unsafe { kfree(hwinfo as *mut c_void) };
        return;
    }

    unsafe { snd_iprintf(buffer, b"guid_hi: 0x%X\n\0".as_ptr() as *const c_char, (*hwinfo).guid_hi) };
    unsafe { snd_iprintf(buffer, b"guid_lo: 0x%X\n\0".as_ptr() as *const c_char, (*hwinfo).guid_lo) };
    unsafe { snd_iprintf(buffer, b"type: 0x%X\n\0".as_ptr() as *const c_char, (*hwinfo).type_) };
    unsafe { snd_iprintf(buffer, b"version: 0x%X\n\0".as_ptr() as *const c_char, (*hwinfo).version) };
    unsafe { snd_iprintf(buffer, b"vendor_name: %s\n\0".as_ptr() as *const c_char, (*hwinfo).vendor_name) };
    unsafe { snd_iprintf(buffer, b"model_name: %s\n\0".as_ptr() as *const c_char, (*hwinfo).model_name) };

    unsafe { snd_iprintf(buffer, b"dsp_version: 0x%X\n\0".as_ptr() as *const c_char, (*hwinfo).dsp_version) };
    unsafe { snd_iprintf(buffer, b"arm_version: 0x%X\n\0".as_ptr() as *const c_char, (*hwinfo).arm_version) };
    unsafe { snd_iprintf(buffer, b"fpga_version: 0x%X\n\0".as_ptr() as *const c_char, (*hwinfo).fpga_version) };

    unsafe { snd_iprintf(buffer, b"flags: 0x%X\n\0".as_ptr() as *const c_char, (*hwinfo).flags) };

    unsafe { snd_iprintf(buffer, b"max_sample_rate: 0x%X\n\0".as_ptr() as *const c_char, (*hwinfo).max_sample_rate) };
    unsafe { snd_iprintf(buffer, b"min_sample_rate: 0x%X\n\0".as_ptr() as *const c_char, (*hwinfo).min_sample_rate) };
    unsafe { snd_iprintf(buffer, b"supported_clock: 0x%X\n\0".as_ptr() as *const c_char, (*hwinfo).supported_clocks) };

    unsafe { snd_iprintf(buffer, b"phys out: 0x%X\n\0".as_ptr() as *const c_char, (*hwinfo).phys_out) };
    unsafe { snd_iprintf(buffer, b"phys in: 0x%X\n\0".as_ptr() as *const c_char, (*hwinfo).phys_in) };

    unsafe { snd_iprintf(buffer, b"phys in grps: 0x%X\n\0".as_ptr() as *const c_char, (*hwinfo).phys_in_grp_count) };
    i = 0;
    while i < unsafe { (*hwinfo).phys_in_grp_count } {
        unsafe {
            snd_iprintf(
                buffer,
                b"phys in grp[%d]: type 0x%X, count 0x%X\n\0".as_ptr() as *const c_char,
                i,
                (*(*hwinfo).phys_out_grps.add(i as usize)).type_,
                (*(*hwinfo).phys_out_grps.add(i as usize)).count,
            )
        };
        i += 1;
    }

    unsafe { snd_iprintf(buffer, b"phys out grps: 0x%X\n\0".as_ptr() as *const c_char, (*hwinfo).phys_out_grp_count) };
    i = 0;
    while i < unsafe { (*hwinfo).phys_out_grp_count } {
        unsafe {
            snd_iprintf(
                buffer,
                b"phys out grps[%d]: type 0x%X, count 0x%X\n\0".as_ptr() as *const c_char,
                i,
                (*(*hwinfo).phys_out_grps.add(i as usize)).type_,
                (*(*hwinfo).phys_out_grps.add(i as usize)).count,
            )
        };
        i += 1;
    }

    unsafe { snd_iprintf(buffer, b"amdtp rx pcm channels 1x: 0x%X\n\0".as_ptr() as *const c_char, (*hwinfo).amdtp_rx_pcm_channels) };
    unsafe { snd_iprintf(buffer, b"amdtp tx pcm channels 1x: 0x%X\n\0".as_ptr() as *const c_char, (*hwinfo).amdtp_tx_pcm_channels) };
    unsafe { snd_iprintf(buffer, b"amdtp rx pcm channels 2x: 0x%X\n\0".as_ptr() as *const c_char, (*hwinfo).amdtp_rx_pcm_channels_2x) };
    unsafe { snd_iprintf(buffer, b"amdtp tx pcm channels 2x: 0x%X\n\0".as_ptr() as *const c_char, (*hwinfo).amdtp_tx_pcm_channels_2x) };
    unsafe { snd_iprintf(buffer, b"amdtp rx pcm channels 4x: 0x%X\n\0".as_ptr() as *const c_char, (*hwinfo).amdtp_rx_pcm_channels_4x) };
    unsafe { snd_iprintf(buffer, b"amdtp tx pcm channels 4x: 0x%X\n\0".as_ptr() as *const c_char, (*hwinfo).amdtp_tx_pcm_channels_4x) };

    unsafe { snd_iprintf(buffer, b"midi out ports: 0x%X\n\0".as_ptr() as *const c_char, (*hwinfo).midi_out_ports) };
    unsafe { snd_iprintf(buffer, b"midi in ports: 0x%X\n\0".as_ptr() as *const c_char, (*hwinfo).midi_in_ports) };

    unsafe { snd_iprintf(buffer, b"mixer playback channels: 0x%X\n\0".as_ptr() as *const c_char, (*hwinfo).mixer_playback_channels) };
    unsafe { snd_iprintf(buffer, b"mixer capture channels: 0x%X\n\0".as_ptr() as *const c_char, (*hwinfo).mixer_capture_channels) };

    unsafe { kfree(hwinfo as *mut c_void) };
}

unsafe extern "C" fn proc_read_clock(
    entry: *mut snd_info_entry,
    buffer: *mut snd_info_buffer,
) {
    let efw = unsafe { (*entry).private_data as *mut snd_efw };
    let mut clock_source: snd_efw_clock_source = snd_efw_clock_source::_Unknown;
    let mut sampling_rate: c_uint = 0;

    if unsafe { snd_efw_command_get_clock_source(efw, &mut clock_source) } < 0 {
        return;
    }

    if unsafe { snd_efw_command_get_sampling_rate(efw, &mut sampling_rate) } < 0 {
        return;
    }

    unsafe { snd_iprintf(buffer, b"Clock Source: %d\n\0".as_ptr() as *const c_char, clock_source as c_int) };
    unsafe { snd_iprintf(buffer, b"Sampling Rate: %d\n\0".as_ptr() as *const c_char, sampling_rate) };
}

/*
 * NOTE:
 *  dB = 20 * log10(linear / 0x01000000)
 *  -144.0 dB when linear is 0
 */
unsafe extern "C" fn proc_read_phys_meters(
    entry: *mut snd_info_entry,
    buffer: *mut snd_info_buffer,
) {
    let efw = unsafe { (*entry).private_data as *mut snd_efw };
    let meters: *mut snd_efw_phys_meters;
    let mut g: c_uint;
    let mut c: c_uint;
    let mut m: c_uint;
    let mut max: c_uint;
    let size: c_uint;
    let mut name: *const c_char;
    let mut linear: *mut u32;
    let err: c_int;

    size = (size_of::<snd_efw_phys_meters>()
        + unsafe { ((*efw).phys_in + (*efw).phys_out) as usize } * size_of::<u32>())
        as c_uint;
    meters = unsafe { kzalloc(size as usize, GFP_KERNEL) as *mut snd_efw_phys_meters };
    if meters == ptr::null_mut() {
        return;
    }

    err = unsafe { snd_efw_command_get_phys_meters(efw, meters, size) };
    if err < 0 {
        unsafe { kfree(meters as *mut c_void) };
        return;
    }

    unsafe { snd_iprintf(buffer, b"Physical Meters:\n\0".as_ptr() as *const c_char) };

    m = 0;
    max = unsafe {
        if (*efw).phys_out < (*meters).out_meters {
            (*efw).phys_out
        } else {
            (*meters).out_meters
        }
    };
    linear = unsafe { (*meters).values.as_ptr() as *mut u32 };
    unsafe { snd_iprintf(buffer, b" %d Outputs:\n\0".as_ptr() as *const c_char, max) };
    g = 0;
    while g < unsafe { (*efw).phys_out_grp_count } {
        name = unsafe { get_phys_name((*efw).phys_out_grps.add(g as usize), false) };
        c = 0;
        while c < unsafe { (*(*efw).phys_out_grps.add(g as usize)).count } {
            if m < max {
                unsafe {
                    snd_iprintf(
                        buffer,
                        b"\t%s [%d]: %d\n\0".as_ptr() as *const c_char,
                        name,
                        c,
                        *linear.add(m as usize),
                    )
                };
                m += 1;
            }
            c += 1;
        }
        g += 1;
    }

    m = 0;
    max = unsafe {
        if (*efw).phys_in < (*meters).in_meters {
            (*efw).phys_in
        } else {
            (*meters).in_meters
        }
    };
    linear = unsafe { ((*meters).values.as_ptr() as *mut u32).add((*meters).out_meters as usize) };
    unsafe { snd_iprintf(buffer, b" %d Inputs:\n\0".as_ptr() as *const c_char, max) };
    g = 0;
    while g < unsafe { (*efw).phys_in_grp_count } {
        name = unsafe { get_phys_name((*efw).phys_in_grps.add(g as usize), true) };
        c = 0;
        while c < unsafe { (*(*efw).phys_in_grps.add(g as usize)).count } {
            if m < max {
                unsafe {
                    snd_iprintf(
                        buffer,
                        b"\t%s [%d]: %d\n\0".as_ptr() as *const c_char,
                        name,
                        c,
                        *linear.add(m as usize),
                    )
                };
                m += 1;
            }
            c += 1;
        }
        g += 1;
    }

    unsafe { kfree(meters as *mut c_void) };
}

unsafe extern "C" fn proc_read_queues_state(
    entry: *mut snd_info_entry,
    buffer: *mut snd_info_buffer,
) {
    let efw = unsafe { (*entry).private_data as *mut snd_efw };
    let consumed: c_uint;

    if unsafe { (*efw).pull_ptr > (*efw).push_ptr } {
        consumed = unsafe {
            snd_efw_resp_buf_size
                - ((*efw).pull_ptr.offset_from((*efw).push_ptr) as c_uint)
        };
    } else {
        consumed = unsafe { (*efw).push_ptr.offset_from((*efw).pull_ptr) as c_uint };
    }

    unsafe {
        snd_iprintf(
            buffer,
            b"%d/%d\n\0".as_ptr() as *const c_char,
            consumed,
            snd_efw_resp_buf_size,
        )
    };
}

unsafe fn add_node(
    efw: *mut snd_efw,
    root: *mut snd_info_entry,
    name: *const c_char,
    op: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
) {
    let entry: *mut snd_info_entry;

    entry = unsafe { snd_info_create_card_entry((*efw).card, name, root) };
    if !entry.is_null() {
        unsafe { snd_info_set_text_ops(entry, efw as *mut c_void, op) };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_efw_proc_init(efw: *mut snd_efw) {
    let root: *mut snd_info_entry;

    /*
     * All nodes are automatically removed at snd_card_disconnect(),
     * by following to link list.
     */
    root = unsafe {
        snd_info_create_card_entry(
            (*efw).card,
            b"firewire\0".as_ptr() as *const c_char,
            (*(*efw).card).proc_root,
        )
    };
    if root == ptr::null_mut() {
        return;
    }
    unsafe {
        (*root).mode = S_IFDIR | 0o555;
    }

    unsafe { add_node(efw, root, b"clock\0".as_ptr() as *const c_char, Some(proc_read_clock)) };
    unsafe { add_node(efw, root, b"firmware\0".as_ptr() as *const c_char, Some(proc_read_hwinfo)) };
    unsafe { add_node(efw, root, b"meters\0".as_ptr() as *const c_char, Some(proc_read_phys_meters)) };
    unsafe { add_node(efw, root, b"queues\0".as_ptr() as *const c_char, Some(proc_read_queues_state)) };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
