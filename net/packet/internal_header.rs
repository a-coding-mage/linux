/* SPDX-License-Identifier: GPL-2.0 */

/* Translation of the C header.  Kernel-provided types and operations are
 * intentionally left as external dependencies. */

pub const DEFAULT_PRB_RETIRE_TOV: u32 = 8;
pub const PACKET_FANOUT_MAX: u32 = 1 << 16;

#[repr(C)]
pub struct packet_mclist {
    pub next: *mut packet_mclist,
    pub ifindex: libc::c_int,
    pub count: libc::c_int,
    pub type_: u16,
    pub alen: u16,
    pub addr: [u8; MAX_ADDR_LEN],
    pub remove_list: list_head,
}

#[repr(C)]
pub struct tpacket_kbdq_core {
    pub pkbdq: *mut pgv,
    pub feature_req_word: libc::c_uint,
    pub hdrlen: libc::c_uint,
    pub reset_pending_on_curr_blk: u8,
    pub kactive_blk_num: u16,
    pub blk_sizeof_priv: u16,
    pub version: u16,
    pub pkblk_start: *mut libc::c_char,
    pub pkblk_end: *mut libc::c_char,
    pub kblk_size: libc::c_int,
    pub max_frame_len: libc::c_uint,
    pub knum_blocks: libc::c_uint,
    pub knxt_seq_num: u64,
    pub prev: *mut libc::c_char,
    pub nxt_offset: *mut libc::c_char,
    pub skb: *mut sk_buff,
    pub blk_fill_in_prog_lock: rwlock_t,
    pub interval_ktime: ktime_t,
    pub retire_blk_timer: hrtimer,
}

#[repr(C)]
pub struct pgv {
    pub buffer: *mut libc::c_char,
}

#[repr(C)]
pub union packet_ring_buffer__bindgen_ty_1 {
    pub rx_owner_map: *mut libc::c_ulong,
    pub prb_bdqc: tpacket_kbdq_core,
}

#[repr(C)]
pub struct packet_ring_buffer {
    pub pg_vec: *mut pgv,
    pub head: libc::c_uint,
    pub frames_per_block: libc::c_uint,
    pub frame_size: libc::c_uint,
    pub frame_max: libc::c_uint,
    pub pg_vec_order: libc::c_uint,
    pub pg_vec_pages: libc::c_uint,
    pub pg_vec_len: libc::c_uint,
    pub pending_refcnt: *mut libc::c_uint,
    pub bindgen_union: packet_ring_buffer__bindgen_ty_1,
}

extern "C" {
    pub static mut fanout_mutex: mutex;
}

#[repr(C)]
pub union packet_fanout__bindgen_ty_1 {
    pub rr_cur: atomic_t,
    pub bpf_prog: *mut bpf_prog,
}

#[repr(C)]
pub struct packet_fanout {
    pub net: possible_net_t,
    pub num_members: libc::c_uint,
    pub max_num_members: u32,
    pub id: u16,
    pub type_: u8,
    pub flags: u8,
    pub bindgen_union: packet_fanout__bindgen_ty_1,
    pub list: list_head,
    pub lock: spinlock_t,
    pub sk_ref: refcount_t,
    pub prot_hook: packet_type,
    pub arr: *mut *mut sock,
}

#[repr(C)]
pub struct packet_rollover {
    pub sock: libc::c_int,
    pub num: atomic_long_t,
    pub num_huge: atomic_long_t,
    pub num_failed: atomic_long_t,
    pub history: [u32; ROLLOVER_HLEN],
}

pub const ROLLOVER_HLEN: usize = L1_CACHE_BYTES / core::mem::size_of::<u32>();

#[repr(C)]
pub struct packet_sock {
    pub sk: sock,
    pub fanout: *mut packet_fanout,
    pub stats: tpacket_stats_u,
    pub rx_ring: packet_ring_buffer,
    pub tx_ring: packet_ring_buffer,
    pub copy_thresh: libc::c_int,
    pub bind_lock: spinlock_t,
    pub pg_vec_lock: mutex,
    pub flags: libc::c_ulong,
    pub ifindex: libc::c_int,
    pub vnet_hdr_sz: u8,
    pub num: __be16,
    pub rollover: *mut packet_rollover,
    pub mclist: *mut packet_mclist,
    pub mapped: atomic_long_t,
    pub tp_version: tpacket_versions,
    pub tp_hdrlen: libc::c_uint,
    pub tp_reserve: libc::c_uint,
    pub tp_tstamp: libc::c_uint,
    pub skb_completion: completion,
    pub cached_dev: *mut net_device,
    pub prot_hook: packet_type,
    pub tp_drops: atomic_t,
}

#[repr(u32)]
pub enum packet_sock_flags {
    PACKET_SOCK_ORIGDEV,
    PACKET_SOCK_AUXDATA,
    PACKET_SOCK_TX_HAS_OFF,
    PACKET_SOCK_TP_LOSS,
    PACKET_SOCK_RUNNING,
    PACKET_SOCK_PRESSURE,
    PACKET_SOCK_QDISC_BYPASS,
}

/* pkt_sk(ptr) is the C container_of_const conversion from sock to packet_sock. */
#[inline]
pub unsafe fn pkt_sk(ptr: *const sock) -> *const packet_sock {
    (ptr as *const u8).sub(core::mem::offset_of!(packet_sock, sk)) as *const packet_sock
}

#[inline]
pub unsafe fn packet_sock_flag_set(po: *mut packet_sock, flag: packet_sock_flags, val: bool) {
    if val {
        set_bit(flag as usize, &mut (*po).flags);
    } else {
        clear_bit(flag as usize, &mut (*po).flags);
    }
}

#[inline]
pub unsafe fn packet_sock_flag(po: *const packet_sock, flag: packet_sock_flags) -> bool {
    test_bit(flag as usize, &(*po).flags)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
