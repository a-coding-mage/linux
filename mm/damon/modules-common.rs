// SPDX-License-Identifier: GPL-2.0
/*
 * Common Code for DAMON Modules
 */

// Dependency declarations supplied by <linux/damon.h> and "modules-common.h"
// are intentionally left external to this translation unit.

#[repr(C)]
pub struct damon_ctx {
    _private: [u8; 0],
}

#[repr(C)]
pub struct damon_target {
    _private: [u8; 0],
}

extern "C" {
    fn damon_new_ctx() -> *mut damon_ctx;
    fn damon_select_ops(ctx: *mut damon_ctx, ops: i32) -> i32;
    fn damon_destroy_ctx(ctx: *mut damon_ctx);
    fn damon_new_target() -> *mut damon_target;
    fn damon_add_target(ctx: *mut damon_ctx, target: *mut damon_target);
}

// Supplied by <linux/damon.h> (the C source uses the preprocessor constant).

/*
 * Allocate, set, and return a DAMON context for the physical address space.
 * @ctxp:  Pointer to save the point to the newly created context
 * @targetp:       Pointer to save the point to the newly created target
 */
#[no_mangle]
pub unsafe extern "C" fn damon_modules_new_paddr_ctx_target(
    ctxp: *mut *mut damon_ctx,
    targetp: *mut *mut damon_target,
) -> i32 {
    let ctx: *mut damon_ctx;
    let target: *mut damon_target;

    ctx = damon_new_ctx();
    if ctx.is_null() {
        return -12; // -ENOMEM
    }

    if damon_select_ops(ctx, DAMON_OPS_PADDR) != 0 {
        damon_destroy_ctx(ctx);
        return -22; // -EINVAL
    }

    target = damon_new_target();
    if target.is_null() {
        damon_destroy_ctx(ctx);
        return -12; // -ENOMEM
    }
    damon_add_target(ctx, target);

    *ctxp = ctx;
    *targetp = target;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
