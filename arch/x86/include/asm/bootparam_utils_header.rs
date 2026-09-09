/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from asm/bootparam_utils.h. */
/*
 * This file is included from multiple environments.  Do not
 * add completing includes to make it standalone.
 */

/*
 * Deal with bootloaders which fail to initialize unknown fields in
 * boot_params to zero.  The list fields in this list are taken from
 * analysis of kexec-tools; if other broken bootloaders initialize a
 * different set of fields we will need to figure out how to disambiguate.
 *
 * Note: efi_info is commonly left uninitialized, but that field has a
 * private magic, so it is better to leave it unchanged.
 */

#[repr(C)]
pub struct BootParamsToSave {
    pub start: core::ffi::c_uint,
    pub len: core::ffi::c_uint,
}

/* Equivalent of BOOT_PARAM_PRESERVE(struct_member). */
macro_rules! preserve {
    ($member:ident) => {
        BootParamsToSave {
            start: core::mem::offset_of!(boot_params, $member) as core::ffi::c_uint,
            len: core::mem::size_of_val(&(*core::ptr::null::<boot_params>()).$member)
                as core::ffi::c_uint,
        }
    };
}

#[inline]
pub unsafe fn sanitize_boot_params(boot_params: *mut boot_params) {
    /*
     * IMPORTANT NOTE TO BOOTLOADER AUTHORS: do not simply clear
     * this field.  The purpose of this field is to guarantee
     * compliance with the x86 boot spec located in
     * Documentation/arch/x86/boot.rst .  That spec says that the
     * *whole* structure should be cleared, after which only the
     * portion defined by struct setup_header (boot_params->hdr)
     * should be copied in.
     *
     * If you're having an issue because the sentinel is set, you
     * need to change the whole structure to be cleared, not this
     * (or any other) individual field, or you will soon have
     * problems again.
     */
    if (*core::ptr::addr_of!((*boot_params).sentinel) != 0) {
        static mut SCRATCH: core::mem::MaybeUninit<boot_params> =
            core::mem::MaybeUninit::uninit();

        let bp_base = boot_params.cast::<u8>();
        let save_base = SCRATCH.as_mut_ptr().cast::<u8>();

        let to_save: [BootParamsToSave; 20] = [
            preserve!(screen_info),
            preserve!(apm_bios_info),
            preserve!(tboot_addr),
            preserve!(ist_info),
            preserve!(hd0_info),
            preserve!(hd1_info),
            preserve!(sys_desc_table),
            preserve!(olpc_ofw_header),
            preserve!(efi_info),
            preserve!(alt_mem_k),
            preserve!(scratch),
            preserve!(e820_entries),
            preserve!(eddbuf_entries),
            preserve!(edd_mbr_sig_buf_entries),
            preserve!(edd_mbr_sig_buffer),
            preserve!(secure_boot),
            preserve!(hdr),
            preserve!(e820_table),
            preserve!(eddbuf),
            preserve!(cc_blob_address),
        ];

        core::ptr::write_bytes(save_base, 0, core::mem::size_of::<boot_params>());

        for entry in to_save.iter() {
            core::ptr::copy_nonoverlapping(
                bp_base.add(entry.start as usize),
                save_base.add(entry.start as usize),
                entry.len as usize,
            );
        }

        core::ptr::copy_nonoverlapping(
            save_base,
            boot_params.cast::<u8>(),
            core::mem::size_of::<boot_params>(),
        );
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
