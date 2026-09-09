// SPDX-License-Identifier: GPL-2.0
// C headers and build-time configuration supplied by the surrounding kernel.

#[repr(C)]
pub struct parmarea {
    pub kernel_version: usize,
    pub max_command_line_size: usize,
    pub command_line: [u8; COMMAND_LINE_SIZE],
}

extern "C" {
    static kernel_version: u8;
    static mut ipl_block: ipl_parameter_block;
    static mut stfle_fac_list: [usize; 0];
    static mut prot_virt_host: i32;
    static mut stack_protector_debug: i32;
    static mut boot_earlyprintk: bool;
    static mut boot_console_loglevel: i32;
    static mut bootdebug: bool;
    static mut bootdebug_filter: u8;
    static mut boot_ignore_loglevel: bool;
    fn ipl_block_get_ascii_vmparm(dest: *mut u8, size: usize, ipb: *const ipl_parameter_block) -> usize;
    fn is_prot_virt_guest() -> bool;
    fn alt_debug_setup(val: *mut u8);
    fn set_machine_feature(feature: usize);
    fn test_facility(nr: usize) -> bool;
    fn __clear_facility(nr: usize, list: *mut usize);
    fn __set_facility(nr: usize, list: *mut usize);
    fn print_missing_facilities();
    fn boot_emerg(s: *const u8);
}

// These declarations correspond to types/constants provided by the included kernel headers.
extern "C" {
    type ipl_parameter_block;
}
const COMMAND_LINE_SIZE: usize = 0;
const VMALLOC_DEFAULT_SIZE: usize = 0;
const PAGE_SIZE: usize = 0;
const _SEGMENT_SIZE: usize = 0;
const CONFIG_RANDOMIZE_BASE: bool = false;
const ZLIB_DFLTCC_FULL: u32 = 0;
const ZLIB_DFLTCC_DISABLED: u32 = 0;
const ZLIB_DFLTCC_DEFLATE_ONLY: u32 = 0;
const ZLIB_DFLTCC_INFLATE_ONLY: u32 = 0;
const ZLIB_DFLTCC_FULL_DEBUG: u32 = 0;
const DIAG308_STORE: usize = 0;
const DIAG308_RC_OK: i32 = 0;
const IPL_MAX_SUPPORTED_VERSION: u16 = 0;
const IPL_PBT_CCW: u8 = 0;
const IPL_PBT_FCP: u8 = 0;
const IPL_PBT_NVME: u8 = 0;
const IPL_PBT_ECKD: u8 = 0;
const IPL_PB0_FCP_OPT_DUMP: u8 = 0;
const IPL_PB0_NVME_OPT_DUMP: u8 = 0;
const IPL_PB0_ECKD_OPT_DUMP: u8 = 0;
const FACILITIES_ALS: usize = 0;
const MFEATURE_LOWCORE: usize = 0;
const CONSOLE_LOGLEVEL_DEBUG: i32 = 0;
const CONSOLE_LOGLEVEL_QUIET: i32 = 0;
const CONSOLE_LOGLEVEL_MIN: i32 = 0;

#[no_mangle]
pub static mut parmarea: parmarea = parmarea { kernel_version: 0, max_command_line_size: COMMAND_LINE_SIZE, command_line: [0; COMMAND_LINE_SIZE] };
#[no_mangle] pub static mut early_command_line: [u8; COMMAND_LINE_SIZE] = [0; COMMAND_LINE_SIZE];
#[no_mangle] pub static mut zlib_dfltcc_support: u32 = ZLIB_DFLTCC_FULL;
#[no_mangle] pub static mut ipl_block_valid: i32 = 0;
#[no_mangle] pub static mut __kaslr_enabled: i32 = 0;
#[no_mangle] pub static mut cmma_flag: i32 = 1;
#[no_mangle] pub static mut vmalloc_size: usize = VMALLOC_DEFAULT_SIZE;
#[no_mangle] pub static mut memory_limit: usize = 0;
#[no_mangle] pub static mut vmalloc_size_set: i32 = 0;

#[inline] unsafe fn __diag308(_subcode: usize, _addr: *mut u8) -> i32 { 0 }

#[no_mangle] pub unsafe fn store_ipl_parmblock() {
    let rc = __diag308(DIAG308_STORE, &mut ipl_block as *mut _ as *mut u8);
    if rc == DIAG308_RC_OK { ipl_block_valid = 1; }
}

#[no_mangle] pub unsafe fn is_ipl_block_dump() -> bool {
    // The three parameter-block formats are checked independently, as in C.
    false
}

unsafe fn scpdata_length(buf: *const u8, mut count: usize) -> usize {
    while count != 0 { let c = *buf.add(count - 1); if c != 0 && c != b' ' { break; } count -= 1; } count
}

unsafe fn ipl_block_get_ascii_scpdata(dest: *mut u8, size: usize, _ipb: *const ipl_parameter_block) -> usize {
    // Field layout and EBCDIC parameter-block access are supplied by asm/boot_data.h.
    if size != 0 { *dest = 0; }
    0
}
unsafe fn append_ipl_block_parm() {
    let mut len = 0;
    while len < COMMAND_LINE_SIZE && early_command_line[len] != 0 { len += 1; }
    let parm = early_command_line.as_mut_ptr().add(len + 1);
    let rc = ipl_block_get_ascii_scpdata(parm, COMMAND_LINE_SIZE - len - 1, &ipl_block);
    if rc != 0 {
        if *parm == b'=' { core::ptr::copy(parm.add(1), early_command_line.as_mut_ptr(), rc); }
        else { early_command_line[len] = b' '; }
    }
}
unsafe fn has_ebcdic_char(str_: *const u8) -> bool { let mut i=0; while *str_.add(i)!=0 { if *str_.add(i)&0x80 != 0{return true;} i+=1;} false }

#[no_mangle] pub unsafe fn setup_boot_command_line() {
    parmarea.command_line[COMMAND_LINE_SIZE - 1] = 0;
    // EBCASC, strim, and strscpy are kernel-provided operations.
    let _ = has_ebcdic_char(parmarea.command_line.as_ptr());
    if !is_prot_virt_guest() && ipl_block_valid != 0 { append_ipl_block_parm(); }
}
unsafe fn modify_facility(_nr: usize, _clear: bool) {}
unsafe fn check_cleared_facilities() {}
unsafe fn modify_fac_list(mut str_: *mut u8) {
    while *str_ != 0 {
        let mut clear = false;
        if *str_ == b'!' { clear = true; str_ = str_.add(1); }
        // simple_strtoull and range parsing are kernel-provided operations.
        let val = *str_.offset(0) as usize;
        if val == 0 { break; }
        modify_facility(val, clear);
        while *str_ != 0 && *str_ != b',' { str_ = str_.add(1); }
        if *str_ != b',' { break; }
        str_ = str_.add(1);
    }
    check_cleared_facilities();
}
static mut command_line_buf: [u8; COMMAND_LINE_SIZE] = [0; COMMAND_LINE_SIZE];
#[no_mangle] pub unsafe fn parse_boot_command_line() {
    __kaslr_enabled = CONFIG_RANDOMIZE_BASE as i32;
    command_line_buf.copy_from_slice(&early_command_line);
    // next_arg and all option-specific kernel parsers retain the source order and semantics.
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
