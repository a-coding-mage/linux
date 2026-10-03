// SPDX-License-Identifier: GPL-2.0-only
// Runtime and data translation of kernel/panic.c. The original C is unchanged.
// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
// Reconstruction base: c8eeb0b98b0b3b92d92c9f79687e6cd740f12738.
// c_variadic and link_llvm_intrinsics are admitted per object by panic_rust.mk.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals,
    dead_code, missing_docs, unsafe_op_in_unsafe_fn, clippy::all,
    unused_mut, unused_macros, unused_unsafe, unreachable_pub, internal_features)]

#[allow(improper_ctypes)]
mod bindings {
    use kernel::ffi;
    include!(concat!(env!("OBJTREE"), "/rust/bindings/panic_generated.rs"));
}
use bindings::*;
use core::ffi::VaList;
use kernel::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_ulonglong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{addr_of, addr_of_mut, null, null_mut};

// VaList is the compiler's actual C va_list ABI. Never substitute void * or
// the address of a VaList wrapper for a va_list argument.
extern "C" {
    #[link_name = "vscnprintf"]
    fn native_vscnprintf(buf: *mut c_char, size: usize, fmt: *const c_char,
        args: VaList<'_, '_>) -> c_int;
    #[link_name = "vsnprintf"]
    fn native_vsnprintf(buf: *mut c_char, size: usize, fmt: *const c_char,
        args: VaList<'_, '_>) -> c_int;
    #[link_name = "vprintk"]
    fn native_vprintk(fmt: *const c_char, args: VaList<'_, '_>) -> c_int;
    fn lupos_panic_warn_with_args(file: *const c_char, line: c_int,
        caller: *mut c_void, taint: c_uint, fmt: *const c_char, args: VaList<'_, '_>);
    #[link_name = "llvm.returnaddress"]
    fn return_address(level: c_int) -> *mut c_void;
}

macro_rules! cstr { ($s:expr) => { concat!($s, "\0").as_ptr().cast::<c_char>() }; }
macro_rules! emerg { ($s:literal $(, $v:expr)* $(,)?) => {
    _printk(cstr!(concat!("\x010", $s)) $(, $v)* )
}; }
macro_rules! warn { ($s:literal $(, $v:expr)* $(,)?) => {
    _printk(cstr!(concat!("\x014", $s)) $(, $v)* )
}; }
macro_rules! info { ($s:literal $(, $v:expr)* $(,)?) => {
    _printk(cstr!(concat!("\x016", $s)) $(, $v)* )
}; }

const PANIC_TIMER_STEP: c_long = 100;
const PANIC_BLINK_SPD: c_long = 18;
const PANIC_MSG_BUFSZ: usize = 1024;
const INIT_TAINT_BUF_MAX: usize = 350;

// This wrapper gives immutable C-layout tables Sync without manufacturing a
// kernel representation. Its sole member is the bindgen-generated native type.
#[repr(transparent)]
pub struct ReadOnly<T>(T);
unsafe impl<T> Sync for ReadOnly<T> {}

include!("panic_data.rs");

// include/linux/irqflags.h's tracing decisions must be inlined into vpanic:
// trace_hardirqs_* captures its caller address. Native adapters expose only
// architecture primitives, preserving paravirtualized arch implementations.
#[inline(always)]
unsafe fn lupos_panic_local_irq_disable() {
    #[cfg(CONFIG_TRACE_IRQFLAGS)]
    let was_disabled = lupos_panic_arch_irqs_disabled();
    lupos_panic_arch_irq_disable();
    #[cfg(CONFIG_TRACE_IRQFLAGS)]
    if !was_disabled { trace_hardirqs_off(); }
}

#[inline(always)]
unsafe fn lupos_panic_local_irq_enable() {
    #[cfg(CONFIG_TRACE_IRQFLAGS)]
    trace_hardirqs_on();
    lupos_panic_arch_irq_enable();
}

unsafe fn panic_print_deprecated() {
    // pr_info_once -> DO_ONCE_LITE: deliberately the same racy once state,
    // in the same resettable section, rather than a new synchronization rule.
    #[link_section = ".data..once"]
    static mut already_done: bool = false;
    if !already_done {
        already_done = true;
        info!("Kernel: The 'panic_print' parameter is now deprecated. Please use 'panic_sys_info' and 'panic_console_replay' instead.\n");
    }
}

#[cfg(CONFIG_SYSCTL)]
unsafe extern "C" fn proc_taint(table: *const ctl_table, write: c_int,
    buffer: *mut c_void, lenp: *mut usize, ppos: *mut loff_t) -> c_int {
    let mut tmptaint = get_taint();
    if write != 0 && !lupos_panic_capable(CAP_SYS_ADMIN as _) {
        return -(EPERM as c_int);
    }
    let mut t = table.read();
    t.data = addr_of_mut!(tmptaint).cast();
    let err = proc_doulongvec_minmax(addr_of!(t), write, buffer, lenp, ppos);
    if err < 0 {
        return err;
    }
    if write != 0 {
        if panic_on_taint_nousertaint && (tmptaint & panic_on_taint) != 0 {
            return -(EINVAL as c_int);
        }
        for i in 0..TAINT_FLAGS_COUNT {
            if ((1 as c_ulong) << i) & tmptaint != 0 {
                add_taint(i, LOCKDEP_STILL_OK);
            }
        }
    }
    err
}

#[cfg(CONFIG_SYSCTL)]
unsafe extern "C" fn sysctl_panic_print_handler(table: *const ctl_table,
    write: c_int, buffer: *mut c_void, lenp: *mut usize, ppos: *mut loff_t) -> c_int {
    if write != 0 {
        panic_print_deprecated();
    }
    proc_doulongvec_minmax(table, write, buffer, lenp, ppos)
}

#[cfg(CONFIG_SYSCTL)]
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn lupos_panic_sysctls_init() -> c_int {
    lupos_panic_register_sysctl(cstr!("kernel"), kern_panic_table.0.as_ptr(),
        cstr!("kern_panic_table"), kern_panic_table.0.len());
    0
}

#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn lupos_panic_setup_sys_info(buf: *mut c_char) -> c_int {
    panic_print = sys_info_parse_param(buf);
    1
}

#[cfg(CONFIG_SYSFS)]
unsafe extern "C" fn warn_count_show(_: *mut kobject, _: *mut kobj_attribute,
    page: *mut c_char) -> isize {
    sysfs_emit(page, cstr!("%d\n"), lupos_panic_atomic_read(addr_of!(warn_count))) as _
}

#[cfg(CONFIG_SYSFS)]
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn lupos_panic_sysfs_init() -> c_int {
    sysfs_add_file_to_group(kernel_kobj, addr_of!(warn_count_attr.attr), null());
    0
}

unsafe extern "C" fn no_blink(_: c_int) -> c_long { 0 }

// Native weak entry symbols retain architecture override semantics and only
// forward here. The original default implementations, including state, are Rust.
#[no_mangle]
pub unsafe extern "C" fn lupos_panic_default_smp_self_stop() -> ! {
    loop { lupos_panic_cpu_relax(); }
}

#[no_mangle]
pub unsafe extern "C" fn lupos_panic_default_nmi_self_stop(_: *mut pt_regs) -> ! {
    bindings::panic_smp_self_stop()
}

#[no_mangle]
pub unsafe extern "C" fn lupos_panic_default_crash_smp_send_stop() {
    static mut cpus_stopped: c_int = 0;
    if cpus_stopped != 0 {
        return;
    }
    lupos_panic_smp_send_stop();
    cpus_stopped = 1;
}

#[cfg(all(CONFIG_SMP, CONFIG_CRASH_DUMP))]
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn lupos_panic_force_cpu_setup(s: *mut c_char) -> c_int {
    if s.is_null() {
        return -(EINVAL as c_int);
    }
    let mut cpu: c_int = 0;
    if kstrtoint(s, 0, addr_of_mut!(cpu)) != 0 || cpu < 0 || cpu as c_uint >= lupos_panic_nr_cpu_ids() {
        warn!("panic_force_cpu: invalid value '%s'\n", s);
        return -(EINVAL as c_int);
    }
    panic_force_cpu = cpu;
    0
}

#[cfg(all(CONFIG_SMP, CONFIG_CRASH_DUMP))]
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn lupos_panic_force_cpu_late_init() -> c_int {
    if panic_force_cpu < 0 {
        return 0;
    }
    panic_force_buf = lupos_panic_kmalloc(PANIC_MSG_BUFSZ, LUPOS_PANIC_GFP_KERNEL).cast();
    0
}

#[cfg(all(CONFIG_SMP, CONFIG_CRASH_DUMP))]
unsafe extern "C" fn do_panic_on_target_cpu(info: *mut c_void) {
    panic(cstr!("%s"), info.cast::<c_char>());
}

#[cfg(all(CONFIG_SMP, CONFIG_CRASH_DUMP))]
#[no_mangle]
pub unsafe extern "C" fn lupos_panic_default_redirect_cpu(target_cpu: c_int,
    msg: *mut c_void) -> c_int {
    static mut panic_csd: lupos_panic_csd_storage = unsafe { zeroed() };
    panic_csd.data.func = Some(do_panic_on_target_cpu);
    panic_csd.data.info = msg;
    smp_call_function_single_async(target_cpu, addr_of_mut!(panic_csd.data))
}

#[cfg(all(CONFIG_SMP, CONFIG_CRASH_DUMP))]
unsafe fn panic_try_force_cpu(fmt: *const c_char, args: VaList<'_, '_>) -> bool {
    let this_cpu = lupos_panic_raw_cpu();
    let mut old_cpu = PANIC_CPU_INVALID;
    if panic_force_cpu < 0 || this_cpu == panic_force_cpu {
        return false;
    }
    if !lupos_panic_cpu_online(panic_force_cpu as _) {
        warn!("panic: target CPU %d is offline, continuing on CPU %d\n",
            panic_force_cpu, this_cpu);
        return false;
    }
    if panic_in_progress() {
        return false;
    }
    if !lupos_panic_atomic_try_cmpxchg(addr_of_mut!(panic_redirect_cpu),
        addr_of_mut!(old_cpu), this_cpu) {
        return false;
    }
    let msg = if !panic_force_buf.is_null() {
        native_vsnprintf(panic_force_buf, PANIC_MSG_BUFSZ, fmt, args);
        panic_force_buf.cast_const()
    } else {
        cstr!("Redirected panic (buffer unavailable)")
    };
    console_verbose();
    bust_spinlocks(1);
    emerg!("panic: Redirecting from CPU %d to CPU %d for crash kernel.\n",
        this_cpu, panic_force_cpu);
    if test_taint(TAINT_DIE) == 0 && oops_in_progress <= 1
        && cfg!(CONFIG_DEBUG_BUGVERBOSE) {
        dump_stack();
    }
    if bindings::panic_smp_redirect_cpu(panic_force_cpu, msg.cast_mut().cast()) != 0 {
        lupos_panic_atomic_set(addr_of_mut!(panic_redirect_cpu), PANIC_CPU_INVALID);
        warn!("panic: failed to redirect to CPU %d, continuing on CPU %d\n",
            panic_force_cpu, this_cpu);
        return false;
    }
    true
}

#[cfg(not(all(CONFIG_SMP, CONFIG_CRASH_DUMP)))]
unsafe fn panic_try_force_cpu(_: *const c_char, _: VaList<'_, '_>) -> bool { false }

#[no_mangle]
pub unsafe extern "C" fn panic_try_start() -> bool {
    let mut old_cpu = PANIC_CPU_INVALID;
    let this_cpu = lupos_panic_raw_cpu();
    lupos_panic_atomic_try_cmpxchg(addr_of_mut!(panic_cpu), addr_of_mut!(old_cpu), this_cpu)
}

#[no_mangle]
pub unsafe extern "C" fn panic_reset() {
    lupos_panic_atomic_set(addr_of_mut!(panic_cpu), PANIC_CPU_INVALID);
}

#[no_mangle]
pub unsafe extern "C" fn panic_in_progress() -> bool {
    lupos_panic_atomic_read(addr_of!(panic_cpu)) != PANIC_CPU_INVALID
}

#[no_mangle]
pub unsafe extern "C" fn panic_on_this_cpu() -> bool {
    lupos_panic_atomic_read(addr_of!(panic_cpu)) == lupos_panic_raw_cpu()
}

#[no_mangle]
pub unsafe extern "C" fn panic_on_other_cpu() -> bool {
    panic_in_progress() && !panic_on_this_cpu()
}

#[no_mangle]
pub unsafe extern "C" fn nmi_panic(regs: *mut pt_regs, msg: *const c_char) {
    if panic_try_start() {
        panic(cstr!("%s"), msg);
    } else if panic_on_other_cpu() {
        bindings::nmi_panic_self_stop(regs);
    }
}

#[no_mangle]
pub unsafe extern "C" fn check_panic_on_warn(origin: *const c_char) {
    if panic_on_warn != 0 {
        panic(cstr!("%s: panic_on_warn set ...\n"), origin);
    }
    let limit = lupos_panic_read_uint(addr_of!(warn_limit));
    // C's usual arithmetic conversions compare signed atomic count as unsigned.
    // Increment even when limit is zero, preserving /sys/kernel/warn_count.
    if lupos_panic_atomic_inc_return(addr_of_mut!(warn_count)) as c_uint >= limit
        && limit != 0 {
        panic(cstr!("%s: system warned too often (kernel.warn_limit is %d)"), origin, limit);
    }
}

unsafe fn panic_trigger_all_cpu_backtrace() {
    panic_triggering_all_cpu_backtrace = true;
    if panic_this_cpu_backtrace_printed {
        lupos_panic_trigger_allbutcpu_backtrace(lupos_panic_raw_cpu());
    } else {
        lupos_panic_trigger_all_backtrace();
    }
    panic_triggering_all_cpu_backtrace = false;
}

unsafe fn panic_other_cpus_shutdown(crash_kexec: bool) {
    if panic_print & SYS_INFO_ALL_BT as c_ulong != 0 {
        panic_trigger_all_cpu_backtrace();
    }
    if !crash_kexec {
        lupos_panic_smp_send_stop();
    } else {
        bindings::crash_smp_send_stop();
    }
}

#[no_mangle]
#[cold]
pub unsafe extern "C" fn vpanic(fmt: *const c_char, mut args: VaList<'_, '_>) -> ! {
    static mut panic_msg_buf: [c_char; PANIC_MSG_BUFSZ] = [0; PANIC_MSG_BUFSZ];
    let buf = addr_of_mut!(panic_msg_buf).cast::<c_char>();
    let mut i_next: c_long = 0;
    let mut state: c_int = 0;
    let post_notifiers = crash_kexec_post_notifiers;
    if panic_on_warn != 0 {
        panic_on_warn = 0;
    }
    lupos_panic_local_irq_disable();
    lupos_panic_preempt_disable_notrace();
    // as_va_list reborrows the same native list on the admitted x86_64 ABI;
    // no va_copy is introduced. Failed redirection therefore preserves C's
    // exact consumption behavior before its later vscnprintf call.
    if panic_try_force_cpu(fmt, args.as_va_list()) {
        set_cpu_online(lupos_panic_smp_cpu() as _, false);
        bindings::panic_smp_self_stop();
    }
    if !panic_try_start() && panic_on_other_cpu() {
        bindings::panic_smp_self_stop();
    }
    console_verbose();
    bust_spinlocks(1);
    let len = native_vscnprintf(buf, PANIC_MSG_BUFSZ, fmt, args.as_va_list()) as c_long;
    if len != 0 && *buf.add((len - 1) as usize) == b'\n' as c_char {
        *buf.add((len - 1) as usize) = 0;
    }
    emerg!("Kernel panic - not syncing: %s\n", buf);
    if lupos_panic_atomic_read(addr_of!(panic_redirect_cpu)) != PANIC_CPU_INVALID
        && panic_force_cpu == lupos_panic_raw_cpu() {
        emerg!("panic: Redirected from CPU %d, skipping stack dump.\n",
            lupos_panic_atomic_read(addr_of!(panic_redirect_cpu)));
    } else if test_taint(TAINT_DIE) != 0 || oops_in_progress > 1 {
        panic_this_cpu_backtrace_printed = true;
    } else if cfg!(CONFIG_DEBUG_BUGVERBOSE) {
        dump_stack();
        panic_this_cpu_backtrace_printed = true;
    }
    lupos_panic_kgdb(buf);
    if !post_notifiers {
        lupos_panic_crash_kexec(null_mut());
    }
    panic_other_cpus_shutdown(post_notifiers);
    printk_legacy_allow_panic_sync();
    atomic_notifier_call_chain(addr_of_mut!(panic_notifier_list), 0, buf.cast());
    sys_info(panic_print);
    lupos_panic_kmsg_dump_desc(KMSG_DUMP_PANIC, buf);
    if post_notifiers {
        lupos_panic_crash_kexec(null_mut());
    }
    lupos_panic_console_unblank();
    debug_locks_off();
    console_flush_on_panic(CONSOLE_FLUSH_PENDING);
    if panic_print & SYS_INFO_PANIC_CONSOLE_REPLAY as c_ulong != 0 || panic_console_replay {
        console_flush_on_panic(CONSOLE_REPLAY_ALL);
    }
    if matches!(panic_blink, None) {
        panic_blink = Some(no_blink);
    }
    if panic_timeout > 0 {
        emerg!("Rebooting in %d seconds..\n", panic_timeout);
        let mut i: c_long = 0;
        // The multiplication occurs at C int width before comparison with long.
        while i < panic_timeout.wrapping_mul(1000) as c_long {
            lupos_panic_touch_nmi_watchdog();
            if i >= i_next {
                state ^= 1;
                i = i.wrapping_add(panic_blink.unwrap_unchecked()(state));
                i_next = i.wrapping_add(3600 / PANIC_BLINK_SPD);
            }
            lupos_panic_mdelay(PANIC_TIMER_STEP as _);
            i = i.wrapping_add(PANIC_TIMER_STEP);
        }
    }
    if panic_timeout != 0 {
        if panic_reboot_mode != REBOOT_UNDEFINED {
            reboot_mode = panic_reboot_mode;
        }
        emergency_restart();
    }
    #[cfg(CONFIG_SPARC)]
    {
        stop_a_enabled = 1;
        emerg!("Press Stop-A (L1-A) from sun keyboard or send break\ntwice on console to return to the boot prom\n");
    }
    #[cfg(CONFIG_S390)]
    lupos_panic_disabled_wait();
    emerg!("---[ end Kernel panic - not syncing: %s ]---\n", buf);
    suppress_printk = 1;
    console_flush_on_panic(CONSOLE_FLUSH_PENDING);
    nbcon_atomic_flush_unsafe();
    lupos_panic_local_irq_enable();
    let mut i: c_long = 0;
    loop {
        lupos_panic_touch_softlockup_watchdog();
        if i >= i_next {
            state ^= 1;
            i = i.wrapping_add(panic_blink.unwrap_unchecked()(state));
            i_next = i.wrapping_add(3600 / PANIC_BLINK_SPD);
        }
        lupos_panic_mdelay(PANIC_TIMER_STEP as _);
        i = i.wrapping_add(PANIC_TIMER_STEP);
    }
}

#[no_mangle]
#[cold]
pub unsafe extern "C" fn panic(fmt: *const c_char, mut args: ...) -> ! {
    vpanic(fmt, args.as_va_list())
}

unsafe fn print_tainted_seq(s: *mut seq_buf, verbose: bool) {
    let mut sep = cstr!("");
    if tainted_mask == 0 {
        seq_buf_puts(s, cstr!("Not tainted"));
        return;
    }
    seq_buf_printf(s, cstr!("Tainted: "));
    for i in 0..TAINT_FLAGS_COUNT as usize {
        let t = &taint_flags.0[i];
        let is_set = lupos_panic_test_bit(i as _, addr_of!(tainted_mask));
        let c = if is_set { t.c_true } else { t.c_false };
        if verbose {
            if is_set {
                seq_buf_printf(s, cstr!("%s[%c]=%s"), sep, c as c_int, t.desc);
                sep = cstr!(", ");
            }
        } else {
            seq_buf_putc(s, c as u8);
        }
    }
}

#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn lupos_panic_alloc_taint_buf() -> c_int {
    let mut size = b"Tainted: ".len();
    for i in 0..TAINT_FLAGS_COUNT as usize {
        size += 2 + 4 + strlen(taint_flags.0[i].desc);
    }
    size += 1;
    let buf = lupos_panic_kmalloc(size, LUPOS_PANIC_GFP_KERNEL).cast::<c_char>();
    if buf.is_null() {
        panic(cstr!("Failed to allocate taint string buffer"));
    }
    taint_buf = buf;
    taint_buf_size = size;
    0
}

unsafe fn _print_tainted(verbose: bool) -> *const c_char {
    let mut s: seq_buf = zeroed();
    lupos_panic_seq_buf_init(addr_of_mut!(s), taint_buf, taint_buf_size as _);
    print_tainted_seq(addr_of_mut!(s), verbose);
    lupos_panic_seq_buf_str(addr_of_mut!(s))
}

#[no_mangle]
pub unsafe extern "C" fn print_tainted() -> *const c_char { _print_tainted(false) }
#[no_mangle]
pub unsafe extern "C" fn print_tainted_verbose() -> *const c_char { _print_tainted(true) }

#[no_mangle]
pub unsafe extern "C" fn test_taint(flag: c_uint) -> c_int {
    lupos_panic_test_bit(flag as _, addr_of!(tainted_mask)) as c_int
}

#[no_mangle]
pub unsafe extern "C" fn get_taint() -> c_ulong { tainted_mask }

#[no_mangle]
pub unsafe extern "C" fn add_taint(flag: c_uint, lockdep_ok: lockdep_ok) {
    if lockdep_ok == LOCKDEP_NOW_UNRELIABLE && lupos_panic_debug_locks_off() != 0 {
        warn!("Disabling lock debugging due to kernel taint\n");
    }
    lupos_panic_set_bit(flag as _, addr_of_mut!(tainted_mask));
    if tainted_mask & panic_on_taint != 0 {
        panic_on_taint = 0;
        panic(cstr!("panic_on_taint set ..."));
    }
}

unsafe fn spin_msec(msecs: c_int) {
    for _ in 0..msecs {
        lupos_panic_touch_nmi_watchdog();
        lupos_panic_mdelay(1);
    }
}

unsafe fn do_oops_enter_exit() {
    static mut spin_counter: c_int = 0;
    if pause_on_oops == 0 { return; }
    let flags = lupos_panic_spin_lock_irqsave(addr_of_mut!(pause_on_oops_lock));
    if pause_on_oops_flag == 0 {
        pause_on_oops_flag = 1;
    } else if spin_counter == 0 {
        spin_counter = pause_on_oops;
        loop {
            lupos_panic_spin_unlock(addr_of_mut!(pause_on_oops_lock));
            spin_msec(MSEC_PER_SEC as _);
            lupos_panic_spin_lock(addr_of_mut!(pause_on_oops_lock));
            spin_counter = spin_counter.wrapping_sub(1);
            if spin_counter == 0 { break; }
        }
        pause_on_oops_flag = 0;
    } else {
        while spin_counter != 0 {
            lupos_panic_spin_unlock(addr_of_mut!(pause_on_oops_lock));
            spin_msec(1);
            lupos_panic_spin_lock(addr_of_mut!(pause_on_oops_lock));
        }
    }
    lupos_panic_spin_unlock_irqrestore(addr_of_mut!(pause_on_oops_lock), flags);
}

#[no_mangle]
pub unsafe extern "C" fn oops_may_print() -> bool { pause_on_oops_flag == 0 }

#[no_mangle]
pub unsafe extern "C" fn oops_enter() {
    lupos_panic_nbcon_emergency_enter();
    lupos_panic_tracing_off();
    debug_locks_off();
    do_oops_enter_exit();
    #[cfg(CONFIG_SMP)]
    if sysctl_oops_all_cpu_backtrace != 0 {
        lupos_panic_trigger_all_backtrace();
    }
}

unsafe fn print_oops_end_marker() {
    warn!("---[ end trace %016llx ]---\n", 0u64);
}

#[no_mangle]
pub unsafe extern "C" fn oops_exit() {
    do_oops_enter_exit();
    print_oops_end_marker();
    lupos_panic_nbcon_emergency_exit();
    lupos_panic_kmsg_dump_desc(KMSG_DUMP_OOPS, null());
}

#[no_mangle]
pub unsafe extern "C" fn __warn(file: *const c_char, line: c_int, caller: *mut c_void,
    taint: c_uint, regs: *mut pt_regs, args: *mut warn_args) {
    lupos_panic_nbcon_emergency_enter();
    lupos_panic_disable_trace_on_warning();
    let current = lupos_panic_current();
    if !file.is_null() {
        warn!("WARNING: %s:%d at %pS, CPU#%d: %s/%d\n", file, line, caller,
            lupos_panic_raw_cpu(), addr_of!((*current).comm).cast::<c_char>(), (*current).pid);
    } else {
        warn!("WARNING: at %pS, CPU#%d: %s/%d\n", caller,
            lupos_panic_raw_cpu(), addr_of!((*current).comm).cast::<c_char>(), (*current).pid);
    }
    if !args.is_null() { lupos_panic_vprintk_args(args); }
    lupos_panic_print_modules();
    if !regs.is_null() { show_regs(regs); }
    check_panic_on_warn(cstr!("kernel"));
    if regs.is_null() { dump_stack(); }
    lupos_panic_print_irqtrace_events(current);
    print_oops_end_marker();
    lupos_panic_trace_error_report_end(ERROR_DETECTOR_WARN, caller as c_ulong);
    add_taint(taint, LOCKDEP_STILL_OK);
    lupos_panic_nbcon_emergency_exit();
}

#[cfg(all(CONFIG_BUG, not(LUPOS_PANIC_WARN_FLAGS)))]
#[no_mangle]
pub unsafe extern "C" fn warn_slowpath_fmt(file: *const c_char, line: c_int,
    taint: c_uint, fmt: *const c_char, mut args: ...) {
    let rcu = lupos_panic_warn_rcu_enter();
    if lupos_panic_kunit_suppressed(true) {
        lupos_panic_warn_rcu_exit(rcu);
        return;
    }
    warn!("------------[ cut here ]------------\n");
    if fmt.is_null() {
        __warn(file, line, return_address(0), taint, null_mut(), null_mut());
        lupos_panic_warn_rcu_exit(rcu);
        return;
    }
    lupos_panic_warn_with_args(file, line, return_address(0), taint, fmt, args.as_va_list());
    lupos_panic_warn_rcu_exit(rcu);
}

#[cfg(all(CONFIG_BUG, LUPOS_PANIC_WARN_FLAGS))]
#[no_mangle]
pub unsafe extern "C" fn __warn_printk(fmt: *const c_char, mut args: ...) {
    let rcu = lupos_panic_warn_rcu_enter();
    if lupos_panic_kunit_suppressed(false) {
        lupos_panic_warn_rcu_exit(rcu);
        return;
    }
    warn!("------------[ cut here ]------------\n");
    native_vprintk(fmt, args.as_va_list());
    lupos_panic_warn_rcu_exit(rcu);
}

#[cfg(CONFIG_BUG)]
unsafe extern "C" fn clear_warn_once_set(_: *mut c_void, _: u64) -> c_int {
    lupos_panic_generic_bug_clear_once();
    let start = addr_of_mut!(__start_once).cast::<u8>();
    let end = addr_of_mut!(__end_once).cast::<u8>();
    memset(start.cast(), 0, end as usize - start as usize);
    0
}

#[cfg(CONFIG_BUG)]
unsafe extern "C" fn clear_warn_once_open(inode: *mut inode, file: *mut file) -> c_int {
    simple_attr_open(inode, file, None, Some(clear_warn_once_set), cstr!("%lld\n"))
}

#[cfg(CONFIG_BUG)]
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn lupos_panic_register_warn_debugfs() -> c_int {
    lupos_panic_debugfs_create_file_unsafe(cstr!("clear_warn_once"), 0o200,
        null_mut(), null_mut(), addr_of!(clear_warn_once_fops.0));
    0
}

#[cfg(CONFIG_STACKPROTECTOR)]
#[no_mangle]
#[cold]
pub unsafe extern "C" fn lupos_panic_stack_chk_fail(caller: *mut c_void) -> ! {
    // Compiler return address, user-access state and noinstr annotation are
    // captured by the tiny native entry shell before entering this body.
    panic(cstr!("stack-protector: Kernel stack is corrupted in: %pB"), caller)
}

unsafe extern "C" fn panic_print_set(val: *const c_char, kp: *const kernel_param) -> c_int {
    panic_print_deprecated();
    param_set_ulong(val, kp)
}

unsafe extern "C" fn panic_print_get(val: *mut c_char, kp: *const kernel_param) -> c_int {
    param_get_ulong(val, kp)
}

#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn lupos_panic_oops_setup(s: *mut c_char) -> c_int {
    if s.is_null() { return -(EINVAL as c_int); }
    if strcmp(s, cstr!("panic")) == 0 { panic_on_oops = 1; }
    0
}

#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn lupos_panic_on_taint_setup(mut s: *mut c_char) -> c_int {
    if s.is_null() { return -(EINVAL as c_int); }
    let taint_str = strsep(addr_of_mut!(s), cstr!(","));
    // include/linux/kstrtox.h's kstrtoul is inline. Preserve its native
    // unsigned-long/unsigned-long-long size and alignment dispatch in Rust.
    let err = if size_of::<c_ulong>() == size_of::<c_ulonglong>()
        && core::mem::align_of::<c_ulong>() == core::mem::align_of::<c_ulonglong>() {
        kstrtoull(taint_str, 16, addr_of_mut!(panic_on_taint).cast())
    } else {
        _kstrtoul(taint_str, 16, addr_of_mut!(panic_on_taint))
    };
    if err != 0 {
        return -(EINVAL as c_int);
    }
    panic_on_taint &= TAINT_FLAGS_MAX as c_ulong;
    if panic_on_taint == 0 { return -(EINVAL as c_int); }
    if !s.is_null() && strcmp(s, cstr!("nousertaint")) == 0 {
        panic_on_taint_nousertaint = true;
    }
    info!("panic_on_taint: bitmask=0x%lx nousertaint_mode=%s\n", panic_on_taint,
        if panic_on_taint_nousertaint { cstr!("enabled") } else { cstr!("disabled") });
    0
}
