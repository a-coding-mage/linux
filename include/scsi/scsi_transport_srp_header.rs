/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than redefined: `device`, `mutex`, `delayed_work`,
// `scsi_transport_template`, `Scsi_Host`, `scsi_cmnd`, and `scsi_timeout_action`.

pub const SRP_RPORT_ROLE_INITIATOR: u32 = 0;
pub const SRP_RPORT_ROLE_TARGET: u32 = 1;

#[repr(C)]
pub struct srp_rport_identifiers {
    pub port_id: [u8; 16],
    pub roles: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum srp_rport_state {
    SRP_RPORT_RUNNING,
    SRP_RPORT_BLOCKED,
    SRP_RPORT_FAIL_FAST,
    SRP_RPORT_LOST,
}

#[repr(C)]
pub struct srp_rport {
    /* for initiator and target drivers */
    pub dev: device,
    pub port_id: [u8; 16],
    pub roles: u8,

    /* for initiator drivers */
    pub lld_data: *mut core::ffi::c_void,
    pub mutex: mutex,
    pub state: srp_rport_state,
    pub reconnect_delay: core::ffi::c_int,
    pub failed_reconnects: core::ffi::c_int,
    pub reconnect_work: delayed_work,
    pub fast_io_fail_tmo: core::ffi::c_int,
    pub dev_loss_tmo: core::ffi::c_int,
    pub fast_io_fail_work: delayed_work,
    pub dev_loss_work: delayed_work,
}

#[repr(C)]
pub struct srp_function_template {
    /* for initiator drivers */
    pub has_rport_state: bool,
    pub reset_timer_if_blocked: bool,
    pub reconnect_delay: *mut core::ffi::c_int,
    pub fast_io_fail_tmo: *mut core::ffi::c_int,
    pub dev_loss_tmo: *mut core::ffi::c_int,
    pub reconnect: Option<unsafe extern "C" fn(*mut srp_rport) -> core::ffi::c_int>,
    pub terminate_rport_io: Option<unsafe extern "C" fn(*mut srp_rport)>,
    pub rport_delete: Option<unsafe extern "C" fn(*mut srp_rport)>,
}

extern "C" {
    pub fn srp_attach_transport(
        template: *mut srp_function_template,
    ) -> *mut scsi_transport_template;
    pub fn srp_release_transport(template: *mut scsi_transport_template);
    pub fn srp_rport_get(rport: *mut srp_rport);
    pub fn srp_rport_put(rport: *mut srp_rport);
    pub fn srp_rport_add(
        host: *mut Scsi_Host,
        ids: *mut srp_rport_identifiers,
    ) -> *mut srp_rport;
    pub fn srp_rport_del(rport: *mut srp_rport);
    pub fn srp_tmo_valid(
        reconnect_delay: core::ffi::c_int,
        fast_io_fail_tmo: core::ffi::c_int,
        dev_loss_tmo: core::ffi::c_long,
    ) -> core::ffi::c_int;
    pub fn srp_parse_tmo(tmo: *mut core::ffi::c_int, buf: *const core::ffi::c_char)
        -> core::ffi::c_int;
    pub fn srp_reconnect_rport(rport: *mut srp_rport) -> core::ffi::c_int;
    pub fn srp_start_tl_fail_timers(rport: *mut srp_rport);
    pub fn srp_remove_host(host: *mut Scsi_Host);
    pub fn srp_stop_rport_timers(rport: *mut srp_rport);
    pub fn srp_timed_out(scmd: *mut scsi_cmnd) -> scsi_timeout_action;
}

/**
 * srp_chkready() - evaluate the transport layer state before I/O
 * @rport: SRP target port pointer.
 *
 * Returns: a SCSI result code that can be returned by the LLD queuecommand()
 * implementation. The role of this function is similar to that of
 * fc_remote_port_chkready().
 */
#[inline]
pub unsafe fn srp_chkready(rport: *mut srp_rport) -> core::ffi::c_int {
    match (*rport).state {
        srp_rport_state::SRP_RPORT_RUNNING | srp_rport_state::SRP_RPORT_BLOCKED => 0,
        srp_rport_state::SRP_RPORT_FAIL_FAST => DID_TRANSPORT_FAILFAST << 16,
        srp_rport_state::SRP_RPORT_LOST => DID_NO_CONNECT << 16,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
