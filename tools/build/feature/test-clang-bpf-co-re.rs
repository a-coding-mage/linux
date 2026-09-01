// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2020 Facebook

#[repr(C)]
pub struct test {
    pub a: i32,
    pub b: i32,
}
// C source uses: __attribute__((preserve_access_index)).

// C source declares this object volatile.
#[no_mangle]
pub static mut global_value_for_test: test = test { a: 0, b: 0 };

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
