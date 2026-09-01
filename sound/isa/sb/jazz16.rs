// SPDX-License-Identifier: GPL-2.0
/*
 * jazz16.c - driver for Media Vision Jazz16 based soundcards.
 * Copyright (C) 2009 Krzysztof Helt <krzysztof.h1@wp.pl>
 * Based on patches posted by Rask Ingemann Lambertsen and Rene Herman.
 * Based on OSS Sound Blaster driver.
 */

// C includes translated as dependency intent:
// linux/init.h, linux/module.h, linux/io.h, linux/delay.h, linux/string.h,
// asm/dma.h, linux/isa.h, sound/core.h, sound/mpu401.h, sound/opl3.h,
// sound/sb.h, sound/initval.h.
// #define SNDRV_LEGACY_FIND_FREE_IRQ
// #define SNDRV_LEGACY_FIND_FREE_DMA

const PFX: &[u8; 9] = b"jazz16: ";

// MODULE_DESCRIPTION("Media Vision Jazz16");
// MODULE_AUTHOR("Krzysztof Helt <krzysztof.h1@wp.pl>");
// MODULE_LICENSE("GPL");

type c_char = i8;
type c_int = i32;
type c_uint = u32;
type c_ulong = u64;
type c_void = core::ffi::c_void;
type bool_ = bool;
type irqreturn_t = c_int;
type pm_message_t = c_int;

const SNDRV_CARDS: usize = 8;
const SNDRV_DEFAULT_IDX: [c_int; SNDRV_CARDS] = [0; SNDRV_CARDS];
const SNDRV_DEFAULT_STR: [*mut c_char; SNDRV_CARDS] = [core::ptr::null_mut(); SNDRV_CARDS];
const SNDRV_DEFAULT_ENABLE: [bool_; SNDRV_CARDS] = [false; SNDRV_CARDS];
const SNDRV_DEFAULT_PORT: [c_ulong; SNDRV_CARDS] = [0; SNDRV_CARDS];
const SNDRV_DEFAULT_IRQ: [c_int; SNDRV_CARDS] = [0; SNDRV_CARDS];
const SNDRV_DEFAULT_DMA: [c_int; SNDRV_CARDS] = [0; SNDRV_CARDS];

static mut index: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IDX; /* Index 0-MAX */
static mut id: [*mut c_char; SNDRV_CARDS] = SNDRV_DEFAULT_STR; /* ID for this card */
static mut enable: [bool_; SNDRV_CARDS] = SNDRV_DEFAULT_ENABLE; /* Enable this card */
static mut port: [c_ulong; SNDRV_CARDS] = SNDRV_DEFAULT_PORT;
static mut mpu_port: [c_ulong; SNDRV_CARDS] = SNDRV_DEFAULT_PORT;
static mut irq: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IRQ;
static mut mpu_irq: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IRQ;
static mut dma8: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_DMA;
static mut dma16: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_DMA;

// module_param_array(index, int, NULL, 0444);
// MODULE_PARM_DESC(index, "Index value for Media Vision Jazz16 based soundcard.");
// module_param_array(id, charp, NULL, 0444);
// MODULE_PARM_DESC(id, "ID string for Media Vision Jazz16 based soundcard.");
// module_param_array(enable, bool, NULL, 0444);
// MODULE_PARM_DESC(enable, "Enable Media Vision Jazz16 based soundcard.");
// module_param_hw_array(port, long, ioport, NULL, 0444);
// MODULE_PARM_DESC(port, "Port # for jazz16 driver.");
// module_param_hw_array(mpu_port, long, ioport, NULL, 0444);
// MODULE_PARM_DESC(mpu_port, "MPU-401 port # for jazz16 driver.");
// module_param_hw_array(irq, int, irq, NULL, 0444);
// MODULE_PARM_DESC(irq, "IRQ # for jazz16 driver.");
// module_param_hw_array(mpu_irq, int, irq, NULL, 0444);
// MODULE_PARM_DESC(mpu_irq, "MPU-401 IRQ # for jazz16 driver.");
// module_param_hw_array(dma8, int, dma, NULL, 0444);
// MODULE_PARM_DESC(dma8, "DMA8 # for jazz16 driver.");
// module_param_hw_array(dma16, int, dma, NULL, 0444);
// MODULE_PARM_DESC(dma16, "DMA16 # for jazz16 driver.");

const SB_JAZZ16_WAKEUP: c_int = 0xaf;
const SB_JAZZ16_SET_PORTS: c_int = 0x50;
const SB_DSP_GET_JAZZ_BRD_REV: c_int = 0xfa;
const SB_JAZZ16_SET_DMAINTR: c_int = 0xfb;
const SB_DSP_GET_JAZZ_MODEL: c_int = 0xfe;

const EBUSY: c_int = 16;
const ENODEV: c_int = 19;
const EINVAL: c_int = 22;
const SNDRV_AUTO_PORT: c_ulong = !0;
const SNDRV_AUTO_IRQ: c_int = -1;
const SNDRV_AUTO_DMA: c_int = -1;
const SB_HW_JAZZ16: c_int = 0;
const OPL3_HW_AUTO: c_int = 0;
const MPU401_HW_MPU401: c_int = 0;
const SNDRV_CTL_POWER_D3HOT: c_int = 0;
const SNDRV_CTL_POWER_D0: c_int = 0;
const THIS_MODULE: *mut c_void = core::ptr::null_mut();

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_opl3 {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_sb {
    port: c_ulong,
    irq: c_int,
    dma8: c_int,
    dma16: c_int,
}

#[repr(C)]
pub struct snd_card {
    dev: *mut device,
    private_data: *mut c_void,
    driver: [c_char; 16],
    shortname: [c_char; 32],
    longname: [c_char; 80],
}

#[repr(C)]
pub struct isa_driver_driver {
    name: *const c_char,
}

#[repr(C)]
pub struct isa_driver {
    match_: Option<unsafe extern "C" fn(*mut device, c_uint) -> c_int>,
    probe: Option<unsafe extern "C" fn(*mut device, c_uint) -> c_int>,
    // CONFIG_PM fields are present when that kernel build option is enabled.
    suspend: Option<unsafe extern "C" fn(*mut device, c_uint, pm_message_t) -> c_int>,
    resume: Option<unsafe extern "C" fn(*mut device, c_uint) -> c_int>,
    driver: isa_driver_driver,
}

#[repr(C)]
struct snd_card_jazz16 {
    chip: *mut snd_sb,
}

extern "C" {
    fn snd_sb8dsp_interrupt(chip: *mut c_void) -> irqreturn_t;
    fn request_region(start: c_ulong, n: c_ulong, name: *const c_char) -> *mut c_void;
    fn release_region(start: c_ulong, n: c_ulong);
    fn outb(value: u8, port: c_ulong);
    fn udelay(usecs: c_ulong);
    fn snd_sbdsp_reset(chip: *mut snd_sb) -> c_int;
    fn snd_sbdsp_command(chip: *mut snd_sb, val: u8) -> c_int;
    fn snd_sbdsp_get_byte(chip: *mut snd_sb) -> c_int;
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
    fn snd_sbdsp_create(
        card: *mut snd_card,
        port: c_ulong,
        irq: c_int,
        irq_handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t,
        dma8: c_int,
        dma16: c_int,
        hardware: c_int,
        chip: *mut *mut snd_sb,
    ) -> c_int;
    fn strscpy(dest: *mut c_char, src: *const c_char);
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn snd_sb8dsp_pcm(chip: *mut snd_sb, device: c_int) -> c_int;
    fn snd_sbmixer_new(chip: *mut snd_sb) -> c_int;
    fn snd_opl3_create(
        card: *mut snd_card,
        l_port: c_ulong,
        r_port: c_ulong,
        hardware: c_int,
        integrated: c_int,
        ropl3: *mut *mut snd_opl3,
    ) -> c_int;
    fn snd_opl3_hwdep_new(opl3: *mut snd_opl3, device: c_int, seq_device: c_int, private_data: *mut c_void) -> c_int;
    fn snd_mpu401_uart_new(
        card: *mut snd_card,
        device: c_int,
        hardware: c_int,
        port: c_ulong,
        info_flags: c_int,
        irq: c_int,
        private_data: *mut c_void,
    ) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn snd_power_change_state(card: *mut snd_card, state: c_int);
    fn snd_sbmixer_suspend(chip: *mut snd_sb);
    fn snd_sbmixer_resume(chip: *mut snd_sb);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
}

unsafe extern "C" fn jazz16_interrupt(_irq: c_int, chip: *mut c_void) -> irqreturn_t {
    unsafe { snd_sb8dsp_interrupt(chip) }
}

unsafe fn jazz16_configure_ports(
    card: *mut snd_card,
    port_: c_ulong,
    mpu_port_: c_ulong,
    idx: c_int,
) -> c_int {
    let mut val: u8;

    if unsafe { request_region(0x201, 1, c"jazz16 config".as_ptr()) }.is_null() {
        unsafe { dev_err((*card).dev, c"config port region is already in use.\n".as_ptr()) };
        return -EBUSY;
    }
    unsafe { outb((SB_JAZZ16_WAKEUP - idx) as u8, 0x201) };
    unsafe { udelay(100) };
    unsafe { outb((SB_JAZZ16_SET_PORTS + idx) as u8, 0x201) };
    unsafe { udelay(100) };
    val = (port_ & 0x70) as u8;
    val |= ((mpu_port_ & 0x30) >> 4) as u8;
    unsafe { outb(val, 0x201) };

    unsafe { release_region(0x201, 1) };
    0
}

unsafe fn jazz16_detect_board(card: *mut snd_card, port_: c_ulong, mpu_port_: c_ulong) -> c_int {
    let mut err: c_int;
    let mut val: c_int;
    let mut chip = snd_sb {
        port: 0,
        irq: 0,
        dma8: 0,
        dma16: 0,
    };

    if unsafe { request_region(port_, 0x10, c"jazz16".as_ptr()) }.is_null() {
        unsafe { dev_err((*card).dev, c"I/O port region is already in use.\n".as_ptr()) };
        return -EBUSY;
    }
    /* just to call snd_sbdsp_command/reset/get_byte() */
    chip.port = port_;

    err = unsafe { snd_sbdsp_reset(&mut chip) };
    if err < 0 {
        val = 0;
        while val < 4 {
            err = unsafe { jazz16_configure_ports(card, port_, mpu_port_, val) };
            if err < 0 {
                break;
            }

            err = unsafe { snd_sbdsp_reset(&mut chip) };
            if err == 0 {
                break;
            }
            val += 1;
        }
    }
    if err < 0 {
        err = -ENODEV;
        unsafe { release_region(port_, 0x10) };
        return err;
    }
    if unsafe { snd_sbdsp_command(&mut chip, SB_DSP_GET_JAZZ_BRD_REV as u8) } == 0 {
        err = -EBUSY;
        unsafe { release_region(port_, 0x10) };
        return err;
    }
    val = unsafe { snd_sbdsp_get_byte(&mut chip) };
    if val >= 0x30 {
        unsafe { snd_sbdsp_get_byte(&mut chip) };
    }

    if (val & 0xf0) != 0x10 {
        err = -ENODEV;
        unsafe { release_region(port_, 0x10) };
        return err;
    }
    if unsafe { snd_sbdsp_command(&mut chip, SB_DSP_GET_JAZZ_MODEL as u8) } == 0 {
        err = -EBUSY;
        unsafe { release_region(port_, 0x10) };
        return err;
    }
    unsafe { snd_sbdsp_get_byte(&mut chip) };
    err = unsafe { snd_sbdsp_get_byte(&mut chip) };
    unsafe {
        dev_dbg(
            (*card).dev,
            c"Media Vision Jazz16 board detected: rev 0x%x, model 0x%x\n".as_ptr(),
            val,
            err,
        )
    };

    err = 0;

    unsafe { release_region(port_, 0x10) };
    err
}

unsafe fn jazz16_configure_board(chip: *mut snd_sb, mpu_irq_: c_int) -> c_int {
    static JAZZ_IRQ_BITS: [u8; 16] = [0, 0, 2, 3, 0, 1, 0, 4, 0, 2, 5, 0, 0, 0, 0, 6];
    static JAZZ_DMA_BITS: [u8; 8] = [0, 1, 0, 2, 0, 3, 0, 4];

    if JAZZ_DMA_BITS[unsafe { (*chip).dma8 } as usize] == 0
        || JAZZ_DMA_BITS[unsafe { (*chip).dma16 } as usize] == 0
        || JAZZ_IRQ_BITS[unsafe { (*chip).irq } as usize] == 0
    {
        return -EINVAL;
    }

    if unsafe { snd_sbdsp_command(chip, SB_JAZZ16_SET_DMAINTR as u8) } == 0 {
        return -EBUSY;
    }

    if unsafe {
        snd_sbdsp_command(
            chip,
            (JAZZ_DMA_BITS[(*chip).dma8 as usize] | (JAZZ_DMA_BITS[(*chip).dma16 as usize] << 4))
                as u8,
        )
    } == 0
    {
        return -EBUSY;
    }

    if unsafe {
        snd_sbdsp_command(
            chip,
            (JAZZ_IRQ_BITS[(*chip).irq as usize] | (JAZZ_IRQ_BITS[mpu_irq_ as usize] << 4)) as u8,
        )
    } == 0
    {
        return -EBUSY;
    }

    0
}

unsafe extern "C" fn snd_jazz16_match(devptr: *mut device, dev: c_uint) -> c_int {
    let dev = dev as usize;

    if unsafe { !enable[dev] } {
        return 0;
    }
    if unsafe { port[dev] } == SNDRV_AUTO_PORT {
        unsafe { dev_err(devptr, c"please specify port\n".as_ptr()) };
        return 0;
    } else if unsafe { port[dev] } == 0x200 || (unsafe { port[dev] } & !0x270) != 0 {
        unsafe { dev_err(devptr, c"incorrect port specified\n".as_ptr()) };
        return 0;
    }
    if unsafe { dma8[dev] } != SNDRV_AUTO_DMA && unsafe { dma8[dev] } != 1 && unsafe { dma8[dev] } != 3 {
        unsafe { dev_err(devptr, c"dma8 must be 1 or 3\n".as_ptr()) };
        return 0;
    }
    if unsafe { dma16[dev] } != SNDRV_AUTO_DMA && unsafe { dma16[dev] } != 5 && unsafe { dma16[dev] } != 7 {
        unsafe { dev_err(devptr, c"dma16 must be 5 or 7\n".as_ptr()) };
        return 0;
    }
    if unsafe { mpu_port[dev] } != SNDRV_AUTO_PORT && (unsafe { mpu_port[dev] } & !0x030) != 0x300 {
        unsafe { dev_err(devptr, c"incorrect mpu_port specified\n".as_ptr()) };
        return 0;
    }
    if unsafe { mpu_irq[dev] } != SNDRV_AUTO_DMA
        && unsafe { mpu_irq[dev] } != 2
        && unsafe { mpu_irq[dev] } != 3
        && unsafe { mpu_irq[dev] } != 5
        && unsafe { mpu_irq[dev] } != 7
    {
        unsafe { dev_err(devptr, c"mpu_irq must be 2, 3, 5 or 7\n".as_ptr()) };
        return 0;
    }
    1
}

unsafe extern "C" fn snd_jazz16_probe(devptr: *mut device, dev: c_uint) -> c_int {
    let mut card: *mut snd_card = core::ptr::null_mut();
    let jazz16: *mut snd_card_jazz16;
    let mut chip: *mut snd_sb = core::ptr::null_mut();
    let mut opl3: *mut snd_opl3 = core::ptr::null_mut();
    static POSSIBLE_IRQS: [c_int; 8] = [2, 3, 5, 7, 9, 10, 15, -1];
    static POSSIBLE_DMAS8: [c_int; 3] = [1, 3, -1];
    static POSSIBLE_DMAS16: [c_int; 3] = [5, 7, -1];
    let mut err: c_int;
    let mut xirq: c_int;
    let mut xdma8: c_int;
    let mut xdma16: c_int;
    let mut xmpu_port: c_int;
    let mut xmpu_irq: c_int;
    let dev = dev as usize;

    err = unsafe {
        snd_devm_card_new(
            devptr,
            index[dev],
            id[dev],
            THIS_MODULE,
            core::mem::size_of::<snd_card_jazz16>(),
            &mut card,
        )
    };
    if err < 0 {
        return err;
    }

    jazz16 = unsafe { (*card).private_data as *mut snd_card_jazz16 };

    xirq = unsafe { irq[dev] };
    if xirq == SNDRV_AUTO_IRQ {
        xirq = unsafe { snd_legacy_find_free_irq(POSSIBLE_IRQS.as_ptr()) };
        if xirq < 0 {
            unsafe { dev_err(devptr, c"unable to find a free IRQ\n".as_ptr()) };
            return -EBUSY;
        }
    }
    xdma8 = unsafe { dma8[dev] };
    if xdma8 == SNDRV_AUTO_DMA {
        xdma8 = unsafe { snd_legacy_find_free_dma(POSSIBLE_DMAS8.as_ptr()) };
        if xdma8 < 0 {
            unsafe { dev_err(devptr, c"unable to find a free DMA8\n".as_ptr()) };
            return -EBUSY;
        }
    }
    xdma16 = unsafe { dma16[dev] };
    if xdma16 == SNDRV_AUTO_DMA {
        xdma16 = unsafe { snd_legacy_find_free_dma(POSSIBLE_DMAS16.as_ptr()) };
        if xdma16 < 0 {
            unsafe { dev_err(devptr, c"unable to find a free DMA16\n".as_ptr()) };
            return -EBUSY;
        }
    }

    xmpu_port = unsafe { mpu_port[dev] as c_int };
    if xmpu_port as c_ulong == SNDRV_AUTO_PORT {
        xmpu_port = 0;
    }
    err = unsafe { jazz16_detect_board(card, port[dev], xmpu_port as c_ulong) };
    if err < 0 {
        unsafe { dev_err(devptr, c"Media Vision Jazz16 board not detected\n".as_ptr()) };
        return err;
    }
    err = unsafe {
        snd_sbdsp_create(
            card,
            port[dev],
            irq[dev],
            jazz16_interrupt,
            dma8[dev],
            dma16[dev],
            SB_HW_JAZZ16,
            &mut chip,
        )
    };
    if err < 0 {
        return err;
    }

    xmpu_irq = unsafe { mpu_irq[dev] };
    if xmpu_irq == SNDRV_AUTO_IRQ || unsafe { mpu_port[dev] } == SNDRV_AUTO_PORT {
        xmpu_irq = 0;
    }
    err = unsafe { jazz16_configure_board(chip, xmpu_irq) };
    if err < 0 {
        unsafe { dev_err(devptr, c"Media Vision Jazz16 configuration failed\n".as_ptr()) };
        return err;
    }

    unsafe { (*jazz16).chip = chip };

    unsafe { strscpy((*card).driver.as_mut_ptr(), c"jazz16".as_ptr()) };
    unsafe { strscpy((*card).shortname.as_mut_ptr(), c"Media Vision Jazz16".as_ptr()) };
    unsafe {
        sprintf(
            (*card).longname.as_mut_ptr(),
            c"Media Vision Jazz16 at 0x%lx, irq %d, dma8 %d, dma16 %d".as_ptr(),
            port[dev],
            xirq,
            xdma8,
            xdma16,
        )
    };

    err = unsafe { snd_sb8dsp_pcm(chip, 0) };
    if err < 0 {
        return err;
    }
    err = unsafe { snd_sbmixer_new(chip) };
    if err < 0 {
        return err;
    }

    err = unsafe { snd_opl3_create(card, (*chip).port, (*chip).port + 2, OPL3_HW_AUTO, 1, &mut opl3) };
    if err < 0 {
        unsafe {
            dev_warn(
                devptr,
                c"no OPL device at 0x%lx-0x%lx\n".as_ptr(),
                (*chip).port,
                (*chip).port + 2,
            )
        };
    } else {
        err = unsafe { snd_opl3_hwdep_new(opl3, 0, 1, core::ptr::null_mut()) };
        if err < 0 {
            return err;
        }
    }
    if unsafe { mpu_port[dev] } > 0 && unsafe { mpu_port[dev] } != SNDRV_AUTO_PORT {
        if unsafe { mpu_irq[dev] } == SNDRV_AUTO_IRQ {
            unsafe { mpu_irq[dev] = -1 };
        }

        if unsafe {
            snd_mpu401_uart_new(
                card,
                0,
                MPU401_HW_MPU401,
                mpu_port[dev],
                0,
                mpu_irq[dev],
                core::ptr::null_mut(),
            )
        } < 0
        {
            unsafe { dev_err(devptr, c"no MPU-401 device at 0x%lx\n".as_ptr(), mpu_port[dev]) };
        }
    }

    err = unsafe { snd_card_register(card) };
    if err < 0 {
        return err;
    }

    unsafe { dev_set_drvdata(devptr, card as *mut c_void) };
    0
}

// #ifdef CONFIG_PM
unsafe extern "C" fn snd_jazz16_suspend(pdev: *mut device, _n: c_uint, _state: pm_message_t) -> c_int {
    let card = unsafe { dev_get_drvdata(pdev) as *mut snd_card };
    let acard = unsafe { (*card).private_data as *mut snd_card_jazz16 };
    let chip = unsafe { (*acard).chip };

    unsafe { snd_power_change_state(card, SNDRV_CTL_POWER_D3HOT) };
    unsafe { snd_sbmixer_suspend(chip) };
    0
}

unsafe extern "C" fn snd_jazz16_resume(pdev: *mut device, _n: c_uint) -> c_int {
    let card = unsafe { dev_get_drvdata(pdev) as *mut snd_card };
    let acard = unsafe { (*card).private_data as *mut snd_card_jazz16 };
    let chip = unsafe { (*acard).chip };

    unsafe { snd_sbdsp_reset(chip) };
    unsafe { snd_sbmixer_resume(chip) };
    unsafe { snd_power_change_state(card, SNDRV_CTL_POWER_D0) };
    0
}
// #endif

static mut snd_jazz16_driver: isa_driver = isa_driver {
    match_: Some(snd_jazz16_match),
    probe: Some(snd_jazz16_probe),
    // #ifdef CONFIG_PM
    suspend: Some(snd_jazz16_suspend),
    resume: Some(snd_jazz16_resume),
    // #endif
    driver: isa_driver_driver {
        name: c"jazz16".as_ptr(),
    },
};

// module_isa_driver(snd_jazz16_driver, SNDRV_CARDS);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
