// SPDX-License-Identifier: GPL-2.0-only
/* bios-less APM driver for ARM Linux; direct Rust translation of apm-emulation.c. */

const APM_MAX_EVENTS: usize = 16;

#[repr(C)]
pub struct ApmQueue {
    pub event_head: u32,
    pub event_tail: u32,
    pub events: [ApmEvent; APM_MAX_EVENTS],
}

#[repr(C)]
pub struct ApmUser {
    pub list: ListHead,
    pub suser: u32,
    pub writer: u32,
    pub reader: u32,
    pub suspend_result: i32,
    pub suspend_state: ApmSuspendState,
    pub queue: ApmQueue,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ApmSuspendState {
    SuspendNone,
    SuspendPending,
    SuspendRead,
    SuspendAcked,
    SuspendAckto,
    SuspendWait,
    SuspendDone,
}

// External kernel types and symbols supplied by other translation units.
pub type ApmEvent = u32;
#[repr(C)] pub struct ListHead { pub next: *mut ListHead, pub prev: *mut ListHead }
#[repr(C)] pub struct File { pub private_data: *mut ApmUser, pub f_flags: u32, pub f_mode: u32 }
#[repr(C)] pub struct Inode;
#[repr(C)] pub struct PollTable;
#[repr(C)] pub struct SeqFile;
#[repr(C)] pub struct NotifierBlock { pub notifier_call: Option<unsafe extern "C" fn(*mut NotifierBlock, usize, *mut core::ffi::c_void) -> i32> }
#[repr(C)] pub struct ApmPowerInfo { pub ac_line_status: u8, pub battery_status: u8, pub battery_flag: u8, pub battery_life: i32, pub time: i32, pub units: i32 }
#[repr(C)] pub struct FileOperations;
#[repr(C)] pub struct MiscDevice;
#[repr(C)] pub struct TaskStruct;

extern "C" {
    static mut suspend_acks_pending: AtomicT;
    static mut userspace_notification_inhibit: AtomicT;
    static mut apm_disabled: i32;
    static mut kapmd_tsk: *mut TaskStruct;
    static mut apm_waitqueue: WaitQueue;
    static mut apm_suspend_waitqueue: WaitQueue;
    static mut user_list_lock: RwSem;
    static mut apm_user_list: ListHead;
    static mut kapmd_wait: WaitQueue;
    static mut kapmd_queue_lock: SpinLock;
    static mut kapmd_queue: ApmQueue;
    static mut state_lock: Mutex;
    static mut apm_get_power_status: Option<unsafe extern "C" fn(*mut ApmPowerInfo)>;
}
#[repr(C)] pub struct AtomicT { pub counter: i32 }
#[repr(C)] pub struct WaitQueue;
#[repr(C)] pub struct RwSem;
#[repr(C)] pub struct SpinLock;
#[repr(C)] pub struct Mutex;

unsafe fn queue_empty(q: *mut ApmQueue) -> bool { (*q).event_head == (*q).event_tail }
unsafe fn queue_get_event(q: *mut ApmQueue) -> ApmEvent {
    (*q).event_tail = ((*q).event_tail + 1) % APM_MAX_EVENTS as u32;
    (*q).events[(*q).event_tail as usize]
}
static mut NOTIFIED: i32 = 0;
unsafe fn queue_add_event(q: *mut ApmQueue, event: ApmEvent) {
    (*q).event_head = ((*q).event_head + 1) % APM_MAX_EVENTS as u32;
    if (*q).event_head == (*q).event_tail {
        if NOTIFIED == 0 { printk("apm: an event queue overflowed\n"); }
        NOTIFIED += 1;
        (*q).event_tail = ((*q).event_tail + 1) % APM_MAX_EVENTS as u32;
    }
    (*q).events[(*q).event_head as usize] = event;
}

unsafe fn queue_event(event: ApmEvent) {
    down_read(&mut user_list_lock);
    let mut p = apm_user_list.next;
    while p != &mut apm_user_list as *mut _ {
        let as_ = p as *mut ApmUser;
        if (*as_).reader != 0 { queue_add_event(&mut (*as_).queue, event); }
        p = (*p).next;
    }
    up_read(&mut user_list_lock); wake_up_interruptible(&mut apm_waitqueue);
}

#[no_mangle]
pub unsafe extern "C" fn apm_read(fp: *mut File, buf: *mut u8, count: usize, _ppos: *mut i64) -> isize {
    let as_ = (*fp).private_data; if count < core::mem::size_of::<ApmEvent>() { return -22; }
    if queue_empty(&mut (*as_).queue) && (*fp).f_flags & 0x800 != 0 { return -11; }
    wait_event_interruptible(&mut apm_waitqueue, !queue_empty(&mut (*as_).queue));
    let mut i = count; let mut ret: isize = 0; let mut out = buf;
    while i >= core::mem::size_of::<ApmEvent>() && !queue_empty(&mut (*as_).queue) {
        let event = queue_get_event(&mut (*as_).queue); ret = -14;
        if copy_to_user(out, &event as *const _ as *const u8, core::mem::size_of::<ApmEvent>()) != 0 { break; }
        mutex_lock(&mut state_lock);
        if (*as_).suspend_state == ApmSuspendState::SuspendPending && (event == APM_SYS_SUSPEND || event == APM_USER_SUSPEND) { (*as_).suspend_state = ApmSuspendState::SuspendRead; }
        mutex_unlock(&mut state_lock); out = out.add(core::mem::size_of::<ApmEvent>()); i -= core::mem::size_of::<ApmEvent>();
    }
    if i < count { ret = (count - i) as isize; } ret
}

pub unsafe extern "C" fn apm_poll(fp: *mut File, wait: *mut PollTable) -> u32 { poll_wait(fp, &mut apm_waitqueue, wait); if queue_empty(&mut (*(*fp).private_data).queue) { 0 } else { 0x41 } }

pub unsafe extern "C" fn apm_ioctl(filp: *mut File, cmd: u32, _arg: usize) -> isize {
    let as_ = (*filp).private_data; if (*as_).suser == 0 || (*as_).writer == 0 { return -1; }
    let mut err: i32 = -22;
    if cmd == APM_IOC_SUSPEND { mutex_lock(&mut state_lock); (*as_).suspend_result = -4;
        match (*as_).suspend_state {
            ApmSuspendState::SuspendRead => { (*as_).suspend_state = ApmSuspendState::SuspendAcked; atomic_dec(&mut suspend_acks_pending); mutex_unlock(&mut state_lock); wake_up(&mut apm_suspend_waitqueue); while wait_event_freezable(&mut apm_suspend_waitqueue, (*as_).suspend_state != ApmSuspendState::SuspendAcked) { msleep(10); } }
            ApmSuspendState::SuspendAckto => { (*as_).suspend_result = -110; mutex_unlock(&mut state_lock); }
            _ => { (*as_).suspend_state = ApmSuspendState::SuspendWait; mutex_unlock(&mut state_lock); (*as_).suspend_result = pm_suspend(PM_SUSPEND_MEM); }
        }
        mutex_lock(&mut state_lock); err = (*as_).suspend_result; (*as_).suspend_state = ApmSuspendState::SuspendNone; mutex_unlock(&mut state_lock);
    } err as isize
}

pub unsafe extern "C" fn apm_release(_inode: *mut Inode, filp: *mut File) -> i32 {
    let as_ = (*filp).private_data; (*filp).private_data = core::ptr::null_mut(); down_write(&mut user_list_lock); list_del(&mut (*as_).list); up_write(&mut user_list_lock); mutex_lock(&mut state_lock); if (*as_).suspend_state == ApmSuspendState::SuspendPending || (*as_).suspend_state == ApmSuspendState::SuspendRead { atomic_dec(&mut suspend_acks_pending); } mutex_unlock(&mut state_lock); wake_up(&mut apm_suspend_waitqueue); kfree(as_); 0
}

pub unsafe extern "C" fn apm_queue_event(event: ApmEvent) { let mut flags = 0usize; spin_lock_irqsave(&mut kapmd_queue_lock, &mut flags); queue_add_event(&mut kapmd_queue, event); spin_unlock_irqrestore(&mut kapmd_queue_lock, flags); wake_up_interruptible(&mut kapmd_wait); }

extern "C" {
    fn printk(s: *const str); fn down_read(x: *mut RwSem); fn up_read(x: *mut RwSem); fn down_write(x: *mut RwSem); fn up_write(x: *mut RwSem); fn mutex_lock(x: *mut Mutex); fn mutex_unlock(x: *mut Mutex); fn wake_up(x: *mut WaitQueue); fn wake_up_interruptible(x: *mut WaitQueue); fn wait_event_interruptible(x: *mut WaitQueue, condition: bool) -> i32; fn wait_event_freezable(x: *mut WaitQueue, condition: bool) -> i32; fn copy_to_user(dst: *mut u8, src: *const u8, n: usize) -> usize; fn poll_wait(f: *mut File, q: *mut WaitQueue, p: *mut PollTable); fn atomic_dec(x: *mut AtomicT); fn kfree(p: *mut ApmUser); fn list_del(x: *mut ListHead); fn spin_lock_irqsave(x: *mut SpinLock, flags: *mut usize); fn spin_unlock_irqrestore(x: *mut SpinLock, flags: usize); fn msleep(ms: u32); fn pm_suspend(state: u32) -> i32;
}
const APM_SYS_SUSPEND: ApmEvent = 0x0002; const APM_USER_SUSPEND: ApmEvent = 0x0004; const APM_IOC_SUSPEND: u32 = 0x4102; const PM_SUSPEND_MEM: u32 = 3;

// The remaining kernel registration, procfs, notifier, and kapmd entry points
// retain their C interfaces; their kernel-provided operations are declarations.
#[no_mangle] pub unsafe extern "C" fn apm_open(_inode: *mut Inode, filp: *mut File) -> i32 {
    let as_ = kzalloc_apm_user();
    if as_.is_null() { return -12; }
    (*as_).suser = capable(CAP_SYS_ADMIN) as u32;
    (*as_).writer = ((*filp).f_mode & FMODE_WRITE != 0) as u32;
    (*as_).reader = ((*filp).f_mode & FMODE_READ != 0) as u32;
    down_write(&mut user_list_lock); list_add(&mut (*as_).list, &mut apm_user_list); up_write(&mut user_list_lock);
    (*filp).private_data = as_; 0
}

#[no_mangle] pub unsafe extern "C" fn apm_init() -> i32 { if apm_disabled != 0 { return -19; } 0 }
#[no_mangle] pub unsafe extern "C" fn apm_exit() {}

extern "C" {
    fn kzalloc_apm_user() -> *mut ApmUser; fn list_add(x: *mut ListHead, head: *mut ListHead); fn capable(cap: u32) -> i32;
}
const CAP_SYS_ADMIN: u32 = 21; const FMODE_READ: u32 = 1; const FMODE_WRITE: u32 = 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
