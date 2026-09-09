// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/drivers/base/map.c
 *
 * (C) Copyright Al Viro 2002,2003
 *
 * NOTE: data structure needs to be changed.  It works, but for large dev_t
 * it will be too slow.  It is isolated, though, so these changes will be
 * local to that file.
 */

use core::ffi::c_void;

pub type DevT = u64;
pub type KobjProbe = unsafe extern "C" fn(DevT, *mut i32, *mut c_void) -> *mut Kobject;

#[repr(C)]
pub struct Module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Kobject {
    _private: [u8; 0],
}

#[repr(C)]
struct Probe {
    next: *mut Probe,
    dev: DevT,
    range: usize,
    owner: *mut Module,
    get: Option<KobjProbe>,
    lock: Option<unsafe extern "C" fn(DevT, *mut c_void) -> i32>,
    data: *mut c_void,
}

#[repr(C)]
pub struct KobjMap {
    probes: [*mut Probe; 255],
    lock: *mut Mutex,
}

extern "C" {
    fn major(dev: DevT) -> u32;
    fn kmalloc_probes(n: usize) -> *mut Probe;
    fn kmalloc_map() -> *mut KobjMap;
    fn kmalloc_probe() -> *mut Probe;
    fn kzalloc_probe() -> *mut Probe;
    fn kfree(ptr: *mut c_void);
    fn mutex_lock(lock: *mut Mutex);
    fn mutex_unlock(lock: *mut Mutex);
    fn try_module_get(module: *mut Module) -> bool;
    fn module_put(module: *mut Module);
}

pub unsafe fn kobj_map(
    domain: *mut KobjMap,
    dev: DevT,
    range: usize,
    module: *mut Module,
    probe: Option<KobjProbe>,
    lock: Option<unsafe extern "C" fn(DevT, *mut c_void) -> i32>,
    data: *mut c_void,
) -> i32 {
    let mut n = (major(dev.wrapping_add(range as u64).wrapping_sub(1))
        - major(dev)
        + 1) as usize;
    let mut index = major(dev);
    let mut p: *mut Probe;

    if n > 255 {
        n = 255;
    }

    p = kmalloc_probes(n);
    if p.is_null() {
        return -12;
    }

    for i in 0..n {
        let current = p.add(i);
        (*current).owner = module;
        (*current).get = probe;
        (*current).lock = lock;
        (*current).dev = dev;
        (*current).range = range;
        (*current).data = data;
    }
    mutex_lock((*domain).lock);
    p = p.sub(n);
    for _ in 0..n {
        let bucket = &mut (*domain).probes[(index as usize) % 255];
        let mut s: *mut *mut Probe = bucket;
        while !(*s).is_null() && (**s).range < range {
            s = &mut (**s).next;
        }
        (*p).next = *s;
        *s = p;
        p = p.add(1);
        index += 1;
    }
    mutex_unlock((*domain).lock);
    0
}

pub unsafe fn kobj_unmap(domain: *mut KobjMap, dev: DevT, range: usize) {
    let mut n = (major(dev.wrapping_add(range as u64).wrapping_sub(1))
        - major(dev)
        + 1) as usize;
    let mut index = major(dev);
    let mut found: *mut Probe = core::ptr::null_mut();

    if n > 255 {
        n = 255;
    }

    mutex_lock((*domain).lock);
    for _ in 0..n {
        let mut s: *mut *mut Probe = &mut (*domain).probes[(index as usize) % 255];
        while !(*s).is_null() {
            let p = *s;
            if (*p).dev == dev && (*p).range == range {
                *s = (*p).next;
                if found.is_null() {
                    found = p;
                }
                break;
            }
            s = &mut (*(*s)).next;
        }
        index += 1;
    }
    mutex_unlock((*domain).lock);
    kfree(found.cast());
}

pub unsafe fn kobj_lookup(
    domain: *mut KobjMap,
    dev: DevT,
    index: *mut i32,
) -> *mut Kobject {
    let mut best = usize::MAX;

    'retry: loop {
        mutex_lock((*domain).lock);
        let mut p = (*domain).probes[(major(dev) as usize) % 255];
        while !p.is_null() {
            if (*p).dev > dev || (*p).dev.wrapping_add((*p).range as u64).wrapping_sub(1) < dev {
                p = (*p).next;
                continue;
            }
            if (*p).range.wrapping_sub(1) >= best {
                break;
            }
            if !try_module_get((*p).owner) {
                p = (*p).next;
                continue;
            }
            let owner = (*p).owner;
            let data = (*p).data;
            let probe = (*p).get.unwrap();
            best = (*p).range - 1;
            *index = dev.wrapping_sub((*p).dev) as i32;
            if let Some(lock) = (*p).lock {
                if lock(dev, data) < 0 {
                    module_put(owner);
                    p = (*p).next;
                    continue;
                }
            }
            mutex_unlock((*domain).lock);
            let kobj = probe(dev, index, data);
            // Currently ->owner protects _only_ ->probe() itself.
            module_put(owner);
            if !kobj.is_null() {
                return kobj;
            }
            continue 'retry;
        }
        mutex_unlock((*domain).lock);
        return core::ptr::null_mut();
    }
}

pub unsafe fn kobj_map_init(base_probe: Option<KobjProbe>, lock: *mut Mutex) -> *mut KobjMap {
    let p = kmalloc_map();
    let base = kzalloc_probe();

    if p.is_null() || base.is_null() {
        kfree(p.cast());
        kfree(base.cast());
        return core::ptr::null_mut();
    }

    (*base).dev = 1;
    (*base).range = usize::MAX;
    (*base).get = base_probe;
    for i in 0..255 {
        (*p).probes[i] = base;
    }
    (*p).lock = lock;
    p
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
