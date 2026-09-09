/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * SNMP MIB entries for the IP subsystem.
 *
 * The Linux header dependencies are supplied by other translation units.
 */

/* Mibs are stored in array of unsigned long. */
/*
 * struct snmp_mib{}
 *  - list of entries for particular API (such as /proc/net/snmp)
 *  - name of entries.
 */
#[repr(C)]
pub struct snmp_mib {
    pub name: *const ::std::os::raw::c_char,
    pub entry: ::std::os::raw::c_int,
}

#[macro_export]
macro_rules! SNMP_MIB_ITEM {
    ($name:expr, $entry:expr) => {
        $crate::snmp_mib { name: $name, entry: $entry }
    };
}

/* We use unsigned longs for most mibs but u64 for ipstats. */

/* IPstats */
pub const IPSTATS_MIB_MAX: usize = __IPSTATS_MIB_MAX as usize;
#[repr(C)]
pub struct ipstats_mib {
    /* mibs[] must be first field of struct ipstats_mib */
    pub mibs: [u64; IPSTATS_MIB_MAX],
    pub syncp: u64_stats_sync,
}

/* ICMP */
pub const ICMP_MIB_MAX: usize = __ICMP_MIB_MAX as usize;
#[repr(C)]
pub struct icmp_mib {
    pub mibs: [::std::os::raw::c_ulong; ICMP_MIB_MAX],
}

pub const ICMPMSG_MIB_MAX: usize = __ICMPMSG_MIB_MAX as usize;
#[repr(C)]
pub struct icmpmsg_mib {
    pub mibs: [atomic_long_t; ICMPMSG_MIB_MAX],
}

/* ICMP6 (IPv6-ICMP) */
pub const ICMP6_MIB_MAX: usize = __ICMP6_MIB_MAX as usize;
/* per network ns counters */
#[repr(C)]
pub struct icmpv6_mib {
    pub mibs: [::std::os::raw::c_ulong; ICMP6_MIB_MAX],
}
/* per device counters, (shared on all cpus) */
#[repr(C)]
pub struct icmpv6_mib_device {
    pub mibs: [atomic_long_t; ICMP6_MIB_MAX],
}

pub const ICMP6MSG_MIB_MAX: usize = __ICMP6MSG_MIB_MAX as usize;
/* per network ns counters */
#[repr(C)]
pub struct icmpv6msg_mib {
    pub mibs: [atomic_long_t; ICMP6MSG_MIB_MAX],
}
/* per device counters, (shared on all cpus) */
#[repr(C)]
pub struct icmpv6msg_mib_device {
    pub mibs: [atomic_long_t; ICMP6MSG_MIB_MAX],
}

/* TCP */
pub const TCP_MIB_MAX: usize = __TCP_MIB_MAX as usize;
#[repr(C)]
pub struct tcp_mib { pub mibs: [::std::os::raw::c_ulong; TCP_MIB_MAX] }

/* UDP */
pub const UDP_MIB_MAX: usize = __UDP_MIB_MAX as usize;
#[repr(C)]
pub struct udp_mib { pub mibs: [::std::os::raw::c_ulong; UDP_MIB_MAX] }

/* Linux */
pub const LINUX_MIB_MAX: usize = __LINUX_MIB_MAX as usize;
#[repr(C)]
pub struct linux_mib { pub mibs: [::std::os::raw::c_ulong; LINUX_MIB_MAX] }

/* Linux Xfrm */
pub const LINUX_MIB_XFRMMAX: usize = __LINUX_MIB_XFRMMAX as usize;
#[repr(C)]
pub struct linux_xfrm_mib { pub mibs: [::std::os::raw::c_ulong; LINUX_MIB_XFRMMAX] }

/* Linux TLS */
pub const LINUX_MIB_TLSMAX: usize = __LINUX_MIB_TLSMAX as usize;
#[repr(C)]
pub struct linux_tls_mib { pub mibs: [::std::os::raw::c_ulong; LINUX_MIB_TLSMAX] }

/* External Linux per-CPU/atomic operations are intentionally retained as macro calls. */
#[macro_export]
macro_rules! DEFINE_SNMP_STAT { ($type:ty, $name:ident) => { pub static mut $name: *mut $type = ::std::ptr::null_mut(); }; }
#[macro_export]
macro_rules! DEFINE_SNMP_STAT_ATOMIC { ($type:ty, $name:ident) => { pub static mut $name: *mut $type = ::std::ptr::null_mut(); }; }
#[macro_export]
macro_rules! DECLARE_SNMP_STAT { ($type:ty, $name:ident) => { unsafe extern "C" { pub static mut $name: *mut $type; } }; }

#[macro_export] macro_rules! __SNMP_INC_STATS { ($m:expr, $f:expr) => { __this_cpu_inc!($m.mibs[$f]); }; }
#[macro_export] macro_rules! SNMP_INC_STATS_ATOMIC_LONG { ($m:expr, $f:expr) => { atomic_long_inc!(&mut $m.mibs[$f]); }; }
#[macro_export] macro_rules! SNMP_INC_STATS { ($m:expr, $f:expr) => { this_cpu_inc!($m.mibs[$f]); }; }
#[macro_export] macro_rules! SNMP_DEC_STATS { ($m:expr, $f:expr) => { this_cpu_dec!($m.mibs[$f]); }; }
#[macro_export] macro_rules! __SNMP_ADD_STATS { ($m:expr, $f:expr, $a:expr) => { __this_cpu_add!($m.mibs[$f], $a); }; }
#[macro_export] macro_rules! SNMP_ADD_STATS { ($m:expr, $f:expr, $a:expr) => { this_cpu_add!($m.mibs[$f], $a); }; }

#[macro_export]
macro_rules! SNMP_UPD_PO_STATS { ($m:expr, $basefield:ident, $addend:expr) => {{ this_cpu_inc!($m.mibs[$basefield##PKTS]); this_cpu_add!($m.mibs[$basefield##OCTETS], $addend); }}; }
#[macro_export]
macro_rules! __SNMP_UPD_PO_STATS { ($m:expr, $basefield:ident, $addend:expr) => {{ __this_cpu_inc!($m.mibs[$basefield##PKTS]); __this_cpu_add!($m.mibs[$basefield##OCTETS], $addend); }}; }

/* On 32-bit systems the u64 statistics update is serialized by syncp. */
#[macro_export] macro_rules! __SNMP_INC_STATS64 { ($m:expr, $f:expr) => { SNMP_ADD_STATS64!($m, $f, 1); }; }
#[macro_export] macro_rules! SNMP_INC_STATS64 { ($m:expr, $f:expr) => { SNMP_ADD_STATS64!($m, $f, 1); }; }
#[macro_export] macro_rules! SNMP_DEC_STATS64 { ($m:expr, $f:expr) => { SNMP_ADD_STATS64!($m, $f, -1); }; }
#[macro_export] macro_rules! __SNMP_ADD_STATS64 { ($m:expr, $f:expr, $a:expr) => { __SNMP_ADD_STATS!($m, $f, $a); }; }
#[macro_export] macro_rules! SNMP_ADD_STATS64 { ($m:expr, $f:expr, $a:expr) => { SNMP_ADD_STATS!($m, $f, $a); }; }
#[macro_export] macro_rules! SNMP_UPD_PO_STATS64 { ($m:expr, $b:ident, $a:expr) => { SNMP_UPD_PO_STATS!($m, $b, $a); }; }
#[macro_export] macro_rules! __SNMP_UPD_PO_STATS64 { ($m:expr, $b:ident, $a:expr) => { __SNMP_UPD_PO_STATS!($m, $b, $a); }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
