// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   ALSA modem driver for VIA VT82xx (South Bridge)
 *
 *   VT82C686A/B/C, VT8233A/C, VT8235
 *
 *	Copyright (c) 2000 Jaroslav Kysela <perex@perex.cz>
 *	                   Tjeerd.Mulder <Tjeerd.Mulder@fujitsu-siemens.com>
 *                    2002 Takashi Iwai <tiwai@suse.de>
 */

/*
 * Changes:
 *
 * Sep. 2,  2004  Sasha Khapyorsky <sashak@alsa-project.org>
 *      Modified from original audio driver 'via82xx.c' to support AC97
 *      modems.
 */

// C includes translated as external kernel/ALSA dependencies:
// linux/io.h, delay.h, interrupt.h, init.h, pci.h, slab.h, module.h
// sound/core.h, pcm.h, pcm_params.h, info.h, ac97_codec.h, initval.h

type c_int = i32;
type c_uint = u32;
type c_ulong = u64;
type c_uchar = u8;
type c_ushort = u16;
type bool_t = bool;
type snd_pcm_uframes_t = c_ulong;
type irqreturn_t = c_int;
type spinlock_t = usize;
type __le32 = u32;
type u32_t = u32;

const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;

const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const EIO: c_int = 5;
const EBUSY: c_int = 16;

const MODULE_AUTHOR_TEXT: &str = "Jaroslav Kysela <perex@perex.cz>";
const MODULE_DESCRIPTION_TEXT: &str = "VIA VT82xx modem";
const MODULE_LICENSE_TEXT: &str = "GPL";

static mut index: c_int = -2; /* Exclude the first card */
static mut id: *mut i8 = SNDRV_DEFAULT_STR1 as *mut i8; /* ID for this card */
static mut ac97_clock: c_int = 48000;

/* just for backward compatibility */
static mut enable: bool_t = false;

/*
 *  Direct registers
 */

unsafe fn VIAREG(via: *mut via82xx_modem, x: c_ulong) -> c_ulong {
    (*via).port.wrapping_add(x)
}

unsafe fn VIADEV_REG(viadev: *mut viadev, x: c_ulong) -> c_ulong {
    (*viadev).port.wrapping_add(x)
}

/* common offsets */
const VIA_REG_OFFSET_STATUS: c_ulong = 0x00; /* byte - channel status */
const VIA_REG_STAT_ACTIVE: c_uint = 0x80; /* RO */
const VIA_REG_STAT_PAUSED: c_uint = 0x40; /* RO */
const VIA_REG_STAT_TRIGGER_QUEUED: c_uint = 0x08; /* RO */
const VIA_REG_STAT_STOPPED: c_uint = 0x04; /* RWC */
const VIA_REG_STAT_EOL: c_uint = 0x02; /* RWC */
const VIA_REG_STAT_FLAG: c_uint = 0x01; /* RWC */
const VIA_REG_OFFSET_CONTROL: c_ulong = 0x01; /* byte - channel control */
const VIA_REG_CTRL_START: c_uint = 0x80; /* WO */
const VIA_REG_CTRL_TERMINATE: c_uint = 0x40; /* WO */
const VIA_REG_CTRL_AUTOSTART: c_uint = 0x20;
const VIA_REG_CTRL_PAUSE: c_uint = 0x08; /* RW */
const VIA_REG_CTRL_INT_STOP: c_uint = 0x04;
const VIA_REG_CTRL_INT_EOL: c_uint = 0x02;
const VIA_REG_CTRL_INT_FLAG: c_uint = 0x01;
const VIA_REG_CTRL_RESET: c_uint = 0x01; /* RW - probably reset? undocumented */
const VIA_REG_CTRL_INT: c_uint = VIA_REG_CTRL_INT_FLAG | VIA_REG_CTRL_INT_EOL | VIA_REG_CTRL_AUTOSTART;
const VIA_REG_OFFSET_TYPE: c_ulong = 0x02; /* byte - channel type (686 only) */
const VIA_REG_TYPE_AUTOSTART: c_uint = 0x80; /* RW - autostart at EOL */
const VIA_REG_TYPE_16BIT: c_uint = 0x20; /* RW */
const VIA_REG_TYPE_STEREO: c_uint = 0x10; /* RW */
const VIA_REG_TYPE_INT_LLINE: c_uint = 0x00;
const VIA_REG_TYPE_INT_LSAMPLE: c_uint = 0x04;
const VIA_REG_TYPE_INT_LESSONE: c_uint = 0x08;
const VIA_REG_TYPE_INT_MASK: c_uint = 0x0c;
const VIA_REG_TYPE_INT_EOL: c_uint = 0x02;
const VIA_REG_TYPE_INT_FLAG: c_uint = 0x01;
const VIA_REG_OFFSET_TABLE_PTR: c_ulong = 0x04; /* dword - channel table pointer */
const VIA_REG_OFFSET_CURR_PTR: c_ulong = 0x04; /* dword - channel current pointer */
const VIA_REG_OFFSET_STOP_IDX: c_ulong = 0x08; /* dword - stop index, channel type, sample rate */
const VIA_REG_OFFSET_CURR_COUNT: c_ulong = 0x0c; /* dword - channel current count (24 bit) */
const VIA_REG_OFFSET_CURR_INDEX: c_ulong = 0x0f; /* byte - channel current index (for via8233 only) */

/* modem block */
const VIA_REG_MO_STATUS: c_ulong = 0x40;
const VIA_REG_MO_CONTROL: c_ulong = 0x41;
const VIA_REG_MO_TYPE: c_ulong = 0x42;
const VIA_REG_MO_TABLE_PTR: c_ulong = 0x44;
const VIA_REG_MO_CURR_PTR: c_ulong = 0x44;
const VIA_REG_MO_STOP_IDX: c_ulong = 0x48;
const VIA_REG_MO_CURR_COUNT: c_ulong = 0x4c;
const VIA_REG_MI_STATUS: c_ulong = 0x50;
const VIA_REG_MI_CONTROL: c_ulong = 0x51;
const VIA_REG_MI_TYPE: c_ulong = 0x52;
const VIA_REG_MI_TABLE_PTR: c_ulong = 0x54;
const VIA_REG_MI_CURR_PTR: c_ulong = 0x54;
const VIA_REG_MI_STOP_IDX: c_ulong = 0x58;
const VIA_REG_MI_CURR_COUNT: c_ulong = 0x5c;

/* AC'97 */
const VIA_REG_AC97: c_ulong = 0x80; /* dword */
const VIA_REG_AC97_CODEC_ID_MASK: c_uint = 3 << 30;
const VIA_REG_AC97_CODEC_ID_SHIFT: c_uint = 30;
const VIA_REG_AC97_CODEC_ID_PRIMARY: c_uint = 0x00;
const VIA_REG_AC97_CODEC_ID_SECONDARY: c_uint = 0x01;
const VIA_REG_AC97_SECONDARY_VALID: c_uint = 1 << 27;
const VIA_REG_AC97_PRIMARY_VALID: c_uint = 1 << 25;
const VIA_REG_AC97_BUSY: c_uint = 1 << 24;
const VIA_REG_AC97_READ: c_uint = 1 << 23;
const VIA_REG_AC97_CMD_SHIFT: c_uint = 16;
const VIA_REG_AC97_CMD_MASK: c_uint = 0x7e;
const VIA_REG_AC97_DATA_SHIFT: c_uint = 0;
const VIA_REG_AC97_DATA_MASK: c_uint = 0xffff;

const VIA_REG_SGD_SHADOW: c_ulong = 0x84; /* dword */
const VIA_REG_SGD_STAT_PB_FLAG: c_uint = 1 << 0;
const VIA_REG_SGD_STAT_CP_FLAG: c_uint = 1 << 1;
const VIA_REG_SGD_STAT_FM_FLAG: c_uint = 1 << 2;
const VIA_REG_SGD_STAT_PB_EOL: c_uint = 1 << 4;
const VIA_REG_SGD_STAT_CP_EOL: c_uint = 1 << 5;
const VIA_REG_SGD_STAT_FM_EOL: c_uint = 1 << 6;
const VIA_REG_SGD_STAT_PB_STOP: c_uint = 1 << 8;
const VIA_REG_SGD_STAT_CP_STOP: c_uint = 1 << 9;
const VIA_REG_SGD_STAT_FM_STOP: c_uint = 1 << 10;
const VIA_REG_SGD_STAT_PB_ACTIVE: c_uint = 1 << 12;
const VIA_REG_SGD_STAT_CP_ACTIVE: c_uint = 1 << 13;
const VIA_REG_SGD_STAT_FM_ACTIVE: c_uint = 1 << 14;
const VIA_REG_SGD_STAT_MR_FLAG: c_uint = 1 << 16;
const VIA_REG_SGD_STAT_MW_FLAG: c_uint = 1 << 17;
const VIA_REG_SGD_STAT_MR_EOL: c_uint = 1 << 20;
const VIA_REG_SGD_STAT_MW_EOL: c_uint = 1 << 21;
const VIA_REG_SGD_STAT_MR_STOP: c_uint = 1 << 24;
const VIA_REG_SGD_STAT_MW_STOP: c_uint = 1 << 25;
const VIA_REG_SGD_STAT_MR_ACTIVE: c_uint = 1 << 28;
const VIA_REG_SGD_STAT_MW_ACTIVE: c_uint = 1 << 29;

const VIA_REG_GPI_STATUS: c_ulong = 0x88;
const VIA_REG_GPI_INTR: c_ulong = 0x8c;

const VIA_TBL_BIT_FLAG: c_uint = 0x40000000;
const VIA_TBL_BIT_EOL: c_uint = 0x80000000;

/* pci space */
const VIA_ACLINK_STAT: c_int = 0x40;
const VIA_ACLINK_C11_READY: c_uint = 0x20;
const VIA_ACLINK_C10_READY: c_uint = 0x10;
const VIA_ACLINK_C01_READY: c_uint = 0x04; /* secondary codec ready */
const VIA_ACLINK_LOWPOWER: c_uint = 0x02; /* low-power state */
const VIA_ACLINK_C00_READY: c_uint = 0x01; /* primary codec ready */
const VIA_ACLINK_CTRL: c_int = 0x41;
const VIA_ACLINK_CTRL_ENABLE: c_uint = 0x80; /* 0: disable, 1: enable */
const VIA_ACLINK_CTRL_RESET: c_uint = 0x40; /* 0: assert, 1: de-assert */
const VIA_ACLINK_CTRL_SYNC: c_uint = 0x20; /* 0: release SYNC, 1: force SYNC hi */
const VIA_ACLINK_CTRL_SDO: c_uint = 0x10; /* 0: release SDO, 1: force SDO hi */
const VIA_ACLINK_CTRL_VRA: c_uint = 0x08; /* 0: disable VRA, 1: enable VRA */
const VIA_ACLINK_CTRL_PCM: c_uint = 0x04; /* 0: disable PCM, 1: enable PCM */
const VIA_ACLINK_CTRL_FM: c_uint = 0x02; /* via686 only */
const VIA_ACLINK_CTRL_SB: c_uint = 0x01; /* via686 only */
const VIA_ACLINK_CTRL_INIT: c_uint = VIA_ACLINK_CTRL_ENABLE | VIA_ACLINK_CTRL_RESET | VIA_ACLINK_CTRL_PCM;
const VIA_FUNC_ENABLE: c_int = 0x42;
const VIA_FUNC_MIDI_PNP: c_uint = 0x80; /* FIXME: it's 0x40 in the datasheet! */
const VIA_FUNC_MIDI_IRQMASK: c_uint = 0x40; /* FIXME: not documented! */
const VIA_FUNC_RX2C_WRITE: c_uint = 0x20;
const VIA_FUNC_SB_FIFO_EMPTY: c_uint = 0x10;
const VIA_FUNC_ENABLE_GAME: c_uint = 0x08;
const VIA_FUNC_ENABLE_FM: c_uint = 0x04;
const VIA_FUNC_ENABLE_MIDI: c_uint = 0x02;
const VIA_FUNC_ENABLE_SB: c_uint = 0x01;
const VIA_PNP_CONTROL: c_int = 0x43;
const VIA_MC97_CTRL: c_int = 0x44;
const VIA_MC97_CTRL_ENABLE: c_uint = 0x80;
const VIA_MC97_CTRL_SECONDARY: c_uint = 0x40;
const VIA_MC97_CTRL_INIT: c_uint = VIA_MC97_CTRL_ENABLE | VIA_MC97_CTRL_SECONDARY;

/*
 * pcm stream
 */

#[repr(C)]
struct snd_via_sg_table {
    offset: c_uint,
    size: c_uint,
}

const VIA_TABLE_SIZE: usize = 255;

#[repr(C)]
struct viadev {
    reg_offset: c_uint,
    port: c_ulong,
    direction: c_int, /* playback = 0, capture = 1 */
    substream: *mut snd_pcm_substream,
    running: c_int,
    tbl_entries: c_uint, /* # descriptors */
    table: snd_dma_buffer,
    idx_table: *mut snd_via_sg_table,
    /* for recovery from the unexpected pointer */
    lastpos: c_uint,
    bufsize: c_uint,
    bufsize2: c_uint,
}

const TYPE_CARD_VIA82XX_MODEM: c_int = 1;

const VIA_MAX_MODEM_DEVS: usize = 2;

#[repr(C)]
struct via82xx_modem {
    irq: c_int,
    port: c_ulong,
    intr_mask: c_uint, /* SGD_SHADOW mask to check interrupts */
    pci: *mut pci_dev,
    card: *mut snd_card,
    num_devs: c_uint,
    playback_devno: c_uint,
    capture_devno: c_uint,
    devs: [viadev; VIA_MAX_MODEM_DEVS],
    pcms: [*mut snd_pcm; 2],
    ac97_bus: *mut snd_ac97_bus,
    ac97: *mut snd_ac97,
    ac97_clock: c_uint,
    ac97_secondary: c_uint, /* secondary AC'97 codec is present */
    reg_lock: spinlock_t,
    proc_entry: *mut snd_info_entry,
}

static snd_via82xx_modem_ids: [pci_device_id; 2] = [
    pci_device_id { vendor: PCI_VENDOR_ID_VIA, device: 0x3068, subvendor: PCI_ANY_ID, subdevice: PCI_ANY_ID, class: 0, class_mask: 0, driver_data: TYPE_CARD_VIA82XX_MODEM as c_ulong },
    pci_device_id { vendor: 0, device: 0, subvendor: 0, subdevice: 0, class: 0, class_mask: 0, driver_data: 0 },
];

/*
 * allocate and initialize the descriptor buffers
 * periods = number of periods
 * fragsize = period size in bytes
 */
unsafe fn build_via_table(dev: *mut viadev, substream: *mut snd_pcm_substream, pci: *mut pci_dev, periods: c_uint, fragsize: c_uint) -> c_int {
    let mut i: c_uint;
    let mut idx: c_uint;
    let mut ofs: c_uint;
    let mut rest: c_uint;
    let chip: *mut via82xx_modem = snd_pcm_substream_chip(substream) as *mut via82xx_modem;
    let mut pgtbl: *mut __le32;

    if (*dev).table.area.is_null() {
        /* the start of each lists must be aligned to 8 bytes,
         * but the kernel pages are much bigger, so we don't care
         */
        if snd_dma_alloc_pages(SNDRV_DMA_TYPE_DEV, &mut (*(*chip).pci).dev, PAGE_ALIGN((VIA_TABLE_SIZE * 2 * 8) as c_ulong), &mut (*dev).table) < 0 {
            return -ENOMEM;
        }
    }
    if (*dev).idx_table.is_null() {
        (*dev).idx_table = kmalloc_objs(core::mem::size_of::<snd_via_sg_table>(), VIA_TABLE_SIZE) as *mut snd_via_sg_table;
        if (*dev).idx_table.is_null() {
            return -ENOMEM;
        }
    }

    /* fill the entries */
    idx = 0;
    ofs = 0;
    pgtbl = (*dev).table.area as *mut __le32;
    i = 0;
    while i < periods {
        rest = fragsize;
        /* fill descriptors for a period.
         * a period can be split to several descriptors if it's
         * over page boundary.
         */
        loop {
            let mut r: c_uint;
            let flag: c_uint;
            let addr: c_uint;

            if idx >= VIA_TABLE_SIZE as c_uint {
                dev_err(&mut (*pci).dev, "too much table size!\n\0".as_ptr() as *const i8);
                return -EINVAL;
            }
            addr = snd_pcm_sgbuf_get_addr(substream, ofs) as c_uint;
            *pgtbl.add((idx << 1) as usize) = cpu_to_le32(addr);
            r = PAGE_SIZE as c_uint - (ofs % PAGE_SIZE as c_uint);
            if rest < r {
                r = rest;
            }
            rest = rest.wrapping_sub(r);
            if rest == 0 {
                if i == periods - 1 {
                    flag = VIA_TBL_BIT_EOL; /* buffer boundary */
                } else {
                    flag = VIA_TBL_BIT_FLAG; /* period boundary */
                }
            } else {
                flag = 0; /* period continues to the next */
            }
            /*
            dev_dbg(&pci->dev,
                "tbl %d: at %d  size %d (rest %d)\n",
                idx, ofs, r, rest);
            */
            *pgtbl.add(((idx << 1) + 1) as usize) = cpu_to_le32(r | flag);
            (*(*dev).idx_table.add(idx as usize)).offset = ofs;
            (*(*dev).idx_table.add(idx as usize)).size = r;
            ofs = ofs.wrapping_add(r);
            idx = idx.wrapping_add(1);
            if rest == 0 {
                break;
            }
        }
        i = i.wrapping_add(1);
    }
    (*dev).tbl_entries = idx;
    (*dev).bufsize = periods.wrapping_mul(fragsize);
    (*dev).bufsize2 = (*dev).bufsize / 2;
    0
}

unsafe fn clean_via_table(dev: *mut viadev, _substream: *mut snd_pcm_substream, _pci: *mut pci_dev) -> c_int {
    if !(*dev).table.area.is_null() {
        snd_dma_free_pages(&mut (*dev).table);
        (*dev).table.area = core::ptr::null_mut();
    }
    kfree((*dev).idx_table as *mut core::ffi::c_void);
    (*dev).idx_table = core::ptr::null_mut();
    0
}

/*
 *  Basic I/O
 */

unsafe fn snd_via82xx_codec_xread(chip: *mut via82xx_modem) -> c_uint {
    inl(VIAREG(chip, VIA_REG_AC97))
}

unsafe fn snd_via82xx_codec_xwrite(chip: *mut via82xx_modem, val: c_uint) {
    outl(val, VIAREG(chip, VIA_REG_AC97));
}

unsafe fn snd_via82xx_codec_ready(chip: *mut via82xx_modem, secondary: c_int) -> c_int {
    let mut timeout: c_uint = 1000; /* 1ms */
    let mut val: c_uint;

    while timeout > 0 {
        timeout = timeout.wrapping_sub(1);
        udelay(1);
        val = snd_via82xx_codec_xread(chip);
        if (val & VIA_REG_AC97_BUSY) == 0 {
            return (val & 0xffff) as c_int;
        }
    }
    dev_err((*(*chip).card).dev, "codec_ready: codec %i is not ready [0x%x]\n\0".as_ptr() as *const i8, secondary, snd_via82xx_codec_xread(chip));
    -EIO
}

unsafe fn snd_via82xx_codec_valid(chip: *mut via82xx_modem, secondary: c_int) -> c_int {
    let mut timeout: c_uint = 1000; /* 1ms */
    let mut val: c_uint;
    let mut val1: c_uint;
    let stat: c_uint = if secondary == 0 { VIA_REG_AC97_PRIMARY_VALID } else { VIA_REG_AC97_SECONDARY_VALID };

    while timeout > 0 {
        timeout = timeout.wrapping_sub(1);
        val = snd_via82xx_codec_xread(chip);
        val1 = val & (VIA_REG_AC97_BUSY | stat);
        if val1 == stat {
            return (val & 0xffff) as c_int;
        }
        udelay(1);
    }
    -EIO
}

unsafe fn snd_via82xx_codec_wait(ac97: *mut snd_ac97) {
    let chip: *mut via82xx_modem = (*ac97).private_data as *mut via82xx_modem;
    let _err: c_int;
    _err = snd_via82xx_codec_ready(chip, (*ac97).num);
    /* here we need to wait fairly for long time.. */
    msleep(500);
}

unsafe fn snd_via82xx_codec_write(ac97: *mut snd_ac97, reg: c_ushort, val: c_ushort) {
    let chip: *mut via82xx_modem = (*ac97).private_data as *mut via82xx_modem;
    let mut xval: c_uint;
    if reg as c_uint == AC97_GPIO_STATUS {
        outl(val as c_uint, VIAREG(chip, VIA_REG_GPI_STATUS));
        return;
    }
    xval = if (*ac97).num == 0 { VIA_REG_AC97_CODEC_ID_PRIMARY } else { VIA_REG_AC97_CODEC_ID_SECONDARY };
    xval <<= VIA_REG_AC97_CODEC_ID_SHIFT;
    xval |= (reg as c_uint) << VIA_REG_AC97_CMD_SHIFT;
    xval |= (val as c_uint) << VIA_REG_AC97_DATA_SHIFT;
    snd_via82xx_codec_xwrite(chip, xval);
    snd_via82xx_codec_ready(chip, (*ac97).num);
}

unsafe fn snd_via82xx_codec_read(ac97: *mut snd_ac97, reg: c_ushort) -> c_ushort {
    let chip: *mut via82xx_modem = (*ac97).private_data as *mut via82xx_modem;
    let mut xval: c_uint;
    let mut val: c_uint = 0xffff;
    let mut again: c_int = 0;

    xval = ((*ac97).num as c_uint) << VIA_REG_AC97_CODEC_ID_SHIFT;
    xval |= if (*ac97).num != 0 { VIA_REG_AC97_SECONDARY_VALID } else { VIA_REG_AC97_PRIMARY_VALID };
    xval |= VIA_REG_AC97_READ;
    xval |= ((reg as c_uint) & 0x7f) << VIA_REG_AC97_CMD_SHIFT;
    loop {
        if again > 3 {
            dev_err((*(*chip).card).dev, "codec_read: codec %i is not valid [0x%x]\n\0".as_ptr() as *const i8, (*ac97).num, snd_via82xx_codec_xread(chip));
            return 0xffff;
        }
        again += 1;
        snd_via82xx_codec_xwrite(chip, xval);
        udelay(20);
        if snd_via82xx_codec_valid(chip, (*ac97).num) >= 0 {
            udelay(25);
            val = snd_via82xx_codec_xread(chip);
            break;
        }
    }
    (val & 0xffff) as c_ushort
}

unsafe fn snd_via82xx_channel_reset(_chip: *mut via82xx_modem, viadev: *mut viadev) {
    outb((VIA_REG_CTRL_PAUSE | VIA_REG_CTRL_TERMINATE | VIA_REG_CTRL_RESET) as c_uchar, VIADEV_REG(viadev, VIA_REG_OFFSET_CONTROL));
    inb(VIADEV_REG(viadev, VIA_REG_OFFSET_CONTROL));
    udelay(50);
    /* disable interrupts */
    outb(0x00, VIADEV_REG(viadev, VIA_REG_OFFSET_CONTROL));
    /* clear interrupts */
    outb(0x03, VIADEV_REG(viadev, VIA_REG_OFFSET_STATUS));
    outb(0x00, VIADEV_REG(viadev, VIA_REG_OFFSET_TYPE)); /* for via686 */
    // outl(0, VIADEV_REG(viadev, VIA_REG_OFFSET_CURR_PTR));
    (*viadev).lastpos = 0;
}

/*
 *  Interrupt handler
 */

unsafe fn snd_via82xx_interrupt(_irq: c_int, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let chip: *mut via82xx_modem = dev_id as *mut via82xx_modem;
    let status: c_uint;
    let mut i: c_uint;

    status = inl(VIAREG(chip, VIA_REG_SGD_SHADOW));
    if (status & (*chip).intr_mask) == 0 {
        return IRQ_NONE;
    }
    // _skip_sgd:

    /* check status for each stream */
    spin_lock(&mut (*chip).reg_lock);
    i = 0;
    while i < (*chip).num_devs {
        let viadev: *mut viadev = &mut (*chip).devs[i as usize];
        let mut c_status: c_uchar = inb(VIADEV_REG(viadev, VIA_REG_OFFSET_STATUS));
        c_status &= (VIA_REG_STAT_EOL | VIA_REG_STAT_FLAG | VIA_REG_STAT_STOPPED) as c_uchar;
        if c_status != 0 {
            if !(*viadev).substream.is_null() && (*viadev).running != 0 {
                spin_unlock(&mut (*chip).reg_lock);
                snd_pcm_period_elapsed((*viadev).substream);
                spin_lock(&mut (*chip).reg_lock);
            }
            outb(c_status, VIADEV_REG(viadev, VIA_REG_OFFSET_STATUS)); /* ack */
        }
        i = i.wrapping_add(1);
    }
    spin_unlock(&mut (*chip).reg_lock);
    IRQ_HANDLED
}

/*
 *  PCM callbacks
 */

/*
 * trigger callback
 */
unsafe fn snd_via82xx_pcm_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let chip: *mut via82xx_modem = snd_pcm_substream_chip(substream) as *mut via82xx_modem;
    let viadev: *mut viadev = (*(*substream).runtime).private_data as *mut viadev;
    let mut val: c_uchar = 0;

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_SUSPEND => {
            val |= VIA_REG_CTRL_START as c_uchar;
            (*viadev).running = 1;
        }
        SNDRV_PCM_TRIGGER_STOP => {
            val = VIA_REG_CTRL_TERMINATE as c_uchar;
            (*viadev).running = 0;
        }
        SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            val |= VIA_REG_CTRL_PAUSE as c_uchar;
            (*viadev).running = 0;
        }
        SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            (*viadev).running = 1;
        }
        _ => return -EINVAL,
    }
    outb(val, VIADEV_REG(viadev, VIA_REG_OFFSET_CONTROL));
    if cmd == SNDRV_PCM_TRIGGER_STOP {
        snd_via82xx_channel_reset(chip, viadev);
    }
    0
}

/*
 * pointer callbacks
 */

/*
 * calculate the linear position at the given sg-buffer index and the rest count
 */

unsafe fn check_invalid_pos(viadev: *mut viadev, pos: c_uint) -> bool {
    pos < (*viadev).lastpos && (pos >= (*viadev).bufsize2 || (*viadev).lastpos < (*viadev).bufsize2)
}

unsafe fn calc_linear_pos(chip: *mut via82xx_modem, viadev: *mut viadev, idx: c_uint, count: c_uint) -> c_uint {
    let size: c_uint;
    let mut res: c_uint;

    size = (*(*viadev).idx_table.add(idx as usize)).size;
    res = (*(*viadev).idx_table.add(idx as usize)).offset.wrapping_add(size).wrapping_sub(count);

    /* check the validity of the calculated position */
    if size < count {
        dev_err((*(*chip).card).dev, "invalid via82xx_cur_ptr (size = %d, count = %d)\n\0".as_ptr() as *const i8, size as c_int, count as c_int);
        res = (*viadev).lastpos;
    } else if check_invalid_pos(viadev, res) {
        /* POINTER_DEBUG conditional debug block omitted unless enabled at build time. */
        if count == 0 {
            /* bogus count 0 on the DMA boundary? */
            res = (*(*viadev).idx_table.add(idx as usize)).offset;
        } else {
            /* count register returns full size
             * when end of buffer is reached
             */
            res = (*(*viadev).idx_table.add(idx as usize)).offset.wrapping_add(size);
        }
        if check_invalid_pos(viadev, res) {
            dev_dbg((*(*chip).card).dev, "invalid via82xx_cur_ptr (2), using last valid pointer\n\0".as_ptr() as *const i8);
            res = (*viadev).lastpos;
        }
    }
    (*viadev).lastpos = res; /* remember the last position */
    if res >= (*viadev).bufsize {
        res = res.wrapping_sub((*viadev).bufsize);
    }
    res
}

/*
 * get the current pointer on via686
 */
unsafe fn snd_via686_pcm_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let chip: *mut via82xx_modem = snd_pcm_substream_chip(substream) as *mut via82xx_modem;
    let viadev: *mut viadev = (*(*substream).runtime).private_data as *mut viadev;
    let idx: c_uint;
    let ptr: c_uint;
    let count: c_uint;
    let res: c_uint;

    if snd_BUG_ON((*viadev).tbl_entries == 0) {
        return 0;
    }
    if (inb(VIADEV_REG(viadev, VIA_REG_OFFSET_STATUS)) as c_uint & VIA_REG_STAT_ACTIVE) == 0 {
        return 0;
    }

    spin_lock(&mut (*chip).reg_lock);
    count = inl(VIADEV_REG(viadev, VIA_REG_OFFSET_CURR_COUNT)) & 0xffffff;
    /* The via686a does not have the current index register,
     * so we need to calculate the index from CURR_PTR.
     */
    ptr = inl(VIADEV_REG(viadev, VIA_REG_OFFSET_CURR_PTR));
    if ptr <= (*viadev).table.addr as c_uint {
        idx = 0;
    } else {
        /* CURR_PTR holds the address + 8 */
        idx = ((ptr.wrapping_sub((*viadev).table.addr as c_uint)) / 8 - 1) % (*viadev).tbl_entries;
    }
    res = calc_linear_pos(chip, viadev, idx, count);
    spin_unlock(&mut (*chip).reg_lock);

    bytes_to_frames((*substream).runtime, res)
}

/*
 * hw_params callback:
 * allocate the buffer and build up the buffer description table
 */
unsafe fn snd_via82xx_hw_params(substream: *mut snd_pcm_substream, hw_params: *mut snd_pcm_hw_params) -> c_int {
    let chip: *mut via82xx_modem = snd_pcm_substream_chip(substream) as *mut via82xx_modem;
    let viadev: *mut viadev = (*(*substream).runtime).private_data as *mut viadev;
    let err: c_int;

    err = build_via_table(viadev, substream, (*chip).pci, params_periods(hw_params), params_period_bytes(hw_params));
    if err < 0 {
        return err;
    }

    snd_ac97_write((*chip).ac97, AC97_LINE1_RATE, params_rate(hw_params));
    snd_ac97_write((*chip).ac97, AC97_LINE1_LEVEL, 0);

    0
}

/*
 * hw_free callback:
 * clean up the buffer description table and release the buffer
 */
unsafe fn snd_via82xx_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    let chip: *mut via82xx_modem = snd_pcm_substream_chip(substream) as *mut via82xx_modem;
    let viadev: *mut viadev = (*(*substream).runtime).private_data as *mut viadev;

    clean_via_table(viadev, substream, (*chip).pci);
    0
}

/*
 * set up the table pointer
 */
unsafe fn snd_via82xx_set_table_ptr(chip: *mut via82xx_modem, viadev: *mut viadev) {
    snd_via82xx_codec_ready(chip, (*chip).ac97_secondary as c_int);
    outl((*viadev).table.addr as u32_t, VIADEV_REG(viadev, VIA_REG_OFFSET_TABLE_PTR));
    udelay(20);
    snd_via82xx_codec_ready(chip, (*chip).ac97_secondary as c_int);
}

/*
 * prepare callback for playback and capture
 */
unsafe fn snd_via82xx_pcm_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let chip: *mut via82xx_modem = snd_pcm_substream_chip(substream) as *mut via82xx_modem;
    let viadev: *mut viadev = (*(*substream).runtime).private_data as *mut viadev;

    snd_via82xx_channel_reset(chip, viadev);
    /* this must be set after channel_reset */
    snd_via82xx_set_table_ptr(chip, viadev);
    outb((VIA_REG_TYPE_AUTOSTART | VIA_REG_TYPE_INT_EOL | VIA_REG_TYPE_INT_FLAG) as c_uchar, VIADEV_REG(viadev, VIA_REG_OFFSET_TYPE));
    0
}

/*
 * pcm hardware definition, identical for both playback and capture
 */
static snd_via82xx_hw: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_BLOCK_TRANSFER | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_PAUSE,
    formats: SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S16_LE,
    rates: SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_KNOT,
    rate_min: 8000,
    rate_max: 16000,
    channels_min: 1,
    channels_max: 1,
    buffer_bytes_max: 128 * 1024,
    period_bytes_min: 32,
    period_bytes_max: 128 * 1024,
    periods_min: 2,
    periods_max: (VIA_TABLE_SIZE / 2) as c_uint,
    fifo_size: 0,
};

/*
 * open callback skeleton
 */
unsafe fn snd_via82xx_modem_pcm_open(chip: *mut via82xx_modem, viadev: *mut viadev, substream: *mut snd_pcm_substream) -> c_int {
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let mut err: c_int;
    static rates: [c_uint; 4] = [8000, 9600, 12000, 16000];
    static hw_constraints_rates: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
        count: rates.len() as c_uint,
        list: rates.as_ptr(),
        mask: 0,
    };

    (*runtime).hw = snd_via82xx_hw;

    err = snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_RATE, &hw_constraints_rates);
    if err < 0 {
        return err;
    }

    /* we may remove following constaint when we modify table entries
       in interrupt */
    err = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if err < 0 {
        return err;
    }

    (*runtime).private_data = viadev as *mut core::ffi::c_void;
    (*viadev).substream = substream;

    0
}

/*
 * open callback for playback
 */
unsafe fn snd_via82xx_playback_open(substream: *mut snd_pcm_substream) -> c_int {
    let chip: *mut via82xx_modem = snd_pcm_substream_chip(substream) as *mut via82xx_modem;
    let viadev: *mut viadev = &mut (*chip).devs[((*chip).playback_devno + (*substream).number) as usize];

    snd_via82xx_modem_pcm_open(chip, viadev, substream)
}

/*
 * open callback for capture
 */
unsafe fn snd_via82xx_capture_open(substream: *mut snd_pcm_substream) -> c_int {
    let chip: *mut via82xx_modem = snd_pcm_substream_chip(substream) as *mut via82xx_modem;
    let viadev: *mut viadev = &mut (*chip).devs[((*chip).capture_devno + (*(*substream).pcm).device) as usize];

    snd_via82xx_modem_pcm_open(chip, viadev, substream)
}

/*
 * close callback
 */
unsafe fn snd_via82xx_pcm_close(substream: *mut snd_pcm_substream) -> c_int {
    let viadev: *mut viadev = (*(*substream).runtime).private_data as *mut viadev;

    (*viadev).substream = core::ptr::null_mut();
    0
}

/* via686 playback callbacks */
static snd_via686_playback_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_via82xx_playback_open),
    close: Some(snd_via82xx_pcm_close),
    hw_params: Some(snd_via82xx_hw_params),
    hw_free: Some(snd_via82xx_hw_free),
    prepare: Some(snd_via82xx_pcm_prepare),
    trigger: Some(snd_via82xx_pcm_trigger),
    pointer: Some(snd_via686_pcm_pointer),
};

/* via686 capture callbacks */
static snd_via686_capture_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_via82xx_capture_open),
    close: Some(snd_via82xx_pcm_close),
    hw_params: Some(snd_via82xx_hw_params),
    hw_free: Some(snd_via82xx_hw_free),
    prepare: Some(snd_via82xx_pcm_prepare),
    trigger: Some(snd_via82xx_pcm_trigger),
    pointer: Some(snd_via686_pcm_pointer),
};

unsafe fn init_viadev(chip: *mut via82xx_modem, idx: c_int, reg_offset: c_uint, direction: c_int) {
    (*chip).devs[idx as usize].reg_offset = reg_offset;
    (*chip).devs[idx as usize].direction = direction;
    (*chip).devs[idx as usize].port = (*chip).port.wrapping_add(reg_offset as c_ulong);
}

/*
 * create a pcm instance for via686a/b
 */
unsafe fn snd_via686_pcm_new(chip: *mut via82xx_modem) -> c_int {
    let mut pcm: *mut snd_pcm = core::ptr::null_mut();
    let err: c_int;

    (*chip).playback_devno = 0;
    (*chip).capture_devno = 1;
    (*chip).num_devs = 2;
    (*chip).intr_mask = 0x330000; /* FLAGS | EOL for MR, MW */

    err = snd_pcm_new((*chip).card, (*(*chip).card).shortname.as_mut_ptr(), 0, 1, 1, &mut pcm);
    if err < 0 {
        return err;
    }
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_via686_playback_ops);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_via686_capture_ops);
    (*pcm).dev_class = SNDRV_PCM_CLASS_MODEM;
    (*pcm).private_data = chip as *mut core::ffi::c_void;
    strscpy((*pcm).name.as_mut_ptr(), (*(*chip).card).shortname.as_ptr());
    (*chip).pcms[0] = pcm;
    init_viadev(chip, 0, VIA_REG_MO_STATUS as c_uint, 0);
    init_viadev(chip, 1, VIA_REG_MI_STATUS as c_uint, 1);

    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV_SG, &mut (*(*chip).pci).dev, 64 * 1024, 128 * 1024);
    0
}

/*
 *  Mixer part
 */

unsafe fn snd_via82xx_mixer_free_ac97_bus(bus: *mut snd_ac97_bus) {
    let chip: *mut via82xx_modem = (*bus).private_data as *mut via82xx_modem;
    (*chip).ac97_bus = core::ptr::null_mut();
}

unsafe fn snd_via82xx_mixer_free_ac97(ac97: *mut snd_ac97) {
    let chip: *mut via82xx_modem = (*ac97).private_data as *mut via82xx_modem;
    (*chip).ac97 = core::ptr::null_mut();
}

unsafe fn snd_via82xx_mixer_new(chip: *mut via82xx_modem) -> c_int {
    let mut ac97: snd_ac97_template = core::mem::zeroed();
    let mut err: c_int;
    static ops: snd_ac97_bus_ops = snd_ac97_bus_ops {
        write: Some(snd_via82xx_codec_write),
        read: Some(snd_via82xx_codec_read),
        wait: Some(snd_via82xx_codec_wait),
    };

    err = snd_ac97_bus((*chip).card, 0, &ops, chip as *mut core::ffi::c_void, &mut (*chip).ac97_bus);
    if err < 0 {
        return err;
    }
    (*(*chip).ac97_bus).private_free = Some(snd_via82xx_mixer_free_ac97_bus);
    (*(*chip).ac97_bus).clock = (*chip).ac97_clock;

    ac97.private_data = chip as *mut core::ffi::c_void;
    ac97.private_free = Some(snd_via82xx_mixer_free_ac97);
    ac97.pci = (*chip).pci;
    ac97.scaps = AC97_SCAP_SKIP_AUDIO | AC97_SCAP_POWER_SAVE;
    ac97.num = (*chip).ac97_secondary as c_int;

    err = snd_ac97_mixer((*chip).ac97_bus, &mut ac97, &mut (*chip).ac97);
    if err < 0 {
        return err;
    }

    0
}

/*
 * proc interface
 */
unsafe fn snd_via82xx_proc_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let chip: *mut via82xx_modem = (*entry).private_data as *mut via82xx_modem;
    let mut i: c_int;

    snd_iprintf(buffer, "%s\n\n\0".as_ptr() as *const i8, (*(*chip).card).longname.as_ptr());
    i = 0;
    while i < 0xa0 {
        snd_iprintf(buffer, "%02x: %08x\n\0".as_ptr() as *const i8, i, inl((*chip).port.wrapping_add(i as c_ulong)));
        i += 4;
    }
}

unsafe fn snd_via82xx_proc_init(chip: *mut via82xx_modem) {
    snd_card_ro_proc_new((*chip).card, "via82xx\0".as_ptr() as *const i8, chip as *mut core::ffi::c_void, Some(snd_via82xx_proc_read));
}

unsafe fn snd_via82xx_chip_init(chip: *mut via82xx_modem) -> c_int {
    let mut val: c_uint;
    let mut end_time: c_ulong;
    let mut pval: c_uchar = 0;

    pci_read_config_byte((*chip).pci, VIA_MC97_CTRL, &mut pval);
    if (pval as c_uint & VIA_MC97_CTRL_INIT) != VIA_MC97_CTRL_INIT {
        pci_write_config_byte((*chip).pci, 0x44, pval | VIA_MC97_CTRL_INIT as c_uchar);
        udelay(100);
    }

    pci_read_config_byte((*chip).pci, VIA_ACLINK_STAT, &mut pval);
    if (pval as c_uint & VIA_ACLINK_C00_READY) == 0 {
        /* codec not ready? */
        /* deassert ACLink reset, force SYNC */
        pci_write_config_byte((*chip).pci, VIA_ACLINK_CTRL, (VIA_ACLINK_CTRL_ENABLE | VIA_ACLINK_CTRL_RESET | VIA_ACLINK_CTRL_SYNC) as c_uchar);
        udelay(100);
        /* FIXME: should we do full reset here for all chip models? */
        pci_write_config_byte((*chip).pci, VIA_ACLINK_CTRL, 0x00);
        udelay(100);
        /* ACLink on, deassert ACLink reset, VSR, SGD data out */
        pci_write_config_byte((*chip).pci, VIA_ACLINK_CTRL, VIA_ACLINK_CTRL_INIT as c_uchar);
        udelay(100);
    }

    pci_read_config_byte((*chip).pci, VIA_ACLINK_CTRL, &mut pval);
    if (pval as c_uint & VIA_ACLINK_CTRL_INIT) != VIA_ACLINK_CTRL_INIT {
        /* ACLink on, deassert ACLink reset, VSR, SGD data out */
        pci_write_config_byte((*chip).pci, VIA_ACLINK_CTRL, VIA_ACLINK_CTRL_INIT as c_uchar);
        udelay(100);
    }

    /* wait until codec ready */
    end_time = jiffies.wrapping_add(msecs_to_jiffies(750));
    loop {
        pci_read_config_byte((*chip).pci, VIA_ACLINK_STAT, &mut pval);
        if (pval as c_uint & VIA_ACLINK_C00_READY) != 0 {
            break;
        }
        schedule_timeout_uninterruptible(1);
        if !time_before(jiffies, end_time) {
            break;
        }
    }

    val = snd_via82xx_codec_xread(chip);
    if (val & VIA_REG_AC97_BUSY) != 0 {
        dev_err((*(*chip).card).dev, "AC'97 codec is not ready [0x%x]\n\0".as_ptr() as *const i8, val);
    }

    snd_via82xx_codec_xwrite(chip, VIA_REG_AC97_READ | VIA_REG_AC97_SECONDARY_VALID | (VIA_REG_AC97_CODEC_ID_SECONDARY << VIA_REG_AC97_CODEC_ID_SHIFT));
    end_time = jiffies.wrapping_add(msecs_to_jiffies(750));
    snd_via82xx_codec_xwrite(chip, VIA_REG_AC97_READ | VIA_REG_AC97_SECONDARY_VALID | (VIA_REG_AC97_CODEC_ID_SECONDARY << VIA_REG_AC97_CODEC_ID_SHIFT));
    loop {
        val = snd_via82xx_codec_xread(chip);
        if (val & VIA_REG_AC97_SECONDARY_VALID) != 0 {
            (*chip).ac97_secondary = 1;
            break;
        }
        schedule_timeout_uninterruptible(1);
        if !time_before(jiffies, end_time) {
            break;
        }
    }
    /* This is ok, the most of motherboards have only one codec */

    /* route FM trap to IRQ, disable FM trap */
    // pci_write_config_byte(chip->pci, VIA_FM_NMI_CTRL, 0);
    /* disable all GPI interrupts */
    outl(0, VIAREG(chip, VIA_REG_GPI_INTR));

    0
}

/*
 * power management
 */
unsafe fn snd_via82xx_suspend(dev: *mut device) -> c_int {
    let card: *mut snd_card = dev_get_drvdata(dev) as *mut snd_card;
    let chip: *mut via82xx_modem = (*card).private_data as *mut via82xx_modem;
    let mut i: c_int;

    snd_power_change_state(card, SNDRV_CTL_POWER_D3hot);
    i = 0;
    while i < (*chip).num_devs as c_int {
        snd_via82xx_channel_reset(chip, &mut (*chip).devs[i as usize]);
        i += 1;
    }
    snd_ac97_suspend((*chip).ac97);
    0
}

unsafe fn snd_via82xx_resume(dev: *mut device) -> c_int {
    let card: *mut snd_card = dev_get_drvdata(dev) as *mut snd_card;
    let chip: *mut via82xx_modem = (*card).private_data as *mut via82xx_modem;
    let mut i: c_int;

    snd_via82xx_chip_init(chip);

    snd_ac97_resume((*chip).ac97);

    i = 0;
    while i < (*chip).num_devs as c_int {
        snd_via82xx_channel_reset(chip, &mut (*chip).devs[i as usize]);
        i += 1;
    }

    snd_power_change_state(card, SNDRV_CTL_POWER_D0);
    0
}

static snd_via82xx_pm: dev_pm_ops = dev_pm_ops {
    suspend: Some(snd_via82xx_suspend),
    resume: Some(snd_via82xx_resume),
};

unsafe fn snd_via82xx_free(card: *mut snd_card) {
    let chip: *mut via82xx_modem = (*card).private_data as *mut via82xx_modem;
    let mut i: c_uint;

    /* disable interrupts */
    i = 0;
    while i < (*chip).num_devs {
        snd_via82xx_channel_reset(chip, &mut (*chip).devs[i as usize]);
        i = i.wrapping_add(1);
    }
}

unsafe fn snd_via82xx_create(card: *mut snd_card, pci: *mut pci_dev, _chip_type: c_int, _revision: c_int, ac97_clock_arg: c_uint) -> c_int {
    let chip: *mut via82xx_modem = (*card).private_data as *mut via82xx_modem;
    let mut err: c_int;

    err = pcim_enable_device(pci);
    if err < 0 {
        return err;
    }

    spin_lock_init(&mut (*chip).reg_lock);
    (*chip).card = card;
    (*chip).pci = pci;
    (*chip).irq = -1;

    err = pcim_request_all_regions(pci, (*card).driver.as_ptr());
    if err < 0 {
        return err;
    }
    (*chip).port = pci_resource_start(pci, 0);
    if devm_request_irq(&mut (*pci).dev, (*pci).irq, Some(snd_via82xx_interrupt), IRQF_SHARED, KBUILD_MODNAME.as_ptr(), chip as *mut core::ffi::c_void) != 0 {
        dev_err((*card).dev, "unable to grab IRQ %d\n\0".as_ptr() as *const i8, (*pci).irq);
        return -EBUSY;
    }
    (*chip).irq = (*pci).irq;
    (*card).sync_irq = (*chip).irq;
    (*card).private_free = Some(snd_via82xx_free);
    if ac97_clock_arg >= 8000 && ac97_clock_arg <= 48000 {
        (*chip).ac97_clock = ac97_clock_arg;
    }

    err = snd_via82xx_chip_init(chip);
    if err < 0 {
        return err;
    }

    /* The 8233 ac97 controller does not implement the master bit
     * in the pci command register. IMHO this is a violation of the PCI spec.
     * We call pci_set_master here because it does not hurt. */
    pci_set_master(pci);
    0
}

unsafe fn __snd_via82xx_probe(pci: *mut pci_dev, pci_id: *const pci_device_id) -> c_int {
    let mut card: *mut snd_card = core::ptr::null_mut();
    let chip: *mut via82xx_modem;
    let chip_type: c_int = 0;
    let card_type: c_int;
    let mut i: c_uint;
    let mut err: c_int;

    err = snd_devm_card_new(&mut (*pci).dev, index, id, THIS_MODULE, core::mem::size_of::<via82xx_modem>(), &mut card);
    if err < 0 {
        return err;
    }
    chip = (*card).private_data as *mut via82xx_modem;

    card_type = (*pci_id).driver_data as c_int;
    match card_type {
        TYPE_CARD_VIA82XX_MODEM => {
            strscpy((*card).driver.as_mut_ptr(), "VIA82XX-MODEM\0".as_ptr() as *const i8);
            sprintf((*card).shortname.as_mut_ptr(), "VIA 82XX modem\0".as_ptr() as *const i8);
        }
        _ => {
            dev_err((*card).dev, "invalid card type %d\n\0".as_ptr() as *const i8, card_type);
            return -EINVAL;
        }
    }

    err = snd_via82xx_create(card, pci, chip_type, (*pci).revision as c_int, ac97_clock as c_uint);
    if err < 0 {
        return err;
    }
    err = snd_via82xx_mixer_new(chip);
    if err < 0 {
        return err;
    }

    err = snd_via686_pcm_new(chip);
    if err < 0 {
        return err;
    }

    /* disable interrupts */
    i = 0;
    while i < (*chip).num_devs {
        snd_via82xx_channel_reset(chip, &mut (*chip).devs[i as usize]);
        i = i.wrapping_add(1);
    }

    sprintf((*card).longname.as_mut_ptr(), "%s at 0x%lx, irq %d\0".as_ptr() as *const i8, (*card).shortname.as_ptr(), (*chip).port, (*chip).irq);

    snd_via82xx_proc_init(chip);

    err = snd_card_register(card);
    if err < 0 {
        return err;
    }
    pci_set_drvdata(pci, card as *mut core::ffi::c_void);
    0
}

unsafe fn snd_via82xx_probe(pci: *mut pci_dev, pci_id: *const pci_device_id) -> c_int {
    snd_card_free_on_error(&mut (*pci).dev, __snd_via82xx_probe(pci, pci_id))
}

static mut via82xx_modem_driver: pci_driver = pci_driver {
    name: KBUILD_MODNAME.as_ptr(),
    id_table: snd_via82xx_modem_ids.as_ptr(),
    probe: Some(snd_via82xx_probe),
    driver: device_driver {
        pm: &snd_via82xx_pm,
    },
};

// module_pci_driver(via82xx_modem_driver);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
