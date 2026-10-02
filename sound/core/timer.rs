// SPDX-License-Identifier: GPL-2.0-or-later
/* Timers abstract layer. Rust implementation of the unchanged timer.c.
 * C helpers expose header-defined synchronization and ABI boundaries only.
 */
#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code,
    missing_docs,
    unsafe_op_in_unsafe_fn,
    clippy::all,
    unreachable_pub
)]
#[allow(improper_ctypes)]
mod bindings {
    use kernel::ffi;
    include!(concat!(
        env!("OBJTREE"),
        "/rust/bindings/snd_timer_generated.rs"
    ));
}
use bindings::*;
use sized_strscpy as strscpy;
#[cfg(CONFIG_SND_PROC_FS)]
macro_rules! snd_iprintf { ($buffer:expr, $fmt:expr $(, $arg:expr)* $(,)?) => { seq_printf((*$buffer).buffer.cast(), $fmt $(, $arg)*) }; }
use core::mem::{offset_of, size_of, zeroed};
use core::ptr::{addr_of_mut, null_mut};
use kernel::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
const SNDRV_TIMER_IFLG_PAUSED: u32 = 0x00010000;
const SNDRV_TIMER_IFLG_DEAD: u32 = 0x00020000;
const MAX_SLAVE_INSTANCES: c_int = 1000;
#[no_mangle]
pub static mut rust_snd_timer_limit: c_int = RUST_DEFAULT_TIMER_LIMIT as _;
#[no_mangle]
pub static mut rust_snd_timer_tstamp_monotonic: c_int = 1;
static mut num_slaves: c_int = 0;
static mut timer_dev: *mut device = null_mut();
macro_rules! err {
    ($name:ident) => {
        -($name as c_int)
    };
}
macro_rules! for_each {
    ($item:ident, $head:expr, $ty:ty, $field:ident, $body:block) => {{
        let head = $head;
        let mut node = (*head).next;
        while node != head {
            let $item = node.cast::<u8>().sub(offset_of!($ty, $field)).cast::<$ty>();
            node = (*node).next;
            $body
        }
    }};
}
#[inline]
unsafe fn list_empty(p: *const list_head) -> bool {
    (*p).next == p.cast_mut()
}
use rust_timer_list_add_tail as list_add_tail;
use rust_timer_list_del_init as list_del_init;
use rust_timer_list_init as list_init;
use rust_timer_list_move_tail as list_move_tail;
struct MutexGuard(*mut mutex);
impl MutexGuard {
    unsafe fn new(p: *mut mutex) -> Self {
        rust_timer_mutex_lock(p);
        Self(p)
    }
}
impl Drop for MutexGuard {
    fn drop(&mut self) {
        unsafe { rust_timer_mutex_unlock(self.0) }
    }
}
struct SpinGuard {
    p: *mut spinlock_t,
    kind: u8,
    flags: c_ulong,
}
impl SpinGuard {
    unsafe fn new(p: *mut spinlock_t) -> Self {
        rust_timer_spin_lock(p);
        Self {
            p,
            kind: 0,
            flags: 0,
        }
    }
    unsafe fn irq(p: *mut spinlock_t) -> Self {
        rust_timer_spin_lock_irq(p);
        Self {
            p,
            kind: 1,
            flags: 0,
        }
    }
    unsafe fn irqsave(p: *mut spinlock_t) -> Self {
        let flags = rust_timer_spin_lock_irqsave(p);
        Self { p, kind: 2, flags }
    }
}
impl Drop for SpinGuard {
    fn drop(&mut self) {
        unsafe {
            match self.kind {
                0 => rust_timer_spin_unlock(self.p),
                1 => rust_timer_spin_unlock_irq(self.p),
                _ => rust_timer_spin_unlock_irqrestore(self.p, self.flags),
            }
        }
    }
}
struct ReadGuard {
    flags: c_ulong,
}
impl ReadGuard {
    unsafe fn new() -> Self {
        Self {
            flags: rust_timer_read_lock_irqsave(rust_timer_instance_lock()),
        }
    }
}
impl Drop for ReadGuard {
    fn drop(&mut self) {
        unsafe { rust_timer_read_unlock_irqrestore(rust_timer_instance_lock(), self.flags) }
    }
}
struct WriteGuard;
impl WriteGuard {
    unsafe fn new() -> Self {
        rust_timer_write_lock_irq(rust_timer_instance_lock());
        Self
    }
}
impl Drop for WriteGuard {
    fn drop(&mut self) {
        unsafe { rust_timer_write_unlock_irq(rust_timer_instance_lock()) }
    }
}
struct TimerRef(*mut snd_timer);
impl Drop for TimerRef {
    fn drop(&mut self) {
        unsafe {
            if !self.0.is_null() {
                snd_timer_ref_put(self.0)
            }
        }
    }
}
struct Allocation<T>(*mut T);
impl<T> Drop for Allocation<T> {
    fn drop(&mut self) {
        unsafe { kfree(self.0.cast()) }
    }
}
#[inline]
fn is_err<T>(p: *const T) -> bool {
    p as usize >= usize::MAX - 4094
}
#[inline]
unsafe fn copy_from<T>(dst: *mut T, src: *const T) -> bool {
    rust_timer_copy_from_user(dst.cast(), src.cast(), size_of::<T>() as _) != 0
}
#[inline]
unsafe fn copy_to<T>(dst: *mut T, src: *const T) -> bool {
    rust_timer_copy_to_user(dst.cast(), src.cast(), size_of::<T>() as _) != 0
}
#[inline]
unsafe fn card_shutdown(t: *mut snd_timer) -> bool {
    !(*t).card.is_null() && rust_timer_card_shutdown((*t).card)
}
#[inline]
unsafe fn card_number(t: *mut snd_timer) -> c_int {
    if (*t).card.is_null() {
        -1
    } else {
        rust_timer_card_number((*t).card)
    }
}
unsafe fn timestamp(t: *mut timespec64) {
    if rust_snd_timer_tstamp_monotonic != 0 {
        ktime_get_ts64(t)
    } else {
        ktime_get_real_ts64(t)
    }
}
#[no_mangle]
pub unsafe extern "C" fn snd_timer_instance_new(owner: *const c_char) -> *mut snd_timer_instance {
    let ti: *mut snd_timer_instance = rust_timer_zalloc(size_of::<snd_timer_instance>()).cast();
    if ti.is_null() {
        return null_mut();
    }
    (*ti).owner = rust_timer_strdup(owner);
    if (*ti).owner.is_null() {
        kfree(ti.cast());
        return null_mut();
    }
    list_init(addr_of_mut!((*ti).open_list));
    list_init(addr_of_mut!((*ti).active_list));
    list_init(addr_of_mut!((*ti).master_list));
    list_init(addr_of_mut!((*ti).ack_list));
    list_init(addr_of_mut!((*ti).slave_list_head));
    list_init(addr_of_mut!((*ti).slave_active_head));
    ti
}
#[no_mangle]
pub unsafe extern "C" fn snd_timer_instance_free(ti: *mut snd_timer_instance) {
    if !ti.is_null() {
        if let Some(f) = (*ti).private_free {
            f(ti);
        }
        kfree((*ti).owner.cast());
        kfree(ti.cast());
    }
}
unsafe fn snd_timer_find(tid: *mut snd_timer_id) -> *mut snd_timer {
    for_each!(t, rust_timer_list(), snd_timer, device_list, {
        if (*t).tmr_class != (*tid).dev_class {
            continue;
        }
        if ((*t).tmr_class == SNDRV_TIMER_CLASS_CARD as c_int
            || (*t).tmr_class == SNDRV_TIMER_CLASS_PCM as c_int)
            && ((*t).card.is_null() || card_number(t) != (*tid).card)
        {
            continue;
        }
        if (*t).tmr_device == (*tid).device && (*t).tmr_subdevice == (*tid).subdevice {
            return t;
        }
    });
    null_mut()
}
#[cfg(CONFIG_MODULES)]
unsafe fn snd_timer_request(tid: *mut snd_timer_id) {
    match (*tid).dev_class {
        x if x == SNDRV_TIMER_CLASS_GLOBAL as c_int => {
            if (*tid).device < rust_snd_timer_limit {
                rust_timer_request_global((*tid).device);
            }
        }
        x if x == SNDRV_TIMER_CLASS_CARD as c_int || x == SNDRV_TIMER_CLASS_PCM as c_int => {
            if (*tid).card < rust_timer_ecards_limit() {
                rust_timer_request_card((*tid).card);
            }
        }
        _ => (),
    }
}
unsafe fn snd_timer_ref_get(t: *mut snd_timer) {
    rust_timer_ref_get(addr_of_mut!((*t).kref));
}
unsafe fn snd_timer_ref_put(t: *mut snd_timer) {
    rust_timer_ref_put(addr_of_mut!((*t).kref), Some(snd_timer_kref_release));
}
#[no_mangle]
pub unsafe extern "C" fn snd_timeri_timer_get(ti: *mut snd_timer_instance) -> *mut snd_timer {
    let _lock = ReadGuard::new();
    let t = (*ti).timer;
    if !t.is_null() {
        snd_timer_ref_get(t);
    }
    t
}
#[no_mangle]
pub unsafe extern "C" fn snd_timeri_timer_put(t: *mut snd_timer) {
    snd_timer_ref_put(t);
}
unsafe fn check_matching_master_slave(
    master: *mut snd_timer_instance,
    slave: *mut snd_timer_instance,
) -> c_int {
    if (*slave).slave_class != (*master).slave_class || (*slave).slave_id != (*master).slave_id {
        return 0;
    }
    let t = (*master).timer;
    if (*t).num_instances >= (*t).max_instances {
        return err!(EBUSY);
    }
    list_move_tail(
        addr_of_mut!((*slave).open_list),
        addr_of_mut!((*master).slave_list_head),
    );
    (*t).num_instances += 1;
    snd_timer_ref_get(t);
    let _w = WriteGuard::new();
    let _s = SpinGuard::new(addr_of_mut!((*t).lock));
    (*slave).master = master;
    (*slave).timer = t;
    if (*slave).flags & SNDRV_TIMER_IFLG_RUNNING != 0 {
        list_add_tail(
            addr_of_mut!((*slave).active_list),
            addr_of_mut!((*master).slave_active_head),
        );
    }
    1
}
unsafe fn snd_timer_has_slave_key(ti: *mut snd_timer_instance) -> bool {
    (*ti).flags & SNDRV_TIMER_IFLG_SLAVE == 0
        && (*ti).slave_class > SNDRV_TIMER_SCLASS_NONE as c_int
}
unsafe fn snd_timer_check_slave(slave: *mut snd_timer_instance) -> c_int {
    for_each!(
        master,
        rust_timer_master_list(),
        snd_timer_instance,
        master_list,
        {
            let err = check_matching_master_slave(master, slave);
            if err != 0 {
                return err.min(0);
            }
        }
    );
    0
}
unsafe fn snd_timer_check_master(master: *mut snd_timer_instance) -> c_int {
    for_each!(
        slave,
        rust_timer_slave_list(),
        snd_timer_instance,
        open_list,
        {
            let err = check_matching_master_slave(master, slave);
            if err < 0 {
                return err;
            }
        }
    );
    0
}
#[no_mangle]
pub unsafe extern "C" fn snd_timer_open(
    ti: *mut snd_timer_instance,
    tid: *mut snd_timer_id,
    slave_id: c_uint,
) -> c_int {
    let mut card_dev = null_mut();
    rust_timer_mutex_lock(rust_timer_register_mutex());
    let ret = (|| {
        let ret;
        if (*tid).dev_class == SNDRV_TIMER_CLASS_SLAVE as c_int {
            if (*tid).dev_sclass <= SNDRV_TIMER_SCLASS_NONE as c_int
                || (*tid).dev_sclass > SNDRV_TIMER_SCLASS_OSS_SEQUENCER as c_int
            {
                rust_timer_invalid_slave((*tid).dev_sclass);
                return err!(EINVAL);
            }
            if num_slaves >= MAX_SLAVE_INSTANCES {
                return err!(EBUSY);
            }
            (*ti).slave_class = (*tid).dev_sclass;
            (*ti).slave_id = (*tid).device as _;
            (*ti).flags |= SNDRV_TIMER_IFLG_SLAVE;
            list_add_tail(addr_of_mut!((*ti).open_list), rust_timer_slave_list());
            num_slaves += 1;
            ret = snd_timer_check_slave(ti);
        } else {
            #[allow(unused_mut)]
            let mut t = snd_timer_find(tid);
            #[cfg(CONFIG_MODULES)]
            if t.is_null() {
                rust_timer_mutex_unlock(rust_timer_register_mutex());
                snd_timer_request(tid);
                rust_timer_mutex_lock(rust_timer_register_mutex());
                t = snd_timer_find(tid);
            }
            if t.is_null() {
                return err!(ENODEV);
            }
            if !list_empty(addr_of_mut!((*t).open_list_head)) {
                let first = (*t)
                    .open_list_head
                    .next
                    .cast::<u8>()
                    .sub(offset_of!(snd_timer_instance, open_list))
                    .cast::<snd_timer_instance>();
                if (*first).flags & SNDRV_TIMER_IFLG_EXCLUSIVE != 0 {
                    return err!(EBUSY);
                }
            }
            if (*t).num_instances >= (*t).max_instances {
                return err!(EBUSY);
            }
            if !rust_timer_try_module_get((*t).module) {
                return err!(EBUSY);
            }
            if !(*t).card.is_null() {
                card_dev = rust_timer_card_device((*t).card);
                get_device(card_dev);
            }
            if list_empty(addr_of_mut!((*t).open_list_head)) {
                if let Some(open) = (*t).hw.open {
                    let err = open(t);
                    if err != 0 {
                        rust_timer_module_put((*t).module);
                        return err;
                    }
                }
            }
            (*ti).timer = t;
            (*ti).slave_class = (*tid).dev_sclass;
            (*ti).slave_id = slave_id;
            list_add_tail(
                addr_of_mut!((*ti).open_list),
                addr_of_mut!((*t).open_list_head),
            );
            if snd_timer_has_slave_key(ti) {
                list_add_tail(addr_of_mut!((*ti).master_list), rust_timer_master_list());
            }
            (*t).num_instances += 1;
            snd_timer_ref_get(t);
            ret = snd_timer_check_master(ti);
        }
        if ret < 0 {
            snd_timer_close_locked(ti, &raw mut card_dev);
        }
        ret
    })();
    rust_timer_mutex_unlock(rust_timer_register_mutex());
    if ret < 0 && !card_dev.is_null() {
        put_device(card_dev);
    }
    ret
}
unsafe fn remove_slave_links(ti: *mut snd_timer_instance, t: *mut snd_timer) {
    let _w = WriteGuard::new();
    let _s = SpinGuard::new(addr_of_mut!((*t).lock));
    (*ti).timer = null_mut();
    for_each!(
        slave,
        addr_of_mut!((*ti).slave_list_head),
        snd_timer_instance,
        open_list,
        {
            list_move_tail(addr_of_mut!((*slave).open_list), rust_timer_slave_list());
            (*t).num_instances -= 1;
            snd_timer_ref_put(t);
            (*slave).master = null_mut();
            (*slave).timer = null_mut();
            list_del_init(addr_of_mut!((*slave).ack_list));
            list_del_init(addr_of_mut!((*slave).active_list));
        }
    );
    (*ti).flags &= !SNDRV_TIMER_IFLG_DEAD;
}
unsafe fn snd_timer_close_locked(ti: *mut snd_timer_instance, card_dev: *mut *mut device) {
    let t = (*ti).timer;
    if !t.is_null() {
        let _s = SpinGuard::irq(addr_of_mut!((*t).lock));
        if (*ti).flags & SNDRV_TIMER_IFLG_DEAD != 0 {
            return;
        }
        (*ti).flags |= SNDRV_TIMER_IFLG_DEAD;
    }
    if !list_empty(addr_of_mut!((*ti).open_list)) {
        list_del_init(addr_of_mut!((*ti).open_list));
        if (*ti).flags & SNDRV_TIMER_IFLG_SLAVE != 0 {
            num_slaves -= 1;
        }
    }
    if !list_empty(addr_of_mut!((*ti).master_list)) {
        list_del_init(addr_of_mut!((*ti).master_list));
    }
    snd_timer_stop(ti);
    if !t.is_null() {
        (*t).num_instances -= 1;
        rust_timer_spin_lock_irq(addr_of_mut!((*t).lock));
        for_each!(
            slave,
            addr_of_mut!((*ti).slave_list_head),
            snd_timer_instance,
            open_list,
            {
                list_del_init(addr_of_mut!((*slave).ack_list));
            }
        );
        loop {
            let mut busy = (*ti).flags & SNDRV_TIMER_IFLG_CALLBACK != 0;
            for_each!(
                slave,
                addr_of_mut!((*ti).slave_list_head),
                snd_timer_instance,
                open_list,
                {
                    busy |= (*slave).flags & SNDRV_TIMER_IFLG_CALLBACK != 0;
                }
            );
            if !busy {
                break;
            }
            rust_timer_spin_unlock_irq(addr_of_mut!((*t).lock));
            rust_timer_udelay(10);
            rust_timer_spin_lock_irq(addr_of_mut!((*t).lock));
        }
        rust_timer_spin_unlock_irq(addr_of_mut!((*t).lock));
        remove_slave_links(ti, t);
        if (*ti).flags & SNDRV_TIMER_IFLG_SLAVE == 0 {
            if list_empty(addr_of_mut!((*t).open_list_head)) {
                if let Some(close) = (*t).hw.close {
                    close(t);
                }
            }
            if !(*t).card.is_null() {
                *card_dev = rust_timer_card_device((*t).card);
            }
            rust_timer_module_put((*t).module);
        }
        snd_timer_ref_put(t);
    }
}
#[no_mangle]
pub unsafe extern "C" fn snd_timer_close(ti: *mut snd_timer_instance) {
    if rust_timer_bug_on(ti.is_null()) {
        return;
    }
    let mut card_dev = null_mut();
    {
        let _m = MutexGuard::new(rust_timer_register_mutex());
        snd_timer_close_locked(ti, &raw mut card_dev);
    }
    if !card_dev.is_null() {
        put_device(card_dev);
    }
}
unsafe fn snd_timer_hw_resolution(t: *mut snd_timer) -> c_ulong {
    if let Some(f) = (*t).hw.c_resolution {
        f(t)
    } else {
        (*t).hw.resolution
    }
}
#[no_mangle]
pub unsafe extern "C" fn snd_timer_resolution(ti: *mut snd_timer_instance) -> c_ulong {
    if ti.is_null() {
        return 0;
    }
    let hold = TimerRef(snd_timeri_timer_get(ti));
    let t = hold.0;
    if t.is_null() {
        return 0;
    }
    let _s = SpinGuard::irqsave(addr_of_mut!((*t).lock));
    snd_timer_hw_resolution(t)
}
unsafe fn snd_timer_notify1(ti: *mut snd_timer_instance, mut event: c_int) {
    let t = (*ti).timer;
    let mut resolution = 0;
    let mut stamp: timespec64 = zeroed();
    timestamp(&raw mut stamp);
    if rust_timer_bug_on(
        event < SNDRV_TIMER_EVENT_START as _ || event > SNDRV_TIMER_EVENT_PAUSE as _,
    ) {
        return;
    }
    if !t.is_null()
        && (event == SNDRV_TIMER_EVENT_START as _ || event == SNDRV_TIMER_EVENT_CONTINUE as _)
    {
        resolution = snd_timer_hw_resolution(t);
    }
    if let Some(f) = (*ti).ccallback {
        f(ti, event, &raw mut stamp, resolution);
    }
    if (*ti).flags & SNDRV_TIMER_IFLG_SLAVE != 0
        || t.is_null()
        || (*t).hw.flags & SNDRV_TIMER_HW_SLAVE != 0
    {
        return;
    }
    event += 10;
    for_each!(
        ts,
        addr_of_mut!((*ti).slave_active_head),
        snd_timer_instance,
        active_list,
        {
            if let Some(f) = (*ts).ccallback {
                f(ts, event, &raw mut stamp, resolution);
            }
        }
    );
}
unsafe fn snd_timer_start1(ti: *mut snd_timer_instance, start: bool, ticks: c_ulong) -> c_int {
    let t = (*ti).timer;
    if t.is_null() {
        return err!(EINVAL);
    }
    let _s = SpinGuard::new(addr_of_mut!((*t).lock));
    if (*ti).flags & SNDRV_TIMER_IFLG_DEAD != 0 {
        return err!(EINVAL);
    }
    if card_shutdown(t) {
        return err!(ENODEV);
    }
    if (*ti).flags & (SNDRV_TIMER_IFLG_RUNNING | SNDRV_TIMER_IFLG_START) != 0 {
        return err!(EBUSY);
    }
    if start
        && (*t).hw.flags & SNDRV_TIMER_HW_SLAVE == 0
        && (snd_timer_hw_resolution(t) as u64).wrapping_mul(ticks as u64) < 100000
    {
        return err!(EINVAL);
    }
    if start {
        (*ti).ticks = ticks;
        (*ti).cticks = ticks;
    } else if (*ti).cticks == 0 {
        (*ti).cticks = 1;
    }
    (*ti).pticks = 0;
    list_move_tail(
        addr_of_mut!((*ti).active_list),
        addr_of_mut!((*t).active_list_head),
    );
    let result;
    if (*t).running != 0 && (*t).hw.flags & SNDRV_TIMER_HW_SLAVE == 0 {
        (*t).flags |= SNDRV_TIMER_FLG_RESCHED;
        (*ti).flags |= SNDRV_TIMER_IFLG_START;
        result = 1;
    } else {
        if (*t).running == 0 {
            if start {
                (*t).sticks = ticks;
            }
            (*t).hw.start.unwrap_unchecked()(t);
        }
        (*t).running += 1;
        (*ti).flags |= SNDRV_TIMER_IFLG_RUNNING;
        result = 0;
    }
    snd_timer_notify1(
        ti,
        if start {
            SNDRV_TIMER_EVENT_START as _
        } else {
            SNDRV_TIMER_EVENT_CONTINUE as _
        },
    );
    result
}
unsafe fn snd_timer_start_slave(ti: *mut snd_timer_instance, start: bool) -> c_int {
    if (*ti).flags & SNDRV_TIMER_IFLG_DEAD != 0 {
        return err!(EINVAL);
    }
    if (*ti).flags & SNDRV_TIMER_IFLG_RUNNING != 0 {
        return err!(EBUSY);
    }
    (*ti).flags |= SNDRV_TIMER_IFLG_RUNNING;
    if !(*ti).master.is_null() && !(*ti).timer.is_null() {
        let _s = SpinGuard::new(addr_of_mut!((*(*ti).timer).lock));
        list_add_tail(
            addr_of_mut!((*ti).active_list),
            addr_of_mut!((*(*ti).master).slave_active_head),
        );
        snd_timer_notify1(
            ti,
            if start {
                SNDRV_TIMER_EVENT_START as _
            } else {
                SNDRV_TIMER_EVENT_CONTINUE as _
            },
        );
    }
    1
}
unsafe fn snd_timer_stop1(ti: *mut snd_timer_instance, stop: bool) -> c_int {
    let t = (*ti).timer;
    if t.is_null() {
        return err!(EINVAL);
    }
    let _s = SpinGuard::new(addr_of_mut!((*t).lock));
    list_del_init(addr_of_mut!((*ti).ack_list));
    list_del_init(addr_of_mut!((*ti).active_list));
    if (*ti).flags & (SNDRV_TIMER_IFLG_RUNNING | SNDRV_TIMER_IFLG_START) == 0 {
        return err!(EBUSY);
    }
    if card_shutdown(t) {
        return 0;
    }
    if stop {
        (*ti).cticks = (*ti).ticks;
        (*ti).pticks = 0;
    }
    if (*ti).flags & SNDRV_TIMER_IFLG_RUNNING != 0 {
        (*t).running -= 1;
        if (*t).running == 0 {
            (*t).hw.stop.unwrap_unchecked()(t);
            if (*t).flags & SNDRV_TIMER_FLG_RESCHED != 0 {
                (*t).flags &= !SNDRV_TIMER_FLG_RESCHED;
                snd_timer_reschedule(t, 0);
                if (*t).flags & SNDRV_TIMER_FLG_CHANGE != 0 {
                    (*t).flags &= !SNDRV_TIMER_FLG_CHANGE;
                    (*t).hw.start.unwrap_unchecked()(t);
                }
            }
        }
    }
    (*ti).flags &= !(SNDRV_TIMER_IFLG_RUNNING | SNDRV_TIMER_IFLG_START);
    if stop {
        (*ti).flags &= !SNDRV_TIMER_IFLG_PAUSED;
    } else {
        (*ti).flags |= SNDRV_TIMER_IFLG_PAUSED;
    }
    snd_timer_notify1(
        ti,
        if stop {
            SNDRV_TIMER_EVENT_STOP as _
        } else {
            SNDRV_TIMER_EVENT_PAUSE as _
        },
    );
    0
}
unsafe fn snd_timer_stop_slave(ti: *mut snd_timer_instance, stop: bool) -> c_int {
    let running = (*ti).flags & SNDRV_TIMER_IFLG_RUNNING != 0;
    (*ti).flags &= !SNDRV_TIMER_IFLG_RUNNING;
    if !(*ti).timer.is_null() {
        let _s = SpinGuard::new(addr_of_mut!((*(*ti).timer).lock));
        list_del_init(addr_of_mut!((*ti).ack_list));
        list_del_init(addr_of_mut!((*ti).active_list));
        if running {
            snd_timer_notify1(
                ti,
                if stop {
                    SNDRV_TIMER_EVENT_STOP as _
                } else {
                    SNDRV_TIMER_EVENT_PAUSE as _
                },
            );
        }
    }
    if running {
        0
    } else {
        err!(EBUSY)
    }
}
#[no_mangle]
pub unsafe extern "C" fn snd_timer_start(ti: *mut snd_timer_instance, ticks: c_uint) -> c_int {
    if ti.is_null() || ticks < 1 {
        return err!(EINVAL);
    }
    let _r = ReadGuard::new();
    if (*ti).flags & SNDRV_TIMER_IFLG_SLAVE != 0 {
        snd_timer_start_slave(ti, true)
    } else {
        snd_timer_start1(ti, true, ticks as _)
    }
}
#[no_mangle]
pub unsafe extern "C" fn snd_timer_stop(ti: *mut snd_timer_instance) -> c_int {
    let _r = ReadGuard::new();
    if (*ti).flags & SNDRV_TIMER_IFLG_SLAVE != 0 {
        snd_timer_stop_slave(ti, true)
    } else {
        snd_timer_stop1(ti, true)
    }
}
#[no_mangle]
pub unsafe extern "C" fn snd_timer_continue(ti: *mut snd_timer_instance) -> c_int {
    if (*ti).flags & SNDRV_TIMER_IFLG_PAUSED == 0 {
        return err!(EINVAL);
    }
    let _r = ReadGuard::new();
    if (*ti).flags & SNDRV_TIMER_IFLG_SLAVE != 0 {
        snd_timer_start_slave(ti, false)
    } else {
        snd_timer_start1(ti, false, 0)
    }
}
#[no_mangle]
pub unsafe extern "C" fn snd_timer_pause(ti: *mut snd_timer_instance) -> c_int {
    let _r = ReadGuard::new();
    if (*ti).flags & SNDRV_TIMER_IFLG_SLAVE != 0 {
        snd_timer_stop_slave(ti, false)
    } else {
        snd_timer_stop1(ti, false)
    }
}
unsafe fn snd_timer_reschedule(t: *mut snd_timer, ticks_left: c_ulong) {
    let mut ticks = c_ulong::MAX;
    for_each!(
        ti,
        addr_of_mut!((*t).active_list_head),
        snd_timer_instance,
        active_list,
        {
            if (*ti).flags & SNDRV_TIMER_IFLG_START != 0 {
                (*ti).flags &= !SNDRV_TIMER_IFLG_START;
                (*ti).flags |= SNDRV_TIMER_IFLG_RUNNING;
                (*t).running += 1;
            }
            if (*ti).flags & SNDRV_TIMER_IFLG_RUNNING != 0 {
                ticks = ticks.min((*ti).cticks);
            }
        }
    );
    if ticks == c_ulong::MAX {
        (*t).flags &= !SNDRV_TIMER_FLG_RESCHED;
        return;
    }
    ticks = ticks.min((*t).hw.ticks);
    if ticks_left != ticks {
        (*t).flags |= SNDRV_TIMER_FLG_CHANGE;
    }
    (*t).sticks = ticks;
}
unsafe fn snd_timer_process_callbacks(t: *mut snd_timer, head: *mut list_head) {
    while !list_empty(head) {
        let ti = (*head)
            .next
            .cast::<u8>()
            .sub(offset_of!(snd_timer_instance, ack_list))
            .cast::<snd_timer_instance>();
        list_del_init(addr_of_mut!((*ti).ack_list));
        if (*ti).flags & SNDRV_TIMER_IFLG_DEAD == 0 {
            let ticks = (*ti).pticks;
            (*ti).pticks = 0;
            let resolution = (*ti).resolution;
            (*ti).flags |= SNDRV_TIMER_IFLG_CALLBACK;
            rust_timer_spin_unlock(addr_of_mut!((*t).lock));
            if let Some(f) = (*ti).callback {
                f(ti, resolution, ticks);
            }
            rust_timer_spin_lock(addr_of_mut!((*t).lock));
            (*ti).flags &= !SNDRV_TIMER_IFLG_CALLBACK;
        }
    }
}
unsafe fn snd_timer_clear_callbacks(t: *mut snd_timer, head: *mut list_head) {
    let _s = SpinGuard::irqsave(addr_of_mut!((*t).lock));
    while !list_empty(head) {
        list_del_init((*head).next);
    }
}
unsafe extern "C" fn snd_timer_work(work: *mut work_struct) {
    let t = work
        .cast::<u8>()
        .sub(offset_of!(snd_timer, task_work))
        .cast::<snd_timer>();
    if card_shutdown(t) {
        snd_timer_clear_callbacks(t, addr_of_mut!((*t).sack_list_head));
        return;
    }
    let _s = SpinGuard::irqsave(addr_of_mut!((*t).lock));
    snd_timer_process_callbacks(t, addr_of_mut!((*t).sack_list_head));
}
#[no_mangle]
pub unsafe extern "C" fn snd_timer_interrupt(t: *mut snd_timer, ticks_left: c_ulong) {
    if t.is_null() {
        return;
    }
    if card_shutdown(t) {
        snd_timer_clear_callbacks(t, addr_of_mut!((*t).ack_list_head));
        return;
    }
    let _s = SpinGuard::irqsave(addr_of_mut!((*t).lock));
    let resolution = snd_timer_hw_resolution(t);
    for_each!(
        ti,
        addr_of_mut!((*t).active_list_head),
        snd_timer_instance,
        active_list,
        {
            if (*ti).flags & SNDRV_TIMER_IFLG_DEAD != 0
                || (*ti).flags & SNDRV_TIMER_IFLG_RUNNING == 0
            {
                continue;
            }
            (*ti).pticks = (*ti).pticks.wrapping_add(ticks_left);
            (*ti).resolution = resolution;
            (*ti).cticks = (*ti).cticks.saturating_sub(ticks_left);
            if (*ti).cticks != 0 {
                continue;
            }
            if (*ti).flags & SNDRV_TIMER_IFLG_AUTO != 0 {
                (*ti).cticks = (*ti).ticks;
            } else {
                (*ti).flags &= !SNDRV_TIMER_IFLG_RUNNING;
                (*t).running -= 1;
                list_del_init(addr_of_mut!((*ti).active_list));
            }
            let ack = if (*t).hw.flags & SNDRV_TIMER_HW_WORK != 0
                || (*ti).flags & SNDRV_TIMER_IFLG_FAST != 0
            {
                addr_of_mut!((*t).ack_list_head)
            } else {
                addr_of_mut!((*t).sack_list_head)
            };
            if list_empty(addr_of_mut!((*ti).ack_list))
                && (*ti).flags & SNDRV_TIMER_IFLG_CALLBACK == 0
            {
                list_add_tail(addr_of_mut!((*ti).ack_list), ack);
            }
            for_each!(
                ts,
                addr_of_mut!((*ti).slave_active_head),
                snd_timer_instance,
                active_list,
                {
                    (*ts).pticks = (*ti).pticks;
                    (*ts).resolution = resolution;
                    if list_empty(addr_of_mut!((*ts).ack_list))
                        && (*ts).flags & SNDRV_TIMER_IFLG_CALLBACK == 0
                    {
                        list_add_tail(addr_of_mut!((*ts).ack_list), ack);
                    }
                }
            );
        }
    );
    if (*t).flags & SNDRV_TIMER_FLG_RESCHED != 0 {
        snd_timer_reschedule(t, (*t).sticks);
    }
    if (*t).running != 0 {
        if (*t).hw.flags & SNDRV_TIMER_HW_STOP != 0 {
            (*t).hw.stop.unwrap_unchecked()(t);
            (*t).flags |= SNDRV_TIMER_FLG_CHANGE;
        }
        if (*t).hw.flags & SNDRV_TIMER_HW_AUTO == 0 || (*t).flags & SNDRV_TIMER_FLG_CHANGE != 0 {
            (*t).flags &= !SNDRV_TIMER_FLG_CHANGE;
            (*t).hw.start.unwrap_unchecked()(t);
        }
    } else {
        (*t).hw.stop.unwrap_unchecked()(t);
    }
    snd_timer_process_callbacks(t, addr_of_mut!((*t).ack_list_head));
    if !list_empty(addr_of_mut!((*t).sack_list_head)) {
        rust_timer_queue_work(addr_of_mut!((*t).task_work));
    }
}
#[no_mangle]
pub unsafe extern "C" fn snd_timer_new(
    card: *mut snd_card,
    id: *mut c_char,
    tid: *mut snd_timer_id,
    rtimer: *mut *mut snd_timer,
) -> c_int {
    static mut OPS: snd_device_ops = snd_device_ops {
        dev_free: Some(snd_timer_dev_free),
        dev_register: Some(snd_timer_dev_register),
        dev_disconnect: Some(snd_timer_dev_disconnect),
    };
    if rust_timer_bug_on(tid.is_null()) {
        return err!(EINVAL);
    }
    if ((*tid).dev_class == SNDRV_TIMER_CLASS_CARD as c_int
        || (*tid).dev_class == SNDRV_TIMER_CLASS_PCM as c_int)
        && rust_timer_warn_on(card.is_null())
    {
        return err!(EINVAL);
    }
    if !rtimer.is_null() {
        *rtimer = null_mut();
    }
    let t: *mut snd_timer = rust_timer_zalloc(size_of::<snd_timer>()).cast();
    if t.is_null() {
        return err!(ENOMEM);
    }
    (*t).tmr_class = (*tid).dev_class;
    (*t).card = card;
    (*t).tmr_device = (*tid).device;
    (*t).tmr_subdevice = (*tid).subdevice;
    if !id.is_null() {
        strscpy(addr_of_mut!((*t).id).cast(), id, size_of_val_id());
    }
    (*t).sticks = 1;
    list_init(addr_of_mut!((*t).device_list));
    list_init(addr_of_mut!((*t).open_list_head));
    list_init(addr_of_mut!((*t).active_list_head));
    list_init(addr_of_mut!((*t).ack_list_head));
    list_init(addr_of_mut!((*t).sack_list_head));
    rust_timer_spin_init(addr_of_mut!((*t).lock));
    rust_timer_work_init(addr_of_mut!((*t).task_work), Some(snd_timer_work));
    (*t).max_instances = 1000;
    rust_timer_ref_init(addr_of_mut!((*t).kref));
    if !card.is_null() {
        (*t).module = rust_timer_card_module(card);
        let err = rust_timer_device_new(card, t.cast(), &raw const OPS);
        if err < 0 {
            snd_timer_free(t);
            return err;
        }
    }
    if !rtimer.is_null() {
        *rtimer = t;
    }
    0
}
const fn size_of_val_id() -> usize {
    size_of::<[c_char; 64]>()
}
unsafe extern "C" fn snd_timer_kref_release(k: *mut kref) {
    let t = k
        .cast::<u8>()
        .sub(offset_of!(snd_timer, kref))
        .cast::<snd_timer>();
    if let Some(f) = (*t).private_free {
        f(t);
    }
    kfree(t.cast());
}
unsafe fn snd_timer_free(t: *mut snd_timer) -> c_int {
    if t.is_null() {
        return 0;
    }
    {
        let _m = MutexGuard::new(rust_timer_register_mutex());
        for_each!(
            ti,
            addr_of_mut!((*t).open_list_head),
            snd_timer_instance,
            open_list,
            {
                let mut card_dev = null_mut();
                snd_timer_close_locked(ti, &raw mut card_dev);
                put_device(card_dev);
            }
        );
        rust_timer_list_del(addr_of_mut!((*t).device_list));
    }
    disable_work_sync(addr_of_mut!((*t).task_work));
    snd_timer_ref_put(t);
    0
}
unsafe extern "C" fn snd_timer_dev_free(d: *mut snd_device) -> c_int {
    snd_timer_free((*d).device_data.cast())
}
unsafe extern "C" fn snd_timer_dev_register(d: *mut snd_device) -> c_int {
    let t: *mut snd_timer = (*d).device_data.cast();
    if rust_timer_bug_on(t.is_null() || (*t).hw.start.is_none() || (*t).hw.stop.is_none()) {
        return err!(ENXIO);
    }
    if (*t).hw.flags & SNDRV_TIMER_HW_SLAVE == 0
        && (*t).hw.resolution == 0
        && (*t).hw.c_resolution.is_none()
    {
        return err!(EINVAL);
    }
    let _m = MutexGuard::new(rust_timer_register_mutex());
    let mut before = rust_timer_list();
    for_each!(other, rust_timer_list(), snd_timer, device_list, {
        if (*other).tmr_class > (*t).tmr_class {
            before = addr_of_mut!((*other).device_list);
            break;
        }
        if (*other).tmr_class < (*t).tmr_class {
            continue;
        }
        if !(*other).card.is_null() && !(*t).card.is_null() {
            if card_number(other) > card_number(t) {
                before = addr_of_mut!((*other).device_list);
                break;
            }
            if card_number(other) < card_number(t) {
                continue;
            }
        }
        if (*other).tmr_device > (*t).tmr_device {
            before = addr_of_mut!((*other).device_list);
            break;
        }
        if (*other).tmr_device < (*t).tmr_device {
            continue;
        }
        if (*other).tmr_subdevice > (*t).tmr_subdevice {
            before = addr_of_mut!((*other).device_list);
            break;
        }
        if (*other).tmr_subdevice < (*t).tmr_subdevice {
            continue;
        }
        return err!(EBUSY);
    });
    list_add_tail(addr_of_mut!((*t).device_list), before);
    0
}
unsafe extern "C" fn snd_timer_dev_disconnect(d: *mut snd_device) -> c_int {
    let t: *mut snd_timer = (*d).device_data.cast();
    let _m = MutexGuard::new(rust_timer_register_mutex());
    list_del_init(addr_of_mut!((*t).device_list));
    for_each!(
        ti,
        addr_of_mut!((*t).open_list_head),
        snd_timer_instance,
        open_list,
        {
            if let Some(f) = (*ti).disconnect {
                f(ti);
            }
        }
    );
    0
}
#[no_mangle]
pub unsafe extern "C" fn snd_timer_notify(t: *mut snd_timer, event: c_int, stamp: *mut timespec64) {
    if card_shutdown(t) || (*t).hw.flags & SNDRV_TIMER_HW_SLAVE == 0 {
        return;
    }
    if rust_timer_bug_on(
        event < SNDRV_TIMER_EVENT_MSTART as _ || event > SNDRV_TIMER_EVENT_MRESUME as _,
    ) {
        return;
    }
    let _s = SpinGuard::irqsave(addr_of_mut!((*t).lock));
    let resolution = if event == SNDRV_TIMER_EVENT_MSTART as _
        || event == SNDRV_TIMER_EVENT_MCONTINUE as _
        || event == SNDRV_TIMER_EVENT_MRESUME as _
    {
        snd_timer_hw_resolution(t)
    } else {
        0
    };
    for_each!(
        ti,
        addr_of_mut!((*t).active_list_head),
        snd_timer_instance,
        active_list,
        {
            if let Some(f) = (*ti).ccallback {
                f(ti, event, stamp, resolution);
            }
            for_each!(
                ts,
                addr_of_mut!((*ti).slave_active_head),
                snd_timer_instance,
                active_list,
                {
                    if let Some(f) = (*ts).ccallback {
                        f(ts, event, stamp, resolution);
                    }
                }
            );
        }
    );
}
#[no_mangle]
pub unsafe extern "C" fn snd_timer_global_new(
    id: *mut c_char,
    device: c_int,
    rtimer: *mut *mut snd_timer,
) -> c_int {
    let mut tid = snd_timer_id {
        dev_class: SNDRV_TIMER_CLASS_GLOBAL as _,
        dev_sclass: SNDRV_TIMER_SCLASS_NONE as _,
        card: -1,
        device,
        subdevice: 0,
    };
    snd_timer_new(null_mut(), id, &raw mut tid, rtimer)
}
#[no_mangle]
pub unsafe extern "C" fn snd_timer_global_free(t: *mut snd_timer) -> c_int {
    snd_timer_free(t)
}
#[no_mangle]
pub unsafe extern "C" fn snd_timer_global_register(t: *mut snd_timer) -> c_int {
    let mut dev: snd_device = zeroed();
    dev.device_data = t.cast();
    snd_timer_dev_register(&raw mut dev)
}
unsafe extern "C" fn snd_timer_s_function(p: *mut timer_list) {
    let private = p
        .cast::<u8>()
        .sub(offset_of!(snd_timer_system_private, tlist))
        .cast::<snd_timer_system_private>();
    let jiff = rust_timer_jiffies();
    if ((*private).last_expires.wrapping_sub(jiff) as c_long) < 0 {
        (*private).correction = (*private)
            .correction
            .wrapping_add(jiff.wrapping_sub((*private).last_expires));
    }
    snd_timer_interrupt(
        (*private).snd_timer,
        jiff.wrapping_sub((*private).last_jiffies),
    );
}
unsafe extern "C" fn snd_timer_s_start(t: *mut snd_timer) -> c_int {
    let private: *mut snd_timer_system_private = (*t).private_data.cast();
    let mut next = rust_timer_jiffies();
    (*private).last_jiffies = next;
    if (*private).correction > (*t).sticks.wrapping_sub(1) {
        (*private).correction = (*private)
            .correction
            .wrapping_sub((*t).sticks.wrapping_sub(1));
        next = next.wrapping_add(1);
    } else {
        next = next.wrapping_add((*t).sticks.wrapping_sub((*private).correction));
        (*private).correction = 0;
    }
    (*private).last_expires = next;
    mod_timer(addr_of_mut!((*private).tlist), next);
    0
}
unsafe extern "C" fn snd_timer_s_stop(t: *mut snd_timer) -> c_int {
    let private: *mut snd_timer_system_private = (*t).private_data.cast();
    timer_delete(addr_of_mut!((*private).tlist));
    let jiff = rust_timer_jiffies();
    (*t).sticks = if (jiff.wrapping_sub((*private).last_expires) as c_long) < 0 {
        (*private).last_expires.wrapping_sub(jiff)
    } else {
        1
    };
    (*private).correction = 0;
    0
}
unsafe extern "C" fn snd_timer_s_close(t: *mut snd_timer) -> c_int {
    let private: *mut snd_timer_system_private = (*t).private_data.cast();
    timer_delete_sync(addr_of_mut!((*private).tlist));
    0
}
unsafe extern "C" fn snd_timer_free_system(t: *mut snd_timer) {
    kfree((*t).private_data);
}
unsafe fn snd_timer_register_system() -> c_int {
    let mut t = null_mut();
    let err = snd_timer_global_new(
        c"system".as_ptr().cast::<c_char>().cast_mut(),
        SNDRV_TIMER_GLOBAL_SYSTEM as _,
        &raw mut t,
    );
    if err < 0 {
        return err;
    }
    strscpy(
        addr_of_mut!((*t).name).cast(),
        c"system timer".as_ptr().cast::<c_char>(),
        size_of::<[c_char; 80]>(),
    );
    (*t).hw = snd_timer_hardware {
        flags: SNDRV_TIMER_HW_FIRST | SNDRV_TIMER_HW_WORK,
        resolution: RUST_TIMER_SYSTEM_RESOLUTION as _,
        ticks: 10000000,
        close: Some(snd_timer_s_close),
        start: Some(snd_timer_s_start),
        stop: Some(snd_timer_s_stop),
        ..zeroed()
    };
    let private: *mut snd_timer_system_private =
        rust_timer_zalloc(size_of::<snd_timer_system_private>()).cast();
    if private.is_null() {
        snd_timer_free(t);
        return err!(ENOMEM);
    }
    (*private).snd_timer = t;
    rust_timer_setup(addr_of_mut!((*private).tlist), Some(snd_timer_s_function));
    (*t).private_data = private.cast();
    (*t).private_free = Some(snd_timer_free_system);
    snd_timer_global_register(t)
}
#[cfg(CONFIG_SND_PROC_FS)]
unsafe extern "C" fn snd_timer_proc_read(
    _entry: *mut snd_info_entry,
    buffer: *mut snd_info_buffer,
) {
    let _m = MutexGuard::new(rust_timer_register_mutex());
    for_each!(t, rust_timer_list(), snd_timer, device_list, {
        if card_shutdown(t) {
            continue;
        }
        match (*t).tmr_class {
            x if x == SNDRV_TIMER_CLASS_GLOBAL as c_int => {
                snd_iprintf!(buffer, c"G%i: ".as_ptr().cast::<c_char>(), (*t).tmr_device);
            }
            x if x == SNDRV_TIMER_CLASS_CARD as c_int => {
                snd_iprintf!(
                    buffer,
                    c"C%i-%i: ".as_ptr().cast::<c_char>(),
                    card_number(t),
                    (*t).tmr_device
                );
            }
            x if x == SNDRV_TIMER_CLASS_PCM as c_int => {
                snd_iprintf!(
                    buffer,
                    c"P%i-%i-%i: ".as_ptr().cast::<c_char>(),
                    card_number(t),
                    (*t).tmr_device,
                    (*t).tmr_subdevice
                );
            }
            _ => {
                snd_iprintf!(
                    buffer,
                    c"?%i-%i-%i-%i: ".as_ptr().cast::<c_char>(),
                    (*t).tmr_class,
                    card_number(t),
                    (*t).tmr_device,
                    (*t).tmr_subdevice
                );
            }
        }
        snd_iprintf!(
            buffer,
            c"%s :".as_ptr().cast::<c_char>(),
            addr_of_mut!((*t).name).cast::<c_char>()
        );
        let resolution = {
            let _s = SpinGuard::irq(addr_of_mut!((*t).lock));
            snd_timer_hw_resolution(t)
        };
        if resolution != 0 {
            snd_iprintf!(
                buffer,
                c" %lu.%03luus (%lu ticks)".as_ptr().cast::<c_char>(),
                resolution / 1000,
                resolution % 1000,
                (*t).hw.ticks
            );
        }
        if (*t).hw.flags & SNDRV_TIMER_HW_SLAVE != 0 {
            snd_iprintf!(buffer, c" SLAVE".as_ptr().cast::<c_char>());
        }
        snd_iprintf!(buffer, c"\n".as_ptr().cast::<c_char>());
        for_each!(
            ti,
            addr_of_mut!((*t).open_list_head),
            snd_timer_instance,
            open_list,
            {
                snd_iprintf!(
                    buffer,
                    c"  Client %s : %s\n".as_ptr().cast::<c_char>(),
                    if (*ti).owner.is_null() {
                        c"unknown".as_ptr().cast::<c_char>()
                    } else {
                        (*ti).owner
                    },
                    if (*ti).flags & (SNDRV_TIMER_IFLG_START | SNDRV_TIMER_IFLG_RUNNING) != 0 {
                        c"running".as_ptr().cast::<c_char>()
                    } else {
                        c"stopped".as_ptr().cast::<c_char>()
                    }
                );
            }
        );
    });
}
#[cfg(CONFIG_SND_PROC_FS)]
static mut snd_timer_proc_entry: *mut snd_info_entry = null_mut();
#[link_section = ".init.text"]
unsafe fn snd_timer_proc_init() {
    #[cfg(CONFIG_SND_PROC_FS)]
    {
        let mut entry = snd_info_create_module_entry(
            rust_timer_this_module(),
            c"timers".as_ptr().cast::<c_char>(),
            null_mut(),
        );
        if !entry.is_null() {
            rust_timer_info_set_read(entry, Some(snd_timer_proc_read));
            if snd_info_register(entry) < 0 {
                snd_info_free_entry(entry);
                entry = null_mut();
            }
        }
        snd_timer_proc_entry = entry;
    }
}
#[link_section = ".exit.text"]
unsafe fn snd_timer_proc_done() {
    #[cfg(CONFIG_SND_PROC_FS)]
    snd_info_free_entry(snd_timer_proc_entry);
}
/* User queues and ABI. */
const TREAD_FORMAT_NONE: c_int = timer_tread_format_TREAD_FORMAT_NONE as _;
const TREAD_FORMAT_TIME64: c_int = timer_tread_format_TREAD_FORMAT_TIME64 as _;
const TREAD_FORMAT_TIME32: c_int = timer_tread_format_TREAD_FORMAT_TIME32 as _;
#[inline]
fn next_user_queue_index(index: c_int, size: c_int) -> c_int {
    // Open initializes size to 128; PARAMS only replaces it with 32..=1024,
    // and TREAD preserves it. Reallocation and PARAMS reset both indices to
    // zero under qlock; every advance also holds qlock. Thus index is in
    // 0..size, and index + 1 is at most 1024. This is the original remainder
    // operation without introducing Rust-only division panic paths.
    let next = index + 1;
    if next == size {
        0
    } else {
        next
    }
}
unsafe fn wake_user(tu: *mut snd_timer_user) {
    snd_kill_fasync((*tu).fasync, SIGIO as _, POLL_IN as _);
    rust_timer_wake_up(addr_of_mut!((*tu).qchange_sleep));
}
unsafe extern "C" fn snd_timer_user_interrupt(
    ti: *mut snd_timer_instance,
    resolution: c_ulong,
    ticks: c_ulong,
) {
    let tu: *mut snd_timer_user = (*ti).callback_data.cast();
    let _s = SpinGuard::new(addr_of_mut!((*tu).qlock));
    if (*tu).qused > 0 {
        let prev = if (*tu).qtail == 0 {
            (*tu).queue_size - 1
        } else {
            (*tu).qtail - 1
        };
        let r = (*tu).queue.add(prev as usize);
        if (*r).resolution as c_ulong == resolution {
            (*r).ticks = (*r).ticks.wrapping_add(ticks as _);
            wake_user(tu);
            return;
        }
    }
    if (*tu).qused >= (*tu).queue_size {
        (*tu).overrun = (*tu).overrun.wrapping_add(1);
    } else {
        let r = (*tu).queue.add((*tu).qtail as usize);
        (*tu).qtail = next_user_queue_index((*tu).qtail, (*tu).queue_size);
        (*r).resolution = resolution as _;
        (*r).ticks = ticks as _;
        (*tu).qused += 1;
    }
    wake_user(tu);
}
unsafe fn snd_timer_user_append_to_tqueue(
    tu: *mut snd_timer_user,
    tread: *const snd_timer_tread64,
) {
    if (*tu).qused >= (*tu).queue_size {
        (*tu).overrun = (*tu).overrun.wrapping_add(1);
    } else {
        *(*tu).tqueue.add((*tu).qtail as usize) = *tread;
        (*tu).qtail = next_user_queue_index((*tu).qtail, (*tu).queue_size);
        (*tu).qused += 1;
    }
}
unsafe extern "C" fn snd_timer_user_ccallback(
    ti: *mut snd_timer_instance,
    event: c_int,
    stamp: *mut timespec64,
    resolution: c_ulong,
) {
    let tu: *mut snd_timer_user = (*ti).callback_data.cast();
    if event >= SNDRV_TIMER_EVENT_START as _ && event <= SNDRV_TIMER_EVENT_PAUSE as _ {
        (*tu).tstamp = *stamp;
    }
    if (*tu).filter & (1u32 << event) == 0 || (*tu).tread == 0 {
        return;
    }
    let r = snd_timer_tread64 {
        event,
        tstamp_sec: (*stamp).tv_sec,
        tstamp_nsec: (*stamp).tv_nsec as _,
        val: resolution as _,
        ..zeroed()
    };
    {
        let _s = SpinGuard::irqsave(addr_of_mut!((*tu).qlock));
        snd_timer_user_append_to_tqueue(tu, &raw const r);
    }
    wake_user(tu);
}
unsafe extern "C" fn snd_timer_user_disconnect(ti: *mut snd_timer_instance) {
    let tu: *mut snd_timer_user = (*ti).callback_data.cast();
    (*tu).disconnected = true;
    rust_timer_wake_up(addr_of_mut!((*tu).qchange_sleep));
}
unsafe extern "C" fn snd_timer_user_tinterrupt(
    ti: *mut snd_timer_instance,
    resolution: c_ulong,
    ticks: c_ulong,
) {
    let tu: *mut snd_timer_user = (*ti).callback_data.cast();
    let mut r: snd_timer_tread64 = zeroed();
    let mut stamp: timespec64 = zeroed();
    let mut append = 0;
    {
        let _s = SpinGuard::new(addr_of_mut!((*tu).qlock));
        if (*tu).filter & ((1 << SNDRV_TIMER_EVENT_RESOLUTION) | (1 << SNDRV_TIMER_EVENT_TICK)) == 0
        {
            return;
        }
        if (*tu).last_resolution != resolution || ticks > 0 {
            timestamp(&raw mut stamp);
        }
        if (*tu).filter & (1 << SNDRV_TIMER_EVENT_RESOLUTION) != 0
            && (*tu).last_resolution != resolution
        {
            r.event = SNDRV_TIMER_EVENT_RESOLUTION as _;
            r.tstamp_sec = stamp.tv_sec;
            r.tstamp_nsec = stamp.tv_nsec as _;
            r.val = resolution as _;
            snd_timer_user_append_to_tqueue(tu, &raw const r);
            (*tu).last_resolution = resolution;
            append += 1;
        }
        if (*tu).filter & (1 << SNDRV_TIMER_EVENT_TICK) != 0 && ticks != 0 {
            let prev = if (*tu).qtail == 0 {
                (*tu).queue_size - 1
            } else {
                (*tu).qtail - 1
            };
            let previous = (*tu).tqueue.add(prev as usize);
            if (*tu).qused > 0 && (*previous).event == SNDRV_TIMER_EVENT_TICK as _ {
                (*previous).tstamp_sec = stamp.tv_sec;
                (*previous).tstamp_nsec = stamp.tv_nsec as _;
                (*previous).val = (*previous).val.wrapping_add(ticks as _);
                append += 1;
            } else {
                r.event = SNDRV_TIMER_EVENT_TICK as _;
                r.tstamp_sec = stamp.tv_sec;
                r.tstamp_nsec = stamp.tv_nsec as _;
                r.val = ticks as _;
                snd_timer_user_append_to_tqueue(tu, &raw const r);
                append += 1;
            }
        }
    }
    if append != 0 {
        wake_user(tu);
    }
}
unsafe fn realloc_user_queue(tu: *mut snd_timer_user, size: c_int) -> c_int {
    let mut queue: *mut snd_timer_read = null_mut();
    let mut tqueue: *mut snd_timer_tread64 = null_mut();
    if (*tu).tread != 0 {
        tqueue = rust_timer_zalloc_array(size as usize, size_of::<snd_timer_tread64>()).cast();
        if tqueue.is_null() {
            return err!(ENOMEM);
        }
    } else {
        queue = rust_timer_zalloc_array(size as usize, size_of::<snd_timer_read>()).cast();
        if queue.is_null() {
            return err!(ENOMEM);
        }
    }
    let _s = SpinGuard::irq(addr_of_mut!((*tu).qlock));
    kfree((*tu).queue.cast());
    kfree((*tu).tqueue.cast());
    (*tu).queue_size = size;
    (*tu).queue = queue;
    (*tu).tqueue = tqueue;
    (*tu).qhead = 0;
    (*tu).qtail = 0;
    (*tu).qused = 0;
    0
}
unsafe extern "C" fn snd_timer_user_open(inode: *mut inode, file: *mut file) -> c_int {
    stream_open(inode, file);
    let tu: *mut snd_timer_user = rust_timer_zalloc(size_of::<snd_timer_user>()).cast();
    if tu.is_null() {
        return err!(ENOMEM);
    }
    rust_timer_spin_init(addr_of_mut!((*tu).qlock));
    rust_timer_wait_head_init(addr_of_mut!((*tu).qchange_sleep));
    rust_timer_mutex_init(addr_of_mut!((*tu).ioctl_lock));
    (*tu).ticks = 1;
    if realloc_user_queue(tu, 128) < 0 {
        kfree(tu.cast());
        return err!(ENOMEM);
    }
    rust_timer_file_set_private(file, tu.cast());
    0
}
unsafe extern "C" fn snd_timer_user_release(_inode: *mut inode, file: *mut file) -> c_int {
    let tu: *mut snd_timer_user = rust_timer_file_private(file).cast();
    if !tu.is_null() {
        rust_timer_file_set_private(file, null_mut());
        {
            let _m = MutexGuard::new(addr_of_mut!((*tu).ioctl_lock));
            if !(*tu).timeri.is_null() {
                snd_timer_close((*tu).timeri);
                snd_timer_instance_free((*tu).timeri);
            }
        }
        snd_fasync_free((*tu).fasync);
        kfree((*tu).queue.cast());
        kfree((*tu).tqueue.cast());
        kfree(tu.cast());
    }
    0
}
unsafe fn snd_timer_user_zero_id(id: *mut snd_timer_id) {
    (*id).dev_class = SNDRV_TIMER_CLASS_NONE as _;
    (*id).dev_sclass = SNDRV_TIMER_SCLASS_NONE as _;
    (*id).card = -1;
    (*id).device = -1;
    (*id).subdevice = -1;
}
unsafe fn snd_timer_user_copy_id(id: *mut snd_timer_id, t: *mut snd_timer) {
    (*id).dev_class = (*t).tmr_class;
    (*id).dev_sclass = SNDRV_TIMER_SCLASS_NONE as _;
    (*id).card = card_number(t);
    (*id).device = (*t).tmr_device;
    (*id).subdevice = (*t).tmr_subdevice;
}
unsafe fn get_next_device(id: *mut snd_timer_id) {
    let head = rust_timer_list();
    if (*id).dev_class < 0 {
        if list_empty(head) {
            snd_timer_user_zero_id(id);
        } else {
            snd_timer_user_copy_id(
                id,
                (*head)
                    .next
                    .cast::<u8>()
                    .sub(offset_of!(snd_timer, device_list))
                    .cast(),
            );
        }
        return;
    }
    if (*id).dev_class == SNDRV_TIMER_CLASS_GLOBAL as _ {
        (*id).device = if (*id).device < 0 {
            0
        } else {
            (*id).device.wrapping_add(1)
        };
        for_each!(t, head, snd_timer, device_list, {
            if (*t).tmr_class > SNDRV_TIMER_CLASS_GLOBAL as _ || (*t).tmr_device >= (*id).device {
                snd_timer_user_copy_id(id, t);
                return;
            }
        });
    } else if (*id).dev_class == SNDRV_TIMER_CLASS_CARD as _
        || (*id).dev_class == SNDRV_TIMER_CLASS_PCM as _
    {
        if (*id).card < 0 {
            (*id).card = 0;
        } else if (*id).device < 0 {
            (*id).device = 0;
        } else if (*id).subdevice < 0 {
            (*id).subdevice = 0;
        } else if (*id).subdevice < c_int::MAX {
            (*id).subdevice += 1;
        }
        for_each!(t, head, snd_timer, device_list, {
            if (*t).tmr_class > (*id).dev_class {
                snd_timer_user_copy_id(id, t);
                return;
            }
            if (*t).tmr_class < (*id).dev_class {
                continue;
            }
            if card_number(t) > (*id).card {
                snd_timer_user_copy_id(id, t);
                return;
            }
            if card_number(t) < (*id).card {
                continue;
            }
            if (*t).tmr_device > (*id).device {
                snd_timer_user_copy_id(id, t);
                return;
            }
            if (*t).tmr_device < (*id).device {
                continue;
            }
            if (*t).tmr_subdevice >= (*id).subdevice {
                snd_timer_user_copy_id(id, t);
                return;
            }
        });
    }
    snd_timer_user_zero_id(id);
}
unsafe fn snd_timer_user_next_device(user: *mut snd_timer_id) -> c_int {
    let mut id: snd_timer_id = zeroed();
    if copy_from(&raw mut id, user) {
        return err!(EFAULT);
    }
    {
        let _m = MutexGuard::new(rust_timer_register_mutex());
        get_next_device(&raw mut id);
    }
    if copy_to(user, &raw const id) {
        err!(EFAULT)
    } else {
        0
    }
}
unsafe fn snd_timer_user_ginfo(user: *mut snd_timer_ginfo) -> c_int {
    let info: *mut snd_timer_ginfo = memdup_user(user.cast(), size_of::<snd_timer_ginfo>()).cast();
    if is_err(info) {
        return info as isize as c_int;
    }
    let _alloc = Allocation(info);
    let mut tid = (*info).tid;
    core::ptr::write_bytes(info, 0, 1);
    (*info).tid = tid;
    {
        let _m = MutexGuard::new(rust_timer_register_mutex());
        let t = snd_timer_find(&raw mut tid);
        if t.is_null() {
            return err!(ENODEV);
        }
        (*info).card = card_number(t);
        if (*t).hw.flags & SNDRV_TIMER_HW_SLAVE != 0 {
            (*info).flags |= SNDRV_TIMER_FLG_SLAVE;
        }
        strscpy(
            addr_of_mut!((*info).id).cast(),
            addr_of_mut!((*t).id).cast(),
            64,
        );
        strscpy(
            addr_of_mut!((*info).name).cast(),
            addr_of_mut!((*t).name).cast(),
            80,
        );
        {
            let _s = SpinGuard::irq(addr_of_mut!((*t).lock));
            (*info).resolution = snd_timer_hw_resolution(t);
        }
        if (*t).hw.resolution_min > 0 {
            (*info).resolution_min = (*t).hw.resolution_min;
            (*info).resolution_max = (*t).hw.resolution_max;
        }
        let head = addr_of_mut!((*t).open_list_head);
        let mut node = (*head).next;
        while node != head {
            (*info).clients = (*info).clients.wrapping_add(1);
            node = (*node).next;
        }
    }
    if copy_to(user, info) {
        err!(EFAULT)
    } else {
        0
    }
}
unsafe fn timer_set_gparams(params: *mut snd_timer_gparams) -> c_int {
    let _m = MutexGuard::new(rust_timer_register_mutex());
    let t = snd_timer_find(addr_of_mut!((*params).tid));
    if t.is_null() {
        return err!(ENODEV);
    }
    if !list_empty(addr_of_mut!((*t).open_list_head)) {
        return err!(EBUSY);
    }
    if let Some(f) = (*t).hw.set_period {
        f(t, (*params).period_num, (*params).period_den)
    } else {
        err!(ENOSYS)
    }
}
unsafe fn snd_timer_user_gparams(user: *mut snd_timer_gparams) -> c_int {
    let mut params: snd_timer_gparams = zeroed();
    if copy_from(&raw mut params, user) {
        return err!(EFAULT);
    }
    timer_set_gparams(&raw mut params)
}
unsafe fn snd_timer_user_gstatus(user: *mut snd_timer_gstatus) -> c_int {
    let mut status: snd_timer_gstatus = zeroed();
    if copy_from(&raw mut status, user) {
        return err!(EFAULT);
    }
    let mut tid = status.tid;
    status = zeroed();
    status.tid = tid;
    {
        let _m = MutexGuard::new(rust_timer_register_mutex());
        let t = snd_timer_find(&raw mut tid);
        if t.is_null() {
            return err!(ENODEV);
        }
        let _s = SpinGuard::irq(addr_of_mut!((*t).lock));
        status.resolution = snd_timer_hw_resolution(t);
        if let Some(f) = (*t).hw.precise_resolution {
            f(
                t,
                &raw mut status.resolution_num,
                &raw mut status.resolution_den,
            );
        } else {
            status.resolution_num = status.resolution;
            status.resolution_den = 1000000000;
        }
    }
    if copy_to(user, &raw const status) {
        err!(EFAULT)
    } else {
        0
    }
}
unsafe fn snd_timer_user_tselect(file: *mut file, user: *mut snd_timer_select) -> c_int {
    let tu: *mut snd_timer_user = rust_timer_file_private(file).cast();
    if !(*tu).timeri.is_null() {
        snd_timer_close((*tu).timeri);
        snd_timer_instance_free((*tu).timeri);
        (*tu).timeri = null_mut();
    }
    let mut select: snd_timer_select = zeroed();
    if copy_from(&raw mut select, user) {
        return err!(EFAULT);
    }
    let mut owner: [c_char; 32] = [0; 32];
    sprintf(
        owner.as_mut_ptr(),
        c"application %i".as_ptr().cast(),
        rust_timer_pid(),
    );
    if select.id.dev_class != SNDRV_TIMER_CLASS_SLAVE as _ {
        select.id.dev_sclass = SNDRV_TIMER_SCLASS_APPLICATION as _;
    }
    let ti = snd_timer_instance_new(owner.as_ptr());
    (*tu).timeri = ti;
    if ti.is_null() {
        return err!(ENOMEM);
    }
    (*ti).flags |= SNDRV_TIMER_IFLG_FAST;
    (*ti).callback = if (*tu).tread != 0 {
        Some(snd_timer_user_tinterrupt)
    } else {
        Some(snd_timer_user_interrupt)
    };
    (*ti).ccallback = Some(snd_timer_user_ccallback);
    (*ti).callback_data = tu.cast();
    (*ti).disconnect = Some(snd_timer_user_disconnect);
    let err = snd_timer_open(ti, &raw mut select.id, rust_timer_pid() as _);
    if err < 0 {
        snd_timer_instance_free(ti);
        (*tu).timeri = null_mut();
    }
    err
}
unsafe fn snd_timer_user_info(file: *mut file, user: *mut snd_timer_info) -> c_int {
    let tu: *mut snd_timer_user = rust_timer_file_private(file).cast();
    if (*tu).timeri.is_null() {
        return err!(EBADFD);
    }
    let hold = TimerRef(snd_timeri_timer_get((*tu).timeri));
    let t = hold.0;
    if t.is_null() {
        return err!(EBADFD);
    }
    let info: *mut snd_timer_info = rust_timer_zalloc(size_of::<snd_timer_info>()).cast();
    if info.is_null() {
        return err!(ENOMEM);
    }
    let _alloc = Allocation(info);
    (*info).card = card_number(t);
    if (*t).hw.flags & SNDRV_TIMER_HW_SLAVE != 0 {
        (*info).flags |= SNDRV_TIMER_FLG_SLAVE;
    }
    strscpy(
        addr_of_mut!((*info).id).cast(),
        addr_of_mut!((*t).id).cast(),
        64,
    );
    strscpy(
        addr_of_mut!((*info).name).cast(),
        addr_of_mut!((*t).name).cast(),
        80,
    );
    {
        let _s = SpinGuard::irq(addr_of_mut!((*t).lock));
        (*info).resolution = snd_timer_hw_resolution(t);
    }
    if copy_to(user, info) {
        err!(EFAULT)
    } else {
        0
    }
}
unsafe fn snd_timer_user_params(file: *mut file, user: *mut snd_timer_params) -> c_int {
    let tu: *mut snd_timer_user = rust_timer_file_private(file).cast();
    if (*tu).timeri.is_null() {
        return err!(EBADFD);
    }
    let hold = TimerRef(snd_timeri_timer_get((*tu).timeri));
    let t = hold.0;
    if t.is_null() {
        return err!(EBADFD);
    }
    let mut params: snd_timer_params = zeroed();
    if copy_from(&raw mut params, user) {
        return err!(EFAULT);
    }
    let ret = (|| {
        if (*t).hw.flags & SNDRV_TIMER_HW_SLAVE == 0 {
            if params.ticks < 1
                || (snd_timer_resolution((*tu).timeri) as u64).wrapping_mul(params.ticks as u64)
                    < 1000000
            {
                return err!(EINVAL);
            }
        }
        if params.queue_size > 0 && (params.queue_size < 32 || params.queue_size > 1024) {
            return err!(EINVAL);
        }
        let filter = (1 << SNDRV_TIMER_EVENT_RESOLUTION)
            | (1 << SNDRV_TIMER_EVENT_TICK)
            | (1 << SNDRV_TIMER_EVENT_START)
            | (1 << SNDRV_TIMER_EVENT_STOP)
            | (1 << SNDRV_TIMER_EVENT_CONTINUE)
            | (1 << SNDRV_TIMER_EVENT_PAUSE)
            | (1 << SNDRV_TIMER_EVENT_SUSPEND)
            | (1 << SNDRV_TIMER_EVENT_RESUME)
            | (1 << SNDRV_TIMER_EVENT_MSTART)
            | (1 << SNDRV_TIMER_EVENT_MSTOP)
            | (1 << SNDRV_TIMER_EVENT_MCONTINUE)
            | (1 << SNDRV_TIMER_EVENT_MPAUSE)
            | (1 << SNDRV_TIMER_EVENT_MSUSPEND)
            | (1 << SNDRV_TIMER_EVENT_MRESUME);
        if params.filter & !filter != 0 {
            return err!(EINVAL);
        }
        snd_timer_stop((*tu).timeri);
        {
            let _s = SpinGuard::irq(addr_of_mut!((*t).lock));
            (*(*tu).timeri).flags &= !(SNDRV_TIMER_IFLG_AUTO
                | SNDRV_TIMER_IFLG_EXCLUSIVE
                | SNDRV_TIMER_IFLG_EARLY_EVENT);
            if params.flags & SNDRV_TIMER_PSFLG_AUTO != 0 {
                (*(*tu).timeri).flags |= SNDRV_TIMER_IFLG_AUTO;
            }
            if params.flags & SNDRV_TIMER_PSFLG_EXCLUSIVE != 0 {
                (*(*tu).timeri).flags |= SNDRV_TIMER_IFLG_EXCLUSIVE;
            }
            if params.flags & SNDRV_TIMER_PSFLG_EARLY_EVENT != 0 {
                (*(*tu).timeri).flags |= SNDRV_TIMER_IFLG_EARLY_EVENT;
            }
        }
        if params.queue_size > 0 && (*tu).queue_size as u32 != params.queue_size {
            let ret = realloc_user_queue(tu, params.queue_size as _);
            if ret < 0 {
                return ret;
            }
        }
        {
            let _s = SpinGuard::irq(addr_of_mut!((*tu).qlock));
            (*tu).qhead = 0;
            (*tu).qtail = 0;
            (*tu).qused = 0;
            if (*(*tu).timeri).flags & SNDRV_TIMER_IFLG_EARLY_EVENT != 0 {
                if (*tu).tread != 0 {
                    let r = snd_timer_tread64 {
                        event: SNDRV_TIMER_EVENT_EARLY as _,
                        ..zeroed()
                    };
                    snd_timer_user_append_to_tqueue(tu, &raw const r);
                } else {
                    (*(*tu).queue).resolution = 0;
                    (*(*tu).queue).ticks = 0;
                    (*tu).qused += 1;
                    (*tu).qtail += 1;
                }
            }
            (*tu).filter = params.filter;
            (*tu).ticks = params.ticks as _;
        }
        0
    })();
    if copy_to(user, &raw const params) {
        err!(EFAULT)
    } else {
        ret
    }
}
unsafe fn snd_timer_user_status32(file: *mut file, user: *mut snd_timer_status32) -> c_int {
    let tu: *mut snd_timer_user = rust_timer_file_private(file).cast();
    if (*tu).timeri.is_null() {
        return err!(EBADFD);
    }
    let mut status: snd_timer_status32 = zeroed();
    status.tstamp_sec = (*tu).tstamp.tv_sec as _;
    status.tstamp_nsec = (*tu).tstamp.tv_nsec as _;
    status.resolution = snd_timer_resolution((*tu).timeri) as _;
    status.lost = (*(*tu).timeri).lost as _;
    status.overrun = (*tu).overrun as _;
    {
        let _s = SpinGuard::irq(addr_of_mut!((*tu).qlock));
        status.queue = (*tu).qused as _;
    }
    if copy_to(user, &raw const status) {
        err!(EFAULT)
    } else {
        0
    }
}
unsafe fn snd_timer_user_status64(file: *mut file, user: *mut snd_timer_status64) -> c_int {
    let tu: *mut snd_timer_user = rust_timer_file_private(file).cast();
    if (*tu).timeri.is_null() {
        return err!(EBADFD);
    }
    let mut status: snd_timer_status64 = zeroed();
    status.tstamp_sec = (*tu).tstamp.tv_sec;
    status.tstamp_nsec = (*tu).tstamp.tv_nsec as _;
    status.resolution = snd_timer_resolution((*tu).timeri) as _;
    status.lost = (*(*tu).timeri).lost as _;
    status.overrun = (*tu).overrun as _;
    {
        let _s = SpinGuard::irq(addr_of_mut!((*tu).qlock));
        status.queue = (*tu).qused as _;
    }
    if copy_to(user, &raw const status) {
        err!(EFAULT)
    } else {
        0
    }
}
unsafe fn snd_timer_user_start(file: *mut file) -> c_int {
    let tu: *mut snd_timer_user = rust_timer_file_private(file).cast();
    if (*tu).timeri.is_null() {
        return err!(EBADFD);
    }
    snd_timer_stop((*tu).timeri);
    (*(*tu).timeri).lost = 0;
    (*tu).last_resolution = 0;
    snd_timer_start((*tu).timeri, (*tu).ticks as _).min(0)
}
unsafe fn snd_timer_user_stop(file: *mut file) -> c_int {
    let tu: *mut snd_timer_user = rust_timer_file_private(file).cast();
    if (*tu).timeri.is_null() {
        return err!(EBADFD);
    }
    snd_timer_stop((*tu).timeri).min(0)
}
unsafe fn snd_timer_user_continue(file: *mut file) -> c_int {
    let tu: *mut snd_timer_user = rust_timer_file_private(file).cast();
    if (*tu).timeri.is_null() {
        return err!(EBADFD);
    }
    if (*(*tu).timeri).flags & SNDRV_TIMER_IFLG_PAUSED == 0 {
        return snd_timer_user_start(file);
    }
    (*(*tu).timeri).lost = 0;
    snd_timer_continue((*tu).timeri).min(0)
}
unsafe fn snd_timer_user_pause(file: *mut file) -> c_int {
    let tu: *mut snd_timer_user = rust_timer_file_private(file).cast();
    if (*tu).timeri.is_null() {
        return err!(EBADFD);
    }
    snd_timer_pause((*tu).timeri).min(0)
}
unsafe fn snd_timer_user_tread(
    arg: *mut c_void,
    tu: *mut snd_timer_user,
    cmd: c_uint,
    compat: bool,
) -> c_int {
    if !(*tu).timeri.is_null() {
        return err!(EBUSY);
    }
    let mut xarg = 0;
    if rust_timer_get_int(arg.cast(), &raw mut xarg) != 0 {
        return err!(EFAULT);
    }
    let old = (*tu).tread;
    (*tu).tread = if xarg == 0 {
        TREAD_FORMAT_NONE
    } else if cmd == RUST_SNDRV_TIMER_IOCTL_TREAD64 || (cfg!(CONFIG_64BIT) && !compat) {
        TREAD_FORMAT_TIME64
    } else {
        TREAD_FORMAT_TIME32
    };
    if (*tu).tread != old && realloc_user_queue(tu, (*tu).queue_size) < 0 {
        (*tu).tread = old;
        return err!(ENOMEM);
    }
    0
}
#[cfg(CONFIG_SND_UTIMER)]
unsafe fn snd_utimer_put_id(ut: *mut snd_utimer) {
    let id = (*ut).id as c_int;
    rust_timer_bug_on(id < 0 || id >= SNDRV_UTIMERS_MAX_COUNT as _);
    rust_timer_put_id(id as _);
}
#[cfg(CONFIG_SND_UTIMER)]
unsafe fn snd_utimer_free(ut: *mut snd_utimer) {
    snd_timer_free((*ut).timer);
    snd_utimer_put_id(ut);
    kfree((*ut).name.cast());
    kfree(ut.cast());
}
#[cfg(CONFIG_SND_UTIMER)]
unsafe extern "C" fn snd_utimer_release(_inode: *mut inode, file: *mut file) -> c_int {
    snd_utimer_free(rust_timer_file_private(file).cast());
    0
}
#[cfg(CONFIG_SND_UTIMER)]
unsafe extern "C" fn snd_utimer_ioctl(file: *mut file, cmd: c_uint, _arg: c_ulong) -> c_long {
    if cmd == RUST_SNDRV_TIMER_IOCTL_TRIGGER {
        let ut: *mut snd_utimer = rust_timer_file_private(file).cast();
        let t = (*ut).timer;
        snd_timer_interrupt(t, (*t).sticks);
        return 0;
    }
    err!(ENOTTY) as _
}
#[cfg(CONFIG_SND_UTIMER)]
static mut snd_utimer_fops: file_operations = file_operations {
    llseek: Some(noop_llseek),
    release: Some(snd_utimer_release),
    unlocked_ioctl: Some(snd_utimer_ioctl),
    ..unsafe { zeroed() }
};
// The virtual hardware's four callbacks deliberately do nothing in the original.
#[cfg(CONFIG_SND_UTIMER)]
unsafe extern "C" fn snd_utimer_start(_t: *mut snd_timer) -> c_int {
    0
}
#[cfg(CONFIG_SND_UTIMER)]
unsafe extern "C" fn snd_utimer_stop(_t: *mut snd_timer) -> c_int {
    0
}
#[cfg(CONFIG_SND_UTIMER)]
unsafe extern "C" fn snd_utimer_open(_t: *mut snd_timer) -> c_int {
    0
}
#[cfg(CONFIG_SND_UTIMER)]
unsafe extern "C" fn snd_utimer_close(_t: *mut snd_timer) -> c_int {
    0
}
#[cfg(CONFIG_SND_UTIMER)]
unsafe fn snd_utimer_create(info: *mut snd_timer_uinfo, out: *mut *mut snd_utimer) -> c_int {
    if info.is_null() || (*info).resolution == 0 {
        return err!(EINVAL);
    }
    let ut: *mut snd_utimer = rust_timer_zalloc(size_of::<snd_utimer>()).cast();
    if ut.is_null() {
        return err!(ENOMEM);
    }
    let id = rust_timer_take_id((SNDRV_UTIMERS_MAX_COUNT - 1) as _);
    if id < 0 {
        kfree(ut.cast());
        return id;
    }
    (*ut).id = id as _;
    (*ut).name = kasprintf(
        RUST_TIMER_GFP_KERNEL as _,
        c"snd-utimer%d".as_ptr().cast::<c_char>(),
        id,
    );
    if (*ut).name.is_null() {
        snd_utimer_put_id(ut);
        kfree(ut.cast());
        return err!(ENOMEM);
    }
    let mut tid = snd_timer_id {
        dev_sclass: SNDRV_TIMER_SCLASS_APPLICATION as _,
        dev_class: SNDRV_TIMER_CLASS_GLOBAL as _,
        card: -1,
        device: SNDRV_TIMER_GLOBAL_UDRIVEN as _,
        subdevice: id,
    };
    let mut t = null_mut();
    let mut err = snd_timer_new(null_mut(), (*ut).name, &raw mut tid, &raw mut t);
    if err < 0 {
        _printk(
            c"\x013Can't create userspace-driven timer\n"
                .as_ptr()
                .cast::<c_char>(),
        );
    } else {
        (*t).module = rust_timer_this_module();
        (*t).hw = snd_timer_hardware {
            flags: SNDRV_TIMER_HW_AUTO | SNDRV_TIMER_HW_WORK,
            open: Some(snd_utimer_open),
            close: Some(snd_utimer_close),
            start: Some(snd_utimer_start),
            stop: Some(snd_utimer_stop),
            resolution: (*info).resolution as _,
            ticks: 1,
            ..zeroed()
        };
        (*t).max_instances = MAX_SLAVE_INSTANCES;
        (*ut).timer = t;
        err = snd_timer_global_register(t);
        if err < 0 {
            _printk(
                c"\x013Can't register a userspace-driven timer\n"
                    .as_ptr()
                    .cast::<c_char>(),
            );
            snd_timer_free(t);
        } else {
            *out = ut;
            return 0;
        }
    }
    kfree((*ut).name.cast());
    snd_utimer_put_id(ut);
    kfree(ut.cast());
    err
}
#[cfg(CONFIG_SND_UTIMER)]
unsafe fn snd_utimer_ioctl_create(user: *mut snd_timer_uinfo) -> c_int {
    let info: *mut snd_timer_uinfo = memdup_user(user.cast(), size_of::<snd_timer_uinfo>()).cast();
    if is_err(info) {
        return info as isize as c_int;
    }
    let _alloc = Allocation(info);
    let mut ut = null_mut();
    let err = snd_utimer_create(info, &raw mut ut);
    if err < 0 {
        return err;
    }
    (*info).id = (*ut).id as _;
    let fd = anon_inode_getfd(
        (*ut).name,
        &raw const snd_utimer_fops,
        ut.cast(),
        (O_RDWR | O_CLOEXEC) as _,
    );
    if fd < 0 {
        snd_utimer_free(ut);
        return fd;
    }
    (*info).fd = fd;
    // As in C, anon_inode_getfd has published this fd: a failed copy cannot close it.
    if copy_to(user, info) {
        err!(EFAULT)
    } else {
        0
    }
}
#[cfg(not(CONFIG_SND_UTIMER))]
unsafe fn snd_utimer_ioctl_create(_user: *mut snd_timer_uinfo) -> c_int {
    err!(ENOTTY)
}
unsafe fn __snd_timer_user_ioctl(
    file: *mut file,
    cmd: c_uint,
    arg: c_ulong,
    compat: bool,
) -> c_long {
    let tu: *mut snd_timer_user = rust_timer_file_private(file).cast();
    let p = arg as *mut c_void;
    (match cmd {
        RUST_SNDRV_TIMER_IOCTL_PVERSION => {
            if rust_timer_put_int(p.cast(), RUST_SNDRV_TIMER_VERSION as _) != 0 {
                err!(EFAULT)
            } else {
                0
            }
        }
        RUST_SNDRV_TIMER_IOCTL_NEXT_DEVICE => snd_timer_user_next_device(p.cast()),
        RUST_SNDRV_TIMER_IOCTL_TREAD_OLD | RUST_SNDRV_TIMER_IOCTL_TREAD64 => {
            snd_timer_user_tread(p, tu, cmd, compat)
        }
        RUST_SNDRV_TIMER_IOCTL_GINFO => snd_timer_user_ginfo(p.cast()),
        RUST_SNDRV_TIMER_IOCTL_GPARAMS => snd_timer_user_gparams(p.cast()),
        RUST_SNDRV_TIMER_IOCTL_GSTATUS => snd_timer_user_gstatus(p.cast()),
        RUST_SNDRV_TIMER_IOCTL_SELECT => snd_timer_user_tselect(file, p.cast()),
        RUST_SNDRV_TIMER_IOCTL_INFO => snd_timer_user_info(file, p.cast()),
        RUST_SNDRV_TIMER_IOCTL_PARAMS => snd_timer_user_params(file, p.cast()),
        RUST_SNDRV_TIMER_IOCTL_STATUS32 => snd_timer_user_status32(file, p.cast()),
        RUST_SNDRV_TIMER_IOCTL_STATUS64 => snd_timer_user_status64(file, p.cast()),
        RUST_SNDRV_TIMER_IOCTL_START | RUST_SNDRV_TIMER_IOCTL_START_OLD => {
            snd_timer_user_start(file)
        }
        RUST_SNDRV_TIMER_IOCTL_STOP | RUST_SNDRV_TIMER_IOCTL_STOP_OLD => snd_timer_user_stop(file),
        RUST_SNDRV_TIMER_IOCTL_CONTINUE | RUST_SNDRV_TIMER_IOCTL_CONTINUE_OLD => {
            snd_timer_user_continue(file)
        }
        RUST_SNDRV_TIMER_IOCTL_PAUSE | RUST_SNDRV_TIMER_IOCTL_PAUSE_OLD => {
            snd_timer_user_pause(file)
        }
        RUST_SNDRV_TIMER_IOCTL_CREATE => snd_utimer_ioctl_create(p.cast()),
        _ => err!(ENOTTY),
    }) as _
}
unsafe extern "C" fn snd_timer_user_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    let tu: *mut snd_timer_user = rust_timer_file_private(file).cast();
    let _m = MutexGuard::new(addr_of_mut!((*tu).ioctl_lock));
    __snd_timer_user_ioctl(file, cmd, arg, false)
}
unsafe extern "C" fn snd_timer_user_fasync(fd: c_int, file: *mut file, on: c_int) -> c_int {
    let tu: *mut snd_timer_user = rust_timer_file_private(file).cast();
    snd_fasync_helper(fd, file, on, addr_of_mut!((*tu).fasync))
}
unsafe extern "C" fn snd_timer_user_read(
    file: *mut file,
    mut buffer: *mut c_char,
    count: usize,
    _offset: *mut loff_t,
) -> isize {
    let tu: *mut snd_timer_user = rust_timer_file_private(file).cast();
    let unit = match (*tu).tread {
        TREAD_FORMAT_TIME64 => size_of::<snd_timer_tread64>(),
        TREAD_FORMAT_TIME32 => size_of::<snd_timer_tread32>(),
        TREAD_FORMAT_NONE => size_of::<snd_timer_read>(),
        _ => {
            rust_timer_warn_corrupt();
            return err!(ENOTSUPP) as _;
        }
    } as c_long;
    let mut result: c_long = 0;
    rust_timer_mutex_lock(addr_of_mut!((*tu).ioctl_lock));
    rust_timer_spin_lock_irq(addr_of_mut!((*tu).qlock));
    let error = (|| {
        while (count as c_long).wrapping_sub(result) >= unit {
            while (*tu).qused == 0 {
                if rust_timer_file_flags(file) & O_NONBLOCK != 0 || result > 0 {
                    return err!(EAGAIN);
                }
                let mut wait: wait_queue_entry_t = zeroed();
                rust_timer_current_interruptible();
                rust_timer_wait_init(&raw mut wait);
                add_wait_queue(addr_of_mut!((*tu).qchange_sleep), &raw mut wait);
                rust_timer_spin_unlock_irq(addr_of_mut!((*tu).qlock));
                rust_timer_mutex_unlock(addr_of_mut!((*tu).ioctl_lock));
                schedule();
                rust_timer_mutex_lock(addr_of_mut!((*tu).ioctl_lock));
                rust_timer_spin_lock_irq(addr_of_mut!((*tu).qlock));
                remove_wait_queue(addr_of_mut!((*tu).qchange_sleep), &raw mut wait);
                if (*tu).disconnected {
                    return err!(ENODEV);
                }
                if rust_timer_signal_pending() {
                    return err!(ERESTARTSYS);
                }
            }
            let qhead = (*tu).qhead as usize;
            (*tu).qhead = next_user_queue_index((*tu).qhead, (*tu).queue_size);
            (*tu).qused -= 1;
            rust_timer_spin_unlock_irq(addr_of_mut!((*tu).qlock));
            // Only form a timestamp queue pointer for a timestamp format. The C
            // expression forms a null-derived pointer for NONE but never reads it.
            let ret = match (*tu).tread {
                TREAD_FORMAT_TIME64 => copy_to(buffer.cast(), (*tu).tqueue.add(qhead)),
                TREAD_FORMAT_TIME32 => {
                    let tread = (*tu).tqueue.add(qhead);
                    let r = snd_timer_tread32 {
                        event: (*tread).event,
                        tstamp_sec: (*tread).tstamp_sec as _,
                        tstamp_nsec: (*tread).tstamp_nsec as _,
                        val: (*tread).val,
                    };
                    copy_to(buffer.cast(), &raw const r)
                }
                TREAD_FORMAT_NONE => copy_to(buffer.cast(), (*tu).queue.add(qhead)),
                _ => {
                    rust_timer_spin_lock_irq(addr_of_mut!((*tu).qlock));
                    return err!(ENOTSUPP);
                }
            };
            rust_timer_spin_lock_irq(addr_of_mut!((*tu).qlock));
            if ret {
                return err!(EFAULT);
            }
            result += unit;
            buffer = buffer.wrapping_add(unit as usize);
        }
        0
    })();
    rust_timer_spin_unlock_irq(addr_of_mut!((*tu).qlock));
    rust_timer_mutex_unlock(addr_of_mut!((*tu).ioctl_lock));
    if result > 0 {
        result as _
    } else {
        error as _
    }
}
unsafe extern "C" fn snd_timer_user_poll(file: *mut file, wait: *mut poll_table) -> __poll_t {
    let tu: *mut snd_timer_user = rust_timer_file_private(file).cast();
    rust_timer_poll_wait(file, addr_of_mut!((*tu).qchange_sleep), wait);
    let _s = SpinGuard::irq(addr_of_mut!((*tu).qlock));
    let mut mask = 0;
    if (*tu).qused != 0 {
        mask |= RUST_EPOLLIN | RUST_EPOLLRDNORM;
    }
    if (*tu).disconnected {
        mask |= RUST_EPOLLERR;
    }
    mask
}
#[cfg(CONFIG_COMPAT)]
unsafe fn snd_timer_user_gparams_compat(user: *mut snd_timer_gparams32) -> c_int {
    let mut params: snd_timer_gparams = zeroed();
    if copy_from(
        &raw mut params.tid,
        user.cast::<u8>()
            .wrapping_add(offset_of!(snd_timer_gparams32, tid))
            .cast(),
    ) || rust_timer_get_u32(
        user.cast::<u8>()
            .wrapping_add(offset_of!(snd_timer_gparams32, period_num))
            .cast(),
        &raw mut params.period_num,
    ) != 0
        || rust_timer_get_u32(
            user.cast::<u8>()
                .wrapping_add(offset_of!(snd_timer_gparams32, period_den))
                .cast(),
            &raw mut params.period_den,
        ) != 0
    {
        return err!(EFAULT);
    }
    timer_set_gparams(&raw mut params)
}
#[cfg(CONFIG_COMPAT)]
unsafe fn snd_timer_user_info_compat(file: *mut file, user: *mut snd_timer_info32) -> c_int {
    let tu: *mut snd_timer_user = rust_timer_file_private(file).cast();
    if (*tu).timeri.is_null() {
        return err!(EBADFD);
    }
    let hold = TimerRef(snd_timeri_timer_get((*tu).timeri));
    let t = hold.0;
    if t.is_null() {
        return err!(EBADFD);
    }
    let mut info: snd_timer_info32 = zeroed();
    info.card = card_number(t);
    if (*t).hw.flags & SNDRV_TIMER_HW_SLAVE != 0 {
        info.flags |= SNDRV_TIMER_FLG_SLAVE;
    }
    strscpy(
        info.id.as_mut_ptr().cast(),
        addr_of_mut!((*t).id).cast(),
        64,
    );
    strscpy(
        info.name.as_mut_ptr().cast(),
        addr_of_mut!((*t).name).cast(),
        80,
    );
    info.resolution = (*t).hw.resolution as _;
    if copy_to(user, &raw const info) {
        err!(EFAULT)
    } else {
        0
    }
}
#[cfg(CONFIG_COMPAT)]
unsafe fn __snd_timer_user_ioctl_compat(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    let p = rust_timer_compat_ptr(arg);
    match cmd {
        RUST_SNDRV_TIMER_IOCTL_PVERSION
        | RUST_SNDRV_TIMER_IOCTL_TREAD_OLD
        | RUST_SNDRV_TIMER_IOCTL_TREAD64
        | RUST_SNDRV_TIMER_IOCTL_GINFO
        | RUST_SNDRV_TIMER_IOCTL_GSTATUS
        | RUST_SNDRV_TIMER_IOCTL_SELECT
        | RUST_SNDRV_TIMER_IOCTL_PARAMS
        | RUST_SNDRV_TIMER_IOCTL_START
        | RUST_SNDRV_TIMER_IOCTL_START_OLD
        | RUST_SNDRV_TIMER_IOCTL_STOP
        | RUST_SNDRV_TIMER_IOCTL_STOP_OLD
        | RUST_SNDRV_TIMER_IOCTL_CONTINUE
        | RUST_SNDRV_TIMER_IOCTL_CONTINUE_OLD
        | RUST_SNDRV_TIMER_IOCTL_PAUSE
        | RUST_SNDRV_TIMER_IOCTL_PAUSE_OLD
        | RUST_SNDRV_TIMER_IOCTL_NEXT_DEVICE => {
            __snd_timer_user_ioctl(file, cmd, p as c_ulong, true)
        }
        RUST_SNDRV_TIMER_IOCTL_GPARAMS32 => snd_timer_user_gparams_compat(p.cast()) as _,
        RUST_SNDRV_TIMER_IOCTL_INFO32 => snd_timer_user_info_compat(file, p.cast()) as _,
        RUST_SNDRV_TIMER_IOCTL_STATUS_COMPAT32 => snd_timer_user_status32(file, p.cast()) as _,
        RUST_SNDRV_TIMER_IOCTL_STATUS_COMPAT64 => snd_timer_user_status64(file, p.cast()) as _,
        _ => err!(ENOIOCTLCMD) as _,
    }
}
#[cfg(CONFIG_COMPAT)]
unsafe extern "C" fn snd_timer_user_ioctl_compat(
    file: *mut file,
    cmd: c_uint,
    arg: c_ulong,
) -> c_long {
    let tu: *mut snd_timer_user = rust_timer_file_private(file).cast();
    let _m = MutexGuard::new(addr_of_mut!((*tu).ioctl_lock));
    __snd_timer_user_ioctl_compat(file, cmd, arg)
}
static mut snd_timer_f_ops: file_operations = file_operations {
    read: Some(snd_timer_user_read),
    open: Some(snd_timer_user_open),
    release: Some(snd_timer_user_release),
    poll: Some(snd_timer_user_poll),
    unlocked_ioctl: Some(snd_timer_user_ioctl),
    #[cfg(CONFIG_COMPAT)]
    compat_ioctl: Some(snd_timer_user_ioctl_compat),
    fasync: Some(snd_timer_user_fasync),
    ..unsafe { zeroed() }
};
unsafe fn snd_timer_free_all() {
    for_each!(t, rust_timer_list(), snd_timer, device_list, {
        snd_timer_free(t);
    });
}
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn rust_alsa_timer_init() -> c_int {
    let mut ret = snd_device_alloc(&raw mut timer_dev, null_mut());
    if ret < 0 {
        return ret;
    }
    dev_set_name(timer_dev, c"timer".as_ptr().cast::<c_char>());
    rust_timer_oss_register();
    ret = snd_timer_register_system();
    if ret < 0 {
        _printk(
            c"\x013ALSA: unable to register system timer (%i)\n"
                .as_ptr()
                .cast::<c_char>(),
            ret,
        );
        put_device(timer_dev);
        return ret;
    }
    snd_timer_f_ops.owner = rust_timer_this_module();
    ret = snd_register_device(
        SNDRV_DEVICE_TYPE_TIMER as _,
        null_mut(),
        0,
        &raw const snd_timer_f_ops,
        null_mut(),
        timer_dev,
    );
    if ret < 0 {
        _printk(
            c"\x013ALSA: unable to register timer device (%i)\n"
                .as_ptr()
                .cast::<c_char>(),
            ret,
        );
        snd_timer_free_all();
        put_device(timer_dev);
        return ret;
    }
    snd_timer_proc_init();
    0
}
#[no_mangle]
#[link_section = ".exit.text"]
pub unsafe extern "C" fn rust_alsa_timer_exit() {
    snd_unregister_device(timer_dev);
    snd_timer_free_all();
    put_device(timer_dev);
    snd_timer_proc_done();
    rust_timer_oss_unregister();
}
#[path = "../../rust/ffi_export.rs"]
mod ffi_export;
ffi_export::export_symbol!(snd_timer_instance_new, snd_timer_instance_new, "", "");
ffi_export::export_symbol!(snd_timer_instance_free, snd_timer_instance_free, "", "");
ffi_export::export_symbol!(snd_timeri_timer_get, snd_timeri_timer_get, "GPL", "");
ffi_export::export_symbol!(snd_timeri_timer_put, snd_timeri_timer_put, "GPL", "");
ffi_export::export_symbol!(snd_timer_open, snd_timer_open, "", "");
ffi_export::export_symbol!(snd_timer_close, snd_timer_close, "", "");
ffi_export::export_symbol!(snd_timer_resolution, snd_timer_resolution, "", "");
ffi_export::export_symbol!(snd_timer_start, snd_timer_start, "", "");
ffi_export::export_symbol!(snd_timer_stop, snd_timer_stop, "", "");
ffi_export::export_symbol!(snd_timer_continue, snd_timer_continue, "", "");
ffi_export::export_symbol!(snd_timer_pause, snd_timer_pause, "", "");
ffi_export::export_symbol!(snd_timer_interrupt, snd_timer_interrupt, "", "");
ffi_export::export_symbol!(snd_timer_new, snd_timer_new, "", "");
ffi_export::export_symbol!(snd_timer_notify, snd_timer_notify, "", "");
ffi_export::export_symbol!(snd_timer_global_new, snd_timer_global_new, "", "");
ffi_export::export_symbol!(snd_timer_global_free, snd_timer_global_free, "", "");
ffi_export::export_symbol!(snd_timer_global_register, snd_timer_global_register, "", "");
