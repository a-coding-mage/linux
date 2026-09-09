/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

pub const GENERIC_HDLC_VERSION: u32 = 4; /* For synchronization with sethdlc utility */

pub const CLOCK_DEFAULT: u32 = 0; /* Default setting */
pub const CLOCK_EXT: u32 = 1; /* External TX and RX clock - DTE */
pub const CLOCK_INT: u32 = 2; /* Internal TX and RX clock - DCE */
pub const CLOCK_TXINT: u32 = 3; /* Internal TX and external RX clock */
pub const CLOCK_TXFROMRX: u32 = 4; /* TX clock derived from external RX clock */

pub const ENCODING_DEFAULT: u32 = 0; /* Default setting */
pub const ENCODING_NRZ: u32 = 1;
pub const ENCODING_NRZI: u32 = 2;
pub const ENCODING_FM_MARK: u32 = 3;
pub const ENCODING_FM_SPACE: u32 = 4;
pub const ENCODING_MANCHESTER: u32 = 5;

pub const PARITY_DEFAULT: u32 = 0; /* Default setting */
pub const PARITY_NONE: u32 = 1; /* No parity */
pub const PARITY_CRC16_PR0: u32 = 2; /* CRC16, initial value 0x0000 */
pub const PARITY_CRC16_PR1: u32 = 3; /* CRC16, initial value 0xFFFF */
pub const PARITY_CRC16_PR0_CCITT: u32 = 4; /* CRC16, initial 0x0000, ITU-T version */
pub const PARITY_CRC16_PR1_CCITT: u32 = 5; /* CRC16, initial 0xFFFF, ITU-T version */
pub const PARITY_CRC32_PR0_CCITT: u32 = 6; /* CRC32, initial value 0x00000000 */
pub const PARITY_CRC32_PR1_CCITT: u32 = 7; /* CRC32, initial value 0xFFFFFFFF */

pub const LMI_DEFAULT: u32 = 0; /* Default setting */
pub const LMI_NONE: u32 = 1; /* No LMI, all PVCs are static */
pub const LMI_ANSI: u32 = 2; /* ANSI Annex D */
pub const LMI_CCITT: u32 = 3; /* ITU-T Annex A */
pub const LMI_CISCO: u32 = 4; /* The "original" LMI, aka Gang of Four */

#[repr(C)]
pub struct sync_serial_settings {
    pub clock_rate: u32, /* bits per second */
    pub clock_type: u32, /* internal, external, TX-internal etc. */
    pub loopback: u16,
}

#[repr(C)]
pub struct te1_settings {
    pub clock_rate: u32, /* bits per second */
    pub clock_type: u32, /* internal, external, TX-internal etc. */
    pub loopback: u16,
    pub slot_map: u32,
}

#[repr(C)]
pub struct raw_hdlc_proto {
    pub encoding: u16,
    pub parity: u16,
}

#[repr(C)]
pub struct fr_proto {
    pub t391: u32,
    pub t392: u32,
    pub n391: u32,
    pub n392: u32,
    pub n393: u32,
    pub lmi: u16,
    pub dce: u16, /* 1 for DCE (network side) operation */
}

#[repr(C)]
pub struct fr_proto_pvc {
    pub dlci: u32,
}

#[repr(C)]
pub struct fr_proto_pvc_info {
    pub dlci: u32,
    pub master: [core::ffi::c_char; IFNAMSIZ], /* Name of master FRAD device */
}

#[repr(C)]
pub struct cisco_proto {
    pub interval: u32,
    pub timeout: u32,
}

#[repr(C)]
pub struct x25_hdlc_proto {
    pub dce: u16, /* 1 for DCE (network side) operation */
    pub modulo: u32, /* modulo (8 = basic / 128 = extended) */
    pub window: u32, /* frame window size */
    pub t1: u32, /* timeout t1 */
    pub t2: u32, /* timeout t2 */
    pub n2: u32, /* frame retry counter */
}

/* PPP doesn't need any info now - supply length = 0 to ioctl */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
