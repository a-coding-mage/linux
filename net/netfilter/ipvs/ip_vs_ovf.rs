// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * IPVS:        Overflow-Connection Scheduling module
 *
 * Authors:     Raducu Deaconu <rhadoo_io@yahoo.com>
 *
 * Scheduler implements "overflow" loadbalancing according to number of active
 * connections, will keep all connections to the node with the highest weight
 * and overflow to the next node if the number of connections exceeds the node's
 * weight.
 * Note that this scheduler might not be suitable for UDP because it only uses
 * active connections
 */

// C dependencies supplied by the surrounding IPVS/kernel translation.
use core::ffi::{c_char, c_int, c_ushort, c_void};

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct atomic_t {
    pub counter: c_int,
}

#[repr(C)]
pub struct ip_vs_dest {
    pub n_list: list_head,
    pub flags: u32,
    pub activeconns: atomic_t,
    pub weight: atomic_t,
    pub af: c_int,
    pub addr: [u8; 16],
    pub port: c_ushort,
}

#[repr(C)]
pub struct ip_vs_service {
    pub destinations: list_head,
}

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ip_vs_iphdr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ip_vs_scheduler {
    pub name: *const c_char,
    pub refcnt: atomic_t,
    pub module: *mut c_void,
    pub n_list: list_head,
    pub schedule: Option<
        unsafe extern "C" fn(
            svc: *mut ip_vs_service,
            skb: *const sk_buff,
            iph: *mut ip_vs_iphdr,
        ) -> *mut ip_vs_dest,
    >,
}

extern "C" {
    static THIS_MODULE: *mut c_void;
    fn register_ip_vs_scheduler(scheduler: *mut ip_vs_scheduler) -> c_int;
    fn unregister_ip_vs_scheduler(scheduler: *mut ip_vs_scheduler);
    fn synchronize_rcu();
    fn ip_vs_scheduler_err(svc: *mut ip_vs_service, message: *const c_char);
    fn atomic_read(value: *const atomic_t) -> c_int;
}

const IP_VS_DEST_F_OVERLOAD: u32 = 1 << 2;

/* OVF Connection scheduling */
unsafe extern "C" fn ip_vs_ovf_schedule(
    svc: *mut ip_vs_service,
    _skb: *const sk_buff,
    _iph: *mut ip_vs_iphdr,
) -> *mut ip_vs_dest {
    let mut dest: *mut ip_vs_dest;
    let mut h: *mut ip_vs_dest = core::ptr::null_mut();
    let mut hw: c_int = 0;
    let mut w: c_int;

    // IP_VS_DBG(6, "ip_vs_ovf_schedule(): Scheduling...\n");
    // Select the node with highest weight, going to the next in line if active
    // connections exceed weight. The list_for_each_entry_rcu traversal is
    // represented by the surrounding IPVS list implementation.
    let mut pos = (*svc).destinations.next;
    while pos != &mut (*svc).destinations as *mut list_head {
        dest = (pos as *mut u8).sub(0) as *mut ip_vs_dest;
        pos = (*pos).next;
        w = atomic_read(&(*dest).weight);
        if ((*dest).flags & IP_VS_DEST_F_OVERLOAD) != 0
            || atomic_read(&(*dest).activeconns) > w
            || w == 0
        {
            continue;
        }
        if h.is_null() || w > hw {
            h = dest;
            hw = w;
        }
    }

    if !h.is_null() {
        // IP_VS_DBG_BUF(6, "OVF: server %s:%u active %d w %d\n", ...);
        return h;
    }

    ip_vs_scheduler_err(svc, b"no destination available\0".as_ptr() as *const c_char);
    core::ptr::null_mut()
}

static mut ip_vs_ovf_scheduler: ip_vs_scheduler = ip_vs_scheduler {
    name: b"ovf\0".as_ptr() as *const c_char,
    refcnt: atomic_t { counter: 0 },
    module: core::ptr::null_mut(),
    n_list: list_head {
        next: core::ptr::null_mut(),
        prev: core::ptr::null_mut(),
    },
    schedule: Some(ip_vs_ovf_schedule),
};

unsafe extern "C" fn ip_vs_ovf_init() -> c_int {
    register_ip_vs_scheduler(&raw mut ip_vs_ovf_scheduler)
}

unsafe extern "C" fn ip_vs_ovf_cleanup() {
    unregister_ip_vs_scheduler(&raw mut ip_vs_ovf_scheduler);
    synchronize_rcu();
}

// module_init(ip_vs_ovf_init);
// module_exit(ip_vs_ovf_cleanup);
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("ipvs overflow connection scheduler");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
