/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * async.h: Asynchronous function calls for boot performance
 *
 * (C) Copyright 2009 Intel Corporation
 * Author: Arjan van de Ven <arjan@linux.intel.com>
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/types.h, linux/list.h, linux/numa.h, and linux/device.h

pub type async_cookie_t = u64;
pub type async_func_t = unsafe extern "C" fn(data: *mut core::ffi::c_void, cookie: async_cookie_t);

#[repr(C)]
pub struct async_domain {
    pub pending: list_head,
    pub registered: u8,
}

/*
 * domain participates in global async_synchronize_full
 */
#[macro_export]
macro_rules! ASYNC_DOMAIN {
    ($name:ident) => {
        let mut $name: async_domain = async_domain {
            pending: LIST_HEAD_INIT!($name.pending),
            registered: 1,
        };
    };
}

/*
 * domain is free to go out of scope as soon as all pending work is
 * complete, this domain does not participate in async_synchronize_full
 */
#[macro_export]
macro_rules! ASYNC_DOMAIN_EXCLUSIVE {
    ($name:ident) => {
        let mut $name: async_domain = async_domain {
            pending: LIST_HEAD_INIT!($name.pending),
            registered: 0,
        };
    };
}

extern "C" {
    pub fn async_schedule_node(
        func: async_func_t,
        data: *mut core::ffi::c_void,
        node: i32,
    ) -> async_cookie_t;
    pub fn async_schedule_node_domain(
        func: async_func_t,
        data: *mut core::ffi::c_void,
        node: i32,
        domain: *mut async_domain,
    ) -> async_cookie_t;
}

/**
 * async_schedule - schedule a function for asynchronous execution
 * @func: function to execute asynchronously
 * @data: data pointer to pass to the function
 *
 * Returns an async_cookie_t that may be used for checkpointing later.
 * Note: This function may be called from atomic or non-atomic contexts.
 */
#[inline]
pub unsafe fn async_schedule(func: async_func_t, data: *mut core::ffi::c_void) -> async_cookie_t {
    async_schedule_node(func, data, NUMA_NO_NODE)
}

/**
 * async_schedule_domain - schedule a function for asynchronous execution within a certain domain
 * @func: function to execute asynchronously
 * @data: data pointer to pass to the function
 * @domain: the domain
 *
 * Returns an async_cookie_t that may be used for checkpointing later.
 * @domain may be used in the async_synchronize_*_domain() functions to
 * wait within a certain synchronization domain rather than globally.
 * Note: This function may be called from atomic or non-atomic contexts.
 */
#[inline]
pub unsafe fn async_schedule_domain(
    func: async_func_t,
    data: *mut core::ffi::c_void,
    domain: *mut async_domain,
) -> async_cookie_t {
    async_schedule_node_domain(func, data, NUMA_NO_NODE, domain)
}

/**
 * async_schedule_dev - A device specific version of async_schedule
 * @func: function to execute asynchronously
 * @dev: device argument to be passed to function
 *
 * Returns an async_cookie_t that may be used for checkpointing later.
 * @dev is used as both the argument for the function and to provide NUMA
 * context for where to run the function. By doing this we can try to
 * provide for the best possible outcome by operating on the device on the
 * CPUs closest to the device.
 * Note: This function may be called from atomic or non-atomic contexts.
 */
#[inline]
pub unsafe fn async_schedule_dev(func: async_func_t, dev: *mut device) -> async_cookie_t {
    async_schedule_node(func, dev.cast(), dev_to_node(dev))
}

extern "C" {
    pub fn async_schedule_dev_nocall(func: async_func_t, dev: *mut device) -> bool;
}

/**
 * async_schedule_dev_domain - A device specific version of async_schedule_domain
 * @func: function to execute asynchronously
 * @dev: device argument to be passed to function
 * @domain: the domain
 *
 * Returns an async_cookie_t that may be used for checkpointing later.
 * @dev is used as both the argument for the function and to provide NUMA
 * context for where to run the function. By doing this we can try to
 * provide for the best possible outcome by operating on the device on the
 * CPUs closest to the device.
 * @domain may be used in the async_synchronize_*_domain() functions to
 * wait within a certain synchronization domain rather than globally.
 * Note: This function may be called from atomic or non-atomic contexts.
 */
#[inline]
pub unsafe fn async_schedule_dev_domain(
    func: async_func_t,
    dev: *mut device,
    domain: *mut async_domain,
) -> async_cookie_t {
    async_schedule_node_domain(func, dev.cast(), dev_to_node(dev), domain)
}

extern "C" {
    pub fn async_synchronize_full();
    pub fn async_synchronize_full_domain(domain: *mut async_domain);
    pub fn async_synchronize_cookie(cookie: async_cookie_t);
    pub fn async_synchronize_cookie_domain(cookie: async_cookie_t, domain: *mut async_domain);
    pub fn current_is_async() -> bool;
    pub fn async_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
