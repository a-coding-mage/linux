/* SPDX-License-Identifier: GPL-2.0 */

/* Declarations translated from xen/events.h.  C header includes and
 * build-time configuration are supplied by the surrounding translation. */

#[repr(C)]
pub struct xenbus_device {
    _private: [u8; 0],
}

extern "C" {
    pub fn xen_evtchn_nr_channels() -> ::kernel::ffi::c_uint;

    pub fn bind_evtchn_to_irq(evtchn: evtchn_port_t) -> ::kernel::ffi::c_int;
    pub fn bind_evtchn_to_irq_lateeoi(evtchn: evtchn_port_t) -> ::kernel::ffi::c_int;
    pub fn bind_evtchn_to_irqhandler(
        evtchn: evtchn_port_t,
        handler: irq_handler_t,
        irqflags: ::kernel::ffi::c_ulong,
        devname: *const ::kernel::ffi::c_char,
        dev_id: *mut ::kernel::ffi::c_void,
    ) -> ::kernel::ffi::c_int;
    pub fn bind_evtchn_to_irqhandler_lateeoi(
        evtchn: evtchn_port_t,
        handler: irq_handler_t,
        irqflags: ::kernel::ffi::c_ulong,
        devname: *const ::kernel::ffi::c_char,
        dev_id: *mut ::kernel::ffi::c_void,
    ) -> ::kernel::ffi::c_int;
    pub fn bind_virq_to_irq(
        virq: ::kernel::ffi::c_uint,
        cpu: ::kernel::ffi::c_uint,
        percpu: bool,
    ) -> ::kernel::ffi::c_int;
    pub fn bind_virq_to_irqhandler(
        virq: ::kernel::ffi::c_uint,
        cpu: ::kernel::ffi::c_uint,
        handler: irq_handler_t,
        irqflags: ::kernel::ffi::c_ulong,
        devname: *const ::kernel::ffi::c_char,
        dev_id: *mut ::kernel::ffi::c_void,
    ) -> ::kernel::ffi::c_int;
    pub fn bind_ipi_to_irqhandler(
        ipi: ipi_vector,
        cpu: ::kernel::ffi::c_uint,
        handler: irq_handler_t,
        irqflags: ::kernel::ffi::c_ulong,
        devname: *const ::kernel::ffi::c_char,
        dev_id: *mut ::kernel::ffi::c_void,
    ) -> ::kernel::ffi::c_int;
    pub fn bind_interdomain_evtchn_to_irq_lateeoi(
        dev: *mut xenbus_device,
        remote_port: evtchn_port_t,
    ) -> ::kernel::ffi::c_int;
    pub fn bind_interdomain_evtchn_to_irqhandler_lateeoi(
        dev: *mut xenbus_device,
        remote_port: evtchn_port_t,
        handler: irq_handler_t,
        irqflags: ::kernel::ffi::c_ulong,
        devname: *const ::kernel::ffi::c_char,
        dev_id: *mut ::kernel::ffi::c_void,
    ) -> ::kernel::ffi::c_int;

    pub fn unbind_from_irqhandler(irq: ::kernel::ffi::c_uint, dev_id: *mut ::kernel::ffi::c_void);
    pub fn xen_irq_lateeoi(irq: ::kernel::ffi::c_uint, eoi_flags: ::kernel::ffi::c_uint);

    pub fn xen_set_irq_priority(
        irq: ::kernel::ffi::c_uint,
        priority: ::kernel::ffi::c_uint,
    ) -> ::kernel::ffi::c_int;
    pub fn evtchn_make_refcounted(evtchn: evtchn_port_t, is_static: bool) -> ::kernel::ffi::c_int;
    pub fn evtchn_get(evtchn: evtchn_port_t) -> ::kernel::ffi::c_int;
    pub fn evtchn_put(evtchn: evtchn_port_t);

    pub fn xen_send_IPI_one(cpu: ::kernel::ffi::c_uint, vector: ipi_vector);
    pub fn rebind_evtchn_irq(evtchn: evtchn_port_t, irq: ::kernel::ffi::c_int);
    pub fn notify_remote_via_irq(irq: ::kernel::ffi::c_int);
    pub fn xen_irq_resume();
    pub fn xen_clear_irq_pending(irq: ::kernel::ffi::c_int);
    pub fn xen_test_irq_pending(irq: ::kernel::ffi::c_int) -> bool;
    pub fn xen_poll_irq(irq: ::kernel::ffi::c_int);
    pub fn xen_poll_irq_timeout(irq: ::kernel::ffi::c_int, timeout: u64);
    pub fn irq_from_evtchn(evtchn: evtchn_port_t) -> ::kernel::ffi::c_uint;
    pub fn irq_evtchn_from_virq(
        cpu: ::kernel::ffi::c_uint,
        virq: ::kernel::ffi::c_uint,
        evtchn: *mut evtchn_port_t,
    ) -> ::kernel::ffi::c_int;
    pub fn xen_set_callback_via(via: u64) -> ::kernel::ffi::c_int;
    pub fn xen_evtchn_do_upcall() -> ::kernel::ffi::c_int;
    pub fn xen_bind_pirq_gsi_to_irq(
        gsi: ::kernel::ffi::c_uint,
        pirq: ::kernel::ffi::c_uint,
        shareable: ::kernel::ffi::c_int,
        name: *mut ::kernel::ffi::c_char,
    ) -> ::kernel::ffi::c_int;
    pub fn xen_destroy_irq(irq: ::kernel::ffi::c_int) -> ::kernel::ffi::c_int;
    pub fn xen_pirq_from_irq(irq: ::kernel::ffi::c_uint) -> ::kernel::ffi::c_int;
    pub fn xen_irq_from_gsi(gsi: ::kernel::ffi::c_uint) -> ::kernel::ffi::c_int;
    pub fn xen_test_irq_shared(irq: ::kernel::ffi::c_int) -> ::kernel::ffi::c_int;
    pub fn xen_init_IRQ();
    pub fn xen_debug_interrupt(irq: ::kernel::ffi::c_int, dev_id: *mut ::kernel::ffi::c_void) -> irqreturn_t;
}

pub const XEN_EOI_FLAG_SPURIOUS: ::kernel::ffi::c_uint = 0x00000001;
pub const XEN_IRQ_PRIORITY_MAX: u32 = EVTCHN_FIFO_PRIORITY_MAX;
pub const XEN_IRQ_PRIORITY_DEFAULT: u32 = EVTCHN_FIFO_PRIORITY_DEFAULT;
pub const XEN_IRQ_PRIORITY_MIN: u32 = EVTCHN_FIFO_PRIORITY_MIN;

pub unsafe fn notify_remote_via_evtchn(port: evtchn_port_t) {
    let mut send = evtchn_send { port };
    let _ = HYPERVISOR_event_channel_op(EVTCHNOP_send, &mut send);
}

#[cfg(CONFIG_PCI_MSI)]
extern "C" {
    pub fn xen_allocate_pirq_msi(dev: *mut pci_dev, msidesc: *mut msi_desc) -> ::kernel::ffi::c_int;
    pub fn xen_bind_pirq_msi_to_irq(
        dev: *mut pci_dev,
        msidesc: *mut msi_desc,
        pirq: ::kernel::ffi::c_int,
        nvec: ::kernel::ffi::c_int,
        name: *const ::kernel::ffi::c_char,
        domid: domid_t,
    ) -> ::kernel::ffi::c_int;
}

extern "C" {
    pub static mut xen_fifo_events: bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
