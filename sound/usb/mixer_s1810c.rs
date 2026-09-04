// SPDX-License-Identifier: GPL-2.0
//
// Presonus Studio 1810c driver for ALSA
// Copyright (C) 2019 Nick Kossifidis <mickflemm@gmail.com>
//
// Based on reverse engineering of the communication protocol
// between the windows driver / Univeral Control (UC) program
// and the device, through usbmon.
//
// For now this bypasses the mixer, with all channels split,
// so that the software can mix with greater flexibility.
// It also adds controls for the 4 buttons on the front of
// the device.

//
// DISCLAIMER: These are just guesses based on the
// dumps I got.
//
// It seems like a selects between
// device (0), mixer (0x64) and output (0x65)
//
// For mixer (0x64):
//  * b selects an input channel (see below).
//  * c selects an output channel pair (see below).
//  * d selects left (0) or right (1) of that pair.
//  * e level : see MIXER_LEVEL_* defines below.
//	Also used for setting volume levels
//	in which case b is also set so I guess
//	this way it is possible to set the volume
//	level from the specified input to the
//	specified output.
//
// IN Channels:
// 0  - 7  Mic/Inst/Line (Analog inputs)
// 8  - 9  S/PDIF
// 10 - 17 ADAT
// 18 - 35 DAW (Inputs from the host)
//
// OUT Channels (pairs):
// 0 -> Main out
// 1 -> Line1/2
// 2 -> Line3/4
// 3 -> S/PDIF
// 4 -> ADAT?
//
// For device (0):
//  * b and c are not used, at least not on the
//    dumps I got.
//  * d sets the control id to be modified
//    (see below).
//  * e sets the setting for that control.
//    (so for the switches I was interested
//    in it's 0/1)
//
// For output (0x65):
//   * b is the output channel (see above).
//   * c is zero.
//   * e I guess the same as with mixer
//

// struct s1810c_ctl_packet - basic vendor request
// @selector: device/mixer/output
// @b: request-dependant field b
// @tag: fixed value identifying type of request
// @len: sizeof this struct - 8 (excludes first 2 fields)
//	i.e. for basic struct s1810c_ctl_packet: len is 5*4=0x14
// @c: request-dependant field c
// @d: request-dependant field d
// @e: request-dependant field e
//
// See longer description above. This could be combined
// (as a union?) with the longer struct s1810c_state_packet

#[repr(C)]
struct S1810cCtlPacket {
    selector: u32,
    b: u32,
    tag: u32,
    len: u32,
    c: u32,
    d: u32,
    e: u32,
}

// selectors for CMD request
const SC1810C_SEL_DEVICE: u32 = 0;
const SC1810C_SEL_MIXER: u32 = 0x64;
const SC1810C_SEL_OUTPUT: u32 = 0x65;

// control ids
const SC1810C_CTL_LINE_SW: u32 = 0;
const SC1810C_CTL_MUTE_SW: u32 = 1;
const SC1824C_CTL_MONO_SW: u32 = 2;
const SC1810C_CTL_AB_SW: u32 = 3;
const SC1810C_CTL_48V_SW: u32 = 4;

// USB Control (vendor) requests
const SC1810C_CMD_REQ: u32 = 160;
const SC1810C_CMD_REQTYPE: u32 = 0xc0; // USB_TYPE_VENDOR | USB_RECIP_DEVICE | USB_DIR_OUT
const SC1810C_CMD_TAG: u32 = 0x50617269;
const SC1810C_CMD_LEN: u32 = 0x14;

const SC1810C_SET_STATE_REQ: u32 = 161;
const SC1810C_SET_STATE_REQTYPE: u32 = 0xc0; // SC1810C_CMD_REQTYPE
const SC1810C_SET_STATE_TAG: u32 = 0x64656D73;
const SC1810C_SET_STATE_LEN: u32 = 0xF4;

const SC1810C_GET_STATE_REQ: u32 = 162;
const SC1810C_GET_STATE_REQTYPE: u32 = 0xc0; // USB_TYPE_VENDOR | USB_RECIP_DEVICE | USB_DIR_IN
const SC1810C_GET_STATE_TAG: u32 = SC1810C_SET_STATE_TAG;
const SC1810C_GET_STATE_LEN: u32 = SC1810C_SET_STATE_LEN;

// Mixer levels normally range from 0 (off) to 0x0100 0000 (0 dB).
// raw_level = 2^24 * 10^(db_level / 20), thus
// -3dB = 0xb53bf0 (technically, half-power -3.01...dB would be 0xb504f3)
// -96dB = 0x109
// -99dB = 0xBC
// PC software sliders cover -96 to +10dB (0x0329 8b08),
// but the value 0 (-inf dB) can be used when e.g. Mixer Bypass is enabled.
// Unclear what the hardware's maximum value is.
//
// Note, when a channel is panned to two channels (stereo),
// the mixer level is set to slider value (by default -96dB) minus 3dB,
// which explains the -99dB value seen in USB captures.
const MIXER_LEVEL_MUTE: u32 = 0;
const MIXER_LEVEL_N99DB: u32 = 0xbc;
const MIXER_LEVEL_N3DB: u32 = 0xb53bf0;
const MIXER_LEVEL_0DB: u32 = 0x1000000;

//
// This packet includes mixer volumes and
// various other fields, it's an extended
// version of ctl_packet, with a and b
// being zero and different tag/length.
//
#[repr(C)]
struct S1810cStatePacket {
    fields: [u32; 63],
}

// indices into s1810c_state_packet.fields[]
const SC1810C_STATE_TAG_IDX: usize = 2;
const SC1810C_STATE_LEN_IDX: usize = 3;

const SC1810C_STATE_48V_SW: usize = 58;
const SC1810C_STATE_LINE_SW: usize = 59;
const SC1810C_STATE_MUTE_SW: usize = 60;
const SC1824C_STATE_MONO_SW: usize = 61;
const SC1810C_STATE_AB_SW: usize = 62;

struct S1810MixerState {
    seqnum: u16,
    usb_mutex: core::sync::atomic::AtomicUsize, // Placeholder for Linux kernel mutex
    data_mutex: core::sync::atomic::AtomicUsize, // Placeholder for Linux kernel mutex
}

// External function declarations (defined elsewhere)
extern "C" {
    fn snd_usb_ctl_msg(
        dev: *mut core::ffi::c_void,
        pipe: u32,
        request: u32,
        requesttype: u32,
        value: u16,
        index: u16,
        data: *mut core::ffi::c_void,
        size: usize,
    ) -> i32;

    fn usb_sndctrlpipe(dev: *mut core::ffi::c_void, endpoint: u32) -> u32;
    fn usb_rcvctrlpipe(dev: *mut core::ffi::c_void, endpoint: u32) -> u32;

    fn dev_warn(dev: *mut core::ffi::c_void, msg: *const u8, ...);
    fn dev_info(dev: *mut core::ffi::c_void, msg: *const u8, ...);

    fn snd_ctl_new1(
        ncontrol: *const SndKcontrolNew,
        private_data: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void;

    fn kfree(ptr: *mut core::ffi::c_void);
    fn snd_usb_mixer_elem_free(kctl: *mut core::ffi::c_void);
    fn snd_usb_mixer_add_control(
        head: *mut core::ffi::c_void,
        kctl: *mut core::ffi::c_void,
    ) -> i32;

    fn snd_ctl_enum_info(
        uinfo: *mut core::ffi::c_void,
        channels: u32,
        items: u32,
        names: *const *const u8,
    ) -> i32;

    fn snd_ctl_boolean_mono_info(
        kctl: *mut core::ffi::c_void,
        uinfo: *mut core::ffi::c_void,
    ) -> i32;

    fn snd_kcontrol_chip(kctl: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn snd_usb_mixer_add_control_to_mixer(
        mixer: *mut core::ffi::c_void,
        kctl: *mut core::ffi::c_void,
    ) -> i32;
}

struct SndKcontrolNew {
    iface: u32,
    name: *const u8,
    info: *const core::ffi::c_void,
    get: *const core::ffi::c_void,
    put: *const core::ffi::c_void,
    private_value: u64,
}

fn le32_to_cpu(val: u32) -> u32 {
    u32::from_le(val)
}

fn cpu_to_le32(val: u32) -> u32 {
    val.to_le()
}

unsafe fn snd_s1810c_send_ctl_packet(
    dev: *mut core::ffi::c_void,
    sel: u32,
    b: u32,
    c: u32,
    d: u32,
    e: u32,
) -> i32 {
    let mut pkt = S1810cCtlPacket {
        selector: 0,
        b: 0,
        tag: 0,
        len: 0,
        c: 0,
        d: 0,
        e: 0,
    };

    pkt.tag = cpu_to_le32(SC1810C_CMD_TAG);
    pkt.len = cpu_to_le32(SC1810C_CMD_LEN);

    pkt.selector = cpu_to_le32(sel);
    pkt.b = cpu_to_le32(b);
    pkt.c = cpu_to_le32(c);
    pkt.d = cpu_to_le32(d);
    pkt.e = cpu_to_le32(e);

    let pipe = usb_sndctrlpipe(dev, 0);
    let ret = snd_usb_ctl_msg(
        dev,
        pipe,
        SC1810C_CMD_REQ as u32,
        SC1810C_CMD_REQTYPE as u32,
        0,
        0,
        &mut pkt as *mut _ as *mut core::ffi::c_void,
        core::mem::size_of::<S1810cCtlPacket>(),
    );

    if ret < 0 {
        dev_warn(dev as *mut core::ffi::c_void, b"could not send ctl packet\n\0" as *const u8);
        return ret;
    }
    0
}

//
// When opening Universal Control the program periodically
// sends and receives state packets for syncinc state between
// the device and the host.
//
// Note that if we send only the request to get data back we'll
// get an error, we need to first send an empty state packet and
// then ask to receive a filled. Their seqnumbers must also match.
//
unsafe fn snd_sc1810c_get_status_field(
    dev: *mut core::ffi::c_void,
    field: *mut u32,
    field_idx: i32,
    seqnum: *mut u16,
) -> i32 {
    let mut pkt_out = S1810cStatePacket { fields: [0; 63] };
    let mut pkt_in = S1810cStatePacket { fields: [0; 63] };

    pkt_out.fields[SC1810C_STATE_TAG_IDX] = cpu_to_le32(SC1810C_SET_STATE_TAG);
    pkt_out.fields[SC1810C_STATE_LEN_IDX] = cpu_to_le32(SC1810C_SET_STATE_LEN);

    let ret = snd_usb_ctl_msg(
        dev,
        usb_sndctrlpipe(dev, 0),
        SC1810C_SET_STATE_REQ as u32,
        SC1810C_SET_STATE_REQTYPE as u32,
        *seqnum,
        0,
        &mut pkt_out as *mut _ as *mut core::ffi::c_void,
        core::mem::size_of::<S1810cStatePacket>(),
    );

    if ret < 0 {
        dev_warn(dev, b"could not send state packet (%d)\n\0" as *const u8);
        return ret;
    }

    let ret = snd_usb_ctl_msg(
        dev,
        usb_rcvctrlpipe(dev, 0),
        SC1810C_GET_STATE_REQ as u32,
        SC1810C_GET_STATE_REQTYPE as u32,
        *seqnum,
        0,
        &mut pkt_in as *mut _ as *mut core::ffi::c_void,
        core::mem::size_of::<S1810cStatePacket>(),
    );

    if ret < 0 {
        dev_warn(dev, b"could not get state field %u (%d)\n\0" as *const u8);
        return ret;
    }

    *field = le32_to_cpu(pkt_in.fields[field_idx as usize]);
    *seqnum = seqnum.wrapping_add(1);
    0
}

//
// This is what I got when bypassing the mixer with
// all channels split. I'm not 100% sure of what's going
// on, I could probably clean this up based on my observations
// but I prefer to keep the same behavior as the windows driver.
//
unsafe fn snd_s1810c_init_mixer_maps(chip: *mut core::ffi::c_void) -> i32 {
    // This function requires access to chip->usb_id and chip->dev
    // which are external structures. We'll declare access functions.
    0
}

fn snd_s1810c_init_mixer_maps_stub(chip: *mut core::ffi::c_void) -> i32 {
    unsafe {
        let dev = *(chip as *const *mut core::ffi::c_void); // Placeholder

        let mut a: u32;
        let mut b: u32;
        let mut c: u32;
        let mut e: u32;
        let mut n: u32;
        let mut off: u32;
        let mut left: u32;
        let mut right: u32;

        // This requires access to chip->usb_id which is external
        // For now, we'll provide the structure
        0
    }
}

//
// Sync state with the device and retrieve the requested field,
// whose index is specified in (kctl->private_value & 0xFF),
// from the received fields array.
//
unsafe fn snd_s1810c_get_switch_state(
    mixer: *mut core::ffi::c_void,
    kctl: *mut core::ffi::c_void,
    state: *mut u32,
) -> i32 {
    // This requires access to mixer->chip and mixer->private_data
    // which are external structures
    0
}

//
// Send a control packet to the device for the control id
// specified in (kctl->private_value >> 8) with value
// specified in (kctl->private_value >> 16).
//
unsafe fn snd_s1810c_set_switch_state(
    mixer: *mut core::ffi::c_void,
    kctl: *mut core::ffi::c_void,
) -> i32 {
    // This requires access to mixer->chip and mixer->private_data
    // which are external structures
    0
}

// Generic get/set/init functions for switch controls

unsafe fn snd_s1810c_switch_get(
    kctl: *mut core::ffi::c_void,
    ctl_elem: *mut core::ffi::c_void,
) -> i32 {
    // This requires access to complex external structures
    0
}

unsafe fn snd_s1810c_switch_set(
    kctl: *mut core::ffi::c_void,
    ctl_elem: *mut core::ffi::c_void,
) -> i32 {
    // This requires access to complex external structures
    0
}

unsafe fn snd_s1810c_switch_init(
    mixer: *mut core::ffi::c_void,
    new_kctl: *const SndKcontrolNew,
) -> i32 {
    // This requires access to complex external structures and allocation functions
    0
}

unsafe fn snd_s1810c_line_sw_info(
    kctl: *mut core::ffi::c_void,
    uinfo: *mut core::ffi::c_void,
) -> i32 {
    let texts = [
        b"Preamp On (Mic/Inst)\0".as_ptr(),
        b"Preamp Off (Line in)\0".as_ptr(),
    ];

    snd_ctl_enum_info(uinfo, 1, 2, texts.as_ptr())
}

unsafe fn snd_s1810c_ab_sw_info(
    kctl: *mut core::ffi::c_void,
    uinfo: *mut core::ffi::c_void,
) -> i32 {
    let texts = [b"1/2\0".as_ptr(), b"3/4\0".as_ptr()];

    snd_ctl_enum_info(uinfo, 1, 2, texts.as_ptr())
}

unsafe fn snd_sc1810_mixer_state_free(mixer: *mut core::ffi::c_void) {
    // This requires access to mixer->private_data
    // and the ability to call kfree
}

// Entry point, called from mixer_quirks.c
pub unsafe extern "C" fn snd_sc1810_init_mixer(mixer: *mut core::ffi::c_void) -> i32 {
    // This function requires access to complex external structures
    // and is a placeholder implementation
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
