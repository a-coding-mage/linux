/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2016 ARM Limited
 */

// Dependency equivalent of: #include <asm-generic/sections.h>

unsafe extern "C" {
    pub static mut __alt_instructions: u8;
    pub static mut __alt_instructions_end: u8;
    pub static mut __hibernate_exit_text_start: u8;
    pub static mut __hibernate_exit_text_end: u8;
    pub static mut __hyp_idmap_text_start: u8;
    pub static mut __hyp_idmap_text_end: u8;
    pub static mut __hyp_text_start: u8;
    pub static mut __hyp_text_end: u8;
    pub static mut __hyp_data_start: u8;
    pub static mut __hyp_data_end: u8;
    pub static mut __hyp_rodata_start: u8;
    pub static mut __hyp_rodata_end: u8;
    pub static mut __hyp_reloc_begin: u8;
    pub static mut __hyp_reloc_end: u8;
    pub static mut __hyp_bss_start: u8;
    pub static mut __hyp_bss_end: u8;
    pub static mut __idmap_text_start: u8;
    pub static mut __idmap_text_end: u8;
    pub static mut __initdata_begin: u8;
    pub static mut __initdata_end: u8;
    pub static mut __inittext_begin: u8;
    pub static mut __inittext_end: u8;
    pub static mut __exittext_begin: u8;
    pub static mut __exittext_end: u8;
    pub static mut __irqentry_text_start: u8;
    pub static mut __irqentry_text_end: u8;
    pub static mut __mmuoff_data_start: u8;
    pub static mut __mmuoff_data_end: u8;
    pub static mut __entry_tramp_text_start: u8;
    pub static mut __entry_tramp_text_end: u8;
    pub static mut __relocate_new_kernel_start: u8;
    pub static mut __relocate_new_kernel_end: u8;
}

pub unsafe fn entry_tramp_text_size() -> usize {
    core::ptr::addr_of!(__entry_tramp_text_end)
        .offset_from(core::ptr::addr_of!(__entry_tramp_text_start)) as usize
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
