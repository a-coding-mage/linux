/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust source-level translation of linux/drivers/char/serial_core.h. */

/* C headers and configuration conditionals are supplied by the surrounding kernel bindings. */

#[repr(C)] pub struct uart_port { pub lock: spinlock_t, pub iobase: c_ulong, pub membase: *mut u8,
    pub serial_in: Option<unsafe extern "C" fn(*mut uart_port, c_uint) -> u32>,
    pub serial_out: Option<unsafe extern "C" fn(*mut uart_port, c_uint, u32)>,
    pub set_termios: Option<unsafe extern "C" fn(*mut uart_port, *mut ktermios, *const ktermios)>,
    pub set_ldisc: Option<unsafe extern "C" fn(*mut uart_port, *mut ktermios)>,
    pub get_mctrl: Option<unsafe extern "C" fn(*mut uart_port) -> c_uint>,
    pub set_mctrl: Option<unsafe extern "C" fn(*mut uart_port, c_uint)>,
    pub get_divisor: Option<unsafe extern "C" fn(*mut uart_port, c_uint, *mut c_uint) -> c_uint>,
    pub set_divisor: Option<unsafe extern "C" fn(*mut uart_port, c_uint, c_uint, c_uint)>,
    pub get_rxtrig: Option<unsafe extern "C" fn(*mut uart_port) -> c_int>,
    pub set_rxtrig: Option<unsafe extern "C" fn(*mut uart_port, u8) -> c_int>,
    pub startup: Option<unsafe extern "C" fn(*mut uart_port) -> c_int>, pub shutdown: Option<unsafe extern "C" fn(*mut uart_port)>,
    pub throttle: Option<unsafe extern "C" fn(*mut uart_port)>, pub unthrottle: Option<unsafe extern "C" fn(*mut uart_port)>,
    pub break_ctl: Option<unsafe extern "C" fn(*mut uart_port, c_int)>, pub handle_irq: Option<unsafe extern "C" fn(*mut uart_port) -> c_int>,
    pub pm: Option<unsafe extern "C" fn(*mut uart_port, c_uint, c_uint)>, pub handle_break: Option<unsafe extern "C" fn(*mut uart_port)>,
    pub rs485_config: Option<unsafe extern "C" fn(*mut uart_port, *mut ktermios, *mut serial_rs485) -> c_int>,
    pub iso7816_config: Option<unsafe extern "C" fn(*mut uart_port, *mut serial_iso7816) -> c_int>,
    pub ctrl_id: c_uint, pub port_id: c_uint, pub irq: c_uint, pub irqflags: c_ulong, pub uartclk: c_uint, pub fifosize: c_uint,
    pub x_char: u8, pub regshift: u8, pub quirks: u8, pub iotype: uart_iotype, pub read_status_mask: c_uint, pub ignore_status_mask: c_uint,
    pub state: *mut uart_state, pub icount: uart_icount, pub cons: *mut console, pub flags: upf_t, pub status: upstat_t,
    pub hw_stopped: bool, pub cons_flow: bool, pub mctrl: c_uint, pub frame_time: c_uint, pub type_: c_uint,
    pub ops: *const uart_ops, pub custom_divisor: c_uint, pub line: c_uint, pub minor: c_uint, pub mapbase: resource_size_t, pub mapsize: resource_size_t,
    pub dev: *mut device, pub port_dev: *mut serial_port_device, pub sysrq: c_ulong, pub sysrq_ch: u8, pub has_sysrq: u8, pub sysrq_seq: u8,
    pub hub6: u8, pub suspended: u8, pub console_reinit: u8, pub name: *const c_char, pub attr_group: *mut attribute_group,
    pub tty_groups: *const *const attribute_group, pub rs485: serial_rs485, pub rs485_supported: serial_rs485,
    pub rs485_term_gpio: *mut gpio_desc, pub rs485_rx_during_tx_gpio: *mut gpio_desc, pub iso7816: serial_iso7816, pub private_data: *mut c_void }

#[repr(C)] pub struct uart_ops {
 pub tx_empty: Option<unsafe extern "C" fn(*mut uart_port)->c_uint>, pub set_mctrl: Option<unsafe extern "C" fn(*mut uart_port,c_uint)>, pub get_mctrl: Option<unsafe extern "C" fn(*mut uart_port)->c_uint>,
 pub stop_tx: Option<unsafe extern "C" fn(*mut uart_port)>, pub start_tx: Option<unsafe extern "C" fn(*mut uart_port)>, pub throttle: Option<unsafe extern "C" fn(*mut uart_port)>, pub unthrottle: Option<unsafe extern "C" fn(*mut uart_port)>, pub send_xchar: Option<unsafe extern "C" fn(*mut uart_port,c_char)>, pub stop_rx: Option<unsafe extern "C" fn(*mut uart_port)>, pub start_rx: Option<unsafe extern "C" fn(*mut uart_port)>, pub enable_ms: Option<unsafe extern "C" fn(*mut uart_port)>, pub break_ctl: Option<unsafe extern "C" fn(*mut uart_port,c_int)>, pub startup: Option<unsafe extern "C" fn(*mut uart_port)->c_int>, pub shutdown: Option<unsafe extern "C" fn(*mut uart_port)>, pub flush_buffer: Option<unsafe extern "C" fn(*mut uart_port)>, pub set_termios: Option<unsafe extern "C" fn(*mut uart_port,*mut ktermios,*const ktermios)>, pub set_ldisc: Option<unsafe extern "C" fn(*mut uart_port,*mut ktermios)>, pub pm: Option<unsafe extern "C" fn(*mut uart_port,c_uint,c_uint)>, pub type_: Option<unsafe extern "C" fn(*mut uart_port)->*const c_char>, pub release_port: Option<unsafe extern "C" fn(*mut uart_port)>, pub request_port: Option<unsafe extern "C" fn(*mut uart_port)->c_int>, pub config_port: Option<unsafe extern "C" fn(*mut uart_port,c_int)>, pub verify_port: Option<unsafe extern "C" fn(*mut uart_port,*mut serial_struct)->c_int>, pub ioctl: Option<unsafe extern "C" fn(*mut uart_port,c_uint,c_ulong)->c_int> }

#[repr(C)] pub struct uart_icount { pub cts:u32,pub dsr:u32,pub rng:u32,pub dcd:u32,pub rx:u32,pub tx:u32,pub frame:u32,pub overrun:u32,pub parity:u32,pub brk:u32,pub buf_overrun:u32 }
pub type upf_t=u64; pub type upstat_t=c_uint;
#[repr(C)] #[derive(Copy,Clone)] pub enum uart_iotype { UPIO_UNKNOWN=-1, UPIO_PORT=0, UPIO_HUB6=1, UPIO_MEM=2, UPIO_MEM32=3, UPIO_AU=4, UPIO_TSI=5, UPIO_MEM32BE=6, UPIO_MEM16=7, UPIO_BUS=8 }
pub const NO_POLL_CHAR:c_uint=0x00ff0000; pub const UART_CONFIG_TYPE:c_int=1; pub const UART_CONFIG_IRQ:c_int=2; pub const UPQ_NO_TXEN_TEST:u8=1;
pub const UPSTAT_CTS_ENABLE:upstat_t=1<<0; pub const UPSTAT_DCD_ENABLE:upstat_t=1<<1; pub const UPSTAT_AUTORTS:upstat_t=1<<2; pub const UPSTAT_AUTOCTS:upstat_t=1<<3; pub const UPSTAT_AUTOXOFF:upstat_t=1<<4; pub const UPSTAT_SYNC_FIFO:upstat_t=1<<5;
pub const UPF_NO_THRE_TEST:upf_t=1<<19; pub const UPF_AUTO_CTS:upf_t=1<<20; pub const UPF_AUTO_RTS:upf_t=1<<21; pub const UPF_HARD_FLOW:upf_t=UPF_AUTO_CTS|UPF_AUTO_RTS; pub const UPF_SOFT_FLOW:upf_t=1<<22; pub const UPF_CONS_FLOW:upf_t=1<<23; pub const UPF_SHARE_IRQ:upf_t=1<<24; pub const UPF_EXAR_EFR:upf_t=1<<25; pub const UPF_BUG_THRE:upf_t=1<<26; pub const UPF_FIXED_TYPE:upf_t=1<<27; pub const UPF_BOOT_AUTOCONF:upf_t=1<<28; pub const UPF_FIXED_PORT:upf_t=1<<29; pub const UPF_DEAD:upf_t=1<<30; pub const UPF_IOREMAP:upf_t=1<<31; pub const UPF_FULL_PROBE:upf_t=1<<32; pub const __UPF_CHANGE_MASK:u64=0x17fff;

#[repr(C)] pub struct uart_state { pub port: tty_port, pub pm_state: uart_pm_state, pub refcount: atomic_t, pub remove_wait: wait_queue_head_t, pub uart_port:*mut uart_port }
#[repr(C)] pub struct uart_driver { pub owner:*mut module,pub driver_name:*const c_char,pub dev_name:*const c_char,pub major:c_int,pub minor:c_int,pub nr:c_int,pub cons:*mut console,pub state:*mut uart_state,pub tty_driver:*mut tty_driver }
#[repr(C)] pub struct earlycon_device { pub con:*mut console,pub port:uart_port,pub options:[c_char;32],pub baud:c_uint }
#[repr(C)] pub struct earlycon_id { pub name:[c_char;15],pub name_term:c_char,pub compatible:[c_char;128],pub setup:Option<unsafe extern "C" fn(*mut earlycon_device,*const c_char)->c_int> }
#[repr(C)] #[derive(Copy,Clone)] pub enum uart_pm_state { UART_PM_STATE_ON=0, UART_PM_STATE_OFF=3, UART_PM_STATE_UNDEFINED }
pub const UART_XMIT_SIZE:usize=4096; pub const WAKEUP_CHARS:c_uint=256;
#[repr(C)] pub struct UART_TX_FLAGS; pub const UART_TX_NOSTOP:c_uint=1;

extern "C" { pub fn uart_write_wakeup(*mut uart_port); pub fn uart_update_timeout(*mut uart_port,c_uint,c_uint); pub fn uart_get_baud_rate(*mut uart_port,*mut ktermios,*const ktermios,c_uint,c_uint)->c_uint; pub fn uart_get_divisor(*mut uart_port,c_uint)->c_uint; pub fn uart_register_driver(*mut uart_driver)->c_int; pub fn uart_unregister_driver(*mut uart_driver); pub fn uart_add_one_port(*mut uart_driver,*mut uart_port)->c_int; pub fn uart_remove_one_port(*mut uart_driver,*mut uart_port); pub fn uart_suspend_port(*mut uart_driver,*mut uart_port)->c_int; pub fn uart_resume_port(*mut uart_driver,*mut uart_port)->c_int; }

#[inline] pub unsafe fn serial_port_in(up:*mut uart_port, offset:c_int)->c_int { ((*up).serial_in.unwrap())(up,offset as c_uint) as c_int }
#[inline] pub unsafe fn serial_port_out(up:*mut uart_port, offset:c_int, value:c_int) { ((*up).serial_out.unwrap())(up,offset as c_uint,value as u32) }
#[inline] pub unsafe fn uart_tx_stopped(port:*mut uart_port)->c_int { let tty=(*(*port).state).port.tty; if (!tty.is_null() && (*tty).flow.stopped) || (*port).hw_stopped {1} else {0} }
#[inline] pub unsafe fn uart_cts_enabled(p:*mut uart_port)->bool { ((*p).status & UPSTAT_CTS_ENABLE)!=0 }
#[inline] pub unsafe fn uart_softcts_mode(p:*mut uart_port)->bool { ((*p).status & (UPSTAT_CTS_ENABLE|UPSTAT_AUTOCTS))==UPSTAT_CTS_ENABLE }
#[inline] pub unsafe fn uart_set_cons_flow_enabled(p:*mut uart_port,e:bool){(*p).cons_flow=e}
#[inline] pub unsafe fn uart_cons_flow_enabled(p:*const uart_port)->bool{(*p).cons_flow}
#[inline] pub unsafe fn uart_console(_p:*mut uart_port)->bool { false } // CONFIG_SERIAL_CORE_CONSOLE selects the kernel-specific implementation.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
