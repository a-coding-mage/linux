/*
 * The DSP56001 Device Driver, saviour of the Free World(tm)
 *
 * Rust translation of the original Linux driver implementation.
 * Original authors: Fredrik Noring, lars brinkhoff, Tomas Berndtsson.
 * Copyright (C) 1996,1997 Fredrik Noring, lars brinkhoff & Tomas Berndtsson
 * Licensed under the GNU General Public License.
 */

// Linux kernel and Atari architecture headers from the C source provide the
// external types, constants, functions, and globals referenced below.

const DSP56K_DEV_56001: i32 = 0;
const TIMEOUT: i64 = 10;
const MAXIO: i64 = 2048;
const DSP56K_MAX_BINARY_LENGTH: i32 = 3 * 64 * 1024;

#[repr(C)]
struct Dsp56kDevice { in_use: c_ulong, maxio: c_long, timeout: c_long, tx_wsize: c_int, rx_wsize: c_int }

#[repr(C)]
struct Class { name: *const c_char }

static mut dsp56k_mutex: Mutex = Mutex::new();
static mut dsp56k: Dsp56kDevice = Dsp56kDevice { in_use: 0, maxio: 0, timeout: 0, tx_wsize: 0, rx_wsize: 0 };
static dsp56k_class: Class = Class { name: b"dsp56k\0".as_ptr() as *const c_char };

macro_rules! dsp56k_tx_int_on { () => { unsafe { dsp56k_host_interface.icr |= DSP56K_ICR_TREQ; } } }
macro_rules! dsp56k_rx_int_on { () => { unsafe { dsp56k_host_interface.icr |= DSP56K_ICR_RREQ; } } }
macro_rules! dsp56k_tx_int_off { () => { unsafe { dsp56k_host_interface.icr &= !DSP56K_ICR_TREQ; } } }
macro_rules! dsp56k_rx_int_off { () => { unsafe { dsp56k_host_interface.icr &= !DSP56K_ICR_RREQ; } } }

unsafe fn dsp56k_reset() -> c_int {
    let mut status: u8;
    sound_ym.rd_data_reg_sel = 14;
    status = sound_ym.rd_data_reg_sel & 0xef;
    sound_ym.wd_data = status;
    sound_ym.wd_data = status | 0x10;
    udelay(10);
    sound_ym.rd_data_reg_sel = 14;
    sound_ym.wd_data = sound_ym.rd_data_reg_sel & 0xef;
    0
}

unsafe fn dsp56k_upload(mut bin: *mut u8, len: c_int) -> c_int {
    let mut pdev: *mut PlatformDevice;
    let mut fw: *const Firmware = core::ptr::null();
    let fw_name = b"dsp56k/bootstrap.bin\0";
    let mut err: c_int;
    let mut i: usize;
    dsp56k_reset();
    pdev = platform_device_register_simple(b"dsp56k\0".as_ptr() as *const c_char, 0, core::ptr::null_mut(), 0);
    if is_err(pdev) {
        printk(b"Failed to register device for \"%s\"\n\0".as_ptr() as *const c_char, fw_name.as_ptr());
        return -EINVAL;
    }
    err = request_firmware(&mut fw, fw_name.as_ptr() as *const c_char, (*pdev).dev());
    platform_device_unregister(pdev);
    if err != 0 { printk(b"Failed to load image \"%s\" err %d\n\0".as_ptr() as *const c_char, fw_name.as_ptr(), err); return err; }
    if (*fw).size % 3 != 0 { printk(b"Bogus length %d in image \"%s\"\n\0".as_ptr() as *const c_char, (*fw).size, fw_name.as_ptr()); release_firmware(fw); return -EINVAL; }
    i = 0;
    while i < (*fw).size as usize { dsp56k_host_interface.data.b[1] = *(*fw).data.add(i); dsp56k_host_interface.data.b[2] = *(*fw).data.add(i + 1); dsp56k_host_interface.data.b[3] = *(*fw).data.add(i + 2); i += 3; }
    release_firmware(fw);
    while i < 512 { dsp56k_host_interface.data.b[1] = 0; dsp56k_host_interface.data.b[2] = 0; dsp56k_host_interface.data.b[3] = 0; i += 1; }
    i = 0;
    while i < len as usize { tx_wait!(10); get_user!(dsp56k_host_interface.data.b[1], bin); bin = bin.add(1); get_user!(dsp56k_host_interface.data.b[2], bin); bin = bin.add(1); get_user!(dsp56k_host_interface.data.b[3], bin); bin = bin.add(1); i += 1; }
    tx_wait!(10); dsp56k_host_interface.data.l = 3; 0
}

unsafe fn dsp56k_read(file: *mut File, buf: *mut u8, mut count: usize, _ppos: *mut LoffT) -> Isize {
    let dev = iminor(file_inode(file)) & 0x0f; if dev != DSP56K_DEV_56001 { printk(b"DSP56k driver: Unknown minor device: %d\n\0".as_ptr() as *const c_char, dev); return -ENXIO as Isize; } if count == 0 { return 0; }
    let mut n: c_long = 0;
    match dsp56k.rx_wsize { 1 => { handshake!(count, dsp56k.maxio, dsp56k.timeout, dsp56k_receive!(), put_user!(dsp56k_host_interface.data.b[3], buf.add(n as usize))); n as Isize }, 2 => { count /= 2; let data = buf as *mut i16; handshake!(count, dsp56k.maxio, dsp56k.timeout, dsp56k_receive!(), put_user!(dsp56k_host_interface.data.w[1], data.add(n as usize))); (2*n) as Isize }, 3 => { count /= 3; handshake!(count, dsp56k.maxio, dsp56k.timeout, dsp56k_receive!(), put_user!(dsp56k_host_interface.data.b[1], buf.add(n as usize))); (3*n) as Isize }, 4 => { count /= 4; let data = buf as *mut c_long; handshake!(count, dsp56k.maxio, dsp56k.timeout, dsp56k_receive!(), put_user!(dsp56k_host_interface.data.l, data.add(n as usize))); (4*n) as Isize }, _ => -EFAULT as Isize }
}

unsafe fn dsp56k_write(file: *mut File, buf: *const u8, mut count: usize, _ppos: *mut LoffT) -> Isize {
    let dev = iminor(file_inode(file)) & 0x0f; if dev != DSP56K_DEV_56001 { printk(b"DSP56k driver: Unknown minor device: %d\n\0".as_ptr() as *const c_char, dev); return -ENXIO as Isize; } if count == 0 { return 0; }
    let mut n: c_long = 0;
    match dsp56k.tx_wsize { 1 => { handshake!(count, dsp56k.maxio, dsp56k.timeout, dsp56k_transmit!(), get_user!(dsp56k_host_interface.data.b[3], buf.add(n as usize))); n as Isize }, 2 => { count /= 2; let data = buf as *const i16; handshake!(count, dsp56k.maxio, dsp56k.timeout, dsp56k_transmit!(), get_user!(dsp56k_host_interface.data.w[1], data.add(n as usize))); (2*n) as Isize }, 3 => { count /= 3; handshake!(count, dsp56k.maxio, dsp56k.timeout, dsp56k_transmit!(), get_user!(dsp56k_host_interface.data.b[1], buf.add(n as usize))); (3*n) as Isize }, 4 => { count /= 4; let data = buf as *const c_long; handshake!(count, dsp56k.maxio, dsp56k.timeout, dsp56k_transmit!(), get_user!(dsp56k_host_interface.data.l, data.add(n as usize))); (4*n) as Isize }, _ => -EFAULT as Isize }
}

unsafe fn dsp56k_ioctl(file: *mut File, cmd: c_uint, arg: c_ulong) -> c_long {
    let dev = iminor(file_inode(file)) & 0x0f; if dev != DSP56K_DEV_56001 { printk(b"DSP56k driver: Unknown minor device: %d\n\0".as_ptr() as *const c_char, dev); return -ENXIO; }
    match cmd { DSP56K_UPLOAD => { let binary = arg as *mut Dsp56kUpload; let len = (*binary).len; let bin = (*binary).bin; if len <= 0 || len > DSP56K_MAX_BINARY_LENGTH { return -EINVAL; } mutex_lock(&mut dsp56k_mutex); let r = dsp56k_upload(bin, len); mutex_unlock(&mut dsp56k_mutex); if r < 0 { return r as c_long; } }, DSP56K_SET_TX_WSIZE => { if arg > 4 || arg < 1 { return -EINVAL; } mutex_lock(&mut dsp56k_mutex); dsp56k.tx_wsize = arg as c_int; mutex_unlock(&mut dsp56k_mutex); }, DSP56K_SET_RX_WSIZE => { if arg > 4 || arg < 1 { return -EINVAL; } mutex_lock(&mut dsp56k_mutex); dsp56k.rx_wsize = arg as c_int; mutex_unlock(&mut dsp56k_mutex); }, DSP56K_HOST_CMD => { if arg > 31 { return -EINVAL; } mutex_lock(&mut dsp56k_mutex); dsp56k_host_interface.cvr = ((arg & DSP56K_CVR_HV_MASK as c_ulong) | DSP56K_CVR_HC as c_ulong) as u8; mutex_unlock(&mut dsp56k_mutex); }, _ => return -EINVAL }
    0
}

unsafe fn dsp56k_open(inode: *mut Inode, _file: *mut File) -> c_int { let dev = iminor(inode) & 0x0f; let mut ret = 0; mutex_lock(&mut dsp56k_mutex); if dev == DSP56K_DEV_56001 { if test_and_set_bit(0, &mut dsp56k.in_use) { ret = -EBUSY; } else { dsp56k.timeout = TIMEOUT; dsp56k.maxio = MAXIO; dsp56k.rx_wsize = 4; dsp56k.tx_wsize = 4; dsp56k_tx_int_off!(); dsp56k_rx_int_off!(); dsp56k_host_interface.icr &= !DSP56K_ICR_HF0; dsp56k_host_interface.icr &= !DSP56K_ICR_HF1; } } else { ret = -ENODEV; } mutex_unlock(&mut dsp56k_mutex); ret }
unsafe fn dsp56k_release(inode: *mut Inode, _file: *mut File) -> c_int { let dev = iminor(inode) & 0x0f; if dev == DSP56K_DEV_56001 { clear_bit(0, &mut dsp56k.in_use); 0 } else { printk(b"DSP56k driver: Unknown minor device: %d\n\0".as_ptr() as *const c_char, dev); -ENXIO } }

// The C file's #if 0 poll implementation is intentionally retained as disabled source-level intent.
// module_init(dsp56k_init_driver), module_exit(dsp56k_cleanup_driver), and module metadata are
// represented by the declarations below; kernel registration remains an external dependency.
unsafe fn dsp56k_init_driver() -> c_int { if !MACH_IS_ATARI || !ATARIHW_PRESENT(DSP56K) { printk(b"DSP56k driver: Hardware not present\n\0".as_ptr() as *const c_char); return -ENODEV; } if register_chrdev(DSP56K_MAJOR, b"dsp56k\0".as_ptr() as *const c_char, &dsp56k_fops) != 0 { printk(b"DSP56k driver: Unable to register driver\n\0".as_ptr() as *const c_char); return -ENODEV; } let err = class_register(&dsp56k_class); if err != 0 { unregister_chrdev(DSP56K_MAJOR, b"dsp56k\0".as_ptr() as *const c_char); return err; } device_create(&dsp56k_class, core::ptr::null_mut(), mkdev(DSP56K_MAJOR, 0), core::ptr::null_mut(), b"dsp56k\0".as_ptr() as *const c_char); printk(b"DSP56k driver installed\n\0".as_ptr() as *const c_char); err }
unsafe fn dsp56k_cleanup_driver() { device_destroy(&dsp56k_class, mkdev(DSP56K_MAJOR, 0)); class_unregister(&dsp56k_class); unregister_chrdev(DSP56K_MAJOR, b"dsp56k\0".as_ptr() as *const c_char); }

#[repr(C)] struct FileOperations { owner: *mut core::ffi::c_void, read: Option<unsafe fn(*mut File,*mut u8,usize,*mut LoffT)->Isize>, write: Option<unsafe fn(*mut File,*const u8,usize,*mut LoffT)->Isize>, unlocked_ioctl: Option<unsafe fn(*mut File,c_uint,c_ulong)->c_long>, open: Option<unsafe fn(*mut Inode,*mut File)->c_int>, release: Option<unsafe fn(*mut Inode,*mut File)->c_int>, llseek: Option<unsafe fn()> }
static dsp56k_fops: FileOperations = FileOperations { owner: core::ptr::null_mut(), read: Some(dsp56k_read), write: Some(dsp56k_write), unlocked_ioctl: Some(dsp56k_ioctl), open: Some(dsp56k_open), release: Some(dsp56k_release), llseek: None };

// External kernel/architecture symbols and C-compatible types are supplied by other translation units.
#[allow(non_camel_case_types)] type c_int = i32; type c_uint = u32; type c_long = isize; type c_ulong = usize; type c_char = i8; type Isize = isize; type LooffT = i64;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
