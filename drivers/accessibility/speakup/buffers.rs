// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the surrounding kernel/Speakup translation.
use core::ffi::c_int;

const SYNTH_BUF_SIZE: usize = 8192; // currently 8K bytes

static mut SYNTH_BUFFER: [u16; SYNTH_BUF_SIZE] = [0; SYNTH_BUF_SIZE]; // guess what this is for!
static mut BUFF_IN: *mut u16 = core::ptr::addr_of_mut!(SYNTH_BUFFER) as *mut u16;
static mut BUFF_OUT: *mut u16 = core::ptr::addr_of_mut!(SYNTH_BUFFER) as *mut u16;
static mut BUFFER_END: *mut u16 =
    (core::ptr::addr_of_mut!(SYNTH_BUFFER) as *mut u16).wrapping_add(SYNTH_BUF_SIZE - 1);

// These try to throttle applications by stopping the TTYs
// Note: we need to make sure that we will restart them eventually, which is
// usually not possible to do from the notifiers. TODO: it should be possible
// starting from linux 2.6.26.
//
// So we only stop when we know alive == 1 (else we discard the data anyway),
// and the alive synth will eventually call start_ttys from the thread context.

extern "C" {
    static mut speakup_console: [*mut SpeakupConsole; MAX_NR_CONSOLES];
    static mut vc_cons: [VcCons; MAX_NR_CONSOLES];
    static mut synth: *mut Synth;
    static mut spk_paused: bool;

    fn start_tty(tty: *mut Tty);
    fn stop_tty(tty: *mut Tty);
    fn synth_start();
}

extern "C" {
    static MAX_NR_CONSOLES: usize;
}

#[repr(C)]
struct SpeakupConsole {
    tty_stopped: bool,
}

#[repr(C)]
struct VcCons {
    d: *mut VcData,
}

#[repr(C)]
struct VcData {
    port: VcPort,
}

#[repr(C)]
struct VcPort {
    tty: *mut Tty,
}

#[repr(C)]
struct Tty {
    _private: [u8; 0],
}

#[repr(C)]
struct Synth {
    alive: bool,
}

pub unsafe extern "C" fn speakup_start_ttys() {
    let mut i: c_int = 0;

    while (i as usize) < MAX_NR_CONSOLES {
        if !speakup_console[i as usize].is_null()
            && (*speakup_console[i as usize]).tty_stopped
        {
            i += 1;
            continue;
        }
        if !vc_cons[i as usize].d.is_null()
            && !(*vc_cons[i as usize]).port.tty.is_null()
        {
            start_tty((*vc_cons[i as usize]).port.tty);
        }
        i += 1;
    }
}

unsafe fn speakup_stop_ttys() {
    let mut i: c_int = 0;

    while (i as usize) < MAX_NR_CONSOLES {
        if !vc_cons[i as usize].d.is_null()
            && !(*vc_cons[i as usize]).port.tty.is_null()
        {
            stop_tty((*vc_cons[i as usize]).port.tty);
        }
        i += 1;
    }
}

unsafe fn synth_buffer_free() -> c_int {
    let chars_free: c_int;

    if BUFF_IN >= BUFF_OUT {
        chars_free = SYNTH_BUF_SIZE as c_int - BUFF_IN.offset_from(BUFF_OUT) as c_int;
    } else {
        chars_free = BUFF_OUT.offset_from(BUFF_IN) as c_int;
    }
    chars_free
}

pub unsafe extern "C" fn synth_buffer_empty() -> bool {
    BUFF_IN == BUFF_OUT
}

pub unsafe extern "C" fn synth_buffer_add(ch: u16) {
    if !(*synth).alive {
        // This makes sure that we won't stop TTYs if there is no synth
        // to restart them
        return;
    }
    if synth_buffer_free() <= 100 {
        synth_start();
        speakup_stop_ttys();
    }
    if synth_buffer_free() <= 1 {
        return;
    }
    *BUFF_IN = ch;
    BUFF_IN = BUFF_IN.add(1);
    if BUFF_IN > BUFFER_END {
        BUFF_IN = core::ptr::addr_of_mut!(SYNTH_BUFFER) as *mut u16;
    }
    // We have written something to the speech synthesis, so we are not
    // paused any more.
    spk_paused = false;
}

pub unsafe extern "C" fn synth_buffer_getc() -> u16 {
    let ch: u16;

    if BUFF_OUT == BUFF_IN {
        return 0;
    }
    ch = *BUFF_OUT;
    BUFF_OUT = BUFF_OUT.add(1);
    if BUFF_OUT > BUFFER_END {
        BUFF_OUT = core::ptr::addr_of_mut!(SYNTH_BUFFER) as *mut u16;
    }
    ch
}

pub unsafe extern "C" fn synth_buffer_peek() -> u16 {
    if BUFF_OUT == BUFF_IN {
        return 0;
    }
    *BUFF_OUT
}

pub unsafe extern "C" fn synth_buffer_skip_nonlatin1() {
    while BUFF_OUT != BUFF_IN {
        if *BUFF_OUT < 0x100 {
            return;
        }
        BUFF_OUT = BUFF_OUT.add(1);
        if BUFF_OUT > BUFFER_END {
            BUFF_OUT = core::ptr::addr_of_mut!(SYNTH_BUFFER) as *mut u16;
        }
    }
}

pub unsafe extern "C" fn synth_buffer_clear() {
    BUFF_IN = core::ptr::addr_of_mut!(SYNTH_BUFFER) as *mut u16;
    BUFF_OUT = core::ptr::addr_of_mut!(SYNTH_BUFFER) as *mut u16;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
