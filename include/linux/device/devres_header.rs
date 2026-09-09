/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation are intentionally external.

pub enum device {}
pub enum device_node {}
pub enum resource {}

pub type dr_release_t = Option<unsafe extern "C" fn(dev: *mut device, res: *mut core::ffi::c_void)>;
pub type dr_match_t = Option<unsafe extern "C" fn(
    dev: *mut device,
    res: *mut core::ffi::c_void,
    match_data: *mut core::ffi::c_void,
) -> core::ffi::c_int>;

extern "C" {
    pub fn __devres_alloc_node(
        release: dr_release_t,
        size: usize,
        gfp: gfp_t,
        nid: core::ffi::c_int,
        name: *const core::ffi::c_char,
    ) -> *mut core::ffi::c_void;
    pub fn devres_free(res: *mut core::ffi::c_void);
    pub fn devres_add(dev: *mut device, res: *mut core::ffi::c_void);
    pub fn devres_find(dev: *mut device, release: dr_release_t, r#match: dr_match_t, match_data: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    pub fn devres_get(dev: *mut device, new_res: *mut core::ffi::c_void, r#match: dr_match_t, match_data: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    pub fn devres_remove(dev: *mut device, release: dr_release_t, r#match: dr_match_t, match_data: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    pub fn devres_destroy(dev: *mut device, release: dr_release_t, r#match: dr_match_t, match_data: *mut core::ffi::c_void) -> core::ffi::c_int;
    pub fn devres_release(dev: *mut device, release: dr_release_t, r#match: dr_match_t, match_data: *mut core::ffi::c_void) -> core::ffi::c_int;

    pub fn devres_open_group(dev: *mut device, id: *mut core::ffi::c_void, gfp: gfp_t) -> *mut core::ffi::c_void;
    pub fn devres_close_group(dev: *mut device, id: *mut core::ffi::c_void);
    pub fn devres_remove_group(dev: *mut device, id: *mut core::ffi::c_void);
    pub fn devres_release_group(dev: *mut device, id: *mut core::ffi::c_void) -> core::ffi::c_int;

    pub fn devm_kmalloc(dev: *mut device, size: usize, gfp: gfp_t) -> *mut core::ffi::c_void;
    pub fn devm_krealloc(dev: *mut device, ptr: *mut core::ffi::c_void, size: usize, gfp: gfp_t) -> *mut core::ffi::c_void;
    pub fn devm_kfree(dev: *mut device, p: *const core::ffi::c_void);
    pub fn devm_kmemdup(dev: *mut device, src: *const core::ffi::c_void, len: usize, gfp: gfp_t) -> *mut core::ffi::c_void;
    pub fn devm_kmemdup_const(dev: *mut device, src: *const core::ffi::c_void, len: usize, gfp: gfp_t) -> *const core::ffi::c_void;
    pub fn devm_kstrdup(dev: *mut device, s: *const core::ffi::c_char, gfp: gfp_t) -> *mut core::ffi::c_char;
    pub fn devm_kstrdup_const(dev: *mut device, s: *const core::ffi::c_char, gfp: gfp_t) -> *const core::ffi::c_char;
    pub fn devm_kvasprintf(dev: *mut device, gfp: gfp_t, fmt: *const core::ffi::c_char, ap: va_list) -> *mut core::ffi::c_char;
    pub fn devm_kasprintf(dev: *mut device, gfp: gfp_t, fmt: *const core::ffi::c_char, ...) -> *mut core::ffi::c_char;
    pub fn __devm_alloc_percpu(dev: *mut device, size: usize, align: usize) -> *mut core::ffi::c_void;
    pub fn devm_get_free_pages(dev: *mut device, gfp_mask: gfp_t, order: core::ffi::c_uint) -> c_ulong;
    pub fn devm_free_pages(dev: *mut device, addr: c_ulong);
    pub fn devm_remove_action_nowarn(dev: *mut device, action: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>, data: *mut core::ffi::c_void) -> core::ffi::c_int;
    pub fn devm_release_action(dev: *mut device, action: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>, data: *mut core::ffi::c_void);
    pub fn __devm_add_action(dev: *mut device, action: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>, data: *mut core::ffi::c_void, name: *const core::ffi::c_char) -> core::ffi::c_int;
    pub fn devm_is_action_added(dev: *mut device, action: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>, data: *mut core::ffi::c_void) -> bool;
}

// C macros devres_alloc/devres_alloc_node, devm_alloc_percpu, and action helpers
// retain their source-level forms here because they depend on kernel typeof/stringification.

pub unsafe fn devm_kzalloc(dev: *mut device, size: usize, gfp: gfp_t) -> *mut core::ffi::c_void {
    devm_kmalloc(dev, size, gfp | __GFP_ZERO)
}

pub unsafe fn devm_kmalloc_array(dev: *mut device, n: usize, size: usize, flags: gfp_t) -> *mut core::ffi::c_void {
    let bytes = match n.checked_mul(size) { Some(v) => v, None => return core::ptr::null_mut() };
    devm_kmalloc(dev, bytes, flags)
}

pub unsafe fn devm_kcalloc(dev: *mut device, n: usize, size: usize, flags: gfp_t) -> *mut core::ffi::c_void {
    devm_kmalloc_array(dev, n, size, flags | __GFP_ZERO)
}

pub unsafe fn devm_krealloc_array(dev: *mut device, p: *mut core::ffi::c_void, new_n: usize, new_size: usize, flags: gfp_t) -> *mut core::ffi::c_void {
    let bytes = match new_n.checked_mul(new_size) { Some(v) => v, None => return core::ptr::null_mut() };
    devm_krealloc(dev, p, bytes, flags)
}

pub unsafe fn devm_kmemdup_array(dev: *mut device, src: *const core::ffi::c_void, n: usize, size: usize, flags: gfp_t) -> *mut core::ffi::c_void {
    devm_kmemdup(dev, src, size.wrapping_mul(n), flags)
}

#[cfg(CONFIG_HAS_IOMEM)]
extern "C" {
    pub fn devm_ioremap_resource(dev: *mut device, res: *const resource) -> *mut core::ffi::c_void;
    pub fn devm_ioremap_resource_wc(dev: *mut device, res: *const resource) -> *mut core::ffi::c_void;
    pub fn devm_of_iomap(dev: *mut device, node: *mut device_node, index: core::ffi::c_int, size: *mut resource_size_t) -> *mut core::ffi::c_void;
}

pub unsafe fn devm_remove_action(dev: *mut device, action: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>, data: *mut core::ffi::c_void) {
    let _ = devm_remove_action_nowarn(dev, action, data);
}

pub unsafe fn __devm_add_action_or_reset(dev: *mut device, action: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>, data: *mut core::ffi::c_void, name: *const core::ffi::c_char) -> core::ffi::c_int {
    let ret = __devm_add_action(dev, action, data, name);
    if ret != 0 { if let Some(f) = action { f(data); } }
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
