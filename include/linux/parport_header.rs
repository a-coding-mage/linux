/* Rust translation of linux/parport.h. External kernel types and functions are
 * intentionally referenced as dependencies supplied by the surrounding tree. */

use core::ffi::{c_char, c_int, c_void};

#[repr(C)] pub struct pardevice;
#[repr(C)] pub struct module;
#[repr(C)] pub struct device;
#[repr(C)] pub struct device_driver;
#[repr(C)] pub struct list_head;
#[repr(C)] pub struct timer_list;
#[repr(C)] pub struct wait_queue_head_t;
#[repr(C)] pub struct semaphore;
#[repr(C)] pub struct spinlock_t;
#[repr(C)] pub struct rwlock_t;
#[repr(C)] pub struct atomic_t;
pub type irqreturn_t = c_int;
pub type parport_device_class = c_int;

#[repr(C)] pub struct pc_parport_state { pub ctr: u32, pub ecr: u32 }
#[repr(C)] pub struct ax_parport_state { pub ctr: u32, pub ecr: u32, pub dcsr: u32 }
#[repr(C)] pub struct amiga_parport_state { pub data: u8, pub datadir: u8, pub status: u8, pub statusdir: u8 }
#[repr(C)] pub struct ip32_parport_state { pub dcr: u32, pub ecr: u32 }

#[repr(C)] pub union parport_state_u {
    pub pc: pc_parport_state,
    pub ax: ax_parport_state,
    pub amiga: amiga_parport_state,
    pub ip32: ip32_parport_state,
    pub misc: *mut c_void,
}
#[repr(C)] pub struct parport_state { pub u: parport_state_u }

pub type WriteData = unsafe extern "C" fn(*mut parport, u8);
pub type ReadData = unsafe extern "C" fn(*mut parport) -> u8;
pub type WriteControl = unsafe extern "C" fn(*mut parport, u8);
pub type ReadControl = unsafe extern "C" fn(*mut parport) -> u8;
pub type FrobControl = unsafe extern "C" fn(*mut parport, u8, u8) -> u8;
pub type PortVoid = unsafe extern "C" fn(*mut parport);
pub type InitState = unsafe extern "C" fn(*mut pardevice, *mut parport_state);
pub type SaveState = unsafe extern "C" fn(*mut parport, *mut parport_state);
pub type BlockWrite = unsafe extern "C" fn(*mut parport, *const c_void, usize, c_int) -> usize;
pub type BlockRead = unsafe extern "C" fn(*mut parport, *mut c_void, usize, c_int) -> usize;

#[repr(C)] pub struct parport_operations {
    pub write_data: Option<WriteData>, pub read_data: Option<ReadData>,
    pub write_control: Option<WriteControl>, pub read_control: Option<ReadControl>,
    pub frob_control: Option<FrobControl>, pub read_status: Option<ReadData>,
    pub enable_irq: Option<PortVoid>, pub disable_irq: Option<PortVoid>,
    pub data_forward: Option<PortVoid>, pub data_reverse: Option<PortVoid>,
    pub init_state: Option<InitState>, pub save_state: Option<SaveState>,
    pub restore_state: Option<SaveState>,
    pub epp_write_data: Option<BlockWrite>, pub epp_read_data: Option<BlockRead>,
    pub epp_write_addr: Option<BlockWrite>, pub epp_read_addr: Option<BlockRead>,
    pub ecp_write_data: Option<BlockWrite>, pub ecp_read_data: Option<BlockRead>,
    pub ecp_write_addr: Option<BlockWrite>, pub compat_write_data: Option<BlockWrite>,
    pub nibble_read_data: Option<BlockRead>, pub byte_read_data: Option<BlockRead>,
    pub owner: *mut module,
}

#[repr(C)] pub struct parport_device_info { pub class: parport_device_class, pub class_name: *const c_char, pub mfr: *const c_char, pub model: *const c_char, pub cmdset: *const c_char, pub description: *const c_char }

#[repr(C)] pub struct pardevice {
    pub name: *const c_char, pub port: *mut parport, pub daisy: c_int,
    pub preempt: Option<unsafe extern "C" fn(*mut c_void) -> c_int>, pub wakeup: Option<unsafe extern "C" fn(*mut c_void)>, pub private: *mut c_void,
    pub irq_func: Option<unsafe extern "C" fn(*mut c_void)>, pub flags: u32, pub next: *mut pardevice, pub prev: *mut pardevice,
    pub dev: device, pub devmodel: bool, pub state: *mut parport_state, pub wait_q: wait_queue_head_t,
    pub time: usize, pub timeslice: usize, pub timeout: isize, pub waiting: usize, pub waitprev: *mut pardevice, pub waitnext: *mut pardevice, pub sysctl_table: *mut c_void,
}

#[repr(C)] pub enum ieee1284_phase { IEEE1284_PH_FWD_DATA, IEEE1284_PH_FWD_IDLE, IEEE1284_PH_TERMINATE, IEEE1284_PH_NEGOTIATION, IEEE1284_PH_HBUSY_DNA, IEEE1284_PH_REV_IDLE, IEEE1284_PH_HBUSY_DAVAIL, IEEE1284_PH_REV_DATA, IEEE1284_PH_ECP_SETUP, IEEE1284_PH_ECP_FWD_TO_REV, IEEE1284_PH_ECP_REV_TO_FWD, IEEE1284_PH_ECP_DIR_UNKNOWN }
#[repr(C)] pub struct ieee1284_info { pub mode: c_int, pub phase: ieee1284_phase, pub irq: semaphore }

#[repr(C)] pub struct parport {
    pub base: usize, pub base_hi: usize, pub size: u32, pub name: *const c_char, pub modes: u32, pub irq: c_int, pub dma: c_int, pub muxport: c_int, pub portnum: c_int,
    pub dev: *mut device, pub bus_dev: device, pub physport: *mut parport, pub devices: *mut pardevice, pub cad: *mut pardevice, pub daisy: c_int, pub muxsel: c_int,
    pub waithead: *mut pardevice, pub waittail: *mut pardevice, pub list: list_head, pub timer: timer_list, pub flags: u32, pub sysctl_table: *mut c_void,
    pub probe_info: [parport_device_info; 5], pub ieee1284: ieee1284_info, pub ops: *mut parport_operations, pub private_data: *mut c_void, pub number: c_int,
    pub pardevice_lock: spinlock_t, pub waitlist_lock: spinlock_t, pub cad_lock: rwlock_t, pub spintime: c_int, pub ref_count: atomic_t, pub devflags: usize,
    pub proc_device: *mut pardevice, pub full_list: list_head, pub slaves: [*mut parport; 3],
}

#[repr(C)] pub struct parport_driver { pub name: *const c_char, pub detach: Option<unsafe extern "C" fn(*mut parport)>, pub match_port: Option<unsafe extern "C" fn(*mut parport)>, pub probe: Option<unsafe extern "C" fn(*mut pardevice) -> c_int>, pub driver: device_driver }
#[repr(C)] pub struct pardev_cb { pub preempt: Option<unsafe extern "C" fn(*mut c_void) -> c_int>, pub wakeup: Option<unsafe extern "C" fn(*mut c_void)>, pub private: *mut c_void, pub irq_func: Option<unsafe extern "C" fn(*mut c_void)>, pub flags: u32 }

pub const DEFAULT_SPIN_TIME: c_int = 500;
pub const PARPORT_DEVPROC_REGISTERED: usize = 0;
pub const PARPORT_ANNOUNCED: usize = 1;
pub const PARPORT_DEV_TRAN: u32 = 0;
pub const PARPORT_DEV_LURK: u32 = 1 << 0;
pub const PARPORT_DEV_EXCL: u32 = 1 << 1;
pub const PARPORT_FLAG_EXCL: u32 = 1 << 1;
pub const PARPORT_INACTIVITY_O_NONBLOCK: c_int = 1;
pub const daisy_dev_name: &[u8] = b"Device ID probe\0";

extern "C" {
    pub fn parport_bus_init() -> c_int; pub fn parport_bus_exit();
    pub fn parport_register_port(base: usize, irq: c_int, dma: c_int, ops: *mut parport_operations) -> *mut parport;
    pub fn parport_announce_port(port: *mut parport); pub fn parport_remove_port(port: *mut parport);
    pub fn __parport_register_driver(driver: *mut parport_driver, owner: *mut module, mod_name: *const c_char) -> c_int;
    pub fn parport_unregister_driver(driver: *mut parport_driver);
    pub fn parport_find_number(n: c_int) -> *mut parport; pub fn parport_find_base(base: usize) -> *mut parport;
    pub fn parport_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
    pub fn parport_get_port(port: *mut parport) -> *mut parport; pub fn parport_put_port(port: *mut parport); pub fn parport_del_port(port: *mut parport);
    pub fn parport_register_dev_model(port: *mut parport, name: *const c_char, cb: *const pardev_cb, cnt: c_int) -> *mut pardevice;
    pub fn parport_unregister_device(dev: *mut pardevice); pub fn parport_claim(dev: *mut pardevice) -> c_int; pub fn parport_claim_or_block(dev: *mut pardevice) -> c_int; pub fn parport_release(dev: *mut pardevice);
    pub fn parport_ieee1284_interrupt(arg: *mut c_void); pub fn parport_negotiate(port: *mut parport, mode: c_int) -> c_int; pub fn parport_write(port: *mut parport, buf: *const c_void, len: usize) -> isize; pub fn parport_read(port: *mut parport, buf: *mut c_void, len: usize) -> isize;
    pub fn parport_set_timeout(dev: *mut pardevice, inactivity: isize) -> isize; pub fn parport_wait_event(port: *mut parport, timeout: isize) -> c_int;
    pub fn parport_wait_peripheral(port: *mut parport, mask: u8, val: u8) -> c_int; pub fn parport_poll_peripheral(port: *mut parport, mask: u8, val: u8, usec: c_int) -> c_int;
    pub fn parport_daisy_init(port: *mut parport) -> c_int; pub fn parport_daisy_fini(port: *mut parport); pub fn parport_open(devnum: c_int, name: *const c_char) -> *mut pardevice; pub fn parport_close(dev: *mut pardevice); pub fn parport_device_id(devnum: c_int, buffer: *mut c_char, len: usize) -> isize; pub fn parport_daisy_deselect_all(port: *mut parport); pub fn parport_daisy_select(port: *mut parport, daisy: c_int, mode: c_int) -> c_int;
    pub fn parport_proc_register(pp: *mut parport) -> c_int; pub fn parport_proc_unregister(pp: *mut parport) -> c_int; pub fn parport_device_proc_register(device: *mut pardevice) -> c_int; pub fn parport_device_proc_unregister(device: *mut pardevice) -> c_int;
    pub static mut parport_default_timeslice: usize; pub static mut parport_default_spintime: c_int;
}

pub unsafe fn parport_yield(dev: *mut pardevice) -> c_int { let timeslip = jiffies().wrapping_sub((*dev).time); if (*dev).port.is_null() || (*(*dev).port).waithead.is_null() || timeslip < (*dev).timeslice { return 0; } parport_release(dev); parport_claim(dev) }
pub unsafe fn parport_yield_blocking(dev: *mut pardevice) -> c_int { let timeslip = jiffies().wrapping_sub((*dev).time); if (*dev).port.is_null() || (*(*dev).port).waithead.is_null() || timeslip < (*dev).timeslice { return 0; } parport_release(dev); parport_claim_or_block(dev) }
pub unsafe fn parport_generic_irq(port: *mut parport) { parport_ieee1284_interrupt(port.cast()); /* read_lock/read_unlock around cad access are supplied by the kernel dependency. */ if !(*port).cad.is_null() { let cad = (*port).cad; if let Some(f) = (*cad).irq_func { f((*cad).private); } } }

extern "C" { fn jiffies() -> usize; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
