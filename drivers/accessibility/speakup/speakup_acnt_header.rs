/* SPDX-License-Identifier: GPL-2.0 */
/* speakup_acntpc.h - header file for speakups Accent-PC driver. */

pub const SYNTH_IO_EXTENT: u8 = 0x02;

pub const SYNTH_CLEAR: u8 = 0x18; // stops speech

// Port Status Flags
pub const SYNTH_READABLE: u8 = 0x01; // mask for bit which is nonzero if a byte can be read from the data port
pub const SYNTH_WRITABLE: u8 = 0x02; // mask for RDY bit, which when set to 1, indicates the data port is ready to accept a byte of data.
pub const SYNTH_QUIET: u8 = b'S'; // synth is not speaking
pub const SYNTH_FULL: u8 = b'F'; // synth is full.
pub const SYNTH_ALMOST_EMPTY: u8 = b'M'; // synth has less than 2 seconds of text left
pub const SYNTH_SPEAKING: u8 = b's'; // synth is speaking and has a fare way to go

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
