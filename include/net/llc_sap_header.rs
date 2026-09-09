/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Copyright (c) 1997 by Procom Technology,Inc.
 * 		 2001-2003 by Arnaldo Carvalho de Melo <acme@conectiva.com.br>
 */

/* Opaque declarations corresponding to the C forward declarations. */
#[repr(C)]
pub struct llc_sap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sock {
    _private: [u8; 0],
}

extern "C" {
    pub fn llc_sap_rtn_pdu(sap: *mut llc_sap, skb: *mut sk_buff);
    pub fn llc_save_primitive(sk: *mut sock, skb: *mut sk_buff, prim: u8);
    pub fn llc_alloc_frame(
        sk: *mut sock,
        dev: *mut net_device,
        type_: u8,
        data_size: u32,
    ) -> *mut sk_buff;

    pub fn llc_build_and_send_test_pkt(
        sap: *mut llc_sap,
        skb: *mut sk_buff,
        dmac: *mut u8,
        dsap: u8,
    );
    pub fn llc_build_and_send_xid_pkt(
        sap: *mut llc_sap,
        skb: *mut sk_buff,
        dmac: *mut u8,
        dsap: u8,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
