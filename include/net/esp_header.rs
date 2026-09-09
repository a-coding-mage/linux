/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by linux/skbuff.h in the original header.
use core::ffi::c_int;

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ip_esp_hdr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xfrm_state {
    _private: [u8; 0],
}

extern "C" {
    fn skb_transport_header(skb: *const sk_buff) -> *mut u8;
}

#[inline]
pub unsafe fn ip_esp_hdr(skb: *const sk_buff) -> *mut ip_esp_hdr {
    skb_transport_header(skb) as *mut ip_esp_hdr
}

#[inline]
pub unsafe fn esp_output_fill_trailer(tail: *mut u8, tfclen: c_int, plen: c_int, proto: u8) {
    // Fill padding...
    let mut tail = tail;
    if tfclen != 0 {
        core::ptr::write_bytes(tail, 0, tfclen as usize);
        tail = tail.add(tfclen as usize);
    }
    loop {
        let mut i: c_int = 0;
        while i < plen - 2 {
            *tail.add(i as usize) = (i + 1) as u8;
            i += 1;
        }
        break;
    }
    *tail.add((plen - 2) as usize) = (plen - 2) as u8;
    *tail.add((plen - 1) as usize) = proto;
}

#[repr(C)]
pub struct esp_info {
    pub esph: *mut ip_esp_hdr,
    pub seqno: u64,
    pub tfclen: c_int,
    pub tailen: c_int,
    pub plen: c_int,
    pub clen: c_int,
    pub len: c_int,
    pub nfrags: c_int,
    pub proto: u8,
    pub inplace: bool,
}

extern "C" {
    pub fn esp_output_head(
        x: *mut xfrm_state,
        skb: *mut sk_buff,
        esp: *mut esp_info,
    ) -> c_int;
    pub fn esp_output_tail(
        x: *mut xfrm_state,
        skb: *mut sk_buff,
        esp: *mut esp_info,
    ) -> c_int;
    pub fn esp_input_done2(skb: *mut sk_buff, err: c_int) -> c_int;
    pub fn esp6_output_head(
        x: *mut xfrm_state,
        skb: *mut sk_buff,
        esp: *mut esp_info,
    ) -> c_int;
    pub fn esp6_output_tail(
        x: *mut xfrm_state,
        skb: *mut sk_buff,
        esp: *mut esp_info,
    ) -> c_int;
    pub fn esp6_input_done2(skb: *mut sk_buff, err: c_int) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
