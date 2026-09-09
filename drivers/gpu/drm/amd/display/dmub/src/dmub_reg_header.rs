/*
 * Copyright 2019 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// Dependency supplied by ../inc/dmub_cmd.h.

pub enum dmub_srv {}

/* Register offset and field lookup. */

macro_rules! BASE { ($seg:ident) => { BASE_INNER!($seg) }; }
macro_rules! REG_OFFSET { ($reg_name:ident) => { BASE!(mm$reg_name_BASE_IDX) + mm$reg_name }; }
macro_rules! FD_SHIFT { ($reg_name:ident, $field:ident) => { $reg_name##__$field##__SHIFT }; }
macro_rules! FD_MASK { ($reg_name:ident, $field:ident) => { $reg_name##__$field##_MASK }; }
macro_rules! REG { ($reg:ident) => { REGS.offset.$reg }; }
macro_rules! FD { ($reg_field:ident) => { (REGS.shift.$reg_field, REGS.mask.$reg_field) }; }
macro_rules! FN { ($reg_name:ident, $field:ident) => { FD!($reg_name##__$field) }; }

/* Register reads and writes. */

macro_rules! REG_READ { ($reg:ident) => { (CTX.funcs.reg_read)(CTX.user_ctx, REG!($reg)) }; }
macro_rules! REG_WRITE { ($reg:ident, $val:expr) => { (CTX.funcs.reg_write)(CTX.user_ctx, REG!($reg), $val) }; }

/* Register field setting. */

macro_rules! REG_SET_N {
    ($reg_name:ident, $n:expr, $initial_val:expr $(, $arg:expr)*) => {
        dmub_reg_set(CTX, REG!($reg_name), $initial_val, $n $(, $arg)*)
    };
}
macro_rules! REG_SET {
    ($reg_name:ident, $initial_val:expr, $field:ident, $val:expr) => {
        REG_SET_N!($reg_name, 1, $initial_val, FN!($reg_name, $field), $val)
    };
}
macro_rules! REG_SET_2 {
    ($reg:ident, $init_value:expr, $f1:ident, $v1:expr, $f2:ident, $v2:expr) => {
        REG_SET_N!($reg, 2, $init_value, FN!($reg, $f1), $v1, FN!($reg, $f2), $v2)
    };
}
macro_rules! REG_SET_3 {
    ($reg:ident, $init_value:expr, $f1:ident, $v1:expr, $f2:ident, $v2:expr, $f3:ident, $v3:expr) => {
        REG_SET_N!($reg, 3, $init_value, FN!($reg, $f1), $v1, FN!($reg, $f2), $v2, FN!($reg, $f3), $v3)
    };
}
macro_rules! REG_SET_4 {
    ($reg:ident, $init_value:expr, $f1:ident, $v1:expr, $f2:ident, $v2:expr, $f3:ident, $v3:expr, $f4:ident, $v4:expr) => {
        REG_SET_N!($reg, 4, $init_value, FN!($reg, $f1), $v1, FN!($reg, $f2), $v2, FN!($reg, $f3), $v3, FN!($reg, $f4), $v4)
    };
}

/* Register field updating. */

macro_rules! REG_UPDATE_N {
    ($reg_name:ident, $n:expr $(, $arg:expr)*) => {
        dmub_reg_update(CTX, REG!($reg_name), $n $(, $arg)*)
    };
}
macro_rules! REG_UPDATE { ($reg_name:ident, $field:ident, $val:expr) => { REG_UPDATE_N!($reg_name, 1, FN!($reg_name, $field), $val) }; }
macro_rules! REG_UPDATE_2 { ($reg:ident, $f1:ident, $v1:expr, $f2:ident, $v2:expr) => { REG_UPDATE_N!($reg, 2, FN!($reg, $f1), $v1, FN!($reg, $f2), $v2) }; }
macro_rules! REG_UPDATE_3 { ($reg:ident, $f1:ident, $v1:expr, $f2:ident, $v2:expr, $f3:ident, $v3:expr) => { REG_UPDATE_N!($reg, 3, FN!($reg, $f1), $v1, FN!($reg, $f2), $v2, FN!($reg, $f3), $v3) }; }
macro_rules! REG_UPDATE_4 { ($reg:ident, $f1:ident, $v1:expr, $f2:ident, $v2:expr, $f3:ident, $v3:expr, $f4:ident, $v4:expr) => { REG_UPDATE_N!($reg, 4, FN!($reg, $f1), $v1, FN!($reg, $f2), $v2, FN!($reg, $f3), $v3, FN!($reg, $f4), $v4) }; }

/* Register field getting. */
macro_rules! REG_GET { ($reg_name:ident, $field:ident, $val:expr) => { dmub_reg_get(CTX, REG!($reg_name), FN!($reg_name, $field), $val) }; }

extern "C" {
    pub fn dmub_reg_set(srv: *mut dmub_srv, addr: u32, reg_val: u32, n: i32,
                         shift1: u8, mask1: u32, field_value1: u32, ...);
    pub fn dmub_reg_update(srv: *mut dmub_srv, addr: u32, n: i32,
                           shift1: u8, mask1: u32, field_value1: u32, ...);
    pub fn dmub_reg_get(srv: *mut dmub_srv, addr: u32, shift: u8,
                        mask: u32, field_value: *mut u32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
