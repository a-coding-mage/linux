// SPDX-License-Identifier: GPL-2.0-only
/*
 * Line 6 Linux USB driver
 *
 * Copyright (C) 2004-2010 Markus Grabner (line6@grabner-graz.at)
 */

// Includes from C source:
// #include <linux/slab.h>  - provides kmalloc, kfree, GFP_KERNEL, ENOMEM
// #include "midibuf.h"     - defines midi_buffer struct and LINE6_MIDIBUF_READ_RX

#[repr(C)]
pub struct midi_buffer {
    pub buf: *mut u8,
    pub size: i32,
    pub pos_read: i32,
    pub pos_write: i32,
    pub full: i32,
    pub split: i32,
    pub command_prev: i32,
}

extern "C" {
    fn kmalloc(size: usize, flags: u32) -> *mut u8;
    fn kfree(ptr: *mut u8);
}

const GFP_KERNEL: u32 = 0x200d0;
const ENOMEM: i32 = -12;
const EINVAL: i32 = -22;
// LINE6_MIDIBUF_READ_RX is defined in midibuf.h; assumed to be available

fn midibuf_message_length(code: u8) -> i32 {
    let message_length: i32;

    if code < 0x80 {
        message_length = -1;
    } else if code < 0xf0 {
        let length: [i32; 7] = [3, 3, 3, 3, 2, 2, 3];
        message_length = length[((code >> 4) - 8) as usize];
    } else {
        let length: [i32; 16] = [-1, 2, 2, 2, -1, -1, 1, 1, 1, -1, 1, 1, 1, -1, 1, 1];
        message_length = length[(code & 0x0f) as usize];
    }

    message_length
}

fn midibuf_is_empty(this: *const midi_buffer) -> i32 {
    unsafe {
        if (*this).pos_read == (*this).pos_write && (*this).full == 0 {
            1
        } else {
            0
        }
    }
}

fn midibuf_is_full(this: *const midi_buffer) -> i32 {
    unsafe {
        if (*this).full != 0 {
            1
        } else {
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn line6_midibuf_reset(this: *mut midi_buffer) {
    unsafe {
        (*this).pos_read = 0;
        (*this).pos_write = 0;
        (*this).full = 0;
        (*this).command_prev = -1;
    }
}

#[no_mangle]
pub extern "C" fn line6_midibuf_init(this: *mut midi_buffer, size: i32, split: i32) -> i32 {
    unsafe {
        (*this).buf = kmalloc(size as usize, GFP_KERNEL);

        if (*this).buf.is_null() {
            return -ENOMEM;
        }

        (*this).size = size;
        (*this).split = split;
        line6_midibuf_reset(this);
        0
    }
}

#[no_mangle]
pub extern "C" fn line6_midibuf_bytes_free(this: *mut midi_buffer) -> i32 {
    unsafe {
        if midibuf_is_full(this as *const _) != 0 {
            0
        } else {
            ((*this).pos_read - (*this).pos_write + (*this).size - 1) % (*this).size + 1
        }
    }
}

#[no_mangle]
pub extern "C" fn line6_midibuf_bytes_used(this: *mut midi_buffer) -> i32 {
    unsafe {
        if midibuf_is_empty(this as *const _) != 0 {
            0
        } else {
            ((*this).pos_write - (*this).pos_read + (*this).size - 1) % (*this).size + 1
        }
    }
}

#[no_mangle]
pub extern "C" fn line6_midibuf_write(
    this: *mut midi_buffer,
    data: *const u8,
    mut length: i32,
) -> i32 {
    unsafe {
        if midibuf_is_full(this as *const _) != 0 || length <= 0 {
            return 0;
        }

        let mut skip_active_sense = 0;

        if *data.add((length - 1) as usize) == 0xfe {
            length -= 1;
            skip_active_sense = 1;
        }

        let bytes_free = line6_midibuf_bytes_free(this);

        let length = if length > bytes_free {
            bytes_free
        } else {
            length
        };

        if length > 0 {
            let length1 = (*this).size - (*this).pos_write;

            if length < length1 {
                std::ptr::copy_nonoverlapping(
                    data,
                    (*this).buf.add((*this).pos_write as usize),
                    length as usize,
                );
                (*this).pos_write += length;
            } else {
                let length2 = length - length1;
                std::ptr::copy_nonoverlapping(
                    data,
                    (*this).buf.add((*this).pos_write as usize),
                    length1 as usize,
                );
                std::ptr::copy_nonoverlapping(
                    data.add(length1 as usize),
                    (*this).buf,
                    length2 as usize,
                );
                (*this).pos_write = length2;
            }

            if (*this).pos_write == (*this).pos_read {
                (*this).full = 1;
            }
        }

        length + skip_active_sense
    }
}

#[no_mangle]
pub extern "C" fn line6_midibuf_read(
    this: *mut midi_buffer,
    data: *mut u8,
    mut length: i32,
    read_type: i32,
) -> i32 {
    unsafe {
        if length < 3 {
            return -EINVAL;
        }

        if midibuf_is_empty(this as *const _) != 0 {
            return 0;
        }

        let bytes_used = line6_midibuf_bytes_used(this);

        if length > bytes_used {
            length = bytes_used;
        }

        let length1 = (*this).size - (*this).pos_read;

        let mut command = *(*this).buf.add((*this).pos_read as usize) as i32;

        if read_type == 2 {
            if command == 0xb2 || command == 0xc2 || command == 0xf2 {
                let fixed = (command & 0xf0) as u8;
                *(*this).buf.add((*this).pos_read as usize) = fixed;
                command = fixed as i32;
            }
        }

        let mut midi_length: i32;
        let mut repeat = 0;

        if (command & 0x80) != 0 {
            midi_length = midibuf_message_length(command as u8);
            (*this).command_prev = command;
        } else if (*this).command_prev > 0 {
            let midi_length_prev = midibuf_message_length((*this).command_prev as u8);

            if midi_length_prev > 1 {
                midi_length = midi_length_prev - 1;
                repeat = 1;
            } else {
                midi_length = -1;
            }
        } else {
            midi_length = -1;
        }

        if midi_length < 0 {
            if length < length1 {
                let mut i = 1;
                while i < length {
                    if (*(*this).buf.add(((*this).pos_read + i) as usize) & 0x80) != 0 {
                        break;
                    }
                    i += 1;
                }
                midi_length = i;
            } else {
                let length2 = length - length1;

                let mut i = 1;
                while i < length1 {
                    if (*(*this).buf.add(((*this).pos_read + i) as usize) & 0x80) != 0 {
                        break;
                    }
                    i += 1;
                }

                if i < length1 {
                    midi_length = i;
                } else {
                    let mut i = 0;
                    while i < length2 {
                        if (*(*this).buf.add(i as usize) & 0x80) != 0 {
                            break;
                        }
                        i += 1;
                    }

                    midi_length = length1 + i;
                }
            }

            if midi_length == length {
                midi_length = -1;
            }
        }

        if midi_length < 0 {
            if (*this).split == 0 {
                return 0;
            }
        } else {
            if length < midi_length {
                return 0;
            }

            length = midi_length;
        }

        if length < length1 {
            std::ptr::copy_nonoverlapping(
                (*this).buf.add((*this).pos_read as usize),
                data.add(repeat as usize),
                length as usize,
            );
            (*this).pos_read += length;
        } else {
            let length2 = length - length1;
            std::ptr::copy_nonoverlapping(
                (*this).buf.add((*this).pos_read as usize),
                data.add(repeat as usize),
                length1 as usize,
            );
            std::ptr::copy_nonoverlapping(
                (*this).buf,
                data.add((repeat + length1) as usize),
                length2 as usize,
            );
            (*this).pos_read = length2;
        }

        if repeat != 0 {
            *data = (*this).command_prev as u8;
        }

        (*this).full = 0;
        length + repeat
    }
}

#[no_mangle]
pub extern "C" fn line6_midibuf_ignore(this: *mut midi_buffer, mut length: i32) -> i32 {
    unsafe {
        let bytes_used = line6_midibuf_bytes_used(this);

        if length > bytes_used {
            length = bytes_used;
        }

        (*this).pos_read = ((*this).pos_read + length) % (*this).size;
        (*this).full = 0;
        length
    }
}

#[no_mangle]
pub extern "C" fn line6_midibuf_destroy(this: *mut midi_buffer) {
    unsafe {
        kfree((*this).buf);
        (*this).buf = core::ptr::null_mut();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
