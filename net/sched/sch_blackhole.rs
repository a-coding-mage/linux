// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * net/sched/sch_blackhole.c\tBlack hole queue
 *
 * Authors:\tThomas Graf <tgraf@suug.ch>
 *
 * Note: Quantum tunneling is not supported.
 */

// External kernel declarations supplied by the surrounding translation unit.
#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Qdisc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Qdisc_ops {
    pub id: *const u8,
    pub priv_size: usize,
    pub enqueue:
        Option<unsafe extern "C" fn(*mut sk_buff, *mut Qdisc, *mut *mut sk_buff) -> i32>,
    pub dequeue: Option<unsafe extern "C" fn(*mut Qdisc) -> *mut sk_buff>,
    pub peek: Option<unsafe extern "C" fn(*mut Qdisc) -> *mut sk_buff>,
    pub owner: *mut core::ffi::c_void,
}

unsafe extern "C" {
    fn qdisc_drop(skb: *mut sk_buff, sch: *mut Qdisc, to_free: *mut *mut sk_buff);
    fn register_qdisc(ops: *mut Qdisc_ops) -> i32;
    static mut THIS_MODULE: *mut core::ffi::c_void;
}

pub const NET_XMIT_SUCCESS: i32 = 0;
pub const __NET_XMIT_BYPASS: i32 = 1 << 0;

unsafe extern "C" fn blackhole_enqueue(
    skb: *mut sk_buff,
    sch: *mut Qdisc,
    to_free: *mut *mut sk_buff,
) -> i32 {
    unsafe {
        qdisc_drop(skb, sch, to_free);
    }
    NET_XMIT_SUCCESS | __NET_XMIT_BYPASS
}

unsafe extern "C" fn blackhole_dequeue(_sch: *mut Qdisc) -> *mut sk_buff {
    core::ptr::null_mut()
}

#[link_section = ".data..read_mostly"]
static mut blackhole_qdisc_ops: Qdisc_ops = Qdisc_ops {
    id: b"blackhole\0".as_ptr(),
    priv_size: 0,
    enqueue: Some(blackhole_enqueue),
    dequeue: Some(blackhole_dequeue),
    peek: Some(blackhole_dequeue),
    owner: core::ptr::null_mut(),
};

unsafe extern "C" fn blackhole_init() -> i32 {
    unsafe { register_qdisc(&raw mut blackhole_qdisc_ops) }
}

// device_initcall(blackhole_init)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
