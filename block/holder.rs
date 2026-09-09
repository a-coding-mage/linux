// SPDX-License-Identifier: GPL-2.0-only
// Translated from the Linux block-device holder implementation.

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct kobject {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct block_device {
    pub bd_disk: *mut gendisk,
    pub bd_holder_dir: *mut kobject,
    pub bd_holder: *mut c_void,
}

#[repr(C)]
pub struct gendisk {
    pub slave_dir: *mut kobject,
    pub slave_bdevs: list_head,
    pub dev: *mut device,
}

#[repr(C)]
pub struct device {
    pub kobj: kobject,
}

#[repr(C)]
struct bd_holder_disk {
    list: list_head,
    holder_dir: *mut kobject,
    refcnt: c_int,
}

extern "C" {
    static mut blk_holder_mutex: mutex;

    fn sysfs_create_link(from: *mut kobject, to: *mut kobject, name: *const i8) -> c_int;
    fn sysfs_remove_link(from: *mut kobject, name: *const i8);
    fn kobject_name(kobj: *mut kobject) -> *const i8;
    fn kobject_get(kobj: *mut kobject) -> *mut kobject;
    fn kobject_put(kobj: *mut kobject);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn disk_live(disk: *mut gendisk) -> bool;
    fn bdev_kobj(bdev: *mut block_device) -> *mut kobject;
    fn disk_to_dev(disk: *mut gendisk) -> *mut device;
    fn kzalloc(size: usize) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn list_del_init(entry: *mut list_head);
    fn warn_on_once(condition: bool) -> bool;
}

unsafe fn bd_find_holder_disk(
    bdev: *mut block_device,
    disk: *mut gendisk,
) -> *mut bd_holder_disk {
    // C's list_for_each_entry(holder, &disk->slave_bdevs, list).
    let mut pos = (*disk).slave_bdevs.next;
    while pos != &mut (*disk).slave_bdevs as *mut list_head {
        let holder = (pos as *mut u8).sub(core::mem::offset_of!(bd_holder_disk, list))
            as *mut bd_holder_disk;
        if (*holder).holder_dir == (*bdev).bd_holder_dir {
            return holder;
        }
        pos = (*pos).next;
    }
    core::ptr::null_mut()
}

unsafe fn add_symlink(from: *mut kobject, to: *mut kobject) -> c_int {
    sysfs_create_link(from, to, kobject_name(to))
}

unsafe fn del_symlink(from: *mut kobject, to: *mut kobject) {
    sysfs_remove_link(from, kobject_name(to));
}

pub unsafe fn bd_link_disk_holder(bdev: *mut block_device, disk: *mut gendisk) -> c_int {
    let mut holder: *mut bd_holder_disk;
    let mut ret: c_int = 0;

    if warn_on_once((*disk).slave_dir.is_null()) {
        return -22;
    }
    if (*bdev).bd_disk == disk {
        return -22;
    }

    mutex_lock(&mut (*(*bdev).bd_disk).open_mutex as *mut mutex);
    if !disk_live((*bdev).bd_disk) {
        mutex_unlock(&mut (*(*bdev).bd_disk).open_mutex as *mut mutex);
        return -19;
    }
    kobject_get((*bdev).bd_holder_dir);
    mutex_unlock(&mut (*(*bdev).bd_disk).open_mutex as *mut mutex);

    mutex_lock(&mut blk_holder_mutex);
    warn_on_once((*bdev).bd_holder.is_null());

    holder = bd_find_holder_disk(bdev, disk);
    if !holder.is_null() {
        kobject_put((*bdev).bd_holder_dir);
        (*holder).refcnt += 1;
        mutex_unlock(&mut blk_holder_mutex);
        return 0;
    }

    holder = kzalloc(core::mem::size_of::<bd_holder_disk>()) as *mut bd_holder_disk;
    if holder.is_null() {
        ret = -12;
        mutex_unlock(&mut blk_holder_mutex);
        return ret;
    }

    (*holder).list.next = &mut (*holder).list;
    (*holder).list.prev = &mut (*holder).list;
    (*holder).refcnt = 1;
    (*holder).holder_dir = (*bdev).bd_holder_dir;

    ret = add_symlink((*disk).slave_dir, bdev_kobj(bdev));
    if ret != 0 {
        kfree(holder as *mut c_void);
        mutex_unlock(&mut blk_holder_mutex);
        kobject_put((*bdev).bd_holder_dir);
        return ret;
    }
    ret = add_symlink((*bdev).bd_holder_dir, &mut (*disk_to_dev(disk)).kobj);
    if ret != 0 {
        del_symlink((*disk).slave_dir, bdev_kobj(bdev));
        kfree(holder as *mut c_void);
        mutex_unlock(&mut blk_holder_mutex);
        kobject_put((*bdev).bd_holder_dir);
        return ret;
    }
    list_add(&mut (*holder).list, &mut (*disk).slave_bdevs);

    mutex_unlock(&mut blk_holder_mutex);
    0
}

pub unsafe fn bd_unlink_disk_holder(bdev: *mut block_device, disk: *mut gendisk) {
    if warn_on_once((*disk).slave_dir.is_null()) {
        return;
    }

    mutex_lock(&mut blk_holder_mutex);
    let holder = bd_find_holder_disk(bdev, disk);
    if !warn_on_once(holder.is_null()) {
        (*holder).refcnt -= 1;
        if (*holder).refcnt == 0 {
            del_symlink((*disk).slave_dir, bdev_kobj(bdev));
            del_symlink((*holder).holder_dir, &mut (*disk_to_dev(disk)).kobj);
            kobject_put((*holder).holder_dir);
            list_del_init(&mut (*holder).list);
            kfree(holder as *mut c_void);
        }
    }
    mutex_unlock(&mut blk_holder_mutex);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
