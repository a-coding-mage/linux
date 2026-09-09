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
 *
 */

// Dependency declarations are supplied by the translated color_table.h.

static mut pq_table: [fixed31_32; MAX_HW_POINTS + 2] = unsafe { core::mem::zeroed() };
static mut de_pq_table: [fixed31_32; MAX_HW_POINTS + 2] = unsafe { core::mem::zeroed() };
static mut pq_initialized: bool = false;
static mut de_pg_initialized: bool = false;

pub unsafe extern "C" fn mod_color_is_table_init(type_: table_type) -> bool {
	let mut ret: bool = false;

	if type_ == type_pq_table {
		ret = pq_initialized;
	}
	if type_ == type_de_pq_table {
		ret = de_pg_initialized;
	}

	ret
}

pub unsafe extern "C" fn mod_color_get_table(type_: table_type) -> *mut fixed31_32 {
	let mut table: *mut fixed31_32 = core::ptr::null_mut();

	if type_ == type_pq_table {
		table = pq_table.as_mut_ptr();
	}
	if type_ == type_de_pq_table {
		table = de_pq_table.as_mut_ptr();
	}

	table
}

pub unsafe extern "C" fn mod_color_set_table_init_state(type_: table_type, state: bool) {
	if type_ == type_pq_table {
		pq_initialized = state;
	}
	if type_ == type_de_pq_table {
		de_pg_initialized = state;
	}
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
