// SPDX-License-Identifier: GPL-2.0-only
/*
 *  Copyright (c) 2013
 *  Minchan Kim <minchan@kernel.org>
 */

// Dependency declarations supplied by the surrounding kernel/SquashFS code
// are intentionally not reproduced here.

/*
 * This file implements multi-threaded decompression in the
 * decompressor framework
 */

/*
 * The reason that multiply two is that a CPU can request new I/O
 * while it is waiting previous request.
 */
const MAX_DECOMPRESSOR: i32 = num_online_cpus() * 2;

unsafe fn squashfs_max_decompressors() -> i32 {
    MAX_DECOMPRESSOR
}

#[repr(C)]
struct squashfs_stream {
    comp_opts: *mut core::ffi::c_void,
    strm_list: list_head,
    mutex: mutex,
    avail_decomp: i32,
    wait: wait_queue_head_t,
}

#[repr(C)]
struct decomp_stream {
    stream: *mut core::ffi::c_void,
    list: list_head,
}

unsafe fn put_decomp_stream(
    decomp_strm: *mut decomp_stream,
    stream: *mut squashfs_stream,
) {
    mutex_lock(core::ptr::addr_of_mut!((*stream).mutex));
    list_add(
        core::ptr::addr_of_mut!((*decomp_strm).list),
        core::ptr::addr_of_mut!((*stream).strm_list),
    );
    mutex_unlock(core::ptr::addr_of_mut!((*stream).mutex));
    wake_up(core::ptr::addr_of_mut!((*stream).wait));
}

unsafe fn squashfs_decompressor_create(
    msblk: *mut squashfs_sb_info,
    comp_opts: *mut core::ffi::c_void,
) -> *mut core::ffi::c_void {
    let mut stream: *mut squashfs_stream;
    let mut decomp_strm: *mut decomp_stream = core::ptr::null_mut();
    let mut err: i32 = -ENOMEM;

    stream = kzalloc_obj::<squashfs_stream>();
    if stream.is_null() {
        return ERR_PTR(err);
    }

    (*stream).comp_opts = comp_opts;
    mutex_init(core::ptr::addr_of_mut!((*stream).mutex));
    INIT_LIST_HEAD(core::ptr::addr_of_mut!((*stream).strm_list));
    init_waitqueue_head(core::ptr::addr_of_mut!((*stream).wait));

    /*
     * We should have a decompressor at least as default
     * so if we fail to allocate new decompressor dynamically,
     * we could always fall back to default decompressor and
     * file system works.
     */
    decomp_strm = kmalloc_obj::<decomp_stream>();
    if decomp_strm.is_null() {
        kfree(stream as *mut core::ffi::c_void);
        return ERR_PTR(err);
    }

    (*decomp_strm).stream = (*(*msblk).decompressor).init(msblk, (*stream).comp_opts);
    if IS_ERR((*decomp_strm).stream) {
        err = PTR_ERR((*decomp_strm).stream);
        kfree(decomp_strm as *mut core::ffi::c_void);
        kfree(stream as *mut core::ffi::c_void);
        return ERR_PTR(err);
    }

    list_add(
        core::ptr::addr_of_mut!((*decomp_strm).list),
        core::ptr::addr_of_mut!((*stream).strm_list),
    );
    (*stream).avail_decomp = 1;
    stream as *mut core::ffi::c_void
}

unsafe fn squashfs_decompressor_destroy(msblk: *mut squashfs_sb_info) {
    let stream = (*msblk).stream as *mut squashfs_stream;
    if !stream.is_null() {
        while !list_empty(core::ptr::addr_of!((*stream).strm_list)) {
            let decomp_strm = list_entry::<decomp_stream>(
                (*stream).strm_list.prev,
                core::mem::offset_of!(decomp_stream, list),
            );
            list_del(core::ptr::addr_of_mut!((*decomp_strm).list));
            (*(*msblk).decompressor).free((*decomp_strm).stream);
            kfree(decomp_strm as *mut core::ffi::c_void);
            (*stream).avail_decomp -= 1;
        }
        WARN_ON((*stream).avail_decomp != 0);
        kfree((*stream).comp_opts);
        kfree(stream as *mut core::ffi::c_void);
    }
}

unsafe fn get_decomp_stream(
    msblk: *mut squashfs_sb_info,
    stream: *mut squashfs_stream,
) -> *mut decomp_stream {
    loop {
        mutex_lock(core::ptr::addr_of_mut!((*stream).mutex));
        if !list_empty(core::ptr::addr_of!((*stream).strm_list)) {
            let decomp_strm = list_entry::<decomp_stream>(
                (*stream).strm_list.prev,
                core::mem::offset_of!(decomp_stream, list),
            );
            list_del(core::ptr::addr_of_mut!((*decomp_strm).list));
            mutex_unlock(core::ptr::addr_of_mut!((*stream).mutex));
            return decomp_strm;
        }

        if (*stream).avail_decomp >= (*msblk).max_thread_num {
            mutex_unlock(core::ptr::addr_of_mut!((*stream).mutex));
            wait_event((*stream).wait, !list_empty(core::ptr::addr_of!((*stream).strm_list)));
            continue;
        }

        let decomp_strm = kmalloc_obj::<decomp_stream>();
        if decomp_strm.is_null() {
            mutex_unlock(core::ptr::addr_of_mut!((*stream).mutex));
            wait_event((*stream).wait, !list_empty(core::ptr::addr_of!((*stream).strm_list)));
            continue;
        }
        (*decomp_strm).stream = (*(*msblk).decompressor).init(msblk, (*stream).comp_opts);
        if IS_ERR((*decomp_strm).stream) {
            kfree(decomp_strm as *mut core::ffi::c_void);
            mutex_unlock(core::ptr::addr_of_mut!((*stream).mutex));
            wait_event((*stream).wait, !list_empty(core::ptr::addr_of!((*stream).strm_list)));
            continue;
        }
        (*stream).avail_decomp += 1;
        WARN_ON((*stream).avail_decomp > (*msblk).max_thread_num);
        mutex_unlock(core::ptr::addr_of_mut!((*stream).mutex));
        return decomp_strm;
    }
}

unsafe fn squashfs_decompress(
    msblk: *mut squashfs_sb_info,
    bio: *mut bio,
    offset: i32,
    length: i32,
    output: *mut squashfs_page_actor,
) -> i32 {
    let stream = (*msblk).stream as *mut squashfs_stream;
    let decomp_stream = get_decomp_stream(msblk, stream);
    let res = (*(*msblk).decompressor).decompress(
        msblk, (*decomp_stream).stream, bio, offset, length, output,
    );
    put_decomp_stream(decomp_stream, stream);
    if res < 0 {
        ERROR!("%s decompression failed, data probably corrupt\n", (*(*msblk).decompressor).name);
    }
    res
}

#[no_mangle]
pub static mut squashfs_decompressor_multi: squashfs_decompressor_thread_ops =
    squashfs_decompressor_thread_ops {
        create: Some(squashfs_decompressor_create),
        destroy: Some(squashfs_decompressor_destroy),
        decompress: Some(squashfs_decompress),
        max_decompressors: Some(squashfs_max_decompressors),
    };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
