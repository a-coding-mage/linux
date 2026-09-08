// SPDX-License-Identifier: GPL-2.0-only
//
// Dependencies supplied by list.h, expr.h, and mnconf-common.h are referenced
// here but are intentionally not redefined in this translation unit.

pub static mut jump_key_char: ::core::ffi::c_int = 0;

pub fn next_jump_key(mut key: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if key < '1' as ::core::ffi::c_int || key > '9' as ::core::ffi::c_int {
        return '1' as ::core::ffi::c_int;
    }

    key += 1;

    if key > '9' as ::core::ffi::c_int {
        key = '1' as ::core::ffi::c_int;
    }

    key
}

pub unsafe fn handle_search_keys(
    key: ::core::ffi::c_int,
    start: usize,
    end: usize,
    _data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    // `search_data`, `jump_key`, and the list traversal operation are supplied
    // by the translated declarations from the included headers.
    let data = _data as *mut search_data;
    let mut pos: *mut jump_key;
    let mut index: ::core::ffi::c_int = 0;

    if key < '1' as ::core::ffi::c_int || key > '9' as ::core::ffi::c_int {
        return 0;
    }

    // Direct translation of list_for_each_entry(pos, data->head, entries).
    // The list implementation and its container-of operation are external
    // dependencies of this source file.
    list_for_each_entry!(pos, (*data).head, entries, {
        index = next_jump_key(index);

        if (*pos).offset < start {
            continue;
        }

        if (*pos).offset >= end {
            break;
        }

        if key == index {
            (*data).target = (*pos).target;
            return 1;
        }
    });

    0
}

pub unsafe fn get_jump_key_char() -> ::core::ffi::c_int {
    jump_key_char = next_jump_key(jump_key_char);

    jump_key_char
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
