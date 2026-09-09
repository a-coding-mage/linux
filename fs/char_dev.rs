// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/char_dev.c
 *
 *  Copyright (C) 1991, 1992  Linus Torvalds
 */

// Linux kernel dependencies supplied by other translation units.

static mut CDEV_MAP: *mut kobj_map = core::ptr::null_mut();

static mut CHRDEVS_LOCK: mutex = mutex {};

const CHRDEV_MAJOR_HASH_SIZE: usize = 255;

#[repr(C)]
struct char_device_struct {
    next: *mut char_device_struct,
    major: c_uint,
    baseminor: c_uint,
    minorct: c_int,
    name: [c_char; 64],
    cdev: *mut cdev, /* will die */
}

static mut CHRDEVS: [*mut char_device_struct; CHRDEV_MAJOR_HASH_SIZE] =
    [core::ptr::null_mut(); CHRDEV_MAJOR_HASH_SIZE];

/* index in the above */
#[inline]
unsafe fn major_to_index(major: c_uint) -> c_int {
    (major % CHRDEV_MAJOR_HASH_SIZE as c_uint) as c_int
}

#[cfg(CONFIG_PROC_FS)]
unsafe fn chrdev_show(f: *mut seq_file, offset: off_t) {
    let mut cd: *mut char_device_struct;
    mutex_lock(&raw mut CHRDEVS_LOCK);
    cd = CHRDEVS[major_to_index(offset as c_uint) as usize];
    while !cd.is_null() {
        if (*cd).major == offset as c_uint {
            seq_printf(f, "%3d %s\n", (*cd).major, (*cd).name.as_ptr());
        }
        cd = (*cd).next;
    }
    mutex_unlock(&raw mut CHRDEVS_LOCK);
}

unsafe fn find_dynamic_major() -> c_int {
    let mut i: c_int;
    let mut cd: *mut char_device_struct;
    i = CHRDEVS.len() as c_int - 1;
    while i >= CHRDEV_MAJOR_DYN_END {
        if CHRDEVS[i as usize].is_null() { return i; }
        i -= 1;
    }
    i = CHRDEV_MAJOR_DYN_EXT_START;
    while i >= CHRDEV_MAJOR_DYN_EXT_END {
        cd = CHRDEVS[major_to_index(i as c_uint) as usize];
        while !cd.is_null() {
            if (*cd).major == i as c_uint { break; }
            cd = (*cd).next;
        }
        if cd.is_null() { return i; }
        i -= 1;
    }
    -EBUSY
}

/* Register a single major with a specified minor range. */
unsafe fn __register_chrdev_region(mut major: c_uint, baseminor: c_uint,
                                   minorct: c_int, name: *const c_char)
                                   -> *mut char_device_struct {
    let cd = kzalloc_obj::<char_device_struct>();
    if cd.is_null() { return ERR_PTR(-ENOMEM); }
    if major >= CHRDEV_MAJOR_MAX {
        pr_err("CHRDEV \"%s\" major requested (%u) is greater than the maximum (%u)\n", name, major, CHRDEV_MAJOR_MAX - 1);
        kfree(cd as *mut c_void);
        return ERR_PTR(-EINVAL);
    }
    if minorct > MINORMASK + 1 - baseminor as c_int {
        pr_err("CHRDEV \"%s\" minor range requested (%u-%u) is out of range of maximum range (%u-%u) for a single major\n", name, baseminor, baseminor + minorct as c_uint - 1, 0, MINORMASK);
        kfree(cd as *mut c_void);
        return ERR_PTR(-EINVAL);
    }
    mutex_lock(&raw mut CHRDEVS_LOCK);
    if major == 0 {
        let ret = find_dynamic_major();
        if ret < 0 { mutex_unlock(&raw mut CHRDEVS_LOCK); kfree(cd as *mut c_void); return ERR_PTR(ret); }
        major = ret as c_uint;
    }
    let i = major_to_index(major) as usize;
    let mut prev: *mut char_device_struct = core::ptr::null_mut();
    let mut curr = CHRDEVS[i];
    while !curr.is_null() {
        if (*curr).major < major { prev = curr; curr = (*curr).next; continue; }
        if (*curr).major > major { break; }
        if (*curr).baseminor + (*curr).minorct as c_uint <= baseminor { prev = curr; curr = (*curr).next; continue; }
        if (*curr).baseminor >= baseminor + minorct as c_uint { break; }
        mutex_unlock(&raw mut CHRDEVS_LOCK); kfree(cd as *mut c_void); return ERR_PTR(-EBUSY);
    }
    (*cd).major = major; (*cd).baseminor = baseminor; (*cd).minorct = minorct;
    strscpy((*cd).name.as_mut_ptr(), name, (*cd).name.len());
    (*cd).next = curr;
    if prev.is_null() { CHRDEVS[i] = cd; } else { (*prev).next = cd; }
    mutex_unlock(&raw mut CHRDEVS_LOCK);
    cd
}

unsafe fn __unregister_chrdev_region(major: c_uint, baseminor: c_uint, minorct: c_int) -> *mut char_device_struct {
    let i = major_to_index(major) as usize;
    mutex_lock(&raw mut CHRDEVS_LOCK);
    let mut pp: *mut *mut char_device_struct = &raw mut CHRDEVS[i];
    while !(*pp).is_null() {
        let cd = *pp;
        if (*cd).major == major && (*cd).baseminor == baseminor && (*cd).minorct == minorct {
            *pp = (*cd).next; mutex_unlock(&raw mut CHRDEVS_LOCK); return cd;
        }
        pp = &mut (*(*pp)).next;
    }
    mutex_unlock(&raw mut CHRDEVS_LOCK); core::ptr::null_mut()
}

unsafe fn register_chrdev_region(from: dev_t, count: c_uint, name: *const c_char) -> c_int {
    let to = from + count as dev_t; let mut n = from; let mut next;
    while n < to { next = MKDEV(MAJOR(n) + 1, 0); if next > to { next = to; }
        let cd = __register_chrdev_region(MAJOR(n), MINOR(n), (next - n) as c_int, name);
        if IS_ERR(cd) { let mut m = from; while m < n { let x = MKDEV(MAJOR(m)+1,0); kfree(__unregister_chrdev_region(MAJOR(m),MINOR(m),(x-m) as c_int) as *mut c_void); m=x; } return PTR_ERR(cd); } n = next;
    } 0
}

unsafe fn alloc_chrdev_region(dev: *mut dev_t, baseminor: c_uint, count: c_uint, name: *const c_char) -> c_int {
    let cd = __register_chrdev_region(0, baseminor, count as c_int, name); if IS_ERR(cd) { return PTR_ERR(cd); }
    *dev = MKDEV((*cd).major, (*cd).baseminor); 0
}

unsafe fn __register_chrdev(major: c_uint, baseminor: c_uint, count: c_uint, name: *const c_char, fops: *const file_operations) -> c_int {
    let cd = __register_chrdev_region(major, baseminor, count as c_int, name); if IS_ERR(cd) { return PTR_ERR(cd); }
    let cdev = cdev_alloc(); if cdev.is_null() { kfree(__unregister_chrdev_region((*cd).major,baseminor,count as c_int) as *mut c_void); return -ENOMEM; }
    (*cdev).owner = (*fops).owner; (*cdev).ops = fops; kobject_set_name(&mut (*cdev).kobj, "%s", name);
    let err = cdev_add(cdev, MKDEV((*cd).major, baseminor), count); if err != 0 { kobject_put(&mut (*cdev).kobj); kfree(__unregister_chrdev_region((*cd).major,baseminor,count as c_int) as *mut c_void); return err; }
    (*cd).cdev = cdev; if major != 0 { 0 } else { (*cd).major as c_int }
}

unsafe fn unregister_chrdev_region(from: dev_t, count: c_uint) { let to=from+count as dev_t; let mut n=from; while n<to { let next0=MKDEV(MAJOR(n)+1,0); let next=if next0>to{to}else{next0}; kfree(__unregister_chrdev_region(MAJOR(n),MINOR(n),(next-n) as c_int) as *mut c_void); n=next; } }

unsafe fn __unregister_chrdev(major:c_uint, baseminor:c_uint, count:c_uint, _name:*const c_char) { let cd=__unregister_chrdev_region(major,baseminor,count as c_int); if !cd.is_null() && !(*cd).cdev.is_null(){cdev_del((*cd).cdev);} kfree(cd as *mut c_void); }

static mut CDEV_LOCK: spinlock = spinlock {};

unsafe fn cdev_get(p:*mut cdev)->*mut kobject { let owner=(*p).owner; if try_module_get(owner)==0{return core::ptr::null_mut();} let k=kobject_get_unless_zero(&mut (*p).kobj); if k.is_null(){module_put(owner);} k }
unsafe fn cdev_put(p:*mut cdev){if !p.is_null(){let owner=(*p).owner;kobject_put(&mut (*p).kobj);module_put(owner);}}

unsafe fn chrdev_open(inode:*mut inode, filp:*mut file)->c_int {
    spin_lock(&raw mut CDEV_LOCK); let mut p=(*inode).i_cdev; let mut new=core::ptr::null_mut(); let mut ret=0;
    if p.is_null(){spin_unlock(&raw mut CDEV_LOCK);let mut idx=0;let k=kobj_lookup(CDEV_MAP,(*inode).i_rdev,&mut idx);if k.is_null(){return -ENXIO;}new=container_of_cdev(k);spin_lock(&raw mut CDEV_LOCK);p=(*inode).i_cdev;if p.is_null(){(*inode).i_cdev=new;list_add(&mut (*inode).i_devices,&mut (*new).list);new=core::ptr::null_mut();}else if cdev_get(p).is_null(){ret=-ENXIO;}}else if cdev_get(p).is_null(){ret=-ENXIO;}
    spin_unlock(&raw mut CDEV_LOCK);cdev_put(new);if ret!=0{return ret;}let fops=fops_get((*p).ops);if fops.is_null(){cdev_put(p);return -ENXIO;}replace_fops(filp,fops);if let Some(open)=(*(*filp).f_op).open{ret=open(inode,filp);if ret!=0{cdev_put(p);return ret;}}0
}

unsafe fn cd_forget(inode:*mut inode){spin_lock(&raw mut CDEV_LOCK);list_del_init(&mut (*inode).i_devices);(*inode).i_cdev=core::ptr::null_mut();(*inode).i_mapping=&mut (*inode).i_data;spin_unlock(&raw mut CDEV_LOCK);}
unsafe fn cdev_purge(cdev:*mut cdev){spin_lock(&raw mut CDEV_LOCK);while !list_empty(&(*cdev).list){let inode=container_of_inode((*cdev).list.next);list_del_init(&mut (*inode).i_devices);(*inode).i_cdev=core::ptr::null_mut();}spin_unlock(&raw mut CDEV_LOCK);}

#[repr(C)] pub struct file_operations { pub open: Option<unsafe extern "C" fn(*mut inode,*mut file)->c_int>, pub llseek: Option<unsafe extern "C" fn()> , pub owner:*mut module }
pub static mut def_chr_fops:file_operations=file_operations{open:Some(chrdev_open),llseek:Some(noop_llseek),owner:core::ptr::null_mut()};

unsafe fn exact_match(_dev:dev_t,_part:*mut c_int,data:*mut c_void)->*mut kobject{&mut (*(data as *mut cdev)).kobj}
unsafe fn exact_lock(_dev:dev_t,data:*mut c_void)->c_int{if !cdev_get(data as *mut cdev).is_null(){0}else{-1}}

unsafe fn cdev_add(p:*mut cdev,dev:dev_t,count:c_uint)->c_int{(*p).dev=dev;(*p).count=count;let e=if dev==WHITEOUT_DEV{-EBUSY}else{kobj_map(CDEV_MAP,dev,count,core::ptr::null_mut(),Some(exact_match),Some(exact_lock),p)};if e!=0{ kfree_const((*p).kobj.name);(*p).kobj.name=core::ptr::null_mut();}else{kobject_get((*p).kobj.parent);}e}
unsafe fn cdev_set_parent(p:*mut cdev,kobj:*mut kobject){WARN_ON(!(*kobj).state_initialized);(*p).kobj.parent=kobj;}
unsafe fn cdev_device_add(cdev:*mut cdev,dev:*mut device)->c_int{let mut rc=0;if (*dev).devt!=0{cdev_set_parent(cdev,&mut (*dev).kobj);rc=cdev_add(cdev,(*dev).devt,1);if rc!=0{return rc;}}rc=device_add(dev);if rc!=0&&(*dev).devt!=0{cdev_del(cdev);}rc}
unsafe fn cdev_device_del(cdev:*mut cdev,dev:*mut device){device_del(dev);if (*dev).devt!=0{cdev_del(cdev);}}
unsafe fn cdev_unmap(dev:dev_t,count:c_uint){kobj_unmap(CDEV_MAP,dev,count);}
unsafe fn cdev_del(p:*mut cdev){cdev_unmap((*p).dev,(*p).count);kobject_put(&mut (*p).kobj);}
unsafe fn cdev_default_release(kobj:*mut kobject){let p=container_of_cdev(kobj);let parent=(*kobj).parent;cdev_purge(p);kobject_put(parent);}
unsafe fn cdev_dynamic_release(kobj:*mut kobject){let p=container_of_cdev(kobj);let parent=(*kobj).parent;cdev_purge(p);kfree(p as *mut c_void);kobject_put(parent);}
static mut KTYPE_CDEV_DEFAULT:kobj_type=kobj_type{release:Some(cdev_default_release)};
static mut KTYPE_CDEV_DYNAMIC:kobj_type=kobj_type{release:Some(cdev_dynamic_release)};
unsafe fn cdev_alloc()->*mut cdev{let p=kzalloc_obj::<cdev>();if !p.is_null(){INIT_LIST_HEAD(&mut (*p).list);kobject_init(&mut (*p).kobj,&raw mut KTYPE_CDEV_DYNAMIC);}p}
unsafe fn cdev_init(cdev:*mut cdev,fops:*const file_operations){memset(cdev as *mut c_void,0,core::mem::size_of::<cdev>());INIT_LIST_HEAD(&mut (*cdev).list);kobject_init(&mut (*cdev).kobj,&raw mut KTYPE_CDEV_DEFAULT);(*cdev).ops=fops;}
unsafe fn base_probe(dev:dev_t,_part:*mut c_int,_data:*mut c_void)->*mut kobject{if request_module("char-major-%d-%d",MAJOR(dev),MINOR(dev))>0{request_module("char-major-%d",MAJOR(dev));}core::ptr::null_mut()}
unsafe fn chrdev_init(){CDEV_MAP=kobj_map_init(Some(base_probe),&raw mut CHRDEVS_LOCK);}

// External kernel declarations intentionally remain unresolved here.
extern "C" { fn noop_llseek(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
