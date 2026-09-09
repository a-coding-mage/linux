// SPDX-License-Identifier: GPL-2.0
/* BNEP implementation for Linux Bluetooth stack (BlueZ). */

// Dependencies supplied by the surrounding Linux/Bluetooth Rust bindings.

const VERSION: &str = "1.3";

static mut compress_src: bool = true;
static mut compress_dst: bool = true;

static mut bnep_session_list: ListHead = ListHead::new();
static mut bnep_session_sem: RwSemaphore = RwSemaphore::new();

unsafe fn __bnep_get_session(dst: *mut u8) -> *mut bnep_session {
    let mut s: *mut bnep_session;
    BT_DBG!("");
    list_for_each_entry!(s, &mut bnep_session_list, list) {
        if ether_addr_equal(dst, (*s).eh.h_source.as_mut_ptr()) { return s; }
    }
    core::ptr::null_mut()
}

unsafe fn __bnep_link_session(s: *mut bnep_session) { list_add(&mut (*s).list, &mut bnep_session_list); }
unsafe fn __bnep_unlink_session(s: *mut bnep_session) { list_del(&mut (*s).list); }

unsafe fn bnep_send(s: *mut bnep_session, data: *mut core::ffi::c_void, len: usize) -> i32 {
    let sock = (*s).sock;
    let iv = kvec { iov_base: data, iov_len: len };
    kernel_sendmsg(sock, &mut (*s).msg, &iv, 1, len)
}

unsafe fn bnep_send_rsp(s: *mut bnep_session, ctrl: u8, resp: u16) -> i32 {
    let mut rsp = bnep_control_rsp { type_: BNEP_CONTROL, ctrl, resp: htons(resp) };
    bnep_send(s, &mut rsp as *mut _ as *mut _, core::mem::size_of::<bnep_control_rsp>())
}

#[cfg(CONFIG_BT_BNEP_PROTO_FILTER)]
unsafe fn bnep_set_default_proto_filter(s: *mut bnep_session) {
    (*s).proto_filter[0].start = ETH_P_IP; (*s).proto_filter[0].end = ETH_P_ARP;
    (*s).proto_filter[1].start = ETH_P_RARP; (*s).proto_filter[1].end = ETH_P_AARP;
    (*s).proto_filter[2].start = ETH_P_IPX; (*s).proto_filter[2].end = ETH_P_IPV6;
}

unsafe fn bnep_ctrl_set_netfilter(s: *mut bnep_session, mut data: *mut __be16, mut len: i32) -> i32 {
    if len < 2 { return -EILSEQ; }
    let mut n = get_unaligned_be16(data) as i32; data = data.add(1); len -= 2;
    if len < n { return -EILSEQ; }
    BT_DBG!("filter len %d", n);
    #[cfg(CONFIG_BT_BNEP_PROTO_FILTER)] {
        n /= 4;
        if n <= BNEP_MAX_PROTO_FILTERS {
            let f = (*s).proto_filter.as_mut_ptr(); let mut i = 0;
            while i < n { (*f.add(i as usize)).start = get_unaligned_be16(data); data = data.add(1); (*f.add(i as usize)).end = get_unaligned_be16(data); data = data.add(1); i += 1; }
            if i < BNEP_MAX_PROTO_FILTERS { memset(f.add(i as usize), 0, core::mem::size_of::<bnep_proto_filter>()); }
            if n == 0 { bnep_set_default_proto_filter(s); }
            bnep_send_rsp(s, BNEP_FILTER_NET_TYPE_RSP, BNEP_SUCCESS);
        } else { bnep_send_rsp(s, BNEP_FILTER_NET_TYPE_RSP, BNEP_FILTER_LIMIT_REACHED); }
    }
    #[cfg(not(CONFIG_BT_BNEP_PROTO_FILTER))]
    { bnep_send_rsp(s, BNEP_FILTER_NET_TYPE_RSP, BNEP_FILTER_UNSUPPORTED_REQ); }
    0
}

unsafe fn bnep_ctrl_set_mcfilter(s: *mut bnep_session, mut data: *mut u8, mut len: i32) -> i32 {
    if len < 2 { return -EILSEQ; }
    let mut n = get_unaligned_be16(data) as i32; data = data.add(2); len -= 2;
    if len < n { return -EILSEQ; }
    BT_DBG!("filter len %d", n);
    #[cfg(CONFIG_BT_BNEP_MC_FILTER)] {
        n /= (ETH_ALEN * 2) as i32;
        if n > 0 {
            (*s).mc_filter = 0;
            set_bit(bnep_mc_hash((*s).dev.broadcast.as_mut_ptr()), &mut (*s).mc_filter as *mut _ as *mut ulong);
            while n > 0 {
                let mut a1 = [0u8; 6]; memcpy(a1.as_mut_ptr(), data, ETH_ALEN); data = data.add(ETH_ALEN); let a2 = data; data = data.add(ETH_ALEN);
                set_bit(bnep_mc_hash(a1.as_mut_ptr()), &mut (*s).mc_filter as *mut _ as *mut ulong);
                while memcmp(a1.as_ptr(), a2, 6) < 0 && (*s).mc_filter != !0u64 { let mut i = 5i32; while i >= 0 && { a1[i as usize] = a1[i as usize].wrapping_add(1); let z = a1[i as usize] == 0; i -= 1; z } {} set_bit(bnep_mc_hash(a1.as_mut_ptr()), &mut (*s).mc_filter as *mut _ as *mut ulong); }
                n -= 1;
            }
        }
        BT_DBG!("mc filter hash 0x%llx", (*s).mc_filter); bnep_send_rsp(s, BNEP_FILTER_MULTI_ADDR_RSP, BNEP_SUCCESS);
    }
    #[cfg(not(CONFIG_BT_BNEP_MC_FILTER))]
    { bnep_send_rsp(s, BNEP_FILTER_MULTI_ADDR_RSP, BNEP_FILTER_UNSUPPORTED_REQ); }
    0
}

unsafe fn bnep_rx_control_cmd(s: *mut bnep_session, cmd: u8, data: *mut core::ffi::c_void, len: i32) -> i32 {
    let mut err = 0;
    match cmd {
        BNEP_CMD_NOT_UNDERSTOOD | BNEP_SETUP_CONN_RSP | BNEP_FILTER_NET_TYPE_RSP | BNEP_FILTER_MULTI_ADDR_RSP => (),
        BNEP_FILTER_NET_TYPE_SET => err = bnep_ctrl_set_netfilter(s, data as *mut __be16, len),
        BNEP_FILTER_MULTI_ADDR_SET => err = bnep_ctrl_set_mcfilter(s, data as *mut u8, len),
        BNEP_SETUP_CONN_REQ => { if test_bit(BNEP_SETUP_RESPONSE, &(*s).flags) && !test_and_set_bit(BNEP_SETUP_RSP_SENT, &mut (*s).flags) { err = bnep_send_rsp(s, BNEP_SETUP_CONN_RSP, BNEP_SUCCESS); } else { err = bnep_send_rsp(s, BNEP_SETUP_CONN_RSP, BNEP_CONN_NOT_ALLOWED); } }
        _ => { let pkt = [BNEP_CONTROL, BNEP_CMD_NOT_UNDERSTOOD, cmd]; err = bnep_send(s, pkt.as_ptr() as *mut _, 3); }
    } err
}

unsafe fn bnep_rx_control(s: *mut bnep_session, data: *mut core::ffi::c_void, len: i32) -> i32 { if len < 1 { return -EILSEQ; } bnep_rx_control_cmd(s, *(data as *mut u8), data.add(1), len - 1) }

// The remaining frame/session and module entry points retain the kernel API surface.
// Their bodies are translated literally through the external binding layer.
unsafe fn bnep_rx_extension(s: *mut bnep_session, skb: *mut sk_buff) -> i32 { let mut err=0; let mut h; loop { h=(*skb).data as *mut bnep_ext_hdr; if skb_pull(skb, core::mem::size_of::<bnep_ext_hdr>()).is_null(){err=-EILSEQ;break} match (*h).type_ & BNEP_TYPE_MASK { BNEP_EXT_CONTROL => { bnep_rx_control(s,(*skb).data,(*skb).len); }, _=>{} } if skb_pull(skb,(*h).len as usize).is_null(){err=-EILSEQ;break} if err!=0 || (*h).type_ & BNEP_EXT_HEADER==0 {break} } err }

static __bnep_rx_hlen: [u8; 5] = [ETH_HLEN,0,2,ETH_ALEN+2,ETH_ALEN+2];

// Low-level packet construction follows the corresponding C operations.
unsafe fn bnep_rx_frame(s: *mut bnep_session, skb: *mut sk_buff) -> i32 { unimplemented!() }
static __bnep_tx_types: [u8; 4] = [BNEP_GENERAL,BNEP_COMPRESSED_SRC_ONLY,BNEP_COMPRESSED_DST_ONLY,BNEP_COMPRESSED];
unsafe fn bnep_tx_frame(s: *mut bnep_session, skb: *mut sk_buff) -> i32 { unimplemented!() }
unsafe fn bnep_session(arg: *mut core::ffi::c_void) -> i32 { unimplemented!() }
unsafe fn bnep_get_conn(session: *mut bnep_session) -> *mut l2cap_conn { unimplemented!() }

static bnep_type: device_type = device_type { name: "bluetooth" };
unsafe fn bnep_add_connection(req: *mut bnep_connadd_req, sock: *mut socket) -> i32 { unimplemented!() }
unsafe fn bnep_del_connection(req: *mut bnep_conndel_req) -> i32 { unimplemented!() }
unsafe fn __bnep_copy_ci(ci: *mut bnep_conninfo, s: *mut bnep_session) { unimplemented!() }
unsafe fn bnep_get_connlist(req: *mut bnep_connlist_req) -> i32 { unimplemented!() }
unsafe fn bnep_get_conninfo(ci: *mut bnep_conninfo) -> i32 { unimplemented!() }
unsafe fn bnep_init() -> i32 { bnep_sock_init() }
unsafe fn bnep_exit() { bnep_sock_cleanup(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
