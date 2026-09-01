// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Routines for Gravis UltraSound soundcards
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};

// C includes translated as external dependency intent:
// <linux/init.h>, <linux/interrupt.h>, <linux/delay.h>, <linux/slab.h>,
// <linux/ioport.h>, <linux/module.h>, <sound/core.h>, <sound/gus.h>,
// <sound/control.h>, <asm/dma.h>
// MODULE_AUTHOR("Jaroslav Kysela <perex@perex.cz>");
// MODULE_DESCRIPTION("Routines for Gravis UltraSound soundcards");
// MODULE_LICENSE("GPL");

const SNDRV_CTL_ELEM_TYPE_INTEGER: c_int = 2;
const SNDRV_CTL_ELEM_IFACE_CARD: c_int = 0;
const SNDRV_DEV_LOWLEVEL: c_int = 0;
const SNDRV_CTL_POWER_D3HOT: c_int = 3;
const SNDRV_CTL_POWER_D0: c_int = 0;
const ENOMEM: c_int = 12;
const EBUSY: c_int = 16;
const EINVAL: c_int = 22;

const GF1PAGE: c_ulong = 0;
const GF1REGSEL: c_ulong = 0;
const GF1DATAHIGH: c_ulong = 0;
const GF1DATALOW: c_ulong = 0;
const IRQSTAT: c_ulong = 0;
const DRAM: c_ulong = 0;
const TIMERCNTRL: c_ulong = 0;
const TIMERDATA: c_ulong = 0;
const REGCNTRLS: c_ulong = 0;
const MIXCNTRLREG: c_ulong = 0;
const IRQDMACNTRLREG: c_ulong = 0;
const BOARDVERSION: c_ulong = 0;
const SNDRV_GF1_GB_JOYSTICK_DAC_LEVEL: c_int = 0;

#[repr(C)]
pub struct resource {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm {
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
pub struct snd_card {
    pub dev: *mut device,
    pub sync_irq: c_int,
    pub driver: [c_char; 16],
    pub shortname: [c_char; 32],
    pub longname: [c_char; 80],
}

#[repr(C)]
pub struct snd_device {
    pub device_data: *mut c_void,
}

#[repr(C)]
pub struct snd_device_ops {
    pub dev_free: Option<unsafe extern "C" fn(*mut snd_device) -> c_int>,
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_int,
    pub name: *const c_char,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: c_int,
    pub count: c_int,
    pub value: snd_ctl_elem_info_value,
}

#[repr(C)]
pub union snd_ctl_elem_info_value {
    pub integer: snd_ctl_elem_info_integer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_info_integer {
    pub min: i64,
    pub max: i64,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer {
    pub value: [i64; 128],
}

#[repr(C)]
pub struct snd_gf1_mem_bank {
    pub address: c_int,
    pub size: c_int,
}

#[repr(C)]
pub struct snd_gf1_mem_alloc {
    pub banks_8: [snd_gf1_mem_bank; 4],
    pub banks_16: [snd_gf1_mem_bank; 4],
}

#[repr(C)]
pub struct snd_gf1 {
    pub res_port1: *mut resource,
    pub res_port2: *mut resource,
    pub irq: c_int,
    pub dma1: c_int,
    pub dma2: c_int,
    pub port: c_ulong,
    pub reg_page: c_ulong,
    pub reg_regsel: c_ulong,
    pub reg_data8: c_ulong,
    pub reg_data16: c_ulong,
    pub reg_irqstat: c_ulong,
    pub reg_dram: c_ulong,
    pub reg_timerctrl: c_ulong,
    pub reg_timerdata: c_ulong,
    pub effect: c_int,
    pub active_voices: c_int,
    pub pcm_channels: c_int,
    pub volume_ramp: c_int,
    pub smooth_pan: c_int,
    pub memory: c_int,
    pub mem_alloc: snd_gf1_mem_alloc,
}

#[repr(C)]
pub struct snd_gus_card {
    pub reg_lock: spinlock_t,
    pub voice_alloc: spinlock_t,
    pub active_voice_lock: spinlock_t,
    pub event_lock: spinlock_t,
    pub dma_lock: spinlock_t,
    pub pcm_volume_level_lock: spinlock_t,
    pub uart_cmd_lock: spinlock_t,
    pub dma_mutex: mutex,
    pub gf1: snd_gf1,
    pub card: *mut snd_card,
    pub equal_dma: c_int,
    pub timer_dev: c_int,
    pub ace_flag: c_int,
    pub codec_flag: c_int,
    pub ess_flag: c_int,
    pub mix_cntrl_reg: u8,
    pub ics_flag: c_int,
    pub ics_flipped: c_int,
    pub max_flag: c_int,
    pub uart_enable: c_int,
    pub interwave: c_int,
    pub initialized: c_int,
    pub joystick_dac: u8,
    pub pcm: *mut snd_pcm,
}

unsafe extern "C" {
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut c_void;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_ctl_new1(kcontrol: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn snd_gf1_write8(gus: *mut snd_gus_card, reg: c_int, data: u8);
    fn snd_gf1_stop(gus: *mut snd_gus_card);
    fn release_and_free_resource(res: *mut resource);
    fn free_irq(irq: c_int, dev_id: *mut c_void);
    fn disable_dma(dma: c_int);
    fn free_dma(dma: c_int);
    fn kfree(ptr: *mut c_void);
    fn kzalloc(size: usize, flags: c_int) -> *mut c_void;
    fn spin_lock_init(lock: *mut spinlock_t);
    fn mutex_init(lock: *mut mutex);
    fn request_region(start: c_ulong, n: c_ulong, name: *const c_char) -> *mut resource;
    fn request_irq(
        irq: c_int,
        handler: unsafe extern "C" fn(c_int, *mut c_void) -> c_int,
        flags: c_ulong,
        name: *const c_char,
        dev: *mut c_void,
    ) -> c_int;
    fn snd_gus_interrupt(irq: c_int, dev_id: *mut c_void) -> c_int;
    fn request_dma(dma: c_int, device_id: *const c_char) -> c_int;
    fn snd_device_new(
        card: *mut snd_card,
        type_: c_int,
        device_data: *mut c_void,
        ops: *const snd_device_ops,
    ) -> c_int;
    fn snd_gf1_poke(gus: *mut snd_gus_card, addr: c_ulong, data: u8);
    fn snd_gf1_peek(gus: *mut snd_gus_card, addr: c_ulong) -> u8;
    fn outb(value: u8, port: c_ulong);
    fn inb(port: c_ulong) -> u8;
    fn udelay(usecs: c_ulong);
    fn snd_gf1_delay(gus: *mut snd_gus_card);
    fn strscpy(dst: *mut c_char, src: *const c_char, size: usize) -> isize;
    fn snd_gf1_start(gus: *mut snd_gus_card);
    fn snd_pcm_suspend_all(pcm: *mut snd_pcm) -> c_int;
    fn snd_gf1_suspend(gus: *mut snd_gus_card) -> c_int;
    fn snd_power_change_state(card: *mut snd_card, state: c_int);
    fn snd_gf1_resume(gus: *mut snd_gus_card) -> c_int;
}

unsafe fn gusp(gus: *mut snd_gus_card, reg: c_ulong) -> c_ulong {
    (*gus).gf1.port.wrapping_add(reg)
}

unsafe fn snd_bug_on(condition: bool) -> bool {
    condition
}

unsafe fn dev_err(_dev: *mut device, _fmt: *const c_char, ...) {}
unsafe fn dev_dbg(_dev: *mut device, _fmt: *const c_char, ...) {}

static mut SND_GUS_JOYSTICK_CONTROL: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_CARD,
    name: b"Joystick Speed\0".as_ptr() as *const c_char,
    info: Some(snd_gus_joystick_info),
    get: Some(snd_gus_joystick_get),
    put: Some(snd_gus_joystick_put),
};

unsafe extern "C" fn snd_gus_joystick_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 31;
    0
}

unsafe extern "C" fn snd_gus_joystick_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let gus = snd_kcontrol_chip(kcontrol) as *mut snd_gus_card;

    (*ucontrol).value.integer.value[0] = ((*gus).joystick_dac & 31) as i64;
    0
}

unsafe extern "C" fn snd_gus_joystick_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let gus = snd_kcontrol_chip(kcontrol) as *mut snd_gus_card;
    let change: c_int;
    let nval: u8;

    nval = ((*ucontrol).value.integer.value[0] as u8) & 31;
    // guard(spinlock_irqsave)(&gus->reg_lock);
    change = ((*gus).joystick_dac != nval) as c_int;
    (*gus).joystick_dac = nval;
    snd_gf1_write8(gus, SNDRV_GF1_GB_JOYSTICK_DAC_LEVEL, (*gus).joystick_dac);
    change
}

unsafe fn snd_gus_init_control(gus: *mut snd_gus_card) {
    if (*gus).ace_flag == 0 {
        snd_ctl_add(
            (*gus).card,
            snd_ctl_new1(&raw const SND_GUS_JOYSTICK_CONTROL, gus as *mut c_void),
        );
    }
}

/*
 *
 */

unsafe fn snd_gus_free(gus: *mut snd_gus_card) -> c_int {
    if (*gus).gf1.res_port2 != core::ptr::null_mut() {
        snd_gf1_stop(gus);
        snd_gus_init_dma_irq(gus, 0);
    }
    release_and_free_resource((*gus).gf1.res_port1);
    release_and_free_resource((*gus).gf1.res_port2);
    if (*gus).gf1.irq >= 0 {
        free_irq((*gus).gf1.irq, gus as *mut c_void);
    }
    if (*gus).gf1.dma1 >= 0 {
        disable_dma((*gus).gf1.dma1);
        free_dma((*gus).gf1.dma1);
    }
    if (*gus).equal_dma == 0 && (*gus).gf1.dma2 >= 0 {
        disable_dma((*gus).gf1.dma2);
        free_dma((*gus).gf1.dma2);
    }
    kfree(gus as *mut c_void);
    0
}

unsafe extern "C" fn snd_gus_dev_free(device: *mut snd_device) -> c_int {
    let gus = (*device).device_data as *mut snd_gus_card;
    snd_gus_free(gus)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_gus_create(
    card: *mut snd_card,
    port: c_ulong,
    irq: c_int,
    dma1: c_int,
    dma2: c_int,
    timer_dev: c_int,
    mut voices: c_int,
    mut pcm_channels: c_int,
    effect: c_int,
    rgus: *mut *mut snd_gus_card,
) -> c_int {
    let gus: *mut snd_gus_card;
    let mut err: c_int;
    static OPS: snd_device_ops = snd_device_ops {
        dev_free: Some(snd_gus_dev_free),
    };

    *rgus = core::ptr::null_mut();
    gus = kzalloc(core::mem::size_of::<snd_gus_card>(), 0) as *mut snd_gus_card;
    if gus == core::ptr::null_mut() {
        return -ENOMEM;
    }
    spin_lock_init(&mut (*gus).reg_lock);
    spin_lock_init(&mut (*gus).voice_alloc);
    spin_lock_init(&mut (*gus).active_voice_lock);
    spin_lock_init(&mut (*gus).event_lock);
    spin_lock_init(&mut (*gus).dma_lock);
    spin_lock_init(&mut (*gus).pcm_volume_level_lock);
    spin_lock_init(&mut (*gus).uart_cmd_lock);
    mutex_init(&mut (*gus).dma_mutex);
    (*gus).gf1.irq = -1;
    (*gus).gf1.dma1 = -1;
    (*gus).gf1.dma2 = -1;
    (*gus).card = card;
    (*gus).gf1.port = port;
    /* fill register variables for speedup */
    (*gus).gf1.reg_page = gusp(gus, GF1PAGE);
    (*gus).gf1.reg_regsel = gusp(gus, GF1REGSEL);
    (*gus).gf1.reg_data8 = gusp(gus, GF1DATAHIGH);
    (*gus).gf1.reg_data16 = gusp(gus, GF1DATALOW);
    (*gus).gf1.reg_irqstat = gusp(gus, IRQSTAT);
    (*gus).gf1.reg_dram = gusp(gus, DRAM);
    (*gus).gf1.reg_timerctrl = gusp(gus, TIMERCNTRL);
    (*gus).gf1.reg_timerdata = gusp(gus, TIMERDATA);
    /* allocate resources */
    (*gus).gf1.res_port1 = request_region(port, 16, c"GUS GF1 (Adlib/SB)".as_ptr());
    if (*gus).gf1.res_port1 == core::ptr::null_mut() {
        dev_err((*card).dev, c"gus: can't grab SB port 0x%lx\n".as_ptr(), port);
        snd_gus_free(gus);
        return -EBUSY;
    }
    (*gus).gf1.res_port2 = request_region(port.wrapping_add(0x100), 12, c"GUS GF1 (Synth)".as_ptr());
    if (*gus).gf1.res_port2 == core::ptr::null_mut() {
        dev_err(
            (*card).dev,
            c"gus: can't grab synth port 0x%lx\n".as_ptr(),
            port.wrapping_add(0x100),
        );
        snd_gus_free(gus);
        return -EBUSY;
    }
    if irq >= 0
        && request_irq(
            irq,
            snd_gus_interrupt,
            0,
            c"GUS GF1".as_ptr(),
            gus as *mut c_void,
        ) != 0
    {
        dev_err((*card).dev, c"gus: can't grab irq %d\n".as_ptr(), irq);
        snd_gus_free(gus);
        return -EBUSY;
    }
    (*gus).gf1.irq = irq;
    (*card).sync_irq = irq;
    if request_dma(dma1, c"GUS - 1".as_ptr()) != 0 {
        dev_err((*card).dev, c"gus: can't grab DMA1 %d\n".as_ptr(), dma1);
        snd_gus_free(gus);
        return -EBUSY;
    }
    (*gus).gf1.dma1 = dma1;
    if dma2 >= 0 && dma1 != dma2 {
        if request_dma(dma2, c"GUS - 2".as_ptr()) != 0 {
            dev_err((*card).dev, c"gus: can't grab DMA2 %d\n".as_ptr(), dma2);
            snd_gus_free(gus);
            return -EBUSY;
        }
        (*gus).gf1.dma2 = dma2;
    } else {
        (*gus).gf1.dma2 = (*gus).gf1.dma1;
        (*gus).equal_dma = 1;
    }
    (*gus).timer_dev = timer_dev;
    if voices < 14 {
        voices = 14;
    }
    if voices > 32 {
        voices = 32;
    }
    if pcm_channels < 0 {
        pcm_channels = 0;
    }
    if pcm_channels > 8 {
        pcm_channels = 8;
    }
    pcm_channels += 1;
    pcm_channels &= !1;
    (*gus).gf1.effect = if effect != 0 { 1 } else { 0 };
    (*gus).gf1.active_voices = voices;
    (*gus).gf1.pcm_channels = pcm_channels;
    (*gus).gf1.volume_ramp = 25;
    (*gus).gf1.smooth_pan = 1;
    err = snd_device_new(card, SNDRV_DEV_LOWLEVEL, gus as *mut c_void, &OPS);
    if err < 0 {
        snd_gus_free(gus);
        return err;
    }
    *rgus = gus;
    0
}

/*
 *  Memory detection routine for plain GF1 soundcards
 */

unsafe fn snd_gus_detect_memory(gus: *mut snd_gus_card) -> c_int {
    let mut l: c_int;
    let mut idx: c_int;
    let mut local: c_int;
    let mut d: u8;

    snd_gf1_poke(gus, 0, 0xaa);
    snd_gf1_poke(gus, 1, 0x55);
    if snd_gf1_peek(gus, 0) != 0xaa || snd_gf1_peek(gus, 1) != 0x55 {
        dev_err(
            (*(*gus).card).dev,
            c"plain GF1 card at 0x%lx without onboard DRAM?\n".as_ptr(),
            (*gus).gf1.port,
        );
        return -ENOMEM;
    }
    idx = 1;
    d = 0xab;
    while idx < 4 {
        local = idx << 18;
        snd_gf1_poke(gus, local as c_ulong, d);
        snd_gf1_poke(gus, (local + 1) as c_ulong, d.wrapping_add(1));
        if snd_gf1_peek(gus, local as c_ulong) != d
            || snd_gf1_peek(gus, (local + 1) as c_ulong) != d.wrapping_add(1)
            || snd_gf1_peek(gus, 0) != 0xaa
        {
            break;
        }
        idx += 1;
        d = d.wrapping_add(1);
    }
    // #if 1
    (*gus).gf1.memory = idx << 18;
    // #else
    // gus->gf1.memory = 256 * 1024;
    // #endif
    l = 0;
    local = (*gus).gf1.memory;
    while l < 4 {
        (*gus).gf1.mem_alloc.banks_8[l as usize].address = 0;
        (*gus).gf1.mem_alloc.banks_8[l as usize].size = 0;
        (*gus).gf1.mem_alloc.banks_16[l as usize].address = l << 18;
        (*gus).gf1.mem_alloc.banks_16[l as usize].size =
            if local > 0 { 256 * 1024 } else { 0 };
        l += 1;
        local -= 256 * 1024;
    }
    (*gus).gf1.mem_alloc.banks_8[0].size = (*gus).gf1.memory;
    0 /* some memory were detected */
}

unsafe fn snd_gus_init_dma_irq(gus: *mut snd_gus_card, latches: c_int) -> c_int {
    let card: *mut snd_card;
    let mut irq: c_int;
    let mut dma1: c_int;
    let mut dma2: c_int;
    static IRQS: [u8; 16] = [0, 0, 1, 3, 0, 2, 0, 4, 0, 1, 0, 5, 6, 0, 0, 7];
    static DMAS: [u8; 8] = [6, 1, 0, 2, 0, 3, 4, 5];

    if snd_bug_on(gus.is_null()) {
        return -EINVAL;
    }
    card = (*gus).card;
    if snd_bug_on(card.is_null()) {
        return -EINVAL;
    }

    (*gus).mix_cntrl_reg &= 0xf8;
    (*gus).mix_cntrl_reg |= 0x01; /* disable MIC, LINE IN, enable LINE OUT */
    if (*gus).codec_flag != 0 || (*gus).ess_flag != 0 {
        (*gus).mix_cntrl_reg &= !1; /* enable LINE IN */
        (*gus).mix_cntrl_reg |= 4; /* enable MIC */
    }
    dma1 = (*gus).gf1.dma1;
    dma1 = dma1.abs();
    dma1 = DMAS[(dma1 & 7) as usize] as c_int;
    dma2 = (*gus).gf1.dma2;
    dma2 = dma2.abs();
    dma2 = DMAS[(dma2 & 7) as usize] as c_int;
    dma1 |= if (*gus).equal_dma != 0 { 0x40 } else { dma2 << 3 };

    if (dma1 & 7) == 0 || (dma2 & 7) == 0 {
        dev_err((*gus).card.as_ref().unwrap().dev, c"Error! DMA isn't defined.\n".as_ptr());
        return -EINVAL;
    }
    irq = (*gus).gf1.irq;
    irq = irq.abs();
    irq = IRQS[(irq & 0x0f) as usize] as c_int;
    if irq == 0 {
        dev_err((*gus).card.as_ref().unwrap().dev, c"Error! IRQ isn't defined.\n".as_ptr());
        return -EINVAL;
    }
    irq |= 0x40;
    // #if 0
    // card->mixer.mix_ctrl_reg |= 0x10;
    // #endif

    // scoped_guard(spinlock_irqsave, &gus->reg_lock)
    {
        outb(5, gusp(gus, REGCNTRLS));
        outb((*gus).mix_cntrl_reg, gusp(gus, MIXCNTRLREG));
        outb(0x00, gusp(gus, IRQDMACNTRLREG));
        outb(0, gusp(gus, REGCNTRLS));
    }

    udelay(100);

    // scoped_guard(spinlock_irqsave, &gus->reg_lock)
    {
        outb(0x00 | (*gus).mix_cntrl_reg, gusp(gus, MIXCNTRLREG));
        outb(dma1 as u8, gusp(gus, IRQDMACNTRLREG));
        if latches != 0 {
            outb(0x40 | (*gus).mix_cntrl_reg, gusp(gus, MIXCNTRLREG));
            outb(irq as u8, gusp(gus, IRQDMACNTRLREG));
        }
    }

    udelay(100);

    // scoped_guard(spinlock_irqsave, &gus->reg_lock)
    {
        outb(0x00 | (*gus).mix_cntrl_reg, gusp(gus, MIXCNTRLREG));
        outb(dma1 as u8, gusp(gus, IRQDMACNTRLREG));
        if latches != 0 {
            outb(0x40 | (*gus).mix_cntrl_reg, gusp(gus, MIXCNTRLREG));
            outb(irq as u8, gusp(gus, IRQDMACNTRLREG));
        }
    }

    snd_gf1_delay(gus);

    if latches != 0 {
        (*gus).mix_cntrl_reg |= 0x08; /* enable latches */
    } else {
        (*gus).mix_cntrl_reg &= !0x08; /* disable latches */
    }
    // scoped_guard(spinlock_irqsave, &gus->reg_lock)
    {
        outb((*gus).mix_cntrl_reg, gusp(gus, MIXCNTRLREG));
        outb(0, gusp(gus, GF1PAGE));
    }

    0
}

unsafe fn snd_gus_check_version(gus: *mut snd_gus_card) -> c_int {
    let mut val: u8 = 0;
    let mut rev: u8 = 0;
    let card: *mut snd_card;

    card = (*gus).card;
    // scoped_guard(spinlock_irqsave, &gus->reg_lock)
    {
        outb(0x20, gusp(gus, REGCNTRLS));
        val = inb(gusp(gus, REGCNTRLS));
        rev = inb(gusp(gus, BOARDVERSION));
    }
    dev_dbg(
        (*card).dev,
        c"GF1 [0x%lx] init - val = 0x%x, rev = 0x%x\n".as_ptr(),
        (*gus).gf1.port,
        val as c_int,
        rev as c_int,
    );
    strscpy((*card).driver.as_mut_ptr(), c"GUS".as_ptr(), (*card).driver.len());
    strscpy(
        (*card).longname.as_mut_ptr(),
        c"Gravis UltraSound Classic (2.4)".as_ptr(),
        (*card).longname.len(),
    );
    if (val != 255 && (val & 0x06) != 0) || (rev >= 5 && rev != 255) {
        if rev >= 5 && rev <= 9 {
            (*gus).ics_flag = 1;
            if rev == 5 {
                (*gus).ics_flipped = 1;
            }
            (*card).longname[27] = b'3' as c_char;
            (*card).longname[29] = if rev == 5 { b'5' } else { b'7' } as c_char;
        }
        if rev >= 10 && rev != 255 {
            if rev >= 10 && rev <= 11 {
                strscpy((*card).driver.as_mut_ptr(), c"GUS MAX".as_ptr(), (*card).driver.len());
                strscpy(
                    (*card).longname.as_mut_ptr(),
                    c"Gravis UltraSound MAX".as_ptr(),
                    (*card).longname.len(),
                );
                (*gus).max_flag = 1;
            } else if rev == 0x30 {
                strscpy((*card).driver.as_mut_ptr(), c"GUS ACE".as_ptr(), (*card).driver.len());
                strscpy(
                    (*card).longname.as_mut_ptr(),
                    c"Gravis UltraSound Ace".as_ptr(),
                    (*card).longname.len(),
                );
                (*gus).ace_flag = 1;
            } else if rev == 0x50 {
                strscpy(
                    (*card).driver.as_mut_ptr(),
                    c"GUS Extreme".as_ptr(),
                    (*card).driver.len(),
                );
                strscpy(
                    (*card).longname.as_mut_ptr(),
                    c"Gravis UltraSound Extreme".as_ptr(),
                    (*card).longname.len(),
                );
                (*gus).ess_flag = 1;
            } else {
                dev_err(
                    (*card).dev,
                    c"unknown GF1 revision number at 0x%lx - 0x%x (0x%x)\n".as_ptr(),
                    (*gus).gf1.port,
                    rev as c_int,
                    val as c_int,
                );
                dev_err(
                    (*card).dev,
                    c"  please - report to <perex@perex.cz>\n".as_ptr(),
                );
            }
        }
    }
    strscpy(
        (*card).shortname.as_mut_ptr(),
        (*card).longname.as_ptr(),
        (*card).shortname.len(),
    );
    (*gus).uart_enable = 1; /* standard GUSes doesn't have midi uart trouble */
    snd_gus_init_control(gus);
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_gus_initialize(gus: *mut snd_gus_card) -> c_int {
    let mut err: c_int;

    if (*gus).interwave == 0 {
        err = snd_gus_check_version(gus);
        if err < 0 {
            dev_err((*(*gus).card).dev, c"version check failed\n".as_ptr());
            return err;
        }
        err = snd_gus_detect_memory(gus);
        if err < 0 {
            return err;
        }
    }
    err = snd_gus_init_dma_irq(gus, 1);
    if err < 0 {
        return err;
    }
    snd_gf1_start(gus);
    (*gus).initialized = 1;
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_gus_suspend(gus: *mut snd_gus_card) -> c_int {
    let mut err: c_int;

    if !(*gus).pcm.is_null() {
        err = snd_pcm_suspend_all((*gus).pcm);
        if err < 0 {
            return err;
        }
    }

    err = snd_gf1_suspend(gus);
    if err < 0 {
        return err;
    }

    snd_power_change_state((*gus).card, SNDRV_CTL_POWER_D3HOT);
    0
}
// EXPORT_SYMBOL(snd_gus_suspend);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_gus_resume(gus: *mut snd_gus_card) -> c_int {
    let mut err: c_int;

    err = snd_gus_init_dma_irq(gus, 1);
    if err < 0 {
        return err;
    }

    err = snd_gf1_resume(gus);
    if err < 0 {
        return err;
    }

    snd_power_change_state((*gus).card, SNDRV_CTL_POWER_D0);
    0
}
// EXPORT_SYMBOL(snd_gus_resume);

/* gus_io.c */
// EXPORT_SYMBOL(snd_gf1_delay);
// EXPORT_SYMBOL(snd_gf1_write8);
// EXPORT_SYMBOL(snd_gf1_look8);
// EXPORT_SYMBOL(snd_gf1_write16);
// EXPORT_SYMBOL(snd_gf1_look16);
// EXPORT_SYMBOL(snd_gf1_i_write8);
// EXPORT_SYMBOL(snd_gf1_i_look8);
// EXPORT_SYMBOL(snd_gf1_i_look16);
// EXPORT_SYMBOL(snd_gf1_dram_addr);
// EXPORT_SYMBOL(snd_gf1_write_addr);
// EXPORT_SYMBOL(snd_gf1_poke);
// EXPORT_SYMBOL(snd_gf1_peek);
/* gus_reset.c */
// EXPORT_SYMBOL(snd_gf1_alloc_voice);
// EXPORT_SYMBOL(snd_gf1_free_voice);
// EXPORT_SYMBOL(snd_gf1_ctrl_stop);
// EXPORT_SYMBOL(snd_gf1_stop_voice);
/* gus_mixer.c */
// EXPORT_SYMBOL(snd_gf1_new_mixer);
/* gus_pcm.c */
// EXPORT_SYMBOL(snd_gf1_pcm_new);
/* gus.c */
// EXPORT_SYMBOL(snd_gus_create);
// EXPORT_SYMBOL(snd_gus_initialize);
/* gus_irq.c */
// EXPORT_SYMBOL(snd_gus_interrupt);
/* gus_uart.c */
// EXPORT_SYMBOL(snd_gf1_rawmidi_new);
/* gus_dram.c */
// EXPORT_SYMBOL(snd_gus_dram_write);
// EXPORT_SYMBOL(snd_gus_dram_read);
/* gus_volume.c */
// EXPORT_SYMBOL(snd_gf1_lvol_to_gvol_raw);
// EXPORT_SYMBOL(snd_gf1_translate_freq);
/* gus_mem.c */
// EXPORT_SYMBOL(snd_gf1_mem_alloc);
// EXPORT_SYMBOL(snd_gf1_mem_xfree);
// EXPORT_SYMBOL(snd_gf1_mem_free);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
