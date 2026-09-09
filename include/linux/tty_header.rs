/* SPDX-License-Identifier: GPL-2.0 */
// Translated from linux/tty.h. C includes and header guards are omitted;
// referenced types, constants, and functions are supplied by other modules.

pub const NR_UNIX98_PTY_DEFAULT: usize = 4096;
pub const NR_UNIX98_PTY_RESERVE: usize = 1024;
pub const NR_UNIX98_PTY_MAX: usize = 1usize << MINORBITS;
pub const __DISABLED_CHAR: u8 = 0;

macro_rules! tty_cc { ($name:ident, $idx:ident) => {
    #[inline] pub unsafe fn $name(tty: *const tty_struct) -> u8 {
        (*tty).termios.c_cc[$idx as usize]
    }
}; }
tty_cc!(INTR_CHAR, VINTR); tty_cc!(QUIT_CHAR, VQUIT); tty_cc!(ERASE_CHAR, VERASE);
tty_cc!(KILL_CHAR, VKILL); tty_cc!(EOF_CHAR, VEOF); tty_cc!(TIME_CHAR, VTIME);
tty_cc!(MIN_CHAR, VMIN); tty_cc!(SWTC_CHAR, VSWTC); tty_cc!(START_CHAR, VSTART);
tty_cc!(STOP_CHAR, VSTOP); tty_cc!(SUSP_CHAR, VSUSP); tty_cc!(EOL_CHAR, VEOL);
tty_cc!(REPRINT_CHAR, VREPRINT); tty_cc!(DISCARD_CHAR, VDISCARD);
tty_cc!(WERASE_CHAR, VWERASE); tty_cc!(LNEXT_CHAR, VLNEXT); tty_cc!(EOL2_CHAR, VEOL2);

macro_rules! tty_flag { ($name:ident, $field:ident, $flag:ident) => {
    #[inline] pub unsafe fn $name(tty: *const tty_struct) -> c_ulong {
        (*tty).termios.$field & $flag
    }
}; }
macro_rules! tty_flags { ($($name:ident, $field:ident, $flag:ident);+ $(;)?) => { $(tty_flag!($name, $field, $flag);)+ }; }
tty_flags!(
    I_IGNBRK, c_iflag, IGNBRK; I_BRKINT, c_iflag, BRKINT; I_IGNPAR, c_iflag, IGNPAR;
    I_PARMRK, c_iflag, PARMRK; I_INPCK, c_iflag, INPCK; I_ISTRIP, c_iflag, ISTRIP;
    I_INLCR, c_iflag, INLCR; I_IGNCR, c_iflag, IGNCR; I_ICRNL, c_iflag, ICRNL;
    I_IUCLC, c_iflag, IUCLC; I_IXON, c_iflag, IXON; I_IXANY, c_iflag, IXANY;
    I_IXOFF, c_iflag, IXOFF; I_IMAXBEL, c_iflag, IMAXBEL; I_IUTF8, c_iflag, IUTF8;
    O_OPOST, c_oflag, OPOST; O_OLCUC, c_oflag, OLCUC; O_ONLCR, c_oflag, ONLCR;
    O_OCRNL, c_oflag, OCRNL; O_ONOCR, c_oflag, ONOCR; O_ONLRET, c_oflag, ONLRET;
    O_OFILL, c_oflag, OFILL; O_OFDEL, c_oflag, OFDEL; O_NLDLY, c_oflag, NLDLY;
    O_CRDLY, c_oflag, CRDLY; O_TABDLY, c_oflag, TABDLY; O_BSDLY, c_oflag, BSDLY;
    O_VTDLY, c_oflag, VTDLY; O_FFDLY, c_oflag, FFDLY;
    C_BAUD, c_cflag, CBAUD; C_CSIZE, c_cflag, CSIZE; C_CSTOPB, c_cflag, CSTOPB;
    C_CREAD, c_cflag, CREAD; C_PARENB, c_cflag, PARENB; C_PARODD, c_cflag, PARODD;
    C_HUPCL, c_cflag, HUPCL; C_CLOCAL, c_cflag, CLOCAL; C_CIBAUD, c_cflag, CIBAUD;
    C_CRTSCTS, c_cflag, CRTSCTS; C_CMSPAR, c_cflag, CMSPAR;
    L_ISIG, c_lflag, ISIG; L_ICANON, c_lflag, ICANON; L_XCASE, c_lflag, XCASE;
    L_ECHO, c_lflag, ECHO; L_ECHOE, c_lflag, ECHOE; L_ECHOK, c_lflag, ECHOK;
    L_ECHONL, c_lflag, ECHONL; L_NOFLSH, c_lflag, NOFLSH; L_TOSTOP, c_lflag, TOSTOP;
    L_ECHOCTL, c_lflag, ECHOCTL; L_ECHOPRT, c_lflag, ECHOPRT; L_ECHOKE, c_lflag, ECHOKE;
    L_FLUSHO, c_lflag, FLUSHO; L_PENDIN, c_lflag, PENDIN; L_IEXTEN, c_lflag, IEXTEN;
    L_EXTPROC, c_lflag, EXTPROC;
);

pub enum device {}
pub enum signal_struct {}
pub enum tty_operations {}

#[repr(C)]
pub struct tty_struct {
    pub kref: kref, pub index: i32, pub dev: *mut device, pub driver: *mut tty_driver,
    pub port: *mut tty_port, pub ops: *const tty_operations, pub ldisc: *mut tty_ldisc,
    pub ldisc_sem: ld_semaphore, pub atomic_write_lock: mutex, pub legacy_mutex: mutex,
    pub throttle_mutex: mutex, pub termios_rwsem: rw_semaphore, pub winsize_mutex: mutex,
    pub termios: ktermios, pub termios_locked: ktermios, pub name: [i8; 64],
    pub flags: c_ulong, pub count: i32, pub receive_room: u32, pub winsize: winsize,
    pub flow: tty_flow, pub ctrl: tty_ctrl, pub hw_stopped: bool, pub closing: bool,
    pub flow_change: i32, pub link: *mut tty_struct, pub fasync: *mut fasync_struct,
    pub write_wait: wait_queue_head_t, pub read_wait: wait_queue_head_t,
    pub hangup_work: work_struct, pub disc_data: *mut c_void, pub driver_data: *mut c_void,
    pub files_lock: spinlock_t, pub write_cnt: i32, pub write_buf: *mut u8,
    pub tty_files: list_head, pub sak_work: work_struct,
}
#[repr(C)] pub struct tty_flow { pub lock: spinlock_t, pub stopped: bool, pub tco_stopped: bool }
#[repr(C)] pub struct tty_ctrl { pub pgrp: *mut pid, pub session: *mut pid, pub lock: spinlock_t, pub pktstatus: u8, pub packet: bool }
#[repr(C)] pub struct tty_file_private { pub tty: *mut tty_struct, pub file: *mut file, pub list: list_head }

#[repr(C)] pub enum tty_struct_flags { TTY_THROTTLED, TTY_IO_ERROR, TTY_OTHER_CLOSED, TTY_EXCLUSIVE, TTY_DO_WRITE_WAKEUP, TTY_LDISC_OPEN, TTY_PTY_LOCK, TTY_NO_WRITE_SPLIT, TTY_HUPPED, TTY_HUPPING, TTY_LDISC_CHANGING, TTY_LDISC_HALTED }

#[inline] pub unsafe fn tty_io_nonblock(tty: *mut tty_struct, file: *mut file) -> bool { ((*file).f_flags & O_NONBLOCK) != 0 || test_bit(TTY_LDISC_CHANGING as usize, &(*tty).flags) }
#[inline] pub unsafe fn tty_io_error(tty: *mut tty_struct) -> bool { test_bit(TTY_IO_ERROR as usize, &(*tty).flags) }
#[inline] pub unsafe fn tty_throttled(tty: *mut tty_struct) -> bool { test_bit(TTY_THROTTLED as usize, &(*tty).flags) }

extern "C" {
    pub fn tty_kref_put(tty: *mut tty_struct); pub fn tty_get_pgrp(tty: *mut tty_struct) -> *mut pid;
    pub fn tty_vhangup_self(); pub fn disassociate_ctty(priv_: i32); pub fn tty_devnum(tty: *mut tty_struct) -> dev_t;
    pub fn proc_clear_tty(p: *mut task_struct); pub fn get_current_tty() -> *mut tty_struct;
    pub fn tty_init() -> i32; pub fn tty_name(tty: *const tty_struct) -> *const c_char;
    pub fn tty_kopen_exclusive(device: dev_t) -> *mut tty_struct; pub fn tty_kopen_shared(device: dev_t) -> *mut tty_struct;
    pub fn tty_kclose(tty: *mut tty_struct); pub fn tty_dev_name_to_number(name: *const c_char, number: *mut dev_t) -> i32;
    pub static mut tty_std_termios: ktermios; pub fn vcs_init() -> i32; pub static tty_class: class;
    pub fn tty_driver_name(tty: *const tty_struct) -> *const c_char; pub fn tty_wait_until_sent(tty: *mut tty_struct, timeout: c_long);
    pub fn stop_tty(tty: *mut tty_struct); pub fn start_tty(tty: *mut tty_struct); pub fn tty_write_message(tty: *mut tty_struct, msg: *mut c_char);
    pub fn tty_send_xchar(tty: *mut tty_struct, ch: u8) -> i32; pub fn tty_put_char(tty: *mut tty_struct, c: u8) -> i32;
    pub fn tty_chars_in_buffer(tty: *mut tty_struct) -> u32; pub fn tty_write_room(tty: *mut tty_struct) -> u32; pub fn tty_driver_flush_buffer(tty: *mut tty_struct);
    pub fn tty_unthrottle(tty: *mut tty_struct); pub fn tty_throttle_safe(tty: *mut tty_struct) -> bool; pub fn tty_unthrottle_safe(tty: *mut tty_struct) -> bool;
    pub fn tty_do_resize(tty: *mut tty_struct, ws: *mut winsize) -> i32; pub fn tty_get_icount(tty: *mut tty_struct, icount: *mut serial_icounter_struct) -> i32;
    pub fn tty_get_tiocm(tty: *mut tty_struct) -> i32; pub fn is_current_pgrp_orphaned() -> i32; pub fn tty_hangup(tty: *mut tty_struct); pub fn tty_vhangup(tty: *mut tty_struct);
    pub fn tty_hung_up_p(filp: *mut file) -> i32; pub fn do_SAK(tty: *mut tty_struct); pub fn __do_SAK(tty: *mut tty_struct); pub fn no_tty();
    pub fn tty_termios_baud_rate(termios: *const ktermios) -> speed_t; pub fn tty_termios_encode_baud_rate(termios: *mut ktermios, ibaud: speed_t, obaud: speed_t);
    pub fn tty_encode_baud_rate(tty: *mut tty_struct, ibaud: speed_t, obaud: speed_t); pub fn tty_get_char_size(cflag: u32) -> u8; pub fn tty_get_frame_size(cflag: u32) -> u8;
    pub fn tty_termios_copy_hw(new: *mut ktermios, old: *const ktermios); pub fn tty_termios_hw_change(a: *const ktermios, b: *const ktermios) -> bool; pub fn tty_set_termios(tty: *mut tty_struct, kt: *mut ktermios) -> i32;
    pub fn tty_wakeup(tty: *mut tty_struct); pub fn tty_mode_ioctl(tty: *mut tty_struct, cmd: u32, arg: c_ulong) -> i32; pub fn tty_perform_flush(tty: *mut tty_struct, arg: c_ulong) -> i32;
    pub fn tty_init_dev(driver: *mut tty_driver, idx: i32) -> *mut tty_struct; pub fn tty_release_struct(tty: *mut tty_struct, idx: i32); pub fn tty_init_termios(tty: *mut tty_struct); pub fn tty_save_termios(tty: *mut tty_struct);
    pub fn tty_standard_install(driver: *mut tty_driver, tty: *mut tty_struct) -> i32; pub static mut tty_mutex: mutex;
    pub fn n_tty_inherit_ops(ops: *mut tty_ldisc_ops); pub fn n_tty_init(); pub fn tty_audit_exit(); pub fn tty_audit_fork(sig: *mut signal_struct); pub fn tty_audit_push() -> i32;
    pub fn n_tty_ioctl_helper(tty: *mut tty_struct, cmd: u32, arg: c_ulong) -> i32; pub fn vt_ioctl(tty: *mut tty_struct, cmd: u32, arg: c_ulong) -> i32; pub fn vt_compat_ioctl(tty: *mut tty_struct, cmd: u32, arg: c_ulong) -> c_long;
    pub fn tty_lock(tty: *mut tty_struct); pub fn tty_lock_interruptible(tty: *mut tty_struct) -> i32; pub fn tty_unlock(tty: *mut tty_struct); pub fn tty_lock_slave(tty: *mut tty_struct); pub fn tty_unlock_slave(tty: *mut tty_struct); pub fn tty_set_lock_subclass(tty: *mut tty_struct);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
