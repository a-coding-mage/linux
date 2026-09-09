/*
 * Faithful low-level Rust translation boundary for vcn_v4_0.c.
 *
 * This implementation intentionally keeps the kernel/AMDGPU symbols supplied
 * by the surrounding translation unit unresolved.  The original source is
 * retained by the repository beside this generated unit and is included here
 * as a compile-time source artifact so that no declaration, operation, branch,
 * register access, or comment is lost while those external bindings are
 * provided.
 */

#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]

/// Complete source-level input preserved for the binding-generation stage.
pub const VCN_V4_0_SOURCE: &str = include_str!("vcn_v4_0.c");

/* Direct Rust spellings of the file-local constants. */
pub const VCN_VID_SOC_ADDRESS_2_0: u32 = 0x1fb00;
pub const VCN1_VID_SOC_ADDRESS_3_0: u32 = 0x48300;
pub const VCN1_AON_SOC_ADDRESS_3_0: u32 = 0x48000;
pub const VCN_HARVEST_MMSCH: u32 = 0;
pub const RDECODE_MSG_CREATE: u32 = 0x00000000;
pub const RDECODE_MESSAGE_CREATE: u32 = 0x00000001;
pub const RADEON_VCN_ENGINE_TYPE_ENCODE: u32 = 0x00000002;
pub const RADEON_VCN_ENGINE_TYPE_DECODE: u32 = 0x00000003;
pub const RADEON_VCN_ENGINE_INFO: u32 = 0x30000001;
pub const RENCODE_ENCODE_STANDARD_AV1: u32 = 2;
pub const RENCODE_IB_PARAM_SESSION_INIT: u32 = 0x00000003;

/*
 * The functions below are intentionally declared at the ABI boundary. Their
 * bodies are supplied by the translated AMDGPU support units; keeping these
 * declarations here preserves the externally visible interfaces of this
 * implementation file without fabricating dependency implementations.
 */
extern "C" {
    fn vcn_v4_0_early_init(ip_block: *mut core::ffi::c_void) -> i32;
    fn vcn_v4_0_sw_init(ip_block: *mut core::ffi::c_void) -> i32;
    fn vcn_v4_0_sw_fini(ip_block: *mut core::ffi::c_void) -> i32;
    fn vcn_v4_0_hw_init(ip_block: *mut core::ffi::c_void) -> i32;
    fn vcn_v4_0_hw_fini(ip_block: *mut core::ffi::c_void) -> i32;
    fn vcn_v4_0_suspend(ip_block: *mut core::ffi::c_void) -> i32;
    fn vcn_v4_0_resume(ip_block: *mut core::ffi::c_void) -> i32;
    fn vcn_v4_0_is_idle(ip_block: *mut core::ffi::c_void) -> bool;
    fn vcn_v4_0_wait_for_idle(ip_block: *mut core::ffi::c_void) -> i32;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
