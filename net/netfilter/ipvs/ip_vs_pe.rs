// SPDX-License-Identifier: GPL-2.0-only
// C preprocessor format prefix: "IPVS: "

// Dependencies supplied by the surrounding kernel/IPVS translation.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ip_vs_pe {
    pub n_list: list_head,
    pub module: *mut module,
    pub name: *const c_char,
}

// IPVS pe list
static mut IP_VS_PE: list_head = list_head {
    next: core::ptr::null_mut(),
    prev: core::ptr::null_mut(),
};

// semaphore for IPVS PEs.
static mut IP_VS_PE_MUTEX: mutex = mutex { _private: [] };

extern "C" {
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn try_module_get(module: *mut module) -> bool;
    fn module_put(module: *mut module);
    fn request_module(fmt: *const c_char, ...) -> c_int;
    fn ip_vs_use_count_inc() -> bool;
    fn ip_vs_use_count_dec();
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn pr_err(fmt: *const c_char, ...);
    fn pr_info(fmt: *const c_char, ...);
    fn list_add_rcu(new: *mut list_head, head: *mut list_head);
    fn list_del_rcu(entry: *mut list_head);
}

/* The following list traversal is the direct equivalent of
 * list_for_each_entry_rcu(pe, &ip_vs_pe, n_list). */
unsafe fn next_ip_vs_pe(pe: *mut ip_vs_pe) -> *mut ip_vs_pe {
    (*(pe as *mut ip_vs_pe)).n_list.next as *mut ip_vs_pe
}

/* Get pe in the pe list by name */
pub unsafe fn __ip_vs_pe_getbyname(pe_name: *const c_char) -> *mut ip_vs_pe {
    let mut pe: *mut ip_vs_pe = IP_VS_PE.next as *mut ip_vs_pe;

    rcu_read_lock();
    while pe != (&raw mut IP_VS_PE as *mut list_head) as *mut ip_vs_pe {
        /* Test and get the modules atomically */
        if !(*pe).module.is_null() && !try_module_get((*pe).module) {
            /* This pe is just deleted */
            pe = next_ip_vs_pe(pe);
            continue;
        }
        if strcmp(pe_name, (*pe).name) == 0 {
            /* HIT */
            rcu_read_unlock();
            return pe;
        }
        module_put((*pe).module);
        pe = next_ip_vs_pe(pe);
    }
    rcu_read_unlock();

    core::ptr::null_mut()
}

/* Lookup pe and try to load it if it doesn't exist */
pub unsafe fn ip_vs_pe_getbyname(name: *const c_char) -> *mut ip_vs_pe {
    /* Search for the pe by name */
    let mut pe = __ip_vs_pe_getbyname(name);

    /* If pe not found, load the module and search again */
    if pe.is_null() {
        static FMT: &[u8] = b"ip_vs_pe_%s\0";
        request_module(FMT.as_ptr() as *const c_char, name);
        pe = __ip_vs_pe_getbyname(name);
    }

    pe
}

/* Register a pe in the pe list */
pub unsafe fn register_ip_vs_pe(pe: *mut ip_vs_pe) -> c_int {
    /* increase the module use count */
    if !ip_vs_use_count_inc() {
        return -2; // -ENOENT
    }

    mutex_lock(&raw mut IP_VS_PE_MUTEX);
    /* Make sure that the pe with this name doesn't exist
     * in the pe list.
     */
    let mut tmp = IP_VS_PE.next as *mut ip_vs_pe;
    while tmp != (&raw mut IP_VS_PE as *mut list_head) as *mut ip_vs_pe {
        if strcmp((*tmp).name, (*pe).name) == 0 {
            mutex_unlock(&raw mut IP_VS_PE_MUTEX);
            ip_vs_use_count_dec();
            pr_err(c"%s(): [%s] pe already existed in the system\n", register_ip_vs_pe as *const c_void, (*pe).name);
            return -22; // -EINVAL
        }
        tmp = next_ip_vs_pe(tmp);
    }
    /* Add it into the d-linked pe list */
    list_add_rcu(&raw mut (*pe).n_list, &raw mut IP_VS_PE);
    mutex_unlock(&raw mut IP_VS_PE_MUTEX);

    pr_info(c"[%s] pe registered.\n", (*pe).name);
    0
}

/* Unregister a pe from the pe list */
pub unsafe fn unregister_ip_vs_pe(pe: *mut ip_vs_pe) -> c_int {
    mutex_lock(&raw mut IP_VS_PE_MUTEX);
    /* Remove it from the d-linked pe list */
    list_del_rcu(&raw mut (*pe).n_list);
    mutex_unlock(&raw mut IP_VS_PE_MUTEX);

    /* decrease the module use count */
    ip_vs_use_count_dec();

    pr_info(c"[%s] pe unregistered.\n", (*pe).name);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
