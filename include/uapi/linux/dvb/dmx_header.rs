/* SPDX-License-Identifier: LGPL-2.1+ WITH Linux-syscall-note */
/* Translated from dmx.h. */

pub const DMX_FILTER_SIZE: usize = 16;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dmx_output {
    DMX_OUT_DECODER,
    DMX_OUT_TAP,
    DMX_OUT_TS_TAP,
    DMX_OUT_TSDEMUX_TAP,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dmx_input {
    DMX_IN_FRONTEND,
    DMX_IN_DVR,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dmx_ts_pes {
    DMX_PES_AUDIO0,
    DMX_PES_VIDEO0,
    DMX_PES_TELETEXT0,
    DMX_PES_SUBTITLE0,
    DMX_PES_PCR0,
    DMX_PES_AUDIO1,
    DMX_PES_VIDEO1,
    DMX_PES_TELETEXT1,
    DMX_PES_SUBTITLE1,
    DMX_PES_PCR1,
    DMX_PES_AUDIO2,
    DMX_PES_VIDEO2,
    DMX_PES_TELETEXT2,
    DMX_PES_SUBTITLE2,
    DMX_PES_PCR2,
    DMX_PES_AUDIO3,
    DMX_PES_VIDEO3,
    DMX_PES_TELETEXT3,
    DMX_PES_SUBTITLE3,
    DMX_PES_PCR3,
    DMX_PES_OTHER,
}

pub const DMX_PES_AUDIO: dmx_ts_pes = dmx_ts_pes::DMX_PES_AUDIO0;
pub const DMX_PES_VIDEO: dmx_ts_pes = dmx_ts_pes::DMX_PES_VIDEO0;
pub const DMX_PES_TELETEXT: dmx_ts_pes = dmx_ts_pes::DMX_PES_TELETEXT0;
pub const DMX_PES_SUBTITLE: dmx_ts_pes = dmx_ts_pes::DMX_PES_SUBTITLE0;
pub const DMX_PES_PCR: dmx_ts_pes = dmx_ts_pes::DMX_PES_PCR0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmx_filter {
    pub filter: [u8; DMX_FILTER_SIZE],
    pub mask: [u8; DMX_FILTER_SIZE],
    pub mode: [u8; DMX_FILTER_SIZE],
}

pub const DMX_CHECK_CRC: u32 = 1;
pub const DMX_ONESHOT: u32 = 2;
pub const DMX_IMMEDIATE_START: u32 = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmx_sct_filter_params {
    pub pid: u16,
    pub filter: dmx_filter,
    pub timeout: u32,
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmx_pes_filter_params {
    pub pid: u16,
    pub input: dmx_input,
    pub output: dmx_output,
    pub pes_type: dmx_ts_pes,
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmx_stc {
    pub num: u32,
    pub base: u32,
    pub stc: u64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dmx_buffer_flags {
    DMX_BUFFER_FLAG_HAD_CRC32_DISCARD = 1 << 0,
    DMX_BUFFER_FLAG_TEI = 1 << 1,
    DMX_BUFFER_PKT_COUNTER_MISMATCH = 1 << 2,
    DMX_BUFFER_FLAG_DISCONTINUITY_DETECTED = 1 << 3,
    DMX_BUFFER_FLAG_DISCONTINUITY_INDICATOR = 1 << 4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmx_buffer {
    pub index: u32,
    pub bytesused: u32,
    pub offset: u32,
    pub length: u32,
    pub flags: u32,
    pub count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmx_requestbuffers {
    pub count: u32,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmx_exportbuffer {
    pub index: u32,
    pub flags: u32,
    pub fd: i32,
}

/* ioctl values depend on the external Linux _IO/_IOW/_IOR/_IOWR definitions. */
#[macro_export]
macro_rules! dmx_ioctl_constants {
    ($io:ident, $iow:ident, $ior:ident, $iowr:ident) => {
        pub const DMX_START: _ = $io(b'o', 41);
        pub const DMX_STOP: _ = $io(b'o', 42);
        pub const DMX_SET_FILTER: _ = $iow(b'o', 43, dmx_sct_filter_params);
        pub const DMX_SET_PES_FILTER: _ = $iow(b'o', 44, dmx_pes_filter_params);
        pub const DMX_SET_BUFFER_SIZE: _ = $io(b'o', 45);
        pub const DMX_GET_PES_PIDS: _ = $ior(b'o', 47, [u16; 5]);
        pub const DMX_GET_STC: _ = $iowr(b'o', 50, dmx_stc);
        pub const DMX_ADD_PID: _ = $iow(b'o', 51, u16);
        pub const DMX_REMOVE_PID: _ = $iow(b'o', 52, u16);
        pub const DMX_REQBUFS: _ = $iowr(b'o', 60, dmx_requestbuffers);
        pub const DMX_QUERYBUF: _ = $iowr(b'o', 61, dmx_buffer);
        pub const DMX_EXPBUF: _ = $iowr(b'o', 62, dmx_exportbuffer);
        pub const DMX_QBUF: _ = $iowr(b'o', 63, dmx_buffer);
        pub const DMX_DQBUF: _ = $iowr(b'o', 64, dmx_buffer);
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
