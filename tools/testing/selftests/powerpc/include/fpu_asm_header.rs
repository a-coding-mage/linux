/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 2016, Cyril Bur, IBM Corp.
 */

// C header dependency: "basic_asm.h" supplies STACK_FRAME_MIN_SIZE and
// assembly function wrapper conventions such as FUNC_START/FUNC_END.

macro_rules! PUSH_FPU {
    ($stack_size:expr) => {
        concat!(
            "stfd\tf31,(",
            stringify!($stack_size),
            " + STACK_FRAME_MIN_SIZE)(%r1);\n",
            "stfd\tf30,(",
            stringify!($stack_size),
            " + STACK_FRAME_MIN_SIZE - 8)(%r1);\n",
            "stfd\tf29,(",
            stringify!($stack_size),
            " + STACK_FRAME_MIN_SIZE - 16)(%r1);\n",
            "stfd\tf28,(",
            stringify!($stack_size),
            " + STACK_FRAME_MIN_SIZE - 24)(%r1);\n",
            "stfd\tf27,(",
            stringify!($stack_size),
            " + STACK_FRAME_MIN_SIZE - 32)(%r1);\n",
            "stfd\tf26,(",
            stringify!($stack_size),
            " + STACK_FRAME_MIN_SIZE - 40)(%r1);\n",
            "stfd\tf25,(",
            stringify!($stack_size),
            " + STACK_FRAME_MIN_SIZE - 48)(%r1);\n",
            "stfd\tf24,(",
            stringify!($stack_size),
            " + STACK_FRAME_MIN_SIZE - 56)(%r1);\n",
            "stfd\tf23,(",
            stringify!($stack_size),
            " + STACK_FRAME_MIN_SIZE - 64)(%r1);\n",
            "stfd\tf22,(",
            stringify!($stack_size),
            " + STACK_FRAME_MIN_SIZE - 72)(%r1);\n",
            "stfd\tf21,(",
            stringify!($stack_size),
            " + STACK_FRAME_MIN_SIZE - 80)(%r1);\n",
            "stfd\tf20,(",
            stringify!($stack_size),
            " + STACK_FRAME_MIN_SIZE - 88)(%r1);\n",
            "stfd\tf19,(",
            stringify!($stack_size),
            " + STACK_FRAME_MIN_SIZE - 96)(%r1);\n",
            "stfd\tf18,(",
            stringify!($stack_size),
            " + STACK_FRAME_MIN_SIZE - 104)(%r1);\n",
            "stfd\tf17,(",
            stringify!($stack_size),
            " + STACK_FRAME_MIN_SIZE - 112)(%r1);\n",
            "stfd\tf16,(",
            stringify!($stack_size),
            " + STACK_FRAME_MIN_SIZE - 120)(%r1);\n",
            "stfd\tf15,(",
            stringify!($stack_size),
            " + STACK_FRAME_MIN_SIZE - 128)(%r1);\n",
            "stfd\tf14,(",
            stringify!($stack_size),
            " + STACK_FRAME_MIN_SIZE - 136)(%r1);"
        )
    };
}

macro_rules! POP_FPU {
    ($stack_size:expr) => {
        concat!(
            "lfd\tf31,(",
            stringify!($stack_size),
            " + STACK_FRAME_MIN_SIZE)(%r1);\n",
            "lfd\tf30,(",
            stringify!($stack_size),
            " + STACK_FRAME_MIN_SIZE - 8)(%r1);\n",
            "lfd\tf29,(",
            stringify!($stack_size),
            " + STACK_FRAME_MIN_SIZE - 16)(%r1);\n",
            "lfd\tf28,(",
            stringify!($stack_size),
            " + STACK_FRAME_MIN_SIZE - 24)(%r1);\n",
            "lfd\tf27,(",
            stringify!($stack_size),
            " + STACK_FRAME_MIN_SIZE - 32)(%r1);\n",
            "lfd\tf26,(",
            stringify!($stack_size),
            " + STACK_FRAME_MIN_SIZE - 40)(%r1);\n",
            "lfd\tf25,(",
            stringify!($stack_size),
            " + STACK_FRAME_MIN_SIZE - 48)(%r1);\n",
            "lfd\tf24,(",
            stringify!($stack_size),
            " + STACK_FRAME_MIN_SIZE - 56)(%r1);\n",
            "lfd\tf23,(",
            stringify!($stack_size),
            " + STACK_FRAME_MIN_SIZE - 64)(%r1);\n",
            "lfd\tf22,(",
            stringify!($stack_size),
            " + STACK_FRAME_MIN_SIZE - 72)(%r1);\n",
            "lfd\tf21,(",
            stringify!($stack_size),
            " + STACK_FRAME_MIN_SIZE - 80)(%r1);\n",
            "lfd\tf20,(",
            stringify!($stack_size),
            " + STACK_FRAME_MIN_SIZE - 88)(%r1);\n",
            "lfd\tf19,(",
            stringify!($stack_size),
            " + STACK_FRAME_MIN_SIZE - 96)(%r1);\n",
            "lfd\tf18,(",
            stringify!($stack_size),
            " + STACK_FRAME_MIN_SIZE - 104)(%r1);\n",
            "lfd\tf17,(",
            stringify!($stack_size),
            " + STACK_FRAME_MIN_SIZE - 112)(%r1);\n",
            "lfd\tf16,(",
            stringify!($stack_size),
            " + STACK_FRAME_MIN_SIZE - 120)(%r1);\n",
            "lfd\tf15,(",
            stringify!($stack_size),
            " + STACK_FRAME_MIN_SIZE - 128)(%r1);\n",
            "lfd\tf14,(",
            stringify!($stack_size),
            " + STACK_FRAME_MIN_SIZE - 136)(%r1);"
        )
    };
}

/*
 * Careful calling this, it will 'clobber' fpu (by design)
 * Don't call this from C
 */
core::arch::global_asm!(
    r#"
    .globl load_fpu
load_fpu:
    lfd f14,0(r3)
    lfd f15,8(r3)
    lfd f16,16(r3)
    lfd f17,24(r3)
    lfd f18,32(r3)
    lfd f19,40(r3)
    lfd f20,48(r3)
    lfd f21,56(r3)
    lfd f22,64(r3)
    lfd f23,72(r3)
    lfd f24,80(r3)
    lfd f25,88(r3)
    lfd f26,96(r3)
    lfd f27,104(r3)
    lfd f28,112(r3)
    lfd f29,120(r3)
    lfd f30,128(r3)
    lfd f31,136(r3)
    blr
"#
);
