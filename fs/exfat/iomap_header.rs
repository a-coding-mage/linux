/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (c) 2026 Namjae Jeon <linkinjeon@kernel.org>
 */

// C header guard: _LINUX_EXFAT_IOMAP_H

// Types supplied by the surrounding kernel/exFAT translation unit:
// struct iomap_dio_ops, struct iomap_ops, struct iomap_writeback_ops,
// struct iomap_read_ops, struct swap_info_struct, struct file, and sector_t.

unsafe extern "C" {
    pub static exfat_write_dio_ops: iomap_dio_ops;
    pub static exfat_iomap_ops: iomap_ops;
    pub static exfat_write_iomap_ops: iomap_ops;
    pub static exfat_writeback_ops: iomap_writeback_ops;
    pub static exfat_iomap_bio_read_ops: iomap_read_ops;

    pub fn exfat_iomap_swap_activate(
        sis: *mut swap_info_struct,
        file: *mut file,
        span: *mut sector_t,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
