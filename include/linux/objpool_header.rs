/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the surrounding kernel translation. */

use core::ffi::c_void;

#[repr(C, packed)]
pub struct objpool_slot {
    pub head: u32,
    pub tail: u32,
    pub last: u32,
    pub mask: u32,
    pub entries: [*mut c_void; 0],
}

pub type objpool_init_obj_cb = Option<unsafe extern "C" fn(obj: *mut c_void, context: *mut c_void) -> i32>;
pub type objpool_fini_cb = Option<unsafe extern "C" fn(head: *mut objpool_head, context: *mut c_void) -> i32>;

#[repr(C)]
pub struct objpool_head {
    pub obj_size: i32,
    pub nr_objs: i32,
    pub nr_possible_cpus: i32,
    pub capacity: i32,
    pub gfp: gfp_t,
    pub ref_: refcount_t,
    pub flags: c_ulong,
    pub cpu_slots: *mut *mut objpool_slot,
    pub release: objpool_fini_cb,
    pub context: *mut c_void,
}

pub const OBJPOOL_NR_OBJECT_MAX: c_ulong = 1 as c_ulong << 24;
pub const OBJPOOL_OBJECT_SIZE_MAX: c_ulong = 1 as c_ulong << 16;

extern "C" {
    pub fn objpool_init(
        pool: *mut objpool_head,
        nr_objs: i32,
        object_size: i32,
        gfp: gfp_t,
        context: *mut c_void,
        objinit: objpool_init_obj_cb,
        release: objpool_fini_cb,
    ) -> i32;

    pub fn objpool_drop(obj: *mut c_void, pool: *mut objpool_head) -> i32;
    pub fn objpool_free(pool: *mut objpool_head);
    pub fn objpool_fini(pool: *mut objpool_head);
}

/* try to retrieve object from slot */
#[inline]
pub unsafe fn __objpool_try_get_slot(pool: *mut objpool_head, cpu: i32) -> *mut c_void {
    let slot = *(*pool).cpu_slots.offset(cpu as isize);
    let mut head = smp_load_acquire(&mut (*slot).head);

    while head != READ_ONCE(&(*slot).last) {
        /*
         * data visibility of 'last' and 'head' could be out of
         * order since memory updating of 'last' and 'head' are
         * performed in push() and pop() independently
         */
        if READ_ONCE(&(*slot).last).wrapping_sub(head).wrapping_sub(1) >= (*pool).nr_objs as u32 {
            head = READ_ONCE(&(*slot).head);
            continue;
        }

        let obj = READ_ONCE(&(*slot).entries[(head & (*slot).mask) as usize]);
        if try_cmpxchg_release(&mut (*slot).head, &mut head, head.wrapping_add(1)) {
            return obj;
        }
    }

    core::ptr::null_mut()
}

#[inline]
pub unsafe fn objpool_pop(pool: *mut objpool_head) -> *mut c_void {
    let mut obj = core::ptr::null_mut();
    let mut flags: c_ulong = 0;
    raw_local_irq_save(&mut flags);

    let start = raw_smp_processor_id();
    for_each_possible_cpu_wrap!(cpu, start, {
        obj = __objpool_try_get_slot(pool, cpu);
        if !obj.is_null() { break; }
    });
    raw_local_irq_restore(flags);
    obj
}

#[inline]
pub unsafe fn __objpool_try_add_slot(obj: *mut c_void, pool: *mut objpool_head, cpu: i32) -> i32 {
    let slot = *(*pool).cpu_slots.offset(cpu as isize);
    let mut tail = READ_ONCE(&(*slot).tail);
    let mut head;
    loop {
        head = READ_ONCE(&(*slot).head);
        WARN_ON_ONCE!(tail.wrapping_sub(head) > (*pool).nr_objs as u32);
        if try_cmpxchg_acquire(&mut (*slot).tail, &mut tail, tail.wrapping_add(1)) { break; }
    }
    WRITE_ONCE(&mut (*slot).entries[(tail & (*slot).mask) as usize], obj);
    smp_store_release(&mut (*slot).last, tail.wrapping_add(1));
    0
}

#[inline]
pub unsafe fn objpool_push(obj: *mut c_void, pool: *mut objpool_head) -> i32 {
    let mut flags: c_ulong = 0;
    raw_local_irq_save(&mut flags);
    let rc = __objpool_try_add_slot(obj, pool, raw_smp_processor_id());
    raw_local_irq_restore(flags);
    rc
}

/* External kernel types and primitives are supplied by other translated files. */
extern "C" {
    pub fn smp_load_acquire<T>(ptr: *mut T) -> T;
    pub fn READ_ONCE<T>(ptr: *const T) -> T;
    pub fn try_cmpxchg_release<T>(ptr: *mut T, old: *mut T, new: T) -> bool;
    pub fn try_cmpxchg_acquire<T>(ptr: *mut T, old: *mut T, new: T) -> bool;
    pub fn WRITE_ONCE<T>(ptr: *mut T, value: T);
    pub fn smp_store_release<T>(ptr: *mut T, value: T);
    pub fn raw_local_irq_save(flags: *mut c_ulong);
    pub fn raw_local_irq_restore(flags: c_ulong);
    pub fn raw_smp_processor_id() -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
