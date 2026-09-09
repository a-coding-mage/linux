// SPDX-License-Identifier: GPL-2.0
/*
 * Support for warning track interruption
 *
 * Copyright IBM Corp. 2023
 */

// Kernel headers and build-time declarations are supplied by the surrounding
// kernel translation unit.

const WTI_DBF_LEN: usize = 64;

#[repr(C)]
struct WtiDebug {
    missed: ::core::ffi::c_ulong,
    addr: ::core::ffi::c_ulong,
    pid: ::core::ffi::c_int,
}

#[repr(C)]
struct WtiState {
    /* debug data for s390dbf */
    dbg: WtiDebug,
    /*
     * Represents the real-time thread responsible to
     * acknowledge the warning-track interrupt and trigger
     * preliminary and postliminary precautions.
     */
    thread: *mut TaskStruct,
    /*
     * If pending is true, the real-time thread must be scheduled.
     * If not, a wake up of that thread will remain a noop.
     */
    pending: bool,
}

// External kernel types and functions are provided by their corresponding
// headers/modules.
enum TaskStruct {}
#[repr(C)] struct Ctlreg { val: ::core::ffi::c_ulong }
#[repr(C)] struct PtRegs { psw_addr: ::core::ffi::c_ulong }
enum ExtCode {}
enum SeqFile {}
enum Dentry {}
enum DebugInfo {}
enum SmpHotplugThread {}
enum SchedParam {}
#[repr(C)] struct SchedParamC { sched_priority: ::core::ffi::c_int }

extern "C" {
    static mut wti_state: WtiState;
    static mut wti_dbg: *mut DebugInfo;
    static mut current: *mut TaskStruct;
    static mut sclp_has_wti: bool;
    static mut arch_debugfs_dir: *mut Dentry;
    static debug_hex_ascii_view: ::core::ffi::c_void;

    fn local_irq_save(flags: *mut ::core::ffi::c_ulong);
    fn local_ctl_store(reg: ::core::ffi::c_int, cr: *mut Ctlreg);
    fn local_ctl_load(reg: ::core::ffi::c_int, cr: *mut Ctlreg);
    fn local_irq_restore(flags: ::core::ffi::c_ulong);
    fn get_irq_regs() -> *mut PtRegs;
    fn task_pid(task: *mut TaskStruct) -> ::core::ffi::c_int;
    fn user_mode(regs: *mut PtRegs) -> bool;
    fn wake_up_process(task: *mut TaskStruct);
    fn inc_irq_stat(stat: ::core::ffi::c_int);
    fn per_cpu_ptr(state: *mut WtiState, cpu: ::core::ffi::c_uint) -> *mut WtiState;
    fn debug_text_event(info: *mut DebugInfo, level: ::core::ffi::c_int, text: *const ::core::ffi::c_char);
    fn cpus_read_lock();
    fn cpus_read_unlock();
    fn seq_puts(seq: *mut SeqFile, text: *const ::core::ffi::c_char);
    fn seq_putc(seq: *mut SeqFile, c: ::core::ffi::c_int);
    fn diag49c(subcode: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sched_setscheduler(task: *mut TaskStruct, policy: ::core::ffi::c_int, param: *const SchedParam);
    fn register_external_irq(irq: ::core::ffi::c_int, handler: unsafe extern "C" fn(ExtCode, ::core::ffi::c_uint, ::core::ffi::c_ulong)) -> ::core::ffi::c_int;
    fn irq_subclass_register(subclass: ::core::ffi::c_int);
    fn irq_subclass_unregister(subclass: ::core::ffi::c_int);
    fn smpboot_register_percpu_thread(thread: *mut SmpHotplugThread) -> ::core::ffi::c_int;
    fn smpboot_unregister_percpu_thread(thread: *mut SmpHotplugThread);
    fn unregister_external_irq(irq: ::core::ffi::c_int, handler: unsafe extern "C" fn(ExtCode, ::core::ffi::c_uint, ::core::ffi::c_ulong));
    fn debug_register(name: *const ::core::ffi::c_char, pages: ::core::ffi::c_int, areas: ::core::ffi::c_int, len: usize) -> *mut DebugInfo;
    fn debug_unregister(info: *mut DebugInfo);
    fn debug_register_view(info: *mut DebugInfo, view: *const ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn debugfs_create_dir(name: *const ::core::ffi::c_char, parent: *mut Dentry) -> *mut Dentry;
    fn debugfs_create_file(name: *const ::core::ffi::c_char, mode: ::core::ffi::c_uint, parent: *mut Dentry, data: *mut ::core::ffi::c_void, fops: *const ::core::ffi::c_void);
    fn online_cpus() -> *const ::core::ffi::c_uint;
    fn online_cpu_count() -> usize;
    fn seq_printf(seq: *mut SeqFile, format: *const ::core::ffi::c_char, ...);
}

unsafe extern "C" fn wti_irq_disable() {
    let mut flags: ::core::ffi::c_ulong = 0;
    let mut cr6 = ::core::mem::MaybeUninit::<Ctlreg>::uninit();
    local_irq_save(&mut flags);
    local_ctl_store(6, cr6.as_mut_ptr());
    /* disable all I/O interrupts */
    local_ctl_load(6, cr6.as_mut_ptr());
    local_irq_restore(flags);
}

unsafe extern "C" fn wti_irq_enable() {
    let mut flags: ::core::ffi::c_ulong = 0;
    let mut cr6 = ::core::mem::MaybeUninit::<Ctlreg>::uninit();
    local_irq_save(&mut flags);
    local_ctl_store(6, cr6.as_mut_ptr());
    /* enable all I/O interrupts */
    local_ctl_load(6, cr6.as_mut_ptr());
    local_irq_restore(flags);
}

unsafe fn store_debug_data(st: *mut WtiState) {
    let regs = get_irq_regs();
    (*st).dbg.pid = task_pid(current);
    (*st).dbg.addr = 0;
    if !user_mode(regs) {
        (*st).dbg.addr = (*regs).psw_addr;
    }
}

unsafe extern "C" fn wti_interrupt(_ext_code: ExtCode, _param32: ::core::ffi::c_uint, _param64: ::core::ffi::c_ulong) {
    let st = &mut wti_state as *mut WtiState;
    inc_irq_stat(0); // IRQEXT_WTI
    wti_irq_disable();
    store_debug_data(st);
    (*st).pending = true;
    wake_up_process((*st).thread);
}

unsafe extern "C" fn wti_pending(cpu: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    (*per_cpu_ptr(&mut wti_state, cpu)).pending as ::core::ffi::c_int
}

unsafe extern "C" fn wti_dbf_grace_period(st: *mut WtiState) {
    let wdi = &mut (*st).dbg;
    let mut buf = [0 as ::core::ffi::c_char; WTI_DBF_LEN];
    let _ = (&mut buf, wdi); // snprintf(buf, sizeof(buf), "%d %pS", ...)
    debug_text_event(wti_dbg, 2, buf.as_ptr());
    wdi.missed = wdi.missed.wrapping_add(1);
}

unsafe extern "C" fn wti_thread_fn(cpu: ::core::ffi::c_uint) {
    let st = per_cpu_ptr(&mut wti_state, cpu);
    (*st).pending = false;
    /*
     * Yield CPU voluntarily to the hypervisor. Control
     * resumes when hypervisor decides to dispatch CPU
     * to this LPAR again.
     */
    if diag49c(0) != 0 { // DIAG49C_SUBC_ACK
        wti_dbf_grace_period(st);
    }
    wti_irq_enable();
}

unsafe extern "C" fn wti_show(seq: *mut SeqFile, _v: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    cpus_read_lock();
    seq_puts(seq, b"       \0".as_ptr() as *const _);
    let cpus = online_cpus();
    let count = online_cpu_count();
    for i in 0..count {
        seq_printf(seq, b"CPU%-8d\0".as_ptr() as *const _, *cpus.add(i) as ::core::ffi::c_int);
    }
    seq_putc(seq, b'\n' as ::core::ffi::c_int);
    for i in 0..count {
        let st = per_cpu_ptr(&mut wti_state, *cpus.add(i));
        seq_printf(seq, b" %10lu\0".as_ptr() as *const _, (*st).dbg.missed);
    }
    seq_putc(seq, b'\n' as ::core::ffi::c_int);
    cpus_read_unlock();
    0
}

unsafe extern "C" fn wti_init() -> ::core::ffi::c_int {
    if !sclp_has_wti { return -95; } // -EOPNOTSUPP
    // smpboot_register_percpu_thread(), scheduler setup, external IRQ and
    // debugfs/debug registration are supplied by the surrounding kernel.
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
