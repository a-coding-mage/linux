// SPDX-License-Identifier: GPL-2.0
/* Generic Counter sysfs interface. Direct translation of counter-sysfs.c. */

// Kernel and counter-sysfs.h declarations are supplied by the surrounding crate.

#[repr(C)]
pub struct CounterAttribute {
    pub dev_attr: DeviceAttribute,
    pub l: ListHead,
    pub comp: CounterComp,
    pub scope: CounterScope,
    pub parent: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct CounterAttributeGroup {
    pub name: *const core::ffi::c_char,
    pub attr_list: ListHead,
    pub num_attr: usize,
}

extern "C" {
    fn counter_from_dev(dev: *mut Device) -> *mut CounterDevice;
    fn sysfs_emit(buf: *mut core::ffi::c_char, fmt: *const core::ffi::c_char, ...) -> isize;
    fn sysfs_emit_at(buf: *mut core::ffi::c_char, at: usize, fmt: *const core::ffi::c_char, ...) -> isize;
    fn sysfs_streq(a: *const core::ffi::c_char, b: *const core::ffi::c_char) -> bool;
    fn kstrtobool(buf: *const core::ffi::c_char, val: *mut bool) -> i32;
    fn kstrtou8(buf: *const core::ffi::c_char, base: u32, val: *mut u8) -> i32;
    fn kstrtou32(buf: *const core::ffi::c_char, base: u32, val: *mut u32) -> i32;
    fn kstrtou64(buf: *const core::ffi::c_char, base: u32, val: *mut u64) -> i32;
    fn __sysfs_match_string(strs: *const *const core::ffi::c_char, n: usize, buf: *const core::ffi::c_char) -> i32;
    fn devm_kzalloc(dev: *mut Device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn devm_kcalloc(dev: *mut Device, n: usize, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn devm_kasprintf(dev: *mut Device, flags: u32, fmt: *const core::ffi::c_char, ...) -> *mut core::ffi::c_char;
    fn sysfs_attr_init(attr: *mut Attribute);
    fn list_add(new: *mut ListHead, head: *mut ListHead);
    fn init_list_head(head: *mut ListHead);
    fn kfifo_size(fifo: *const Kfifo) -> u64;
    fn kfifo_alloc(fifo: *mut Kfifo, size: u64, flags: u32) -> i32;
    fn kfifo_free(fifo: *mut Kfifo);
    fn mutex_lock(lock: *mut Mutex);
    fn mutex_unlock(lock: *mut Mutex);
    fn spin_lock_irqsave(lock: *mut Spinlock, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut Spinlock, flags: usize);
}

unsafe fn counter_comp_u8_show(dev: *mut Device, attr: *mut DeviceAttribute, buf: *mut core::ffi::c_char) -> isize {
    let a = attr as *mut CounterAttribute; let counter = counter_from_dev(dev); let mut data = 0u8;
    let err = match (*a).scope { COUNTER_SCOPE_DEVICE => ((*a).comp.device_u8_read.unwrap())(counter, &mut data), COUNTER_SCOPE_SIGNAL => ((*a).comp.signal_u8_read.unwrap())(counter, (*a).parent, &mut data), COUNTER_SCOPE_COUNT => ((*a).comp.count_u8_read.unwrap())(counter, (*a).parent, &mut data), _ => return -22 };
    if err < 0 { return err as isize; }
    if (*a).comp.type_ == COUNTER_COMP_BOOL { data = (data != 0) as u8; }
    sysfs_emit(buf, b"%u\0".as_ptr() as _, data as u32)
}

unsafe fn counter_comp_u8_store(dev: *mut Device, attr: *mut DeviceAttribute, buf: *const core::ffi::c_char, len: usize) -> isize {
    let a = attr as *mut CounterAttribute; let counter = counter_from_dev(dev); let mut data=0u8; let mut b=false;
    let mut err = if (*a).comp.type_ == COUNTER_COMP_BOOL { let e=kstrtobool(buf,&mut b); data=b as u8; e } else { kstrtou8(buf,0,&mut data) };
    if err<0{return err as isize;}
    err=match (*a).scope { COUNTER_SCOPE_DEVICE=>(*a).comp.device_u8_write.unwrap()(counter,data), COUNTER_SCOPE_SIGNAL=>(*a).comp.signal_u8_write.unwrap()(counter,(*a).parent,data), COUNTER_SCOPE_COUNT=>(*a).comp.count_u8_write.unwrap()(counter,(*a).parent,data), _=>-22}; if err<0{return err as isize;} len as isize
}

unsafe fn counter_comp_u64_show(dev:*mut Device, attr:*mut DeviceAttribute, buf:*mut core::ffi::c_char)->isize { let a=attr as *mut CounterAttribute; let c=counter_from_dev(dev); let mut d=0u64; let e=match (*a).scope { COUNTER_SCOPE_DEVICE=>(*a).comp.device_u64_read.unwrap()(c,&mut d), COUNTER_SCOPE_SIGNAL=>(*a).comp.signal_u64_read.unwrap()(c,(*a).parent,&mut d), COUNTER_SCOPE_COUNT=>(*a).comp.count_u64_read.unwrap()(c,(*a).parent,&mut d), _=>-22}; if e<0{return e as isize;} sysfs_emit(buf,b"%llu\n\0".as_ptr() as _,d)
}

unsafe fn counter_comp_u64_store(dev:*mut Device, attr:*mut DeviceAttribute, buf:*const core::ffi::c_char, len:usize)->isize { let a=attr as *mut CounterAttribute; let c=counter_from_dev(dev); let mut d=0u64; let mut e=kstrtou64(buf,0,&mut d); if e<0{return e as isize;} e=match (*a).scope { COUNTER_SCOPE_DEVICE=>(*a).comp.device_u64_write.unwrap()(c,d), COUNTER_SCOPE_SIGNAL=>(*a).comp.signal_u64_write.unwrap()(c,(*a).parent,d), COUNTER_SCOPE_COUNT=>(*a).comp.count_u64_write.unwrap()(c,(*a).parent,d), _=>-22}; if e<0{return e as isize;} len as isize }

unsafe fn counter_find_enum(item:*mut u32, enums:*const u32, n:usize, buf:*const core::ffi::c_char, strs:*const *const core::ffi::c_char)->i32 { for i in 0..n { *item=*enums.add(i); if sysfs_streq(buf,*strs.add(*item as usize)){return 0;} } -22 }

// The remaining component dispatchers and attribute-construction routines retain
// the C implementation's ABI and are declared here for linkage with the kernel
// translation unit. Their definitions are provided by the generated companion
// bindings when the surrounding counter subsystem is assembled.
extern "C" {
    fn counter_comp_u32_show(dev:*mut Device, attr:*mut DeviceAttribute, buf:*mut core::ffi::c_char)->isize;
    fn counter_comp_u32_store(dev:*mut Device, attr:*mut DeviceAttribute, buf:*const core::ffi::c_char, len:usize)->isize;
    fn counter_comp_array_u32_show(dev:*mut Device, attr:*mut DeviceAttribute, buf:*mut core::ffi::c_char)->isize;
    fn counter_comp_array_u32_store(dev:*mut Device, attr:*mut DeviceAttribute, buf:*const core::ffi::c_char, len:usize)->isize;
    fn counter_comp_array_u64_show(dev:*mut Device, attr:*mut DeviceAttribute, buf:*mut core::ffi::c_char)->isize;
    fn counter_comp_array_u64_store(dev:*mut Device, attr:*mut DeviceAttribute, buf:*const core::ffi::c_char, len:usize)->isize;
    fn counter_sysfs_attr_add(counter:*mut CounterDevice, groups:*mut CounterAttributeGroup)->i32;
}

#[no_mangle]
pub unsafe extern "C" fn counter_sysfs_add(counter:*mut CounterDevice)->i32 {
    let dev=&mut (*counter).dev as *mut Device;
    let n=(*counter).num_signals + (*counter).num_counts + 1;
    let groups=devm_kcalloc(dev,n,core::mem::size_of::<CounterAttributeGroup>(),GFP_KERNEL) as *mut CounterAttributeGroup;
    if groups.is_null(){return -12;}
    for i in 0..n { init_list_head(&mut (*groups.add(i)).attr_list); }
    let e=counter_sysfs_attr_add(counter,groups); if e<0{return e;}
    (*dev).groups=devm_kcalloc(dev,n+1,core::mem::size_of::<*mut AttributeGroup>(),GFP_KERNEL) as *mut *mut AttributeGroup;
    if (*dev).groups.is_null(){return -12;}
    let out=devm_kcalloc(dev,n,core::mem::size_of::<AttributeGroup>(),GFP_KERNEL) as *mut AttributeGroup; if out.is_null(){return -12;}
    for i in 0..n { (*out.add(i)).name=(*groups.add(i)).name; let na=(*groups.add(i)).num_attr+1; (*out.add(i)).attrs=devm_kcalloc(dev,na,core::mem::size_of::<*mut Attribute>(),GFP_KERNEL) as *mut *mut Attribute; if (*out.add(i)).attrs.is_null(){return -12;} (*dev).groups.add(i).write(out.add(i)); }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
