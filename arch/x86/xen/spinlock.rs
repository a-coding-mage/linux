// SPDX-License-Identifier: GPL-2.0
/*
 * Split spinlock implementation out into its own file, so it can be
 * compiled in a FTRACE-compatible way.
 */

use core::ffi::c_void;

// Kernel/Xen dependencies supplied by other translation units.
extern "C" {
    static mut nopvspin: bool;
    static mut virt_spin_lock_key: c_void;
    static mut pv_ops_lock: PvOpsLock;

    fn xen_send_IPI_one(cpu: i32, vector: i32);
    fn xen_test_irq_pending(irq: i32) -> bool;
    fn xen_clear_irq_pending(irq: i32);
    fn xen_poll_irq(irq: i32);
    fn in_nmi() -> bool;
    fn xen_vcpu_stolen();
    fn num_possible_cpus() -> i32;
    fn __pv_init_lock_hash();
    fn __pv_queued_spin_lock_slowpath();
    fn __raw_callee_save___pv_queued_spin_unlock();
    fn static_branch_disable(key: *mut c_void);
    fn static_call_update(call: *const c_void, target: unsafe extern "C" fn());
    fn kasprintf(flags: u32, format: *const u8, ...) -> *mut i8;
    fn kfree(ptr: *mut i8);
    fn bind_ipi_to_irqhandler(
        vector: i32,
        cpu: i32,
        handler: unsafe extern "C" fn(i32, *mut c_void) -> IrqReturn,
        flags: u32,
        name: *mut i8,
        dev_id: *mut c_void,
    ) -> i32;
    fn disable_irq(irq: i32);
    fn unbind_from_irqhandler(irq: i32, dev_id: *mut c_void);
    fn printk(format: *const u8, ...);
    fn warn(condition: bool, format: *const u8, ...);
}

#[repr(C)]
pub struct PvOpsLock {
    pub wait: Option<unsafe extern "C" fn(*mut u8, u8)>,
    pub kick: Option<unsafe extern "C" fn(i32)>,
    pub vcpu_is_preempted: Option<unsafe extern "C" fn(i32) -> bool>,
}

#[repr(C)]
pub struct AtomicT {
    pub counter: i32,
}

#[repr(C)]
pub struct IrqReturn(pub i32);

const XEN_SPIN_UNLOCK_VECTOR: i32 = 0;
const GFP_KERNEL: u32 = 0;
const IRQF_PERCPU: u32 = 1 << 0;
const IRQF_NOBALANCING: u32 = 1 << 1;
const IRQ_HANDLED: IrqReturn = IrqReturn(1);

// DEFINE_PER_CPU state; per-CPU accessors are supplied by the kernel.
extern "C" {
    fn per_cpu_lock_kicker_irq(cpu: i32) -> *mut i32;
    fn per_cpu_irq_name(cpu: i32) -> *mut *mut i8;
    fn per_cpu_xen_qlock_wait_nest(cpu: i32) -> *mut AtomicT;
    fn this_cpu_lock_kicker_irq() -> *mut i32;
}

unsafe extern "C" fn xen_qlock_kick(cpu: i32) {
    let irq = *per_cpu_lock_kicker_irq(cpu);

    /* Don't kick if the target's kicker interrupt is not initialized. */
    if irq == -1 {
        return;
    }

    xen_send_IPI_one(cpu, XEN_SPIN_UNLOCK_VECTOR);
}

/*
 * Halt the current CPU & release it back to the host
 */
unsafe extern "C" fn xen_qlock_wait(byte: *mut u8, val: u8) {
    let irq = *this_cpu_lock_kicker_irq();
    let nest_cnt = per_cpu_xen_qlock_wait_nest(0);

    /* If kicker interrupts not initialized yet, just spin */
    if irq == -1 || in_nmi() {
        return;
    }

    /* Detect reentry. */
    (*nest_cnt).counter = (*nest_cnt).counter.wrapping_add(1);

    /* If irq pending already and no nested call clear it. */
    if (*nest_cnt).counter == 1 && xen_test_irq_pending(irq) {
        xen_clear_irq_pending(irq);
    } else if core::ptr::read_volatile(byte) == val {
        /* Block until irq becomes pending (or a spurious wakeup) */
        xen_poll_irq(irq);
    }

    (*nest_cnt).counter = (*nest_cnt).counter.wrapping_sub(1);
}

unsafe extern "C" fn dummy_handler(_irq: i32, _dev_id: *mut c_void) -> IrqReturn {
    panic!("BUG");
}

pub unsafe extern "C" fn xen_init_lock_cpu(cpu: i32) {
    let irq;
    let name;

    if nopvspin {
        return;
    }

    warn(
        *per_cpu_lock_kicker_irq(cpu) >= 0,
        b"spinlock on CPU%d exists on IRQ%d!\n\0".as_ptr(),
        cpu,
        *per_cpu_lock_kicker_irq(cpu),
    );

    name = kasprintf(GFP_KERNEL, b"spinlock%d\0".as_ptr(), cpu);
    *per_cpu_irq_name(cpu) = name;
    irq = bind_ipi_to_irqhandler(
        XEN_SPIN_UNLOCK_VECTOR,
        cpu,
        dummy_handler,
        IRQF_PERCPU | IRQF_NOBALANCING,
        name,
        core::ptr::null_mut(),
    );

    if irq >= 0 {
        disable_irq(irq); /* make sure it's never delivered */
        *per_cpu_lock_kicker_irq(cpu) = irq;
    }

    printk(b"cpu %d spinlock event irq %d\n\0".as_ptr(), cpu, irq);
}

pub unsafe extern "C" fn xen_uninit_lock_cpu(cpu: i32) {
    let irq;

    if nopvspin {
        return;
    }

    kfree(*per_cpu_irq_name(cpu));
    *per_cpu_irq_name(cpu) = core::ptr::null_mut();
    /*
     * When booting the kernel with 'mitigations=auto,nosmt', the secondary
     * CPUs are not activated, and lock_kicker_irq is not initialized.
     */
    irq = *per_cpu_lock_kicker_irq(cpu);
    if irq == -1 {
        return;
    }

    unbind_from_irqhandler(irq, core::ptr::null_mut());
    *per_cpu_lock_kicker_irq(cpu) = -1;
}

/* PV_CALLEE_SAVE_REGS_THUNK(xen_vcpu_stolen); */

/*
 * Our init of PV spinlocks is split in two init functions due to us
 * using paravirt patching and jump labels patching and having to do
 * all of this before SMP code is invoked.
 *
 * The paravirt patching needs to be done _before_ the alternative asm code
 * is started, otherwise we would not patch the core kernel code.
 */
pub unsafe extern "C" fn xen_init_spinlocks() {
    /*  Don't need to use pvqspinlock code if there is only 1 vCPU. */
    if num_possible_cpus() == 1 {
        nopvspin = true;
    }

    if nopvspin {
        printk(b"xen: PV spinlocks disabled\n\0".as_ptr());
        static_branch_disable(&mut virt_spin_lock_key);
        return;
    }
    printk(b"xen: PV spinlocks enabled\n\0".as_ptr());

    __pv_init_lock_hash();
    static_call_update(core::ptr::null(), __pv_queued_spin_lock_slowpath);
    static_call_update(core::ptr::null(), __raw_callee_save___pv_queued_spin_unlock);
    pv_ops_lock.wait = Some(xen_qlock_wait);
    pv_ops_lock.kick = Some(xen_qlock_kick);
    pv_ops_lock.vcpu_is_preempted = Some(xen_vcpu_stolen);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
