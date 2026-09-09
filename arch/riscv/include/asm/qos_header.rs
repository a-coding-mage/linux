/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from the C header.  The included kernel declarations are supplied
 * by the surrounding Rust kernel environment. */

#[cfg(CONFIG_RISCV_ISA_SSQOSID)]
extern "C" {
    pub static mut cpu_srmcfg: u32;
    pub static mut cpu_srmcfg_default: u32;
}

#[cfg(CONFIG_RISCV_ISA_SSQOSID)]
#[inline]
pub unsafe fn __switch_to_srmcfg(next: *mut task_struct) {
    let mut thread_srmcfg: u32 = core::ptr::read_volatile(
        &(*next).thread.srmcfg as *const u32,
    );
    let default_srmcfg: u32 = cpu_srmcfg_default;

    /*
     * RCID and MCID inherit from cpu_srmcfg_default independently.
     * RESCTRL_RESERVED_CLOSID and RESCTRL_RESERVED_RMID are both 0, so a
     * zero field means "unassigned" and takes the CPU default.
     */
    if thread_srmcfg == 0 {
        thread_srmcfg = default_srmcfg;
    } else {
        let mut rcid = field_get(SRMCFG_RCID_MASK, thread_srmcfg);
        let mut mcid = field_get(SRMCFG_MCID_MASK, thread_srmcfg);

        if rcid == 0 || mcid == 0 {
            if rcid == 0 {
                rcid = field_get(SRMCFG_RCID_MASK, default_srmcfg);
            }
            if mcid == 0 {
                mcid = field_get(SRMCFG_MCID_MASK, default_srmcfg);
            }
            thread_srmcfg = field_prep(SRMCFG_RCID_MASK, rcid)
                | field_prep(SRMCFG_MCID_MASK, mcid);
        }
    }

    if thread_srmcfg != cpu_srmcfg {
        /* No fence around the csrw; the tagging inaccuracy is bounded and
         * acceptable for QoS, as in the original implementation. */
        cpu_srmcfg = thread_srmcfg;
        csr_write(CSR_SRMCFG, thread_srmcfg);
    }
}

#[cfg(CONFIG_RISCV_ISA_SSQOSID)]
#[inline(always)]
pub fn has_srmcfg() -> bool {
    riscv_has_extension_unlikely(RISCV_ISA_EXT_SSQOSID)
}

#[cfg(not(CONFIG_RISCV_ISA_SSQOSID))]
pub struct task_struct;

#[cfg(not(CONFIG_RISCV_ISA_SSQOSID))]
#[inline(always)]
pub fn has_srmcfg() -> bool { false }

#[cfg(not(CONFIG_RISCV_ISA_SSQOSID))]
#[inline]
pub unsafe fn __switch_to_srmcfg(_next: *mut task_struct) { }

/* Kernel-provided declarations and constants. */
#[cfg(CONFIG_RISCV_ISA_SSQOSID)]
extern "C" {
    fn csr_write(csr: u32, value: u32);
    fn riscv_has_extension_unlikely(extension: u32) -> bool;
}

#[cfg(CONFIG_RISCV_ISA_SSQOSID)]
#[inline]
unsafe fn field_get(mask: u32, value: u32) -> u32 {
    (value & mask) >> mask.trailing_zeros()
}

#[cfg(CONFIG_RISCV_ISA_SSQOSID)]
#[inline]
unsafe fn field_prep(mask: u32, value: u32) -> u32 {
    (value << mask.trailing_zeros()) & mask
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
