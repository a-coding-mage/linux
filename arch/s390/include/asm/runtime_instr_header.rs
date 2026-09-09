/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: declarations from <uapi/asm/runtime_instr.h> are supplied
// by the surrounding translation unit.

extern "C" {
    pub static mut runtime_instr_empty_cb: runtime_instr_cb;

    fn store_runtime_instr_cb(cb: *mut runtime_instr_cb);
    fn load_runtime_instr_cb(cb: *mut runtime_instr_cb);
}

pub unsafe fn save_ri_cb(cb_prev: *mut runtime_instr_cb) {
    if !cb_prev.is_null() {
        store_runtime_instr_cb(cb_prev);
    }
}

pub unsafe fn restore_ri_cb(
    cb_next: *mut runtime_instr_cb,
    cb_prev: *mut runtime_instr_cb,
) {
    if !cb_next.is_null() {
        load_runtime_instr_cb(cb_next);
    } else if !cb_prev.is_null() {
        load_runtime_instr_cb(&raw mut runtime_instr_empty_cb);
    }
}

// Forward declaration of struct task_struct from the source header.
pub enum task_struct {}

extern "C" {
    pub fn runtime_instr_release(tsk: *mut task_struct);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
