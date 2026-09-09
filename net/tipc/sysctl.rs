/*
 * net/tipc/sysctl.c: sysctl interface to TIPC subsystem
 *
 * Copyright (c) 2013, Wind River Systems
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. Neither the names of the copyright holders nor the names of its
 *    contributors may be used to endorse or promote products derived from
 *    this software without specific prior written permission.
 *
 * Alternatively, this software may be distributed under the terms of the
 * GNU General Public License ("GPL") version 2 as published by the Free
 * Software Foundation.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 */

// Dependencies supplied by the surrounding kernel/TIPC translation.
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct ctl_table_header {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ctl_table {
    pub procname: *const c_char,
    pub data: *mut c_void,
    pub maxlen: usize,
    pub mode: u16,
    pub proc_handler: Option<unsafe extern "C" fn()>,
    pub extra1: *mut c_void,
    pub extra2: *mut c_void,
}

extern "C" {
    static mut init_net: c_void;
    static mut sysctl_tipc_rmem: c_int;
    static mut sysctl_tipc_named_timeout: c_int;
    static mut sysctl_tipc_sk_filter: c_ulong;
    static mut sysctl_tipc_bc_retruni: c_ulong;
    // Present when CONFIG_TIPC_CRYPTO is enabled.
    static mut sysctl_tipc_max_tfms: c_int;
    static mut sysctl_tipc_key_exchange_enabled: c_int;
    static SYSCTL_ONE: c_void;
    static SYSCTL_ZERO: c_void;

    fn proc_dointvec_minmax();
    fn proc_doulongvec_minmax();
    fn register_net_sysctl(
        net: *mut c_void,
        path: *const c_char,
        table: *mut ctl_table,
    ) -> *mut ctl_table_header;
    fn unregister_net_sysctl_table(header: *mut ctl_table_header);
}

type c_ulong = usize;

static mut tipc_ctl_hdr: *mut ctl_table_header = core::ptr::null_mut();

static mut tipc_table: &[ctl_table] = &[
    ctl_table {
        procname: b"tipc_rmem\0".as_ptr() as *const c_char,
        data: unsafe { &raw mut sysctl_tipc_rmem as *mut c_void },
        maxlen: core::mem::size_of::<c_int>(),
        mode: 0o644,
        proc_handler: Some(proc_dointvec_minmax),
        extra1: unsafe { &raw const SYSCTL_ONE as *const c_void as *mut c_void },
        extra2: core::ptr::null_mut(),
    },
    ctl_table {
        procname: b"named_timeout\0".as_ptr() as *const c_char,
        data: unsafe { &raw mut sysctl_tipc_named_timeout as *mut c_void },
        maxlen: core::mem::size_of::<c_int>(),
        mode: 0o644,
        proc_handler: Some(proc_dointvec_minmax),
        extra1: unsafe { &raw const SYSCTL_ZERO as *const c_void as *mut c_void },
        extra2: core::ptr::null_mut(),
    },
    ctl_table {
        procname: b"sk_filter\0".as_ptr() as *const c_char,
        data: unsafe { &raw mut sysctl_tipc_sk_filter as *mut c_void },
        maxlen: core::mem::size_of::<c_ulong>(),
        mode: 0o644,
        proc_handler: Some(proc_doulongvec_minmax),
        extra1: core::ptr::null_mut(),
        extra2: core::ptr::null_mut(),
    },
    // Corresponds to #ifdef CONFIG_TIPC_CRYPTO.
    #[cfg(CONFIG_TIPC_CRYPTO)]
    ctl_table {
        procname: b"max_tfms\0".as_ptr() as *const c_char,
        data: unsafe { &raw mut sysctl_tipc_max_tfms as *mut c_void },
        maxlen: core::mem::size_of::<c_int>(),
        mode: 0o644,
        proc_handler: Some(proc_dointvec_minmax),
        extra1: unsafe { &raw const SYSCTL_ONE as *const c_void as *mut c_void },
        extra2: core::ptr::null_mut(),
    },
    #[cfg(CONFIG_TIPC_CRYPTO)]
    ctl_table {
        procname: b"key_exchange_enabled\0".as_ptr() as *const c_char,
        data: unsafe { &raw mut sysctl_tipc_key_exchange_enabled as *mut c_void },
        maxlen: core::mem::size_of::<c_int>(),
        mode: 0o644,
        proc_handler: Some(proc_dointvec_minmax),
        extra1: unsafe { &raw const SYSCTL_ZERO as *const c_void as *mut c_void },
        extra2: unsafe { &raw const SYSCTL_ONE as *const c_void as *mut c_void },
    },
    ctl_table {
        procname: b"bc_retruni\0".as_ptr() as *const c_char,
        data: unsafe { &raw mut sysctl_tipc_bc_retruni as *mut c_void },
        maxlen: core::mem::size_of::<c_ulong>(),
        mode: 0o644,
        proc_handler: Some(proc_doulongvec_minmax),
        extra1: core::ptr::null_mut(),
        extra2: core::ptr::null_mut(),
    },
];

pub unsafe fn tipc_register_sysctl() -> c_int {
    tipc_ctl_hdr = register_net_sysctl(
        &raw mut init_net,
        b"net/tipc\0".as_ptr() as *const c_char,
        tipc_table.as_ptr() as *mut ctl_table,
    );
    if tipc_ctl_hdr.is_null() {
        return -12; // -ENOMEM
    }
    0
}

pub unsafe fn tipc_unregister_sysctl() {
    unregister_net_sysctl_table(tipc_ctl_hdr);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
