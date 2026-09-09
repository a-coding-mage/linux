/* SPDX-License-Identifier: GPL-2.0 */
/*
 * External livepatch interfaces for patch creation tooling
 */

// Dependency supplied by the surrounding kernel bindings: <linux/types.h>

pub const KLP_RELOC_SEC_PREFIX: &str = ".klp.rela.";
pub const KLP_SYM_PREFIX: &str = ".klp.sym.";

pub const __KLP_PRE_PATCH_PREFIX: &str = "__klp_pre_patch_callback_";
pub const __KLP_POST_PATCH_PREFIX: &str = "__klp_post_patch_callback_";
pub const __KLP_PRE_UNPATCH_PREFIX: &str = "__klp_pre_unpatch_callback_";
pub const __KLP_POST_UNPATCH_PREFIX: &str = "__klp_post_unpatch_callback_";

pub const KLP_PRE_PATCH_PREFIX: &str = "__klp_pre_patch_callback_";
pub const KLP_POST_PATCH_PREFIX: &str = "__klp_post_patch_callback_";
pub const KLP_PRE_UNPATCH_PREFIX: &str = "__klp_pre_unpatch_callback_";
pub const KLP_POST_UNPATCH_PREFIX: &str = "__klp_post_unpatch_callback_";

#[repr(C)]
pub struct klp_object;

pub type klp_pre_patch_t = Option<unsafe extern "C" fn(obj: *mut klp_object) -> core::ffi::c_int>;
pub type klp_post_patch_t = Option<unsafe extern "C" fn(obj: *mut klp_object)>;
pub type klp_pre_unpatch_t = Option<unsafe extern "C" fn(obj: *mut klp_object)>;
pub type klp_post_unpatch_t = Option<unsafe extern "C" fn(obj: *mut klp_object)>;

/**
 * struct klp_callbacks - pre/post live-(un)patch callback structure
 * @pre_patch:          executed before code patching
 * @post_patch:         executed after code patching
 * @pre_unpatch:        executed before code unpatching
 * @post_unpatch:       executed after code unpatching
 * @post_unpatch_enabled:       flag indicating if post-unpatch callback
 *                              should run
 *
 * All callbacks are optional.  Only the pre-patch callback, if provided,
 * will be unconditionally executed.  If the parent klp_object fails to
 * patch for any reason, including a non-zero error status returned from
 * the pre-patch callback, no further callbacks will be executed.
 */
#[repr(C)]
pub struct klp_callbacks {
    pub pre_patch: klp_pre_patch_t,
    pub post_patch: klp_post_patch_t,
    pub pre_unpatch: klp_pre_unpatch_t,
    pub post_unpatch: klp_post_unpatch_t,
    pub post_unpatch_enabled: bool,
}

/*
 * 'struct klp_{func,object}_ext' are compact "external" representations of
 * 'struct klp_{func,object}'.   They are used by objtool for livepatch
 * generation.  The structs are then read by the livepatch module and converted
 * to the real structs before calling klp_enable_patch().
 *
 * TODO make these the official API for klp_enable_patch().  That should
 * simplify livepatch's interface as well as its data structure lifetime
 * management.
 */
#[repr(C)]
pub struct klp_func_ext {
    pub old_name: *const core::ffi::c_char,
    pub new_func: *mut core::ffi::c_void,
    pub sympos: core::ffi::c_ulong,
}

#[repr(C)]
pub struct klp_object_ext {
    pub name: *const core::ffi::c_char,
    pub funcs: *mut klp_func_ext,
    pub callbacks: klp_callbacks,
    pub nr_funcs: core::ffi::c_uint,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
