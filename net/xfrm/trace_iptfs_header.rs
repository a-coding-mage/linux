/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of xfrm_trace_iptfs.h. */

// TRACE_SYSTEM iptfs
// The Linux tracepoint headers and the symbols supplied by them are external
// dependencies of this translation.

use core::ffi::c_void;

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xfrm_iptfs_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct iphdr {
    _private: [u8; 0],
}

// TP_STRUCT__entry for iptfs_egress_recv.
#[repr(C)]
pub struct IptfsEgressRecvEntry {
    pub skb: *mut sk_buff,
    pub head: *mut c_void,
    pub head_pg_addr: *mut c_void,
    pub pg0addr: *mut c_void,
    pub skb_len: u32,
    pub data_len: u32,
    pub headroom: u32,
    pub tailroom: u32,
    pub tail: u32,
    pub end: u32,
    pub pg0off: u32,
    pub head_frag: u8,
    pub frag_list: u8,
    pub nr_frags: u8,
    pub blkoff: u16,
}

// TP_PROTO(struct sk_buff *skb, struct xfrm_iptfs_data *xtfs, u16 blkoff)
// TP_fast_assign and TP_printk:
// "EGRESS: skb=%p len=%u data_len=%u headroom=%u head_frag=%u frag_list=%u nr_frags=%u blkoff=%u\n\t\ttailroom=%u tail=%u end=%u head=%p hdpgaddr=%p pg0->addr=%p pg0->data=%p pg0->off=%u"
pub const IPTFS_EGRESS_RECV: &str = "iptfs_egress_recv";

// TP_STRUCT__entry for iptfs_ingress_preq_event.
#[repr(C)]
pub struct IptfsIngressPreqEventEntry {
    pub skb: *mut sk_buff,
    pub skb_len: u32,
    pub data_len: u32,
    pub pmtu: u32,
    pub queue_size: u32,
    pub proto_seq: u32,
    pub proto: u8,
    pub was_gso: u8,
}

// TP_PROTO(struct sk_buff *skb, struct xfrm_iptfs_data *xtfs, u32 pmtu, u8 was_gso)
// queue_size = xtfs->cfg.max_queue_size - xtfs->queue_size;
// proto = __trace_ip_proto(ip_hdr(skb));
// proto_seq = __trace_ip_proto_seq(ip_hdr(skb));
// TP_printk("INGRPREQ: skb=%p len=%u data_len=%u qsize=%u proto=%u proto_seq=%u pmtu=%u was_gso=%u")
pub const IPTFS_INGRESS_PREQ_EVENT: &str = "iptfs_ingress_preq_event";
pub const IPTFS_ENQUEUE: &str = "iptfs_enqueue";
pub const IPTFS_NO_QUEUE_SPACE: &str = "iptfs_no_queue_space";
pub const IPTFS_TOO_BIG: &str = "iptfs_too_big";

// TP_STRUCT__entry for iptfs_ingress_postq_event.
#[repr(C)]
pub struct IptfsIngressPostqEventEntry {
    pub skb: *mut sk_buff,
    pub skb_len: u32,
    pub data_len: u32,
    pub mtu: u32,
    pub proto_seq: u32,
    pub blkoff: u16,
    pub proto: u8,
}

// TP_PROTO(struct sk_buff *skb, u32 mtu, u16 blkoff, struct iphdr *iph)
// proto = iph ? __trace_ip_proto(iph) : 0;
// proto_seq = iph ? __trace_ip_proto_seq(iph) : 0;
// TP_printk("INGRPSTQ: skb=%p len=%u data_len=%u mtu=%u blkoff=%u proto=%u proto_seq=%u")
pub const IPTFS_INGRESS_POSTQ_EVENT: &str = "iptfs_ingress_postq_event";
pub const IPTFS_FIRST_DEQUEUE: &str = "iptfs_first_dequeue";
pub const IPTFS_FIRST_FRAGMENTING: &str = "iptfs_first_fragmenting";
pub const IPTFS_FIRST_FINAL_FRAGMENT: &str = "iptfs_first_final_fragment";
pub const IPTFS_FIRST_TOOBIG: &str = "iptfs_first_toobig";

// TP_STRUCT__entry for iptfs_ingress_nth_peek.
#[repr(C)]
pub struct IptfsIngressNthPeekEntry {
    pub skb: *mut sk_buff,
    pub skb_len: u32,
    pub remaining: u32,
}

// TP_PROTO(struct sk_buff *skb, u32 remaining)
// TP_printk("INGRPSTQ: NTHPEEK: skb=%p len=%u remaining=%u")
pub const IPTFS_INGRESS_NTH_PEEK: &str = "iptfs_ingress_nth_peek";

// TP_STRUCT__entry for iptfs_ingress_nth_add.
#[repr(C)]
pub struct IptfsIngressNthAddEntry {
    pub skb: *mut sk_buff,
    pub skb_len: u32,
    pub data_len: u32,
    pub share_ok: u8,
    pub head_frag: u8,
    pub pp_recycle: u8,
    pub cloned: u8,
    pub shared: u8,
    pub nr_frags: u8,
    pub frag_list: u8,
}

// TP_PROTO(struct sk_buff *skb, u8 share_ok)
// TP_fast_assign records skb->len, skb->data_len, skb->head_frag,
// skb->pp_recycle, skb_cloned(skb), skb_shared(skb), skb_shinfo(skb)->nr_frags,
// and (bool)skb_shinfo(skb)->frag_list.
// TP_printk("INGRPSTQ: NTHADD: skb=%p len=%u data_len=%u share_ok=%u head_frag=%u pp_recycle=%u cloned=%u shared=%u nr_frags=%u frag_list=%u")
pub const IPTFS_INGRESS_NTH_ADD: &str = "iptfs_ingress_nth_add";

// TP_STRUCT__entry for iptfs_timer_event.
#[repr(C)]
pub struct IptfsTimerEventEntry {
    pub time_val: u64,
    pub set_time: u64,
}

// TP_PROTO(struct xfrm_iptfs_data *xtfs, u64 time_val)
// TP_fast_assign: set_time = xtfs->iptfs_settime;
// TP_printk("TIMER: set_time=%llu time_val=%llu")
pub const IPTFS_TIMER_EVENT: &str = "iptfs_timer_event";
pub const IPTFS_TIMER_START: &str = "iptfs_timer_start";
pub const IPTFS_TIMER_EXPIRE: &str = "iptfs_timer_expire";

// TRACE_INCLUDE_PATH ../../net/xfrm
// TRACE_INCLUDE_FILE trace_iptfs
// <trace/define_trace.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
