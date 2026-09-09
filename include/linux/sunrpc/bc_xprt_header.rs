/* SPDX-License-Identifier: GPL-2.0-only */
/******************************************************************************

(c) 2008 NetApp.  All Rights Reserved.


******************************************************************************/

/*
 * Functions to create and manage the backchannel
 */

/* Dependencies supplied by the surrounding kernel translation unit. */

#[cfg(feature = "CONFIG_SUNRPC_BACKCHANNEL")]
unsafe extern "C" {
    pub fn xprt_lookup_bc_request(
        xprt: *mut rpc_xprt,
        xid: __be32,
    ) -> *mut rpc_rqst;
    pub fn xprt_complete_bc_request(req: *mut rpc_rqst, copied: u32);
    pub fn xprt_init_bc_request(
        req: *mut rpc_rqst,
        task: *mut rpc_task,
        to: *const rpc_timeout,
    );
    pub fn xprt_free_bc_request(req: *mut rpc_rqst);
    pub fn xprt_setup_backchannel(xprt: *mut rpc_xprt, min_reqs: c_uint) -> c_int;
    pub fn xprt_destroy_backchannel(xprt: *mut rpc_xprt, max_reqs: c_uint);
    pub fn xprt_enqueue_bc_request(req: *mut rpc_rqst);

    /* Socket backchannel transport methods */
    pub fn xprt_setup_bc(xprt: *mut rpc_xprt, min_reqs: c_uint) -> c_int;
    pub fn xprt_destroy_bc(xprt: *mut rpc_xprt, max_reqs: c_uint);
    pub fn xprt_free_bc_rqst(req: *mut rpc_rqst);
    pub fn xprt_bc_max_slots(xprt: *mut rpc_xprt) -> c_uint;
    pub fn xprt_svc_shutdown_bc(xprt: *mut rpc_xprt);
    pub fn xprt_svc_destroy_nullify_bc(xprt: *mut rpc_xprt, serv: *mut *mut svc_serv);
}

#[cfg(feature = "CONFIG_SUNRPC_BACKCHANNEL")]
#[inline]
pub unsafe fn svc_is_backchannel(rqstp: *const svc_rqst) -> bool {
    (*(*rqstp).rq_server).sv_bc_enabled
}

#[cfg(feature = "CONFIG_SUNRPC_BACKCHANNEL")]
#[inline]
pub unsafe fn set_bc_enabled(serv: *mut svc_serv) {
    (*serv).sv_bc_enabled = true;
}

#[cfg(not(feature = "CONFIG_SUNRPC_BACKCHANNEL"))]
#[inline]
pub unsafe fn xprt_setup_backchannel(_xprt: *mut rpc_xprt, _min_reqs: c_uint) -> c_int {
    0
}

#[cfg(not(feature = "CONFIG_SUNRPC_BACKCHANNEL"))]
#[inline]
pub unsafe fn xprt_destroy_backchannel(_xprt: *mut rpc_xprt, _max_reqs: c_uint) {}

#[cfg(not(feature = "CONFIG_SUNRPC_BACKCHANNEL"))]
#[inline]
pub unsafe fn svc_is_backchannel(_rqstp: *const svc_rqst) -> bool {
    false
}

#[cfg(not(feature = "CONFIG_SUNRPC_BACKCHANNEL"))]
#[inline]
pub unsafe fn set_bc_enabled(_serv: *mut svc_serv) {}

#[cfg(not(feature = "CONFIG_SUNRPC_BACKCHANNEL"))]
#[inline]
pub unsafe fn xprt_free_bc_request(_req: *mut rpc_rqst) {}

#[cfg(not(feature = "CONFIG_SUNRPC_BACKCHANNEL"))]
#[inline]
pub unsafe fn xprt_svc_shutdown_bc(_xprt: *mut rpc_xprt) {}

#[cfg(not(feature = "CONFIG_SUNRPC_BACKCHANNEL"))]
#[inline]
pub unsafe fn xprt_svc_destroy_nullify_bc(
    _xprt: *mut rpc_xprt,
    serv: *mut *mut svc_serv,
) {
    svc_destroy(serv);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
