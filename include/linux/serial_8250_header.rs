/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of linux/include/linux/serial_8250.h. */

use core::ffi::{c_char, c_int, c_ulong, c_uint, c_void};

/* Dependencies supplied by the surrounding kernel translation. */
#[repr(C)] pub struct uart_port { _private: [u8; 0] }
#[repr(C)] pub struct ktermios { _private: [u8; 0] }
#[repr(C)] pub struct earlycon_device { _private: [u8; 0] }
#[repr(C)] pub struct hrtimer { _private: [u8; 0] }
#[repr(C)] pub struct timer_list { _private: [u8; 0] }
#[repr(C)] pub struct list_head { _private: [u8; 0] }
#[repr(C)] pub struct mctrl_gpios { _private: [u8; 0] }
#[repr(C)] pub struct irq_work { _private: [u8; 0] }
#[repr(C)] pub struct delayed_work { _private: [u8; 0] }
#[repr(C)] pub struct nbcon_write_context { _private: [u8; 0] }
pub type resource_size_t = c_ulong;
pub type upf_t = c_uint;
pub type u16 = core::primitive::u16;
pub type u32 = core::primitive::u32;

#[repr(C)]
pub struct plat_serial8250_port {
    pub iobase: c_ulong,
    pub membase: *mut c_void,
    pub mapbase: resource_size_t,
    pub mapsize: resource_size_t,
    pub uartclk: c_uint,
    pub irq: c_uint,
    pub irqflags: c_ulong,
    pub private_data: *mut c_void,
    pub regshift: u8,
    pub iotype: u8,
    pub hub6: u8,
    pub has_sysrq: u8,
    pub type_: c_uint,
    pub flags: upf_t,
    pub bugs: u16,
    pub serial_in: Option<unsafe extern "C" fn(*mut uart_port, c_uint) -> u32>,
    pub serial_out: Option<unsafe extern "C" fn(*mut uart_port, c_uint, u32)>,
    pub dl_read: Option<unsafe extern "C" fn(*mut uart_8250_port) -> u32>,
    pub dl_write: Option<unsafe extern "C" fn(*mut uart_8250_port, u32)>,
    pub set_termios: Option<unsafe extern "C" fn(*mut uart_port, *mut ktermios, *const ktermios)>,
    pub set_ldisc: Option<unsafe extern "C" fn(*mut uart_port, *mut ktermios)>,
    pub get_mctrl: Option<unsafe extern "C" fn(*mut uart_port) -> c_uint>,
    pub handle_irq: Option<unsafe extern "C" fn(*mut uart_port) -> c_int>,
    pub pm: Option<unsafe extern "C" fn(*mut uart_port, c_uint, c_uint)>,
    pub handle_break: Option<unsafe extern "C" fn(*mut uart_port)>,
}

pub const PLAT8250_DEV_LEGACY: c_int = -1;
pub const PLAT8250_DEV_PLATFORM: c_int = 0;
pub const PLAT8250_DEV_PLATFORM1: c_int = 1;
pub const PLAT8250_DEV_PLATFORM2: c_int = 2;
pub const PLAT8250_DEV_FOURPORT: c_int = 3;
pub const PLAT8250_DEV_ACCENT: c_int = 4;
pub const PLAT8250_DEV_BOCA: c_int = 5;
pub const PLAT8250_DEV_EXAR_ST16C554: c_int = 6;
pub const PLAT8250_DEV_HUB6: c_int = 7;
pub const PLAT8250_DEV_AU1X00: c_int = 8;
pub const PLAT8250_DEV_SM501: c_int = 9;

#[repr(C)] pub struct uart_8250_dma { _private: [u8; 0] }

#[repr(C)]
pub struct uart_8250_ops {
    pub setup_irq: Option<unsafe extern "C" fn(*mut uart_8250_port) -> c_int>,
    pub release_irq: Option<unsafe extern "C" fn(*mut uart_8250_port)>,
    pub setup_timer: Option<unsafe extern "C" fn(*mut uart_8250_port)>,
}

#[repr(C)]
pub struct uart_8250_em485 {
    pub start_tx_timer: hrtimer,
    pub stop_tx_timer: hrtimer,
    pub active_timer: *mut hrtimer,
    pub port: *mut uart_8250_port,
    pub tx_stopped: u32,
}

#[repr(C)]
pub struct uart_8250_port {
    pub port: uart_port,
    pub timer: timer_list,
    pub list: list_head,
    pub capabilities: u32,
    pub bugs: u16,
    pub tx_loadsz: c_uint,
    pub acr: u8, pub fcr: u8, pub ier: u8, pub lcr: u8, pub mcr: u8,
    pub cur_iotype: u8,
    pub rpm_tx_active: c_uint,
    pub canary: u8,
    pub probe: u8,
    pub gpios: *mut mctrl_gpios,
    pub lsr_saved_flags: u16,
    pub lsr_save_mask: u16,
    pub console_line_ended: bool,
    pub console_msr_work_allow: bool,
    pub msr_saved_flags: u8,
    pub console_msr_work: irq_work,
    pub dma: *mut uart_8250_dma,
    pub ops: *const uart_8250_ops,
    pub dl_read: Option<unsafe extern "C" fn(*mut uart_8250_port) -> u32>,
    pub dl_write: Option<unsafe extern "C" fn(*mut uart_8250_port, u32)>,
    pub em485: *mut uart_8250_em485,
    pub rs485_start_tx: Option<unsafe extern "C" fn(*mut uart_8250_port, bool)>,
    pub rs485_stop_tx: Option<unsafe extern "C" fn(*mut uart_8250_port, bool)>,
    pub overrun_backoff: delayed_work,
    pub overrun_backoff_time_ms: u32,
}

pub const UART_PROBE_RSA: u32 = 1 << 0;
pub const LSR_SAVE_FLAGS: u32 = UART_LSR_BRK_ERROR_BITS;
pub const MSR_SAVE_FLAGS: u32 = UART_MSR_ANY_DELTA;
/* Values supplied by serial_reg.h. */
extern "C" { static UART_LSR_BRK_ERROR_BITS: u32; static UART_MSR_ANY_DELTA: u32; }

#[inline]
pub unsafe fn up_to_u8250p(up: *mut uart_port) -> *mut uart_8250_port { up as *mut uart_8250_port }

extern "C" {
    pub fn serial8250_register_8250_port(port: *const uart_8250_port) -> c_int;
    pub fn serial8250_unregister_port(line: c_int); pub fn serial8250_suspend_port(line: c_int); pub fn serial8250_resume_port(line: c_int);
    pub fn early_serial_setup(port: *mut uart_port) -> c_int;
    pub fn early_serial8250_setup(device: *mut earlycon_device, options: *const c_char) -> c_int;
    pub fn serial8250_update_uartclk(port: *mut uart_port, uartclk: c_uint);
    pub fn serial8250_do_set_termios(port: *mut uart_port, termios: *mut ktermios, old: *const ktermios);
    pub fn serial8250_do_set_ldisc(port: *mut uart_port, termios: *mut ktermios);
    pub fn serial8250_do_get_mctrl(port: *mut uart_port) -> c_uint;
    pub fn serial8250_do_startup(port: *mut uart_port) -> c_int; pub fn serial8250_do_shutdown(port: *mut uart_port);
    pub fn serial8250_do_pm(port: *mut uart_port, state: c_uint, oldstate: c_uint);
    pub fn serial8250_do_set_mctrl(port: *mut uart_port, mctrl: c_uint); pub fn serial8250_do_break_ctl(port: *mut uart_port, break_state: c_int);
    pub fn serial8250_do_set_divisor(port: *mut uart_port, baud: c_uint, quot: c_uint);
    pub fn fsl8250_handle_irq(port: *mut uart_port) -> c_int;
    pub fn serial8250_handle_irq_locked(port: *mut uart_port, iir: c_uint); pub fn serial8250_handle_irq(port: *mut uart_port, iir: c_uint) -> c_int;
    pub fn serial8250_rx_chars(up: *mut uart_8250_port, lsr: u16) -> u16; pub fn serial8250_read_char(up: *mut uart_8250_port, lsr: u16);
    pub fn serial8250_tx_chars(up: *mut uart_8250_port); pub fn serial8250_modem_status(up: *mut uart_8250_port) -> c_uint;
    pub fn serial8250_init_port(up: *mut uart_8250_port); pub fn serial8250_set_defaults(up: *mut uart_8250_port);
    pub fn serial8250_console_write(up: *mut uart_8250_port, wctxt: *mut nbcon_write_context, in_atomic: bool);
    pub fn serial8250_console_setup(port: *mut uart_port, options: *mut c_char, probe: bool) -> c_int;
    pub fn serial8250_console_exit(port: *mut uart_port) -> c_int;
    pub fn serial8250_set_isa_configurator(v: Option<unsafe extern "C" fn(c_int, *mut uart_port, *mut u32)>);
}

#[cfg(feature = "CONFIG_SERIAL_8250_RT288X")]
extern "C" { pub fn rt288x_setup(p: *mut uart_port) -> c_int; pub fn au_platform_setup(p: *mut plat_serial8250_port) -> c_int; }
#[cfg(not(feature = "CONFIG_SERIAL_8250_RT288X"))]
#[inline] pub fn rt288x_setup(_: *mut uart_port) -> c_int { -19 }
#[cfg(not(feature = "CONFIG_SERIAL_8250_RT288X"))]
#[inline] pub fn au_platform_setup(_: *mut plat_serial8250_port) -> c_int { -19 }

#[cfg(feature = "CONFIG_SERIAL_8250_CONSOLE")]
extern "C" { pub fn hp300_setup_serial_console() -> c_int; }
#[cfg(not(feature = "CONFIG_SERIAL_8250_CONSOLE"))]
#[inline] pub fn hp300_setup_serial_console() -> c_int { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
