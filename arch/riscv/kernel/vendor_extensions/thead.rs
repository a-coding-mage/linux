// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the surrounding kernel translation.

/* All T-Head vendor extensions supported in Linux */
static RISCV_ISA_VENDOR_EXT_THEAD: [riscv_isa_ext_data; 1] = [
	__RISCV_ISA_EXT_DATA!(xtheadvector, RISCV_ISA_VENDOR_EXT_XTHEADVECTOR),
];

static mut RISCV_ISA_VENDOR_EXT_LIST_THEAD: riscv_isa_vendor_ext_data_list =
	riscv_isa_vendor_ext_data_list {
		ext_data_count: RISCV_ISA_VENDOR_EXT_THEAD.len(),
		ext_data: RISCV_ISA_VENDOR_EXT_THEAD.as_ptr(),
	};

pub unsafe fn disable_xtheadvector() {
	let mut cpu: i32;

	for_each_possible_cpu!(cpu, {
		clear_bit(
			RISCV_ISA_VENDOR_EXT_XTHEADVECTOR,
			(*RISCV_ISA_VENDOR_EXT_LIST_THEAD.per_hart_isa_bitmap.add(cpu as usize))
				.isa,
		);
	});

	clear_bit(
		RISCV_ISA_VENDOR_EXT_XTHEADVECTOR,
		RISCV_ISA_VENDOR_EXT_LIST_THEAD.all_harts_isa_bitmap.isa,
	);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
