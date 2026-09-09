/* SPDX-License-Identifier: GPL-2.0-only */

pub static riscv_gdb_stub_feature: &str = "PacketSize=800;qXfer:features:read+;";

static GDB_XFER_READ_TARGET: &str = "qXfer:features:read:target.xml:";

// The C CONFIG_64BIT build condition is represented as a Rust feature.
#[cfg(feature = "CONFIG_64BIT")]
static GDB_XFER_READ_CPUXML: &str = "qXfer:features:read:riscv-64bit-cpu.xml";

#[cfg(not(feature = "CONFIG_64BIT"))]
static GDB_XFER_READ_CPUXML: &str = "qXfer:features:read:riscv-32bit-cpu.xml";

#[cfg(feature = "CONFIG_64BIT")]
static RISCV_GDB_STUB_TARGET_DESC: &str =
    "l<?xml version=\"1.0\"?>"
    "<!DOCTYPE target SYSTEM \"gdb-target.dtd\">"
    "<target>"
    "<xi:include href=\"riscv-64bit-cpu.xml\"/>"
    "</target>";

#[cfg(not(feature = "CONFIG_64BIT"))]
static RISCV_GDB_STUB_TARGET_DESC: &str =
    "l<?xml version=\"1.0\"?>"
    "<!DOCTYPE target SYSTEM \"gdb-target.dtd\">"
    "<target>"
    "<xi:include href=\"riscv-32bit-cpu.xml\"/>"
    "</target>";

#[cfg(feature = "CONFIG_64BIT")]
static RISCV_GDB_STUB_CPUXML: &str = concat!(
    "l<?xml version=\"1.0\"?>",
    "<!DOCTYPE feature SYSTEM \"gdb-target.dtd\">",
    "<feature name=\"org.gnu.gdb.riscv.cpu\">",
    "<reg name=\"", DBG_REG_ZERO, "\" bitsize=\"64\" type=\"int\" regnum=\"0\"/>",
    "<reg name=\"", DBG_REG_RA, "\" bitsize=\"64\" type=\"code_ptr\"/>",
    "<reg name=\"", DBG_REG_SP, "\" bitsize=\"64\" type=\"data_ptr\"/>",
    "<reg name=\"", DBG_REG_GP, "\" bitsize=\"64\" type=\"data_ptr\"/>",
    "<reg name=\"", DBG_REG_TP, "\" bitsize=\"64\" type=\"data_ptr\"/>",
    "<reg name=\"", DBG_REG_T0, "\" bitsize=\"64\" type=\"int\"/>",
    "<reg name=\"", DBG_REG_T1, "\" bitsize=\"64\" type=\"int\"/>",
    "<reg name=\"", DBG_REG_T2, "\" bitsize=\"64\" type=\"int\"/>",
    "<reg name=\"", DBG_REG_FP, "\" bitsize=\"64\" type=\"data_ptr\"/>",
    "<reg name=\"", DBG_REG_S1, "\" bitsize=\"64\" type=\"int\"/>",
    "<reg name=\"", DBG_REG_A0, "\" bitsize=\"64\" type=\"int\"/>",
    "<reg name=\"", DBG_REG_A1, "\" bitsize=\"64\" type=\"int\"/>",
    "<reg name=\"", DBG_REG_A2, "\" bitsize=\"64\" type=\"int\"/>",
    "<reg name=\"", DBG_REG_A3, "\" bitsize=\"64\" type=\"int\"/>",
    "<reg name=\"", DBG_REG_A4, "\" bitsize=\"64\" type=\"int\"/>",
    "<reg name=\"", DBG_REG_A5, "\" bitsize=\"64\" type=\"int\"/>",
    "<reg name=\"", DBG_REG_A6, "\" bitsize=\"64\" type=\"int\"/>",
    "<reg name=\"", DBG_REG_A7, "\" bitsize=\"64\" type=\"int\"/>",
    "<reg name=\"", DBG_REG_S2, "\" bitsize=\"64\" type=\"int\"/>",
    "<reg name=\"", DBG_REG_S3, "\" bitsize=\"64\" type=\"int\"/>",
    "<reg name=\"", DBG_REG_S4, "\" bitsize=\"64\" type=\"int\"/>",
    "<reg name=\"", DBG_REG_S5, "\" bitsize=\"64\" type=\"int\"/>",
    "<reg name=\"", DBG_REG_S6, "\" bitsize=\"64\" type=\"int\"/>",
    "<reg name=\"", DBG_REG_S7, "\" bitsize=\"64\" type=\"int\"/>",
    "<reg name=\"", DBG_REG_S8, "\" bitsize=\"64\" type=\"int\"/>",
    "<reg name=\"", DBG_REG_S9, "\" bitsize=\"64\" type=\"int\"/>",
    "<reg name=\"", DBG_REG_S10, "\" bitsize=\"64\" type=\"int\"/>",
    "<reg name=\"", DBG_REG_S11, "\" bitsize=\"64\" type=\"int\"/>",
    "<reg name=\"", DBG_REG_T3, "\" bitsize=\"64\" type=\"int\"/>",
    "<reg name=\"", DBG_REG_T4, "\" bitsize=\"64\" type=\"int\"/>",
    "<reg name=\"", DBG_REG_T5, "\" bitsize=\"64\" type=\"int\"/>",
    "<reg name=\"", DBG_REG_T6, "\" bitsize=\"64\" type=\"int\"/>",
    "<reg name=\"", DBG_REG_EPC, "\" bitsize=\"64\" type=\"code_ptr\"/>",
    "<reg name=\"", DBG_REG_STATUS, "\" bitsize=\"64\" type=\"int\"/>",
    "<reg name=\"", DBG_REG_BADADDR, "\" bitsize=\"64\" type=\"int\"/>",
    "<reg name=\"", DBG_REG_CAUSE, "\" bitsize=\"64\" type=\"int\"/>",
    "</feature>"
);

#[cfg(not(feature = "CONFIG_64BIT"))]
static RISCV_GDB_STUB_CPUXML: &str = concat!(
    "l<?xml version=\"1.0\"?><!DOCTYPE feature SYSTEM \"gdb-target.dtd\"><feature name=\"org.gnu.gdb.riscv.cpu\">",
    "<reg name=\"", DBG_REG_ZERO, "\" bitsize=\"32\" type=\"int\" regnum=\"0\"/>",
    "<reg name=\"", DBG_REG_RA, "\" bitsize=\"32\" type=\"code_ptr\"/>", "<reg name=\"", DBG_REG_SP, "\" bitsize=\"32\" type=\"data_ptr\"/>",
    "<reg name=\"", DBG_REG_GP, "\" bitsize=\"32\" type=\"data_ptr\"/>", "<reg name=\"", DBG_REG_TP, "\" bitsize=\"32\" type=\"data_ptr\"/>",
    "<reg name=\"", DBG_REG_T0, "\" bitsize=\"32\" type=\"int\"/>", "<reg name=\"", DBG_REG_T1, "\" bitsize=\"32\" type=\"int\"/>", "<reg name=\"", DBG_REG_T2, "\" bitsize=\"32\" type=\"int\"/>",
    "<reg name=\"", DBG_REG_FP, "\" bitsize=\"32\" type=\"data_ptr\"/>", "<reg name=\"", DBG_REG_S1, "\" bitsize=\"32\" type=\"int\"/>",
    "<reg name=\"", DBG_REG_A0, "\" bitsize=\"32\" type=\"int\"/>", "<reg name=\"", DBG_REG_A1, "\" bitsize=\"32\" type=\"int\"/>", "<reg name=\"", DBG_REG_A2, "\" bitsize=\"32\" type=\"int\"/>", "<reg name=\"", DBG_REG_A3, "\" bitsize=\"32\" type=\"int\"/>",
    "<reg name=\"", DBG_REG_A4, "\" bitsize=\"32\" type=\"int\"/>", "<reg name=\"", DBG_REG_A5, "\" bitsize=\"32\" type=\"int\"/>", "<reg name=\"", DBG_REG_A6, "\" bitsize=\"32\" type=\"int\"/>", "<reg name=\"", DBG_REG_A7, "\" bitsize=\"32\" type=\"int\"/>",
    "<reg name=\"", DBG_REG_S2, "\" bitsize=\"32\" type=\"int\"/>", "<reg name=\"", DBG_REG_S3, "\" bitsize=\"32\" type=\"int\"/>", "<reg name=\"", DBG_REG_S4, "\" bitsize=\"32\" type=\"int\"/>", "<reg name=\"", DBG_REG_S5, "\" bitsize=\"32\" type=\"int\"/>",
    "<reg name=\"", DBG_REG_S6, "\" bitsize=\"32\" type=\"int\"/>", "<reg name=\"", DBG_REG_S7, "\" bitsize=\"32\" type=\"int\"/>", "<reg name=\"", DBG_REG_S8, "\" bitsize=\"32\" type=\"int\"/>", "<reg name=\"", DBG_REG_S9, "\" bitsize=\"32\" type=\"int\"/>",
    "<reg name=\"", DBG_REG_S10, "\" bitsize=\"32\" type=\"int\"/>", "<reg name=\"", DBG_REG_S11, "\" bitsize=\"32\" type=\"int\"/>", "<reg name=\"", DBG_REG_T3, "\" bitsize=\"32\" type=\"int\"/>", "<reg name=\"", DBG_REG_T4, "\" bitsize=\"32\" type=\"int\"/>",
    "<reg name=\"", DBG_REG_T5, "\" bitsize=\"32\" type=\"int\"/>", "<reg name=\"", DBG_REG_T6, "\" bitsize=\"32\" type=\"int\"/>", "<reg name=\"", DBG_REG_EPC, "\" bitsize=\"32\" type=\"code_ptr\"/>", "<reg name=\"", DBG_REG_STATUS, "\" bitsize=\"32\" type=\"int\"/>",
    "<reg name=\"", DBG_REG_BADADDR, "\" bitsize=\"32\" type=\"int\"/>", "<reg name=\"", DBG_REG_CAUSE, "\" bitsize=\"32\" type=\"int\"/>", "</feature>"
);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
