/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Miscellaneous ARCS PROM routines.
 *
 * Copyright (C) 1996 David S. Miller (davem@davemloft.net)
 * Copyright (C) 1999 Ralf Baechle (ralf@gnu.org)
 * Copyright (C) 1999 Silicon Graphics, Inc.
 */

// The declarations below are supplied by the corresponding kernel and ARCS
// interfaces.  The C includes and ARC_CALL macros are intentionally omitted.

extern "C" {
    fn bc_disable();
    fn local_irq_disable();
    fn imode();
    fn GetDisplayStatus(file_id: ULONG) -> *mut DISPLAY_STATUS;
}

extern "C" {
    fn unreachable() -> !;
}

/// Enter ARCS interactive mode.
#[no_mangle]
pub unsafe extern "C" fn ArcEnterInteractiveMode() -> ! {
    bc_disable();
    local_irq_disable();
    imode();

    unreachable();
}

#[no_mangle]
pub unsafe extern "C" fn ArcGetDisplayStatus(file_id: ULONG) -> *mut DISPLAY_STATUS {
    GetDisplayStatus(file_id)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
