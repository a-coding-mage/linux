// SPDX-License-Identifier: GPL-2.0-only
// Dependencies supplied by the kernel and architecture-specific headers.

const LED_MAX_LENGTH: usize = 8; /* maximum chars written to proc file */

unsafe extern "C" {
    fn get_auxio() -> u8;
    fn set_auxio(on: u8, off: u8);
    fn auxio_set_led(state: u8);
    fn memdup_user_nul(buffer: *const core::ffi::c_char, count: usize) -> *mut core::ffi::c_char;
    fn is_err(ptr: *const core::ffi::c_void) -> bool;
    fn ptr_err(ptr: *const core::ffi::c_void) -> isize;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn strcmp(a: *const core::ffi::c_char, b: *const core::ffi::c_char) -> i32;
    fn simple_strtoul(
        buffer: *const core::ffi::c_char,
        end: *mut *mut core::ffi::c_char,
        base: u32,
    ) -> usize;
    fn timer_setup(timer: *mut timer_list, callback: unsafe extern "C" fn(*mut timer_list), flags: u32);
    fn timer_delete_sync(timer: *mut timer_list);
    fn add_timer(timer: *mut timer_list);
    fn proc_create(
        name: *const core::ffi::c_char,
        mode: u32,
        parent: *mut inode,
        ops: *const proc_ops,
    ) -> *mut core::ffi::c_void;
    fn remove_proc_entry(name: *const core::ffi::c_char, parent: *mut inode);
    fn single_open(file: *mut file, show: unsafe extern "C" fn(*mut seq_file, *mut core::ffi::c_void) -> i32, data: *mut core::ffi::c_void) -> i32;
    fn seq_puts(m: *mut seq_file, s: *const core::ffi::c_char);
    fn seq_read(file: *mut file, p: *mut core::ffi::c_void, pos: *mut loff_t) -> isize;
    fn seq_lseek(file: *mut file, pos: loff_t, whence: i32) -> loff_t;
    fn single_release(inode: *mut inode, file: *mut file) -> i32;
    fn printk(fmt: *const core::ffi::c_char, ...);
}

#[repr(C)]
struct timer_list {
    expires: usize,
    _opaque: [u8; 0],
}

#[repr(C)]
struct seq_file { _opaque: [u8; 0] }
#[repr(C)]
struct inode { _opaque: [u8; 0] }
#[repr(C)]
struct file { _opaque: [u8; 0] }
type loff_t = i64;

#[repr(C)]
struct proc_ops {
    proc_open: Option<unsafe extern "C" fn(*mut inode, *mut file) -> i32>,
    proc_read: Option<unsafe extern "C" fn(*mut file, *mut core::ffi::c_void, *mut loff_t) -> isize>,
    proc_lseek: Option<unsafe extern "C" fn(*mut file, loff_t, i32) -> loff_t>,
    proc_release: Option<unsafe extern "C" fn(*mut inode, *mut file) -> i32>,
    proc_write: Option<unsafe extern "C" fn(*mut file, *const core::ffi::c_char, usize, *mut loff_t) -> isize>,
}

const AUXIO_LED: u8 = 1;
const AUXIO_LED_ON: u8 = 1;
const AUXIO_LED_OFF: u8 = 0;
const HZ: usize = 100;
const FSHIFT: usize = 11;

unsafe extern "C" {
    static mut led_blink_timer_timeout: usize;
    static mut jiffies: usize;
    static mut avenrun: [usize; 3];
}

static mut led_blink_timer: timer_list = timer_list { expires: 0, _opaque: [] };

unsafe extern "C" fn led_toggle() {
    let val = get_auxio();
    let (on, off);

    if val & AUXIO_LED != 0 {
        on = 0;
        off = AUXIO_LED;
    } else {
        on = AUXIO_LED;
        off = 0;
    }

    set_auxio(on, off);
}

unsafe extern "C" fn led_blink(_unused: *mut timer_list) {
    let timeout = led_blink_timer_timeout;

    led_toggle();

    /* reschedule */
    if timeout == 0 { /* blink according to load */
        led_blink_timer.expires = jiffies + ((1 + (avenrun[0] >> FSHIFT)) * HZ);
    } else { /* blink at user specified interval */
        led_blink_timer.expires = jiffies + (timeout * HZ);
    }
    add_timer(&raw mut led_blink_timer);
}

#[cfg(CONFIG_PROC_FS)]
unsafe extern "C" fn led_proc_show(m: *mut seq_file, _v: *mut core::ffi::c_void) -> i32 {
    if get_auxio() & AUXIO_LED != 0 {
        seq_puts(m, c"on\n".as_ptr());
    } else {
        seq_puts(m, c"off\n".as_ptr());
    }
    0
}

#[cfg(CONFIG_PROC_FS)]
unsafe extern "C" fn led_proc_open(_inode: *mut inode, file: *mut file) -> i32 {
    single_open(file, led_proc_show, core::ptr::null_mut())
}

#[cfg(CONFIG_PROC_FS)]
unsafe extern "C" fn led_proc_write(_file: *mut file, buffer: *const core::ffi::c_char, mut count: usize, _ppos: *mut loff_t) -> isize {
    if count > LED_MAX_LENGTH { count = LED_MAX_LENGTH; }
    let buf = memdup_user_nul(buffer, count);
    if is_err(buf.cast()) { return ptr_err(buf.cast()); }

    /* work around \\n when echo'ing into proc */
    if count > 0 && *(buf.add(count - 1) as *const u8) == b'\n' { *buf.add(count - 1) = 0; }

    /* before we change anything we want to stop any running timers,
     * otherwise calls such as on will have no persistent effect
     */
    timer_delete_sync(&raw mut led_blink_timer);

    if strcmp(buf, c"on".as_ptr()) == 0 {
        auxio_set_led(AUXIO_LED_ON);
    } else if strcmp(buf, c"toggle".as_ptr()) == 0 {
        led_toggle();
    } else if *buf as u8 > b'0' && *buf as u8 <= b'9' {
        led_blink_timer_timeout = simple_strtoul(buf, core::ptr::null_mut(), 10);
        led_blink(&raw mut led_blink_timer);
    } else if strcmp(buf, c"load".as_ptr()) == 0 {
        led_blink_timer_timeout = 0;
        led_blink(&raw mut led_blink_timer);
    } else {
        auxio_set_led(AUXIO_LED_OFF);
    }

    kfree(buf.cast());
    count as isize
}

#[cfg(CONFIG_PROC_FS)]
static led_proc_ops: proc_ops = proc_ops {
    proc_open: Some(led_proc_open),
    proc_read: Some(seq_read),
    proc_lseek: Some(seq_lseek),
    proc_release: Some(single_release),
    proc_write: Some(led_proc_write),
};

const LED_VERSION: &core::ffi::CStr = c"0.1";

unsafe extern "C" fn led_init() -> i32 {
    timer_setup(&raw mut led_blink_timer, led_blink, 0);

    #[cfg(CONFIG_PROC_FS)]
    if proc_create(c"led".as_ptr(), 0, core::ptr::null_mut(), &led_proc_ops).is_null() { return -12; }

    printk(c"led: version %s, Lars Kotthoff <metalhead@metalhead.ws>\n".as_ptr(), LED_VERSION.as_ptr());
    0
}

unsafe extern "C" fn led_exit() {
    remove_proc_entry(c"led".as_ptr(), core::ptr::null_mut());
    timer_delete_sync(&raw mut led_blink_timer);
}

// module_init(led_init);
// module_exit(led_exit);
// MODULE_AUTHOR("Lars Kotthoff <metalhead@metalhead.ws>");
// MODULE_DESCRIPTION("Provides control of the front LED on SPARC systems.");
// MODULE_LICENSE("GPL");
// MODULE_VERSION(LED_VERSION);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
