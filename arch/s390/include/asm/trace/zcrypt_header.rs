/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Tracepoint definitions for the s390 zcrypt device driver
 *
 * Copyright IBM Corp. 2016,2025
 * Author(s): Harald Freudenberger <freude@de.ibm.com>
 *
 * Currently there are two tracepoint events defined here.
 * An s390_zcrypt_req request event occurs as soon as the request is
 * recognized by the zcrypt ioctl function. This event may act as some kind
 * of request-processing-starts-now indication.
 * As late as possible within the zcrypt ioctl function there occurs the
 * s390_zcrypt_rep event which may act as the point in time where the
 * request has been processed by the kernel and the result is about to be
 * transferred back to userspace.
 * The glue which binds together request and reply event is the ptr
 * parameter, which is the local buffer address where the request from
 * userspace has been stored by the ioctl function.
 *
 * The main purpose of this zcrypt tracepoint api is to get some data for
 * performance measurements together with information about on which card
 * and queue the request has been processed. It is not an ffdc interface as
 * there is already code in the zcrypt device driver to serve the s390
 * debug feature interface.
 */

// The C header guard and tracepoint include machinery have no direct Rust
// equivalent; the declarations below are the generated tracepoint interface.

pub const TP_ICARSAMODEXPO: u32 = 0x0001;
pub const TP_ICARSACRT: u32 = 0x0002;
pub const TB_ZSECSENDCPRB: u32 = 0x0003;
pub const TP_ZSENDEP11CPRB: u32 = 0x0004;
pub const TP_HWRNGCPRB: u32 = 0x0005;

#[inline]
pub const fn show_zcrypt_tp_type(type_: u32) -> Option<&'static str> {
    match type_ {
        TP_ICARSAMODEXPO => Some("ICARSAMODEXPO"),
        TP_ICARSACRT => Some("ICARSACRT"),
        TB_ZSECSENDCPRB => Some("ZSECSENDCPRB"),
        TP_ZSENDEP11CPRB => Some("ZSENDEP11CPRB"),
        TP_HWRNGCPRB => Some("HWRNGCPRB"),
        _ => None,
    }
}

/**
 * trace_s390_zcrypt_req - zcrypt request tracepoint function
 * @ptr:  Address of the local buffer where the request from userspace
 *       is stored. Can be used as a unique id to relate together
 *       request and reply.
 * @type: One of the TP_ defines above.
 *
 * Called when a request from userspace is recognised within the ioctl
 * function of the zcrypt device driver and may act as an entry
 * timestamp.
 */
#[repr(C)]
pub struct S390ZcryptReqEntry {
    pub ptr: *mut core::ffi::c_void,
    pub type_: u32,
}

/**
 * trace_s390_zcrypt_rep - zcrypt reply tracepoint function
 * @ptr:   Address of the local buffer where the request from userspace
 *        is stored. Can be used as a unique id to match together
 *        request and reply.
 * @fc:    Function code.
 * @rc:    The bare returncode as returned by the device driver ioctl
 *        function.
 * @card:  The adapter nr where this request was actually processed.
 * @dom:   Domain id of the device where this request was processed.
 * @psmid: Unique id identifying this request/reply.
 *
 * Called upon recognising the reply from the crypto adapter. This
 * message may act as the exit timestamp for the request but also
 * carries some info about on which adapter the request was processed
 * and the returncode from the device driver.
 */
#[repr(C)]
pub struct S390ZcryptRepEntry {
    pub ptr: *mut core::ffi::c_void,
    pub fc: u32,
    pub rc: u32,
    pub card: u16,
    pub dom: u16,
    pub psmid: u64,
}

// TRACE_EVENT declarations: the kernel supplies the tracepoint implementations.
unsafe extern "C" {
    pub fn trace_s390_zcrypt_req(ptr: *mut core::ffi::c_void, type_: u32);
    pub fn trace_s390_zcrypt_rep(
        ptr: *mut core::ffi::c_void,
        fc: u32,
        rc: u32,
        card: u16,
        dom: u16,
        psmid: u64,
    );
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
