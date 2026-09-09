/*
 * Support for MicroBlaze PVR (processor version register)
 *
 * Copyright (C) 2007-2009 Michal Simek <monstr@monstr.eu>
 * Copyright (C) 2007-2009 PetaLogix
 * Copyright (C) 2007 John Williams <john.williams@petalogix.com>
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive
 * for more details.
 */

/* Dependencies are supplied by the surrounding kernel translation. */

/* Helper macro to map between fields in our struct cpuinfo, and
 * the PVR macros in pvr.h.
 */
macro_rules! ci {
    ($ci:expr, $pvr:expr, $c:ident, $f:ident) => {
        (*$ci).$c = $f($pvr);
    };
}

macro_rules! err_printk {
    ($x:literal) => {
        pr_err(concat!("ERROR: Microblaze ", $x, "-different for PVR and DTS\n").as_ptr());
    };
}

extern "C" {
    fn get_pvr(pvr: *mut pvr_s);
    fn pr_err(format: *const u8);
}

pub unsafe fn set_cpuinfo_pvr_full(ci: *mut cpuinfo, _cpu: *mut device_node) {
    let mut pvr = core::mem::MaybeUninit::<pvr_s>::uninit();
    let mut temp: u32;
    get_pvr(pvr.as_mut_ptr());
    let pvr = pvr.assume_init();

    ci! { ci, pvr, ver_code, PVR_VERSION };
    if (*ci).ver_code == 0 {
        pr_err(b"ERROR: MB has broken PVR regs -> use DTS setting\0".as_ptr());
        return;
    }

    temp = PVR_USE_BARREL(pvr) | PVR_USE_MSR_INSTR(pvr) |
        PVR_USE_PCMP_INSTR(pvr) | PVR_USE_DIV(pvr);
    if (*ci).use_instr != temp {
        err_printk!("BARREL, MSR, PCMP or DIV");
    }
    (*ci).use_instr = temp;

    temp = PVR_USE_HW_MUL(pvr) | PVR_USE_MUL64(pvr);
    if (*ci).use_mult != temp {
        err_printk!("HW_MUL");
    }
    (*ci).use_mult = temp;

    temp = PVR_USE_FPU(pvr) | PVR_USE_FPU2(pvr);
    if (*ci).use_fpu != temp {
        err_printk!("HW_FPU");
    }
    (*ci).use_fpu = temp;

    (*ci).use_exc = PVR_OPCODE_0x0_ILLEGAL(pvr) |
        PVR_UNALIGNED_EXCEPTION(pvr) |
        PVR_ILL_OPCODE_EXCEPTION(pvr) |
        PVR_IOPB_BUS_EXCEPTION(pvr) |
        PVR_DOPB_BUS_EXCEPTION(pvr) |
        PVR_DIV_ZERO_EXCEPTION(pvr) |
        PVR_FPU_EXCEPTION(pvr) |
        PVR_FSL_EXCEPTION(pvr);

    ci! { ci, pvr, pvr_user1, PVR_USER1 };
    ci! { ci, pvr, pvr_user2, PVR_USER2 };

    ci! { ci, pvr, mmu, PVR_USE_MMU };
    ci! { ci, pvr, mmu_privins, PVR_MMU_PRIVINS };
    ci! { ci, pvr, endian, PVR_ENDIAN };

    ci! { ci, pvr, use_icache, PVR_USE_ICACHE };
    ci! { ci, pvr, icache_tagbits, PVR_ICACHE_ADDR_TAG_BITS };
    ci! { ci, pvr, icache_write, PVR_ICACHE_ALLOW_WR };
    (*ci).icache_line_length = PVR_ICACHE_LINE_LEN(pvr) << 2;
    ci! { ci, pvr, icache_size, PVR_ICACHE_BYTE_SIZE };
    ci! { ci, pvr, icache_base, PVR_ICACHE_BASEADDR };
    ci! { ci, pvr, icache_high, PVR_ICACHE_HIGHADDR };

    ci! { ci, pvr, use_dcache, PVR_USE_DCACHE };
    ci! { ci, pvr, dcache_tagbits, PVR_DCACHE_ADDR_TAG_BITS };
    ci! { ci, pvr, dcache_write, PVR_DCACHE_ALLOW_WR };
    (*ci).dcache_line_length = PVR_DCACHE_LINE_LEN(pvr) << 2;
    ci! { ci, pvr, dcache_size, PVR_DCACHE_BYTE_SIZE };
    ci! { ci, pvr, dcache_base, PVR_DCACHE_BASEADDR };
    ci! { ci, pvr, dcache_high, PVR_DCACHE_HIGHADDR };

    temp = PVR_DCACHE_USE_WRITEBACK(pvr);
    if (*ci).dcache_wb != temp {
        err_printk!("DCACHE WB");
    }
    (*ci).dcache_wb = temp;

    ci! { ci, pvr, use_dopb, PVR_D_OPB };
    ci! { ci, pvr, use_iopb, PVR_I_OPB };
    ci! { ci, pvr, use_dlmb, PVR_D_LMB };
    ci! { ci, pvr, use_ilmb, PVR_I_LMB };
    ci! { ci, pvr, num_fsl, PVR_FSL_LINKS };

    ci! { ci, pvr, irq_edge, PVR_INTERRUPT_IS_EDGE };
    ci! { ci, pvr, irq_positive, PVR_EDGE_IS_POSITIVE };

    ci! { ci, pvr, area_optimised, PVR_AREA_OPTIMISED };

    ci! { ci, pvr, hw_debug, PVR_DEBUG_ENABLED };
    ci! { ci, pvr, num_pc_brk, PVR_NUMBER_OF_PC_BRK };
    ci! { ci, pvr, num_rd_brk, PVR_NUMBER_OF_RD_ADDR_BRK };
    ci! { ci, pvr, num_wr_brk, PVR_NUMBER_OF_WR_ADDR_BRK };

    ci! { ci, pvr, fpga_family_code, PVR_TARGET_FAMILY };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
