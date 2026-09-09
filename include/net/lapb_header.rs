/* SPDX-License-Identifier: GPL-2.0 */
// Translated from lapb.h. C header guards and includes are omitted; the
// referenced kernel/project types are supplied by the surrounding crate.

pub const LAPB_HEADER_LEN: usize = MAX_HEADER;

pub const LAPB_ACK_PENDING_CONDITION: u8 = 0x01;
pub const LAPB_REJECT_CONDITION: u8 = 0x02;
pub const LAPB_PEER_RX_BUSY_CONDITION: u8 = 0x04;

/* Control field templates */
pub const LAPB_I: u8 = 0x00; // Information frames
pub const LAPB_S: u8 = 0x01; // Supervisory frames
pub const LAPB_U: u8 = 0x03; // Unnumbered frames

pub const LAPB_RR: u8 = 0x01; // Receiver ready
pub const LAPB_RNR: u8 = 0x05; // Receiver not ready
pub const LAPB_REJ: u8 = 0x09; // Reject

pub const LAPB_SABM: u8 = 0x2F; // Set Asynchronous Balanced Mode
pub const LAPB_SABME: u8 = 0x6F; // Set Asynchronous Balanced Mode Extended
pub const LAPB_DISC: u8 = 0x43; // Disconnect
pub const LAPB_DM: u8 = 0x0F; // Disconnected mode
pub const LAPB_UA: u8 = 0x63; // Unnumbered acknowledge
pub const LAPB_FRMR: u8 = 0x87; // Frame reject

pub const LAPB_ILLEGAL: u16 = 0x100; // Impossible to be a real frame type

pub const LAPB_SPF: u8 = 0x10; // Poll/final bit for standard LAPB
pub const LAPB_EPF: u8 = 0x01; // Poll/final bit for extended LAPB

pub const LAPB_FRMR_W: u8 = 0x01; // Control field invalid
pub const LAPB_FRMR_X: u8 = 0x02; // I field invalid
pub const LAPB_FRMR_Y: u8 = 0x04; // I field too long
pub const LAPB_FRMR_Z: u8 = 0x08; // Invalid N(R)

pub const LAPB_POLLOFF: i32 = 0;
pub const LAPB_POLLON: i32 = 1;

/* LAPB C-bit */
pub const LAPB_COMMAND: i32 = 1;
pub const LAPB_RESPONSE: i32 = 2;

pub const LAPB_ADDR_A: u8 = 0x03;
pub const LAPB_ADDR_B: u8 = 0x01;
pub const LAPB_ADDR_C: u8 = 0x0F;
pub const LAPB_ADDR_D: u8 = 0x07;

/* Define Link State constants. */
#[repr(i32)]
pub enum LapbState {
    LAPB_STATE_0, // Disconnected State
    LAPB_STATE_1, // Awaiting Connection State
    LAPB_STATE_2, // Awaiting Disconnection State
    LAPB_STATE_3, // Data Transfer State
    LAPB_STATE_4, // Frame Reject State
}

pub const LAPB_DEFAULT_MODE: u32 = LAPB_STANDARD | LAPB_SLP | LAPB_DTE;
pub const LAPB_DEFAULT_WINDOW: u8 = 7; // Window=7
pub const LAPB_DEFAULT_T1: u32 = 5 * HZ; // T1=5s
pub const LAPB_DEFAULT_T2: u32 = 1 * HZ; // T2=1s
pub const LAPB_DEFAULT_N2: u16 = 20; // N2=20

pub const LAPB_SMODULUS: u8 = 8;
pub const LAPB_EMODULUS: u8 = 128;

/* Information about the current frame. */
#[repr(C)]
pub struct lapb_frame {
    pub type_: u16, // Parsed type
    pub nr: u16,
    pub ns: u16, // N(R), N(S)
    pub cr: u8, // Command/Response
    pub pf: u8, // Poll/Final
    pub control: [u8; 2], // Original control data
}

/* The per LAPB connection control structure. */
#[repr(C)]
pub struct lapb_cb {
    pub node: list_head,
    pub dev: *mut net_device,
    pub mode: u32,
    pub state: u8,
    pub vs: u16,
    pub vr: u16,
    pub va: u16,
    pub condition: u8,
    pub n2: u16,
    pub n2count: u16,
    pub t1: u16,
    pub t2: u16,
    pub t1timer: timer_list,
    pub t2timer: timer_list,
    pub t1timer_running: bool,
    pub t2timer_running: bool,
    pub write_queue: sk_buff_head,
    pub ack_queue: sk_buff_head,
    pub window: u8,
    pub callbacks: *const lapb_register_struct,
    pub frmr_data: lapb_frame,
    pub frmr_type: u8,
    pub lock: spinlock_t,
    pub refcnt: refcount_t,
}

extern "C" {
    pub fn lapb_connect_confirmation(lapb: *mut lapb_cb, result: i32);
    pub fn lapb_connect_indication(lapb: *mut lapb_cb, result: i32);
    pub fn lapb_disconnect_confirmation(lapb: *mut lapb_cb, result: i32);
    pub fn lapb_disconnect_indication(lapb: *mut lapb_cb, result: i32);
    pub fn lapb_data_indication(lapb: *mut lapb_cb, skb: *mut sk_buff) -> i32;
    pub fn lapb_data_transmit(lapb: *mut lapb_cb, skb: *mut sk_buff) -> i32;
    pub fn lapb_data_input(lapb: *mut lapb_cb, skb: *mut sk_buff);
    pub fn lapb_kick(lapb: *mut lapb_cb);
    pub fn lapb_transmit_buffer(lapb: *mut lapb_cb, skb: *mut sk_buff, reason: i32);
    pub fn lapb_establish_data_link(lapb: *mut lapb_cb);
    pub fn lapb_enquiry_response(lapb: *mut lapb_cb);
    pub fn lapb_timeout_response(lapb: *mut lapb_cb);
    pub fn lapb_check_iframes_acked(lapb: *mut lapb_cb, nr: u16);
    pub fn lapb_check_need_response(lapb: *mut lapb_cb, p: i32, f: i32);
    pub fn lapb_clear_queues(lapb: *mut lapb_cb);
    pub fn lapb_frames_acked(lapb: *mut lapb_cb, nr: u16);
    pub fn lapb_requeue_frames(lapb: *mut lapb_cb);
    pub fn lapb_validate_nr(lapb: *mut lapb_cb, nr: u16) -> i32;
    pub fn lapb_decode(lapb: *mut lapb_cb, skb: *mut sk_buff, frame: *mut lapb_frame) -> i32;
    pub fn lapb_send_control(lapb: *mut lapb_cb, frame_type: i32, pf: i32, cr: i32);
    pub fn lapb_transmit_frmr(lapb: *mut lapb_cb);
    pub fn lapb_start_t1timer(lapb: *mut lapb_cb);
    pub fn lapb_start_t2timer(lapb: *mut lapb_cb);
    pub fn lapb_stop_t1timer(lapb: *mut lapb_cb);
    pub fn lapb_stop_t2timer(lapb: *mut lapb_cb);
    pub fn lapb_t1timer_running(lapb: *mut lapb_cb) -> i32;
}

pub const LAPB_DEBUG: i32 = 0;

#[macro_export]
macro_rules! lapb_dbg {
    ($level:expr, $fmt:expr $(, $arg:expr)*) => {
        if $level < $crate::LAPB_DEBUG {
            $crate::pr_debug!($fmt $(, $arg)*);
        }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
