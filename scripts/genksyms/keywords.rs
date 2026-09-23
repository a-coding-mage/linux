// SPDX-License-Identifier: GPL-2.0-only
// Reserved C spellings recognized by the genksyms declaration lexer.

use super::parser_tables::*;

pub(super) fn keyword(text: &[u8]) -> Option<i16> {
    Some(match text {
        b"__GENKSYMS_EXPORT_SYMBOL" => EXPORT_SYMBOL_KEYW,
        b"__asm" | b"__asm__" | b"asm" => ASM_KEYW,
        b"__attribute" | b"__attribute__" => ATTRIBUTE_KEYW,
        b"__const" | b"__const__" | b"const" => CONST_KEYW,
        b"__extension__" => EXTENSION_KEYW,
        b"__inline" | b"__inline__" | b"inline" => INLINE_KEYW,
        b"__signed" | b"__signed__" | b"signed" => SIGNED_KEYW,
        b"__typeof" | b"__typeof__" | b"__typeof_unqual" | b"__typeof_unqual__" | b"typeof"
        | b"typeof_unqual" => TYPEOF_KEYW,
        b"__volatile" | b"__volatile__" | b"volatile" => VOLATILE_KEYW,
        b"__builtin_va_list" => VA_LIST_KEYW,
        b"__int128" | b"__int128_t" | b"__uint128_t" => BUILTIN_INT_KEYW,
        b"_Bool" => BOOL_KEYW,
        b"__restrict" | b"__restrict__" | b"restrict" => RESTRICT_KEYW,
        b"_Static_assert" => STATIC_ASSERT_KEYW,
        b"__seg_gs" | b"__seg_fs" => X86_SEG_KEYW,
        b"auto" => AUTO_KEYW,
        b"char" => CHAR_KEYW,
        b"double" => DOUBLE_KEYW,
        b"enum" => ENUM_KEYW,
        b"extern" => EXTERN_KEYW,
        b"float" => FLOAT_KEYW,
        b"int" => INT_KEYW,
        b"long" => LONG_KEYW,
        b"register" => REGISTER_KEYW,
        b"short" => SHORT_KEYW,
        b"static" => STATIC_KEYW,
        b"struct" => STRUCT_KEYW,
        b"typedef" => TYPEDEF_KEYW,
        b"union" => UNION_KEYW,
        b"unsigned" => UNSIGNED_KEYW,
        b"void" => VOID_KEYW,
        _ => return None,
    })
}
