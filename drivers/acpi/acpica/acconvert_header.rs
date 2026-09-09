/* SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0 */
/******************************************************************************
 *
 * Module Name: acapps - common include for ACPI applications/tools
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

/* Definitions for comment state */
pub const ASL_COMMENT_STANDARD: u32 = 1;
pub const ASLCOMMENT_INLINE: u32 = 2;
pub const ASL_COMMENT_OPEN_PAREN: u32 = 3;
pub const ASL_COMMENT_CLOSE_PAREN: u32 = 4;
pub const ASL_COMMENT_CLOSE_BRACE: u32 = 5;

/* Definitions for comment print function */
pub const AML_COMMENT_STANDARD: u32 = 1;
pub const AMLCOMMENT_INLINE: u32 = 2;
pub const AML_COMMENT_END_NODE: u32 = 3;
pub const AML_NAMECOMMENT: u32 = 4;
pub const AML_COMMENT_CLOSE_BRACE: u32 = 5;
pub const AML_COMMENT_ENDBLK: u32 = 6;
pub const AML_COMMENT_INCLUDE: u32 = 7;

/* Declarations enabled when ACPI_ASL_COMPILER is defined in the C build. */
#[cfg(feature = "ACPI_ASL_COMPILER")]
unsafe extern "C" {
    pub fn cv_process_comment(
        current_state: asl_comment_state,
        string_buffer: *mut c_char,
        c1: c_int,
    );

    pub fn cv_process_comment_type2(
        current_state: asl_comment_state,
        string_buffer: *mut c_char,
    );

    pub fn cv_calculate_comment_lengths(op: *mut acpi_parse_object) -> u32;

    pub fn cv_process_comment_state(input: c_char);

    pub fn cv_append_inline_comment(
        inline_comment: *mut c_char,
        to_add: *mut c_char,
    ) -> *mut c_char;

    pub fn cv_add_to_comment_list(to_add: *mut c_char);

    pub fn cv_place_comment(type_: u8, comment_string: *mut c_char);

    pub fn cv_parse_op_block_type(op: *mut acpi_parse_object) -> u32;

    pub fn cv_comment_node_calloc() -> *mut acpi_comment_node;

    pub fn cg_write_aml_def_block_comment(op: *mut acpi_parse_object);

    pub fn cg_write_one_aml_comment(
        op: *mut acpi_parse_object,
        comment_to_print: *mut c_char,
        input_option: u8,
    );

    pub fn cg_write_aml_comment(op: *mut acpi_parse_object);

    pub fn cv_init_file_tree(table: *mut acpi_table_header, root_file: *mut FILE);

    pub fn cv_clear_op_comments(op: *mut acpi_parse_object);

    pub fn cv_filename_exists(
        filename: *mut c_char,
        head: *mut acpi_file_node,
    ) -> *mut acpi_file_node;

    pub fn cv_label_file_node(op: *mut acpi_parse_object);

    pub fn cv_capture_list_comments(
        parser_state: *mut acpi_parse_state,
        list_head: *mut acpi_comment_node,
        list_tail: *mut acpi_comment_node,
    );

    pub fn cv_capture_comments_only(parser_state: *mut acpi_parse_state);

    pub fn cv_capture_comments(walk_state: *mut acpi_walk_state);

    pub fn cv_transfer_comments(op: *mut acpi_parse_object);

    pub fn cv_switch_files(level: u32, op: *mut acpi_parse_object);

    pub fn cv_file_has_switched(op: *mut acpi_parse_object) -> u8;

    pub fn cv_close_paren_write_comment(op: *mut acpi_parse_object, level: u32);

    pub fn cv_close_brace_write_comment(op: *mut acpi_parse_object, level: u32);

    pub fn cv_print_one_comment_list(comment_list: *mut acpi_comment_node, level: u32);

    pub fn cv_print_one_comment_type(
        op: *mut acpi_parse_object,
        comment_type: u8,
        end_str: *mut c_char,
        level: u32,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
