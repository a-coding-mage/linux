/* SPDX-License-Identifier: GPL-2.0 */
/* linux/net/inet/arp.h */

/* Dependencies supplied by the corresponding Linux networking headers. */

extern "C" {
    pub static mut arp_tbl: neigh_table;
}

pub unsafe fn arp_hashfn(
    pkey: *const core::ffi::c_void,
    dev: *const net_device,
    hash_rnd: *mut u32,
) -> u32 {
    let key = *(pkey as *const u32);
    let val = key ^ hash32_ptr(dev);

    val.wrapping_mul(*hash_rnd)
}

#[cfg(feature = "CONFIG_INET")]
pub unsafe fn __ipv4_neigh_lookup_noref(
    dev: *mut net_device,
    mut key: u32,
) -> *mut neighbour {
    if (*dev).flags & (IFF_LOOPBACK | IFF_POINTOPOINT) != 0 {
        key = INADDR_ANY;
    }

    ___neigh_lookup_noref(
        &mut arp_tbl,
        neigh_key_eq32,
        arp_hashfn,
        &key,
        dev,
    )
}

#[cfg(not(feature = "CONFIG_INET"))]
pub unsafe fn __ipv4_neigh_lookup_noref(
    _dev: *mut net_device,
    _key: u32,
) -> *mut neighbour {
    core::ptr::null_mut()
}

pub unsafe fn __ipv4_neigh_lookup(dev: *mut net_device, key: u32) -> *mut neighbour {
    let mut n: *mut neighbour;

    rcu_read_lock();
    n = __ipv4_neigh_lookup_noref(dev, key);
    if !n.is_null() && refcount_inc_not_zero(&mut (*n).refcnt) == 0 {
        n = core::ptr::null_mut();
    }
    rcu_read_unlock();

    n
}

pub unsafe fn __ipv4_confirm_neigh(dev: *mut net_device, key: u32) {
    let n: *mut neighbour;

    rcu_read_lock();
    n = __ipv4_neigh_lookup_noref(dev, key);
    neigh_confirm(n);
    rcu_read_unlock();
}

extern "C" {
    pub fn arp_init();
    pub fn arp_ioctl(net: *mut net, cmd: core::ffi::c_uint, arg: *mut core::ffi::c_void) -> core::ffi::c_int;
    pub fn arp_send(
        type_: core::ffi::c_int,
        ptype: core::ffi::c_int,
        dest_ip: __be32,
        dev: *mut net_device,
        src_ip: __be32,
        dest_hw: *const u8,
        src_hw: *const u8,
        th: *const u8,
    );
    pub fn arp_mc_map(
        addr: __be32,
        haddr: *mut u8,
        dev: *mut net_device,
        dir: core::ffi::c_int,
    ) -> core::ffi::c_int;
    pub fn arp_ifdown(dev: *mut net_device);
    pub fn arp_invalidate(dev: *mut net_device, ip: __be32, force: bool) -> core::ffi::c_int;
    pub fn arp_create(
        type_: core::ffi::c_int,
        ptype: core::ffi::c_int,
        dest_ip: __be32,
        dev: *mut net_device,
        src_ip: __be32,
        dest_hw: *const u8,
        src_hw: *const u8,
        target_hw: *const u8,
    ) -> *mut sk_buff;
    pub fn arp_xmit(skb: *mut sk_buff);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
