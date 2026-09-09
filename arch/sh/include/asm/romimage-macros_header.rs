/* SPDX-License-Identifier: GPL-2.0 */

/* The LIST command is used to include comments in the script. */
#[macro_export]
macro_rules! LIST {
    ($comment:expr) => {{
        let _ = &$comment;
    }};
}

/* The ED command is used to write a 32-bit word. */
#[macro_export]
macro_rules! ED {
    ($addr:expr, $data:expr) => {{
        unsafe {
            core::ptr::write_volatile($addr as *mut u32, $data as u32);
        }
    }};
}

/* The EW command is used to write a 16-bit word. */
#[macro_export]
macro_rules! EW {
    ($addr:expr, $data:expr) => {{
        unsafe {
            core::ptr::write_volatile($addr as *mut u16, $data as u16);
        }
    }};
}

/* The EB command is used to write an 8-bit word. */
#[macro_export]
macro_rules! EB {
    ($addr:expr, $data:expr) => {{
        unsafe {
            core::ptr::write_volatile($addr as *mut u8, $data as u8);
        }
    }};
}

/* The WAIT command is used to delay the execution. */
#[macro_export]
macro_rules! WAIT {
    ($time:expr) => {{
        let mut r3 = ($time as u32).wrapping_mul(100);
        while r3 != 0 {
            core::hint::spin_loop();
            r3 = r3.wrapping_sub(1);
        }
    }};
}

/* The DD command is used to read a 32-bit word. */
#[macro_export]
macro_rules! DD {
    ($addr:expr, $addr2:expr, $nr:expr) => {{
        let _ = &$addr2;
        let _ = &$nr;
        unsafe { core::ptr::read_volatile($addr as *const u32) }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
