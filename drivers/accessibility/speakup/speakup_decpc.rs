// SPDX-License-Identifier: GPL-2.0+
/* Rust translation of the DECtalk PC speakup driver. */

use core::ffi::{c_char, c_int, c_uint, c_ulong};

const MODULE_INIT: u16 = 0x0dec;
const MODULE_SELF_TEST: u16 = 0x8800;
const MODULE_RESET: u16 = 0xffff;
const MODE_MASK: u16 = 0xf000;
const MODE_NULL: u16 = 0x0000;
const MODE_TEST: u16 = 0x2000;
const MODE_STATUS: u16 = 0x8000;
const STAT_INT: u16 = 0x0001;
const STAT_TR_CHAR: u16 = 0x0002;
const STAT_RR_CHAR: u16 = 0x0004;
const STAT_CMD_READY: u16 = 0x0008;
const STAT_DMA_READY: u16 = 0x0010;
const STAT_DIGITIZED: u16 = 0x0020;
const STAT_NEW_INDEX: u16 = 0x0040;
const STAT_NEW_STATUS: u16 = 0x0080;
const STAT_DMA_STATE: u16 = 0x0100;
const STAT_INDEX_VALID: u16 = 0x0200;
const STAT_FLUSHING: u16 = 0x0400;
const STAT_SELF_TEST: u16 = 0x0800;
const MODE_READY: u16 = 0xc000;
const READY_BOOT: u16 = 0x0000;
const READY_KERNEL: u16 = 0x0001;
const MODE_ERROR: u16 = 0xf000;
const CMD_MASK: u16 = 0xf000;
const CMD_NULL: u16 = 0x0000;
const CMD_CONTROL: u16 = 0x1000;
const CTRL_MASK: u16 = 0x0f00;
const CTRL_DATA: u16 = 0x00ff;
const CTRL_FLUSH: u16 = 0x0600;
const CMD_TEST: u16 = 0x2000;
const CMD_ID: u16 = 0x3000;
const CMD_DMA: u16 = 0x4000;
const CMD_RESET: u16 = 0x5000;
const CMD_SYNC: u16 = 0x6000;
const CMD_CHAR_IN: u16 = 0x7000;
const CMD_CHAR_OUT: u16 = 0x8000;
const CMD_SPC_MODE: u16 = 0x9000;
const CMD_ERROR: u16 = 0xf000;
const DMA_SINGLE_IN: u8 = 0x01;
const DMA_SYNC: u8 = 0x06;
const DRV_VERSION: &[u8] = b"2.12\0";
const PROCSPEECH: u8 = 0x0b;
const SYNTH_IO_EXTENT: c_int = 8;
const SPACE: u8 = b' ';

// External kernel/Speakup declarations supplied by the surrounding translation.
#[repr(C)] pub struct spk_synth { pub long_name: *const c_char, pub version: *const c_char, pub alive: c_int, pub flush: Option<unsafe extern "C" fn(*mut spk_synth)> }
#[repr(C)] pub struct var_t { pub value: c_int }
extern "C" {
    static mut speakup_info: SpeakupInfo;
    fn inb_p(port: u16) -> u8; fn outb_p(value: u8, port: u16); fn udelay(usecs: c_uint);
    fn synth_release_region(port: c_int, extent: c_int); fn synth_request_region(port: c_int, extent: c_int) -> c_int;
    fn spk_get_var(id: c_int) -> *mut var_t; fn kthread_should_stop() -> c_int; fn schedule_timeout(t: c_ulong);
    fn synth_buffer_skip_nonlatin1(); fn synth_buffer_empty() -> c_int; fn synth_buffer_peek() -> u8; fn synth_buffer_getc();
    fn spk_stop_serial_interrupt(); fn spk_synth_is_alive_nop(s: *mut spk_synth) -> c_int;
}
#[repr(C)] struct SpeakupInfo { port_tts: u16, flushing: c_int, spinlock: usize }

static mut synth_portlist: [c_int; 5] = [0x340, 0x350, 0x240, 0x250, 0];
static mut in_escape: c_int = 0;
static mut is_flushing: c_int = 0;
static mut dt_stat: c_int = 0;
static mut dma_state: c_int = 0;

unsafe fn dt_getstatus() -> c_int { dt_stat = inb_p(speakup_info.port_tts) as c_int | ((inb_p(speakup_info.port_tts + 1) as c_int) << 8); dt_stat }
unsafe fn dt_sendcmd(cmd: c_uint) { outb_p((cmd & 0xff) as u8, speakup_info.port_tts); outb_p(((cmd >> 8) & 0xff) as u8, speakup_info.port_tts + 1); }
unsafe fn dt_waitbit(bit: c_int) -> c_int { let mut timeout = 100; while { timeout -= 1; timeout > 0 } { if (dt_getstatus() & bit) == bit { return 1; } udelay(50); } 0 }
unsafe fn dt_wait_dma() -> c_int { let mut timeout = 100; let state = dma_state; if dt_waitbit(STAT_DMA_READY as c_int) == 0 { return 0; } while { timeout -= 1; timeout > 0 } { if (dt_getstatus() & STAT_DMA_STATE as c_int) == state { return 1; } udelay(50); } dma_state = dt_getstatus() & STAT_DMA_STATE as c_int; 1 }
unsafe fn dt_ctrl(cmd: c_uint) -> c_int { let mut timeout = 10; if dt_waitbit(STAT_CMD_READY as c_int) == 0 { return -1; } outb_p(0, speakup_info.port_tts + 2); outb_p(0, speakup_info.port_tts + 3); dt_getstatus(); dt_sendcmd(CMD_CONTROL as c_uint | cmd); outb_p(0, speakup_info.port_tts + 6); while dt_getstatus() & STAT_CMD_READY as c_int != 0 { udelay(20); timeout -= 1; if timeout == 0 { break; } } dt_sendcmd(CMD_NULL as c_uint); 0 }
unsafe fn synth_flush(_synth: *mut spk_synth) { if is_flushing != 0 { return; } is_flushing = 4; in_escape = 0; let mut timeout = 10; while dt_ctrl(CTRL_FLUSH as c_uint) != 0 { timeout -= 1; if timeout == 0 { break; } udelay(50); } for timeout in 0..10 { if dt_waitbit(STAT_DMA_READY as c_int) != 0 { break; } if timeout == 9 { } udelay(50); } outb_p(DMA_SYNC, speakup_info.port_tts + 4); outb_p(0, speakup_info.port_tts + 4); udelay(100); for _ in 0..10 { if dt_getstatus() & STAT_FLUSHING as c_int == 0 { break; } udelay(50); } dma_state = dt_getstatus() & STAT_DMA_STATE as c_int; dma_state ^= STAT_DMA_STATE as c_int; is_flushing = 0; }
unsafe fn dt_sendchar(ch: u8) -> c_int { if dt_wait_dma() == 0 { return -1; } if dt_stat & STAT_RR_CHAR as c_int == 0 { return -2; } outb_p(DMA_SINGLE_IN, speakup_info.port_tts + 4); outb_p(ch, speakup_info.port_tts + 4); dma_state ^= STAT_DMA_STATE as c_int; 0 }

unsafe fn testkernel() -> c_int { let mut status = 0; if dt_getstatus() == 0xffff { status = -1; } else { dt_sendcmd(CMD_SYNC as c_uint); if dt_waitbit(STAT_CMD_READY as c_int) == 0 { status = -2; } else if dt_stat & 0x8000 != 0 { return 0; } else if dt_stat == 0x0dec { /* pr_warn */ } status = -3; } synth_release_region(speakup_info.port_tts as c_int, SYNTH_IO_EXTENT); speakup_info.port_tts = 0; status }

// The remaining driver registration and Speakup variable metadata are external-kernel declarations.
unsafe fn synth_immediate(_synth: *mut spk_synth, mut buf: *const c_char) -> *const c_char {
    while *buf != 0 { let mut ch = *buf as u8; if ch == b'\n' { ch = PROCSPEECH; } if dt_sendchar(ch) != 0 { return buf; } buf = buf.add(1); } core::ptr::null()
}
unsafe fn synth_probe(synth: *mut spk_synth) -> c_int {
    let mut i = 0; let mut failed = 0;
    while synth_portlist[i] != 0 { let port = synth_portlist[i]; if synth_request_region(port, SYNTH_IO_EXTENT) != 0 { i += 1; continue; } speakup_info.port_tts = port as u16; failed = testkernel(); if failed == 0 { break; } i += 1; }
    if failed != 0 { return -19; } (*synth).alive = 1; 0
}
unsafe fn dtpc_release(_synth: *mut spk_synth) { spk_stop_serial_interrupt(); if speakup_info.port_tts != 0 { synth_release_region(speakup_info.port_tts as c_int, SYNTH_IO_EXTENT); } speakup_info.port_tts = 0; }
unsafe fn do_catch_up(synth: *mut spk_synth) {
    let mut last = 0u8; while kthread_should_stop() == 0 { if speakup_info.flushing != 0 { speakup_info.flushing = 0; if let Some(flush) = (*synth).flush { flush(synth); } continue; } synth_buffer_skip_nonlatin1(); if synth_buffer_empty() != 0 { break; } let mut ch = synth_buffer_peek(); if ch == b'\n' { ch = 0x0d; } if dt_sendchar(ch) != 0 { schedule_timeout(1); continue; } synth_buffer_getc(); if ch == b'[' { in_escape = 1; } else if ch == b']' { in_escape = 0; } else if ch <= SPACE && !((in_escape != 0) && (last == b',' || last == b'.' || last == b'!' || last == b'?' || last == b';' || last == b':')) { dt_sendchar(PROCSPEECH); } last = ch; } if in_escape == 0 { dt_sendchar(PROCSPEECH); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
