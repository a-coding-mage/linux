/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding Linux headers:
// linux/livepatch.h, linux/list.h, and linux/ftrace.h

/**
 * struct klp_ops - structure for tracking registered ftrace ops structs
 *
 * A single ftrace_ops is shared between all enabled replacement functions
 * (klp_func structs) which have the same old_func.  This allows the switch
 * between function versions to happen instantaneously by updating the klp_ops
 * struct's func_stack list.  The winner is the klp_func at the top of the
 * func_stack (front of the list).
 *
 * @node:\tnode for the global klp_ops list
 * @func_stack:\tlist head for the stack of klp_func's (active func is on top)
 * @fops:\tregistered ftrace ops struct
 */
#[repr(C)]
pub struct klp_ops {
    pub node: list_head,
    pub func_stack: list_head,
    pub fops: ftrace_ops,
}

extern "C" {
    pub fn klp_find_ops(old_func: *mut core::ffi::c_void) -> *mut klp_ops;

    pub fn klp_patch_object(obj: *mut klp_object) -> core::ffi::c_int;
    pub fn klp_unpatch_object(obj: *mut klp_object);
    pub fn klp_unpatch_objects(patch: *mut klp_patch);
    pub fn klp_unpatch_objects_dynamic(patch: *mut klp_patch);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
