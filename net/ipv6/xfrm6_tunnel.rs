// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C)2003,2004 USAGI/WIDE Project
 *
 * Authors Mitsuru KANDA  <mk@linux-ipv6.org>
 *         YOSHIFUJI Hideaki <yoshfuji@linux-ipv6.org>
 *
 * Based on net/ipv4/xfrm4_tunnel.c
 */

// C includes omitted; their symbols are supplied by the surrounding kernel bindings.

const XFRM6_TUNNEL_SPI_BYADDR_HSIZE: usize = 256;
const XFRM6_TUNNEL_SPI_BYSPI_HSIZE: usize = 256;
const XFRM6_TUNNEL_SPI_MIN: u32 = 1;
const XFRM6_TUNNEL_SPI_MAX: u32 = 0xffff_ffff;

#[repr(C)]
struct xfrm6_tunnel_net {
    spi_byaddr: [hlist_head; XFRM6_TUNNEL_SPI_BYADDR_HSIZE],
    spi_byspi: [hlist_head; XFRM6_TUNNEL_SPI_BYSPI_HSIZE],
    spi: u32,
}

static mut xfrm6_tunnel_net_id: c_uint = 0;

#[inline]
unsafe fn xfrm6_tunnel_pernet(net: *mut net) -> *mut xfrm6_tunnel_net {
    net_generic(net, xfrm6_tunnel_net_id)
}

/*
 * xfrm_tunnel_spi things are for allocating unique id ("spi")
 * per xfrm_address_t.
 */
#[repr(C)]
struct xfrm6_tunnel_spi {
    list_byaddr: hlist_node,
    list_byspi: hlist_node,
    addr: xfrm_address_t,
    spi: u32,
    refcnt: refcount_t,
    rcu_head: rcu_head,
}

static mut xfrm6_tunnel_spi_lock: spinlock_t = DEFINE_SPINLOCK!();
static mut xfrm6_tunnel_spi_kmem: *mut kmem_cache = core::ptr::null_mut();

#[inline]
unsafe fn xfrm6_tunnel_spi_hash_byaddr(addr: *const xfrm_address_t) -> c_uint {
    let mut h = ipv6_addr_hash(addr as *const in6_addr);
    h ^= h >> 16;
    h ^= h >> 8;
    h &= (XFRM6_TUNNEL_SPI_BYADDR_HSIZE - 1) as c_uint;
    h
}

#[inline]
fn xfrm6_tunnel_spi_hash_byspi(spi: u32) -> c_uint {
    (spi % XFRM6_TUNNEL_SPI_BYSPI_HSIZE as u32) as c_uint
}

unsafe fn __xfrm6_tunnel_spi_lookup(net: *mut net, saddr: *const xfrm_address_t) -> *mut xfrm6_tunnel_spi {
    let xfrm6_tn = xfrm6_tunnel_pernet(net);
    let mut x6spi: *mut xfrm6_tunnel_spi = core::ptr::null_mut();
    hlist_for_each_entry_rcu!(x6spi, (*xfrm6_tn).spi_byaddr[xfrm6_tunnel_spi_hash_byaddr(saddr) as usize], list_byaddr, lockdep_is_held(&xfrm6_tunnel_spi_lock));
    if !x6spi.is_null() && xfrm6_addr_equal(&(*x6spi).addr, saddr) {
        return x6spi;
    }
    core::ptr::null_mut()
}

#[no_mangle]
unsafe fn xfrm6_tunnel_spi_lookup(net: *mut net, saddr: *const xfrm_address_t) -> __be32 {
    rcu_read_lock_bh();
    let x6spi = __xfrm6_tunnel_spi_lookup(net, saddr);
    let spi = if !x6spi.is_null() { (*x6spi).spi } else { 0 };
    rcu_read_unlock_bh();
    htonl(spi)
}

unsafe fn __xfrm6_tunnel_spi_check(net: *mut net, spi: u32) -> c_int {
    let xfrm6_tn = xfrm6_tunnel_pernet(net);
    let index = xfrm6_tunnel_spi_hash_byspi(spi);
    let mut x6spi: *mut xfrm6_tunnel_spi = core::ptr::null_mut();
    hlist_for_each_entry!(x6spi, (*xfrm6_tn).spi_byspi[index as usize], list_byspi);
    if !x6spi.is_null() && (*x6spi).spi == spi { return -1; }
    index as c_int
}

unsafe fn __xfrm6_tunnel_alloc_spi(net: *mut net, saddr: *mut xfrm_address_t) -> u32 {
    let xfrm6_tn = xfrm6_tunnel_pernet(net);
    if (*xfrm6_tn).spi < XFRM6_TUNNEL_SPI_MIN || (*xfrm6_tn).spi >= XFRM6_TUNNEL_SPI_MAX {
        (*xfrm6_tn).spi = XFRM6_TUNNEL_SPI_MIN;
    } else { (*xfrm6_tn).spi += 1; }
    let mut spi = (*xfrm6_tn).spi;
    let mut index: c_int = -1;
    while spi <= XFRM6_TUNNEL_SPI_MAX {
        index = __xfrm6_tunnel_spi_check(net, spi);
        if index >= 0 { break; }
        if spi == XFRM6_TUNNEL_SPI_MAX { break; }
        spi += 1;
    }
    if index < 0 {
        spi = XFRM6_TUNNEL_SPI_MIN;
        while spi < (*xfrm6_tn).spi {
            index = __xfrm6_tunnel_spi_check(net, spi);
            if index >= 0 { break; }
            spi += 1;
        }
    }
    if index < 0 { return 0; }
    (*xfrm6_tn).spi = spi;
    let x6spi = kmem_cache_alloc(xfrm6_tunnel_spi_kmem, GFP_ATOMIC) as *mut xfrm6_tunnel_spi;
    if x6spi.is_null() { return spi; }
    core::ptr::copy_nonoverlapping(saddr, &mut (*x6spi).addr, 1);
    (*x6spi).spi = spi;
    refcount_set(&mut (*x6spi).refcnt, 1);
    hlist_add_head_rcu(&mut (*x6spi).list_byspi, &mut (*xfrm6_tn).spi_byspi[index as usize]);
    let index = xfrm6_tunnel_spi_hash_byaddr(saddr);
    hlist_add_head_rcu(&mut (*x6spi).list_byaddr, &mut (*xfrm6_tn).spi_byaddr[index as usize]);
    spi
}

#[no_mangle]
unsafe fn xfrm6_tunnel_alloc_spi(net: *mut net, saddr: *mut xfrm_address_t) -> __be32 {
    spin_lock_bh(&mut xfrm6_tunnel_spi_lock);
    let x6spi = __xfrm6_tunnel_spi_lookup(net, saddr);
    let spi = if !x6spi.is_null() { refcount_inc(&mut (*x6spi).refcnt); (*x6spi).spi } else { __xfrm6_tunnel_alloc_spi(net, saddr) };
    spin_unlock_bh(&mut xfrm6_tunnel_spi_lock);
    htonl(spi)
}

unsafe fn x6spi_destroy_rcu(head: *mut rcu_head) {
    kmem_cache_free(xfrm6_tunnel_spi_kmem, container_of!(head, xfrm6_tunnel_spi, rcu_head));
}

unsafe fn xfrm6_tunnel_free_spi(net: *mut net, saddr: *mut xfrm_address_t) {
    let xfrm6_tn = xfrm6_tunnel_pernet(net);
    spin_lock_bh(&mut xfrm6_tunnel_spi_lock);
    let mut x6spi: *mut xfrm6_tunnel_spi = core::ptr::null_mut();
    let mut n: *mut hlist_node = core::ptr::null_mut();
    hlist_for_each_entry_safe!(x6spi, n, (*xfrm6_tn).spi_byaddr[xfrm6_tunnel_spi_hash_byaddr(saddr) as usize], list_byaddr);
    if !x6spi.is_null() && xfrm6_addr_equal(&(*x6spi).addr, saddr) && refcount_dec_and_test(&mut (*x6spi).refcnt) {
        hlist_del_rcu(&mut (*x6spi).list_byaddr);
        hlist_del_rcu(&mut (*x6spi).list_byspi);
        call_rcu(&mut (*x6spi).rcu_head, x6spi_destroy_rcu);
    }
    spin_unlock_bh(&mut xfrm6_tunnel_spi_lock);
}

unsafe fn xfrm6_tunnel_output(_x: *mut xfrm_state, skb: *mut sk_buff) -> c_int {
    skb_push(skb, -skb_network_offset(skb));
    0
}

unsafe fn xfrm6_tunnel_input(_x: *mut xfrm_state, skb: *mut sk_buff) -> c_int {
    (*skb_network_header(skb).add((*IP6CB(skb)).nhoff as usize)) as c_int
}

unsafe fn xfrm6_tunnel_rcv(skb: *mut sk_buff) -> c_int {
    let net = dev_net((*skb).dev);
    let iph = ipv6_hdr(skb);
    let spi = xfrm6_tunnel_spi_lookup(net, &(*iph).saddr as *const _ as *const xfrm_address_t);
    xfrm6_rcv_spi(skb, IPPROTO_IPV6, spi, core::ptr::null_mut())
}

unsafe fn xfrm6_tunnel_err(_skb: *mut sk_buff, _opt: *mut inet6_skb_parm, _type: u8, _code: u8, _offset: c_int, _info: __be32) -> c_int {
    /* xfrm6_tunnel native err handling */
    0
}

unsafe fn xfrm6_tunnel_init_state(x: *mut xfrm_state, extack: *mut netlink_ext_ack) -> c_int {
    if (*x).props.mode != XFRM_MODE_TUNNEL { NL_SET_ERR_MSG!(extack, "IPv6 tunnel can only be used with tunnel mode"); return -EINVAL; }
    if !(*x).encap.is_null() { NL_SET_ERR_MSG!(extack, "IPv6 tunnel is not compatible with encapsulation"); return -EINVAL; }
    (*x).props.header_len = core::mem::size_of::<ipv6hdr>();
    0
}

unsafe fn xfrm6_tunnel_destroy(x: *mut xfrm_state) {
    xfrm6_tunnel_free_spi(xs_net(x), &mut (*x).props.saddr as *mut _ as *mut xfrm_address_t);
}

static mut xfrm6_tunnel_type: xfrm_type = xfrm_type {
    owner: THIS_MODULE, proto: IPPROTO_IPV6, init_state: Some(xfrm6_tunnel_init_state), destructor: Some(xfrm6_tunnel_destroy), input: Some(xfrm6_tunnel_input), output: Some(xfrm6_tunnel_output),
};

static mut xfrm6_tunnel_handler: xfrm6_tunnel = xfrm6_tunnel { handler: Some(xfrm6_tunnel_rcv), err_handler: Some(xfrm6_tunnel_err), priority: 3 };
static mut xfrm46_tunnel_handler: xfrm6_tunnel = xfrm6_tunnel { handler: Some(xfrm6_tunnel_rcv), err_handler: Some(xfrm6_tunnel_err), priority: 3 };

unsafe fn xfrm6_tunnel_net_init(net: *mut net) -> c_int {
    let tn = xfrm6_tunnel_pernet(net);
    for i in 0..XFRM6_TUNNEL_SPI_BYADDR_HSIZE { INIT_HLIST_HEAD!(&mut (*tn).spi_byaddr[i]); }
    for i in 0..XFRM6_TUNNEL_SPI_BYSPI_HSIZE { INIT_HLIST_HEAD!(&mut (*tn).spi_byspi[i]); }
    (*tn).spi = 0;
    0
}

unsafe fn xfrm6_tunnel_net_exit(net: *mut net) {
    let tn = xfrm6_tunnel_pernet(net);
    xfrm_state_flush(net, 0, false); xfrm_flush_gc();
    for i in 0..XFRM6_TUNNEL_SPI_BYADDR_HSIZE { WARN_ON_ONCE!(!hlist_empty(&(*tn).spi_byaddr[i])); }
    for i in 0..XFRM6_TUNNEL_SPI_BYSPI_HSIZE { WARN_ON_ONCE!(!hlist_empty(&(*tn).spi_byspi[i])); }
}

static mut xfrm6_tunnel_net_ops: pernet_operations = pernet_operations { init: Some(xfrm6_tunnel_net_init), exit: Some(xfrm6_tunnel_net_exit), id: &mut xfrm6_tunnel_net_id, size: core::mem::size_of::<xfrm6_tunnel_net>() };

unsafe fn xfrm6_tunnel_init() -> c_int {
    xfrm6_tunnel_spi_kmem = KMEM_CACHE!(xfrm6_tunnel_spi, SLAB_HWCACHE_ALIGN);
    if xfrm6_tunnel_spi_kmem.is_null() { return -ENOMEM; }
    let mut rv = register_pernet_subsys(&mut xfrm6_tunnel_net_ops);
    if rv < 0 { kmem_cache_destroy(xfrm6_tunnel_spi_kmem); return rv; }
    rv = xfrm_register_type(&mut xfrm6_tunnel_type, AF_INET6); if rv < 0 { unregister_pernet_subsys(&mut xfrm6_tunnel_net_ops); kmem_cache_destroy(xfrm6_tunnel_spi_kmem); return rv; }
    rv = xfrm6_tunnel_register(&mut xfrm6_tunnel_handler, AF_INET6); if rv < 0 { xfrm_unregister_type(&mut xfrm6_tunnel_type, AF_INET6); unregister_pernet_subsys(&mut xfrm6_tunnel_net_ops); kmem_cache_destroy(xfrm6_tunnel_spi_kmem); return rv; }
    rv = xfrm6_tunnel_register(&mut xfrm46_tunnel_handler, AF_INET); if rv < 0 { xfrm6_tunnel_deregister(&mut xfrm6_tunnel_handler, AF_INET6); xfrm_unregister_type(&mut xfrm6_tunnel_type, AF_INET6); unregister_pernet_subsys(&mut xfrm6_tunnel_net_ops); kmem_cache_destroy(xfrm6_tunnel_spi_kmem); return rv; }
    0
}

unsafe fn xfrm6_tunnel_fini() {
    xfrm6_tunnel_deregister(&mut xfrm46_tunnel_handler, AF_INET);
    xfrm6_tunnel_deregister(&mut xfrm6_tunnel_handler, AF_INET6);
    xfrm_unregister_type(&mut xfrm6_tunnel_type, AF_INET6);
    unregister_pernet_subsys(&mut xfrm6_tunnel_net_ops);
    /* Someone maybe has gotten the xfrm6_tunnel_spi.
     * So need to wait it.
     */
    rcu_barrier();
    kmem_cache_destroy(xfrm6_tunnel_spi_kmem);
}

module_init!(xfrm6_tunnel_init);
module_exit!(xfrm6_tunnel_fini);
MODULE_DESCRIPTION!("IPv6 XFRM tunnel driver");
MODULE_LICENSE!("GPL");
MODULE_ALIAS_XFRM_TYPE!(AF_INET6, XFRM_PROTO_IPV6);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
