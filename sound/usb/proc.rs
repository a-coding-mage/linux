// SPDX-License-Identifier: GPL-2.0-or-later
/*
 */

// External declarations for Linux kernel audio subsystem types and functions.
// These are provided by other translation units and the Linux kernel.

use core::ffi::c_char;

// Convert our full speed USB rate into sampling rate in Hz
#[inline]
fn get_full_speed_hz(usb_rate: u32) -> u32 {
    (usb_rate.wrapping_mul(125).wrapping_add(1 << 12)) >> 13
}

// Convert our high speed USB rate into sampling rate in Hz
#[inline]
fn get_high_speed_hz(usb_rate: u32) -> u32 {
    (usb_rate.wrapping_mul(125).wrapping_add(1 << 9)) >> 10
}

// Types from external headers - these are from the kernel audio subsystem
#[repr(C)]
pub struct snd_info_entry {
    pub private_data: *mut core::ffi::c_void,
    // ... other fields
}

#[repr(C)]
pub struct snd_info_buffer {
    // ... fields
}

#[repr(C)]
pub struct snd_usb_audio {
    pub shutdown: core::sync::atomic::AtomicU32,
    pub dev: *mut usb_device,
    pub usb_id: u32,
    pub card: *mut snd_card,
    pub mutex: kernel::sync::Mutex<()>,
    // ... other fields
}

#[repr(C)]
pub struct usb_device {
    pub bus: *mut usb_bus,
    pub devnum: u32,
    // ... other fields
}

#[repr(C)]
pub struct usb_bus {
    pub busnum: u32,
    // ... other fields
}

#[repr(C)]
pub struct snd_card {
    pub longname: [c_char; 80],
    // ... other fields
}

#[repr(C)]
pub struct snd_usb_stream {
    pub chip: *mut snd_usb_audio,
    pub pcm: *mut snd_pcm,
    pub pcm_index: u32,
    pub substream: [snd_usb_substream; 2],
    // ... other fields
}

#[repr(C)]
pub struct snd_pcm {
    pub name: [c_char; 80],
    // ... other fields
}

#[repr(C)]
pub struct snd_usb_substream {
    pub fmt_list: kernel::collections::List<audioformat>,
    pub running: u32,
    pub cur_audiofmt: *mut audioformat,
    pub data_endpoint: *mut snd_usb_endpoint,
    pub sync_endpoint: *mut snd_usb_endpoint,
    pub speed: u32,
    pub num_formats: u32,
    // ... other fields
}

#[repr(C)]
pub struct audioformat {
    pub list: kernel::collections::ListNode<audioformat>,
    pub iface: u32,
    pub altsetting: u32,
    pub formats: u64,
    pub channels: u32,
    pub endpoint: u8,
    pub ep_attr: u8,
    pub rates: u32,
    pub rate_min: u32,
    pub rate_max: u32,
    pub nr_rates: u32,
    pub rate_table: *mut u32,
    pub fmt_bits: u32,
    pub dsd_raw: u32,
    pub dsd_dop: u32,
    pub dsd_bitrev: u32,
    pub chmap: *mut snd_pcm_chmap_elem,
    pub sync_ep: u8,
    pub sync_iface: u32,
    pub sync_altsetting: u32,
    pub implicit_fb: u32,
    pub datainterval: u32,
    // ... other fields
}

#[repr(C)]
pub struct snd_usb_endpoint {
    pub curpacksize: u32,
    pub freqm: u32,
    pub freqshift: i32,
    pub syncmaxsize: u32,
    // ... other fields
}

#[repr(C)]
pub struct snd_pcm_chmap_elem {
    pub channels: u32,
    pub map: [u32; 0],
}

// Constants
const SNDRV_CHMAP_NA: usize = 0;
const SNDRV_CHMAP_MONO: usize = 1;
const SNDRV_CHMAP_FL: usize = 2;
const SNDRV_CHMAP_FR: usize = 3;
const SNDRV_CHMAP_FC: usize = 4;
const SNDRV_CHMAP_LFE: usize = 5;
const SNDRV_CHMAP_RL: usize = 6;
const SNDRV_CHMAP_RR: usize = 7;
const SNDRV_CHMAP_FLC: usize = 8;
const SNDRV_CHMAP_FRC: usize = 9;
const SNDRV_CHMAP_RC: usize = 10;
const SNDRV_CHMAP_SL: usize = 11;
const SNDRV_CHMAP_SR: usize = 12;
const SNDRV_CHMAP_TC: usize = 13;
const SNDRV_CHMAP_TFL: usize = 14;
const SNDRV_CHMAP_TFC: usize = 15;
const SNDRV_CHMAP_TFR: usize = 16;
const SNDRV_CHMAP_TRL: usize = 17;
const SNDRV_CHMAP_TRC: usize = 18;
const SNDRV_CHMAP_TRR: usize = 19;
const SNDRV_CHMAP_TFLC: usize = 20;
const SNDRV_CHMAP_TFRC: usize = 21;
const SNDRV_CHMAP_LLFE: usize = 22;
const SNDRV_CHMAP_RLFE: usize = 23;
const SNDRV_CHMAP_TSL: usize = 24;
const SNDRV_CHMAP_TSR: usize = 25;
const SNDRV_CHMAP_BC: usize = 26;
const SNDRV_CHMAP_RLC: usize = 27;
const SNDRV_CHMAP_RRC: usize = 28;

const USB_ENDPOINT_NUMBER_MASK: u8 = 0x0F;
const USB_DIR_IN: u8 = 0x80;
const USB_ENDPOINT_SYNCTYPE: u8 = 0x0C;
const USB_SPEED_FULL: u32 = 0;
const SNDRV_PCM_STREAM_PLAYBACK: usize = 0;
const SNDRV_PCM_STREAM_CAPTURE: usize = 1;
const SNDRV_PCM_RATE_CONTINUOUS: u32 = 0x80000000;
const INT_MIN: i32 = -2147483648;
const ARRAY_SIZE_CHANNEL_LABELS: usize = 29;

static CHANNEL_LABELS: [Option<&str>; ARRAY_SIZE_CHANNEL_LABELS] = [
    Some("N/A"),       // SNDRV_CHMAP_NA
    Some("MONO"),      // SNDRV_CHMAP_MONO
    Some("FL"),        // SNDRV_CHMAP_FL
    Some("FR"),        // SNDRV_CHMAP_FR
    Some("FC"),        // SNDRV_CHMAP_FC
    Some("LFE"),       // SNDRV_CHMAP_LFE
    Some("RL"),        // SNDRV_CHMAP_RL
    Some("RR"),        // SNDRV_CHMAP_RR
    Some("FLC"),       // SNDRV_CHMAP_FLC
    Some("FRC"),       // SNDRV_CHMAP_FRC
    Some("RC"),        // SNDRV_CHMAP_RC
    Some("SL"),        // SNDRV_CHMAP_SL
    Some("SR"),        // SNDRV_CHMAP_SR
    Some("TC"),        // SNDRV_CHMAP_TC
    Some("TFL"),       // SNDRV_CHMAP_TFL
    Some("TFC"),       // SNDRV_CHMAP_TFC
    Some("TFR"),       // SNDRV_CHMAP_TFR
    Some("TRL"),       // SNDRV_CHMAP_TRL
    Some("TRC"),       // SNDRV_CHMAP_TRC
    Some("TRR"),       // SNDRV_CHMAP_TRR
    Some("TFLC"),      // SNDRV_CHMAP_TFLC
    Some("TFRC"),      // SNDRV_CHMAP_TFRC
    Some("LLFE"),      // SNDRV_CHMAP_LLFE
    Some("RLFE"),      // SNDRV_CHMAP_RLFE
    Some("TSL"),       // SNDRV_CHMAP_TSL
    Some("TSR"),       // SNDRV_CHMAP_TSR
    Some("BC"),        // SNDRV_CHMAP_BC
    Some("RLC"),       // SNDRV_CHMAP_RLC
    Some("RRC"),       // SNDRV_CHMAP_RRC
];

// External function declarations from the Linux kernel audio subsystem
extern "C" {
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...);
    fn snd_card_ro_proc_new(
        card: *mut snd_card,
        name: *const c_char,
        private_data: *mut core::ffi::c_void,
        read_proc: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
    ) -> i32;
    fn pcm_for_each_format(fmt: *mut snd_pcm_format_t) -> bool;
    fn snd_pcm_format_to_bits(fmt: snd_pcm_format_t) -> u64;
    fn snd_pcm_format_name(fmt: snd_pcm_format_t) -> *const c_char;
    fn atomic_read(v: *const core::sync::atomic::AtomicU32) -> u32;
    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> usize;
    fn USB_ID_VENDOR(id: u32) -> u32;
    fn USB_ID_PRODUCT(id: u32) -> u32;
}

type snd_pcm_format_t = u32;

// proc interface for USB bus and ID info
unsafe extern "C" fn proc_audio_usbbus_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let chip = (*entry).private_data as *mut snd_usb_audio;
    if atomic_read(&(*chip).shutdown) == 0 {
        snd_iprintf(
            buffer,
            b"%03d/%03d\n\0".as_ptr() as *const c_char,
            (*(*(*chip).dev).bus).busnum,
            (*(*chip).dev).devnum,
        );
    }
}

unsafe extern "C" fn proc_audio_usbid_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let chip = (*entry).private_data as *mut snd_usb_audio;
    if atomic_read(&(*chip).shutdown) == 0 {
        snd_iprintf(
            buffer,
            b"%04x:%04x\n\0".as_ptr() as *const c_char,
            USB_ID_VENDOR((*chip).usb_id),
            USB_ID_PRODUCT((*chip).usb_id),
        );
    }
}

pub unsafe extern "C" fn snd_usb_audio_create_proc(chip: *mut snd_usb_audio) {
    snd_card_ro_proc_new(
        (*chip).card,
        b"usbbus\0".as_ptr() as *const c_char,
        chip as *mut core::ffi::c_void,
        Some(proc_audio_usbbus_read),
    );
    snd_card_ro_proc_new(
        (*chip).card,
        b"usbid\0".as_ptr() as *const c_char,
        chip as *mut core::ffi::c_void,
        Some(proc_audio_usbid_read),
    );
}

// proc interface to list the supported pcm formats
unsafe fn proc_dump_substream_formats(subs: *mut snd_usb_substream, buffer: *mut snd_info_buffer) {
    let sync_types = [
        "NONE\0",
        "ASYNC\0",
        "ADAPTIVE\0",
        "SYNC\0",
    ];

    // list_for_each_entry equivalent: iterate through fmt_list
    let mut fp = (*subs).fmt_list.next as *mut audioformat;
    while !fp.is_null() && fp != &mut (*subs).fmt_list as *mut _ as *mut audioformat {
        snd_iprintf(
            buffer,
            b"  Interface %d\n\0".as_ptr() as *const c_char,
            (*fp).iface,
        );
        snd_iprintf(
            buffer,
            b"    Altset %d\n\0".as_ptr() as *const c_char,
            (*fp).altsetting,
        );
        snd_iprintf(buffer, b"    Format:\0".as_ptr() as *const c_char);

        // pcm_for_each_format loop - simplified since we don't have the actual macro
        let mut fmt: snd_pcm_format_t = 0;
        while pcm_for_each_format(&mut fmt) {
            if ((*fp).formats & snd_pcm_format_to_bits(fmt)) != 0 {
                snd_iprintf(
                    buffer,
                    b" %s\0".as_ptr() as *const c_char,
                    snd_pcm_format_name(fmt),
                );
            }
        }
        snd_iprintf(buffer, b"\n\0".as_ptr() as *const c_char);

        snd_iprintf(
            buffer,
            b"    Channels: %d\n\0".as_ptr() as *const c_char,
            (*fp).channels,
        );
        snd_iprintf(
            buffer,
            b"    Endpoint: 0x%02x (%d %s) (%s)\n\0".as_ptr() as *const c_char,
            (*fp).endpoint as u32,
            ((*fp).endpoint & USB_ENDPOINT_NUMBER_MASK) as u32,
            if ((*fp).endpoint & USB_DIR_IN) != 0 { b"IN\0".as_ptr() as *const c_char } else { b"OUT\0".as_ptr() as *const c_char },
            sync_types[(((*fp).ep_attr & USB_ENDPOINT_SYNCTYPE) >> 2) as usize].as_ptr() as *const c_char,
        );

        if ((*fp).rates & SNDRV_PCM_RATE_CONTINUOUS) != 0 {
            snd_iprintf(
                buffer,
                b"    Rates: %d - %d (continuous)\n\0".as_ptr() as *const c_char,
                (*fp).rate_min,
                (*fp).rate_max,
            );
        } else {
            snd_iprintf(buffer, b"    Rates: \0".as_ptr() as *const c_char);
            for i in 0..(*fp).nr_rates {
                if i > 0 {
                    snd_iprintf(buffer, b", \0".as_ptr() as *const c_char);
                }
                snd_iprintf(
                    buffer,
                    b"%d\0".as_ptr() as *const c_char,
                    *(*fp).rate_table.add(i as usize),
                );
            }
            snd_iprintf(buffer, b"\n\0".as_ptr() as *const c_char);
        }

        if (*subs).speed != USB_SPEED_FULL {
            snd_iprintf(
                buffer,
                b"    Data packet interval: %d us\n\0".as_ptr() as *const c_char,
                125 * (1 << (*fp).datainterval),
            );
        }

        snd_iprintf(
            buffer,
            b"    Bits: %d\n\0".as_ptr() as *const c_char,
            (*fp).fmt_bits,
        );

        if (*fp).dsd_raw != 0 {
            snd_iprintf(
                buffer,
                b"    DSD raw: DOP=%d, bitrev=%d\n\0".as_ptr() as *const c_char,
                (*fp).dsd_dop,
                (*fp).dsd_bitrev,
            );
        }

        if !(*fp).chmap.is_null() {
            let map = (*fp).chmap;
            snd_iprintf(buffer, b"    Channel map:\0".as_ptr() as *const c_char);
            for c in 0..(*map).channels {
                let c_idx = c as usize;
                let map_val = *(*map).map.as_ptr().add(c_idx);
                if map_val >= ARRAY_SIZE_CHANNEL_LABELS as u32 || CHANNEL_LABELS[map_val as usize].is_none() {
                    snd_iprintf(buffer, b" --\0".as_ptr() as *const c_char);
                } else {
                    snd_iprintf(
                        buffer,
                        b" %s\0".as_ptr() as *const c_char,
                        CHANNEL_LABELS[map_val as usize].unwrap().as_ptr() as *const c_char,
                    );
                }
            }
            snd_iprintf(buffer, b"\n\0".as_ptr() as *const c_char);
        }

        if (*fp).sync_ep != 0 {
            snd_iprintf(
                buffer,
                b"    Sync Endpoint: 0x%02x (%d %s)\n\0".as_ptr() as *const c_char,
                (*fp).sync_ep as u32,
                ((*fp).sync_ep & USB_ENDPOINT_NUMBER_MASK) as u32,
                if ((*fp).sync_ep & USB_DIR_IN) != 0 { b"IN\0".as_ptr() as *const c_char } else { b"OUT\0".as_ptr() as *const c_char },
            );
            snd_iprintf(
                buffer,
                b"    Sync EP Interface: %d\n\0".as_ptr() as *const c_char,
                (*fp).sync_iface,
            );
            snd_iprintf(
                buffer,
                b"    Sync EP Altset: %d\n\0".as_ptr() as *const c_char,
                (*fp).sync_altsetting,
            );
            snd_iprintf(
                buffer,
                b"    Implicit Feedback Mode: %s\n\0".as_ptr() as *const c_char,
                if (*fp).implicit_fb != 0 { b"Yes\0".as_ptr() as *const c_char } else { b"No\0".as_ptr() as *const c_char },
            );
        }

        fp = (*fp).list.next as *mut audioformat;
    }
}

unsafe fn proc_dump_ep_status(
    subs: *mut snd_usb_substream,
    data_ep: *mut snd_usb_endpoint,
    sync_ep: *mut snd_usb_endpoint,
    buffer: *mut snd_info_buffer,
) {
    if data_ep.is_null() {
        return;
    }

    snd_iprintf(
        buffer,
        b"    Packet Size = %d\n\0".as_ptr() as *const c_char,
        (*data_ep).curpacksize,
    );

    let freq_hz = if (*subs).speed == USB_SPEED_FULL {
        get_full_speed_hz((*data_ep).freqm)
    } else {
        get_high_speed_hz((*data_ep).freqm)
    };

    snd_iprintf(
        buffer,
        b"    Momentary freq = %u Hz (%#x.%04x)\n\0".as_ptr() as *const c_char,
        freq_hz,
        (*data_ep).freqm >> 16,
        (*data_ep).freqm & 0xffff,
    );

    if !sync_ep.is_null() && (*data_ep).freqshift != INT_MIN {
        let res = 16 - (*data_ep).freqshift;
        snd_iprintf(
            buffer,
            b"    Feedback Format = %d.%d\n\0".as_ptr() as *const c_char,
            (if (*sync_ep).syncmaxsize > 3 { 32 } else { 24 }) - res,
            res,
        );
    }
}

unsafe fn proc_dump_substream_status(
    chip: *mut snd_usb_audio,
    subs: *mut snd_usb_substream,
    buffer: *mut snd_info_buffer,
) {
    // guard(mutex) - acquire mutex guard
    let _guard = &(*chip).mutex;

    if (*subs).running != 0 {
        snd_iprintf(buffer, b"  Status: Running\n\0".as_ptr() as *const c_char);
        if !(*subs).cur_audiofmt.is_null() {
            snd_iprintf(
                buffer,
                b"    Interface = %d\n\0".as_ptr() as *const c_char,
                (*(*subs).cur_audiofmt).iface,
            );
            snd_iprintf(
                buffer,
                b"    Altset = %d\n\0".as_ptr() as *const c_char,
                (*(*subs).cur_audiofmt).altsetting,
            );
        }
        proc_dump_ep_status(subs, (*subs).data_endpoint, (*subs).sync_endpoint, buffer);
    } else {
        snd_iprintf(buffer, b"  Status: Stop\n\0".as_ptr() as *const c_char);
    }
}

unsafe extern "C" fn proc_pcm_format_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let stream = (*entry).private_data as *mut snd_usb_stream;
    let chip = (*stream).chip;

    snd_iprintf(
        buffer,
        b"%s : %s\n\0".as_ptr() as *const c_char,
        (*(*chip).card).longname.as_ptr(),
        (*(*stream).pcm).name.as_ptr(),
    );

    if (*stream).substream[SNDRV_PCM_STREAM_PLAYBACK].num_formats != 0 {
        snd_iprintf(buffer, b"\nPlayback:\n\0".as_ptr() as *const c_char);
        proc_dump_substream_status(chip, &mut (*stream).substream[SNDRV_PCM_STREAM_PLAYBACK], buffer);
        proc_dump_substream_formats(&mut (*stream).substream[SNDRV_PCM_STREAM_PLAYBACK], buffer);
    }

    if (*stream).substream[SNDRV_PCM_STREAM_CAPTURE].num_formats != 0 {
        snd_iprintf(buffer, b"\nCapture:\n\0".as_ptr() as *const c_char);
        proc_dump_substream_status(chip, &mut (*stream).substream[SNDRV_PCM_STREAM_CAPTURE], buffer);
        proc_dump_substream_formats(&mut (*stream).substream[SNDRV_PCM_STREAM_CAPTURE], buffer);
    }
}

pub unsafe extern "C" fn snd_usb_proc_pcm_format_add(stream: *mut snd_usb_stream) {
    let mut name = [0 as c_char; 32];
    let card = (*(*stream).chip).card;

    scnprintf(
        name.as_mut_ptr(),
        32,
        b"stream%d\0".as_ptr() as *const c_char,
        (*stream).pcm_index,
    );

    snd_card_ro_proc_new(
        card,
        name.as_ptr(),
        stream as *mut core::ffi::c_void,
        Some(proc_pcm_format_read),
    );
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
