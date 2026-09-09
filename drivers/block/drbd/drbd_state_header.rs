/* SPDX-License-Identifier: GPL-2.0-only */

// Forward declarations supplied by other translation units.
pub struct drbd_device;
pub struct drbd_connection;
pub struct completion;

/* DRBD State macros.  The state unions and field masks are supplied by the
 * corresponding state declarations. */
pub const role_MASK: u32 = R_MASK;
pub const peer_MASK: u32 = R_MASK;
pub const disk_MASK: u32 = D_MASK;
pub const pdsk_MASK: u32 = D_MASK;
pub const conn_MASK: u32 = C_MASK;
pub const susp_MASK: u32 = 1;
pub const user_isp_MASK: u32 = 1;
pub const aftr_isp_MASK: u32 = 1;
pub const susp_nod_MASK: u32 = 1;
pub const susp_fen_MASK: u32 = 1;

// NS/NS2/NS3 and _NS/_NS2/_NS3 are C expression-style macros.  Their
// expansion is retained here as Rust macros, using the union's field setters.
#[macro_export]
macro_rules! NS {
    ($t:ident, $s:expr) => {{
        let mut mask = drbd_state { i: 0 };
        let mut val = drbd_state { i: 0 };
        mask.set_$t($t##_MASK);
        val.set_$t($s);
        (mask, val)
    }};
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union drbd_state {
    pub i: u32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum chg_state_flags {
    CS_HARD = 1 << 0,
    CS_VERBOSE = 1 << 1,
    CS_WAIT_COMPLETE = 1 << 2,
    CS_SERIALIZE = 1 << 3,
    CS_ORDERED = (1 << 2) + (1 << 3),
    CS_LOCAL_ONLY = 1 << 4,
    CS_DC_ROLE = 1 << 5,
    CS_DC_PEER = 1 << 6,
    CS_DC_CONN = 1 << 7,
    CS_DC_DISK = 1 << 8,
    CS_DC_PDSK = 1 << 9,
    CS_DC_SUSP = 1 << 10,
    CS_DC_MASK = (1 << 5) + (1 << 6) + (1 << 7) + (1 << 8) + (1 << 9),
    CS_IGN_OUTD_FAIL = 1 << 11,
    /* Make sure no meta data IO is in flight; used for graceful detach. */
    CS_INHIBIT_MD_IO = 1 << 12,
}

/* drbd_dev_state and drbd_state are different types.  The C bitfields are
 * represented by the packed integer; endian-specific field ordering is the
 * target ABI's responsibility. */
#[repr(C)]
#[derive(Copy, Clone)]
pub union drbd_dev_state {
    pub i: u32,
}

extern "C" {
    pub fn drbd_change_state(device: *mut drbd_device, f: chg_state_flags,
                             mask: drbd_state, val: drbd_state) -> drbd_state_rv;
    pub fn drbd_force_state(device: *mut drbd_device, mask: drbd_state, val: drbd_state);
    pub fn _drbd_request_state(device: *mut drbd_device, mask: drbd_state,
                               val: drbd_state, flags: chg_state_flags) -> drbd_state_rv;
    pub fn _drbd_request_state_holding_state_mutex(device: *mut drbd_device,
        mask: drbd_state, val: drbd_state, flags: chg_state_flags) -> drbd_state_rv;
    pub fn _drbd_set_state(device: *mut drbd_device, val: drbd_state,
                           flags: chg_state_flags, done: *mut completion) -> drbd_state_rv;
    pub fn print_st_err(device: *mut drbd_device, mask: drbd_state,
                        val: drbd_state, rv: drbd_state_rv);
    pub fn _conn_request_state(connection: *mut drbd_connection, mask: drbd_state,
                               val: drbd_state, flags: chg_state_flags) -> drbd_state_rv;
    pub fn conn_request_state(connection: *mut drbd_connection, mask: drbd_state,
                              val: drbd_state, flags: chg_state_flags) -> drbd_state_rv;
    pub fn drbd_resume_al(device: *mut drbd_device);
    pub fn conn_all_vols_unconf(connection: *mut drbd_connection) -> bool;
}

pub unsafe fn drbd_request_state(device: *mut drbd_device, mask: drbd_state,
                                 val: drbd_state) -> i32 {
    _drbd_request_state(device, mask, val,
        chg_state_flags::CS_VERBOSE /* + CS_ORDERED */) as i32
}

extern "C" {
    pub fn drbd_request_detach_interruptible(device: *mut drbd_device) -> i32;
    pub fn conn_highest_role(connection: *mut drbd_connection) -> drbd_role;
    pub fn conn_highest_peer(connection: *mut drbd_connection) -> drbd_role;
    pub fn conn_highest_disk(connection: *mut drbd_connection) -> drbd_disk_state;
    pub fn conn_lowest_disk(connection: *mut drbd_connection) -> drbd_disk_state;
    pub fn conn_highest_pdsk(connection: *mut drbd_connection) -> drbd_disk_state;
    pub fn conn_lowest_conn(connection: *mut drbd_connection) -> drbd_conns;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
