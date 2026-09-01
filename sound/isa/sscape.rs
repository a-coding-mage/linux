// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Low-level ALSA driver for the ENSONIQ SoundScape
 *   Copyright (c) by Chris Rankin
 *
 *   This driver was written in part using information obtained from
 *   the OSS/Free SoundScape driver, written by Hannu Savolainen.
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

extern "C" {
    static mut jiffies: c_ulong;

    static mut index: [c_int; SNDRV_CARDS];
    static mut id: [*mut c_char; SNDRV_CARDS];
    static mut port: [c_long; SNDRV_CARDS];
    static mut wss_port: [c_long; SNDRV_CARDS];
    static mut irq: [c_int; SNDRV_CARDS];
    static mut mpu_irq: [c_int; SNDRV_CARDS];
    static mut dma: [c_int; SNDRV_CARDS];
    static mut dma2: [c_int; SNDRV_CARDS];
    static mut joystick: [bool; SNDRV_CARDS];

    fn outb(value: u8, port: c_ulong);
    fn inb(port: c_ulong) -> u8;
    fn udelay(usecs: c_ulong);
    fn msleep(msecs: c_uint);
    fn msecs_to_jiffies(msecs: c_uint) -> c_ulong;
    fn time_before(a: c_ulong, b: c_ulong) -> bool;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn strscpy(dest: *mut c_char, src: *const c_char) -> isize;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;

    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn devm_request_region(dev: *mut device, start: c_ulong, n: c_ulong, name: *const c_char) -> *mut resource;

    fn spin_lock_init(lock: *mut spinlock_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);

    fn snd_dma_alloc_pages_fallback(ty: c_int, dev: *mut device, size: c_ulong, buf: *mut snd_dma_buffer) -> c_int;
    fn snd_dma_free_pages(buf: *mut snd_dma_buffer);
    fn snd_dma_program(dma: c_int, addr: c_ulong, size: c_ulong, mode: c_int);
    fn snd_devm_request_dma(dev: *mut device, dma: c_int, name: *const c_char) -> c_int;

    fn request_firmware(fw: *mut *const firmware, name: *const c_char, dev: *mut device) -> c_int;
    fn release_firmware(fw: *const firmware);

    fn snd_kcontrol_chip(kctl: *mut snd_kcontrol) -> *mut c_void;
    fn snd_ctl_add(card: *mut snd_card, kctl: *mut snd_kcontrol) -> c_int;
    fn snd_ctl_new1(knew: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;

    fn snd_mpu401_uart_new(card: *mut snd_card, devnum: c_int, hardware: c_int, port: c_ulong, info_flags: c_uint, irq: c_int, rrawmidi: *mut *mut snd_rawmidi) -> c_int;
    fn snd_wss_create(card: *mut snd_card, port: c_ulong, cport: c_long, irq: c_int, dma1: c_int, dma2: c_int, hardware: c_int, hwshare: c_uint, rchip: *mut *mut snd_wss) -> c_int;
    fn snd_wss_mce_up(chip: *mut snd_wss);
    fn snd_wss_mce_down(chip: *mut snd_wss);
    fn snd_wss_out(chip: *mut snd_wss, reg: c_int, val: u8);
    fn snd_wss_pcm(chip: *mut snd_wss, device: c_int) -> c_int;
    fn snd_wss_mixer(chip: *mut snd_wss) -> c_int;
    fn snd_wss_timer(chip: *mut snd_wss, device: c_int) -> c_int;
    fn snd_power_change_state(card: *mut snd_card, state: c_int);
    fn snd_devm_card_new(dev: *mut device, idx: c_int, id: *mut c_char, module: *mut c_void, extra_size: usize, card_ret: *mut *mut snd_card) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;

    fn isa_register_driver(driver: *mut isa_driver, ndev: c_uint) -> c_int;
    fn isa_unregister_driver(driver: *mut isa_driver);

    fn pnp_request_card_device(card: *mut pnp_card_link, id: *const c_char, from: *mut pnp_dev) -> *mut pnp_dev;
    fn pnp_is_active(dev: *mut pnp_dev) -> bool;
    fn pnp_activate_dev(dev: *mut pnp_dev) -> c_int;
    fn pnp_port_start(dev: *mut pnp_dev, bar: c_uint) -> c_ulong;
    fn pnp_irq(dev: *mut pnp_dev, idx: c_uint) -> c_int;
    fn pnp_dma(dev: *mut pnp_dev, idx: c_uint) -> c_int;
    fn pnp_set_card_drvdata(card: *mut pnp_card_link, data: *mut c_void);
    fn pnp_get_card_drvdata(card: *mut pnp_card_link) -> *mut c_void;
    fn pnp_register_card_driver(driver: *mut pnp_card_driver) -> c_int;
    fn pnp_unregister_card_driver(driver: *mut pnp_card_driver);
}

const SNDRV_CARDS: usize = 8;
const SNDRV_DEFAULT_IDX: c_int = -1;
const SNDRV_DEFAULT_STR: *mut c_char = ptr::null_mut();
const SNDRV_DEFAULT_PORT: c_long = -1;
const SNDRV_DEFAULT_IRQ: c_int = -1;
const SNDRV_DEFAULT_DMA: c_int = -1;
const SNDRV_AUTO_PORT: c_long = -1;
const SNDRV_AUTO_IRQ: c_int = -1;
const SNDRV_AUTO_DMA: c_int = -1;
const SNDRV_DMA_TYPE_DEV: c_int = 0;
const PAGE_SIZE: c_ulong = 4096;
const ENOMEM: c_int = 12;
const EAGAIN: c_int = 11;
const EIO: c_int = 5;
const ENXIO: c_int = 6;
const EBUSY: c_int = 16;
const ENODEV: c_int = 19;
const ENOSPC: c_int = 28;
const DMA_MODE_WRITE: c_int = 0x48;
const MPU401_HW_MPU401: c_int = 0;
const MPU401_INFO_INTEGRATED: c_uint = 0;
const WSS_HW_DETECT: c_int = 0;
const WSS_HW_AD1848: c_int = 1;
const WSS_HWSHARE_DMA1: c_uint = 1;
const AD1845_CLOCK: c_int = 0;
const SNDRV_CTL_ELEM_TYPE_INTEGER: c_int = 1;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_int = 2;
const SNDRV_CTL_POWER_D3HOT: c_int = 3;
const SNDRV_CTL_POWER_D0: c_int = 0;
const PNP_DRIVER_RES_DO_NOT_CHANGE: c_uint = 1;
static mut THIS_MODULE: *mut c_void = ptr::null_mut();

#[inline]
const fn PAGE_ALIGN(x: c_ulong) -> c_ulong {
    (x + PAGE_SIZE - 1) & !(PAGE_SIZE - 1)
}

#[inline]
const fn HOST_CTRL_IO(i: c_uint) -> c_ulong {
    (i + 2) as c_ulong
}
#[inline]
const fn HOST_DATA_IO(i: c_uint) -> c_ulong {
    (i + 3) as c_ulong
}
#[inline]
const fn ODIE_ADDR_IO(i: c_uint) -> c_ulong {
    (i + 4) as c_ulong
}
#[inline]
const fn ODIE_DATA_IO(i: c_uint) -> c_ulong {
    (i + 5) as c_ulong
}
#[inline]
const fn CODEC_IO(i: c_long) -> c_long {
    i + 8
}

const IC_ODIE: c_int = 1;
const IC_OPUS: c_int = 2;
const RX_READY: u8 = 0x01;
const TX_READY: u8 = 0x02;
const CMD_ACK: u8 = 0x80;
const CMD_SET_MIDI_VOL: u8 = 0x84;
const CMD_GET_MIDI_VOL: u8 = 0x85;
const CMD_XXX_MIDI_VOL: u8 = 0x86;
const CMD_SET_EXTMIDI: u8 = 0x8a;
const CMD_GET_EXTMIDI: u8 = 0x8b;
const CMD_SET_MT32: u8 = 0x8c;
const CMD_GET_MT32: u8 = 0x8d;
const DMA_8BIT: c_uint = 0x80;
const INVALID_IRQ: c_uint = !0;
const MIDI_DEVNUM: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
enum GA_REG {
    GA_INTSTAT_REG = 0,
    GA_INTENA_REG,
    GA_DMAA_REG,
    GA_DMAB_REG,
    GA_INTCFG_REG,
    GA_DMACFG_REG,
    GA_CDCFG_REG,
    GA_SMCFGA_REG,
    GA_SMCFGB_REG,
    GA_HMCTL_REG,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum card_type {
    MEDIA_FX,
    SSCAPE,
    SSCAPE_PNP,
    SSCAPE_VIVO,
}

#[repr(C)]
struct soundscape {
    lock: spinlock_t,
    io_base: c_uint,
    wss_base: c_ulong,
    irq: c_int,
    mpu_irq: c_int,
    dma1: c_int,
    dma2: c_int,
    ic_type: c_int,
    type_: card_type,
    io_res: *mut resource,
    wss_res: *mut resource,
    chip: *mut snd_wss,
    midi_vol: u8,
    joystick: bool,
    midi_enabled: bool,
    dev: *mut device,
}

#[repr(C)] struct spinlock_t { _private: [u8; 0] }
#[repr(C)] struct resource { _private: [u8; 0] }
#[repr(C)] struct device { _private: [u8; 0] }
#[repr(C)] struct pm_message_t { _private: [u8; 0] }
#[repr(C)] struct snd_kcontrol { _private: [u8; 0] }

#[repr(C)]
struct snd_card {
    private_data: *mut c_void,
    dev: *mut device,
    driver: [c_char; 16],
    shortname: [c_char; 32],
    longname: [c_char; 80],
}

#[repr(C)]
struct snd_wss {
    card: *mut snd_card,
    dma1: c_int,
    dma2: c_int,
    irq: c_int,
    port: c_ulong,
    hardware: c_int,
    reg_lock: spinlock_t,
    suspend: Option<unsafe extern "C" fn(*mut snd_wss)>,
    resume: Option<unsafe extern "C" fn(*mut snd_wss)>,
}

#[repr(C)]
struct snd_dma_buffer {
    area: *mut c_void,
    addr: c_ulong,
    bytes: c_ulong,
}

#[repr(C)]
struct firmware {
    size: usize,
    data: *const u8,
}

#[repr(C)]
struct snd_rawmidi {
    private_data: *mut c_void,
}

#[repr(C)]
struct snd_mpu401 {
    rmidi: *mut snd_rawmidi_card,
    open_input: Option<unsafe extern "C" fn(*mut snd_mpu401) -> c_int>,
    open_output: Option<unsafe extern "C" fn(*mut snd_mpu401) -> c_int>,
    private_data: *mut c_void,
}

#[repr(C)]
struct snd_rawmidi_card {
    card: *mut snd_card,
}

#[repr(C)]
struct snd_ctl_elem_info {
    type_: c_int,
    count: c_uint,
    value: snd_ctl_elem_info_value,
}

#[repr(C)]
struct snd_ctl_elem_info_value {
    integer: snd_ctl_elem_info_integer,
}

#[repr(C)]
struct snd_ctl_elem_info_integer {
    min: c_long,
    max: c_long,
}

#[repr(C)]
struct snd_ctl_elem_value {
    value: snd_ctl_elem_value_value,
}

#[repr(C)]
struct snd_ctl_elem_value_value {
    integer: snd_ctl_elem_value_integer,
}

#[repr(C)]
struct snd_ctl_elem_value_integer {
    value: [c_long; 128],
}

#[repr(C)]
struct snd_kcontrol_new {
    iface: c_int,
    name: *const c_char,
    info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
}

#[repr(C)]
struct driver_inner {
    name: *const c_char,
}

#[repr(C)]
struct isa_driver {
    match_: Option<unsafe extern "C" fn(*mut device, c_uint) -> c_int>,
    probe: Option<unsafe extern "C" fn(*mut device, c_uint) -> c_int>,
    suspend: Option<unsafe extern "C" fn(*mut device, c_uint, pm_message_t) -> c_int>,
    resume: Option<unsafe extern "C" fn(*mut device, c_uint) -> c_int>,
    driver: driver_inner,
}

#[repr(C)]
struct pnp_id { id: [c_char; 8] }
#[repr(C)]
struct pnp_card_device_id { id: [c_char; 8], devs: [pnp_id; 1] }
#[repr(C)] struct pnp_dev { dev: device }
#[repr(C)] struct pnp_card { dev: device }
#[repr(C)] struct pnp_card_link { card: *mut pnp_card }

#[repr(C)]
struct pnp_card_driver {
    flags: c_uint,
    name: *const c_char,
    id_table: *const pnp_card_device_id,
    probe: Option<unsafe extern "C" fn(*mut pnp_card_link, *const pnp_card_device_id) -> c_int>,
    suspend: Option<unsafe extern "C" fn(*mut pnp_card_link, pm_message_t) -> c_int>,
    resume: Option<unsafe extern "C" fn(*mut pnp_card_link) -> c_int>,
}

#[inline]
unsafe fn MPU401C(mpu: *const snd_mpu401) -> c_ulong {
    (*mpu).private_data as c_ulong
}

#[inline]
unsafe fn MPU401D(mpu: *const snd_mpu401) -> c_ulong {
    (*mpu).private_data as c_ulong
}

#[inline]
unsafe fn get_card_soundscape(c: *mut snd_card) -> *mut soundscape {
    (*c).private_data as *mut soundscape
}

/*
 * Store the resolved board settings in the per-card state so that
 * the same configuration can be replayed later if necessary.
 */
unsafe fn sscape_store_settings(sscape: *mut soundscape, dev: c_int) {
    (*sscape).io_base = port[dev as usize] as c_uint;
    (*sscape).wss_base = wss_port[dev as usize] as c_ulong;
    (*sscape).irq = irq[dev as usize];
    (*sscape).mpu_irq = mpu_irq[dev as usize];
    (*sscape).dma1 = dma[dev as usize];
    (*sscape).dma2 = dma2[dev as usize];
    (*sscape).joystick = joystick[dev as usize];
}

unsafe fn get_dmabuf(s: *mut soundscape, buf: *mut snd_dma_buffer, size: c_ulong) -> *mut snd_dma_buffer {
    if !buf.is_null() {
        if snd_dma_alloc_pages_fallback(SNDRV_DMA_TYPE_DEV, (*(*(*s).chip).card).dev, size, buf) < 0 {
            dev_err((*s).dev, b"sscape: Failed to allocate %lu bytes for DMA\n\0".as_ptr() as *const c_char, size);
            return ptr::null_mut();
        }
    }
    buf
}

unsafe fn free_dmabuf(buf: *mut snd_dma_buffer) {
    if !buf.is_null() && !(*buf).area.is_null() {
        snd_dma_free_pages(buf);
    }
}

#[inline]
unsafe fn sscape_write_unsafe(io_base: c_uint, reg: GA_REG, val: u8) {
    outb(reg as u8, ODIE_ADDR_IO(io_base));
    outb(val, ODIE_DATA_IO(io_base));
}

unsafe fn sscape_write(s: *mut soundscape, reg: GA_REG, val: u8) {
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*s).lock, &mut flags);
    sscape_write_unsafe((*s).io_base, reg, val);
    spin_unlock_irqrestore(&mut (*s).lock, flags);
}

#[inline]
unsafe fn sscape_read_unsafe(io_base: c_uint, reg: GA_REG) -> u8 {
    outb(reg as u8, ODIE_ADDR_IO(io_base));
    inb(ODIE_DATA_IO(io_base))
}

#[inline]
unsafe fn set_host_mode_unsafe(io_base: c_uint) {
    outb(0x0, HOST_CTRL_IO(io_base));
}

#[inline]
unsafe fn set_midi_mode_unsafe(io_base: c_uint) {
    outb(0x3, HOST_CTRL_IO(io_base));
}

#[inline]
unsafe fn host_read_unsafe(io_base: c_uint) -> c_int {
    let mut data: c_int = -1;
    if (inb(HOST_CTRL_IO(io_base)) & RX_READY) != 0 {
        data = inb(HOST_DATA_IO(io_base)) as c_int;
    }
    data
}

unsafe fn host_read_ctrl_unsafe(io_base: c_uint, mut timeout: c_uint) -> c_int {
    let mut data = host_read_unsafe(io_base);
    while data < 0 && timeout != 0 {
        udelay(100);
        timeout -= 1;
        data = host_read_unsafe(io_base);
    }
    data
}

#[inline]
unsafe fn host_write_unsafe(io_base: c_uint, data: u8) -> bool {
    if (inb(HOST_CTRL_IO(io_base)) & TX_READY) != 0 {
        outb(data, HOST_DATA_IO(io_base));
        return true;
    }
    false
}

unsafe fn host_write_ctrl_unsafe(io_base: c_uint, data: u8, mut timeout: c_uint) -> bool {
    let mut written = host_write_unsafe(io_base, data);
    while !written && timeout != 0 {
        udelay(100);
        timeout -= 1;
        written = host_write_unsafe(io_base, data);
    }
    written
}

#[inline]
unsafe fn verify_mpu401(mpu: *const snd_mpu401) -> c_int {
    ((inb(MPU401C(mpu)) & 0xc0) == 0x80) as c_int
}

#[inline]
unsafe fn initialise_mpu401(mpu: *const snd_mpu401) {
    outb(0, MPU401D(mpu));
}

unsafe fn activate_ad1845_unsafe(io_base: c_uint) {
    let val = sscape_read_unsafe(io_base, GA_REG::GA_HMCTL_REG);
    sscape_write_unsafe(io_base, GA_REG::GA_HMCTL_REG, (val & 0xcf) | 0x10);
    sscape_write_unsafe(io_base, GA_REG::GA_CDCFG_REG, 0x80);
}

unsafe fn sscape_start_dma_unsafe(io_base: c_uint, reg: GA_REG) {
    sscape_write_unsafe(io_base, reg, sscape_read_unsafe(io_base, reg) | 0x01);
    sscape_write_unsafe(io_base, reg, sscape_read_unsafe(io_base, reg) & 0xfe);
}

unsafe fn sscape_wait_dma_unsafe(io_base: c_uint, reg: GA_REG, mut timeout: c_uint) -> c_int {
    while (sscape_read_unsafe(io_base, reg) & 0x01) == 0 && timeout != 0 {
        udelay(100);
        timeout -= 1;
    }
    (sscape_read_unsafe(io_base, reg) & 0x01) as c_int
}

unsafe fn obp_startup_ack(s: *mut soundscape, timeout: c_uint) -> c_int {
    let end_time = jiffies.wrapping_add(msecs_to_jiffies(timeout));
    loop {
        let mut flags: c_ulong = 0;
        spin_lock_irqsave(&mut (*s).lock, &mut flags);
        let x = host_read_unsafe((*s).io_base);
        spin_unlock_irqrestore(&mut (*s).lock, flags);
        if x == 0xfe || x == 0xff {
            return 1;
        }
        msleep(10);
        if !time_before(jiffies, end_time) {
            break;
        }
    }
    0
}

unsafe fn host_startup_ack(s: *mut soundscape, timeout: c_uint) -> c_int {
    let end_time = jiffies.wrapping_add(msecs_to_jiffies(timeout));
    loop {
        let mut flags: c_ulong = 0;
        spin_lock_irqsave(&mut (*s).lock, &mut flags);
        let x = host_read_unsafe((*s).io_base);
        spin_unlock_irqrestore(&mut (*s).lock, flags);
        if x == 0xfe {
            return 1;
        }
        msleep(10);
        if !time_before(jiffies, end_time) {
            break;
        }
    }
    0
}

unsafe fn upload_dma_data(s: *mut soundscape, mut data: *const u8, mut size: usize) -> c_int {
    let mut dma_buf: snd_dma_buffer = core::mem::zeroed();
    let mut ret: c_int;

    if get_dmabuf(s, &mut dma_buf, PAGE_ALIGN(32 * 1024)).is_null() {
        return -ENOMEM;
    }

    {
        let mut flags: c_ulong = 0;
        spin_lock_irqsave(&mut (*s).lock, &mut flags);

        let mut val = sscape_read_unsafe((*s).io_base, GA_REG::GA_HMCTL_REG);
        sscape_write_unsafe((*s).io_base, GA_REG::GA_HMCTL_REG, val & 0x3f);

        val = (((*(*s).chip).dma1 << 4) as c_uint | DMA_8BIT) as u8;
        sscape_write_unsafe((*s).io_base, GA_REG::GA_DMAA_REG, val);
        sscape_write_unsafe((*s).io_base, GA_REG::GA_DMAB_REG, 0x20);

        val = sscape_read_unsafe((*s).io_base, GA_REG::GA_HMCTL_REG);
        sscape_write_unsafe((*s).io_base, GA_REG::GA_HMCTL_REG, val | 0x80);

        while size != 0 {
            let len = if size < dma_buf.bytes as usize { size } else { dma_buf.bytes as usize };
            memcpy(dma_buf.area, data as *const c_void, len);
            data = data.add(len);
            size -= len;

            snd_dma_program((*(*s).chip).dma1, dma_buf.addr, len as c_ulong, DMA_MODE_WRITE);
            sscape_start_dma_unsafe((*s).io_base, GA_REG::GA_DMAA_REG);
            if sscape_wait_dma_unsafe((*s).io_base, GA_REG::GA_DMAA_REG, 5000) == 0 {
                dev_err((*s).dev, b"sscape: DMA upload has timed out\n\0".as_ptr() as *const c_char);
                ret = -EAGAIN;
                spin_unlock_irqrestore(&mut (*s).lock, flags);
                sscape_write(s, GA_REG::GA_DMAA_REG, if (*s).ic_type == IC_OPUS { 0x40 } else { 0x70 });
                free_dmabuf(&mut dma_buf);
                return ret;
            }
        }

        set_host_mode_unsafe((*s).io_base);
        outb(0x0, (*s).io_base as c_ulong);
        val = sscape_read_unsafe((*s).io_base, GA_REG::GA_HMCTL_REG);
        sscape_write_unsafe((*s).io_base, GA_REG::GA_HMCTL_REG, val | 0x40);
        spin_unlock_irqrestore(&mut (*s).lock, flags);
    }

    ret = 0;
    if obp_startup_ack(s, 5000) == 0 {
        dev_err((*s).dev, b"sscape: No response from on-board processor after upload\n\0".as_ptr() as *const c_char);
        ret = -EAGAIN;
    } else if host_startup_ack(s, 5000) == 0 {
        dev_err((*s).dev, b"sscape: SoundScape failed to initialise\n\0".as_ptr() as *const c_char);
        ret = -EAGAIN;
    }

    sscape_write(s, GA_REG::GA_DMAA_REG, if (*s).ic_type == IC_OPUS { 0x40 } else { 0x70 });
    free_dmabuf(&mut dma_buf);
    ret
}

unsafe fn sscape_upload_bootblock(card: *mut snd_card) -> c_int {
    let sscape = get_card_soundscape(card);
    let mut init_fw: *const firmware = ptr::null();
    let mut data: c_int = 0;
    let mut ret = request_firmware(&mut init_fw, b"scope.cod\0".as_ptr() as *const c_char, (*card).dev);
    if ret < 0 {
        dev_err((*card).dev, b"sscape: Error loading scope.cod\0".as_ptr() as *const c_char);
        return ret;
    }
    ret = upload_dma_data(sscape, (*init_fw).data, (*init_fw).size);

    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*sscape).lock, &mut flags);
    if ret == 0 {
        data = host_read_ctrl_unsafe((*sscape).io_base, 100);
    }
    if (data & 0x10) != 0 {
        sscape_write_unsafe((*sscape).io_base, GA_REG::GA_SMCFGA_REG, 0x2f);
    }
    data &= 0xf;
    spin_unlock_irqrestore(&mut (*sscape).lock, flags);

    if ret == 0 && data > 7 {
        dev_err((*card).dev, b"sscape: timeout reading firmware version\n\0".as_ptr() as *const c_char);
        ret = -EAGAIN;
    }
    release_firmware(init_fw);
    if ret == 0 { data } else { ret }
}

unsafe fn sscape_upload_microcode(card: *mut snd_card, version: c_int) -> c_int {
    let sscape = get_card_soundscape(card);
    let mut init_fw: *const firmware = ptr::null();
    let mut name = [0 as c_char; 14];
    scnprintf(name.as_mut_ptr(), name.len(), b"sndscape.co%d\0".as_ptr() as *const c_char, version);
    let mut err = request_firmware(&mut init_fw, name.as_ptr(), (*card).dev);
    if err < 0 {
        dev_err((*card).dev, b"sscape: Error loading sndscape.co%d\0".as_ptr() as *const c_char, version);
        return err;
    }
    err = upload_dma_data(sscape, (*init_fw).data, (*init_fw).size);
    if err == 0 {
        dev_info((*card).dev, b"sscape: MIDI firmware loaded %zu KBs\n\0".as_ptr() as *const c_char, (*init_fw).size >> 10);
    }
    release_firmware(init_fw);
    err
}

unsafe fn sscape_restore_midi_state(sscape: *mut soundscape) -> c_int {
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*sscape).lock, &mut flags);
    set_host_mode_unsafe((*sscape).io_base);
    let success = host_write_ctrl_unsafe((*sscape).io_base, CMD_SET_MIDI_VOL, 100)
        && host_write_ctrl_unsafe((*sscape).io_base, (*sscape).midi_vol, 100)
        && host_write_ctrl_unsafe((*sscape).io_base, CMD_XXX_MIDI_VOL, 100)
        && host_write_ctrl_unsafe((*sscape).io_base, (*sscape).midi_vol, 100)
        && host_write_ctrl_unsafe((*sscape).io_base, CMD_SET_EXTMIDI, 100)
        && host_write_ctrl_unsafe((*sscape).io_base, 0, 100)
        && host_write_ctrl_unsafe((*sscape).io_base, CMD_ACK, 100);
    set_midi_mode_unsafe((*sscape).io_base);
    spin_unlock_irqrestore(&mut (*sscape).lock, flags);
    if success { 0 } else { -EIO }
}

unsafe extern "C" fn sscape_midi_info(_ctl: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 127;
    0
}

unsafe extern "C" fn sscape_midi_get(kctl: *mut snd_kcontrol, uctl: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kctl) as *mut snd_wss;
    let card = (*chip).card;
    let s = get_card_soundscape(card);
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*s).lock, &mut flags);
    (*uctl).value.integer.value[0] = (*s).midi_vol as c_long;
    spin_unlock_irqrestore(&mut (*s).lock, flags);
    0
}

unsafe extern "C" fn sscape_midi_put(kctl: *mut snd_kcontrol, uctl: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kctl) as *mut snd_wss;
    let card = (*chip).card;
    let s = get_card_soundscape(card);
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*s).lock, &mut flags);
    let new_val = ((*uctl).value.integer.value[0] & 127) as u8;
    set_host_mode_unsafe((*s).io_base);
    let change: c_int;
    if (*s).midi_vol == new_val {
        change = 0;
    } else {
        change = (host_write_ctrl_unsafe((*s).io_base, CMD_SET_MIDI_VOL, 100)
            && host_write_ctrl_unsafe((*s).io_base, new_val, 100)
            && host_write_ctrl_unsafe((*s).io_base, CMD_XXX_MIDI_VOL, 100)
            && host_write_ctrl_unsafe((*s).io_base, new_val, 100)) as c_int;
        (*s).midi_vol = new_val;
    }
    set_midi_mode_unsafe((*s).io_base);
    spin_unlock_irqrestore(&mut (*s).lock, flags);
    change
}

static midi_mixer_ctl: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: b"MIDI\0".as_ptr() as *const c_char,
    info: Some(sscape_midi_info),
    get: Some(sscape_midi_get),
    put: Some(sscape_midi_put),
};

unsafe fn get_irq_config(sscape_type: card_type, irq: c_int) -> c_uint {
    static valid_irq: [c_int; 4] = [9, 5, 7, 10];
    static old_irq: [c_int; 4] = [9, 7, 5, 15];
    if sscape_type == card_type::MEDIA_FX {
        for cfg in 0..old_irq.len() {
            if irq == old_irq[cfg] {
                return cfg as c_uint;
            }
        }
    } else {
        for cfg in 0..valid_irq.len() {
            if irq == valid_irq[cfg] {
                return cfg as c_uint;
            }
        }
    }
    INVALID_IRQ
}

unsafe fn sscape_configure_board(sscape: *mut soundscape) -> c_int {
    let irq_cfg = get_irq_config((*sscape).type_, (*sscape).irq);
    if irq_cfg == INVALID_IRQ {
        return -ENXIO;
    }
    let mut mpu_irq_cfg = get_irq_config((*sscape).type_, (*sscape).mpu_irq);
    if mpu_irq_cfg == INVALID_IRQ {
        return -ENXIO;
    }

    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*sscape).lock, &mut flags);
    if (*sscape).ic_type == IC_OPUS {
        activate_ad1845_unsafe((*sscape).io_base);
    }
    sscape_write_unsafe((*sscape).io_base, GA_REG::GA_SMCFGA_REG, 0x2e);
    sscape_write_unsafe((*sscape).io_base, GA_REG::GA_SMCFGB_REG, 0x00);
    sscape_write_unsafe((*sscape).io_base, GA_REG::GA_DMACFG_REG, 0x50);
    let dma_cfg = if (*sscape).ic_type == IC_OPUS { 0x40 } else { 0x70 };
    sscape_write_unsafe((*sscape).io_base, GA_REG::GA_DMAA_REG, dma_cfg);
    sscape_write_unsafe((*sscape).io_base, GA_REG::GA_DMAB_REG, 0x20);
    mpu_irq_cfg |= mpu_irq_cfg << 2;
    let mut val = (sscape_read_unsafe((*sscape).io_base, GA_REG::GA_HMCTL_REG) & 0xf7) as c_int;
    if (*sscape).joystick {
        val |= 0x08;
    }
    sscape_write_unsafe((*sscape).io_base, GA_REG::GA_HMCTL_REG, (val | 0xd0) as u8);
    sscape_write_unsafe((*sscape).io_base, GA_REG::GA_INTCFG_REG, (0xf0 | mpu_irq_cfg) as u8);
    sscape_write_unsafe((*sscape).io_base, GA_REG::GA_CDCFG_REG, (0x09 | DMA_8BIT | (((*sscape).dma1 as c_uint) << 4) | (irq_cfg << 1)) as u8);
    sscape_write_unsafe((*sscape).io_base, GA_REG::GA_INTENA_REG, 0x80);
    spin_unlock_irqrestore(&mut (*sscape).lock, flags);
    0
}

unsafe fn detect_sscape(s: *mut soundscape, mut wss_io: c_long) -> c_int {
    let mut flags: c_ulong = 0;
    let mut d: c_uint;
    let mut retval = 0;
    spin_lock_irqsave(&mut (*s).lock, &mut flags);

    if (inb(HOST_CTRL_IO((*s).io_base)) & 0x78) != 0 {
        spin_unlock_irqrestore(&mut (*s).lock, flags);
        return retval;
    }
    d = (inb(ODIE_ADDR_IO((*s).io_base)) & 0xf0) as c_uint;
    if (d & 0x80) != 0 {
        spin_unlock_irqrestore(&mut (*s).lock, flags);
        return retval;
    }
    if d == 0 {
        (*s).ic_type = IC_ODIE;
    } else if (d & 0x60) != 0 {
        (*s).ic_type = IC_OPUS;
    } else {
        spin_unlock_irqrestore(&mut (*s).lock, flags);
        return retval;
    }
    outb(0xfa, ODIE_ADDR_IO((*s).io_base));
    if (inb(ODIE_ADDR_IO((*s).io_base)) & 0x9f) != 0x0a {
        spin_unlock_irqrestore(&mut (*s).lock, flags);
        return retval;
    }
    outb(0xfe, ODIE_ADDR_IO((*s).io_base));
    if (inb(ODIE_ADDR_IO((*s).io_base)) & 0x9f) != 0x0e {
        spin_unlock_irqrestore(&mut (*s).lock, flags);
        return retval;
    }
    outb(0xfe, ODIE_ADDR_IO((*s).io_base));
    d = inb(ODIE_DATA_IO((*s).io_base)) as c_uint;
    if (*s).type_ != card_type::SSCAPE_VIVO && (d & 0x9f) != 0x0e {
        spin_unlock_irqrestore(&mut (*s).lock, flags);
        return retval;
    }
    if (*s).ic_type == IC_OPUS {
        activate_ad1845_unsafe((*s).io_base);
    }
    if (*s).type_ == card_type::SSCAPE_VIVO {
        wss_io += 4;
    }
    d = sscape_read_unsafe((*s).io_base, GA_REG::GA_HMCTL_REG) as c_uint;
    sscape_write_unsafe((*s).io_base, GA_REG::GA_HMCTL_REG, (d | 0xc0) as u8);
    d = 0;
    while d < 500 {
        if (inb(wss_io as c_ulong) & 0x80) == 0 {
            break;
        }
        spin_unlock_irqrestore(&mut (*s).lock, flags);
        msleep(1);
        spin_lock_irqsave(&mut (*s).lock, &mut flags);
        d += 1;
    }
    if (inb(wss_io as c_ulong) & 0x80) != 0 || inb((wss_io + 2) as c_ulong) == 0xff {
        spin_unlock_irqrestore(&mut (*s).lock, flags);
        return retval;
    }
    d = (sscape_read_unsafe((*s).io_base, GA_REG::GA_HMCTL_REG) & 0x3f) as c_uint;
    sscape_write_unsafe((*s).io_base, GA_REG::GA_HMCTL_REG, d as u8);
    if (inb(wss_io as c_ulong) & 0x80) != 0 {
        (*s).type_ = card_type::MEDIA_FX;
    }
    d = sscape_read_unsafe((*s).io_base, GA_REG::GA_HMCTL_REG) as c_uint;
    sscape_write_unsafe((*s).io_base, GA_REG::GA_HMCTL_REG, (d | 0xc0) as u8);
    d = 0;
    while d < 500 {
        if (inb(wss_io as c_ulong) & 0x80) == 0 {
            break;
        }
        spin_unlock_irqrestore(&mut (*s).lock, flags);
        msleep(1);
        spin_lock_irqsave(&mut (*s).lock, &mut flags);
        d += 1;
    }
    retval = 1;
    spin_unlock_irqrestore(&mut (*s).lock, flags);
    retval
}

unsafe extern "C" fn mpu401_open(mpu: *mut snd_mpu401) -> c_int {
    if verify_mpu401(mpu) == 0 {
        dev_err((*(*(*mpu).rmidi).card).dev, b"sscape: MIDI disabled, please load firmware\n\0".as_ptr() as *const c_char);
        return -ENODEV;
    }
    0
}

unsafe fn create_mpu401(card: *mut snd_card, devnum: c_int, port: c_ulong, irq: c_int) -> c_int {
    let sscape = get_card_soundscape(card);
    let mut rawmidi: *mut snd_rawmidi = ptr::null_mut();
    let err = snd_mpu401_uart_new(card, devnum, MPU401_HW_MPU401, port, MPU401_INFO_INTEGRATED, irq, &mut rawmidi);
    if err == 0 {
        let mpu = (*rawmidi).private_data as *mut snd_mpu401;
        (*mpu).open_input = Some(mpu401_open);
        (*mpu).open_output = Some(mpu401_open);
        (*mpu).private_data = sscape as *mut c_void;
        initialise_mpu401(mpu);
    }
    err
}

unsafe fn create_ad1845(card: *mut snd_card, mut port: c_uint, irq: c_int, dma1: c_int, dma2: c_int) -> c_int {
    let sscape = get_card_soundscape(card);
    let mut chip: *mut snd_wss = ptr::null_mut();
    let mut codec_type = WSS_HW_DETECT;
    match (*sscape).type_ {
        card_type::MEDIA_FX | card_type::SSCAPE => {
            if (*sscape).ic_type != IC_OPUS {
                codec_type = WSS_HW_AD1848;
            }
        }
        card_type::SSCAPE_VIVO => {
            port += 4;
        }
        _ => {}
    }
    let mut err = snd_wss_create(card, port as c_ulong, -1, irq, dma1, dma2, codec_type, WSS_HWSHARE_DMA1, &mut chip);
    if err == 0 {
        if (*sscape).type_ != card_type::SSCAPE_VIVO {
            snd_wss_mce_up(chip);
            let mut flags: c_ulong = 0;
            spin_lock_irqsave(&mut (*chip).reg_lock, &mut flags);
            snd_wss_out(chip, AD1845_CLOCK, 0x20);
            spin_unlock_irqrestore(&mut (*chip).reg_lock, flags);
            snd_wss_mce_down(chip);
        }
        err = snd_wss_pcm(chip, 0);
        if err < 0 {
            dev_err((*card).dev, b"sscape: No PCM device for AD1845 chip\n\0".as_ptr() as *const c_char);
            return err;
        }
        err = snd_wss_mixer(chip);
        if err < 0 {
            dev_err((*card).dev, b"sscape: No mixer device for AD1845 chip\n\0".as_ptr() as *const c_char);
            return err;
        }
        if (*chip).hardware != WSS_HW_AD1848 {
            err = snd_wss_timer(chip, 0);
            if err < 0 {
                dev_err((*card).dev, b"sscape: No timer device for AD1845 chip\n\0".as_ptr() as *const c_char);
                return err;
            }
        }
        if (*sscape).type_ != card_type::SSCAPE_VIVO {
            err = snd_ctl_add(card, snd_ctl_new1(&midi_mixer_ctl, chip as *mut c_void));
            if err < 0 {
                dev_err((*card).dev, b"sscape: Could not create MIDI mixer control\n\0".as_ptr() as *const c_char);
                return err;
            }
        }
        (*sscape).chip = chip;
    }
    err
}

unsafe fn create_sscape(card: *mut snd_card) -> c_int {
    let sscape = get_card_soundscape(card);
    let io_res = devm_request_region((*card).dev, (*sscape).io_base as c_ulong, 8, b"SoundScape\0".as_ptr() as *const c_char);
    if io_res.is_null() {
        dev_err((*card).dev, b"sscape: can't grab port 0x%x\n\0".as_ptr() as *const c_char, (*sscape).io_base);
        return -EBUSY;
    }
    let mut wss_res: *mut resource = ptr::null_mut();
    if (*sscape).type_ == card_type::SSCAPE_VIVO {
        wss_res = devm_request_region((*card).dev, (*sscape).wss_base, 4, b"SoundScape\0".as_ptr() as *const c_char);
        if wss_res.is_null() {
            dev_err((*card).dev, b"sscape: can't grab port 0x%lx\n\0".as_ptr() as *const c_char, (*sscape).wss_base);
            return -EBUSY;
        }
    }
    let mut err = snd_devm_request_dma((*card).dev, (*sscape).dma1, b"SoundScape\0".as_ptr() as *const c_char);
    if err < 0 {
        dev_err((*card).dev, b"sscape: can't grab DMA %d\n\0".as_ptr() as *const c_char, (*sscape).dma1);
        return err;
    }
    spin_lock_init(&mut (*sscape).lock);
    (*sscape).io_res = io_res;
    (*sscape).wss_res = wss_res;
    if detect_sscape(sscape, (*sscape).wss_base as c_long) == 0 {
        dev_err((*card).dev, b"sscape: hardware not detected at 0x%x\n\0".as_ptr() as *const c_char, (*sscape).io_base);
        return -ENODEV;
    }
    let name = match (*sscape).type_ {
        card_type::MEDIA_FX => b"MediaFX/SoundFX\0".as_ptr() as *const c_char,
        card_type::SSCAPE => b"Soundscape\0".as_ptr() as *const c_char,
        card_type::SSCAPE_PNP => b"Soundscape PnP\0".as_ptr() as *const c_char,
        card_type::SSCAPE_VIVO => b"Soundscape VIVO\0".as_ptr() as *const c_char,
    };
    dev_info((*card).dev, b"sscape: %s card detected at 0x%x, using IRQ %d, DMA %d\n\0".as_ptr() as *const c_char, name, (*sscape).io_base, (*sscape).irq, (*sscape).dma1);
    err = sscape_configure_board(sscape);
    if err < 0 {
        dev_err((*card).dev, b"sscape: Invalid IRQ configuration\n\0".as_ptr() as *const c_char);
        return err;
    }
    err = create_ad1845(card, (*sscape).wss_base as c_uint, (*sscape).irq, (*sscape).dma1, (*sscape).dma2);
    if err < 0 {
        dev_err((*card).dev, b"sscape: No AD1845 device at 0x%lx, IRQ %d\n\0".as_ptr() as *const c_char, (*sscape).wss_base, (*sscape).irq);
        return err;
    }
    strscpy((*card).driver.as_mut_ptr(), b"SoundScape\0".as_ptr() as *const c_char);
    strscpy((*card).shortname.as_mut_ptr(), name);
    snprintf((*card).longname.as_mut_ptr(), (*card).longname.len(), b"%s at 0x%lx, IRQ %d, DMA1 %d, DMA2 %d\n\0".as_ptr() as *const c_char, name, (*(*sscape).chip).port, (*(*sscape).chip).irq, (*(*sscape).chip).dma1, (*(*sscape).chip).dma2);
    if (*sscape).type_ != card_type::SSCAPE_VIVO {
        err = sscape_upload_bootblock(card);
        if err >= 0 {
            err = sscape_upload_microcode(card, err);
        }
        if err == 0 {
            err = create_mpu401(card, MIDI_DEVNUM, (*sscape).io_base as c_ulong, (*sscape).mpu_irq);
            if err < 0 {
                dev_err((*card).dev, b"sscape: Failed to create MPU-401 device at 0x%lx\n\0".as_ptr() as *const c_char, (*sscape).io_base as c_ulong);
                return err;
            }
            (*sscape).midi_vol = 0;
            (*sscape).midi_enabled = true;
            err = sscape_restore_midi_state(sscape);
            if err < 0 {
                dev_warn((*card).dev, b"sscape: MIDI init incomplete: %d\n\0".as_ptr() as *const c_char, err);
            }
        }
    }
    0
}

/* CONFIG_PM: suspend/resume support is conditionally compiled in the C source. */
unsafe fn sscape_resume_midi(card: *mut snd_card) -> c_int {
    let sscape = get_card_soundscape(card);
    if !(*sscape).midi_enabled {
        return 0;
    }
    let version = sscape_upload_bootblock(card);
    if version < 0 {
        return version;
    }
    let err = sscape_upload_microcode(card, version);
    if err < 0 {
        return err;
    }
    outb(0, (*sscape).io_base as c_ulong);
    sscape_restore_midi_state(sscape)
}

unsafe fn snd_sscape_suspend_card(card: *mut snd_card) -> c_int {
    let sscape = get_card_soundscape(card);
    snd_power_change_state(card, SNDRV_CTL_POWER_D3HOT);
    if let Some(suspend) = (*(*sscape).chip).suspend {
        suspend((*sscape).chip);
    }
    0
}

unsafe fn snd_sscape_resume_card(card: *mut snd_card) -> c_int {
    let sscape = get_card_soundscape(card);
    let mut err = sscape_configure_board(sscape);
    if err < 0 {
        return err;
    }
    err = sscape_resume_midi(card);
    if err < 0 {
        dev_warn((*card).dev, b"sscape: MIDI restore failed: %d\n\0".as_ptr() as *const c_char, err);
    }
    if let Some(resume) = (*(*sscape).chip).resume {
        resume((*sscape).chip);
    }
    snd_power_change_state(card, SNDRV_CTL_POWER_D0);
    0
}

unsafe extern "C" fn snd_sscape_suspend(dev: *mut device, _n: c_uint, _state: pm_message_t) -> c_int {
    snd_sscape_suspend_card(dev_get_drvdata(dev) as *mut snd_card)
}

unsafe extern "C" fn snd_sscape_resume(dev: *mut device, _n: c_uint) -> c_int {
    snd_sscape_resume_card(dev_get_drvdata(dev) as *mut snd_card)
}

unsafe extern "C" fn snd_sscape_match(pdev: *mut device, i: c_uint) -> c_int {
    if port[i as usize] == SNDRV_AUTO_PORT {
        return 0;
    }
    if irq[i as usize] == SNDRV_AUTO_IRQ || mpu_irq[i as usize] == SNDRV_AUTO_IRQ || dma[i as usize] == SNDRV_AUTO_DMA {
        dev_info(pdev, b"sscape: insufficient parameters, need IO, IRQ, MPU-IRQ and DMA\n\0".as_ptr() as *const c_char);
        return 0;
    }
    1
}

unsafe extern "C" fn snd_sscape_probe(pdev: *mut device, dev: c_uint) -> c_int {
    let mut card: *mut snd_card = ptr::null_mut();
    let mut ret = snd_devm_card_new(pdev, index[dev as usize], id[dev as usize], THIS_MODULE, size_of::<soundscape>(), &mut card);
    if ret < 0 {
        return ret;
    }
    let sscape = get_card_soundscape(card);
    (*sscape).dev = pdev;
    (*sscape).type_ = card_type::SSCAPE;
    dma[dev as usize] &= 0x03;
    sscape_store_settings(sscape, dev as c_int);
    ret = create_sscape(card);
    if ret < 0 {
        return ret;
    }
    ret = snd_card_register(card);
    if ret < 0 {
        dev_err(pdev, b"sscape: Failed to register sound card\n\0".as_ptr() as *const c_char);
        return ret;
    }
    dev_set_drvdata(pdev, card as *mut c_void);
    0
}

const DEV_NAME: *const c_char = b"sscape\0".as_ptr() as *const c_char;

static mut snd_sscape_driver: isa_driver = isa_driver {
    match_: Some(snd_sscape_match),
    probe: Some(snd_sscape_probe),
    suspend: Some(snd_sscape_suspend),
    resume: Some(snd_sscape_resume),
    driver: driver_inner { name: DEV_NAME },
};

/* CONFIG_PNP: ISA PnP IDs and driver are conditionally compiled in the C source. */
static mut isa_registered: c_int = 0;
static mut pnp_registered: c_int = 0;

static sscape_pnpids: [pnp_card_device_id; 3] = [
    pnp_card_device_id { id: *b"ENS3081\0", devs: [pnp_id { id: *b"ENS0000\0" }] },
    pnp_card_device_id { id: *b"ENS4081\0", devs: [pnp_id { id: *b"ENS1011\0" }] },
    pnp_card_device_id { id: *b"\0\0\0\0\0\0\0\0", devs: [pnp_id { id: *b"\0\0\0\0\0\0\0\0" }] },
];

#[inline]
unsafe fn get_next_autoindex(mut i: c_int) -> c_int {
    while (i as usize) < SNDRV_CARDS && port[i as usize] != SNDRV_AUTO_PORT {
        i += 1;
    }
    i
}

unsafe extern "C" fn sscape_pnp_detect(pcard: *mut pnp_card_link, pid: *const pnp_card_device_id) -> c_int {
    static mut IDX: c_int = 0;
    IDX = get_next_autoindex(IDX);
    if IDX as usize >= SNDRV_CARDS {
        return -ENOSPC;
    }
    let dev = pnp_request_card_device(pcard, (*pid).devs[0].id.as_ptr(), ptr::null_mut());
    if dev.is_null() {
        return -ENODEV;
    }
    if !pnp_is_active(dev) {
        if pnp_activate_dev(dev) < 0 {
            dev_info(&mut (*dev).dev, b"sscape: device is inactive\n\0".as_ptr() as *const c_char);
            return -EBUSY;
        }
    }
    let mut card: *mut snd_card = ptr::null_mut();
    let mut ret = snd_devm_card_new(&mut (*(*pcard).card).dev, index[IDX as usize], id[IDX as usize], THIS_MODULE, size_of::<soundscape>(), &mut card);
    if ret < 0 {
        return ret;
    }
    let sscape = get_card_soundscape(card);
    (*sscape).dev = (*card).dev;
    if strncmp(b"ENS4081\0".as_ptr() as *const c_char, (*pid).id.as_ptr(), 7) == 0 {
        (*sscape).type_ = card_type::SSCAPE_VIVO;
    } else {
        (*sscape).type_ = card_type::SSCAPE_PNP;
    }
    port[IDX as usize] = pnp_port_start(dev, 0) as c_long;
    irq[IDX as usize] = pnp_irq(dev, 0);
    mpu_irq[IDX as usize] = pnp_irq(dev, 1);
    dma[IDX as usize] = pnp_dma(dev, 0) & 0x03;
    if (*sscape).type_ == card_type::SSCAPE_PNP {
        dma2[IDX as usize] = dma[IDX as usize];
        wss_port[IDX as usize] = CODEC_IO(port[IDX as usize]);
    } else {
        wss_port[IDX as usize] = pnp_port_start(dev, 1) as c_long;
        dma2[IDX as usize] = pnp_dma(dev, 1);
    }
    sscape_store_settings(sscape, IDX);
    ret = create_sscape(card);
    if ret < 0 {
        return ret;
    }
    ret = snd_card_register(card);
    if ret < 0 {
        dev_err((*card).dev, b"sscape: Failed to register sound card\n\0".as_ptr() as *const c_char);
        return ret;
    }
    pnp_set_card_drvdata(pcard, card as *mut c_void);
    IDX += 1;
    0
}

unsafe extern "C" fn sscape_pnp_suspend(pcard: *mut pnp_card_link, _state: pm_message_t) -> c_int {
    snd_sscape_suspend_card(pnp_get_card_drvdata(pcard) as *mut snd_card)
}

unsafe extern "C" fn sscape_pnp_resume(pcard: *mut pnp_card_link) -> c_int {
    snd_sscape_resume_card(pnp_get_card_drvdata(pcard) as *mut snd_card)
}

static mut sscape_pnpc_driver: pnp_card_driver = pnp_card_driver {
    flags: PNP_DRIVER_RES_DO_NOT_CHANGE,
    name: b"sscape\0".as_ptr() as *const c_char,
    id_table: sscape_pnpids.as_ptr(),
    probe: Some(sscape_pnp_detect),
    suspend: Some(sscape_pnp_suspend),
    resume: Some(sscape_pnp_resume),
};

unsafe fn sscape_init() -> c_int {
    let mut err = isa_register_driver(&mut snd_sscape_driver, SNDRV_CARDS as c_uint);
    if err == 0 {
        isa_registered = 1;
    }
    err = pnp_register_card_driver(&mut sscape_pnpc_driver);
    if err == 0 {
        pnp_registered = 1;
    }
    if isa_registered != 0 {
        err = 0;
    }
    err
}

unsafe fn sscape_exit() {
    if pnp_registered != 0 {
        pnp_unregister_card_driver(&mut sscape_pnpc_driver);
    }
    if isa_registered != 0 {
        isa_unregister_driver(&mut snd_sscape_driver);
    }
}


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
