/* Translated from net/tipc/trace.h. */

// The C tracepoint includes and tracepoint-generation macros are supplied by
// the surrounding kernel integration and are intentionally not reproduced.

pub const SKB_LMIN: usize = 100;
pub const SKB_LMAX: usize = SKB_LMIN * 2;
pub const LIST_LMIN: usize = SKB_LMIN * 3;
pub const LIST_LMAX: usize = SKB_LMIN * 11;
pub const SK_LMIN: usize = SKB_LMIN * 2;
pub const SK_LMAX: usize = SKB_LMIN * 11;
pub const LINK_LMIN: usize = SKB_LMIN;
pub const LINK_LMAX: usize = SKB_LMIN * 16;
pub const NODE_LMIN: usize = SKB_LMIN;
pub const NODE_LMAX: usize = SKB_LMIN * 11;

#[repr(u16)]
pub enum TipcDump {
    None = 0,
    Transmq = 1,
    Backlogq = 1 << 1,
    Deferdq = 1 << 2,
    Inputq = 1 << 3,
    Wakeup = 1 << 4,
    SkSndq = 1 << 8,
    SkRcvq = 1 << 9,
    SkBklgq = 1 << 10,
    All = 0xffff,
}

pub const TIPC_DUMP_NONE: u16 = 0;
pub const TIPC_DUMP_TRANSMQ: u16 = 1;
pub const TIPC_DUMP_BACKLOGQ: u16 = 1 << 1;
pub const TIPC_DUMP_DEFERDQ: u16 = 1 << 2;
pub const TIPC_DUMP_INPUTQ: u16 = 1 << 3;
pub const TIPC_DUMP_WAKEUP: u16 = 1 << 4;
pub const TIPC_DUMP_SK_SNDQ: u16 = 1 << 8;
pub const TIPC_DUMP_SK_RCVQ: u16 = 1 << 9;
pub const TIPC_DUMP_SK_BKLGQ: u16 = 1 << 10;
pub const TIPC_DUMP_ALL: u16 = 0xffff;

pub fn state_sym(val: u32) -> Option<&'static str> {
    match val {
        0xe => Some("ESTABLISHED"), 0xe0 => Some("ESTABLISHING"),
        0x100 => Some("RESET"), 0x2000 => Some("RESETTING"),
        0xd0000 => Some("PEER_RESET"), 0xf00000 => Some("FAILINGOVER"),
        0xc000000 => Some("SYNCHING"), 0xdd => Some("SELF_DOWN_PEER_DOWN"),
        0xaa => Some("SELF_UP_PEER_UP"), 0xd1 => Some("SELF_DOWN_PEER_LEAVING"),
        0xac => Some("SELF_UP_PEER_COMING"), 0xca => Some("SELF_COMING_PEER_UP"),
        0x1d => Some("SELF_LEAVING_PEER_DOWN"), 0xf0 => Some("FAILINGOVER"),
        0xcc => Some("SYNCHING"), _ => None,
    }
}

pub fn evt_sym(val: u32) -> Option<&'static str> {
    match val {
        0xec1ab1e => Some("ESTABLISH_EVT"), 0x9eed0e => Some("PEER_RESET_EVT"),
        0xfa110e => Some("FAILURE_EVT"), 0x10ca1d0e => Some("RESET_EVT"),
        0xfa110bee => Some("FAILOVER_BEGIN_EVT"), 0xfa110ede => Some("FAILOVER_END_EVT"),
        0xc1ccbee => Some("SYNCH_BEGIN_EVT"), 0xc1ccede => Some("SYNCH_END_EVT"),
        0xece => Some("SELF_ESTABL_CONTACT_EVT"), 0x1ce => Some("SELF_LOST_CONTACT_EVT"),
        0x9ece => Some("PEER_ESTABL_CONTACT_EVT"), 0x91ce => Some("PEER_LOST_CONTACT_EVT"),
        0xfbe => Some("FAILOVER_BEGIN_EVT"), 0xfee => Some("FAILOVER_END_EVT"),
        0xcbe => Some("SYNCH_BEGIN_EVT"), 0xcee => Some("SYNCH_END_EVT"), _ => None,
    }
}

#[repr(C)] pub struct SkBuff { _private: [u8; 0] }
#[repr(C)] pub struct SkBuffHead { _private: [u8; 0] }
#[repr(C)] pub struct Sock { _private: [u8; 0] }
#[repr(C)] pub struct TipcLink { _private: [u8; 0] }
#[repr(C)] pub struct TipcNode { _private: [u8; 0] }

extern "C" {
    pub static mut sysctl_tipc_sk_filter: [core::ffi::c_ulong; 5];
    pub fn tipc_skb_dump(skb: *mut SkBuff, more: bool, buf: *mut core::ffi::c_char) -> i32;
    pub fn tipc_list_dump(list: *mut SkBuffHead, more: bool, buf: *mut core::ffi::c_char) -> i32;
    pub fn tipc_sk_dump(sk: *mut Sock, dqueues: u16, buf: *mut core::ffi::c_char) -> i32;
    pub fn tipc_link_dump(l: *mut TipcLink, dqueues: u16, buf: *mut core::ffi::c_char) -> i32;
    pub fn tipc_node_dump(n: *mut TipcNode, more: bool, buf: *mut core::ffi::c_char) -> i32;
    pub fn tipc_sk_filtering(sk: *mut Sock) -> bool;
}

// Trace event classes and event instances from the C header. Their concrete
// registration and formatting are provided by the target tracepoint system.
pub const TIPC_TRACE_EVENTS: &[&str] = &[
    "tipc_skb_dump", "tipc_proto_build", "tipc_proto_rcv", "tipc_list_dump",
    "tipc_sk_dump", "tipc_sk_create", "tipc_sk_sendmcast", "tipc_sk_sendmsg",
    "tipc_sk_sendstream", "tipc_sk_poll", "tipc_sk_filter_rcv", "tipc_sk_advance_rx",
    "tipc_sk_rej_msg", "tipc_sk_drop_msg", "tipc_sk_release", "tipc_sk_shutdown",
    "tipc_sk_overlimit1", "tipc_sk_overlimit2", "tipc_link_dump", "tipc_link_conges",
    "tipc_link_timeout", "tipc_link_reset", "tipc_link_too_silent", "tipc_link_retrans",
    "tipc_link_bc_ack", "tipc_node_dump", "tipc_node_create", "tipc_node_delete",
    "tipc_node_lost_contact", "tipc_node_timeout", "tipc_node_link_up", "tipc_node_link_down",
    "tipc_node_reset_links", "tipc_node_check_state", "tipc_link_fsm", "tipc_node_fsm",
    "tipc_l2_device_event",
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
