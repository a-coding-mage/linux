/* SPDX-License-Identifier: GPL-2.0 */
/* speakup_dtlk.h - header file for speakups DoubleTalk driver. */

pub const SYNTH_IO_EXTENT: u32 = 0x02;
pub const SYNTH_CLEAR: u32 = 0x18; // stops speech

// TTS Port Status Flags
pub const TTS_READABLE: u32 = 0x80; // mask for bit which is nonzero if a byte can be read from the TTS port
pub const TTS_SPEAKING: u32 = 0x40; // mask for SYNC bit, which is nonzero while DoubleTalk is producing output with TTS, PCM or CVSD synthesizers or tone generators (that is, all but LPC)
pub const TTS_SPEAKING2: u32 = 0x20; // mask for SYNC2 bit, which falls to zero up to 0.4 sec before speech stops
pub const TTS_WRITABLE: u32 = 0x10; // mask for RDY bit, which when set to 1, indicates the TTS port is ready to accept a byte of data. The RDY bit goes zero 2-3 usec after writing, and goes 1 again 180-190 usec later.
pub const TTS_ALMOST_FULL: u32 = 0x08; // mask for AF bit: When set to 1, indicates that less than 300 bytes are available in the TTS input buffer. AF is always 0 in the PCM, TGN and CVSD modes.
pub const TTS_ALMOST_EMPTY: u32 = 0x04; // mask for AE bit: When set to 1, indicates that less than 300 bytes are remaining in DoubleTalk's input (TTS or PCM) buffer. AE is always 1 in the TGN and CVSD modes.

// data returned by Interrogate command
#[repr(C)]
pub struct synth_settings {
    pub serial_number: u16,      // 0-7Fh:0-7Fh
    pub rom_version: [u8; 24],   // null terminated string
    pub mode: u8,                // 0=Character; 1=Phoneme; 2=Text
    pub punc_level: u8,          // nB; 0-7
    pub formant_freq: u8,        // nF; 0-9
    pub pitch: u8,               // nP; 0-99
    pub speed: u8,               // nS; 0-9
    pub volume: u8,              // nV; 0-9
    pub tone: u8,                // nX; 0-2
    pub expression: u8,          // nE; 0-9
    pub ext_dict_loaded: u8,     // 1=exception dictionary loaded
    pub ext_dict_status: u8,     // 1=exception dictionary enabled
    pub free_ram: u8,            // # pages (truncated) remaining for text buffer
    pub articulation: u8,        // nA; 0-9
    pub reverb: u8,              // nR; 0-9
    pub eob: u8,                 // 7Fh value indicating end of parameter block
    pub has_indexing: u8,        // nonzero if indexing is implemented
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
