// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   serial.c
 *   Copyright (c) by Jaroslav Kysela <perex@perex.cz>,
 *                    Isaku Yamahata <yamahata@private.email.ne.jp>,
 *		      George Hansper <ghansper@apana.org.au>,
 *		      Hannu Savolainen
 *
 *   This code is based on the code from ALSA 0.5.9, but heavily rewritten.
 *
 * Sat Mar 31 17:27:57 PST 2001 tim.mann@compaq.com
 *      Added support for the Midiator MS-124T and for the MS-124W in
 *      Single Addressed (S/A) or Multiple Burst (M/B) mode, with
 *      power derived either parasitically from the serial port or
 *      from a separate power supply.
 *
 *      More documentation can be found in serial-u16550.txt.
 */

use core::ffi::{c_char, c_int, c_long, c_ulong, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

const SNDRV_SERIAL_SOUNDCANVAS: c_int = 0; /* Roland Soundcanvas; F5 NN selects part */
const SNDRV_SERIAL_MS124T: c_int = 1; /* Midiator MS-124T */
const SNDRV_SERIAL_MS124W_SA: c_int = 2; /* Midiator MS-124W in S/A mode */
const SNDRV_SERIAL_MS124W_MB: c_int = 3; /* Midiator MS-124W in M/B mode */
const SNDRV_SERIAL_GENERIC: c_int = 4; /* Generic Interface */
const SNDRV_SERIAL_MAX_ADAPTOR: c_int = SNDRV_SERIAL_GENERIC;

static ADAPTOR_NAMES: [*const c_char; 5] = [
    b"Soundcanvas\0".as_ptr() as *const c_char,
    b"MS-124T\0".as_ptr() as *const c_char,
    b"MS-124W S/A\0".as_ptr() as *const c_char,
    b"MS-124W M/B\0".as_ptr() as *const c_char,
    b"Generic\0".as_ptr() as *const c_char,
];

const SNDRV_SERIAL_NORMALBUFF: bool = false; /* Normal blocking buffer operation */
const SNDRV_SERIAL_DROPBUFF: bool = true; /* Non-blocking discard operation */

static mut INDEX: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IDX;
static mut ID: [*mut c_char; SNDRV_CARDS] = SNDRV_DEFAULT_STR;
static mut ENABLE: [bool; SNDRV_CARDS] = SNDRV_DEFAULT_ENABLE;
static mut PORT: [c_long; SNDRV_CARDS] = SNDRV_DEFAULT_PORT;
static mut IRQ: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IRQ;
static mut SPEED: [c_int; SNDRV_CARDS] = [38400; SNDRV_CARDS];
static mut BASE: [c_int; SNDRV_CARDS] = [115200; SNDRV_CARDS];
static mut OUTS: [c_int; SNDRV_CARDS] = [1; SNDRV_CARDS];
static mut INS: [c_int; SNDRV_CARDS] = [1; SNDRV_CARDS];
static mut ADAPTOR: [c_int; SNDRV_CARDS] = [SNDRV_SERIAL_SOUNDCANVAS; SNDRV_CARDS];
static mut DROPONFULL: [bool; SNDRV_CARDS] = [SNDRV_SERIAL_NORMALBUFF; SNDRV_CARDS];

/* module_param_array/module_param_hw_array/MODULE_PARM_DESC declarations are Linux module metadata. */
/*#define SNDRV_SERIAL_MS124W_MB_NOCOMBO 1*/ /* Address outs as 0-3 instead of bitmap */

const SNDRV_SERIAL_MAX_OUTS: usize = 16; /* max 64, min 16 */
const SNDRV_SERIAL_MAX_INS: usize = 16; /* max 64, min 16 */

const TX_BUFF_SIZE: usize = 1 << 15; /* Must be 2^n */
const TX_BUFF_MASK: c_int = (TX_BUFF_SIZE as c_int) - 1;

const SERIAL_MODE_NOT_OPENED: c_int = 0;
const SERIAL_MODE_INPUT_OPEN: c_int = 1 << 0;
const SERIAL_MODE_OUTPUT_OPEN: c_int = 1 << 1;
const SERIAL_MODE_INPUT_TRIGGERED: c_int = 1 << 2;
const SERIAL_MODE_OUTPUT_TRIGGERED: c_int = 1 << 3;

#[repr(C)]
struct SndUart16550 {
    card: *mut snd_card,
    rmidi: *mut snd_rawmidi,
    midi_output: [*mut snd_rawmidi_substream; SNDRV_SERIAL_MAX_OUTS],
    midi_input: [*mut snd_rawmidi_substream; SNDRV_SERIAL_MAX_INS],
    filemode: c_int,
    open_lock: spinlock_t,
    irq: c_int,
    base: c_ulong,
    speed: c_uint,
    speed_base: c_uint,
    divisor: u8,
    old_divisor_lsb: u8,
    old_divisor_msb: u8,
    old_line_ctrl_reg: u8,
    fifo_limit: i16, /* used in uart16550 */
    fifo_count: i16, /* used in uart16550 */
    adaptor: c_int,
    prev_in: c_int,
    rstatus: u8,
    prev_out: c_int,
    prev_status: [u8; SNDRV_SERIAL_MAX_OUTS],
    tx_buff: [u8; TX_BUFF_SIZE],
    buff_in_count: c_int,
    buff_in: c_int,
    buff_out: c_int,
    drop_on_full: c_int,
    timer_running: c_uint,
    buffer_timer: timer_list,
}

static mut DEVICES: [*mut platform_device; SNDRV_CARDS] = [ptr::null_mut(); SNDRV_CARDS];

unsafe fn snd_uart16550_add_timer(uart: *mut SndUart16550) {
    if (*uart).timer_running == 0 {
        /* timer 38600bps * 10bit * 16byte */
        mod_timer(&mut (*uart).buffer_timer, jiffies + (HZ + 255) / 256);
        (*uart).timer_running = 1;
    }
}

unsafe fn snd_uart16550_del_timer(uart: *mut SndUart16550) {
    if (*uart).timer_running != 0 {
        timer_delete(&mut (*uart).buffer_timer);
        (*uart).timer_running = 0;
    }
}

/* This macro is only used in snd_uart16550_io_loop */
unsafe fn snd_uart16550_buffer_output(uart: *mut SndUart16550) {
    let mut buff_out = (*uart).buff_out as u16;
    if (*uart).buff_in_count > 0 {
        outb((*uart).tx_buff[buff_out as usize], (*uart).base + UART_TX as c_ulong);
        (*uart).fifo_count += 1;
        buff_out = buff_out.wrapping_add(1);
        buff_out &= TX_BUFF_MASK as u16;
        (*uart).buff_out = buff_out as c_int;
        (*uart).buff_in_count -= 1;
    }
}

/* This loop should be called with interrupts disabled
 * We don't want to interrupt this,
 * as we're already handling an interrupt
 */
unsafe fn snd_uart16550_io_loop(uart: *mut SndUart16550) {
    let mut c: u8;
    let mut status: u8;
    let mut substream: c_int = (*uart).prev_in;

    loop {
        status = inb((*uart).base + UART_LSR as c_ulong);
        if status & UART_LSR_DR == 0 {
            break;
        }
        c = inb((*uart).base + UART_RX as c_ulong);

        if c & 0x80 != 0 {
            (*uart).rstatus = c;
        }

        if (*uart).adaptor == SNDRV_SERIAL_GENERIC {
            if (*uart).rstatus == 0xf5 {
                if c <= SNDRV_SERIAL_MAX_INS as u8 && c > 0 {
                    substream = c as c_int - 1;
                }
                if c != 0xf5 {
                    /* prevent future bytes from being interpreted as streams */
                    (*uart).rstatus = 0;
                }
            } else if ((*uart).filemode & SERIAL_MODE_INPUT_OPEN) != 0
                && !(*uart).midi_input[substream as usize].is_null()
            {
                snd_rawmidi_receive((*uart).midi_input[substream as usize], &mut c, 1);
            }
        } else if ((*uart).filemode & SERIAL_MODE_INPUT_OPEN) != 0
            && !(*uart).midi_input[substream as usize].is_null()
        {
            snd_rawmidi_receive((*uart).midi_input[substream as usize], &mut c, 1);
        }

        if status & UART_LSR_OE != 0 {
            dev_warn(
                (*(*uart).card).dev,
                b"%s: Overrun on device at 0x%lx\n\0".as_ptr() as *const c_char,
                (*(*uart).rmidi).name.as_ptr(),
                (*uart).base,
            );
        }
    }

    (*uart).prev_in = substream;

    if status & UART_LSR_THRE != 0 {
        (*uart).fifo_count = 0;
    }
    if (*uart).adaptor == SNDRV_SERIAL_MS124W_SA || (*uart).adaptor == SNDRV_SERIAL_GENERIC {
        /* Can't use FIFO, must send only when CTS is true */
        status = inb((*uart).base + UART_MSR as c_ulong);
        while (*uart).fifo_count == 0 && (status & UART_MSR_CTS) != 0 && (*uart).buff_in_count > 0 {
            snd_uart16550_buffer_output(uart);
            status = inb((*uart).base + UART_MSR as c_ulong);
        }
    } else {
        /* Write loop */
        while (*uart).fifo_count < (*uart).fifo_limit && (*uart).buff_in_count > 0 {
            snd_uart16550_buffer_output(uart);
        }
    }
    if (*uart).irq < 0 && (*uart).buff_in_count > 0 {
        snd_uart16550_add_timer(uart);
    }
}

/* NOTES ON SERVICING INTERUPTS
 * ---------------------------
 * After receiving a interrupt, it is important to indicate to the UART that
 * this has been done.
 * For a Rx interrupt, this is done by reading the received byte.
 * For a Tx interrupt this is done by either:
 * a) Writing a byte
 * b) Reading the IIR
 * It is particularly important to read the IIR if a Tx interrupt is received
 * when there is no data in tx_buff[], as in this case there no other
 * indication that the interrupt has been serviced, and it remains outstanding
 * indefinitely. This has the curious side effect that and no further interrupts
 * will be generated from this device AT ALL!!.
 * It is also desirable to clear outstanding interrupts when the device is
 * opened/closed.
 *
 *
 * Note that some devices need OUT2 to be set before they will generate
 * interrupts at all. (Possibly tied to an internal pull-up on CTS?)
 */
unsafe extern "C" fn snd_uart16550_interrupt(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let uart = dev_id as *mut SndUart16550;
    spin_lock(&mut (*uart).open_lock);
    if (*uart).filemode == SERIAL_MODE_NOT_OPENED {
        spin_unlock(&mut (*uart).open_lock);
        return IRQ_NONE;
    }
    inb((*uart).base + UART_IIR as c_ulong);
    snd_uart16550_io_loop(uart);
    spin_unlock(&mut (*uart).open_lock);
    IRQ_HANDLED
}

/* When the polling mode, this function calls snd_uart16550_io_loop. */
unsafe extern "C" fn snd_uart16550_buffer_timer(t: *mut timer_list) {
    let uart = timer_container_of_snd_uart16550(t);
    spin_lock_irqsave(&mut (*uart).open_lock);
    snd_uart16550_del_timer(uart);
    snd_uart16550_io_loop(uart);
    spin_unlock_irqrestore(&mut (*uart).open_lock);
}

/*
 *  this method probes, if an uart sits on given port
 *  return 0 if found
 *  return negative error if not found
 */
unsafe fn snd_uart16550_detect(uart: *mut SndUart16550) -> c_int {
    let io_base = (*uart).base;
    let mut ok: c_int;
    let mut c: u8;

    if io_base == 0 || io_base == SNDRV_AUTO_PORT as c_ulong {
        return -ENODEV;
    }

    if devm_request_region((*(*uart).card).dev, io_base, 8, b"Serial MIDI\0".as_ptr() as *const c_char).is_null() {
        dev_err((*(*uart).card).dev, b"u16550: can't grab port 0x%lx\n\0".as_ptr() as *const c_char, io_base);
        return -EBUSY;
    }

    ok = 1;
    outb(UART_LCR_WLEN8, io_base + UART_LCR as c_ulong);
    c = inb(io_base + UART_IER as c_ulong);
    if (c & 0xf0) != 0 {
        ok = 0;
    }

    outb(0xaa, io_base + UART_SCR as c_ulong);
    c = inb(io_base + UART_SCR as c_ulong);
    if c != 0xaa {
        ok = 0;
    }

    outb(0x55, io_base + UART_SCR as c_ulong);
    c = inb(io_base + UART_SCR as c_ulong);
    if c != 0x55 {
        ok = 0;
    }

    ok
}

unsafe fn snd_uart16550_do_open(uart: *mut SndUart16550) {
    let byte: c_char;

    (*uart).buff_in_count = 0;
    (*uart).buff_in = 0;
    (*uart).buff_out = 0;
    (*uart).fifo_limit = 1;
    (*uart).fifo_count = 0;
    (*uart).timer_running = 0;

    outb(UART_FCR_ENABLE_FIFO | UART_FCR_CLEAR_RCVR | UART_FCR_CLEAR_XMIT | UART_FCR_TRIGGER_4, (*uart).base + UART_FCR as c_ulong);

    if (inb((*uart).base + UART_IIR as c_ulong) & 0xf0) == 0xc0 {
        (*uart).fifo_limit = 16;
    }
    if (*uart).divisor != 0 {
        (*uart).old_line_ctrl_reg = inb((*uart).base + UART_LCR as c_ulong);
        outb(UART_LCR_DLAB, (*uart).base + UART_LCR as c_ulong);
        (*uart).old_divisor_lsb = inb((*uart).base + UART_DLL as c_ulong);
        (*uart).old_divisor_msb = inb((*uart).base + UART_DLM as c_ulong);
        outb((*uart).divisor, (*uart).base + UART_DLL as c_ulong);
        outb(0, (*uart).base + UART_DLM as c_ulong);
    }
    outb(UART_LCR_WLEN8 | 0 | 0 | 0, (*uart).base + UART_LCR as c_ulong);

    match (*uart).adaptor {
        SNDRV_SERIAL_MS124W_SA | SNDRV_SERIAL_MS124W_MB => {
            outb(UART_MCR_RTS | (0 & UART_MCR_DTR) | UART_MCR_OUT2, (*uart).base + UART_MCR as c_ulong);
        }
        SNDRV_SERIAL_MS124T => {
            outb(UART_MCR_RTS | UART_MCR_DTR | UART_MCR_OUT2, (*uart).base + UART_MCR as c_ulong);
        }
        _ => {
            outb(UART_MCR_RTS | UART_MCR_DTR | UART_MCR_OUT2, (*uart).base + UART_MCR as c_ulong);
        }
    }

    if (*uart).irq < 0 {
        byte = ((0 & UART_IER_RDI) | (0 & UART_IER_THRI)) as c_char;
    } else if (*uart).adaptor == SNDRV_SERIAL_MS124W_SA {
        byte = (UART_IER_RDI | UART_IER_MSI) as c_char;
    } else if (*uart).adaptor == SNDRV_SERIAL_GENERIC {
        byte = (UART_IER_RDI | UART_IER_MSI | UART_IER_THRI) as c_char;
    } else {
        byte = (UART_IER_RDI | UART_IER_THRI) as c_char;
    }
    outb(byte as u8, (*uart).base + UART_IER as c_ulong);

    inb((*uart).base + UART_LSR as c_ulong);
    inb((*uart).base + UART_IIR as c_ulong);
    inb((*uart).base + UART_RX as c_ulong);
}

unsafe fn snd_uart16550_do_close(uart: *mut SndUart16550) {
    if (*uart).irq < 0 {
        snd_uart16550_del_timer(uart);
    }

    outb((0 & UART_IER_RDI) | (0 & UART_IER_THRI), (*uart).base + UART_IER as c_ulong);

    match (*uart).adaptor {
        SNDRV_SERIAL_MS124W_SA | SNDRV_SERIAL_MS124W_MB => {
            outb(UART_MCR_RTS | (0 & UART_MCR_DTR) | (0 & UART_MCR_OUT2), (*uart).base + UART_MCR as c_ulong);
        }
        SNDRV_SERIAL_MS124T => {
            outb(UART_MCR_RTS | UART_MCR_DTR | (0 & UART_MCR_OUT2), (*uart).base + UART_MCR as c_ulong);
        }
        _ => {
            outb((0 & UART_MCR_RTS) | (0 & UART_MCR_DTR) | (0 & UART_MCR_OUT2), (*uart).base + UART_MCR as c_ulong);
        }
    }

    inb((*uart).base + UART_IIR as c_ulong);

    if (*uart).divisor != 0 {
        outb(UART_LCR_DLAB, (*uart).base + UART_LCR as c_ulong);
        outb((*uart).old_divisor_lsb, (*uart).base + UART_DLL as c_ulong);
        outb((*uart).old_divisor_msb, (*uart).base + UART_DLM as c_ulong);
        outb((*uart).old_line_ctrl_reg, (*uart).base + UART_LCR as c_ulong);
    }
}

unsafe extern "C" fn snd_uart16550_input_open(substream: *mut snd_rawmidi_substream) -> c_int {
    let uart = (*(*substream).rmidi).private_data as *mut SndUart16550;
    spin_lock_irqsave(&mut (*uart).open_lock);
    if (*uart).filemode == SERIAL_MODE_NOT_OPENED {
        snd_uart16550_do_open(uart);
    }
    (*uart).filemode |= SERIAL_MODE_INPUT_OPEN;
    (*uart).midi_input[(*substream).number as usize] = substream;
    spin_unlock_irqrestore(&mut (*uart).open_lock);
    0
}

unsafe extern "C" fn snd_uart16550_input_close(substream: *mut snd_rawmidi_substream) -> c_int {
    let uart = (*(*substream).rmidi).private_data as *mut SndUart16550;
    spin_lock_irqsave(&mut (*uart).open_lock);
    (*uart).filemode &= !SERIAL_MODE_INPUT_OPEN;
    (*uart).midi_input[(*substream).number as usize] = ptr::null_mut();
    if (*uart).filemode == SERIAL_MODE_NOT_OPENED {
        snd_uart16550_do_close(uart);
    }
    spin_unlock_irqrestore(&mut (*uart).open_lock);
    0
}

unsafe extern "C" fn snd_uart16550_input_trigger(substream: *mut snd_rawmidi_substream, up: c_int) {
    let uart = (*(*substream).rmidi).private_data as *mut SndUart16550;
    spin_lock_irqsave(&mut (*uart).open_lock);
    if up != 0 {
        (*uart).filemode |= SERIAL_MODE_INPUT_TRIGGERED;
    } else {
        (*uart).filemode &= !SERIAL_MODE_INPUT_TRIGGERED;
    }
    spin_unlock_irqrestore(&mut (*uart).open_lock);
}

unsafe extern "C" fn snd_uart16550_output_open(substream: *mut snd_rawmidi_substream) -> c_int {
    let uart = (*(*substream).rmidi).private_data as *mut SndUart16550;
    spin_lock_irqsave(&mut (*uart).open_lock);
    if (*uart).filemode == SERIAL_MODE_NOT_OPENED {
        snd_uart16550_do_open(uart);
    }
    (*uart).filemode |= SERIAL_MODE_OUTPUT_OPEN;
    (*uart).midi_output[(*substream).number as usize] = substream;
    spin_unlock_irqrestore(&mut (*uart).open_lock);
    0
}

unsafe extern "C" fn snd_uart16550_output_close(substream: *mut snd_rawmidi_substream) -> c_int {
    let uart = (*(*substream).rmidi).private_data as *mut SndUart16550;
    spin_lock_irqsave(&mut (*uart).open_lock);
    (*uart).filemode &= !SERIAL_MODE_OUTPUT_OPEN;
    (*uart).midi_output[(*substream).number as usize] = ptr::null_mut();
    if (*uart).filemode == SERIAL_MODE_NOT_OPENED {
        snd_uart16550_do_close(uart);
    }
    spin_unlock_irqrestore(&mut (*uart).open_lock);
    0
}

unsafe fn snd_uart16550_buffer_can_write(uart: *mut SndUart16550, num: c_int) -> c_int {
    if (*uart).buff_in_count + num < TX_BUFF_SIZE as c_int { 1 } else { 0 }
}

unsafe fn snd_uart16550_write_buffer(uart: *mut SndUart16550, byte: u8) -> c_int {
    let mut buff_in = (*uart).buff_in as u16;
    if (*uart).buff_in_count < TX_BUFF_SIZE as c_int {
        (*uart).tx_buff[buff_in as usize] = byte;
        buff_in = buff_in.wrapping_add(1);
        buff_in &= TX_BUFF_MASK as u16;
        (*uart).buff_in = buff_in as c_int;
        (*uart).buff_in_count += 1;
        if (*uart).irq < 0 {
            snd_uart16550_add_timer(uart);
        }
        1
    } else {
        0
    }
}

unsafe fn snd_uart16550_output_byte(
    uart: *mut SndUart16550,
    _substream: *mut snd_rawmidi_substream,
    midi_byte: u8,
) -> c_int {
    if (*uart).buff_in_count == 0
        && (((*uart).adaptor != SNDRV_SERIAL_MS124W_SA && (*uart).adaptor != SNDRV_SERIAL_GENERIC)
            || ((*uart).fifo_count == 0 && (inb((*uart).base + UART_MSR as c_ulong) & UART_MSR_CTS) != 0))
    {
        if (inb((*uart).base + UART_LSR as c_ulong) & UART_LSR_THRE) != 0 {
            (*uart).fifo_count = 1;
            outb(midi_byte, (*uart).base + UART_TX as c_ulong);
        } else if (*uart).fifo_count < (*uart).fifo_limit {
            (*uart).fifo_count += 1;
            outb(midi_byte, (*uart).base + UART_TX as c_ulong);
        } else {
            snd_uart16550_write_buffer(uart, midi_byte);
        }
    } else if snd_uart16550_write_buffer(uart, midi_byte) == 0 {
        dev_warn(
            (*(*uart).card).dev,
            b"%s: Buffer overrun on device at 0x%lx\n\0".as_ptr() as *const c_char,
            (*(*uart).rmidi).name.as_ptr(),
            (*uart).base,
        );
        return 0;
    }
    1
}

unsafe extern "C" fn snd_uart16550_output_write(substream: *mut snd_rawmidi_substream) {
    let mut midi_byte: u8 = 0;
    let mut addr_byte: u8;
    let uart = (*(*substream).rmidi).private_data as *mut SndUart16550;
    let mut first: c_char;
    static mut LASTTIME: c_ulong = 0;

    spin_lock_irqsave(&mut (*uart).open_lock);

    if (*uart).irq < 0 {
        snd_uart16550_io_loop(uart);
    }

    if (*uart).adaptor == SNDRV_SERIAL_MS124W_MB {
        loop {
            if (*uart).buff_in_count > TX_BUFF_SIZE as c_int - 2 {
                break;
            }
            if snd_rawmidi_transmit(substream, &mut midi_byte, 1) != 1 {
                break;
            }
            /* SNDRV_SERIAL_MS124W_MB_NOCOMBO would select exactly one of the four ports. */
            /* select any combination of the four ports */
            addr_byte = (((*substream).number << 4) | 0x08) as u8;
            /* ...except none */
            if addr_byte == 0x08 {
                addr_byte = 0xf8;
            }
            snd_uart16550_output_byte(uart, substream, addr_byte);
            snd_uart16550_output_byte(uart, substream, midi_byte);
        }
    } else {
        first = 0;
        while snd_rawmidi_transmit_peek(substream, &mut midi_byte, 1) == 1 {
            if first == 0
                && ((*uart).adaptor == SNDRV_SERIAL_SOUNDCANVAS || (*uart).adaptor == SNDRV_SERIAL_GENERIC)
                && ((*uart).prev_out != (*substream).number
                    || time_after(jiffies, LASTTIME + 3 * HZ))
            {
                if snd_uart16550_buffer_can_write(uart, 3) != 0 {
                    (*uart).prev_out = (*substream).number;
                    snd_uart16550_output_byte(uart, substream, 0xf5);
                    snd_uart16550_output_byte(uart, substream, ((*uart).prev_out + 1) as u8);
                    if midi_byte < 0x80 && (*uart).adaptor == SNDRV_SERIAL_SOUNDCANVAS {
                        snd_uart16550_output_byte(uart, substream, (*uart).prev_status[(*uart).prev_out as usize]);
                    }
                } else if (*uart).drop_on_full == 0 {
                    break;
                }
            }

            if snd_uart16550_output_byte(uart, substream, midi_byte) == 0 && (*uart).drop_on_full == 0 {
                break;
            }

            if midi_byte >= 0x80 && midi_byte < 0xf0 {
                (*uart).prev_status[(*uart).prev_out as usize] = midi_byte;
            }
            first = 1;

            snd_rawmidi_transmit_ack(substream, 1);
        }
        LASTTIME = jiffies;
    }

    spin_unlock_irqrestore(&mut (*uart).open_lock);
}

unsafe extern "C" fn snd_uart16550_output_trigger(substream: *mut snd_rawmidi_substream, up: c_int) {
    let uart = (*(*substream).rmidi).private_data as *mut SndUart16550;

    spin_lock_irqsave(&mut (*uart).open_lock);
    if up != 0 {
        (*uart).filemode |= SERIAL_MODE_OUTPUT_TRIGGERED;
    } else {
        (*uart).filemode &= !SERIAL_MODE_OUTPUT_TRIGGERED;
    }
    spin_unlock_irqrestore(&mut (*uart).open_lock);

    if up != 0 {
        snd_uart16550_output_write(substream);
    }
}

static SND_UART16550_OUTPUT: snd_rawmidi_ops = snd_rawmidi_ops {
    open: Some(snd_uart16550_output_open),
    close: Some(snd_uart16550_output_close),
    trigger: Some(snd_uart16550_output_trigger),
};

static SND_UART16550_INPUT: snd_rawmidi_ops = snd_rawmidi_ops {
    open: Some(snd_uart16550_input_open),
    close: Some(snd_uart16550_input_close),
    trigger: Some(snd_uart16550_input_trigger),
};

unsafe fn snd_uart16550_create(
    card: *mut snd_card,
    iobase: c_ulong,
    irq: c_int,
    speed: c_uint,
    base: c_uint,
    adaptor: c_int,
    droponfull: c_int,
    ruart: *mut *mut SndUart16550,
) -> c_int {
    let uart = devm_kzalloc((*card).dev, size_of::<SndUart16550>(), GFP_KERNEL) as *mut SndUart16550;
    if uart.is_null() {
        return -ENOMEM;
    }
    (*uart).adaptor = adaptor;
    (*uart).card = card;
    spin_lock_init(&mut (*uart).open_lock);
    (*uart).irq = -1;
    (*uart).base = iobase;
    (*uart).drop_on_full = droponfull;

    let mut err = snd_uart16550_detect(uart);
    if err <= 0 {
        dev_err((*card).dev, b"no UART detected at 0x%lx\n\0".as_ptr() as *const c_char, iobase);
        return -ENODEV;
    }

    if irq >= 0 && irq != SNDRV_AUTO_IRQ {
        if devm_request_irq((*card).dev, irq, Some(snd_uart16550_interrupt), 0, b"Serial MIDI\0".as_ptr() as *const c_char, uart as *mut c_void) != 0 {
            dev_warn((*card).dev, b"irq %d busy. Using Polling.\n\0".as_ptr() as *const c_char, irq);
        } else {
            (*uart).irq = irq;
        }
    }
    (*uart).divisor = (base / speed) as u8;
    (*uart).speed = base / (*uart).divisor as c_uint;
    (*uart).speed_base = base;
    (*uart).prev_out = -1;
    (*uart).prev_in = 0;
    (*uart).rstatus = 0;
    ptr::write_bytes((*uart).prev_status.as_mut_ptr(), 0x80, SNDRV_SERIAL_MAX_OUTS);
    timer_setup(&mut (*uart).buffer_timer, Some(snd_uart16550_buffer_timer), 0);
    (*uart).timer_running = 0;

    match (*uart).adaptor {
        SNDRV_SERIAL_MS124W_SA | SNDRV_SERIAL_MS124W_MB => {
            outb(UART_MCR_RTS | (0 & UART_MCR_DTR), (*uart).base + UART_MCR as c_ulong);
        }
        SNDRV_SERIAL_MS124T => {
            outb(UART_MCR_RTS | UART_MCR_DTR, (*uart).base + UART_MCR as c_ulong);
        }
        _ => {}
    }

    if !ruart.is_null() {
        *ruart = uart;
    }

    0
}

unsafe fn snd_uart16550_substreams(stream: *mut snd_rawmidi_str) {
    let mut substream = list_first_entry(&mut (*stream).substreams) as *mut snd_rawmidi_substream;
    while !substream.is_null() {
        sprintf((*substream).name.as_mut_ptr(), b"Serial MIDI %d\0".as_ptr() as *const c_char, (*substream).number + 1);
        substream = list_next_entry(substream);
    }
}

unsafe fn snd_uart16550_rmidi(
    uart: *mut SndUart16550,
    device: c_int,
    outs: c_int,
    ins: c_int,
    rmidi: *mut *mut snd_rawmidi,
) -> c_int {
    let mut rrawmidi: *mut snd_rawmidi = ptr::null_mut();
    let err = snd_rawmidi_new((*uart).card, b"UART Serial MIDI\0".as_ptr() as *const c_char, device, outs, ins, &mut rrawmidi);
    if err < 0 {
        return err;
    }
    snd_rawmidi_set_ops(rrawmidi, SNDRV_RAWMIDI_STREAM_INPUT, &SND_UART16550_INPUT);
    snd_rawmidi_set_ops(rrawmidi, SNDRV_RAWMIDI_STREAM_OUTPUT, &SND_UART16550_OUTPUT);
    strscpy((*rrawmidi).name.as_mut_ptr(), b"Serial MIDI\0".as_ptr() as *const c_char);
    snd_uart16550_substreams(&mut (*rrawmidi).streams[SNDRV_RAWMIDI_STREAM_OUTPUT as usize]);
    snd_uart16550_substreams(&mut (*rrawmidi).streams[SNDRV_RAWMIDI_STREAM_INPUT as usize]);
    (*rrawmidi).info_flags = SNDRV_RAWMIDI_INFO_OUTPUT | SNDRV_RAWMIDI_INFO_INPUT | SNDRV_RAWMIDI_INFO_DUPLEX;
    (*rrawmidi).private_data = uart as *mut c_void;
    if !rmidi.is_null() {
        *rmidi = rrawmidi;
    }
    0
}

unsafe extern "C" fn snd_serial_probe(devptr: *mut platform_device) -> c_int {
    let mut card: *mut snd_card = ptr::null_mut();
    let mut uart: *mut SndUart16550 = ptr::null_mut();
    let mut dev = (*devptr).id;

    if dev < 0 || dev >= SNDRV_CARDS as c_int {
        dev_warn(&mut (*devptr).dev, b"Invalid card index %d, using default 0\n\0".as_ptr() as *const c_char, dev);
        dev = 0;
    }

    match ADAPTOR[dev as usize] {
        SNDRV_SERIAL_SOUNDCANVAS => INS[dev as usize] = 1,
        SNDRV_SERIAL_MS124T | SNDRV_SERIAL_MS124W_SA => {
            OUTS[dev as usize] = 1;
            INS[dev as usize] = 1;
        }
        SNDRV_SERIAL_MS124W_MB => {
            OUTS[dev as usize] = 16;
            INS[dev as usize] = 1;
        }
        SNDRV_SERIAL_GENERIC => {}
        _ => {
            dev_err(&mut (*devptr).dev, b"Adaptor type is out of range 0-%d (%d)\n\0".as_ptr() as *const c_char, SNDRV_SERIAL_MAX_ADAPTOR, ADAPTOR[dev as usize]);
            return -ENODEV;
        }
    }

    if OUTS[dev as usize] < 1 || OUTS[dev as usize] > SNDRV_SERIAL_MAX_OUTS as c_int {
        dev_err(&mut (*devptr).dev, b"Count of outputs is out of range 1-%d (%d)\n\0".as_ptr() as *const c_char, SNDRV_SERIAL_MAX_OUTS as c_int, OUTS[dev as usize]);
        return -ENODEV;
    }

    if INS[dev as usize] < 1 || INS[dev as usize] > SNDRV_SERIAL_MAX_INS as c_int {
        dev_err(&mut (*devptr).dev, b"Count of inputs is out of range 1-%d (%d)\n\0".as_ptr() as *const c_char, SNDRV_SERIAL_MAX_INS as c_int, INS[dev as usize]);
        return -ENODEV;
    }

    let mut err = snd_devm_card_new(&mut (*devptr).dev, INDEX[dev as usize], ID[dev as usize], THIS_MODULE, 0, &mut card);
    if err < 0 {
        return err;
    }

    strscpy((*card).driver.as_mut_ptr(), b"Serial\0".as_ptr() as *const c_char);
    strscpy((*card).shortname.as_mut_ptr(), b"Serial MIDI (UART16550A)\0".as_ptr() as *const c_char);

    err = snd_uart16550_create(
        card,
        PORT[dev as usize] as c_ulong,
        IRQ[dev as usize],
        SPEED[dev as usize] as c_uint,
        BASE[dev as usize] as c_uint,
        ADAPTOR[dev as usize],
        DROPONFULL[dev as usize] as c_int,
        &mut uart,
    );
    if err < 0 {
        return err;
    }

    err = snd_uart16550_rmidi(uart, 0, OUTS[dev as usize], INS[dev as usize], &mut (*uart).rmidi);
    if err < 0 {
        return err;
    }

    sprintf(
        (*card).longname.as_mut_ptr(),
        b"%s [%s] at %#lx, irq %d\0".as_ptr() as *const c_char,
        (*card).shortname.as_ptr(),
        ADAPTOR_NAMES[(*uart).adaptor as usize],
        (*uart).base,
        (*uart).irq,
    );

    err = snd_card_register(card);
    if err < 0 {
        return err;
    }

    platform_set_drvdata(devptr, card as *mut c_void);
    0
}

const SND_SERIAL_DRIVER: *const c_char = b"snd_serial_u16550\0".as_ptr() as *const c_char;

static mut SND_SERIAL_DRIVER_STRUCT: platform_driver = platform_driver {
    probe: Some(snd_serial_probe),
    driver: device_driver {
        name: SND_SERIAL_DRIVER,
    },
};

unsafe fn snd_serial_unregister_all() {
    let mut i: usize = 0;
    while i < DEVICES.len() {
        platform_device_unregister(DEVICES[i]);
        i += 1;
    }
    platform_driver_unregister(&mut SND_SERIAL_DRIVER_STRUCT);
}

unsafe extern "C" fn alsa_card_serial_init() -> c_int {
    let mut err = platform_driver_register(&mut SND_SERIAL_DRIVER_STRUCT);
    if err < 0 {
        return err;
    }

    let mut cards = 0;
    let mut i = 0;
    while i < SNDRV_CARDS {
        if !ENABLE[i] {
            i += 1;
            continue;
        }
        let device = platform_device_register_simple(SND_SERIAL_DRIVER, i as c_int, ptr::null_mut(), 0);
        if IS_ERR(device as *const c_void) {
            i += 1;
            continue;
        }
        if platform_get_drvdata(device).is_null() {
            platform_device_unregister(device);
            i += 1;
            continue;
        }
        DEVICES[i] = device;
        cards += 1;
        i += 1;
    }
    if cards == 0 {
        /* #ifdef MODULE */
        pr_err(b"serial midi soundcard not found or device busy\n\0".as_ptr() as *const c_char);
        snd_serial_unregister_all();
        return -ENODEV;
    }
    0
}

unsafe extern "C" fn alsa_card_serial_exit() {
    snd_serial_unregister_all();
}

/* module_init(alsa_card_serial_init) */
/* module_exit(alsa_card_serial_exit) */

#[repr(C)]
struct snd_card {
    dev: *mut device,
    driver: [c_char; 16],
    shortname: [c_char; 32],
    longname: [c_char; 80],
}
#[repr(C)]
struct snd_rawmidi {
    name: [c_char; 80],
    streams: [snd_rawmidi_str; 2],
    info_flags: c_uint,
    private_data: *mut c_void,
}
#[repr(C)]
struct snd_rawmidi_str {
    substreams: list_head,
}
#[repr(C)]
struct snd_rawmidi_substream {
    rmidi: *mut snd_rawmidi,
    number: c_int,
    name: [c_char; 32],
    list: list_head,
}
#[repr(C)]
struct snd_rawmidi_ops {
    open: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream) -> c_int>,
    close: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream) -> c_int>,
    trigger: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream, c_int)>,
}
#[repr(C)]
struct platform_device {
    dev: device,
    id: c_int,
}
#[repr(C)]
struct platform_driver {
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    driver: device_driver,
}
#[repr(C)]
struct device_driver {
    name: *const c_char,
}
#[repr(C)]
struct device {
    _private: [u8; 0],
}
#[repr(C)]
struct timer_list {
    _private: [u8; 0],
}
#[repr(C)]
struct spinlock_t {
    _private: [u8; 0],
}
#[repr(C)]
struct list_head {
    _private: [u8; 0],
}

type irqreturn_t = c_uint;

extern "C" {
    static mut jiffies: c_ulong;
    static HZ: c_ulong;
    static THIS_MODULE: *mut c_void;

    static SNDRV_CARDS: usize;
    static SNDRV_DEFAULT_IDX: [c_int; SNDRV_CARDS];
    static SNDRV_DEFAULT_STR: [*mut c_char; SNDRV_CARDS];
    static SNDRV_DEFAULT_ENABLE: [bool; SNDRV_CARDS];
    static SNDRV_DEFAULT_PORT: [c_long; SNDRV_CARDS];
    static SNDRV_DEFAULT_IRQ: [c_int; SNDRV_CARDS];

    static UART_TX: c_uint;
    static UART_RX: c_uint;
    static UART_IER: c_uint;
    static UART_IIR: c_uint;
    static UART_FCR: c_uint;
    static UART_LCR: c_uint;
    static UART_MCR: c_uint;
    static UART_LSR: c_uint;
    static UART_MSR: c_uint;
    static UART_SCR: c_uint;
    static UART_DLL: c_uint;
    static UART_DLM: c_uint;
    static UART_LSR_DR: u8;
    static UART_LSR_OE: u8;
    static UART_LSR_THRE: u8;
    static UART_MSR_CTS: u8;
    static UART_LCR_WLEN8: u8;
    static UART_LCR_DLAB: u8;
    static UART_FCR_ENABLE_FIFO: u8;
    static UART_FCR_CLEAR_RCVR: u8;
    static UART_FCR_CLEAR_XMIT: u8;
    static UART_FCR_TRIGGER_4: u8;
    static UART_MCR_RTS: u8;
    static UART_MCR_DTR: u8;
    static UART_MCR_OUT2: u8;
    static UART_IER_RDI: u8;
    static UART_IER_THRI: u8;
    static UART_IER_MSI: u8;

    static SNDRV_AUTO_PORT: c_long;
    static SNDRV_AUTO_IRQ: c_int;
    static SNDRV_RAWMIDI_STREAM_INPUT: c_int;
    static SNDRV_RAWMIDI_STREAM_OUTPUT: c_int;
    static SNDRV_RAWMIDI_INFO_OUTPUT: c_uint;
    static SNDRV_RAWMIDI_INFO_INPUT: c_uint;
    static SNDRV_RAWMIDI_INFO_DUPLEX: c_uint;
    static ENODEV: c_int;
    static EBUSY: c_int;
    static ENOMEM: c_int;
    static GFP_KERNEL: c_uint;
    static IRQ_NONE: irqreturn_t;
    static IRQ_HANDLED: irqreturn_t;

    fn outb(value: u8, port: c_ulong);
    fn inb(port: c_ulong) -> u8;
    fn mod_timer(timer: *mut timer_list, expires: c_ulong);
    fn timer_delete(timer: *mut timer_list);
    fn timer_setup(timer: *mut timer_list, function: Option<unsafe extern "C" fn(*mut timer_list)>, flags: c_uint);
    fn timer_container_of_snd_uart16550(t: *mut timer_list) -> *mut SndUart16550;
    fn time_after(a: c_ulong, b: c_ulong) -> bool;

    fn spin_lock_init(lock: *mut spinlock_t);
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t);

    fn devm_request_region(dev: *mut device, start: c_ulong, n: c_ulong, name: *const c_char) -> *mut c_void;
    fn devm_request_irq(dev: *mut device, irq: c_int, handler: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>, flags: c_ulong, name: *const c_char, dev_id: *mut c_void) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, gfp: c_uint) -> *mut c_void;
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);

    fn snd_rawmidi_receive(substream: *mut snd_rawmidi_substream, buffer: *mut u8, count: c_int) -> c_int;
    fn snd_rawmidi_transmit(substream: *mut snd_rawmidi_substream, buffer: *mut u8, count: c_int) -> c_int;
    fn snd_rawmidi_transmit_peek(substream: *mut snd_rawmidi_substream, buffer: *mut u8, count: c_int) -> c_int;
    fn snd_rawmidi_transmit_ack(substream: *mut snd_rawmidi_substream, count: c_int) -> c_int;
    fn snd_rawmidi_new(card: *mut snd_card, id: *const c_char, device: c_int, output_count: c_int, input_count: c_int, rrawmidi: *mut *mut snd_rawmidi) -> c_int;
    fn snd_rawmidi_set_ops(rmidi: *mut snd_rawmidi, stream: c_int, ops: *const snd_rawmidi_ops);
    fn snd_devm_card_new(dev: *mut device, idx: c_int, xid: *mut c_char, module: *mut c_void, extra_size: c_int, card_ret: *mut *mut snd_card) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;

    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...);
    fn list_first_entry(head: *mut list_head) -> *mut c_void;
    fn list_next_entry(substream: *mut snd_rawmidi_substream) -> *mut snd_rawmidi_substream;

    fn platform_driver_register(driver: *mut platform_driver) -> c_int;
    fn platform_driver_unregister(driver: *mut platform_driver);
    fn platform_device_register_simple(name: *const c_char, id: c_int, res: *mut c_void, num: c_uint) -> *mut platform_device;
    fn platform_device_unregister(pdev: *mut platform_device);
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
    fn IS_ERR(ptr: *const c_void) -> bool;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
