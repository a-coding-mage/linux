/*
 * Copyright (c) 2006 Oracle.  All rights reserved.
 *
 * This software is available to you under a choice of one of two
 * licenses.  You may choose to be licensed under the terms of the GNU
 * General Public License (GPL) Version 2, available from the file
 * COPYING in the main directory of this source tree, or the
 * OpenIB.org BSD license below:
 *
 *     Redistribution and use in source and binary forms, with or
 *     without modification, are permitted provided that the following
 *     conditions are met:
 *
 *      - Redistributions of source code must retain the above
 *        copyright notice, this list of conditions and the following
 *        disclaimer.
 *
 *      - Redistributions in binary form must reproduce the above
 *        copyright notice, this list of conditions and the following
 *        disclaimer in the documentation and/or other materials
 *        provided with the distribution.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
 * BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
 * ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */

// Linux kernel headers and "ib.h" provide the types, constants, and symbols
// referenced below.

static mut rds_ib_sysctl_hdr: *mut ctl_table_header = core::ptr::null_mut();

#[no_mangle]
pub static mut rds_ib_sysctl_max_send_wr: ::core::ffi::c_ulong = RDS_IB_DEFAULT_SEND_WR;
#[no_mangle]
pub static mut rds_ib_sysctl_max_recv_wr: ::core::ffi::c_ulong = RDS_IB_DEFAULT_RECV_WR;
#[no_mangle]
pub static mut rds_ib_sysctl_max_recv_allocation: ::core::ffi::c_ulong =
    (128 * 1024 * 1024) / RDS_FRAG_SIZE;
static mut rds_ib_sysctl_max_wr_min: ::core::ffi::c_ulong = 1;
/* hardware will fail CQ creation long before this */
static mut rds_ib_sysctl_max_wr_max: ::core::ffi::c_ulong = u32::MAX as ::core::ffi::c_ulong;

#[no_mangle]
pub static mut rds_ib_sysctl_max_unsig_wrs: ::core::ffi::c_ulong = 16;
static mut rds_ib_sysctl_max_unsig_wr_min: ::core::ffi::c_ulong = 1;
static mut rds_ib_sysctl_max_unsig_wr_max: ::core::ffi::c_ulong = 64;

/*
 * This sysctl does nothing.
 *
 * Backwards compatibility with RDS 3.0 wire protocol
 * disables initial FC credit exchange.
 * If it's ever possible to drop 3.0 support,
 * setting this to 1 and moving init/refill of send/recv
 * rings from ib_cm_connect_complete() back into ib_setup_qp()
 * will cause credits to be added before protocol negotiation.
 */
#[no_mangle]
pub static mut rds_ib_sysctl_flow_control: ::core::ffi::c_uint = 0;

static mut rds_ib_sysctl_table: [ctl_table; 5] = [
    ctl_table {
        procname: b"max_send_wr\0".as_ptr() as *mut ::core::ffi::c_char,
        data: unsafe { &raw mut rds_ib_sysctl_max_send_wr as *mut _ as *mut ::core::ffi::c_void },
        maxlen: core::mem::size_of::<::core::ffi::c_ulong>(),
        mode: 0o644,
        proc_handler: Some(proc_doulongvec_minmax),
        extra1: unsafe { &raw mut rds_ib_sysctl_max_wr_min as *mut _ as *mut ::core::ffi::c_void },
        extra2: unsafe { &raw mut rds_ib_sysctl_max_wr_max as *mut _ as *mut ::core::ffi::c_void },
    },
    ctl_table {
        procname: b"max_recv_wr\0".as_ptr() as *mut ::core::ffi::c_char,
        data: unsafe { &raw mut rds_ib_sysctl_max_recv_wr as *mut _ as *mut ::core::ffi::c_void },
        maxlen: core::mem::size_of::<::core::ffi::c_ulong>(),
        mode: 0o644,
        proc_handler: Some(proc_doulongvec_minmax),
        extra1: unsafe { &raw mut rds_ib_sysctl_max_wr_min as *mut _ as *mut ::core::ffi::c_void },
        extra2: unsafe { &raw mut rds_ib_sysctl_max_wr_max as *mut _ as *mut ::core::ffi::c_void },
    },
    ctl_table {
        procname: b"max_unsignaled_wr\0".as_ptr() as *mut ::core::ffi::c_char,
        data: unsafe { &raw mut rds_ib_sysctl_max_unsig_wrs as *mut _ as *mut ::core::ffi::c_void },
        maxlen: core::mem::size_of::<::core::ffi::c_ulong>(),
        mode: 0o644,
        proc_handler: Some(proc_doulongvec_minmax),
        extra1: unsafe { &raw mut rds_ib_sysctl_max_unsig_wr_min as *mut _ as *mut ::core::ffi::c_void },
        extra2: unsafe { &raw mut rds_ib_sysctl_max_unsig_wr_max as *mut _ as *mut ::core::ffi::c_void },
    },
    ctl_table {
        procname: b"max_recv_allocation\0".as_ptr() as *mut ::core::ffi::c_char,
        data: unsafe { &raw mut rds_ib_sysctl_max_recv_allocation as *mut _ as *mut ::core::ffi::c_void },
        maxlen: core::mem::size_of::<::core::ffi::c_ulong>(),
        mode: 0o644,
        proc_handler: Some(proc_doulongvec_minmax),
        extra1: core::ptr::null_mut(),
        extra2: core::ptr::null_mut(),
    },
    ctl_table {
        procname: b"flow_control\0".as_ptr() as *mut ::core::ffi::c_char,
        data: unsafe { &raw mut rds_ib_sysctl_flow_control as *mut _ as *mut ::core::ffi::c_void },
        maxlen: core::mem::size_of::<::core::ffi::c_uint>(),
        mode: 0o644,
        proc_handler: Some(proc_dointvec),
        extra1: core::ptr::null_mut(),
        extra2: core::ptr::null_mut(),
    },
];

#[no_mangle]
pub unsafe extern "C" fn rds_ib_sysctl_exit() {
    if !rds_ib_sysctl_hdr.is_null() {
        unregister_net_sysctl_table(rds_ib_sysctl_hdr);
    }
}

#[no_mangle]
pub unsafe extern "C" fn rds_ib_sysctl_init() -> ::core::ffi::c_int {
    rds_ib_sysctl_hdr = register_net_sysctl(
        &raw mut init_net,
        b"net/rds/ib\0".as_ptr() as *const ::core::ffi::c_char,
        rds_ib_sysctl_table.as_mut_ptr(),
    );
    if rds_ib_sysctl_hdr.is_null() {
        return -(ENOMEM as ::core::ffi::c_int);
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
