/* SPDX-License-Identifier: GPL-2.0 */

// C dependency: <linux/livepatch.h>

extern "C" {
    pub static mut klp_mutex: mutex;
    pub static mut klp_patches: list_head;

    pub fn klp_free_patch_async(patch: *mut klp_patch);
    pub fn klp_free_replaced_patches_async(new_patch: *mut klp_patch);
    pub fn klp_unpatch_replaced_patches(new_patch: *mut klp_patch);
    pub fn klp_discard_nops(new_patch: *mut klp_patch);
}

// C macros:
// klp_for_each_patch_safe(patch, tmp_patch) expands to
// list_for_each_entry_safe(patch, tmp_patch, &klp_patches, list).
// klp_for_each_patch(patch) expands to
// list_for_each_entry(patch, &klp_patches, list).

#[inline]
pub unsafe fn klp_is_object_loaded(obj: *mut klp_object) -> bool {
    (*obj).name.is_null() || !(*obj).mod_.is_null()
}

#[inline]
pub unsafe fn klp_pre_patch_callback(obj: *mut klp_object) -> i32 {
    let mut ret: i32 = 0;

    if let Some(pre_patch) = (*obj).callbacks.pre_patch {
        ret = pre_patch(obj);
    }

    (*obj).callbacks.post_unpatch_enabled = ret == 0;

    ret
}

#[inline]
pub unsafe fn klp_post_patch_callback(obj: *mut klp_object) {
    if let Some(post_patch) = (*obj).callbacks.post_patch {
        post_patch(obj);
    }
}

#[inline]
pub unsafe fn klp_pre_unpatch_callback(obj: *mut klp_object) {
    if let Some(pre_unpatch) = (*obj).callbacks.pre_unpatch {
        pre_unpatch(obj);
    }
}

#[inline]
pub unsafe fn klp_post_unpatch_callback(obj: *mut klp_object) {
    if (*obj).callbacks.post_unpatch_enabled {
        if let Some(post_unpatch) = (*obj).callbacks.post_unpatch {
            post_unpatch(obj);
        }
    }

    (*obj).callbacks.post_unpatch_enabled = false;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
