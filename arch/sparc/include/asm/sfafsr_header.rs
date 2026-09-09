/* SPDX-License-Identifier: GPL-2.0 */

/* Spitfire Asynchronous Fault Status register, ASI=0x4C VA<63:0>=0x0 */

pub const SFAFSR_ME: u64 = 1u64 << SFAFSR_ME_SHIFT;
pub const SFAFSR_ME_SHIFT: u32 = 32;
pub const SFAFSR_PRIV: u64 = 1u64 << SFAFSR_PRIV_SHIFT;
pub const SFAFSR_PRIV_SHIFT: u32 = 31;
pub const SFAFSR_ISAP: u64 = 1u64 << SFAFSR_ISAP_SHIFT;
pub const SFAFSR_ISAP_SHIFT: u32 = 30;
pub const SFAFSR_ETP: u64 = 1u64 << SFAFSR_ETP_SHIFT;
pub const SFAFSR_ETP_SHIFT: u32 = 29;
pub const SFAFSR_IVUE: u64 = 1u64 << SFAFSR_IVUE_SHIFT;
pub const SFAFSR_IVUE_SHIFT: u32 = 28;
pub const SFAFSR_TO: u64 = 1u64 << SFAFSR_TO_SHIFT;
pub const SFAFSR_TO_SHIFT: u32 = 27;
pub const SFAFSR_BERR: u64 = 1u64 << SFAFSR_BERR_SHIFT;
pub const SFAFSR_BERR_SHIFT: u32 = 26;
pub const SFAFSR_LDP: u64 = 1u64 << SFAFSR_LDP_SHIFT;
pub const SFAFSR_LDP_SHIFT: u32 = 25;
pub const SFAFSR_CP: u64 = 1u64 << SFAFSR_CP_SHIFT;
pub const SFAFSR_CP_SHIFT: u32 = 24;
pub const SFAFSR_WP: u64 = 1u64 << SFAFSR_WP_SHIFT;
pub const SFAFSR_WP_SHIFT: u32 = 23;
pub const SFAFSR_EDP: u64 = 1u64 << SFAFSR_EDP_SHIFT;
pub const SFAFSR_EDP_SHIFT: u32 = 22;
pub const SFAFSR_UE: u64 = 1u64 << SFAFSR_UE_SHIFT;
pub const SFAFSR_UE_SHIFT: u32 = 21;
pub const SFAFSR_CE: u64 = 1u64 << SFAFSR_CE_SHIFT;
pub const SFAFSR_CE_SHIFT: u32 = 20;
pub const SFAFSR_ETS: u64 = 0xfu64 << SFAFSR_ETS_SHIFT;
pub const SFAFSR_ETS_SHIFT: u32 = 16;
pub const SFAFSR_PSYND: u64 = 0xffffu64 << SFAFSR_PSYND_SHIFT;
pub const SFAFSR_PSYND_SHIFT: u32 = 0;

/* UDB Error Register, ASI=0x7f VA<63:0>=0x0(High),0x18(Low) for read
 *                     ASI=0x77 VA<63:0>=0x0(High),0x18(Low) for write
 */
pub const UDBE_UE: u64 = 1u64 << 9;
pub const UDBE_CE: u64 = 1u64 << 8;
pub const UDBE_E_SYNDR: u64 = 0xffu64 << 0;

/* The trap handlers for asynchronous errors encode the AFSR and
 * other pieces of information into a 64-bit argument for C code
 * encoded as follows:
 *
 * -----------------------------------------------
 * |  UDB_H  |  UDB_L  | TL>1  |  TT  |   AFSR   |
 * -----------------------------------------------
 *  63     54 53     44    42   41  33 32       0
 *
 * The AFAR is passed in unchanged.
 */
pub const SFSTAT_UDBH_MASK: u64 = 0x3ffu64 << SFSTAT_UDBH_SHIFT;
pub const SFSTAT_UDBH_SHIFT: u32 = 54;
pub const SFSTAT_UDBL_MASK: u64 = 0x3ffu64 << SFSTAT_UDBH_SHIFT;
pub const SFSTAT_UDBL_SHIFT: u32 = 44;
pub const SFSTAT_TL_GT_ONE: u64 = 1u64 << SFSTAT_TL_GT_ONE_SHIFT;
pub const SFSTAT_TL_GT_ONE_SHIFT: u32 = 42;
pub const SFSTAT_TRAP_TYPE: u64 = 0x1ffu64 << SFSTAT_TRAP_TYPE_SHIFT;
pub const SFSTAT_TRAP_TYPE_SHIFT: u32 = 33;
pub const SFSTAT_AFSR_MASK: u64 = 0x1ffffffffu64 << SFSTAT_AFSR_SHIFT;
pub const SFSTAT_AFSR_SHIFT: u32 = 0;

/* ESTATE Error Enable Register, ASI=0x4b VA<63:0>=0x0 */
pub const ESTATE_ERR_CE: u64 = 0x1; /* Correctable errors                    */
pub const ESTATE_ERR_NCE: u64 = 0x2; /* TO, BERR, LDP, ETP, EDP, WP, UE, IVUE */
pub const ESTATE_ERR_ISAP: u64 = 0x4; /* System address parity error           */
pub const ESTATE_ERR_ALL: u64 = ESTATE_ERR_CE | ESTATE_ERR_NCE | ESTATE_ERR_ISAP;

/* The various trap types that report using the above state. */
pub const TRAP_TYPE_IAE: u64 = 0x09; /* Instruction Access Error             */
pub const TRAP_TYPE_DAE: u64 = 0x32; /* Data Access Error                    */
pub const TRAP_TYPE_CEE: u64 = 0x63; /* Correctable ECC Error                */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
