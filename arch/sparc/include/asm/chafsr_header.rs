/* SPDX-License-Identifier: GPL-2.0 */

/* Cheetah Asynchronous Fault Status register, ASI=0x4C VA<63:0>=0x0 */

/* Comments indicate which processor variants on which the bit definition
 * is valid.  Codes are:
 * ch  --> cheetah
 * ch+ --> cheetah plus
 * jp  --> jalapeno
 */

/* All bits of this register except M_SYNDROME and E_SYNDROME are
 * read, write 1 to clear.  M_SYNDROME and E_SYNDROME are read-only.
 */

/* Software bit set by linux trap handlers to indicate that the trap was
 * signalled at %tl >= 1.
 */
pub const CHAFSR_TL1: u64 = 1u64 << 63u32; /* n/a */

/* Unmapped error from system bus for prefetch queue or
 * store queue read operation
 */
pub const CHPAFSR_DTO: u64 = 1u64 << 59u32; /* ch+ */

/* Bus error from system bus for prefetch queue or store queue
 * read operation
 */
pub const CHPAFSR_DBERR: u64 = 1u64 << 58u32; /* ch+ */

/* Hardware corrected E-cache Tag ECC error */
pub const CHPAFSR_THCE: u64 = 1u64 << 57u32; /* ch+ */
/* System interface protocol error, hw timeout caused */
pub const JPAFSR_JETO: u64 = 1u64 << 57u32; /* jp */

/* SW handled correctable E-cache Tag ECC error */
pub const CHPAFSR_TSCE: u64 = 1u64 << 56u32; /* ch+ */
/* Parity error on system snoop results */
pub const JPAFSR_SCE: u64 = 1u64 << 56u32; /* jp */

/* Uncorrectable E-cache Tag ECC error */
pub const CHPAFSR_TUE: u64 = 1u64 << 55u32; /* ch+ */
/* System interface protocol error, illegal command detected */
pub const JPAFSR_JEIC: u64 = 1u64 << 55u32; /* jp */

/* Uncorrectable system bus data ECC error due to prefetch
 * or store fill request
 */
pub const CHPAFSR_DUE: u64 = 1u64 << 54u32; /* ch+ */
/* System interface protocol error, illegal ADTYPE detected */
pub const JPAFSR_JEIT: u64 = 1u64 << 54u32; /* jp */

/* Multiple errors of the same type have occurred.  This bit is set when
 * an uncorrectable error or a SW correctable error occurs and the status
 * bit to report that error is already set.  When multiple errors of
 * different types are indicated by setting multiple status bits.
 *
 * This bit is not set if multiple HW corrected errors with the same
 * status bit occur, only uncorrectable and SW correctable ones have
 * this behavior.
 *
 * This bit is not set when multiple ECC errors happen within a single
 * 64-byte system bus transaction.  Only the first ECC error in a 16-byte
 * subunit will be logged.  All errors in subsequent 16-byte subunits
 * from the same 64-byte transaction are ignored.
 */
pub const CHAFSR_ME: u64 = 1u64 << 53u32; /* ch,ch+,jp */

/* Privileged state error has occurred.  This is a capture of PSTATE.PRIV
 * at the time the error is detected.
 */
pub const CHAFSR_PRIV: u64 = 1u64 << 52u32; /* ch,ch+,jp */

/* The following bits 51 (CHAFSR_PERR) to 33 (CHAFSR_CE) are sticky error
 * bits and record the most recently detected errors.  Bits accumulate
 * errors that have been detected since the last write to clear the bit.
 */

/* System interface protocol error.  The processor asserts its' ERROR
 * pin when this event occurs and it also logs a specific cause code
 * into a JTAG scannable flop.
 */
pub const CHAFSR_PERR: u64 = 1u64 << 51u32; /* ch,ch+,jp */

/* Internal processor error.  The processor asserts its' ERROR
 * pin when this event occurs and it also logs a specific cause code
 * into a JTAG scannable flop.
 */
pub const CHAFSR_IERR: u64 = 1u64 << 50u32; /* ch,ch+,jp */

/* System request parity error on incoming address */
pub const CHAFSR_ISAP: u64 = 1u64 << 49u32; /* ch,ch+,jp */

/* HW Corrected system bus MTAG ECC error */
pub const CHAFSR_EMC: u64 = 1u64 << 48u32; /* ch,ch+ */
/* Parity error on L2 cache tag SRAM */
pub const JPAFSR_ETP: u64 = 1u64 << 48u32; /* jp */

/* Uncorrectable system bus MTAG ECC error */
pub const CHAFSR_EMU: u64 = 1u64 << 47u32; /* ch,ch+ */
/* Out of range memory error has occurred */
pub const JPAFSR_OM: u64 = 1u64 << 47u32; /* jp */

/* HW Corrected system bus data ECC error for read of interrupt vector */
pub const CHAFSR_IVC: u64 = 1u64 << 46u32; /* ch,ch+ */
/* Error due to unsupported store */
pub const JPAFSR_UMS: u64 = 1u64 << 46u32; /* jp */

/* Uncorrectable system bus data ECC error for read of interrupt vector */
pub const CHAFSR_IVU: u64 = 1u64 << 45u32; /* ch,ch+,jp */

/* Unmapped error from system bus */
pub const CHAFSR_TO: u64 = 1u64 << 44u32; /* ch,ch+,jp */

/* Bus error response from system bus */
pub const CHAFSR_BERR: u64 = 1u64 << 43u32; /* ch,ch+,jp */

/* SW Correctable E-cache ECC error for instruction fetch or data access
 * other than block load.
 */
pub const CHAFSR_UCC: u64 = 1u64 << 42u32; /* ch,ch+,jp */

/* Uncorrectable E-cache ECC error for instruction fetch or data access
 * other than block load.
 */
pub const CHAFSR_UCU: u64 = 1u64 << 41u32; /* ch,ch+,jp */

/* Copyout HW Corrected ECC error */
pub const CHAFSR_CPC: u64 = 1u64 << 40u32; /* ch,ch+,jp */

/* Copyout Uncorrectable ECC error */
pub const CHAFSR_CPU: u64 = 1u64 << 39u32; /* ch,ch+,jp */

/* HW Corrected ECC error from E-cache for writeback */
pub const CHAFSR_WDC: u64 = 1u64 << 38u32; /* ch,ch+,jp */

/* Uncorrectable ECC error from E-cache for writeback */
pub const CHAFSR_WDU: u64 = 1u64 << 37u32; /* ch,ch+,jp */

/* HW Corrected ECC error from E-cache for store merge or block load */
pub const CHAFSR_EDC: u64 = 1u64 << 36u32; /* ch,ch+,jp */

/* Uncorrectable ECC error from E-cache for store merge or block load */
pub const CHAFSR_EDU: u64 = 1u64 << 35u32; /* ch,ch+,jp */

/* Uncorrectable system bus data ECC error for read of memory or I/O */
pub const CHAFSR_UE: u64 = 1u64 << 34u32; /* ch,ch+,jp */

/* HW Corrected system bus data ECC error for read of memory or I/O */
pub const CHAFSR_CE: u64 = 1u64 << 33u32; /* ch,ch+,jp */

/* Uncorrectable ECC error from remote cache/memory */
pub const JPAFSR_RUE: u64 = 1u64 << 32u32; /* jp */

/* Correctable ECC error from remote cache/memory */
pub const JPAFSR_RCE: u64 = 1u64 << 31u32; /* jp */

/* JBUS parity error on returned read data */
pub const JPAFSR_BP: u64 = 1u64 << 30u32; /* jp */

/* JBUS parity error on data for writeback or block store */
pub const JPAFSR_WBP: u64 = 1u64 << 29u32; /* jp */

/* Foreign read to DRAM incurring correctable ECC error */
pub const JPAFSR_FRC: u64 = 1u64 << 28u32; /* jp */

/* Foreign read to DRAM incurring uncorrectable ECC error */
pub const JPAFSR_FRU: u64 = 1u64 << 27u32; /* jp */

pub const CHAFSR_ERRORS: u64 = CHAFSR_PERR | CHAFSR_IERR | CHAFSR_ISAP | CHAFSR_EMC |
    CHAFSR_EMU | CHAFSR_IVC | CHAFSR_IVU | CHAFSR_TO |
    CHAFSR_BERR | CHAFSR_UCC | CHAFSR_UCU | CHAFSR_CPC |
    CHAFSR_CPU | CHAFSR_WDC | CHAFSR_WDU | CHAFSR_EDC |
    CHAFSR_EDU | CHAFSR_UE | CHAFSR_CE;

pub const CHPAFSR_ERRORS: u64 = CHPAFSR_DTO | CHPAFSR_DBERR | CHPAFSR_THCE |
    CHPAFSR_TSCE | CHPAFSR_TUE | CHPAFSR_DUE |
    CHAFSR_PERR | CHAFSR_IERR | CHAFSR_ISAP | CHAFSR_EMC |
    CHAFSR_EMU | CHAFSR_IVC | CHAFSR_IVU | CHAFSR_TO |
    CHAFSR_BERR | CHAFSR_UCC | CHAFSR_UCU | CHAFSR_CPC |
    CHAFSR_CPU | CHAFSR_WDC | CHAFSR_WDU | CHAFSR_EDC |
    CHAFSR_EDU | CHAFSR_UE | CHAFSR_CE;

pub const JPAFSR_ERRORS: u64 = JPAFSR_JETO | JPAFSR_SCE | JPAFSR_JEIC |
    JPAFSR_JEIT | CHAFSR_PERR | CHAFSR_IERR |
    CHAFSR_ISAP | JPAFSR_ETP | JPAFSR_OM |
    JPAFSR_UMS | CHAFSR_IVU | CHAFSR_TO |
    CHAFSR_BERR | CHAFSR_UCC | CHAFSR_UCU |
    CHAFSR_CPC | CHAFSR_CPU | CHAFSR_WDC |
    CHAFSR_WDU | CHAFSR_EDC | CHAFSR_EDU |
    CHAFSR_UE | CHAFSR_CE | JPAFSR_RUE |
    JPAFSR_RCE | JPAFSR_BP | JPAFSR_WBP |
    JPAFSR_FRC | JPAFSR_FRU;

/* Active JBUS request signal when error occurred */
pub const JPAFSR_JBREQ: u64 = 0x7u64 << 24u32; /* jp */
pub const JPAFSR_JBREQ_SHIFT: u64 = 24u64;

/* L2 cache way information */
pub const JPAFSR_ETW: u64 = 0x3u64 << 22u32; /* jp */
pub const JPAFSR_ETW_SHIFT: u64 = 22u64;

/* System bus MTAG ECC syndrome.  This field captures the status of the
 * first occurrence of the highest-priority error according to the M_SYND
 * overwrite policy.  After the AFSR sticky bit, corresponding to the error
 * for which the M_SYND is reported, is cleared, the contents of the M_SYND
 * field will be unchanged by will be unfrozen for further error capture.
 */
pub const CHAFSR_M_SYNDROME: u64 = 0xfu64 << 16u32; /* ch,ch+,jp */
pub const CHAFSR_M_SYNDROME_SHIFT: u64 = 16u64;

/* Agenid Id of the foreign device causing the UE/CE errors */
pub const JPAFSR_AID: u64 = 0x1fu64 << 9u32; /* jp */
pub const JPAFSR_AID_SHIFT: u64 = 9u64;

/* System bus or E-cache data ECC syndrome.  This field captures the status
 * of the first occurrence of the highest-priority error according to the
 * E_SYND overwrite policy.  After the AFSR sticky bit, corresponding to the
 * error for which the E_SYND is reported, is cleare, the contents of the E_SYND
 * field will be unchanged but will be unfrozen for further error capture.
 */
pub const CHAFSR_E_SYNDROME: u64 = 0x1ffu64 << 0u32; /* ch,ch+,jp */
pub const CHAFSR_E_SYNDROME_SHIFT: u64 = 0u64;

/* The AFSR must be explicitly cleared by software, it is not cleared automatically
 * by a read.  Writes to bits <51:33> with bits set will clear the corresponding
 * bits in the AFSR.  Bits associated with disrupting traps must be cleared before
 * interrupts are re-enabled to prevent multiple traps for the same error.  I.e.
 * PSTATE.IE and AFSR bits control delivery of disrupting traps.
 *
 * Since there is only one AFAR, when multiple events have been logged by the
 * bits in the AFSR, at most one of these events will have its status captured
 * in the AFAR.  The highest priority of those event bits will get AFAR logging.
 * The AFAR will be unlocked and available to capture the address of another event
 * as soon as the one bit in AFSR that corresponds to the event logged in AFAR is
 * cleared.  For example, if AFSR.CE is detected, then AFSR.UE (which overwrites
 * the AFAR), and AFSR.UE is cleared by not AFSR.CE, then the AFAR will be unlocked
 * and ready for another event, even though AFSR.CE is still set.  The same rules
 * also apply to the M_SYNDROME and E_SYNDROME fields of the AFSR.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
