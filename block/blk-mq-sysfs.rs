// SPDX-License-Identifier: GPL-2.0
// Translated from blk-mq-sysfs.c; Linux kernel dependencies are supplied externally.

unsafe fn blk_mq_sysfs_release(kobj: *mut kobject) {
    let ctxs = container_of!(kobj, blk_mq_ctxs, kobj);

    free_percpu((*ctxs).queue_ctx);
    kfree(ctxs);
}

unsafe fn blk_mq_ctx_sysfs_release(kobj: *mut kobject) {
    let ctx = container_of!(kobj, blk_mq_ctx, kobj);

    /* ctx->ctxs won't be released until all ctx are freed */
    kobject_put(&mut (*(*ctx).ctxs).kobj);
}

unsafe fn blk_mq_hw_sysfs_release(kobj: *mut kobject) {
    let hctx = container_of!(kobj, blk_mq_hw_ctx, kobj);

    sbitmap_free(&mut (*hctx).ctx_map);
    free_cpumask_var((*hctx).cpumask);
    kfree((*hctx).ctxs);
    kfree(hctx);
}

#[repr(C)]
struct blk_mq_hw_ctx_sysfs_entry {
    attr: attribute,
    show: Option<unsafe extern "C" fn(*mut blk_mq_hw_ctx, *mut c_char) -> ssize_t>,
}

unsafe fn blk_mq_hw_sysfs_show(
    kobj: *mut kobject,
    attr: *mut attribute,
    page: *mut c_char,
) -> ssize_t {
    let entry = container_of_const!(attr, blk_mq_hw_ctx_sysfs_entry, attr);
    let hctx = container_of!(kobj, blk_mq_hw_ctx, kobj);
    let q = (*hctx).queue;
    let show = (*entry).show;

    if show.is_none() {
        return -EIO;
    }

    mutex_lock(&mut (*q).elevator_lock);
    let res = show.unwrap()(hctx, page);
    mutex_unlock(&mut (*q).elevator_lock);
    res
}

unsafe extern "C" fn blk_mq_hw_sysfs_nr_tags_show(
    hctx: *mut blk_mq_hw_ctx,
    page: *mut c_char,
) -> ssize_t {
    sprintf(page, c!("%u\n"), (*(*hctx).tags).nr_tags)
}

unsafe extern "C" fn blk_mq_hw_sysfs_nr_reserved_tags_show(
    hctx: *mut blk_mq_hw_ctx,
    page: *mut c_char,
) -> ssize_t {
    sprintf(page, c!("%u\n"), (*(*hctx).tags).nr_reserved_tags)
}

unsafe extern "C" fn blk_mq_hw_sysfs_cpus_show(
    hctx: *mut blk_mq_hw_ctx,
    page: *mut c_char,
) -> ssize_t {
    let size: usize = PAGE_SIZE - 1;
    let mut first: c_uint = 1;
    let mut ret: c_int = 0;
    let mut pos: c_int = 0;
    let mut i: c_uint;

    for_each_cpu!(i, (*hctx).cpumask) {
        if first != 0 {
            ret = snprintf(page.add(pos as usize), size - pos as usize, c!("%u"), i);
        } else {
            ret = snprintf(page.add(pos as usize), size - pos as usize, c!(", %u"), i);
        }

        if ret >= (size - pos as usize) as c_int {
            break;
        }

        first = 0;
        pos += ret;
    }

    ret = snprintf(
        page.add(pos as usize),
        size + 1 - pos as usize,
        c!("\n"),
    );
    (pos + ret) as ssize_t
}

static blk_mq_hw_sysfs_nr_tags: blk_mq_hw_ctx_sysfs_entry = blk_mq_hw_ctx_sysfs_entry {
    attr: attribute { name: c!("nr_tags"), mode: 0o444 },
    show: Some(blk_mq_hw_sysfs_nr_tags_show),
};

static blk_mq_hw_sysfs_nr_reserved_tags: blk_mq_hw_ctx_sysfs_entry = blk_mq_hw_ctx_sysfs_entry {
    attr: attribute { name: c!("nr_reserved_tags"), mode: 0o444 },
    show: Some(blk_mq_hw_sysfs_nr_reserved_tags_show),
};

static blk_mq_hw_sysfs_cpus: blk_mq_hw_ctx_sysfs_entry = blk_mq_hw_ctx_sysfs_entry {
    attr: attribute { name: c!("cpu_list"), mode: 0o444 },
    show: Some(blk_mq_hw_sysfs_cpus_show),
};

static default_hw_ctx_attrs: [*const attribute; 4] = [
    &blk_mq_hw_sysfs_nr_tags.attr,
    &blk_mq_hw_sysfs_nr_reserved_tags.attr,
    &blk_mq_hw_sysfs_cpus.attr,
    core::ptr::null(),
];

static blk_mq_hw_sysfs_ops: sysfs_ops = sysfs_ops {
    show: Some(blk_mq_hw_sysfs_show),
};

static blk_mq_ktype: kobj_type = kobj_type {
    release: Some(blk_mq_sysfs_release),
};

static blk_mq_ctx_ktype: kobj_type = kobj_type {
    release: Some(blk_mq_ctx_sysfs_release),
};

static blk_mq_hw_ktype: kobj_type = kobj_type {
    sysfs_ops: &blk_mq_hw_sysfs_ops,
    default_groups: default_hw_ctx_groups,
    release: Some(blk_mq_hw_sysfs_release),
};

unsafe fn blk_mq_unregister_hctx(hctx: *mut blk_mq_hw_ctx) {
    let mut ctx: *mut blk_mq_ctx;
    let mut i: c_int;

    if (*hctx).nr_ctx == 0 {
        return;
    }

    hctx_for_each_ctx!(hctx, ctx, i) {
        if (*ctx).kobj.state_in_sysfs {
            kobject_del(&mut (*ctx).kobj);
        }
    }

    if (*hctx).kobj.state_in_sysfs {
        kobject_del(&mut (*hctx).kobj);
    }
}

unsafe fn blk_mq_register_hctx(hctx: *mut blk_mq_hw_ctx) -> c_int {
    let q = (*hctx).queue;
    let mut ctx: *mut blk_mq_ctx;
    let mut i: c_int;
    let mut j: c_int;
    let mut ret: c_int;

    if (*hctx).nr_ctx == 0 {
        return 0;
    }

    ret = kobject_add(&mut (*hctx).kobj, (*q).mq_kobj, c!("%u"), (*hctx).queue_num);
    if ret != 0 {
        return ret;
    }

    hctx_for_each_ctx!(hctx, ctx, i) {
        ret = kobject_add(&mut (*ctx).kobj, &mut (*hctx).kobj, c!("cpu%u"), (*ctx).cpu);
        if ret != 0 {
            goto!(out);
        }
    }

    return 0;

out:
    hctx_for_each_ctx!(hctx, ctx, j) {
        if j < i {
            kobject_del(&mut (*ctx).kobj);
        }
    }
    kobject_del(&mut (*hctx).kobj);
    ret
}

pub unsafe fn blk_mq_hctx_kobj_init(hctx: *mut blk_mq_hw_ctx) {
    kobject_init(&mut (*hctx).kobj, &blk_mq_hw_ktype);
}

pub unsafe fn blk_mq_sysfs_deinit(q: *mut request_queue) {
    let mut ctx: *mut blk_mq_ctx;
    let mut cpu: c_int;

    for_each_possible_cpu!(cpu) {
        ctx = per_cpu_ptr((*q).queue_ctx, cpu);
        kobject_put(&mut (*ctx).kobj);
    }
    kobject_put((*q).mq_kobj);
}

pub unsafe fn blk_mq_sysfs_init(q: *mut request_queue) {
    let mut ctx: *mut blk_mq_ctx;
    let mut cpu: c_int;

    kobject_init((*q).mq_kobj, &blk_mq_ktype);

    for_each_possible_cpu!(cpu) {
        ctx = per_cpu_ptr((*q).queue_ctx, cpu);

        kobject_get((*q).mq_kobj);
        kobject_init(&mut (*ctx).kobj, &blk_mq_ctx_ktype);
    }
}

pub unsafe fn blk_mq_sysfs_register(disk: *mut gendisk) -> c_int {
    let q = (*disk).queue;
    let mut hctx: *mut blk_mq_hw_ctx;
    let mut i: c_ulong;
    let mut j: c_ulong;
    let mut ret: c_int;

    ret = kobject_add((*q).mq_kobj, &mut (*disk_to_dev(disk)).kobj, c!("mq"));
    if ret < 0 {
        return ret;
    }

    kobject_uevent((*q).mq_kobj, KOBJ_ADD);

    mutex_lock(&mut (*(*q).tag_set).tag_list_lock);
    queue_for_each_hw_ctx!(q, hctx, i) {
        ret = blk_mq_register_hctx(hctx);
        if ret != 0 {
            goto!(out_unreg);
        }
    }
    mutex_unlock(&mut (*(*q).tag_set).tag_list_lock);
    return 0;

out_unreg:
    queue_for_each_hw_ctx!(q, hctx, j) {
        if j < i {
            blk_mq_unregister_hctx(hctx);
        }
    }
    mutex_unlock(&mut (*(*q).tag_set).tag_list_lock);

    kobject_uevent((*q).mq_kobj, KOBJ_REMOVE);
    kobject_del((*q).mq_kobj);
    ret
}

pub unsafe fn blk_mq_sysfs_unregister(disk: *mut gendisk) {
    let q = (*disk).queue;
    let mut hctx: *mut blk_mq_hw_ctx;
    let mut i: c_ulong;

    mutex_lock(&mut (*(*q).tag_set).tag_list_lock);
    queue_for_each_hw_ctx!(q, hctx, i) {
        blk_mq_unregister_hctx(hctx);
    }
    mutex_unlock(&mut (*(*q).tag_set).tag_list_lock);

    kobject_uevent((*q).mq_kobj, KOBJ_REMOVE);
    kobject_del((*q).mq_kobj);
}

pub unsafe fn blk_mq_sysfs_unregister_hctxs(q: *mut request_queue) {
    let mut hctx: *mut blk_mq_hw_ctx;
    let mut i: c_ulong;

    if !blk_queue_registered(q) {
        return;
    }

    queue_for_each_hw_ctx!(q, hctx, i) {
        blk_mq_unregister_hctx(hctx);
    }
}

pub unsafe fn blk_mq_sysfs_register_hctxs(q: *mut request_queue) -> c_int {
    let mut hctx: *mut blk_mq_hw_ctx;
    let mut i: c_ulong;
    let mut ret: c_int = 0;

    if !blk_queue_registered(q) {
        goto!(out);
    }

    queue_for_each_hw_ctx!(q, hctx, i) {
        ret = blk_mq_register_hctx(hctx);
        if ret != 0 {
            break;
        }
    }

out:
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
