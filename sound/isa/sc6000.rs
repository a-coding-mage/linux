// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Driver for Gallant SC-6000 soundcard. This card is also known as
 *  Audio Excel DSP 16 or Zoltrix AV302.
 *  These cards use CompuMedia ASC-9308 chip + AD1848 codec.
 *  SC-6600 and SC-7000 cards are also supported. They are based on
 *  CompuMedia ASC-9408 chip and CS4231 codec.
 *
 *  Copyright (C) 2007 Krzysztof Helt <krzysztof.h1@wp.pl>
 *
 *  I don't have documentation for this card. I used the driver
 *  for OSS/Free included in the kernel source as reference.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::null_mut;

const SNDRV_CARDS: usize = 8;
const SNDRV_DEFAULT_IDX: [c_int; SNDRV_CARDS] = [-1; SNDRV_CARDS];
const SNDRV_DEFAULT_STR: [*mut c_char; SNDRV_CARDS] = [null_mut(); SNDRV_CARDS];
const SNDRV_DEFAULT_ENABLE: [bool; SNDRV_CARDS] = [false; SNDRV_CARDS];
const SNDRV_DEFAULT_PORT: [c_long; SNDRV_CARDS] = [-1; SNDRV_CARDS];
const SNDRV_DEFAULT_IRQ: [c_int; SNDRV_CARDS] = [-1; SNDRV_CARDS];
const SNDRV_DEFAULT_DMA: [c_int; SNDRV_CARDS] = [-1; SNDRV_CARDS];
const SNDRV_AUTO_PORT: c_long = -1;
const SNDRV_AUTO_IRQ: c_int = -1;
const SNDRV_AUTO_DMA: c_int = -1;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_uint = 2;
const SNDRV_CTL_POWER_D3hot: c_int = 3;
const SNDRV_CTL_POWER_D0: c_int = 0;
const WSS_HW_DETECT: c_int = 0;
const OPL3_HW_AUTO: c_int = 0;
const MPU401_HW_MPU401: c_int = 0;

const EAGAIN: c_int = 11;
const EBUSY: c_int = 16;
const ENODEV: c_int = 19;
const EIO: c_int = 5;

// Linux module metadata and module_param declarations are supplied by the
// kernel module framework in the original C translation unit.

static mut index: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IDX; /* Index 0-MAX */
static mut id: [*mut c_char; SNDRV_CARDS] = SNDRV_DEFAULT_STR; /* ID for this card */
static mut enable: [bool; SNDRV_CARDS] = SNDRV_DEFAULT_ENABLE; /* Enable this card */
static mut port: [c_long; SNDRV_CARDS] = SNDRV_DEFAULT_PORT; /* 0x220, 0x240 */
static mut irq: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IRQ; /* 5, 7, 9, 10, 11 */
static mut mss_port: [c_long; SNDRV_CARDS] = SNDRV_DEFAULT_PORT; /* 0x530, 0xe80 */
static mut mpu_port: [c_long; SNDRV_CARDS] = SNDRV_DEFAULT_PORT;
/* 0x300, 0x310, 0x320, 0x330 */
static mut mpu_irq: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IRQ; /* 5, 7, 9, 10, 0 */
static mut dma: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_DMA; /* 0, 1, 3 */
static mut joystick: [bool; SNDRV_CARDS] = [false; SNDRV_CARDS];

/*
 * Commands of SC6000's DSP (SBPRO+special).
 * Some of them are COMMAND_xx, in the future they may change.
 */
const WRITE_MDIRQ_CFG: c_int = 0x50; /* Set M&I&DRQ mask (the real config)	*/
const COMMAND_52: c_int = 0x52; /*					*/
const READ_HARD_CFG: c_int = 0x58; /* Read Hardware Config (I/O base etc)	*/
const COMMAND_5C: c_int = 0x5c; /*					*/
const COMMAND_60: c_int = 0x60; /*					*/
const COMMAND_66: c_int = 0x66; /*					*/
const COMMAND_6C: c_int = 0x6c; /*					*/
const COMMAND_6E: c_int = 0x6e; /*					*/
const COMMAND_88: c_int = 0x88; /* Unknown command 			*/
const DSP_INIT_MSS: c_int = 0x8c; /* Enable Microsoft Sound System mode	*/
const COMMAND_C5: c_int = 0xc5; /*					*/
const GET_DSP_VERSION: c_int = 0xe1; /* Get DSP Version			*/
const GET_DSP_COPYRIGHT: c_int = 0xe3; /* Get DSP Copyright			*/

/*
 * Offsets of SC6000 DSP I/O ports. The offset is added to base I/O port
 * to have the actual I/O port.
 * Register permissions are:
 * (wo) == Write Only
 * (ro) == Read  Only
 * (w-) == Write
 * (r-) == Read
 */
const DSP_RESET: isize = 0x06; /* offset of DSP RESET		(wo) */
const DSP_READ: isize = 0x0a; /* offset of DSP READ		(ro) */
const DSP_WRITE: isize = 0x0c; /* offset of DSP WRITE		(w-) */
const DSP_COMMAND: isize = 0x0c; /* offset of DSP COMMAND	(w-) */
const DSP_STATUS: isize = 0x0c; /* offset of DSP STATUS		(r-) */
const DSP_DATAVAIL: isize = 0x0e; /* offset of DSP DATA AVAILABLE	(ro) */

const PFX: &[u8] = b"sc6000: \0";
const DRV_NAME: &[u8] = b"SC-6000\0";

#[repr(C)]
struct device {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_card {
    private_data: *mut c_void,
    private_free: Option<unsafe extern "C" fn(*mut snd_card)>,
    dev: *mut device,
    driver: [c_char; 16],
    shortname: [c_char; 32],
    longname: [c_char; 80],
}

#[repr(C)]
struct snd_wss {
    card: *mut snd_card,
    suspend: Option<unsafe extern "C" fn(*mut snd_wss)>,
    resume: Option<unsafe extern "C" fn(*mut snd_wss)>,
}

#[repr(C)]
struct snd_opl3 {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_ctl_elem_id {
    numid: c_uint,
    iface: c_uint,
    device: c_uint,
    subdevice: c_uint,
    name: [c_char; 44],
    index: c_uint,
}

#[repr(C)]
struct pm_message_t {
    event: c_int,
}

#[repr(C)]
struct device_driver {
    name: *const c_char,
}

#[repr(C)]
struct isa_driver {
    match_: Option<unsafe extern "C" fn(*mut device, c_uint) -> c_int>,
    probe: Option<unsafe extern "C" fn(*mut device, c_uint) -> c_int>,
    /* CONFIG_PM: suspend/resume fields are present in the C initializer when enabled. */
    suspend: Option<unsafe extern "C" fn(*mut device, c_uint, pm_message_t) -> c_int>,
    resume: Option<unsafe extern "C" fn(*mut device, c_uint) -> c_int>,
    driver: device_driver,
}

#[repr(C)]
struct snd_sc6000 {
    vport: *mut c_char,
    vmss_port: *mut c_char,
    chip: *mut snd_wss,
    mss_config: u8,
    config: u8,
    hw_cfg: [u8; 2],
    old_dsp: bool,
}

unsafe extern "C" {
    static mut THIS_MODULE: *mut c_void;

    fn ioread8(addr: *mut c_char) -> u8;
    fn iowrite8(value: c_int, addr: *mut c_char);
    fn cpu_relax();
    fn udelay(usecs: c_uint);
    fn msleep(msecs: c_uint);

    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);

    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strncmp(cs: *const c_char, ct: *const c_char, count: usize) -> c_int;
    fn strscpy(dest: *mut c_char, src: *const c_char, ...) -> isize;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;

    fn snd_ctl_rename_id(
        card: *mut snd_card,
        src_id: *mut snd_ctl_elem_id,
        dst_id: *mut snd_ctl_elem_id,
    ) -> c_int;
    fn snd_devm_card_new(
        parent: *mut device,
        idx: c_int,
        xid: *mut c_char,
        module: *mut c_void,
        extra_size: usize,
        card_ret: *mut *mut snd_card,
    ) -> c_int;
    fn snd_legacy_find_free_irq(possible_irqs: *const c_int) -> c_int;
    fn snd_legacy_find_free_dma(possible_dmas: *const c_int) -> c_int;
    fn devm_request_region(
        dev: *mut device,
        start: c_long,
        n: c_long,
        name: *const c_char,
    ) -> *mut c_void;
    fn devm_ioport_map(dev: *mut device, port: c_long, nr: c_uint) -> *mut c_char;
    fn snd_wss_create(
        card: *mut snd_card,
        port: c_long,
        cport: c_long,
        irq: c_int,
        dma1: c_int,
        dma2: c_int,
        hardware: c_int,
        hwshare: c_int,
        rchip: *mut *mut snd_wss,
    ) -> c_int;
    fn snd_wss_pcm(chip: *mut snd_wss, device: c_int) -> c_int;
    fn snd_wss_mixer(chip: *mut snd_wss) -> c_int;
    fn snd_opl3_create(
        card: *mut snd_card,
        l_port: c_long,
        r_port: c_long,
        hardware: c_int,
        integrated: c_int,
        ropl3: *mut *mut snd_opl3,
    ) -> c_int;
    fn snd_opl3_hwdep_new(opl3: *mut snd_opl3, device: c_int, seq_device: c_int, ops: *mut c_void)
        -> c_int;
    fn snd_mpu401_uart_new(
        card: *mut snd_card,
        device: c_int,
        hardware: c_uint,
        port: c_long,
        integrated: c_uint,
        irq: c_int,
        rrawmidi: *mut c_void,
    ) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn snd_card_free_on_error(dev: *mut device, err: c_int) -> c_int;
    fn snd_power_change_state(card: *mut snd_card, state: c_int) -> c_int;
}

/* hardware dependent functions */

/*
 * sc6000_irq_to_softcfg - Decode irq number into cfg code.
 */
unsafe fn sc6000_irq_to_softcfg(irq: c_int) -> u8 {
    let mut val: u8 = 0;

    match irq {
        5 => val = 0x28,
        7 => val = 0x8,
        9 => val = 0x10,
        10 => val = 0x18,
        11 => val = 0x20,
        _ => {}
    }
    val
}

/*
 * sc6000_dma_to_softcfg - Decode dma number into cfg code.
 */
unsafe fn sc6000_dma_to_softcfg(dma: c_int) -> u8 {
    let mut val: u8 = 0;

    match dma {
        0 => val = 1,
        1 => val = 2,
        3 => val = 3,
        _ => {}
    }
    val
}

/*
 * sc6000_mpu_irq_to_softcfg - Decode MPU-401 irq number into cfg code.
 */
unsafe fn sc6000_mpu_irq_to_softcfg(mpu_irq: c_int) -> u8 {
    let mut val: u8 = 0;

    match mpu_irq {
        5 => val = 4,
        7 => val = 0x44,
        9 => val = 0x84,
        10 => val = 0xc4,
        _ => {}
    }
    val
}

unsafe fn sc6000_wait_data(vport: *mut c_char) -> c_int {
    let mut loop_: c_int = 1000;
    let mut val: u8 = 0;

    loop {
        val = ioread8(vport.offset(DSP_DATAVAIL));
        if (val & 0x80) != 0 {
            return 0;
        }
        cpu_relax();
        let old = loop_;
        loop_ -= 1;
        if old == 0 {
            break;
        }
    }

    -EAGAIN
}

unsafe fn sc6000_read(vport: *mut c_char) -> c_int {
    if sc6000_wait_data(vport) != 0 {
        return -EBUSY;
    }

    ioread8(vport.offset(DSP_READ)) as c_int
}

unsafe fn sc6000_write(devptr: *mut device, vport: *mut c_char, cmd: c_int) -> c_int {
    let mut val: u8;
    let mut loop_: c_int = 500000;

    loop {
        val = ioread8(vport.offset(DSP_STATUS));
        /*
         * DSP ready to receive data if bit 7 of val == 0
         */
        if (val & 0x80) == 0 {
            iowrite8(cmd, vport.offset(DSP_COMMAND));
            return 0;
        }
        cpu_relax();
        let old = loop_;
        loop_ -= 1;
        if old == 0 {
            break;
        }
    }

    dev_err(devptr, c"DSP Command (0x%x) timeout.\n".as_ptr(), cmd);

    -EIO
}

unsafe fn sc6000_dsp_get_answer(
    devptr: *mut device,
    vport: *mut c_char,
    command: c_int,
    data: *mut c_char,
    data_len: c_int,
) -> c_int {
    let mut len: c_int = 0;

    if sc6000_write(devptr, vport, command) != 0 {
        dev_err(devptr, c"CMD 0x%x: failed!\n".as_ptr(), command);
        return -EIO;
    }

    loop {
        let val = sc6000_read(vport);

        if val < 0 {
            break;
        }

        *data.offset(len as isize) = val as c_char;
        len += 1;

        if len >= data_len {
            break;
        }
    }

    /*
     * If no more data available, return to the caller, no error if len>0.
     * We have no other way to know when the string is finished.
     */
    if len != 0 { len } else { -EIO }
}

unsafe fn sc6000_dsp_reset(vport: *mut c_char) -> c_int {
    iowrite8(1, vport.offset(DSP_RESET));
    udelay(10);
    iowrite8(0, vport.offset(DSP_RESET));
    udelay(20);
    if sc6000_read(vport) == 0xaa {
        return 0;
    }
    -ENODEV
}

/* detection and initialization */
unsafe fn sc6000_hw_cfg_write(devptr: *mut device, vport: *mut c_char, cfg: *const u8) -> c_int {
    if sc6000_write(devptr, vport, COMMAND_6C) < 0 {
        dev_warn(devptr, c"CMD 0x%x: failed!\n".as_ptr(), COMMAND_6C);
        return -EIO;
    }
    if sc6000_write(devptr, vport, COMMAND_5C) < 0 {
        dev_err(devptr, c"CMD 0x%x: failed!\n".as_ptr(), COMMAND_5C);
        return -EIO;
    }
    if sc6000_write(devptr, vport, *cfg.offset(0) as c_int) < 0 {
        dev_err(devptr, c"DATA 0x%x: failed!\n".as_ptr(), *cfg.offset(0) as c_int);
        return -EIO;
    }
    if sc6000_write(devptr, vport, *cfg.offset(1) as c_int) < 0 {
        dev_err(devptr, c"DATA 0x%x: failed!\n".as_ptr(), *cfg.offset(1) as c_int);
        return -EIO;
    }
    if sc6000_write(devptr, vport, COMMAND_C5) < 0 {
        dev_err(devptr, c"CMD 0x%x: failed!\n".as_ptr(), COMMAND_C5);
        return -EIO;
    }

    0
}

unsafe fn sc6000_cfg_write(devptr: *mut device, vport: *mut c_char, softcfg: u8) -> c_int {
    if sc6000_write(devptr, vport, WRITE_MDIRQ_CFG) != 0 {
        dev_err(devptr, c"CMD 0x%x: failed!\n".as_ptr(), WRITE_MDIRQ_CFG);
        return -EIO;
    }
    if sc6000_write(devptr, vport, softcfg as c_int) != 0 {
        dev_err(devptr, c"%s: failed!\n".as_ptr(), c"sc6000_cfg_write".as_ptr());
        return -EIO;
    }
    0
}

unsafe fn sc6000_setup_board(devptr: *mut device, vport: *mut c_char, config: c_int) -> c_int {
    let mut loop_: c_int = 10;

    loop {
        if sc6000_write(devptr, vport, COMMAND_88) != 0 {
            dev_err(devptr, c"CMD 0x%x: failed!\n".as_ptr(), COMMAND_88);
            return -EIO;
        }
        let cond = sc6000_wait_data(vport) < 0;
        let old = loop_;
        loop_ -= 1;
        if !(cond && old != 0) {
            break;
        }
    }

    if sc6000_read(vport) < 0 {
        dev_err(
            devptr,
            c"sc6000_read after CMD 0x%x: failed\n".as_ptr(),
            COMMAND_88,
        );
        return -EIO;
    }

    if sc6000_cfg_write(devptr, vport, config as u8) != 0 {
        return -ENODEV;
    }

    0
}

unsafe fn sc6000_init_mss(
    devptr: *mut device,
    vport: *mut c_char,
    config: c_int,
    vmss_port: *mut c_char,
    mss_config: c_int,
) -> c_int {
    if sc6000_write(devptr, vport, DSP_INIT_MSS) != 0 {
        dev_err(
            devptr,
            c"%s [0x%x]: failed!\n".as_ptr(),
            c"sc6000_init_mss".as_ptr(),
            DSP_INIT_MSS,
        );
        return -EIO;
    }

    msleep(10);

    if sc6000_cfg_write(devptr, vport, config as u8) != 0 {
        return -EIO;
    }

    iowrite8(mss_config, vmss_port);

    0
}

unsafe fn sc6000_hw_cfg_encode(
    devptr: *mut device,
    cfg: *mut u8,
    xport: c_long,
    xmpu: c_long,
    xmss_port: c_long,
    joystick: c_int,
) {
    *cfg.offset(0) = 0;
    *cfg.offset(1) = 0;
    if xport == 0x240 {
        *cfg.offset(0) |= 1;
    }
    if xmpu != SNDRV_AUTO_PORT {
        *cfg.offset(0) |= ((xmpu & 0x30) >> 2) as u8;
        *cfg.offset(1) |= 0x20;
    }
    if xmss_port == 0xe80 {
        *cfg.offset(0) |= 0x10;
    }
    *cfg.offset(0) |= 0x40; /* always set */
    if joystick == 0 {
        *cfg.offset(0) |= 0x02;
    }
    *cfg.offset(1) |= 0x80; /* enable WSS system */
    *cfg.offset(1) &= !0x40; /* disable IDE */
    dev_dbg(
        devptr,
        c"hw cfg %x, %x\n".as_ptr(),
        *cfg.offset(0) as c_int,
        *cfg.offset(1) as c_int,
    );
}

unsafe fn sc6000_prepare_board(
    devptr: *mut device,
    sc6000: *mut snd_sc6000,
    dev: c_uint,
    xirq: c_int,
    xdma: c_int,
) {
    (*sc6000).mss_config = sc6000_irq_to_softcfg(xirq) | sc6000_dma_to_softcfg(xdma);
    (*sc6000).config =
        (*sc6000).mss_config | sc6000_mpu_irq_to_softcfg(mpu_irq[dev as usize]);
    sc6000_hw_cfg_encode(
        devptr,
        (*sc6000).hw_cfg.as_mut_ptr(),
        port[dev as usize],
        mpu_port[dev as usize],
        mss_port[dev as usize],
        joystick[dev as usize] as c_int,
    );
}

unsafe fn sc6000_detect_old_dsp(devptr: *mut device, sc6000: *mut snd_sc6000) {
    sc6000_write(devptr, (*sc6000).vport, COMMAND_5C);
    (*sc6000).old_dsp = sc6000_read((*sc6000).vport) < 0;
}

unsafe fn sc6000_program_board(devptr: *mut device, sc6000: *mut snd_sc6000) -> c_int {
    let mut err: c_int;

    if !(*sc6000).old_dsp {
        if sc6000_hw_cfg_write(devptr, (*sc6000).vport, (*sc6000).hw_cfg.as_ptr()) < 0 {
            dev_err(devptr, c"sc6000_hw_cfg_write: failed!\n".as_ptr());
            return -EIO;
        }
    }

    err = sc6000_setup_board(devptr, (*sc6000).vport, (*sc6000).config as c_int);
    if err < 0 {
        dev_err(devptr, c"sc6000_setup_board: failed!\n".as_ptr());
        return -ENODEV;
    }

    sc6000_dsp_reset((*sc6000).vport);

    if !(*sc6000).old_dsp {
        sc6000_write(devptr, (*sc6000).vport, COMMAND_60);
        sc6000_write(devptr, (*sc6000).vport, 0x02);
        sc6000_dsp_reset((*sc6000).vport);
    }

    err = sc6000_setup_board(devptr, (*sc6000).vport, (*sc6000).config as c_int);
    if err < 0 {
        dev_err(devptr, c"sc6000_setup_board: failed!\n".as_ptr());
        return -ENODEV;
    }

    err = sc6000_init_mss(
        devptr,
        (*sc6000).vport,
        (*sc6000).config as c_int,
        (*sc6000).vmss_port,
        (*sc6000).mss_config as c_int,
    );
    if err < 0 {
        dev_err(
            devptr,
            c"Cannot initialize Microsoft Sound System mode.\n".as_ptr(),
        );
        return -ENODEV;
    }

    0
}

unsafe fn sc6000_init_board(devptr: *mut device, sc6000: *mut snd_sc6000) -> c_int {
    let mut answer: [c_char; 15] = [0; 15];
    let mut version: [c_char; 2] = [0; 2];
    let mut err: c_int;

    err = sc6000_dsp_reset((*sc6000).vport);
    if err < 0 {
        dev_err(devptr, c"sc6000_dsp_reset: failed!\n".as_ptr());
        return err;
    }

    memset(answer.as_mut_ptr() as *mut c_void, 0, size_of::<[c_char; 15]>());
    err = sc6000_dsp_get_answer(
        devptr,
        (*sc6000).vport,
        GET_DSP_COPYRIGHT,
        answer.as_mut_ptr(),
        15,
    );
    if err <= 0 {
        dev_err(devptr, c"sc6000_dsp_copyright: failed!\n".as_ptr());
        return -ENODEV;
    }
    /*
     * My SC-6000 card return "SC-6000" in DSPCopyright, so
     * if we have something different, we have to be warned.
     */
    if strncmp(c"SC-6000".as_ptr(), answer.as_ptr(), 7) != 0 {
        dev_warn(devptr, c"Warning: non SC-6000 audio card!\n".as_ptr());
    }

    if sc6000_dsp_get_answer(
        devptr,
        (*sc6000).vport,
        GET_DSP_VERSION,
        version.as_mut_ptr(),
        2,
    ) < 2
    {
        dev_err(devptr, c"sc6000_dsp_version: failed!\n".as_ptr());
        return -ENODEV;
    }
    dev_info(
        devptr,
        c"Detected model: %s, DSP version %d.%d\n".as_ptr(),
        answer.as_ptr(),
        version[0] as c_int,
        version[1] as c_int,
    );

    sc6000_detect_old_dsp(devptr, sc6000);

    sc6000_program_board(devptr, sc6000)
}

unsafe fn snd_sc6000_mixer(chip: *mut snd_wss) -> c_int {
    let card: *mut snd_card = (*chip).card;
    let mut id1: snd_ctl_elem_id = zeroed();
    let mut id2: snd_ctl_elem_id = zeroed();
    let mut err: c_int;

    memset(
        &mut id1 as *mut snd_ctl_elem_id as *mut c_void,
        0,
        size_of::<snd_ctl_elem_id>(),
    );
    memset(
        &mut id2 as *mut snd_ctl_elem_id as *mut c_void,
        0,
        size_of::<snd_ctl_elem_id>(),
    );
    id1.iface = SNDRV_CTL_ELEM_IFACE_MIXER;
    id2.iface = SNDRV_CTL_ELEM_IFACE_MIXER;
    /* reassign AUX0 to FM */
    strscpy(id1.name.as_mut_ptr(), c"Aux Playback Switch".as_ptr());
    strscpy(id2.name.as_mut_ptr(), c"FM Playback Switch".as_ptr());
    err = snd_ctl_rename_id(card, &mut id1, &mut id2);
    if err < 0 {
        return err;
    }
    strscpy(id1.name.as_mut_ptr(), c"Aux Playback Volume".as_ptr());
    strscpy(id2.name.as_mut_ptr(), c"FM Playback Volume".as_ptr());
    err = snd_ctl_rename_id(card, &mut id1, &mut id2);
    if err < 0 {
        return err;
    }
    /* reassign AUX1 to CD */
    strscpy(id1.name.as_mut_ptr(), c"Aux Playback Switch".as_ptr());
    id1.index = 1;
    strscpy(id2.name.as_mut_ptr(), c"CD Playback Switch".as_ptr());
    err = snd_ctl_rename_id(card, &mut id1, &mut id2);
    if err < 0 {
        return err;
    }
    strscpy(id1.name.as_mut_ptr(), c"Aux Playback Volume".as_ptr());
    strscpy(id2.name.as_mut_ptr(), c"CD Playback Volume".as_ptr());
    err = snd_ctl_rename_id(card, &mut id1, &mut id2);
    if err < 0 {
        return err;
    }

    0
}

unsafe extern "C" fn snd_sc6000_match(devptr: *mut device, dev: c_uint) -> c_int {
    if !enable[dev as usize] {
        return 0;
    }
    if port[dev as usize] == SNDRV_AUTO_PORT {
        dev_err(devptr, c"specify IO port\n".as_ptr());
        return 0;
    }
    if mss_port[dev as usize] == SNDRV_AUTO_PORT {
        dev_err(devptr, c"specify MSS port\n".as_ptr());
        return 0;
    }
    if port[dev as usize] != 0x220 && port[dev as usize] != 0x240 {
        dev_err(devptr, c"Port must be 0x220 or 0x240\n".as_ptr());
        return 0;
    }
    if mss_port[dev as usize] != 0x530 && mss_port[dev as usize] != 0xe80 {
        dev_err(devptr, c"MSS port must be 0x530 or 0xe80\n".as_ptr());
        return 0;
    }
    if irq[dev as usize] != SNDRV_AUTO_IRQ && sc6000_irq_to_softcfg(irq[dev as usize]) == 0 {
        dev_err(devptr, c"invalid IRQ %d\n".as_ptr(), irq[dev as usize]);
        return 0;
    }
    if dma[dev as usize] != SNDRV_AUTO_DMA && sc6000_dma_to_softcfg(dma[dev as usize]) == 0 {
        dev_err(devptr, c"invalid DMA %d\n".as_ptr(), dma[dev as usize]);
        return 0;
    }
    if mpu_port[dev as usize] != SNDRV_AUTO_PORT
        && (mpu_port[dev as usize] & !(0x30 as c_long)) != 0x300
    {
        dev_err(
            devptr,
            c"invalid MPU-401 port %lx\n".as_ptr(),
            mpu_port[dev as usize],
        );
        return 0;
    }
    if mpu_port[dev as usize] != SNDRV_AUTO_PORT
        && mpu_irq[dev as usize] != SNDRV_AUTO_IRQ
        && mpu_irq[dev as usize] != 0
        && sc6000_mpu_irq_to_softcfg(mpu_irq[dev as usize]) == 0
    {
        dev_err(
            devptr,
            c"invalid MPU-401 IRQ %d\n".as_ptr(),
            mpu_irq[dev as usize],
        );
        return 0;
    }
    1
}

unsafe extern "C" fn snd_sc6000_free(card: *mut snd_card) {
    let sc6000: *mut snd_sc6000 = (*card).private_data as *mut snd_sc6000;

    if !(*sc6000).vport.is_null() {
        sc6000_setup_board((*card).dev, (*sc6000).vport, 0);
    }
}

unsafe fn __snd_sc6000_probe(devptr: *mut device, dev: c_uint) -> c_int {
    static possible_irqs: [c_int; 6] = [5, 7, 9, 10, 11, -1];
    static possible_dmas: [c_int; 4] = [1, 3, 0, -1];
    let mut err: c_int;
    let mut xirq: c_int = irq[dev as usize];
    let mut xdma: c_int = dma[dev as usize];
    let mut card: *mut snd_card = null_mut();
    let mut sc6000: *mut snd_sc6000;
    let mut chip: *mut snd_wss = null_mut();
    let mut opl3: *mut snd_opl3 = null_mut();
    let mut vport: *mut c_char;
    let mut vmss_port: *mut c_char;

    err = snd_devm_card_new(
        devptr,
        index[dev as usize],
        id[dev as usize],
        THIS_MODULE,
        size_of::<snd_sc6000>(),
        &mut card,
    );
    if err < 0 {
        return err;
    }
    sc6000 = (*card).private_data as *mut snd_sc6000;

    if xirq == SNDRV_AUTO_IRQ {
        xirq = snd_legacy_find_free_irq(possible_irqs.as_ptr());
        if xirq < 0 {
            dev_err(devptr, c"unable to find a free IRQ\n".as_ptr());
            return -EBUSY;
        }
    }

    if xdma == SNDRV_AUTO_DMA {
        xdma = snd_legacy_find_free_dma(possible_dmas.as_ptr());
        if xdma < 0 {
            dev_err(devptr, c"unable to find a free DMA\n".as_ptr());
            return -EBUSY;
        }
    }

    if devm_request_region(devptr, port[dev as usize], 0x10, DRV_NAME.as_ptr() as *const c_char)
        .is_null()
    {
        dev_err(devptr, c"I/O port region is already in use.\n".as_ptr());
        return -EBUSY;
    }
    vport = devm_ioport_map(devptr, port[dev as usize], 0x10);
    if vport.is_null() {
        dev_err(devptr, c"I/O port cannot be iomapped.\n".as_ptr());
        return -EBUSY;
    }
    (*sc6000).vport = vport;

    /* to make it marked as used */
    if devm_request_region(devptr, mss_port[dev as usize], 4, DRV_NAME.as_ptr() as *const c_char)
        .is_null()
    {
        dev_err(
            devptr,
            c"SC-6000 port I/O port region is already in use.\n".as_ptr(),
        );
        return -EBUSY;
    }
    vmss_port = devm_ioport_map(devptr, mss_port[dev as usize], 4);
    if vmss_port.is_null() {
        dev_err(devptr, c"MSS port I/O cannot be iomapped.\n".as_ptr());
        return -EBUSY;
    }
    (*sc6000).vmss_port = vmss_port;

    dev_dbg(
        devptr,
        c"Initializing BASE[0x%lx] IRQ[%d] DMA[%d] MIRQ[%d]\n".as_ptr(),
        port[dev as usize],
        xirq,
        xdma,
        if mpu_irq[dev as usize] == SNDRV_AUTO_IRQ {
            0
        } else {
            mpu_irq[dev as usize]
        },
    );

    sc6000_prepare_board(devptr, sc6000, dev, xirq, xdma);

    err = sc6000_init_board(devptr, sc6000);
    if err < 0 {
        return err;
    }
    (*card).private_free = Some(snd_sc6000_free);

    err = snd_wss_create(
        card,
        mss_port[dev as usize] + 4,
        -1,
        xirq,
        xdma,
        -1,
        WSS_HW_DETECT,
        0,
        &mut chip,
    );
    if err < 0 {
        return err;
    }
    (*sc6000).chip = chip;

    err = snd_wss_pcm(chip, 0);
    if err < 0 {
        dev_err(devptr, c"error creating new WSS PCM device\n".as_ptr());
        return err;
    }
    err = snd_wss_mixer(chip);
    if err < 0 {
        dev_err(devptr, c"error creating new WSS mixer\n".as_ptr());
        return err;
    }
    err = snd_sc6000_mixer(chip);
    if err < 0 {
        dev_err(devptr, c"the mixer rewrite failed\n".as_ptr());
        return err;
    }
    if snd_opl3_create(card, 0x388, 0x388 + 2, OPL3_HW_AUTO, 0, &mut opl3) < 0 {
        dev_err(
            devptr,
            c"no OPL device at 0x%x-0x%x ?\n".as_ptr(),
            0x388,
            0x388 + 2,
        );
    } else {
        err = snd_opl3_hwdep_new(opl3, 0, 1, null_mut());
        if err < 0 {
            return err;
        }
    }

    if mpu_port[dev as usize] != SNDRV_AUTO_PORT {
        if mpu_irq[dev as usize] == SNDRV_AUTO_IRQ {
            mpu_irq[dev as usize] = -1;
        }
        if snd_mpu401_uart_new(
            card,
            0,
            MPU401_HW_MPU401 as c_uint,
            mpu_port[dev as usize],
            0,
            mpu_irq[dev as usize],
            null_mut(),
        ) < 0
        {
            dev_err(
                devptr,
                c"no MPU-401 device at 0x%lx ?\n".as_ptr(),
                mpu_port[dev as usize],
            );
        }
    }

    strscpy((*card).driver.as_mut_ptr(), DRV_NAME.as_ptr() as *const c_char);
    strscpy((*card).shortname.as_mut_ptr(), c"SC-6000".as_ptr());
    sprintf(
        (*card).longname.as_mut_ptr(),
        c"Gallant SC-6000 at 0x%lx, irq %d, dma %d".as_ptr(),
        mss_port[dev as usize],
        xirq,
        xdma,
    );

    err = snd_card_register(card);
    if err < 0 {
        return err;
    }

    dev_set_drvdata(devptr, card as *mut c_void);
    0
}

unsafe extern "C" fn snd_sc6000_probe(devptr: *mut device, dev: c_uint) -> c_int {
    snd_card_free_on_error(devptr, __snd_sc6000_probe(devptr, dev))
}

/* CONFIG_PM */
unsafe extern "C" fn snd_sc6000_suspend(
    devptr: *mut device,
    _dev: c_uint,
    _state: pm_message_t,
) -> c_int {
    let card: *mut snd_card = dev_get_drvdata(devptr) as *mut snd_card;
    let sc6000: *mut snd_sc6000 = (*card).private_data as *mut snd_sc6000;

    snd_power_change_state(card, SNDRV_CTL_POWER_D3hot);
    if let Some(suspend) = (*(*sc6000).chip).suspend {
        suspend((*sc6000).chip);
    }
    0
}

unsafe extern "C" fn snd_sc6000_resume(devptr: *mut device, _dev: c_uint) -> c_int {
    let card: *mut snd_card = dev_get_drvdata(devptr) as *mut snd_card;
    let sc6000: *mut snd_sc6000 = (*card).private_data as *mut snd_sc6000;
    let mut err: c_int;

    err = sc6000_dsp_reset((*sc6000).vport);
    if err < 0 {
        dev_err(devptr, c"sc6000_dsp_reset: failed!\n".as_ptr());
        return err;
    }

    err = sc6000_program_board(devptr, sc6000);
    if err < 0 {
        return err;
    }

    if let Some(resume) = (*(*sc6000).chip).resume {
        resume((*sc6000).chip);
    }
    snd_power_change_state(card, SNDRV_CTL_POWER_D0);
    0
}

static mut snd_sc6000_driver: isa_driver = isa_driver {
    match_: Some(snd_sc6000_match),
    probe: Some(snd_sc6000_probe),
    /* CONFIG_PM */
    suspend: Some(snd_sc6000_suspend),
    resume: Some(snd_sc6000_resume),
    driver: device_driver {
        name: DRV_NAME.as_ptr() as *const c_char,
    },
};

// module_isa_driver(snd_sc6000_driver, SNDRV_CARDS);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
