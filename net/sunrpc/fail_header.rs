/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2021, Oracle. All rights reserved.
 */

// Dependency: linux/fault-inject.h supplies `fault_attr`.

// Corresponds to: #if IS_ENABLED(CONFIG_FAULT_INJECTION)
#[cfg(CONFIG_FAULT_INJECTION)]
#[repr(C)]
pub struct fail_sunrpc_attr {
    pub attr: fault_attr,
    pub ignore_client_disconnect: bool,
    pub ignore_server_disconnect: bool,
    pub ignore_cache_wait: bool,
}

#[cfg(CONFIG_FAULT_INJECTION)]
extern "C" {
    pub static mut fail_sunrpc: fail_sunrpc_attr;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
