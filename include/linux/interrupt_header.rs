/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of linux/interrupt.h. Included dependencies are supplied externally. */

pub const IRQF_TRIGGER_NONE: u32 = 0x00000000;
pub const IRQF_TRIGGER_RISING: u32 = 0x00000001;
pub const IRQF_TRIGGER_FALLING: u32 = 0x00000002;
pub const IRQF_TRIGGER_HIGH: u32 = 0x00000004;
pub const IRQF_TRIGGER_LOW: u32 = 0x00000008;
pub const IRQF_TRIGGER_MASK: u32 = IRQF_TRIGGER_HIGH | IRQF_TRIGGER_LOW | IRQF_TRIGGER_RISING | IRQF_TRIGGER_FALLING;
pub const IRQF_TRIGGER_PROBE: u32 = 0x00000010;
pub const IRQF_SHARED: u32 = 0x00000080;
pub const IRQF_PROBE_SHARED: u32 = 0x00000100;
pub const __IRQF_TIMER: u32 = 0x00000200;
pub const IRQF_PERCPU: u32 = 0x00000400;
pub const IRQF_NOBALANCING: u32 = 0x00000800;
pub const IRQF_IRQPOLL: u32 = 0x00001000;
pub const IRQF_ONESHOT: u32 = 0x00002000;
pub const IRQF_NO_SUSPEND: u32 = 0x00004000;
pub const IRQF_FORCE_RESUME: u32 = 0x00008000;
pub const IRQF_NO_THREAD: u32 = 0x00010000;
pub const IRQF_EARLY_RESUME: u32 = 0x00020000;
pub const IRQF_COND_SUSPEND: u32 = 0x00040000;
pub const IRQF_NO_AUTOEN: u32 = 0x00080000;
pub const IRQF_NO_DEBUG: u32 = 0x00100000;
pub const IRQF_COND_ONESHOT: u32 = 0x00200000;
pub const IRQF_TIMER: u32 = __IRQF_TIMER | IRQF_NO_SUSPEND | IRQF_NO_THREAD;

#[repr(C)]
pub enum IrqcContext { IRQC_IS_HARDIRQ = 0, IRQC_IS_NESTED }
pub type irq_handler_t = Option<unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> irqreturn_t>;

pub type irqreturn_t = i32;
#[repr(C)] pub struct cpumask { _private: [u8; 0] }
pub type cpumask_t = cpumask;
#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct proc_dir_entry { _private: [u8; 0] }
#[repr(C)] pub struct kref { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }
#[repr(C)] pub struct atomic_t { pub counter: i32 }

#[repr(C)] pub union irqaction_dev_id { pub dev_id: *mut core::ffi::c_void, pub percpu_dev_id: *mut core::ffi::c_void }
#[repr(C)] pub struct irqaction {
    pub handler: irq_handler_t, pub dev_id: irqaction_dev_id, pub affinity: *const cpumask,
    pub next: *mut irqaction, pub thread_fn: irq_handler_t, pub thread: *mut task_struct,
    pub secondary: *mut irqaction, pub irq: u32, pub flags: u32, pub thread_flags: usize,
    pub thread_mask: usize, pub name: *const core::ffi::c_char, pub dir: *mut proc_dir_entry,
}

#[repr(C)] pub struct irq_affinity_notify {
    pub irq: u32, pub kref: kref, pub work: work_struct,
    pub notify: Option<unsafe extern "C" fn(*mut irq_affinity_notify, *const cpumask_t)>,
    pub release: Option<unsafe extern "C" fn(*mut kref)>,
}
pub const IRQ_AFFINITY_MAX_SETS: usize = 4;
#[repr(C)] pub struct irq_affinity {
    pub pre_vectors: u32, pub post_vectors: u32, pub nr_sets: u32,
    pub set_size: [u32; IRQ_AFFINITY_MAX_SETS],
    pub calc_sets: Option<unsafe extern "C" fn(*mut irq_affinity, u32)>, pub priv_: *mut core::ffi::c_void,
}
#[repr(C)] pub struct irq_affinity_desc { pub mask: cpumask, pub is_managed: u32 }

pub const IRQ_NOTCONNECTED: u32 = 1u32 << 31;
pub const NR_SOFTIRQS: usize = 10;
pub const HI_SOFTIRQ: u32 = 0; pub const TIMER_SOFTIRQ: u32 = 1; pub const NET_TX_SOFTIRQ: u32 = 2;
pub const NET_RX_SOFTIRQ: u32 = 3; pub const BLOCK_SOFTIRQ: u32 = 4; pub const IRQ_POLL_SOFTIRQ: u32 = 5;
pub const TASKLET_SOFTIRQ: u32 = 6; pub const SCHED_SOFTIRQ: u32 = 7; pub const HRTIMER_SOFTIRQ: u32 = 8;
pub const RCU_SOFTIRQ: u32 = 9;
pub const SOFTIRQ_HOTPLUG_SAFE_MASK: usize = (1usize << TIMER_SOFTIRQ) | (1usize << IRQ_POLL_SOFTIRQ) | (1usize << HRTIMER_SOFTIRQ) | (1usize << RCU_SOFTIRQ);

#[repr(C)] pub struct softirq_action { pub action: Option<unsafe extern "C" fn()> }
#[repr(C)] pub struct tasklet_struct {
    pub next: *mut tasklet_struct, pub state: usize, pub count: atomic_t, pub use_callback: bool,
    pub callback: Option<unsafe extern "C" fn(*mut tasklet_struct)>, pub data: usize,
}
pub const TASKLET_STATE_SCHED: u32 = 0; pub const TASKLET_STATE_RUN: u32 = 1;

#[repr(C)] pub enum irqchip_irq_state { IRQCHIP_STATE_PENDING, IRQCHIP_STATE_ACTIVE, IRQCHIP_STATE_MASKED, IRQCHIP_STATE_LINE_LEVEL }

extern "C" {
    pub fn no_action(cpl: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t;
    pub fn request_threaded_irq(irq: u32, handler: irq_handler_t, thread_fn: irq_handler_t, flags: usize, name: *const core::ffi::c_char, dev: *mut core::ffi::c_void) -> i32;
    pub fn request_any_context_irq(irq: u32, handler: irq_handler_t, flags: usize, name: *const core::ffi::c_char, dev_id: *mut core::ffi::c_void) -> i32;
    pub fn request_nmi(irq: u32, handler: irq_handler_t, flags: usize, name: *const core::ffi::c_char, dev: *mut core::ffi::c_void) -> i32;
    pub fn free_irq(irq: u32, dev_id: *mut core::ffi::c_void) -> *const core::ffi::c_void;
    pub fn free_nmi(irq: u32, dev_id: *mut core::ffi::c_void) -> *const core::ffi::c_void;
    pub fn disable_irq_nosync(irq: u32); pub fn disable_hardirq(irq: u32) -> bool; pub fn disable_irq(irq: u32);
    pub fn enable_irq(irq: u32); pub fn irq_wake_thread(irq: u32, dev_id: *mut core::ffi::c_void);
    pub fn irq_set_irq_wake(irq: u32, on: u32) -> i32;
    pub fn irq_get_irqchip_state(irq: u32, which: irqchip_irq_state, state: *mut bool) -> i32;
    pub fn irq_set_irqchip_state(irq: u32, which: irqchip_irq_state, state: bool) -> i32;
    pub fn do_softirq(); pub fn __do_softirq(); pub fn open_softirq(nr: i32, action: Option<unsafe extern "C" fn()>);
    pub fn softirq_init(); pub fn __raise_softirq_irqoff(nr: u32); pub fn raise_softirq_irqoff(nr: u32); pub fn raise_softirq(nr: u32);
    pub fn raise_ktimers_thread(nr: u32); pub fn __tasklet_schedule(t: *mut tasklet_struct); pub fn __tasklet_hi_schedule(t: *mut tasklet_struct);
    pub fn tasklet_kill(t: *mut tasklet_struct); pub fn tasklet_init(t: *mut tasklet_struct, func: Option<unsafe extern "C" fn(usize)>, data: usize);
    pub fn tasklet_setup(t: *mut tasklet_struct, callback: Option<unsafe extern "C" fn(*mut tasklet_struct)>);
    pub fn probe_irq_on() -> usize; pub fn probe_irq_off(val: usize) -> i32; pub fn probe_irq_mask(val: usize) -> u32;
    pub fn show_interrupts(p: *mut seq_file, v: *mut core::ffi::c_void) -> i32; pub fn arch_show_interrupts(p: *mut seq_file, prec: i32) -> i32;
    pub fn early_irq_init() -> i32; pub fn arch_probe_nr_irqs() -> i32; pub fn arch_early_irq_init() -> i32;
}

#[inline] pub unsafe fn request_irq(irq: u32, handler: irq_handler_t, flags: usize, name: *const core::ffi::c_char, dev: *mut core::ffi::c_void) -> i32 { request_threaded_irq(irq, handler, None, flags | IRQF_COND_ONESHOT as usize, name, dev) }
#[inline] pub unsafe fn enable_irq_wake(irq: u32) -> i32 { irq_set_irq_wake(irq, 1) }
#[inline] pub unsafe fn disable_irq_wake(irq: u32) -> i32 { irq_set_irq_wake(irq, 0) }

extern "C" {
    pub fn request_percpu_irq_affinity(irq: u32, handler: irq_handler_t, devname: *const core::ffi::c_char, affinity: *const cpumask_t, percpu_dev_id: *mut core::ffi::c_void) -> i32;
    pub fn request_percpu_irq(irq: u32, handler: irq_handler_t, devname: *const core::ffi::c_char, percpu_dev_id: *mut core::ffi::c_void) -> i32;
    pub fn request_percpu_nmi(irq: u32, handler: irq_handler_t, name: *const core::ffi::c_char, affinity: *const cpumask, dev_id: *mut core::ffi::c_void) -> i32;
    pub fn free_percpu_irq(irq: u32, dev_id: *mut core::ffi::c_void); pub fn free_percpu_nmi(irq: u32, dev_id: *mut core::ffi::c_void);
    pub fn devm_request_threaded_irq(dev: *mut device, irq: u32, handler: irq_handler_t, thread_fn: irq_handler_t, irqflags: usize, devname: *const core::ffi::c_char, dev_id: *mut core::ffi::c_void) -> i32;
    pub fn devm_free_irq(dev: *mut device, irq: u32, dev_id: *mut core::ffi::c_void);
    pub fn irq_has_action(irq: u32) -> bool; pub fn disable_percpu_irq(irq: u32); pub fn enable_percpu_irq(irq: u32, type_: u32); pub fn irq_percpu_is_enabled(irq: u32) -> bool;
    pub fn disable_nmi_nosync(irq: u32); pub fn disable_percpu_nmi(irq: u32); pub fn enable_nmi(irq: u32); pub fn enable_percpu_nmi(irq: u32, type_: u32);
    pub fn prepare_percpu_nmi(irq: u32) -> i32; pub fn teardown_percpu_nmi(irq: u32); pub fn irq_inject_interrupt(irq: u32) -> i32;
    pub fn suspend_device_irqs(); pub fn resume_device_irqs(); pub fn rearm_wake_irq(irq: u32);
    pub fn irq_set_affinity(irq: u32, mask: *const cpumask) -> i32; pub fn irq_force_affinity(irq: u32, mask: *const cpumask) -> i32;
    pub fn irq_can_set_affinity(irq: u32) -> i32; pub fn irq_select_affinity(irq: u32) -> i32;
    pub fn irq_update_affinity_desc(irq: u32, affinity: *mut irq_affinity_desc) -> i32;
    pub fn irq_set_affinity_notifier(irq: u32, notify: *mut irq_affinity_notify) -> i32;
    pub fn irq_create_affinity_masks(nvec: u32, affd: *mut irq_affinity) -> *mut irq_affinity_desc;
    pub fn irq_calc_affinity_vectors(minvec: u32, maxvec: u32, affd: *const irq_affinity) -> u32;
    pub fn tasklet_unlock(t: *mut tasklet_struct); pub fn tasklet_unlock_wait(t: *mut tasklet_struct); pub fn tasklet_unlock_spin_wait(t: *mut tasklet_struct);
    pub fn init_irq_proc();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
