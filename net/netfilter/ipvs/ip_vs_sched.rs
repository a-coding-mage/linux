// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * IPVS         An implementation of the IP virtual server support for the
 *              LINUX operating system.  IPVS is now implemented as a module
 *              over the Netfilter framework. IPVS can be used to build a
 *              high-performance and highly available server based on a
 *              cluster of servers.
 *
 * Authors:     Wensong Zhang <wensong@linuxvirtualserver.org>
 *              Peter Kese <peter.kese@ijs.si>
 */

// Linux kernel includes and build-time configuration are supplied by the
// surrounding translation unit.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct module { _private: [u8; 0] }
#[repr(C)] pub struct ip_vs_scheduler {
    pub n_list: list_head,
    pub name: *const c_char,
    pub module: *mut module,
    pub init_service: Option<unsafe extern "C" fn(*mut ip_vs_service) -> c_int>,
    pub done_service: Option<unsafe extern "C" fn(*mut ip_vs_service)>,
}
#[repr(C)] pub struct ip_vs_service {
    pub scheduler: *mut ip_vs_scheduler,
    pub fwmark: u32,
    pub af: u16,
    pub protocol: u16,
    pub addr: ip_vs_addr,
    pub port: u16,
}
#[repr(C)] pub union ip_vs_addr { pub ip: u32, pub in6: [u8; 16] }

extern "C" {
    static mut ip_vs_schedulers: list_head;
    static mut ip_vs_sched_mutex: mutex;
    fn try_module_get(module: *mut module) -> bool;
    fn module_put(module: *mut module);
    fn request_module(fmt: *const c_char, ...);
    fn ip_vs_use_count_inc() -> bool;
    fn ip_vs_use_count_dec();
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn ip_vs_proto_name(protocol: u16) -> *const c_char;
    fn ntohs(port: u16) -> u16;
}

// Kernel logging, RCU, mutex, list, and CONFIG_IP_VS_IPV6 facilities retain
// their original semantics and are provided by the surrounding dependencies.

#[no_mangle]
pub static mut ip_vs_scheduler_err: Option<unsafe extern "C" fn(*mut ip_vs_service, *const c_char)> = None;

pub unsafe fn ip_vs_bind_scheduler(svc: *mut ip_vs_service, scheduler: *mut ip_vs_scheduler) -> c_int {
    if let Some(init_service) = (*scheduler).init_service {
        let ret = init_service(svc);
        if ret != 0 {
            // pr_err!("%s(): init error\n", __func__);
            return ret;
        }
    }
    // rcu_assign_pointer(svc->scheduler, scheduler);
    (*svc).scheduler = scheduler;
    0
}

pub unsafe fn ip_vs_unbind_scheduler(svc: *mut ip_vs_service) {
    // rcu_dereference_protected(svc->scheduler, 1);
    let sched = (*svc).scheduler;
    if sched.is_null() { return; }
    // Reset the scheduler before initiating any RCU callbacks.
    (*svc).scheduler = core::ptr::null_mut();
    // smp_wmb(); paired with smp_rmb() in ip_vs_schedule().
    if let Some(done_service) = (*sched).done_service { done_service(svc); }
}

unsafe fn ip_vs_sched_getbyname(sched_name: *const c_char) -> *mut ip_vs_scheduler {
    // IP_VS_DBG(2, "%s(): sched_name \"%s\"\n", __func__, sched_name);
    // mutex_lock(&ip_vs_sched_mutex);
    let head = &mut ip_vs_schedulers as *mut list_head;
    let mut pos = (*head).next;
    while pos != head {
        let sched = pos as *mut ip_vs_scheduler;
        let module = (*sched).module;
        if !module.is_null() && !try_module_get(module) { pos = (*pos).next; continue; }
        if strcmp(sched_name, (*sched).name) == 0 {
            // mutex_unlock(&ip_vs_sched_mutex);
            return sched;
        }
        module_put(module);
        pos = (*pos).next;
    }
    // mutex_unlock(&ip_vs_sched_mutex);
    core::ptr::null_mut()
}

pub unsafe fn ip_vs_scheduler_get(sched_name: *const c_char) -> *mut ip_vs_scheduler {
    let mut sched = ip_vs_sched_getbyname(sched_name);
    if sched.is_null() {
        // request_module("ip_vs_%s", sched_name);
        request_module(b"ip_vs_%s\0".as_ptr() as *const c_char, sched_name);
        sched = ip_vs_sched_getbyname(sched_name);
    }
    sched
}

pub unsafe fn ip_vs_scheduler_put(scheduler: *mut ip_vs_scheduler) {
    if !scheduler.is_null() { module_put((*scheduler).module); }
}

pub unsafe extern "C" fn ip_vs_scheduler_err(svc: *mut ip_vs_service, msg: *const c_char) {
    let sched = (*svc).scheduler;
    let sched_name = if !sched.is_null() { (*sched).name } else { b"none\0".as_ptr() as *const c_char };
    if (*svc).fwmark != 0 {
        // IP_VS_ERR_RL!("%s: FWM %u 0x%08X - %s\n", sched_name, svc->fwmark, svc->fwmark, msg);
    // #ifdef CONFIG_IP_VS_IPV6
    } else if (*svc).af == 10 /* AF_INET6 */ {
        // IP_VS_ERR_RL!("%s: %s [%pI6c]:%d - %s\n", ...);
    // #endif
    } else {
        // IP_VS_ERR_RL!("%s: %s %pI4:%d - %s\n", ...);
    }
    let _ = (sched_name, msg, ip_vs_proto_name((*svc).protocol), ntohs((*svc).port));
}

pub unsafe fn register_ip_vs_scheduler(scheduler: *mut ip_vs_scheduler) -> c_int {
    if scheduler.is_null() { return -22; /* -EINVAL */ }
    if (*scheduler).name.is_null() { return -22; }
    if !ip_vs_use_count_inc() { return -2; /* -ENOENT */ }
    // mutex_lock(&ip_vs_sched_mutex);
    if (*scheduler).n_list.next != (*scheduler).n_list.prev {
        // mutex_unlock(&ip_vs_sched_mutex); ip_vs_use_count_dec();
        return -22;
    }
    let head = &mut ip_vs_schedulers as *mut list_head;
    let mut pos = (*head).next;
    while pos != head {
        let sched = pos as *mut ip_vs_scheduler;
        if strcmp((*scheduler).name, (*sched).name) == 0 {
            // mutex_unlock(&ip_vs_sched_mutex); ip_vs_use_count_dec();
            return -22;
        }
        pos = (*pos).next;
    }
    (*scheduler).n_list.next = (*head).next;
    (*scheduler).n_list.prev = head;
    (*(*head).next).prev = &mut (*scheduler).n_list;
    (*head).next = &mut (*scheduler).n_list;
    // mutex_unlock(&ip_vs_sched_mutex);
    0
}

pub unsafe fn unregister_ip_vs_scheduler(scheduler: *mut ip_vs_scheduler) -> c_int {
    if scheduler.is_null() { return -22; /* -EINVAL */ }
    // mutex_lock(&ip_vs_sched_mutex);
    if (*scheduler).n_list.next == (*scheduler).n_list.prev {
        // mutex_unlock(&ip_vs_sched_mutex);
        return -22;
    }
    let next = (*scheduler).n_list.next;
    let prev = (*scheduler).n_list.prev;
    (*next).prev = prev;
    (*prev).next = next;
    (*scheduler).n_list.next = (*scheduler).n_list.prev = &mut (*scheduler).n_list;
    // mutex_unlock(&ip_vs_sched_mutex);
    ip_vs_use_count_dec();
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
