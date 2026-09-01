// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Driver for generic CS4232/CS4235/CS4236/CS4236B/CS4237B/CS4238B/CS4239 chips
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 */

// Includes in the original C source:
// linux/init.h, linux/err.h, linux/isa.h, linux/pnp.h, linux/module.h,
// sound/core.h, sound/wss.h, sound/mpu401.h, sound/opl3.h, sound/initval.h

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::size_of_val;
use core::ptr;

const IDENT: &[u8] = b"CS4232+\0";
const DEV_NAME: &[u8] = b"cs4232+\0";
const CS423X_ISAPNP_DRIVER: &[u8] = b"cs4232_isapnp\0";

type bool_ = bool;
type resource_size_t = usize;
type pm_message_t = c_uint;

const SNDRV_CARDS: usize = 8;
const SNDRV_DEFAULT_IDX: [c_int; SNDRV_CARDS] = [0; SNDRV_CARDS];
const SNDRV_DEFAULT_STR: [*mut c_char; SNDRV_CARDS] = [ptr::null_mut(); SNDRV_CARDS];
const SNDRV_DEFAULT_ENABLE_ISAPNP: [bool_; SNDRV_CARDS] = [true; SNDRV_CARDS];
const SNDRV_DEFAULT_PORT: [c_long; SNDRV_CARDS] = [-1; SNDRV_CARDS];
const SNDRV_DEFAULT_IRQ: [c_int; SNDRV_CARDS] = [-1; SNDRV_CARDS];
const SNDRV_DEFAULT_DMA: [c_int; SNDRV_CARDS] = [-1; SNDRV_CARDS];
const SNDRV_AUTO_PORT: c_long = -1;
const SNDRV_AUTO_IRQ: c_int = -1;
const SNDRV_AUTO_DMA: c_int = -1;
const EBUSY: c_int = 16;
const ENOENT: c_int = 2;
const ENODEV: c_int = 19;
const WSS_HW_DETECT3: c_int = 0;
const WSS_HW_CS4236B_MASK: c_int = 0;
const OPL3_HW_OPL3_CS: c_int = 0;
const MPU401_HW_CS4232: c_int = 0;
const SNDRV_CTL_POWER_D3HOT: c_int = 0;
const SNDRV_CTL_POWER_D0: c_int = 0;
const PNP_ID_LEN: usize = 8;
const PNP_DRIVER_RES_DISABLE: c_uint = 0;

static mut index: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IDX; /* Index 0-MAX */
static mut id: [*mut c_char; SNDRV_CARDS] = SNDRV_DEFAULT_STR; /* ID for this card */
static mut enable: [bool_; SNDRV_CARDS] = SNDRV_DEFAULT_ENABLE_ISAPNP; /* Enable this card */

// CONFIG_PNP
static mut isapnp: [bool_; SNDRV_CARDS] = [true; SNDRV_CARDS];

static mut port: [c_long; SNDRV_CARDS] = SNDRV_DEFAULT_PORT; /* PnP setup */
static mut cport: [c_long; SNDRV_CARDS] = SNDRV_DEFAULT_PORT; /* PnP setup */
static mut mpu_port: [c_long; SNDRV_CARDS] = SNDRV_DEFAULT_PORT; /* PnP setup */
static mut fm_port: [c_long; SNDRV_CARDS] = SNDRV_DEFAULT_PORT; /* PnP setup */
static mut sb_port: [c_long; SNDRV_CARDS] = SNDRV_DEFAULT_PORT; /* PnP setup */
static mut irq: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IRQ; /* 5,7,9,11,12,15 */
static mut mpu_irq: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IRQ; /* 9,11,12,15 */
static mut dma1: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_DMA; /* 0,1,3,5,6,7 */
static mut dma2: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_DMA; /* 0,1,3,5,6,7 */

// Module metadata and module parameters from the C source are preserved as comments:
// MODULE_AUTHOR("Jaroslav Kysela <perex@perex.cz>");
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("Cirrus Logic CS4232-9");
// MODULE_ALIAS("snd_cs4232");
// module_param_array/module_param_hw_array declarations for index, id, enable,
// isapnp, port, cport, mpu_port, fm_port, sb_port, irq, mpu_irq, dma1, dma2.

// CONFIG_PNP
static mut isa_registered: c_int = 0;
static mut pnpc_registered: c_int = 0;
static mut pnp_registered: c_int = 0;

#[repr(C)]
struct snd_card_cs4236 {
    chip: *mut snd_wss,
    // CONFIG_PNP
    wss: *mut pnp_dev,
    ctrl: *mut pnp_dev,
    mpu: *mut pnp_dev,
}

#[repr(C)]
struct device {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_card {
    dev: *mut device,
    private_data: *mut c_void,
    driver: [c_char; 32],
    shortname: [c_char; 32],
    longname: [c_char; 80],
}

#[repr(C)]
struct snd_pcm {
    name: *const c_char,
}

#[repr(C)]
struct snd_wss {
    hardware: c_int,
    pcm: *mut snd_pcm,
    port: c_long,
    suspend: Option<unsafe extern "C" fn(*mut snd_wss)>,
    resume: Option<unsafe extern "C" fn(*mut snd_wss)>,
}

#[repr(C)]
struct snd_opl3 {
    _private: [u8; 0],
}

#[repr(C)]
struct pnp_id {
    id: [c_char; PNP_ID_LEN],
}

#[repr(C)]
struct pnp_protocol {
    devices: list_head,
}

#[repr(C)]
struct pnp_dev {
    dev: device,
    id: [pnp_id; 1],
    protocol: *mut pnp_protocol,
    protocol_list: list_head,
}

#[repr(C)]
struct pnp_card {
    dev: device,
}

#[repr(C)]
struct pnp_card_link {
    card: *mut pnp_card,
}

#[repr(C)]
struct pnp_device_id {
    id: [c_char; PNP_ID_LEN],
}

#[repr(C)]
struct pnp_card_device_id_dev {
    id: [c_char; PNP_ID_LEN],
}

#[repr(C)]
struct pnp_card_device_id {
    id: [c_char; PNP_ID_LEN],
    devs: [pnp_card_device_id_dev; 3],
}

#[repr(C)]
struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
struct isa_driver {
    match_: Option<unsafe extern "C" fn(*mut device, c_uint) -> c_int>,
    probe: Option<unsafe extern "C" fn(*mut device, c_uint) -> c_int>,
    suspend: Option<unsafe extern "C" fn(*mut device, c_uint, pm_message_t) -> c_int>,
    resume: Option<unsafe extern "C" fn(*mut device, c_uint) -> c_int>,
    driver: device_driver,
}

#[repr(C)]
struct device_driver {
    name: *const c_char,
}

#[repr(C)]
struct pnp_driver {
    name: *const c_char,
    id_table: *const pnp_device_id,
    probe: Option<unsafe extern "C" fn(*mut pnp_dev, *const pnp_device_id) -> c_int>,
    suspend: Option<unsafe extern "C" fn(*mut pnp_dev, pm_message_t) -> c_int>,
    resume: Option<unsafe extern "C" fn(*mut pnp_dev) -> c_int>,
}

#[repr(C)]
struct pnp_card_driver {
    flags: c_uint,
    name: *const c_char,
    id_table: *const pnp_card_device_id,
    probe: Option<unsafe extern "C" fn(*mut pnp_card_link, *const pnp_card_device_id) -> c_int>,
    suspend: Option<unsafe extern "C" fn(*mut pnp_card_link, pm_message_t) -> c_int>,
    resume: Option<unsafe extern "C" fn(*mut pnp_card_link) -> c_int>,
}

unsafe extern "C" {
    static THIS_MODULE: *mut c_void;

    fn pnp_activate_dev(pdev: *mut pnp_dev) -> c_int;
    fn pnp_port_start(pdev: *mut pnp_dev, idx: c_uint) -> c_long;
    fn pnp_irq(pdev: *mut pnp_dev, idx: c_uint) -> resource_size_t;
    fn pnp_irq_valid(pdev: *mut pnp_dev, idx: c_uint) -> c_int;
    fn pnp_dma(pdev: *mut pnp_dev, idx: c_uint) -> c_int;
    fn pnp_request_card_device(
        card: *mut pnp_card_link,
        id: *const c_char,
        from: *mut pnp_dev,
    ) -> *mut pnp_dev;
    fn pnp_device_is_isapnp(pdev: *mut pnp_dev) -> c_int;
    fn pnp_set_drvdata(pdev: *mut pnp_dev, data: *mut c_void);
    fn pnp_get_drvdata(pdev: *mut pnp_dev) -> *mut c_void;
    fn pnp_set_card_drvdata(pcard: *mut pnp_card_link, data: *mut c_void);
    fn pnp_get_card_drvdata(pcard: *mut pnp_card_link) -> *mut c_void;
    fn pnp_register_driver(driver: *mut pnp_driver) -> c_int;
    fn pnp_unregister_driver(driver: *mut pnp_driver);
    fn pnp_register_card_driver(driver: *mut pnp_card_driver) -> c_int;
    fn pnp_unregister_card_driver(driver: *mut pnp_card_driver);

    fn snd_devm_card_new(
        pdev: *mut device,
        idx: c_int,
        id: *mut c_char,
        module: *mut c_void,
        extra_size: usize,
        cardp: *mut *mut snd_card,
    ) -> c_int;
    fn snd_cs4236_create(
        card: *mut snd_card,
        port: c_long,
        cport: c_long,
        irq: c_int,
        dma1: c_int,
        dma2: c_int,
        hw: c_int,
        flags: c_int,
        rchip: *mut *mut snd_wss,
    ) -> c_int;
    fn snd_cs4236_pcm(chip: *mut snd_wss, device: c_int) -> c_int;
    fn snd_cs4236_mixer(chip: *mut snd_wss) -> c_int;
    fn snd_wss_pcm(chip: *mut snd_wss, device: c_int) -> c_int;
    fn snd_wss_mixer(chip: *mut snd_wss) -> c_int;
    fn snd_wss_timer(chip: *mut snd_wss, device: c_int) -> c_int;
    fn snd_opl3_create(
        card: *mut snd_card,
        l_port: c_long,
        r_port: c_long,
        hardware: c_int,
        integrated: c_int,
        ropl3: *mut *mut snd_opl3,
    ) -> c_int;
    fn snd_opl3_hwdep_new(
        opl3: *mut snd_opl3,
        device: c_int,
        seq_device: c_int,
        rhwdep: *mut c_void,
    ) -> c_int;
    fn snd_mpu401_uart_new(
        card: *mut snd_card,
        device: c_int,
        hardware: c_int,
        port: c_long,
        integrated: c_int,
        irq: c_int,
        rrawmidi: *mut c_void,
    ) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn snd_power_change_state(card: *mut snd_card, state: c_int) -> c_int;

    fn devm_request_region(
        dev: *mut device,
        start: c_long,
        n: c_long,
        name: *const c_char,
    ) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn isa_register_driver(driver: *mut isa_driver, ndev: c_uint) -> c_int;
    fn isa_unregister_driver(driver: *mut isa_driver);

    fn strscpy(dst: *mut c_char, src: *const c_char, count: usize) -> isize;
    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

const fn pnp_device_id(id: &[u8; 8]) -> pnp_device_id {
    pnp_device_id { id: *id as [u8; 8] as [c_char; 8] }
}

const fn pnp_card_dev(id: &[u8; 8]) -> pnp_card_device_id_dev {
    pnp_card_device_id_dev { id: *id as [u8; 8] as [c_char; 8] }
}

const fn pnp_card_id(
    id: &[u8; 8],
    d0: &[u8; 8],
    d1: &[u8; 8],
    d2: &[u8; 8],
) -> pnp_card_device_id {
    pnp_card_device_id {
        id: *id as [u8; 8] as [c_char; 8],
        devs: [pnp_card_dev(d0), pnp_card_dev(d1), pnp_card_dev(d2)],
    }
}

// CONFIG_PNP
/*
 * PNP BIOS
 */
static snd_cs423x_pnpbiosids: [pnp_device_id; 4] = [
    pnp_device_id(b"CSC0100\0"),
    pnp_device_id(b"CSC0000\0"),
    /* Guillemot Turtlebeach something appears to be cs4232 compatible
     * (untested) */
    pnp_device_id(b"GIM0100\0"),
    pnp_device_id(b"\0\0\0\0\0\0\0\0"),
];

static snd_cs423x_pnpids: [pnp_card_device_id; 47] = [
    /* Philips PCA70PS */
    pnp_card_id(b"CSC0d32\0", b"CSC0000\0", b"CSC0010\0", b"PNPb006\0"),
    /* TerraTec Maestro 32/96 (CS4232) */
    pnp_card_id(b"CSC1a32\0", b"CSC0000\0", b"CSC0010\0", b"CSC0003\0"),
    /* HP Omnibook 5500 onboard */
    pnp_card_id(b"CSC4232\0", b"CSC0000\0", b"CSC0002\0", b"CSC0003\0"),
    /* Unnamed CS4236 card (Made in Taiwan) */
    pnp_card_id(b"CSC4236\0", b"CSC0000\0", b"CSC0010\0", b"CSC0003\0"),
    /* Turtle Beach TBS-2000 (CS4232) */
    pnp_card_id(b"CSC7532\0", b"CSC0000\0", b"CSC0010\0", b"CSCb006\0"),
    /* Turtle Beach Tropez Plus (CS4232) */
    pnp_card_id(b"CSC7632\0", b"CSC0000\0", b"CSC0010\0", b"PNPb006\0"),
    /* SIC CrystalWave 32 (CS4232) */
    pnp_card_id(b"CSCf032\0", b"CSC0000\0", b"CSC0010\0", b"CSC0003\0"),
    /* Netfinity 3000 on-board soundcard */
    pnp_card_id(b"CSCe825\0", b"CSC0100\0", b"CSC0110\0", b"CSC010f\0"),
    /* Intel Marlin Spike Motherboard - CS4235 */
    pnp_card_id(b"CSC0225\0", b"CSC0000\0", b"CSC0010\0", b"CSC0003\0"),
    /* Intel Marlin Spike Motherboard (#2) - CS4235 */
    pnp_card_id(b"CSC0225\0", b"CSC0100\0", b"CSC0110\0", b"CSC0103\0"),
    /* Unknown Intel mainboard - CS4235 */
    pnp_card_id(b"CSC0225\0", b"CSC0100\0", b"CSC0110\0", b"\0\0\0\0\0\0\0\0"),
    /* Genius Sound Maker 3DJ - CS4237B */
    pnp_card_id(b"CSC0437\0", b"CSC0000\0", b"CSC0010\0", b"CSC0003\0"),
    /* Digital PC 5000 Onboard - CS4236B */
    pnp_card_id(b"CSC0735\0", b"CSC0000\0", b"CSC0010\0", b"\0\0\0\0\0\0\0\0"),
    /* some unknown CS4236B */
    pnp_card_id(b"CSC0b35\0", b"CSC0000\0", b"CSC0010\0", b"CSC0003\0"),
    /* Intel PR440FX Onboard sound */
    pnp_card_id(b"CSC0b36\0", b"CSC0000\0", b"CSC0010\0", b"CSC0003\0"),
    /* CS4235 on mainboard without MPU */
    pnp_card_id(b"CSC1425\0", b"CSC0100\0", b"CSC0110\0", b"\0\0\0\0\0\0\0\0"),
    /* Gateway E1000 Onboard CS4236B */
    pnp_card_id(b"CSC1335\0", b"CSC0000\0", b"CSC0010\0", b"CSC0003\0"),
    /* HP 6330 Onboard sound */
    pnp_card_id(b"CSC1525\0", b"CSC0100\0", b"CSC0110\0", b"CSC0103\0"),
    /* Crystal Computer TidalWave128 */
    pnp_card_id(b"CSC1e37\0", b"CSC0000\0", b"CSC0010\0", b"CSC0003\0"),
    /* ACER AW37 - CS4235 */
    pnp_card_id(b"CSC4236\0", b"CSC0000\0", b"CSC0010\0", b"CSC0003\0"),
    /* build-in soundcard in EliteGroup P5TX-LA motherboard - CS4237B */
    pnp_card_id(b"CSC4237\0", b"CSC0000\0", b"CSC0010\0", b"CSC0003\0"),
    /* Crystal 3D - CS4237B */
    pnp_card_id(b"CSC4336\0", b"CSC0000\0", b"CSC0010\0", b"CSC0003\0"),
    /* Typhoon Soundsystem PnP - CS4236B */
    pnp_card_id(b"CSC4536\0", b"CSC0000\0", b"CSC0010\0", b"CSC0003\0"),
    /* Crystal CX4235-XQ3 EP - CS4235 */
    pnp_card_id(b"CSC4625\0", b"CSC0100\0", b"CSC0110\0", b"CSC0103\0"),
    /* Crystal Semiconductors CS4237B */
    pnp_card_id(b"CSC4637\0", b"CSC0000\0", b"CSC0010\0", b"CSC0003\0"),
    /* NewClear 3D - CX4237B-XQ3 */
    pnp_card_id(b"CSC4837\0", b"CSC0000\0", b"CSC0010\0", b"CSC0003\0"),
    /* Dell Optiplex GX1 - CS4236B */
    pnp_card_id(b"CSC6835\0", b"CSC0000\0", b"CSC0010\0", b"CSC0003\0"),
    /* Dell P410 motherboard - CS4236B */
    pnp_card_id(b"CSC6835\0", b"CSC0000\0", b"CSC0010\0", b"\0\0\0\0\0\0\0\0"),
    /* Dell Workstation 400 Onboard - CS4236B */
    pnp_card_id(b"CSC6836\0", b"CSC0000\0", b"CSC0010\0", b"CSC0003\0"),
    /* Turtle Beach Malibu - CS4237B */
    pnp_card_id(b"CSC7537\0", b"CSC0000\0", b"CSC0010\0", b"CSC0003\0"),
    /* CS4235 - onboard */
    pnp_card_id(b"CSC8025\0", b"CSC0100\0", b"CSC0110\0", b"CSC0103\0"),
    /* IBM Aptiva 2137 E24 Onboard - CS4237B */
    pnp_card_id(b"CSC8037\0", b"CSC0000\0", b"CSC0010\0", b"CSC0003\0"),
    /* IBM IntelliStation M Pro motherboard */
    pnp_card_id(b"CSCc835\0", b"CSC0000\0", b"CSC0010\0", b"\0\0\0\0\0\0\0\0"),
    /* Guillemot MaxiSound 16 PnP - CS4236B */
    pnp_card_id(b"CSC9836\0", b"CSC0000\0", b"CSC0010\0", b"CSC0003\0"),
    /* Gallant SC-70P */
    pnp_card_id(b"CSC9837\0", b"CSC0000\0", b"CSC0010\0", b"CSC0003\0"),
    /* Techmakers MF-4236PW */
    pnp_card_id(b"CSCa736\0", b"CSC0000\0", b"CSC0010\0", b"CSC0003\0"),
    /* TerraTec AudioSystem EWS64XL - CS4236B */
    pnp_card_id(b"CSCa836\0", b"CSCa800\0", b"CSCa810\0", b"CSCa803\0"),
    /* TerraTec AudioSystem EWS64XL - CS4236B */
    pnp_card_id(b"CSCa836\0", b"CSCa800\0", b"CSCa810\0", b"\0\0\0\0\0\0\0\0"),
    /* ACER AW37/Pro - CS4235 */
    pnp_card_id(b"CSCd925\0", b"CSC0000\0", b"CSC0010\0", b"CSC0003\0"),
    /* ACER AW35/Pro - CS4237B */
    pnp_card_id(b"CSCd937\0", b"CSC0000\0", b"CSC0010\0", b"CSC0003\0"),
    /* CS4235 without MPU401 */
    pnp_card_id(b"CSCe825\0", b"CSC0100\0", b"CSC0110\0", b"\0\0\0\0\0\0\0\0"),
    /* Unknown SiS530 - CS4235 */
    pnp_card_id(b"CSC4825\0", b"CSC0100\0", b"CSC0110\0", b"\0\0\0\0\0\0\0\0"),
    /* IBM IntelliStation M Pro 6898 11U - CS4236B */
    pnp_card_id(b"CSCe835\0", b"CSC0000\0", b"CSC0010\0", b"\0\0\0\0\0\0\0\0"),
    /* IBM PC 300PL Onboard - CS4236B */
    pnp_card_id(b"CSCe836\0", b"CSC0000\0", b"CSC0010\0", b"\0\0\0\0\0\0\0\0"),
    /* Some noname CS4236 based card */
    pnp_card_id(b"CSCe936\0", b"CSC0000\0", b"CSC0010\0", b"CSC0003\0"),
    /* CS4236B */
    pnp_card_id(b"CSCf235\0", b"CSC0000\0", b"CSC0010\0", b"CSC0003\0"),
    /* CS4236B */
    pnp_card_id(b"CSCf238\0", b"CSC0000\0", b"CSC0010\0", b"CSC0003\0"),
    /* --- */
    pnp_card_id(b"\0\0\0\0\0\0\0\0", b"\0\0\0\0\0\0\0\0", b"\0\0\0\0\0\0\0\0", b"\0\0\0\0\0\0\0\0"), /* end */
];

/* WSS initialization */
unsafe extern "C" fn snd_cs423x_pnp_init_wss(dev: c_int, pdev: *mut pnp_dev) -> c_int {
    let dev = dev as usize;
    if pnp_activate_dev(pdev) < 0 {
        dev_err(
            &mut (*pdev).dev,
            cstr!("CS4232+ WSS PnP configure failed for WSS (out of resources?)\n"),
        );
        return -EBUSY;
    }
    port[dev] = pnp_port_start(pdev, 0);
    if fm_port[dev] > 0 {
        fm_port[dev] = pnp_port_start(pdev, 1);
    }
    sb_port[dev] = pnp_port_start(pdev, 2);
    irq[dev] = pnp_irq(pdev, 0) as c_int;
    dma1[dev] = pnp_dma(pdev, 0);
    dma2[dev] = if pnp_dma(pdev, 1) == 4 { -1 } else { pnp_dma(pdev, 1) as c_int };
    dev_dbg(
        &mut (*pdev).dev,
        cstr!("isapnp WSS: wss port=0x%lx, fm port=0x%lx, sb port=0x%lx\n"),
        port[dev],
        fm_port[dev],
        sb_port[dev],
    );
    dev_dbg(
        &mut (*pdev).dev,
        cstr!("isapnp WSS: irq=%i, dma1=%i, dma2=%i\n"),
        irq[dev],
        dma1[dev],
        dma2[dev],
    );
    0
}

/* CTRL initialization */
unsafe extern "C" fn snd_cs423x_pnp_init_ctrl(dev: c_int, pdev: *mut pnp_dev) -> c_int {
    let dev = dev as usize;
    if pnp_activate_dev(pdev) < 0 {
        dev_err(
            &mut (*pdev).dev,
            cstr!("CS4232+ CTRL PnP configure failed for WSS (out of resources?)\n"),
        );
        return -EBUSY;
    }
    cport[dev] = pnp_port_start(pdev, 0);
    dev_dbg(&mut (*pdev).dev, cstr!("isapnp CTRL: control port=0x%lx\n"), cport[dev]);
    0
}

/* MPU initialization */
unsafe extern "C" fn snd_cs423x_pnp_init_mpu(dev: c_int, pdev: *mut pnp_dev) -> c_int {
    let dev = dev as usize;
    if pnp_activate_dev(pdev) < 0 {
        dev_err(
            &mut (*pdev).dev,
            cstr!("CS4232+ MPU401 PnP configure failed for WSS (out of resources?)\n"),
        );
        mpu_port[dev] = SNDRV_AUTO_PORT;
        mpu_irq[dev] = SNDRV_AUTO_IRQ;
    } else {
        mpu_port[dev] = pnp_port_start(pdev, 0);
        if mpu_irq[dev] >= 0 && pnp_irq_valid(pdev, 0) != 0 && pnp_irq(pdev, 0) != !0usize {
            mpu_irq[dev] = pnp_irq(pdev, 0) as c_int;
        } else {
            mpu_irq[dev] = -1; /* disable interrupt */
        }
    }
    dev_dbg(
        &mut (*pdev).dev,
        cstr!("isapnp MPU: port=0x%lx, irq=%i\n"),
        mpu_port[dev],
        mpu_irq[dev],
    );
    0
}

unsafe extern "C" fn snd_card_cs423x_pnp(
    dev: c_int,
    acard: *mut snd_card_cs4236,
    pdev: *mut pnp_dev,
    cdev: *mut pnp_dev,
) -> c_int {
    let udev = dev as usize;
    (*acard).wss = pdev;
    if snd_cs423x_pnp_init_wss(dev, (*acard).wss) < 0 {
        return -EBUSY;
    }
    if !cdev.is_null() {
        cport[udev] = pnp_port_start(cdev, 0);
    } else {
        cport[udev] = -1;
    }
    0
}

unsafe extern "C" fn snd_card_cs423x_pnpc(
    dev: c_int,
    acard: *mut snd_card_cs4236,
    card: *mut pnp_card_link,
    id: *const pnp_card_device_id,
) -> c_int {
    let udev = dev as usize;
    (*acard).wss = pnp_request_card_device(card, (*id).devs[0].id.as_ptr(), ptr::null_mut());
    if (*acard).wss.is_null() {
        return -EBUSY;
    }
    (*acard).ctrl = pnp_request_card_device(card, (*id).devs[1].id.as_ptr(), ptr::null_mut());
    if (*acard).ctrl.is_null() {
        return -EBUSY;
    }
    if (*id).devs[2].id[0] != 0 {
        (*acard).mpu = pnp_request_card_device(card, (*id).devs[2].id.as_ptr(), ptr::null_mut());
        if (*acard).mpu.is_null() {
            return -EBUSY;
        }
    }

    /* WSS initialization */
    if snd_cs423x_pnp_init_wss(dev, (*acard).wss) < 0 {
        return -EBUSY;
    }

    /* CTRL initialization */
    if !(*acard).ctrl.is_null() && cport[udev] > 0 {
        if snd_cs423x_pnp_init_ctrl(dev, (*acard).ctrl) < 0 {
            return -EBUSY;
        }
    }
    /* MPU initialization */
    if !(*acard).mpu.is_null() && mpu_port[udev] > 0 {
        if snd_cs423x_pnp_init_mpu(dev, (*acard).mpu) < 0 {
            return -EBUSY;
        }
    }
    0
}

unsafe fn is_isapnp_selected(dev: usize) -> c_int {
    if isapnp[dev] { 1 } else { 0 }
}

unsafe extern "C" fn snd_cs423x_card_new(
    pdev: *mut device,
    dev: c_int,
    cardp: *mut *mut snd_card,
) -> c_int {
    let mut card: *mut snd_card = ptr::null_mut();
    let err = snd_devm_card_new(
        pdev,
        index[dev as usize],
        id[dev as usize],
        THIS_MODULE,
        core::mem::size_of::<snd_card_cs4236>(),
        &mut card,
    );
    if err < 0 {
        return err;
    }
    *cardp = card;
    0
}

unsafe extern "C" fn snd_cs423x_probe(card: *mut snd_card, dev: c_int) -> c_int {
    let dev = dev as usize;
    let acard: *mut snd_card_cs4236 = (*card).private_data as *mut snd_card_cs4236;
    let mut chip: *mut snd_wss = ptr::null_mut();
    let mut opl3: *mut snd_opl3 = ptr::null_mut();
    let mut err: c_int;

    if sb_port[dev] > 0 && sb_port[dev] != SNDRV_AUTO_PORT {
        if devm_request_region((*card).dev, sb_port[dev], 16, cstr!("CS4232+ SB")).is_null() {
            dev_err(
                (*card).dev,
                cstr!("CS4232+: unable to register SB port at 0x%lx\n"),
                sb_port[dev],
            );
            return -EBUSY;
        }
    }

    err = snd_cs4236_create(
        card,
        port[dev],
        cport[dev],
        irq[dev],
        dma1[dev],
        dma2[dev],
        WSS_HW_DETECT3,
        0,
        &mut chip,
    );
    if err < 0 {
        return err;
    }

    (*acard).chip = chip;
    if ((*chip).hardware & WSS_HW_CS4236B_MASK) != 0 {
        err = snd_cs4236_pcm(chip, 0);
        if err < 0 {
            return err;
        }

        err = snd_cs4236_mixer(chip);
        if err < 0 {
            return err;
        }
    } else {
        err = snd_wss_pcm(chip, 0);
        if err < 0 {
            return err;
        }

        err = snd_wss_mixer(chip);
        if err < 0 {
            return err;
        }
    }
    strscpy((*card).driver.as_mut_ptr(), (*(*chip).pcm).name, size_of_val(&(*card).driver));
    strscpy(
        (*card).shortname.as_mut_ptr(),
        (*(*chip).pcm).name,
        size_of_val(&(*card).shortname),
    );
    if dma2[dev] < 0 {
        scnprintf(
            (*card).longname.as_mut_ptr(),
            size_of_val(&(*card).longname),
            cstr!("%s at 0x%lx, irq %i, dma %i"),
            (*(*chip).pcm).name,
            (*chip).port,
            irq[dev],
            dma1[dev],
        );
    } else {
        scnprintf(
            (*card).longname.as_mut_ptr(),
            size_of_val(&(*card).longname),
            cstr!("%s at 0x%lx, irq %i, dma %i&%d"),
            (*(*chip).pcm).name,
            (*chip).port,
            irq[dev],
            dma1[dev],
            dma2[dev],
        );
    }

    err = snd_wss_timer(chip, 0);
    if err < 0 {
        return err;
    }

    if fm_port[dev] > 0 && fm_port[dev] != SNDRV_AUTO_PORT {
        if snd_opl3_create(
            card,
            fm_port[dev],
            fm_port[dev] + 2,
            OPL3_HW_OPL3_CS,
            0,
            &mut opl3,
        ) < 0
        {
            dev_warn((*card).dev, cstr!("CS4232+: OPL3 not detected\n"));
        } else {
            err = snd_opl3_hwdep_new(opl3, 0, 1, ptr::null_mut());
            if err < 0 {
                return err;
            }
        }
    }

    if mpu_port[dev] > 0 && mpu_port[dev] != SNDRV_AUTO_PORT {
        if mpu_irq[dev] == SNDRV_AUTO_IRQ {
            mpu_irq[dev] = -1;
        }
        if snd_mpu401_uart_new(
            card,
            0,
            MPU401_HW_CS4232,
            mpu_port[dev],
            0,
            mpu_irq[dev],
            ptr::null_mut(),
        ) < 0
        {
            dev_warn((*card).dev, cstr!("CS4232+: MPU401 not detected\n"));
        }
    }

    snd_card_register(card)
}

unsafe extern "C" fn snd_cs423x_isa_match(pdev: *mut device, dev: c_uint) -> c_int {
    let dev = dev as usize;
    if !enable[dev] || is_isapnp_selected(dev) != 0 {
        return 0;
    }

    if port[dev] == SNDRV_AUTO_PORT {
        dev_err(pdev, cstr!("please specify port\n"));
        return 0;
    }
    if cport[dev] == SNDRV_AUTO_PORT {
        dev_err(pdev, cstr!("please specify cport\n"));
        return 0;
    }
    if irq[dev] == SNDRV_AUTO_IRQ {
        dev_err(pdev, cstr!("please specify irq\n"));
        return 0;
    }
    if dma1[dev] == SNDRV_AUTO_DMA {
        dev_err(pdev, cstr!("please specify dma1\n"));
        return 0;
    }
    1
}

unsafe extern "C" fn snd_cs423x_isa_probe(pdev: *mut device, dev: c_uint) -> c_int {
    let mut card: *mut snd_card = ptr::null_mut();
    let mut err = snd_cs423x_card_new(pdev, dev as c_int, &mut card);
    if err < 0 {
        return err;
    }
    err = snd_cs423x_probe(card, dev as c_int);
    if err < 0 {
        return err;
    }
    dev_set_drvdata(pdev, card as *mut c_void);
    0
}

// CONFIG_PM
unsafe extern "C" fn snd_cs423x_suspend(card: *mut snd_card) -> c_int {
    let acard: *mut snd_card_cs4236 = (*card).private_data as *mut snd_card_cs4236;
    snd_power_change_state(card, SNDRV_CTL_POWER_D3HOT);
    ((*(*acard).chip).suspend.unwrap())((*acard).chip);
    0
}

unsafe extern "C" fn snd_cs423x_resume(card: *mut snd_card) -> c_int {
    let acard: *mut snd_card_cs4236 = (*card).private_data as *mut snd_card_cs4236;
    ((*(*acard).chip).resume.unwrap())((*acard).chip);
    snd_power_change_state(card, SNDRV_CTL_POWER_D0);
    0
}

unsafe extern "C" fn snd_cs423x_isa_suspend(
    dev: *mut device,
    _n: c_uint,
    _state: pm_message_t,
) -> c_int {
    snd_cs423x_suspend(dev_get_drvdata(dev) as *mut snd_card)
}

unsafe extern "C" fn snd_cs423x_isa_resume(dev: *mut device, _n: c_uint) -> c_int {
    snd_cs423x_resume(dev_get_drvdata(dev) as *mut snd_card)
}

static mut cs423x_isa_driver: isa_driver = isa_driver {
    match_: Some(snd_cs423x_isa_match),
    probe: Some(snd_cs423x_isa_probe),
    // CONFIG_PM
    suspend: Some(snd_cs423x_isa_suspend),
    resume: Some(snd_cs423x_isa_resume),
    driver: device_driver {
        name: DEV_NAME.as_ptr() as *const c_char,
    },
};

// CONFIG_PNP
unsafe extern "C" fn snd_cs423x_pnpbios_detect(
    pdev: *mut pnp_dev,
    _id: *const pnp_device_id,
) -> c_int {
    static mut dev: c_int = 0;
    let mut err: c_int;
    let mut card: *mut snd_card = ptr::null_mut();
    let mut cdev: *mut pnp_dev;
    let mut iter: *mut pnp_dev;
    let mut cid: [c_char; PNP_ID_LEN] = [0; PNP_ID_LEN];

    if pnp_device_is_isapnp(pdev) != 0 {
        return -ENOENT; /* we have another procedure - card */
    }
    while dev < SNDRV_CARDS as c_int {
        if enable[dev as usize] && isapnp[dev as usize] {
            break;
        }
        dev += 1;
    }
    if dev >= SNDRV_CARDS as c_int {
        return -ENODEV;
    }

    /* prepare second id */
    strscpy(cid.as_mut_ptr(), (*pdev).id[0].id.as_ptr(), PNP_ID_LEN);
    cid[5] = b'1' as c_char;
    cdev = ptr::null_mut();
    // list_for_each_entry(iter, &(pdev->protocol->devices), protocol_list)
    iter = (*(*(*pdev).protocol).devices.next.cast::<pnp_dev>());
    while !ptr::eq(
        &mut (*iter).protocol_list as *mut list_head,
        &mut (*(*pdev).protocol).devices as *mut list_head,
    ) {
        if strcmp((*iter).id[0].id.as_ptr(), cid.as_ptr()) == 0 {
            cdev = iter;
            break;
        }
        iter = (*(*iter).protocol_list.next.cast::<pnp_dev>());
    }
    err = snd_cs423x_card_new(&mut (*pdev).dev, dev, &mut card);
    if err < 0 {
        return err;
    }
    err = snd_card_cs423x_pnp(dev, (*card).private_data as *mut snd_card_cs4236, pdev, cdev);
    if err < 0 {
        dev_err((*card).dev, cstr!("PnP BIOS detection failed for CS4232+\n"));
        return err;
    }
    err = snd_cs423x_probe(card, dev);
    if err < 0 {
        return err;
    }
    pnp_set_drvdata(pdev, card as *mut c_void);
    dev += 1;
    0
}

// CONFIG_PM
unsafe extern "C" fn snd_cs423x_pnp_suspend(
    pdev: *mut pnp_dev,
    _state: pm_message_t,
) -> c_int {
    snd_cs423x_suspend(pnp_get_drvdata(pdev) as *mut snd_card)
}

unsafe extern "C" fn snd_cs423x_pnp_resume(pdev: *mut pnp_dev) -> c_int {
    snd_cs423x_resume(pnp_get_drvdata(pdev) as *mut snd_card)
}

static mut cs423x_pnp_driver: pnp_driver = pnp_driver {
    name: cstr!("cs423x-pnpbios"),
    id_table: snd_cs423x_pnpbiosids.as_ptr(),
    probe: Some(snd_cs423x_pnpbios_detect),
    // CONFIG_PM
    suspend: Some(snd_cs423x_pnp_suspend),
    resume: Some(snd_cs423x_pnp_resume),
};

unsafe extern "C" fn snd_cs423x_pnpc_detect(
    pcard: *mut pnp_card_link,
    pid: *const pnp_card_device_id,
) -> c_int {
    static mut dev: c_int = 0;
    let mut card: *mut snd_card = ptr::null_mut();
    let mut res: c_int;

    while dev < SNDRV_CARDS as c_int {
        if enable[dev as usize] && isapnp[dev as usize] {
            break;
        }
        dev += 1;
    }
    if dev >= SNDRV_CARDS as c_int {
        return -ENODEV;
    }

    res = snd_cs423x_card_new(&mut (*(*pcard).card).dev, dev, &mut card);
    if res < 0 {
        return res;
    }
    res = snd_card_cs423x_pnpc(dev, (*card).private_data as *mut snd_card_cs4236, pcard, pid);
    if res < 0 {
        dev_err(
            (*card).dev,
            cstr!("isapnp detection failed and probing for CS4232+ is not supported\n"),
        );
        return res;
    }
    res = snd_cs423x_probe(card, dev);
    if res < 0 {
        return res;
    }
    pnp_set_card_drvdata(pcard, card as *mut c_void);
    dev += 1;
    0
}

// CONFIG_PM
unsafe extern "C" fn snd_cs423x_pnpc_suspend(
    pcard: *mut pnp_card_link,
    _state: pm_message_t,
) -> c_int {
    snd_cs423x_suspend(pnp_get_card_drvdata(pcard) as *mut snd_card)
}

unsafe extern "C" fn snd_cs423x_pnpc_resume(pcard: *mut pnp_card_link) -> c_int {
    snd_cs423x_resume(pnp_get_card_drvdata(pcard) as *mut snd_card)
}

static mut cs423x_pnpc_driver: pnp_card_driver = pnp_card_driver {
    flags: PNP_DRIVER_RES_DISABLE,
    name: CS423X_ISAPNP_DRIVER.as_ptr() as *const c_char,
    id_table: snd_cs423x_pnpids.as_ptr(),
    probe: Some(snd_cs423x_pnpc_detect),
    // CONFIG_PM
    suspend: Some(snd_cs423x_pnpc_suspend),
    resume: Some(snd_cs423x_pnpc_resume),
};

unsafe extern "C" fn alsa_card_cs423x_init() -> c_int {
    let mut err: c_int;

    err = isa_register_driver(&mut cs423x_isa_driver, SNDRV_CARDS as c_uint);
    // CONFIG_PNP
    if err == 0 {
        isa_registered = 1;
    }
    err = pnp_register_driver(&mut cs423x_pnp_driver);
    if err == 0 {
        pnp_registered = 1;
    }
    err = pnp_register_card_driver(&mut cs423x_pnpc_driver);
    if err == 0 {
        pnpc_registered = 1;
    }
    if pnp_registered != 0 {
        err = 0;
    }
    if isa_registered != 0 {
        err = 0;
    }
    err
}

unsafe extern "C" fn alsa_card_cs423x_exit() {
    // CONFIG_PNP
    if pnpc_registered != 0 {
        pnp_unregister_card_driver(&mut cs423x_pnpc_driver);
    }
    if pnp_registered != 0 {
        pnp_unregister_driver(&mut cs423x_pnp_driver);
    }
    if isa_registered != 0 {
        isa_unregister_driver(&mut cs423x_isa_driver);
    }
}

// module_init(alsa_card_cs423x_init)
// module_exit(alsa_card_cs423x_exit)

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
