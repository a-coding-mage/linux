/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * PROM library initialisation code.
 *
 * Copyright (C) 1999,2000,2004,2005,2012  MIPS Technologies, Inc.
 * All rights reserved.
 * Authors: Carsten Langgaard <carstenl@mips.com>
 *         Maciej W. Rozycki <macro@mips.com>
 *          Steven J. Hill <sjhill@mips.com>
 */

static mut mips_revision_corid: i32 = 0;
pub static mut mips_revision_sconid: i32 = 0;

/* Bonito64 system controller register base. */
pub static mut _pcictrl_bonito: usize = 0;
pub static mut _pcictrl_bonito_pcicfg: usize = 0;

/* GT64120 system controller register base */
pub static mut _pcictrl_gt64120: usize = 0;

/* MIPS System controller register base */
pub static mut _pcictrl_msc: usize = 0;

/* CONFIG_SERIAL_8250_CONSOLE */
unsafe fn console_config() {
    let mut console_string = [0u8; 40];
    let mut baud: i32 = 0;
    let mut parity: u8 = 0;
    let mut bits: u8 = 0;
    let mut flow: u8 = 0;
    let mut s: *mut u8;

    s = fw_getenv("modetty0");
    if !s.is_null() {
        while *s >= b'0' && *s <= b'9' {
            baud = baud.wrapping_mul(10).wrapping_add((*s - b'0') as i32);
            s = s.add(1);
        }
        if *s == b',' { s = s.add(1); }
        if *s != 0 { parity = *s; s = s.add(1); }
        if *s == b',' { s = s.add(1); }
        if *s != 0 { bits = *s; s = s.add(1); }
        if *s == b',' { s = s.add(1); }
        if *s == b'h' { flow = b'r'; }
    }
    if baud == 0 { baud = 38400; }
    if parity != b'n' && parity != b'o' && parity != b'e' { parity = b'n'; }
    if bits != b'7' && bits != b'8' { bits = b'8'; }
    if flow == 0 { flow = b'r'; }

    /* The formatting and firmware string operations are supplied externally. */
    let _ = (&mut console_string, baud, parity, bits, flow);
}

unsafe fn mips_nmi_setup() {
    let base: *mut u8 = if cpu_has_veic {
        (CAC_BASE + 0xa80) as *mut u8
    } else {
        (CAC_BASE + 0x380) as *mut u8
    };
    memcpy(base, except_vec_nmi, 0x80);
    flush_icache_range(base as usize, base as usize + 0x80);
}

unsafe fn mips_ejtag_setup() {
    let base: *mut u8 = if cpu_has_veic {
        (CAC_BASE + 0xa00) as *mut u8
    } else {
        (CAC_BASE + 0x300) as *mut u8
    };
    extern "C" {
        static mut except_vec_ejtag_debug: u8;
    }
    memcpy(base, &mut except_vec_ejtag_debug, 0x80);
    flush_icache_range(base as usize, base as usize + 0x80);
}

pub unsafe fn mips_cpc_default_phys_base() -> phys_addr_t { CPC_BASE_ADDR }

pub unsafe fn prom_init() {
    /* early setup of _pcictrl_bonito so that we can determine the system
     * controller on a CORE_EMUL board */
    _pcictrl_bonito = ioremap(BONITO_REG_BASE, BONITO_REG_SIZE) as usize;
    mips_revision_corid = MIPS_REVISION_CORID;

    if mips_revision_corid == MIPS_REVISION_CORID_CORE_EMUL {
        if BONITO_PCIDID == 0x0001df53 || BONITO_PCIDID == 0x0003df53 {
            mips_revision_corid = MIPS_REVISION_CORID_CORE_EMUL_BON;
        } else { mips_revision_corid = MIPS_REVISION_CORID_CORE_EMUL_MSC; }
    }
    mips_revision_sconid = MIPS_REVISION_SCONID;
    if mips_revision_sconid == MIPS_REVISION_SCON_OTHER {
        mips_revision_sconid = match mips_revision_corid {
            MIPS_REVISION_CORID_QED_RM5261 | MIPS_REVISION_CORID_CORE_LV |
            MIPS_REVISION_CORID_CORE_FPGA | MIPS_REVISION_CORID_CORE_FPGAR2 => MIPS_REVISION_SCON_GT64120,
            MIPS_REVISION_CORID_CORE_EMUL_BON | MIPS_REVISION_CORID_BONITO64 |
            MIPS_REVISION_CORID_CORE_20K => MIPS_REVISION_SCON_BONITO,
            MIPS_REVISION_CORID_CORE_MSC | MIPS_REVISION_CORID_CORE_FPGA2 |
            MIPS_REVISION_CORID_CORE_24K => MIPS_REVISION_SCON_SOCIT,
            _ => MIPS_REVISION_SCON_ROCIT,
        };
    }
    let mut data: u32 = 0;
    let mut mask: u32;
    match mips_revision_sconid {
        MIPS_REVISION_SCON_GT64120 => {
            _pcictrl_gt64120 = ioremap(MIPS_GT_BASE, 0x2000) as usize;
            /* CONFIG_CPU_LITTLE_ENDIAN selects the byte-swapped command value. */
            GT_WRITE(GT_PCI0_CMD_OFS, GT_PCI0_CMD_MBYTESWAP_BIT | GT_PCI0_CMD_SBYTESWAP_BIT);
            let start = GT_READ(GT_PCI0IOLD_OFS);
            let mut map = GT_READ(GT_PCI0IOREMAP_OFS);
            if start & map != 0 { map &= !start; GT_WRITE(GT_PCI0IOREMAP_OFS, map); }
            set_io_port_base(MALTA_GT_PORT_BASE);
        }
        MIPS_REVISION_SCON_BONITO => {
            _pcictrl_bonito_pcicfg = ioremap(BONITO_PCICFG_BASE, BONITO_PCICFG_SIZE) as usize;
            BONITO_PCIMEMBASECFG = BONITO_PCIMEMBASECFG &
                !(BONITO_PCIMEMBASECFG_MEMBASE0_CACHED | BONITO_PCIMEMBASECFG_MEMBASE1_CACHED);
            BONITO_BONGENCFG = BONITO_BONGENCFG | BONITO_BONGENCFG_MSTRBYTESWAP | BONITO_BONGENCFG_BYTESWAP;
            set_io_port_base(MALTA_BONITO_PORT_BASE);
        }
        MIPS_REVISION_SCON_SOCIT | MIPS_REVISION_SCON_ROCIT => {
            _pcictrl_msc = ioremap(MIPS_MSC01_PCI_REG_BASE, 0x2000) as usize;
            mb(); MSC_READ(MSC01_PCI_CFG, data); MSC_WRITE(MSC01_PCI_CFG, data & !MSC01_PCI_CFG_EN_BIT); wmb();
            MSC_WRITE(MSC01_PCI_SWAP, MSC01_PCI_SWAP_NOSWAP);
            mask = if PHYS_OFFSET != 0 { PHYS_OFFSET } else { 0xf0000000 };
            MSC_WRITE(MSC01_PCI_BAR0, mask | PCI_BASE_ADDRESS_MEM_PREFETCH);
            MSC_WRITE(MSC01_PCI_HEAD4, PHYS_OFFSET | PCI_BASE_ADDRESS_MEM_PREFETCH);
            MSC_WRITE(MSC01_PCI_P2SCMSKL, mask); MSC_WRITE(MSC01_PCI_P2SCMAPL, PHYS_OFFSET);
            if data & MSC01_PCI_CFG_MAXRTRY_MSK == MSC01_PCI_CFG_MAXRTRY_MSK {
                data = (data & !(MSC01_PCI_CFG_MAXRTRY_MSK << MSC01_PCI_CFG_MAXRTRY_SHF)) |
                    ((MSC01_PCI_CFG_MAXRTRY_MSK - 1) << MSC01_PCI_CFG_MAXRTRY_SHF);
            }
            wmb(); MSC_WRITE(MSC01_PCI_CFG, data); mb(); set_io_port_base(MALTA_MSC_PORT_BASE);
        }
        MIPS_REVISION_SCON_SOCITSC | MIPS_REVISION_SCON_SOCITSCP => {
            _pcictrl_msc = ioremap(MIPS_SOCITSC_PCI_REG_BASE, 0x2000) as usize;
            mb(); MSC_READ(MSC01_PCI_CFG, data); MSC_WRITE(MSC01_PCI_CFG, data & !MSC01_PCI_CFG_EN_BIT); wmb();
            MSC_WRITE(MSC01_PCI_SWAP, MSC01_PCI_SWAP_NOSWAP); set_io_port_base(MALTA_MSC_PORT_BASE);
        }
        _ => loop { },
    }
    board_nmi_handler_setup = Some(mips_nmi_setup);
    board_ejtag_handler_setup = Some(mips_ejtag_setup);
    fw_init_cmdline();
    fw_meminit();
    /* CONFIG_SERIAL_8250_CONSOLE: console_config(); */
    mips_cpc_probe();
    if !register_cps_smp_ops() { return; }
    if !register_vsmp_smp_ops() { return; }
    register_up_smp_ops();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
