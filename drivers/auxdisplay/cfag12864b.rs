// SPDX-License-Identifier: GPL-2.0
/*
 *    Filename: cfag12864b.c
 *     Version: 0.1.0
 * Description: cfag12864b LCD driver
 *     Depends: ks0108
 *
 *      Author: Copyright (C) Miguel Ojeda <ojeda@kernel.org>
 *        Date: 2006-10-31
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

pub const CFAG12864B_NAME: &[u8] = b"cfag12864b\0";

static mut CFAG12864B_RATE: u32 = CONFIG_CFAG12864B_RATE;

/*
 * cfag12864b Commands
 *
 * E = Enable signal
 * CS1 = First ks0108 controller.
 * CS2 = Second ks0108 controller
 * DI = Data/Instruction
 */

#[inline]
const fn bit(n: u8) -> u8 {
    1u8.wrapping_shl(n as u32)
}

const CFAG12864B_BIT_E: u8 = 0;
const CFAG12864B_BIT_CS1: u8 = 2;
const CFAG12864B_BIT_CS2: u8 = 1;
const CFAG12864B_BIT_DI: u8 = 3;

static mut cfag12864b_state: u8 = 0;

extern "C" {
    static CONFIG_CFAG12864B_RATE: u32;
    static mut cfag12864b_workqueue: *mut workqueue_struct;
    fn ks0108_writecontrol(value: u8);
    fn ks0108_displaystate(state: u8);
    fn ks0108_address(address: u8);
    fn ks0108_page(page: u8);
    fn ks0108_startline(startline: u8);
    fn ks0108_writedata(byte: u8);
    fn ks0108_isinited() -> u8;
    fn mutex_lock(mutex: *mut mutex);
    fn mutex_unlock(mutex: *mut mutex);
    fn queue_delayed_work(queue: *mut workqueue_struct, work: *mut delayed_work, delay: u64);
    fn cancel_delayed_work(work: *mut delayed_work);
    fn flush_workqueue(queue: *mut workqueue_struct);
    fn memcmp(a: *const u8, b: *const u8, size: usize) -> i32;
    fn memcpy(dst: *mut u8, src: *const u8, size: usize) -> *mut u8;
    fn get_zeroed_page(flags: u32) -> *mut u8;
    fn kmalloc(size: usize, flags: u32) -> *mut u8;
    fn kfree(ptr: *mut u8);
    fn free_page(address: usize);
    fn create_singlethread_workqueue(name: *const u8) -> *mut workqueue_struct;
    fn destroy_workqueue(queue: *mut workqueue_struct);
    fn printk(format: *const u8, ...);
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}
#[repr(C)]
pub struct delayed_work {
    _private: [u8; 0],
}
#[repr(C)]
pub struct workqueue_struct {
    _private: [u8; 0],
}
#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

static mut cfag12864b_mutex: mutex = mutex { _private: [] };
static mut cfag12864b_updating: u8 = 0;
static mut cfag12864b_cache: *mut u8 = core::ptr::null_mut();
pub static mut cfag12864b_buffer: *mut u8 = core::ptr::null_mut();
static mut cfag12864b_work: delayed_work = delayed_work { _private: [] };
static mut cfag12864b_inited: u8 = 0;

unsafe fn cfag12864b_set() { ks0108_writecontrol(cfag12864b_state); }
unsafe fn cfag12864b_setbit(state: u8, n: u8) {
    if state != 0 { cfag12864b_state |= bit(n); } else { cfag12864b_state &= !bit(n); }
}
unsafe fn cfag12864b_e(state: u8) { cfag12864b_setbit(state, CFAG12864B_BIT_E); cfag12864b_set(); }
unsafe fn cfag12864b_cs1(state: u8) { cfag12864b_setbit(state, CFAG12864B_BIT_CS1); }
unsafe fn cfag12864b_cs2(state: u8) { cfag12864b_setbit(state, CFAG12864B_BIT_CS2); }
unsafe fn cfag12864b_di(state: u8) { cfag12864b_setbit(state, CFAG12864B_BIT_DI); }

unsafe fn cfag12864b_setcontrollers(first: u8, second: u8) {
    cfag12864b_cs1(if first != 0 { 0 } else { 1 });
    cfag12864b_cs2(if second != 0 { 0 } else { 1 });
}
unsafe fn cfag12864b_controller(which: u8) {
    if which == 0 { cfag12864b_setcontrollers(1, 0); }
    else if which == 1 { cfag12864b_setcontrollers(0, 1); }
}
unsafe fn cfag12864b_displaystate(state: u8) { cfag12864b_di(0); cfag12864b_e(1); ks0108_displaystate(state); cfag12864b_e(0); }
unsafe fn cfag12864b_address(address: u8) { cfag12864b_di(0); cfag12864b_e(1); ks0108_address(address); cfag12864b_e(0); }
unsafe fn cfag12864b_page(page: u8) { cfag12864b_di(0); cfag12864b_e(1); ks0108_page(page); cfag12864b_e(0); }
unsafe fn cfag12864b_startline(startline: u8) { cfag12864b_di(0); cfag12864b_e(1); ks0108_startline(startline); cfag12864b_e(0); }
unsafe fn cfag12864b_writebyte(byte: u8) { cfag12864b_di(1); cfag12864b_e(1); ks0108_writedata(byte); cfag12864b_e(0); }
unsafe fn cfag12864b_nop() { cfag12864b_startline(0); }

unsafe fn cfag12864b_on() { cfag12864b_setcontrollers(1, 1); cfag12864b_displaystate(1); }
unsafe fn cfag12864b_off() { cfag12864b_setcontrollers(1, 1); cfag12864b_displaystate(0); }
unsafe fn cfag12864b_clear() {
    cfag12864b_setcontrollers(1, 1);
    let mut i: u8 = 0;
    while i < CFAG12864B_PAGES { cfag12864b_page(i); cfag12864b_address(0); let mut j: u8 = 0; while j < CFAG12864B_ADDRESSES { cfag12864b_writebyte(0); j = j.wrapping_add(1); } i = i.wrapping_add(1); }
}

unsafe fn cfag12864b_queue() { queue_delayed_work(cfag12864b_workqueue, &mut cfag12864b_work, HZ / CFAG12864B_RATE as u64); }

#[no_mangle]
pub unsafe extern "C" fn cfag12864b_enable() -> u8 {
    mutex_lock(&mut cfag12864b_mutex); let ret; if cfag12864b_updating == 0 { cfag12864b_updating = 1; cfag12864b_queue(); ret = 0; } else { ret = 1; } mutex_unlock(&mut cfag12864b_mutex); ret
}
#[no_mangle]
pub unsafe extern "C" fn cfag12864b_disable() { mutex_lock(&mut cfag12864b_mutex); if cfag12864b_updating != 0 { cfag12864b_updating = 0; cancel_delayed_work(&mut cfag12864b_work); flush_workqueue(cfag12864b_workqueue); } mutex_unlock(&mut cfag12864b_mutex); }

unsafe fn cfag12864b_update(_work: *mut work_struct) {
    if memcmp(cfag12864b_cache, cfag12864b_buffer, CFAG12864B_SIZE as usize) != 0 {
        let mut i: u16 = 0; while i < CFAG12864B_CONTROLLERS { cfag12864b_controller(i as u8); cfag12864b_nop(); let mut j: u16 = 0; while j < CFAG12864B_PAGES { cfag12864b_page(j as u8); cfag12864b_nop(); cfag12864b_address(0); cfag12864b_nop(); let mut k: u16 = 0; while k < CFAG12864B_ADDRESSES { let mut c: u8 = 0; let mut b: u16 = 0; while b < 8 { let index = i as usize * CFAG12864B_ADDRESSES as usize / 8 + k as usize / 8 + (j as usize * 8 + b as usize) * CFAG12864B_WIDTH as usize / 8; if *cfag12864b_buffer.add(index) & bit((k % 8) as u8) != 0 { c |= bit(b as u8); } b += 1; } cfag12864b_writebyte(c); k += 1; } j += 1; } i += 1; }
        memcpy(cfag12864b_cache, cfag12864b_buffer, CFAG12864B_SIZE as usize);
    }
    if cfag12864b_updating != 0 { cfag12864b_queue(); }
}

#[no_mangle]
pub unsafe extern "C" fn cfag12864b_isinited() -> u8 { cfag12864b_inited }

#[no_mangle]
pub unsafe extern "C" fn cfag12864b_init() -> i32 {
    let mut ret: i32 = -22;
    if ks0108_isinited() == 0 { return ret; }

    cfag12864b_buffer = get_zeroed_page(GFP_KERNEL);
    if cfag12864b_buffer.is_null() { ret = -12; return ret; }

    cfag12864b_cache = kmalloc(CFAG12864B_SIZE as usize, GFP_KERNEL);
    if cfag12864b_cache.is_null() { free_page(cfag12864b_buffer as usize); return -12; }

    cfag12864b_workqueue = create_singlethread_workqueue(CFAG12864B_NAME.as_ptr());
    if cfag12864b_workqueue.is_null() { kfree(cfag12864b_cache); free_page(cfag12864b_buffer as usize); return ret; }

    cfag12864b_clear();
    cfag12864b_on();
    cfag12864b_inited = 1;
    0
}

#[no_mangle]
pub unsafe extern "C" fn cfag12864b_exit() {
    cfag12864b_disable();
    cfag12864b_off();
    destroy_workqueue(cfag12864b_workqueue);
    kfree(cfag12864b_cache);
    free_page(cfag12864b_buffer as usize);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
