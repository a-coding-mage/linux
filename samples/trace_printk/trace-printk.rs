// SPDX-License-Identifier: GPL-2.0-only
//
// The Linux kernel headers and build-time trace_printk machinery are supplied
// by the surrounding kernel environment.

use core::mem::MaybeUninit;

#[repr(C)]
pub struct irq_work {
    _private: [u8; 0],
}

extern "C" {
    fn init_irq_work(work: *mut irq_work, func: Option<unsafe extern "C" fn(*mut irq_work)>);
    fn irq_work_queue(work: *mut irq_work) -> bool;
    fn irq_work_sync(work: *mut irq_work);
    fn trace_printk(fmt: *const core::ffi::c_char, ...);
}

/* Must not be static to force gcc to consider these non constant */
#[no_mangle]
pub static mut trace_printk_test_global_str: *mut core::ffi::c_char =
    b"This is a dynamic string that will use trace_puts\n\0".as_ptr() as *mut _;

#[no_mangle]
pub static mut trace_printk_test_global_str_irq: *mut core::ffi::c_char =
    b"(irq) This is a dynamic string that will use trace_puts\n\0".as_ptr() as *mut _;

#[no_mangle]
pub static mut trace_printk_test_global_str_fmt: *mut core::ffi::c_char =
    b"%sThis is a %s that will use trace_printk\n\0".as_ptr() as *mut _;

static mut irqwork: MaybeUninit<irq_work> = MaybeUninit::uninit();

unsafe extern "C" fn trace_printk_irq_work(work: *mut irq_work) {
    let _ = work;
    trace_printk(b"(irq) This is a static string that will use trace_bputs\n\0".as_ptr() as *const _);
    trace_printk(trace_printk_test_global_str_irq as *const _);

    trace_printk(
        b"(irq) This is a %s that will use trace_bprintk()\n\0".as_ptr() as *const _,
        b"static string\0".as_ptr(),
    );

    trace_printk(
        trace_printk_test_global_str_fmt as *const _,
        b"(irq) \0".as_ptr(),
        b"dynamic string\0".as_ptr(),
    );
}

unsafe extern "C" fn trace_printk_init() -> core::ffi::c_int {
    init_irq_work(irqwork.as_mut_ptr(), Some(trace_printk_irq_work));

    trace_printk(b"This is a static string that will use trace_bputs\n\0".as_ptr() as *const _);
    trace_printk(trace_printk_test_global_str as *const _);

    /* Kick off printing in irq context */
    irq_work_queue(irqwork.as_mut_ptr());
    irq_work_sync(irqwork.as_mut_ptr());

    trace_printk(
        b"This is a %s that will use trace_bprintk()\n\0".as_ptr() as *const _,
        b"static string\0".as_ptr(),
    );

    trace_printk(
        trace_printk_test_global_str_fmt as *const _,
        b"\0".as_ptr(),
        b"dynamic string\0".as_ptr(),
    );

    0
}

unsafe extern "C" fn trace_printk_exit() {}

// Equivalent kernel registration and module metadata supplied by the build system:
// module_init(trace_printk_init);
// module_exit(trace_printk_exit);
// MODULE_AUTHOR("Steven Rostedt");
// MODULE_DESCRIPTION("trace-printk");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
