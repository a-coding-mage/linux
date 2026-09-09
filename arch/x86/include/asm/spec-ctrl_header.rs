/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

extern "C" {
    pub fn x86_virt_spec_ctrl(guest_virt_spec_ctrl: u64, guest: bool);

    pub static mut x86_amd_ls_cfg_base: u64;
    pub static mut x86_amd_ls_cfg_ssbd_mask: u64;

    pub fn speculation_ctrl_update(tif: usize);
    pub fn speculation_ctrl_update_current();
    pub static mut itlb_multihit_kvm_mitigation: bool;
}

/// x86_spec_ctrl_set_guest - Set speculation control registers for the guest.
/// Avoids writing to the MSR if the content/bits are the same.
#[inline]
pub unsafe fn x86_spec_ctrl_set_guest(guest_virt_spec_ctrl: u64) {
    x86_virt_spec_ctrl(guest_virt_spec_ctrl, true);
}

/// x86_spec_ctrl_restore_host - Restore host speculation control registers.
/// Avoids writing to the MSR if the content/bits are the same.
#[inline]
pub unsafe fn x86_spec_ctrl_restore_host(guest_virt_spec_ctrl: u64) {
    x86_virt_spec_ctrl(guest_virt_spec_ctrl, false);
}

#[inline]
pub fn ssbd_tif_to_spec_ctrl(tifn: u64) -> u64 {
    // BUILD_BUG_ON(TIF_SSBD < SPEC_CTRL_SSBD_SHIFT)
    (tifn & _TIF_SSBD) >> (TIF_SSBD - SPEC_CTRL_SSBD_SHIFT)
}

#[inline]
pub fn stibp_tif_to_spec_ctrl(tifn: u64) -> u64 {
    // BUILD_BUG_ON(TIF_SPEC_IB < SPEC_CTRL_STIBP_SHIFT)
    (tifn & _TIF_SPEC_IB) >> (TIF_SPEC_IB - SPEC_CTRL_STIBP_SHIFT)
}

#[inline]
pub fn ssbd_spec_ctrl_to_tif(spec_ctrl: u64) -> usize {
    // BUILD_BUG_ON(TIF_SSBD < SPEC_CTRL_SSBD_SHIFT)
    ((spec_ctrl & SPEC_CTRL_SSBD) << (TIF_SSBD - SPEC_CTRL_SSBD_SHIFT)) as usize
}

#[inline]
pub fn stibp_spec_ctrl_to_tif(spec_ctrl: u64) -> usize {
    // BUILD_BUG_ON(TIF_SPEC_IB < SPEC_CTRL_STIBP_SHIFT)
    ((spec_ctrl & SPEC_CTRL_STIBP) << (TIF_SPEC_IB - SPEC_CTRL_STIBP_SHIFT)) as usize
}

#[inline]
pub unsafe fn ssbd_tif_to_amd_ls_cfg(tifn: u64) -> u64 {
    if (tifn & _TIF_SSBD) != 0 {
        x86_amd_ls_cfg_ssbd_mask
    } else {
        0u64
    }
}

/*
 * This can be used in noinstr functions and should only be called in bare
 * metal context.
 */
#[inline(always)]
pub unsafe fn __update_spec_ctrl(val: u64) {
    __this_cpu_write(x86_spec_ctrl_current, val);
    native_wrmsrq(MSR_IA32_SPEC_CTRL, val);
}

#[cfg(CONFIG_SMP)]
extern "C" {
    pub fn speculative_store_bypass_ht_init();
}

#[cfg(not(CONFIG_SMP))]
#[inline]
pub fn speculative_store_bypass_ht_init() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
