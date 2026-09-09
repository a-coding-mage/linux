/* SPDX-License-Identifier: GPL-2.0-only */

// C header dependencies are supplied by the surrounding kernel translation.

extern "C" {
    pub fn psp_dev_create(netdev: *mut net_device, psd_ops: *mut psp_dev_ops,
                          psd_caps: *mut psp_dev_caps, priv_ptr: *mut core::ffi::c_void)
        -> *mut psp_dev;
    pub fn psp_dev_unregister(psd: *mut psp_dev);
    pub fn psp_dev_encapsulate(net: *mut net, skb: *mut sk_buff, spi: __be32,
                               ver: u8, sport: __be16) -> bool;
    pub fn psp_dev_rcv(skb: *mut sk_buff, dev_id: u16, generation: u8,
                       strip_icv: bool) -> i32;
    pub fn psp_assoc_put(pas: *mut psp_assoc);
}

#[inline]
pub unsafe fn psp_assoc_drv_data(pas: *mut psp_assoc) -> *mut core::ffi::c_void {
    (*pas).drv_data
}

// The CONFIG_INET_PSP branch is selected by the kernel build configuration.
#[cfg(feature = "CONFIG_INET_PSP")]
extern "C" {
    pub fn psp_key_size(version: u32) -> u32;
    pub fn psp_sk_assoc_free(sk: *mut sock);
    pub fn psp_twsk_init(tw: *mut inet_timewait_sock, sk: *const sock);
    pub fn psp_twsk_assoc_free(tw: *mut inet_timewait_sock);
    pub fn psp_reply_set_decrypted(sk: *const sock, skb: *mut sk_buff);
}

#[cfg(feature = "CONFIG_INET_PSP")]
#[inline]
pub unsafe fn psp_sk_assoc(sk: *const sock) -> *mut psp_assoc {
    rcu_dereference_check((*sk).psp_assoc, lockdep_sock_is_held(sk))
}

#[cfg(feature = "CONFIG_INET_PSP")]
#[inline]
pub unsafe fn psp_enqueue_set_decrypted(sk: *mut sock, skb: *mut sk_buff) {
    let pas = psp_sk_assoc(sk);
    if !pas.is_null() && (*pas).tx.spi != 0 {
        (*skb).decrypted = 1;
    }
}

#[cfg(feature = "CONFIG_INET_PSP")]
#[inline]
pub unsafe fn __psp_skb_coalesce_diff(one: *const sk_buff, two: *const sk_buff,
                                      mut diffs: usize) -> usize {
    let a = skb_ext_find(one, SKB_EXT_PSP);
    let b = skb_ext_find(two, SKB_EXT_PSP);
    diffs |= (!a.is_null() as usize) ^ (!b.is_null() as usize);
    if diffs == 0 && !a.is_null() {
        diffs |= memcmp(a, b, core::mem::size_of::<psp_skb_ext>());
    }
    diffs
}

#[cfg(feature = "CONFIG_INET_PSP")]
#[inline]
pub unsafe fn psp_is_allowed_nondata(skb: *mut sk_buff, pas: *mut psp_assoc) -> bool {
    let fin = ((TCP_SKB_CB(skb).tcp_flags & TCPHDR_FIN) != 0);
    let end_seq = TCP_SKB_CB(skb).end_seq;
    let seq = TCP_SKB_CB(skb).seq;
    let pure_fin = fin && end_seq.wrapping_sub(seq) == 1;
    seq == end_seq || (pure_fin && seq == (*pas).upgrade_seq)
}

#[cfg(feature = "CONFIG_INET_PSP")]
#[inline]
pub unsafe fn psp_pse_matches_pas(pse: *mut psp_skb_ext, pas: *mut psp_assoc) -> bool {
    !pse.is_null() && (*pas).rx.spi == (*pse).spi &&
        (*pas).generation == (*pse).generation && (*pas).version == (*pse).version &&
        (*pas).dev_id == (*pse).dev_id
}

#[cfg(feature = "CONFIG_INET_PSP")]
#[inline]
pub unsafe fn __psp_sk_rx_policy_check(skb: *mut sk_buff, pas: *mut psp_assoc) -> skb_drop_reason {
    let pse = skb_ext_find(skb, SKB_EXT_PSP);
    if pas.is_null() { return if !pse.is_null() { SKB_DROP_REASON_PSP_INPUT } else { 0 }; }
    if psp_pse_matches_pas(pse, pas) {
        if (*pas).peer_tx == 0 { (*pas).peer_tx = 1; }
        return 0;
    }
    if pse.is_null() && ((*pas).tx.spi == 0 || ((*pas).peer_tx == 0 && psp_is_allowed_nondata(skb, pas))) { return 0; }
    SKB_DROP_REASON_PSP_INPUT
}

#[cfg(feature = "CONFIG_INET_PSP")]
#[inline] pub unsafe fn psp_sk_rx_policy_check(sk: *mut sock, skb: *mut sk_buff) -> skb_drop_reason { __psp_sk_rx_policy_check(skb, psp_sk_assoc(sk)) }
#[cfg(feature = "CONFIG_INET_PSP")]
#[inline] pub unsafe fn psp_twsk_rx_policy_check(tw: *mut inet_timewait_sock, skb: *mut sk_buff) -> skb_drop_reason { __psp_sk_rx_policy_check(skb, rcu_dereference((*tw).psp_assoc)) }
#[cfg(feature = "CONFIG_INET_PSP")]
#[inline] pub unsafe fn psp_sk_get_assoc_rcu(sk: *const sock) -> *mut psp_assoc {
    let state = READ_ONCE((*sk).sk_state);
    if !sk_is_inet(sk) || state == TCP_NEW_SYN_RECV { return core::ptr::null_mut(); }
    if state == TCP_TIME_WAIT { rcu_dereference((*inet_twsk(sk)).psp_assoc) } else { rcu_dereference((*sk).psp_assoc) }
}
#[cfg(feature = "CONFIG_INET_PSP")]
#[inline] pub unsafe fn psp_skb_get_assoc_rcu(skb: *mut sk_buff) -> *mut psp_assoc { if (*skb).decrypted == 0 || (*skb).sk.is_null() { core::ptr::null_mut() } else { psp_sk_get_assoc_rcu((*skb).sk) } }
#[cfg(feature = "CONFIG_INET_PSP")]
#[inline] pub unsafe fn psp_sk_overhead(sk: *const sock) -> u32 { if !rcu_access_pointer((*sk).psp_assoc).is_null() { (core::mem::size_of::<udphdr>() + PSP_HDR_SIZE + PSP_TRL_SIZE) as u32 } else { 0 } }

#[cfg(not(feature = "CONFIG_INET_PSP"))]
#[inline] pub unsafe fn psp_sk_assoc_free(_: *mut sock) {}
#[cfg(not(feature = "CONFIG_INET_PSP"))]
#[inline] pub unsafe fn psp_twsk_init(_: *mut inet_timewait_sock, _: *const sock) {}
#[cfg(not(feature = "CONFIG_INET_PSP"))]
#[inline] pub unsafe fn psp_twsk_assoc_free(_: *mut inet_timewait_sock) {}
#[cfg(not(feature = "CONFIG_INET_PSP"))]
#[inline] pub unsafe fn psp_reply_set_decrypted(_: *const sock, _: *mut sk_buff) {}
#[cfg(not(feature = "CONFIG_INET_PSP"))]
#[inline] pub unsafe fn psp_sk_assoc(_: *const sock) -> *mut psp_assoc { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_INET_PSP"))]
#[inline] pub unsafe fn psp_enqueue_set_decrypted(_: *mut sock, _: *mut sk_buff) {}
#[cfg(not(feature = "CONFIG_INET_PSP"))]
#[inline] pub unsafe fn __psp_skb_coalesce_diff(_: *const sk_buff, _: *const sk_buff, diffs: usize) -> usize { diffs }
#[cfg(not(feature = "CONFIG_INET_PSP"))]
#[inline] pub unsafe fn psp_sk_rx_policy_check(_: *mut sock, _: *mut sk_buff) -> skb_drop_reason { 0 }
#[cfg(not(feature = "CONFIG_INET_PSP"))]
#[inline] pub unsafe fn psp_twsk_rx_policy_check(_: *mut inet_timewait_sock, _: *mut sk_buff) -> skb_drop_reason { 0 }
#[cfg(not(feature = "CONFIG_INET_PSP"))]
#[inline] pub unsafe fn psp_skb_get_assoc_rcu(_: *mut sk_buff) -> *mut psp_assoc { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_INET_PSP"))]
#[inline] pub unsafe fn psp_sk_overhead(_: *const sock) -> u32 { 0 }

#[inline]
pub unsafe fn psp_skb_coalesce_diff(one: *const sk_buff, two: *const sk_buff) -> usize {
    __psp_skb_coalesce_diff(one, two, 0)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
