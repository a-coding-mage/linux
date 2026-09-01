// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (c) 1998-2002 by Paul Davis <pbd@op.net>
 */

/* Original C dependencies:
 * <linux/io.h>, <linux/init.h>, <linux/time.h>, <linux/wait.h>,
 * <linux/slab.h>, <linux/module.h>, <linux/firmware.h>,
 * <sound/core.h>, <sound/snd_wavefront.h>, <sound/initval.h>
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

/* Control bits for the Load Control Register
 */

const FX_LSB_TRANSFER: c_int = 0x01; /* transfer after DSP LSB byte written */
const FX_MSB_TRANSFER: c_int = 0x02; /* transfer after DSP MSB byte written */
const FX_AUTO_INCR: c_int = 0x04; /* auto-increment DSP address after transfer */

const WAIT_IDLE: c_uint = 0xff;

const EINVAL: c_int = 22;
const EIO: c_int = 5;
const EFAULT: c_int = 14;
const ENODEV: c_int = 19;
const ENOTTY: c_int = 25;

const WFFX_MUTE: c_int = 0;
const WFFX_MEMSET: c_int = 1;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
    pub module: *mut module,
    pub private_data: *mut c_void,
}

#[repr(C)]
pub struct snd_hwdep {
    pub card: *mut snd_card,
}

#[repr(C)]
pub struct file {
    pub private_data: *mut c_void,
}

#[repr(C)]
pub struct snd_wavefront_t {
    pub fx_status: c_ulong,
    pub fx_op: c_ulong,
    pub fx_lcr: c_ulong,
    pub fx_dsp_page: c_ulong,
    pub fx_dsp_addr: c_ulong,
    pub fx_dsp_msb: c_ulong,
    pub fx_dsp_lsb: c_ulong,
    pub card: *mut snd_card,
    pub fx_initialized: c_int,
    pub base: c_ulong,
}

#[repr(C)]
pub struct snd_wavefront_card_t {
    pub wavefront: snd_wavefront_t,
}

#[repr(C)]
pub struct wavefront_fx_info {
    pub request: c_int,
    pub data: [c_ulong; 4],
}

#[repr(C)]
pub struct firmware {
    pub size: usize,
    pub data: *const u8,
}

unsafe extern "C" {
    fn inb(port: c_ulong) -> c_uint;
    fn outb(value: c_uint, port: c_ulong);
    fn try_module_get(module: *mut module) -> c_int;
    fn module_put(module: *mut module);
    fn copy_from_user(to: *mut c_void, from: *const c_void, n: c_ulong) -> c_ulong;
    fn memdup_array_user(src: *const u8, n: c_ulong, size: c_ulong) -> *mut c_ushort;
    fn IS_ERR(ptr: *const c_void) -> c_int;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn kfree(ptr: *const c_void);
    fn request_firmware(
        firmware: *mut *const firmware,
        name: *const c_char,
        device: *mut device,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn snd_BUG_ON(condition: bool) -> c_int;
}

type c_ushort = u16;

unsafe fn wavefront_fx_idle(dev: *mut snd_wavefront_t) -> c_int {
    let mut i: c_int;
    let mut x: c_uint = 0x80;

    i = 0;
    while i < 1000 {
        x = unsafe { inb((*dev).fx_status) };
        if (x & 0x80) == 0 {
            break;
        }
        i += 1;
    }

    if (x & 0x80) != 0 {
        unsafe {
            dev_err(
                (*(*dev).card).dev,
                c"FX device never idle.\n".as_ptr(),
            );
        }
        return 0;
    }

    1
}

unsafe fn wavefront_fx_mute(dev: *mut snd_wavefront_t, onoff: c_int) {
    if unsafe { wavefront_fx_idle(dev) } == 0 {
        return;
    }

    unsafe {
        outb(if onoff != 0 { 0x02 } else { 0x00 }, (*dev).fx_op);
    }
}

unsafe fn wavefront_fx_memset(
    dev: *mut snd_wavefront_t,
    page: c_int,
    addr: c_int,
    cnt: c_int,
    data: *mut c_ushort,
) -> c_int {
    if page < 0 || page > 7 {
        unsafe {
            dev_err(
                (*(*dev).card).dev,
                c"FX memset: page must be >= 0 and <= 7\n".as_ptr(),
            );
        }
        return -EINVAL;
    }

    if addr < 0 || addr > 0x7f {
        unsafe {
            dev_err(
                (*(*dev).card).dev,
                c"FX memset: addr must be >= 0 and <= 7f\n".as_ptr(),
            );
        }
        return -EINVAL;
    }

    if cnt == 1 {
        unsafe {
            outb(FX_LSB_TRANSFER as c_uint, (*dev).fx_lcr);
            outb(page as c_uint, (*dev).fx_dsp_page);
            outb(addr as c_uint, (*dev).fx_dsp_addr);
            outb(((*data.add(0) >> 8) as c_uint), (*dev).fx_dsp_msb);
            outb(((*data.add(0) & 0xff) as c_uint), (*dev).fx_dsp_lsb);

            dev_err(
                (*(*dev).card).dev,
                c"FX: addr %d:%x set to 0x%x\n".as_ptr(),
                page,
                addr,
                *data.add(0) as c_int,
            );
        }
    } else {
        let mut i: c_int;

        unsafe {
            outb((FX_AUTO_INCR | FX_LSB_TRANSFER) as c_uint, (*dev).fx_lcr);
            outb(page as c_uint, (*dev).fx_dsp_page);
            outb(addr as c_uint, (*dev).fx_dsp_addr);
        }

        i = 0;
        while i < cnt {
            unsafe {
                outb(((*data.add(i as usize) >> 8) as c_uint), (*dev).fx_dsp_msb);
                outb(((*data.add(i as usize) & 0xff) as c_uint), (*dev).fx_dsp_lsb);
            }
            if unsafe { wavefront_fx_idle(dev) } == 0 {
                break;
            }
            i += 1;
        }

        if i != cnt {
            unsafe {
                dev_err(
                    (*(*dev).card).dev,
                    c"FX memset (0x%x, 0x%x, 0x%lx, %d) incomplete\n".as_ptr(),
                    page,
                    addr,
                    data as c_ulong,
                    cnt,
                );
            }
            return -EIO;
        }
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_wavefront_fx_detect(dev: *mut snd_wavefront_t) -> c_int {
    /* This is a crude check, but its the best one I have for now.
       Certainly on the Maui and the Tropez, wavefront_fx_idle() will
       report "never idle", which suggests that this test should
       work OK.
    */

    if unsafe { inb((*dev).fx_status) } & 0x80 != 0 {
        unsafe {
            dev_err(
                (*(*dev).card).dev,
                c"Hmm, probably a Maui or Tropez.\n".as_ptr(),
            );
        }
        return -1;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_wavefront_fx_open(hw: *mut snd_hwdep, file: *mut file) -> c_int {
    if unsafe { try_module_get((*(*hw).card).module) } == 0 {
        return -EFAULT;
    }
    unsafe {
        (*file).private_data = hw as *mut c_void;
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_wavefront_fx_release(hw: *mut snd_hwdep, _file: *mut file) -> c_int {
    unsafe {
        module_put((*(*hw).card).module);
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_wavefront_fx_ioctl(
    sdev: *mut snd_hwdep,
    _file: *mut file,
    cmd: c_uint,
    arg: c_ulong,
) -> c_int {
    let card: *mut snd_card;
    let acard: *mut snd_wavefront_card_t;
    let dev: *mut snd_wavefront_t;
    let mut r: wavefront_fx_info = unsafe { core::mem::zeroed() };
    let mut page_data: *mut c_ushort = ptr::null_mut();
    let pd: *mut c_ushort;
    let mut err: c_int = 0;

    let _ = cmd;

    unsafe {
        card = (*sdev).card;
        if snd_BUG_ON(card.is_null()) != 0 {
            return -ENODEV;
        }
        if snd_BUG_ON((*card).private_data.is_null()) != 0 {
            return -ENODEV;
        }

        acard = (*card).private_data as *mut snd_wavefront_card_t;
        dev = &mut (*acard).wavefront;

        if copy_from_user(
            &mut r as *mut wavefront_fx_info as *mut c_void,
            arg as *const c_void,
            size_of::<wavefront_fx_info>() as c_ulong,
        ) != 0
        {
            return -EFAULT;
        }

        match r.request {
            WFFX_MUTE => {
                wavefront_fx_mute(dev, r.data[0] as c_int);
                return -EIO;
            }

            WFFX_MEMSET => {
                if r.data[2] as c_long <= 0 {
                    dev_err(
                        (*(*dev).card).dev,
                        c"cannot write <= 0 bytes to FX\n".as_ptr(),
                    );
                    return -EIO;
                } else if r.data[2] == 1 {
                    pd = &mut r.data[3] as *mut c_ulong as *mut c_ushort;
                } else {
                    if r.data[2] > 256 {
                        dev_err(
                            (*(*dev).card).dev,
                            c"cannot write > 512 bytes to FX\n".as_ptr(),
                        );
                        return -EIO;
                    }
                    page_data = memdup_array_user(
                        r.data[3] as *const u8,
                        r.data[2],
                        size_of::<c_short>() as c_ulong,
                    );
                    if IS_ERR(page_data as *const c_void) != 0 {
                        return PTR_ERR(page_data as *const c_void);
                    }
                    pd = page_data;
                }

                err = wavefront_fx_memset(
                    dev,
                    r.data[0] as c_int, /* page */
                    r.data[1] as c_int, /* addr */
                    r.data[2] as c_int, /* cnt */
                    pd,
                );
                kfree(page_data as *const c_void);
            }

            _ => {
                dev_err(
                    (*(*dev).card).dev,
                    c"FX: ioctl %d not yet supported\n".as_ptr(),
                    r.request,
                );
                return -ENOTTY;
            }
        }
    }
    err
}

type c_short = i16;
type c_long = isize;

/* YSS225 initialization.

   This code was developed using DOSEMU. The Turtle Beach SETUPSND
   utility was run with I/O tracing in DOSEMU enabled, and a reconstruction
   of the port I/O done, using the Yamaha faxback document as a guide
   to add more logic to the code. Its really pretty weird.

   This is the approach of just dumping the whole I/O
   sequence as a series of port/value pairs and a simple loop
   that outputs it.
*/

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_wavefront_fx_start(dev: *mut snd_wavefront_t) -> c_int {
    let mut i: c_uint;
    let err: c_int;
    let mut firmware: *const firmware = ptr::null();

    unsafe {
        if (*dev).fx_initialized != 0 {
            return 0;
        }

        err = request_firmware(
            &mut firmware as *mut *const firmware,
            c"yamaha/yss225_registers.bin".as_ptr(),
            (*(*dev).card).dev,
        );
        if err < 0 {
            return -1;
        }

        i = 0;
        while (i as usize) + 1 < (*firmware).size {
            let addr = *(*firmware).data.add(i as usize);
            if addr >= 8 && addr < 16 {
                outb(
                    *(*firmware).data.add(i as usize + 1) as c_uint,
                    (*dev).base + addr as c_ulong,
                );
            } else if addr as c_uint == WAIT_IDLE {
                if wavefront_fx_idle(dev) == 0 {
                    return -1;
                }
            } else {
                dev_err(
                    (*(*dev).card).dev,
                    c"invalid address in register data\n".as_ptr(),
                );
                return -1;
            }
            i += 2;
        }

        (*dev).fx_initialized = 1;
    }
    0
}

/* MODULE_FIRMWARE("yamaha/yss225_registers.bin"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
