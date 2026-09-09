// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the surrounding kernel/null_blk translation.

const NULL_ZONE_INVALID_WP: sector_t = (-1isize) as sector_t;

#[inline]
unsafe fn mb_to_sects(mb: c_ulong) -> sector_t {
    ((mb as sector_t).wrapping_mul(SZ_1M as sector_t)) >> SECTOR_SHIFT
}

#[inline]
unsafe fn null_zone_no(dev: *mut nullb_device, sect: sector_t) -> c_uint {
    if (*dev).zone_size_sects == 0 {
        WARN_ON_ONCE(true);
        return 0;
    }
    sect >> ilog2((*dev).zone_size_sects)
}

#[inline]
unsafe fn null_init_zone_lock(dev: *mut nullb_device, zone: *mut nullb_zone) {
    if !(*dev).memory_backed {
        spin_lock_init(&mut (*zone).spinlock);
    } else {
        mutex_init(&mut (*zone).mutex);
    }
}

#[inline]
unsafe fn null_lock_zone(dev: *mut nullb_device, zone: *mut nullb_zone) {
    if !(*dev).memory_backed {
        spin_lock_irq(&mut (*zone).spinlock);
    } else {
        mutex_lock(&mut (*zone).mutex);
    }
}

#[inline]
unsafe fn null_unlock_zone(dev: *mut nullb_device, zone: *mut nullb_zone) {
    if !(*dev).memory_backed {
        spin_unlock_irq(&mut (*zone).spinlock);
    } else {
        mutex_unlock(&mut (*zone).mutex);
    }
}

pub unsafe fn null_init_zoned_dev(
    dev: *mut nullb_device,
    lim: *mut queue_limits,
) -> c_int {
    let mut dev_capacity_sects: sector_t;
    let mut zone_capacity_sects: sector_t;
    let mut zone: *mut nullb_zone;
    let mut sector: sector_t = 0;
    let mut i: c_uint;

    if (*dev).zone_size == 0 || !is_power_of_2((*dev).zone_size) {
        pr_err!("zone_size must be non-zero power-of-two\n");
        return -EINVAL;
    }
    if (*dev).zone_size > (*dev).size {
        pr_err!("Zone size larger than device capacity\n");
        return -EINVAL;
    }

    if (*dev).zone_capacity == 0 {
        (*dev).zone_capacity = (*dev).zone_size;
    }
    if (*dev).zone_capacity > (*dev).zone_size {
        pr_err!("zone capacity (%lu MB) larger than zone size (%lu MB)\n", (*dev).zone_capacity, (*dev).zone_size);
        return -EINVAL;
    }
    if (*dev).zone_capacity != (*dev).zone_size
        && ((*dev).size & ((*dev).zone_size - 1)) != 0
    {
        pr_err!("A smaller last zone is not allowed with zone capacity smaller than zone size.\n");
        return -EINVAL;
    }

    zone_capacity_sects = mb_to_sects((*dev).zone_capacity);
    dev_capacity_sects = mb_to_sects((*dev).size);
    (*dev).zone_size_sects = mb_to_sects((*dev).zone_size);
    if (*dev).zone_size_sects == 0 {
        pr_err!("zone_size too large or too small, leads to zero sectors\n");
        return -EINVAL;
    }
    (*dev).nr_zones = round_up(dev_capacity_sects, (*dev).zone_size_sects)
        >> ilog2((*dev).zone_size_sects);

    (*dev).zones = kvmalloc_objs::<nullb_zone>((*dev).nr_zones, GFP_KERNEL | __GFP_ZERO);
    if (*dev).zones.is_null() {
        return -ENOMEM;
    }

    spin_lock_init(&mut (*dev).zone_res_lock);
    if (*dev).zone_nr_conv >= (*dev).nr_zones {
        (*dev).zone_nr_conv = (*dev).nr_zones - 1;
        pr_info!("changed the number of conventional zones to %u", (*dev).zone_nr_conv);
    }

    (*dev).zone_append_max_sectors = min(
        ALIGN_DOWN((*dev).zone_append_max_sectors, (*dev).blocksize >> SECTOR_SHIFT),
        zone_capacity_sects,
    );
    if (*dev).zone_max_active >= (*dev).nr_zones - (*dev).zone_nr_conv {
        (*dev).zone_max_active = 0;
        pr_info!("zone_max_active limit disabled, limit >= zone count\n");
    }
    if (*dev).zone_max_active != 0 && (*dev).zone_max_open > (*dev).zone_max_active {
        (*dev).zone_max_open = (*dev).zone_max_active;
        pr_info!("changed the maximum number of open zones to %u\n", (*dev).zone_max_open);
    } else if (*dev).zone_max_open >= (*dev).nr_zones - (*dev).zone_nr_conv {
        (*dev).zone_max_open = 0;
        pr_info!("zone_max_open limit disabled, limit >= zone count\n");
    }
    (*dev).need_zone_res_mgmt = (*dev).zone_max_active != 0 || (*dev).zone_max_open != 0;
    (*dev).imp_close_zone_no = (*dev).zone_nr_conv;

    for i in 0..(*dev).zone_nr_conv {
        zone = (*dev).zones.add(i as usize);
        null_init_zone_lock(dev, zone);
        (*zone).start = sector;
        (*zone).len = (*dev).zone_size_sects;
        (*zone).capacity = (*zone).len;
        (*zone).wp = (*zone).start + (*zone).len;
        (*zone).type_ = BLK_ZONE_TYPE_CONVENTIONAL;
        (*zone).cond = BLK_ZONE_COND_NOT_WP;
        sector += (*dev).zone_size_sects;
    }
    for i in (*dev).zone_nr_conv..(*dev).nr_zones {
        zone = (*dev).zones.add(i as usize);
        null_init_zone_lock(dev, zone);
        (*zone).start = sector;
        (*zone).len = if (*zone).start + (*dev).zone_size_sects > dev_capacity_sects {
            dev_capacity_sects - (*zone).start
        } else { (*dev).zone_size_sects };
        (*zone).capacity = min((*zone).len, zone_capacity_sects);
        (*zone).type_ = BLK_ZONE_TYPE_SEQWRITE_REQ;
        if (*dev).zone_full {
            (*zone).cond = BLK_ZONE_COND_FULL;
            (*zone).wp = (*zone).start + (*zone).capacity;
        } else {
            (*zone).cond = BLK_ZONE_COND_EMPTY;
            (*zone).wp = (*zone).start;
        }
        sector += (*dev).zone_size_sects;
    }
    (*lim).features |= BLK_FEAT_ZONED;
    (*lim).chunk_sectors = (*dev).zone_size_sects;
    (*lim).max_hw_zone_append_sectors = (*dev).zone_append_max_sectors;
    (*lim).max_open_zones = (*dev).zone_max_open;
    (*lim).max_active_zones = (*dev).zone_max_active;
    0
}

pub unsafe fn null_register_zoned_dev(nullb: *mut nullb) -> c_int {
    let q = (*nullb).q;
    let disk = (*nullb).disk;
    pr_info!("%s: using %s zone append\n", (*disk).disk_name,
        if queue_emulates_zone_append(q) { "emulated" } else { "native" });
    blk_revalidate_disk_zones(disk)
}

pub unsafe fn null_free_zoned_dev(dev: *mut nullb_device) {
    kvfree((*dev).zones);
    (*dev).zones = core::ptr::null_mut();
}

pub unsafe fn null_report_zones(
    disk: *mut gendisk, sector: sector_t, mut nr_zones: c_uint,
    args: *mut blk_report_zones_args,
) -> c_int {
    let nullb = (*disk).private_data as *mut nullb;
    let dev = (*nullb).dev;
    let first_zone = null_zone_no(dev, sector);
    if first_zone >= (*dev).nr_zones { return 0; }
    nr_zones = min(nr_zones, (*dev).nr_zones - first_zone);
    trace_nullb_report_zones!(nullb, nr_zones);
    let mut blkz: blk_zone = core::mem::zeroed();
    let mut zone = (*dev).zones.add(first_zone as usize);
    for i in 0..nr_zones {
        null_lock_zone(dev, zone);
        blkz.start = (*zone).start;
        blkz.len = (*zone).len;
        blkz.wp = (*zone).wp;
        blkz.type_ = (*zone).type_;
        blkz.cond = (*zone).cond;
        blkz.capacity = (*zone).capacity;
        null_unlock_zone(dev, zone);
        let error = disk_report_zone(disk, &mut blkz, i, args);
        if error != 0 { return error; }
        zone = zone.add(1);
    }
    nr_zones as c_int
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
