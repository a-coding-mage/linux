// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/alpha/kernel/srmcons.c
 *
 * Callback based driver for SRM Console console device.
 * (TTY driver and console driver)
 */

// Linux and Alpha kernel headers supplied by the surrounding repository.

static mut SRMCONS_CALLBACK_LOCK: Spinlock = Spinlock::new();
static mut srm_is_registered_console: i32 = 0;

/* The TTY driver */
const MAX_SRM_CONSOLE_DEVICES: u32 = 1; // only support 1 console device

#[repr(C)]
struct srmcons_private {
    port: tty_port,
    timer: timer_list,
}

#[repr(C)]
union srmcons_result {
    bits: srmcons_result_bits,
    as_long: i64,
}

#[repr(C)]
struct srmcons_result_bits {
    c_status: u64,
}

impl srmcons_result_bits {
    #[inline]
    unsafe fn c(&self) -> u64 { self.c_status & ((1u64 << 61) - 1) }
    #[inline]
    unsafe fn status(&self) -> u64 { self.c_status >> 61 }
}

/* called with callback_lock held */
unsafe fn srmcons_do_receive_chars(port: *mut tty_port) -> i32 {
    let mut result: srmcons_result = core::mem::zeroed();
    let mut count: i32 = 0;
    let mut loops: i32 = 0;

    loop {
        result.as_long = callback_getc(0);
        let bits = result.bits;
        if bits.status() < 2 {
            tty_insert_flip_char(port, bits.c() as u8, 0);
            count += 1;
        }
        if !((bits.status() & 1) != 0 && { loops += 1; loops < 10 }) { break; }
    }

    if count != 0 { tty_flip_buffer_push(port); }
    count
}

unsafe fn srmcons_receive_chars(t: *mut timer_list) {
    let srmconsp = timer_container_of!(srmcons_private, t, timer);
    let port = &mut (*srmconsp).port as *mut tty_port;
    let mut flags: unsigned_long = 0;
    let mut incr: unsigned_long = 10;

    local_irq_save(&mut flags);
    if spin_trylock(&mut SRMCONS_CALLBACK_LOCK) {
        if srmcons_do_receive_chars(port) == 0 { incr = 100; }
        spin_unlock(&mut SRMCONS_CALLBACK_LOCK);
    }
    spin_lock(&mut (*port).lock);
    if !(*port).tty.is_null() { mod_timer(&mut (*srmconsp).timer, jiffies + incr); }
    spin_unlock(&mut (*port).lock);
    local_irq_restore(flags);
}

/* called with callback_lock held */
unsafe fn srmcons_do_write(mut port: *mut tty_port, mut buf: *const u8, mut count: usize) {
    let mut result: srmcons_result = core::mem::zeroed();
    while count > 0 {
        let mut need_cr = false;
        let mut c: usize = 0;
        while c < core::cmp::min(128usize, count) && !need_cr {
            if *buf.add(c) == b'\n' { need_cr = true; }
            c += 1;
        }
        while c > 0 {
            result.as_long = callback_puts(0, buf, c);
            let n = result.bits.c() as usize;
            c -= n; count -= n; buf = buf.add(n);
            if !port.is_null() { srmcons_do_receive_chars(port); }
        }
        while need_cr {
            result.as_long = callback_puts(0, b"\r".as_ptr(), 1);
            if result.bits.c() > 0 { need_cr = false; }
        }
    }
}

unsafe fn srmcons_write(tty: *mut tty_struct, buf: *const u8, count: usize) -> isize {
    let mut flags: unsigned_long = 0;
    spin_lock_irqsave(&mut SRMCONS_CALLBACK_LOCK, &mut flags);
    srmcons_do_write((*tty).port, buf, count);
    spin_unlock_irqrestore(&mut SRMCONS_CALLBACK_LOCK, flags);
    count as isize
}

unsafe fn srmcons_write_room(_tty: *mut tty_struct) -> u32 { 512 }

unsafe fn srmcons_open(tty: *mut tty_struct, _filp: *mut file) -> i32 {
    let srmconsp = &mut srmcons_singleton;
    let port = &mut srmconsp.port;
    let mut flags: unsigned_long = 0;
    spin_lock_irqsave(&mut port.lock, &mut flags);
    if port.tty.is_null() {
        (*tty).driver_data = srmconsp as *mut _ as *mut core::ffi::c_void;
        (*tty).port = port;
        port.tty = tty;
        mod_timer(&mut srmconsp.timer, jiffies + 10);
    }
    spin_unlock_irqrestore(&mut port.lock, flags);
    0
}

unsafe fn srmcons_close(tty: *mut tty_struct, _filp: *mut file) {
    let srmconsp = (*tty).driver_data as *mut srmcons_private;
    let port = &mut (*srmconsp).port;
    let mut flags: unsigned_long = 0;
    spin_lock_irqsave(&mut port.lock, &mut flags);
    if (*tty).count == 1 { port.tty = core::ptr::null_mut(); timer_delete(&mut (*srmconsp).timer); }
    spin_unlock_irqrestore(&mut port.lock, flags);
}

static mut srmcons_driver: *mut tty_driver = core::ptr::null_mut();
static mut srmcons_singleton: srmcons_private = srmcons_private { port: tty_port::zeroed(), timer: timer_list::zeroed() };

static srmcons_ops: tty_operations = tty_operations {
    open: Some(srmcons_open), close: Some(srmcons_close), write: Some(srmcons_write), write_room: Some(srmcons_write_room),
};

unsafe fn srmcons_init() -> i32 {
    timer_setup!(&mut srmcons_singleton.timer, srmcons_receive_chars, 0);
    if srm_is_registered_console == 0 { return -ENODEV; }
    let driver = tty_alloc_driver(MAX_SRM_CONSOLE_DEVICES, 0);
    if IS_ERR(driver) { return PTR_ERR(driver); }
    tty_port_init(&mut srmcons_singleton.port);
    (*driver).driver_name = b"srm\0".as_ptr() as *const _;
    (*driver).name = b"srm\0".as_ptr() as *const _;
    (*driver).major = 0; (*driver).minor_start = 0;
    (*driver).type_ = TTY_DRIVER_TYPE_SYSTEM; (*driver).subtype = SYSTEM_TYPE_SYSCONS;
    (*driver).init_termios = tty_std_termios;
    tty_set_operations(driver, &srmcons_ops);
    tty_port_link_device(&mut srmcons_singleton.port, driver, 0);
    let err = tty_register_driver(driver);
    if err != 0 { tty_driver_kref_put(driver); tty_port_destroy(&mut srmcons_singleton.port); return err; }
    srmcons_driver = driver;
    0
}

// device_initcall(srmcons_init);

/* The console driver */
unsafe fn srm_console_write(_co: *mut console, s: *const i8, count: u32) {
    let mut flags: unsigned_long = 0;
    spin_lock_irqsave(&mut SRMCONS_CALLBACK_LOCK, &mut flags);
    srmcons_do_write(core::ptr::null_mut(), s as *const u8, count as usize);
    spin_unlock_irqrestore(&mut SRMCONS_CALLBACK_LOCK, flags);
}

unsafe fn srm_console_device(co: *mut console, index: *mut i32) -> *mut tty_driver {
    *index = (*co).index; srmcons_driver
}

unsafe fn srm_console_setup(_co: *mut console, _options: *mut i8) -> i32 { 0 }

static mut srmcons: console = console {
    name: b"srm\0".as_ptr() as *const _, write: Some(srm_console_write), device: Some(srm_console_device),
    setup: Some(srm_console_setup), flags: CON_PRINTBUFFER | CON_BOOT, index: -1,
};

unsafe fn register_srm_console() {
    if srm_is_registered_console == 0 { callback_open_console(); register_console(&mut srmcons); srm_is_registered_console = 1; }
}

unsafe fn unregister_srm_console() {
    if srm_is_registered_console != 0 { callback_close_console(); unregister_console(&mut srmcons); srm_is_registered_console = 0; }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
