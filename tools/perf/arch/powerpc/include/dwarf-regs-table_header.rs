/* SPDX-License-Identifier: GPL-2.0 */
/* Original C conditional: #ifdef DEFINE_DWARF_REGSTR_TABLE */
/* This is included in perf/util/dwarf-regs.c */

/*
 * Reference:
 * http://refspecs.linuxfoundation.org/ELF/ppc64/PPC-elf64abi-1.9.html
 * http://refspecs.linux-foundation.org/elf/elfspec_ppc.pdf
 */

pub const powerpc_regstr_tbl: [*const ::core::ffi::c_char; 120] = {
    let mut tbl = [::core::ptr::null(); 120];

    tbl[0] = b"%gpr0\0".as_ptr() as *const ::core::ffi::c_char;
    tbl[1] = b"%gpr1\0".as_ptr() as *const ::core::ffi::c_char;
    tbl[2] = b"%gpr2\0".as_ptr() as *const ::core::ffi::c_char;
    tbl[3] = b"%gpr3\0".as_ptr() as *const ::core::ffi::c_char;
    tbl[4] = b"%gpr4\0".as_ptr() as *const ::core::ffi::c_char;
    tbl[5] = b"%gpr5\0".as_ptr() as *const ::core::ffi::c_char;
    tbl[6] = b"%gpr6\0".as_ptr() as *const ::core::ffi::c_char;
    tbl[7] = b"%gpr7\0".as_ptr() as *const ::core::ffi::c_char;
    tbl[8] = b"%gpr8\0".as_ptr() as *const ::core::ffi::c_char;
    tbl[9] = b"%gpr9\0".as_ptr() as *const ::core::ffi::c_char;
    tbl[10] = b"%gpr10\0".as_ptr() as *const ::core::ffi::c_char;
    tbl[11] = b"%gpr11\0".as_ptr() as *const ::core::ffi::c_char;
    tbl[12] = b"%gpr12\0".as_ptr() as *const ::core::ffi::c_char;
    tbl[13] = b"%gpr13\0".as_ptr() as *const ::core::ffi::c_char;
    tbl[14] = b"%gpr14\0".as_ptr() as *const ::core::ffi::c_char;
    tbl[15] = b"%gpr15\0".as_ptr() as *const ::core::ffi::c_char;
    tbl[16] = b"%gpr16\0".as_ptr() as *const ::core::ffi::c_char;
    tbl[17] = b"%gpr17\0".as_ptr() as *const ::core::ffi::c_char;
    tbl[18] = b"%gpr18\0".as_ptr() as *const ::core::ffi::c_char;
    tbl[19] = b"%gpr19\0".as_ptr() as *const ::core::ffi::c_char;
    tbl[20] = b"%gpr20\0".as_ptr() as *const ::core::ffi::c_char;
    tbl[21] = b"%gpr21\0".as_ptr() as *const ::core::ffi::c_char;
    tbl[22] = b"%gpr22\0".as_ptr() as *const ::core::ffi::c_char;
    tbl[23] = b"%gpr23\0".as_ptr() as *const ::core::ffi::c_char;
    tbl[24] = b"%gpr24\0".as_ptr() as *const ::core::ffi::c_char;
    tbl[25] = b"%gpr25\0".as_ptr() as *const ::core::ffi::c_char;
    tbl[26] = b"%gpr26\0".as_ptr() as *const ::core::ffi::c_char;
    tbl[27] = b"%gpr27\0".as_ptr() as *const ::core::ffi::c_char;
    tbl[28] = b"%gpr28\0".as_ptr() as *const ::core::ffi::c_char;
    tbl[29] = b"%gpr29\0".as_ptr() as *const ::core::ffi::c_char;
    tbl[30] = b"%gpr30\0".as_ptr() as *const ::core::ffi::c_char;
    tbl[31] = b"%gpr31\0".as_ptr() as *const ::core::ffi::c_char;
    tbl[66] = b"%msr\0".as_ptr() as *const ::core::ffi::c_char;
    tbl[109] = b"%ctr\0".as_ptr() as *const ::core::ffi::c_char;
    tbl[108] = b"%link\0".as_ptr() as *const ::core::ffi::c_char;
    tbl[101] = b"%xer\0".as_ptr() as *const ::core::ffi::c_char;
    tbl[119] = b"%dar\0".as_ptr() as *const ::core::ffi::c_char;
    tbl[118] = b"%dsisr\0".as_ptr() as *const ::core::ffi::c_char;

    tbl
};
