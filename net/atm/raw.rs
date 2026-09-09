// SPDX-License-Identifier: GPL-2.0
/* net/atm/raw.c - Raw AAL0 and AAL5 transports */

/* Written 1995-2000 by Werner Almesberger, EPFL LRC/ICA */

use core::ffi::c_int;

#[repr(C)]
pub struct atm_vcc {
    pub vpi: u32,
    pub vci: u32,
    pub push: Option<unsafe extern "C" fn(*mut atm_vcc, *mut sk_buff)>,
    pub pop: Option<unsafe extern "C" fn(*mut atm_vcc, *mut sk_buff)>,
    pub send: Option<unsafe extern "C" fn(*mut atm_vcc, *mut sk_buff) -> c_int>,
    pub dev: *mut atm_dev,
}

#[repr(C)]
pub struct sk_buff {
    pub data: *mut u8,
}

#[repr(C)]
pub struct sock {
    pub sk_receive_queue: sk_buff_head,
    pub sk_data_ready: Option<unsafe extern "C" fn(*mut sock)>,
    pub sk_write_space: Option<unsafe extern "C" fn(*mut sock)>,
}

#[repr(C)]
pub struct sk_buff_head {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct atm_dev {
    pub ops: *mut atm_dev_ops,
}

#[repr(C)]
pub struct atm_dev_ops {
    pub send: Option<unsafe extern "C" fn(*mut atm_vcc, *mut sk_buff) -> c_int>,
}

const CAP_NET_ADMIN: c_int = 12;
const ATM_HDR_VPI_MASK: u32 = 0x0f000000;
const ATM_HDR_VCI_MASK: u32 = 0x00fff000;
const ATM_HDR_VPI_SHIFT: u32 = 20;
const ATM_HDR_VCI_SHIFT: u32 = 4;
const EADDRNOTAVAIL: c_int = 99;

extern "C" {
    fn capable(cap: c_int) -> bool;
    fn sk_atm(vcc: *mut atm_vcc) -> *mut sock;
    fn skb_queue_tail(queue: *mut sk_buff_head, skb: *mut sk_buff);
    fn atm_return_tx(vcc: *mut atm_vcc, skb: *mut sk_buff);
    fn dev_kfree_skb_any(skb: *mut sk_buff);
    fn kfree_skb(skb: *mut sk_buff);
    fn sk_wmem_alloc_get(sk: *mut sock) -> c_int;
    fn atm_skb_acct_truesize(skb: *mut sk_buff) -> c_int;
}

/*
 * SKB == NULL indicates that the link is being closed
 */

unsafe extern "C" fn atm_push_raw(vcc: *mut atm_vcc, skb: *mut sk_buff) {
    if !skb.is_null() {
        let sk = sk_atm(vcc);
        skb_queue_tail(&mut (*sk).sk_receive_queue, skb);
        if let Some(data_ready) = (*sk).sk_data_ready {
            data_ready(sk);
        }
    }
}

unsafe extern "C" fn atm_pop_raw(vcc: *mut atm_vcc, skb: *mut sk_buff) {
    let sk = sk_atm(vcc);
    let _ = sk_wmem_alloc_get(sk);
    let _ = atm_skb_acct_truesize(skb);
    atm_return_tx(vcc, skb);
    dev_kfree_skb_any(skb);
    if let Some(write_space) = (*sk).sk_write_space {
        write_space(sk);
    }
}

unsafe extern "C" fn atm_send_aal0(vcc: *mut atm_vcc, skb: *mut sk_buff) -> c_int {
    /*
     * Note that if vpi/vci are _ANY or _UNSPEC the below will
     * still work
     */
    if !capable(CAP_NET_ADMIN) {
        let header = *( (*skb).data as *const u32 );
        if (header & (ATM_HDR_VPI_MASK | ATM_HDR_VCI_MASK))
            != ((*vcc).vpi << ATM_HDR_VPI_SHIFT) |
               ((*vcc).vci << ATM_HDR_VCI_SHIFT)
        {
            kfree_skb(skb);
            return -EADDRNOTAVAIL;
        }
    }
    ((*(*vcc).dev).ops.as_ref().unwrap()).send.unwrap()(vcc, skb)
}

pub unsafe extern "C" fn atm_init_aal0(vcc: *mut atm_vcc) -> c_int {
    (*vcc).push = Some(atm_push_raw);
    (*vcc).pop = Some(atm_pop_raw);
    (*vcc).send = Some(atm_send_aal0);
    0
}

pub unsafe extern "C" fn atm_init_aal5(vcc: *mut atm_vcc) -> c_int {
    (*vcc).push = Some(atm_push_raw);
    (*vcc).pop = Some(atm_pop_raw);
    (*vcc).send = Some((*(*vcc).dev).ops.as_ref().unwrap().send.unwrap());
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
