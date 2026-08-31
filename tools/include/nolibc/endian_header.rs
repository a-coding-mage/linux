/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * Byte order conversion for NOLIBC
 * Copyright (C) 2026 Thomas Weißschuh <linux@weissschuh.net>
 */

/* make sure to include all global symbols */
/* C dependencies: "nolibc.h", "stdint.h", <asm/byteorder.h> */

macro_rules! htobe16 {
    ($_x:expr) => {
        __cpu_to_be16($_x)
    };
}

macro_rules! htole16 {
    ($_x:expr) => {
        __cpu_to_le16($_x)
    };
}

macro_rules! be16toh {
    ($_x:expr) => {
        __be16_to_cpu($_x)
    };
}

macro_rules! le16toh {
    ($_x:expr) => {
        __le16_to_cpu($_x)
    };
}

macro_rules! htobe32 {
    ($_x:expr) => {
        __cpu_to_be32($_x)
    };
}

macro_rules! htole32 {
    ($_x:expr) => {
        __cpu_to_le32($_x)
    };
}

macro_rules! be32toh {
    ($_x:expr) => {
        __be32_to_cpu($_x)
    };
}

macro_rules! le32toh {
    ($_x:expr) => {
        __le32_to_cpu($_x)
    };
}

macro_rules! htobe64 {
    ($_x:expr) => {
        __cpu_to_be64($_x)
    };
}

macro_rules! htole64 {
    ($_x:expr) => {
        __cpu_to_le64($_x)
    };
}

macro_rules! be64toh {
    ($_x:expr) => {
        __be64_to_cpu($_x)
    };
}

macro_rules! le64toh {
    ($_x:expr) => {
        __le64_to_cpu($_x)
    };
}
