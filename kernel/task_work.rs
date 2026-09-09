// SPDX-License-Identifier: GPL-2.0

use core::ffi::c_void;

#[repr(C)]
pub struct callback_head {
    pub next: *mut callback_head,
    pub func: Option<unsafe extern "C" fn(*mut callback_head)>,
}

#[repr(C)]
pub struct task_struct {
    pub task_works: *mut callback_head,
    pub pi_lock: c_void,
    pub flags: usize,
}

#[repr(C)]
pub struct irq_work {
    _private: [u8; 0],
}

pub type task_work_func_t = Option<unsafe extern "C" fn(*mut callback_head)>;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum task_work_notify_mode {
    TWA_NONE,
    TWA_RESUME,
    TWA_SIGNAL,
    TWA_SIGNAL_NO_IPI,
    TWA_NMI_CURRENT,
}

unsafe extern "C" {
    static mut current: *mut task_struct;

    fn kasan_record_aux_stack(work: *mut callback_head);
    fn set_notify_resume(task: *mut task_struct);
    fn set_notify_signal(task: *mut task_struct);
    fn __set_notify_signal(task: *mut task_struct);
    fn set_tsk_thread_flag(task: *mut task_struct, flag: usize);
    fn irq_work_queue(work: *mut irq_work);
    fn task_work_pending(task: *mut task_struct) -> bool;
    fn raw_spin_lock_irqsave(lock: *mut c_void, flags: *mut usize);
    fn raw_spin_unlock_irqrestore(lock: *mut c_void, flags: usize);
    fn raw_spin_lock_irq(lock: *mut c_void);
    fn raw_spin_unlock_irq(lock: *mut c_void);
    fn cond_resched();
    fn try_cmpxchg(
        ptr: *mut *mut callback_head,
        old: *mut callback_head,
        new: *mut callback_head,
    ) -> bool;
    fn warn_on_once(condition: bool) -> bool;
}

const EINVAL: i32 = 22;
const ESRCH: i32 = 3;
const PF_EXITING: usize = 0x00000004;
const TIF_NOTIFY_RESUME: usize = 0;

static mut WORK_EXITED: callback_head = callback_head {
    next: core::ptr::null_mut(),
    func: None,
};

#[cfg(CONFIG_IRQ_WORK)]
unsafe extern "C" fn task_work_set_notify_irq(_entry: *mut irq_work) {
    /*
     * no-op IPI
     *
     * TWA_NMI_CURRENT will already have set the TIF flag, all
     * this interrupt does it tickle the return-to-user path.
     */
}

#[cfg(CONFIG_IRQ_WORK)]
static mut IRQ_WORK_NMI_RESUME: irq_work = irq_work { _private: [] };

pub unsafe extern "C" fn task_work_add(
    task: *mut task_struct,
    work: *mut callback_head,
    notify: task_work_notify_mode,
) -> i32 {
    let mut head: *mut callback_head;

    if notify == task_work_notify_mode::TWA_NMI_CURRENT {
        if warn_on_once(task != current) {
            return -EINVAL;
        }
        #[cfg(not(CONFIG_IRQ_WORK))]
        {
            return -EINVAL;
        }
    } else {
        kasan_record_aux_stack(work);
    }

    head = (*task).task_works;
    loop {
        if head == core::ptr::addr_of_mut!(WORK_EXITED) {
            return -ESRCH;
        }
        (*work).next = head;
        if try_cmpxchg(&mut (*task).task_works, head, work) {
            break;
        }
        head = (*task).task_works;
    }

    match notify {
        task_work_notify_mode::TWA_NONE => {}
        task_work_notify_mode::TWA_RESUME => set_notify_resume(task),
        task_work_notify_mode::TWA_SIGNAL => set_notify_signal(task),
        task_work_notify_mode::TWA_SIGNAL_NO_IPI => __set_notify_signal(task),
        #[cfg(CONFIG_IRQ_WORK)]
        task_work_notify_mode::TWA_NMI_CURRENT => {
            set_tsk_thread_flag(current, TIF_NOTIFY_RESUME);
            irq_work_queue(core::ptr::addr_of_mut!(IRQ_WORK_NMI_RESUME));
        }
        #[cfg(not(CONFIG_IRQ_WORK))]
        task_work_notify_mode::TWA_NMI_CURRENT => {}
    }

    0
}

pub unsafe extern "C" fn task_work_cancel_match(
    task: *mut task_struct,
    r#match: unsafe extern "C" fn(*mut callback_head, *mut c_void) -> bool,
    data: *mut c_void,
) -> *mut callback_head {
    let mut pprev: *mut *mut callback_head = &mut (*task).task_works;
    let mut work: *mut callback_head;
    let mut flags = 0usize;

    if !task_work_pending(task) {
        return core::ptr::null_mut();
    }
    raw_spin_lock_irqsave(&mut (*task).pi_lock, &mut flags);
    work = *pprev;
    while !work.is_null() {
        if !r#match(work, data) {
            pprev = &mut (*work).next;
            work = *pprev;
        } else if try_cmpxchg(pprev, work, (*work).next) {
            break;
        }
    }
    raw_spin_unlock_irqrestore(&mut (*task).pi_lock, flags);
    work
}

unsafe extern "C" fn task_work_func_match(cb: *mut callback_head, data: *mut c_void) -> bool {
    (*cb).func.map(|f| f as *const () == data as *const ()).unwrap_or(false)
}

pub unsafe extern "C" fn task_work_cancel_func(
    task: *mut task_struct,
    func: task_work_func_t,
) -> *mut callback_head {
    task_work_cancel_match(task, task_work_func_match, func.map(|f| f as *mut c_void).unwrap_or(core::ptr::null_mut()))
}

unsafe extern "C" fn task_work_match(cb: *mut callback_head, data: *mut c_void) -> bool {
    cb == data as *mut callback_head
}

pub unsafe extern "C" fn task_work_cancel(task: *mut task_struct, cb: *mut callback_head) -> bool {
    task_work_cancel_match(task, task_work_match, cb as *mut c_void) == cb
}

pub unsafe extern "C" fn task_work_run() {
    let task = current;
    let mut work: *mut callback_head;
    let mut head: *mut callback_head;
    let mut next: *mut callback_head;

    loop {
        work = (*task).task_works;
        loop {
            head = core::ptr::null_mut();
            if work.is_null() {
                if (*task).flags & PF_EXITING != 0 {
                    head = core::ptr::addr_of_mut!(WORK_EXITED);
                } else {
                    break;
                }
            }
            if try_cmpxchg(&mut (*task).task_works, work, head) {
                break;
            }
            work = (*task).task_works;
        }
        if work.is_null() {
            break;
        }
        raw_spin_lock_irq(&mut (*task).pi_lock);
        raw_spin_unlock_irq(&mut (*task).pi_lock);
        loop {
            next = (*work).next;
            if let Some(func) = (*work).func {
                func(work);
            }
            work = next;
            cond_resched();
            if work.is_null() {
                break;
            }
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
