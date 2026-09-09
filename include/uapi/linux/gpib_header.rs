/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/***************************************************************************
 *    copyright           : (C) 2002 by Frank Mori Hess
 **************************************************************************/

pub const GPIB_MAX_NUM_BOARDS: i32 = 16;
pub const GPIB_MAX_NUM_DESCRIPTORS: i32 = 0x1000;

pub const DCAS_NUM: i32 = 0;
pub const DTAS_NUM: i32 = 1;
pub const LACS_NUM: i32 = 2;
pub const TACS_NUM: i32 = 3;
pub const ATN_NUM: i32 = 4;
pub const CIC_NUM: i32 = 5;
pub const REM_NUM: i32 = 6;
pub const LOK_NUM: i32 = 7;
pub const CMPL_NUM: i32 = 8;
pub const EVENT_NUM: i32 = 9;
pub const SPOLL_NUM: i32 = 10;
pub const RQS_NUM: i32 = 11;
pub const SRQI_NUM: i32 = 12;
pub const END_NUM: i32 = 13;
pub const TIMO_NUM: i32 = 14;
pub const ERR_NUM: i32 = 15;

/* IBSTA status bits (returned by all functions) */
pub const DCAS: i32 = 1 << DCAS_NUM; /* device clear state */
pub const DTAS: i32 = 1 << DTAS_NUM; /* device trigger state */
pub const LACS: i32 = 1 << LACS_NUM; /* GPIB interface is addressed as Listener */
pub const TACS: i32 = 1 << TACS_NUM; /* GPIB interface is addressed as Talker */
pub const ATN: i32 = 1 << ATN_NUM; /* Attention is asserted */
pub const CIC: i32 = 1 << CIC_NUM; /* GPIB interface is Controller-in-Charge */
pub const REM: i32 = 1 << REM_NUM; /* remote state */
pub const LOK: i32 = 1 << LOK_NUM; /* lockout state */
pub const CMPL: i32 = 1 << CMPL_NUM; /* I/O is complete */
pub const EVENT: i32 = 1 << EVENT_NUM; /* DCAS, DTAS, or IFC has occurred */
pub const SPOLL: i32 = 1 << SPOLL_NUM; /* board serial polled by busmaster */
pub const RQS: i32 = 1 << RQS_NUM; /* Device requesting service */
pub const SRQI: i32 = 1 << SRQI_NUM; /* SRQ is asserted */
pub const END: i32 = 1 << END_NUM; /* EOI or EOS encountered */
pub const TIMO: i32 = 1 << TIMO_NUM; /* Time limit on I/O or wait function exceeded */
pub const ERR: i32 = 1 << ERR_NUM; /* Function call terminated on error */

pub const device_status_mask: i32 = ERR | TIMO | END | CMPL | RQS;
pub const board_status_mask: i32 = ERR | TIMO | END | CMPL | SPOLL |
    EVENT | LOK | REM | CIC | ATN | TACS | LACS | DTAS | DCAS | SRQI;

/* End-of-string (EOS) modes for use with ibeos */
pub const EOS_MASK: i32 = 0x1c00;
pub const REOS: i32 = 0x0400; /* Terminate reads on EOS */
pub const XEOS: i32 = 0x800; /* assert EOI when EOS char is sent */
pub const BIN: i32 = 0x1000; /* Do 8-bit compare on EOS */

/* GPIB Bus Control Lines bit vector */
pub const VALID_DAV: i32 = 0x01;
pub const VALID_NDAC: i32 = 0x02;
pub const VALID_NRFD: i32 = 0x04;
pub const VALID_IFC: i32 = 0x08;
pub const VALID_REN: i32 = 0x10;
pub const VALID_SRQ: i32 = 0x20;
pub const VALID_ATN: i32 = 0x40;
pub const VALID_EOI: i32 = 0x80;
pub const VALID_ALL: i32 = 0xff;
pub const BUS_DAV: i32 = 0x0100; /* DAV line status bit */
pub const BUS_NDAC: i32 = 0x0200; /* NDAC line status bit */
pub const BUS_NRFD: i32 = 0x0400; /* NRFD line status bit */
pub const BUS_IFC: i32 = 0x0800; /* IFC line status bit */
pub const BUS_REN: i32 = 0x1000; /* REN line status bit */
pub const BUS_SRQ: i32 = 0x2000; /* SRQ line status bit */
pub const BUS_ATN: i32 = 0x4000; /* ATN line status bit */
pub const BUS_EOI: i32 = 0x8000; /* EOI line status bit */

pub const PPC_DISABLE: i32 = 0x10;
pub const PPC_SENSE: i32 = 0x8; /* parallel poll sense bit */
pub const PPC_DIO_MASK: i32 = 0x7;

pub const request_service_bit: i32 = 0x40;

pub const EVENT_NONE: i32 = 0;
pub const EVENT_DEV_TRG: i32 = 1;
pub const EVENT_DEV_CLR: i32 = 2;
pub const EVENT_IFC: i32 = 3;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
