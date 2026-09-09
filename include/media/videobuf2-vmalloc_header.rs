/*
 * videobuf2-vmalloc.h - vmalloc memory allocator for videobuf2
 *
 * Copyright (C) 2010 Samsung Electronics
 *
 * Author: Pawel Osciak <pawel@osciak.com>
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation.
 */

// Original include: <media/videobuf2-v4l2.h>
// The C header guard _MEDIA_VIDEOBUF2_VMALLOC_H is omitted from Rust syntax.

extern "C" {
    pub static vb2_vmalloc_memops: vb2_mem_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
