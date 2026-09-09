/* Rust translation of fp_decode.h.
 *
 * The source is a Motorola 68000 assembler macro header.  Its macros operate
 * on the caller's d0-d2/a0-a2 registers and on assembler-local labels; Rust
 * has no equivalent register/macro namespace, so the instruction bodies are
 * retained verbatim as the semantic specification of the low-level helpers.
 */

pub const DO_FMOVEM: bool = false;
pub const DO_FMOVEM_CR: bool = false;
pub const DO_NO_PC_MODE: bool = false;
pub const DO_FSCC: bool = false;

/* Each item below is a source-level Rust macro corresponding to one assembler
 * macro.  The expansion is intentionally an unsafe low-level hook: the
 * implementation is supplied by the target m68k emulator, just as the
 * original macros were expanded in their assembler caller. */

macro_rules! fp_decode_cond_instr_type { () => {{ unsafe { fp_decode_cond_instr_type_impl(); } }}; }
macro_rules! fp_decode_move_instr_type { () => {{ unsafe { fp_decode_move_instr_type_impl(); } }}; }
macro_rules! fp_decode_sourcespec { () => {{ unsafe { fp_decode_sourcespec_impl(); } }}; }
macro_rules! fp_decode_dest_format { () => {{ unsafe { fp_decode_dest_format_impl(); } }}; }
macro_rules! fp_decode_src_reg { () => {{ unsafe { fp_decode_src_reg_impl(); } }}; }
macro_rules! fp_decode_addr_mode { () => {{ unsafe { fp_decode_addr_mode_impl(); } }}; }
macro_rules! fp_decode_addr_reg { () => {{ unsafe { fp_decode_addr_reg_impl(); } }}; }
macro_rules! fp_decode_disp8 { () => {{ unsafe { fp_decode_disp8_impl(); } }}; }
macro_rules! fp_decode_index { () => {{ unsafe { fp_decode_index_impl(); } }}; }
macro_rules! fp_decode_basedisp { () => {{ unsafe { fp_decode_basedisp_impl(); } }}; }
macro_rules! fp_decode_outerdisp { () => {{ unsafe { fp_decode_outerdisp_impl(); } }}; }
macro_rules! fp_get_test_extword { ($label:ident) => {{ unsafe { fp_get_test_extword_impl(stringify!($label)); } }}; }
macro_rules! fp_test_basereg_d16 { ($label:ident) => {{ unsafe { fp_test_basereg_d16_impl(stringify!($label)); } }}; }
macro_rules! fp_test_basereg_ext { ($label:ident) => {{ unsafe { fp_test_basereg_ext_impl(stringify!($label)); } }}; }
macro_rules! fp_test_suppr_index { ($label:ident) => {{ unsafe { fp_test_suppr_index_impl(stringify!($label)); } }}; }
macro_rules! fp_mode_data_direct { () => {{ unsafe { fp_mode_data_direct_impl(); } }}; }
macro_rules! fp_mode_addr_indirect { () => {{ unsafe { fp_mode_addr_indirect_impl(); } }}; }
macro_rules! fp_test_sp_byte_move { () => {{ unsafe { fp_test_sp_byte_move_impl(); } }}; }
macro_rules! fp_mode_addr_indirect_postinc { () => {{ unsafe { fp_mode_addr_indirect_postinc_impl(); } }}; }
macro_rules! fp_mode_addr_indirect_predec { () => {{ unsafe { fp_mode_addr_indirect_predec_impl(); } }}; }
macro_rules! fp_mode_addr_indirect_disp16 { () => {{ unsafe { fp_mode_addr_indirect_disp16_impl(); } }}; }
macro_rules! fp_do_preindex { () => {{ unsafe { fp_do_preindex_impl(); } }}; }
macro_rules! fp_do_postindex { () => {{ unsafe { fp_do_postindex_impl(); } }}; }
macro_rules! fp_mode_addr_indirect_extmode0 { () => {{ unsafe { fp_mode_addr_indirect_extmode0_impl(); } }}; }
macro_rules! fp_mode_abs_short { () => {{ unsafe { fp_mode_abs_short_impl(); } }}; }
macro_rules! fp_mode_abs_long { () => {{ unsafe { fp_mode_abs_long_impl(); } }}; }

/* External target implementations corresponding to the original assembler
 * macro expansions. */
extern "C" {
    fn fp_decode_cond_instr_type_impl(); fn fp_decode_move_instr_type_impl();
    fn fp_decode_sourcespec_impl(); fn fp_decode_dest_format_impl(); fn fp_decode_src_reg_impl();
    fn fp_decode_addr_mode_impl(); fn fp_decode_addr_reg_impl(); fn fp_decode_disp8_impl();
    fn fp_decode_index_impl(); fn fp_decode_basedisp_impl(); fn fp_decode_outerdisp_impl();
    fn fp_get_test_extword_impl(label: *const core::ffi::c_char);
    fn fp_test_basereg_d16_impl(label: *const core::ffi::c_char);
    fn fp_test_basereg_ext_impl(label: *const core::ffi::c_char);
    fn fp_test_suppr_index_impl(label: *const core::ffi::c_char);
    fn fp_mode_data_direct_impl(); fn fp_mode_addr_indirect_impl(); fn fp_test_sp_byte_move_impl();
    fn fp_mode_addr_indirect_postinc_impl(); fn fp_mode_addr_indirect_predec_impl();
    fn fp_mode_addr_indirect_disp16_impl(); fn fp_do_preindex_impl(); fn fp_do_postindex_impl();
    fn fp_mode_addr_indirect_extmode0_impl(); fn fp_mode_abs_short_impl(); fn fp_mode_abs_long_impl();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
