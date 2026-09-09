// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of x86/kernel/nmi.c. Kernel-provided types, macros, and
 * functions remain external dependencies. */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_uchar, c_void};

#[repr(C)] pub struct raw_spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct pt_regs { pub ip: c_ulong }
#[repr(C)] pub struct nmiaction { pub list: list_head, pub handler: Option<unsafe extern "C" fn(c_uint, *mut pt_regs) -> c_int>, pub flags: c_ulong, pub name: *const c_char, pub max_duration: u64 }
pub type nmi_handler_t = Option<unsafe extern "C" fn(c_uint, *mut pt_regs) -> c_int>;
#[repr(C)] pub struct nmi_desc { pub lock: raw_spinlock_t, pub emerg_handler: nmi_handler_t, pub head: list_head }
#[repr(C)] pub struct nmi_stats { pub normal: c_uint, pub unknown: c_uint, pub external: c_uint, pub swallow: c_uint, pub recv_jiffies: c_ulong, pub idt_seq: c_ulong, pub idt_nmi_seq: c_ulong, pub idt_ignored: c_ulong, pub idt_calls: c_ulong, pub idt_seq_snap: c_ulong, pub idt_nmi_seq_snap: c_ulong, pub idt_ignored_snap: c_ulong, pub idt_calls_snap: c_long }
pub type c_long = isize;

const NMI_LOCAL: usize = 0; const NMI_UNKNOWN: usize = 1; const NMI_SERR: usize = 2; const NMI_IO_CHECK: usize = 3;
const NMI_MAX: usize = 4; const NMI_FLAG_FIRST: c_ulong = 1;
const NMI_REASON_CLEAR_MASK: c_uchar = 0x0f; const NMI_REASON_CLEAR_SERR: c_uchar = 4; const NMI_REASON_CLEAR_IOCHK: c_uchar = 8;
const NMI_REASON_MASK: c_uchar = 0xc0; const NMI_REASON_SERR: c_uchar = 0x80; const NMI_REASON_IOCHK: c_uchar = 0x40; const NMI_REASON_PORT: u16 = 0x61;

static mut NMI_DESC: [nmi_desc; NMI_MAX] = [nmi_desc { lock: raw_spinlock_t { _private: [] }, emerg_handler: None, head: list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() } }; NMI_MAX];
static mut IGNORE_NMIS: c_int = 0;
#[no_mangle] pub static mut unknown_nmi_panic: c_int = 0;
#[no_mangle] pub static mut panic_on_unrecovered_nmi: c_int = 0;
#[no_mangle] pub static mut panic_on_io_nmi: c_int = 0;
static mut NMI_LONGEST_NS: u64 = 1_000_000;
static mut NMI_STATS: nmi_stats = nmi_stats { normal: 0, unknown: 0, external: 0, swallow: 0, recv_jiffies: 0, idt_seq: 0, idt_nmi_seq: 0, idt_ignored: 0, idt_calls: 0, idt_seq_snap: 0, idt_nmi_seq_snap: 0, idt_ignored_snap: 0, idt_calls_snap: 0 };
static mut SWALLOW_NMI: bool = false; static mut LAST_NMI_RIP: c_ulong = 0;

extern "C" {
    fn sched_clock() -> u64; fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn raw_spin_lock_irqsave(lock: *mut raw_spinlock_t, flags: *mut c_ulong); fn raw_spin_unlock_irqrestore(lock: *mut raw_spinlock_t, flags: c_ulong);
    fn raw_spin_trylock(lock: *mut raw_spinlock_t) -> bool; fn raw_spin_unlock(lock: *mut raw_spinlock_t);
    fn synchronize_rcu(); fn in_nmi() -> bool; fn smp_processor_id() -> c_int; fn nmi_panic(r: *mut pt_regs, s: *const c_char) -> !;
    fn outb(v: c_uchar, p: u16); fn show_regs(r: *mut pt_regs); fn touch_nmi_watchdog(); fn udelay(v: c_uint);
    fn read_cr2() -> c_ulong; fn write_cr2(v: c_ulong); fn local_db_save() -> c_ulong; fn local_db_restore(v: c_ulong);
    fn irqentry_nmi_enter(r: *mut pt_regs) -> c_ulong; fn irqentry_nmi_exit(r: *mut pt_regs, s: c_ulong);
    fn default_do_nmi(r: *mut pt_regs); fn instrumentation_begin(); fn instrumentation_end(); fn cpu_relax();
    fn microcode_nmi_handler_enabled() -> bool; fn microcode_nmi_handler() -> bool; fn microcode_offline_nmi_handler(); fn arch_cpu_is_offline(cpu: c_int) -> bool;
    fn run_crash_ipi_callback(r: *mut pt_regs); fn x86_get_nmi_reason() -> c_uchar; fn reassert_nmi();
}

unsafe fn nmi_handle(ty: c_uint, regs: *mut pt_regs) -> c_int {
    let d = &mut NMI_DESC[ty as usize]; if let Some(h) = d.emerg_handler { return h(ty, regs); }
    let mut handled = 0; let mut p = d.head.next;
    while !p.is_null() && p != &mut d.head as *mut _ { let a = &mut *(p as *mut nmiaction); if let Some(h) = a.handler { handled += h(ty, regs); } p = (*p).next; }
    handled
}

#[no_mangle] pub unsafe extern "C" fn __register_nmi_handler(ty: c_uint, action: *mut nmiaction) -> c_int {
    let d = &mut NMI_DESC[ty as usize]; if action.is_null() || (*action).handler.is_none() { return -22; }
    let mut flags = 0; raw_spin_lock_irqsave(&mut d.lock, &mut flags); let l = &mut (*action).list;
    if (*action).flags & NMI_FLAG_FIRST != 0 { l.next = d.head.next; l.prev = &mut d.head; } else { l.prev = d.head.prev; l.next = &mut d.head; }
    (*l.prev).next = l; (*l.next).prev = l; raw_spin_unlock_irqrestore(&mut d.lock, flags); 0
}

#[no_mangle] pub unsafe extern "C" fn unregister_nmi_handler(ty: c_uint, name: *const c_char) { let d=&mut NMI_DESC[ty as usize]; let mut p=d.head.next; while p != &mut d.head { let n=&mut *(p as *mut nmiaction); let next=(*p).next; if strcmp(n.name,name)==0 { (*n.list.prev).next=n.list.next; (*n.list.next).prev=n.list.prev; synchronize_rcu(); n.list.next=&mut n.list; n.list.prev=&mut n.list; break } p=next; } }
#[no_mangle] pub unsafe extern "C" fn set_emergency_nmi_handler(ty: c_uint, h: nmi_handler_t) { NMI_DESC[ty as usize].emerg_handler=h; }

unsafe fn pci_serr_error(mut reason: c_uchar, regs: *mut pt_regs) { if nmi_handle(NMI_SERR as _,regs)!=0{return} if panic_on_unrecovered_nmi!=0 { nmi_panic(regs, b"NMI: Not continuing\0".as_ptr() as _); } reason=(reason & NMI_REASON_CLEAR_MASK)|NMI_REASON_CLEAR_SERR; outb(reason,NMI_REASON_PORT); }
unsafe fn io_check_error(mut reason: c_uchar, regs: *mut pt_regs) { if nmi_handle(NMI_IO_CHECK as _,regs)!=0{return} show_regs(regs); if panic_on_io_nmi!=0 { nmi_panic(regs,b"NMI IOCK error: Not continuing\0".as_ptr() as _); } reason=(reason&NMI_REASON_CLEAR_MASK)|NMI_REASON_CLEAR_IOCHK; outb(reason,NMI_REASON_PORT); let mut i=20000; while i>0 { touch_nmi_watchdog(); udelay(100); i-=1; } outb(reason & !NMI_REASON_CLEAR_IOCHK,NMI_REASON_PORT); }
unsafe fn unknown_nmi_error(reason: c_uchar, regs: *mut pt_regs) { let h=nmi_handle(NMI_UNKNOWN as _,regs); if h!=0 { NMI_STATS.unknown+=h as u32; return } NMI_STATS.unknown+=1; if unknown_nmi_panic!=0 || panic_on_unrecovered_nmi!=0 { nmi_panic(regs,b"NMI: Not continuing\0".as_ptr() as _); } let _=reason; }

#[no_mangle] pub unsafe extern "C" fn default_do_nmi(regs: *mut pt_regs) {
    let mut reason=0u8; let b2b=(*regs).ip==LAST_NMI_RIP; if !b2b { SWALLOW_NMI=false; } LAST_NMI_RIP=(*regs).ip;
    instrumentation_begin();
    if microcode_nmi_handler_enabled() && microcode_nmi_handler() { instrumentation_end(); return; }
    let handled=nmi_handle(NMI_LOCAL as _,regs); NMI_STATS.normal+=handled as u32;
    if handled!=0 { if handled>1 { SWALLOW_NMI=true; } instrumentation_end(); return; }
    while !raw_spin_trylock(core::ptr::null_mut()) { run_crash_ipi_callback(regs); cpu_relax(); }
    reason=x86_get_nmi_reason();
    if reason & NMI_REASON_MASK != 0 { if reason & NMI_REASON_SERR != 0 { pci_serr_error(reason,regs); } else if reason & NMI_REASON_IOCHK != 0 { io_check_error(reason,regs); } NMI_STATS.external+=1; raw_spin_unlock(core::ptr::null_mut()); instrumentation_end(); return; }
    raw_spin_unlock(core::ptr::null_mut()); if b2b && SWALLOW_NMI { NMI_STATS.swallow+=1; } else { unknown_nmi_error(reason,regs); } instrumentation_end();
}

#[no_mangle] pub unsafe extern "C" fn exc_nmi(regs: *mut pt_regs) {
    if arch_cpu_is_offline(smp_processor_id()) { if microcode_nmi_handler_enabled() { microcode_offline_nmi_handler(); } return; }
    let old=read_cr2(); default_do_nmi(regs); if old!=read_cr2() { write_cr2(old); }
}

#[cfg(feature="kvm_intel")]
#[no_mangle] pub unsafe extern "C" fn exc_nmi_kvm_vmx(regs: *mut pt_regs) { exc_nmi(regs); }

/* CONFIG_NMI_CHECK_CPU supplies the following diagnostics in the kernel. */
#[cfg(feature="nmi_check_cpu")]
pub unsafe extern "C" fn nmi_backtrace_stall_snap(_btp: *const c_void) { }
#[cfg(feature="nmi_check_cpu")]
pub unsafe extern "C" fn nmi_backtrace_stall_check(_btp: *const c_void) { }

#[no_mangle] pub unsafe extern "C" fn local_touch_nmi() { LAST_NMI_RIP=0; }
#[no_mangle] pub unsafe extern "C" fn stop_nmi() { IGNORE_NMIS+=1; }
#[no_mangle] pub unsafe extern "C" fn restart_nmi() { IGNORE_NMIS-=1; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
