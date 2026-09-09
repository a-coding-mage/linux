/* SPDX-License-Identifier: GPL-2.0-or-later */
/* ASN.1 BER/DER/CER encoding definitions
 *
 * Copyright (C) 2012 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

/* Class */
#[repr(i32)]
pub enum asn1_class {
    ASN1_UNIV = 0,  /* Universal */
    ASN1_APPL = 1,  /* Application */
    ASN1_CONT = 2,  /* Context */
    ASN1_PRIV = 3,  /* Private */
}
pub const ASN1_CLASS_BITS: i32 = 0xc0;

#[repr(i32)]
pub enum asn1_method {
    ASN1_PRIM = 0,  /* Primitive */
    ASN1_CONS = 1,  /* Constructed */
}
pub const ASN1_CONS_BIT: i32 = 0x20;

/* Tag */
#[repr(i32)]
pub enum asn1_tag {
    ASN1_EOC = 0,       /* End Of Contents or N/A */
    ASN1_BOOL = 1,      /* Boolean */
    ASN1_INT = 2,       /* Integer */
    ASN1_BTS = 3,       /* Bit String */
    ASN1_OTS = 4,       /* Octet String */
    ASN1_NULL = 5,      /* Null */
    ASN1_OID = 6,       /* Object Identifier  */
    ASN1_ODE = 7,       /* Object Description */
    ASN1_EXT = 8,       /* External */
    ASN1_REAL = 9,      /* Real float */
    ASN1_ENUM = 10,     /* Enumerated */
    ASN1_EPDV = 11,     /* Embedded PDV */
    ASN1_UTF8STR = 12,  /* UTF8 String */
    ASN1_RELOID = 13,   /* Relative OID */
    /* 14 - Reserved */
    /* 15 - Reserved */
    ASN1_SEQ = 16,      /* Sequence and Sequence of */
    ASN1_SET = 17,      /* Set and Set of */
    ASN1_NUMSTR = 18,   /* Numerical String */
    ASN1_PRNSTR = 19,   /* Printable String */
    ASN1_TEXSTR = 20,   /* T61 String / Teletext String */
    ASN1_VIDSTR = 21,   /* Videotex String */
    ASN1_IA5STR = 22,   /* IA5 String */
    ASN1_UNITIM = 23,   /* Universal Time */
    ASN1_GENTIM = 24,   /* General Time */
    ASN1_GRASTR = 25,   /* Graphic String */
    ASN1_VISSTR = 26,   /* Visible String */
    ASN1_GENSTR = 27,   /* General String */
    ASN1_UNISTR = 28,   /* Universal String */
    ASN1_CHRSTR = 29,   /* Character String */
    ASN1_BMPSTR = 30,   /* BMP String */
    ASN1_LONG_TAG = 31, /* Long form tag */
}

pub const ASN1_INDEFINITE_LENGTH: i32 = 0x80;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
