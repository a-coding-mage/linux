// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   ALSA driver for ATI IXP 150/200/250 AC97 modem controllers
 *
 *	Copyright (c) 2004 Takashi Iwai <tiwai@suse.de>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;
type u16 = u16;
type u32 = u32;
type __le32 = u32;
type snd_pcm_uframes_t = c_ulong;
type irqreturn_t = c_int;
type spinlock_t = c_void;

const MODULE_AUTHOR_TEXT: &[u8] = b"Takashi Iwai <tiwai@suse.de>\0";
const MODULE_DESCRIPTION_TEXT: &[u8] = b"ATI IXP MC97 controller\0";
const MODULE_LICENSE_TEXT: &[u8] = b"GPL\0";

static mut index: c_int = -2; /* Exclude the first card */
static mut id: *mut c_char = SNDRV_DEFAULT_STR1 as *mut c_char; /* ID for this card */
static mut ac97_clock: c_int = 48000;

/* module_param(index, int, 0444); */
/* MODULE_PARM_DESC(index, "Index value for ATI IXP controller."); */
/* module_param(id, charp, 0444); */
/* MODULE_PARM_DESC(id, "ID string for ATI IXP controller."); */
/* module_param(ac97_clock, int, 0444); */
/* MODULE_PARM_DESC(ac97_clock, "AC'97 codec clock (default 48000Hz)."); */

/* just for backward compatibility */
static mut enable: bool_ = false;
/* module_param(enable, bool, 0444); */

/*
 */

const ATI_REG_ISR: c_uint = 0x00; /* interrupt source */
const ATI_REG_ISR_MODEM_IN_XRUN: c_uint = 1u32 << 0;
const ATI_REG_ISR_MODEM_IN_STATUS: c_uint = 1u32 << 1;
const ATI_REG_ISR_MODEM_OUT1_XRUN: c_uint = 1u32 << 2;
const ATI_REG_ISR_MODEM_OUT1_STATUS: c_uint = 1u32 << 3;
const ATI_REG_ISR_MODEM_OUT2_XRUN: c_uint = 1u32 << 4;
const ATI_REG_ISR_MODEM_OUT2_STATUS: c_uint = 1u32 << 5;
const ATI_REG_ISR_MODEM_OUT3_XRUN: c_uint = 1u32 << 6;
const ATI_REG_ISR_MODEM_OUT3_STATUS: c_uint = 1u32 << 7;
const ATI_REG_ISR_PHYS_INTR: c_uint = 1u32 << 8;
const ATI_REG_ISR_PHYS_MISMATCH: c_uint = 1u32 << 9;
const ATI_REG_ISR_CODEC0_NOT_READY: c_uint = 1u32 << 10;
const ATI_REG_ISR_CODEC1_NOT_READY: c_uint = 1u32 << 11;
const ATI_REG_ISR_CODEC2_NOT_READY: c_uint = 1u32 << 12;
const ATI_REG_ISR_NEW_FRAME: c_uint = 1u32 << 13;
const ATI_REG_ISR_MODEM_GPIO_DATA: c_uint = 1u32 << 14;

const ATI_REG_IER: c_uint = 0x04; /* interrupt enable */
const ATI_REG_IER_MODEM_IN_XRUN_EN: c_uint = 1u32 << 0;
const ATI_REG_IER_MODEM_STATUS_EN: c_uint = 1u32 << 1;
const ATI_REG_IER_MODEM_OUT1_XRUN_EN: c_uint = 1u32 << 2;
const ATI_REG_IER_MODEM_OUT2_XRUN_EN: c_uint = 1u32 << 4;
const ATI_REG_IER_MODEM_OUT3_XRUN_EN: c_uint = 1u32 << 6;
const ATI_REG_IER_PHYS_INTR_EN: c_uint = 1u32 << 8;
const ATI_REG_IER_PHYS_MISMATCH_EN: c_uint = 1u32 << 9;
const ATI_REG_IER_CODEC0_INTR_EN: c_uint = 1u32 << 10;
const ATI_REG_IER_CODEC1_INTR_EN: c_uint = 1u32 << 11;
const ATI_REG_IER_CODEC2_INTR_EN: c_uint = 1u32 << 12;
const ATI_REG_IER_NEW_FRAME_EN: c_uint = 1u32 << 13; /* (RO */
const ATI_REG_IER_MODEM_GPIO_DATA_EN: c_uint = 1u32 << 14; /* (WO) modem is running */
const ATI_REG_IER_MODEM_SET_BUS_BUSY: c_uint = 1u32 << 15;

const ATI_REG_CMD: c_uint = 0x08; /* command */
const ATI_REG_CMD_POWERDOWN: c_uint = 1u32 << 0;
const ATI_REG_CMD_MODEM_RECEIVE_EN: c_uint = 1u32 << 1; /* modem only */
const ATI_REG_CMD_MODEM_SEND1_EN: c_uint = 1u32 << 2; /* modem only */
const ATI_REG_CMD_MODEM_SEND2_EN: c_uint = 1u32 << 3; /* modem only */
const ATI_REG_CMD_MODEM_SEND3_EN: c_uint = 1u32 << 4; /* modem only */
const ATI_REG_CMD_MODEM_STATUS_MEM: c_uint = 1u32 << 5; /* modem only */
const ATI_REG_CMD_MODEM_IN_DMA_EN: c_uint = 1u32 << 8; /* modem only */
const ATI_REG_CMD_MODEM_OUT_DMA1_EN: c_uint = 1u32 << 9; /* modem only */
const ATI_REG_CMD_MODEM_OUT_DMA2_EN: c_uint = 1u32 << 10; /* modem only */
const ATI_REG_CMD_MODEM_OUT_DMA3_EN: c_uint = 1u32 << 11; /* modem only */
const ATI_REG_CMD_AUDIO_PRESENT: c_uint = 1u32 << 20;
const ATI_REG_CMD_MODEM_GPIO_THRU_DMA: c_uint = 1u32 << 22; /* modem only */
const ATI_REG_CMD_LOOPBACK_EN: c_uint = 1u32 << 23;
const ATI_REG_CMD_PACKED_DIS: c_uint = 1u32 << 24;
const ATI_REG_CMD_BURST_EN: c_uint = 1u32 << 25;
const ATI_REG_CMD_PANIC_EN: c_uint = 1u32 << 26;
const ATI_REG_CMD_MODEM_PRESENT: c_uint = 1u32 << 27;
const ATI_REG_CMD_ACLINK_ACTIVE: c_uint = 1u32 << 28;
const ATI_REG_CMD_AC_SOFT_RESET: c_uint = 1u32 << 29;
const ATI_REG_CMD_AC_SYNC: c_uint = 1u32 << 30;
const ATI_REG_CMD_AC_RESET: c_uint = 1u32 << 31;

const ATI_REG_PHYS_OUT_ADDR: c_uint = 0x0c;
const ATI_REG_PHYS_OUT_CODEC_MASK: c_uint = 3u32 << 0;
const ATI_REG_PHYS_OUT_RW: c_uint = 1u32 << 2;
const ATI_REG_PHYS_OUT_ADDR_EN: c_uint = 1u32 << 8;
const ATI_REG_PHYS_OUT_ADDR_SHIFT: c_uint = 9;
const ATI_REG_PHYS_OUT_DATA_SHIFT: c_uint = 16;

const ATI_REG_PHYS_IN_ADDR: c_uint = 0x10;
const ATI_REG_PHYS_IN_READ_FLAG: c_uint = 1u32 << 8;
const ATI_REG_PHYS_IN_ADDR_SHIFT: c_uint = 9;
const ATI_REG_PHYS_IN_DATA_SHIFT: c_uint = 16;

const ATI_REG_SLOTREQ: c_uint = 0x14;

const ATI_REG_COUNTER: c_uint = 0x18;
const ATI_REG_COUNTER_SLOT: c_uint = 3u32 << 0; /* slot # */
const ATI_REG_COUNTER_BITCLOCK: c_uint = 31u32 << 8;

const ATI_REG_IN_FIFO_THRESHOLD: c_uint = 0x1c;

const ATI_REG_MODEM_IN_DMA_LINKPTR: c_uint = 0x20;
const ATI_REG_MODEM_IN_DMA_DT_START: c_uint = 0x24; /* RO */
const ATI_REG_MODEM_IN_DMA_DT_NEXT: c_uint = 0x28; /* RO */
const ATI_REG_MODEM_IN_DMA_DT_CUR: c_uint = 0x2c; /* RO */
const ATI_REG_MODEM_IN_DMA_DT_SIZE: c_uint = 0x30;
const ATI_REG_MODEM_OUT_FIFO: c_uint = 0x34; /* output threshold */
const ATI_REG_MODEM_OUT1_DMA_THRESHOLD_MASK: c_uint = 0xf << 16;
const ATI_REG_MODEM_OUT1_DMA_THRESHOLD_SHIFT: c_uint = 16;
const ATI_REG_MODEM_OUT_DMA1_LINKPTR: c_uint = 0x38;
const ATI_REG_MODEM_OUT_DMA2_LINKPTR: c_uint = 0x3c;
const ATI_REG_MODEM_OUT_DMA3_LINKPTR: c_uint = 0x40;
const ATI_REG_MODEM_OUT_DMA1_DT_START: c_uint = 0x44;
const ATI_REG_MODEM_OUT_DMA1_DT_NEXT: c_uint = 0x48;
const ATI_REG_MODEM_OUT_DMA1_DT_CUR: c_uint = 0x4c;
const ATI_REG_MODEM_OUT_DMA2_DT_START: c_uint = 0x50;
const ATI_REG_MODEM_OUT_DMA2_DT_NEXT: c_uint = 0x54;
const ATI_REG_MODEM_OUT_DMA2_DT_CUR: c_uint = 0x58;
const ATI_REG_MODEM_OUT_DMA3_DT_START: c_uint = 0x5c;
const ATI_REG_MODEM_OUT_DMA3_DT_NEXT: c_uint = 0x60;
const ATI_REG_MODEM_OUT_DMA3_DT_CUR: c_uint = 0x64;
const ATI_REG_MODEM_OUT_DMA12_DT_SIZE: c_uint = 0x68;
const ATI_REG_MODEM_OUT_DMA3_DT_SIZE: c_uint = 0x6c;
const ATI_REG_MODEM_OUT_FIFO_USED: c_uint = 0x70;
const ATI_REG_MODEM_OUT_GPIO: c_uint = 0x74;
const ATI_REG_MODEM_OUT_GPIO_EN: c_uint = 1;
const ATI_REG_MODEM_OUT_GPIO_DATA_SHIFT: c_uint = 5;
const ATI_REG_MODEM_IN_GPIO: c_uint = 0x78;

const ATI_REG_MODEM_MIRROR: c_uint = 0x7c;
const ATI_REG_AUDIO_MIRROR: c_uint = 0x80;

const ATI_REG_MODEM_FIFO_FLUSH: c_uint = 0x88;
const ATI_REG_MODEM_FIFO_OUT1_FLUSH: c_uint = 1u32 << 0;
const ATI_REG_MODEM_FIFO_OUT2_FLUSH: c_uint = 1u32 << 1;
const ATI_REG_MODEM_FIFO_OUT3_FLUSH: c_uint = 1u32 << 2;
const ATI_REG_MODEM_FIFO_IN_FLUSH: c_uint = 1u32 << 3;

/* LINKPTR */
const ATI_REG_LINKPTR_EN: c_uint = 1u32 << 0;

const ATI_MAX_DESCRIPTORS: c_uint = 256; /* max number of descriptor packets */

/*
 * DMA packate descriptor
 */

#[repr(C)]
struct atiixp_dma_desc {
    addr: __le32, /* DMA buffer address */
    status: u16, /* status bits */
    size: u16, /* size of the packet in dwords */
    next: __le32, /* address of the next packet descriptor */
}

/*
 * stream enum
 */
const ATI_DMA_PLAYBACK: usize = 0;
const ATI_DMA_CAPTURE: usize = 1;
const NUM_ATI_DMAS: usize = 2; /* DMAs */
const ATI_PCM_OUT: usize = 0;
const ATI_PCM_IN: usize = 1;
const NUM_ATI_PCMS: usize = 2; /* AC97 pcm slots */
const ATI_PCMDEV_ANALOG: usize = 0;
const NUM_ATI_PCMDEVS: usize = 1; /* pcm devices */

const NUM_ATI_CODECS: usize = 3;

/*
 * constants and callbacks for each DMA type
 */
#[repr(C)]
struct atiixp_dma_ops {
    type_: c_int, /* ATI_DMA_XXX */
    llp_offset: c_uint, /* LINKPTR offset */
    dt_cur: c_uint, /* DT_CUR offset */
    /* called from open callback */
    enable_dma: Option<unsafe extern "C" fn(chip: *mut atiixp_modem, on: c_int)>,
    /* called from trigger (START/STOP) */
    enable_transfer: Option<unsafe extern "C" fn(chip: *mut atiixp_modem, on: c_int)>,
    /* called from trigger (STOP only) */
    flush_dma: Option<unsafe extern "C" fn(chip: *mut atiixp_modem)>,
}

/*
 * DMA stream
 */
#[repr(C)]
struct atiixp_dma {
    ops: *const atiixp_dma_ops,
    desc_buf: snd_dma_buffer,
    substream: *mut snd_pcm_substream, /* assigned PCM substream */
    buf_addr: c_uint,
    buf_bytes: c_uint, /* DMA buffer address, bytes */
    period_bytes: c_uint,
    periods: c_uint,
    opened: c_int,
    running: c_int,
    pcm_open_flag: c_int,
    ac97_pcm_type: c_int, /* index # of ac97_pcm to access, -1 = not used */
}

/*
 * ATI IXP chip
 */
#[repr(C)]
struct atiixp_modem {
    card: *mut snd_card,
    pci: *mut pci_dev,

    res: *mut resource, /* memory i/o */
    addr: c_ulong,
    remap_addr: *mut c_void,
    irq: c_int,

    ac97_bus: *mut snd_ac97_bus,
    ac97: [*mut snd_ac97; NUM_ATI_CODECS],

    reg_lock: spinlock_t,

    dmas: [atiixp_dma; NUM_ATI_DMAS],
    pcms: [*mut ac97_pcm; NUM_ATI_PCMS],
    pcmdevs: [*mut snd_pcm; NUM_ATI_PCMDEVS],

    max_channels: c_int, /* max. channels for PCM out */

    codec_not_ready_bits: c_uint, /* for codec detection */

    spdif_over_aclink: c_int, /* passed from the module option */
    open_mutex: mutex, /* playback open mutex */
}

#[repr(C)]
struct pci_device_id {
    vendor: c_uint,
    device: c_uint,
    subvendor: c_uint,
    subdevice: c_uint,
    class: c_uint,
    class_mask: c_uint,
    driver_data: c_ulong,
}

const PCI_VENDOR_ID_ATI: c_uint = 0x1002;
const PCI_ANY_ID: c_uint = !0;

const fn PCI_VDEVICE_ATI(device: c_uint) -> pci_device_id {
    pci_device_id {
        vendor: PCI_VENDOR_ID_ATI,
        device,
        subvendor: PCI_ANY_ID,
        subdevice: PCI_ANY_ID,
        class: 0,
        class_mask: 0,
        driver_data: 0,
    }
}

/*
 */
static snd_atiixp_ids: [pci_device_id; 3] = [
    PCI_VDEVICE_ATI(0x434d), /* SB200 */
    PCI_VDEVICE_ATI(0x4378), /* SB400 */
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

/* MODULE_DEVICE_TABLE(pci, snd_atiixp_ids); */

/*
 * lowlevel functions
 */

/*
 * update the bits of the given register.
 * return 1 if the bits changed.
 */
unsafe fn snd_atiixp_update_bits(
    chip: *mut atiixp_modem,
    reg: c_uint,
    mask: c_uint,
    value: c_uint,
) -> c_int {
    let addr = ((*chip).remap_addr as *mut u8).add(reg as usize) as *mut c_void;
    let mut data: c_uint;
    let old_data: c_uint;
    old_data = readl(addr);
    data = old_data;
    data &= !mask;
    data |= value;
    if old_data == data {
        return 0;
    }
    writel(data, addr);
    1
}

/*
 * macros for easy use
 */
unsafe fn atiixp_write(chip: *mut atiixp_modem, reg: c_uint, value: c_uint) {
    writel(value, ((*chip).remap_addr as *mut u8).add(reg as usize) as *mut c_void);
}

unsafe fn atiixp_read(chip: *mut atiixp_modem, reg: c_uint) -> c_uint {
    readl(((*chip).remap_addr as *mut u8).add(reg as usize) as *mut c_void)
}

unsafe fn atiixp_update(chip: *mut atiixp_modem, reg: c_uint, mask: c_uint, val: c_uint) -> c_int {
    snd_atiixp_update_bits(chip, reg, mask, val)
}

/*
 * handling DMA packets
 *
 * we allocate a linear buffer for the DMA, and split it to  each packet.
 * in a future version, a scatter-gather buffer should be implemented.
 */

const ATI_DESC_LIST_SIZE: usize =
    PAGE_ALIGN(ATI_MAX_DESCRIPTORS as usize * size_of::<atiixp_dma_desc>());

/*
 * build packets ring for the given buffer size.
 *
 * IXP handles the buffer descriptors, which are connected as a linked
 * list.  although we can change the list dynamically, in this version,
 * a static RING of buffer descriptors is used.
 *
 * the ring is built in this function, and is set up to the hardware.
 */
unsafe fn atiixp_build_dma_packets(
    chip: *mut atiixp_modem,
    dma: *mut atiixp_dma,
    substream: *mut snd_pcm_substream,
    periods: c_uint,
    period_bytes: c_uint,
) -> c_int {
    let mut i: c_uint;
    let mut addr: u32;
    let mut desc_addr: u32;

    if periods > ATI_MAX_DESCRIPTORS {
        return -ENOMEM;
    }

    if (*dma).desc_buf.area.is_null() {
        if snd_dma_alloc_pages(
            SNDRV_DMA_TYPE_DEV,
            &mut (*(*chip).pci).dev,
            ATI_DESC_LIST_SIZE,
            &mut (*dma).desc_buf,
        ) < 0
        {
            return -ENOMEM;
        }
        (*dma).periods = 0; /* clear */
        (*dma).period_bytes = (*dma).periods;
    }

    if (*dma).periods == periods && (*dma).period_bytes == period_bytes {
        return 0;
    }

    /* reset DMA before changing the descriptor table */
    spin_lock_irqsave(&mut (*chip).reg_lock);
    writel(0, ((*chip).remap_addr as *mut u8).add((*(*dma).ops).llp_offset as usize) as *mut c_void);
    ((*(*dma).ops).enable_dma.unwrap())(chip, 0);
    ((*(*dma).ops).enable_dma.unwrap())(chip, 1);
    spin_unlock_irqrestore(&mut (*chip).reg_lock);

    /* fill the entries */
    addr = (*(*substream).runtime).dma_addr as u32;
    desc_addr = (*dma).desc_buf.addr as u32;
    i = 0;
    while i < periods {
        let desc: *mut atiixp_dma_desc =
            ((*dma).desc_buf.area as *mut atiixp_dma_desc).add(i as usize);
        (*desc).addr = cpu_to_le32(addr);
        (*desc).status = 0;
        (*desc).size = (period_bytes >> 2) as u16; /* in dwords */
        desc_addr = desc_addr.wrapping_add(size_of::<atiixp_dma_desc>() as u32);
        if i == periods - 1 {
            (*desc).next = cpu_to_le32((*dma).desc_buf.addr as u32);
        } else {
            (*desc).next = cpu_to_le32(desc_addr);
        }
        addr = addr.wrapping_add(period_bytes);
        i += 1;
    }

    writel(
        ((*dma).desc_buf.addr as u32) | ATI_REG_LINKPTR_EN,
        ((*chip).remap_addr as *mut u8).add((*(*dma).ops).llp_offset as usize) as *mut c_void,
    );

    (*dma).period_bytes = period_bytes;
    (*dma).periods = periods;

    0
}

/*
 * remove the ring buffer and release it if assigned
 */
unsafe fn atiixp_clear_dma_packets(
    chip: *mut atiixp_modem,
    dma: *mut atiixp_dma,
    _substream: *mut snd_pcm_substream,
) {
    if !(*dma).desc_buf.area.is_null() {
        writel(0, ((*chip).remap_addr as *mut u8).add((*(*dma).ops).llp_offset as usize) as *mut c_void);
        snd_dma_free_pages(&mut (*dma).desc_buf);
        (*dma).desc_buf.area = ptr::null_mut();
    }
}

/*
 * AC97 interface
 */
unsafe fn snd_atiixp_acquire_codec(chip: *mut atiixp_modem) -> c_int {
    let mut timeout: c_int = 1000;

    while atiixp_read(chip, ATI_REG_PHYS_OUT_ADDR) & ATI_REG_PHYS_OUT_ADDR_EN != 0 {
        if timeout == 0 {
            dev_warn((*(*chip).card).dev, c"codec acquire timeout\n".as_ptr());
            return -EBUSY;
        }
        timeout -= 1;
        udelay(1);
    }
    0
}

unsafe fn snd_atiixp_codec_read(
    chip: *mut atiixp_modem,
    codec: u16,
    reg: u16,
) -> u16 {
    let mut data: c_uint;
    let mut timeout: c_int;

    if snd_atiixp_acquire_codec(chip) < 0 {
        return 0xffff;
    }
    data = ((reg as c_uint) << ATI_REG_PHYS_OUT_ADDR_SHIFT)
        | ATI_REG_PHYS_OUT_ADDR_EN
        | ATI_REG_PHYS_OUT_RW
        | codec as c_uint;
    atiixp_write(chip, ATI_REG_PHYS_OUT_ADDR, data);
    if snd_atiixp_acquire_codec(chip) < 0 {
        return 0xffff;
    }
    timeout = 1000;
    loop {
        data = atiixp_read(chip, ATI_REG_PHYS_IN_ADDR);
        if data & ATI_REG_PHYS_IN_READ_FLAG != 0 {
            return (data >> ATI_REG_PHYS_IN_DATA_SHIFT) as u16;
        }
        udelay(1);
        timeout -= 1;
        if timeout == 0 {
            break;
        }
    }
    /* time out may happen during reset */
    if reg < 0x7c {
        dev_warn((*(*chip).card).dev, c"codec read timeout (reg %x)\n".as_ptr(), reg as c_int);
    }
    0xffff
}

unsafe fn snd_atiixp_codec_write(
    chip: *mut atiixp_modem,
    codec: u16,
    reg: u16,
    val: u16,
) {
    let data: c_uint;

    if snd_atiixp_acquire_codec(chip) < 0 {
        return;
    }
    data = ((val as c_uint) << ATI_REG_PHYS_OUT_DATA_SHIFT)
        | ((reg as c_uint) << ATI_REG_PHYS_OUT_ADDR_SHIFT)
        | ATI_REG_PHYS_OUT_ADDR_EN
        | codec as c_uint;
    atiixp_write(chip, ATI_REG_PHYS_OUT_ADDR, data);
}

unsafe extern "C" fn snd_atiixp_ac97_read(ac97: *mut snd_ac97, reg: u16) -> u16 {
    let chip: *mut atiixp_modem = (*ac97).private_data as *mut atiixp_modem;
    snd_atiixp_codec_read(chip, (*ac97).num as u16, reg)
}

unsafe extern "C" fn snd_atiixp_ac97_write(ac97: *mut snd_ac97, reg: u16, val: u16) {
    let chip: *mut atiixp_modem = (*ac97).private_data as *mut atiixp_modem;
    if reg == AC97_GPIO_STATUS {
        atiixp_write(
            chip,
            ATI_REG_MODEM_OUT_GPIO,
            ((val as c_uint) << ATI_REG_MODEM_OUT_GPIO_DATA_SHIFT) | ATI_REG_MODEM_OUT_GPIO_EN,
        );
        return;
    }
    snd_atiixp_codec_write(chip, (*ac97).num as u16, reg, val);
}

/*
 * reset AC link
 */
unsafe fn snd_atiixp_aclink_reset(chip: *mut atiixp_modem) -> c_int {
    let mut timeout: c_int;

    /* reset powerdoewn */
    if atiixp_update(chip, ATI_REG_CMD, ATI_REG_CMD_POWERDOWN, 0) != 0 {
        udelay(10);
    }

    /* perform a software reset */
    atiixp_update(chip, ATI_REG_CMD, ATI_REG_CMD_AC_SOFT_RESET, ATI_REG_CMD_AC_SOFT_RESET);
    atiixp_read(chip, ATI_REG_CMD);
    udelay(10);
    atiixp_update(chip, ATI_REG_CMD, ATI_REG_CMD_AC_SOFT_RESET, 0);

    timeout = 10;
    while atiixp_read(chip, ATI_REG_CMD) & ATI_REG_CMD_ACLINK_ACTIVE == 0 {
        /* do a hard reset */
        atiixp_update(
            chip,
            ATI_REG_CMD,
            ATI_REG_CMD_AC_SYNC | ATI_REG_CMD_AC_RESET,
            ATI_REG_CMD_AC_SYNC,
        );
        atiixp_read(chip, ATI_REG_CMD);
        msleep(1);
        atiixp_update(chip, ATI_REG_CMD, ATI_REG_CMD_AC_RESET, ATI_REG_CMD_AC_RESET);
        timeout -= 1;
        if timeout == 0 {
            dev_err((*(*chip).card).dev, c"codec reset timeout\n".as_ptr());
            break;
        }
    }

    /* deassert RESET and assert SYNC to make sure */
    atiixp_update(
        chip,
        ATI_REG_CMD,
        ATI_REG_CMD_AC_SYNC | ATI_REG_CMD_AC_RESET,
        ATI_REG_CMD_AC_SYNC | ATI_REG_CMD_AC_RESET,
    );

    0
}

unsafe fn snd_atiixp_aclink_down(chip: *mut atiixp_modem) -> c_int {
    // if (atiixp_read(chip, MODEM_MIRROR) & 0x1) /* modem running, too? */
    //	return -EBUSY;
    atiixp_update(
        chip,
        ATI_REG_CMD,
        ATI_REG_CMD_POWERDOWN | ATI_REG_CMD_AC_RESET,
        ATI_REG_CMD_POWERDOWN,
    );
    0
}

/*
 * auto-detection of codecs
 *
 * the IXP chip can generate interrupts for the non-existing codecs.
 * NEW_FRAME interrupt is used to make sure that the interrupt is generated
 * even if all three codecs are connected.
 */

const ALL_CODEC_NOT_READY: c_uint =
    ATI_REG_ISR_CODEC0_NOT_READY | ATI_REG_ISR_CODEC1_NOT_READY | ATI_REG_ISR_CODEC2_NOT_READY;
const CODEC_CHECK_BITS: c_uint = ALL_CODEC_NOT_READY | ATI_REG_ISR_NEW_FRAME;

unsafe fn snd_atiixp_codec_detect(chip: *mut atiixp_modem) -> c_int {
    let mut timeout: c_int;

    (*chip).codec_not_ready_bits = 0;
    atiixp_write(chip, ATI_REG_IER, CODEC_CHECK_BITS);
    /* wait for the interrupts */
    timeout = 50;
    while timeout > 0 {
        timeout -= 1;
        msleep(1);
        if (*chip).codec_not_ready_bits != 0 {
            break;
        }
    }
    atiixp_write(chip, ATI_REG_IER, 0); /* disable irqs */

    if ((*chip).codec_not_ready_bits & ALL_CODEC_NOT_READY) == ALL_CODEC_NOT_READY {
        dev_err((*(*chip).card).dev, c"no codec detected!\n".as_ptr());
        return -ENXIO;
    }
    0
}

/*
 * enable DMA and irqs
 */
unsafe fn snd_atiixp_chip_start(chip: *mut atiixp_modem) -> c_int {
    let mut reg: c_uint;

    /* set up spdif, enable burst mode */
    reg = atiixp_read(chip, ATI_REG_CMD);
    reg |= ATI_REG_CMD_BURST_EN;
    if reg & ATI_REG_CMD_MODEM_PRESENT == 0 {
        reg |= ATI_REG_CMD_MODEM_PRESENT;
    }
    atiixp_write(chip, ATI_REG_CMD, reg);

    /* clear all interrupt source */
    atiixp_write(chip, ATI_REG_ISR, 0xffffffff);
    /* enable irqs */
    atiixp_write(
        chip,
        ATI_REG_IER,
        ATI_REG_IER_MODEM_STATUS_EN | ATI_REG_IER_MODEM_IN_XRUN_EN | ATI_REG_IER_MODEM_OUT1_XRUN_EN,
    );
    0
}

/*
 * disable DMA and IRQs
 */
unsafe fn snd_atiixp_chip_stop(chip: *mut atiixp_modem) -> c_int {
    /* clear interrupt source */
    atiixp_write(chip, ATI_REG_ISR, atiixp_read(chip, ATI_REG_ISR));
    /* disable irqs */
    atiixp_write(chip, ATI_REG_IER, 0);
    0
}

/*
 * PCM section
 */

/*
 * pointer callback simplly reads XXX_DMA_DT_CUR register as the current
 * position.  when SG-buffer is implemented, the offset must be calculated
 * correctly...
 */
unsafe extern "C" fn snd_atiixp_pcm_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let chip: *mut atiixp_modem = snd_pcm_substream_chip(substream) as *mut atiixp_modem;
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let dma: *mut atiixp_dma = (*runtime).private_data as *mut atiixp_dma;
    let mut curptr: c_uint;
    let mut timeout: c_int = 1000;

    while timeout != 0 {
        timeout -= 1;
        curptr = readl(((*chip).remap_addr as *mut u8).add((*(*dma).ops).dt_cur as usize) as *mut c_void);
        if curptr < (*dma).buf_addr {
            continue;
        }
        curptr = curptr.wrapping_sub((*dma).buf_addr);
        if curptr >= (*dma).buf_bytes {
            continue;
        }
        return bytes_to_frames(runtime, curptr);
    }
    dev_dbg(
        (*(*chip).card).dev,
        c"invalid DMA pointer read 0x%x (buf=%x)\n".as_ptr(),
        readl(((*chip).remap_addr as *mut u8).add((*(*dma).ops).dt_cur as usize) as *mut c_void),
        (*dma).buf_addr,
    );
    0
}

/*
 * XRUN detected, and stop the PCM substream
 */
unsafe fn snd_atiixp_xrun_dma(chip: *mut atiixp_modem, dma: *mut atiixp_dma) {
    if (*dma).substream.is_null() || (*dma).running == 0 {
        return;
    }
    dev_dbg((*(*chip).card).dev, c"XRUN detected (DMA %d)\n".as_ptr(), (*(*dma).ops).type_);
    snd_pcm_stop_xrun((*dma).substream);
}

/*
 * the period ack.  update the substream.
 */
unsafe fn snd_atiixp_update_dma(_chip: *mut atiixp_modem, dma: *mut atiixp_dma) {
    if (*dma).substream.is_null() || (*dma).running == 0 {
        return;
    }
    snd_pcm_period_elapsed((*dma).substream);
}

/* set BUS_BUSY interrupt bit if any DMA is running */
/* call with spinlock held */
unsafe fn snd_atiixp_check_bus_busy(chip: *mut atiixp_modem) {
    let bus_busy: c_uint;
    if atiixp_read(chip, ATI_REG_CMD) & (ATI_REG_CMD_MODEM_SEND1_EN | ATI_REG_CMD_MODEM_RECEIVE_EN) != 0 {
        bus_busy = ATI_REG_IER_MODEM_SET_BUS_BUSY;
    } else {
        bus_busy = 0;
    }
    atiixp_update(chip, ATI_REG_IER, ATI_REG_IER_MODEM_SET_BUS_BUSY, bus_busy);
}

/* common trigger callback
 * calling the lowlevel callbacks in it
 */
unsafe extern "C" fn snd_atiixp_pcm_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let chip: *mut atiixp_modem = snd_pcm_substream_chip(substream) as *mut atiixp_modem;
    let dma: *mut atiixp_dma = (*(*substream).runtime).private_data as *mut atiixp_dma;
    let mut err: c_int = 0;

    if snd_BUG_ON((*(*dma).ops).enable_transfer.is_none() || (*(*dma).ops).flush_dma.is_none()) != 0 {
        return -EINVAL;
    }

    spin_lock(&mut (*chip).reg_lock);
    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            ((*(*dma).ops).enable_transfer.unwrap())(chip, 1);
            (*dma).running = 1;
        }
        SNDRV_PCM_TRIGGER_STOP => {
            ((*(*dma).ops).enable_transfer.unwrap())(chip, 0);
            (*dma).running = 0;
        }
        _ => {
            err = -EINVAL;
        }
    }
    if err == 0 {
        snd_atiixp_check_bus_busy(chip);
        if cmd == SNDRV_PCM_TRIGGER_STOP {
            ((*(*dma).ops).flush_dma.unwrap())(chip);
            snd_atiixp_check_bus_busy(chip);
        }
    }
    spin_unlock(&mut (*chip).reg_lock);
    err
}

/*
 * lowlevel callbacks for each DMA type
 *
 * every callback is supposed to be called in chip->reg_lock spinlock
 */

/* flush FIFO of analog OUT DMA */
unsafe extern "C" fn atiixp_out_flush_dma(chip: *mut atiixp_modem) {
    atiixp_write(chip, ATI_REG_MODEM_FIFO_FLUSH, ATI_REG_MODEM_FIFO_OUT1_FLUSH);
}

/* enable/disable analog OUT DMA */
unsafe extern "C" fn atiixp_out_enable_dma(chip: *mut atiixp_modem, on: c_int) {
    let mut data: c_uint;
    data = atiixp_read(chip, ATI_REG_CMD);
    if on != 0 {
        if data & ATI_REG_CMD_MODEM_OUT_DMA1_EN != 0 {
            return;
        }
        atiixp_out_flush_dma(chip);
        data |= ATI_REG_CMD_MODEM_OUT_DMA1_EN;
    } else {
        data &= !ATI_REG_CMD_MODEM_OUT_DMA1_EN;
    }
    atiixp_write(chip, ATI_REG_CMD, data);
}

/* start/stop transfer over OUT DMA */
unsafe extern "C" fn atiixp_out_enable_transfer(chip: *mut atiixp_modem, on: c_int) {
    atiixp_update(
        chip,
        ATI_REG_CMD,
        ATI_REG_CMD_MODEM_SEND1_EN,
        if on != 0 { ATI_REG_CMD_MODEM_SEND1_EN } else { 0 },
    );
}

/* enable/disable analog IN DMA */
unsafe extern "C" fn atiixp_in_enable_dma(chip: *mut atiixp_modem, on: c_int) {
    atiixp_update(
        chip,
        ATI_REG_CMD,
        ATI_REG_CMD_MODEM_IN_DMA_EN,
        if on != 0 { ATI_REG_CMD_MODEM_IN_DMA_EN } else { 0 },
    );
}

/* start/stop analog IN DMA */
unsafe extern "C" fn atiixp_in_enable_transfer(chip: *mut atiixp_modem, on: c_int) {
    if on != 0 {
        let mut data: c_uint = atiixp_read(chip, ATI_REG_CMD);
        if data & ATI_REG_CMD_MODEM_RECEIVE_EN == 0 {
            data |= ATI_REG_CMD_MODEM_RECEIVE_EN;
            atiixp_write(chip, ATI_REG_CMD, data);
        }
    } else {
        atiixp_update(chip, ATI_REG_CMD, ATI_REG_CMD_MODEM_RECEIVE_EN, 0);
    }
}

/* flush FIFO of analog IN DMA */
unsafe extern "C" fn atiixp_in_flush_dma(chip: *mut atiixp_modem) {
    atiixp_write(chip, ATI_REG_MODEM_FIFO_FLUSH, ATI_REG_MODEM_FIFO_IN_FLUSH);
}

/* set up slots and formats for analog OUT */
unsafe extern "C" fn snd_atiixp_playback_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let chip: *mut atiixp_modem = snd_pcm_substream_chip(substream) as *mut atiixp_modem;
    let mut data: c_uint;

    spin_lock_irq(&mut (*chip).reg_lock);
    /* set output threshold */
    data = atiixp_read(chip, ATI_REG_MODEM_OUT_FIFO);
    data &= !ATI_REG_MODEM_OUT1_DMA_THRESHOLD_MASK;
    data |= 0x04 << ATI_REG_MODEM_OUT1_DMA_THRESHOLD_SHIFT;
    atiixp_write(chip, ATI_REG_MODEM_OUT_FIFO, data);
    spin_unlock_irq(&mut (*chip).reg_lock);
    0
}

/* set up slots and formats for analog IN */
unsafe extern "C" fn snd_atiixp_capture_prepare(_substream: *mut snd_pcm_substream) -> c_int {
    0
}

/*
 * hw_params - allocate the buffer and set up buffer descriptors
 */
unsafe extern "C" fn snd_atiixp_pcm_hw_params(
    substream: *mut snd_pcm_substream,
    hw_params: *mut snd_pcm_hw_params,
) -> c_int {
    let chip: *mut atiixp_modem = snd_pcm_substream_chip(substream) as *mut atiixp_modem;
    let dma: *mut atiixp_dma = (*(*substream).runtime).private_data as *mut atiixp_dma;
    let mut err: c_int;
    let mut i: c_int;

    (*dma).buf_addr = (*(*substream).runtime).dma_addr as c_uint;
    (*dma).buf_bytes = params_buffer_bytes(hw_params);

    err = atiixp_build_dma_packets(
        chip,
        dma,
        substream,
        params_periods(hw_params),
        params_period_bytes(hw_params),
    );
    if err < 0 {
        return err;
    }

    /* set up modem rate */
    i = 0;
    while i < NUM_ATI_CODECS as c_int {
        if !(*chip).ac97[i as usize].is_null() {
            snd_ac97_write((*chip).ac97[i as usize], AC97_LINE1_RATE, params_rate(hw_params));
            snd_ac97_write((*chip).ac97[i as usize], AC97_LINE1_LEVEL, 0);
        }
        i += 1;
    }

    err
}

unsafe extern "C" fn snd_atiixp_pcm_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    let chip: *mut atiixp_modem = snd_pcm_substream_chip(substream) as *mut atiixp_modem;
    let dma: *mut atiixp_dma = (*(*substream).runtime).private_data as *mut atiixp_dma;

    atiixp_clear_dma_packets(chip, dma, substream);
    0
}

/*
 * pcm hardware definition, identical for all DMA types
 */
static snd_atiixp_pcm_hw: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_BLOCK_TRANSFER
        | SNDRV_PCM_INFO_MMAP_VALID,
    formats: SNDRV_PCM_FMTBIT_S16_LE,
    rates: SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_KNOT,
    rate_min: 8000,
    rate_max: 16000,
    channels_min: 2,
    channels_max: 2,
    buffer_bytes_max: 256 * 1024,
    period_bytes_min: 32,
    period_bytes_max: 128 * 1024,
    periods_min: 2,
    periods_max: ATI_MAX_DESCRIPTORS,
};

unsafe fn snd_atiixp_pcm_open(
    substream: *mut snd_pcm_substream,
    dma: *mut atiixp_dma,
    pcm_type: c_int,
) -> c_int {
    let chip: *mut atiixp_modem = snd_pcm_substream_chip(substream) as *mut atiixp_modem;
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let mut err: c_int;
    static rates: [c_uint; 4] = [8000, 9600, 12000, 16000];
    static hw_constraints_rates: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
        count: ARRAY_SIZE_4,
        list: rates.as_ptr(),
        mask: 0,
    };

    if snd_BUG_ON((*dma).ops.is_null() || (*(*dma).ops).enable_dma.is_none()) != 0 {
        return -EINVAL;
    }

    if (*dma).opened != 0 {
        return -EBUSY;
    }
    (*dma).substream = substream;
    (*runtime).hw = snd_atiixp_pcm_hw;
    (*dma).ac97_pcm_type = pcm_type;
    err = snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_RATE, &hw_constraints_rates);
    if err < 0 {
        return err;
    }
    err = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if err < 0 {
        return err;
    }
    (*runtime).private_data = dma as *mut c_void;

    /* enable DMA bits */
    spin_lock_irq(&mut (*chip).reg_lock);
    ((*(*dma).ops).enable_dma.unwrap())(chip, 1);
    spin_unlock_irq(&mut (*chip).reg_lock);
    (*dma).opened = 1;

    0
}

unsafe fn snd_atiixp_pcm_close(substream: *mut snd_pcm_substream, dma: *mut atiixp_dma) -> c_int {
    let chip: *mut atiixp_modem = snd_pcm_substream_chip(substream) as *mut atiixp_modem;
    /* disable DMA bits */
    if snd_BUG_ON((*dma).ops.is_null() || (*(*dma).ops).enable_dma.is_none()) != 0 {
        return -EINVAL;
    }
    spin_lock_irq(&mut (*chip).reg_lock);
    ((*(*dma).ops).enable_dma.unwrap())(chip, 0);
    spin_unlock_irq(&mut (*chip).reg_lock);
    (*dma).substream = ptr::null_mut();
    (*dma).opened = 0;
    0
}

/*
 */
unsafe extern "C" fn snd_atiixp_playback_open(substream: *mut snd_pcm_substream) -> c_int {
    let chip: *mut atiixp_modem = snd_pcm_substream_chip(substream) as *mut atiixp_modem;

    mutex_lock(&mut (*chip).open_mutex);
    let ret = snd_atiixp_pcm_open(substream, &mut (*chip).dmas[ATI_DMA_PLAYBACK], 0);
    mutex_unlock(&mut (*chip).open_mutex);
    ret
}

unsafe extern "C" fn snd_atiixp_playback_close(substream: *mut snd_pcm_substream) -> c_int {
    let chip: *mut atiixp_modem = snd_pcm_substream_chip(substream) as *mut atiixp_modem;

    mutex_lock(&mut (*chip).open_mutex);
    let ret = snd_atiixp_pcm_close(substream, &mut (*chip).dmas[ATI_DMA_PLAYBACK]);
    mutex_unlock(&mut (*chip).open_mutex);
    ret
}

unsafe extern "C" fn snd_atiixp_capture_open(substream: *mut snd_pcm_substream) -> c_int {
    let chip: *mut atiixp_modem = snd_pcm_substream_chip(substream) as *mut atiixp_modem;
    snd_atiixp_pcm_open(substream, &mut (*chip).dmas[ATI_DMA_CAPTURE], 1)
}

unsafe extern "C" fn snd_atiixp_capture_close(substream: *mut snd_pcm_substream) -> c_int {
    let chip: *mut atiixp_modem = snd_pcm_substream_chip(substream) as *mut atiixp_modem;
    snd_atiixp_pcm_close(substream, &mut (*chip).dmas[ATI_DMA_CAPTURE])
}

/* AC97 playback */
static snd_atiixp_playback_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_atiixp_playback_open),
    close: Some(snd_atiixp_playback_close),
    hw_params: Some(snd_atiixp_pcm_hw_params),
    hw_free: Some(snd_atiixp_pcm_hw_free),
    prepare: Some(snd_atiixp_playback_prepare),
    trigger: Some(snd_atiixp_pcm_trigger),
    pointer: Some(snd_atiixp_pcm_pointer),
};

/* AC97 capture */
static snd_atiixp_capture_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_atiixp_capture_open),
    close: Some(snd_atiixp_capture_close),
    hw_params: Some(snd_atiixp_pcm_hw_params),
    hw_free: Some(snd_atiixp_pcm_hw_free),
    prepare: Some(snd_atiixp_capture_prepare),
    trigger: Some(snd_atiixp_pcm_trigger),
    pointer: Some(snd_atiixp_pcm_pointer),
};

static snd_atiixp_playback_dma_ops: atiixp_dma_ops = atiixp_dma_ops {
    type_: ATI_DMA_PLAYBACK as c_int,
    llp_offset: ATI_REG_MODEM_OUT_DMA1_LINKPTR,
    dt_cur: ATI_REG_MODEM_OUT_DMA1_DT_CUR,
    enable_dma: Some(atiixp_out_enable_dma),
    enable_transfer: Some(atiixp_out_enable_transfer),
    flush_dma: Some(atiixp_out_flush_dma),
};

static snd_atiixp_capture_dma_ops: atiixp_dma_ops = atiixp_dma_ops {
    type_: ATI_DMA_CAPTURE as c_int,
    llp_offset: ATI_REG_MODEM_IN_DMA_LINKPTR,
    dt_cur: ATI_REG_MODEM_IN_DMA_DT_CUR,
    enable_dma: Some(atiixp_in_enable_dma),
    enable_transfer: Some(atiixp_in_enable_transfer),
    flush_dma: Some(atiixp_in_flush_dma),
};

unsafe fn snd_atiixp_pcm_new(chip: *mut atiixp_modem) -> c_int {
    let mut pcm: *mut snd_pcm = ptr::null_mut();
    let mut err: c_int;

    /* initialize constants */
    (*chip).dmas[ATI_DMA_PLAYBACK].ops = &snd_atiixp_playback_dma_ops;
    (*chip).dmas[ATI_DMA_CAPTURE].ops = &snd_atiixp_capture_dma_ops;

    /* PCM #0: analog I/O */
    err = snd_pcm_new((*chip).card, c"ATI IXP MC97".as_ptr(), ATI_PCMDEV_ANALOG as c_int, 1, 1, &mut pcm);
    if err < 0 {
        return err;
    }
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_atiixp_playback_ops);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_atiixp_capture_ops);
    (*pcm).dev_class = SNDRV_PCM_CLASS_MODEM;
    (*pcm).private_data = chip as *mut c_void;
    strscpy((*pcm).name.as_mut_ptr(), c"ATI IXP MC97".as_ptr());
    (*chip).pcmdevs[ATI_PCMDEV_ANALOG] = pcm;

    snd_pcm_set_managed_buffer_all(
        pcm,
        SNDRV_DMA_TYPE_DEV,
        &mut (*(*chip).pci).dev,
        64 * 1024,
        128 * 1024,
    );

    0
}

/*
 * interrupt handler
 */
unsafe extern "C" fn snd_atiixp_interrupt(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let chip: *mut atiixp_modem = dev_id as *mut atiixp_modem;
    let status: c_uint;

    status = atiixp_read(chip, ATI_REG_ISR);

    if status == 0 {
        return IRQ_NONE;
    }

    /* process audio DMA */
    if status & ATI_REG_ISR_MODEM_OUT1_XRUN != 0 {
        snd_atiixp_xrun_dma(chip, &mut (*chip).dmas[ATI_DMA_PLAYBACK]);
    } else if status & ATI_REG_ISR_MODEM_OUT1_STATUS != 0 {
        snd_atiixp_update_dma(chip, &mut (*chip).dmas[ATI_DMA_PLAYBACK]);
    }
    if status & ATI_REG_ISR_MODEM_IN_XRUN != 0 {
        snd_atiixp_xrun_dma(chip, &mut (*chip).dmas[ATI_DMA_CAPTURE]);
    } else if status & ATI_REG_ISR_MODEM_IN_STATUS != 0 {
        snd_atiixp_update_dma(chip, &mut (*chip).dmas[ATI_DMA_CAPTURE]);
    }

    /* for codec detection */
    if status & CODEC_CHECK_BITS != 0 {
        let detected: c_uint;
        detected = status & CODEC_CHECK_BITS;
        spin_lock(&mut (*chip).reg_lock);
        (*chip).codec_not_ready_bits |= detected;
        atiixp_update(chip, ATI_REG_IER, detected, 0); /* disable the detected irqs */
        spin_unlock(&mut (*chip).reg_lock);
    }

    /* ack */
    atiixp_write(chip, ATI_REG_ISR, status);

    IRQ_HANDLED
}

/*
 * ac97 mixer section
 */

unsafe fn snd_atiixp_mixer_new(chip: *mut atiixp_modem, clock: c_int) -> c_int {
    let mut pbus: *mut snd_ac97_bus = ptr::null_mut();
    let mut ac97: snd_ac97_template = core::mem::zeroed();
    let mut i: c_int;
    let mut err: c_int;
    let mut codec_count: c_int;
    static ops: snd_ac97_bus_ops = snd_ac97_bus_ops {
        write: Some(snd_atiixp_ac97_write),
        read: Some(snd_atiixp_ac97_read),
    };
    static codec_skip: [c_uint; NUM_ATI_CODECS] = [
        ATI_REG_ISR_CODEC0_NOT_READY,
        ATI_REG_ISR_CODEC1_NOT_READY,
        ATI_REG_ISR_CODEC2_NOT_READY,
    ];

    if snd_atiixp_codec_detect(chip) < 0 {
        return -ENXIO;
    }

    err = snd_ac97_bus((*chip).card, 0, &ops, chip as *mut c_void, &mut pbus);
    if err < 0 {
        return err;
    }
    (*pbus).clock = clock;
    (*chip).ac97_bus = pbus;

    codec_count = 0;
    i = 0;
    while i < NUM_ATI_CODECS as c_int {
        if (*chip).codec_not_ready_bits & codec_skip[i as usize] != 0 {
            i += 1;
            continue;
        }
        ac97 = core::mem::zeroed();
        ac97.private_data = chip as *mut c_void;
        ac97.pci = (*chip).pci;
        ac97.num = i;
        ac97.scaps = AC97_SCAP_SKIP_AUDIO | AC97_SCAP_POWER_SAVE;
        err = snd_ac97_mixer(pbus, &mut ac97, &mut (*chip).ac97[i as usize]);
        if err < 0 {
            (*chip).ac97[i as usize] = ptr::null_mut(); /* to be sure */
            dev_dbg((*(*chip).card).dev, c"codec %d not available for modem\n".as_ptr(), i);
            i += 1;
            continue;
        }
        codec_count += 1;
        i += 1;
    }

    if codec_count == 0 {
        dev_err((*(*chip).card).dev, c"no codec available\n".as_ptr());
        return -ENODEV;
    }

    /* snd_ac97_tune_hardware(chip->ac97, ac97_quirks); */

    0
}

/*
 * power management
 */
unsafe extern "C" fn snd_atiixp_suspend(dev: *mut device) -> c_int {
    let card: *mut snd_card = dev_get_drvdata(dev) as *mut snd_card;
    let chip: *mut atiixp_modem = (*card).private_data as *mut atiixp_modem;
    let mut i: c_int;

    snd_power_change_state(card, SNDRV_CTL_POWER_D3hot);
    i = 0;
    while i < NUM_ATI_CODECS as c_int {
        snd_ac97_suspend((*chip).ac97[i as usize]);
        i += 1;
    }
    snd_atiixp_aclink_down(chip);
    snd_atiixp_chip_stop(chip);
    0
}

unsafe extern "C" fn snd_atiixp_resume(dev: *mut device) -> c_int {
    let card: *mut snd_card = dev_get_drvdata(dev) as *mut snd_card;
    let chip: *mut atiixp_modem = (*card).private_data as *mut atiixp_modem;
    let mut i: c_int;

    snd_atiixp_aclink_reset(chip);
    snd_atiixp_chip_start(chip);

    i = 0;
    while i < NUM_ATI_CODECS as c_int {
        snd_ac97_resume((*chip).ac97[i as usize]);
        i += 1;
    }

    snd_power_change_state(card, SNDRV_CTL_POWER_D0);
    0
}

static snd_atiixp_pm: dev_pm_ops = DEFINE_SIMPLE_DEV_PM_OPS(snd_atiixp_suspend, snd_atiixp_resume);

/*
 * proc interface for register dump
 */

unsafe extern "C" fn snd_atiixp_proc_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let chip: *mut atiixp_modem = (*entry).private_data as *mut atiixp_modem;
    let mut i: c_int;

    i = 0;
    while i < 256 {
        snd_iprintf(
            buffer,
            c"%02x: %08x\n".as_ptr(),
            i,
            readl(((*chip).remap_addr as *mut u8).add(i as usize) as *mut c_void),
        );
        i += 4;
    }
}

unsafe fn snd_atiixp_proc_init(chip: *mut atiixp_modem) {
    snd_card_ro_proc_new((*chip).card, c"atiixp-modem".as_ptr(), chip as *mut c_void, Some(snd_atiixp_proc_read));
}

/*
 * destructor
 */

unsafe extern "C" fn snd_atiixp_free(card: *mut snd_card) {
    snd_atiixp_chip_stop((*card).private_data as *mut atiixp_modem);
}

/*
 * constructor for chip instance
 */
unsafe fn snd_atiixp_init(card: *mut snd_card, pci: *mut pci_dev) -> c_int {
    let chip: *mut atiixp_modem = (*card).private_data as *mut atiixp_modem;
    let mut err: c_int;

    err = pcim_enable_device(pci);
    if err < 0 {
        return err;
    }

    spin_lock_init(&mut (*chip).reg_lock);
    mutex_init(&mut (*chip).open_mutex);
    (*chip).card = card;
    (*chip).pci = pci;
    (*chip).irq = -1;
    (*chip).remap_addr = pcim_iomap_region(pci, 0, c"ATI IXP MC97".as_ptr());
    if IS_ERR((*chip).remap_addr) {
        return PTR_ERR((*chip).remap_addr);
    }
    (*chip).addr = pci_resource_start(pci, 0);

    if devm_request_irq(
        &mut (*pci).dev,
        (*pci).irq,
        Some(snd_atiixp_interrupt),
        IRQF_SHARED,
        KBUILD_MODNAME,
        chip as *mut c_void,
    ) != 0
    {
        dev_err((*card).dev, c"unable to grab IRQ %d\n".as_ptr(), (*pci).irq);
        return -EBUSY;
    }
    (*chip).irq = (*pci).irq;
    (*card).sync_irq = (*chip).irq;
    (*card).private_free = Some(snd_atiixp_free);
    pci_set_master(pci);

    0
}

unsafe fn __snd_atiixp_probe(pci: *mut pci_dev, _pci_id: *const pci_device_id) -> c_int {
    let mut card: *mut snd_card = ptr::null_mut();
    let chip: *mut atiixp_modem;
    let mut err: c_int;

    err = snd_devm_card_new(
        &mut (*pci).dev,
        index,
        id,
        THIS_MODULE,
        size_of::<atiixp_modem>(),
        &mut card,
    );
    if err < 0 {
        return err;
    }
    chip = (*card).private_data as *mut atiixp_modem;

    strscpy((*card).driver.as_mut_ptr(), c"ATIIXP-MODEM".as_ptr());
    strscpy((*card).shortname.as_mut_ptr(), c"ATI IXP Modem".as_ptr());
    err = snd_atiixp_init(card, pci);
    if err < 0 {
        return err;
    }

    err = snd_atiixp_aclink_reset(chip);
    if err < 0 {
        return err;
    }

    err = snd_atiixp_mixer_new(chip, ac97_clock);
    if err < 0 {
        return err;
    }

    err = snd_atiixp_pcm_new(chip);
    if err < 0 {
        return err;
    }

    snd_atiixp_proc_init(chip);

    snd_atiixp_chip_start(chip);

    sprintf(
        (*card).longname.as_mut_ptr(),
        c"%s rev %x at 0x%lx, irq %i".as_ptr(),
        (*card).shortname.as_ptr(),
        (*pci).revision as c_uint,
        (*chip).addr,
        (*chip).irq,
    );

    err = snd_card_register(card);
    if err < 0 {
        return err;
    }

    pci_set_drvdata(pci, card as *mut c_void);
    0
}

unsafe extern "C" fn snd_atiixp_probe(pci: *mut pci_dev, pci_id: *const pci_device_id) -> c_int {
    snd_card_free_on_error(&mut (*pci).dev, __snd_atiixp_probe(pci, pci_id))
}

#[repr(C)]
struct pci_driver_inner {
    pm: *const dev_pm_ops,
}

#[repr(C)]
struct pci_driver {
    name: *const c_char,
    id_table: *const pci_device_id,
    probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> c_int>,
    driver: pci_driver_inner,
}

static mut atiixp_modem_driver: pci_driver = pci_driver {
    name: KBUILD_MODNAME,
    id_table: snd_atiixp_ids.as_ptr(),
    probe: Some(snd_atiixp_probe),
    driver: pci_driver_inner {
        pm: &snd_atiixp_pm,
    },
};

/* module_pci_driver(atiixp_modem_driver); */

#[repr(C)]
struct snd_dma_buffer {
    area: *mut c_void,
    addr: c_ulong,
}

#[repr(C)]
struct snd_pcm_substream {
    runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
struct snd_pcm_runtime {
    dma_addr: c_ulong,
    private_data: *mut c_void,
    hw: snd_pcm_hardware,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct snd_pcm_hardware {
    info: c_uint,
    formats: c_uint,
    rates: c_uint,
    rate_min: c_uint,
    rate_max: c_uint,
    channels_min: c_uint,
    channels_max: c_uint,
    buffer_bytes_max: c_uint,
    period_bytes_min: c_uint,
    period_bytes_max: c_uint,
    periods_min: c_uint,
    periods_max: c_uint,
}

#[repr(C)]
struct snd_pcm_hw_params;

#[repr(C)]
struct snd_pcm_hw_constraint_list {
    count: c_uint,
    list: *const c_uint,
    mask: c_uint,
}

#[repr(C)]
struct snd_pcm_ops {
    open: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    close: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
    hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>,
    pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> snd_pcm_uframes_t>,
}

#[repr(C)]
struct snd_card {
    dev: *mut device,
    private_data: *mut c_void,
    sync_irq: c_int,
    private_free: Option<unsafe extern "C" fn(*mut snd_card)>,
    driver: [c_char; 32],
    shortname: [c_char; 32],
    longname: [c_char; 80],
}

#[repr(C)]
struct pci_dev {
    dev: device,
    irq: c_int,
    revision: u8,
}

#[repr(C)]
struct device;
#[repr(C)]
struct resource;
#[repr(C)]
struct snd_ac97_bus {
    clock: c_int,
}
#[repr(C)]
struct snd_ac97 {
    private_data: *mut c_void,
    num: c_int,
}
#[repr(C)]
struct ac97_pcm;
#[repr(C)]
struct snd_pcm {
    dev_class: c_int,
    private_data: *mut c_void,
    name: [c_char; 80],
}
#[repr(C)]
struct mutex;
#[repr(C)]
struct snd_info_entry {
    private_data: *mut c_void,
}
#[repr(C)]
struct snd_info_buffer;
#[repr(C)]
struct snd_ac97_template {
    private_data: *mut c_void,
    pci: *mut pci_dev,
    num: c_int,
    scaps: c_uint,
}
#[repr(C)]
struct snd_ac97_bus_ops {
    write: Option<unsafe extern "C" fn(*mut snd_ac97, u16, u16)>,
    read: Option<unsafe extern "C" fn(*mut snd_ac97, u16) -> u16>,
}
#[repr(C)]
struct dev_pm_ops;

unsafe extern "C" {
    static SNDRV_DEFAULT_STR1: *const c_char;
    static KBUILD_MODNAME: *const c_char;
    static THIS_MODULE: *mut c_void;

    fn readl(addr: *mut c_void) -> c_uint;
    fn writel(value: c_uint, addr: *mut c_void);
    fn udelay(usecs: c_uint);
    fn msleep(msecs: c_uint);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn snd_dma_alloc_pages(
        type_: c_int,
        device: *mut device,
        size: usize,
        dmab: *mut snd_dma_buffer,
    ) -> c_int;
    fn snd_dma_free_pages(dmab: *mut snd_dma_buffer);
    fn cpu_to_le32(x: u32) -> __le32;
    fn spin_lock_irqsave(lock: *mut spinlock_t);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t);
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn spin_lock_irq(lock: *mut spinlock_t);
    fn spin_unlock_irq(lock: *mut spinlock_t);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn mutex_init(mutex: *mut mutex);
    fn mutex_lock(mutex: *mut mutex);
    fn mutex_unlock(mutex: *mut mutex);
    fn snd_pcm_substream_chip(substream: *mut snd_pcm_substream) -> *mut c_void;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, bytes: c_uint) -> snd_pcm_uframes_t;
    fn snd_pcm_stop_xrun(substream: *mut snd_pcm_substream);
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_BUG_ON(condition: bool) -> c_int;
    fn params_buffer_bytes(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_periods(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_period_bytes(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_ac97_write(ac97: *mut snd_ac97, reg: u16, value: c_uint);
    fn snd_pcm_hw_constraint_list(
        runtime: *mut snd_pcm_runtime,
        cond: c_uint,
        var: c_uint,
        l: *const snd_pcm_hw_constraint_list,
    ) -> c_int;
    fn snd_pcm_hw_constraint_integer(runtime: *mut snd_pcm_runtime, var: c_uint) -> c_int;
    fn snd_pcm_new(
        card: *mut snd_card,
        id: *const c_char,
        device: c_int,
        playback_count: c_int,
        capture_count: c_int,
        rpcm: *mut *mut snd_pcm,
    ) -> c_int;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, direction: c_int, ops: *const snd_pcm_ops);
    fn strscpy(dst: *mut c_char, src: *const c_char);
    fn snd_pcm_set_managed_buffer_all(
        pcm: *mut snd_pcm,
        type_: c_int,
        data: *mut device,
        min: usize,
        max: usize,
    );
    fn snd_ac97_bus(
        card: *mut snd_card,
        num: c_int,
        ops: *const snd_ac97_bus_ops,
        private_data: *mut c_void,
        rbus: *mut *mut snd_ac97_bus,
    ) -> c_int;
    fn snd_ac97_mixer(
        bus: *mut snd_ac97_bus,
        template: *mut snd_ac97_template,
        rac97: *mut *mut snd_ac97,
    ) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn snd_power_change_state(card: *mut snd_card, state: c_int);
    fn snd_ac97_suspend(ac97: *mut snd_ac97);
    fn snd_ac97_resume(ac97: *mut snd_ac97);
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...);
    fn snd_card_ro_proc_new(
        card: *mut snd_card,
        name: *const c_char,
        data: *mut c_void,
        read: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
    );
    fn pcim_enable_device(pci: *mut pci_dev) -> c_int;
    fn pcim_iomap_region(pci: *mut pci_dev, bar: c_int, name: *const c_char) -> *mut c_void;
    fn IS_ERR(ptr: *mut c_void) -> bool;
    fn PTR_ERR(ptr: *mut c_void) -> c_int;
    fn pci_resource_start(pci: *mut pci_dev, bar: c_int) -> c_ulong;
    fn devm_request_irq(
        dev: *mut device,
        irq: c_int,
        handler: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
        flags: c_ulong,
        name: *const c_char,
        dev_id: *mut c_void,
    ) -> c_int;
    fn pci_set_master(pci: *mut pci_dev);
    fn snd_devm_card_new(
        dev: *mut device,
        idx: c_int,
        xid: *mut c_char,
        module: *mut c_void,
        extra_size: usize,
        card_ret: *mut *mut snd_card,
    ) -> c_int;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...);
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn pci_set_drvdata(pci: *mut pci_dev, data: *mut c_void);
    fn snd_card_free_on_error(dev: *mut device, ret: c_int) -> c_int;
}

const fn PAGE_ALIGN(size: usize) -> usize {
    (size + 4095) & !4095
}

const fn DEFINE_SIMPLE_DEV_PM_OPS(
    _suspend: unsafe extern "C" fn(*mut device) -> c_int,
    _resume: unsafe extern "C" fn(*mut device) -> c_int,
) -> dev_pm_ops {
    dev_pm_ops
}

const ARRAY_SIZE_4: c_uint = 4;
const ENOMEM: c_int = 12;
const EBUSY: c_int = 16;
const EINVAL: c_int = 22;
const ENXIO: c_int = 6;
const ENODEV: c_int = 19;
const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const IRQF_SHARED: c_ulong = 0x80;
const SNDRV_DMA_TYPE_DEV: c_int = 1;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_INFO_MMAP: c_uint = 1 << 0;
const SNDRV_PCM_INFO_INTERLEAVED: c_uint = 1 << 1;
const SNDRV_PCM_INFO_BLOCK_TRANSFER: c_uint = 1 << 2;
const SNDRV_PCM_INFO_MMAP_VALID: c_uint = 1 << 3;
const SNDRV_PCM_FMTBIT_S16_LE: c_uint = 1 << 0;
const SNDRV_PCM_RATE_8000: c_uint = 1 << 0;
const SNDRV_PCM_RATE_16000: c_uint = 1 << 1;
const SNDRV_PCM_RATE_KNOT: c_uint = 1 << 2;
const SNDRV_PCM_HW_PARAM_RATE: c_uint = 0;
const SNDRV_PCM_HW_PARAM_PERIODS: c_uint = 1;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SNDRV_PCM_CLASS_MODEM: c_int = 2;
const AC97_GPIO_STATUS: u16 = 0x54;
const AC97_LINE1_RATE: u16 = 0x5c;
const AC97_LINE1_LEVEL: u16 = 0x5a;
const AC97_SCAP_SKIP_AUDIO: c_uint = 1 << 0;
const AC97_SCAP_POWER_SAVE: c_uint = 1 << 1;
const SNDRV_CTL_POWER_D3hot: c_int = 3;
const SNDRV_CTL_POWER_D0: c_int = 0;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
