// SPDX-License-Identifier: GPL-2.0-only
// Driver for CS4231 sound chips found on Sparcs.
// Copyright (C) 2002, 2008 David S. Miller <davem@davemloft.net>
//
// Based entirely upon drivers/sbus/audio/cs4231.c which is:
// Copyright (C) 1996, 1997, 1998 Derrick J Brashear (shadow@andrew.cmu.edu)
// and also sound/isa/cs423x/cs4231_lib.c which is:
// Copyright (c) by Jaroslav Kysela <perex@perex.cz>

// Linux kernel module includes, sound framework includes, and platform headers
// would be required from external dependencies: linux/module, sound/core, etc.

// Build-time configuration flags
const SBUS_SUPPORT: bool = cfg!(feature = "sbus_support");
const EBUS_SUPPORT: bool = cfg!(all(feature = "ebus_support", target_arch = "sparc64"));

static mut INDEX: [i32; 32] = [0; 32];
static mut ID: [&str; 32] = [""; 32];
static mut ENABLE: [bool; 32] = [true; 32];

static mut DEV: i32 = 0;

#[cfg(feature = "sbus_support")]
#[repr(C)]
struct SbusDmaInfo {
    lock: std::sync::SpinLock<()>,
    dir: i32,
    regs: *mut std::ffi::c_void,
}

#[repr(C)]
struct Cs4231DmaControl {
    prepare: unsafe extern "C" fn(*mut Cs4231DmaControl, i32),
    enable: unsafe extern "C" fn(*mut Cs4231DmaControl, i32),
    request: unsafe extern "C" fn(*mut Cs4231DmaControl, u64, usize) -> i32,
    address: unsafe extern "C" fn(*mut Cs4231DmaControl) -> u32,
    #[cfg(feature = "ebus_support")]
    ebus_info: EbusDmaInfo,
    #[cfg(feature = "sbus_support")]
    sbus_info: SbusDmaInfo,
}

#[repr(C)]
struct SndCs4231 {
    lock: std::sync::SpinLock<()>,
    port: *mut std::ffi::c_void,
    p_dma: Cs4231DmaControl,
    c_dma: Cs4231DmaControl,
    flags: u32,
    card: *mut std::ffi::c_void,
    pcm: *mut std::ffi::c_void,
    playback_substream: *mut std::ffi::c_void,
    p_periods_sent: u32,
    capture_substream: *mut std::ffi::c_void,
    c_periods_sent: u32,
    timer: *mut std::ffi::c_void,
    mode: u16,
    image: [u8; 32],
    mce_bit: i32,
    calibrate_mute: i32,
    mce_mutex: std::sync::Mutex<()>,
    open_mutex: std::sync::Mutex<()>,
    op: *mut std::ffi::c_void,
    irq: [u32; 2],
    regs_size: u32,
    next: *mut SndCs4231,
}

const CS4231_FLAG_EBUS: u32 = 0x00000001;
const CS4231_FLAG_PLAYBACK: u32 = 0x00000002;
const CS4231_FLAG_CAPTURE: u32 = 0x00000004;

const CS4231_MODE_NONE: u16 = 0x0000;
const CS4231_MODE_PLAY: u16 = 0x0001;
const CS4231_MODE_RECORD: u16 = 0x0002;
const CS4231_MODE_TIMER: u16 = 0x0004;
const CS4231_MODE_OPEN: u16 = CS4231_MODE_PLAY | CS4231_MODE_RECORD | CS4231_MODE_TIMER;

// SBUS DMA register defines
const APCCSR: usize = 0x10;
const APCCVA: usize = 0x20;
const APCCC: usize = 0x24;
const APCCNVA: usize = 0x28;
const APCCNC: usize = 0x2c;
const APCPVA: usize = 0x30;
const APCPC: usize = 0x34;
const APCPNVA: usize = 0x38;
const APCPNC: usize = 0x3c;

const APCVA: usize = 0x0;
const APCC: usize = 0x4;
const APCNVA: usize = 0x8;
const APCNC: usize = 0xc;
const APC_PLAY: usize = 0x30;
const APC_RECORD: usize = 0x20;

const APC_INT_PENDING: u32 = 0x800000;
const APC_PLAY_INT: u32 = 0x400000;
const APC_CAPT_INT: u32 = 0x200000;
const APC_GENL_INT: u32 = 0x100000;
const APC_XINT_ENA: u32 = 0x80000;
const APC_XINT_PLAY: u32 = 0x40000;
const APC_XINT_CAPT: u32 = 0x20000;
const APC_XINT_GENL: u32 = 0x10000;
const APC_XINT_EMPT: u32 = 0x8000;
const APC_XINT_PEMP: u32 = 0x4000;
const APC_XINT_PNVA: u32 = 0x2000;
const APC_XINT_PENA: u32 = 0x1000;
const APC_XINT_COVF: u32 = 0x800;
const APC_XINT_CNVA: u32 = 0x400;
const APC_XINT_CEMP: u32 = 0x200;
const APC_XINT_CENA: u32 = 0x100;
const APC_PPAUSE: u32 = 0x80;
const APC_CPAUSE: u32 = 0x40;
const APC_CDC_RESET: u32 = 0x20;
const APC_PDMA_READY: u32 = 0x08;
const APC_CDMA_READY: u32 = 0x04;
const APC_CHIP_RESET: u32 = 0x01;

const EBDMA_CSR: usize = 0x00;
const EBDMA_ADDR: usize = 0x04;
const EBDMA_COUNT: usize = 0x08;

static FREQ_BITS: [u8; 14] = [
    0x00, 0x0E, 0x00, 0x0E, 0x02, 0x02, 0x04, 0x06,
    0x04, 0x06, 0x0C, 0x08, 0x0A, 0x0C,
];

static RATES: [u32; 14] = [
    5510, 6620, 8000, 9600, 11025, 16000, 18900, 22050,
    27042, 32000, 33075, 37800, 44100, 48000
];

static SND_CS4231_ORIGINAL_IMAGE: [u8; 32] = [
    0x00, 0x00, 0x9f, 0x9f, 0x9f, 0x9f, 0xbf, 0xbf,
    0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x80, 0x01, 0x9f, 0x9f, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00,
];

// External declarations for Linux kernel functions and symbols
extern "C" {
    fn __cs4231_readb(cp: *mut SndCs4231, reg_addr: *mut std::ffi::c_void) -> u8;
    fn __cs4231_writeb(cp: *mut SndCs4231, val: u8, reg_addr: *mut std::ffi::c_void);

    fn snd_cs4231_xrate(runtime: *mut std::ffi::c_void) -> i32;
    fn snd_pcm_hw_constraint_list(
        runtime: *mut std::ffi::c_void,
        cond: u32,
        var: u32,
        list: *const std::ffi::c_void,
    ) -> i32;

    fn snd_pcm_lib_period_bytes(substream: *mut std::ffi::c_void) -> u32;
    fn snd_pcm_substream_chip(substream: *mut std::ffi::c_void) -> *mut SndCs4231;
    fn snd_pcm_group_for_each_entry(
        s: *mut *mut std::ffi::c_void,
        substream: *mut std::ffi::c_void,
    );
    fn snd_pcm_trigger_done(s: *mut std::ffi::c_void, substream: *mut std::ffi::c_void);

    fn snd_timer_chip(timer: *mut std::ffi::c_void) -> *mut SndCs4231;
    fn snd_timer_interrupt(timer: *mut std::ffi::c_void, ticks: u32);

    fn snd_kcontrol_chip(kcontrol: *mut std::ffi::c_void) -> *mut SndCs4231;
    fn snd_ctl_enum_info(
        uinfo: *mut std::ffi::c_void,
        channels: u32,
        items: u32,
        texts: *const *const std::ffi::c_char,
    ) -> i32;

    fn snd_pcm_new(
        card: *mut std::ffi::c_void,
        id: *const std::ffi::c_char,
        device: i32,
        playback_count: i32,
        capture_count: i32,
        pcm: *mut *mut std::ffi::c_void,
    ) -> i32;
    fn snd_pcm_set_ops(
        pcm: *mut std::ffi::c_void,
        stream: i32,
        ops: *const std::ffi::c_void,
    );
    fn snd_pcm_set_sync(substream: *mut std::ffi::c_void);
    fn snd_pcm_set_managed_buffer_all(
        pcm: *mut std::ffi::c_void,
        type_: i32,
        dev: *mut std::ffi::c_void,
        prealloc: usize,
        max: usize,
    );
    fn snd_pcm_period_elapsed(substream: *mut std::ffi::c_void);

    fn snd_timer_new(
        card: *mut std::ffi::c_void,
        id: *const std::ffi::c_char,
        tid: *const std::ffi::c_void,
        timer: *mut *mut std::ffi::c_void,
    ) -> i32;

    fn snd_card_new(
        dev: *mut std::ffi::c_void,
        idx: i32,
        xid: *const std::ffi::c_char,
        module: *mut std::ffi::c_void,
        extra_size: usize,
        card_ret: *mut *mut std::ffi::c_void,
    ) -> i32;
    fn snd_card_register(card: *mut std::ffi::c_void) -> i32;
    fn snd_card_free(card: *mut std::ffi::c_void);

    fn snd_device_new(
        card: *mut std::ffi::c_void,
        type_: i32,
        device_data: *mut std::ffi::c_void,
        ops: *const std::ffi::c_void,
    ) -> i32;

    fn snd_ctl_add(card: *mut std::ffi::c_void, kcontrol: *mut std::ffi::c_void) -> i32;
    fn snd_ctl_new1(
        kcontrolp: *const std::ffi::c_void,
        private_data: *mut std::ffi::c_void,
    ) -> *mut std::ffi::c_void;

    fn request_irq(
        irq: u32,
        handler: unsafe extern "C" fn(i32, *mut std::ffi::c_void) -> i32,
        flags: u32,
        name: *const std::ffi::c_char,
        dev: *mut std::ffi::c_void,
    ) -> i32;
    fn free_irq(irq: u32, dev_id: *mut std::ffi::c_void);

    fn of_ioremap(
        res: *const std::ffi::c_void,
        offset: u32,
        size: u32,
        name: *const std::ffi::c_char,
    ) -> *mut std::ffi::c_void;
    fn of_iounmap(res: *const std::ffi::c_void, virtual_: *mut std::ffi::c_void, size: u32);

    fn ebus_dma_register(info: *mut std::ffi::c_void) -> i32;
    fn ebus_dma_unregister(info: *mut std::ffi::c_void);
    fn ebus_dma_irq_enable(info: *mut std::ffi::c_void, on: i32) -> i32;
    fn ebus_dma_request(info: *mut std::ffi::c_void, bus_addr: u64, len: usize) -> i32;
    fn ebus_dma_enable(info: *mut std::ffi::c_void, on: i32);
    fn ebus_dma_prepare(info: *mut std::ffi::c_void, dir: i32);
    fn ebus_dma_addr(info: *mut std::ffi::c_void) -> u32;

    fn sbus_readb(addr: *const std::ffi::c_void) -> u8;
    fn sbus_writeb(val: u8, addr: *mut std::ffi::c_void);
    fn sbus_readl(addr: *const std::ffi::c_void) -> u32;
    fn sbus_writel(val: u32, addr: *mut std::ffi::c_void);

    fn readb(addr: *const std::ffi::c_void) -> u8;
    fn writeb(val: u8, addr: *mut std::ffi::c_void);

    fn memcpy(dest: *mut std::ffi::c_void, src: *const std::ffi::c_void, n: usize) -> *mut std::ffi::c_void;
    fn strscpy(dest: *mut std::ffi::c_char, src: *const std::ffi::c_char, count: usize) -> isize;
    fn sprintf(s: *mut std::ffi::c_char, format: *const std::ffi::c_char, ...) -> i32;

    fn udelay(usecs: u32);
    fn msleep(msecs: u32);
    fn mdelay(msecs: u32);

    fn jiffies_to_msecs(j: u64) -> u32;
    fn msecs_to_jiffies(msecs: u32) -> u64;
    fn time_before(a: u64, b: u64) -> bool;

    static jiffies: u64;
}

const CS4231_PLAYBACK_ENABLE: i32 = 0x0001;
const CS4231_RECORD_ENABLE: i32 = 0x0002;

unsafe fn cs4231_u(chip: *mut SndCs4231, x: usize) -> *mut std::ffi::c_void {
    ((*chip).port as usize + (x << 2)) as *mut std::ffi::c_void
}

#[repr(C)]
#[derive(Copy, Clone)]
struct EbusDmaInfo {
    // Placeholder for ebus_dma_info structure from external dependency
    regs: *mut std::ffi::c_void,
    lock: std::sync::SpinLock<()>,
}

unsafe fn __cs4231_readb_impl(cp: *mut SndCs4231, reg_addr: *mut std::ffi::c_void) -> u8 {
    if (*cp).flags & CS4231_FLAG_EBUS != 0 {
        readb(reg_addr as *const std::ffi::c_void)
    } else {
        sbus_readb(reg_addr as *const std::ffi::c_void)
    }
}

unsafe fn __cs4231_writeb_impl(cp: *mut SndCs4231, val: u8, reg_addr: *mut std::ffi::c_void) {
    if (*cp).flags & CS4231_FLAG_EBUS != 0 {
        writeb(val, reg_addr);
    } else {
        sbus_writeb(val, reg_addr);
    }
}

unsafe fn snd_cs4231_ready(chip: *mut SndCs4231) {
    let mut timeout = 250i32;
    loop {
        if timeout <= 0 {
            break;
        }
        let val = __cs4231_readb(chip, cs4231_u(chip, 0));
        if (val as i32 & 0x01) == 0 {
            break;
        }
        udelay(100);
        timeout -= 1;
    }
}

unsafe fn snd_cs4231_dout(chip: *mut SndCs4231, reg: u8, value: u8) {
    snd_cs4231_ready(chip);
    #[cfg(feature = "config_snd_debug")]
    if __cs4231_readb(chip, cs4231_u(chip, 0)) as i32 & 0x01 != 0 {
        // dev_dbg would be called here
    }
    __cs4231_writeb(chip, ((*chip).mce_bit as u8) | reg, cs4231_u(chip, 0));
    std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::Release);
    __cs4231_writeb(chip, value, cs4231_u(chip, 1));
    std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
}

unsafe fn snd_cs4231_outm(chip: *mut SndCs4231, reg: u8, mask: u8, value: u8) {
    let tmp = ((*chip).image[reg as usize] & mask) | value;
    (*chip).image[reg as usize] = tmp;
    if (*chip).calibrate_mute == 0 {
        snd_cs4231_dout(chip, reg, tmp);
    }
}

unsafe fn snd_cs4231_out(chip: *mut SndCs4231, reg: u8, value: u8) {
    snd_cs4231_dout(chip, reg, value);
    (*chip).image[reg as usize] = value;
    std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
}

unsafe fn snd_cs4231_in(chip: *mut SndCs4231, reg: u8) -> u8 {
    snd_cs4231_ready(chip);
    #[cfg(feature = "config_snd_debug")]
    if __cs4231_readb(chip, cs4231_u(chip, 0)) as i32 & 0x01 != 0 {
        // dev_dbg would be called here
    }
    __cs4231_writeb(chip, ((*chip).mce_bit as u8) | reg, cs4231_u(chip, 0));
    std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
    __cs4231_readb(chip, cs4231_u(chip, 1))
}

unsafe fn snd_cs4231_busy_wait(chip: *mut SndCs4231) {
    let mut timeout = 5i32;
    while timeout > 0 {
        __cs4231_readb(chip, cs4231_u(chip, 0));
        timeout -= 1;
    }

    timeout = 500i32;
    loop {
        if timeout <= 0 {
            break;
        }
        let val = __cs4231_readb(chip, cs4231_u(chip, 0)) as i32;
        if (val & 0x01) == 0 {
            break;
        }
        msleep(1);
        timeout -= 1;
    }
}

unsafe fn snd_cs4231_mce_up(chip: *mut SndCs4231) {
    let _guard = std::sync::SpinLockGuard::new(&mut (*chip).lock);
    snd_cs4231_ready(chip);
    #[cfg(feature = "config_snd_debug")]
    if __cs4231_readb(chip, cs4231_u(chip, 0)) as i32 & 0x01 != 0 {
        // dev_dbg would be called here
    }
    (*chip).mce_bit |= 0x40;
    let timeout = __cs4231_readb(chip, cs4231_u(chip, 0)) as i32;
    if timeout == 0x80 {
        // dev_dbg would be called here
    }
    if (timeout & 0x40) == 0 {
        __cs4231_writeb(chip, (((*chip).mce_bit as u8) | (timeout as u8 & 0x1f)), cs4231_u(chip, 0));
    }
}

unsafe fn snd_cs4231_mce_down(chip: *mut SndCs4231) {
    snd_cs4231_busy_wait(chip);
    let _lock = std::sync::SpinLockGuard::new(&mut (*chip).lock);
    #[cfg(feature = "config_snd_debug")]
    if __cs4231_readb(chip, cs4231_u(chip, 0)) as i32 & 0x01 != 0 {
        // dev_dbg would be called here
    }
    (*chip).mce_bit &= !0x40;
    let reg = __cs4231_readb(chip, cs4231_u(chip, 0)) as i32;
    __cs4231_writeb(chip, (((*chip).mce_bit as u8) | (reg as u8 & 0x1f)), cs4231_u(chip, 0));
    if reg == 0x80 {
        // dev_dbg would be called here
    }
    if (reg & 0x40) == 0 {
        return;
    }

    let timeout = jiffies + msecs_to_jiffies(250);
    loop {
        drop(_lock);
        msleep(1);
        let _lock2 = std::sync::SpinLockGuard::new(&mut (*chip).lock);
        let reg_val = snd_cs4231_in(chip, 0x12);
        let reg_test = (reg_val as i32) & 0x20;
        if !(reg_test != 0 && time_before(jiffies, timeout)) {
            break;
        }
    }

    let reg = snd_cs4231_in(chip, 0x12) as i32;
    if reg & 0x20 != 0 {
        // dev_err would be called here
    }
}

unsafe fn snd_cs4231_advance_dma(
    dma_cont: *mut Cs4231DmaControl,
    substream: *mut std::ffi::c_void,
    periods_sent: *mut u32,
) {
    loop {
        let period_size = snd_pcm_lib_period_bytes(substream);
        let offset = period_size.wrapping_mul(*periods_sent);

        if period_size >= (1u32 << 24) {
            return;
        }

        if (*dma_cont).request(dma_cont, offset as u64, period_size as usize) != 0 {
            return;
        }
        *periods_sent = (*periods_sent).wrapping_add(1);
    }
}

unsafe fn cs4231_dma_trigger(substream: *mut std::ffi::c_void, what: i32, on: i32) {
    let chip = snd_pcm_substream_chip(substream);

    if what & CS4231_PLAYBACK_ENABLE != 0 {
        let dma_cont = &mut (*chip).p_dma;
        if on != 0 {
            (dma_cont.prepare)(dma_cont, 0);
            (dma_cont.enable)(dma_cont, 1);
            snd_cs4231_advance_dma(dma_cont, (*chip).playback_substream, &mut (*chip).p_periods_sent);
        } else {
            (dma_cont.enable)(dma_cont, 0);
        }
    }
    if what & CS4231_RECORD_ENABLE != 0 {
        let dma_cont = &mut (*chip).c_dma;
        if on != 0 {
            (dma_cont.prepare)(dma_cont, 1);
            (dma_cont.enable)(dma_cont, 1);
            snd_cs4231_advance_dma(dma_cont, (*chip).capture_substream, &mut (*chip).c_periods_sent);
        } else {
            (dma_cont.enable)(dma_cont, 0);
        }
    }
}

unsafe fn snd_cs4231_trigger(substream: *mut std::ffi::c_void, cmd: i32) -> i32 {
    let chip = snd_pcm_substream_chip(substream);

    match cmd {
        0x01 | 0x00 => {
            let mut what = 0i32;
            let mut s: *mut std::ffi::c_void = std::ptr::null_mut();

            snd_pcm_group_for_each_entry(&mut s, substream);

            if s == (*chip).playback_substream {
                what |= CS4231_PLAYBACK_ENABLE;
                snd_pcm_trigger_done(s, substream);
            } else if s == (*chip).capture_substream {
                what |= CS4231_RECORD_ENABLE;
                snd_pcm_trigger_done(s, substream);
            }

            let _guard = std::sync::SpinLockGuard::new(&mut (*chip).lock);
            if cmd == 0x01 {
                cs4231_dma_trigger(substream, what, 1);
                (*chip).image[0x0A] = ((*chip).image[0x0A] as i32 | what) as u8;
            } else {
                cs4231_dma_trigger(substream, what, 0);
                (*chip).image[0x0A] = ((*chip).image[0x0A] as i32 & !what) as u8;
            }
            snd_cs4231_out(chip, 0x0A, (*chip).image[0x0A]);
            0
        }
        _ => -22,
    }
}

unsafe fn snd_cs4231_get_rate(rate: u32) -> u8 {
    for i in 0..14 {
        if rate == RATES[i] {
            return FREQ_BITS[i];
        }
    }
    FREQ_BITS[13]
}

unsafe fn snd_cs4231_get_format(_chip: *mut SndCs4231, format: i32, channels: i32) -> u8 {
    let mut rformat = 0u8;
    match format {
        1 => rformat = 0x00,
        2 => rformat = 0x10,
        3 => rformat = 0x20,
        4 => rformat = 0x01,
        5 => rformat = 0x02,
        6 => rformat = 0x03,
        _ => rformat = 0x00,
    }
    if channels > 1 {
        rformat |= 0x80;
    }
    rformat
}

unsafe fn snd_cs4231_calibrate_mute(chip: *mut SndCs4231, mute_val: i32) {
    let mute = if mute_val != 0 { 1 } else { 0 };
    let _guard = std::sync::SpinLockGuard::new(&mut (*chip).lock);
    if (*chip).calibrate_mute == mute {
        return;
    }

    if mute == 0 {
        snd_cs4231_dout(chip, 0x00, (*chip).image[0x00]);
        snd_cs4231_dout(chip, 0x01, (*chip).image[0x01]);
        snd_cs4231_dout(chip, 0x0D, (*chip).image[0x0D]);
    }

    snd_cs4231_dout(chip, 0x02, if mute != 0 { 0x80 } else { (*chip).image[0x02] });
    snd_cs4231_dout(chip, 0x03, if mute != 0 { 0x80 } else { (*chip).image[0x03] });
    snd_cs4231_dout(chip, 0x04, if mute != 0 { 0x80 } else { (*chip).image[0x04] });
    snd_cs4231_dout(chip, 0x05, if mute != 0 { 0x80 } else { (*chip).image[0x05] });
    snd_cs4231_dout(chip, 0x06, if mute != 0 { 0x80 } else { (*chip).image[0x06] });
    snd_cs4231_dout(chip, 0x07, if mute != 0 { 0x80 } else { (*chip).image[0x07] });
    snd_cs4231_dout(chip, 0x08, if mute != 0 { 0x80 } else { (*chip).image[0x08] });
    snd_cs4231_dout(chip, 0x09, if mute != 0 { 0x80 } else { (*chip).image[0x09] });
    snd_cs4231_dout(chip, 0x0E, if mute != 0 { 0xc0 } else { (*chip).image[0x0E] });
    (*chip).calibrate_mute = mute;
}

unsafe fn snd_cs4231_playback_format(
    chip: *mut SndCs4231,
    _params: *mut std::ffi::c_void,
    pdfr: u8,
) {
    let _guard = std::sync::Mutex::new();
    snd_cs4231_calibrate_mute(chip, 1);
    snd_cs4231_mce_up(chip);

    let _lock_guard = std::sync::SpinLockGuard::new(&mut (*chip).lock);
    if (*chip).image[0x0A] as i32 & 0x02 != 0 {
        let val = ((pdfr & 0xf0) as i32 | ((*chip).image[0x0C] as i32 & 0x0f)) as u8;
        snd_cs4231_out(chip, 0x0B, val);
    } else {
        snd_cs4231_out(chip, 0x0B, pdfr);
    }
    drop(_lock_guard);

    snd_cs4231_mce_down(chip);
    snd_cs4231_calibrate_mute(chip, 0);
}

unsafe fn snd_cs4231_capture_format(
    chip: *mut SndCs4231,
    _params: *mut std::ffi::c_void,
    cdfr: u8,
) {
    let _guard = std::sync::Mutex::new();
    snd_cs4231_calibrate_mute(chip, 1);
    snd_cs4231_mce_up(chip);

    let mut flags = 0u32;
    if !((*chip).image[0x0A] as i32 & 0x01 != 0) {
        snd_cs4231_out(chip, 0x0B, (((*chip).image[0x0B] as i32 & 0xf0) | (cdfr as i32 & 0x0f)) as u8);
        snd_cs4231_mce_down(chip);
        snd_cs4231_mce_up(chip);
    }
    snd_cs4231_out(chip, 0x0C, cdfr);

    snd_cs4231_mce_down(chip);
    snd_cs4231_calibrate_mute(chip, 0);
}

unsafe fn snd_cs4231_timer_resolution(_timer: *mut std::ffi::c_void) -> u32 {
    let chip = snd_timer_chip(_timer);
    if (*chip).image[0x0B] as i32 & 1 != 0 { 9969 } else { 9920 }
}

unsafe fn snd_cs4231_timer_start(timer: *mut std::ffi::c_void) -> i32 {
    let chip = snd_timer_chip(timer);
    let _guard = std::sync::SpinLockGuard::new(&mut (*chip).lock);
    let ticks = 0u32;

    if ((*chip).image[0x11] as i32 & 0x80) == 0 ||
       ((ticks >> 8) as u8 != (*chip).image[0x0F]) ||
       (ticks as u8 != (*chip).image[0x10]) {
        snd_cs4231_out(chip, 0x0F, ((ticks >> 8) as u8));
        (*chip).image[0x0F] = ((ticks >> 8) as u8);
        snd_cs4231_out(chip, 0x10, (ticks as u8));
        (*chip).image[0x10] = (ticks as u8);
        snd_cs4231_out(chip, 0x11, ((*chip).image[0x11] as i32 | 0x80) as u8);
        (*chip).image[0x11] = ((*chip).image[0x11] as i32 | 0x80) as u8;
    }
    0
}

unsafe fn snd_cs4231_timer_stop(timer: *mut std::ffi::c_void) -> i32 {
    let chip = snd_timer_chip(timer);
    let _guard = std::sync::SpinLockGuard::new(&mut (*chip).lock);
    (*chip).image[0x11] = ((*chip).image[0x11] as i32 & !0x80) as u8;
    snd_cs4231_out(chip, 0x11, (*chip).image[0x11]);
    0
}

unsafe fn snd_cs4231_init(chip: *mut SndCs4231) {
    snd_cs4231_mce_down(chip);
    snd_cs4231_mce_up(chip);

    let _lock = std::sync::SpinLockGuard::new(&mut (*chip).lock);
    (*chip).image[0x0A] = ((*chip).image[0x0A] as i32 & !(0x02 | 0x08 | 0x04 | 0x20 | 0x01)) as u8;
    (*chip).image[0x0A] = ((*chip).image[0x0A] as i32 | 0x40) as u8;
    snd_cs4231_out(chip, 0x0A, (*chip).image[0x0A]);
    drop(_lock);

    snd_cs4231_mce_down(chip);
    snd_cs4231_mce_up(chip);

    let _lock = std::sync::SpinLockGuard::new(&mut (*chip).lock);
    snd_cs4231_out(chip, 0x11, (*chip).image[0x11]);
    drop(_lock);

    snd_cs4231_mce_down(chip);

    let _lock = std::sync::SpinLockGuard::new(&mut (*chip).lock);
    snd_cs4231_out(chip, 0x12, (*chip).image[0x12]);
    drop(_lock);

    snd_cs4231_mce_up(chip);

    let _lock = std::sync::SpinLockGuard::new(&mut (*chip).lock);
    snd_cs4231_out(chip, 0x0B, (*chip).image[0x0B]);
    drop(_lock);

    snd_cs4231_mce_down(chip);
    snd_cs4231_mce_up(chip);

    let _lock = std::sync::SpinLockGuard::new(&mut (*chip).lock);
    snd_cs4231_out(chip, 0x0C, (*chip).image[0x0C]);
    drop(_lock);

    snd_cs4231_mce_down(chip);
}

unsafe fn snd_cs4231_open(chip: *mut SndCs4231, mode: u32) -> i32 {
    let _guard = std::sync::Mutex::new();
    if (*chip).mode as u32 & mode != 0 {
        return -11;
    }
    if (*chip).mode as u32 & CS4231_MODE_OPEN as u32 != 0 {
        (*chip).mode = ((*chip).mode as u32 | mode) as u16;
        return 0;
    }

    let _lock = std::sync::SpinLockGuard::new(&mut (*chip).lock);
    snd_cs4231_out(chip, 0x13, 0x28);
    snd_cs4231_out(chip, 0x13, 0);
    __cs4231_writeb(chip, 0, cs4231_u(chip, 0x04));
    __cs4231_writeb(chip, 0, cs4231_u(chip, 0x04));

    snd_cs4231_out(chip, 0x13, 0x28);
    snd_cs4231_out(chip, 0x13, 0);

    (*chip).mode = mode as u16;
    0
}

unsafe fn snd_cs4231_close(chip: *mut SndCs4231, mode: u32) {
    let _guard = std::sync::Mutex::new();
    (*chip).mode = ((*chip).mode as u32 & !mode) as u16;
    if (*chip).mode as u32 & CS4231_MODE_OPEN as u32 != 0 {
        return;
    }
    snd_cs4231_calibrate_mute(chip, 1);

    let mut flags = 0u32;
    let _lock = std::sync::SpinLockGuard::new(&mut (*chip).lock);
    snd_cs4231_out(chip, 0x13, 0);
    __cs4231_writeb(chip, 0, cs4231_u(chip, 0x04));
    __cs4231_writeb(chip, 0, cs4231_u(chip, 0x04));

    if (*chip).image[0x0A] as i32 & (0x02 | 0x08 | 0x04 | 0x20) != 0 {
        drop(_lock);
        snd_cs4231_mce_up(chip);
        let _lock2 = std::sync::SpinLockGuard::new(&mut (*chip).lock);
        (*chip).image[0x0A] = ((*chip).image[0x0A] as i32 & !(0x02 | 0x08 | 0x04 | 0x20)) as u8;
        snd_cs4231_out(chip, 0x0A, (*chip).image[0x0A]);
        drop(_lock2);
        snd_cs4231_mce_down(chip);
    }

    snd_cs4231_out(chip, 0x13, 0);
    __cs4231_writeb(chip, 0, cs4231_u(chip, 0x04));
    __cs4231_writeb(chip, 0, cs4231_u(chip, 0x04));

    snd_cs4231_calibrate_mute(chip, 0);
    (*chip).mode = 0;
}

unsafe fn snd_cs4231_timer_open(_timer: *mut std::ffi::c_void) -> i32 {
    let chip = snd_timer_chip(_timer);
    snd_cs4231_open(chip, CS4231_MODE_TIMER as u32);
    0
}

unsafe fn snd_cs4231_timer_close(_timer: *mut std::ffi::c_void) -> i32 {
    let chip = snd_timer_chip(_timer);
    snd_cs4231_close(chip, CS4231_MODE_TIMER as u32);
    0
}

unsafe fn snd_cs4231_playback_hw_params(substream: *mut std::ffi::c_void, _hw_params: *mut std::ffi::c_void) -> i32 {
    let chip = snd_pcm_substream_chip(substream);
    let _new_pdfr = snd_cs4231_get_format(chip, 4, 1);
    snd_cs4231_playback_format(chip, _hw_params, _new_pdfr);
    0
}

unsafe fn snd_cs4231_playback_prepare(substream: *mut std::ffi::c_void) -> i32 {
    let chip = snd_pcm_substream_chip(substream);

    let _guard = std::sync::SpinLockGuard::new(&mut (*chip).lock);
    (*chip).image[0x0A] = ((*chip).image[0x0A] as i32 & !(0x02 | 0x08)) as u8;
    (*chip).p_periods_sent = 0;
    0
}

unsafe fn snd_cs4231_capture_hw_params(substream: *mut std::ffi::c_void, _hw_params: *mut std::ffi::c_void) -> i32 {
    let chip = snd_pcm_substream_chip(substream);
    let _new_cdfr = snd_cs4231_get_format(chip, 4, 1);
    snd_cs4231_capture_format(chip, _hw_params, _new_cdfr);
    0
}

unsafe fn snd_cs4231_capture_prepare(substream: *mut std::ffi::c_void) -> i32 {
    let chip = snd_pcm_substream_chip(substream);

    let _guard = std::sync::SpinLockGuard::new(&mut (*chip).lock);
    (*chip).image[0x0A] = ((*chip).image[0x0A] as i32 & !(0x04 | 0x20)) as u8;
    (*chip).c_periods_sent = 0;
    0
}

unsafe fn snd_cs4231_overrange(chip: *mut SndCs4231) {
    let _guard = std::sync::SpinLockGuard::new(&mut (*chip).lock);
    let res = snd_cs4231_in(chip, 0x12);
    if (res as i32) & (0x08 | 0x02) != 0 {
        // (*capture_substream).runtime.overrange += 1
    }
}

unsafe fn snd_cs4231_play_callback(chip: *mut SndCs4231) {
    if (*chip).image[0x0A] as i32 & 0x02 != 0 {
        snd_pcm_period_elapsed((*chip).playback_substream);
        snd_cs4231_advance_dma(&mut (*chip).p_dma, (*chip).playback_substream, &mut (*chip).p_periods_sent);
    }
}

unsafe fn snd_cs4231_capture_callback(chip: *mut SndCs4231) {
    if (*chip).image[0x0A] as i32 & 0x04 != 0 {
        snd_pcm_period_elapsed((*chip).capture_substream);
        snd_cs4231_advance_dma(&mut (*chip).c_dma, (*chip).capture_substream, &mut (*chip).c_periods_sent);
    }
}

unsafe fn snd_cs4231_playback_pointer(substream: *mut std::ffi::c_void) -> usize {
    let chip = snd_pcm_substream_chip(substream);
    let dma_cont = &mut (*chip).p_dma;

    if (*chip).image[0x0A] as i32 & 0x02 == 0 {
        return 0;
    }
    let ptr = (dma_cont.address)(dma_cont) as usize;
    if ptr != 0 {
        return ptr;
    }
    0
}

unsafe fn snd_cs4231_capture_pointer(substream: *mut std::ffi::c_void) -> usize {
    let chip = snd_pcm_substream_chip(substream);
    let dma_cont = &mut (*chip).c_dma;

    if (*chip).image[0x0A] as i32 & 0x04 == 0 {
        return 0;
    }
    let ptr = (dma_cont.address)(dma_cont) as usize;
    if ptr != 0 {
        return ptr;
    }
    0
}

unsafe fn snd_cs4231_probe(chip: *mut SndCs4231) -> i32 {
    let mut id = 0i32;
    let mut vers = 0i32;

    for _i in 0..50 {
        std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
        if (__cs4231_readb(chip, cs4231_u(chip, 0)) as i32 & 0x01) != 0 {
            msleep(2);
        } else {
            let _guard = std::sync::SpinLockGuard::new(&mut (*chip).lock);
            snd_cs4231_out(chip, 0x14, 0x40);
            id = snd_cs4231_in(chip, 0x14) as i32 & 0x0f;
            vers = snd_cs4231_in(chip, 0x19) as i32;
            if id == 0x0a {
                break;
            }
        }
    }

    if id != 0x0a {
        return -19;
    }

    let _guard = std::sync::SpinLockGuard::new(&mut (*chip).lock);
    __cs4231_readb(chip, cs4231_u(chip, 0x04));
    __cs4231_writeb(chip, 0, cs4231_u(chip, 0x04));
    std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);

    (*chip).image[0x14] = 0x40;
    (*chip).image[0x0A] = ((*chip).image[0x0A] as i32 & !0x10) as u8;
    (*chip).image[0x11] = 0x80;
    (*chip).image[0x12] = 0x01;
    if vers & 0x20 != 0 {
        (*chip).image[0x12] = ((*chip).image[0x12] as i32 | 0x02) as u8;
    }

    snd_cs4231_mce_down(chip);

    let _lock2 = std::sync::SpinLockGuard::new(&mut (*chip).lock);
    let ptr = &mut (*chip).image as *mut u8;
    for i in 0..32 {
        snd_cs4231_out(chip, i as u8, *ptr.add(i));
    }
    drop(_lock2);

    snd_cs4231_mce_up(chip);
    snd_cs4231_mce_down(chip);

    mdelay(2);
    0
}

unsafe fn snd_cs4231_info_mux(_kcontrol: *mut std::ffi::c_void, _uinfo: *mut std::ffi::c_void) -> i32 {
    0
}

unsafe fn snd_cs4231_get_mux(_kcontrol: *mut std::ffi::c_void, _ucontrol: *mut std::ffi::c_void) -> i32 {
    0
}

unsafe fn snd_cs4231_put_mux(_kcontrol: *mut std::ffi::c_void, _ucontrol: *mut std::ffi::c_void) -> i32 {
    0
}

unsafe fn snd_cs4231_info_single(_kcontrol: *mut std::ffi::c_void, _uinfo: *mut std::ffi::c_void) -> i32 {
    0
}

unsafe fn snd_cs4231_get_single(_kcontrol: *mut std::ffi::c_void, _ucontrol: *mut std::ffi::c_void) -> i32 {
    0
}

unsafe fn snd_cs4231_put_single(_kcontrol: *mut std::ffi::c_void, _ucontrol: *mut std::ffi::c_void) -> i32 {
    0
}

unsafe fn snd_cs4231_info_double(_kcontrol: *mut std::ffi::c_void, _uinfo: *mut std::ffi::c_void) -> i32 {
    0
}

unsafe fn snd_cs4231_get_double(_kcontrol: *mut std::ffi::c_void, _ucontrol: *mut std::ffi::c_void) -> i32 {
    0
}

unsafe fn snd_cs4231_put_double(_kcontrol: *mut std::ffi::c_void, _ucontrol: *mut std::ffi::c_void) -> i32 {
    0
}

unsafe fn snd_cs4231_pcm(card: *mut std::ffi::c_void) -> i32 {
    0
}

unsafe fn snd_cs4231_mixer(card: *mut std::ffi::c_void) -> i32 {
    0
}

unsafe fn snd_cs4231_timer(card: *mut std::ffi::c_void) -> i32 {
    0
}

unsafe fn cs4231_attach_begin(op: *mut std::ffi::c_void, rcard: *mut *mut std::ffi::c_void) -> i32 {
    *rcard = std::ptr::null_mut();
    0
}

unsafe fn cs4231_attach_finish(card: *mut std::ffi::c_void) -> i32 {
    0
}

#[cfg(feature = "sbus_support")]
unsafe fn snd_cs4231_sbus_free(chip: *mut SndCs4231) -> i32 {
    0
}

#[cfg(feature = "sbus_support")]
unsafe fn snd_cs4231_sbus_dev_free(_device: *mut std::ffi::c_void) -> i32 {
    0
}

#[cfg(feature = "sbus_support")]
unsafe fn snd_cs4231_sbus_create(card: *mut std::ffi::c_void, _op: *mut std::ffi::c_void, _dev: i32) -> i32 {
    0
}

#[cfg(feature = "sbus_support")]
unsafe fn cs4231_sbus_probe(_op: *mut std::ffi::c_void) -> i32 {
    0
}

#[cfg(feature = "ebus_support")]
unsafe fn snd_cs4231_ebus_play_callback(_p: *mut std::ffi::c_void, _event: i32, _cookie: *mut std::ffi::c_void) {
}

#[cfg(feature = "ebus_support")]
unsafe fn snd_cs4231_ebus_capture_callback(_p: *mut std::ffi::c_void, _event: i32, _cookie: *mut std::ffi::c_void) {
}

#[cfg(feature = "ebus_support")]
unsafe fn snd_cs4231_ebus_free(chip: *mut SndCs4231) -> i32 {
    0
}

#[cfg(feature = "ebus_support")]
unsafe fn snd_cs4231_ebus_dev_free(_device: *mut std::ffi::c_void) -> i32 {
    0
}

#[cfg(feature = "ebus_support")]
unsafe fn snd_cs4231_ebus_create(card: *mut std::ffi::c_void, _op: *mut std::ffi::c_void, _dev: i32) -> i32 {
    0
}

#[cfg(feature = "ebus_support")]
unsafe fn cs4231_ebus_probe(_op: *mut std::ffi::c_void) -> i32 {
    0
}

unsafe fn cs4231_probe(op: *mut std::ffi::c_void) -> i32 {
    #[cfg(feature = "ebus_support")]
    {
        // if of_node_name_eq((*op).dev.of_node.parent, "ebus")
        //     return cs4231_ebus_probe(op);
    }
    #[cfg(feature = "sbus_support")]
    {
        // if of_node_name_eq((*op).dev.of_node.parent, "sbus") ||
        //    of_node_name_eq((*op).dev.of_node.parent, "sbi")
        //     return cs4231_sbus_probe(op);
    }
    -19
}

unsafe fn cs4231_remove(_op: *mut std::ffi::c_void) {
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
