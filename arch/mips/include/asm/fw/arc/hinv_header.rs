/* SPDX-License-Identifier: GPL-2.0 */
/*
 * ARCS hardware/memory inventory/configuration and system ID definitions.
 * Translated from the C header; external types are supplied by dependencies.
 */

/* configuration query defines */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CONFIGCLASS {
    SystemClass,
    ProcessorClass,
    CacheClass,
    #[cfg(not(feature = "_NT_PROM"))]
    MemoryClass,
    AdapterClass,
    ControllerClass,
    PeripheralClass,
    #[cfg(feature = "_NT_PROM")]
    MemoryClass,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CONFIGTYPE {
    ARC,
    CPU,
    FPU,
    PrimaryICache,
    PrimaryDCache,
    SecondaryICache,
    SecondaryDCache,
    SecondaryCache,
    #[cfg(not(feature = "_NT_PROM"))]
    Memory,
    EISAAdapter,
    TCAdapter,
    SCSIAdapter,
    DTIAdapter,
    MultiFunctionAdapter,
    DiskController,
    TapeController,
    CDROMController,
    WORMController,
    SerialController,
    NetworkController,
    DisplayController,
    ParallelController,
    PointerController,
    KeyboardController,
    AudioController,
    OtherController,
    DiskPeripheral,
    FloppyDiskPeripheral,
    TapePeripheral,
    ModemPeripheral,
    MonitorPeripheral,
    PrinterPeripheral,
    PointerPeripheral,
    KeyboardPeripheral,
    TerminalPeripheral,
    LinePeripheral,
    NetworkPeripheral,
    #[cfg(feature = "_NT_PROM")]
    Memory,
    OtherPeripheral,
    /* new stuff for IP30; added without moving anything except ANONYMOUS. */
    XTalkAdapter,
    PCIAdapter,
    GIOAdapter,
    TPUAdapter,
    Anonymous,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IDENTIFIERFLAG {
    Failed = 1,
    ReadOnly = 2,
    Removable = 4,
    ConsoleIn = 8,
    ConsoleOut = 16,
    Input = 32,
    Output = 64,
}

pub const NULL: usize = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub union key_u {
    pub cache: key_u_cache,
    pub FullKey: ULONG,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct key_u_cache {
    #[cfg(target_endian = "big")]
    pub c_bsize: u8,
    #[cfg(target_endian = "big")]
    pub c_lsize: u8,
    #[cfg(target_endian = "big")]
    pub c_size: u16,
    #[cfg(target_endian = "little")]
    pub c_size: u16,
    #[cfg(target_endian = "little")]
    pub c_lsize: u8,
    #[cfg(target_endian = "little")]
    pub c_bsize: u8,
}

#[cfg(target_pointer_width = "64")]
pub const SGI_ARCS_VERS: i32 = 64;
#[cfg(target_pointer_width = "64")]
pub const SGI_ARCS_REV: i32 = 0;
#[cfg(not(target_pointer_width = "64"))]
pub const SGI_ARCS_VERS: i32 = 1;
#[cfg(not(target_pointer_width = "64"))]
pub const SGI_ARCS_REV: i32 = 10;

#[repr(C)]
pub struct COMPONENT {
    pub Class: CONFIGCLASS,
    pub Type: CONFIGTYPE,
    pub Flags: IDENTIFIERFLAG,
    pub Version: USHORT,
    pub Revision: USHORT,
    pub Key: ULONG,
    pub AffinityMask: ULONG,
    pub ConfigurationDataSize: ULONG,
    pub IdentifierLength: ULONG,
    pub Identifier: *mut core::ffi::c_char,
}

/* internal structure that holds pathname parsing data */
#[repr(C)]
pub struct cfgdata {
    pub name: *mut core::ffi::c_char,
    pub minlen: i32,
    pub r#type: CONFIGTYPE,
}

/* System ID */
#[repr(C)]
pub struct SYSTEMID {
    pub VendorId: [CHAR; 8],
    pub ProductId: [CHAR; 8],
}

/* memory query functions */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MEMORYTYPE {
    ExceptionBlock,
    SPBPage,
    #[cfg(not(feature = "_NT_PROM"))]
    FreeContiguous,
    FreeMemory,
    BadMemory,
    LoadedProgram,
    FirmwareTemporary,
    FirmwarePermanent,
    #[cfg(feature = "_NT_PROM")]
    FreeContiguous,
}

#[repr(C)]
pub struct MEMORYDESCRIPTOR {
    pub Type: MEMORYTYPE,
    pub BasePage: LONG,
    pub PageCount: LONG,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
