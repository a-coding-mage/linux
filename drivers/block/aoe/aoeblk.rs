/* Copyright (c) 2013 Coraid, Inc. See COPYING for GPL terms. */
/*
 * aoeblk.c
 * block device routines
 *
 * Kernel and project declarations supplied by the surrounding translation.
 */

static mut AOEBLK_MUTEX: Mutex = DEFINE_MUTEX!();
static mut BUF_POOL_CACHE: *mut kmem_cache = core::ptr::null_mut();
static mut AOE_DEBUGFS_DIR: *mut dentry = core::ptr::null_mut();

/* random default picked from the historic block max_sectors cap */
static mut aoe_maxsectors: i32 = 2560;
module_param!(aoe_maxsectors, i32, 0o644);
module_param_desc!(aoe_maxsectors,
    "When nonzero, set the maximum number of sectors per I/O request");

unsafe extern "C" fn aoedisk_show_state(
    dev: *mut device,
    _attr: *mut device_attribute,
    page: *mut c_char,
) -> ssize_t {
    let disk = dev_to_disk(dev);
    let d = (*disk).private_data as *mut aoedev;

    sysfs_emit!(page, "{}{}\n",
        if (*d).flags & DEVFL_UP != 0 { "up" } else { "down" },
        if (*d).flags & DEVFL_KICKME != 0 { ",kickme" }
        else if (*d).nopen != 0 && (*d).flags & DEVFL_UP == 0 { ",closewait" }
        else { "" });
    /* I'd rather see nopen exported so we can ditch closewait */
}

unsafe extern "C" fn aoedisk_show_mac(
    dev: *mut device, _attr: *mut device_attribute, page: *mut c_char,
) -> ssize_t {
    let disk = dev_to_disk(dev);
    let d = (*disk).private_data as *mut aoedev;
    let t = (*d).targets[0];
    if t.is_null() { return sysfs_emit!(page, "none\n"); }
    sysfs_emit!(page, "%pm\n", (*t).addr)
}

unsafe extern "C" fn aoedisk_show_netif(
    dev: *mut device, _attr: *mut device_attribute, page: *mut c_char,
) -> ssize_t {
    let disk = dev_to_disk(dev);
    let d = (*disk).private_data as *mut aoedev;
    let mut nds: [*mut net_device; 8] = [core::ptr::null_mut(); 8];
    let mut nd = nds.as_mut_ptr();
    let ne = nd.add(nds.len());
    let mut t = (*d).targets;
    let te = t.add((*d).ntargets as usize);
    while t < te && !(*t).is_null() {
        let mut ifp = (*(*t)).ifs;
        let e = ifp.add(NAOEIFS as usize);
        while ifp < e && !(*ifp).nd.is_null() {
            let mut nnd = nds.as_mut_ptr();
            while nnd < nd && **nnd != (*ifp).nd { nnd = nnd.add(1); }
            if nnd == nd && nd != ne { *nd = (*ifp).nd; nd = nd.add(1); }
            ifp = ifp.add(1);
        }
        t = t.add(1);
    }
    let ne = nd;
    nd = nds.as_mut_ptr();
    if (*nd).is_null() { return sysfs_emit!(page, "none\n"); }
    let mut p = page;
    while nd < ne {
        p = p.add(scnprintf!(p, PAGE_SIZE - p.offset_from(page) as usize,
            "{}{}", if p == page { "" } else { "," }, cstr!((*nd).name)) as usize);
        nd = nd.add(1);
    }
    p = p.add(scnprintf!(p, PAGE_SIZE - p.offset_from(page) as usize, "\n") as usize);
    p.offset_from(page)
}

/* firmware version */
unsafe extern "C" fn aoedisk_show_fwver(
    dev: *mut device, _attr: *mut device_attribute, page: *mut c_char,
) -> ssize_t {
    let disk = dev_to_disk(dev);
    let d = (*disk).private_data as *mut aoedev;
    sysfs_emit!(page, "0x{:04x}\n", (*d).fw_ver as u32)
}

unsafe extern "C" fn aoedisk_show_payload(
    dev: *mut device, _attr: *mut device_attribute, page: *mut c_char,
) -> ssize_t {
    let disk = dev_to_disk(dev);
    let d = (*disk).private_data as *mut aoedev;
    sysfs_emit!(page, "{}\n", (*d).maxbcnt)
}

unsafe extern "C" fn aoe_debugfs_show(s: *mut seq_file, _ignored: *mut c_void) -> i32 {
    let d = (*s).private as *mut aoedev;
    let mut t = (*d).targets;
    let te = t.add((*d).ntargets as usize);
    seq_printf!(s, "rttavg: {} rttdev: {}\n", (*d).rttavg >> RTTSCALE, (*d).rttdev >> RTTDSCALE);
    seq_printf!(s, "nskbpool: {}\n", skb_queue_len(&(*d).skbpool));
    seq_printf!(s, "kicked: {}\n", (*d).kicked);
    seq_printf!(s, "maxbcnt: {}\n", (*d).maxbcnt);
    seq_printf!(s, "ref: {}\n", (*d).ref_);
    let mut flags: ulong = 0;
    spin_lock_irqsave!(&(*d).lock, flags);
    while t < te && !(*t).is_null() {
        let mut c = b'\t';
        seq_printf!(s, "falloc: {}\n", (*(*t)).falloc);
        seq_printf!(s, "ffree: {:p}\n", if list_empty!(&(*(*t)).ffree) { core::ptr::null_mut() } else { (*(*t)).ffree.next });
        seq_printf!(s, "%pm:{}:{}:{}\n", (*(*t)).addr, (*(*t)).nout, (*(*t)).maxout, (*(*t)).nframes);
        seq_printf!(s, "\tssthresh:{}\n", (*(*t)).ssthresh);
        seq_printf!(s, "\ttaint:{}\n", (*(*t)).taint);
        seq_printf!(s, "\tr:{}\n", (*(*t)).rpkts);
        seq_printf!(s, "\tw:{}\n", (*(*t)).wpkts);
        let mut ifp = (*(*t)).ifs;
        let ife = ifp.add(core::mem::size_of_val(&(*(*t)).ifs) / core::mem::size_of::<aoeif>());
        while ifp < ife && !(*ifp).nd.is_null() {
            seq_printf!(s, "{}{}", c as char, cstr!((*ifp).nd.name));
            c = b','; ifp = ifp.add(1);
        }
        seq_puts!(s, "\n"); t = t.add(1);
    }
    spin_unlock_irqrestore!(&(*d).lock, flags);
    0
}

DEFINE_SHOW_ATTRIBUTE!(aoe_debugfs);
static DEVICE_ATTR!(state, 0o444, aoedisk_show_state, None);
static DEVICE_ATTR!(mac, 0o444, aoedisk_show_mac, None);
static DEVICE_ATTR!(netif, 0o444, aoedisk_show_netif, None);
static mut dev_attr_firmware_version: device_attribute = device_attribute {
    attr: attribute { name: cstr_ptr!("firmware-version"), mode: 0o444 },
    show: Some(aoedisk_show_fwver), store: None,
};
static DEVICE_ATTR!(payload, 0o444, aoedisk_show_payload, None);

static mut AOE_ATTRS: [*mut attribute; 6] = [
    &mut dev_attr_state.attr, &mut dev_attr_mac.attr, &mut dev_attr_netif.attr,
    &mut dev_attr_firmware_version.attr, &mut dev_attr_payload.attr, core::ptr::null_mut(),
];
static AOE_ATTR_GROUP: attribute_group = attribute_group { attrs: AOE_ATTRS.as_mut_ptr() };
static AOE_ATTR_GROUPS: [*const attribute_group; 2] = [&AOE_ATTR_GROUP, core::ptr::null()];

unsafe fn aoedisk_add_debugfs(d: *mut aoedev) {
    if AOE_DEBUGFS_DIR.is_null() { return; }
    let mut p = strchr!((*(*d).gd).disk_name, b'/');
    if p.is_null() { p = (*(*d).gd).disk_name.as_mut_ptr(); } else { p = p.add(1); }
    BUG_ON!(*p == 0);
    (*d).debugfs = debugfs_create_file!(p, 0o444, AOE_DEBUGFS_DIR, d, &aoe_debugfs_fops);
}

pub unsafe extern "C" fn aoedisk_rm_debugfs(d: *mut aoedev) {
    debugfs_remove!((*d).debugfs); (*d).debugfs = core::ptr::null_mut();
}

unsafe extern "C" fn aoeblk_open(disk: *mut gendisk, _mode: blk_mode_t) -> i32 {
    let d = (*disk).private_data as *mut aoedev; let mut flags: ulong = 0;
    if !virt_addr_valid!(d) { pr_crit!("aoe: invalid device pointer in {}\n", "aoeblk_open"); WARN_ON!(true); return -ENODEV; }
    if (*d).flags & DEVFL_UP == 0 || (*d).flags & DEVFL_TKILL != 0 { return -ENODEV; }
    mutex_lock!(&mut AOEBLK_MUTEX); spin_lock_irqsave!(&(*d).lock, flags);
    if (*d).flags & DEVFL_UP != 0 && (*d).flags & DEVFL_TKILL == 0 { (*d).nopen += 1; spin_unlock_irqrestore!(&(*d).lock, flags); mutex_unlock!(&mut AOEBLK_MUTEX); return 0; }
    spin_unlock_irqrestore!(&(*d).lock, flags); mutex_unlock!(&mut AOEBLK_MUTEX); -ENODEV
}

unsafe extern "C" fn aoeblk_release(disk: *mut gendisk) {
    let d = (*disk).private_data as *mut aoedev; let mut flags: ulong = 0;
    spin_lock_irqsave!(&(*d).lock, flags); (*d).nopen -= 1;
    if (*d).nopen == 0 { spin_unlock_irqrestore!(&(*d).lock, flags); aoecmd_cfg!((*d).aoemajor, (*d).aoeminor); return; }
    spin_unlock_irqrestore!(&(*d).lock, flags);
}

unsafe extern "C" fn aoeblk_queue_rq(hctx: *mut blk_mq_hw_ctx, bd: *const blk_mq_queue_data) -> blk_status_t {
    let d = (*(*hctx).queue).queuedata as *mut aoedev; spin_lock_irq!(&(*d).lock);
    if (*d).flags & DEVFL_UP == 0 { pr_info_ratelimited!("aoe: device {}.{} is not up\n", (*d).aoemajor, (*d).aoeminor); spin_unlock_irq!(&(*d).lock); blk_mq_start_request!((*bd).rq); return BLK_STS_IOERR; }
    list_add_tail!(&(*(*bd).rq).queuelist, &(*d).rq_list); aoecmd_work!(d); spin_unlock_irq!(&(*d).lock); BLK_STS_OK
}

unsafe extern "C" fn aoeblk_getgeo(disk: *mut gendisk, geo: *mut hd_geometry) -> i32 {
    let d = (*disk).private_data as *mut aoedev;
    if (*d).flags & DEVFL_UP == 0 { printk!(KERN_ERR "aoe: disk not up\n"); return -ENODEV; }
    (*geo).cylinders = (*d).geo.cylinders; (*geo).heads = (*d).geo.heads; (*geo).sectors = (*d).geo.sectors; 0
}

unsafe extern "C" fn aoeblk_ioctl(bdev: *mut block_device, _mode: blk_mode_t, cmd: uint, arg: ulong) -> i32 {
    if arg == 0 { return -EINVAL; }
    let d = (*(*bdev).bd_disk).private_data as *mut aoedev;
    if (*d).flags & DEVFL_UP == 0 { pr_err!("aoe: disk not up\n"); return -ENODEV; }
    if cmd == HDIO_GET_IDENTITY { if copy_to_user!(arg as *mut c_void, &(*d).ident, core::mem::size_of_val(&(*d).ident)) == 0 { return 0; } return -EFAULT; }
    if cmd != SG_IO { pr_info!("aoe: unknown ioctl 0x{:x}\n", cmd); } -ENOTTY
}

static AOE_BDOPS: block_device_operations = block_device_operations { open: Some(aoeblk_open), release: Some(aoeblk_release), ioctl: Some(aoeblk_ioctl), compat_ioctl: Some(blkdev_compat_ptr_ioctl), getgeo: Some(aoeblk_getgeo), owner: THIS_MODULE };
static AOE_MQ_OPS: blk_mq_ops = blk_mq_ops { queue_rq: Some(aoeblk_queue_rq) };

/* blk_mq_alloc_disk and add_disk can sleep */
pub unsafe extern "C" fn aoeblk_gdalloc(vp: *mut c_void) {
    let d = vp as *mut aoedev; let mut gd: *mut gendisk; let mut mp: *mut mempool_t; let mut set: *mut blk_mq_tag_set; let ssize: sector_t; let mut flags: ulong = 0; let mut late = 0; let mut err: i32;
    let lim = queue_limits { max_hw_sectors: aoe_maxsectors, io_opt: SZ_2M, features: BLK_FEAT_ROTATIONAL };
    spin_lock_irqsave!(&(*d).lock, flags);
    if (*d).flags & DEVFL_GDALLOC != 0 && (*d).flags & DEVFL_TKILL == 0 && (*d).flags & DEVFL_GD_NOW == 0 { (*d).flags |= DEVFL_GD_NOW; } else { late = 1; }
    spin_unlock_irqrestore!(&(*d).lock, flags); if late != 0 { return; }
    mp = mempool_create!(MIN_BUFS, mempool_alloc_slab, mempool_free_slab, BUF_POOL_CACHE);
    if mp.is_null() { printk!(KERN_ERR "aoe: cannot allocate bufpool for {}.{}\n", (*d).aoemajor, (*d).aoeminor); goto err; }
    set = &mut (*d).tag_set; (*set).ops = &AOE_MQ_OPS; (*set).cmd_size = core::mem::size_of::<aoe_req>(); (*set).nr_hw_queues = 1; (*set).queue_depth = 128; (*set).numa_node = NUMA_NO_NODE;
    err = blk_mq_alloc_tag_set!(set); if err != 0 { pr_err!("aoe: cannot allocate tag set for {}.{}\n", (*d).aoemajor, (*d).aoeminor); goto err_mempool; }
    gd = blk_mq_alloc_disk!(set, &lim, d); if IS_ERR!(gd) { pr_err!("aoe: cannot allocate block queue for {}.{}\n", (*d).aoemajor, (*d).aoeminor); goto err_tagset; }
    spin_lock_irqsave!(&(*d).lock, flags); WARN_ON!((*d).flags & DEVFL_GD_NOW == 0); WARN_ON!((*d).flags & DEVFL_GDALLOC == 0); WARN_ON!((*d).flags & DEVFL_TKILL != 0); WARN_ON!(!(*d).gd.is_null()); WARN_ON!((*d).flags & DEVFL_UP != 0);
    (*d).bufpool = mp; (*d).blkq = (*gd).queue; (*d).gd = gd; (*gd).major = AOE_MAJOR; (*gd).first_minor = (*d).sysminor; (*gd).minors = AOE_PARTITIONS; (*gd).fops = &AOE_BDOPS; (*gd).private_data = d; ssize = (*d).ssize; snprintf!((*gd).disk_name, "etherd/e{}.{}", (*d).aoemajor, (*d).aoeminor); (*d).flags &= !DEVFL_GDALLOC; (*d).flags |= DEVFL_UP; spin_unlock_irqrestore!(&(*d).lock, flags);
    set_capacity!(gd, ssize); err = device_add_disk!(core::ptr::null_mut(), gd, AOE_ATTR_GROUPS.as_ptr()); if err != 0 { goto out_disk_cleanup; } aoedisk_add_debugfs(d); spin_lock_irqsave!(&(*d).lock, flags); WARN_ON!((*d).flags & DEVFL_GD_NOW == 0); (*d).flags &= !DEVFL_GD_NOW; spin_unlock_irqrestore!(&(*d).lock, flags); return;
out_disk_cleanup: put_disk!(gd);
err_tagset: blk_mq_free_tag_set!(set);
err_mempool: mempool_destroy!(mp);
err: spin_lock_irqsave!(&(*d).lock, flags); (*d).flags &= !DEVFL_GD_NOW; queue_work!(aoe_wq, &mut (*d).work); spin_unlock_irqrestore!(&(*d).lock, flags);
}

pub unsafe extern "C" fn aoeblk_exit() { debugfs_remove_recursive!(AOE_DEBUGFS_DIR); AOE_DEBUGFS_DIR = core::ptr::null_mut(); kmem_cache_destroy!(BUF_POOL_CACHE); }

pub unsafe extern "C" fn aoeblk_init() -> i32 {
    BUF_POOL_CACHE = kmem_cache_create!("aoe_bufs", core::mem::size_of::<buf>(), 0, 0, None);
    if BUF_POOL_CACHE.is_null() { return -ENOMEM; }
    AOE_DEBUGFS_DIR = debugfs_create_dir!("aoe", core::ptr::null_mut()); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
