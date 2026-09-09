// SPDX-License-Identifier: GPL-2.0-or-later

// Kernel and backend declarations are supplied by other translation units.

static BACKENDS: [*const zcomp_ops; 8] = [
    // CONFIG_ZRAM_BACKEND_LZO
    &backend_lzorle,
    &backend_lzo,
    // CONFIG_ZRAM_BACKEND_LZ4
    &backend_lz4,
    // CONFIG_ZRAM_BACKEND_LZ4HC
    &backend_lz4hc,
    // CONFIG_ZRAM_BACKEND_ZSTD
    &backend_zstd,
    // CONFIG_ZRAM_BACKEND_DEFLATE
    &backend_deflate,
    // CONFIG_ZRAM_BACKEND_842
    &backend_842,
    core::ptr::null(),
];

unsafe fn zcomp_strm_free(comp: *mut zcomp, zstrm: *mut zcomp_strm) {
    ((*(*comp).ops).destroy_ctx)(&mut (*zstrm).ctx);
    vfree((*zstrm).local_copy);
    vfree((*zstrm).buffer);
    (*zstrm).buffer = core::ptr::null_mut();
}

unsafe fn zcomp_strm_init(comp: *mut zcomp, zstrm: *mut zcomp_strm) -> i32 {
    let ret = ((*(*comp).ops).create_ctx)((*comp).params, &mut (*zstrm).ctx);
    if ret != 0 {
        return ret;
    }

    (*zstrm).local_copy = vzalloc(PAGE_SIZE);
    /*
     * allocate 2 pages. 1 for compressed data, plus 1 extra for the
     * case when compressed size is larger than the original one
     */
    (*zstrm).buffer = vzalloc(2 * PAGE_SIZE);
    if (*zstrm).buffer.is_null() || (*zstrm).local_copy.is_null() {
        zcomp_strm_free(comp, zstrm);
        return -ENOMEM;
    }
    0
}

unsafe fn lookup_backend_ops(comp: *const core::ffi::c_char) -> *const zcomp_ops {
    let mut i = 0usize;
    while !BACKENDS[i].is_null() {
        if sysfs_streq(comp, (*BACKENDS[i]).name) {
            break;
        }
        i += 1;
    }
    BACKENDS[i]
}

pub unsafe fn zcomp_lookup_backend_name(comp: *const core::ffi::c_char) -> *const core::ffi::c_char {
    let backend = lookup_backend_ops(comp);
    if !backend.is_null() {
        return (*backend).name;
    }
    core::ptr::null()
}

/* show available compressors */
pub unsafe fn zcomp_available_show(
    comp: *const core::ffi::c_char,
    buf: *mut core::ffi::c_char,
    mut at: isize,
) -> isize {
    for i in 0..BACKENDS.len() - 1 {
        if strcmp(comp, (*BACKENDS[i]).name) == 0 {
            at += sysfs_emit_at(buf, at, "[%s] ", (*BACKENDS[i]).name);
        } else {
            at += sysfs_emit_at(buf, at, "%s ", (*BACKENDS[i]).name);
        }
    }
    at += sysfs_emit_at(buf, at, "\n");
    at
}

pub unsafe fn zcomp_stream_get(comp: *mut zcomp) -> *mut zcomp_strm {
    loop {
        let zstrm = raw_cpu_ptr((*comp).stream);
        /*
         * Inspired by zswap
         *
         * stream is returned with ->mutex locked which prevents
         * cpu_dead() from releasing this stream under us, however
         * there is still a race window between raw_cpu_ptr() and
         * mutex_lock(), during which we could have been migrated
         * from a CPU that has already destroyed its stream.  If
         * so then unlock and re-try on the current CPU.
         */
        mutex_lock(&mut (*zstrm).lock);
        if !(*zstrm).buffer.is_null() {
            return zstrm;
        }
        mutex_unlock(&mut (*zstrm).lock);
    }
}

pub unsafe fn zcomp_stream_put(zstrm: *mut zcomp_strm) {
    mutex_unlock(&mut (*zstrm).lock);
}

pub unsafe fn zcomp_compress(
    comp: *mut zcomp,
    zstrm: *mut zcomp_strm,
    src: *const core::ffi::c_void,
    dst_len: *mut u32,
) -> i32 {
    let mut req = zcomp_req {
        src,
        dst: (*zstrm).buffer,
        src_len: PAGE_SIZE,
        dst_len: 2 * PAGE_SIZE,
    };
    might_sleep();
    let ret = ((*(*comp).ops).compress)((*comp).params, &mut (*zstrm).ctx, &mut req);
    if ret == 0 {
        *dst_len = req.dst_len;
    }
    ret
}

pub unsafe fn zcomp_decompress(
    comp: *mut zcomp,
    zstrm: *mut zcomp_strm,
    src: *const core::ffi::c_void,
    src_len: u32,
    dst: *mut core::ffi::c_void,
) -> i32 {
    let mut req = zcomp_req { src, dst, src_len, dst_len: PAGE_SIZE };
    might_sleep();
    ((*(*comp).ops).decompress)((*comp).params, &mut (*zstrm).ctx, &mut req)
}

pub unsafe fn zcomp_cpu_up_prepare(cpu: u32, node: *mut hlist_node) -> i32 {
    let comp = hlist_entry::<zcomp>(node);
    let zstrm = per_cpu_ptr((*comp).stream, cpu);
    let ret = zcomp_strm_init(comp, zstrm);
    if ret != 0 {
        pr_err("Can't allocate a compression stream\n");
    }
    ret
}

pub unsafe fn zcomp_cpu_dead(cpu: u32, node: *mut hlist_node) -> i32 {
    let comp = hlist_entry::<zcomp>(node);
    let zstrm = per_cpu_ptr((*comp).stream, cpu);
    mutex_lock(&mut (*zstrm).lock);
    zcomp_strm_free(comp, zstrm);
    mutex_unlock(&mut (*zstrm).lock);
    0
}

unsafe fn zcomp_init(comp: *mut zcomp, params: *mut zcomp_params) -> i32 {
    let mut ret;
    (*comp).stream = alloc_percpu::<zcomp_strm>();
    if (*comp).stream.is_null() {
        return -ENOMEM;
    }
    (*comp).params = params;
    ret = ((*(*comp).ops).setup_params)((*comp).params);
    if ret != 0 {
        goto_cleanup(comp, ret);
    }
    for_each_possible_cpu!(cpu, {
        mutex_init(&mut (*per_cpu_ptr((*comp).stream, cpu)).lock);
    });
    ret = cpuhp_state_add_instance(CPUHP_ZCOMP_PREPARE, &mut (*comp).node);
    if ret < 0 {
        goto_cleanup(comp, ret);
    }
    0
}

unsafe fn goto_cleanup(comp: *mut zcomp, ret: i32) -> i32 {
    ((*(*comp).ops).release_params)((*comp).params);
    free_percpu((*comp).stream);
    ret
}

pub unsafe fn zcomp_destroy(comp: *mut zcomp) {
    cpuhp_state_remove_instance(CPUHP_ZCOMP_PREPARE, &mut (*comp).node);
    ((*(*comp).ops).release_params)((*comp).params);
    free_percpu((*comp).stream);
    kfree(comp);
}

pub unsafe fn zcomp_create(
    alg: *const core::ffi::c_char,
    params: *mut zcomp_params,
) -> *mut zcomp {
    // The backends array has a sentinel NULL value and at least one backend.
    build_bug_on!(BACKENDS.len() <= 1);
    let comp = kzalloc_obj::<zcomp>();
    if comp.is_null() {
        return err_ptr(-ENOMEM);
    }
    (*comp).ops = lookup_backend_ops(alg);
    if (*comp).ops.is_null() {
        kfree(comp);
        return err_ptr(-EINVAL);
    }
    let error = zcomp_init(comp, params);
    if error != 0 {
        kfree(comp);
        return err_ptr(error);
    }
    comp
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
