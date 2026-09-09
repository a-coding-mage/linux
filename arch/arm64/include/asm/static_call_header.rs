/* SPDX-License-Identifier: GPL-2.0 */

// Architecture static-call trampoline definitions.
// The assembly is emitted into the same sections and preserves the AArch64
// trampoline layout of the C header.
#[macro_export]
macro_rules! __arch_define_static_call_tramp {
    ($name:expr, $target:expr) => {
        core::arch::global_asm!(concat!(
            "\t.pushsection .static_call.text, \"ax\"\n",
            "\t.align\t4\n",
            "\t.globl\t", $name, "\n",
            $name, ":\n",
            "\thint\t34\t/* BTI C */\n",
            "\tadrp\tx16, 1f\n",
            "\tldr\tx16, [x16, :lo12:1f]\n",
            "\tbr\tx16\n",
            "\t.type\t", $name, ", %function\n",
            "\t.size\t", $name, ", . - ", $name, "\n",
            "\t.popsection\n",
            "\t.pushsection .rodata, \"a\"\n",
            "\t.align\t3\n",
            "1:\t.quad\t", $target, "\n",
            "\t.popsection\n",
        ));
    };
}

#[macro_export]
macro_rules! arch_define_static_call_tramp {
    ($name:ident, $func:path) => {
        $crate::__arch_define_static_call_tramp!(
            stringify!($name),
            stringify!($func)
        );
    };
}

#[macro_export]
macro_rules! arch_define_static_call_null_tramp {
    ($name:ident) => {
        $crate::arch_define_static_call_tramp!($name, __static_call_return0);
    };
}

#[macro_export]
macro_rules! arch_define_static_call_ret0_tramp {
    ($name:ident) => {
        $crate::arch_define_static_call_tramp!($name, __static_call_return0);
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
