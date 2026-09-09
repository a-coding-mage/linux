/* SPDX-License-Identifier: GPL-2.0 */

// Translated from <net/sch_priv.h>.
// Dependency declarations are supplied by the surrounding networking code.

#[repr(C)]
pub struct mq_sched {
    pub qdiscs: *mut *mut Qdisc,
}

extern "C" {
    pub fn mq_init_common(
        sch: *mut Qdisc,
        opt: *mut nlattr,
        extack: *mut netlink_ext_ack,
        qdisc_ops: *const Qdisc_ops,
    ) -> ::core::ffi::c_int;
    pub fn mq_destroy_common(sch: *mut Qdisc);
    pub fn mq_attach(sch: *mut Qdisc);
    pub fn mq_dump_common(sch: *mut Qdisc, skb: *mut sk_buff);
    pub fn mq_select_queue(sch: *mut Qdisc, tcm: *mut tcmsg) -> *mut netdev_queue;
    pub fn mq_leaf(sch: *mut Qdisc, cl: ::core::ffi::c_ulong) -> *mut Qdisc;
    pub fn mq_find(sch: *mut Qdisc, classid: u32) -> ::core::ffi::c_ulong;
    pub fn mq_dump_class(
        sch: *mut Qdisc,
        cl: ::core::ffi::c_ulong,
        skb: *mut sk_buff,
        tcm: *mut tcmsg,
    ) -> ::core::ffi::c_int;
    pub fn mq_dump_class_stats(
        sch: *mut Qdisc,
        cl: ::core::ffi::c_ulong,
        d: *mut gnet_dump,
    ) -> ::core::ffi::c_int;
    pub fn mq_walk(sch: *mut Qdisc, arg: *mut qdisc_walker);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
