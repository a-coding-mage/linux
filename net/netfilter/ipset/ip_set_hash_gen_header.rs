/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (C) 2013 Jozsef Kadlecsik <kadlec@netfilter.org> */

/* Translated from ip_set_hash_gen.h. Kernel dependencies are supplied externally. */

pub const AHASH_INIT_SIZE: usize = 2;
pub const AHASH_MAX_SIZE: usize = 6 * AHASH_INIT_SIZE;
pub const AHASH_MAX_TUNED: usize = 64;
pub const HTABLE_REGION_BITS: u8 = 10;
pub const CIDR_MAX_COUNT: u32 = (1 << 24) - 1;

#[macro_export]
macro_rules! __ipset_dereference { ($p:expr) => { rcu_dereference_protected!($p, 1) }; }
#[macro_export]
macro_rules! ipset_dereference_nfnl { ($p:expr) => { rcu_dereference_protected!($p, lockdep_nfnl_is_held(NFNL_SUBSYS_IPSET)) }; }
#[macro_export]
macro_rules! ipset_dereference_set { ($p:expr, $set:expr) => { rcu_dereference_protected!($p, lockdep_nfnl_is_held(NFNL_SUBSYS_IPSET) || lockdep_is_held(&(*$set).lock)) }; }
#[macro_export]
macro_rules! ipset_dereference_bh_nfnl { ($p:expr) => { rcu_dereference_bh_check!($p, lockdep_nfnl_is_held(NFNL_SUBSYS_IPSET)) }; }

/* Hashing uses arrays to resolve clashes; the table doubles when searching is too long. */

#[repr(C)]
pub struct hbucket {
    pub rcu: rcu_head,
    /* DECLARE_BITMAP(used, AHASH_MAX_TUNED) */
    pub used: [u64; (AHASH_MAX_TUNED + 63) / 64],
    pub size: u8,
    pub pos: u8,
    pub value: [u8; 0],
}

#[repr(C)]
pub struct htable_gc {
    pub dwork: delayed_work,
    pub set: *mut ip_set,
    pub lock: spinlock_t,
    pub region: u32,
}

#[repr(C)]
pub struct htable {
    pub resizing: bool,
    pub uref: atomic_t,
    pub htable_bits: u8,
    pub maxelem: u32,
    pub ad: list_head,
    pub hregion: *mut ip_set_region,
    pub bucket: [*mut hbucket; 0],
}

#[macro_export]
macro_rules! AHASH_MAX { ($h:expr) => { (*$h).bucketsize }; }
#[macro_export]
macro_rules! hbucket { ($h:expr, $i:expr) => { (*$h).bucket[$i] }; }
#[macro_export]
macro_rules! ext_size { ($n:expr, $dsize:expr) => { core::mem::size_of::<hbucket>() + ($n) * ($dsize) }; }

#[macro_export]
macro_rules! ahash_numof_locks {
    ($htable_bits:expr) => { if $htable_bits < HTABLE_REGION_BITS { 1 } else { jhash_size($htable_bits - HTABLE_REGION_BITS) } };
}
#[macro_export]
macro_rules! ahash_sizeof_regions { ($htable_bits:expr) => { ahash_numof_locks!($htable_bits) * core::mem::size_of::<ip_set_region>() }; }
#[macro_export]
macro_rules! ahash_region { ($n:expr) => { ($n) / jhash_size(HTABLE_REGION_BITS) }; }
#[macro_export]
macro_rules! ahash_bucket_start { ($h:expr, $bits:expr) => { if $bits < HTABLE_REGION_BITS { 0 } else { ($h) * jhash_size(HTABLE_REGION_BITS) } }; }
#[macro_export]
macro_rules! ahash_bucket_end { ($h:expr, $bits:expr) => { if $bits < HTABLE_REGION_BITS { jhash_size($bits) } else { (($h) + 1) * jhash_size(HTABLE_REGION_BITS) } }; }

#[repr(C)]
pub struct net_prefix {
    /* C bit-fields: cidr:8 and count:24, packed into one u32. */
    pub cidr_count: u32,
}

#[repr(C)]
pub struct net_prefixes {
    pub rcu: rcu_head,
    pub seq: seqcount_spinlock_t,
    pub len: u8,
    pub nets: [net_prefix; 0],
}

pub const IPSET_NET_COUNT: usize = 1;

#[cfg(any(IP_SET_HASH_WITH_NETMASK, IP_SET_HASH_WITH_BITMASK))]
#[repr(C)]
pub union nf_inet_addr {
    pub all: [u32; 4],
}

#[cfg(any(IP_SET_HASH_WITH_NETMASK, IP_SET_HASH_WITH_BITMASK))]
pub static ONESMASK: nf_inet_addr = nf_inet_addr { all: [0xffff_ffff; 4] };
#[cfg(any(IP_SET_HASH_WITH_NETMASK, IP_SET_HASH_WITH_BITMASK))]
pub static ZEROMASK: nf_inet_addr = nf_inet_addr { all: [0; 4] };

#[inline]
pub unsafe fn htable_size(hbits: u8) -> usize {
    let hsize: usize;
    if hbits > 31 { return 0; }
    hsize = jhash_size(hbits) as usize;
    if ((i32::MAX as usize - core::mem::size_of::<htable>()) / core::mem::size_of::<*mut hbucket>()) < hsize {
        return 0;
    }
    hsize * core::mem::size_of::<*mut hbucket>() + core::mem::size_of::<htable>()
}

/* IP_SET_HASH_WITH_NETS, IP_SET_HASH_WITH_NETS_PACKED, and family-specific
 * MTYPE/HTYPE/HOST_MASK selections are build-time C conditions retained here. */
#[macro_export]
macro_rules! SET_ELEM_EXPIRED { ($set:expr, $d:expr) => { SET_WITH_TIMEOUT!($set) && ip_set_timeout_expired!(ext_timeout!($d, $set)) }; }

/* The following template names are intentionally undefined here, matching the C #undef list. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
