/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_int};

/* C dependency: #include "libslang.h" */

pub const K_DOWN: c_int = SL_KEY_DOWN;
pub const K_END: c_int = SL_KEY_END;
pub const K_ENTER: c_int = b'\r' as c_int;
pub const K_ESC: c_int = 0o33;
pub const K_F1: c_int = SL_KEY_F!(1);
pub const K_HOME: c_int = SL_KEY_HOME;
pub const K_LEFT: c_int = SL_KEY_LEFT;
pub const K_PGDN: c_int = SL_KEY_NPAGE;
pub const K_PGUP: c_int = SL_KEY_PPAGE;
pub const K_RIGHT: c_int = SL_KEY_RIGHT;
pub const K_TAB: c_int = b'\t' as c_int;
pub const K_UNTAB: c_int = SL_KEY_UNTAB;
pub const K_UP: c_int = SL_KEY_UP;
pub const K_BKSPC: c_int = 0x7f;
pub const K_DEL: c_int = SL_KEY_DELETE;

/* Not really keys */
pub const K_TIMER: c_int = -1;
pub const K_ERROR: c_int = -2;
pub const K_RESIZE: c_int = -3;
pub const K_SWITCH_INPUT_DATA: c_int = -4;
pub const K_RELOAD: c_int = -5;

unsafe extern "C" {
    pub fn key_name(key: c_int, bf: *mut c_char, size: usize) -> *const c_char;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
