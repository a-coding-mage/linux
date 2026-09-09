/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * ultrasound.h - Macros for programming the Gravis Ultrasound.
 * These macros are extremely device dependent and not portable.
 *
 * Private events for Gravis Ultrasound (GUS).
 */

pub const _GUS_NUMVOICES: u8 = 0x00;
pub const _GUS_VOICESAMPLE: u8 = 0x01; /* OBSOLETE */
pub const _GUS_VOICEON: u8 = 0x02;
pub const _GUS_VOICEOFF: u8 = 0x03;
pub const _GUS_VOICEMODE: u8 = 0x04;
pub const _GUS_VOICEBALA: u8 = 0x05;
pub const _GUS_VOICEFREQ: u8 = 0x06;
pub const _GUS_VOICEVOL: u8 = 0x07;
pub const _GUS_RAMPRANGE: u8 = 0x08;
pub const _GUS_RAMPRATE: u8 = 0x09;
pub const _GUS_RAMPMODE: u8 = 0x0a;
pub const _GUS_RAMPON: u8 = 0x0b;
pub const _GUS_RAMPOFF: u8 = 0x0c;
pub const _GUS_VOICEFADE: u8 = 0x0d;
pub const _GUS_VOLUME_SCALE: u8 = 0x0e;
pub const _GUS_VOICEVOL2: u8 = 0x0f;
pub const _GUS_VOICE_POS: u8 = 0x10;

/* GUS API macros. _SEQ_NEEDBUF, _SEQ_ADVBUF, _seqbuf, _seqbufptr, and
 * SEQ_PRIVATE are supplied by the surrounding sequencer interface. */
#[macro_export]
macro_rules! _GUS_CMD {
    ($chn:expr, $voice:expr, $cmd:expr, $p1:expr, $p2:expr) => {{
        _SEQ_NEEDBUF!(8);
        _seqbuf[_seqbufptr] = SEQ_PRIVATE;
        _seqbuf[_seqbufptr + 1] = $chn;
        _seqbuf[_seqbufptr + 2] = $cmd;
        _seqbuf[_seqbufptr + 3] = $voice;
        unsafe {
            core::ptr::write_unaligned(
                _seqbuf.as_mut_ptr().add(_seqbufptr + 4) as *mut u16,
                $p1 as u16,
            );
            core::ptr::write_unaligned(
                _seqbuf.as_mut_ptr().add(_seqbufptr + 6) as *mut u16,
                $p2 as u16,
            );
        }
        _SEQ_ADVBUF!(8);
    }};
}

#[macro_export] macro_rules! GUS_NUMVOICES { ($chn:expr, $p1:expr) => { _GUS_CMD!($chn, 0, _GUS_NUMVOICES, $p1, 0) }; }
#[macro_export] macro_rules! GUS_VOICESAMPLE { ($chn:expr, $voice:expr, $p1:expr) => { _GUS_CMD!($chn, $voice, _GUS_VOICESAMPLE, $p1, 0) }; }
#[macro_export] macro_rules! GUS_VOICEON { ($chn:expr, $voice:expr, $p1:expr) => { _GUS_CMD!($chn, $voice, _GUS_VOICEON, $p1, 0) }; }
#[macro_export] macro_rules! GUS_VOICEOFF { ($chn:expr, $voice:expr) => { _GUS_CMD!($chn, $voice, _GUS_VOICEOFF, 0, 0) }; }
#[macro_export] macro_rules! GUS_VOICEFADE { ($chn:expr, $voice:expr) => { _GUS_CMD!($chn, $voice, _GUS_VOICEFADE, 0, 0) }; }
#[macro_export] macro_rules! GUS_VOICEMODE { ($chn:expr, $voice:expr, $p1:expr) => { _GUS_CMD!($chn, $voice, _GUS_VOICEMODE, $p1, 0) }; }
#[macro_export] macro_rules! GUS_VOICEBALA { ($chn:expr, $voice:expr, $p1:expr) => { _GUS_CMD!($chn, $voice, _GUS_VOICEBALA, $p1, 0) }; }
#[macro_export] macro_rules! GUS_VOICEFREQ { ($chn:expr, $voice:expr, $p:expr) => { _GUS_CMD!($chn, $voice, _GUS_VOICEFREQ, ($p) & 0xffff, (($p) >> 16) & 0xffff) }; }
#[macro_export] macro_rules! GUS_VOICEVOL { ($chn:expr, $voice:expr, $p1:expr) => { _GUS_CMD!($chn, $voice, _GUS_VOICEVOL, $p1, 0) }; }
#[macro_export] macro_rules! GUS_VOICEVOL2 { ($chn:expr, $voice:expr, $p1:expr) => { _GUS_CMD!($chn, $voice, _GUS_VOICEVOL2, $p1, 0) }; }
#[macro_export] macro_rules! GUS_RAMPRANGE { ($chn:expr, $voice:expr, $low:expr, $high:expr) => { _GUS_CMD!($chn, $voice, _GUS_RAMPRANGE, $low, $high) }; }
#[macro_export] macro_rules! GUS_RAMPRATE { ($chn:expr, $voice:expr, $p1:expr, $p2:expr) => { _GUS_CMD!($chn, $voice, _GUS_RAMPRATE, $p1, $p2) }; }
#[macro_export] macro_rules! GUS_RAMPMODE { ($chn:expr, $voice:expr, $p1:expr) => { _GUS_CMD!($chn, $voice, _GUS_RAMPMODE, $p1, 0) }; }
#[macro_export] macro_rules! GUS_RAMPON { ($chn:expr, $voice:expr, $p1:expr) => { _GUS_CMD!($chn, $voice, _GUS_RAMPON, $p1, 0) }; }
#[macro_export] macro_rules! GUS_RAMPOFF { ($chn:expr, $voice:expr) => { _GUS_CMD!($chn, $voice, _GUS_RAMPOFF, 0, 0) }; }
#[macro_export] macro_rules! GUS_VOLUME_SCALE { ($chn:expr, $voice:expr, $p1:expr, $p2:expr) => { _GUS_CMD!($chn, $voice, _GUS_VOLUME_SCALE, $p1, $p2) }; }
#[macro_export] macro_rules! GUS_VOICE_POS { ($chn:expr, $voice:expr, $p:expr) => { _GUS_CMD!($chn, $voice, _GUS_VOICE_POS, ($p) & 0xffff, (($p) >> 16) & 0xffff) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
