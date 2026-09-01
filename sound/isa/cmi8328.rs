// SPDX-License-Identifier: GPL-2.0-only
/*
 * Driver for C-Media CMI8328-based soundcards, such as AudioExcel AV500
 * Copyright (c) 2012 Ondrej Zary
 *
 * AudioExcel AV500 card consists of:
 *  - CMI8328 - main chip (SB Pro emulation, gameport, OPL3, MPU401, CD-ROM)
 *  - CS4231A - WSS codec
 *  - Dream SAM9233+GMS950400+RAM+ROM: Wavetable MIDI, connected to MPU401
 */

// Dependencies from the original C includes:
// linux/init.h, linux/isa.h, linux/module.h, linux/gameport.h, asm/dma.h,
// sound/core.h, sound/wss.h, sound/opl3.h, sound/mpu401.h, sound/initval.h.

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem;
use core::ptr;

type u8 = u8;
type u16 = u16;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct resource {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
    pub private_data: *mut c_void,
    pub driver: [c_char; 16],
    pub shortname: [c_char; 32],
    pub longname: [c_char; 80],
}

#[repr(C)]
pub struct snd_wss {
    pub card: *mut snd_card,
    pub port: c_long,
    pub suspend: Option<unsafe extern "C" fn(*mut snd_wss)>,
    pub resume: Option<unsafe extern "C" fn(*mut snd_wss)>,
}

#[repr(C)]
pub struct snd_opl3 {
    _private: [u8; 0],
}

#[cfg(SUPPORT_JOYSTICK)]
#[repr(C)]
pub struct gameport {
    pub io: c_uint,
}

#[repr(C)]
pub struct snd_ctl_elem_id {
    pub numid: c_uint,
    pub iface: c_uint,
    pub device: c_uint,
    pub subdevice: c_uint,
    pub name: [c_char; 44],
    pub index: c_uint,
}

#[repr(C)]
pub struct pm_message_t {
    pub event: c_int,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
}

#[repr(C)]
pub struct isa_driver {
    pub probe: Option<unsafe extern "C" fn(*mut device, c_uint) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut device, c_uint)>,
    #[cfg(CONFIG_PM)]
    pub suspend: Option<unsafe extern "C" fn(*mut device, c_uint, pm_message_t) -> c_int>,
    #[cfg(CONFIG_PM)]
    pub resume: Option<unsafe extern "C" fn(*mut device, c_uint) -> c_int>,
    pub driver: device_driver,
}

#[repr(C)]
pub struct snd_cmi8328 {
    port: u16,
    cfg: [u8; 3],
    wss_cfg: u8,
    card: *mut snd_card,
    wss: *mut snd_wss,
    #[cfg(SUPPORT_JOYSTICK)]
    gameport: *mut gameport,
}

extern "C" {
    static THIS_MODULE: *mut module;

    fn outb(value: u8, port: u16);
    fn inb(port: u16) -> u8;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn strscpy(dest: *mut c_char, src: *const c_char) -> isize;

    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_name(dev: *mut device) -> *const c_char;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn devm_request_region(
        dev: *mut device,
        start: c_long,
        n: c_long,
        name: *const c_char,
    ) -> *mut resource;

    fn snd_ctl_rename_id(
        card: *mut snd_card,
        src_id: *mut snd_ctl_elem_id,
        dst_id: *mut snd_ctl_elem_id,
    ) -> c_int;
    fn snd_devm_card_new(
        parent: *mut device,
        idx: c_int,
        xid: *mut c_char,
        module: *mut module,
        extra_size: usize,
        card_ret: *mut *mut snd_card,
    ) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn snd_power_change_state(card: *mut snd_card, state: c_int);

    fn snd_wss_create(
        card: *mut snd_card,
        port: u16,
        cport: c_int,
        irq: c_int,
        dma1: c_int,
        dma2: c_int,
        hardware: c_int,
        hwshare: c_int,
        rchip: *mut *mut snd_wss,
    ) -> c_int;
    fn snd_wss_pcm(chip: *mut snd_wss, device: c_int) -> c_int;
    fn snd_wss_mixer(chip: *mut snd_wss) -> c_int;
    fn snd_wss_timer(chip: *mut snd_wss, device: c_int) -> c_int;

    fn snd_mpu401_uart_new(
        card: *mut snd_card,
        device: c_int,
        hardware: c_int,
        port: c_long,
        integrated: c_int,
        irq: c_int,
        rrawmidi: *mut c_void,
    ) -> c_int;
    fn snd_opl3_create(
        card: *mut snd_card,
        l_port: c_long,
        r_port: c_long,
        hardware: c_int,
        integrated: c_int,
        ropl3: *mut *mut snd_opl3,
    ) -> c_int;
    fn snd_opl3_hwdep_new(opl3: *mut snd_opl3, device: c_int, seq_device: c_int, rhwdep: *mut c_void)
        -> c_int;

    fn snd_legacy_find_free_irq(table: *const c_int) -> c_int;
    fn snd_legacy_find_free_dma(table: *const c_int) -> c_int;
    fn snd_legacy_find_free_ioport(table: *const c_long, size: c_long) -> c_long;

    #[cfg(SUPPORT_JOYSTICK)]
    fn gameport_allocate_port() -> *mut gameport;
    #[cfg(SUPPORT_JOYSTICK)]
    fn gameport_set_name(gameport: *mut gameport, name: *const c_char);
    #[cfg(SUPPORT_JOYSTICK)]
    fn gameport_set_phys(gameport: *mut gameport, fmt: *const c_char, ...);
    #[cfg(SUPPORT_JOYSTICK)]
    fn gameport_set_dev_parent(gameport: *mut gameport, dev: *mut device);
    #[cfg(SUPPORT_JOYSTICK)]
    fn gameport_register_port(gameport: *mut gameport);
    #[cfg(SUPPORT_JOYSTICK)]
    fn gameport_unregister_port(gameport: *mut gameport);
}

const NULL: *mut c_void = ptr::null_mut();
const ENODEV: c_int = 19;
const EBUSY: c_int = 16;
const EINVAL: c_int = 22;
const SNDRV_AUTO_PORT: c_long = -1;
const SNDRV_AUTO_IRQ: c_int = -1;
const SNDRV_AUTO_DMA: c_int = -1;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_uint = 2;
const WSS_HW_DETECT: c_int = 0;
const MPU401_HW_MPU401: c_int = 1;
const OPL3_HW_AUTO: c_int = 0;
const SNDRV_CTL_POWER_D3hot: c_int = 3;
const SNDRV_CTL_POWER_D0: c_int = 0;

/* I/O port is configured by jumpers on the card to one of these */
static cmi8328_ports: [c_int; 4] = [0x530, 0xe80, 0xf40, 0x604];
const CMI8328_MAX: usize = cmi8328_ports.len();

static mut index: [c_int; CMI8328_MAX] = [-1; CMI8328_MAX];
static mut id: [*mut c_char; CMI8328_MAX] = [ptr::null_mut(); CMI8328_MAX];
static mut port: [c_long; CMI8328_MAX] = [SNDRV_AUTO_PORT; CMI8328_MAX];
static mut irq: [c_int; CMI8328_MAX] = [SNDRV_AUTO_IRQ; CMI8328_MAX];
static mut dma1: [c_int; CMI8328_MAX] = [SNDRV_AUTO_DMA; CMI8328_MAX];
static mut dma2: [c_int; CMI8328_MAX] = [SNDRV_AUTO_DMA; CMI8328_MAX];
static mut mpuport: [c_long; CMI8328_MAX] = [SNDRV_AUTO_PORT; CMI8328_MAX];
static mut mpuirq: [c_int; CMI8328_MAX] = [SNDRV_AUTO_IRQ; CMI8328_MAX];
#[cfg(SUPPORT_JOYSTICK)]
static mut gameport: [bool; CMI8328_MAX] = [true; CMI8328_MAX];

// Original module metadata and module_param declarations are Linux module annotations:
// MODULE_AUTHOR("Ondrej Zary <linux@rainbow-software.org>");
// MODULE_DESCRIPTION("C-Media CMI8328");
// MODULE_LICENSE("GPL");

/* CMI8328 configuration registers */
const CFG1: u8 = 0x61;
const CFG1_SB_DISABLE: u8 = 1 << 0;
const CFG1_GAMEPORT: u8 = 1 << 1;
/*
 * bit 0:    SB: 0=enabled, 1=disabled
 * bit 1:    gameport: 0=disabled, 1=enabled
 * bits 2-4: SB IRQ: 001=3, 010=5, 011=7, 100=9, 101=10, 110=11
 * bits 5-6: SB DMA: 00=disabled (when SB disabled), 01=DMA0, 10=DMA1, 11=DMA3
 * bit 7:    SB port: 0=0x220, 1=0x240
 */
const CFG2: u8 = 0x62;
const CFG2_MPU_ENABLE: u8 = 1 << 2;
/*
 * bits 0-1: CD-ROM mode: 00=disabled, 01=Panasonic, 10=Sony/Mitsumi/Wearnes,
 *			  11=IDE
 * bit 2:    MPU401: 0=disabled, 1=enabled
 * bits 3-4: MPU401 IRQ: 00=3, 01=5, 10=7, 11=9,
 * bits 5-7: MPU401 port: 000=0x300, 001=0x310, 010=0x320, 011=0x330, 100=0x332,
 *			  101=0x334, 110=0x336
 */
const CFG3: u8 = 0x63;
/*
 * bits 0-2: CD-ROM IRQ: 000=disabled, 001=3, 010=5, 011=7, 100=9, 101=10,
 *			 110=11
 * bits 3-4: CD-ROM DMA: 00=disabled, 01=DMA0, 10=DMA1, 11=DMA3
 * bits 5-7: CD-ROM port: 000=0x300, 001=0x310, 010=0x320, 011=0x330, 100=0x340,
 *			  101=0x350, 110=0x360, 111=0x370
 */

unsafe fn snd_cmi8328_cfg_read(port: u16, reg: u8) -> u8 {
    unsafe {
        outb(0x43, port.wrapping_add(3));
        outb(0x21, port.wrapping_add(3));
        outb(reg, port.wrapping_add(3));
        inb(port)
    }
}

unsafe fn snd_cmi8328_cfg_write(port: u16, reg: u8, val: u8) {
    unsafe {
        outb(0x43, port.wrapping_add(3));
        outb(0x21, port.wrapping_add(3));
        outb(reg, port.wrapping_add(3));
        outb(val, port.wrapping_add(3)); /* yes, value goes to the same port as index */
    }
}

#[cfg(CONFIG_PM)]
unsafe fn snd_cmi8328_cfg_save(port: u16, cfg: *mut u8) {
    unsafe {
        *cfg.add(0) = snd_cmi8328_cfg_read(port, CFG1);
        *cfg.add(1) = snd_cmi8328_cfg_read(port, CFG2);
        *cfg.add(2) = snd_cmi8328_cfg_read(port, CFG3);
    }
}

#[cfg(CONFIG_PM)]
unsafe fn snd_cmi8328_cfg_restore(port: u16, cfg: *mut u8) {
    unsafe {
        snd_cmi8328_cfg_write(port, CFG1, *cfg.add(0));
        snd_cmi8328_cfg_write(port, CFG2, *cfg.add(1));
        snd_cmi8328_cfg_write(port, CFG3, *cfg.add(2));
    }
}

unsafe fn snd_cmi8328_mixer(chip: *mut snd_wss) -> c_int {
    unsafe {
        let mut card: *mut snd_card;
        let mut id1: snd_ctl_elem_id = mem::zeroed();
        let mut id2: snd_ctl_elem_id = mem::zeroed();
        let mut err: c_int;

        card = (*chip).card;

        memset(
            &mut id1 as *mut _ as *mut c_void,
            0,
            mem::size_of::<snd_ctl_elem_id>(),
        );
        memset(
            &mut id2 as *mut _ as *mut c_void,
            0,
            mem::size_of::<snd_ctl_elem_id>(),
        );
        id2.iface = SNDRV_CTL_ELEM_IFACE_MIXER;
        id1.iface = id2.iface;
        /* rename AUX0 switch to CD */
        strscpy(id1.name.as_mut_ptr(), c"Aux Playback Switch".as_ptr());
        strscpy(id2.name.as_mut_ptr(), c"CD Playback Switch".as_ptr());
        err = snd_ctl_rename_id(card, &mut id1, &mut id2);
        if err < 0 {
            dev_err((*card).dev, c"error renaming control\n".as_ptr());
            return err;
        }
        /* rename AUX0 volume to CD */
        strscpy(id1.name.as_mut_ptr(), c"Aux Playback Volume".as_ptr());
        strscpy(id2.name.as_mut_ptr(), c"CD Playback Volume".as_ptr());
        err = snd_ctl_rename_id(card, &mut id1, &mut id2);
        if err < 0 {
            dev_err((*card).dev, c"error renaming control\n".as_ptr());
            return err;
        }
        /* rename AUX1 switch to Synth */
        strscpy(id1.name.as_mut_ptr(), c"Aux Playback Switch".as_ptr());
        id1.index = 1;
        strscpy(id2.name.as_mut_ptr(), c"Synth Playback Switch".as_ptr());
        err = snd_ctl_rename_id(card, &mut id1, &mut id2);
        if err < 0 {
            dev_err((*card).dev, c"error renaming control\n".as_ptr());
            return err;
        }
        /* rename AUX1 volume to Synth */
        strscpy(id1.name.as_mut_ptr(), c"Aux Playback Volume".as_ptr());
        id1.index = 1;
        strscpy(id2.name.as_mut_ptr(), c"Synth Playback Volume".as_ptr());
        err = snd_ctl_rename_id(card, &mut id1, &mut id2);
        if err < 0 {
            dev_err((*card).dev, c"error renaming control\n".as_ptr());
            return err;
        }

        0
    }
}

/* find index of an item in "-1"-ended array */
unsafe fn array_find(array: *const c_int, item: c_int) -> c_int {
    unsafe {
        let mut i: c_int = 0;

        while *array.offset(i as isize) != -1 {
            if *array.offset(i as isize) == item {
                return i;
            }
            i += 1;
        }

        -1
    }
}

/* the same for long */
unsafe fn array_find_l(array: *const c_long, item: c_long) -> c_int {
    unsafe {
        let mut i: c_int = 0;

        while *array.offset(i as isize) != -1 {
            if *array.offset(i as isize) == item {
                return i;
            }
            i += 1;
        }

        -1
    }
}

unsafe extern "C" fn snd_cmi8328_probe(pdev: *mut device, ndev: c_uint) -> c_int {
    unsafe {
        let mut card: *mut snd_card = ptr::null_mut();
        let mut opl3: *mut snd_opl3 = ptr::null_mut();
        let mut cmi: *mut snd_cmi8328;
        #[cfg(SUPPORT_JOYSTICK)]
        let mut res: *mut resource;
        let mut err: c_int;
        let mut pos: c_int;
        static mpu_ports: [c_long; 8] = [0x330, 0x300, 0x310, 0x320, 0x332, 0x334, 0x336, -1];
        static mpu_port_bits: [u8; 7] = [3, 0, 1, 2, 4, 5, 6];
        static mpu_irqs: [c_int; 5] = [9, 7, 5, 3, -1];
        static mpu_irq_bits: [u8; 4] = [3, 2, 1, 0];
        static irqs: [c_int; 5] = [9, 10, 11, 7, -1];
        static irq_bits: [u8; 4] = [2, 3, 4, 1];
        static dma1s: [c_int; 4] = [3, 1, 0, -1];
        static dma_bits: [u8; 3] = [3, 2, 1];
        static dma2s: [[c_int; 2]; 4] = [[1, -1], [0, -1], [-1, -1], [0, -1]];
        let port: u16 = cmi8328_ports[ndev as usize] as u16;
        let mut val: u8;

        /* 0xff is invalid configuration (but settable - hope it isn't set) */
        if snd_cmi8328_cfg_read(port, CFG1) == 0xff {
            return -ENODEV;
        }
        /* the SB disable bit must NEVER EVER be cleared or the WSS dies */
        snd_cmi8328_cfg_write(port, CFG1, CFG1_SB_DISABLE);
        if snd_cmi8328_cfg_read(port, CFG1) != CFG1_SB_DISABLE {
            return -ENODEV;
        }
        /* disable everything first */
        snd_cmi8328_cfg_write(port, CFG2, 0); /* disable CDROM and MPU401 */
        snd_cmi8328_cfg_write(port, CFG3, 0); /* disable CDROM IRQ and DMA */

        if irq[ndev as usize] == SNDRV_AUTO_IRQ {
            irq[ndev as usize] = snd_legacy_find_free_irq(irqs.as_ptr());
            if irq[ndev as usize] < 0 {
                dev_err(pdev, c"unable to find a free IRQ\n".as_ptr());
                return -EBUSY;
            }
        }
        if dma1[ndev as usize] == SNDRV_AUTO_DMA {
            dma1[ndev as usize] = snd_legacy_find_free_dma(dma1s.as_ptr());
            if dma1[ndev as usize] < 0 {
                dev_err(pdev, c"unable to find a free DMA1\n".as_ptr());
                return -EBUSY;
            }
        }
        if dma2[ndev as usize] == SNDRV_AUTO_DMA {
            dma2[ndev as usize] =
                snd_legacy_find_free_dma(dma2s[(dma1[ndev as usize] % 4) as usize].as_ptr());
            if dma2[ndev as usize] < 0 {
                dev_warn(
                    pdev,
                    c"unable to find a free DMA2, full-duplex will not work\n".as_ptr(),
                );
                dma2[ndev as usize] = -1;
            }
        }
        /* configure WSS IRQ... */
        pos = array_find(irqs.as_ptr(), irq[ndev as usize]);
        if pos < 0 {
            dev_err(pdev, c"invalid IRQ %d\n".as_ptr(), irq[ndev as usize]);
            return -EINVAL;
        }
        val = irq_bits[pos as usize] << 3;
        /* ...and DMA... */
        pos = array_find(dma1s.as_ptr(), dma1[ndev as usize]);
        if pos < 0 {
            dev_err(pdev, c"invalid DMA1 %d\n".as_ptr(), dma1[ndev as usize]);
            return -EINVAL;
        }
        val |= dma_bits[pos as usize];
        /* ...and DMA2 */
        if dma2[ndev as usize] >= 0 && dma1[ndev as usize] != dma2[ndev as usize] {
            pos = array_find(dma2s[dma1[ndev as usize] as usize].as_ptr(), dma2[ndev as usize]);
            if pos < 0 {
                dev_err(pdev, c"invalid DMA2 %d\n".as_ptr(), dma2[ndev as usize]);
                return -EINVAL;
            }
            val |= 0x04; /* enable separate capture DMA */
        }
        outb(val, port);

        err = snd_devm_card_new(
            pdev,
            index[ndev as usize],
            id[ndev as usize],
            THIS_MODULE,
            mem::size_of::<snd_cmi8328>(),
            &mut card,
        );
        if err < 0 {
            return err;
        }
        cmi = (*card).private_data as *mut snd_cmi8328;
        (*cmi).card = card;
        (*cmi).port = port;
        (*cmi).wss_cfg = val;

        err = snd_wss_create(
            card,
            port.wrapping_add(4),
            -1,
            irq[ndev as usize],
            dma1[ndev as usize],
            dma2[ndev as usize],
            WSS_HW_DETECT,
            0,
            &mut (*cmi).wss,
        );
        if err < 0 {
            return err;
        }

        err = snd_wss_pcm((*cmi).wss, 0);
        if err < 0 {
            return err;
        }

        err = snd_wss_mixer((*cmi).wss);
        if err < 0 {
            return err;
        }
        err = snd_cmi8328_mixer((*cmi).wss);
        if err < 0 {
            return err;
        }

        if snd_wss_timer((*cmi).wss, 0) < 0 {
            dev_warn(pdev, c"error initializing WSS timer\n".as_ptr());
        }

        if mpuport[ndev as usize] == SNDRV_AUTO_PORT {
            mpuport[ndev as usize] = snd_legacy_find_free_ioport(mpu_ports.as_ptr(), 2);
            if mpuport[ndev as usize] < 0 {
                dev_err(pdev, c"unable to find a free MPU401 port\n".as_ptr());
            }
        }
        if mpuirq[ndev as usize] == SNDRV_AUTO_IRQ {
            mpuirq[ndev as usize] = snd_legacy_find_free_irq(mpu_irqs.as_ptr());
            if mpuirq[ndev as usize] < 0 {
                dev_err(pdev, c"unable to find a free MPU401 IRQ\n".as_ptr());
            }
        }
        /* enable and configure MPU401 */
        if mpuport[ndev as usize] > 0 && mpuirq[ndev as usize] > 0 {
            val = CFG2_MPU_ENABLE;
            pos = array_find_l(mpu_ports.as_ptr(), mpuport[ndev as usize]);
            if pos < 0 {
                dev_warn(
                    pdev,
                    c"invalid MPU401 port 0x%lx\n".as_ptr(),
                    mpuport[ndev as usize],
                );
            } else {
                val |= mpu_port_bits[pos as usize] << 5;
                pos = array_find(mpu_irqs.as_ptr(), mpuirq[ndev as usize]);
                if pos < 0 {
                    dev_warn(
                        pdev,
                        c"invalid MPU401 IRQ %d\n".as_ptr(),
                        mpuirq[ndev as usize],
                    );
                } else {
                    val |= mpu_irq_bits[pos as usize] << 3;
                    snd_cmi8328_cfg_write(port, CFG2, val);
                    if snd_mpu401_uart_new(
                        card,
                        0,
                        MPU401_HW_MPU401,
                        mpuport[ndev as usize],
                        0,
                        mpuirq[ndev as usize],
                        NULL,
                    ) < 0
                    {
                        dev_err(pdev, c"error initializing MPU401\n".as_ptr());
                    }
                }
            }
        }
        /* OPL3 is hardwired to 0x388 and cannot be disabled */
        if snd_opl3_create(card, 0x388, 0x38a, OPL3_HW_AUTO, 0, &mut opl3) < 0 {
            dev_err(pdev, c"error initializing OPL3\n".as_ptr());
        } else if snd_opl3_hwdep_new(opl3, 0, 1, NULL) < 0 {
            dev_warn(pdev, c"error initializing OPL3 hwdep\n".as_ptr());
        }

        strscpy((*card).driver.as_mut_ptr(), c"CMI8328".as_ptr());
        strscpy((*card).shortname.as_mut_ptr(), c"C-Media CMI8328".as_ptr());
        sprintf(
            (*card).longname.as_mut_ptr(),
            c"%s at 0x%lx, irq %d, dma %d,%d".as_ptr(),
            (*card).shortname.as_ptr(),
            (*(*cmi).wss).port,
            irq[ndev as usize],
            dma1[ndev as usize],
            if dma2[ndev as usize] >= 0 {
                dma2[ndev as usize]
            } else {
                dma1[ndev as usize]
            },
        );

        dev_set_drvdata(pdev, card as *mut c_void);
        err = snd_card_register(card);
        if err < 0 {
            return err;
        }
        #[cfg(SUPPORT_JOYSTICK)]
        {
            if !gameport[ndev as usize] {
                return 0;
            }
            /* gameport is hardwired to 0x200 */
            res = devm_request_region(pdev, 0x200, 8, c"CMI8328 gameport".as_ptr());
            if res.is_null() {
                dev_warn(pdev, c"unable to allocate gameport I/O port\n".as_ptr());
            } else {
                let gp: *mut gameport = {
                    (*cmi).gameport = gameport_allocate_port();
                    (*cmi).gameport
                };
                if !(*cmi).gameport.is_null() {
                    gameport_set_name(gp, c"CMI8328 Gameport".as_ptr());
                    gameport_set_phys(gp, c"%s/gameport0".as_ptr(), dev_name(pdev));
                    gameport_set_dev_parent(gp, pdev);
                    (*gp).io = 0x200;
                    /* Enable gameport */
                    snd_cmi8328_cfg_write(port, CFG1, CFG1_SB_DISABLE | CFG1_GAMEPORT);
                    gameport_register_port(gp);
                }
            }
        }
        0
    }
}

unsafe extern "C" fn snd_cmi8328_remove(pdev: *mut device, _dev: c_uint) {
    unsafe {
        let card: *mut snd_card = dev_get_drvdata(pdev) as *mut snd_card;
        let cmi: *mut snd_cmi8328 = (*card).private_data as *mut snd_cmi8328;

        #[cfg(SUPPORT_JOYSTICK)]
        {
            if !(*cmi).gameport.is_null() {
                gameport_unregister_port((*cmi).gameport);
            }
        }
        /* disable everything */
        snd_cmi8328_cfg_write((*cmi).port, CFG1, CFG1_SB_DISABLE);
        snd_cmi8328_cfg_write((*cmi).port, CFG2, 0);
        snd_cmi8328_cfg_write((*cmi).port, CFG3, 0);
    }
}

#[cfg(CONFIG_PM)]
unsafe extern "C" fn snd_cmi8328_suspend(
    pdev: *mut device,
    _n: c_uint,
    _state: pm_message_t,
) -> c_int {
    unsafe {
        let card: *mut snd_card = dev_get_drvdata(pdev) as *mut snd_card;
        let mut cmi: *mut snd_cmi8328;

        if card.is_null() {
            /* ignore absent devices */
            return 0;
        }
        cmi = (*card).private_data as *mut snd_cmi8328;
        snd_cmi8328_cfg_save((*cmi).port, (*cmi).cfg.as_mut_ptr());
        snd_power_change_state(card, SNDRV_CTL_POWER_D3hot);
        ((*(*cmi).wss).suspend.unwrap())((*cmi).wss);

        0
    }
}

#[cfg(CONFIG_PM)]
unsafe extern "C" fn snd_cmi8328_resume(pdev: *mut device, _n: c_uint) -> c_int {
    unsafe {
        let card: *mut snd_card = dev_get_drvdata(pdev) as *mut snd_card;
        let mut cmi: *mut snd_cmi8328;

        if card.is_null() {
            /* ignore absent devices */
            return 0;
        }
        cmi = (*card).private_data as *mut snd_cmi8328;
        snd_cmi8328_cfg_restore((*cmi).port, (*cmi).cfg.as_mut_ptr());
        outb((*cmi).wss_cfg, (*cmi).port);
        ((*(*cmi).wss).resume.unwrap())((*cmi).wss);
        snd_power_change_state(card, SNDRV_CTL_POWER_D0);

        0
    }
}

static mut snd_cmi8328_driver: isa_driver = isa_driver {
    probe: Some(snd_cmi8328_probe),
    remove: Some(snd_cmi8328_remove),
    #[cfg(CONFIG_PM)]
    suspend: Some(snd_cmi8328_suspend),
    #[cfg(CONFIG_PM)]
    resume: Some(snd_cmi8328_resume),
    driver: device_driver {
        name: c"cmi8328".as_ptr(),
    },
};

// Original registration:
// module_isa_driver(snd_cmi8328_driver, CMI8328_MAX);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
