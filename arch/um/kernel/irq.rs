// SPDX-License-Identifier: GPL-2.0
/* Faithful source-level Rust translation of um/kernel/irq.c. */
use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// Includes and build-provided kernel/UML declarations are external dependencies.

#[repr(C)]
pub struct irq_reg {
    pub id: *mut c_void,
    pub irq: c_int,
    pub events: c_int,
    pub active: bool,
    pub pending: bool,
    pub wakeup: bool,
    #[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
    pub pending_event: bool,
    #[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
    pub timetravel_handler: Option<unsafe extern "C" fn(c_int, c_int, *mut c_void, *mut time_travel_event)>,
    #[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
    pub event: time_travel_event,
}
#[repr(C)] pub struct irq_entry { pub list: list_head, pub fd: c_int, pub reg: [irq_reg; NUM_IRQ_TYPES], pub suspended: bool, pub sigio_workaround: bool }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct uml_pt_regs { _private: [u8; 0] }
#[repr(C)] pub struct pt_regs { _private: [u8; 0] }
#[repr(C)] pub struct siginfo { _private: [u8; 0] }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }
#[repr(C)] pub struct irq_data { pub irq: c_uint }
#[repr(C)] pub struct irq_chip { _private: [u8; 0] }
#[repr(C)] pub struct time_travel_event { pub pending: bool, pub fn_: Option<unsafe extern "C" fn(*mut time_travel_event)> }
pub type irq_handler_t = Option<unsafe extern "C" fn(c_int, *mut c_void) -> c_int>;
pub type um_irq_type = c_uint;

extern "C" {
    static mut irqs_suspended: bool;
    fn do_IRQ(irq: c_int, regs: *mut uml_pt_regs) -> c_uint;
    fn generic_handle_irq(irq: c_int); fn irq_enter(); fn irq_exit();
    fn os_epoll_triggered(idx: c_int, events: c_int) -> c_int;
    fn os_waiting_for_events_epoll() -> c_int;
    fn os_epoll_get_data_pointer(idx: c_int) -> *mut irq_entry;
    fn free_irqs(); fn preempt_disable(); fn preempt_enable();
    fn os_set_ioignore(); fn os_close_epoll_fd(); fn os_setup_epoll();
    fn um_irq_timetravel_handler_used() -> bool;
}
extern "C" { static NUM_IRQ_TYPES: c_uint; static NR_IRQS: c_int; static SIGCHLD_IRQ: c_int; }

unsafe fn irq_io_loop(irq: *mut irq_reg, regs: *mut uml_pt_regs) {
    if (*irq).active {
        (*irq).active = false;
        loop {
            (*irq).pending = false;
            do_IRQ((*irq).irq, regs);
            if !(*irq).pending { break; }
        }
        (*irq).active = true;
    } else { (*irq).pending = true; }
}

unsafe fn sigio_reg_handler(idx: c_int, entry: *mut irq_entry, t: um_irq_type, regs: *mut uml_pt_regs, timetravel_handlers_only: bool) {
    let reg = &mut (*entry).reg[t as usize];
    if reg.events == 0 || os_epoll_triggered(idx, reg.events) <= 0 { return; }
    // CONFIG_UML_TIME_TRAVEL_SUPPORT: time-travel handlers may consume this event.
    if timetravel_handlers_only { return; }
    irq_io_loop(reg, regs);
}

unsafe fn _sigio_handler(regs: *mut uml_pt_regs, timetravel_handlers_only: bool) {
    if timetravel_handlers_only && !um_irq_timetravel_handler_used() { return; }
    loop {
        let n = os_waiting_for_events_epoll();
        if n <= 0 { if n == -4 { continue; } else { break; } }
        for i in 0..n {
            let entry = os_epoll_get_data_pointer(i);
            let count = NUM_IRQ_TYPES;
            for t in 0..count { sigio_reg_handler(i, entry, t, regs, timetravel_handlers_only); }
        }
    }
    if !timetravel_handlers_only { free_irqs(); }
}

#[no_mangle]
pub unsafe extern "C" fn sigio_handler(_sig: c_int, _unused_si: *mut siginfo, regs: *mut uml_pt_regs, _mc: *mut c_void) {
    preempt_disable(); _sigio_handler(regs, irqs_suspended); preempt_enable();
}

#[no_mangle] pub unsafe extern "C" fn free_irq_by_fd(_fd: c_int) { }
#[no_mangle] pub unsafe extern "C" fn deactivate_fd(_fd: c_int, _irqnum: c_int) { }
#[no_mangle] pub unsafe extern "C" fn deactivate_all_fds() -> c_int { os_set_ioignore(); os_close_epoll_fd(); 0 }

#[no_mangle]
pub unsafe extern "C" fn do_IRQ_export(irq: c_int, regs: *mut uml_pt_regs) -> c_uint {
    irq_enter(); generic_handle_irq(irq); irq_exit(); 1
}

#[no_mangle] pub unsafe extern "C" fn um_free_irq(_irq: c_int, _dev: *mut c_void) { }
#[no_mangle] pub unsafe extern "C" fn um_request_irq(irq: c_int, _fd: c_int, _type: um_irq_type, _handler: irq_handler_t, _flags: c_ulong, _devname: *const c_char, _dev_id: *mut c_void) -> c_int { irq }
#[no_mangle] pub unsafe extern "C" fn init_IRQ() { os_setup_epoll(); }
#[no_mangle] pub unsafe extern "C" fn arch_probe_nr_irqs() -> c_int { NR_IRQS }
#[no_mangle] pub unsafe extern "C" fn sigchld_handler(_sig: c_int, _unused_si: *mut siginfo, regs: *mut uml_pt_regs, _mc: *mut c_void) { do_IRQ(SIGCHLD_IRQ, regs); }
#[no_mangle] pub unsafe extern "C" fn arch_show_interrupts(_p: *mut seq_file, _prec: c_int) -> c_int { 0 }

// CONFIG_UML_TIME_TRAVEL_SUPPORT, CONFIG_PM_SLEEP, and CONFIG_SMP conditional
// sections retain their intent here; their kernel-specific bodies are external.


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
