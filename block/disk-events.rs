// SPDX-License-Identifier: GPL-2.0
/*
 * Disk events - monitor disk events like media change and eject request.
 */

// Kernel dependencies supplied by other translation units are intentionally
// referenced here rather than reimplemented.

#[repr(C)]
struct disk_events {
    node: list_head,              /* all disk_event's */
    disk: *mut gendisk,           /* the associated disk */
    lock: spinlock_t,

    block_mutex: mutex,           /* protects blocking */
    block: c_int,                 /* event blocking depth */
    pending: c_uint,              /* events already sent out */
    clearing: c_uint,             /* events being cleared */

    poll_msecs: c_long,           /* interval, -1 for default */
    dwork: delayed_work,
}

static disk_events_strs: [*const c_char; 2] = [
    b"media_change\0".as_ptr() as *const c_char,
    b"eject_request\0".as_ptr() as *const c_char,
];

static mut disk_uevents: [*mut c_char; 2] = [
    b"DISK_MEDIA_CHANGE=1\0".as_ptr() as *mut c_char,
    b"DISK_EJECT_REQUEST=1\0".as_ptr() as *mut c_char,
];

/* list of all disk_events */
static mut disk_events_mutex: mutex = DEFINE_MUTEX!();
static mut disk_events: list_head = LIST_HEAD!();

/* disable in-kernel polling by default */
static mut disk_events_dfl_poll_msecs: c_ulong = 0;

unsafe fn disk_events_poll_jiffies(disk: *mut gendisk) -> c_ulong {
    let ev = (*disk).ev;
    let mut intv_msecs: c_long = 0;

    /*
     * If device-specific poll interval is set, always use it.  If
     * the default is being used, poll if the POLL flag is set.
     */
    if (*ev).poll_msecs >= 0 {
        intv_msecs = (*ev).poll_msecs;
    } else if (*disk).event_flags & DISK_EVENT_FLAG_POLL != 0 {
        intv_msecs = disk_events_dfl_poll_msecs as c_long;
    }

    msecs_to_jiffies(intv_msecs as c_uint)
}

/**
 * disk_block_events - block and flush disk event checking
 * @disk: disk to block events for
 */
pub unsafe fn disk_block_events(disk: *mut gendisk) {
    let ev = (*disk).ev;
    let mut flags: c_ulong = 0;
    let cancel: bool;

    if ev.is_null() { return; }

    mutex_lock(&mut (*ev).block_mutex);
    spin_lock_irqsave(&mut (*ev).lock, &mut flags);
    cancel = (*ev).block == 0;
    (*ev).block += 1;
    spin_unlock_irqrestore(&mut (*ev).lock, flags);

    if cancel { cancel_delayed_work_sync(&mut (*ev).dwork); }
    mutex_unlock(&mut (*ev).block_mutex);
}

unsafe fn __disk_unblock_events(disk: *mut gendisk, check_now: bool) {
    let ev = (*disk).ev;
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*ev).lock, &mut flags);

    if WARN_ON_ONCE!((*ev).block <= 0) { spin_unlock_irqrestore(&mut (*ev).lock, flags); return; }
    (*ev).block -= 1;
    if (*ev).block != 0 { spin_unlock_irqrestore(&mut (*ev).lock, flags); return; }

    let intv = disk_events_poll_jiffies(disk);
    if check_now {
        queue_delayed_work(system_freezable_power_efficient_wq, &mut (*ev).dwork, 0);
    } else if intv != 0 {
        queue_delayed_work(system_freezable_power_efficient_wq, &mut (*ev).dwork, intv);
    }
    spin_unlock_irqrestore(&mut (*ev).lock, flags);
}

pub unsafe fn disk_unblock_events(disk: *mut gendisk) {
    if !(*disk).ev.is_null() { __disk_unblock_events(disk, false); }
}

pub unsafe fn disk_flush_events(disk: *mut gendisk, mask: c_uint) {
    let ev = (*disk).ev;
    if ev.is_null() { return; }
    spin_lock_irq(&mut (*ev).lock);
    (*ev).clearing |= mask;
    if (*ev).block == 0 {
        mod_delayed_work(system_freezable_power_efficient_wq, &mut (*ev).dwork, 0);
    }
    spin_unlock_irq(&mut (*ev).lock);
}

unsafe fn disk_event_uevent(disk: *mut gendisk, events: c_uint) {
    let mut envp: [*mut c_char; 3] = [core::ptr::null_mut(); 3];
    let mut nr_events = 0usize;
    for i in 0..disk_uevents.len() {
        if events & (*disk).events & (1u32 << i) != 0 {
            envp[nr_events] = disk_uevents[i]; nr_events += 1;
        }
    }
    if nr_events != 0 { kobject_uevent_env(&mut disk_to_dev(disk).kobj, KOBJ_CHANGE, envp.as_mut_ptr()); }
}

unsafe fn disk_check_events(ev: *mut disk_events, clearing_ptr: *mut c_uint) {
    let disk = (*ev).disk;
    let clearing = *clearing_ptr;
    let mut events = ((*disk).fops).check_events.unwrap()(disk, clearing);
    spin_lock_irq(&mut (*ev).lock);
    events &= !(*ev).pending; (*ev).pending |= events; *clearing_ptr &= !clearing;
    let intv = disk_events_poll_jiffies(disk);
    if (*ev).block == 0 && intv != 0 { queue_delayed_work(system_freezable_power_efficient_wq, &mut (*ev).dwork, intv); }
    spin_unlock_irq(&mut (*ev).lock);
    if events & DISK_EVENT_MEDIA_CHANGE != 0 { inc_diskseq(disk); }
    if (*disk).event_flags & DISK_EVENT_FLAG_UEVENT != 0 { disk_event_uevent(disk, events); }
}

unsafe fn disk_clear_events(disk: *mut gendisk, mask: c_uint) -> c_uint {
    let ev = (*disk).ev;
    if ev.is_null() { return 0; }
    disk_block_events(disk);
    let mut clearing = mask;
    spin_lock_irq(&mut (*ev).lock); clearing |= (*ev).clearing; (*ev).clearing = 0; spin_unlock_irq(&mut (*ev).lock);
    disk_check_events(ev, &mut clearing);
    __disk_unblock_events(disk, (*ev).clearing != 0);
    spin_lock_irq(&mut (*ev).lock); let pending = (*ev).pending & mask; (*ev).pending &= !mask; spin_unlock_irq(&mut (*ev).lock);
    WARN_ON_ONCE!(clearing & mask != 0); pending
}

pub unsafe fn disk_check_media_change(disk: *mut gendisk) -> bool {
    let events = disk_clear_events(disk, DISK_EVENT_MEDIA_CHANGE | DISK_EVENT_EJECT_REQUEST);
    if events & DISK_EVENT_MEDIA_CHANGE != 0 { set_bit(GD_NEED_PART_SCAN, &mut (*disk).state); return true; }
    false
}

pub unsafe fn disk_force_media_change(disk: *mut gendisk) {
    disk_event_uevent(disk, DISK_EVENT_MEDIA_CHANGE); inc_diskseq(disk); bdev_mark_dead((*disk).part0, true);
}

unsafe fn disk_events_workfn(work: *mut work_struct) {
    let dwork = to_delayed_work(work);
    let ev = container_of!(dwork, disk_events, dwork);
    disk_check_events(ev, &mut (*ev).clearing);
}

unsafe fn __disk_events_show(events: c_uint, buf: *mut c_char) -> ssize_t {
    let mut pos: ssize_t = 0; let mut first = true;
    for i in 0..disk_events_strs.len() { if events & (1u32 << i) != 0 { pos += sprintf(buf.add(pos as usize), if first { b"%s\0".as_ptr() } else { b" %s\0".as_ptr() }, disk_events_strs[i]); first = false; } }
    if pos != 0 { pos += sprintf(buf.add(pos as usize), b"\n\0".as_ptr()); } pos
}

unsafe fn disk_events_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let disk = dev_to_disk(dev); if (*disk).event_flags & DISK_EVENT_FLAG_UEVENT == 0 { return 0; } __disk_events_show((*disk).events, buf)
}
unsafe fn disk_events_async_show(_dev: *mut device, _attr: *mut device_attribute, _buf: *mut c_char) -> ssize_t { 0 }
unsafe fn disk_events_poll_msecs_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t { let disk = dev_to_disk(dev); if (*disk).ev.is_null() { sprintf(buf, b"-1\n\0".as_ptr()) } else { sprintf(buf, b"%ld\n\0".as_ptr(), (*(*disk).ev).poll_msecs) } }
unsafe fn disk_events_poll_msecs_store(dev: *mut device, _attr: *mut device_attribute, buf: *const c_char, count: usize) -> ssize_t {
    let disk = dev_to_disk(dev); let mut intv: c_long = 0;
    if count == 0 || sscanf(buf, b"%ld\0".as_ptr(), &mut intv) == 0 { return -EINVAL as ssize_t; }
    if intv < 0 && intv != -1 { return -EINVAL as ssize_t; } if (*disk).ev.is_null() { return -ENODEV as ssize_t; }
    disk_block_events(disk); (*(*disk).ev).poll_msecs = intv; __disk_unblock_events(disk, true); count as ssize_t
}

unsafe fn disk_events_set_dfl_poll_msecs(val: *const c_char, kp: *const kernel_param) -> c_int {
    let ret = param_set_ulong(val, kp); if ret < 0 { return ret; }
    mutex_lock(&mut disk_events_mutex); list_for_each_entry!(ev, &mut disk_events, node, { disk_flush_events((*ev).disk, 0); }); mutex_unlock(&mut disk_events_mutex); 0
}

pub unsafe fn disk_alloc_events(disk: *mut gendisk) -> c_int {
    if (*disk).fops.check_events.is_none() || (*disk).events == 0 { return 0; }
    let ev = kzalloc_obj::<disk_events>(); if ev.is_null() { pr_warn!("%s: failed to initialize events\n", (*disk).disk_name); return -ENOMEM; }
    INIT_LIST_HEAD!(&mut (*ev).node); (*ev).disk = disk; spin_lock_init(&mut (*ev).lock); mutex_init(&mut (*ev).block_mutex); (*ev).block = 1; (*ev).poll_msecs = -1; INIT_DELAYED_WORK!(&mut (*ev).dwork, disk_events_workfn); (*disk).ev = ev; 0
}

pub unsafe fn disk_add_events(disk: *mut gendisk) { if (*disk).ev.is_null() { return; } mutex_lock(&mut disk_events_mutex); list_add_tail!(&mut (*(*disk).ev).node, &mut disk_events); mutex_unlock(&mut disk_events_mutex); __disk_unblock_events(disk, true); }
pub unsafe fn disk_del_events(disk: *mut gendisk) { if !(*disk).ev.is_null() { disk_block_events(disk); mutex_lock(&mut disk_events_mutex); list_del_init!(&mut (*(*disk).ev).node); mutex_unlock(&mut disk_events_mutex); } }
pub unsafe fn disk_release_events(disk: *mut gendisk) { WARN_ON_ONCE!(!(*disk).ev.is_null() && (*(*disk).ev).block != 1); kfree((*disk).ev); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
