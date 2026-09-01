// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Support for Digigram Lola PCI-e boards
 *
 *  Copyright (c) 2011 Takashi Iwai <tiwai@suse.de>
 */

/* Includes from the C source are external dependencies:
 * linux/kernel.h, linux/init.h, linux/module.h, linux/dma-mapping.h,
 * linux/delay.h, linux/interrupt.h, linux/slab.h, linux/pci.h,
 * sound/core.h, sound/control.h, sound/pcm.h, sound/initval.h, "lola.h"
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::ptr;

type Bool = bool;
type U32 = u32;
type Le32 = u32;
type IrqReturnT = c_int;

#[repr(C)]
pub struct lola {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    pub private_data: *mut c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_card)>,
    pub dev: *mut c_void,
    pub sync_irq: c_int,
    pub driver: [c_char; 16],
    pub shortname: [c_char; 32],
    pub longname: [c_char; 80],
    pub mixername: [c_char; 80],
}

#[repr(C)]
pub struct pci_dev {
    pub dev: c_void,
    pub irq: c_int,
}

#[repr(C)]
pub struct pci_device_id {
    pub vendor: u32,
    pub device: u32,
    pub subvendor: u32,
    pub subdevice: u32,
    pub class: u32,
    pub class_mask: u32,
    pub driver_data: usize,
}

#[repr(C)]
pub struct pci_driver {
    pub name: *const c_char,
    pub id_table: *const pci_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> c_int>,
}

unsafe extern "C" {
    static mut jiffies: c_ulong;

    static KBUILD_MODNAME: *const c_char;
    static THIS_MODULE: *mut c_void;

    fn pr_debug(fmt: *const c_char, ...);
    fn dev_warn(dev: *mut c_void, fmt: *const c_char, ...);
    fn dev_err(dev: *mut c_void, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut c_void, fmt: *const c_char, ...);

    fn spin_lock(lock: *mut c_void);
    fn spin_unlock(lock: *mut c_void);
    fn spin_lock_irq(lock: *mut c_void);
    fn spin_unlock_irq(lock: *mut c_void);
    fn spin_lock_irqsave(lock: *mut c_void) -> c_ulong;
    fn spin_unlock_irqrestore(lock: *mut c_void, flags: c_ulong);
    fn spin_lock_init(lock: *mut c_void);
    fn mutex_init(lock: *mut c_void);

    fn smp_wmb();
    fn msecs_to_jiffies(msecs: c_uint) -> c_ulong;
    fn time_after(a: c_ulong, b: c_ulong) -> bool;
    fn time_before(a: c_ulong, b: c_ulong) -> bool;
    fn udelay(usecs: c_ulong);
    fn msleep(msecs: c_uint);
    fn cond_resched();

    fn cpu_to_le32(x: u32) -> Le32;
    fn le32_to_cpu(x: Le32) -> u32;
    fn upper_32_bits(x: u64) -> u32;

    fn lola_readb(chip: *mut lola, bar: c_int, reg: c_int) -> u8;
    fn lola_readw(chip: *mut lola, bar: c_int, reg: c_int) -> u16;
    fn lola_readl(chip: *mut lola, bar: c_int, reg: c_int) -> u32;
    fn lola_writeb(chip: *mut lola, bar: c_int, reg: c_int, val: u8);
    fn lola_writew(chip: *mut lola, bar: c_int, reg: c_int, val: u32);
    fn lola_writel(chip: *mut lola, bar: c_int, reg: c_int, val: u32);
    fn lola_dsd_read(chip: *mut lola, stream: c_int, reg: c_int) -> u32;
    fn lola_dsd_write(chip: *mut lola, stream: c_int, reg: c_int, val: u32);

    fn lola_update_ext_clock_freq(chip: *mut lola, res: u32);
    fn lola_pcm_update(chip: *mut lola, pcm: *mut c_void, notify: u32);
    fn lola_set_granularity(chip: *mut lola, granularity: c_int, force: bool) -> c_int;
    fn lola_set_clock_index(chip: *mut lola, index: c_int) -> c_int;
    fn lola_enable_clock_events(chip: *mut lola) -> c_int;
    fn lola_setup_all_analog_gains(chip: *mut lola, dir: c_int, update: bool) -> c_int;
    fn lola_set_src_config(chip: *mut lola, mask: u32, update: bool) -> c_int;
    fn lola_read_param(chip: *mut lola, nid: c_uint, param: c_uint, val: *mut c_uint) -> c_int;
    fn lola_init_pcm(chip: *mut lola, dir: c_int, nid: *mut c_int) -> c_int;
    fn lola_init_pins(chip: *mut lola, dir: c_int, nid: *mut c_int) -> c_int;
    fn lola_init_clock_widget(chip: *mut lola, nid: c_int) -> c_int;
    fn lola_init_mixer_widget(chip: *mut lola, nid: c_int) -> c_int;
    fn lola_free_mixer(chip: *mut lola);
    fn lola_create_pcm(chip: *mut lola) -> c_int;
    fn lola_create_mixer(chip: *mut lola) -> c_int;
    fn lola_proc_debug_new(chip: *mut lola);

    fn snd_devm_alloc_pages(dev: *mut c_void, ty: c_int, size: usize) -> *mut c_void;
    fn pcim_enable_device(pci: *mut pci_dev) -> c_int;
    fn pcim_iomap_region(pci: *mut pci_dev, bar: c_int, name: *const c_char) -> *mut c_void;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn pci_resource_start(pci: *mut pci_dev, bar: c_int) -> c_ulong;
    fn pci_set_master(pci: *mut pci_dev);
    fn devm_request_irq(
        dev: *mut c_void,
        irq: c_int,
        handler: unsafe extern "C" fn(c_int, *mut c_void) -> IrqReturnT,
        flags: c_ulong,
        name: *const c_char,
        dev_id: *mut c_void,
    ) -> c_int;
    fn strscpy(dst: *mut c_char, src: *const c_char, size: usize) -> isize;
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn snd_devm_card_new(
        dev: *mut c_void,
        idx: c_int,
        id: *mut c_char,
        module: *mut c_void,
        extra_size: usize,
        card_ret: *mut *mut snd_card,
    ) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn snd_card_free_on_error(dev: *mut c_void, ret: c_int) -> c_int;
    fn pci_set_drvdata(pci: *mut pci_dev, data: *mut c_void);
    fn IRQ_RETVAL(x: c_int) -> IrqReturnT;
}

type c_uint = u32;

const SNDRV_CARDS: usize = 8;
const SNDRV_DEFAULT_IDX: c_int = -1;
const SNDRV_DEFAULT_STR: *mut c_char = ptr::null_mut();
const SNDRV_DEFAULT_ENABLE_PNP: bool = true;
const LOLA_GRANULARITY_MAX: c_int = 32;
const LOLA_GRANULARITY_MIN: c_int = 8;
const EIO: c_int = 5;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const EBUSY: c_int = 16;
const ENODEV: c_int = 19;
const ENOENT: c_int = 2;
const CAPT: usize = 0;
const PLAY: usize = 1;
const MAX_STREAM_IN_COUNT: c_int = 16;
const MAX_STREAM_OUT_COUNT: c_int = 16;
const MAX_AUDIO_INOUT_COUNT: c_uint = 16;
const LOLA_CORB_ENTRIES: u32 = 256;
const PAGE_SIZE: usize = 4096;
const SNDRV_DMA_TYPE_DEV: c_int = 0;
const IRQF_SHARED: c_ulong = 0x80;
const BAR0: c_int = 0;
const BAR1: c_int = 1;
const GCTL: c_int = 0;
const BOARD_MODE: c_int = 0;
const CORBWP: c_int = 0;
const RIRBWP: c_int = 0;
const RIRBSTS: c_int = 0;
const CORBSTS: c_int = 0;
const DINTSTS: c_int = 0;
const DIINTSTS: c_int = 0;
const DOINTSTS: c_int = 0;
const STS: c_int = 0;
const DOINTCTL: c_int = 0;
const DIINTCTL: c_int = 0;
const DINTCTL: c_int = 0;
const RIRBCTL: c_int = 0;
const CORBCTL: c_int = 0;
const CORBLBASE: c_int = 0;
const CORBUBASE: c_int = 0;
const CORBSIZE: c_int = 0;
const CORBRP: c_int = 0;
const RIRBLBASE: c_int = 0;
const RIRBUBASE: c_int = 0;
const RIRBSIZE: c_int = 0;
const RINTCNT: c_int = 0;
const DEVER: c_int = 0;
const DRVNAME: *const c_char = c"Lola".as_ptr();
const LOLA_RIRB_EX_UNSOL_EV: u32 = 0;
const LOLA_RIRB_EX_ERROR: u32 = 0;
const LOLA_DSD_STS_DESE: u32 = 0;
const LOLA_DSD_STS_BCIS: u32 = 0;
const LOLA_DINT_CTRL: u32 = 0;
const LOLA_RIRB_INT_MASK: u8 = 0;
const LOLA_CORB_INT_MASK: u8 = 0;
const LOLA_DINT_FIFOERR: u32 = 0;
const LOLA_DINT_MUERR: u32 = 0;
const LOLA_GCTL_RESET: u32 = 0;
const LOLA_DINT_GLOBAL: u32 = 0;
const LOLA_RBRWP_CLR: u32 = 0;
const LOLA_RBCTL_DMA_EN: u8 = 0;
const LOLA_RBCTL_IRQ_EN: u8 = 0;
const LOLA_PAR_VENDOR_ID: c_uint = 0;
const LOLA_PAR_FUNCTION_TYPE: c_uint = 0;
const LOLA_PAR_SPECIFIC_CAPS: c_uint = 0;

unsafe extern "C" {
    fn lola_get_last_cmd_nid(chip: *mut lola) -> *mut c_uint;
    fn lola_get_last_verb(chip: *mut lola) -> *mut c_uint;
    fn lola_get_last_data(chip: *mut lola) -> *mut c_uint;
    fn lola_get_last_extdata(chip: *mut lola) -> *mut c_uint;
    fn lola_get_reg_lock(chip: *mut lola) -> *mut c_void;
    fn lola_get_rirb_cmds(chip: *mut lola) -> *mut u32;
    fn lola_get_corb_wp(chip: *mut lola) -> *mut u32;
    fn lola_get_corb_buf(chip: *mut lola) -> *mut Le32;
    fn lola_get_rirb_wp(chip: *mut lola) -> *mut u32;
    fn lola_get_rirb_rp(chip: *mut lola) -> *mut u32;
    fn lola_get_rirb_buf(chip: *mut lola) -> *mut Le32;
    fn lola_get_res(chip: *mut lola) -> *mut u32;
    fn lola_get_res_ex(chip: *mut lola) -> *mut u32;
    fn lola_get_polling_mode(chip: *mut lola) -> *mut c_int;
    fn lola_get_card(chip: *mut lola) -> *mut *mut snd_card;
    fn lola_get_pcm_num_streams(chip: *mut lola, dir: usize) -> *mut c_int;
    fn lola_get_pcm(chip: *mut lola, dir: usize) -> *mut c_void;
    fn lola_get_cold_reset(chip: *mut lola) -> *mut c_int;
    fn lola_get_rb(chip: *mut lola) -> *mut *mut c_void;
    fn lola_dma_addr(page: *mut c_void) -> u64;
    fn lola_dma_area(page: *mut c_void) -> *mut u8;
    fn lola_get_corb_addr(chip: *mut lola) -> *mut u64;
    fn lola_get_rirb_addr(chip: *mut lola) -> *mut u64;
    fn lola_get_granularity(chip: *mut lola) -> *mut c_int;
    fn lola_get_clock_cur_index(chip: *mut lola) -> *mut c_int;
    fn lola_get_input_src_mask(chip: *mut lola) -> *mut u32;
    fn lola_get_pin_num_pins(chip: *mut lola, dir: usize) -> *mut c_uint;
    fn lola_get_lola_caps(chip: *mut lola) -> *mut u32;
    fn lola_get_initialized(chip: *mut lola) -> *mut c_int;
    fn lola_get_pci(chip: *mut lola) -> *mut *mut pci_dev;
    fn lola_get_irq(chip: *mut lola) -> *mut c_int;
    fn lola_get_open_mutex(chip: *mut lola) -> *mut c_void;
    fn lola_get_sample_rate_max(chip: *mut lola) -> *mut c_int;
    fn lola_get_sample_rate_min(chip: *mut lola) -> *mut c_int;
    fn lola_get_bar_remap_addr(chip: *mut lola, bar: usize) -> *mut *mut c_void;
    fn lola_get_bar_addr(chip: *mut lola, bar: usize) -> *mut c_ulong;
    fn lola_get_version(chip: *mut lola) -> *mut u32;
    fn LOLA_AFG_INPUT_PIN_COUNT(caps: u32) -> c_uint;
    fn LOLA_AFG_OUTPUT_PIN_COUNT(caps: u32) -> c_uint;
    fn LOLA_AFG_CLOCK_WIDGET_PRESENT(caps: u32) -> bool;
    fn LOLA_AFG_MIXER_WIDGET_PRESENT(caps: u32) -> bool;
}

/* Standard options */
static mut index: [c_int; SNDRV_CARDS] = [SNDRV_DEFAULT_IDX; SNDRV_CARDS];
static mut id: [*mut c_char; SNDRV_CARDS] = [SNDRV_DEFAULT_STR; SNDRV_CARDS];
static mut enable: [Bool; SNDRV_CARDS] = [SNDRV_DEFAULT_ENABLE_PNP; SNDRV_CARDS];

/* module_param_array(index, int, NULL, 0444); */
/* MODULE_PARM_DESC(index, "Index value for Digigram Lola driver."); */
/* module_param_array(id, charp, NULL, 0444); */
/* MODULE_PARM_DESC(id, "ID string for Digigram Lola driver."); */
/* module_param_array(enable, bool, NULL, 0444); */
/* MODULE_PARM_DESC(enable, "Enable Digigram Lola driver."); */

/* Lola-specific options */

/* for instance use always max granularity which is compatible
 * with all sample rates
 */
static mut granularity: [c_int; SNDRV_CARDS] = [LOLA_GRANULARITY_MAX; SNDRV_CARDS];

/* below a sample_rate of 16kHz the analogue audio quality is NOT excellent */
static mut sample_rate_min: [c_int; SNDRV_CARDS] = [16000; SNDRV_CARDS];

/* module_param_array(granularity, int, NULL, 0444); */
/* MODULE_PARM_DESC(granularity, "Granularity value"); */
/* module_param_array(sample_rate_min, int, NULL, 0444); */
/* MODULE_PARM_DESC(sample_rate_min, "Minimal sample rate"); */

/*
 */

/* MODULE_LICENSE("GPL"); */
/* MODULE_DESCRIPTION("Digigram Lola driver"); */
/* MODULE_AUTHOR("Takashi Iwai <tiwai@suse.de>"); */

/* CONFIG_SND_DEBUG_VERBOSE controls whether verbose_debug emits pr_debug. */
static mut debug: c_int = 0;

unsafe fn verbose_debug(_fmt: *const c_char) {}

/*
 * pseudo-codec read/write via CORB/RIRB
 */

unsafe fn corb_send_verb(
    chip: *mut lola,
    nid: c_uint,
    verb: c_uint,
    mut data: c_uint,
    extdata: c_uint,
) -> c_int {
    let mut ret: c_int = -EIO;

    *lola_get_last_cmd_nid(chip) = nid;
    *lola_get_last_verb(chip) = verb;
    *lola_get_last_data(chip) = data;
    *lola_get_last_extdata(chip) = extdata;
    data |= (nid << 20) | (verb << 8);

    let flags = spin_lock_irqsave(lola_get_reg_lock(chip));
    if *lola_get_rirb_cmds(chip) < LOLA_CORB_ENTRIES - 1 {
        let mut wp = *lola_get_corb_wp(chip) + 1;
        wp %= LOLA_CORB_ENTRIES;
        *lola_get_corb_wp(chip) = wp;
        *lola_get_corb_buf(chip).add((wp * 2) as usize) = cpu_to_le32(data);
        *lola_get_corb_buf(chip).add((wp * 2 + 1) as usize) = cpu_to_le32(extdata);
        lola_writew(chip, BAR0, CORBWP, wp);
        *lola_get_rirb_cmds(chip) += 1;
        smp_wmb();
        ret = 0;
    }
    spin_unlock_irqrestore(lola_get_reg_lock(chip), flags);
    ret
}

unsafe fn lola_queue_unsol_event(chip: *mut lola, res: c_uint, _res_ex: c_uint) {
    lola_update_ext_clock_freq(chip, res);
}

/* retrieve RIRB entry - called from interrupt handler */
unsafe fn lola_update_rirb(chip: *mut lola) {
    let mut rp: c_uint;
    let res: U32;
    let res_ex: U32;

    let wp = lola_readw(chip, BAR0, RIRBWP) as c_uint;
    if wp == *lola_get_rirb_wp(chip) {
        return;
    }
    *lola_get_rirb_wp(chip) = wp;

    while *lola_get_rirb_rp(chip) != wp {
        *lola_get_rirb_rp(chip) += 1;
        *lola_get_rirb_rp(chip) %= LOLA_CORB_ENTRIES;

        rp = *lola_get_rirb_rp(chip) << 1; /* an RIRB entry is 8-bytes */
        res_ex = le32_to_cpu(*lola_get_rirb_buf(chip).add((rp + 1) as usize));
        res = le32_to_cpu(*lola_get_rirb_buf(chip).add(rp as usize));
        if (res_ex & LOLA_RIRB_EX_UNSOL_EV) != 0 {
            lola_queue_unsol_event(chip, res, res_ex);
        } else if *lola_get_rirb_cmds(chip) != 0 {
            *lola_get_res(chip) = res;
            *lola_get_res_ex(chip) = res_ex;
            smp_wmb();
            *lola_get_rirb_cmds(chip) -= 1;
        }
    }
}

unsafe fn rirb_get_response(chip: *mut lola, val: *mut c_uint, extval: *mut c_uint) -> c_int {
    'again: loop {
        let timeout = jiffies + msecs_to_jiffies(1000);
        loop {
            if *lola_get_polling_mode(chip) != 0 {
                spin_lock_irq(lola_get_reg_lock(chip));
                lola_update_rirb(chip);
                spin_unlock_irq(lola_get_reg_lock(chip));
            }
            if *lola_get_rirb_cmds(chip) == 0 {
                *val = *lola_get_res(chip);
                if !extval.is_null() {
                    *extval = *lola_get_res_ex(chip);
                }
                verbose_debug(c"get_response: %x, %x\n".as_ptr());
                if (*lola_get_res_ex(chip) & LOLA_RIRB_EX_ERROR) != 0 {
                    dev_warn(
                        (**lola_get_card(chip)).dev,
                        c"RIRB ERROR: NID=%x, verb=%x, data=%x, ext=%x\n".as_ptr(),
                        *lola_get_last_cmd_nid(chip),
                        *lola_get_last_verb(chip),
                        *lola_get_last_data(chip),
                        *lola_get_last_extdata(chip),
                    );
                    return -EIO;
                }
                return 0;
            }
            if time_after(jiffies, timeout) {
                break;
            }
            udelay(20);
            cond_resched();
        }
        dev_warn((**lola_get_card(chip)).dev, c"RIRB response error\n".as_ptr());
        if *lola_get_polling_mode(chip) == 0 {
            dev_warn((**lola_get_card(chip)).dev, c"switching to polling mode\n".as_ptr());
            *lola_get_polling_mode(chip) = 1;
            continue 'again;
        }
        return -EIO;
    }
}

/* aynchronous write of a codec verb with data */
pub unsafe extern "C" fn lola_codec_write(
    chip: *mut lola,
    nid: c_uint,
    verb: c_uint,
    data: c_uint,
    extdata: c_uint,
) -> c_int {
    verbose_debug(c"codec_write NID=%x, verb=%x, data=%x, ext=%x\n".as_ptr());
    corb_send_verb(chip, nid, verb, data, extdata)
}

/* write a codec verb with data and read the returned status */
pub unsafe extern "C" fn lola_codec_read(
    chip: *mut lola,
    nid: c_uint,
    verb: c_uint,
    data: c_uint,
    extdata: c_uint,
    val: *mut c_uint,
    extval: *mut c_uint,
) -> c_int {
    let mut err: c_int;

    verbose_debug(c"codec_read NID=%x, verb=%x, data=%x, ext=%x\n".as_ptr());
    err = corb_send_verb(chip, nid, verb, data, extdata);
    if err < 0 {
        return err;
    }
    err = rirb_get_response(chip, val, extval);
    err
}

/* flush all pending codec writes */
pub unsafe extern "C" fn lola_codec_flush(chip: *mut lola) -> c_int {
    let mut tmp: c_uint = 0;
    rirb_get_response(chip, &mut tmp, ptr::null_mut())
}

/*
 * interrupt handler
 */
unsafe extern "C" fn lola_interrupt(_irq: c_int, dev_id: *mut c_void) -> IrqReturnT {
    let chip = dev_id as *mut lola;
    let mut notify_ins: c_uint;
    let mut notify_outs: c_uint;
    let mut error_ins: c_uint;
    let mut error_outs: c_uint;
    let mut handled: c_int = 0;
    let mut i: c_int;

    notify_ins = 0;
    notify_outs = 0;
    error_ins = 0;
    error_outs = 0;
    spin_lock(lola_get_reg_lock(chip));
    loop {
        let status: c_uint;
        let mut in_sts: c_uint;
        let mut out_sts: c_uint;
        let reg: c_uint;

        status = lola_readl(chip, BAR1, DINTSTS);
        if status == 0 || status == u32::MAX {
            break;
        }

        in_sts = lola_readl(chip, BAR1, DIINTSTS);
        out_sts = lola_readl(chip, BAR1, DOINTSTS);

        /* clear Input Interrupts */
        i = 0;
        while in_sts != 0 && i < *lola_get_pcm_num_streams(chip, CAPT) {
            if (in_sts & (1u32 << i)) == 0 {
                i += 1;
                continue;
            }
            in_sts &= !(1u32 << i);
            reg = lola_dsd_read(chip, i, STS);
            if (reg & LOLA_DSD_STS_DESE) != 0 {
                /* error */
                error_ins |= 1u32 << i;
            }
            if (reg & LOLA_DSD_STS_BCIS) != 0 {
                /* notify */
                notify_ins |= 1u32 << i;
            }
            /* clear */
            lola_dsd_write(chip, i, STS, reg);
            i += 1;
        }

        /* clear Output Interrupts */
        i = 0;
        while out_sts != 0 && i < *lola_get_pcm_num_streams(chip, PLAY) {
            if (out_sts & (1u32 << i)) == 0 {
                i += 1;
                continue;
            }
            out_sts &= !(1u32 << i);
            reg = lola_dsd_read(chip, i + MAX_STREAM_IN_COUNT, STS);
            if (reg & LOLA_DSD_STS_DESE) != 0 {
                /* error */
                error_outs |= 1u32 << i;
            }
            if (reg & LOLA_DSD_STS_BCIS) != 0 {
                /* notify */
                notify_outs |= 1u32 << i;
            }
            lola_dsd_write(chip, i + MAX_STREAM_IN_COUNT, STS, reg);
            i += 1;
        }

        if (status & LOLA_DINT_CTRL) != 0 {
            let mut rbsts: u8; /* ring status is byte access */
            rbsts = lola_readb(chip, BAR0, RIRBSTS);
            rbsts &= LOLA_RIRB_INT_MASK;
            if rbsts != 0 {
                lola_writeb(chip, BAR0, RIRBSTS, rbsts);
            }
            rbsts = lola_readb(chip, BAR0, CORBSTS);
            rbsts &= LOLA_CORB_INT_MASK;
            if rbsts != 0 {
                lola_writeb(chip, BAR0, CORBSTS, rbsts);
            }

            lola_update_rirb(chip);
        }

        if (status & (LOLA_DINT_FIFOERR | LOLA_DINT_MUERR)) != 0 {
            /* clear global fifo error interrupt */
            lola_writel(
                chip,
                BAR1,
                DINTSTS,
                status & (LOLA_DINT_FIFOERR | LOLA_DINT_MUERR),
            );
        }
        handled = 1;
    }
    spin_unlock(lola_get_reg_lock(chip));

    lola_pcm_update(chip, lola_get_pcm(chip, CAPT), notify_ins);
    lola_pcm_update(chip, lola_get_pcm(chip, PLAY), notify_outs);

    IRQ_RETVAL(handled)
}

/*
 * controller
 */
unsafe fn reset_controller(chip: *mut lola) -> c_int {
    let mut gctl = lola_readl(chip, BAR0, GCTL);
    let end_time: c_ulong;

    if gctl != 0 {
        /* to be sure */
        lola_writel(chip, BAR1, BOARD_MODE, 0);
        return 0;
    }

    *lola_get_cold_reset(chip) = 1;
    lola_writel(chip, BAR0, GCTL, LOLA_GCTL_RESET);
    end_time = jiffies + msecs_to_jiffies(200);
    loop {
        msleep(1);
        gctl = lola_readl(chip, BAR0, GCTL);
        if gctl != 0 {
            break;
        }
        if !time_before(jiffies, end_time) {
            break;
        }
    }
    if gctl == 0 {
        dev_err((**lola_get_card(chip)).dev, c"cannot reset controller\n".as_ptr());
        return -EIO;
    }
    0
}

unsafe fn lola_irq_enable(chip: *mut lola) {
    let mut val: c_uint;

    /* enalbe all I/O streams */
    val = (1u32 << *lola_get_pcm_num_streams(chip, PLAY)) - 1;
    lola_writel(chip, BAR1, DOINTCTL, val);
    val = (1u32 << *lola_get_pcm_num_streams(chip, CAPT)) - 1;
    lola_writel(chip, BAR1, DIINTCTL, val);

    /* enable global irqs */
    val = LOLA_DINT_GLOBAL | LOLA_DINT_CTRL | LOLA_DINT_FIFOERR | LOLA_DINT_MUERR;
    lola_writel(chip, BAR1, DINTCTL, val);
}

unsafe fn lola_irq_disable(chip: *mut lola) {
    lola_writel(chip, BAR1, DINTCTL, 0);
    lola_writel(chip, BAR1, DIINTCTL, 0);
    lola_writel(chip, BAR1, DOINTCTL, 0);
}

unsafe fn setup_corb_rirb(chip: *mut lola) -> c_int {
    let mut tmp: u8;
    let end_time: c_ulong;

    *lola_get_rb(chip) = snd_devm_alloc_pages(
        &mut (**lola_get_pci(chip)).dev as *mut c_void,
        SNDRV_DMA_TYPE_DEV,
        PAGE_SIZE,
    );
    if (*lola_get_rb(chip)).is_null() {
        return -ENOMEM;
    }

    *lola_get_corb_addr(chip) = lola_dma_addr(*lola_get_rb(chip));
    *lola_get_corb_buf(chip) = lola_dma_area(*lola_get_rb(chip)) as Le32;
    *lola_get_rirb_addr(chip) = lola_dma_addr(*lola_get_rb(chip)) + 2048;
    *lola_get_rirb_buf(chip) = lola_dma_area(*lola_get_rb(chip)).add(2048) as Le32;

    /* disable ringbuffer DMAs */
    lola_writeb(chip, BAR0, RIRBCTL, 0);
    lola_writeb(chip, BAR0, CORBCTL, 0);

    end_time = jiffies + msecs_to_jiffies(200);
    loop {
        if lola_readb(chip, BAR0, RIRBCTL) == 0 && lola_readb(chip, BAR0, CORBCTL) == 0 {
            break;
        }
        msleep(1);
        if !time_before(jiffies, end_time) {
            break;
        }
    }

    /* CORB set up */
    lola_writel(chip, BAR0, CORBLBASE, *lola_get_corb_addr(chip) as u32);
    lola_writel(chip, BAR0, CORBUBASE, upper_32_bits(*lola_get_corb_addr(chip)));
    /* set the corb size to 256 entries */
    lola_writeb(chip, BAR0, CORBSIZE, 0x02);
    /* set the corb write pointer to 0 */
    lola_writew(chip, BAR0, CORBWP, 0);
    /* reset the corb hw read pointer */
    lola_writew(chip, BAR0, CORBRP, LOLA_RBRWP_CLR);
    /* enable corb dma */
    lola_writeb(chip, BAR0, CORBCTL, LOLA_RBCTL_DMA_EN);
    /* clear flags if set */
    tmp = lola_readb(chip, BAR0, CORBSTS) & LOLA_CORB_INT_MASK;
    if tmp != 0 {
        lola_writeb(chip, BAR0, CORBSTS, tmp);
    }
    *lola_get_corb_wp(chip) = 0;

    /* RIRB set up */
    lola_writel(chip, BAR0, RIRBLBASE, *lola_get_rirb_addr(chip) as u32);
    lola_writel(chip, BAR0, RIRBUBASE, upper_32_bits(*lola_get_rirb_addr(chip)));
    /* set the rirb size to 256 entries */
    lola_writeb(chip, BAR0, RIRBSIZE, 0x02);
    /* reset the rirb hw write pointer */
    lola_writew(chip, BAR0, RIRBWP, LOLA_RBRWP_CLR);
    /* set N=1, get RIRB response interrupt for new entry */
    lola_writew(chip, BAR0, RINTCNT, 1);
    /* enable rirb dma and response irq */
    lola_writeb(chip, BAR0, RIRBCTL, LOLA_RBCTL_DMA_EN | LOLA_RBCTL_IRQ_EN);
    /* clear flags if set */
    tmp = lola_readb(chip, BAR0, RIRBSTS) & LOLA_RIRB_INT_MASK;
    if tmp != 0 {
        lola_writeb(chip, BAR0, RIRBSTS, tmp);
    }
    *lola_get_rirb_rp(chip) = 0;
    *lola_get_rirb_cmds(chip) = 0;

    0
}

unsafe fn stop_corb_rirb(chip: *mut lola) {
    /* disable ringbuffer DMAs */
    lola_writeb(chip, BAR0, RIRBCTL, 0);
    lola_writeb(chip, BAR0, CORBCTL, 0);
}

unsafe fn lola_reset_setups(chip: *mut lola) {
    /* update the granularity */
    lola_set_granularity(chip, *lola_get_granularity(chip), true);
    /* update the sample clock */
    lola_set_clock_index(chip, *lola_get_clock_cur_index(chip));
    /* enable unsolicited events of the clock widget */
    lola_enable_clock_events(chip);
    /* update the analog gains */
    lola_setup_all_analog_gains(chip, CAPT as c_int, false); /* input, update */
    /* update SRC configuration if applicable */
    lola_set_src_config(chip, *lola_get_input_src_mask(chip), false);
    /* update the analog outputs */
    lola_setup_all_analog_gains(chip, PLAY as c_int, false); /* output, update */
}

unsafe fn lola_parse_tree(chip: *mut lola) -> c_int {
    let mut val: c_uint = 0;
    let mut nid: c_int;
    let mut err: c_int;

    err = lola_read_param(chip, 0, LOLA_PAR_VENDOR_ID, &mut val);
    if err < 0 {
        dev_err((**lola_get_card(chip)).dev, c"Can't read VENDOR_ID\n".as_ptr());
        return err;
    }
    val >>= 16;
    if val != 0x1369 {
        dev_err((**lola_get_card(chip)).dev, c"Unknown codec vendor 0x%x\n".as_ptr(), val);
        return -EINVAL;
    }

    err = lola_read_param(chip, 1, LOLA_PAR_FUNCTION_TYPE, &mut val);
    if err < 0 {
        dev_err((**lola_get_card(chip)).dev, c"Can't read FUNCTION_TYPE\n".as_ptr());
        return err;
    }
    if val != 1 {
        dev_err((**lola_get_card(chip)).dev, c"Unknown function type %d\n".as_ptr(), val);
        return -EINVAL;
    }

    err = lola_read_param(chip, 1, LOLA_PAR_SPECIFIC_CAPS, &mut val);
    if err < 0 {
        dev_err((**lola_get_card(chip)).dev, c"Can't read SPECCAPS\n".as_ptr());
        return err;
    }
    *lola_get_lola_caps(chip) = val;
    *lola_get_pin_num_pins(chip, CAPT) = LOLA_AFG_INPUT_PIN_COUNT(*lola_get_lola_caps(chip));
    *lola_get_pin_num_pins(chip, PLAY) = LOLA_AFG_OUTPUT_PIN_COUNT(*lola_get_lola_caps(chip));
    dev_dbg(
        (**lola_get_card(chip)).dev,
        c"speccaps=0x%x, pins in=%d, out=%d\n".as_ptr(),
        *lola_get_lola_caps(chip),
        *lola_get_pin_num_pins(chip, CAPT),
        *lola_get_pin_num_pins(chip, PLAY),
    );

    if *lola_get_pin_num_pins(chip, CAPT) > MAX_AUDIO_INOUT_COUNT
        || *lola_get_pin_num_pins(chip, PLAY) > MAX_AUDIO_INOUT_COUNT
    {
        dev_err((**lola_get_card(chip)).dev, c"Invalid Lola-spec caps 0x%x\n".as_ptr(), val);
        return -EINVAL;
    }

    nid = 0x02;
    err = lola_init_pcm(chip, CAPT as c_int, &mut nid);
    if err < 0 {
        return err;
    }
    err = lola_init_pcm(chip, PLAY as c_int, &mut nid);
    if err < 0 {
        return err;
    }

    err = lola_init_pins(chip, CAPT as c_int, &mut nid);
    if err < 0 {
        return err;
    }
    err = lola_init_pins(chip, PLAY as c_int, &mut nid);
    if err < 0 {
        return err;
    }

    if LOLA_AFG_CLOCK_WIDGET_PRESENT(*lola_get_lola_caps(chip)) {
        err = lola_init_clock_widget(chip, nid);
        if err < 0 {
            return err;
        }
        nid += 1;
    }
    if LOLA_AFG_MIXER_WIDGET_PRESENT(*lola_get_lola_caps(chip)) {
        err = lola_init_mixer_widget(chip, nid);
        if err < 0 {
            return err;
        }
        nid += 1;
    }

    /* enable unsolicited events of the clock widget */
    err = lola_enable_clock_events(chip);
    if err < 0 {
        return err;
    }

    /* if last ResetController was not a ColdReset, we don't know
     * the state of the card; initialize here again
     */
    if *lola_get_cold_reset(chip) == 0 {
        lola_reset_setups(chip);
        *lola_get_cold_reset(chip) = 1;
    } else {
        /* set the granularity if it is not the default */
        if *lola_get_granularity(chip) != LOLA_GRANULARITY_MIN {
            lola_set_granularity(chip, *lola_get_granularity(chip), true);
        }
    }

    0
}

unsafe fn lola_stop_hw(chip: *mut lola) {
    stop_corb_rirb(chip);
    lola_irq_disable(chip);
}

unsafe extern "C" fn lola_free(card: *mut snd_card) {
    let chip = (*card).private_data as *mut lola;

    if *lola_get_initialized(chip) != 0 {
        lola_stop_hw(chip);
    }
    lola_free_mixer(chip);
}

unsafe fn lola_create(card: *mut snd_card, pci: *mut pci_dev, dev: c_int) -> c_int {
    let chip = (*card).private_data as *mut lola;
    let mut err: c_int;
    let dever: c_uint;
    let mut iomem: *mut c_void;

    err = pcim_enable_device(pci);
    if err < 0 {
        return err;
    }

    spin_lock_init(lola_get_reg_lock(chip));
    mutex_init(lola_get_open_mutex(chip));
    *lola_get_card(chip) = card;
    *lola_get_pci(chip) = pci;
    *lola_get_irq(chip) = -1;
    (*card).private_free = Some(lola_free);

    *lola_get_granularity(chip) = granularity[dev as usize];
    match *lola_get_granularity(chip) {
        8 => {
            *lola_get_sample_rate_max(chip) = 48000;
        }
        16 => {
            *lola_get_sample_rate_max(chip) = 96000;
        }
        32 => {
            *lola_get_sample_rate_max(chip) = 192000;
        }
        _ => {
            dev_warn(
                (*(*lola_get_card(chip))).dev,
                c"Invalid granularity %d, reset to %d\n".as_ptr(),
                *lola_get_granularity(chip),
                LOLA_GRANULARITY_MAX,
            );
            *lola_get_granularity(chip) = LOLA_GRANULARITY_MAX;
            *lola_get_sample_rate_max(chip) = 192000;
        }
    }
    *lola_get_sample_rate_min(chip) = sample_rate_min[dev as usize];
    if *lola_get_sample_rate_min(chip) > *lola_get_sample_rate_max(chip) {
        dev_warn(
            (*(*lola_get_card(chip))).dev,
            c"Invalid sample_rate_min %d, reset to 16000\n".as_ptr(),
            *lola_get_sample_rate_min(chip),
        );
        *lola_get_sample_rate_min(chip) = 16000;
    }

    iomem = pcim_iomap_region(pci, 0, DRVNAME);
    if IS_ERR(iomem) {
        return PTR_ERR(iomem);
    }

    *lola_get_bar_remap_addr(chip, 0) = iomem;
    *lola_get_bar_addr(chip, 0) = pci_resource_start(pci, 0);

    iomem = pcim_iomap_region(pci, 2, DRVNAME);
    if IS_ERR(iomem) {
        return PTR_ERR(iomem);
    }

    *lola_get_bar_remap_addr(chip, 1) = iomem;
    *lola_get_bar_addr(chip, 1) = pci_resource_start(pci, 2);

    pci_set_master(pci);

    err = reset_controller(chip);
    if err < 0 {
        return err;
    }

    if devm_request_irq(
        &mut (*pci).dev as *mut c_void,
        (*pci).irq,
        lola_interrupt,
        IRQF_SHARED,
        KBUILD_MODNAME,
        chip as *mut c_void,
    ) != 0
    {
        dev_err((*lola_get_card(chip)).as_ref().unwrap().dev, c"unable to grab IRQ %d\n".as_ptr(), (*pci).irq);
        return -EBUSY;
    }
    *lola_get_irq(chip) = (*pci).irq;
    (*card).sync_irq = *lola_get_irq(chip);

    dever = lola_readl(chip, BAR1, DEVER);
    *lola_get_pcm_num_streams(chip, CAPT) = ((dever >> 0) & 0x3ff) as c_int;
    *lola_get_pcm_num_streams(chip, PLAY) = ((dever >> 10) & 0x3ff) as c_int;
    *lola_get_version(chip) = (dever >> 24) & 0xff;
    dev_dbg(
        (*lola_get_card(chip)).as_ref().unwrap().dev,
        c"streams in=%d, out=%d, version=0x%x\n".as_ptr(),
        *lola_get_pcm_num_streams(chip, CAPT),
        *lola_get_pcm_num_streams(chip, PLAY),
        *lola_get_version(chip),
    );

    /* Test LOLA_BAR1_DEVER */
    if *lola_get_pcm_num_streams(chip, CAPT) > MAX_STREAM_IN_COUNT
        || *lola_get_pcm_num_streams(chip, PLAY) > MAX_STREAM_OUT_COUNT
        || (*lola_get_pcm_num_streams(chip, CAPT) == 0
            && *lola_get_pcm_num_streams(chip, PLAY) == 0)
    {
        dev_err((*lola_get_card(chip)).as_ref().unwrap().dev, c"invalid DEVER = %x\n".as_ptr(), dever);
        return -EINVAL;
    }

    err = setup_corb_rirb(chip);
    if err < 0 {
        return err;
    }

    strscpy((*card).driver.as_mut_ptr(), c"Lola".as_ptr(), (*card).driver.len());
    strscpy(
        (*card).shortname.as_mut_ptr(),
        c"Digigram Lola".as_ptr(),
        (*card).shortname.len(),
    );
    snprintf(
        (*card).longname.as_mut_ptr(),
        (*card).longname.len(),
        c"%s at 0x%lx irq %i".as_ptr(),
        (*card).shortname.as_ptr(),
        *lola_get_bar_addr(chip, 0),
        *lola_get_irq(chip),
    );
    strscpy(
        (*card).mixername.as_mut_ptr(),
        (*card).shortname.as_ptr(),
        (*card).mixername.len(),
    );

    lola_irq_enable(chip);

    *lola_get_initialized(chip) = 1;
    0
}

unsafe fn __lola_probe(pci: *mut pci_dev, pci_id: *const pci_device_id) -> c_int {
    static mut dev: c_int = 0;
    let mut card: *mut snd_card = ptr::null_mut();
    let chip: *mut lola;
    let mut err: c_int;

    if dev >= SNDRV_CARDS as c_int {
        return -ENODEV;
    }
    if !enable[dev as usize] {
        dev += 1;
        return -ENOENT;
    }

    err = snd_devm_card_new(
        &mut (*pci).dev as *mut c_void,
        index[dev as usize],
        id[dev as usize],
        THIS_MODULE,
        core::mem::size_of::<lola>(),
        &mut card,
    );
    if err < 0 {
        dev_err(&mut (*pci).dev as *mut c_void, c"Error creating card!\n".as_ptr());
        return err;
    }
    chip = (*card).private_data as *mut lola;

    err = lola_create(card, pci, dev);
    if err < 0 {
        return err;
    }

    err = lola_parse_tree(chip);
    if err < 0 {
        return err;
    }

    err = lola_create_pcm(chip);
    if err < 0 {
        return err;
    }

    err = lola_create_mixer(chip);
    if err < 0 {
        return err;
    }

    lola_proc_debug_new(chip);

    err = snd_card_register(card);
    if err < 0 {
        return err;
    }

    pci_set_drvdata(pci, card as *mut c_void);
    dev += 1;
    let _ = pci_id;
    0
}

unsafe extern "C" fn lola_probe(pci: *mut pci_dev, pci_id: *const pci_device_id) -> c_int {
    snd_card_free_on_error(&mut (*pci).dev as *mut c_void, __lola_probe(pci, pci_id))
}

/* PCI IDs */
static lola_ids: [pci_device_id; 2] = [
    pci_device_id {
        vendor: 0x1369, /* PCI_VDEVICE(DIGIGRAM, 0x0001) */
        device: 0x0001,
        subvendor: 0,
        subdevice: 0,
        class: 0,
        class_mask: 0,
        driver_data: 0,
    },
    pci_device_id {
        vendor: 0,
        device: 0,
        subvendor: 0,
        subdevice: 0,
        class: 0,
        class_mask: 0,
        driver_data: 0,
    },
];
/* MODULE_DEVICE_TABLE(pci, lola_ids); */

/* pci_driver definition */
static mut lola_driver: pci_driver = pci_driver {
    name: unsafe { KBUILD_MODNAME },
    id_table: lola_ids.as_ptr(),
    probe: Some(lola_probe),
};

/* module_pci_driver(lola_driver); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
