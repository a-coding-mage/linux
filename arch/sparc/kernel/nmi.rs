// SPDX-License-Identifier: GPL-2.0-only
/* Pseudo NMI support on sparc64 systems.
 *
 * Copyright (C) 2009 David S. Miller <davem@davemloft.net>
 *
 * The NMI watchdog support and infrastructure is based almost
 * entirely upon the x86 NMI support code.
 */

// Kernel dependencies supplied by other translation units.
use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

extern "C" {
    static mut panic_on_oops: c_int;
    static mut HZ: c_uint;
    static mut nr_cpu_ids: c_int;
    static mut pcr_ops: *mut PcrOps;
    fn atomic_read(v: *const AtomicT) -> c_int;
    fn atomic_set(v: *mut AtomicT, i: c_int);
    fn atomic_inc(v: *mut AtomicT);
    fn atomic_dec(v: *mut AtomicT);
    fn notify_die(a: c_int, b: *const c_char, c: *mut PtRegs, d: c_int, e: c_int, f: c_int) -> c_int;
    fn pt_regs_trap_type(r: *mut PtRegs) -> c_int;
    fn panic(fmt: *const c_char, ...);
    fn warn(cond: c_int, fmt: *const c_char, ...);
    fn smp_processor_id() -> c_int;
    fn clear_softint(v: c_ulong);
    fn local_cpu_data() -> *mut CpuData;
    fn nmi_enter();
    fn nmi_exit();
    fn set_hardirq_stack() -> *mut c_void;
    fn restore_hardirq_stack(p: *mut c_void);
    fn cpu_data(cpu: c_int) -> *mut CpuData;
    fn mb();
    fn printk(fmt: *const c_char, ...);
    fn kmalloc_array(n: usize, size: usize, flags: c_ulong) -> *mut c_void;
    fn kfree(p: *mut c_void);
    fn smp_call_function(f: unsafe extern "C" fn(*mut c_void), data: *mut c_void, wait: c_int);
    fn local_irq_enable();
    fn mdelay(ms: c_uint);
    fn on_each_cpu(f: unsafe extern "C" fn(*mut c_void), data: *mut c_void, wait: c_int);
    fn register_reboot_notifier(nb: *mut NotifierBlock) -> c_int;
    fn smp_call_function_single(cpu: c_uint, f: unsafe extern "C" fn(*mut c_void), data: *mut c_void, wait: c_int);
    fn strncmp(a: *const c_char, b: *const c_char, n: usize) -> c_int;
}

#[repr(C)] pub struct AtomicT { pub counter: c_int }
#[repr(C)] pub struct PtRegs { _private: [u8; 0] }
#[repr(C)] pub struct CpuData { pub nmi_count: c_uint, pub irq0_irqs: c_uint }
#[repr(C)] pub struct PcrOps {
    pub write_pcr: unsafe extern "C" fn(c_int, c_ulong),
    pub write_pic: unsafe extern "C" fn(c_int, c_ulong),
    pub nmi_picl_value: unsafe extern "C" fn(c_uint) -> c_ulong,
    pub pcr_nmi_disable: c_ulong,
    pub pcr_nmi_enable: c_ulong,
}
#[repr(C)] pub struct NotifierBlock { pub notifier_call: Option<unsafe extern "C" fn(*mut NotifierBlock, c_ulong, *mut c_void) -> c_int> }

static mut panic_on_timeout: c_int = 0;
#[no_mangle] pub static mut nmi_active: AtomicT = AtomicT { counter: 0 };
static mut nmi_init_done: c_int = 0;
static mut nmi_hz: c_uint = 0;
static mut endflag: c_int = 0;
static mut wd_enabled: c_int = 0;
static mut last_irq_sum: c_uint = 0;
static mut alert_counter: c_ulong = 0;
static mut nmi_touch: c_int = 0;

#[no_mangle] pub unsafe extern "C" fn arch_touch_nmi_watchdog() {
    if atomic_read(&nmi_active) != 0 { nmi_touch = 1; }
}

#[no_mangle] pub unsafe extern "C" fn watchdog_hardlockup_probe() -> c_int { 0 }

unsafe fn die_nmi(str_: *const c_char, regs: *mut PtRegs, do_panic: c_int) {
    let this_cpu = smp_processor_id();
    if notify_die(0, str_, regs, 0, pt_regs_trap_type(regs), 2) == 0 { return; }
    if do_panic != 0 || panic_on_oops != 0 { panic(b"Watchdog detected hard LOCKUP on cpu %d\0".as_ptr() as *const c_char, this_cpu); }
    else { warn(1, b"Watchdog detected hard LOCKUP on cpu %d\0".as_ptr() as *const c_char, this_cpu); }
}

#[no_mangle] pub unsafe extern "C" fn perfctr_irq(irq: c_int, regs: *mut PtRegs) {
    let mut touched = 0;
    clear_softint(1u64 << irq);
    (*local_cpu_data()).nmi_count += 1;
    nmi_enter();
    let orig_sp = set_hardirq_stack();
    if notify_die(1, b"nmi\0".as_ptr() as *const c_char, regs, 0, pt_regs_trap_type(regs), 2) == 0 { touched = 1; }
    else { ((*pcr_ops).write_pcr)(0, (*pcr_ops).pcr_nmi_disable); }
    let sum = (*local_cpu_data()).irq0_irqs;
    if nmi_touch != 0 { nmi_touch = 0; touched = 1; }
    if touched == 0 && last_irq_sum == sum {
        alert_counter = alert_counter.wrapping_add(1);
        if alert_counter == 30u64.wrapping_mul(nmi_hz as u64) { die_nmi(b"BUG: NMI Watchdog detected LOCKUP\0".as_ptr() as *const c_char, regs, panic_on_timeout); }
    } else { last_irq_sum = sum; alert_counter = 0; }
    if wd_enabled != 0 { ((*pcr_ops).write_pic)(0, ((*pcr_ops).nmi_picl_value)(nmi_hz)); ((*pcr_ops).write_pcr)(0, (*pcr_ops).pcr_nmi_enable); }
    restore_hardirq_stack(orig_sp); nmi_exit();
}

unsafe fn get_nmi_count(cpu: c_int) -> c_uint { (*cpu_data(cpu)).nmi_count }
unsafe extern "C" fn nmi_cpu_busy(_: *mut c_void) { while endflag == 0 { mb(); } }

unsafe fn report_broken_nmi(cpu: c_int, prev: *mut c_uint) {
    printk(b"\n\0".as_ptr() as *const c_char);
    printk(b"WARNING: CPU#%d: NMI appears to be stuck (%d->%d)!\n\0".as_ptr() as *const c_char, cpu, *prev.add(cpu as usize), get_nmi_count(cpu));
    printk(b"Please report this to bugzilla.kernel.org,\n\0".as_ptr() as *const c_char);
    printk(b"and attach the output of the 'dmesg' command.\n\0".as_ptr() as *const c_char);
    wd_enabled = 0; atomic_dec(&mut nmi_active);
}

#[no_mangle] pub unsafe extern "C" fn stop_nmi_watchdog(_: *mut c_void) { if wd_enabled == 0 { return; } ((*pcr_ops).write_pcr)(0, (*pcr_ops).pcr_nmi_disable); wd_enabled = 0; atomic_dec(&mut nmi_active); }

unsafe fn check_nmi_watchdog() -> c_int { if atomic_read(&nmi_active) == 0 { return 0; } 0 }
#[no_mangle] pub unsafe extern "C" fn start_nmi_watchdog(_: *mut c_void) { if wd_enabled != 0 { return; } wd_enabled = 1; atomic_inc(&mut nmi_active); ((*pcr_ops).write_pcr)(0, (*pcr_ops).pcr_nmi_disable); ((*pcr_ops).write_pic)(0, ((*pcr_ops).nmi_picl_value)(nmi_hz)); ((*pcr_ops).write_pcr)(0, (*pcr_ops).pcr_nmi_enable); }
unsafe extern "C" fn nmi_adjust_hz_one(_: *mut c_void) { if wd_enabled != 0 { ((*pcr_ops).write_pcr)(0, (*pcr_ops).pcr_nmi_disable); ((*pcr_ops).write_pic)(0, ((*pcr_ops).nmi_picl_value)(nmi_hz)); ((*pcr_ops).write_pcr)(0, (*pcr_ops).pcr_nmi_enable); } }
#[no_mangle] pub unsafe extern "C" fn nmi_adjust_hz(new_hz: c_uint) { nmi_hz = new_hz; on_each_cpu(nmi_adjust_hz_one, core::ptr::null_mut(), 1); }
unsafe extern "C" fn nmi_shutdown(_: *mut NotifierBlock, _: c_ulong, _: *mut c_void) -> c_int { on_each_cpu(stop_nmi_watchdog, core::ptr::null_mut(), 1); 0 }
static mut nmi_reboot_notifier: NotifierBlock = NotifierBlock { notifier_call: Some(nmi_shutdown) };
#[no_mangle] pub unsafe extern "C" fn nmi_init() -> c_int { on_each_cpu(start_nmi_watchdog, core::ptr::null_mut(), 1); let mut err = check_nmi_watchdog(); if err == 0 { err = register_reboot_notifier(&mut nmi_reboot_notifier); if err != 0 { on_each_cpu(stop_nmi_watchdog, core::ptr::null_mut(), 1); atomic_set(&mut nmi_active, -1); } } nmi_init_done = 1; err }
#[no_mangle] pub unsafe extern "C" fn setup_nmi_watchdog(str_: *mut c_char) -> c_int { if strncmp(str_, b"panic\0".as_ptr() as *const c_char, 5) == 0 { panic_on_timeout = 1; } 1 }
#[no_mangle] pub unsafe extern "C" fn watchdog_hardlockup_enable(cpu: c_uint) { if atomic_read(&nmi_active) == -1 { return; } if nmi_init_done == 0 { return; } smp_call_function_single(cpu, start_nmi_watchdog, core::ptr::null_mut(), 1); }
#[no_mangle] pub unsafe extern "C" fn watchdog_hardlockup_disable(cpu: c_uint) { if atomic_read(&nmi_active) == -1 { } else { smp_call_function_single(cpu, stop_nmi_watchdog, core::ptr::null_mut(), 1); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
