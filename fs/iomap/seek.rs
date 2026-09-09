// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2017 Red Hat, Inc.
 * Copyright (c) 2018-2021 Christoph Hellwig.
 */

// Linux iomap and pagemap declarations are supplied by the surrounding crate.

type loff_t = i64;

#[repr(C)]
pub struct address_space {
    _private: [u8; 0],
}

#[repr(C)]
pub struct inode {
    pub i_mapping: *mut address_space,
    _private: [u8; 0],
}

#[repr(C)]
pub struct iomap_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct iomap {
    pub type_: i32,
    _private: [u8; 0],
}

#[repr(C)]
pub struct iomap_iter {
    pub inode: *mut inode,
    pub pos: loff_t,
    pub len: loff_t,
    pub flags: u32,
    pub status: i32,
    pub iomap: iomap,
}

const IOMAP_UNWRITTEN: i32 = 1;
const IOMAP_HOLE: i32 = 2;
const IOMAP_REPORT: u32 = 1;
const SEEK_HOLE: i32 = 4;
const SEEK_DATA: i32 = 3;

extern "C" {
    fn iomap_length(iter: *const iomap_iter) -> loff_t;
    fn mapping_seek_hole_data(
        mapping: *mut address_space,
        start: loff_t,
        end: loff_t,
        whence: i32,
    ) -> loff_t;
    fn iomap_iter_advance(iter: *mut iomap_iter, length: loff_t) -> i32;
    fn i_size_read(inode: *const inode) -> loff_t;
    fn iomap_iter(iter: *mut iomap_iter, ops: *const iomap_ops) -> i32;
}

unsafe fn iomap_seek_hole_iter(iter: *mut iomap_iter, hole_pos: *mut loff_t) -> i32 {
    let length = iomap_length(iter);

    match (*iter).iomap.type_ {
        IOMAP_UNWRITTEN => {
            *hole_pos = mapping_seek_hole_data(
                (*(*iter).inode).i_mapping,
                (*iter).pos,
                (*iter).pos.wrapping_add(length),
                SEEK_HOLE,
            );
            if *hole_pos == (*iter).pos.wrapping_add(length) {
                iomap_iter_advance(iter, length)
            } else {
                0
            }
        }
        IOMAP_HOLE => {
            *hole_pos = (*iter).pos;
            0
        }
        _ => iomap_iter_advance(iter, length),
    }
}

#[no_mangle]
pub unsafe extern "C" fn iomap_seek_hole(
    inode: *mut inode,
    pos: loff_t,
    ops: *const iomap_ops,
) -> loff_t {
    let size = i_size_read(inode);
    let mut iter = iomap_iter {
        inode,
        pos,
        len: 0,
        flags: IOMAP_REPORT,
        status: 0,
        iomap: iomap {
            type_: 0,
            _private: [],
        },
    };
    let mut hole_pos = pos;
    let mut ret: i32;

    /* Nothing to be found before or beyond the end of the file. */
    if pos < 0 || pos >= size {
        return -6;
    }

    iter.len = size.wrapping_sub(pos);
    while {
        ret = iomap_iter(&mut iter, ops);
        ret > 0
    } {
        iter.status = iomap_seek_hole_iter(&mut iter, &mut hole_pos);
    }
    if ret < 0 {
        return ret as loff_t;
    }
    if iter.len != 0 {
        return iter.pos;
    }
    size
}

unsafe fn iomap_seek_data_iter(iter: *mut iomap_iter, hole_pos: *mut loff_t) -> i32 {
    let length = iomap_length(iter);

    match (*iter).iomap.type_ {
        IOMAP_HOLE => iomap_iter_advance(iter, length),
        IOMAP_UNWRITTEN => {
            *hole_pos = mapping_seek_hole_data(
                (*(*iter).inode).i_mapping,
                (*iter).pos,
                (*iter).pos.wrapping_add(length),
                SEEK_DATA,
            );
            if *hole_pos < 0 {
                iomap_iter_advance(iter, length)
            } else {
                0
            }
        }
        _ => {
            *hole_pos = (*iter).pos;
            0
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn iomap_seek_data(
    inode: *mut inode,
    pos: loff_t,
    ops: *const iomap_ops,
) -> loff_t {
    let size = i_size_read(inode);
    let mut iter = iomap_iter {
        inode,
        pos,
        len: 0,
        flags: IOMAP_REPORT,
        status: 0,
        iomap: iomap {
            type_: 0,
            _private: [],
        },
    };
    let mut hole_pos = pos;
    let mut ret: i32;

    /* Nothing to be found before or beyond the end of the file. */
    if pos < 0 || pos >= size {
        return -6;
    }

    iter.len = size.wrapping_sub(pos);
    while {
        ret = iomap_iter(&mut iter, ops);
        ret > 0
    } {
        iter.status = iomap_seek_data_iter(&mut iter, &mut hole_pos);
    }
    if ret < 0 {
        return ret as loff_t;
    }
    if iter.len != 0 {
        return iter.pos;
    }
    -6
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
