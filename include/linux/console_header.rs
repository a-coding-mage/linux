/*
 * Translated from linux/include/linux/console.h.
 * C preprocessor includes and configuration conditions are represented by
 * Rust declarations and comments; dependent kernel definitions are external.
 */

use core::ffi::c_char;

#[repr(C)]
pub struct vc_data { _private: [u8; 0] }
#[repr(C)]
pub struct console_font_op { _private: [u8; 0] }
#[repr(C)]
pub struct console_font { _private: [u8; 0] }
#[repr(C)]
pub struct module { _private: [u8; 0] }
#[repr(C)]
pub struct tty_struct { _private: [u8; 0] }
#[repr(C)]
pub struct notifier_block { _private: [u8; 0] }
#[repr(C)]
pub struct screen_info { _private: [u8; 0] }
#[repr(C)]
pub struct tty_driver { _private: [u8; 0] }
#[repr(C)]
pub struct printk_buffers { _private: [u8; 0] }
#[repr(C)]
pub struct task_struct { _private: [u8; 0] }
#[repr(C)]
pub struct rcuwait { _private: [u8; 0] }
#[repr(C)]
pub struct irq_work { _private: [u8; 0] }
#[repr(C)]
pub struct hlist_node { _private: [u8; 0] }
#[repr(C)]
pub struct hlist_head { _private: [u8; 0] }
#[repr(C)]
pub struct atomic_t { pub counter: i32 }
#[repr(C)]
pub struct atomic_long_t { pub counter: isize }

pub type u8_ = u8;
pub type u16 = u16;
pub type u32 = u32;
pub type u64 = u64;
pub type uint = u32;
pub type pid_t = i32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct consw {
    pub owner: *mut module,
    pub con_startup: Option<unsafe extern "C" fn() -> *const c_char>,
    pub con_init: Option<unsafe extern "C" fn(*mut vc_data, bool)>,
    pub con_deinit: Option<unsafe extern "C" fn(*mut vc_data)>,
    pub con_clear: Option<unsafe extern "C" fn(*mut vc_data, u32, u32, u32)>,
    pub con_putc: Option<unsafe extern "C" fn(*mut vc_data, u16, u32, u32)>,
    pub con_putcs: Option<unsafe extern "C" fn(*mut vc_data, *const u16, u32, u32, u32)>,
    pub con_cursor: Option<unsafe extern "C" fn(*mut vc_data, bool)>,
    pub con_scroll: Option<unsafe extern "C" fn(*mut vc_data, u32, u32, con_scroll, u32) -> bool>,
    pub con_switch: Option<unsafe extern "C" fn(*mut vc_data) -> bool>,
    pub con_blank: Option<unsafe extern "C" fn(*mut vc_data, vesa_blank_mode, bool) -> bool>,
    pub con_font_set: Option<unsafe extern "C" fn(*mut vc_data, *const console_font, u32, u32) -> i32>,
    pub con_font_get: Option<unsafe extern "C" fn(*mut vc_data, *mut console_font, u32) -> i32>,
    pub con_font_default: Option<unsafe extern "C" fn(*mut vc_data, *mut console_font, *const c_char) -> i32>,
    pub con_resize: Option<unsafe extern "C" fn(*mut vc_data, u32, u32, bool) -> i32>,
    pub con_set_palette: Option<unsafe extern "C" fn(*mut vc_data, *const u8)>,
    pub con_scrolldelta: Option<unsafe extern "C" fn(*mut vc_data, i32)>,
    pub con_set_origin: Option<unsafe extern "C" fn(*mut vc_data) -> bool>,
    pub con_save_screen: Option<unsafe extern "C" fn(*mut vc_data)>,
    pub con_build_attr: Option<unsafe extern "C" fn(*mut vc_data, u8, vc_intensity, bool, bool, bool, bool) -> u8>,
    pub con_invert_region: Option<unsafe extern "C" fn(*mut vc_data, *mut u16, i32)>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum con_scroll { SM_UP, SM_DOWN }
#[repr(C)] pub enum vc_intensity { }
#[repr(C)] pub enum vesa_blank_mode { }

pub static mut conswitchp: *const consw = core::ptr::null();
pub static mut dummy_con: consw = consw { owner: core::ptr::null_mut(), con_startup: None, con_init: None, con_deinit: None, con_clear: None, con_putc: None, con_putcs: None, con_cursor: None, con_scroll: None, con_switch: None, con_blank: None, con_font_set: None, con_font_get: None, con_font_default: None, con_resize: None, con_set_palette: None, con_scrolldelta: None, con_set_origin: None, con_save_screen: None, con_build_attr: None, con_invert_region: None };
pub static mut vga_con: consw = dummy_con;
pub static mut newport_con: consw = dummy_con;

extern "C" {
    pub fn vgacon_register_screen(si: *mut screen_info);
    pub fn con_debug_enter(vc: *mut vc_data);
    pub fn con_debug_leave();
    pub fn lockdep_assert_console_list_lock_held();
    pub fn con_is_bound(csw: *const consw) -> i32;
    pub fn do_unregister_con_driver(csw: *const consw) -> i32;
    pub fn do_take_over_console(sw: *const consw, first: i32, last: i32, deflt: i32) -> i32;
    pub fn give_up_console(sw: *const consw);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum cons_flags {
    CON_PRINTBUFFER = 1 << 0, CON_CONSDEV = 1 << 1, CON_ENABLED = 1 << 2,
    CON_BOOT = 1 << 3, CON_ANYTIME = 1 << 4, CON_BRL = 1 << 5,
    CON_EXTENDED = 1 << 6, CON_SUSPENDED = 1 << 7, CON_NBCON = 1 << 8,
    CON_NBCON_ATOMIC_UNSAFE = 1 << 9,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum nbcon_prio { NBCON_PRIO_NONE = 0, NBCON_PRIO_NORMAL, NBCON_PRIO_EMERGENCY, NBCON_PRIO_PANIC, NBCON_PRIO_MAX }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nbcon_state { pub atom: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nbcon_context {
    pub console: *mut console,
    pub spinwait_max_us: u32,
    pub prio: nbcon_prio,
    pub allow_unsafe_takeover: u32,
    pub backlog: u32,
    pub pbufs: *mut printk_buffers,
    pub seq: u64,
}

#[repr(C)]
pub struct nbcon_write_context {
    pub ctxt: nbcon_context,
    pub outbuf: *mut c_char,
    pub len: u32,
    pub unsafe_takeover: bool,
    #[cfg(CONFIG_PRINTK_EXECUTION_CTX)] pub cpu: i32,
    #[cfg(CONFIG_PRINTK_EXECUTION_CTX)] pub pid: pid_t,
    #[cfg(CONFIG_PRINTK_EXECUTION_CTX)] pub comm: [c_char; 16],
}

#[repr(C)]
pub struct console {
    pub name: [c_char; 16],
    pub write: Option<unsafe extern "C" fn(*mut console, *const c_char, u32)>,
    pub read: Option<unsafe extern "C" fn(*mut console, *mut c_char, u32) -> i32>,
    pub device: Option<unsafe extern "C" fn(*mut console, *mut i32) -> *mut tty_driver>,
    pub unblank: Option<unsafe extern "C" fn()>,
    pub setup: Option<unsafe extern "C" fn(*mut console, *mut c_char) -> i32>,
    pub exit: Option<unsafe extern "C" fn(*mut console) -> i32>,
    pub r#match: Option<unsafe extern "C" fn(*mut console, *mut c_char, i32, *mut c_char) -> i32>,
    pub flags: i16, pub index: i16, pub cflag: i32, pub ispeed: uint, pub ospeed: uint,
    pub seq: u64, pub dropped: usize, pub data: *mut core::ffi::c_void, pub node: hlist_node,
    pub write_atomic: Option<unsafe extern "C" fn(*mut console, *mut nbcon_write_context)>,
    pub write_thread: Option<unsafe extern "C" fn(*mut console, *mut nbcon_write_context)>,
    pub device_lock: Option<unsafe extern "C" fn(*mut console, *mut usize)>,
    pub device_unlock: Option<unsafe extern "C" fn(*mut console, usize)>,
    pub nbcon_state: atomic_t, pub nbcon_seq: atomic_long_t, pub nbcon_device_ctxt: nbcon_context,
    pub nbcon_prev_seq: atomic_long_t, pub pbufs: *mut printk_buffers, pub kthread: *mut task_struct,
    pub rcuwait: rcuwait, pub irq_work: irq_work,
}

#[repr(C)] pub enum con_flush_mode { CONSOLE_FLUSH_PENDING, CONSOLE_REPLAY_ALL }

extern "C" {
    pub fn console_srcu_read_lock_is_held() -> bool;
    pub fn console_srcu_read_lock() -> i32;
    pub fn console_srcu_read_unlock(cookie: i32);
    pub fn console_list_lock(); pub fn console_list_unlock();
    pub static mut console_list: hlist_head;
    pub fn console_is_registered_locked(con: *const console) -> bool;
    pub fn console_is_registered(con: *const console) -> bool;
    pub fn console_srcu_read_flags(con: *const console) -> i16;
    pub fn console_srcu_write_flags(con: *mut console, flags: i16);
    pub fn console_is_usable(con: *mut console, flags: i16, use_atomic: bool) -> bool;
    pub fn nbcon_cpu_emergency_enter();
    pub fn nbcon_cpu_emergency_exit();
    pub fn nbcon_can_proceed(wctxt: *mut nbcon_write_context) -> bool;
    pub fn nbcon_write_context_set_buf(wctxt: *mut nbcon_write_context, buf: *mut c_char, len: u32);
    pub fn nbcon_enter_unsafe(wctxt: *mut nbcon_write_context) -> bool;
    pub fn nbcon_exit_unsafe(wctxt: *mut nbcon_write_context) -> bool;
    pub fn nbcon_reacquire_nobuf(wctxt: *mut nbcon_write_context);
    pub fn nbcon_allow_unsafe_takeover() -> bool;
    pub fn nbcon_kdb_try_acquire(con: *mut console, wctxt: *mut nbcon_write_context) -> bool;
    pub fn nbcon_kdb_release(wctxt: *mut nbcon_write_context);
    pub static mut console_set_on_cmdline: i32;
    pub static mut early_console: *mut console;
    pub fn add_preferred_console(name: *const c_char, idx: i16, options: *mut c_char) -> i32;
    pub fn console_force_preferred_locked(con: *mut console);
    pub fn register_console(con: *mut console); pub fn unregister_console(con: *mut console) -> i32;
    pub fn console_lock(); pub fn console_trylock() -> i32; pub fn console_unlock(); pub fn console_unblank();
    pub fn console_flush_on_panic(mode: con_flush_mode); pub fn console_device(index: *mut i32) -> *mut tty_driver;
    pub fn console_suspend(con: *mut console); pub fn console_resume(con: *mut console); pub fn is_console_locked() -> i32;
    pub fn braille_register_console(con: *mut console, index: i32, console_options: *mut c_char, braille_options: *mut c_char) -> i32;
    pub fn braille_unregister_console(con: *mut console) -> i32;
    pub fn console_suspend_all(); pub fn console_resume_all();
    pub fn console_sysfs_notify();
    pub fn vcs_make_sysfs(index: i32); pub fn vcs_remove_sysfs(index: i32);
    pub static mut ignore_console_lock_warning: atomic_t;
    pub fn console_init();
    pub fn dummycon_register_output_notifier(nb: *mut notifier_block);
    pub fn dummycon_unregister_output_notifier(nb: *mut notifier_block);
}

// The CONFIG_* branches in the original header select external declarations
// or empty inline stubs. Their build-time intent is retained above.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
