// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the surrounding kernel translation.

const JUMPLABEL_ERR: &str = "ARC: jump_label: ERROR: ";

/* Halt system on fatal error to make debug easier */
macro_rules! arc_jl_fatal {
    ($($arg:tt)*) => {{
        pr_err!("{}{}", JUMPLABEL_ERR, format_args!($($arg)*));
        bug!();
    }};
}

#[inline]
unsafe fn arc_gen_nop() -> u32 {
    /* 1x 32bit NOP in middle endian */
    0x7000264a
}

/*
 * Atomic update of patched instruction is only available if this
 * instruction doesn't cross L1 cache line boundary. You can read about the
 * way we achieve this in arc/include/asm/jump_label.h
 */
#[inline]
unsafe fn instruction_align_assert(addr: *mut core::ffi::c_void, len: i32) {
    let a = addr as usize;

    if (a >> L1_CACHE_SHIFT) != ((a + len as usize - 1) >> L1_CACHE_SHIFT) {
        arc_jl_fatal!("instruction (addr %px) cross L1 cache line border", addr);
    }
}

/*
 * ARCv2 'Branch unconditionally' instruction:
 * 00000ssssssssss1SSSSSSSSSSNRtttt
 * s S[n:0] lower bits signed immediate (number is bitfield size)
 * S S[m:n+1] upper bits signed immediate (number is bitfield size)
 * t S[24:21] upper bits signed immediate (branch unconditionally far)
 * N N <.d> delay slot mode
 * R R Reserved
 */
#[inline]
unsafe fn arc_gen_branch(pc: jump_label_t, target: jump_label_t) -> u32 {
    let instruction_l: u32;
    let instruction_r: u32;
    let pcl = pc & genmask!(31, 2);
    let u_offset = target.wrapping_sub(pcl);
    let s: u32;
    let upper_s: u32;
    let t: u32;

    /*
     * Offset in 32-bit branch instruction must to fit into s25.
     * Something is terribly broken if we get such huge offset within one
     * function.
     */
    if (u_offset as i32) < -16777216 || (u_offset as i32) > 16777214 {
        arc_jl_fatal!("gen branch with offset ({}) not fit in s25", u_offset as i32);
    }

    /*
     * All instructions are aligned by 2 bytes so we should never get offset
     * here which is not 2 bytes aligned.
     */
    if u_offset & 0x1 != 0 {
        arc_jl_fatal!("gen branch with offset ({}) unaligned to 2 bytes", u_offset as i32);
    }

    s = (u_offset >> 1) & genmask!(9, 0);
    upper_s = (u_offset >> 11) & genmask!(9, 0);
    t = (u_offset >> 21) & genmask!(3, 0);

    /* 00000ssssssssss1 */
    instruction_l = (s << 1) | 0x1;
    /* SSSSSSSSSSNRtttt */
    instruction_r = (upper_s << 6) | t;

    (instruction_r << 16) | (instruction_l & genmask!(15, 0))
}

pub unsafe fn arch_jump_label_transform(entry: *mut jump_entry, type_: jump_label_type) {
    let instr_addr = (*entry).code as *mut jump_label_t;
    let instr: u32;

    instruction_align_assert(instr_addr as *mut core::ffi::c_void, JUMP_LABEL_NOP_SIZE);

    if type_ == JUMP_LABEL_JMP {
        instr = arc_gen_branch((*entry).code, (*entry).target);
    } else {
        instr = arc_gen_nop();
    }

    write_once!(instr_addr, instr);
    flush_icache_range((*entry).code, (*entry).code + JUMP_LABEL_NOP_SIZE);
}

#[cfg(CONFIG_ARC_DBG_JUMP_LABEL)]
mod debug_jump_label {
    const SELFTEST_MSG: &str = "ARC: instruction generation self-test: ";

    #[repr(C)]
    struct arc_gen_branch_testdata {
        pc: jump_label_t,
        target_address: jump_label_t,
        expected_instr: u32,
    }

    unsafe fn branch_gen_test(test: *const arc_gen_branch_testdata) -> i32 {
        let instr_got = super::arc_gen_branch((*test).pc, (*test).target_address);
        if instr_got == (*test).expected_instr {
            return 0;
        }

        pr_err!(
            "{}FAIL:\n arc_gen_branch(0x{:08x}, 0x{:08x}) != 0x{:08x}, got 0x{:08x}\n",
            SELFTEST_MSG,
            (*test).pc,
            (*test).target_address,
            (*test).expected_instr,
            instr_got
        );

        -EFAULT
    }

    /*
     * Offset field in branch instruction is not continuous. Test all
     * available offset field and sign combinations. Test data is generated
     * from real working code.
     */
    static arcgenbr_test_data: [arc_gen_branch_testdata; 12] = [
        arc_gen_branch_testdata { pc: 0x90007548, target_address: 0x90007514, expected_instr: 0xffcf07cd }, /* tiny (-52) offs */
        arc_gen_branch_testdata { pc: 0x9000c9c0, target_address: 0x9000c782, expected_instr: 0xffcf05c3 }, /* tiny (-574) offs */
        arc_gen_branch_testdata { pc: 0x9000cc1c, target_address: 0x9000c782, expected_instr: 0xffcf0367 }, /* tiny (-1178) offs */
        arc_gen_branch_testdata { pc: 0x9009dce0, target_address: 0x9009d106, expected_instr: 0xff8f0427 }, /* small (-3034) offs */
        arc_gen_branch_testdata { pc: 0x9000f5de, target_address: 0x90007d30, expected_instr: 0xfc0f0755 }, /* big  (-30892) offs */
        arc_gen_branch_testdata { pc: 0x900a2444, target_address: 0x90035f64, expected_instr: 0xc9cf0321 }, /* huge (-443616) offs */
        arc_gen_branch_testdata { pc: 0x90007514, target_address: 0x9000752c, expected_instr: 0x00000019 }, /* tiny (+24) offs */
        arc_gen_branch_testdata { pc: 0x9001a578, target_address: 0x9001a77a, expected_instr: 0x00000203 }, /* tiny (+514) offs */
        arc_gen_branch_testdata { pc: 0x90031ed8, target_address: 0x90032634, expected_instr: 0x0000075d }, /* tiny (+1884) offs */
        arc_gen_branch_testdata { pc: 0x9008c7f2, target_address: 0x9008d3f0, expected_instr: 0x00400401 }, /* small (+3072) offs */
        arc_gen_branch_testdata { pc: 0x9000bb38, target_address: 0x9003b340, expected_instr: 0x17c00009 }, /* big  (+194568) offs */
        arc_gen_branch_testdata { pc: 0x90008f44, target_address: 0x90578d80, expected_instr: 0xb7c2063d }, /* huge (+5701180) offs */
    ];

    unsafe fn instr_gen_test() -> i32 {
        let mut i = 0;
        while i < arcgenbr_test_data.len() {
            if branch_gen_test(&arcgenbr_test_data[i]) != 0 {
                return -EFAULT;
            }
            i += 1;
        }

        pr_info!("{}OK\n", SELFTEST_MSG);
        0
    }

    early_initcall!(instr_gen_test);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
