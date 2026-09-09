// SPDX-License-Identifier: GPL-2.0-only
/*
 * Implementation of the Xen vTPM device frontend
 *
 * Author:  Daniel De Graaf <dgdegra@tycho.nsa.gov>
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// External Linux/Xen types and functions supplied by the surrounding kernel.
#[repr(C)] pub struct tpm_chip { pub ops: *const tpm_class_ops, pub flags: c_ulong, pub timeout_c: c_ulong, pub dev: device }
#[repr(C)] pub struct device;
#[repr(C)] pub struct xenbus_device { pub dev: device, pub nodename: *const c_char, pub otherend: *const c_char, pub state: xenbus_state }
#[repr(C)] pub struct xenbus_device_id;
#[repr(C)] pub struct vtpm_shared_page { pub state: c_uint, pub length: usize, pub nr_extra_pages: usize, pub extra_pages: [u8; 0] }
#[repr(C)] pub struct wait_queue_head_t { _private: [u8; 0] }
#[repr(C)] pub struct xenbus_transaction { _private: [u8; 0] }
#[repr(C)] pub struct tpm_class_ops {
    pub status: Option<unsafe extern "C" fn(*mut tpm_chip) -> u8>,
    pub recv: Option<unsafe extern "C" fn(*mut tpm_chip, *mut u8, usize) -> c_int>,
    pub send: Option<unsafe extern "C" fn(*mut tpm_chip, *mut u8, usize, usize) -> c_int>,
    pub cancel: Option<unsafe extern "C" fn(*mut tpm_chip)>,
    pub req_complete_mask: u8, pub req_complete_val: u8,
    pub req_canceled: Option<unsafe extern "C" fn(*mut tpm_chip, u8) -> bool>,
}
#[repr(C)] pub struct tpm_header { pub ordinal: u32 }
#[repr(C)] pub struct xenbus_driver {
    pub ids: *const xenbus_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut xenbus_device, *const xenbus_device_id) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut xenbus_device)>,
    pub resume: Option<unsafe extern "C" fn(*mut xenbus_device) -> c_int>,
    pub otherend_changed: Option<unsafe extern "C" fn(*mut xenbus_device, xenbus_state)>,
}
#[repr(C)] pub enum xenbus_state { XenbusStateInitialised, XenbusStateConnected, XenbusStateClosing, XenbusStateClosed }
extern "C" {
    fn dev_get_drvdata(*const device) -> *mut c_void; fn dev_set_drvdata(*mut device, *mut c_void);
    fn wait_event_interruptible_timeout(*mut wait_queue_head_t, bool, c_ulong) -> c_long;
    fn freezing(*mut c_void) -> bool; fn clear_thread_flag(c_int); fn tpm_msleep(c_ulong);
    fn tpm_calc_ordinal_duration(*mut tpm_chip, u32) -> c_ulong; fn notify_remote_via_evtchn(c_uint);
    fn wmb(); fn barrier(); fn wake_up_interruptible(*mut wait_queue_head_t);
    fn tpmm_chip_alloc(*mut device, *const tpm_class_ops) -> *mut tpm_chip; fn init_waitqueue_head(*mut wait_queue_head_t);
    fn xenbus_setup_ring(*mut xenbus_device, c_uint, *mut *mut c_void, c_uint, *mut c_int) -> c_int;
    fn xenbus_alloc_evtchn(*mut xenbus_device, *mut c_uint) -> c_int;
    fn bind_evtchn_to_irqhandler(c_uint, unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, c_ulong, *const c_char, *mut c_void) -> c_int;
    fn xenbus_dev_fatal(*mut xenbus_device, c_int, *const c_char, ...); fn xenbus_transaction_start(*mut xenbus_transaction) -> c_int;
    fn xenbus_printf(xenbus_transaction, *const c_char, *const c_char, *const c_char, ...) -> c_int;
    fn xenbus_transaction_end(xenbus_transaction, c_int) -> c_int; fn xenbus_switch_state(*mut xenbus_device, xenbus_state);
    fn xenbus_dev_error(*mut xenbus_device, c_int, *const c_char, ...); fn xenbus_teardown_ring(*mut *mut c_void, c_uint, *mut c_int);
    fn unbind_from_irqhandler(c_int, *mut c_void); fn kfree(*mut c_void); fn tpm_get_timeouts(*mut tpm_chip) -> c_int;
    fn tpm_chip_register(*mut tpm_chip) -> c_int; fn tpm_chip_unregister(*mut tpm_chip); fn device_unregister(*mut device);
    fn xenbus_frontend_closed(*mut xenbus_device); fn xenbus_read_unsigned(*const c_char, *const c_char, c_uint) -> c_uint;
    fn xenbus_register_frontend(*mut xenbus_driver) -> c_int; fn xenbus_unregister_driver(*mut xenbus_driver);
    fn xen_domain() -> bool; fn xen_has_pv_devices() -> bool;
}
type c_long = isize;
type domid_t = u16; type irqreturn_t = c_int;
const EINVAL: c_int = 22; const ETIME: c_int = 62; const ECANCELED: c_int = 125; const ERESTARTSYS: c_int = 512;
const ENOMEM: c_int = 12; const ENODEV: c_int = 19; const EIO: c_int = 5; const EAGAIN: c_int = 11;
const PAGE_SIZE: usize = 4096; const TPM_CHIP_FLAG_IRQ: c_ulong = 0x1; const TIF_SIGPENDING: c_int = 0;
const TPM_TIMEOUT: c_ulong = 5; const GFP_KERNEL: c_uint = 0;
const VTPM_STATE_IDLE: c_uint = 0; const VTPM_STATE_FINISH: c_uint = 1; const VTPM_STATE_SUBMIT: c_uint = 2; const VTPM_STATE_CANCEL: c_uint = 3;
const IRQ_HANDLED: irqreturn_t = 1;
const VTPM_STATUS_RUNNING: u8 = 0x1; const VTPM_STATUS_IDLE: u8 = 0x2; const VTPM_STATUS_RESULT: u8 = 0x4; const VTPM_STATUS_CANCELED: u8 = 0x8;

#[repr(C)] struct tpm_private { chip: *mut tpm_chip, dev: *mut xenbus_device, shr: *mut vtpm_shared_page, evtchn: c_uint, ring_ref: c_int, backend_id: domid_t, irq: c_int, read_queue: wait_queue_head_t }

unsafe fn wait_for_tpm_stat_cond(chip: *mut tpm_chip, mask: u8, check_cancel: bool, canceled: *mut bool) -> bool {
    let status = ((*(*chip).ops).status.unwrap())(chip); *canceled = false;
    if status & mask == mask { return true; }
    if check_cancel && ((*(*chip).ops).req_canceled.unwrap())(chip, status) { *canceled = true; return true; } false
}
unsafe fn wait_for_tpm_stat(chip: *mut tpm_chip, mask: u8, timeout: c_ulong, queue: *mut wait_queue_head_t, check_cancel: bool) -> c_int {
    if ((*(*chip).ops).status.unwrap())(chip) & mask == mask { return 0; }
    let mut canceled = false; let rc = wait_event_interruptible_timeout(queue, wait_for_tpm_stat_cond(chip, mask, check_cancel, &mut canceled), timeout);
    if rc > 0 { return if canceled { -ECANCELED } else { 0 }; } if rc == -ERESTARTSYS { return -ETIME; } -ETIME
}
unsafe extern "C" fn vtpm_status(chip: *mut tpm_chip) -> u8 { let p = dev_get_drvdata(&(*chip).dev) as *mut tpm_private; match (*(*p).shr).state { VTPM_STATE_IDLE => VTPM_STATUS_IDLE | VTPM_STATUS_CANCELED, VTPM_STATE_FINISH => VTPM_STATUS_IDLE | VTPM_STATUS_RESULT, VTPM_STATE_SUBMIT | VTPM_STATE_CANCEL => VTPM_STATUS_RUNNING, _ => 0 } }
unsafe extern "C" fn vtpm_req_canceled(_: *mut tpm_chip, status: u8) -> bool { status & VTPM_STATUS_CANCELED != 0 }
unsafe extern "C" fn vtpm_cancel(chip: *mut tpm_chip) { let p = dev_get_drvdata(&(*chip).dev) as *mut tpm_private; (*(*p).shr).state = VTPM_STATE_CANCEL; wmb(); notify_remote_via_evtchn((*p).evtchn); }
unsafe fn shr_data_offset(shr: *mut vtpm_shared_page) -> usize { core::mem::size_of::<vtpm_shared_page>() + (*shr).nr_extra_pages * core::mem::size_of::<usize>() }
unsafe extern "C" fn vtpm_send(chip: *mut tpm_chip, buf: *mut u8, _: usize, count: usize) -> c_int { let p = dev_get_drvdata(&(*chip).dev) as *mut tpm_private; let s = (*p).shr; let o = shr_data_offset(s); if o > PAGE_SIZE || o + count > PAGE_SIZE { return -EINVAL; } if wait_for_tpm_stat(chip, VTPM_STATUS_IDLE, (*chip).timeout_c, &mut (*p).read_queue, true) < 0 { vtpm_cancel(chip); return -ETIME; } core::ptr::copy_nonoverlapping(buf, (s as *mut u8).add(o), count); (*s).length = count; barrier(); (*s).state = VTPM_STATE_SUBMIT; wmb(); notify_remote_via_evtchn((*p).evtchn); let ordinal = u32::from_be((*buf.cast::<tpm_header>()).ordinal); if wait_for_tpm_stat(chip, VTPM_STATUS_IDLE, tpm_calc_ordinal_duration(chip, ordinal), &mut (*p).read_queue, true) < 0 { vtpm_cancel(chip); return -ETIME; } 0 }
unsafe extern "C" fn vtpm_recv(chip: *mut tpm_chip, buf: *mut u8, count: usize) -> c_int { let p = dev_get_drvdata(&(*chip).dev) as *mut tpm_private; let s = (*p).shr; let o = shr_data_offset(s); let mut l = (*s).length; if (*s).state == VTPM_STATE_IDLE { return -ECANCELED; } if wait_for_tpm_stat(chip, VTPM_STATUS_RESULT, (*chip).timeout_c, &mut (*p).read_queue, true) < 0 { vtpm_cancel(chip); return -ETIME; } if o > PAGE_SIZE { return -EIO; } if o + l > PAGE_SIZE { l = PAGE_SIZE - o; } if l > count { l = count; } core::ptr::copy_nonoverlapping((s as *const u8).add(o), buf, l); l as c_int }

static TPM_VTPM: tpm_class_ops = tpm_class_ops { status: Some(vtpm_status), recv: Some(vtpm_recv), send: Some(vtpm_send), cancel: Some(vtpm_cancel), req_complete_mask: VTPM_STATUS_IDLE | VTPM_STATUS_RESULT, req_complete_val: VTPM_STATUS_IDLE | VTPM_STATUS_RESULT, req_canceled: Some(vtpm_req_canceled) };

unsafe extern "C" fn tpmif_interrupt(_: c_int, dev_id: *mut c_void) -> irqreturn_t { let p = dev_id as *mut tpm_private; match (*(*p).shr).state { VTPM_STATE_IDLE | VTPM_STATE_FINISH => wake_up_interruptible(&mut (*p).read_queue), _ => {} } IRQ_HANDLED }

unsafe fn setup_chip(dev: *mut device, p: *mut tpm_private) -> c_int { let chip = tpmm_chip_alloc(dev, &TPM_VTPM); if chip.is_null() { return -ENOMEM; } init_waitqueue_head(&mut (*p).read_queue); (*p).chip = chip; dev_set_drvdata(&mut (*chip).dev, p as *mut c_void); 0 }
unsafe fn setup_ring(dev: *mut xenbus_device, p: *mut tpm_private) -> c_int { let mut ring = core::ptr::null_mut(); let mut x = xenbus_transaction { _private: [] }; let mut message: *const c_char = core::ptr::null(); let mut rv = xenbus_setup_ring(dev, GFP_KERNEL, &mut ring, 1, &mut (*p).ring_ref); (*p).shr = ring as *mut vtpm_shared_page; if rv < 0 { return rv; } rv = xenbus_alloc_evtchn(dev, &mut (*p).evtchn); if rv != 0 { return rv; } rv = bind_evtchn_to_irqhandler((*p).evtchn, tpmif_interrupt, 0, b"tpmif\0".as_ptr() as *const c_char, p as *mut c_void); if rv <= 0 { xenbus_dev_fatal(dev, rv, b"allocating TPM irq\0".as_ptr() as *const c_char); return rv; } (*p).irq = rv; loop { rv = xenbus_transaction_start(&mut x); if rv != 0 { return rv; } rv = xenbus_printf(x, (*dev).nodename, b"ring-ref\0".as_ptr() as *const c_char, b"%u\0".as_ptr() as *const c_char, (*p).ring_ref); if rv != 0 { message = b"writing ring-ref\0".as_ptr() as *const c_char; } if rv == 0 { rv = xenbus_printf(x, (*dev).nodename, b"event-channel\0".as_ptr() as *const c_char, b"%u\0".as_ptr() as *const c_char, (*p).evtchn); if rv != 0 { message = b"writing event-channel\0".as_ptr() as *const c_char; } } if rv == 0 { rv = xenbus_printf(x, (*dev).nodename, b"feature-protocol-v2\0".as_ptr() as *const c_char, b"1\0".as_ptr() as *const c_char); if rv != 0 { message = b"writing feature-protocol-v2\0".as_ptr() as *const c_char; } } if !message.is_null() { xenbus_transaction_end(x, 1); xenbus_dev_error(dev, rv, b"%s\0".as_ptr() as *const c_char, message); return rv; } rv = xenbus_transaction_end(x, 0); if rv == -EAGAIN { continue; } if rv != 0 { return rv; } break; } xenbus_switch_state(dev, xenbus_state::XenbusStateInitialised); 0 }
unsafe fn ring_free(p: *mut tpm_private) { if p.is_null() { return; } xenbus_teardown_ring(&mut (*p).shr as *mut _ as *mut *mut c_void, 1, &mut (*p).ring_ref); if (*p).irq != 0 { unbind_from_irqhandler((*p).irq, p as *mut c_void); } kfree(p as *mut c_void); }
unsafe extern "C" fn tpmfront_probe(dev: *mut xenbus_device, _: *const xenbus_device_id) -> c_int { let p = libc_kzalloc(core::mem::size_of::<tpm_private>()) as *mut tpm_private; if p.is_null() { return -ENOMEM; } let mut rv = setup_chip(&mut (*dev).dev, p); if rv != 0 { kfree(p as *mut c_void); return rv; } rv = setup_ring(dev, p); if rv != 0 { ring_free(p); return rv; } tpm_get_timeouts((*p).chip); rv = tpm_chip_register((*p).chip); rv }
unsafe extern "C" fn tpmfront_remove(dev: *mut xenbus_device) { let chip = dev_get_drvdata(&(*dev).dev) as *mut tpm_chip; let p = dev_get_drvdata(&(*chip).dev) as *mut tpm_private; tpm_chip_unregister(chip); ring_free(p); dev_set_drvdata(&mut (*chip).dev, core::ptr::null_mut()); }
unsafe extern "C" fn tpmfront_resume(dev: *mut xenbus_device) -> c_int { tpmfront_remove(dev); tpmfront_probe(dev, core::ptr::null()) }
unsafe extern "C" fn backend_changed(dev: *mut xenbus_device, state: xenbus_state) { match state { xenbus_state::XenbusStateInitialised | xenbus_state::XenbusStateConnected => { if (*dev).state == xenbus_state::XenbusStateConnected { return; } if xenbus_read_unsigned((*dev).otherend, b"feature-protocol-v2\0".as_ptr() as *const c_char, 0) == 0 { return; } xenbus_switch_state(dev, xenbus_state::XenbusStateConnected); }, xenbus_state::XenbusStateClosing | xenbus_state::XenbusStateClosed => { device_unregister(&mut (*dev).dev); xenbus_frontend_closed(dev); }, _ => {} } }
extern "C" { fn libc_kzalloc(usize) -> *mut c_void; }
unsafe extern "C" fn xen_tpmfront_init() -> c_int { if !xen_domain() || !xen_has_pv_devices() { return -ENODEV; } xenbus_register_frontend(&mut TPMFRONT_DRIVER) }
unsafe extern "C" fn xen_tpmfront_exit() { xenbus_unregister_driver(&mut TPMFRONT_DRIVER); }
static mut TPMFRONT_DRIVER: xenbus_driver = xenbus_driver { ids: core::ptr::null(), probe: Some(tpmfront_probe), remove: Some(tpmfront_remove), resume: Some(tpmfront_resume), otherend_changed: Some(backend_changed) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
