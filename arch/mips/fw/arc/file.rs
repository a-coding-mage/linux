/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * ARC firmware interface.
 *
 * Copyright (C) 1994, 1995, 1996, 1999 Ralf Baechle
 * Copyright (C) 1999 Silicon Graphics, Inc.
 */

// Dependencies supplied by the surrounding ARC firmware environment:
// asm/fw/arc/types.h and asm/sgialib.h

pub unsafe fn ArcRead(
    FileID: ULONG,
    Buffer: *mut VOID,
    N: ULONG,
    Count: *mut ULONG,
) -> LONG {
    ARC_CALL4!(read, FileID, Buffer, N, Count)
}

pub unsafe fn ArcWrite(
    FileID: ULONG,
    Buffer: PVOID,
    N: ULONG,
    Count: PULONG,
) -> LONG {
    ARC_CALL4!(write, FileID, Buffer, N, Count)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
