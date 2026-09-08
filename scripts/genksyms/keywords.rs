// SPDX-License-Identifier: GPL-2.0-only

use std::os::raw::c_char;

#[repr(C)]
struct resword {
    name: *const c_char,
    token: i32,
}

static keywords: [resword; 53] = [
    resword { name: b"__GENKSYMS_EXPORT_SYMBOL\0".as_ptr() as *const c_char, token: EXPORT_SYMBOL_KEYW },
    resword { name: b"__asm\0".as_ptr() as *const c_char, token: ASM_KEYW },
    resword { name: b"__asm__\0".as_ptr() as *const c_char, token: ASM_KEYW },
    resword { name: b"__attribute\0".as_ptr() as *const c_char, token: ATTRIBUTE_KEYW },
    resword { name: b"__attribute__\0".as_ptr() as *const c_char, token: ATTRIBUTE_KEYW },
    resword { name: b"__const\0".as_ptr() as *const c_char, token: CONST_KEYW },
    resword { name: b"__const__\0".as_ptr() as *const c_char, token: CONST_KEYW },
    resword { name: b"__extension__\0".as_ptr() as *const c_char, token: EXTENSION_KEYW },
    resword { name: b"__inline\0".as_ptr() as *const c_char, token: INLINE_KEYW },
    resword { name: b"__inline__\0".as_ptr() as *const c_char, token: INLINE_KEYW },
    resword { name: b"__signed\0".as_ptr() as *const c_char, token: SIGNED_KEYW },
    resword { name: b"__signed__\0".as_ptr() as *const c_char, token: SIGNED_KEYW },
    resword { name: b"__typeof\0".as_ptr() as *const c_char, token: TYPEOF_KEYW },
    resword { name: b"__typeof__\0".as_ptr() as *const c_char, token: TYPEOF_KEYW },
    resword { name: b"__typeof_unqual\0".as_ptr() as *const c_char, token: TYPEOF_KEYW },
    resword { name: b"__typeof_unqual__\0".as_ptr() as *const c_char, token: TYPEOF_KEYW },
    resword { name: b"__volatile\0".as_ptr() as *const c_char, token: VOLATILE_KEYW },
    resword { name: b"__volatile__\0".as_ptr() as *const c_char, token: VOLATILE_KEYW },
    resword { name: b"__builtin_va_list\0".as_ptr() as *const c_char, token: VA_LIST_KEYW },
    resword { name: b"__int128\0".as_ptr() as *const c_char, token: BUILTIN_INT_KEYW },
    resword { name: b"__int128_t\0".as_ptr() as *const c_char, token: BUILTIN_INT_KEYW },
    resword { name: b"__uint128_t\0".as_ptr() as *const c_char, token: BUILTIN_INT_KEYW },
    // According to rth, c99 defines "_Bool", "__restrict", "__restrict__", "restrict".  KAO
    resword { name: b"_Bool\0".as_ptr() as *const c_char, token: BOOL_KEYW },
    resword { name: b"__restrict\0".as_ptr() as *const c_char, token: RESTRICT_KEYW },
    resword { name: b"__restrict__\0".as_ptr() as *const c_char, token: RESTRICT_KEYW },
    resword { name: b"restrict\0".as_ptr() as *const c_char, token: RESTRICT_KEYW },
    resword { name: b"asm\0".as_ptr() as *const c_char, token: ASM_KEYW },
    // c11 keywords that can be used at module scope
    resword { name: b"_Static_assert\0".as_ptr() as *const c_char, token: STATIC_ASSERT_KEYW },
    // X86 named address space qualifiers
    resword { name: b"__seg_gs\0".as_ptr() as *const c_char, token: X86_SEG_KEYW },
    resword { name: b"__seg_fs\0".as_ptr() as *const c_char, token: X86_SEG_KEYW },
    resword { name: b"auto\0".as_ptr() as *const c_char, token: AUTO_KEYW },
    resword { name: b"char\0".as_ptr() as *const c_char, token: CHAR_KEYW },
    resword { name: b"const\0".as_ptr() as *const c_char, token: CONST_KEYW },
    resword { name: b"double\0".as_ptr() as *const c_char, token: DOUBLE_KEYW },
    resword { name: b"enum\0".as_ptr() as *const c_char, token: ENUM_KEYW },
    resword { name: b"extern\0".as_ptr() as *const c_char, token: EXTERN_KEYW },
    resword { name: b"float\0".as_ptr() as *const c_char, token: FLOAT_KEYW },
    resword { name: b"inline\0".as_ptr() as *const c_char, token: INLINE_KEYW },
    resword { name: b"int\0".as_ptr() as *const c_char, token: INT_KEYW },
    resword { name: b"long\0".as_ptr() as *const c_char, token: LONG_KEYW },
    resword { name: b"register\0".as_ptr() as *const c_char, token: REGISTER_KEYW },
    resword { name: b"short\0".as_ptr() as *const c_char, token: SHORT_KEYW },
    resword { name: b"signed\0".as_ptr() as *const c_char, token: SIGNED_KEYW },
    resword { name: b"static\0".as_ptr() as *const c_char, token: STATIC_KEYW },
    resword { name: b"struct\0".as_ptr() as *const c_char, token: STRUCT_KEYW },
    resword { name: b"typedef\0".as_ptr() as *const c_char, token: TYPEDEF_KEYW },
    resword { name: b"typeof\0".as_ptr() as *const c_char, token: TYPEOF_KEYW },
    resword { name: b"typeof_unqual\0".as_ptr() as *const c_char, token: TYPEOF_KEYW },
    resword { name: b"union\0".as_ptr() as *const c_char, token: UNION_KEYW },
    resword { name: b"unsigned\0".as_ptr() as *const c_char, token: UNSIGNED_KEYW },
    resword { name: b"void\0".as_ptr() as *const c_char, token: VOID_KEYW },
    resword { name: b"volatile\0".as_ptr() as *const c_char, token: VOLATILE_KEYW },
];

const NR_KEYWORDS: usize = keywords.len();

unsafe extern "C" {
    fn strlen(s: *const c_char) -> usize;
    fn memcmp(a: *const std::ffi::c_void, b: *const std::ffi::c_void, n: usize) -> i32;
}

unsafe fn is_reserved_word(str_: *const c_char, len: u32) -> i32 {
    let mut i = 0usize;
    while i < NR_KEYWORDS {
        let r = &keywords[i];
        let l = strlen(r.name);
        if len as usize == l && memcmp(str_ as *const _, r.name as *const _, len as usize) == 0 {
            return r.token;
        }
        i += 1;
    }
    -1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
