// SPDX-License-Identifier: GPL-2.0-only
/*****************************************************************************
 *
 * Copyright (C) 2008 Cedric Bregardis <cedric.bregardis@free.fr> and
 * Jean-Christian Hassler <jhassler@free.fr>
 *
 * This file is part of the Audiowerk2 ALSA driver
 *
 *****************************************************************************/

// Rust translation of aw2-saa7146.c. C includes and the AW2_SAA7146_M marker
// are build-system/header concerns; external symbols are expected from the
// translated kernel and driver dependencies.

use core::ffi::c_void;

pub type snd_aw2_saa7146_it_cb = Option<unsafe extern "C" fn(*mut c_void)>;
pub type irqreturn_t = c_int;
pub type c_int = i32;

#[repr(C)]
pub struct snd_aw2_saa7146 {
    pub base_addr: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_aw2_saa7146_cb_param {
    pub p_it_callback: snd_aw2_saa7146_it_cb,
    pub p_callback_param: *mut c_void,
}

const EMPTY_CB_PARAM: snd_aw2_saa7146_cb_param = snd_aw2_saa7146_cb_param {
    p_it_callback: None,
    p_callback_param: core::ptr::null_mut(),
};

extern "C" {
    fn writel(value: u32, addr: *mut c_void);
    fn readl(addr: *mut c_void) -> u32;
    fn pr_err(fmt: *const u8, ...);
}

extern "C" {
    static tsl1: [u32; 8];
    static tsl2: [u32; 8];
}

const NB_STREAM_PLAYBACK: usize = 2;
const NB_STREAM_CAPTURE: usize = 1;

extern "Rust" {
    static IER: usize;
    static MC1: usize;
    static ACON1: usize;
    static PCI_BT_A: usize;
    static ACON2: usize;
    static TSL1: usize;
    static TSL2: usize;
    static PageA2_out: usize;
    static BaseA2_out: usize;
    static ProtA2_out: usize;
    static PageA1_out: usize;
    static BaseA1_out: usize;
    static ProtA1_out: usize;
    static PageA1_in: usize;
    static BaseA1_in: usize;
    static ProtA1_in: usize;
    static ISR: usize;
    static IICSTA: usize;
    static PCI_ADP3: usize;
    static PCI_ADP1: usize;
    static PCI_ADP2: usize;
    static GPIO_CTRL: usize;

    static MRST_N: u32;
    static A1_SWAP: u32;
    static A2_SWAP: u32;
    static WS1_CTRL: u32;
    static WS2_CTRL: u32;
    static WS3_CTRL: u32;
    static WS4_CTRL: u32;
    static AUDIO_MODE: u32;
    static BurstA1_in: u32;
    static ThreshA1_in: u32;
    static BurstA1_out: u32;
    static ThreshA1_out: u32;
    static BurstA2_out: u32;
    static ThreshA2_out: u32;
    static EAP: u32;
    static EI2C: u32;
    static A1_out: u32;
    static A2_out: u32;
    static A1_in: u32;
    static IIC_S: u32;
    static IIC_E: u32;
    static A2_CLKSRC: u32;
    static BCLK1_OEN: u32;
    static TR_E_A2_OUT: u32;
    static TR_E_A1_OUT: u32;
    static TR_E_A1_IN: u32;
    static IRQ_NONE: irqreturn_t;
    static IRQ_HANDLED: irqreturn_t;
}

static mut arr_substream_it_playback_cb: [snd_aw2_saa7146_cb_param; NB_STREAM_PLAYBACK] =
    [EMPTY_CB_PARAM; NB_STREAM_PLAYBACK];
static mut arr_substream_it_capture_cb: [snd_aw2_saa7146_cb_param; NB_STREAM_CAPTURE] =
    [EMPTY_CB_PARAM; NB_STREAM_CAPTURE];

#[inline]
unsafe fn WRITEREG(chip: *mut snd_aw2_saa7146, value: u32, addr: usize) {
    writel(value, ((*chip).base_addr as *mut u8).add(addr) as *mut c_void);
}

#[inline]
unsafe fn READREG(chip: *mut snd_aw2_saa7146, addr: usize) -> u32 {
    readl(((*chip).base_addr as *mut u8).add(addr) as *mut c_void)
}

static mut snd_aw2_saa7146_get_limit_decl: Option<unsafe fn(c_int) -> c_int> =
    Some(snd_aw2_saa7146_get_limit);

/* chip-specific destructor */
pub unsafe extern "C" fn snd_aw2_saa7146_free(chip: *mut snd_aw2_saa7146) -> c_int {
    /* disable all irqs */
    WRITEREG(chip, 0, IER);

    /* reset saa7146 */
    WRITEREG(chip, MRST_N << 16, MC1);

    /* Unset base addr */
    (*chip).base_addr = core::ptr::null_mut();

    0
}

pub unsafe extern "C" fn snd_aw2_saa7146_setup(
    chip: *mut snd_aw2_saa7146,
    pci_base_addr: *mut c_void,
) {
    /* set PCI burst/threshold

       Burst length definition
       VALUE    BURST LENGTH
       000      1 Dword
       001      2 Dwords
       010      4 Dwords
       011      8 Dwords
       100      16 Dwords
       101      32 Dwords
       110      64 Dwords
       111      128 Dwords

       Threshold definition
       VALUE    WRITE MODE              READ MODE
       00       1 Dword of valid data   1 empty Dword
       01       4 Dwords of valid data  4 empty Dwords
       10       8 Dwords of valid data  8 empty Dwords
       11       16 Dwords of valid data 16 empty Dwords */

    let mut acon2: u32;
    let mut acon1: u32 = 0;
    let mut i: c_int;

    /* Set base addr */
    (*chip).base_addr = pci_base_addr;

    /* disable all irqs */
    WRITEREG(chip, 0, IER);

    /* reset saa7146 */
    WRITEREG(chip, MRST_N << 16, MC1);

    /* enable audio interface */
    #[cfg(target_endian = "big")]
    {
        acon1 |= A1_SWAP;
        acon1 |= A2_SWAP;
    }
    /* WS0_CTRL, WS0_SYNC: input TSL1, I2S */

    /* At initialization WS1 and WS2 are disabled (configured as input) */
    acon1 |= 0 * WS1_CTRL;
    acon1 |= 0 * WS2_CTRL;

    /* WS4 is not used. So it must not restart A2.
       This is why it is configured as output (force to low) */
    acon1 |= 3 * WS4_CTRL;

    /* WS3_CTRL, WS3_SYNC: output TSL2, I2S */
    acon1 |= 2 * WS3_CTRL;

    /* A1 and A2 are active and asynchronous */
    acon1 |= 3 * AUDIO_MODE;
    WRITEREG(chip, acon1, ACON1);

    /* The following comes from original windows driver.
       It is needed to have a correct behavior of input and output
       simultenously, but I don't know why ! */
    WRITEREG(
        chip,
        3 * BurstA1_in
            + 3 * ThreshA1_in
            + 3 * BurstA1_out
            + 3 * ThreshA1_out
            + 3 * BurstA2_out
            + 3 * ThreshA2_out,
        PCI_BT_A,
    );

    /* enable audio port pins */
    WRITEREG(chip, (EAP << 16) | EAP, MC1);

    /* enable I2C */
    WRITEREG(chip, (EI2C << 16) | EI2C, MC1);
    /* enable interrupts */
    WRITEREG(chip, A1_out | A2_out | A1_in | IIC_S | IIC_E, IER);

    /* audio configuration */
    acon2 = A2_CLKSRC | BCLK1_OEN;
    WRITEREG(chip, acon2, ACON2);

    /* By default use analog input */
    snd_aw2_saa7146_use_digital_input(chip, 0);

    /* TSL setup */
    i = 0;
    while i < 8 {
        WRITEREG(chip, tsl1[i as usize], TSL1 + ((i * 4) as usize));
        WRITEREG(chip, tsl2[i as usize], TSL2 + ((i * 4) as usize));
        i += 1;
    }
}

pub unsafe extern "C" fn snd_aw2_saa7146_pcm_init_playback(
    chip: *mut snd_aw2_saa7146,
    stream_number: c_int,
    dma_addr: c_ulong,
    period_size: c_ulong,
    buffer_size: c_ulong,
) {
    let mut dw_page: c_ulong;
    let dw_limit: c_ulong;

    /* Configure DMA for substream
       Configuration informations: ALSA has allocated continuous memory
       pages. So we don't need to use MMU of saa7146.
     */

    /* No MMU -> nothing to do with PageA1, we only configure the limit of
       PageAx_out register */
    /* Disable MMU */
    dw_page = 0 << 11;

    /* Configure Limit for DMA access.
       The limit register defines an address limit, which generates
       an interrupt if passed by the actual PCI address pointer.
       '0001' means an interrupt will be generated if the lower
       6 bits (64 bytes) of the PCI address are zero. '0010'
       defines a limit of 128 bytes, '0011' one of 256 bytes, and
       so on up to 1 Mbyte defined by '1111'. This interrupt range
       can be calculated as follows:
       Range = 2^(5 + Limit) bytes.
     */
    dw_limit = snd_aw2_saa7146_get_limit(period_size as c_int) as c_ulong;
    dw_page |= dw_limit << 4;

    if stream_number == 0 {
        WRITEREG(chip, dw_page as u32, PageA2_out);

        /* Base address for DMA transfert. */
        /* This address has been reserved by ALSA. */
        /* This is a physical address */
        WRITEREG(chip, dma_addr as u32, BaseA2_out);

        /* Define upper limit for DMA access */
        WRITEREG(chip, dma_addr.wrapping_add(buffer_size) as u32, ProtA2_out);
    } else if stream_number == 1 {
        WRITEREG(chip, dw_page as u32, PageA1_out);

        /* Base address for DMA transfert. */
        /* This address has been reserved by ALSA. */
        /* This is a physical address */
        WRITEREG(chip, dma_addr as u32, BaseA1_out);

        /* Define upper limit for DMA access */
        WRITEREG(chip, dma_addr.wrapping_add(buffer_size) as u32, ProtA1_out);
    } else {
        pr_err(
            b"aw2: snd_aw2_saa7146_pcm_init_playback: Substream number is not 0 or 1 -> not managed\n\0"
                .as_ptr(),
        );
    }
}

pub type c_ulong = usize;
pub type c_long = isize;
pub type size_t = usize;

pub unsafe extern "C" fn snd_aw2_saa7146_pcm_init_capture(
    chip: *mut snd_aw2_saa7146,
    stream_number: c_int,
    dma_addr: c_ulong,
    period_size: c_ulong,
    buffer_size: c_ulong,
) {
    let mut dw_page: c_ulong;
    let dw_limit: c_ulong;

    /* Configure DMA for substream
       Configuration informations: ALSA has allocated continuous memory
       pages. So we don't need to use MMU of saa7146.
     */

    /* No MMU -> nothing to do with PageA1, we only configure the limit of
       PageAx_out register */
    /* Disable MMU */
    dw_page = 0 << 11;

    /* Configure Limit for DMA access.
       The limit register defines an address limit, which generates
       an interrupt if passed by the actual PCI address pointer.
       '0001' means an interrupt will be generated if the lower
       6 bits (64 bytes) of the PCI address are zero. '0010'
       defines a limit of 128 bytes, '0011' one of 256 bytes, and
       so on up to 1 Mbyte defined by '1111'. This interrupt range
       can be calculated as follows:
       Range = 2^(5 + Limit) bytes.
     */
    dw_limit = snd_aw2_saa7146_get_limit(period_size as c_int) as c_ulong;
    dw_page |= dw_limit << 4;

    if stream_number == 0 {
        WRITEREG(chip, dw_page as u32, PageA1_in);

        /* Base address for DMA transfert. */
        /* This address has been reserved by ALSA. */
        /* This is a physical address */
        WRITEREG(chip, dma_addr as u32, BaseA1_in);

        /* Define upper limit for DMA access  */
        WRITEREG(chip, dma_addr.wrapping_add(buffer_size) as u32, ProtA1_in);
    } else {
        pr_err(
            b"aw2: snd_aw2_saa7146_pcm_init_capture: Substream number is not 0 -> not managed\n\0"
                .as_ptr(),
        );
    }
}

pub unsafe extern "C" fn snd_aw2_saa7146_define_it_playback_callback(
    stream_number: u32,
    p_it_callback: snd_aw2_saa7146_it_cb,
    p_callback_param: *mut c_void,
) {
    if (stream_number as usize) < NB_STREAM_PLAYBACK {
        arr_substream_it_playback_cb[stream_number as usize].p_it_callback =
            p_it_callback as snd_aw2_saa7146_it_cb;
        arr_substream_it_playback_cb[stream_number as usize].p_callback_param =
            p_callback_param as *mut c_void;
    }
}

pub unsafe extern "C" fn snd_aw2_saa7146_define_it_capture_callback(
    stream_number: u32,
    p_it_callback: snd_aw2_saa7146_it_cb,
    p_callback_param: *mut c_void,
) {
    if (stream_number as usize) < NB_STREAM_CAPTURE {
        arr_substream_it_capture_cb[stream_number as usize].p_it_callback =
            p_it_callback as snd_aw2_saa7146_it_cb;
        arr_substream_it_capture_cb[stream_number as usize].p_callback_param =
            p_callback_param as *mut c_void;
    }
}

pub unsafe extern "C" fn snd_aw2_saa7146_pcm_trigger_start_playback(
    chip: *mut snd_aw2_saa7146,
    stream_number: c_int,
) {
    let mut acon1: u32 = 0;
    /* In aw8 driver, dma transfert is always active. It is
       started and stopped in a larger "space" */
    acon1 = READREG(chip, ACON1);
    if stream_number == 0 {
        WRITEREG(chip, (TR_E_A2_OUT << 16) | TR_E_A2_OUT, MC1);

        /* WS2_CTRL, WS2_SYNC: output TSL2, I2S */
        acon1 |= 2 * WS2_CTRL;
        WRITEREG(chip, acon1, ACON1);
    } else if stream_number == 1 {
        WRITEREG(chip, (TR_E_A1_OUT << 16) | TR_E_A1_OUT, MC1);

        /* WS1_CTRL, WS1_SYNC: output TSL1, I2S */
        acon1 |= WS1_CTRL;
        WRITEREG(chip, acon1, ACON1);
    }
}

pub unsafe extern "C" fn snd_aw2_saa7146_pcm_trigger_stop_playback(
    chip: *mut snd_aw2_saa7146,
    stream_number: c_int,
) {
    let mut acon1: u32 = 0;
    acon1 = READREG(chip, ACON1);
    if stream_number == 0 {
        /* WS2_CTRL, WS2_SYNC: output TSL2, I2S */
        acon1 &= !(3 * WS2_CTRL);
        WRITEREG(chip, acon1, ACON1);

        WRITEREG(chip, TR_E_A2_OUT << 16, MC1);
    } else if stream_number == 1 {
        /* WS1_CTRL, WS1_SYNC: output TSL1, I2S */
        acon1 &= !(3 * WS1_CTRL);
        WRITEREG(chip, acon1, ACON1);

        WRITEREG(chip, TR_E_A1_OUT << 16, MC1);
    }
}

pub unsafe extern "C" fn snd_aw2_saa7146_pcm_trigger_start_capture(
    chip: *mut snd_aw2_saa7146,
    stream_number: c_int,
) {
    /* In aw8 driver, dma transfert is always active. It is
       started and stopped in a larger "space" */
    if stream_number == 0 {
        WRITEREG(chip, (TR_E_A1_IN << 16) | TR_E_A1_IN, MC1);
    }
}

pub unsafe extern "C" fn snd_aw2_saa7146_pcm_trigger_stop_capture(
    chip: *mut snd_aw2_saa7146,
    stream_number: c_int,
) {
    if stream_number == 0 {
        WRITEREG(chip, TR_E_A1_IN << 16, MC1);
    }
}

pub unsafe extern "C" fn snd_aw2_saa7146_interrupt(
    _irq: c_int,
    dev_id: *mut c_void,
) -> irqreturn_t {
    let isr: u32;
    let mut _iicsta: u32;
    let chip: *mut snd_aw2_saa7146 = dev_id as *mut snd_aw2_saa7146;

    isr = READREG(chip, ISR);
    if isr == 0 {
        return IRQ_NONE;
    }

    WRITEREG(chip, isr, ISR);

    if (isr & (IIC_S | IIC_E)) != 0 {
        _iicsta = READREG(chip, IICSTA);
        WRITEREG(chip, 0x100, IICSTA);
    }

    if (isr & A1_out) != 0 {
        if arr_substream_it_playback_cb[1].p_it_callback.is_some() {
            (arr_substream_it_playback_cb[1].p_it_callback.unwrap())(
                arr_substream_it_playback_cb[1].p_callback_param,
            );
        }
    }
    if (isr & A2_out) != 0 {
        if arr_substream_it_playback_cb[0].p_it_callback.is_some() {
            (arr_substream_it_playback_cb[0].p_it_callback.unwrap())(
                arr_substream_it_playback_cb[0].p_callback_param,
            );
        }
    }
    if (isr & A1_in) != 0 {
        if arr_substream_it_capture_cb[0].p_it_callback.is_some() {
            (arr_substream_it_capture_cb[0].p_it_callback.unwrap())(
                arr_substream_it_capture_cb[0].p_callback_param,
            );
        }
    }
    IRQ_HANDLED
}

pub unsafe extern "C" fn snd_aw2_saa7146_get_hw_ptr_playback(
    chip: *mut snd_aw2_saa7146,
    stream_number: c_int,
    start_addr: *mut u8,
    buffer_size: u32,
) -> u32 {
    let mut pci_adp: c_long = 0;
    let mut ptr: size_t = 0;

    if stream_number == 0 {
        pci_adp = READREG(chip, PCI_ADP3) as c_long;
        ptr = pci_adp.wrapping_sub(start_addr as c_long) as size_t;

        if ptr == buffer_size as size_t {
            ptr = 0;
        }
    }
    if stream_number == 1 {
        pci_adp = READREG(chip, PCI_ADP1) as c_long;
        ptr = (pci_adp as size_t).wrapping_sub(start_addr as size_t);

        if ptr == buffer_size as size_t {
            ptr = 0;
        }
    }
    ptr as u32
}

pub unsafe extern "C" fn snd_aw2_saa7146_get_hw_ptr_capture(
    chip: *mut snd_aw2_saa7146,
    stream_number: c_int,
    start_addr: *mut u8,
    buffer_size: u32,
) -> u32 {
    let mut pci_adp: size_t = 0;
    let mut ptr: size_t = 0;
    if stream_number == 0 {
        pci_adp = READREG(chip, PCI_ADP2) as size_t;
        ptr = pci_adp.wrapping_sub(start_addr as size_t);

        if ptr == buffer_size as size_t {
            ptr = 0;
        }
    }
    ptr as u32
}

pub unsafe extern "C" fn snd_aw2_saa7146_use_digital_input(
    chip: *mut snd_aw2_saa7146,
    use_digital: c_int,
) {
    /* FIXME: switch between analog and digital input does not always work.
       It can produce a kind of white noise. It seams that received data
       are inverted sometime (endian inversion). Why ? I don't know, maybe
       a problem of synchronization... However for the time being I have
       not found the problem. Workaround: switch again (and again) between
       digital and analog input until it works. */
    if use_digital != 0 {
        WRITEREG(chip, 0x40, GPIO_CTRL);
    } else {
        WRITEREG(chip, 0x50, GPIO_CTRL);
    }
}

pub unsafe extern "C" fn snd_aw2_saa7146_is_using_digital_input(
    chip: *mut snd_aw2_saa7146,
) -> c_int {
    let reg_val: u32 = READREG(chip, GPIO_CTRL);
    if (reg_val & 0xFF) == 0x40 {
        1
    } else {
        0
    }
}

unsafe fn snd_aw2_saa7146_get_limit(size: c_int) -> c_int {
    let mut limitsize: c_int = 32;
    let mut limit: c_int = 0;
    while limitsize < size {
        limitsize *= 2;
        limit += 1;
    }
    limit
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
