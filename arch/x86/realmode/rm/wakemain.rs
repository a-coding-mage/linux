// SPDX-License-Identifier: GPL-2.0
//
// C dependencies supplied by wakeup.h and boot.h are declared externally.

use core::ffi::{c_char, c_int, c_uint};

#[repr(C)]
pub struct port_io_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct WakeupHeader {
    pub real_magic: u32,
    pub realmode_flags: u32,
    pub video_mode: u16,
}

unsafe extern "C" {
    pub static mut wakeup_header: WakeupHeader;
    pub fn io_delay();
    pub fn outb(value: u16, port: u16);
    pub fn inb(port: u16) -> u8;
    pub fn init_default_io_ops();
    pub fn probe_cards(mode: c_int);
    pub fn set_mode(mode: u16);
}

pub static mut pio_ops: port_io_ops = port_io_ops { _private: [] };

unsafe fn udelay(mut loops: c_int) {
    while loops != 0 {
        loops -= 1;
        io_delay(); // Approximately 1 us
    }
}

unsafe fn beep(hz: c_uint) {
    let enable: u8;

    if hz == 0 {
        enable = 0x00; // Turn off speaker
    } else {
        let div: u16 = (1193181u32 / hz) as u16;

        outb(0xb6, 0x43); // Ctr 2, squarewave, load, binary
        io_delay();
        outb(div, 0x42); // LSB of counter
        io_delay();
        outb(div >> 8, 0x42); // MSB of counter
        io_delay();

        enable = 0x03; // Turn on speaker
    }
    inb(0x61); // Dummy read of System Control Port B
    io_delay();
    outb(enable as u16, 0x61); // Enable timer 2 output to speaker
    io_delay();
}

const DOT_HZ: c_uint = 880;
const DASH_HZ: c_uint = 587;
const US_PER_DOT: c_int = 125000;

// Okay, this is totally silly, but it's kind of fun.
unsafe fn send_morse(mut pattern: *const c_char) {
    loop {
        let s = *pattern as u8 as char;
        pattern = pattern.add(1);
        if s == '\0' {
            break;
        }
        match s {
            '.' => {
                beep(DOT_HZ);
                udelay(US_PER_DOT);
                beep(0);
                udelay(US_PER_DOT);
            }
            '-' => {
                beep(DASH_HZ);
                udelay(US_PER_DOT * 3);
                beep(0);
                udelay(US_PER_DOT);
            }
            _ => {
                // Assume it's a space
                udelay(US_PER_DOT * 3);
            }
        }
    }
}

pub unsafe fn main() {
    init_default_io_ops();

    // Kill machine if structures are wrong
    if wakeup_header.real_magic != 0x12345678 {
        loop {}
    }

    if wakeup_header.realmode_flags & 4 != 0 {
        send_morse(b"...-\0".as_ptr() as *const c_char);
    }

    if wakeup_header.realmode_flags & 1 != 0 {
        // The original C performs a far call to BIOS here.
        core::arch::asm!("lcallw $0xc000,$3");
    }

    if wakeup_header.realmode_flags & 2 != 0 {
        // Need to call BIOS
        probe_cards(0);
        set_mode(wakeup_header.video_mode);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
