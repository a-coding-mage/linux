// Translated from nf_tproxy.h.
// C include dependency: <net/tcp.h>

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum nf_tproxy_lookup_t {
    NF_TPROXY_LOOKUP_LISTENER,
    NF_TPROXY_LOOKUP_ESTABLISHED,
}

#[inline]
pub unsafe fn nf_tproxy_sk_is_transparent(sk: *mut sock) -> bool {
    if inet_sk_transparent(sk) {
        true
    } else {
        sock_gen_put(sk);
        false
    }
}

#[inline]
pub unsafe fn nf_tproxy_twsk_deschedule_put(tw: *mut inet_timewait_sock) {
    local_bh_disable();
    inet_twsk_deschedule_put(tw);
    local_bh_enable();
}

/* assign a socket to the skb -- consumes sk */
#[inline]
pub unsafe fn nf_tproxy_assign_sock(skb: *mut sk_buff, sk: *mut sock) {
    skb_orphan(skb);
    (*skb).sk = sk;
    (*skb).destructor = Some(sock_edemux);
}

extern "C" {
    pub fn nf_tproxy_laddr4(
        skb: *mut sk_buff,
        user_laddr: __be32,
        daddr: __be32,
    ) -> __be32;

    /**
     * nf_tproxy_handle_time_wait4 - handle IPv4 TCP TIME_WAIT reopen redirections
     * @net: The network namespace.
     * @skb: The skb being processed.
     * @laddr: IPv4 address to redirect to or zero.
     * @lport: TCP port to redirect to or zero.
     * @sk: The TIME_WAIT TCP socket found by the lookup.
     *
     * We have to handle SYN packets arriving to TIME_WAIT sockets
     * differently: instead of reopening the connection we should rather
     * redirect the new connection to the proxy if there's a listener
     * socket present.
     *
     * nf_tproxy_handle_time_wait4() consumes the socket reference passed in.
     *
     * Returns: the listener socket if there's one, the TIME_WAIT socket if
     * no such listener is found, or NULL if the TCP header is incomplete.
     */
    pub fn nf_tproxy_handle_time_wait4(
        net: *mut net,
        skb: *mut sk_buff,
        laddr: __be32,
        lport: __be16,
        sk: *mut sock,
    ) -> *mut sock;

    /*
     * This is used when the user wants to intercept a connection matching
     * an explicit iptables rule. In this case the sockets are assumed
     * matching in preference order:
     *
     *   - match: if there's a fully established connection matching the
     *     _packet_ tuple, it is returned, assuming the redirection
     *     already took place and we process a packet belonging to an
     *     established connection
     *
     *   - match: if there's a listening socket matching the redirection
     *     (e.g. on-port & on-ip of the connection), it is returned,
     *     regardless if it was bound to 0.0.0.0 or an explicit
     *     address. The reasoning is that if there's an explicit rule, it
     *     does not really matter if the listener is bound to an interface
     *     or to 0. The user already stated that he wants redirection
     *     (since he added the rule).
     *
     * Please note that there's an overlap between what a TPROXY target
     * and a socket match will match. Normally if you have both rules the
     * "socket" match will be the first one, effectively all packets
     * belonging to established connections going through that one.
     */
    pub fn nf_tproxy_get_sock_v4(
        net: *mut net,
        skb: *mut sk_buff,
        protocol: u8,
        saddr: __be32,
        daddr: __be32,
        sport: __be16,
        dport: __be16,
        in_: *const net_device,
        lookup_type: nf_tproxy_lookup_t,
    ) -> *mut sock;

    pub fn nf_tproxy_laddr6(
        skb: *mut sk_buff,
        user_laddr: *const in6_addr,
        daddr: *const in6_addr,
    ) -> *const in6_addr;

    /**
     * nf_tproxy_handle_time_wait6 - handle IPv6 TCP TIME_WAIT reopen redirections
     * @skb: The skb being processed.
     * @tproto: Transport protocol.
     * @thoff: Transport protocol header offset.
     * @net: Network namespace.
     * @laddr: IPv6 address to redirect to.
     * @lport: TCP port to redirect to or zero.
     * @sk: The TIME_WAIT TCP socket found by the lookup.
     *
     * We have to handle SYN packets arriving to TIME_WAIT sockets
     * differently: instead of reopening the connection we should rather
     * redirect the new connection to the proxy if there's a listener
     * socket present.
     *
     * nf_tproxy_handle_time_wait6() consumes the socket reference passed in.
     *
     * Returns: the listener socket if there's one, the TIME_WAIT socket if
     * no such listener is found, or NULL if the TCP header is incomplete.
     */
    pub fn nf_tproxy_handle_time_wait6(
        skb: *mut sk_buff,
        tproto: i32,
        thoff: i32,
        net: *mut net,
        laddr: *const in6_addr,
        lport: __be16,
        sk: *mut sock,
    ) -> *mut sock;

    pub fn nf_tproxy_get_sock_v6(
        net: *mut net,
        skb: *mut sk_buff,
        thoff: i32,
        protocol: u8,
        saddr: *const in6_addr,
        daddr: *const in6_addr,
        sport: __be16,
        dport: __be16,
        in_: *const net_device,
        lookup_type: nf_tproxy_lookup_t,
    ) -> *mut sock;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
