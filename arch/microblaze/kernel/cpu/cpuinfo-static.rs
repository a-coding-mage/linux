/*
 * Copyright (C) 2007-2009 Michal Simek <monstr@monstr.eu>
 * Copyright (C) 2007-2009 PetaLogix
 * Copyright (C) 2007 John Williams <john.williams@petalogix.com>
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/kernel.h, linux/init.h, linux/string.h, asm/cpuinfo.h, asm/pvr.h

static FAMILY_STRING: &[u8] = CONFIG_XILINX_MICROBLAZE0_FAMILY;
static CPU_VER_STRING: &[u8] = CONFIG_XILINX_MICROBLAZE0_HW_VER;

macro_rules! err_printk {
    ($x:expr) => {
        pr_err!("ERROR: Microblaze {}-different for kernel and DTS\n", $x);
    };
}

pub unsafe fn set_cpuinfo_static(ci: *mut cpuinfo, cpu: *mut device_node) {
    let mut i: u32 = 0;

    (*ci).use_instr =
        if fcpu(cpu, b"xlnx,use-barrel\0") != 0 { PVR0_USE_BARREL_MASK } else { 0 } |
        if fcpu(cpu, b"xlnx,use-msr-instr\0") != 0 { PVR2_USE_MSR_INSTR } else { 0 } |
        if fcpu(cpu, b"xlnx,use-pcmp-instr\0") != 0 { PVR2_USE_PCMP_INSTR } else { 0 } |
        if fcpu(cpu, b"xlnx,use-div\0") != 0 { PVR0_USE_DIV_MASK } else { 0 };
    if CONFIG_XILINX_MICROBLAZE0_USE_BARREL {
        i |= PVR0_USE_BARREL_MASK;
    }
    if CONFIG_XILINX_MICROBLAZE0_USE_MSR_INSTR {
        i |= PVR2_USE_MSR_INSTR;
    }
    if CONFIG_XILINX_MICROBLAZE0_USE_PCMP_INSTR {
        i |= PVR2_USE_PCMP_INSTR;
    }
    if CONFIG_XILINX_MICROBLAZE0_USE_DIV {
        i |= PVR0_USE_DIV_MASK;
    }
    if (*ci).use_instr != i {
        err_printk!("BARREL, MSR, PCMP or DIV");
    }

    (*ci).use_mult = fcpu(cpu, b"xlnx,use-hw-mul\0");
    if (*ci).use_mult != CONFIG_XILINX_MICROBLAZE0_USE_HW_MUL {
        err_printk!("HW_MUL");
    }
    (*ci).use_mult = if (*ci).use_mult > 1 {
        PVR2_USE_MUL64_MASK | PVR0_USE_HW_MUL_MASK
    } else if (*ci).use_mult == 1 {
        PVR0_USE_HW_MUL_MASK
    } else { 0 };

    (*ci).use_fpu = fcpu(cpu, b"xlnx,use-fpu\0");
    if (*ci).use_fpu != CONFIG_XILINX_MICROBLAZE0_USE_FPU {
        err_printk!("HW_FPU");
    }
    (*ci).use_fpu = if (*ci).use_fpu > 1 {
        PVR2_USE_FPU2_MASK | PVR0_USE_FPU_MASK
    } else if (*ci).use_fpu == 1 {
        PVR0_USE_FPU_MASK
    } else { 0 };

    (*ci).use_exc =
        if fcpu(cpu, b"xlnx,unaligned-exceptions\0") != 0 { PVR2_UNALIGNED_EXC_MASK } else { 0 } |
        if fcpu(cpu, b"xlnx,ill-opcode-exception\0") != 0 { PVR2_ILL_OPCODE_EXC_MASK } else { 0 } |
        if fcpu(cpu, b"xlnx,iopb-bus-exception\0") != 0 { PVR2_IOPB_BUS_EXC_MASK } else { 0 } |
        if fcpu(cpu, b"xlnx,dopb-bus-exception\0") != 0 { PVR2_DOPB_BUS_EXC_MASK } else { 0 } |
        if fcpu(cpu, b"xlnx,div-zero-exception\0") != 0 { PVR2_DIV_ZERO_EXC_MASK } else { 0 } |
        if fcpu(cpu, b"xlnx,fpu-exception\0") != 0 { PVR2_FPU_EXC_MASK } else { 0 } |
        if fcpu(cpu, b"xlnx,fsl-exception\0") != 0 { PVR2_USE_EXTEND_FSL } else { 0 };

    (*ci).use_icache = fcpu(cpu, b"xlnx,use-icache\0");
    (*ci).icache_tagbits = fcpu(cpu, b"xlnx,addr-tag-bits\0");
    (*ci).icache_write = fcpu(cpu, b"xlnx,allow-icache-wr\0");
    (*ci).icache_line_length = fcpu(cpu, b"xlnx,icache-line-len\0") << 2;
    if (*ci).icache_line_length == 0 { (*ci).icache_line_length = if fcpu(cpu, b"xlnx,icache-use-fsl\0") != 0 { 4 << 2 } else { 1 << 2 }; }
    (*ci).icache_size = fcpu(cpu, b"i-cache-size\0");
    (*ci).icache_base = fcpu(cpu, b"i-cache-baseaddr\0");
    (*ci).icache_high = fcpu(cpu, b"i-cache-highaddr\0");

    (*ci).use_dcache = fcpu(cpu, b"xlnx,use-dcache\0");
    (*ci).dcache_tagbits = fcpu(cpu, b"xlnx,dcache-addr-tag\0");
    (*ci).dcache_write = fcpu(cpu, b"xlnx,allow-dcache-wr\0");
    (*ci).dcache_line_length = fcpu(cpu, b"xlnx,dcache-line-len\0") << 2;
    if (*ci).dcache_line_length == 0 { (*ci).dcache_line_length = if fcpu(cpu, b"xlnx,dcache-use-fsl\0") != 0 { 4 << 2 } else { 1 << 2 }; }
    (*ci).dcache_size = fcpu(cpu, b"d-cache-size\0");
    (*ci).dcache_base = fcpu(cpu, b"d-cache-baseaddr\0");
    (*ci).dcache_high = fcpu(cpu, b"d-cache-highaddr\0");
    (*ci).dcache_wb = fcpu(cpu, b"xlnx,dcache-use-writeback\0");

    (*ci).use_dopb = fcpu(cpu, b"xlnx,d-opb\0");
    (*ci).use_iopb = fcpu(cpu, b"xlnx,i-opb\0");
    (*ci).use_dlmb = fcpu(cpu, b"xlnx,d-lmb\0");
    (*ci).use_ilmb = fcpu(cpu, b"xlnx,i-lmb\0");
    (*ci).num_fsl = fcpu(cpu, b"xlnx,fsl-links\0");
    (*ci).irq_edge = fcpu(cpu, b"xlnx,interrupt-is-edge\0");
    (*ci).irq_positive = fcpu(cpu, b"xlnx,edge-is-positive\0");
    (*ci).area_optimised = 0;
    (*ci).hw_debug = fcpu(cpu, b"xlnx,debug-enabled\0");
    (*ci).num_pc_brk = fcpu(cpu, b"xlnx,number-of-pc-brk\0");
    (*ci).num_rd_brk = fcpu(cpu, b"xlnx,number-of-rd-addr-brk\0");
    (*ci).num_wr_brk = fcpu(cpu, b"xlnx,number-of-wr-addr-brk\0");
    (*ci).pvr_user1 = fcpu(cpu, b"xlnx,pvr-user1\0");
    (*ci).pvr_user2 = fcpu(cpu, b"xlnx,pvr-user2\0");
    (*ci).mmu = fcpu(cpu, b"xlnx,use-mmu\0");
    (*ci).mmu_privins = fcpu(cpu, b"xlnx,mmu-privileged-instr\0");
    (*ci).endian = fcpu(cpu, b"xlnx,endianness\0");
    (*ci).ver_code = 0;
    (*ci).fpga_family_code = 0;

    /* Do various fixups based on CPU version and FPGA family strings */
    /* Resolved the CPU version code */
    for i = 0; !cpu_ver_lookup[i].s.is_null(); i += 1 {
        if strcmp(cpu_ver_lookup[i].s, CPU_VER_STRING) == 0 { (*ci).ver_code = cpu_ver_lookup[i].k; }
    }
    /* Resolved the fpga family code */
    for i = 0; !family_string_lookup[i].s.is_null(); i += 1 {
        if strcmp(family_string_lookup[i].s, FAMILY_STRING) == 0 { (*ci).fpga_family_code = family_string_lookup[i].k; }
    }
    /* FIXME - mb3 and spartan2 do not exist in PVR */
    /* This is mb3 and on a non Spartan2 */
    if (*ci).ver_code == 0x20 && (*ci).fpga_family_code != 0xf0 {
        /* Hardware Multiplier in use */
        (*ci).use_mult = 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
