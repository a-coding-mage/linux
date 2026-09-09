/* SPDX-License-Identifier: GPL-2.0-only */
/* Direct Rust translation of net/libeth/xdp.h. Kernel-provided names remain external. */

#[allow(non_camel_case_types, non_snake_case, dead_code)]
pub mod libeth_xdp {
    use core::ffi::c_void;

    pub const LIBETH_XDP_PASS: u32 = 0;
    pub const LIBETH_XDP_DROP: u32 = 1 << 0;
    pub const LIBETH_XDP_ABORTED: u32 = 1 << 1;
    pub const LIBETH_XDP_TX: u32 = 1 << 2;
    pub const LIBETH_XDP_REDIRECT: u32 = 1 << 3;

    pub const LIBETH_XDP_TX_BULK: u32 = DEV_MAP_BULK_SIZE;
    pub const LIBETH_XDP_TX_BATCH: u32 = 8;
    pub const LIBETH_XDP_TX_DROP: u32 = 1 << 0;
    pub const LIBETH_XDP_TX_NDO: u32 = 1 << 1;
    pub const LIBETH_XDP_TX_XSK: u32 = 1 << 2;
    pub const LIBETH_XDP_TX_LEN: u32 = 0xffff;
    pub const LIBETH_XDP_TX_CSUM: u32 = XDP_TXMD_FLAGS_CHECKSUM;
    pub const LIBETH_XDP_TX_XSKMD: u32 = LIBETH_XDP_TX_LEN;
    pub const LIBETH_XDP_TX_FIRST: u32 = 1 << 16;
    pub const LIBETH_XDP_TX_LAST: u32 = 1 << 17;
    pub const LIBETH_XDP_TX_MULTI: u32 = 1 << 18;
    pub const LIBETH_XDP_TX_FLAGS: u32 = 0xffff0000;

    #[repr(C)]
    pub union libeth_xdp_buff_base { pub base: xdp_buff, pub data: *mut c_void }
    #[repr(C, align(16))]
    pub struct libeth_xdp_buff { pub u: libeth_xdp_buff_base, pub desc: *const c_void, pub priv_: [c_ulong; 0] }

    #[repr(C)]
    pub union libeth_xdp_tx_frame_u {
        pub tx: libeth_xdp_tx_frame_tx,
        pub frag: skb_frag_t,
        pub xmit: libeth_xdp_tx_frame_xmit,
        pub desc: xdp_desc,
    }
    #[repr(C)] pub struct libeth_xdp_tx_frame_tx { pub data: *mut c_void, pub len_fl: u32, pub soff: u32 }
    #[repr(C)] pub union libeth_xdp_tx_frame_xmit_ptr { pub xdpf: *mut xdp_frame, pub dma: dma_addr_t, pub xsk: *mut libeth_xdp_buff }
    #[repr(C)] pub union libeth_xdp_tx_frame_xmit_len { pub fields: libeth_xdp_tx_frame_len, pub opts: aligned_u64 }
    #[repr(C)] pub struct libeth_xdp_tx_frame_len { pub len: u32, pub flags: u32 }
    #[repr(C)] pub struct libeth_xdp_tx_frame_xmit { pub ptr: libeth_xdp_tx_frame_xmit_ptr, pub len: libeth_xdp_tx_frame_xmit_len }
    #[repr(C, align(16))] pub struct libeth_xdp_tx_frame { pub u: libeth_xdp_tx_frame_u }

    #[repr(C)] pub struct libeth_xdp_tx_bulk {
        pub prog: *const bpf_prog, pub dev: *mut net_device, pub xdpsq: *mut c_void,
        pub act_mask: u32, pub count: u32, pub bulk: [libeth_xdp_tx_frame; LIBETH_XDP_TX_BULK as usize],
    }
    #[repr(C)] pub struct libeth_xdpsq { pub pool: *mut xsk_buff_pool, pub sqes: *mut libeth_sqe, pub descs: *mut c_void, pub ntu: *mut u32, pub count: u32, pub pending: *mut u32, pub xdp_tx: *mut u32, pub lock: *mut libeth_xdpsq_lock }
    #[repr(C, align(16))] pub struct libeth_xdp_tx_desc { pub addr: dma_addr_t, pub u: libeth_xdp_tx_desc_u }
    #[repr(C)] pub union libeth_xdp_tx_desc_u { pub fields: libeth_xdp_tx_frame_len, pub opts: aligned_u64 }

    extern "C" {
        pub static mut libeth_xdpsq_share: static_key_false;
        pub fn __libeth_xdpsq_get(lock: *mut libeth_xdpsq_lock, dev: *const net_device);
        pub fn __libeth_xdpsq_put(lock: *mut libeth_xdpsq_lock, dev: *const net_device);
        pub fn __libeth_xdpsq_lock(lock: *mut libeth_xdpsq_lock);
        pub fn __libeth_xdpsq_unlock(lock: *mut libeth_xdpsq_lock);
        pub fn libeth_xdpsq_init_timer(timer: *mut libeth_xdpsq_timer, xdpsq: *mut c_void, lock: *mut libeth_xdpsq_lock, poll: Option<unsafe extern "C" fn(*mut work_struct)>);
        pub fn libeth_xdp_return_buff_slow(xdp: *mut libeth_xdp_buff);
        pub fn libeth_xdp_tx_exception(bq: *mut libeth_xdp_tx_bulk, sent: u32, flags: u32);
        pub fn libeth_xdp_load_stash(dst: *mut libeth_xdp_buff, src: *const libeth_xdp_buff_stash);
        pub fn libeth_xdp_save_stash(dst: *mut libeth_xdp_buff_stash, src: *const libeth_xdp_buff);
        pub fn __libeth_xdp_return_stash(stash: *mut libeth_xdp_buff_stash);
        pub fn libeth_xdp_buff_add_frag(xdp: *mut libeth_xdp_buff, fqe: *const libeth_fqe, len: u32) -> bool;
        pub fn libeth_xdp_prog_exception(bq: *const libeth_xdp_tx_bulk, xdp: *mut libeth_xdp_buff, act: xdp_action, ret: i32) -> u32;
        pub fn libeth_xdp_xmit_return_bulk(bq: *const libeth_xdp_tx_frame, count: u32, dev: *const net_device) -> u32;
        pub fn libeth_xdp_return_buff_bulk(sinfo: *const skb_shared_info, bq: *mut xdp_frame_bulk, frags: bool);
        pub fn libeth_xsk_buff_free_slow(xdp: *mut libeth_xdp_buff);
        pub fn libeth_xdp_queue_threshold(count: u32) -> u32;
        pub fn __libeth_xdp_set_features(dev: *mut net_device, xmo: *const xdp_metadata_ops, zc_segs: u32, tmo: *const xsk_tx_metadata_ops);
        pub fn libeth_xdp_set_redirect(dev: *mut net_device, enable: bool);
    }

    #[inline] pub unsafe fn libeth_xdpsq_num(rxq: u32, txq: u32, max: u32) -> u32 { core::cmp::min(nr_cpu_ids, core::cmp::max(rxq, 0)) .min(max - txq) }
    #[inline] pub unsafe fn libeth_xdpsq_shared(num: u32) -> bool { num < nr_cpu_ids }
    #[inline] pub unsafe fn libeth_xdpsq_id(num: u32) -> u32 { let mut ret = raw_smp_processor_id(); if static_branch_unlikely(&libeth_xdpsq_share) && libeth_xdpsq_shared(num) { ret %= num; } ret }
    #[inline] pub unsafe fn libeth_xdpsq_get(lock: *mut libeth_xdpsq_lock, dev: *const net_device, share: bool) { if unlikely(share) { __libeth_xdpsq_get(lock, dev) } }
    #[inline] pub unsafe fn libeth_xdpsq_put(lock: *mut libeth_xdpsq_lock, dev: *const net_device) { if static_branch_unlikely(&libeth_xdpsq_share) && (*lock).share { __libeth_xdpsq_put(lock, dev) } }
    #[inline] pub unsafe fn libeth_xdpsq_lock(lock: *mut libeth_xdpsq_lock) { if static_branch_unlikely(&libeth_xdpsq_share) && (*lock).share { __libeth_xdpsq_lock(lock) } }
    #[inline] pub unsafe fn libeth_xdpsq_unlock(lock: *mut libeth_xdpsq_lock) { if static_branch_unlikely(&libeth_xdpsq_share) && (*lock).share { __libeth_xdpsq_unlock(lock) } }

    #[macro_export] macro_rules! libeth_xdp_ptr_to_priv { ($p:expr) => { $p as u64 }; }
    #[macro_export] macro_rules! libeth_xdp_priv_to_ptr { ($p:expr) => { $p as usize as *const c_void }; }
    #[macro_export] macro_rules! libeth_xdp_return_buff { ($x:expr) => { unsafe { __libeth_xdp_return_buff($x, true) } }; }

    pub unsafe fn __libeth_xdp_return_buff(xdp: *mut libeth_xdp_buff, napi: bool) { if !xdp_buff_has_frags(&(*xdp).u.base) { } else { libeth_xdp_return_frags(xdp_get_shared_info_from_buff(&(*xdp).u.base), napi); } libeth_xdp_return_va((*xdp).u.data, napi); (*xdp).u.data = core::ptr::null_mut(); }
    pub unsafe fn libeth_xdp_return_va(data: *const c_void, napi: bool) { let n = virt_to_netmem(data); page_pool_put_full_netmem(__netmem_get_pp(n), n, napi); }
    pub unsafe fn libeth_xdp_return_frags(sinfo: *const skb_shared_info, napi: bool) { for i in 0..(*sinfo).nr_frags { let n = skb_frag_netmem(&(*sinfo).frags[i as usize]); page_pool_put_full_netmem(netmem_get_pp(n), n, napi); } }
    pub unsafe fn libeth_xdp_prep_desc(xdp: *mut libeth_xdp_buff, desc: *const c_void) { (*xdp).desc = desc; }

    pub unsafe fn libeth_xdp_init_buff(dst: *mut libeth_xdp_buff, src: *const libeth_xdp_buff_stash, rxq: *mut xdp_rxq_info) { if !(*src).data.is_null() { libeth_xdp_load_stash(dst, src) } else { (*dst).u.data = core::ptr::null_mut() } (*dst).u.base.rxq = rxq; }
    pub unsafe fn libeth_xdp_save_buff(dst: *mut libeth_xdp_buff_stash, src: *const libeth_xdp_buff) { if (*src).u.data.is_null() { (*dst).data = core::ptr::null_mut() } else { libeth_xdp_save_stash(dst, src) } }
    pub unsafe fn libeth_xdp_return_stash(stash: *mut libeth_xdp_buff_stash) { if !(*stash).data.is_null() { __libeth_xdp_return_stash(stash) } }
    pub unsafe fn libeth_xdp_buff_stats_frags(ss: *mut libeth_rq_napi_stats, xdp: *const libeth_xdp_buff) { let s = xdp_get_shared_info_from_buff(&(*xdp).u.base); (*ss).bytes += (*s).xdp_frags_size; (*ss).fragments += (*s).nr_frags + 1; }
    pub unsafe fn libeth_xdpmo_rx_hash(hash: *mut u32, rss_type: *mut xdp_rss_hash_type, val: u32, pt: libeth_rx_pt) -> i32 { if val == 0 { return -ENODATA; } *hash = val; *rss_type = pt.hash_type; 0 }

    /* Macro-only driver helpers retain their original invocation shape. */
    #[macro_export] macro_rules! LIBETH_XDP_ONSTACK_BULK { ($bq:ident) => { let mut $bq: libeth_xdp_tx_bulk = unsafe { core::mem::zeroed() }; }; }
    #[macro_export] macro_rules! libeth_xdp_set_features { ($d:expr $(, $x:expr)*) => { unsafe { __libeth_xdp_set_features($d, core::ptr::null(), 0, core::ptr::null()) } }; }
    #[macro_export] macro_rules! libeth_xdp_set_features_noredir { ($d:expr $(, $x:expr)*) => {{ unsafe { libeth_xdp_set_features($d $(, $x)*); libeth_xdp_set_redirect($d, false); } }}; }
    pub const libeth_xsktmo: *const c_void = GOLDEN_RATIO_PRIME as *const c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
