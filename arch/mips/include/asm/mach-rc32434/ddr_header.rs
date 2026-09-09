/*
 *  Definitions for the DDR registers
 *
 *  Copyright 2002 Ryan Holm <ryan.holmQVist@idt.com>
 *  Copyright 2008 Florian Fainelli <florian@openwrt.org>
 *
 *  This program is free software; you can redistribute  it and/or modify it
 *  under the terms of  the GNU General  Public License as published by the
 *  Free Software Foundation; either version 2 of the License, or (at your
 *  option) any later version.
 *
 *  THIS SOFTWARE IS PROVIDED ``AS IS'' AND ANY EXPRESS OR IMPLIED
 *  WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
 *  MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.
 *  IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT,
 *  INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING,
 *  BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF
 *  USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON
 *  ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 *  (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 *  THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 *  Dependency: BIT_TO_MASK is supplied by asm/mach-rc32434/rb.h.
 */

/// DDR register structure.
#[repr(C)]
pub struct ddr_ram {
    pub ddrbase: u32,
    pub ddrmask: u32,
    pub res1: u32,
    pub res2: u32,
    pub ddrc: u32,
    pub ddrabase: u32,
    pub ddramask: u32,
    pub ddramap: u32,
    pub ddrcust: u32,
    pub ddrrdc: u32,
    pub ddrspare: u32,
}

pub const DDR0_PHYS_ADDR: u32 = 0x18018000;

/* DDR banks masks */
pub const DDR_MASK: u32 = 0xffff0000;
pub const DDR0_BASE_MSK: u32 = DDR_MASK;
pub const DDR1_BASE_MSK: u32 = DDR_MASK;

/* DDR bank0 registers */
pub const RC32434_DDR0_ATA_BIT: u32 = 5;
pub const RC32434_DDR0_ATA_MSK: u32 = 0x000000E0;
pub const RC32434_DDR0_DBW_BIT: u32 = 8;
pub const RC32434_DDR0_DBW_MSK: u32 = 0x00000100;
pub const RC32434_DDR0_WR_BIT: u32 = 9;
pub const RC32434_DDR0_WR_MSK: u32 = 0x00000600;
pub const RC32434_DDR0_PS_BIT: u32 = 11;
pub const RC32434_DDR0_PS_MSK: u32 = 0x00001800;
pub const RC32434_DDR0_DTYPE_BIT: u32 = 13;
pub const RC32434_DDR0_DTYPE_MSK: u32 = 0x0000e000;
pub const RC32434_DDR0_RFC_BIT: u32 = 16;
pub const RC32434_DDR0_RFC_MSK: u32 = 0x000f0000;
pub const RC32434_DDR0_RP_BIT: u32 = 20;
pub const RC32434_DDR0_RP_MSK: u32 = 0x00300000;
pub const RC32434_DDR0_AP_BIT: u32 = 22;
pub const RC32434_DDR0_AP_MSK: u32 = 0x00400000;
pub const RC32434_DDR0_RCD_BIT: u32 = 23;
pub const RC32434_DDR0_RCD_MSK: u32 = 0x01800000;
pub const RC32434_DDR0_CL_BIT: u32 = 25;
pub const RC32434_DDR0_CL_MSK: u32 = 0x06000000;
pub const RC32434_DDR0_DBM_BIT: u32 = 27;
pub const RC32434_DDR0_DBM_MSK: u32 = 0x08000000;
pub const RC32434_DDR0_SDS_BIT: u32 = 28;
pub const RC32434_DDR0_SDS_MSK: u32 = 0x10000000;
pub const RC32434_DDR0_ATP_BIT: u32 = 29;
pub const RC32434_DDR0_ATP_MSK: u32 = 0x60000000;
pub const RC32434_DDR0_RE_BIT: u32 = 31;
pub const RC32434_DDR0_RE_MSK: u32 = 0x80000000;

/* DDR bank C registers */
pub const fn RC32434_DDRC_MSK(x: u32) -> u32 { BIT_TO_MASK(x) }
pub const RC32434_DDRC_CES_BIT: u32 = 0;
pub const RC32434_DDRC_ACE_BIT: u32 = 1;

/* Custom DDR bank registers */
pub const fn RC32434_DCST_MSK(x: u32) -> u32 { BIT_TO_MASK(x) }
pub const RC32434_DCST_CS_BIT: u32 = 0;
pub const RC32434_DCST_CS_MSK: u32 = 0x00000003;
pub const RC32434_DCST_WE_BIT: u32 = 2;
pub const RC32434_DCST_RAS_BIT: u32 = 3;
pub const RC32434_DCST_CAS_BIT: u32 = 4;
pub const RC32434_DSCT_CKE_BIT: u32 = 5;
pub const RC32434_DSCT_BA_BIT: u32 = 6;
pub const RC32434_DSCT_BA_MSK: u32 = 0x000000c0;

/* DDR QSC registers */
pub const RC32434_QSC_DM_BIT: u32 = 0;
pub const RC32434_QSC_DM_MSK: u32 = 0x00000003;
pub const RC32434_QSC_DQSBS_BIT: u32 = 2;
pub const RC32434_QSC_DQSBS_MSK: u32 = 0x000000fc;
pub const RC32434_QSC_DB_BIT: u32 = 8;
pub const RC32434_QSC_DB_MSK: u32 = 0x00000100;
pub const RC32434_QSC_DBSP_BIT: u32 = 9;
pub const RC32434_QSC_DBSP_MSK: u32 = 0x01fffe00;
pub const RC32434_QSC_BDP_BIT: u32 = 25;
pub const RC32434_QSC_BDP_MSK: u32 = 0x7e000000;

/* DDR LLC registers */
pub const RC32434_LLC_EAO_BIT: u32 = 0;
pub const RC32434_LLC_EAO_MSK: u32 = 0x00000001;
pub const RC32434_LLC_EO_BIT: u32 = 1;
pub const RC32434_LLC_EO_MSK: u32 = 0x0000003e;
pub const RC32434_LLC_FS_BIT: u32 = 6;
pub const RC32434_LLC_FS_MSK: u32 = 0x000000c0;
pub const RC32434_LLC_AS_BIT: u32 = 8;
pub const RC32434_LLC_AS_MSK: u32 = 0x00000700;
pub const RC32434_LLC_SP_BIT: u32 = 11;
pub const RC32434_LLC_SP_MSK: u32 = 0x001ff800;

/* DDR LLFC registers */
pub const fn RC32434_LLFC_MSK(x: u32) -> u32 { BIT_TO_MASK(x) }
pub const RC32434_LLFC_MEN_BIT: u32 = 0;
pub const RC32434_LLFC_EAN_BIT: u32 = 1;
pub const RC32434_LLFC_FF_BIT: u32 = 2;

/* DDR DLLTA registers */
pub const RC32434_DLLTA_ADDR_BIT: u32 = 2;
pub const RC32434_DLLTA_ADDR_MSK: u32 = 0xfffffffc;

/* DDR DLLED registers */
pub const fn RC32434_DLLED_MSK(x: u32) -> u32 { BIT_TO_MASK(x) }
pub const RC32434_DLLED_DBE_BIT: u32 = 0;
pub const RC32434_DLLED_DTE_BIT: u32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
