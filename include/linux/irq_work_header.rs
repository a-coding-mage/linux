/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the corresponding Linux Rust translation units:
 * irq_work_types, rcuwait, smp_types, and architecture irq_work support.
 */

/*
 * An entry can be in one of four states:
 *
 * free       NULL, 0 -> {claimed}       : free to be used
 * claimed   NULL, 3 -> {pending}       : claimed to be enqueued
 * pending   next, 3 -> {busy}          : queued, pending callback
 * busy      NULL, 2 -> {free, claimed} : callback in progress, can be claimed
 */

#[inline]
pub unsafe fn __irq_work_init(
    func: Option<unsafe extern "C" fn(work: *mut irq_work)>,
    flags: u32,
) -> irq_work {
    irq_work {
        node: irq_work_node { u_flags: flags },
        func,
        irqwait: __RCUWAIT_INITIALIZER(),
    }
}

#[inline]
pub unsafe fn IRQ_WORK_INIT(
    func: Option<unsafe extern "C" fn(work: *mut irq_work)>,
) -> irq_work {
    __irq_work_init(func, 0)
}

#[inline]
pub unsafe fn IRQ_WORK_INIT_LAZY(
    func: Option<unsafe extern "C" fn(work: *mut irq_work)>,
) -> irq_work {
    __irq_work_init(func, IRQ_WORK_LAZY)
}

#[inline]
pub unsafe fn IRQ_WORK_INIT_HARD(
    func: Option<unsafe extern "C" fn(work: *mut irq_work)>,
) -> irq_work {
    __irq_work_init(func, IRQ_WORK_HARD_IRQ)
}

/* C macro: struct irq_work name = IRQ_WORK_INIT(_f) */

#[inline]
pub unsafe fn init_irq_work(
    work: *mut irq_work,
    func: Option<unsafe extern "C" fn(work: *mut irq_work)>,
) {
    *work = IRQ_WORK_INIT(func);
}

#[inline]
pub unsafe fn irq_work_is_pending(work: *mut irq_work) -> bool {
    (atomic_read(&mut (*work).node.a_flags) & IRQ_WORK_PENDING) != 0
}

#[inline]
pub unsafe fn irq_work_is_busy(work: *mut irq_work) -> bool {
    (atomic_read(&mut (*work).node.a_flags) & IRQ_WORK_BUSY) != 0
}

#[inline]
pub unsafe fn irq_work_is_hard(work: *mut irq_work) -> bool {
    (atomic_read(&mut (*work).node.a_flags) & IRQ_WORK_HARD_IRQ) != 0
}

extern "C" {
    pub fn irq_work_queue(work: *mut irq_work) -> bool;
    pub fn irq_work_queue_on(work: *mut irq_work, cpu: i32) -> bool;

    pub fn irq_work_tick();
    pub fn irq_work_sync(work: *mut irq_work);
}

/* CONFIG_IRQ_WORK controls whether the architecture implementation is used. */
#[cfg(feature = "CONFIG_IRQ_WORK")]
extern "C" {
    pub fn irq_work_run();
    pub fn irq_work_needs_cpu() -> bool;
    pub fn irq_work_single(arg: *mut core::ffi::c_void);

    pub fn arch_irq_work_raise();
}

#[cfg(not(feature = "CONFIG_IRQ_WORK"))]
#[inline]
pub fn irq_work_needs_cpu() -> bool { false }

#[cfg(not(feature = "CONFIG_IRQ_WORK"))]
#[inline]
pub fn irq_work_run() {}

#[cfg(not(feature = "CONFIG_IRQ_WORK"))]
#[inline]
pub fn irq_work_single(_arg: *mut core::ffi::c_void) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
