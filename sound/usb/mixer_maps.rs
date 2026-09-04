// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Additional mixer mapping
 *
 *   Copyright (c) 2002 by Takashi Iwai <tiwai@suse.de>
 */

use std::ffi::c_char;
use std::ptr;

#[repr(C)]
pub struct usbmix_dB_map {
    pub min: i32,
    pub max: i32,
    pub min_mute: bool,
}

#[repr(C)]
pub struct usbmix_name_map {
    pub id: i32,
    pub name: *const c_char,
    pub control: i32,
    pub dB: *const usbmix_dB_map,
}

#[repr(C)]
pub struct usbmix_selector_map {
    pub id: i32,
    pub count: i32,
    pub names: *const *const c_char,
}

#[repr(C)]
pub struct usbmix_ctl_map {
    pub id: u32,
    pub map: *const usbmix_name_map,
    pub selector_map: *const usbmix_selector_map,
    pub connector_map: *const usbmix_connector_map,
}

// External struct - defined elsewhere
#[repr(C)]
pub struct usbmix_connector_map {
    pub placeholder: u32, // Placeholder to indicate external dependency
}

/*
 * USB control mappers for SB Extigy
 */

/*
 * Topology of SB Extigy (see on the wide screen :)

USB_IN[1] --->FU[2]------------------------------+->MU[16]-->PU[17]-+->FU[18]--+->EU[27]--+->EU[21]-->FU[22]--+->FU[23] > Dig_OUT[24]
                                                 ^                  |          |          |                   |
USB_IN[3] -+->SU[5]-->FU[6]--+->MU[14] ->PU[15]->+                  |          |          |                   +->FU[25] > Dig_OUT[26]
           ^                 ^                   |                  |          |          |
Dig_IN[4] -+                 |                   |                  |          |          +->FU[28]---------------------> Spk_OUT[19]
                             |                   |                  |          |
Lin-IN[7] -+-->FU[8]---------+                   |                  |          +----------------------------------------> Hph_OUT[20]
           |                                     |                  |
Mic-IN[9] --+->FU[10]----------------------------+                  |
           ||                                                       |
           ||  +----------------------------------------------------+
           VV  V
           ++--+->SU[11]-->FU[12] --------------------------------------------------------------------------------------> USB_OUT[13]
*/

pub static EXTIGY_MAP: &[usbmix_name_map] = &[
    // 1: IT pcm
    usbmix_name_map { id: 2, name: c"PCM Playback".as_ptr(), control: 0, dB: ptr::null() }, // FU
    // 3: IT pcm
    // 4: IT digital in
    usbmix_name_map { id: 5, name: ptr::null(), control: 0, dB: ptr::null() }, // DISABLED: this seems to be bogus on some firmware
    usbmix_name_map { id: 6, name: c"Digital In".as_ptr(), control: 0, dB: ptr::null() }, // FU
    // 7: IT line
    usbmix_name_map { id: 8, name: c"Line Playback".as_ptr(), control: 0, dB: ptr::null() }, // FU
    // 9: IT mic
    usbmix_name_map { id: 10, name: c"Mic Playback".as_ptr(), control: 0, dB: ptr::null() }, // FU
    usbmix_name_map { id: 11, name: c"Capture Source".as_ptr(), control: 0, dB: ptr::null() }, // SU
    usbmix_name_map { id: 12, name: c"Capture".as_ptr(), control: 0, dB: ptr::null() }, // FU
    // 13: OT pcm capture
    // 14: MU (w/o controls)
    // 15: PU (3D enh)
    // 16: MU (w/o controls)
    usbmix_name_map { id: 17, name: ptr::null(), control: 1, dB: ptr::null() }, // DISABLED: PU-switch (any effect?)
    usbmix_name_map { id: 17, name: c"Channel Routing".as_ptr(), control: 2, dB: ptr::null() }, // PU: mode select
    // Need external constant UAC_FU_BASS
    usbmix_name_map { id: 18, name: c"Tone Control - Bass".as_ptr(), control: 0, dB: ptr::null() }, // FU - using placeholder for UAC_FU_BASS
    // Need external constant UAC_FU_TREBLE
    usbmix_name_map { id: 18, name: c"Tone Control - Treble".as_ptr(), control: 0, dB: ptr::null() }, // FU - using placeholder for UAC_FU_TREBLE
    usbmix_name_map { id: 18, name: c"Master Playback".as_ptr(), control: 0, dB: ptr::null() }, // FU; others
    // 19: OT speaker
    // 20: OT headphone
    usbmix_name_map { id: 21, name: ptr::null(), control: 0, dB: ptr::null() }, // DISABLED: EU (for what?)
    usbmix_name_map { id: 22, name: c"Digital Out Playback".as_ptr(), control: 0, dB: ptr::null() }, // FU
    usbmix_name_map { id: 23, name: c"Digital Out1 Playback".as_ptr(), control: 0, dB: ptr::null() }, // FU  // FIXME: corresponds to 24
    // 24: OT digital out
    usbmix_name_map { id: 25, name: c"IEC958 Optical Playback".as_ptr(), control: 0, dB: ptr::null() }, // FU
    usbmix_name_map { id: 26, name: c"IEC958 Optical Playback".as_ptr(), control: 0, dB: ptr::null() }, // OT
    usbmix_name_map { id: 27, name: ptr::null(), control: 0, dB: ptr::null() }, // DISABLED: EU (for what?)
    // 28: FU speaker (mute)
    usbmix_name_map { id: 29, name: ptr::null(), control: 0, dB: ptr::null() }, // Digital Input Playback Source?
    usbmix_name_map { id: 0, name: ptr::null(), control: 0, dB: ptr::null() }, // terminator
];

// Sound Blaster MP3+ controls mapping
// The default mixer channels have totally misleading names,
// e.g. no Master and fake PCM volume
// Pavel Mihaylov <bin@bash.info>

pub static MP3PLUS_DB_1: usbmix_dB_map = usbmix_dB_map { min: -4781, max: 0, min_mute: false }; // just guess
pub static MP3PLUS_DB_2: usbmix_dB_map = usbmix_dB_map { min: -1781, max: 618, min_mute: false }; // just guess

pub static MP3PLUS_MAP: &[usbmix_name_map] = &[
    // 1: IT pcm
    // 2: IT mic
    // 3: IT line
    // 4: IT digital in
    // 5: OT digital out
    // 6: OT speaker
    // 7: OT pcm capture
    usbmix_name_map { id: 8, name: c"Capture Source".as_ptr(), control: 0, dB: ptr::null() }, // FU, default PCM Capture Source
        // (Mic, Input 1 = Line input, Input 2 = Optical input)
    usbmix_name_map { id: 9, name: c"Master Playback".as_ptr(), control: 0, dB: ptr::null() }, // FU, default Speaker 1
    // { 10, "Mic Capture", 1 }, // FU, Mic Capture
    usbmix_name_map { id: 10, name: ptr::null(), control: 2, dB: &MP3PLUS_DB_2 }, // FU, Mic Capture
    usbmix_name_map { id: 10, name: c"Mic Boost".as_ptr(), control: 7, dB: ptr::null() }, // FU, default Auto Gain Input
    usbmix_name_map { id: 11, name: c"Line Capture".as_ptr(), control: 0, dB: &MP3PLUS_DB_2 }, // FU, default PCM Capture
    usbmix_name_map { id: 12, name: c"Digital In Playback".as_ptr(), control: 0, dB: ptr::null() }, // FU, default PCM 1
    usbmix_name_map { id: 13, name: ptr::null(), control: 0, dB: &MP3PLUS_DB_1 }, // FU, default Mic Playback
    usbmix_name_map { id: 14, name: c"Line Playback".as_ptr(), control: 0, dB: &MP3PLUS_DB_1 }, // FU, default Speaker
    // 15: MU
    usbmix_name_map { id: 0, name: ptr::null(), control: 0, dB: ptr::null() }, // terminator
];

// Topology of SB Audigy 2 NX
//
//           +----------------------------->EU[27]--+
//           |                                      v
//           | +----------------------------------->SU[29]---->FU[22]-->Dig_OUT[24]
//           | |                                    ^
// USB_IN[1]-+------------+              +->EU[17]->+->FU[11]-+
//             |          v              |          v         |
// Dig_IN[4]---+->FU[6]-->MU[16]->FU[18]-+->EU[21]->SU[31]----->FU[30]->Hph_OUT[20]
//             |          ^              |                    |
// Lin_IN[7]-+--->FU[8]---+              +->EU[23]->FU[28]------------->Spk_OUT[19]
//           | |                                              v
//           +--->FU[12]------------------------------------->SU[14]--->USB_OUT[15]
//             |                                              ^
//             +->FU[13]--------------------------------------+

pub static AUDIGY2NX_MAP: &[usbmix_name_map] = &[
    // 1: IT pcm playback
    // 4: IT digital in
    usbmix_name_map { id: 6, name: c"Digital In Playback".as_ptr(), control: 0, dB: ptr::null() }, // FU
    // 7: IT line in
    usbmix_name_map { id: 8, name: c"Line Playback".as_ptr(), control: 0, dB: ptr::null() }, // FU
    usbmix_name_map { id: 11, name: c"What-U-Hear Capture".as_ptr(), control: 0, dB: ptr::null() }, // FU
    usbmix_name_map { id: 12, name: c"Line Capture".as_ptr(), control: 0, dB: ptr::null() }, // FU
    usbmix_name_map { id: 13, name: c"Digital In Capture".as_ptr(), control: 0, dB: ptr::null() }, // FU
    usbmix_name_map { id: 14, name: c"Capture Source".as_ptr(), control: 0, dB: ptr::null() }, // SU
    // 15: OT pcm capture
    // 16: MU w/o controls
    usbmix_name_map { id: 17, name: ptr::null(), control: 0, dB: ptr::null() }, // DISABLED: EU (for what?)
    usbmix_name_map { id: 18, name: c"Master Playback".as_ptr(), control: 0, dB: ptr::null() }, // FU
    // 19: OT speaker
    // 20: OT headphone
    usbmix_name_map { id: 21, name: ptr::null(), control: 0, dB: ptr::null() }, // DISABLED: EU (for what?)
    usbmix_name_map { id: 22, name: c"Digital Out Playback".as_ptr(), control: 0, dB: ptr::null() }, // FU
    usbmix_name_map { id: 23, name: ptr::null(), control: 0, dB: ptr::null() }, // DISABLED: EU (for what?)
    // 24: OT digital out
    usbmix_name_map { id: 27, name: ptr::null(), control: 0, dB: ptr::null() }, // DISABLED: EU (for what?)
    usbmix_name_map { id: 28, name: c"Speaker Playback".as_ptr(), control: 0, dB: ptr::null() }, // FU
    usbmix_name_map { id: 29, name: c"Digital Out Source".as_ptr(), control: 0, dB: ptr::null() }, // SU
    usbmix_name_map { id: 30, name: c"Headphone Playback".as_ptr(), control: 0, dB: ptr::null() }, // FU
    usbmix_name_map { id: 31, name: c"Headphone Source".as_ptr(), control: 0, dB: ptr::null() }, // SU
    usbmix_name_map { id: 0, name: ptr::null(), control: 0, dB: ptr::null() }, // terminator
];

pub static MBOX1_MAP: &[usbmix_name_map] = &[
    usbmix_name_map { id: 1, name: c"Clock".as_ptr(), control: 0, dB: ptr::null() },
    usbmix_name_map { id: 0, name: ptr::null(), control: 0, dB: ptr::null() }, // terminator
];

const C400_SELECTOR_NAMES_0: &[*const c_char; 2] = &[
    c"Internal".as_ptr(),
    c"SPDIF".as_ptr(),
];

pub static C400_SELECTORS: &[usbmix_selector_map] = &[
    usbmix_selector_map {
        id: 0x80,
        count: 2,
        names: C400_SELECTOR_NAMES_0.as_ptr(),
    },
    usbmix_selector_map { id: 0, count: 0, names: ptr::null() }, // terminator
];

const AUDIGY2NX_SELECTOR_NAMES_0: &[*const c_char; 3] = &[
    c"Line".as_ptr(),
    c"Digital In".as_ptr(),
    c"What-U-Hear".as_ptr(),
];

const AUDIGY2NX_SELECTOR_NAMES_1: &[*const c_char; 3] = &[
    c"Front".as_ptr(),
    c"PCM".as_ptr(),
    c"Digital In".as_ptr(),
];

const AUDIGY2NX_SELECTOR_NAMES_2: &[*const c_char; 2] = &[
    c"Front".as_ptr(),
    c"Side".as_ptr(),
];

pub static AUDIGY2NX_SELECTORS: &[usbmix_selector_map] = &[
    usbmix_selector_map {
        id: 14, // Capture Source
        count: 3,
        names: AUDIGY2NX_SELECTOR_NAMES_0.as_ptr(),
    },
    usbmix_selector_map {
        id: 29, // Digital Out Source
        count: 3,
        names: AUDIGY2NX_SELECTOR_NAMES_1.as_ptr(),
    },
    usbmix_selector_map {
        id: 31, // Headphone Source
        count: 2,
        names: AUDIGY2NX_SELECTOR_NAMES_2.as_ptr(),
    },
    usbmix_selector_map { id: 0, count: 0, names: ptr::null() }, // terminator
];

// Creative SoundBlaster Live! 24-bit External
pub static LIVE24EXT_MAP: &[usbmix_name_map] = &[
    // 2: PCM Playback Volume
    usbmix_name_map { id: 5, name: c"Mic Capture".as_ptr(), control: 0, dB: ptr::null() }, // FU, default PCM Capture Volume
    usbmix_name_map { id: 0, name: ptr::null(), control: 0, dB: ptr::null() }, // terminator
];

// LineX FM Transmitter entry - needed to bypass controls bug
pub static LINEX_MAP: &[usbmix_name_map] = &[
    // 1: IT pcm
    // 2: OT Speaker
    usbmix_name_map { id: 3, name: c"Master".as_ptr(), control: 0, dB: ptr::null() }, // FU: master volume - left / right / mute
    usbmix_name_map { id: 0, name: ptr::null(), control: 0, dB: ptr::null() }, // terminator
];

pub static MAYA44_MAP: &[usbmix_name_map] = &[
    // 1: IT line
    usbmix_name_map { id: 2, name: c"Line Playback".as_ptr(), control: 0, dB: ptr::null() }, // FU
    // 3: IT line
    usbmix_name_map { id: 4, name: c"Line Playback".as_ptr(), control: 0, dB: ptr::null() }, // FU
    // 5: IT pcm playback
    // 6: MU
    usbmix_name_map { id: 7, name: c"Master Playback".as_ptr(), control: 0, dB: ptr::null() }, // FU
    // 8: OT speaker
    // 9: IT line
    usbmix_name_map { id: 10, name: c"Line Capture".as_ptr(), control: 0, dB: ptr::null() }, // FU
    // 11: MU
    // 12: OT pcm capture
    usbmix_name_map { id: 0, name: ptr::null(), control: 0, dB: ptr::null() }, // terminator
];

// Section "justlink_map" below added by James Courtier-Dutton <James@superbug.demon.co.uk>
// sourced from Maplin Electronics (https://www.maplin.co.uk), part number A56AK
// Part has 2 connectors that act as a single output. (TOSLINK Optical for digital out, and 3.5mm Jack for Analogue out.)
// The USB Mixer publishes a Microphone and extra Volume controls for it, but none exist on the device,
// so this map removes all unwanted sliders from alsamixer

pub static JUSTLINK_MAP: &[usbmix_name_map] = &[
    // 1: IT pcm playback
    // 2: Not present
    usbmix_name_map { id: 3, name: ptr::null(), control: 0, dB: ptr::null() }, // IT mic (No mic input on device)
    // 4: Not present
    // 5: OT speaker
    // 6: OT pcm capture
    usbmix_name_map { id: 7, name: c"Master Playback".as_ptr(), control: 0, dB: ptr::null() }, // Mute/volume for speaker
    usbmix_name_map { id: 8, name: ptr::null(), control: 0, dB: ptr::null() }, // Capture Switch (No capture inputs on device)
    usbmix_name_map { id: 9, name: ptr::null(), control: 0, dB: ptr::null() }, // Capture Mute/volume (No capture inputs on device)
    // 0xa: Not present
    // 0xb: MU (w/o controls)
    usbmix_name_map { id: 0xc, name: ptr::null(), control: 0, dB: ptr::null() }, // Mic feedback Mute/volume (No capture inputs on device)
    usbmix_name_map { id: 0, name: ptr::null(), control: 0, dB: ptr::null() }, // terminator
];

// TerraTec Aureon 5.1 MkII USB
pub static AUREON_51_2_MAP: &[usbmix_name_map] = &[
    // 1: IT USB
    // 2: IT Mic
    // 3: IT Line
    // 4: IT SPDIF
    // 5: OT SPDIF
    // 6: OT Speaker
    // 7: OT USB
    usbmix_name_map { id: 8, name: c"Capture Source".as_ptr(), control: 0, dB: ptr::null() }, // SU
    usbmix_name_map { id: 9, name: c"Master Playback".as_ptr(), control: 0, dB: ptr::null() }, // FU
    usbmix_name_map { id: 10, name: c"Mic Capture".as_ptr(), control: 0, dB: ptr::null() }, // FU
    usbmix_name_map { id: 11, name: c"Line Capture".as_ptr(), control: 0, dB: ptr::null() }, // FU
    usbmix_name_map { id: 12, name: c"IEC958 In Capture".as_ptr(), control: 0, dB: ptr::null() }, // FU
    usbmix_name_map { id: 13, name: c"Mic Playback".as_ptr(), control: 0, dB: ptr::null() }, // FU
    usbmix_name_map { id: 14, name: c"Line Playback".as_ptr(), control: 0, dB: ptr::null() }, // FU
    // 15: MU
    usbmix_name_map { id: 0, name: ptr::null(), control: 0, dB: ptr::null() }, // terminator
];

pub static SCRATCH_LIVE_MAP: &[usbmix_name_map] = &[
    // 1: IT Line 1 (USB streaming)
    // 2: OT Line 1 (Speaker)
    // 3: IT Line 1 (Line connector)
    usbmix_name_map { id: 4, name: c"Line 1 In".as_ptr(), control: 0, dB: ptr::null() }, // FU
    // 5: OT Line 1 (USB streaming)
    // 6: IT Line 2 (USB streaming)
    // 7: OT Line 2 (Speaker)
    // 8: IT Line 2 (Line connector)
    usbmix_name_map { id: 9, name: c"Line 2 In".as_ptr(), control: 0, dB: ptr::null() }, // FU
    // 10: OT Line 2 (USB streaming)
    // 11: IT Mic (Line connector)
    // 12: OT Mic (USB streaming)
    usbmix_name_map { id: 0, name: ptr::null(), control: 0, dB: ptr::null() }, // terminator
];

pub static EBOX44_MAP: &[usbmix_name_map] = &[
    usbmix_name_map { id: 4, name: ptr::null(), control: 0, dB: ptr::null() }, // FU
    usbmix_name_map { id: 6, name: ptr::null(), control: 0, dB: ptr::null() }, // MU
    usbmix_name_map { id: 7, name: ptr::null(), control: 0, dB: ptr::null() }, // FU
    usbmix_name_map { id: 10, name: ptr::null(), control: 0, dB: ptr::null() }, // FU
    usbmix_name_map { id: 11, name: ptr::null(), control: 0, dB: ptr::null() }, // MU
    usbmix_name_map { id: 0, name: ptr::null(), control: 0, dB: ptr::null() }, // terminator
];

// "Gamesurround Muse Pocket LT" looks same like "Sound Blaster MP3+"
//  most important difference is SU[8], it should be set to "Capture Source"
//  to make alsamixer and PA working properly.
//  FIXME: or mp3plus_map should use "Capture Source" too,
//  so this maps can be merged

pub static HERCULES_USB51_MAP: &[usbmix_name_map] = &[
    usbmix_name_map { id: 8, name: c"Capture Source".as_ptr(), control: 0, dB: ptr::null() }, // SU, default "PCM Capture Source"
    usbmix_name_map { id: 9, name: c"Master Playback".as_ptr(), control: 0, dB: ptr::null() }, // FU, default "Speaker Playback"
    usbmix_name_map { id: 10, name: c"Mic Boost".as_ptr(), control: 7, dB: ptr::null() }, // FU, default "Auto Gain Input"
    usbmix_name_map { id: 11, name: c"Line Capture".as_ptr(), control: 0, dB: ptr::null() }, // FU, default "PCM Capture"
    usbmix_name_map { id: 13, name: c"Mic Bypass Playback".as_ptr(), control: 0, dB: ptr::null() }, // FU, default "Mic Playback"
    usbmix_name_map { id: 14, name: c"Line Bypass Playback".as_ptr(), control: 0, dB: ptr::null() }, // FU, default "Line Playback"
    usbmix_name_map { id: 0, name: ptr::null(), control: 0, dB: ptr::null() }, // terminator
];

// Plantronics Gamecom 780 has a broken volume control, better to disable it
pub static GAMECOM780_MAP: &[usbmix_name_map] = &[
    usbmix_name_map { id: 9, name: ptr::null(), control: 0, dB: ptr::null() }, // FU, speaker out
    usbmix_name_map { id: 0, name: ptr::null(), control: 0, dB: ptr::null() }, // terminator
];

// some (all?) SCMS USB3318 devices are affected by a firmware lock up
// when anything attempts to access FU 10 (control)

pub static SCMS_USB3318_MAP: &[usbmix_name_map] = &[
    usbmix_name_map { id: 10, name: ptr::null(), control: 0, dB: ptr::null() },
    usbmix_name_map { id: 0, name: ptr::null(), control: 0, dB: ptr::null() }, // terminator
];

// Bose companion 5, the dB conversion factor is 16 instead of 256

pub static BOSE_COMPANION5_DB: usbmix_dB_map = usbmix_dB_map { min: -5006, max: -6, min_mute: false };
pub static BOSE_COMPANION5_MAP: &[usbmix_name_map] = &[
    usbmix_name_map { id: 3, name: ptr::null(), control: 0, dB: &BOSE_COMPANION5_DB },
    usbmix_name_map { id: 0, name: ptr::null(), control: 0, dB: ptr::null() }, // terminator
];

// Bose Revolve+ SoundLink, correction of dB maps

pub static BOSE_SOUNDLINK_DB: usbmix_dB_map = usbmix_dB_map { min: -8283, max: 0, min_mute: true };
pub static BOSE_SOUNDLINK_MAP: &[usbmix_name_map] = &[
    usbmix_name_map { id: 2, name: ptr::null(), control: 0, dB: &BOSE_SOUNDLINK_DB },
    usbmix_name_map { id: 0, name: ptr::null(), control: 0, dB: ptr::null() }, // terminator
];

/*
 * Razer Barracuda X 2.4: Firmware reports cval->min = -16800 in 1/256 dB units
 * (-65.62 dB), which stock ALSA misinterprets as a -168 dB floor
 */

pub static RAZER_BARRACUDA_X_2_4_DB: usbmix_dB_map = usbmix_dB_map { min: -6562, max: 0, min_mute: false };
pub static RAZER_BARRACUDA_X_2_4_MAP: &[usbmix_name_map] = &[
    usbmix_name_map { id: 2, name: ptr::null(), control: 0, dB: &RAZER_BARRACUDA_X_2_4_DB },
    usbmix_name_map { id: 0, name: ptr::null(), control: 0, dB: ptr::null() }, // terminator
];

// Sennheiser Communications Headset [PC 8], the dB value is reported as -6 negative maximum

pub static SENNHEISER_PC8_DB: usbmix_dB_map = usbmix_dB_map { min: -9500, max: 0, min_mute: false };
pub static SENNHEISER_PC8_MAP: &[usbmix_name_map] = &[
    usbmix_name_map { id: 9, name: ptr::null(), control: 0, dB: &SENNHEISER_PC8_DB },
    usbmix_name_map { id: 0, name: ptr::null(), control: 0, dB: ptr::null() }, // terminator
];

/*
 * Dell usb dock with ALC4020 codec had a firmware problem where it got
 * screwed up when zero volume is passed; just skip it as a workaround
 *
 * Also the extension unit gives an access error, so skip it as well.
 */

pub static DELL_ALC4020_MAP: &[usbmix_name_map] = &[
    usbmix_name_map { id: 4, name: ptr::null(), control: 0, dB: ptr::null() }, // extension unit
    usbmix_name_map { id: 16, name: ptr::null(), control: 0, dB: ptr::null() },
    usbmix_name_map { id: 19, name: ptr::null(), control: 0, dB: ptr::null() },
    usbmix_name_map { id: 0, name: ptr::null(), control: 0, dB: ptr::null() }, // terminator
];

/*
 * Corsair Virtuoso calls everything "Headset" without this, leading to
 * applications moving the sidetone control instead of the main one.
 */

pub static CORSAIR_VIRTUOSO_MAP: &[usbmix_name_map] = &[
    usbmix_name_map { id: 3, name: c"Mic Capture".as_ptr(), control: 0, dB: ptr::null() },
    usbmix_name_map { id: 6, name: c"Sidetone Playback".as_ptr(), control: 0, dB: ptr::null() },
    usbmix_name_map { id: 0, name: ptr::null(), control: 0, dB: ptr::null() }, // terminator
];

// Microsoft USB Link headset
// a guess work: raw playback volume values are from 2 to 129

pub static MS_USB_LINK_DB: usbmix_dB_map = usbmix_dB_map { min: -3225, max: 0, min_mute: true };
pub static MS_USB_LINK_MAP: &[usbmix_name_map] = &[
    usbmix_name_map { id: 9, name: ptr::null(), control: 0, dB: &MS_USB_LINK_DB },
    usbmix_name_map { id: 10, name: ptr::null(), control: 0, dB: ptr::null() }, // Headset Capture volume; seems non-working, disabled
    usbmix_name_map { id: 0, name: ptr::null(), control: 0, dB: ptr::null() }, // terminator
];

// KTMicro USB

pub static S31B2_0022_MAP: &[usbmix_name_map] = &[
    usbmix_name_map { id: 23, name: c"Speaker Playback".as_ptr(), control: 0, dB: ptr::null() },
    usbmix_name_map { id: 18, name: c"Headphone Playback".as_ptr(), control: 0, dB: ptr::null() },
    usbmix_name_map { id: 0, name: ptr::null(), control: 0, dB: ptr::null() }, // terminator
];

// ASUS ROG Zenith II with Realtek ALC1220-VB

pub static ASUS_ZENITH_II_MAP: &[usbmix_name_map] = &[
    usbmix_name_map { id: 19, name: ptr::null(), control: 12, dB: ptr::null() }, // FU, Input Gain Pad - broken response, disabled
    usbmix_name_map { id: 16, name: c"Speaker".as_ptr(), control: 0, dB: ptr::null() }, // OT
    usbmix_name_map { id: 22, name: c"Speaker Playback".as_ptr(), control: 0, dB: ptr::null() }, // FU
    usbmix_name_map { id: 7, name: c"Line".as_ptr(), control: 0, dB: ptr::null() }, // IT
    usbmix_name_map { id: 19, name: c"Line Capture".as_ptr(), control: 0, dB: ptr::null() }, // FU
    usbmix_name_map { id: 8, name: c"Mic".as_ptr(), control: 0, dB: ptr::null() }, // IT
    usbmix_name_map { id: 20, name: c"Mic Capture".as_ptr(), control: 0, dB: ptr::null() }, // FU
    usbmix_name_map { id: 9, name: c"Front Mic".as_ptr(), control: 0, dB: ptr::null() }, // IT
    usbmix_name_map { id: 21, name: c"Front Mic Capture".as_ptr(), control: 0, dB: ptr::null() }, // FU
    usbmix_name_map { id: 17, name: c"IEC958".as_ptr(), control: 0, dB: ptr::null() }, // OT
    usbmix_name_map { id: 23, name: c"IEC958 Playback".as_ptr(), control: 0, dB: ptr::null() }, // FU
    usbmix_name_map { id: 0, name: ptr::null(), control: 0, dB: ptr::null() }, // terminator
];

pub static ASUS_ZENITH_II_CONNECTOR_MAP: &[usbmix_connector_map] = &[
    usbmix_connector_map { placeholder: 10 | (16 << 16) }, // (Back) Speaker
    usbmix_connector_map { placeholder: 11 | (17 << 16) }, // SPDIF
    usbmix_connector_map { placeholder: 13 | (7 << 16) }, // Line
    usbmix_connector_map { placeholder: 14 | (8 << 16) }, // Mic
    usbmix_connector_map { placeholder: 15 | (9 << 16) }, // Front Mic
    usbmix_connector_map { placeholder: 0 }, // terminator
];

pub static LENOVO_P620_REAR_MAP: &[usbmix_name_map] = &[
    usbmix_name_map { id: 19, name: ptr::null(), control: 12, dB: ptr::null() }, // FU, Input Gain Pad
    usbmix_name_map { id: 0, name: ptr::null(), control: 0, dB: ptr::null() }, // terminator
];

// TRX40 mobos with Realtek ALC1220-VB

pub static TRX40_MOBO_MAP: &[usbmix_name_map] = &[
    usbmix_name_map { id: 18, name: ptr::null(), control: 0, dB: ptr::null() }, // OT, IEC958 - broken response, disabled
    usbmix_name_map { id: 19, name: ptr::null(), control: 12, dB: ptr::null() }, // FU, Input Gain Pad - broken response, disabled
    usbmix_name_map { id: 16, name: c"Speaker".as_ptr(), control: 0, dB: ptr::null() }, // OT
    usbmix_name_map { id: 22, name: c"Speaker Playback".as_ptr(), control: 0, dB: ptr::null() }, // FU
    usbmix_name_map { id: 7, name: c"Line".as_ptr(), control: 0, dB: ptr::null() }, // IT
    usbmix_name_map { id: 19, name: c"Line Capture".as_ptr(), control: 0, dB: ptr::null() }, // FU
    usbmix_name_map { id: 17, name: c"Front Headphone".as_ptr(), control: 0, dB: ptr::null() }, // OT
    usbmix_name_map { id: 23, name: c"Front Headphone Playback".as_ptr(), control: 0, dB: ptr::null() }, // FU
    usbmix_name_map { id: 8, name: c"Mic".as_ptr(), control: 0, dB: ptr::null() }, // IT
    usbmix_name_map { id: 20, name: c"Mic Capture".as_ptr(), control: 0, dB: ptr::null() }, // FU
    usbmix_name_map { id: 9, name: c"Front Mic".as_ptr(), control: 0, dB: ptr::null() }, // IT
    usbmix_name_map { id: 21, name: c"Front Mic Capture".as_ptr(), control: 0, dB: ptr::null() }, // FU
    usbmix_name_map { id: 24, name: c"IEC958 Playback".as_ptr(), control: 0, dB: ptr::null() }, // FU
    usbmix_name_map { id: 0, name: ptr::null(), control: 0, dB: ptr::null() }, // terminator
];

pub static TRX40_MOBO_CONNECTOR_MAP: &[usbmix_connector_map] = &[
    usbmix_connector_map { placeholder: 10 | (16 << 16) }, // (Back) Speaker
    usbmix_connector_map { placeholder: 11 | (17 << 16) }, // Front Headphone
    usbmix_connector_map { placeholder: 13 | (7 << 16) }, // Line
    usbmix_connector_map { placeholder: 14 | (8 << 16) }, // Mic
    usbmix_connector_map { placeholder: 15 | (9 << 16) }, // Front Mic
    usbmix_connector_map { placeholder: 0 }, // terminator
];

// Rear panel + front mic on Gigabyte TRX40 Aorus Master with ALC1220-VB

pub static AORUS_MASTER_ALC1220VB_MAP: &[usbmix_name_map] = &[
    usbmix_name_map { id: 17, name: ptr::null(), control: 0, dB: ptr::null() }, // OT, IEC958?, disabled
    usbmix_name_map { id: 19, name: ptr::null(), control: 12, dB: ptr::null() }, // FU, Input Gain Pad - broken response, disabled
    usbmix_name_map { id: 16, name: c"Line Out".as_ptr(), control: 0, dB: ptr::null() }, // OT
    usbmix_name_map { id: 22, name: c"Line Out Playback".as_ptr(), control: 0, dB: ptr::null() }, // FU
    usbmix_name_map { id: 7, name: c"Line".as_ptr(), control: 0, dB: ptr::null() }, // IT
    usbmix_name_map { id: 19, name: c"Line Capture".as_ptr(), control: 0, dB: ptr::null() }, // FU
    usbmix_name_map { id: 8, name: c"Mic".as_ptr(), control: 0, dB: ptr::null() }, // IT
    usbmix_name_map { id: 20, name: c"Mic Capture".as_ptr(), control: 0, dB: ptr::null() }, // FU
    usbmix_name_map { id: 9, name: c"Front Mic".as_ptr(), control: 0, dB: ptr::null() }, // IT
    usbmix_name_map { id: 21, name: c"Front Mic Capture".as_ptr(), control: 0, dB: ptr::null() }, // FU
    usbmix_name_map { id: 0, name: ptr::null(), control: 0, dB: ptr::null() }, // terminator
];

// MSI MPG X570S Carbon Max Wifi with ALC4080

pub static MSI_MPG_X570S_CARBON_MAX_WIFI_ALC4080_MAP: &[usbmix_name_map] = &[
    usbmix_name_map { id: 29, name: c"Speaker Playback".as_ptr(), control: 0, dB: ptr::null() },
    usbmix_name_map { id: 30, name: c"Front Headphone Playback".as_ptr(), control: 0, dB: ptr::null() },
    usbmix_name_map { id: 32, name: c"IEC958 Playback".as_ptr(), control: 0, dB: ptr::null() },
    usbmix_name_map { id: 0, name: ptr::null(), control: 0, dB: ptr::null() }, // terminator
];

// Gigabyte B450/550 Mobo

pub static GIGABYTE_B450_MAP: &[usbmix_name_map] = &[
    usbmix_name_map { id: 24, name: ptr::null(), control: 0, dB: ptr::null() }, // OT, IEC958?, disabled
    usbmix_name_map { id: 21, name: c"Speaker".as_ptr(), control: 0, dB: ptr::null() }, // OT
    usbmix_name_map { id: 29, name: c"Speaker Playback".as_ptr(), control: 0, dB: ptr::null() }, // FU
    usbmix_name_map { id: 22, name: c"Headphone".as_ptr(), control: 0, dB: ptr::null() }, // OT
    usbmix_name_map { id: 30, name: c"Headphone Playback".as_ptr(), control: 0, dB: ptr::null() }, // FU
    usbmix_name_map { id: 11, name: c"Line".as_ptr(), control: 0, dB: ptr::null() }, // IT
    usbmix_name_map { id: 27, name: c"Line Capture".as_ptr(), control: 0, dB: ptr::null() }, // FU
    usbmix_name_map { id: 12, name: c"Mic".as_ptr(), control: 0, dB: ptr::null() }, // IT
    usbmix_name_map { id: 28, name: c"Mic Capture".as_ptr(), control: 0, dB: ptr::null() }, // FU
    usbmix_name_map { id: 9, name: c"Front Mic".as_ptr(), control: 0, dB: ptr::null() }, // IT
    usbmix_name_map { id: 25, name: c"Front Mic Capture".as_ptr(), control: 0, dB: ptr::null() }, // FU
    usbmix_name_map { id: 0, name: ptr::null(), control: 0, dB: ptr::null() }, // terminator
];

pub static GIGABYTE_B450_CONNECTOR_MAP: &[usbmix_connector_map] = &[
    usbmix_connector_map { placeholder: 13 | (21 << 16) }, // Speaker
    usbmix_connector_map { placeholder: 14 | (22 << 16) }, // Headphone
    usbmix_connector_map { placeholder: 19 | (11 << 16) }, // Line
    usbmix_connector_map { placeholder: 20 | (12 << 16) }, // Mic
    usbmix_connector_map { placeholder: 17 | (9 << 16) }, // Front Mic
    usbmix_connector_map { placeholder: 0 }, // terminator
];

// Audient iD14 MkI and MkII: FU 12 sits on the monitor mixer branch but is
// traced through to the Speaker output terminal, so it is named "Speaker
// Playback Volume".  On MkII it controls only 4 of 6 playback channels.  MkI
// testing found asymmetric attenuation within the main stereo pair.  Userspace
// adopts this control as the stream's hardware volume, causing imbalance below
// 0 dB.  Give it a non-standard name so that userspace no longer treats it as
// the stream master, while keeping the monitor gain reachable.

pub static AUDIENT_ID14_MAP: &[usbmix_name_map] = &[
    usbmix_name_map { id: 12, name: c"Monitor Mix Playback".as_ptr(), control: 0, dB: ptr::null() }, // FU, partial coverage
    usbmix_name_map { id: 0, name: ptr::null(), control: 0, dB: ptr::null() }, // terminator
];

/*
 * Control map entries
 */

// Helper macro equivalent: USB_ID(vendor, product) -> (vendor << 16) | product
// External constants referenced (to be supplied elsewhere):
// - UAC3_BADD_FU_ID2, UAC3_BADD_FU_ID5, UAC3_BADD_FU_ID7
// - UAC3_FUNCTION_SUBCLASS_*

pub static USBMIX_CTL_MAPS: &[usbmix_ctl_map] = &[
    usbmix_ctl_map {
        id: 0x041e3000, // USB_ID(0x041e, 0x3000)
        map: EXTIGY_MAP.as_ptr(),
        selector_map: ptr::null(),
        connector_map: ptr::null(),
    },
    usbmix_ctl_map {
        id: 0x041e3010, // USB_ID(0x041e, 0x3010)
        map: MP3PLUS_MAP.as_ptr(),
        selector_map: ptr::null(),
        connector_map: ptr::null(),
    },
    usbmix_ctl_map {
        id: 0x041e3020, // USB_ID(0x041e, 0x3020)
        map: AUDIGY2NX_MAP.as_ptr(),
        selector_map: AUDIGY2NX_SELECTORS.as_ptr(),
        connector_map: ptr::null(),
    },
    usbmix_ctl_map {
        id: 0x041e3040, // USB_ID(0x041e, 0x3040)
        map: LIVE24EXT_MAP.as_ptr(),
        selector_map: ptr::null(),
        connector_map: ptr::null(),
    },
    usbmix_ctl_map {
        id: 0x041e3048, // USB_ID(0x041e, 0x3048)
        map: AUDIGY2NX_MAP.as_ptr(),
        selector_map: AUDIGY2NX_SELECTORS.as_ptr(),
        connector_map: ptr::null(),
    },
    usbmix_ctl_map {
        id: 0x047fc010, // USB_ID(0x047f, 0xc010) Plantronics GameCom 780
        map: GAMECOM780_MAP.as_ptr(),
        selector_map: ptr::null(),
        connector_map: ptr::null(),
    },
    usbmix_ctl_map {
        id: 0x06f8c000, // USB_ID(0x06f8, 0xc000) Hercules Gamesurround Muse Pocket LT (USB 5.1 Channel Audio Adapter)
        map: HERCULES_USB51_MAP.as_ptr(),
        selector_map: ptr::null(),
        connector_map: ptr::null(),
    },
    usbmix_ctl_map {
        id: 0x07632030, // USB_ID(0x0763, 0x2030)
        map: ptr::null(),
        selector_map: C400_SELECTORS.as_ptr(),
        connector_map: ptr::null(),
    },
    usbmix_ctl_map {
        id: 0x07632031, // USB_ID(0x0763, 0x2031)
        map: ptr::null(),
        selector_map: C400_SELECTORS.as_ptr(),
        connector_map: ptr::null(),
    },
    usbmix_ctl_map {
        id: 0x08bb2702, // USB_ID(0x08bb, 0x2702)
        map: LINEX_MAP.as_ptr(),
        selector_map: ptr::null(),
        connector_map: ptr::null(),
    },
    usbmix_ctl_map {
        id: 0x0a920091, // USB_ID(0x0a92, 0x0091)
        map: MAYA44_MAP.as_ptr(),
        selector_map: ptr::null(),
        connector_map: ptr::null(),
    },
    usbmix_ctl_map {
        id: 0x0c451158, // USB_ID(0x0c45, 0x1158)
        map: JUSTLINK_MAP.as_ptr(),
        selector_map: ptr::null(),
        connector_map: ptr::null(),
    },
    usbmix_ctl_map {
        id: 0x0ccd0028, // USB_ID(0x0ccd, 0x0028)
        map: AUREON_51_2_MAP.as_ptr(),
        selector_map: ptr::null(),
        connector_map: ptr::null(),
    },
    usbmix_ctl_map {
        id: 0x0bda4014, // USB_ID(0x0bda, 0x4014)
        map: DELL_ALC4020_MAP.as_ptr(),
        selector_map: ptr::null(),
        connector_map: ptr::null(),
    },
    usbmix_ctl_map {
        id: 0x0dba1000, // USB_ID(0x0dba, 0x1000)
        map: MBOX1_MAP.as_ptr(),
        selector_map: ptr::null(),
        connector_map: ptr::null(),
    },
    usbmix_ctl_map {
        id: 0x13e50001, // USB_ID(0x13e5, 0x0001)
        map: SCRATCH_LIVE_MAP.as_ptr(),
        selector_map: ptr::null(),
        connector_map: ptr::null(),
    },
    usbmix_ctl_map {
        id: 0x200c1018, // USB_ID(0x200c, 0x1018)
        map: EBOX44_MAP.as_ptr(),
        selector_map: ptr::null(),
        connector_map: ptr::null(),
    },
    usbmix_ctl_map {
        id: 0x25730008, // USB_ID(0x2573, 0x0008) MAYA44 USB+
        map: MAYA44_MAP.as_ptr(),
        selector_map: ptr::null(),
        connector_map: ptr::null(),
    },
    usbmix_ctl_map {
        id: 0x27080002, // USB_ID(0x2708, 0x0002) Audient iD14 MkI
        map: AUDIENT_ID14_MAP.as_ptr(),
        selector_map: ptr::null(),
        connector_map: ptr::null(),
    },
    usbmix_ctl_map {
        id: 0x27080008, // USB_ID(0x2708, 0x0008) Audient iD14 MkII
        map: AUDIENT_ID14_MAP.as_ptr(),
        selector_map: ptr::null(),
        connector_map: ptr::null(),
    },
    usbmix_ctl_map {
        id: 0x27ac1000, // USB_ID(0x27ac, 0x1000) KEF X300A
        map: SCMS_USB3318_MAP.as_ptr(),
        selector_map: ptr::null(),
        connector_map: ptr::null(),
    },
    usbmix_ctl_map {
        id: 0x25c40003, // USB_ID(0x25c4, 0x0003) Arcam rPAC
        map: SCMS_USB3318_MAP.as_ptr(),
        selector_map: ptr::null(),
        connector_map: ptr::null(),
    },
    usbmix_ctl_map {
        id: 0x05a71020, // USB_ID(0x05a7, 0x1020) Bose Companion 5
        map: BOSE_COMPANION5_MAP.as_ptr(),
        selector_map: ptr::null(),
        connector_map: ptr::null(),
    },
    usbmix_ctl_map {
        id: 0x05a740fa, // USB_ID(0x05a7, 0x40fa) Bose Revolve+ SoundLink
        map: BOSE_SOUNDLINK_MAP.as_ptr(),
        selector_map: ptr::null(),
        connector_map: ptr::null(),
    },
    usbmix_ctl_map {
        id: 0x1b1c0a3f, // USB_ID(0x1b1c, 0x0a3f) Corsair Virtuoso SE Latest (wired mode)
        map: CORSAIR_VIRTUOSO_MAP.as_ptr(),
        selector_map: ptr::null(),
        connector_map: ptr::null(),
    },
    usbmix_ctl_map {
        id: 0x1b1c0a40, // USB_ID(0x1b1c, 0x0a40) Corsair Virtuoso SE Latest (wireless mode)
        map: CORSAIR_VIRTUOSO_MAP.as_ptr(),
        selector_map: ptr::null(),
        connector_map: ptr::null(),
    },
    usbmix_ctl_map {
        id: 0x1b1c0a3d, // USB_ID(0x1b1c, 0x0a3d) Corsair Virtuoso SE (wired mode)
        map: CORSAIR_VIRTUOSO_MAP.as_ptr(),
        selector_map: ptr::null(),
        connector_map: ptr::null(),
    },
    usbmix_ctl_map {
        id: 0x1b1c0a3e, // USB_ID(0x1b1c, 0x0a3e) Corsair Virtuoso SE (wireless mode)
        map: CORSAIR_VIRTUOSO_MAP.as_ptr(),
        selector_map: ptr::null(),
        connector_map: ptr::null(),
    },
    usbmix_ctl_map {
        id: 0x1b1c0a41, // USB_ID(0x1b1c, 0x0a41) Corsair Virtuoso (wired mode)
        map: CORSAIR_VIRTUOSO_MAP.as_ptr(),
        selector_map: ptr::null(),
        connector_map: ptr::null(),
    },
    usbmix_ctl_map {
        id: 0x1b1c0a42, // USB_ID(0x1b1c, 0x0a42) Corsair Virtuoso (wireless mode)
        map: CORSAIR_VIRTUOSO_MAP.as_ptr(),
        selector_map: ptr::null(),
        connector_map: ptr::null(),
    },
    usbmix_ctl_map {
        id: 0x1b1c0a43, // USB_ID(0x1b1c, 0x0a43) Corsair Virtuoso (wired mode, later revision)
        map: CORSAIR_VIRTUOSO_MAP.as_ptr(),
        selector_map: ptr::null(),
        connector_map: ptr::null(),
    },
    usbmix_ctl_map {
        id: 0x1b1c0a44, // USB_ID(0x1b1c, 0x0a44) Corsair Virtuoso (wireless mode, later revision)
        map: CORSAIR_VIRTUOSO_MAP.as_ptr(),
        selector_map: ptr::null(),
        connector_map: ptr::null(),
    },
    usbmix_ctl_map {
        id: 0x1b1c0a6a, // USB_ID(0x1b1c, 0x0a6a) Corsair HS80 RGB Wireless (wired mode)
        map: CORSAIR_VIRTUOSO_MAP.as_ptr(),
        selector_map: ptr::null(),
        connector_map: ptr::null(),
    },
    usbmix_ctl_map {
        id: 0x1b1c0a6b, // USB_ID(0x1b1c, 0x0a6b) Corsair HS80 RGB Wireless (wireless mode)
        map: CORSAIR_VIRTUOSO_MAP.as_ptr(),
        selector_map: ptr::null(),
        connector_map: ptr::null(),
    },
    usbmix_ctl_map {
        id: 0x0414a001, // USB_ID(0x0414, 0xa001) Gigabyte TRX40 Aorus Master (rear panel + front mic)
        map: AORUS_MASTER_ALC1220VB_MAP.as_ptr(),
        selector_map: ptr::null(),
        connector_map: ptr::null(),
    },
    usbmix_ctl_map {
        id: 0x0414a002, // USB_ID(0x0414, 0xa002) Gigabyte TRX40 Aorus Pro WiFi
        map: TRX40_MOBO_MAP.as_ptr(),
        selector_map: ptr::null(),
        connector_map: TRX40_MOBO_CONNECTOR_MAP.as_ptr(),
    },
    usbmix_ctl_map {
        id: 0x0414a00d, // USB_ID(0x0414, 0xa00d) Gigabyte B450/550 Mobo
        map: GIGABYTE_B450_MAP.as_ptr(),
        selector_map: ptr::null(),
        connector_map: GIGABYTE_B450_CONNECTOR_MAP.as_ptr(),
    },
    usbmix_ctl_map {
        id: 0x0b051916, // USB_ID(0x0b05, 0x1916) ASUS ROG Zenith II (main audio)
        map: ASUS_ZENITH_II_MAP.as_ptr(),
        selector_map: ptr::null(),
        connector_map: ASUS_ZENITH_II_CONNECTOR_MAP.as_ptr(),
    },
    usbmix_ctl_map {
        id: 0x0b051917, // USB_ID(0x0b05, 0x1917) ASUS ROG Strix
        map: TRX40_MOBO_MAP.as_ptr(),
        selector_map: ptr::null(),
        connector_map: TRX40_MOBO_CONNECTOR_MAP.as_ptr(),
    },
    usbmix_ctl_map {
        id: 0x0db00d64, // USB_ID(0x0db0, 0x0d64) MSI TRX40 Creator
        map: TRX40_MOBO_MAP.as_ptr(),
        selector_map: ptr::null(),
        connector_map: TRX40_MOBO_CONNECTOR_MAP.as_ptr(),
    },
    usbmix_ctl_map {
        id: 0x0db0419c, // USB_ID(0x0db0, 0x419c) MSI MPG X570S Carbon Max Wifi
        map: MSI_MPG_X570S_CARBON_MAX_WIFI_ALC4080_MAP.as_ptr(),
        selector_map: ptr::null(),
        connector_map: ptr::null(),
    },
    usbmix_ctl_map {
        id: 0x0db0a073, // USB_ID(0x0db0, 0xa073) MSI MAG X570S Torpedo Max
        map: MSI_MPG_X570S_CARBON_MAX_WIFI_ALC4080_MAP.as_ptr(),
        selector_map: ptr::null(),
        connector_map: ptr::null(),
    },
    usbmix_ctl_map {
        id: 0x0db0543d, // USB_ID(0x0db0, 0x543d) MSI TRX40
        map: TRX40_MOBO_MAP.as_ptr(),
        selector_map: ptr::null(),
        connector_map: TRX40_MOBO_CONNECTOR_MAP.as_ptr(),
    },
    usbmix_ctl_map {
        id: 0x26ce0a01, // USB_ID(0x26ce, 0x0a01) Asrock TRX40 Creator
        map: TRX40_MOBO_MAP.as_ptr(),
        selector_map: ptr::null(),
        connector_map: TRX40_MOBO_CONNECTOR_MAP.as_ptr(),
    },
    usbmix_ctl_map {
        id: 0x17aa1046, // USB_ID(0x17aa, 0x1046) Lenovo ThinkStation P620 Rear
        map: LENOVO_P620_REAR_MAP.as_ptr(),
        selector_map: ptr::null(),
        connector_map: ptr::null(),
    },
    usbmix_ctl_map {
        id: 0x15320552, // USB_ID(0x1532, 0x0552) Razer Barracuda X 2.4
        map: RAZER_BARRACUDA_X_2_4_MAP.as_ptr(),
        selector_map: ptr::null(),
        connector_map: ptr::null(),
    },
    usbmix_ctl_map {
        id: 0x13950025, // USB_ID(0x1395, 0x0025) Sennheiser Communications Headset [PC 8]
        map: SENNHEISER_PC8_MAP.as_ptr(),
        selector_map: ptr::null(),
        connector_map: ptr::null(),
    },
    usbmix_ctl_map {
        id: 0x045e083c, // USB_ID(0x045e, 0x083c) Microsoft USB Link headset
        map: MS_USB_LINK_MAP.as_ptr(),
        selector_map: ptr::null(),
        connector_map: ptr::null(),
    },
    usbmix_ctl_map {
        id: 0x31b20022, // USB_ID(0X31b2, 0x0022) KTMicro USB
        map: S31B2_0022_MAP.as_ptr(),
        selector_map: ptr::null(),
        connector_map: ptr::null(),
    },
    usbmix_ctl_map { id: 0, map: ptr::null(), selector_map: ptr::null(), connector_map: ptr::null() }, // terminator
];

/*
 * Control map entries for UAC3 BADD profiles
 */

// External constants referenced (to be supplied elsewhere):
// - UAC3_BADD_FU_ID2, UAC3_BADD_FU_ID5, UAC3_BADD_FU_ID7
// - UAC3_FUNCTION_SUBCLASS_*

pub static UAC3_BADD_GENERIC_IO_MAP: &[usbmix_name_map] = &[
    // using placeholder values for external constants:
    usbmix_name_map { id: 2, name: c"Generic Out Playback".as_ptr(), control: 0, dB: ptr::null() }, // UAC3_BADD_FU_ID2
    usbmix_name_map { id: 5, name: c"Generic In Capture".as_ptr(), control: 0, dB: ptr::null() }, // UAC3_BADD_FU_ID5
    usbmix_name_map { id: 0, name: ptr::null(), control: 0, dB: ptr::null() }, // terminator
];

pub static UAC3_BADD_HEADPHONE_MAP: &[usbmix_name_map] = &[
    usbmix_name_map { id: 2, name: c"Headphone Playback".as_ptr(), control: 0, dB: ptr::null() }, // UAC3_BADD_FU_ID2
    usbmix_name_map { id: 0, name: ptr::null(), control: 0, dB: ptr::null() }, // terminator
];

pub static UAC3_BADD_SPEAKER_MAP: &[usbmix_name_map] = &[
    usbmix_name_map { id: 2, name: c"Speaker Playback".as_ptr(), control: 0, dB: ptr::null() }, // UAC3_BADD_FU_ID2
    usbmix_name_map { id: 0, name: ptr::null(), control: 0, dB: ptr::null() }, // terminator
];

pub static UAC3_BADD_MICROPHONE_MAP: &[usbmix_name_map] = &[
    usbmix_name_map { id: 5, name: c"Mic Capture".as_ptr(), control: 0, dB: ptr::null() }, // UAC3_BADD_FU_ID5
    usbmix_name_map { id: 0, name: ptr::null(), control: 0, dB: ptr::null() }, // terminator
];

// Covers also 'headset adapter' profile
pub static UAC3_BADD_HEADSET_MAP: &[usbmix_name_map] = &[
    usbmix_name_map { id: 2, name: c"Headset Playback".as_ptr(), control: 0, dB: ptr::null() }, // UAC3_BADD_FU_ID2
    usbmix_name_map { id: 5, name: c"Headset Capture".as_ptr(), control: 0, dB: ptr::null() }, // UAC3_BADD_FU_ID5
    usbmix_name_map { id: 7, name: c"Sidetone Mixing".as_ptr(), control: 0, dB: ptr::null() }, // UAC3_BADD_FU_ID7
    usbmix_name_map { id: 0, name: ptr::null(), control: 0, dB: ptr::null() }, // terminator
];

pub static UAC3_BADD_SPEAKERPHONE_MAP: &[usbmix_name_map] = &[
    usbmix_name_map { id: 2, name: c"Speaker Playback".as_ptr(), control: 0, dB: ptr::null() }, // UAC3_BADD_FU_ID2
    usbmix_name_map { id: 5, name: c"Mic Capture".as_ptr(), control: 0, dB: ptr::null() }, // UAC3_BADD_FU_ID5
    usbmix_name_map { id: 0, name: ptr::null(), control: 0, dB: ptr::null() }, // terminator
];

pub static UAC3_BADD_USBMIX_CTL_MAPS: &[usbmix_ctl_map] = &[
    usbmix_ctl_map {
        id: 0x01, // UAC3_FUNCTION_SUBCLASS_GENERIC_IO
        map: UAC3_BADD_GENERIC_IO_MAP.as_ptr(),
        selector_map: ptr::null(),
        connector_map: ptr::null(),
    },
    usbmix_ctl_map {
        id: 0x02, // UAC3_FUNCTION_SUBCLASS_HEADPHONE
        map: UAC3_BADD_HEADPHONE_MAP.as_ptr(),
        selector_map: ptr::null(),
        connector_map: ptr::null(),
    },
    usbmix_ctl_map {
        id: 0x03, // UAC3_FUNCTION_SUBCLASS_SPEAKER
        map: UAC3_BADD_SPEAKER_MAP.as_ptr(),
        selector_map: ptr::null(),
        connector_map: ptr::null(),
    },
    usbmix_ctl_map {
        id: 0x04, // UAC3_FUNCTION_SUBCLASS_MICROPHONE
        map: UAC3_BADD_MICROPHONE_MAP.as_ptr(),
        selector_map: ptr::null(),
        connector_map: ptr::null(),
    },
    usbmix_ctl_map {
        id: 0x05, // UAC3_FUNCTION_SUBCLASS_HEADSET
        map: UAC3_BADD_HEADSET_MAP.as_ptr(),
        selector_map: ptr::null(),
        connector_map: ptr::null(),
    },
    usbmix_ctl_map {
        id: 0x06, // UAC3_FUNCTION_SUBCLASS_HEADSET_ADAPTER
        map: UAC3_BADD_HEADSET_MAP.as_ptr(),
        selector_map: ptr::null(),
        connector_map: ptr::null(),
    },
    usbmix_ctl_map {
        id: 0x07, // UAC3_FUNCTION_SUBCLASS_SPEAKERPHONE
        map: UAC3_BADD_SPEAKERPHONE_MAP.as_ptr(),
        selector_map: ptr::null(),
        connector_map: ptr::null(),
    },
    usbmix_ctl_map { id: 0, map: ptr::null(), selector_map: ptr::null(), connector_map: ptr::null() }, // terminator
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
