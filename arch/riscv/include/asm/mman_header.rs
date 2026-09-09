/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding Linux headers:
// linux/compiler.h, linux/types.h, linux/mm.h, and uapi/asm/mman.h.

#[inline]
pub fn arch_calc_vm_prot_bits(prot: usize, _pkey: usize) -> usize {
    let mut ret: usize = 0;

    /*
     * If PROT_WRITE was specified, force it to VM_READ | VM_WRITE.
     * Only VM_WRITE means shadow stack.
     */
    if prot & PROT_WRITE != 0 {
        ret = VM_READ | VM_WRITE;
    }
    ret
}

// The C macro arch_calc_vm_prot_bits(prot, pkey) expands to the function of
// the same name; the Rust function above provides the equivalent interface.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
