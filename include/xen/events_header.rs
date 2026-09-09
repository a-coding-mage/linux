/* SPDX-License-Identifier: GPL-2.0 */

/* Declarations translated from xen/events.h.  C header includes and
 * build-time configuration are supplied by the surrounding translation. */

#[repr(C)]
pub struct xenbus_device {
    _private: [u8; 0],
}

extern "C" {
    pub fn xen_evtchn_nr_channels() -> ::core::ffi::c_uint;

    pub fn bind_evtchn_to_irq(evtchn: evtchn_port_t) -> ::core::ffi::c_int;
    pub fn bind_evtchn_to_irq_lateeoi(evtchn: evtchn_port_t) -> ::core::ffi::c_int;
    pub fn bind_evtchn_to_irqhandler(
        evtchn: evtchn_port_t,
        handler: irq_handler_t,
        irqflags: ::core::ffi::c_ulong,
        devname: *const ::core::ffi::c_char,
        dev_id: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    pub fn bind_evtchn_to_irqhandler_lateeoi(
        evtchn: evtchn_port_t,
        handler: irq_handler_t,
        irqflags: ::core::ffi::c_ulong,
        devname: *const ::core::ffi::c_char,
        dev_id: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    pub fn bind_virq_to_irq(
        virq: ::core::ffi::c_uint,
        cpu: ::core::ffi::c_uint,
        percpu: bool,
    ) -> ::core::ffi::c_int;
    pub fn bind_virq_to_irqhandler(
        virq: ::core::ffi::c_uint,
        cpu: ::core::ffi::c_uint,
        handler: irq_handler_t,
        irqflags: ::core::ffi::c_ulong,
        devname: *const ::core::ffi::c_char,
        dev_id: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    pub fn bind_ipi_to_irqhandler(
        ipi: ipi_vector,
        cpu: ::core::ffi::c_uint,
        handler: irq_handler_t,
        irqflags: ::core::ffi::c_ulong,
        devname: *const ::core::ffi::c_char,
        dev_id: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    pub fn bind_interdomain_evtchn_to_irq_lateeoi(
        dev: *mut xenbus_device,
        remote_port: evtchn_port_t,
    ) -> ::core::ffi::c_int;
    pub fn bind_interdomain_evtchn_to_irqhandler_lateeoi(
        dev: *mut xenbus_device,
        remote_port: evtchn_port_t,
        handler: irq_handler_t,
        irqflags: ::core::ffi::c_ulong,
        devname: *const ::core::ffi::c_char,
        dev_id: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;

    pub fn unbind_from_irqhandler(irq: ::core::ffi::c_uint, dev_id: *mut ::core::ffi::c_void);
    pub fn xen_irq_lateeoi(irq: ::core::ffi::c_uint, eoi_flags: ::core::ffi::c_uint);

    pub fn xen_set_irq_priority(
        irq: ::core::ffi::c_uint,
        priority: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    pub fn evtchn_make_refcounted(evtchn: evtchn_port_t, is_static: bool) -> ::core::ffi::c_int;
    pub fn evtchn_get(evtchn: evtchn_port_t) -> ::core::ffi::c_int;
    pub fn evtchn_put(evtchn: evtchn_port_t);

    pub fn xen_send_IPI_one(cpu: ::core::ffi::c_uint, vector: ipi_vector);
    pub fn rebind_evtchn_irq(evtchn: evtchn_port_t, irq: ::core::ffi::c_int);
    pub fn notify_remote_via_irq(irq: ::core::ffi::c_int);
    pub fn xen_irq_resume();
    pub fn xen_clear_irq_pending(irq: ::core::ffi::c_int);
    pub fn xen_test_irq_pending(irq: ::core::ffi::c_int) -> bool;
    pub fn xen_poll_irq(irq: ::core::ffi::c_int);
    pub fn xen_poll_irq_timeout(irq: ::core::ffi::c_int, timeout: u64);
    pub fn irq_from_evtchn(evtchn: evtchn_port_t) -> ::core::ffi::c_uint;
    pub fn irq_evtchn_from_virq(
        cpu: ::core::ffi::c_uint,
        virq: ::core::ffi::c_uint,
        evtchn: *mut evtchn_port_t,
    ) -> ::core::ffi::c_int;
    pub fn xen_set_callback_via(via: u64) -> ::core::ffi::c_int;
    pub fn xen_evtchn_do_upcall() -> ::core::ffi::c_int;
    pub fn xen_bind_pirq_gsi_to_irq(
        gsi: ::core::ffi::c_uint,
        pirq: ::core::ffi::c_uint,
        shareable: ::core::ffi::c_int,
        name: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    pub fn xen_destroy_irq(irq: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn xen_pirq_from_irq(irq: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn xen_irq_from_gsi(gsi: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn xen_test_irq_shared(irq: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn xen_init_IRQ();
    pub fn xen_debug_interrupt(irq: ::core::ffi::c_int, dev_id: *mut ::core::ffi::c_void) -> irqreturn_t;
}

pub const XEN_EOI_FLAG_SPURIOUS: ::core::ffi::c_uint = 0x00000001;
pub const XEN_IRQ_PRIORITY_MAX: _ = EVTCHN_FIFO_PRIORITY_MAX;
pub const XEN_IRQ_PRIORITY_DEFAULT: _ = EVTCHN_FIFO_PRIORITY_DEFAULT;
pub const XEN_IRQ_PRIORITY_MIN: _ = EVTCHN_FIFO_PRIORITY_MIN;

pub unsafe fn notify_remote_via_evtchn(port: evtchn_port_t) {
    let mut send = evtchn_send { port };
    let _ = HYPERVISOR_event_channel_op(EVTCHNOP_send, &mut send);
}

#[cfg(feature = "CONFIG_PCI_MSI")]
extern "C" {
    pub fn xen_allocate_pirq_msi(dev: *mut pci_dev, msidesc: *mut msi_desc) -> ::core::ffi::c_int;
    pub fn xen_bind_pirq_msi_to_irq(
        dev: *mut pci_dev,
        msidesc: *mut msi_desc,
        pirq: ::core::ffi::c_int,
        nvec: ::core::ffi::c_int,
        name: *const ::core::ffi::c_char,
        domid: domid_t,
    ) -> ::core::ffi::c_int;
}

extern "C" {
    pub static mut xen_fifo_events: bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
