/* SPDX-License-Identifier: GPL-2.0 */

/* Dependency types and aliases are supplied by the surrounding translation. */

pub const VS_DATA_PACKET_HEADER: u8 = 0xff;
pub const VS_CONTROL_PACKET_HEADER: u8 = 0xfe;
pub const VS_QUERY_PACKET_HEADER: u8 = 0xfd;
pub const VS_QUERY_RESPONSE_PACKET_HEADER: u8 = 0xfc;

/* control verbs */
pub const VSV_SET_MODEM_CTL: u16 = 1; /* to service processor only */
pub const VSV_MODEM_CTL_UPDATE: u16 = 2; /* from service processor only */
pub const VSV_CLOSE_PROTOCOL: u16 = 3;

/* query verbs */
pub const VSV_SEND_VERSION_NUMBER: u16 = 1;
pub const VSV_SEND_MODEM_CTL_STATUS: u16 = 2;

/* yes, these masks are not consecutive. */
pub const HVSI_TSDTR: u16 = 0x01;
pub const HVSI_TSCD: u16 = 0x20;

pub const HVSI_MAX_OUTGOING_DATA: usize = 12;
pub const HVSI_VERSION: u8 = 1;

#[repr(C, packed)]
pub struct hvsi_header {
    pub type_: u8,
    pub len: u8,
    pub seqno: u16,
}

#[repr(C, packed)]
pub struct hvsi_data {
    pub hdr: hvsi_header,
    pub data: [u8; HVSI_MAX_OUTGOING_DATA],
}

#[repr(C, packed)]
pub struct hvsi_control {
    pub hdr: hvsi_header,
    pub verb: u16,
    /* optional depending on verb: */
    pub word: u32,
    pub mask: u32,
}

#[repr(C, packed)]
pub struct hvsi_query {
    pub hdr: hvsi_header,
    pub verb: u16,
}

#[repr(C, packed)]
pub struct hvsi_query_response {
    pub hdr: hvsi_header,
    pub verb: u16,
    pub query_seqno: u16,
    pub u: hvsi_query_response_u,
}

#[repr(C)]
pub union hvsi_query_response_u {
    pub version: u8,
    pub mctrl_word: u32,
}

/* hvsi lib struct definitions */
pub const HVSI_INBUF_SIZE: usize = 255;

pub enum tty_struct {}

#[repr(C)]
pub struct hvsi_priv {
    pub inbuf_len: libc::c_uint, /* data in input buffer */
    pub inbuf: [u8; HVSI_INBUF_SIZE],
    pub inbuf_cur: libc::c_uint, /* Cursor in input buffer */
    pub inbuf_pktlen: usize, /* packet length from cursor */
    pub seqno: atomic_t, /* packet sequence number */
    /* C bit-fields: opened:1, established:1, is_console:1, mctrl_update:1. */
    pub opened: libc::c_uint, /* driver opened */
    pub established: libc::c_uint, /* protocol established */
    pub is_console: libc::c_uint, /* used as a kernel console device */
    pub mctrl_update: libc::c_uint, /* modem control updated */
    pub mctrl: u16, /* modem control */
    pub tty: *mut tty_struct, /* tty structure */
    pub get_chars: Option<unsafe extern "C" fn(termno: u32, buf: *mut u8, count: usize) -> isize>,
    pub put_chars: Option<unsafe extern "C" fn(termno: u32, buf: *const u8, count: usize) -> isize>,
    pub termno: u32,
}

/* hvsi lib functions */
pub enum hvc_struct {}

unsafe extern "C" {
    pub fn hvsilib_init(
        pv: *mut hvsi_priv,
        get_chars: Option<unsafe extern "C" fn(termno: u32, buf: *mut u8, count: usize) -> isize>,
        put_chars: Option<unsafe extern "C" fn(termno: u32, buf: *const u8, count: usize) -> isize>,
        termno: libc::c_int,
        is_console: libc::c_int,
    );
    pub fn hvsilib_open(pv: *mut hvsi_priv, hp: *mut hvc_struct) -> libc::c_int;
    pub fn hvsilib_close(pv: *mut hvsi_priv, hp: *mut hvc_struct);
    pub fn hvsilib_read_mctrl(pv: *mut hvsi_priv) -> libc::c_int;
    pub fn hvsilib_write_mctrl(pv: *mut hvsi_priv, dtr: libc::c_int) -> libc::c_int;
    pub fn hvsilib_establish(pv: *mut hvsi_priv);
    pub fn hvsilib_get_chars(pv: *mut hvsi_priv, buf: *mut u8, count: usize) -> isize;
    pub fn hvsilib_put_chars(pv: *mut hvsi_priv, buf: *const u8, count: usize) -> isize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
