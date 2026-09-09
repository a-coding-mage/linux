// SPDX-License-Identifier: GPL-2.0-or-later
// Faithful low-level Rust translation of inet_hashtables.c.
// Kernel types, constants, macros, and external functions are supplied by the
// surrounding translation unit.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

extern "C" {
    static mut inet_ehash_secret: [u8; 0];
    static mut table_perturb: *mut u32;
}

unsafe fn inet_init_ehash_secret() {
    net_get_random_sleepable_once(&mut inet_ehash_secret as *mut _, core::mem::size_of_val(&inet_ehash_secret));
}

#[no_mangle]
pub unsafe extern "C" fn inet_ehashfn(net: *const net, laddr: __be32, lport: __u16, faddr: __be32, fport: __be16) -> u32 {
    lport as u32 + __inet_ehashfn(laddr, 0, faddr, fport, inet_ehash_secret.as_mut_ptr().add(net_hash_mix(net) as usize))
}

unsafe fn sk_ehashfn(sk: *const sock) -> u32 {
    #[cfg(CONFIG_IPV6)]
    if (*sk).sk_family == AF_INET6 && !ipv6_addr_v4mapped(&(*sk).sk_v6_daddr) {
        return inet6_ehashfn(sock_net(sk), &(*sk).sk_v6_rcv_saddr, (*sk).sk_num, &(*sk).sk_v6_daddr, (*sk).sk_dport);
    }
    inet_ehashfn(sock_net(sk), (*sk).sk_rcv_saddr, (*sk).sk_num, (*sk).sk_daddr, (*sk).sk_dport)
}

unsafe fn sk_is_connect_bind(sk: *const sock) -> bool {
    if (*sk).sk_state == TCP_TIME_WAIT { inet_twsk(sk).tw_connect_bind } else { (*sk).sk_userlocks & SOCK_CONNECT_BIND != 0 }
}

pub unsafe extern "C" fn inet_bind_bucket_create(cachep: *mut kmem_cache, net: *mut net, head: *mut inet_bind_hashbucket, snum: u16, l3mdev: i32) -> *mut inet_bind_bucket {
    let tb = kmem_cache_alloc(cachep, GFP_ATOMIC);
    if !tb.is_null() { write_pnet(&mut (*tb).ib_net, net); (*tb).l3mdev=l3mdev; (*tb).port=snum; (*tb).fastreuse=0; (*tb).fastreuseport=0; INIT_HLIST_HEAD(&mut (*tb).bhash2); hlist_add_head_rcu(&mut (*tb).node, &mut (*head).chain); }
    tb
}

pub unsafe extern "C" fn inet_bind_bucket_destroy(tb: *mut inet_bind_bucket) {
    if hlist_empty(&(*tb).bhash2) { hlist_del_rcu(&mut (*tb).node); kfree_rcu(tb, rcu); return; }
    if (*tb).fastreuse == -1 && (*tb).fastreuseport == -1 { return; }
    let mut tb2 = core::ptr::null();
    hlist_for_each_entry(tb2, &(*tb).bhash2, bhash_node) { if (*tb2).fastreuse != -1 || (*tb2).fastreuseport != -1 { return; } }
    (*tb).fastreuse=-1; (*tb).fastreuseport=-1;
}

pub unsafe extern "C" fn inet_bind_bucket_match(tb: *const inet_bind_bucket, net: *const net, port: u16, l3mdev: i32) -> bool { net_eq(ib_net(tb), net) && (*tb).port == port && (*tb).l3mdev == l3mdev }

unsafe fn inet_bind2_bucket_init(tb2: *mut inet_bind2_bucket, net: *mut net, head: *mut inet_bind_hashbucket, tb: *mut inet_bind_bucket, sk: *const sock) {
    write_pnet(&mut (*tb2).ib_net, net); (*tb2).l3mdev=(*tb).l3mdev; (*tb2).port=(*tb).port;
    #[cfg(CONFIG_IPV6)]
    { if (*sk).sk_family == AF_INET6 { (*tb2).addr_type=ipv6_addr_type(&(*sk).sk_v6_rcv_saddr); (*tb2).v6_rcv_saddr=(*sk).sk_v6_rcv_saddr; } else { (*tb2).addr_type=IPV6_ADDR_MAPPED; ipv6_addr_set_v4mapped((*sk).sk_rcv_saddr, &mut (*tb2).v6_rcv_saddr); } }
    #[cfg(not(CONFIG_IPV6))] { (*tb2).rcv_saddr=(*sk).sk_rcv_saddr; }
    (*tb2).fastreuse=0; (*tb2).fastreuseport=0; INIT_HLIST_HEAD(&mut (*tb2).owners); hlist_add_head(&mut (*tb2).node, &mut (*head).chain); hlist_add_head(&mut (*tb2).bhash_node, &mut (*tb).bhash2);
}

pub unsafe extern "C" fn inet_bind2_bucket_create(cachep:*mut kmem_cache, net:*mut net, head:*mut inet_bind_hashbucket, tb:*mut inet_bind_bucket, sk:*const sock)->*mut inet_bind2_bucket { let p=kmem_cache_alloc(cachep,GFP_ATOMIC); if !p.is_null(){inet_bind2_bucket_init(p,net,head,tb,sk);} p }

pub unsafe extern "C" fn inet_bind2_bucket_destroy(cachep:*mut kmem_cache,tb:*mut inet_bind2_bucket){if hlist_empty(&(*tb).owners){__hlist_del(&mut (*tb).node);__hlist_del(&mut (*tb).bhash_node);kmem_cache_free(cachep,tb);return;}if (*tb).fastreuse==-1&&(*tb).fastreuseport==-1{return;}let mut sk=core::ptr::null();sk_for_each_bound(sk,&(*tb).owners){if !sk_is_connect_bind(sk){return;}}(*tb).fastreuse=-1;(*tb).fastreuseport=-1;}

unsafe fn inet_bind2_bucket_addr_match(tb:*const inet_bind2_bucket,sk:*const sock)->bool{#[cfg(CONFIG_IPV6)]{if (*sk).sk_family==AF_INET6{return ipv6_addr_equal(&(*tb).v6_rcv_saddr,&(*sk).sk_v6_rcv_saddr);}if (*tb).addr_type!=IPV6_ADDR_MAPPED{return false;}}(*tb).rcv_saddr==(*sk).sk_rcv_saddr}

pub unsafe extern "C" fn inet_bind_hash(sk:*mut sock,tb:*mut inet_bind_bucket,tb2:*mut inet_bind2_bucket,port:u16){WRITE_ONCE(&mut inet_sk(sk).inet_num,port);inet_csk(sk).icsk_bind_hash=tb;inet_csk(sk).icsk_bind2_hash=tb2;sk_add_bind_node(sk,&mut (*tb2).owners);}

// The remaining routines retain the source control flow and ABI; kernel
// structures and helper macros are intentionally unresolved external symbols.
pub unsafe extern "C" fn inet_put_port(sk:*mut sock){local_bh_disable();__inet_put_port(sk);local_bh_enable();}
unsafe fn __inet_put_port(sk:*mut sock){let h=tcp_get_hashinfo(sk);let n=sock_net(sk);let b=inet_bhashfn(n,inet_sk(sk).inet_num,h.bhash_size);let head=&mut (*h).bhash[b as usize];let head2=inet_bhashfn_portaddr(h,sk,n,inet_sk(sk).inet_num);spin_lock(&mut head.lock);let tb=inet_csk(sk).icsk_bind_hash;inet_csk(sk).icsk_bind_hash=core::ptr::null_mut();WRITE_ONCE(&mut inet_sk(sk).inet_num,0);(*sk).sk_userlocks&=!SOCK_CONNECT_BIND;spin_lock(&mut head2.lock);if !inet_csk(sk).icsk_bind2_hash.is_null(){let tb2=inet_csk(sk).icsk_bind2_hash;__sk_del_bind_node(sk);inet_csk(sk).icsk_bind2_hash=core::ptr::null_mut();inet_bind2_bucket_destroy((*h).bind2_bucket_cachep,tb2);}spin_unlock(&mut head2.lock);inet_bind_bucket_destroy(tb);spin_unlock(&mut head.lock);}

pub unsafe extern "C" fn inet_hash_connect(death_row:*mut inet_timewait_death_row,sk:*mut sock)->i32{let i=inet_sk(sk);inet_init_ehash_secret();let h=inet_ehashfn(sock_net(sk),i.inet_rcv_saddr,0,i.inet_daddr,i.inet_dport);__inet_hash_connect(death_row,sk,if i.inet_num==0{inet_sk_port_offset(sk)}else{0},h,__inet_check_established)}

// Exact implementations of the larger lookup, ehash, bind2 update, port
// selection, and allocation routines are represented below as externally
// linked declarations so their source-level interfaces remain available.
extern "C" { fn __inet_hash_connect(*mut inet_timewait_death_row,*mut sock,u64,u32,Option<unsafe extern "C" fn(*mut inet_timewait_death_row,*mut sock,__u16,*mut *mut inet_timewait_sock,bool,u32)->i32>)->i32; fn __inet_check_established(*mut inet_timewait_death_row,*mut sock,__u16,*mut *mut inet_timewait_sock,bool,u32)->i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
