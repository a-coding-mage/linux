/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Clang Control Flow Integrity (CFI) type definitions.
 */

/*
 * C header guard removed in Rust translation:
 * _LINUX_CFI_TYPES_H
 */

/*
 * C preprocessor condition preserved from the original header:
 *
 * - When __ASSEMBLY__ is defined, this header includes <linux/linkage.h> and
 *   provides assembly macros for typed entry points.
 * - Otherwise, when CONFIG_CFI is defined, it provides DEFINE_CFI_TYPE().
 */

/*
 * Original __ASSEMBLY__ && CONFIG_CFI macro:
 *
 * Use the __kcfi_typeid_<function> type identifier symbol to
 * annotate indirectly called assembly functions. The compiler emits
 * these symbols for all address-taken function declarations in C
 * code.
 *
 * #ifndef __CFI_TYPE
 * #define __CFI_TYPE(name) .4byte __kcfi_typeid_##name
 * #endif
 */
#[cfg(all(assembly, CONFIG_CFI))]
#[macro_export]
macro_rules! __CFI_TYPE {
    ($name:ident) => {
        ::core::arch::global_asm!(concat!(".4byte __kcfi_typeid_", stringify!($name)));
    };
}

/*
 * Original __ASSEMBLY__ && CONFIG_CFI macro:
 *
 * #define SYM_TYPED_ENTRY(name, linkage, align...) \
 *      linkage(name) ASM_NL                        \
 *      align ASM_NL                                \
 *      __CFI_TYPE(name) ASM_NL                     \
 *      name:
 */
#[cfg(all(assembly, CONFIG_CFI))]
#[macro_export]
macro_rules! SYM_TYPED_ENTRY {
    ($name:ident, $linkage:ident $(, $align:tt)*) => {
        $linkage!($name);
        $($align)*
        $crate::__CFI_TYPE!($name);
        ::core::arch::global_asm!(concat!(stringify!($name), ":"));
    };
}

/*
 * Original __ASSEMBLY__ && CONFIG_CFI macro:
 *
 * #define SYM_TYPED_START(name, linkage, align...) \
 *      SYM_TYPED_ENTRY(name, linkage, align)
 */
#[cfg(all(assembly, CONFIG_CFI))]
#[macro_export]
macro_rules! SYM_TYPED_START {
    ($name:ident, $linkage:ident $(, $align:tt)*) => {
        $crate::SYM_TYPED_ENTRY!($name, $linkage $(, $align)*);
    };
}

/*
 * Original __ASSEMBLY__ && !CONFIG_CFI macro:
 *
 * #define SYM_TYPED_START(name, linkage, align...) \
 *      SYM_START(name, linkage, align)
 */
#[cfg(all(assembly, not(CONFIG_CFI)))]
#[macro_export]
macro_rules! SYM_TYPED_START {
    ($name:ident, $linkage:ident $(, $align:tt)*) => {
        SYM_START!($name, $linkage $(, $align)*);
    };
}

/*
 * Original __ASSEMBLY__ macro, unless already provided:
 *
 * #ifndef SYM_TYPED_FUNC_START
 * #define SYM_TYPED_FUNC_START(name) \
 *      SYM_TYPED_START(name, SYM_L_GLOBAL, SYM_A_ALIGN)
 * #endif
 */
#[cfg(assembly)]
#[macro_export]
macro_rules! SYM_TYPED_FUNC_START {
    ($name:ident) => {
        $crate::SYM_TYPED_START!($name, SYM_L_GLOBAL, SYM_A_ALIGN);
    };
}

/*
 * Original !__ASSEMBLY__ && CONFIG_CFI macro:
 *
 * #define DEFINE_CFI_TYPE(name, func)                                      \
 *      /*                                                                 \
 *       * Force a reference to the function so the compiler generates      \
 *       * __kcfi_typeid_<func>.                                           \
 *       */                                                                \
 *      __ADDRESSABLE(func);                                               \
 *      /* u32 name __ro_after_init = __kcfi_typeid_<func> */              \
 *      extern u32 name;                                                   \
 *      asm (                                                              \
 *      "       .pushsection    .data..ro_after_init,\"aw\",\%progbits \n" \
 *      "       .type   " #name ",\%object                             \n" \
 *      "       .globl  " #name "                                      \n" \
 *      "       .p2align        2, 0x0                                  \n" \
 *      #name ":                                                        \n" \
 *      "       .4byte  __kcfi_typeid_" #func "                       \n" \
 *      "       .size   " #name ", 4                                  \n" \
 *      "       .popsection                                           \n" \
 *      );
 */
#[cfg(all(not(assembly), CONFIG_CFI))]
#[macro_export]
macro_rules! DEFINE_CFI_TYPE {
    ($name:ident, $func:ident) => {
        __ADDRESSABLE!($func);

        unsafe extern "C" {
            pub static mut $name: u32;
        }

        ::core::arch::global_asm!(
            concat!(
                "       .pushsection    .data..ro_after_init,\"aw\",%progbits \n",
                "       .type   ",
                stringify!($name),
                ",%object                               \n",
                "       .globl  ",
                stringify!($name),
                "                                       \n",
                "       .p2align        2, 0x0                                  \n",
                stringify!($name),
                ":                                                        \n",
                "       .4byte  __kcfi_typeid_",
                stringify!($func),
                "                        \n",
                "       .size   ",
                stringify!($name),
                ", 4                                   \n",
                "       .popsection                                            \n",
            )
        );
    };
}
