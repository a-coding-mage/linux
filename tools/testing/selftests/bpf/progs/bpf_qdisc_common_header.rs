/* SPDX-License-Identifier: GPL-2.0 */

pub const NET_XMIT_SUCCESS: u32 = 0x00;
pub const NET_XMIT_DROP: u32 = 0x01; /* skb dropped                  */
pub const NET_XMIT_CN: u32 = 0x02; /* congestion notification      */

pub const TC_PRIO_CONTROL: u32 = 7;
pub const TC_PRIO_MAX: u32 = 15;

/* C macro:
 * #define private(name) SEC(".data." #name) __hidden __attribute__((aligned(8)))
 *
 * This is a declaration attribute helper in C. Apply the equivalent Rust
 * attributes at each translated use site when the named data section is known.
 */

/* Forward declaration in C:
 * struct bpf_sk_buff_ptr;
 */

#[inline]
pub unsafe fn qdisc_skb_cb(skb: *const sk_buff) -> *mut qdisc_skb_cb {
    unsafe { core::ptr::addr_of!((*skb).cb) as *mut qdisc_skb_cb }
}

#[inline]
pub unsafe fn qdisc_pkt_len(skb: *const sk_buff) -> u32 {
    unsafe { (*qdisc_skb_cb(skb)).pkt_len }
}
