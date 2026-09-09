// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/sh/boards/hp6xx/setup.c
 *
 * Copyright (C) 2002 Andriy Skulysh
 * Copyright (C) 2007 Kristoffer Ericson <Kristoffer_e1@hotmail.com>
 *
 * Setup code for HP620/HP660/HP680/HP690 (internal peripherials only)
 */

// C header dependencies are supplied by the surrounding kernel translation.

const SCPCR: usize = 0xa4000116;
const SCPDR: usize = 0xa4000136;

/* CF Slot */
static mut cf_ide_resources: [resource; 3] = [
    resource {
        start: 0x15000000 + 0x1f0,
        end: 0x15000000 + 0x1f0 + 0x08 - 0x01,
        flags: IORESOURCE_MEM,
    },
    resource {
        start: 0x15000000 + 0x1fe,
        end: 0x15000000 + 0x1fe + 0x01,
        flags: IORESOURCE_MEM,
    },
    resource {
        start: evt2irq(0xba0),
        end: 0,
        flags: IORESOURCE_IRQ,
    },
];

static mut cf_ide_device: platform_device = platform_device {
    name: "pata_platform",
    id: -1,
    num_resources: cf_ide_resources.len(),
    resource: cf_ide_resources.as_mut_ptr(),
    dev: platform_device_dev {},
};

static mut jornadakbd_device: platform_device = platform_device {
    name: "jornada680_kbd",
    id: -1,
    num_resources: 0,
    resource: core::ptr::null_mut(),
    dev: platform_device_dev {},
};

unsafe extern "C" fn dac_audio_start(pdata: *mut dac_audio_pdata) {
    let mut v: u16;
    let mut v8: u8;

    /* HP Jornada 680/690 speaker on */
    v = inw(HD64461_GPADR);
    v &= !HD64461_GPADR_SPEAKER;
    outw(v, HD64461_GPADR);

    /* HP Palmtop 620lx/660lx speaker on */
    v8 = inb(PKDR);
    v8 &= !PKDR_SPEAKER;
    outb(v8, PKDR);

    sh_dac_enable((*pdata).channel);
}

unsafe extern "C" fn dac_audio_stop(pdata: *mut dac_audio_pdata) {
    let mut v: u16;
    let mut v8: u8;

    /* HP Jornada 680/690 speaker off */
    v = inw(HD64461_GPADR);
    v |= HD64461_GPADR_SPEAKER;
    outw(v, HD64461_GPADR);

    /* HP Palmtop 620lx/660lx speaker off */
    v8 = inb(PKDR);
    v8 |= PKDR_SPEAKER;
    outb(v8, PKDR);

    sh_dac_output(0, (*pdata).channel);
    sh_dac_disable((*pdata).channel);
}

static mut dac_audio_platform_data: dac_audio_pdata = dac_audio_pdata {
    buffer_size: 64000,
    channel: 1,
    start: Some(dac_audio_start),
    stop: Some(dac_audio_stop),
};

static mut dac_audio_device: platform_device = platform_device {
    name: "dac_audio",
    id: -1,
    num_resources: 0,
    resource: core::ptr::null_mut(),
    dev: platform_device_dev {
        platform_data: &mut dac_audio_platform_data as *mut _ as *mut core::ffi::c_void,
    },
};

static mut hp6xx_devices: [*mut platform_device; 3] = [
    &mut cf_ide_device,
    &mut jornadakbd_device,
    &mut dac_audio_device,
];

unsafe extern "C" fn hp6xx_init_irq() {
    /* Gets touchscreen and powerbutton IRQ working */
    plat_irq_setup_pins(IRQ_MODE_IRQ);
}

unsafe extern "C" fn hp6xx_devices_setup() -> i32 {
    platform_add_devices(hp6xx_devices.as_mut_ptr(), hp6xx_devices.len())
}

unsafe extern "C" fn hp6xx_setup(_cmdline_p: *mut *mut u8) {
    let mut v8: u8;
    let mut v: u16;

    v = inw(HD64461_STBCR);
    v |= HD64461_STBCR_SURTST | HD64461_STBCR_SIRST |
        HD64461_STBCR_STM1ST | HD64461_STBCR_STM0ST |
        HD64461_STBCR_SAFEST | HD64461_STBCR_SPC0ST |
        HD64461_STBCR_SMIAST | HD64461_STBCR_SAFECKE_OST |
        HD64461_STBCR_SAFECKE_IST;
    // #ifndef CONFIG_HD64461_ENABLER
    v |= HD64461_STBCR_SPC1ST;
    // #endif
    outw(v, HD64461_STBCR);
    v = inw(HD64461_GPADR);
    v |= HD64461_GPADR_SPEAKER | HD64461_GPADR_PCMCIA0;
    outw(v, HD64461_GPADR);

    outw(HD64461_PCCGCR_VCC0 | HD64461_PCCSCR_VCC1, HD64461_PCC0GCR);
    // #ifndef CONFIG_HD64461_ENABLER
    outw(HD64461_PCCGCR_VCC0 | HD64461_PCCSCR_VCC1, HD64461_PCC1GCR);
    // #endif

    sh_dac_output(0, DAC_SPEAKER_VOLUME);
    sh_dac_disable(DAC_SPEAKER_VOLUME);
    v8 = __raw_readb(DACR);
    v8 &= !DACR_DAE;
    __raw_writeb(v8, DACR);

    v8 = __raw_readb(SCPDR);
    v8 |= SCPDR_TS_SCAN_X | SCPDR_TS_SCAN_Y;
    v8 &= !SCPDR_TS_SCAN_ENABLE;
    __raw_writeb(v8, SCPDR);

    v = __raw_readw(SCPCR);
    v &= !SCPCR_TS_MASK;
    v |= SCPCR_TS_ENABLE;
    __raw_writew(v, SCPCR);
}

// device_initcall(hp6xx_devices_setup);

static mut mv_hp6xx: sh_machine_vector = sh_machine_vector {
    mv_name: "hp6xx",
    mv_setup: Some(hp6xx_setup),
    /* Enable IRQ0 -> IRQ3 in IRQ_MODE */
    mv_init_irq: Some(hp6xx_init_irq),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
