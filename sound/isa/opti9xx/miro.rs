// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   ALSA soundcard driver for Miro miroSOUND PCM1 pro
 *                                  miroSOUND PCM12
 *                                  miroSOUND PCM20 Radio
 *
 *   Copyright (C) 2004-2005 Martin Langer <martin-langer@gmx.de>
 *
 *   Based on OSS ACI and ALSA OPTi9xx drivers
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_uchar, c_ushort, c_void};
use core::ptr;

/* Dependencies supplied by the original Linux/ALSA headers:
 * linux/init.h, linux/err.h, linux/isa.h, linux/pnp.h, linux/delay.h,
 * linux/ioport.h, linux/module.h, linux/io.h, asm/dma.h, sound/core.h,
 * sound/wss.h, sound/mpu401.h, sound/opl4.h, sound/control.h, sound/info.h,
 * sound/initval.h, sound/aci.h.
 */

extern "C" {
    static mut SNDRV_DEFAULT_IDX1: c_int;
    static mut SNDRV_DEFAULT_STR1: *mut c_char;
    static mut SNDRV_DEFAULT_PORT1: c_long;
    static mut SNDRV_DEFAULT_IRQ1: c_int;
    static mut SNDRV_DEFAULT_DMA1: c_int;
    static mut SNDRV_AUTO_PORT: c_long;
    static mut SNDRV_AUTO_IRQ: c_int;
    static mut SNDRV_AUTO_DMA: c_int;
    static mut THIS_MODULE: *mut c_void;
    static mut HZ: c_long;

    fn inb(port: c_ulong) -> c_uchar;
    fn outb(value: c_uchar, port: c_ulong);
    fn set_current_state(state: c_long);
    fn schedule_timeout(timeout: c_long) -> c_long;
    fn mutex_lock_interruptible(mutex: *mut mutex) -> c_int;
    fn mutex_unlock(mutex: *mut mutex);
    fn mutex_init(mutex: *mut mutex);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t) -> c_ulong;
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn snd_ctl_boolean_mono_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut c_void;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_ctl_new1(ncontrol: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn snd_BUG_ON(cond: bool) -> c_int;
    fn snd_BUG();
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...);
    fn snd_card_ro_proc_new(
        card: *mut snd_card,
        name: *const c_char,
        data: *mut c_void,
        read: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
    ) -> c_int;
    fn devm_request_region(
        dev: *mut device,
        start: c_ulong,
        n: c_ulong,
        name: *const c_char,
    ) -> *mut resource;
    fn devm_release_resource(dev: *mut device, res: *mut resource);
    fn snd_wss_create(
        card: *mut snd_card,
        port: c_ulong,
        cport: c_long,
        irq: c_int,
        dma1: c_int,
        dma2: c_int,
        hardware: c_int,
        hwshare: c_int,
        rchip: *mut *mut snd_wss,
    ) -> c_int;
    fn snd_wss_pcm(codec: *mut snd_wss, device: c_int) -> c_int;
    fn snd_wss_mixer(codec: *mut snd_wss) -> c_int;
    fn snd_wss_timer(codec: *mut snd_wss, device: c_int) -> c_int;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn snd_mpu401_uart_new(
        card: *mut snd_card,
        device: c_int,
        hardware: c_int,
        port: c_ulong,
        integrated: c_int,
        irq: c_int,
        rrawmidi: *mut *mut snd_rawmidi,
    ) -> c_int;
    fn snd_opl4_create(
        card: *mut snd_card,
        fm_port: c_long,
        pcm_port: c_long,
        seq_device: c_int,
        ropl3: *mut *mut snd_opl3,
        ropl4: *mut *mut snd_opl4,
    ) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn snd_devm_card_new(
        dev: *mut device,
        idx: c_int,
        xid: *mut c_char,
        module: *mut c_void,
        extra_size: usize,
        card_ret: *mut *mut snd_card,
    ) -> c_int;
    fn snd_legacy_find_free_ioport(possible_ports: *const c_long, size: c_int) -> c_long;
    fn snd_legacy_find_free_irq(possible_irqs: *const c_int) -> c_int;
    fn snd_legacy_find_free_dma(possible_dmas: *const c_int) -> c_int;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn snd_power_change_state(card: *mut snd_card, state: c_int);
    fn isa_register_driver(driver: *mut isa_driver, ndev: c_uint) -> c_int;
    fn isa_unregister_driver(driver: *mut isa_driver);

    /* CONFIG_PNP */
    fn pnp_request_card_device(card: *mut pnp_card_link, id: *const c_char, from: *mut pnp_dev) -> *mut pnp_dev;
    fn pnp_activate_dev(dev: *mut pnp_dev) -> c_int;
    fn pnp_port_start(dev: *mut pnp_dev, bar: c_uint) -> c_ulong;
    fn pnp_port_len(dev: *mut pnp_dev, bar: c_uint) -> c_ulong;
    fn pnp_irq(dev: *mut pnp_dev, bar: c_uint) -> c_int;
    fn pnp_dma(dev: *mut pnp_dev, bar: c_uint) -> c_int;
    fn pnp_set_card_drvdata(card: *mut pnp_card_link, data: *mut c_void);
    fn pnp_get_card_drvdata(card: *mut pnp_card_link) -> *mut c_void;
    fn pnp_register_card_driver(driver: *mut pnp_card_driver) -> c_int;
    fn pnp_unregister_card_driver(driver: *mut pnp_card_driver);
}

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct resource { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm { pub name: [c_char; 80] }
#[repr(C)] pub struct snd_rawmidi { _private: [u8; 0] }
#[repr(C)] pub struct snd_opl3 { _private: [u8; 0] }
#[repr(C)] pub struct snd_opl4 { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct pm_message_t { _private: [u8; 0] }

#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
    pub private_data: *mut c_void,
    pub mixername: [c_char; 80],
    pub shortname: [c_char; 32],
    pub driver: [c_char; 16],
    pub longname: [c_char; 80],
}

#[repr(C)]
pub struct snd_wss {
    pub pcm: *mut snd_pcm,
    pub suspend: Option<unsafe extern "C" fn(*mut snd_wss)>,
    pub resume: Option<unsafe extern "C" fn(*mut snd_wss)>,
}

#[repr(C)]
pub struct snd_miro_aci {
    pub card: *mut snd_card,
    pub aci_mutex: mutex,
    pub aci_port: c_ulong,
    pub aci_vendor: c_int,
    pub aci_product: c_int,
    pub aci_version: c_int,
    pub aci_amp: c_int,
    pub aci_preamp: c_int,
    pub aci_solomode: c_int,
}

#[repr(C)] pub struct snd_kcontrol { pub private_value: c_ulong }
#[repr(C)] pub struct snd_info_entry { pub private_data: *mut c_void }
#[repr(C)] pub struct snd_info_buffer { _private: [u8; 0] }

#[repr(C)]
pub struct snd_ctl_elem_info_integer { pub min: c_long, pub max: c_long }
#[repr(C)] pub union snd_ctl_elem_info_value { pub integer: snd_ctl_elem_info_integer }
#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: c_uint,
    pub count: c_uint,
    pub value: snd_ctl_elem_info_value,
}

#[repr(C)] pub struct snd_ctl_elem_value_integer { pub value: [c_long; 128] }
#[repr(C)] pub union snd_ctl_elem_value_value { pub integer: snd_ctl_elem_value_integer }
#[repr(C)] pub struct snd_ctl_elem_value { pub value: snd_ctl_elem_value_value }

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_uint,
    pub name: *const c_char,
    pub index: c_uint,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub private_value: c_ulong,
}

#[repr(C)] pub struct device_driver { pub name: *const c_char }
#[repr(C)]
pub struct isa_driver {
    pub match_: Option<unsafe extern "C" fn(*mut device, c_uint) -> c_int>,
    pub probe: Option<unsafe extern "C" fn(*mut device, c_uint) -> c_int>,
    pub suspend: Option<unsafe extern "C" fn(*mut device, c_uint, pm_message_t) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut device, c_uint) -> c_int>,
    pub driver: device_driver,
}

#[repr(C)] pub struct pnp_dev { _private: [u8; 0] }
#[repr(C)] pub struct pnp_card { pub dev: device }
#[repr(C)] pub struct pnp_card_link { pub card: *mut pnp_card }
#[repr(C)] pub struct pnp_id { pub id: [c_char; 8] }
#[repr(C)] pub struct pnp_card_device_id { pub id: [c_char; 8], pub devs: [pnp_id; 3] }
#[repr(C)]
pub struct pnp_card_driver {
    pub flags: c_uint,
    pub name: *const c_char,
    pub id_table: *const pnp_card_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut pnp_card_link, *const pnp_card_device_id) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut pnp_card_link)>,
    pub suspend: Option<unsafe extern "C" fn(*mut pnp_card_link, pm_message_t) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut pnp_card_link) -> c_int>,
}

const TASK_UNINTERRUPTIBLE: c_long = 2;
const EBUSY: c_int = 16;
const EINTR: c_int = 4;
const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;
const ENXIO: c_int = 6;
const SNDRV_CTL_ELEM_TYPE_INTEGER: c_uint = 2;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_uint = 2;
const WSS_HW_DETECT: c_int = 0;
const MPU401_HW_MPU401: c_int = 1;
const SNDRV_CTL_POWER_D3hot: c_int = 3;
const SNDRV_CTL_POWER_D0: c_int = 0;
const PNP_DRIVER_RES_DISABLE: c_uint = 1;

extern "C" {
    static ACI_MINTIME: c_long;
    static ACI_REG_BUSY: c_ulong;
    static ACI_REG_COMMAND: c_ulong;
    static ACI_REG_STATUS: c_ulong;
    static ACI_STATUS: c_int;
    static ACI_S_GENERAL: c_int;
    static ACI_SET_SOLOMODE: c_int;
    static ACI_GET_PREAMP: c_int;
    static ACI_SET_PREAMP: c_int;
    static ACI_SET_POWERAMP: c_int;
    static ACI_GET_EQ1: c_int;
    static ACI_GET_EQ2: c_int;
    static ACI_GET_EQ3: c_int;
    static ACI_GET_EQ4: c_int;
    static ACI_GET_EQ5: c_int;
    static ACI_GET_EQ6: c_int;
    static ACI_GET_EQ7: c_int;
    static ACI_SET_MASTER: c_int;
    static ACI_SET_MIC: c_int;
    static ACI_SET_LINE: c_int;
    static ACI_SET_CD: c_int;
    static ACI_SET_SYNTH: c_int;
    static ACI_SET_PCM: c_int;
    static ACI_SET_LINE1: c_int;
    static ACI_SET_LINE2: c_int;
    static ACI_SET_EQ1: c_int;
    static ACI_SET_EQ2: c_int;
    static ACI_SET_EQ3: c_int;
    static ACI_SET_EQ4: c_int;
    static ACI_SET_EQ5: c_int;
    static ACI_SET_EQ6: c_int;
    static ACI_SET_EQ7: c_int;
    static ACI_GET_MASTER: c_int;
    static ACI_GET_MIC: c_int;
    static ACI_GET_LINE: c_int;
    static ACI_GET_CD: c_int;
    static ACI_GET_SYNTH: c_int;
    static ACI_GET_PCM: c_int;
    static ACI_GET_LINE1: c_int;
    static ACI_GET_LINE2: c_int;
    static ACI_SET_MUTE: c_int;
    static ACI_SET_WSS: c_int;
    static ACI_SET_IDE: c_int;
    static ACI_ERROR_OP: c_int;
    static ACI_INIT: c_int;
    static ACI_READ_IDCODE: c_int;
    static ACI_READ_VERSION: c_int;
}

/* Module metadata and module_param declarations from C are kernel build metadata. */

static mut index: c_int = 0;
static mut id: *mut c_char = ptr::null_mut();
static mut port: c_long = 0;
static mut mpu_port: c_long = 0;
static mut fm_port: c_long = 0;
static mut irq: c_int = 0;
static mut mpu_irq: c_int = 0;
static mut dma1: c_int = 0;
static mut dma2: c_int = 0;
static mut wss: c_int = 0;
static mut ide: c_int = 0;
/* CONFIG_PNP */
static mut isapnp: bool = true;

const OPTi9XX_HW_DETECT: c_ushort = 0;
const OPTi9XX_HW_82C928: c_ushort = 1;
const OPTi9XX_HW_82C929: c_ushort = 2;
const OPTi9XX_HW_82C924: c_ushort = 3;
const OPTi9XX_HW_82C925: c_ushort = 4;
const OPTi9XX_HW_82C930: c_ushort = 5;
const OPTi9XX_HW_82C931: c_ushort = 6;
const OPTi9XX_HW_82C933: c_ushort = 7;
const OPTi9XX_HW_LAST: c_ushort = OPTi9XX_HW_82C933;

const fn OPTi9XX_MC_REG(n: c_uchar) -> c_uchar { n }

const MIRO_ACI_MASTER: usize = 0;
const MIRO_ACI_MIC: usize = 1;
const MIRO_ACI_LINE: usize = 2;
const MIRO_ACI_CD: usize = 3;
const MIRO_ACI_SYNTH: usize = 4;
const MIRO_ACI_PCM: usize = 5;
const MIRO_ACI_LINE1: usize = 6;
const MIRO_ACI_LINE2: usize = 7;
const MIRO_ACI_EQ1: usize = 8;
const MIRO_ACI_EQ2: usize = 9;
const MIRO_ACI_EQ3: usize = 10;
const MIRO_ACI_EQ4: usize = 11;
const MIRO_ACI_EQ5: usize = 12;
const MIRO_ACI_EQ6: usize = 13;
const MIRO_ACI_EQ7: usize = 14;
const MIRO_ACI_COUNT: usize = 15;

#[repr(C)]
pub struct snd_miro {
    pub hardware: c_ushort,
    pub password: c_uchar,
    pub name: [c_char; 7],
    pub res_mc_base: *mut resource,
    pub res_aci_port: *mut resource,
    pub mc_base: c_ulong,
    pub mc_base_size: c_ulong,
    pub pwd_reg: c_ulong,
    pub lock: spinlock_t,
    pub pcm: *mut snd_pcm,
    pub codec: *mut snd_wss,
    pub wss_base: c_long,
    pub irq: c_int,
    pub dma1: c_int,
    pub dma2: c_int,
    pub mpu_port: c_long,
    pub mpu_irq: c_int,
    pub card: *mut snd_card,
    pub aci: *mut snd_miro_aci,
    /* CONFIG_PM */
    pub aci_saved: [[c_uchar; 2]; MIRO_ACI_COUNT],
    pub aci_saved_amp: c_uchar,
    pub aci_saved_preamp: c_uchar,
    pub aci_saved_solomode: c_uchar,
}

static mut aci_device: snd_miro_aci = snd_miro_aci {
    card: ptr::null_mut(),
    aci_mutex: mutex { _private: [] },
    aci_port: 0,
    aci_vendor: 0,
    aci_product: 0,
    aci_version: 0,
    aci_amp: 0,
    aci_preamp: 0,
    aci_solomode: 0,
};

static snd_opti9xx_names: [*const c_char; 8] = [
    b"unknown\0".as_ptr() as *const c_char,
    b"82C928\0".as_ptr() as *const c_char,
    b"82C929\0".as_ptr() as *const c_char,
    b"82C924\0".as_ptr() as *const c_char,
    b"82C925\0".as_ptr() as *const c_char,
    b"82C930\0".as_ptr() as *const c_char,
    b"82C931\0".as_ptr() as *const c_char,
    b"82C933\0".as_ptr() as *const c_char,
];

static mut snd_miro_pnp_is_probed: c_int = 0;

/* CONFIG_PNP */
static snd_miro_pnpids: [pnp_card_device_id; 2] = [
    pnp_card_device_id {
        id: *b"MIR0924\0",
        devs: [
            pnp_id { id: *b"MIR0000\0" },
            pnp_id { id: *b"MIR0002\0" },
            pnp_id { id: *b"MIR0005\0" },
        ],
    },
    pnp_card_device_id { id: *b"\0\0\0\0\0\0\0\0", devs: [pnp_id { id: *b"\0\0\0\0\0\0\0\0" }, pnp_id { id: *b"\0\0\0\0\0\0\0\0" }, pnp_id { id: *b"\0\0\0\0\0\0\0\0" }] },
];

/*
 *  ACI control
 */

unsafe extern "C" fn aci_busy_wait(aci: *mut snd_miro_aci) -> c_int {
    let mut timeout: c_long = 1;
    while timeout <= ACI_MINTIME + 30 {
        let byte = inb((*aci).aci_port + ACI_REG_BUSY);
        if (byte & 1) == 0 {
            if timeout >= ACI_MINTIME {
                dev_dbg((*(*aci).card).dev, b"aci ready in round %ld.\n\0".as_ptr() as *const c_char, timeout - ACI_MINTIME);
            }
            return byte as c_int;
        }
        if timeout >= ACI_MINTIME {
            let mut out = 10 * HZ;
            match timeout - ACI_MINTIME {
                0..=9 => {
                    out /= 10;
                    out /= 10;
                    out /= 10;
                    set_current_state(TASK_UNINTERRUPTIBLE);
                    schedule_timeout(out);
                }
                10..=19 => {
                    out /= 10;
                    out /= 10;
                    set_current_state(TASK_UNINTERRUPTIBLE);
                    schedule_timeout(out);
                }
                20..=30 => {
                    out /= 10;
                    set_current_state(TASK_UNINTERRUPTIBLE);
                    schedule_timeout(out);
                }
                _ => {
                    set_current_state(TASK_UNINTERRUPTIBLE);
                    schedule_timeout(out);
                }
            }
        }
        timeout += 1;
    }
    dev_err((*(*aci).card).dev, b"%s() time out\n\0".as_ptr() as *const c_char, b"aci_busy_wait\0".as_ptr() as *const c_char);
    -EBUSY
}

unsafe extern "C" fn aci_write(aci: *mut snd_miro_aci, byte: c_uchar) -> c_int {
    if aci_busy_wait(aci) >= 0 {
        outb(byte, (*aci).aci_port + ACI_REG_COMMAND);
        0
    } else {
        dev_err((*(*aci).card).dev, b"aci busy, %s(0x%x) stopped.\n\0".as_ptr() as *const c_char, b"aci_write\0".as_ptr() as *const c_char, byte as c_int);
        -EBUSY
    }
}

unsafe extern "C" fn aci_read(aci: *mut snd_miro_aci) -> c_int {
    if aci_busy_wait(aci) >= 0 {
        inb((*aci).aci_port + ACI_REG_STATUS) as c_int
    } else {
        dev_err((*(*aci).card).dev, b"aci busy, %s() stopped.\n\0".as_ptr() as *const c_char, b"aci_read\0".as_ptr() as *const c_char);
        -EBUSY
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_aci_cmd(aci: *mut snd_miro_aci, write1: c_int, write2: c_int, write3: c_int) -> c_int {
    let write = [write1, write2, write3];
    let mut value: c_int;
    let mut i = 0;

    if mutex_lock_interruptible(&mut (*aci).aci_mutex) != 0 {
        return -EINTR;
    }

    while i < 3 {
        if write[i] < 0 || write[i] > 255 {
            break;
        } else {
            value = aci_write(aci, write[i] as c_uchar);
            if value < 0 {
                mutex_unlock(&mut (*aci).aci_mutex);
                return value;
            }
        }
        i += 1;
    }

    value = aci_read(aci);
    mutex_unlock(&mut (*aci).aci_mutex);
    value
}

unsafe extern "C" fn aci_getvalue(aci: *mut snd_miro_aci, index: c_uchar) -> c_int {
    snd_aci_cmd(aci, ACI_STATUS, index as c_int, -1)
}

unsafe extern "C" fn aci_setvalue(aci: *mut snd_miro_aci, index: c_uchar, value: c_int) -> c_int {
    snd_aci_cmd(aci, index as c_int, value, -1)
}

#[no_mangle]
pub unsafe extern "C" fn snd_aci_get_aci() -> *mut snd_miro_aci {
    if aci_device.aci_port == 0 { ptr::null_mut() } else { &mut aci_device }
}

/*
 *  MIXER part
 */

unsafe extern "C" fn snd_miro_get_capture(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let miro = snd_kcontrol_chip(kcontrol) as *mut snd_miro;
    let value = aci_getvalue((*miro).aci, ACI_S_GENERAL as c_uchar);
    if value < 0 {
        dev_err((*(*miro).card).dev, b"%s() failed: %d\n\0".as_ptr() as *const c_char, b"snd_miro_get_capture\0".as_ptr() as *const c_char, value);
        return value;
    }
    (*ucontrol).value.integer.value[0] = (value & 0x20) as c_long;
    0
}

unsafe extern "C" fn snd_miro_put_capture(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let miro = snd_kcontrol_chip(kcontrol) as *mut snd_miro;
    let value = if (*ucontrol).value.integer.value[0] == 0 { 1 } else { 0 };
    let error = aci_setvalue((*miro).aci, ACI_SET_SOLOMODE as c_uchar, value);
    if error < 0 {
        dev_err((*(*miro).card).dev, b"%s() failed: %d\n\0".as_ptr() as *const c_char, b"snd_miro_put_capture\0".as_ptr() as *const c_char, error);
        return error;
    }
    let change = (value != (*(*miro).aci).aci_solomode) as c_int;
    (*(*miro).aci).aci_solomode = value;
    change
}

unsafe extern "C" fn snd_miro_info_preamp(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 3;
    0
}

unsafe extern "C" fn snd_miro_get_preamp(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let miro = snd_kcontrol_chip(kcontrol) as *mut snd_miro;
    if (*(*miro).aci).aci_version <= 176 {
        /*
           OSS says it's not readable with versions < 176.
           But it doesn't work on my card,
           which is a PCM12 with aci_version = 176.
        */
        (*ucontrol).value.integer.value[0] = (*(*miro).aci).aci_preamp as c_long;
        return 0;
    }
    let value = aci_getvalue((*miro).aci, ACI_GET_PREAMP as c_uchar);
    if value < 0 {
        dev_err((*(*miro).card).dev, b"%s() failed: %d\n\0".as_ptr() as *const c_char, b"snd_miro_get_preamp\0".as_ptr() as *const c_char, value);
        return value;
    }
    (*ucontrol).value.integer.value[0] = value as c_long;
    0
}

unsafe extern "C" fn snd_miro_put_preamp(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let miro = snd_kcontrol_chip(kcontrol) as *mut snd_miro;
    let value = (*ucontrol).value.integer.value[0] as c_int;
    let error = aci_setvalue((*miro).aci, ACI_SET_PREAMP as c_uchar, value);
    if error < 0 {
        dev_err((*(*miro).card).dev, b"%s() failed: %d\n\0".as_ptr() as *const c_char, b"snd_miro_put_preamp\0".as_ptr() as *const c_char, error);
        return error;
    }
    let change = (value != (*(*miro).aci).aci_preamp) as c_int;
    (*(*miro).aci).aci_preamp = value;
    change
}

unsafe extern "C" fn snd_miro_get_amp(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let miro = snd_kcontrol_chip(kcontrol) as *mut snd_miro;
    (*ucontrol).value.integer.value[0] = (*(*miro).aci).aci_amp as c_long;
    0
}

unsafe extern "C" fn snd_miro_put_amp(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let miro = snd_kcontrol_chip(kcontrol) as *mut snd_miro;
    let value = (*ucontrol).value.integer.value[0] as c_int;
    let error = aci_setvalue((*miro).aci, ACI_SET_POWERAMP as c_uchar, value);
    if error < 0 {
        dev_err((*(*miro).card).dev, b"%s() to %d failed: %d\n\0".as_ptr() as *const c_char, b"snd_miro_put_amp\0".as_ptr() as *const c_char, value, error);
        return error;
    }
    let change = (value != (*(*miro).aci).aci_amp) as c_int;
    (*(*miro).aci).aci_amp = value;
    change
}

const fn MIRO_DOUBLE_VALUE(get_right_reg: c_int, set_right_reg: c_int) -> c_ulong {
    (get_right_reg as c_ulong) | ((set_right_reg as c_ulong) << 8)
}

unsafe extern "C" fn snd_miro_info_double(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    let reg = ((*kcontrol).private_value & 0xff) as c_int;
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 2;
    if reg >= ACI_GET_EQ1 && reg <= ACI_GET_EQ7 {
        /* equalizer elements */
        (*uinfo).value.integer.min = -0x7f;
        (*uinfo).value.integer.max = 0x7f;
    } else {
        /* non-equalizer elements */
        (*uinfo).value.integer.min = 0;
        (*uinfo).value.integer.max = 0x20;
    }
    0
}

unsafe extern "C" fn snd_miro_get_double(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_value) -> c_int {
    let miro = snd_kcontrol_chip(kcontrol) as *mut snd_miro;
    let right_reg = ((*kcontrol).private_value & 0xff) as c_int;
    let left_reg = right_reg + 1;
    let right_val = aci_getvalue((*miro).aci, right_reg as c_uchar);
    if right_val < 0 {
        dev_err((*(*miro).card).dev, b"aci_getvalue(%d) failed: %d\n\0".as_ptr() as *const c_char, right_reg, right_val);
        return right_val;
    }
    let left_val = aci_getvalue((*miro).aci, left_reg as c_uchar);
    if left_val < 0 {
        dev_err((*(*miro).card).dev, b"aci_getvalue(%d) failed: %d\n\0".as_ptr() as *const c_char, left_reg, left_val);
        return left_val;
    }
    if right_reg >= ACI_GET_EQ1 && right_reg <= ACI_GET_EQ7 {
        /* equalizer elements */
        (*uinfo).value.integer.value[0] = if left_val < 0x80 { left_val } else { 0x80 - left_val } as c_long;
        (*uinfo).value.integer.value[1] = if right_val < 0x80 { right_val } else { 0x80 - right_val } as c_long;
    } else {
        /* non-equalizer elements */
        (*uinfo).value.integer.value[0] = (0x20 - left_val) as c_long;
        (*uinfo).value.integer.value[1] = (0x20 - right_val) as c_long;
    }
    0
}

unsafe extern "C" fn snd_miro_put_double(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let miro = snd_kcontrol_chip(kcontrol) as *mut snd_miro;
    let aci = (*miro).aci;
    let left = (*ucontrol).value.integer.value[0] as c_int;
    let right = (*ucontrol).value.integer.value[1] as c_int;
    let setreg_right = (((*kcontrol).private_value >> 8) & 0xff) as c_int;
    let mut setreg_left = setreg_right + 8;
    if setreg_right == ACI_SET_MASTER {
        setreg_left -= 7;
    }
    let getreg_right = ((*kcontrol).private_value & 0xff) as c_int;
    let getreg_left = getreg_right + 1;

    let mut left_old = aci_getvalue(aci, getreg_left as c_uchar);
    if left_old < 0 {
        dev_err((*(*miro).card).dev, b"aci_getvalue(%d) failed: %d\n\0".as_ptr() as *const c_char, getreg_left, left_old);
        return left_old;
    }
    let mut right_old = aci_getvalue(aci, getreg_right as c_uchar);
    if right_old < 0 {
        dev_err((*(*miro).card).dev, b"aci_getvalue(%d) failed: %d\n\0".as_ptr() as *const c_char, getreg_right, right_old);
        return right_old;
    }

    if getreg_right >= ACI_GET_EQ1 && getreg_right <= ACI_GET_EQ7 {
        /* equalizer elements */
        if left < -0x7f || left > 0x7f || right < -0x7f || right > 0x7f {
            return -EINVAL;
        }
        if left_old > 0x80 { left_old = 0x80 - left_old; }
        if right_old > 0x80 { right_old = 0x80 - right_old; }

        let mut error;
        if left >= 0 {
            error = aci_setvalue(aci, setreg_left as c_uchar, left);
            if error < 0 {
                dev_err((*(*miro).card).dev, b"aci_setvalue(%d) failed: %d\n\0".as_ptr() as *const c_char, left, error);
                return error;
            }
        } else {
            error = aci_setvalue(aci, setreg_left as c_uchar, 0x80 - left);
            if error < 0 {
                dev_err((*(*miro).card).dev, b"aci_setvalue(%d) failed: %d\n\0".as_ptr() as *const c_char, 0x80 - left, error);
                return error;
            }
        }
        if right >= 0 {
            error = aci_setvalue(aci, setreg_right as c_uchar, right);
            if error < 0 {
                dev_err((*(*miro).card).dev, b"aci_setvalue(%d) failed: %d\n\0".as_ptr() as *const c_char, right, error);
                return error;
            }
        } else {
            error = aci_setvalue(aci, setreg_right as c_uchar, 0x80 - right);
            if error < 0 {
                dev_err((*(*miro).card).dev, b"aci_setvalue(%d) failed: %d\n\0".as_ptr() as *const c_char, 0x80 - right, error);
                return error;
            }
        }
    } else {
        /* non-equalizer elements */
        if left < 0 || left > 0x20 || right < 0 || right > 0x20 {
            return -EINVAL;
        }
        left_old = 0x20 - left_old;
        right_old = 0x20 - right_old;
        let mut error = aci_setvalue(aci, setreg_left as c_uchar, 0x20 - left);
        if error < 0 {
            dev_err((*(*miro).card).dev, b"aci_setvalue(%d) failed: %d\n\0".as_ptr() as *const c_char, 0x20 - left, error);
            return error;
        }
        error = aci_setvalue(aci, setreg_right as c_uchar, 0x20 - right);
        if error < 0 {
            dev_err((*(*miro).card).dev, b"aci_setvalue(%d) failed: %d\n\0".as_ptr() as *const c_char, 0x20 - right, error);
            return error;
        }
    }

    ((left != left_old) || (right != right_old)) as c_int
}

macro_rules! miro_double {
    ($ctl_name:expr, $ctl_index:expr, $get_right_reg:expr, $set_right_reg:expr) => {
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            name: $ctl_name.as_ptr() as *const c_char,
            index: $ctl_index,
            info: Some(snd_miro_info_double),
            get: Some(snd_miro_get_double),
            put: Some(snd_miro_put_double),
            private_value: MIRO_DOUBLE_VALUE($get_right_reg, $set_right_reg),
        }
    };
}

static snd_miro_controls: [snd_kcontrol_new; 7] = [
    miro_double!(b"Master Playback Volume\0", 0, ACI_GET_MASTER, ACI_SET_MASTER),
    miro_double!(b"Mic Playback Volume\0", 1, ACI_GET_MIC, ACI_SET_MIC),
    miro_double!(b"Line Playback Volume\0", 1, ACI_GET_LINE, ACI_SET_LINE),
    miro_double!(b"CD Playback Volume\0", 0, ACI_GET_CD, ACI_SET_CD),
    miro_double!(b"Synth Playback Volume\0", 0, ACI_GET_SYNTH, ACI_SET_SYNTH),
    miro_double!(b"PCM Playback Volume\0", 1, ACI_GET_PCM, ACI_SET_PCM),
    miro_double!(b"Aux Playback Volume\0", 2, ACI_GET_LINE2, ACI_SET_LINE2),
];

/* Equalizer with seven bands (only PCM20)
   from -12dB up to +12dB on each band */
static snd_miro_eq_controls: [snd_kcontrol_new; 7] = [
    miro_double!(b"Tone Control - 28 Hz\0", 0, ACI_GET_EQ1, ACI_SET_EQ1),
    miro_double!(b"Tone Control - 160 Hz\0", 0, ACI_GET_EQ2, ACI_SET_EQ2),
    miro_double!(b"Tone Control - 400 Hz\0", 0, ACI_GET_EQ3, ACI_SET_EQ3),
    miro_double!(b"Tone Control - 1 kHz\0", 0, ACI_GET_EQ4, ACI_SET_EQ4),
    miro_double!(b"Tone Control - 2.5 kHz\0", 0, ACI_GET_EQ5, ACI_SET_EQ5),
    miro_double!(b"Tone Control - 6.3 kHz\0", 0, ACI_GET_EQ6, ACI_SET_EQ6),
    miro_double!(b"Tone Control - 16 kHz\0", 0, ACI_GET_EQ7, ACI_SET_EQ7),
];

static snd_miro_radio_control: [snd_kcontrol_new; 1] = [
    miro_double!(b"Radio Playback Volume\0", 0, ACI_GET_LINE1, ACI_SET_LINE1),
];

static snd_miro_line_control: [snd_kcontrol_new; 1] = [
    miro_double!(b"Line Playback Volume\0", 2, ACI_GET_LINE1, ACI_SET_LINE1),
];

static snd_miro_preamp_control: [snd_kcontrol_new; 1] = [snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: b"Mic Boost\0".as_ptr() as *const c_char,
    index: 1,
    info: Some(snd_miro_info_preamp),
    get: Some(snd_miro_get_preamp),
    put: Some(snd_miro_put_preamp),
    private_value: 0,
}];

static snd_miro_amp_control: [snd_kcontrol_new; 1] = [snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: b"Line Boost\0".as_ptr() as *const c_char,
    index: 0,
    info: Some(snd_ctl_boolean_mono_info),
    get: Some(snd_miro_get_amp),
    put: Some(snd_miro_put_amp),
    private_value: 0,
}];

static snd_miro_capture_control: [snd_kcontrol_new; 1] = [snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: b"PCM Capture Switch\0".as_ptr() as *const c_char,
    index: 0,
    info: Some(snd_ctl_boolean_mono_info),
    get: Some(snd_miro_get_capture),
    put: Some(snd_miro_put_capture),
    private_value: 0,
}];

static aci_init_values: [[c_uchar; 2]; 20] = [
    [ACI_SET_MUTE as c_uchar, 0x00],
    [ACI_SET_POWERAMP as c_uchar, 0x00],
    [ACI_SET_PREAMP as c_uchar, 0x00],
    [ACI_SET_SOLOMODE as c_uchar, 0x00],
    [(ACI_SET_MIC + 0) as c_uchar, 0x20],
    [(ACI_SET_MIC + 8) as c_uchar, 0x20],
    [(ACI_SET_LINE + 0) as c_uchar, 0x20],
    [(ACI_SET_LINE + 8) as c_uchar, 0x20],
    [(ACI_SET_CD + 0) as c_uchar, 0x20],
    [(ACI_SET_CD + 8) as c_uchar, 0x20],
    [(ACI_SET_PCM + 0) as c_uchar, 0x20],
    [(ACI_SET_PCM + 8) as c_uchar, 0x20],
    [(ACI_SET_LINE1 + 0) as c_uchar, 0x20],
    [(ACI_SET_LINE1 + 8) as c_uchar, 0x20],
    [(ACI_SET_LINE2 + 0) as c_uchar, 0x20],
    [(ACI_SET_LINE2 + 8) as c_uchar, 0x20],
    [(ACI_SET_SYNTH + 0) as c_uchar, 0x20],
    [(ACI_SET_SYNTH + 8) as c_uchar, 0x20],
    [(ACI_SET_MASTER + 0) as c_uchar, 0x20],
    [(ACI_SET_MASTER + 1) as c_uchar, 0x20],
];

/* CONFIG_PM */
static snd_miro_saved_get_regs: [c_uchar; MIRO_ACI_COUNT] = [
    ACI_GET_MASTER as c_uchar, ACI_GET_MIC as c_uchar, ACI_GET_LINE as c_uchar,
    ACI_GET_CD as c_uchar, ACI_GET_SYNTH as c_uchar, ACI_GET_PCM as c_uchar,
    ACI_GET_LINE1 as c_uchar, ACI_GET_LINE2 as c_uchar, ACI_GET_EQ1 as c_uchar,
    ACI_GET_EQ2 as c_uchar, ACI_GET_EQ3 as c_uchar, ACI_GET_EQ4 as c_uchar,
    ACI_GET_EQ5 as c_uchar, ACI_GET_EQ6 as c_uchar, ACI_GET_EQ7 as c_uchar,
];

/* CONFIG_PM */
static snd_miro_saved_set_regs: [c_uchar; MIRO_ACI_COUNT] = [
    ACI_SET_MASTER as c_uchar, ACI_SET_MIC as c_uchar, ACI_SET_LINE as c_uchar,
    ACI_SET_CD as c_uchar, ACI_SET_SYNTH as c_uchar, ACI_SET_PCM as c_uchar,
    ACI_SET_LINE1 as c_uchar, ACI_SET_LINE2 as c_uchar, ACI_SET_EQ1 as c_uchar,
    ACI_SET_EQ2 as c_uchar, ACI_SET_EQ3 as c_uchar, ACI_SET_EQ4 as c_uchar,
    ACI_SET_EQ5 as c_uchar, ACI_SET_EQ6 as c_uchar, ACI_SET_EQ7 as c_uchar,
];

unsafe extern "C" fn snd_set_aci_init_values(miro: *mut snd_miro) -> c_int {
    let aci = (*miro).aci;

    /* enable WSS on PCM1 */
    if (*aci).aci_product == 'A' as c_int && wss != 0 {
        let error = aci_setvalue(aci, ACI_SET_WSS as c_uchar, wss);
        if error < 0 {
            dev_err((*(*miro).card).dev, b"enabling WSS mode failed\n\0".as_ptr() as *const c_char);
            return error;
        }
    }

    /* enable IDE port */
    if ide != 0 {
        let error = aci_setvalue(aci, ACI_SET_IDE as c_uchar, ide);
        if error < 0 {
            dev_err((*(*miro).card).dev, b"enabling IDE port failed\n\0".as_ptr() as *const c_char);
            return error;
        }
    }

    /* set common aci values */
    let mut idx = 0;
    while idx < aci_init_values.len() {
        let error = aci_setvalue(aci, aci_init_values[idx][0], aci_init_values[idx][1] as c_int);
        if error < 0 {
            dev_err((*(*miro).card).dev, b"aci_setvalue(%d) failed: %d\n\0".as_ptr() as *const c_char, aci_init_values[idx][0] as c_int, error);
            return error;
        }
        idx += 1;
    }
    (*aci).aci_amp = 0;
    (*aci).aci_preamp = 0;
    (*aci).aci_solomode = 0;
    0
}

unsafe extern "C" fn snd_miro_aci_force_known_state(aci: *mut snd_miro_aci) -> c_int {
    let mut i = 0;
    while i < 3 {
        let err = snd_aci_cmd(aci, ACI_ERROR_OP, -1, -1);
        if err < 0 { return err; }
        i += 1;
    }
    0
}

unsafe extern "C" fn snd_miro_aci_initialize(aci: *mut snd_miro_aci) -> c_int {
    let mut err = snd_aci_cmd(aci, ACI_INIT, -1, -1);
    if err < 0 { return err; }
    err = snd_aci_cmd(aci, ACI_ERROR_OP, ACI_ERROR_OP, ACI_ERROR_OP);
    if err < 0 { return err; }
    snd_aci_cmd(aci, ACI_ERROR_OP, ACI_ERROR_OP, ACI_ERROR_OP)
}

/* CONFIG_PM */
unsafe extern "C" fn snd_miro_save_aci_state(miro: *mut snd_miro) -> c_int {
    let aci = (*miro).aci;
    let limit = if (*aci).aci_product == 'C' as c_int { MIRO_ACI_COUNT } else { MIRO_ACI_LINE2 + 1 };
    let mut i = 0;
    while i < limit {
        let mut value = aci_getvalue(aci, snd_miro_saved_get_regs[i]);
        if value < 0 { return value; }
        (*miro).aci_saved[i][1] = value as c_uchar;
        value = aci_getvalue(aci, snd_miro_saved_get_regs[i].wrapping_add(1));
        if value < 0 { return value; }
        (*miro).aci_saved[i][0] = value as c_uchar;
        i += 1;
    }

    (*miro).aci_saved_amp = (*aci).aci_amp as c_uchar;
    if (*aci).aci_version <= 176 {
        (*miro).aci_saved_preamp = (*aci).aci_preamp as c_uchar;
    } else {
        let value = aci_getvalue(aci, ACI_GET_PREAMP as c_uchar);
        if value < 0 { return value; }
        (*miro).aci_saved_preamp = value as c_uchar;
    }
    let value = aci_getvalue(aci, ACI_S_GENERAL as c_uchar);
    if value < 0 { return value; }
    (*miro).aci_saved_solomode = if (value & 0x20) == 0 { 1 } else { 0 };
    0
}

/* CONFIG_PM */
unsafe extern "C" fn snd_miro_restore_aci_state(miro: *mut snd_miro) -> c_int {
    let aci = (*miro).aci;
    let mut err = snd_set_aci_init_values(miro);
    if err < 0 { return err; }
    let limit = if (*aci).aci_product == 'C' as c_int { MIRO_ACI_COUNT } else { MIRO_ACI_LINE2 + 1 };
    let mut i = 0;
    while i < limit {
        let left_reg = if snd_miro_saved_set_regs[i] as c_int == ACI_SET_MASTER {
            snd_miro_saved_set_regs[i].wrapping_add(1)
        } else {
            snd_miro_saved_set_regs[i].wrapping_add(8)
        };
        err = aci_setvalue(aci, left_reg, (*miro).aci_saved[i][0] as c_int);
        if err < 0 { return err; }
        err = aci_setvalue(aci, snd_miro_saved_set_regs[i], (*miro).aci_saved[i][1] as c_int);
        if err < 0 { return err; }
        i += 1;
    }

    err = aci_setvalue(aci, ACI_SET_POWERAMP as c_uchar, (*miro).aci_saved_amp as c_int);
    if err < 0 { return err; }
    err = aci_setvalue(aci, ACI_SET_PREAMP as c_uchar, (*miro).aci_saved_preamp as c_int);
    if err < 0 { return err; }
    err = aci_setvalue(aci, ACI_SET_SOLOMODE as c_uchar, (*miro).aci_saved_solomode as c_int);
    if err < 0 { return err; }

    (*aci).aci_amp = (*miro).aci_saved_amp as c_int;
    (*aci).aci_preamp = (*miro).aci_saved_preamp as c_int;
    (*aci).aci_solomode = (*miro).aci_saved_solomode as c_int;
    0
}

unsafe extern "C" fn snd_miro_mixer(card: *mut snd_card, miro: *mut snd_miro) -> c_int {
    if snd_BUG_ON(miro.is_null() || card.is_null()) != 0 {
        return -EINVAL;
    }

    match (*miro).hardware {
        OPTi9XX_HW_82C924 => { strscpy((*card).mixername.as_mut_ptr(), b"ACI & OPTi924\0".as_ptr() as *const c_char); }
        OPTi9XX_HW_82C929 => { strscpy((*card).mixername.as_mut_ptr(), b"ACI & OPTi929\0".as_ptr() as *const c_char); }
        _ => { snd_BUG(); }
    }

    let mut idx = 0;
    while idx < snd_miro_controls.len() {
        let err = snd_ctl_add(card, snd_ctl_new1(&snd_miro_controls[idx], miro as *mut c_void));
        if err < 0 { return err; }
        idx += 1;
    }

    if (*(*miro).aci).aci_product == 'A' as c_int || (*(*miro).aci).aci_product == 'B' as c_int {
        /* PCM1/PCM12 with power-amp and Line 2 */
        let mut err = snd_ctl_add(card, snd_ctl_new1(&snd_miro_line_control[0], miro as *mut c_void));
        if err < 0 { return err; }
        err = snd_ctl_add(card, snd_ctl_new1(&snd_miro_amp_control[0], miro as *mut c_void));
        if err < 0 { return err; }
    }

    if (*(*miro).aci).aci_product == 'B' as c_int || (*(*miro).aci).aci_product == 'C' as c_int {
        /* PCM12/PCM20 with mic-preamp */
        let mut err = snd_ctl_add(card, snd_ctl_new1(&snd_miro_preamp_control[0], miro as *mut c_void));
        if err < 0 { return err; }
        if (*(*miro).aci).aci_version >= 176 {
            err = snd_ctl_add(card, snd_ctl_new1(&snd_miro_capture_control[0], miro as *mut c_void));
            if err < 0 { return err; }
        }
    }

    if (*(*miro).aci).aci_product == 'C' as c_int {
        /* PCM20 with radio and 7 band equalizer */
        let mut err = snd_ctl_add(card, snd_ctl_new1(&snd_miro_radio_control[0], miro as *mut c_void));
        if err < 0 { return err; }
        idx = 0;
        while idx < snd_miro_eq_controls.len() {
            err = snd_ctl_add(card, snd_ctl_new1(&snd_miro_eq_controls[idx], miro as *mut c_void));
            if err < 0 { return err; }
            idx += 1;
        }
    }
    0
}

unsafe extern "C" fn snd_miro_init(chip: *mut snd_miro, hardware: c_ushort) -> c_int {
    static opti9xx_mc_size: [c_int; 7] = [7, 7, 10, 10, 2, 2, 2];
    (*chip).hardware = hardware;
    strscpy((*chip).name.as_mut_ptr(), snd_opti9xx_names[hardware as usize]);
    (*chip).mc_base_size = opti9xx_mc_size[hardware as usize] as c_ulong;
    spin_lock_init(&mut (*chip).lock);
    (*chip).wss_base = -1;
    (*chip).irq = -1;
    (*chip).dma1 = -1;
    (*chip).dma2 = -1;
    (*chip).mpu_port = -1;
    (*chip).mpu_irq = -1;
    (*chip).pwd_reg = 3;

    /* CONFIG_PNP */
    if isapnp && (*chip).mc_base != 0 {
        /* PnP resource gives the least 10 bits */
        (*chip).mc_base |= 0xc00;
    } else {
        (*chip).mc_base = 0xf8c;
    }

    match hardware {
        OPTi9XX_HW_82C929 => (*chip).password = 0xe3,
        OPTi9XX_HW_82C924 => (*chip).password = 0xe5,
        _ => {
            dev_err((*(*chip).card).dev, b"sorry, no support for %d\n\0".as_ptr() as *const c_char, hardware as c_int);
            return -ENODEV;
        }
    }
    0
}

unsafe extern "C" fn snd_miro_read(chip: *mut snd_miro, reg: c_uchar) -> c_uchar {
    let mut retval: c_uchar = 0xff;
    let flags = spin_lock_irqsave(&mut (*chip).lock);
    outb((*chip).password, (*chip).mc_base + (*chip).pwd_reg);
    match (*chip).hardware {
        OPTi9XX_HW_82C924 => {
            if reg > 7 {
                outb(reg, (*chip).mc_base + 8);
                outb((*chip).password, (*chip).mc_base + (*chip).pwd_reg);
                retval = inb((*chip).mc_base + 9);
            } else {
                retval = inb((*chip).mc_base + reg as c_ulong);
            }
        }
        OPTi9XX_HW_82C929 => retval = inb((*chip).mc_base + reg as c_ulong),
        _ => dev_err((*(*chip).card).dev, b"sorry, no support for %d\n\0".as_ptr() as *const c_char, (*chip).hardware as c_int),
    }
    spin_unlock_irqrestore(&mut (*chip).lock, flags);
    retval
}

unsafe extern "C" fn snd_miro_write(chip: *mut snd_miro, reg: c_uchar, value: c_uchar) {
    let flags = spin_lock_irqsave(&mut (*chip).lock);
    outb((*chip).password, (*chip).mc_base + (*chip).pwd_reg);
    match (*chip).hardware {
        OPTi9XX_HW_82C924 => {
            if reg > 7 {
                outb(reg, (*chip).mc_base + 8);
                outb((*chip).password, (*chip).mc_base + (*chip).pwd_reg);
                outb(value, (*chip).mc_base + 9);
            } else {
                outb(value, (*chip).mc_base + reg as c_ulong);
            }
        }
        OPTi9XX_HW_82C929 => outb(value, (*chip).mc_base + reg as c_ulong),
        _ => dev_err((*(*chip).card).dev, b"sorry, no support for %d\n\0".as_ptr() as *const c_char, (*chip).hardware as c_int),
    }
    spin_unlock_irqrestore(&mut (*chip).lock, flags);
}

unsafe extern "C" fn snd_miro_write_mask(chip: *mut snd_miro, reg: c_uchar, value: c_uchar, mask: c_uchar) {
    let oldval = snd_miro_read(chip, reg);
    snd_miro_write(chip, reg, (oldval & !mask) | (value & mask));
}

/*
 *  Proc Interface
 */

unsafe extern "C" fn snd_miro_proc_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let miro = (*entry).private_data as *mut snd_miro;
    let aci = (*miro).aci;
    let mut model = b"unknown\0".as_ptr() as *const c_char;

    /* miroSOUND PCM1 pro, early PCM12 */
    if (*miro).hardware == OPTi9XX_HW_82C929 && (*aci).aci_vendor == 'm' as c_int && (*aci).aci_product == 'A' as c_int {
        model = match (*aci).aci_version {
            3 => b"miroSOUND PCM1 pro\0".as_ptr() as *const c_char,
            _ => b"miroSOUND PCM1 pro / (early) PCM12\0".as_ptr() as *const c_char,
        };
    }

    /* miroSOUND PCM12, PCM12 (Rev. E), PCM12 pnp */
    if (*miro).hardware == OPTi9XX_HW_82C924 && (*aci).aci_vendor == 'm' as c_int && (*aci).aci_product == 'B' as c_int {
        model = match (*aci).aci_version {
            4 => b"miroSOUND PCM12\0".as_ptr() as *const c_char,
            176 => b"miroSOUND PCM12 (Rev. E)\0".as_ptr() as *const c_char,
            _ => b"miroSOUND PCM12 / PCM12 pnp\0".as_ptr() as *const c_char,
        };
    }

    /* miroSOUND PCM20 radio */
    if (*miro).hardware == OPTi9XX_HW_82C924 && (*aci).aci_vendor == 'm' as c_int && (*aci).aci_product == 'C' as c_int {
        model = match (*aci).aci_version {
            7 => b"miroSOUND PCM20 radio (Rev. E)\0".as_ptr() as *const c_char,
            _ => b"miroSOUND PCM20 radio\0".as_ptr() as *const c_char,
        };
    }

    snd_iprintf(buffer, b"\nGeneral information:\n\0".as_ptr() as *const c_char);
    snd_iprintf(buffer, b"  model   : %s\n\0".as_ptr() as *const c_char, model);
    snd_iprintf(buffer, b"  opti    : %s\n\0".as_ptr() as *const c_char, (*miro).name.as_ptr());
    snd_iprintf(buffer, b"  codec   : %s\n\0".as_ptr() as *const c_char, (*(*miro).pcm).name.as_ptr());
    snd_iprintf(buffer, b"  port    : 0x%lx\n\0".as_ptr() as *const c_char, (*miro).wss_base);
    snd_iprintf(buffer, b"  irq     : %d\n\0".as_ptr() as *const c_char, (*miro).irq);
    snd_iprintf(buffer, b"  dma     : %d,%d\n\n\0".as_ptr() as *const c_char, (*miro).dma1, (*miro).dma2);

    snd_iprintf(buffer, b"MPU-401:\n\0".as_ptr() as *const c_char);
    snd_iprintf(buffer, b"  port    : 0x%lx\n\0".as_ptr() as *const c_char, (*miro).mpu_port);
    snd_iprintf(buffer, b"  irq     : %d\n\n\0".as_ptr() as *const c_char, (*miro).mpu_irq);

    snd_iprintf(buffer, b"ACI information:\n\0".as_ptr() as *const c_char);
    snd_iprintf(buffer, b"  vendor  : \0".as_ptr() as *const c_char);
    match (*aci).aci_vendor {
        x if x == 'm' as c_int => snd_iprintf(buffer, b"Miro\n\0".as_ptr() as *const c_char),
        _ => snd_iprintf(buffer, b"unknown (0x%x)\n\0".as_ptr() as *const c_char, (*aci).aci_vendor),
    }
    snd_iprintf(buffer, b"  product : \0".as_ptr() as *const c_char);
    match (*aci).aci_product {
        x if x == 'A' as c_int => snd_iprintf(buffer, b"miroSOUND PCM1 pro / (early) PCM12\n\0".as_ptr() as *const c_char),
        x if x == 'B' as c_int => snd_iprintf(buffer, b"miroSOUND PCM12\n\0".as_ptr() as *const c_char),
        x if x == 'C' as c_int => snd_iprintf(buffer, b"miroSOUND PCM20 radio\n\0".as_ptr() as *const c_char),
        _ => snd_iprintf(buffer, b"unknown (0x%x)\n\0".as_ptr() as *const c_char, (*aci).aci_product),
    }
    snd_iprintf(buffer, b"  firmware: %d (0x%x)\n\0".as_ptr() as *const c_char, (*aci).aci_version, (*aci).aci_version);
    snd_iprintf(buffer, b"  port    : 0x%lx-0x%lx\n\0".as_ptr() as *const c_char, (*aci).aci_port, (*aci).aci_port + 2);
    snd_iprintf(buffer, b"  wss     : 0x%x\n\0".as_ptr() as *const c_char, wss);
    snd_iprintf(buffer, b"  ide     : 0x%x\n\0".as_ptr() as *const c_char, ide);
    snd_iprintf(buffer, b"  solomode: 0x%x\n\0".as_ptr() as *const c_char, (*aci).aci_solomode);
    snd_iprintf(buffer, b"  amp     : 0x%x\n\0".as_ptr() as *const c_char, (*aci).aci_amp);
    snd_iprintf(buffer, b"  preamp  : 0x%x\n\0".as_ptr() as *const c_char, (*aci).aci_preamp);
}

unsafe extern "C" fn snd_miro_proc_init(card: *mut snd_card, miro: *mut snd_miro) {
    snd_card_ro_proc_new(card, b"miro\0".as_ptr() as *const c_char, miro as *mut c_void, Some(snd_miro_proc_read));
}

/*
 *  Init
 */

unsafe extern "C" fn snd_miro_configure(chip: *mut snd_miro) -> c_int {
    let mut wss_base_bits: c_uchar;
    let irq_bits: c_uchar;
    let mut dma_bits: c_uchar;
    let mut mpu_port_bits: c_uchar = 0;
    let mpu_irq_bits: c_uchar;

    snd_miro_write_mask(chip, OPTi9XX_MC_REG(1), 0x80, 0x80);
    snd_miro_write_mask(chip, OPTi9XX_MC_REG(2), 0x20, 0x20); /* OPL4 */
    snd_miro_write_mask(chip, OPTi9XX_MC_REG(5), 0x02, 0x02);

    match (*chip).hardware {
        OPTi9XX_HW_82C924 => {
            snd_miro_write_mask(chip, OPTi9XX_MC_REG(6), 0x02, 0x02);
            snd_miro_write_mask(chip, OPTi9XX_MC_REG(3), 0xf0, 0xff);
        }
        OPTi9XX_HW_82C929 => {
            /* untested init commands for OPTi929 */
            snd_miro_write_mask(chip, OPTi9XX_MC_REG(4), 0x00, 0x0c);
        }
        _ => {
            dev_err((*(*chip).card).dev, b"chip %d not supported\n\0".as_ptr() as *const c_char, (*chip).hardware as c_int);
            return -EINVAL;
        }
    }

    /* PnP resource says it decodes only 10 bits of address */
    match (*chip).wss_base & 0x3ff {
        0x130 => { (*chip).wss_base = 0x530; wss_base_bits = 0x00; }
        0x204 => { (*chip).wss_base = 0x604; wss_base_bits = 0x03; }
        0x280 => { (*chip).wss_base = 0xe80; wss_base_bits = 0x01; }
        0x340 => { (*chip).wss_base = 0xf40; wss_base_bits = 0x02; }
        _ => {
            dev_err((*(*chip).card).dev, b"WSS port 0x%lx not valid\n\0".as_ptr() as *const c_char, (*chip).wss_base);
            wss_base_bits = 0;
            goto_skip_base(chip, mpu_port_bits);
            return 0;
        }
    }
    snd_miro_write_mask(chip, OPTi9XX_MC_REG(1), wss_base_bits << 4, 0x30);

    irq_bits = match (*chip).irq {
        5 => 0x05,
        7 => 0x01,
        9 => 0x02,
        10 => 0x03,
        11 => 0x04,
        _ => {
            dev_err((*(*chip).card).dev, b"WSS irq # %d not valid\n\0".as_ptr() as *const c_char, (*chip).irq);
            return snd_miro_configure_mpu(chip, mpu_port_bits);
        }
    };

    dma_bits = match (*chip).dma1 {
        0 => 0x01,
        1 => 0x02,
        3 => 0x03,
        _ => {
            dev_err((*(*chip).card).dev, b"WSS dma1 # %d not valid\n\0".as_ptr() as *const c_char, (*chip).dma1);
            return snd_miro_configure_mpu(chip, mpu_port_bits);
        }
    };

    if (*chip).dma1 == (*chip).dma2 {
        dev_err((*(*chip).card).dev, b"don't want to share dmas\n\0".as_ptr() as *const c_char);
        return -EBUSY;
    }

    match (*chip).dma2 {
        0 | 1 => {}
        _ => {
            dev_err((*(*chip).card).dev, b"WSS dma2 # %d not valid\n\0".as_ptr() as *const c_char, (*chip).dma2);
            return snd_miro_configure_mpu(chip, mpu_port_bits);
        }
    }
    dma_bits |= 0x04;
    let flags = spin_lock_irqsave(&mut (*chip).lock);
    outb((irq_bits << 3) | dma_bits, (*chip).wss_base as c_ulong);
    spin_unlock_irqrestore(&mut (*chip).lock, flags);

    snd_miro_configure_mpu(chip, mpu_port_bits)
}

unsafe fn goto_skip_base(chip: *mut snd_miro, mpu_port_bits: c_uchar) -> c_int {
    snd_miro_configure_mpu(chip, mpu_port_bits)
}

unsafe fn snd_miro_configure_mpu(chip: *mut snd_miro, mut mpu_port_bits: c_uchar) -> c_int {
    if (*chip).hardware > OPTi9XX_HW_82C928 {
        mpu_port_bits = match (*chip).mpu_port {
            0 | -1 => 0,
            0x300 => 0x03,
            0x310 => 0x02,
            0x320 => 0x01,
            0x330 => 0x00,
            _ => {
                dev_err((*(*chip).card).dev, b"MPU-401 port 0x%lx not valid\n\0".as_ptr() as *const c_char, (*chip).mpu_port);
                return 0;
            }
        };
        let mpu_irq_bits = match (*chip).mpu_irq {
            5 => 0x02,
            7 => 0x03,
            9 => 0x00,
            10 => 0x01,
            _ => {
                dev_err((*(*chip).card).dev, b"MPU-401 irq # %d not valid\n\0".as_ptr() as *const c_char, (*chip).mpu_irq);
                return 0;
            }
        };
        snd_miro_write_mask(
            chip,
            OPTi9XX_MC_REG(6),
            if (*chip).mpu_port <= 0 { 0x00 } else { 0x80 | (mpu_port_bits << 5) | (mpu_irq_bits << 3) },
            0xf8,
        );
    }
    0
}

unsafe extern "C" fn snd_miro_opti_check(card: *mut snd_card, chip: *mut snd_miro) -> c_int {
    (*chip).res_mc_base = devm_request_region((*card).dev, (*chip).mc_base, (*chip).mc_base_size, b"OPTi9xx MC\0".as_ptr() as *const c_char);
    if (*chip).res_mc_base.is_null() { return -ENOMEM; }
    let value = snd_miro_read(chip, OPTi9XX_MC_REG(1));
    if value != 0xff && value != inb((*chip).mc_base + OPTi9XX_MC_REG(1) as c_ulong) {
        if value == snd_miro_read(chip, OPTi9XX_MC_REG(1)) {
            return 0;
        }
    }
    devm_release_resource((*card).dev, (*chip).res_mc_base);
    (*chip).res_mc_base = ptr::null_mut();
    -ENODEV
}

unsafe extern "C" fn snd_card_miro_detect(card: *mut snd_card, chip: *mut snd_miro) -> c_int {
    let mut i = OPTi9XX_HW_82C929;
    while i <= OPTi9XX_HW_82C924 {
        let mut err = snd_miro_init(chip, i);
        if err < 0 { return err; }
        err = snd_miro_opti_check(card, chip);
        if err == 0 { return 1; }
        i += 1;
    }
    -ENODEV
}

unsafe extern "C" fn snd_card_miro_aci_detect(card: *mut snd_card, miro: *mut snd_miro) -> c_int {
    let aci = &mut aci_device as *mut snd_miro_aci;
    (*miro).aci = aci;
    (*aci).card = card;
    mutex_init(&mut (*aci).aci_mutex);
    /* get ACI port from OPTi9xx MC 4 */
    let regval = inb((*miro).mc_base + 4);
    (*aci).aci_port = if (regval & 0x10) != 0 { 0x344 } else { 0x354 };
    (*miro).res_aci_port = devm_request_region((*card).dev, (*aci).aci_port, 3, b"miro aci\0".as_ptr() as *const c_char);
    if (*miro).res_aci_port.is_null() {
        dev_err((*card).dev, b"aci i/o area 0x%lx-0x%lx already used.\n\0".as_ptr() as *const c_char, (*aci).aci_port, (*aci).aci_port + 2);
        return -ENOMEM;
    }
    /* force ACI into a known state */
    let mut err = snd_miro_aci_force_known_state(aci);
    if err < 0 {
        dev_err((*card).dev, b"can't force aci into known state.\n\0".as_ptr() as *const c_char);
        return -ENXIO;
    }
    (*aci).aci_vendor = snd_aci_cmd(aci, ACI_READ_IDCODE, -1, -1);
    (*aci).aci_product = snd_aci_cmd(aci, ACI_READ_IDCODE, -1, -1);
    if (*aci).aci_vendor < 0 || (*aci).aci_product < 0 {
        dev_err((*card).dev, b"can't read aci id on 0x%lx.\n\0".as_ptr() as *const c_char, (*aci).aci_port);
        return -ENXIO;
    }
    (*aci).aci_version = snd_aci_cmd(aci, ACI_READ_VERSION, -1, -1);
    if (*aci).aci_version < 0 {
        dev_err((*card).dev, b"can't read aci version on 0x%lx.\n\0".as_ptr() as *const c_char, (*aci).aci_port);
        return -ENXIO;
    }
    err = snd_miro_aci_initialize(aci);
    if err < 0 {
        dev_err((*card).dev, b"can't initialize aci.\n\0".as_ptr() as *const c_char);
        return -ENXIO;
    }
    0
}

unsafe extern "C" fn snd_miro_probe(card: *mut snd_card) -> c_int {
    let miro = (*card).private_data as *mut snd_miro;
    let mut codec: *mut snd_wss = ptr::null_mut();
    let mut rmidi: *mut snd_rawmidi;
    if (*miro).res_mc_base.is_null() {
        (*miro).res_mc_base = devm_request_region((*card).dev, (*miro).mc_base, (*miro).mc_base_size, b"miro (OPTi9xx MC)\0".as_ptr() as *const c_char);
        if (*miro).res_mc_base.is_null() {
            dev_err((*card).dev, b"request for OPTI9xx MC failed\n\0".as_ptr() as *const c_char);
            return -ENOMEM;
        }
    }
    let mut error = snd_card_miro_aci_detect(card, miro);
    if error < 0 {
        dev_err((*card).dev, b"unable to detect aci chip\n\0".as_ptr() as *const c_char);
        return -ENODEV;
    }
    (*miro).wss_base = port;
    (*miro).mpu_port = mpu_port;
    (*miro).irq = irq;
    (*miro).mpu_irq = mpu_irq;
    (*miro).dma1 = dma1;
    (*miro).dma2 = dma2;
    /* init proc interface */
    snd_miro_proc_init(card, miro);
    error = snd_miro_configure(miro);
    if error != 0 { return error; }
    error = snd_wss_create(card, ((*miro).wss_base + 4) as c_ulong, -1, (*miro).irq, (*miro).dma1, (*miro).dma2, WSS_HW_DETECT, 0, &mut codec);
    if error < 0 { return error; }
    (*miro).codec = codec;
    error = snd_wss_pcm(codec, 0);
    if error < 0 { return error; }
    error = snd_wss_mixer(codec);
    if error < 0 { return error; }
    error = snd_wss_timer(codec, 0);
    if error < 0 { return error; }
    (*miro).pcm = (*codec).pcm;
    error = snd_miro_mixer(card, miro);
    if error < 0 { return error; }
    if (*(*miro).aci).aci_vendor == 'm' as c_int {
        /* It looks like a miro sound card. */
        match (*(*miro).aci).aci_product {
            x if x == 'A' as c_int => { sprintf((*card).shortname.as_mut_ptr(), b"miroSOUND PCM1 pro / PCM12\0".as_ptr() as *const c_char); }
            x if x == 'B' as c_int => { sprintf((*card).shortname.as_mut_ptr(), b"miroSOUND PCM12\0".as_ptr() as *const c_char); }
            x if x == 'C' as c_int => { sprintf((*card).shortname.as_mut_ptr(), b"miroSOUND PCM20 radio\0".as_ptr() as *const c_char); }
            _ => {
                sprintf((*card).shortname.as_mut_ptr(), b"unknown miro\0".as_ptr() as *const c_char);
                dev_info((*card).dev, b"unknown miro aci id\n\0".as_ptr() as *const c_char);
            }
        }
    } else {
        dev_info((*card).dev, b"found unsupported aci card\n\0".as_ptr() as *const c_char);
        sprintf((*card).shortname.as_mut_ptr(), b"unknown Cardinal Technologies\0".as_ptr() as *const c_char);
    }
    strscpy((*card).driver.as_mut_ptr(), b"miro\0".as_ptr() as *const c_char);
    scnprintf((*card).longname.as_mut_ptr(), (*card).longname.len(), b"%s: OPTi%s, %s at 0x%lx, irq %d, dma %d&%d\0".as_ptr() as *const c_char, (*card).shortname.as_ptr(), (*miro).name.as_ptr(), (*(*codec).pcm).name.as_ptr(), (*miro).wss_base + 4, (*miro).irq, (*miro).dma1, (*miro).dma2);

    if mpu_port <= 0 || mpu_port == SNDRV_AUTO_PORT {
        rmidi = ptr::null_mut();
    } else {
        error = snd_mpu401_uart_new(card, 0, MPU401_HW_MPU401, mpu_port as c_ulong, 0, (*miro).mpu_irq, &mut rmidi);
        if error < 0 {
            dev_warn((*card).dev, b"no MPU-401 device at 0x%lx?\n\0".as_ptr() as *const c_char, mpu_port);
        }
    }

    if fm_port > 0 && fm_port != SNDRV_AUTO_PORT {
        let mut opl3: *mut snd_opl3 = ptr::null_mut();
        let mut opl4: *mut snd_opl4 = ptr::null_mut();
        if snd_opl4_create(card, fm_port, fm_port - 8, 2, &mut opl3, &mut opl4) < 0 {
            dev_warn((*card).dev, b"no OPL4 device at 0x%lx\n\0".as_ptr() as *const c_char, fm_port);
        }
    }
    error = snd_set_aci_init_values(miro);
    if error < 0 { return error; }
    snd_card_register(card)
}

unsafe extern "C" fn snd_miro_isa_match(_devptr: *mut device, _n: c_uint) -> c_int {
    /* CONFIG_PNP */
    if snd_miro_pnp_is_probed != 0 { return 0; }
    if isapnp { return 0; }
    1
}

unsafe extern "C" fn snd_miro_isa_probe(devptr: *mut device, _n: c_uint) -> c_int {
    static possible_ports: [c_long; 5] = [0x530, 0xe80, 0xf40, 0x604, -1];
    static possible_mpu_ports: [c_long; 5] = [0x330, 0x300, 0x310, 0x320, -1];
    static possible_irqs: [c_int; 5] = [11, 9, 10, 7, -1];
    static possible_mpu_irqs: [c_int; 5] = [10, 5, 9, 7, -1];
    static possible_dma1s: [c_int; 4] = [3, 1, 0, -1];
    static possible_dma2s: [[c_int; 2]; 4] = [[1, -1], [0, -1], [-1, -1], [0, -1]];
    let mut card: *mut snd_card = ptr::null_mut();
    let mut error = snd_devm_card_new(devptr, index, id, THIS_MODULE, core::mem::size_of::<snd_miro>(), &mut card);
    if error < 0 { return error; }
    let miro = (*card).private_data as *mut snd_miro;
    (*miro).card = card;
    error = snd_card_miro_detect(card, miro);
    if error < 0 {
        dev_err((*card).dev, b"unable to detect OPTi9xx chip\n\0".as_ptr() as *const c_char);
        return -ENODEV;
    }
    if port == SNDRV_AUTO_PORT {
        port = snd_legacy_find_free_ioport(possible_ports.as_ptr(), 4);
        if port < 0 {
            dev_err((*card).dev, b"unable to find a free WSS port\n\0".as_ptr() as *const c_char);
            return -EBUSY;
        }
    }
    if mpu_port == SNDRV_AUTO_PORT {
        mpu_port = snd_legacy_find_free_ioport(possible_mpu_ports.as_ptr(), 2);
        if mpu_port < 0 {
            dev_err((*card).dev, b"unable to find a free MPU401 port\n\0".as_ptr() as *const c_char);
            return -EBUSY;
        }
    }
    if irq == SNDRV_AUTO_IRQ {
        irq = snd_legacy_find_free_irq(possible_irqs.as_ptr());
        if irq < 0 {
            dev_err((*card).dev, b"unable to find a free IRQ\n\0".as_ptr() as *const c_char);
            return -EBUSY;
        }
    }
    if mpu_irq == SNDRV_AUTO_IRQ {
        mpu_irq = snd_legacy_find_free_irq(possible_mpu_irqs.as_ptr());
        if mpu_irq < 0 {
            dev_err((*card).dev, b"unable to find a free MPU401 IRQ\n\0".as_ptr() as *const c_char);
            return -EBUSY;
        }
    }
    if dma1 == SNDRV_AUTO_DMA {
        dma1 = snd_legacy_find_free_dma(possible_dma1s.as_ptr());
        if dma1 < 0 {
            dev_err((*card).dev, b"unable to find a free DMA1\n\0".as_ptr() as *const c_char);
            return -EBUSY;
        }
    }
    if dma2 == SNDRV_AUTO_DMA {
        dma2 = snd_legacy_find_free_dma(possible_dma2s[(dma1 % 4) as usize].as_ptr());
        if dma2 < 0 {
            dev_err((*card).dev, b"unable to find a free DMA2\n\0".as_ptr() as *const c_char);
            return -EBUSY;
        }
    }
    error = snd_miro_probe(card);
    if error < 0 { return error; }
    dev_set_drvdata(devptr, card as *mut c_void);
    0
}

/* CONFIG_PM */
unsafe extern "C" fn snd_miro_suspend(card: *mut snd_card) -> c_int {
    let miro = (*card).private_data as *mut snd_miro;
    let error = snd_miro_save_aci_state(miro);
    if error < 0 { return error; }
    snd_power_change_state(card, SNDRV_CTL_POWER_D3hot);
    if let Some(suspend) = (*(*miro).codec).suspend { suspend((*miro).codec); }
    0
}

/* CONFIG_PM */
unsafe extern "C" fn snd_miro_resume(card: *mut snd_card) -> c_int {
    let miro = (*card).private_data as *mut snd_miro;
    let mut error = snd_miro_configure(miro);
    if error < 0 { return error; }
    error = snd_miro_aci_force_known_state((*miro).aci);
    if error < 0 {
        dev_err((*card).dev, b"can't force aci into known state\n\0".as_ptr() as *const c_char);
        return error;
    }
    error = snd_miro_aci_initialize((*miro).aci);
    if error < 0 {
        dev_err((*card).dev, b"can't initialize aci\n\0".as_ptr() as *const c_char);
        return error;
    }
    error = snd_miro_restore_aci_state(miro);
    if error < 0 { return error; }
    if let Some(resume) = (*(*miro).codec).resume { resume((*miro).codec); }
    snd_power_change_state(card, SNDRV_CTL_POWER_D0);
    0
}

/* CONFIG_PM */
unsafe extern "C" fn snd_miro_isa_suspend(dev: *mut device, _n: c_uint, _state: pm_message_t) -> c_int {
    snd_miro_suspend(dev_get_drvdata(dev) as *mut snd_card)
}

/* CONFIG_PM */
unsafe extern "C" fn snd_miro_isa_resume(dev: *mut device, _n: c_uint) -> c_int {
    snd_miro_resume(dev_get_drvdata(dev) as *mut snd_card)
}

const DEV_NAME: *const c_char = b"miro\0".as_ptr() as *const c_char;

static mut snd_miro_driver: isa_driver = isa_driver {
    match_: Some(snd_miro_isa_match),
    probe: Some(snd_miro_isa_probe),
    /* CONFIG_PM */
    suspend: Some(snd_miro_isa_suspend),
    resume: Some(snd_miro_isa_resume),
    driver: device_driver { name: DEV_NAME },
};

/* CONFIG_PNP */
unsafe extern "C" fn snd_card_miro_pnp(chip: *mut snd_miro, card: *mut pnp_card_link, pid: *const pnp_card_device_id) -> c_int {
    let pdev = pnp_request_card_device(card, (*pid).devs[0].id.as_ptr(), ptr::null_mut());
    if pdev.is_null() { return -EBUSY; }
    let devmpu = pnp_request_card_device(card, (*pid).devs[1].id.as_ptr(), ptr::null_mut());
    if devmpu.is_null() { return -EBUSY; }
    let devmc = pnp_request_card_device(card, (*pid).devs[2].id.as_ptr(), ptr::null_mut());
    if devmc.is_null() { return -EBUSY; }
    let mut err = pnp_activate_dev(pdev);
    if err < 0 {
        dev_err((*(*chip).card).dev, b"AUDIO pnp configure failure: %d\n\0".as_ptr() as *const c_char, err);
        return err;
    }
    err = pnp_activate_dev(devmc);
    if err < 0 {
        dev_err((*(*chip).card).dev, b"MC pnp configure failure: %d\n\0".as_ptr() as *const c_char, err);
        return err;
    }
    port = pnp_port_start(pdev, 1) as c_long;
    fm_port = pnp_port_start(pdev, 2) as c_long + 8;
    /*
     * The MC(0) is never accessed and the miroSOUND PCM20 card does not
     * include it in the PnP resource range. OPTI93x include it.
     */
    (*chip).mc_base = pnp_port_start(devmc, 0) - 1;
    (*chip).mc_base_size = pnp_port_len(devmc, 0) + 1;
    irq = pnp_irq(pdev, 0);
    dma1 = pnp_dma(pdev, 0);
    dma2 = pnp_dma(pdev, 1);
    if mpu_port > 0 {
        err = pnp_activate_dev(devmpu);
        if err < 0 {
            dev_err((*(*chip).card).dev, b"MPU401 pnp configure failure\n\0".as_ptr() as *const c_char);
            mpu_port = -1;
            return err;
        }
        mpu_port = pnp_port_start(devmpu, 0) as c_long;
        mpu_irq = pnp_irq(devmpu, 0);
    }
    0
}

/* CONFIG_PNP */
unsafe extern "C" fn snd_miro_pnp_probe(pcard: *mut pnp_card_link, pid: *const pnp_card_device_id) -> c_int {
    let mut card: *mut snd_card = ptr::null_mut();
    if snd_miro_pnp_is_probed != 0 { return -EBUSY; }
    if !isapnp { return -ENODEV; }
    let mut err = snd_devm_card_new(&mut (*(*pcard).card).dev, index, id, THIS_MODULE, core::mem::size_of::<snd_miro>(), &mut card);
    if err < 0 { return err; }
    let miro = (*card).private_data as *mut snd_miro;
    (*miro).card = card;
    err = snd_card_miro_pnp(miro, pcard, pid);
    if err != 0 { return err; }
    /* only miroSOUND PCM20 and PCM12 == OPTi924 */
    err = snd_miro_init(miro, OPTi9XX_HW_82C924);
    if err != 0 { return err; }
    err = snd_miro_opti_check(card, miro);
    if err != 0 {
        dev_err((*card).dev, b"OPTI chip not found\n\0".as_ptr() as *const c_char);
        return err;
    }
    err = snd_miro_probe(card);
    if err < 0 { return err; }
    pnp_set_card_drvdata(pcard, card as *mut c_void);
    snd_miro_pnp_is_probed = 1;
    0
}

/* CONFIG_PNP */
unsafe extern "C" fn snd_miro_pnp_remove(_pcard: *mut pnp_card_link) {
    snd_miro_pnp_is_probed = 0;
}

/* CONFIG_PNP && CONFIG_PM */
unsafe extern "C" fn snd_miro_pnp_suspend(pcard: *mut pnp_card_link, _state: pm_message_t) -> c_int {
    snd_miro_suspend(pnp_get_card_drvdata(pcard) as *mut snd_card)
}

/* CONFIG_PNP && CONFIG_PM */
unsafe extern "C" fn snd_miro_pnp_resume(pcard: *mut pnp_card_link) -> c_int {
    snd_miro_resume(pnp_get_card_drvdata(pcard) as *mut snd_card)
}

/* CONFIG_PNP */
static mut miro_pnpc_driver: pnp_card_driver = pnp_card_driver {
    flags: PNP_DRIVER_RES_DISABLE,
    name: b"miro\0".as_ptr() as *const c_char,
    id_table: snd_miro_pnpids.as_ptr(),
    probe: Some(snd_miro_pnp_probe),
    remove: Some(snd_miro_pnp_remove),
    /* CONFIG_PM */
    suspend: Some(snd_miro_pnp_suspend),
    resume: Some(snd_miro_pnp_resume),
};

unsafe extern "C" fn alsa_card_miro_init() -> c_int {
    /* CONFIG_PNP */
    pnp_register_card_driver(&mut miro_pnpc_driver);
    if snd_miro_pnp_is_probed != 0 {
        return 0;
    }
    pnp_unregister_card_driver(&mut miro_pnpc_driver);
    isa_register_driver(&mut snd_miro_driver, 1)
}

unsafe extern "C" fn alsa_card_miro_exit() {
    if snd_miro_pnp_is_probed == 0 {
        isa_unregister_driver(&mut snd_miro_driver);
        return;
    }
    /* CONFIG_PNP */
    pnp_unregister_card_driver(&mut miro_pnpc_driver);
}

/* module_init(alsa_card_miro_init) */
/* module_exit(alsa_card_miro_exit) */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
