/* SPDX-License-Identifier: GPL-2.0 */

// C conditional: #ifdef DEFINE_DWARF_REGSTR_TABLE
// This is included in perf/util/dwarf-regs.c

pub static arm_regstr_tbl: [*const ::std::os::raw::c_char; 16] = [
    b"%r0\0".as_ptr() as *const ::std::os::raw::c_char,
    b"%r1\0".as_ptr() as *const ::std::os::raw::c_char,
    b"%r2\0".as_ptr() as *const ::std::os::raw::c_char,
    b"%r3\0".as_ptr() as *const ::std::os::raw::c_char,
    b"%r4\0".as_ptr() as *const ::std::os::raw::c_char,
    b"%r5\0".as_ptr() as *const ::std::os::raw::c_char,
    b"%r6\0".as_ptr() as *const ::std::os::raw::c_char,
    b"%r7\0".as_ptr() as *const ::std::os::raw::c_char,
    b"%r8\0".as_ptr() as *const ::std::os::raw::c_char,
    b"%r9\0".as_ptr() as *const ::std::os::raw::c_char,
    b"%r10\0".as_ptr() as *const ::std::os::raw::c_char,
    b"%fp\0".as_ptr() as *const ::std::os::raw::c_char,
    b"%ip\0".as_ptr() as *const ::std::os::raw::c_char,
    b"%sp\0".as_ptr() as *const ::std::os::raw::c_char,
    b"%lr\0".as_ptr() as *const ::std::os::raw::c_char,
    b"%pc\0".as_ptr() as *const ::std::os::raw::c_char,
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
