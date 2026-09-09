// SPDX-License-Identifier: GPL-2.0

// Dependency: declarations supplied by "blk-cgroup.h".

/**
 * blkcg_set_fc_appid - set the fc_app_id field associted to blkcg
 * @app_id: application identifier
 * @cgrp_id: cgroup id
 * @app_id_len: size of application identifier
 */
pub unsafe fn blkcg_set_fc_appid(
    app_id: *mut core::ffi::c_char,
    cgrp_id: u64,
    app_id_len: usize,
) -> i32 {
    let cgrp: *mut cgroup;
    let css: *mut cgroup_subsys_state;
    let blkcg: *mut blkcg;
    let mut ret: i32 = 0;

    if app_id_len > FC_APPID_LEN {
        return -EINVAL;
    }

    cgrp = cgroup_get_from_id(cgrp_id);
    if IS_ERR(cgrp) {
        return PTR_ERR(cgrp);
    }
    css = cgroup_get_e_css(cgrp, &mut io_cgrp_subsys);
    if css.is_null() {
        ret = -ENOENT;
        goto_out_cgrp_put(cgrp, ret)
    }
    blkcg = css_to_blkcg(css);
    /*
     * There is a slight race condition on setting the appid.
     * Worst case an I/O may not find the right id.
     * This is no different from the I/O we let pass while obtaining
     * the vmid from the fabric.
     * Adding the overhead of a lock is not necessary.
     */
    strscpy((*blkcg).fc_app_id.as_mut_ptr(), app_id, app_id_len);
    css_put(css);
    goto_out_cgrp_put(cgrp, ret)
}

#[inline(always)]
unsafe fn goto_out_cgrp_put(cgrp: *mut cgroup, ret: i32) -> i32 {
    cgroup_put(cgrp);
    ret
}

// EXPORT_SYMBOL_GPL(blkcg_set_fc_appid);

/**
 * blkcg_get_fc_appid - get the fc app identifier associated with a bio
 * @bio: target bio
 *
 * On success return the fc_app_id, on failure return NULL
 */
pub unsafe fn blkcg_get_fc_appid(bio: *mut bio) -> *mut core::ffi::c_char {
    if (*bio).bi_blkg.is_null()
        || (*(*bio).bi_blkg).blkcg.is_null()
        || (*(*(*bio).bi_blkg).blkcg).fc_app_id[0] == 0
    {
        return core::ptr::null_mut();
    }
    (*(*(*bio).bi_blkg).blkcg).fc_app_id.as_mut_ptr()
}

// EXPORT_SYMBOL_GPL(blkcg_get_fc_appid);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
