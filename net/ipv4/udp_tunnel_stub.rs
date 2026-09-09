// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2020 Facebook Inc.

// Dependency supplied by net/udp_tunnel.h.

pub static mut udp_tunnel_nic_ops: *const udp_tunnel_nic_ops = core::ptr::null();

// EXPORT_SYMBOL_GPL(udp_tunnel_nic_ops);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
