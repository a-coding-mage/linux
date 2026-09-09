/* SPDX-License-Identifier: GPL-2.0
 * Copyright 2022-2023 NXP
 */

// C tracepoint include guard: _NET_DSA_TRACE_H / TRACE_HEADER_MULTI_READ.
// The Linux tracepoint preprocessor definitions are intentionally represented
// below as Rust declarations and marker types.

/// Enough to fit "bridge %s num %d" where num has 3 digits.
pub const DSA_DB_BUFSIZ: usize = IFNAMSIZ + 16;

extern "C" {
    pub fn dsa_db_print(db: *const dsa_db, buf: *mut core::ffi::c_char);
    pub fn dsa_port_kind(dp: *const dsa_port) -> *const core::ffi::c_char;
}

// External Linux kernel types supplied by the translated dependencies.
#[repr(C)]
pub struct dsa_db {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dsa_port {
    _private: [u8; 0],
}
#[repr(C)]
pub struct net_device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct switchdev_obj_port_vlan {
    _private: [u8; 0],
}
#[repr(C)]
pub struct refcount_t {
    _private: [u8; 0],
}

// DECLARE_EVENT_CLASS(dsa_port_addr_op_hw)
// TP_PROTO(const struct dsa_port *dp, const unsigned char *addr, u16 vid,
//          const struct dsa_db *db, int err)
// TP_STRUCT__entry: dev, kind, port: int, addr: [u8; ETH_ALEN], vid: u16,
//                   db_buf: [c_char; DSA_DB_BUFSIZ], err: int
// TP_fast_assign copies dev/kind, dp->index, addr, vid, db text, and err.
// TP_printk("%s %s port %d addr %pM vid %u db \"%s\" err %d", ...)
pub struct dsa_port_addr_op_hw;
pub struct dsa_fdb_add_hw;
pub struct dsa_mdb_add_hw;
pub struct dsa_fdb_del_hw;
pub struct dsa_mdb_del_hw;

// DECLARE_EVENT_CLASS(dsa_port_addr_op_refcount)
// TP_PROTO(const struct dsa_port *dp, const unsigned char *addr, u16 vid,
//          const struct dsa_db *db, const refcount_t *refcount)
// Entry fields: dev, kind, port, addr, vid, db_buf, refcount: unsigned int.
// Assignment copies the address and reads refcount with refcount_read().
// TP_printk("%s %s port %d addr %pM vid %u db \"%s\" refcount %u", ...)
pub struct dsa_port_addr_op_refcount;
pub struct dsa_fdb_add_bump;
pub struct dsa_mdb_add_bump;
pub struct dsa_fdb_del_drop;
pub struct dsa_mdb_del_drop;

// DECLARE_EVENT_CLASS(dsa_port_addr_del_not_found)
// TP_PROTO(const struct dsa_port *dp, const unsigned char *addr, u16 vid,
//          const struct dsa_db *db); entry: dev, kind, port, addr, vid, db_buf.
// TP_printk("%s %s port %d addr %pM vid %u db \"%s\"", ...)
pub struct dsa_port_addr_del_not_found;
pub struct dsa_fdb_del_not_found;
pub struct dsa_mdb_del_not_found;

// TRACE_EVENT declarations for LAG FDB operations.  Each event has lag_dev,
// addr[ETH_ALEN], vid, db_buf[DSA_DB_BUFSIZ], and either err or refcount where
// shown by its C prototype; assignments copy addr and print the fields.
pub struct dsa_lag_fdb_add_hw;
pub struct dsa_lag_fdb_add_bump;
pub struct dsa_lag_fdb_del_hw;
pub struct dsa_lag_fdb_del_drop;
pub struct dsa_lag_fdb_del_not_found;

// DECLARE_EVENT_CLASS(dsa_vlan_op_hw)
// TP_PROTO(const struct dsa_port *dp,
//          const struct switchdev_obj_port_vlan *vlan, int err)
// Entry: dev, kind, port: int, vid: u16, flags: u16, changed: bool, err: int.
// TP_printk includes " pvid", " untagged", and " (changed)" conditionally.
pub struct dsa_vlan_op_hw;
pub struct dsa_vlan_add_hw;
pub struct dsa_vlan_del_hw;

// DECLARE_EVENT_CLASS(dsa_vlan_op_refcount)
// Entry: dev, kind, port: int, vid: u16, flags: u16, changed: bool,
// refcount: unsigned int; refcount is obtained with refcount_read().
// TP_printk appends the same conditional VLAN flags and refcount.
pub struct dsa_vlan_op_refcount;
pub struct dsa_vlan_add_bump;
pub struct dsa_vlan_del_drop;

// TRACE_EVENT(dsa_vlan_del_not_found)
// TP_PROTO(const struct dsa_port *dp,
//          const struct switchdev_obj_port_vlan *vlan)
// Entry: dev, kind, port: int, vid: u16.
pub struct dsa_vlan_del_not_found;

// TRACE_INCLUDE_PATH ., TRACE_INCLUDE_FILE trace, and trace/define_trace.h
// are C preprocessor/build-system directives and have no executable Rust form.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
