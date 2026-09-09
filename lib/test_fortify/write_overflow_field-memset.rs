// SPDX-License-Identifier: GPL-2.0-only

// Dependency intent: declarations and types supplied by "test_fortify.h".

macro_rules! TEST {
    ($instance:expr) => {{
        unsafe {
            core::ptr::write_bytes(
                (&mut ($instance).buf) as *mut _ as *mut u8,
                0x42,
                core::mem::size_of_val(&($instance).buf) + 1,
            );
        }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
