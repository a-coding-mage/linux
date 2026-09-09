// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright IBM Corp. 1999, 2023
 */

// Kernel and architecture dependencies are supplied by the surrounding tree.

const __SUBCODE_MASK: u16 = 0x0600;
const __PF_RES_FIELD: u64 = 0x8000000000000000;

/*
 * 'pfault' pseudo page faults routines.
 */
static mut pfault_disable: i32 = 0;

unsafe extern "C" {
    fn diag_stat_inc(stat: i32);
    fn virt_to_phys(addr: *const pfault_refbk) -> u64;
    fn pfault_init() -> i32;
    fn register_external_irq(irq: i32, handler: unsafe extern "C" fn(ext_code, u32, usize));
    fn unregister_external_irq(irq: i32, handler: unsafe extern "C" fn(ext_code, u32, usize));
    fn irq_subclass_register(subclass: i32);
    fn cpuhp_setup_state_nocalls(state: i32, name: *const u8, startup: usize, teardown: unsafe extern "C" fn(u32) -> i32) -> i32;
    fn inc_irq_stat(irq: i32);
    fn find_task_by_pid_ns(pid: i32, ns: *const pid_namespace) -> *mut task_struct;
    fn get_task_struct(tsk: *mut task_struct);
    fn put_task_struct(tsk: *mut task_struct);
    fn wake_up_process(tsk: *mut task_struct);
    fn task_is_running(tsk: *mut task_struct) -> bool;
    fn set_need_resched_current();
    fn __set_current_state(state: i32);
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn spin_lock_irq(lock: *mut spinlock_t);
    fn spin_unlock_irq(lock: *mut spinlock_t);
    fn list_del(list: *mut list_head);
    fn list_add(list: *mut list_head, head: *mut list_head);
}

#[repr(C)]
pub struct ext_code {
    pub subcode: u16,
}

#[repr(C)]
pub struct pfault_refbk {
    pub refdiagc: u16,
    pub reffcode: u16,
    pub refdwlen: u16,
    pub refversn: u16,
    pub refgaddr: u64,
    pub refselmk: u64,
    pub refcmpmk: u64,
    pub reserved: u64,
}

const __LC_LPP: u64 = 0; // supplied by asm-offsets.h

static mut pfault_init_refbk: pfault_refbk = pfault_refbk {
    refdiagc: 0x258,
    reffcode: 0,
    refdwlen: 5,
    refversn: 2,
    refgaddr: __LC_LPP,
    refselmk: 1u64 << 48,
    refcmpmk: 1u64 << 48,
    reserved: __PF_RES_FIELD,
};

pub unsafe extern "C" fn __pfault_init() -> i32 {
    let mut rc: i32 = -95; // -EOPNOTSUPP
    if pfault_disable != 0 {
        return rc;
    }
    diag_stat_inc(0); // DIAG_STAT_X258
    // The s390 DIAG 0x258 instruction and exception-table entry are supplied by the target architecture.
    core::arch::asm!("diag {0}, {1}, 0x258", in(reg) virt_to_phys(&pfault_init_refbk), inout(reg) rc => rc, options(nostack));
    rc
}

static mut pfault_fini_refbk: pfault_refbk = pfault_refbk {
    refdiagc: 0x258,
    reffcode: 1,
    refdwlen: 5,
    refversn: 2,
    refgaddr: 0,
    refselmk: 0,
    refcmpmk: 0,
    reserved: 0,
};

pub unsafe extern "C" fn __pfault_fini() {
    if pfault_disable != 0 {
        return;
    }
    diag_stat_inc(0); // DIAG_STAT_X258
    core::arch::asm!("diag {0}, 0, 0x258", in(reg) virt_to_phys(&pfault_fini_refbk), options(nostack));
}

#[repr(C)]
pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct pid_namespace { _private: [u8; 0] }
#[repr(C)] pub struct thread_struct { pub pfault_wait: i32, pub list: list_head }
#[repr(C)] pub struct task_struct { pub thread: thread_struct }

static mut pfault_lock: spinlock_t = spinlock_t { _private: [] };
static mut pfault_list: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };

const PF_COMPLETE: u16 = 0x0080;
const LPP_PID_MASK: usize = usize::MAX; // supplied by asm/pfault.h
const IRQEXT_PFL: i32 = 0;
const EXT_IRQ_CP_SERVICE: i32 = 0;
const IRQ_SUBCLASS_SERVICE_SIGNAL: i32 = 0;
const TASK_UNINTERRUPTIBLE: i32 = 0;

unsafe extern "C" fn pfault_interrupt(ext_code: ext_code, _param32: u32, param64: usize) {
    let subcode = ext_code.subcode;
    if (subcode & 0xff00) != __SUBCODE_MASK { return; }
    inc_irq_stat(IRQEXT_PFL);
    let pid = (param64 & LPP_PID_MASK) as i32;
    let tsk = find_task_by_pid_ns(pid, core::ptr::null());
    if tsk.is_null() { return; }
    get_task_struct(tsk);
    spin_lock(&mut pfault_lock);
    if (subcode & PF_COMPLETE) != 0 {
        if (*tsk).thread.pfault_wait == 1 {
            (*tsk).thread.pfault_wait = 0;
            list_del(&mut (*tsk).thread.list);
            wake_up_process(tsk);
            put_task_struct(tsk);
        } else if task_is_running(tsk) {
            (*tsk).thread.pfault_wait = -1;
        }
    } else if (*tsk).thread.pfault_wait == 1 {
        __set_current_state(TASK_UNINTERRUPTIBLE);
        set_need_resched_current();
    } else if (*tsk).thread.pfault_wait == -1 {
        (*tsk).thread.pfault_wait = 0;
    } else {
        get_task_struct(tsk);
        (*tsk).thread.pfault_wait = 1;
        list_add(&mut (*tsk).thread.list, &mut pfault_list);
        __set_current_state(TASK_UNINTERRUPTIBLE);
        set_need_resched_current();
    }
    spin_unlock(&mut pfault_lock);
    put_task_struct(tsk);
}

unsafe extern "C" fn pfault_cpu_dead(_cpu: u32) -> i32 {
    spin_lock_irq(&mut pfault_lock);
    // list_for_each_entry_safe over pfault_list; list topology is supplied by the kernel.
    spin_unlock_irq(&mut pfault_lock);
    0
}

unsafe extern "C" fn pfault_irq_init() -> i32 {
    let mut rc = register_external_irq(EXT_IRQ_CP_SERVICE, pfault_interrupt);
    if rc != 0 { pfault_disable = 1; return rc; }
    rc = if pfault_init() == 0 { 0 } else { -95 };
    if rc != 0 {
        unregister_external_irq(EXT_IRQ_CP_SERVICE, pfault_interrupt);
        pfault_disable = 1;
        return rc;
    }
    irq_subclass_register(IRQ_SUBCLASS_SERVICE_SIGNAL);
    cpuhp_setup_state_nocalls(0, b"s390/pfault:dead\0".as_ptr(), 0, pfault_cpu_dead);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
