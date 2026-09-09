/* SPDX-License-Identifier: GPL-2.0 */

/*
 * The C header guard is intentionally omitted; Rust items are module-scoped.
 * The following macro preserves the PowerPC trampoline emitted by __PPC_SCT.
 */
macro_rules! __PPC_SCT {
	($name:ident, $inst:literal) => {
		core::arch::global_asm!(concat!(
			".pushsection .text, \"ax\"\n",
			".align 5\n",
			".globl ", stringify!($name), "\n",
			stringify!($name), ":\n",
			$inst, "\n",
			"\tlis\t12,2f@ha\n",
			"\tlwz\t12,2f@l(12)\n",
			"\tmtctr\t12\n",
			"\tbctr\n",
			"1:\tli\t3, 0\n",
			"\tblr\n",
			"2:\t.long 0\n",
			".type ", stringify!($name), ", @function\n",
			".size ", stringify!($name), ", . - ", stringify!($name), "\n",
			".popsection\n",
		));
	};
}

pub const PPC_SCT_RET0: usize = 20; // Offset of label 1
pub const PPC_SCT_DATA: usize = 28; // Offset of label 2

macro_rules! ARCH_DEFINE_STATIC_CALL_TRAMP {
	($name:ident, $func:ident) => {
		__PPC_SCT!($name, concat!("b ", stringify!($func)));
	};
}

macro_rules! ARCH_DEFINE_STATIC_CALL_NULL_TRAMP {
	($name:ident) => {
		__PPC_SCT!($name, "blr");
	};
}

macro_rules! ARCH_DEFINE_STATIC_CALL_RET0_TRAMP {
	($name:ident) => {
		__PPC_SCT!($name, "b .+20");
	};
}

pub const CALL_INSN_SIZE: usize = 4;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
