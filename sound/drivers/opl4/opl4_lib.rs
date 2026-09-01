// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Functions for accessing OPL4 devices
 * Copyright (c) 2003 by Clemens Ladisch <clemens@ladisch.de>
 */

// Dependencies from "opl4_local.h", <sound/initval.h>, <linux/ioport.h>,
// <linux/slab.h>, <linux/init.h>, <linux/module.h>, and <linux/io.h> are
// expected to be supplied by the surrounding translated repository.

use core::ffi::{c_char, c_int, c_ulong, c_void};

extern "C" {
    static OPL4_STATUS_BUSY: u8;
    static OPL4_MODE_BIT: u8;
    static OPL4_DEVICE_ID_MASK: u8;

    static OPL4_REG_MEMORY_CONFIGURATION: u8;
    static OPL4_REG_MEMORY_ADDRESS_HIGH: u8;
    static OPL4_REG_MEMORY_ADDRESS_MID: u8;
    static OPL4_REG_MEMORY_ADDRESS_LOW: u8;
    static OPL4_REG_MEMORY_DATA: u8;
    static OPL4_REG_MIX_CONTROL_FM: u8;
    static OPL4_REG_MIX_CONTROL_PCM: u8;

    static OPL3_REG_MODE: u8;
    static OPL3_OPL3_ENABLE: u8;
    static OPL3_OPL4_ENABLE: u8;
    static OPL3_HW_OPL4: c_int;
    static OPL3_HW_OPL4_ML: c_int;

    static ENODEV: c_int;
    static ENOMEM: c_int;
    static EBUSY: c_int;
    static SNDRV_DEV_CODEC: c_int;
    static SNDRV_SEQ_DEV_ID_OPL4: *const c_char;

    fn inb(port: c_ulong) -> u8;
    fn outb(value: u8, port: c_ulong);
    fn insb(port: c_ulong, addr: *mut c_char, count: c_int);
    fn outsb(port: c_ulong, addr: *const c_char, count: c_int);

    fn dev_dbg(dev: *mut c_void, fmt: *const c_char, ...);
    fn dev_err(dev: *mut c_void, fmt: *const c_char, ...);
    fn request_region(start: c_ulong, n: c_ulong, name: *const c_char) -> *mut resource;
    fn release_and_free_resource(res: *mut resource);
    fn kfree(ptr: *mut c_void);
    fn kzalloc_obj_snd_opl4() -> *mut snd_opl4;
    fn spin_lock_init(lock: *mut spinlock_t);
    fn mutex_init(mutex: *mut mutex);
    fn guard_spinlock_irqsave(lock: *mut spinlock_t) -> spinlock_irqsave_guard;

    fn snd_opl4_free_proc(opl4: *mut snd_opl4);
    fn snd_opl4_create_mixer(opl4: *mut snd_opl4);
    fn snd_opl4_create_proc(opl4: *mut snd_opl4);
    fn snd_device_new(
        card: *mut snd_card,
        type_: c_int,
        device_data: *mut c_void,
        ops: *const snd_device_ops,
    ) -> c_int;
    fn snd_device_free(card: *mut snd_card, device_data: *mut c_void) -> c_int;
    fn snd_opl3_create(
        card: *mut snd_card,
        l_port: c_ulong,
        r_port: c_ulong,
        hardware: c_int,
        integrated: c_int,
        ropl3: *mut *mut snd_opl3,
    ) -> c_int;

    fn snd_seq_device_new(
        card: *mut snd_card,
        device: c_int,
        id: *const c_char,
        argsize: usize,
        result: *mut *mut snd_seq_device,
    ) -> c_int;
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn SNDRV_SEQ_DEVICE_ARGPTR(seq_dev: *mut snd_seq_device) -> *mut c_void;
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut c_void,
}

#[repr(C)]
pub struct snd_opl4 {
    pub card: *mut snd_card,
    pub fm_port: c_ulong,
    pub pcm_port: c_ulong,
    pub reg_lock: spinlock_t,
    pub access_mutex: mutex,
    pub res_fm_port: *mut resource,
    pub res_pcm_port: *mut resource,
    pub hardware: c_int,
    pub seq_dev: *mut snd_seq_device,
    pub seq_dev_num: c_int,
    pub seq_client: c_int,
}

#[repr(C)]
pub struct snd_opl3 {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_device {
    pub device_data: *mut c_void,
}

#[repr(C)]
pub struct snd_device_ops {
    pub dev_free: Option<unsafe extern "C" fn(device: *mut snd_device) -> c_int>,
}

#[repr(C)]
pub struct snd_seq_device {
    pub name: [c_char; 80],
    pub private_data: *mut c_void,
    pub private_free: Option<unsafe extern "C" fn(seq_dev: *mut snd_seq_device)>,
}

#[repr(C)]
pub struct resource {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_irqsave_guard {
    _private: [u8; 0],
}

// MODULE_AUTHOR("Clemens Ladisch <clemens@ladisch.de>");
// MODULE_DESCRIPTION("OPL4 driver");
// MODULE_LICENSE("GPL");

#[inline]
unsafe fn snd_opl4_wait(opl4: *mut snd_opl4) {
    let mut timeout: c_int = 10;
    while (inb((*opl4).fm_port) & OPL4_STATUS_BUSY) != 0 && {
        timeout -= 1;
        timeout > 0
    } {}
}

pub unsafe extern "C" fn snd_opl4_write(opl4: *mut snd_opl4, reg: u8, value: u8) {
    snd_opl4_wait(opl4);
    outb(reg, (*opl4).pcm_port);

    snd_opl4_wait(opl4);
    outb(value, (*opl4).pcm_port + 1);
}

// EXPORT_SYMBOL(snd_opl4_write);

pub unsafe extern "C" fn snd_opl4_read(opl4: *mut snd_opl4, reg: u8) -> u8 {
    snd_opl4_wait(opl4);
    outb(reg, (*opl4).pcm_port);

    snd_opl4_wait(opl4);
    inb((*opl4).pcm_port + 1)
}

// EXPORT_SYMBOL(snd_opl4_read);

pub unsafe extern "C" fn snd_opl4_read_memory(
    opl4: *mut snd_opl4,
    buf: *mut c_char,
    offset: c_int,
    size: c_int,
) {
    let memcfg: u8;

    let _guard = guard_spinlock_irqsave(&mut (*opl4).reg_lock);

    memcfg = snd_opl4_read(opl4, OPL4_REG_MEMORY_CONFIGURATION);
    snd_opl4_write(
        opl4,
        OPL4_REG_MEMORY_CONFIGURATION,
        memcfg | OPL4_MODE_BIT,
    );

    snd_opl4_write(opl4, OPL4_REG_MEMORY_ADDRESS_HIGH, (offset >> 16) as u8);
    snd_opl4_write(opl4, OPL4_REG_MEMORY_ADDRESS_MID, (offset >> 8) as u8);
    snd_opl4_write(opl4, OPL4_REG_MEMORY_ADDRESS_LOW, offset as u8);

    snd_opl4_wait(opl4);
    outb(OPL4_REG_MEMORY_DATA, (*opl4).pcm_port);
    snd_opl4_wait(opl4);
    insb((*opl4).pcm_port + 1, buf, size);

    snd_opl4_write(opl4, OPL4_REG_MEMORY_CONFIGURATION, memcfg);
}

// EXPORT_SYMBOL(snd_opl4_read_memory);

pub unsafe extern "C" fn snd_opl4_write_memory(
    opl4: *mut snd_opl4,
    buf: *const c_char,
    offset: c_int,
    size: c_int,
) {
    let memcfg: u8;

    let _guard = guard_spinlock_irqsave(&mut (*opl4).reg_lock);

    memcfg = snd_opl4_read(opl4, OPL4_REG_MEMORY_CONFIGURATION);
    snd_opl4_write(
        opl4,
        OPL4_REG_MEMORY_CONFIGURATION,
        memcfg | OPL4_MODE_BIT,
    );

    snd_opl4_write(opl4, OPL4_REG_MEMORY_ADDRESS_HIGH, (offset >> 16) as u8);
    snd_opl4_write(opl4, OPL4_REG_MEMORY_ADDRESS_MID, (offset >> 8) as u8);
    snd_opl4_write(opl4, OPL4_REG_MEMORY_ADDRESS_LOW, offset as u8);

    snd_opl4_wait(opl4);
    outb(OPL4_REG_MEMORY_DATA, (*opl4).pcm_port);
    snd_opl4_wait(opl4);
    outsb((*opl4).pcm_port + 1, buf, size);

    snd_opl4_write(opl4, OPL4_REG_MEMORY_CONFIGURATION, memcfg);
}

// EXPORT_SYMBOL(snd_opl4_write_memory);

unsafe fn snd_opl4_enable_opl4(opl4: *mut snd_opl4) {
    outb(OPL3_REG_MODE, (*opl4).fm_port + 2);
    inb((*opl4).fm_port);
    inb((*opl4).fm_port);
    outb(OPL3_OPL3_ENABLE | OPL3_OPL4_ENABLE, (*opl4).fm_port + 3);
    inb((*opl4).fm_port);
    inb((*opl4).fm_port);
}

unsafe fn snd_opl4_detect(opl4: *mut snd_opl4) -> c_int {
    let mut id1: u8;
    let id2: u8;

    snd_opl4_enable_opl4(opl4);

    id1 = snd_opl4_read(opl4, OPL4_REG_MEMORY_CONFIGURATION);
    dev_dbg(
        (*(*opl4).card).dev,
        b"OPL4[02]=%02x\n\0".as_ptr() as *const c_char,
        id1 as c_int,
    );
    match id1 & OPL4_DEVICE_ID_MASK {
        0x20 => {
            (*opl4).hardware = OPL3_HW_OPL4;
        }
        0x40 => {
            (*opl4).hardware = OPL3_HW_OPL4_ML;
        }
        _ => {
            return -ENODEV;
        }
    }

    snd_opl4_write(opl4, OPL4_REG_MIX_CONTROL_FM, 0x00);
    snd_opl4_write(opl4, OPL4_REG_MIX_CONTROL_PCM, 0xff);
    id1 = snd_opl4_read(opl4, OPL4_REG_MIX_CONTROL_FM);
    id2 = snd_opl4_read(opl4, OPL4_REG_MIX_CONTROL_PCM);
    dev_dbg(
        (*(*opl4).card).dev,
        b"OPL4 id1=%02x id2=%02x\n\0".as_ptr() as *const c_char,
        id1 as c_int,
        id2 as c_int,
    );
    if id1 != 0x00 || id2 != 0xff {
        return -ENODEV;
    }

    snd_opl4_write(opl4, OPL4_REG_MIX_CONTROL_FM, 0x3f);
    snd_opl4_write(opl4, OPL4_REG_MIX_CONTROL_PCM, 0x3f);
    snd_opl4_write(opl4, OPL4_REG_MEMORY_CONFIGURATION, 0x00);
    0
}

// Original condition: #if IS_ENABLED(CONFIG_SND_SEQUENCER)
unsafe fn snd_opl4_seq_dev_free(seq_dev: *mut snd_seq_device) {
    let opl4 = (*seq_dev).private_data as *mut snd_opl4;
    (*opl4).seq_dev = core::ptr::null_mut();
}

// Original condition: #if IS_ENABLED(CONFIG_SND_SEQUENCER)
unsafe fn snd_opl4_create_seq_dev(opl4: *mut snd_opl4, seq_device: c_int) -> c_int {
    (*opl4).seq_dev_num = seq_device;
    if snd_seq_device_new(
        (*opl4).card,
        seq_device,
        SNDRV_SEQ_DEV_ID_OPL4,
        core::mem::size_of::<*mut snd_opl4>(),
        &mut (*opl4).seq_dev,
    ) >= 0
    {
        strscpy(
            (*(*opl4).seq_dev).name.as_mut_ptr(),
            b"OPL4 Wavetable\0".as_ptr() as *const c_char,
        );
        *(SNDRV_SEQ_DEVICE_ARGPTR((*opl4).seq_dev) as *mut *mut snd_opl4) = opl4;
        (*(*opl4).seq_dev).private_data = opl4 as *mut c_void;
        (*(*opl4).seq_dev).private_free = Some(snd_opl4_seq_dev_free);
    }
    0
}

unsafe fn snd_opl4_free(opl4: *mut snd_opl4) {
    snd_opl4_free_proc(opl4);
    release_and_free_resource((*opl4).res_fm_port);
    release_and_free_resource((*opl4).res_pcm_port);
    kfree(opl4 as *mut c_void);
}

unsafe extern "C" fn snd_opl4_dev_free(device: *mut snd_device) -> c_int {
    let opl4 = (*device).device_data as *mut snd_opl4;
    snd_opl4_free(opl4);
    0
}

pub unsafe extern "C" fn snd_opl4_create(
    card: *mut snd_card,
    fm_port: c_ulong,
    pcm_port: c_ulong,
    seq_device: c_int,
    ropl3: *mut *mut snd_opl3,
    ropl4: *mut *mut snd_opl4,
) -> c_int {
    let opl4: *mut snd_opl4;
    let mut opl3: *mut snd_opl3 = core::ptr::null_mut();
    let mut err: c_int;
    static OPS: snd_device_ops = snd_device_ops {
        dev_free: Some(snd_opl4_dev_free),
    };

    if !ropl3.is_null() {
        *ropl3 = core::ptr::null_mut();
    }
    if !ropl4.is_null() {
        *ropl4 = core::ptr::null_mut();
    }

    opl4 = kzalloc_obj_snd_opl4();
    if opl4.is_null() {
        return -ENOMEM;
    }

    (*opl4).res_fm_port = request_region(fm_port, 8, b"OPL4 FM\0".as_ptr() as *const c_char);
    (*opl4).res_pcm_port =
        request_region(pcm_port, 8, b"OPL4 PCM/MIX\0".as_ptr() as *const c_char);
    if (*opl4).res_fm_port.is_null() || (*opl4).res_pcm_port.is_null() {
        dev_err(
            (*card).dev,
            b"opl4: can't grab ports 0x%lx, 0x%lx\n\0".as_ptr() as *const c_char,
            fm_port,
            pcm_port,
        );
        snd_opl4_free(opl4);
        return -EBUSY;
    }

    (*opl4).card = card;
    (*opl4).fm_port = fm_port;
    (*opl4).pcm_port = pcm_port;
    spin_lock_init(&mut (*opl4).reg_lock);
    mutex_init(&mut (*opl4).access_mutex);

    err = snd_opl4_detect(opl4);
    if err < 0 {
        snd_opl4_free(opl4);
        dev_dbg(
            (*card).dev,
            b"OPL4 chip not detected at %#lx/%#lx\n\0".as_ptr() as *const c_char,
            fm_port,
            pcm_port,
        );
        return err;
    }

    err = snd_device_new(card, SNDRV_DEV_CODEC, opl4 as *mut c_void, &OPS);
    if err < 0 {
        snd_opl4_free(opl4);
        return err;
    }

    err = snd_opl3_create(card, fm_port, fm_port + 2, (*opl4).hardware, 1, &mut opl3);
    if err < 0 {
        snd_device_free(card, opl4 as *mut c_void);
        return err;
    }

    /* opl3 initialization disabled opl4, so reenable */
    snd_opl4_enable_opl4(opl4);

    snd_opl4_create_mixer(opl4);
    snd_opl4_create_proc(opl4);

    // Original condition: #if IS_ENABLED(CONFIG_SND_SEQUENCER)
    (*opl4).seq_client = -1;
    if (*opl4).hardware < OPL3_HW_OPL4_ML {
        snd_opl4_create_seq_dev(opl4, seq_device);
    }

    if !ropl3.is_null() {
        *ropl3 = opl3;
    }
    if !ropl4.is_null() {
        *ropl4 = opl4;
    }
    0
}

// EXPORT_SYMBOL(snd_opl4_create);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
