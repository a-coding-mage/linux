/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Cache coherency maintenance operation device drivers
 *
 * Copyright Huawei 2025
 */

// C header dependencies: <linux/list.h>, <linux/kref.h>, <linux/types.h>

#[repr(C)]
pub struct cc_inval_params {
    pub addr: phys_addr_t,
    pub size: usize,
}

#[repr(C)]
pub struct cache_coherency_ops {
    pub wbinv: Option<unsafe extern "C" fn(
        cci: *mut cache_coherency_ops_inst,
        invp: *mut cc_inval_params,
    ) -> core::ffi::c_int>,
    pub done: Option<unsafe extern "C" fn(
        cci: *mut cache_coherency_ops_inst,
    ) -> core::ffi::c_int>,
}

#[repr(C)]
pub struct cache_coherency_ops_inst {
    pub kref: kref,
    pub node: list_head,
    pub ops: *const cache_coherency_ops,
}

extern "C" {
    pub fn cache_coherency_ops_instance_register(
        cci: *mut cache_coherency_ops_inst,
    ) -> core::ffi::c_int;
    pub fn cache_coherency_ops_instance_unregister(
        cci: *mut cache_coherency_ops_inst,
    );

    pub fn _cache_coherency_ops_instance_alloc(
        ops: *const cache_coherency_ops,
        size: usize,
    ) -> *mut cache_coherency_ops_inst;

    pub fn cache_coherency_ops_instance_put(cci: *mut cache_coherency_ops_inst);
}

/**
 * cache_coherency_ops_instance_alloc - Allocate cache coherency ops instance
 * @ops: Cache maintenance operations
 * @drv_struct: structure that contains the struct cache_coherency_ops_inst
 * @member: Name of the struct cache_coherency_ops_inst member in @drv_struct.
 *
 * This allocates a driver specific structure and initializes the
 * cache_coherency_ops_inst embedded in the drv_struct. Upon success the
 * pointer must be freed via cache_coherency_ops_instance_put().
 *
 * Returns a &drv_struct * on success, %NULL on error.
 */
#[macro_export]
macro_rules! cache_coherency_ops_instance_alloc {
    ($ops:expr, $drv_struct:ty, $member:ident) => {{
        const _: () = {
            // C: static_assert(__same_type(struct cache_coherency_ops_inst,
            //                              ((drv_struct *)NULL)->member));
            // C: static_assert(offsetof(drv_struct, member) == 0);
        };
        unsafe {
            _cache_coherency_ops_instance_alloc(
                $ops,
                core::mem::size_of::<$drv_struct>(),
            ) as *mut $drv_struct
        }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
