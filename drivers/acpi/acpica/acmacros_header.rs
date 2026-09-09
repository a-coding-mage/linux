/* SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0 */
/* Rust translation of acmacros.h. External ACPICA types and functions are supplied elsewhere. */

#[macro_export] macro_rules! ACPI_CAST8 { ($p:expr) => { $p as *mut u8 }; }
#[macro_export] macro_rules! ACPI_CAST16 { ($p:expr) => { $p as *mut u16 }; }
#[macro_export] macro_rules! ACPI_CAST32 { ($p:expr) => { $p as *mut u32 }; }
#[macro_export] macro_rules! ACPI_CAST64 { ($p:expr) => { $p as *mut u64 }; }
#[macro_export] macro_rules! ACPI_GET8 { ($p:expr) => { unsafe { *ACPI_CAST8!($p) } }; }
#[macro_export] macro_rules! ACPI_GET16 { ($p:expr) => { unsafe { *ACPI_CAST16!($p) } }; }
#[macro_export] macro_rules! ACPI_GET32 { ($p:expr) => { unsafe { *ACPI_CAST32!($p) } }; }
#[macro_export] macro_rules! ACPI_GET64 { ($p:expr) => { unsafe { *ACPI_CAST64!($p) } }; }
#[macro_export] macro_rules! ACPI_SET8 { ($p:expr,$v:expr) => { unsafe { *ACPI_CAST8!($p) = $v as u8 } }; }
#[macro_export] macro_rules! ACPI_SET16 { ($p:expr,$v:expr) => { unsafe { *ACPI_CAST16!($p) = $v as u16 } }; }
#[macro_export] macro_rules! ACPI_SET32 { ($p:expr,$v:expr) => { unsafe { *ACPI_CAST32!($p) = $v as u32 } }; }
#[macro_export] macro_rules! ACPI_SET64 { ($p:expr,$v:expr) => { unsafe { *ACPI_CAST64!($p) = $v as u64 } }; }
#[macro_export] macro_rules! ACPI_FORMAT_UINT64 { ($i:expr) => { (($i >> 32) as u32, $i as u32) }; }

/* Little-endian, unaligned-transfer form; define the bytewise form when required by the build. */
#[cfg(not(feature = "acpi_big_endian"))]
#[macro_export] macro_rules! ACPI_MOVE_16_TO_16 { ($d:expr,$s:expr) => { unsafe { *(($d) as *mut u16) = *(($s) as *const u16) } }; }
#[cfg(not(feature = "acpi_big_endian"))]
#[macro_export] macro_rules! ACPI_MOVE_16_TO_32 { ($d:expr,$s:expr) => { unsafe { *(($d) as *mut u32) = *(($s) as *const u16) as u32 } }; }
#[cfg(not(feature = "acpi_big_endian"))]
#[macro_export] macro_rules! ACPI_MOVE_16_TO_64 { ($d:expr,$s:expr) => { unsafe { *(($d) as *mut u64) = *(($s) as *const u16) as u64 } }; }
#[cfg(not(feature = "acpi_big_endian"))]
#[macro_export] macro_rules! ACPI_MOVE_32_TO_16 { ($d:expr,$s:expr) => { ACPI_MOVE_16_TO_16!($d,$s) }; }
#[cfg(not(feature = "acpi_big_endian"))]
#[macro_export] macro_rules! ACPI_MOVE_32_TO_32 { ($d:expr,$s:expr) => { unsafe { *(($d) as *mut u32) = *(($s) as *const u32) } }; }
#[cfg(not(feature = "acpi_big_endian"))]
#[macro_export] macro_rules! ACPI_MOVE_32_TO_64 { ($d:expr,$s:expr) => { unsafe { *(($d) as *mut u64) = *(($s) as *const u32) as u64 } }; }
#[cfg(not(feature = "acpi_big_endian"))]
#[macro_export] macro_rules! ACPI_MOVE_64_TO_16 { ($d:expr,$s:expr) => { ACPI_MOVE_16_TO_16!($d,$s) }; }
#[cfg(not(feature = "acpi_big_endian"))]
#[macro_export] macro_rules! ACPI_MOVE_64_TO_32 { ($d:expr,$s:expr) => { ACPI_MOVE_32_TO_32!($d,$s) }; }
#[cfg(not(feature = "acpi_big_endian"))]
#[macro_export] macro_rules! ACPI_MOVE_64_TO_64 { ($d:expr,$s:expr) => { unsafe { *(($d) as *mut u64) = *(($s) as *const u64) } }; }

#[macro_export] macro_rules! _ACPI_DIV { ($v:expr,$p:expr) => { (($v as u32) >> $p) }; }
#[macro_export] macro_rules! _ACPI_MUL { ($v:expr,$p:expr) => { (($v as u32) << $p) }; }
#[macro_export] macro_rules! _ACPI_MOD { ($v:expr,$d:expr) => { (($v as u32) & (($d) - 1)) }; }
#[macro_export] macro_rules! ACPI_DIV_2 { ($a:expr) => { _ACPI_DIV!($a,1) }; }
#[macro_export] macro_rules! ACPI_MUL_2 { ($a:expr) => { _ACPI_MUL!($a,1) }; }
#[macro_export] macro_rules! ACPI_MOD_2 { ($a:expr) => { _ACPI_MOD!($a,2) }; }
#[macro_export] macro_rules! ACPI_DIV_4 { ($a:expr) => { _ACPI_DIV!($a,2) }; }
#[macro_export] macro_rules! ACPI_MUL_4 { ($a:expr) => { _ACPI_MUL!($a,2) }; }
#[macro_export] macro_rules! ACPI_MOD_4 { ($a:expr) => { _ACPI_MOD!($a,4) }; }
#[macro_export] macro_rules! ACPI_DIV_8 { ($a:expr) => { _ACPI_DIV!($a,3) }; }
#[macro_export] macro_rules! ACPI_MUL_8 { ($a:expr) => { _ACPI_MUL!($a,3) }; }
#[macro_export] macro_rules! ACPI_MOD_8 { ($a:expr) => { _ACPI_MOD!($a,8) }; }
#[macro_export] macro_rules! ACPI_DIV_16 { ($a:expr) => { _ACPI_DIV!($a,4) }; }
#[macro_export] macro_rules! ACPI_MUL_16 { ($a:expr) => { _ACPI_MUL!($a,4) }; }
#[macro_export] macro_rules! ACPI_MOD_16 { ($a:expr) => { _ACPI_MOD!($a,16) }; }
#[macro_export] macro_rules! ACPI_DIV_32 { ($a:expr) => { _ACPI_DIV!($a,5) }; }
#[macro_export] macro_rules! ACPI_MUL_32 { ($a:expr) => { _ACPI_MUL!($a,5) }; }
#[macro_export] macro_rules! ACPI_MOD_32 { ($a:expr) => { _ACPI_MOD!($a,32) }; }
#[macro_export] macro_rules! ACPI_IS_ASCII { ($c:expr) => { ($c) < 0x80 }; }
pub const ACPI_SIGN_POSITIVE: u32 = 0;
pub const ACPI_SIGN_NEGATIVE: u32 = 1;
#[macro_export] macro_rules! ACPI_ROUND_DOWN { ($v:expr,$b:expr) => { (($v as acpi_size) & !((($b as acpi_size)-1))) }; }
#[macro_export] macro_rules! ACPI_ROUND_UP { ($v:expr,$b:expr) => { (((($v as acpi_size)+(($b as acpi_size)-1)) & !(($b as acpi_size)-1))) }; }
#[macro_export] macro_rules! ACPI_ROUND_DOWN_TO_32BIT { ($a:expr) => { ACPI_ROUND_DOWN!($a,4) }; }
#[macro_export] macro_rules! ACPI_ROUND_DOWN_TO_64BIT { ($a:expr) => { ACPI_ROUND_DOWN!($a,8) }; }
#[macro_export] macro_rules! ACPI_ROUND_DOWN_TO_NATIVE_WORD { ($a:expr) => { ACPI_ROUND_DOWN!($a,core::mem::size_of::<acpi_size>()) }; }
#[macro_export] macro_rules! ACPI_ROUND_UP_TO_32BIT { ($a:expr) => { ACPI_ROUND_UP!($a,4) }; }
#[macro_export] macro_rules! ACPI_ROUND_UP_TO_64BIT { ($a:expr) => { ACPI_ROUND_UP!($a,8) }; }
#[macro_export] macro_rules! ACPI_ROUND_UP_TO_NATIVE_WORD { ($a:expr) => { ACPI_ROUND_UP!($a,core::mem::size_of::<acpi_size>()) }; }
#[macro_export] macro_rules! ACPI_ROUND_BITS_UP_TO_BYTES { ($a:expr) => { ACPI_DIV_8!(($a)+7) }; }
#[macro_export] macro_rules! ACPI_ROUND_BITS_DOWN_TO_BYTES { ($a:expr) => { ACPI_DIV_8!($a) }; }
#[macro_export] macro_rules! ACPI_ROUND_UP_TO_1K { ($a:expr) => { (($a+1023)>>10) }; }
#[macro_export] macro_rules! ACPI_ROUND_UP_TO { ($v:expr,$b:expr) => { (($v+(($b)-1))/($b)) }; }
#[macro_export] macro_rules! ACPI_IS_MISALIGNED { ($v:expr) => { (($v as acpi_size)&(core::mem::size_of::<acpi_size>()-1)) }; }

#[macro_export] macro_rules! ACPI_FIND_LAST_BIT_8 { ($a:expr) => { if ($a)!=0 { (8-($a as u8).leading_zeros()) as u32 } else { 0 } }; }
#[macro_export] macro_rules! ACPI_FIND_LAST_BIT_16 { ($a:expr) => { if ($a)!=0 { (16-($a as u16).leading_zeros()) as u32 } else { 0 } }; }
#[macro_export] macro_rules! ACPI_FIND_LAST_BIT_32 { ($a:expr) => { if ($a)!=0 { (32-($a as u32).leading_zeros()) as u32 } else { 0 } }; }
#[macro_export] macro_rules! ACPI_FIND_LAST_BIT_64 { ($a:expr) => { if ($a)!=0 { (64-($a as u64).leading_zeros()) as u32 } else { 0 } }; }
#[macro_export] macro_rules! ACPI_FIND_FIRST_BIT_8 { ($a:expr) => { if ($a)!=0 { (($a as u8).trailing_zeros()+1) as u32 } else { 0 } }; }
#[macro_export] macro_rules! ACPI_FIND_FIRST_BIT_16 { ($a:expr) => { if ($a)!=0 { (($a as u16).trailing_zeros()+1) as u32 } else { 0 } }; }
#[macro_export] macro_rules! ACPI_FIND_FIRST_BIT_32 { ($a:expr) => { if ($a)!=0 { (($a as u32).trailing_zeros()+1) as u32 } else { 0 } }; }
#[macro_export] macro_rules! ACPI_FIND_FIRST_BIT_64 { ($a:expr) => { if ($a)!=0 { (($a as u64).trailing_zeros()+1) as u32 } else { 0 } }; }
#[macro_export] macro_rules! ACPI_ROUND_UP_POWER_OF_TWO_8 { ($a:expr) => { (1u16 << ACPI_FIND_LAST_BIT_8!(($a)-1)) as u8 }; }
#[macro_export] macro_rules! ACPI_ROUND_DOWN_POWER_OF_TWO_8 { ($a:expr) => { (1u16 << (ACPI_FIND_LAST_BIT_8!($a)-1)) as u8 }; }
#[macro_export] macro_rules! ACPI_ROUND_UP_POWER_OF_TWO_16 { ($a:expr) => { (1u32 << ACPI_FIND_LAST_BIT_16!(($a)-1)) as u16 }; }
#[macro_export] macro_rules! ACPI_ROUND_DOWN_POWER_OF_TWO_16 { ($a:expr) => { (1u32 << (ACPI_FIND_LAST_BIT_16!($a)-1)) as u16 }; }
#[macro_export] macro_rules! ACPI_ROUND_UP_POWER_OF_TWO_32 { ($a:expr) => { (1u64 << ACPI_FIND_LAST_BIT_32!(($a)-1)) as u32 }; }
#[macro_export] macro_rules! ACPI_ROUND_DOWN_POWER_OF_TWO_32 { ($a:expr) => { (1u64 << (ACPI_FIND_LAST_BIT_32!($a)-1)) as u32 }; }
#[macro_export] macro_rules! ACPI_IS_ALIGNED { ($a:expr,$s:expr) => { (($a & (($s)-1))==0) }; }
#[macro_export] macro_rules! ACPI_IS_POWER_OF_TWO { ($a:expr) => { ACPI_IS_ALIGNED!($a,$a) }; }
#[macro_export] macro_rules! ACPI_MASK_BITS_ABOVE { ($p:expr) => { !((ACPI_UINT64_MAX) << ($p as u32)) }; }
#[macro_export] macro_rules! ACPI_MASK_BITS_BELOW { ($p:expr) => { (ACPI_UINT64_MAX) << ($p as u32) }; }
#[macro_export] macro_rules! ACPI_MASK_BITS_ABOVE_32 { ($w:expr) => { ACPI_MASK_BITS_ABOVE!($w) as u32 }; }
#[macro_export] macro_rules! ACPI_MASK_BITS_BELOW_32 { ($w:expr) => { ACPI_MASK_BITS_BELOW!($w) as u32 }; }
#[macro_export] macro_rules! ACPI_MASK_BITS_ABOVE_64 { ($w:expr) => { if $w == ACPI_INTEGER_BIT_SIZE { ACPI_UINT64_MAX } else { ACPI_MASK_BITS_ABOVE!($w) } }; }
#[macro_export] macro_rules! ACPI_MASK_BITS_BELOW_64 { ($w:expr) => { if $w == ACPI_INTEGER_BIT_SIZE { 0u64 } else { ACPI_MASK_BITS_BELOW!($w) } }; }
#[macro_export] macro_rules! ACPI_REGISTER_PREPARE_BITS { ($v:expr,$p:expr,$m:expr) => { (($v << $p) & $m) }; }
#[macro_export] macro_rules! ACPI_REGISTER_INSERT_VALUE { ($r:expr,$p:expr,$m:expr,$v:expr) => { $r = (($r & !$m) | ACPI_REGISTER_PREPARE_BITS!($v,$p,$m)) }; }
#[macro_export] macro_rules! ACPI_INSERT_BITS { ($t:expr,$m:expr,$s:expr) => { $t = (($t & !$m) | ($s & $m)) }; }
#[macro_export] macro_rules! ACPI_GET_BITS { ($p:expr,$pos:expr,$mask:expr) => { ((*($p) >> $pos) & $mask) }; }
#[macro_export] macro_rules! ACPI_SET_BITS { ($p:expr,$pos:expr,$mask:expr,$v:expr) => { *($p) |= (($v & $mask) << $pos) }; }
pub const ACPI_1BIT_MASK: u32=0x00000001; pub const ACPI_2BIT_MASK:u32=0x00000003; pub const ACPI_3BIT_MASK:u32=0x00000007; pub const ACPI_4BIT_MASK:u32=0x0000000F; pub const ACPI_5BIT_MASK:u32=0x0000001F; pub const ACPI_6BIT_MASK:u32=0x0000003F; pub const ACPI_7BIT_MASK:u32=0x0000007F; pub const ACPI_8BIT_MASK:u32=0x000000FF; pub const ACPI_16BIT_MASK:u32=0x0000FFFF; pub const ACPI_24BIT_MASK:u32=0x00FFFFFF;
#[macro_export] macro_rules! ACPI_GET_1BIT_FLAG { ($v:expr) => { ($v)&ACPI_1BIT_MASK }; } #[macro_export] macro_rules! ACPI_GET_2BIT_FLAG { ($v:expr) => { ($v)&ACPI_2BIT_MASK }; } #[macro_export] macro_rules! ACPI_GET_3BIT_FLAG { ($v:expr) => { ($v)&ACPI_3BIT_MASK }; } #[macro_export] macro_rules! ACPI_GET_4BIT_FLAG { ($v:expr) => { ($v)&ACPI_4BIT_MASK }; }
#[macro_export] macro_rules! ACPI_EXTRACT_1BIT_FLAG { ($f:expr,$p:expr) => { ACPI_GET_1BIT_FLAG!(($f)>>$p) }; } #[macro_export] macro_rules! ACPI_EXTRACT_2BIT_FLAG { ($f:expr,$p:expr) => { ACPI_GET_2BIT_FLAG!(($f)>>$p) }; } #[macro_export] macro_rules! ACPI_EXTRACT_3BIT_FLAG { ($f:expr,$p:expr) => { ACPI_GET_3BIT_FLAG!(($f)>>$p) }; } #[macro_export] macro_rules! ACPI_EXTRACT_4BIT_FLAG { ($f:expr,$p:expr) => { ACPI_GET_4BIT_FLAG!(($f)>>$p) }; }
#[macro_export] macro_rules! ACPI_IS_ROOT_PREFIX { ($c:expr) => { ($c)==0x5Cu8 }; } #[macro_export] macro_rules! ACPI_IS_PARENT_PREFIX { ($c:expr) => { ($c)==0x5Eu8 }; } #[macro_export] macro_rules! ACPI_IS_PATH_SEPARATOR { ($c:expr) => { ($c)==0x2Eu8 }; }
#[macro_export] macro_rules! ARG_TYPE_WIDTH { () => { 5 }; } #[macro_export] macro_rules! ARG_1 { ($x:expr) => { $x as u32 }; } #[macro_export] macro_rules! ARG_2 { ($x:expr) => { ($x as u32)<<5 }; } #[macro_export] macro_rules! ARG_3 { ($x:expr) => { ($x as u32)<<10 }; } #[macro_export] macro_rules! ARG_4 { ($x:expr) => { ($x as u32)<<15 }; } #[macro_export] macro_rules! ARG_5 { ($x:expr) => { ($x as u32)<<20 }; } #[macro_export] macro_rules! ARG_6 { ($x:expr) => { ($x as u32)<<25 }; }
#[macro_export] macro_rules! ARGI_LIST1 { ($a:expr) => { ARG_1!($a) }; } #[macro_export] macro_rules! ARGI_LIST2 { ($a:expr,$b:expr) => { ARG_1!($b)|ARG_2!($a) }; } #[macro_export] macro_rules! ARGI_LIST3 { ($a:expr,$b:expr,$c:expr) => { ARG_1!($c)|ARG_2!($b)|ARG_3!($a) }; } #[macro_export] macro_rules! ARGI_LIST4 { ($a:expr,$b:expr,$c:expr,$d:expr) => { ARG_1!($d)|ARG_2!($c)|ARG_3!($b)|ARG_4!($a) }; } #[macro_export] macro_rules! ARGI_LIST5 { ($a:expr,$b:expr,$c:expr,$d:expr,$e:expr) => { ARG_1!($e)|ARG_2!($d)|ARG_3!($c)|ARG_4!($b)|ARG_5!($a) }; } #[macro_export] macro_rules! ARGI_LIST6 { ($a:expr,$b:expr,$c:expr,$d:expr,$e:expr,$f:expr) => { ARG_1!($f)|ARG_2!($e)|ARG_3!($d)|ARG_4!($c)|ARG_5!($b)|ARG_6!($a) }; }
#[macro_export] macro_rules! ARGP_LIST1 { ($a:expr) => { ARG_1!($a) }; } #[macro_export] macro_rules! ARGP_LIST2 { ($a:expr,$b:expr) => { ARG_1!($a)|ARG_2!($b) }; } #[macro_export] macro_rules! ARGP_LIST3 { ($a:expr,$b:expr,$c:expr) => { ARG_1!($a)|ARG_2!($b)|ARG_3!($c) }; } #[macro_export] macro_rules! ARGP_LIST4 { ($a:expr,$b:expr,$c:expr,$d:expr) => { ARG_1!($a)|ARG_2!($b)|ARG_3!($c)|ARG_4!($d) }; } #[macro_export] macro_rules! ARGP_LIST5 { ($a:expr,$b:expr,$c:expr,$d:expr,$e:expr) => { ARG_1!($a)|ARG_2!($b)|ARG_3!($c)|ARG_4!($d)|ARG_5!($e) }; } #[macro_export] macro_rules! ARGP_LIST6 { ($a:expr,$b:expr,$c:expr,$d:expr,$e:expr,$f:expr) => { ARG_1!($a)|ARG_2!($b)|ARG_3!($c)|ARG_4!($d)|ARG_5!($e)|ARG_6!($f) }; }
#[macro_export] macro_rules! GET_CURRENT_ARG_TYPE { ($l:expr) => { $l & 0x1F }; } #[macro_export] macro_rules! INCREMENT_ARG_LIST { ($l:expr) => { $l >>= 5 }; }
#[macro_export] macro_rules! ACPI_IS_OCTAL_DIGIT { ($d:expr) => { (($d as i8)>='0' as i8)&&(($d as i8)<='7' as i8) }; }
#[macro_export] macro_rules! ACPI_INIT_UUID { ($a:expr,$b:expr,$c:expr,$($d:expr),*) => { (($a)&0xFF),(($a>>8)&0xFF),(($a>>16)&0xFF),(($a>>24)&0xFF),(($b)&0xFF),(($b>>8)&0xFF),(($c)&0xFF),(($c>>8)&0xFF),$($d),* }; }

/* Descriptor accessors retain the C union layout through caller-provided fields. */
#[macro_export] macro_rules! ACPI_GET_DESCRIPTOR_PTR { ($d:expr) => { unsafe { (*($d as *mut acpi_descriptor)).common.common_pointer } }; }
#[macro_export] macro_rules! ACPI_SET_DESCRIPTOR_PTR { ($d:expr,$p:expr) => { unsafe { (*($d as *mut acpi_descriptor)).common.common_pointer = $p } }; }
#[macro_export] macro_rules! ACPI_GET_DESCRIPTOR_TYPE { ($d:expr) => { unsafe { (*($d as *mut acpi_descriptor)).common.descriptor_type } }; }
#[macro_export] macro_rules! ACPI_SET_DESCRIPTOR_TYPE { ($d:expr,$t:expr) => { unsafe { (*($d as *mut acpi_descriptor)).common.descriptor_type = $t } }; }

#[macro_export] macro_rules! ACPI_GET_BITS_FROM_PTR { ($p:expr,$position:expr,$mask:expr) => { ((*($p) >> $position) & $mask) }; }
/* Error and converter hooks are intentionally external; build configurations select their bodies. */
#[macro_export] macro_rules! ACPI_ERROR_NAMESPACE { ($s:expr,$p:expr,$e:expr) => { acpi_ut_prefixed_namespace_error(AE_INFO,$s,$p,$e); }; }
#[macro_export] macro_rules! ACPI_ERROR_METHOD { ($s:expr,$n:expr,$p:expr,$e:expr) => { acpi_ut_method_error(AE_INFO,$s,$n,$p,$e); }; }
#[macro_export] macro_rules! ACPI_WARN_PREDEFINED { ($($x:tt)*) => { acpi_ut_predefined_warning!($($x)*); }; }
#[macro_export] macro_rules! ACPI_INFO_PREDEFINED { ($($x:tt)*) => { acpi_ut_predefined_info!($($x)*); }; }
#[macro_export] macro_rules! ACPI_BIOS_ERROR_PREDEFINED { ($($x:tt)*) => { acpi_ut_predefined_bios_error!($($x)*); }; }
#[macro_export] macro_rules! ACPI_ERROR_ONLY { ($s:expr) => { $s }; }
#[macro_export] macro_rules! ACPI_HW_OPTIONAL_FUNCTION { ($a:expr) => { $a }; }
#[macro_export] macro_rules! ASL_CV_LABEL_FILENODE { ($a:expr) => { cv_label_file_node($a); }; }
#[macro_export] macro_rules! ASL_CV_CAPTURE_COMMENTS_ONLY { ($a:expr) => { cv_capture_comments_only($a); }; }
#[macro_export] macro_rules! ASL_CV_CAPTURE_COMMENTS { ($a:expr) => { cv_capture_comments($a); }; }
#[macro_export] macro_rules! ASL_CV_TRANSFER_COMMENTS { ($a:expr) => { cv_transfer_comments($a); }; }
#[macro_export] macro_rules! ASL_CV_CLOSE_PAREN { ($a:expr,$b:expr) => { cv_close_paren_write_comment($a,$b); }; }
#[macro_export] macro_rules! ASL_CV_CLOSE_BRACE { ($a:expr,$b:expr) => { cv_close_brace_write_comment($a,$b); }; }
#[macro_export] macro_rules! ASL_CV_SWITCH_FILES { ($a:expr,$b:expr) => { cv_switch_files($a,$b); }; }
#[macro_export] macro_rules! ASL_CV_CLEAR_OP_COMMENTS { ($a:expr) => { cv_clear_op_comments($a); }; }
#[macro_export] macro_rules! ASL_CV_PRINT_ONE_COMMENT { ($a:expr,$b:expr,$c:expr,$d:expr) => { cv_print_one_comment_type($a,$b,$c,$d); }; }
#[macro_export] macro_rules! ASL_CV_PRINT_ONE_COMMENT_LIST { ($a:expr,$b:expr) => { cv_print_one_comment_list($a,$b); }; }
#[macro_export] macro_rules! ASL_CV_FILE_HAS_SWITCHED { ($a:expr) => { cv_file_has_switched($a) }; }
#[macro_export] macro_rules! ASL_CV_INIT_FILETREE { ($a:expr,$b:expr) => { cv_init_file_tree($a,$b); }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
