//! Rust translation of the Linux neighbour tracepoint header.
//!
//! The C tracepoint framework supplies the surrounding registration and print
//! machinery.  The declarations below preserve the event layouts, arguments,
//! symbolic state names, and assignment semantics without implementing that
//! external framework.

pub const TRACE_SYSTEM: &str = "neigh";

/// Equivalent of `neigh_state_str()` / `__print_symbolic()`.
pub fn neigh_state_str(state: u8) -> &'static str {
    // NUD_* values are supplied by the neighbour subsystem.
    match state {
        0x01 => "incomplete",
        0x02 => "reachable",
        0x04 => "stale",
        0x08 => "delay",
        0x10 => "probe",
        0x20 => "failed",
        0x40 => "noarp",
        0x80 => "permanent",
        _ => "unknown",
    }
}

pub const MAX_ADDR_LEN: usize = 32;

#[repr(C)]
pub struct NeighCreateEntry {
    pub family: u32,
    pub dev: [u8; 16],
    pub entries: i32,
    pub created: u8,
    pub gc_exempt: u8,
    pub primary_key4: [u8; 4],
    pub primary_key6: [u8; 16],
}

#[repr(C)]
pub struct NeighUpdateEntry {
    pub family: u32,
    pub dev: [u8; 16],
    pub lladdr: [u8; MAX_ADDR_LEN],
    pub lladdr_len: u8,
    pub flags: u8,
    pub nud_state: u8,
    pub type_: u8,
    pub dead: u8,
    pub refcnt: i32,
    pub primary_key4: [u8; 4],
    pub primary_key6: [u8; 16],
    pub confirmed: usize,
    pub updated: usize,
    pub used: usize,
    pub new_lladdr: [u8; MAX_ADDR_LEN],
    pub new_state: u8,
    pub update_flags: u32,
    pub pid: u32,
}

#[repr(C)]
pub struct NeighUpdateDoneEntry {
    pub family: u32,
    pub dev: [u8; 16],
    pub lladdr: [u8; MAX_ADDR_LEN],
    pub lladdr_len: u8,
    pub flags: u8,
    pub nud_state: u8,
    pub type_: u8,
    pub dead: u8,
    pub refcnt: i32,
    pub primary_key4: [u8; 4],
    pub primary_key6: [u8; 16],
    pub confirmed: usize,
    pub updated: usize,
    pub used: usize,
    pub err: u32,
}

/// Tracepoint argument signatures retained from `TP_PROTO`.
pub type NeighCreateArgs = (*mut core::ffi::c_void, *mut core::ffi::c_void,
    *const core::ffi::c_void, *const core::ffi::c_void, bool);
pub type NeighUpdateArgs = (*mut core::ffi::c_void, *const u8, u8, u32, u32);
pub type NeighUpdateErrorArgs = (*mut core::ffi::c_void, i32);

/// Event names emitted by the original `TRACE_EVENT`/`DEFINE_EVENT` macros.
pub const NEIGH_CREATE: &str = "neigh_create";
pub const NEIGH_UPDATE: &str = "neigh_update";
pub const NEIGH_UPDATE_DONE: &str = "neigh_update_done";
pub const NEIGH_TIMER_HANDLER: &str = "neigh_timer_handler";
pub const NEIGH_EVENT_SEND_DONE: &str = "neigh_event_send_done";
pub const NEIGH_EVENT_SEND_DEAD: &str = "neigh_event_send_dead";
pub const NEIGH_CLEANUP_AND_RELEASE: &str = "neigh_cleanup_and_release";

// C TP_printk format strings, retained as declarations for the external
// tracepoint printer:
pub const NEIGH_CREATE_FORMAT: &str = "family %d dev %s entries %d primary_key4 %pI4 primary_key6 %pI6c created %d gc_exempt %d";
pub const NEIGH_UPDATE_FORMAT: &str = "family %d dev %s lladdr %s flags %02x nud_state %s type %02x dead %d refcnt %d primary_key4 %pI4 primary_key6 %pI6c confirmed %lu updated %lu used %lu new_lladdr %s new_state %s update_flags %02x pid %d";
pub const NEIGH_UPDATE_ERROR_FORMAT: &str = "family %d dev %s lladdr %s flags %02x nud_state %s type %02x dead %d refcnt %d primary_key4 %pI4 primary_key6 %pI6c confirmed %lu updated %lu used %lu err %d";

// The IPv4/IPv6 key copies, neighbour-field reads, conditional lladdr copy,
// v4-mapped IPv6 fallback, and trace registration are performed by the Linux
// tracepoint macros and the external neighbour/networking definitions.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
