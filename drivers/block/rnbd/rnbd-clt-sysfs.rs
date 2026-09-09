// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * RDMA Network Block Driver
 *
 * Copyright (c) 2014 - 2018 ProfitBricks GmbH. All rights reserved.
 * Copyright (c) 2018 - 2019 1&1 IONOS Cloud GmbH. All rights reserved.
 * Copyright (c) 2019 - 2020 1&1 IONOS SE. All rights reserved.
 */

// Kernel includes and symbols from rnbd-clt.h are supplied by other translation units.

static mut rnbd_dev: *mut device = core::ptr::null_mut();
static mut rnbd_dev_class: class = class { name: "rnbd-client\0".as_ptr() as *const i8 };
static mut rnbd_devs_kobj: *mut kobject = core::ptr::null_mut();

const RNBD_OPT_ERR: i32 = 0;
const RNBD_OPT_DEST_PORT: i32 = 1 << 0;
const RNBD_OPT_PATH: i32 = 1 << 1;
const RNBD_OPT_DEV_PATH: i32 = 1 << 2;
const RNBD_OPT_ACCESS_MODE: i32 = 1 << 3;
const RNBD_OPT_SESSNAME: i32 = 1 << 6;
const RNBD_OPT_NR_POLL_QUEUES: i32 = 1 << 7;

static rnbd_opt_mandatory: [u32; 2] = [RNBD_OPT_DEV_PATH as u32, RNBD_OPT_SESSNAME as u32];
static rnbd_opt_tokens: match_table_t = match_table_t::new(); // { path=%s, device_path=%s, dest_port=%d, access_mode=%s, sessname=%s, nr_poll_queues=%d, terminator }

#[repr(C)]
struct rnbd_map_options {
    sessname: *mut i8,
    paths: *mut rtrs_addr,
    path_cnt: *mut usize,
    pathname: *mut i8,
    dest_port: *mut u16,
    access_mode: *mut rnbd_access_mode,
    nr_poll_queues: *mut u32,
}

unsafe fn rnbd_clt_parse_map_options(buf: *const i8, max_path_cnt: usize,
                                      opt: *mut rnbd_map_options) -> i32 {
    let mut options = kstrdup(buf, GFP_KERNEL);
    if options.is_null() { return -ENOMEM; }
    let mut sep_opt = strstrip(options);
    let mut opt_mask: i32 = 0;
    let mut nr_poll_queues: i32 = 0;
    let mut dest_port: i32 = 0;
    let mut p_cnt: i32 = 0;
    let mut ret = -EINVAL;
    let mut args: [substring_t; MAX_OPT_ARGS] = core::mem::zeroed();

    loop {
        let p = strsep(&mut sep_opt, " \0".as_ptr() as *const i8);
        if p.is_null() { break; }
        if *p == 0 { continue; }
        let token = match_token(p, &rnbd_opt_tokens, args.as_mut_ptr());
        opt_mask |= token;
        match token {
            RNBD_OPT_SESSNAME => {
                let q = match_strdup(args.as_mut_ptr());
                if q.is_null() { ret = -ENOMEM; break; }
                if strlen(q) > NAME_MAX { pr_err!("map_device: sessname too long\n"); kfree(q); ret = -EINVAL; break; }
                strscpy((*opt).sessname, q, NAME_MAX); kfree(q);
            }
            RNBD_OPT_PATH => {
                if p_cnt as usize >= max_path_cnt { pr_err!("map_device: too many (> %zu) paths provided\n", max_path_cnt); ret = -ENOMEM; break; }
                let q = match_strdup(args.as_mut_ptr());
                if q.is_null() { ret = -ENOMEM; break; }
                ret = rtrs_addr_to_sockaddr(q, strlen(q), *(*opt).dest_port,
                                            (*(*opt).paths.add(p_cnt as usize)).src);
                if ret != 0 { pr_err!("Can't parse path %s: %d\n", q, ret); kfree(q); break; }
                p_cnt += 1; kfree(q);
            }
            RNBD_OPT_DEV_PATH => {
                let q = match_strdup(args.as_mut_ptr());
                if q.is_null() { ret = -ENOMEM; break; }
                if strlen(q) > NAME_MAX { pr_err!("map_device: Device path too long\n"); kfree(q); ret = -EINVAL; break; }
                strscpy((*opt).pathname, q, NAME_MAX); kfree(q);
            }
            RNBD_OPT_DEST_PORT => {
                if match_int(args.as_mut_ptr(), &mut dest_port) != 0 || dest_port < 0 || dest_port > 65535 {
                    pr_err!("bad destination port number parameter '%d'\n", dest_port); ret = -EINVAL; break;
                }
                *(*opt).dest_port = dest_port as u16;
            }
            RNBD_OPT_ACCESS_MODE => {
                let q = match_strdup(args.as_mut_ptr());
                if q.is_null() { ret = -ENOMEM; break; }
                if strcmp(q, "ro\0".as_ptr() as *const i8) == 0 { *(*opt).access_mode = RNBD_ACCESS_RO; }
                else if strcmp(q, "rw\0".as_ptr() as *const i8) == 0 { *(*opt).access_mode = RNBD_ACCESS_RW; }
                else if strcmp(q, "migration\0".as_ptr() as *const i8) == 0 { *(*opt).access_mode = RNBD_ACCESS_MIGRATION; }
                else { pr_err!("map_device: Invalid access_mode: '%s'\n", q); kfree(q); ret = -EINVAL; break; }
                kfree(q);
            }
            RNBD_OPT_NR_POLL_QUEUES => {
                if match_int(args.as_mut_ptr(), &mut nr_poll_queues) != 0 || nr_poll_queues < -1 || nr_poll_queues > nr_cpu_ids as i32 {
                    pr_err!("bad nr_poll_queues parameter '%d'\n", nr_poll_queues); ret = -EINVAL; break;
                }
                if nr_poll_queues == -1 { nr_poll_queues = nr_cpu_ids as i32; }
                *(*opt).nr_poll_queues = nr_poll_queues as u32;
            }
            _ => { pr_err!("map_device: Unknown parameter or missing value '%s'\n", p); ret = -EINVAL; break; }
        }
    }
    if ret == -EINVAL {
        for i in 0..rnbd_opt_mandatory.len() {
            if (opt_mask & rnbd_opt_mandatory[i] as i32) != 0 { ret = 0; }
            else { pr_err!("map_device: Parameters missing\n"); ret = -EINVAL; break; }
        }
    }
    *(*opt).path_cnt = p_cnt as usize;
    kfree(options);
    ret
}

unsafe fn state_show(kobj: *mut kobject, _attr: *mut kobj_attribute, page: *mut i8) -> isize {
    let dev = container_of!(kobj, rnbd_clt_dev, kobj);
    match (*dev).dev_state {
        DEV_STATE_INIT => sysfs_emit!(page, "init\n"),
        DEV_STATE_MAPPED => { /* TODO fix cli tool before changing to proper state */ sysfs_emit!(page, "open\n") },
        DEV_STATE_MAPPED_DISCONNECTED => { /* TODO fix cli tool before changing to proper state */ sysfs_emit!(page, "closed\n") },
        DEV_STATE_UNMAPPED => sysfs_emit!(page, "unmapped\n"),
        _ => sysfs_emit!(page, "unknown\n"),
    }
}

static mut rnbd_clt_state_attr: kobj_attribute = __ATTR_RO!(state);

unsafe fn nr_poll_queues_show(kobj: *mut kobject, _attr: *mut kobj_attribute, page: *mut i8) -> isize {
    let dev = container_of!(kobj, rnbd_clt_dev, kobj); sysfs_emit!(page, "%d\n", (*dev).nr_poll_queues)
}
static mut rnbd_clt_nr_poll_queues: kobj_attribute = __ATTR_RO!(nr_poll_queues);

unsafe fn mapping_path_show(kobj: *mut kobject, _attr: *mut kobj_attribute, page: *mut i8) -> isize {
    let dev = container_of!(kobj, rnbd_clt_dev, kobj); sysfs_emit!(page, "%s\n", (*dev).pathname)
}
static mut rnbd_clt_mapping_path_attr: kobj_attribute = __ATTR_RO!(mapping_path);

unsafe fn access_mode_show(kobj: *mut kobject, _attr: *mut kobj_attribute, page: *mut i8) -> isize {
    let dev = container_of!(kobj, rnbd_clt_dev, kobj); sysfs_emit!(page, "%s\n", rnbd_access_modes[(*dev).access_mode].str_)
}
static mut rnbd_clt_access_mode: kobj_attribute = __ATTR_RO!(access_mode);

unsafe fn rnbd_clt_unmap_dev_show(_kobj: *mut kobject, attr: *mut kobj_attribute, page: *mut i8) -> isize {
    sysfs_emit!(page, "Usage: echo <normal|force> > %s\n", (*attr).attr.name)
}

unsafe fn rnbd_clt_unmap_dev_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *const i8, count: usize) -> isize {
    let opt = kstrdup(buf, GFP_KERNEL); if opt.is_null() { return -ENOMEM; }
    let options = strstrip(opt); let dev = container_of!(kobj, rnbd_clt_dev, kobj);
    let force = if sysfs_streq(options, "normal\0".as_ptr() as *const i8) { false }
        else if sysfs_streq(options, "force\0".as_ptr() as *const i8) { true }
        else { rnbd_clt_err!(dev, "unmap_device: Invalid value: %s\n", options); kfree(opt); return -EINVAL; };
    rnbd_clt_info!(dev, "Unmapping device, option: %s.\n", if force { "force" } else { "normal" });
    if !try_module_get(THIS_MODULE) { kfree(opt); return -ENODEV; }
    let mut err = rnbd_clt_unmap_device(dev, force, &mut (*attr).attr);
    if err != 0 { if err != -EALREADY { rnbd_clt_err!(dev, "unmap_device: %d\n", err); } module_put(THIS_MODULE); kfree(opt); return err; }
    err = count as isize; module_put(THIS_MODULE); kfree(opt); err
}
static mut rnbd_clt_unmap_device_attr: kobj_attribute = __ATTR!(unmap_device, 0o644, rnbd_clt_unmap_dev_show, rnbd_clt_unmap_dev_store);

unsafe fn rnbd_clt_resize_dev_show(_kobj: *mut kobject, attr: *mut kobj_attribute, page: *mut i8) -> isize { sysfs_emit!(page, "Usage: echo <new size in sectors> > %s\n", (*attr).attr.name) }
unsafe fn rnbd_clt_resize_dev_store(kobj: *mut kobject, _attr: *mut kobj_attribute, buf: *const i8, count: usize) -> isize {
    let dev = container_of!(kobj, rnbd_clt_dev, kobj); let mut sectors = 0UL; let ret = kstrtoul(buf, 0, &mut sectors); if ret != 0 { return ret as isize; }
    let ret = rnbd_clt_resize_disk(dev, sectors); if ret != 0 { return ret as isize; } count as isize
}
static mut rnbd_clt_resize_dev_attr: kobj_attribute = __ATTR!(resize, 0o644, rnbd_clt_resize_dev_show, rnbd_clt_resize_dev_store);

unsafe fn rnbd_clt_remap_dev_show(_kobj: *mut kobject, attr: *mut kobj_attribute, page: *mut i8) -> isize { sysfs_emit!(page, "Usage: echo <1> > %s\n", (*attr).attr.name) }
unsafe fn rnbd_clt_remap_dev_store(kobj: *mut kobject, _attr: *mut kobj_attribute, buf: *const i8, count: usize) -> isize {
    let opt = kstrdup(buf, GFP_KERNEL); if opt.is_null() { return -ENOMEM; } let options = strstrip(opt); let dev = container_of!(kobj, rnbd_clt_dev, kobj);
    if !sysfs_streq(options, "1\0".as_ptr() as *const i8) { rnbd_clt_err!(dev, "remap_device: Invalid value: %s\n", options); kfree(opt); return -EINVAL; }
    let mut err = rnbd_clt_remap_device(dev); if err == 0 { err = count as i32; } kfree(opt); err as isize
}
static mut rnbd_clt_remap_device_attr: kobj_attribute = __ATTR!(remap_device, 0o644, rnbd_clt_remap_dev_show, rnbd_clt_remap_dev_store);

unsafe fn session_show(kobj: *mut kobject, _attr: *mut kobj_attribute, page: *mut i8) -> isize { let dev = container_of!(kobj, rnbd_clt_dev, kobj); sysfs_emit!(page, "%s\n", (*(*dev).sess).sessname) }
static mut rnbd_clt_session_attr: kobj_attribute = __ATTR_RO!(session);

static mut rnbd_dev_attrs: [*mut attribute; 9] = [
    &mut rnbd_clt_unmap_device_attr.attr, &mut rnbd_clt_resize_dev_attr.attr, &mut rnbd_clt_remap_device_attr.attr,
    &mut rnbd_clt_mapping_path_attr.attr, &mut rnbd_clt_state_attr.attr, &mut rnbd_clt_session_attr.attr,
    &mut rnbd_clt_access_mode.attr, &mut rnbd_clt_nr_poll_queues.attr, core::ptr::null_mut(),
];

unsafe fn rnbd_clt_remove_dev_symlink(dev: *mut rnbd_clt_dev) {
    if !(*dev).blk_symlink_name.is_null() { if try_module_get(THIS_MODULE) { sysfs_remove_link(rnbd_devs_kobj, (*dev).blk_symlink_name); module_put(THIS_MODULE); } kfree((*dev).blk_symlink_name); (*dev).blk_symlink_name = core::ptr::null_mut(); }
}
unsafe fn rnbd_dev_release(kobj: *mut kobject) { let dev = container_of!(kobj, rnbd_clt_dev, kobj); kfree(dev); }
static mut rnbd_dev_ktype: kobj_type = kobj_type { sysfs_ops: &kobj_sysfs_ops, default_groups: rnbd_dev_groups, release: Some(rnbd_dev_release) };

unsafe fn rnbd_clt_add_dev_kobj(dev: *mut rnbd_clt_dev) -> i32 {
    let gd_kobj = &mut (*disk_to_dev((*dev).gd)).kobj; let ret = kobject_init_and_add(&mut (*dev).kobj, &rnbd_dev_ktype, gd_kobj, "%s\0".as_ptr() as *const i8, "rnbd\0".as_ptr() as *const i8);
    if ret != 0 { rnbd_clt_err!(dev, "Failed to create device sysfs dir, err: %d\n", ret); kobject_put(&mut (*dev).kobj); } kobject_uevent(gd_kobj, KOBJ_ONLINE); ret
}

unsafe fn rnbd_clt_get_path_name(dev: *mut rnbd_clt_dev, buf: *mut i8, len: usize) -> i32 {
    let mut pathname: [i8; NAME_MAX] = core::mem::zeroed(); strscpy(pathname.as_mut_ptr(), (*dev).pathname, pathname.len()); let mut s = strchr(pathname.as_mut_ptr(), b'/' as i32); while !s.is_null() { *s = b'!' as i8; s = strchr(pathname.as_mut_ptr(), b'/' as i32); }
    let ret = snprintf(buf, len, "%s@%s\0".as_ptr() as *const i8, pathname.as_ptr(), (*(*dev).sess).sessname); if ret >= len as i32 { -ENAMETOOLONG } else { 0 }
}

unsafe fn rnbd_clt_add_dev_symlink(dev: *mut rnbd_clt_dev) -> i32 {
    let gd_kobj = &mut (*disk_to_dev((*dev).gd)).kobj; let len = strlen((*dev).pathname) + strlen((*(*dev).sess).sessname) + 2; (*dev).blk_symlink_name = kzalloc(len, GFP_KERNEL); if (*dev).blk_symlink_name.is_null() { rnbd_clt_err!(dev, "Failed to allocate memory for blk_symlink_name\n"); return -ENOMEM; }
    let ret = rnbd_clt_get_path_name(dev, (*dev).blk_symlink_name, len); if ret != 0 { rnbd_clt_err!(dev, "Failed to get /sys/block symlink path, err: %d\n", ret); kfree((*dev).blk_symlink_name); (*dev).blk_symlink_name = core::ptr::null_mut(); return ret; }
    let ret = sysfs_create_link(rnbd_devs_kobj, gd_kobj, (*dev).blk_symlink_name); if ret != 0 { rnbd_clt_err!(dev, "Creating /sys/block symlink failed, err: %d\n", ret); kfree((*dev).blk_symlink_name); (*dev).blk_symlink_name = core::ptr::null_mut(); } ret
}

unsafe fn rnbd_clt_map_device_show(_kobj: *mut kobject, attr: *mut kobj_attribute, page: *mut i8) -> isize { sysfs_emit!(page, "Usage: echo \"[dest_port=server port number] sessname=<name of the rtrs session> path=<[srcaddr@]dstaddr> [path=<[srcaddr@]dstaddr>] device_path=<full path on remote side> [access_mode=<ro|rw|migration>] [nr_poll_queues=<number of queues>]\" > %s\n\naddr ::= [ ip:<ipv4> | ip:<ipv6> | gid:<gid> ]\n", (*attr).attr.name) }

unsafe fn rnbd_clt_map_device_store(_kobj: *mut kobject, _attr: *mut kobj_attribute, buf: *const i8, count: usize) -> isize {
    let mut opt: rnbd_map_options = core::mem::zeroed();
    let mut pathname: [i8; NAME_MAX] = core::mem::zeroed();
    let mut sessname: [i8; NAME_MAX] = core::mem::zeroed();
    let mut access_mode = RNBD_ACCESS_RW;
    let mut port_nr: u16 = RTRS_PORT as u16;
    let mut nr_poll_queues: u32 = 0;
    let mut paths: [rtrs_addr; 6] = core::mem::zeroed();
    let mut path_cnt: usize = 0;
    let addrs = kzalloc_objs::<sockaddr_storage>(12, GFP_KERNEL);
    if addrs.is_null() { return -ENOMEM; }
    opt.sessname = sessname.as_mut_ptr(); opt.paths = paths.as_mut_ptr(); opt.path_cnt = &mut path_cnt;
    opt.pathname = pathname.as_mut_ptr(); opt.dest_port = &mut port_nr; opt.access_mode = &mut access_mode; opt.nr_poll_queues = &mut nr_poll_queues;
    for i in 0..paths.len() { paths[i].src = addrs.add(i * 2); paths[i].dst = addrs.add(i * 2 + 1); }
    let mut ret = rnbd_clt_parse_map_options(buf, paths.len(), &mut opt);
    if ret != 0 { kfree(addrs); return ret as isize; }
    pr_info!("Mapping device %s on session %s, (access_mode: %s, nr_poll_queues: %d)\n", pathname.as_ptr(), sessname.as_ptr(), rnbd_access_modes[access_mode].str_, nr_poll_queues);
    let dev = rnbd_clt_map_device(sessname.as_mut_ptr(), paths.as_mut_ptr(), path_cnt, port_nr, pathname.as_mut_ptr(), access_mode, nr_poll_queues);
    if IS_ERR(dev) { ret = PTR_ERR(dev); kfree(addrs); return ret as isize; }
    ret = rnbd_clt_add_dev_kobj(dev);
    if ret != 0 { rnbd_clt_unmap_device(dev, true, core::ptr::null_mut()); kfree(addrs); return ret as isize; }
    ret = rnbd_clt_add_dev_symlink(dev);
    if ret != 0 { rnbd_clt_unmap_device(dev, true, core::ptr::null_mut()); kfree(addrs); return ret as isize; }
    kfree(addrs); count as isize
}
static mut rnbd_clt_map_device_attr: kobj_attribute = __ATTR!(map_device, 0o644, rnbd_clt_map_device_show, rnbd_clt_map_device_store);

unsafe fn rnbd_clt_create_sysfs_files() -> i32 { let mut err = class_register(&mut rnbd_dev_class); if err != 0 { return err; } rnbd_dev = device_create_with_groups(&rnbd_dev_class, core::ptr::null_mut(), MKDEV(0, 0), core::ptr::null_mut(), default_attr_groups, "ctl\0".as_ptr() as *const i8); if IS_ERR(rnbd_dev) { err = PTR_ERR(rnbd_dev); class_unregister(&mut rnbd_dev_class); return err; } rnbd_devs_kobj = kobject_create_and_add("devices\0".as_ptr() as *const i8, &mut (*rnbd_dev).kobj); if rnbd_devs_kobj.is_null() { device_destroy(&rnbd_dev_class, MKDEV(0, 0)); class_unregister(&mut rnbd_dev_class); return -ENOMEM; } 0 }
unsafe fn rnbd_clt_destroy_sysfs_files() { sysfs_remove_group(&mut (*rnbd_dev).kobj, &default_attr_group); kobject_del(rnbd_devs_kobj); kobject_put(rnbd_devs_kobj); device_destroy(&rnbd_dev_class, MKDEV(0, 0)); class_unregister(&mut rnbd_dev_class); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
