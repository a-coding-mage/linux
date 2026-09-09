// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/drivers/char/ttyprintk.c
 *
 *  Copyright (C) 2010  Samo Pogacnik
 */

/*
 * This pseudo device allows user to make printk messages. It is possible
 * to store "console" messages inline with kernel messages for better analyses
 * of the boot process, for example.
 */

// Kernel dependencies supplied by other translation units.

#[repr(C)]
struct ttyprintk_port {
    port: tty_port,
    spinlock: spinlock_t,
}

static mut tpk_port: ttyprintk_port = ttyprintk_port {
    port: unsafe { core::mem::zeroed() },
    spinlock: unsafe { core::mem::zeroed() },
};

/*
 * Our simple preformatting supports transparent output of (time-stamped)
 * printk messages (also suitable for logging service):
 * - any cr is replaced by nl
 * - adds a ttyprintk source tag in front of each line
 * - too long message is fragmented, with '\\'nl between fragments
 * - TPK_STR_SIZE isn't really the write_room limiting factor, because
 *   it is emptied on the fly during preformatting.
 */
const TPK_STR_SIZE: usize = 508; /* should be bigger then max expected line length */
const TPK_MAX_ROOM: u32 = 4096; /* we could assume 4K for instance */
// TPK_PREFIX is KERN_SOH __stringify(CONFIG_TTY_PRINTK_LEVEL).
const TPK_PREFIX: &[u8] = b"\x01";

static mut tpk_curr: i32 = 0;
static mut tpk_buffer: [u8; TPK_STR_SIZE + 4] = [0; TPK_STR_SIZE + 4];

unsafe fn tpk_flush() {
    if tpk_curr > 0 {
        tpk_buffer[tpk_curr as usize] = b'\0';
        printk(TPK_PREFIX.as_ptr(), tpk_buffer.as_ptr());
        tpk_curr = 0;
    }
}

unsafe fn tpk_printk(buf: *const u8, count: usize) -> isize {
    let mut i = 0usize;
    while i < count {
        if tpk_curr >= TPK_STR_SIZE as i32 {
            /* end of tmp buffer reached: cut the message in two */
            tpk_buffer[tpk_curr as usize] = b'\\';
            tpk_curr += 1;
            tpk_flush();
        }

        match *buf.add(i) {
            b'\r' => {
                tpk_flush();
                if i + 1 < count && *buf.add(i + 1) == b'\n' {
                    i += 1;
                }
            }
            b'\n' => {
                tpk_flush();
            }
            ch => {
                tpk_buffer[tpk_curr as usize] = ch;
                tpk_curr += 1;
            }
        }
        i += 1;
    }
    count as isize
}

/* TTY operations open function. */
unsafe extern "C" fn tpk_open(tty: *mut tty_struct, filp: *mut file) -> i32 {
    (*tty).driver_data = &raw mut tpk_port as *mut _ as *mut core::ffi::c_void;
    tty_port_open(&raw mut tpk_port as *mut _, tty, filp)
}

/* TTY operations close function. */
unsafe extern "C" fn tpk_close(tty: *mut tty_struct, filp: *mut file) {
    let tpkp = (*tty).driver_data as *mut ttyprintk_port;
    tty_port_close(&mut (*tpkp).port, tty, filp);
}

/* TTY operations write function. */
unsafe extern "C" fn tpk_write(tty: *mut tty_struct, buf: *const u8, count: usize) -> isize {
    let tpkp = (*tty).driver_data as *mut ttyprintk_port;
    let mut flags = 0ul;
    spin_lock_irqsave(&mut (*tpkp).spinlock, &mut flags);
    let ret = tpk_printk(buf, count);
    spin_unlock_irqrestore(&mut (*tpkp).spinlock, flags);
    ret
}

/* TTY operations write_room function. */
unsafe extern "C" fn tpk_write_room(_tty: *mut tty_struct) -> u32 { TPK_MAX_ROOM }

/* TTY operations hangup function. */
unsafe extern "C" fn tpk_hangup(tty: *mut tty_struct) {
    let tpkp = (*tty).driver_data as *mut ttyprintk_port;
    tty_port_hangup(&mut (*tpkp).port);
}

/* TTY port operations shutdown function. */
unsafe extern "C" fn tpk_port_shutdown(tport: *mut tty_port) {
    let tpkp = (tport as *mut u8).sub(core::mem::offset_of!(ttyprintk_port, port)) as *mut ttyprintk_port;
    let mut flags = 0ul;
    spin_lock_irqsave(&mut (*tpkp).spinlock, &mut flags);
    tpk_flush();
    spin_unlock_irqrestore(&mut (*tpkp).spinlock, flags);
}

static ttyprintk_ops: tty_operations = tty_operations {
    open: Some(tpk_open), close: Some(tpk_close), write: Some(tpk_write),
    write_room: Some(tpk_write_room), hangup: Some(tpk_hangup),
};
static tpk_port_ops: tty_port_operations = tty_port_operations { shutdown: Some(tpk_port_shutdown) };
static mut ttyprintk_driver: *mut tty_driver = core::ptr::null_mut();

unsafe extern "C" fn ttyprintk_console_device(_c: *mut console, index: *mut i32) -> *mut tty_driver {
    *index = 0;
    ttyprintk_driver
}

static mut ttyprintk_console: console = console { name: b"ttyprintk\0".as_ptr(), device: Some(ttyprintk_console_device) };

unsafe extern "C" fn ttyprintk_init() -> i32 {
    let mut ret: i32;
    spin_lock_init(&mut tpk_port.spinlock);
    ttyprintk_driver = tty_alloc_driver(1, TTY_DRIVER_RESET_TERMIOS | TTY_DRIVER_REAL_RAW | TTY_DRIVER_UNNUMBERED_NODE);
    if is_err(ttyprintk_driver) { return ptr_err(ttyprintk_driver); }
    tty_port_init(&mut tpk_port.port);
    tpk_port.port.ops = &tpk_port_ops;
    (*ttyprintk_driver).driver_name = b"ttyprintk\0".as_ptr();
    (*ttyprintk_driver).name = b"ttyprintk\0".as_ptr();
    (*ttyprintk_driver).major = TTYAUX_MAJOR;
    (*ttyprintk_driver).minor_start = 3;
    (*ttyprintk_driver).type_ = TTY_DRIVER_TYPE_CONSOLE;
    (*ttyprintk_driver).init_termios = tty_std_termios;
    (*ttyprintk_driver).init_termios.c_oflag = OPOST | OCRNL | ONOCR | ONLRET;
    tty_set_operations(ttyprintk_driver, &ttyprintk_ops);
    tty_port_link_device(&mut tpk_port.port, ttyprintk_driver, 0);
    ret = tty_register_driver(ttyprintk_driver);
    if ret < 0 {
        printk(KERN_ERR.as_ptr(), b"Couldn't register ttyprintk driver\n".as_ptr());
        tty_driver_kref_put(ttyprintk_driver);
        tty_port_destroy(&mut tpk_port.port);
        return ret;
    }
    register_console(&mut ttyprintk_console);
    0
}

unsafe extern "C" fn ttyprintk_exit() {
    unregister_console(&mut ttyprintk_console);
    tty_unregister_driver(ttyprintk_driver);
    tty_driver_kref_put(ttyprintk_driver);
    tty_port_destroy(&mut tpk_port.port);
}

// device_initcall(ttyprintk_init);
// module_exit(ttyprintk_exit);
// MODULE_DESCRIPTION("TTY driver to output user messages via printk");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
