/* Copyright (C) 2005 - 2008 Jeff Dike <jdike@{linux.intel,addtoit}.com> */

/* Much of this ripped from drivers/char/hw_random.c, see there for other
 * copyright.
 *
 * This software may be used and distributed according to the terms
 * of the GNU General Public License, incorporated herein by reference.
 */
/* C dependencies: linux/sched/signal.h, linux/module.h, linux/fs.h,
 * linux/interrupt.h, linux/miscdevice.h, linux/hw_random.h, linux/delay.h,
 * linux/uaccess.h, init.h, irq_kern.h, and os.h. */

/* core module information */
pub const RNG_MODULE_NAME: &[u8] = b"hw_random\0";

/* Changed at init time, in the non-modular case, and at module load
 * time, in the module case.  Presumably, the module subsystem
 * protects against a module being loaded twice at the same time.
 */
static mut random_fd: i32 = -1;

#[repr(C)]
pub struct hwrng {
    pub name: *const i8,
    pub read: Option<unsafe extern "C" fn(*mut hwrng, *mut core::ffi::c_void, usize, bool) -> i32>,
}

/* Opaque completion object supplied by the kernel headers. */
#[repr(C)]
pub struct completion {
    _private: [u8; 0],
}

static mut hwrng_instance: hwrng = hwrng { name: core::ptr::null(), read: None };
static mut have_data: completion = completion { _private: [] };

extern "C" {
    fn os_read_file(fd: i32, buf: *mut core::ffi::c_void, max: usize) -> i32;
    fn add_sigio_fd(fd: i32);
    fn wait_for_completion_killable(completion: *mut completion) -> i32;
    fn ignore_sigio_fd(fd: i32);
    fn deactivate_fd(fd: i32, irq: i32);
    fn complete(completion: *mut completion);
    fn os_open_file(path: *const i8, flags: i32, mode: i32) -> i32;
    fn um_request_irq(irq: i32, fd: i32, irq_type: i32,
                     handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> i32,
                     flags: i32, name: *const i8, data: *mut core::ffi::c_void) -> i32;
    fn sigio_broken();
    fn hwrng_register(rng: *mut hwrng) -> i32;
    fn hwrng_unregister(rng: *mut hwrng);
    fn os_close_file(fd: i32);
    fn free_irq_by_fd(fd: i32);
}

const EAGAIN: i32 = 11;
const RANDOM_IRQ: i32 = 0;
const IRQ_READ: i32 = 0;

unsafe extern "C" fn rng_dev_read(
    _rng: *mut hwrng,
    buf: *mut core::ffi::c_void,
    max: usize,
    block: bool,
) -> i32 {
    let mut ret: i32;

    loop {
        ret = os_read_file(random_fd, buf, max);
        if block && ret == -EAGAIN {
            add_sigio_fd(random_fd);

            ret = wait_for_completion_killable(&raw mut have_data);

            ignore_sigio_fd(random_fd);
            deactivate_fd(random_fd, RANDOM_IRQ);

            if ret < 0 {
                break;
            }
        } else {
            break;
        }
    }

    if ret != -EAGAIN { ret } else { 0 }
}

unsafe extern "C" fn random_interrupt(_irq: i32, _data: *mut core::ffi::c_void) -> i32 {
    complete(&raw mut have_data);

    1 /* IRQ_HANDLED */
}

/* rng_init - initialize RNG module */
pub unsafe extern "C" fn rng_init() -> i32 {
    let mut err = os_open_file(b"/dev/random\0".as_ptr() as *const i8, 0, 0);
    if err < 0 {
        return err;
    }

    random_fd = err;
    err = um_request_irq(RANDOM_IRQ, random_fd, IRQ_READ, random_interrupt,
                         0, b"random\0".as_ptr() as *const i8, core::ptr::null_mut());
    if err < 0 {
        os_close_file(random_fd);
        random_fd = -1;
        return err;
    }

    sigio_broken();
    hwrng_instance.name = RNG_MODULE_NAME.as_ptr() as *const i8;
    hwrng_instance.read = Some(rng_dev_read);

    err = hwrng_register(&raw mut hwrng_instance);
    if err != 0 {
        /* pr_err(RNG_MODULE_NAME " registering failed (%d)\n", err); */
        os_close_file(random_fd);
        random_fd = -1;
        return err;
    }
    err
}

/* rng_cleanup - shutdown RNG module */
unsafe fn cleanup() {
    free_irq_by_fd(random_fd);
    os_close_file(random_fd);
}

pub unsafe extern "C" fn rng_cleanup() {
    hwrng_unregister(&raw mut hwrng_instance);
    os_close_file(random_fd);
}

/* module_init(rng_init); module_exit(rng_cleanup); __uml_exitcall(cleanup); */
/* MODULE_DESCRIPTION("UML Host Random Number Generator (RNG) driver"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
