/* SPDX-License-Identifier: GPL-2.0 */

// The assembler-only portion of the C header defines AArch64 assembler
// register aliases and macros.  It is retained here as conditional intent;
// those macros are not Rust items.
//
// #ifdef __ASSEMBLER__
// #ifdef CONFIG_SHADOW_CALL_STACK
// scs_sp .req x18
//
// .macro scs_load_current_base
//     get_current_task scs_sp
//     ldr scs_sp, [scs_sp, #TSK_TI_SCS_BASE]
// .endm
//
// .macro scs_load_current
//     get_current_task scs_sp
//     ldr scs_sp, [scs_sp, #TSK_TI_SCS_SP]
// .endm
//
// .macro scs_save tsk
//     str scs_sp, [\\tsk, #TSK_TI_SCS_SP]
// .endm
// #else
// .macro scs_load_current_base
// .endm
// .macro scs_load_current
// .endm
// .macro scs_save tsk
// .endm
// #endif

#[cfg(CONFIG_UNWIND_PATCH_PAC_INTO_SCS)]
extern "C" {
    static __pi_dynamic_scs_is_enabled: bool;
    static mut dynamic_scs_enabled: core::ffi::c_void;
    fn pr_info(fmt: *const u8, ...);
    fn static_branch_enable(key: *mut core::ffi::c_void);
}

#[cfg(CONFIG_UNWIND_PATCH_PAC_INTO_SCS)]
pub unsafe fn dynamic_scs_init() {
    if __pi_dynamic_scs_is_enabled {
        let message = b"Enabling dynamic shadow call stack\n\0";
        pr_info(message.as_ptr(),);
        static_branch_enable(core::ptr::addr_of_mut!(dynamic_scs_enabled));
    }
}

#[cfg(not(CONFIG_UNWIND_PATCH_PAC_INTO_SCS))]
pub fn dynamic_scs_init() {}

pub const EDYNSCS_INVALID_CIE_HEADER: i32 = 1;
pub const EDYNSCS_INVALID_CIE_SDATA_SIZE: i32 = 2;
pub const EDYNSCS_INVALID_FDE_AUGM_DATA_SIZE: i32 = 3;
pub const EDYNSCS_INVALID_CFA_OPCODE: i32 = 4;

extern "C" {
    pub fn __pi_scs_patch(eh_frame: *const u8, size: core::ffi::c_int, skip_dry_run: bool);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
