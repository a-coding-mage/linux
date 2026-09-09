// Translated from nf_conntrack_count.h.
// Dependencies supplied by the included Linux and netfilter headers are
// intentionally referenced by name rather than implemented here.

// #include <linux/list.h>
// #include <linux/spinlock.h>
// #include <net/netfilter/nf_conntrack_tuple.h>
// #include <net/netfilter/nf_conntrack_zones.h>

pub struct nf_conncount_data;

#[repr(C)]
pub struct nf_conncount_list {
    pub list_lock: spinlock_t,
    pub last_gc: u32, // jiffies at most recent gc
    pub head: list_head, // connections with the same filtering key
    pub count: ::core::ffi::c_uint, // length of list
    pub last_gc_count: ::core::ffi::c_uint, // length of list at most recent gc
}

// External types provided by the included headers.
// type spinlock_t;
// type list_head;
// struct net;
// struct sk_buff;

extern "C" {
    pub fn nf_conncount_init(
        net: *mut net,
        keylen: ::core::ffi::c_uint,
    ) -> *mut nf_conncount_data;

    pub fn nf_conncount_destroy(
        net: *mut net,
        data: *mut nf_conncount_data,
    );

    pub fn nf_conncount_count_skb(
        net: *mut net,
        skb: *const sk_buff,
        l3num: u16,
        data: *mut nf_conncount_data,
        key: *const u32,
    ) -> ::core::ffi::c_uint;

    pub fn nf_conncount_add_skb(
        net: *mut net,
        skb: *const sk_buff,
        l3num: u16,
        list: *mut nf_conncount_list,
    ) -> ::core::ffi::c_int;

    pub fn nf_conncount_list_init(list: *mut nf_conncount_list);

    pub fn nf_conncount_gc_list(
        net: *mut net,
        list: *mut nf_conncount_list,
    ) -> bool;

    pub fn nf_conncount_cache_free(list: *mut nf_conncount_list);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
