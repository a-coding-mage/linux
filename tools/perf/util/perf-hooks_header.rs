/* SPDX-License-Identifier: GPL-2.0 */

pub type perf_hook_func_t = Option<unsafe extern "C" fn(ctx: *mut core::ffi::c_void)>;

#[repr(C)]
pub struct perf_hook_desc {
    pub hook_name: *const core::ffi::c_char,
    pub p_hook_func: *mut perf_hook_func_t,
    pub hook_ctx: *mut core::ffi::c_void,
}

unsafe extern "C" {
    pub fn perf_hooks__invoke(arg1: *const perf_hook_desc);
    pub fn perf_hooks__recover();
}

/*
 * C macro intent:
 *
 * #define PERF_HOOK(name)                                      \
 * extern struct perf_hook_desc __perf_hook_desc_##name;        \
 * static inline void perf_hooks__invoke_##name(void)           \
 * {                                                            \
 *      perf_hooks__invoke(&__perf_hook_desc_##name);           \
 * }
 *
 * The macro is expanded by including "perf-hooks-list.h", which is outside
 * this isolated translation unit. Each listed hook declares an external
 * __perf_hook_desc_<name> and an inline perf_hooks__invoke_<name>() wrapper
 * that calls perf_hooks__invoke(&__perf_hook_desc_<name>).
 */

unsafe extern "C" {
    pub fn perf_hooks__set_hook(
        hook_name: *const core::ffi::c_char,
        hook_func: perf_hook_func_t,
        hook_ctx: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;

    pub fn perf_hooks__get_hook(hook_name: *const core::ffi::c_char) -> perf_hook_func_t;
}
