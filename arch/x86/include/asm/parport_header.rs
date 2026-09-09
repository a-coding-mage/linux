/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: _ASM_X86_PARPORT_H

// Declaration of the file-local function supplied by the surrounding
// translation unit.
unsafe extern "C" {
    fn parport_pc_find_isa_ports(autoirq: i32, autodma: i32) -> i32;
}

unsafe fn parport_pc_find_nonpci_ports(autoirq: i32, autodma: i32) -> i32 {
    unsafe { parport_pc_find_isa_ports(autoirq, autodma) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
