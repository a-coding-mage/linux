// SPDX-License-Identifier: GPL-2.0
// Dependencies: linux/fs_dirent.h and linux/export.h

/*
 * fs on-disk file type to dirent file type conversion
 */
static FS_DTYPE_BY_FTYPE: [u8; FT_MAX as usize] = {
    let mut table = [0u8; FT_MAX as usize];
    table[FT_UNKNOWN as usize] = DT_UNKNOWN;
    table[FT_REG_FILE as usize] = DT_REG;
    table[FT_DIR as usize] = DT_DIR;
    table[FT_CHRDEV as usize] = DT_CHR;
    table[FT_BLKDEV as usize] = DT_BLK;
    table[FT_FIFO as usize] = DT_FIFO;
    table[FT_SOCK as usize] = DT_SOCK;
    table[FT_SYMLINK as usize] = DT_LNK;
    table
};

/**
 * fs_ftype_to_dtype() - fs on-disk file type to dirent type.
 * @filetype: The on-disk file type to convert.
 *
 * This function converts the on-disk file type value (FT_*) to the directory
 * entry type (DT_*).
 *
 * Context: Any context.
 * Return:
 * * DT_UNKNOWN        - Unknown type
 * * DT_FIFO           - FIFO
 * * DT_CHR            - Character device
 * * DT_DIR            - Directory
 * * DT_BLK            - Block device
 * * DT_REG            - Regular file
 * * DT_LNK            - Symbolic link
 * * DT_SOCK           - Local-domain socket
 */
pub fn fs_ftype_to_dtype(filetype: u32) -> u8 {
    if filetype >= FT_MAX {
        return DT_UNKNOWN;
    }

    FS_DTYPE_BY_FTYPE[filetype as usize]
}

// EXPORT_SYMBOL_GPL(fs_ftype_to_dtype);

/*
 * dirent file type to fs on-disk file type conversion
 * Values not initialized explicitly are FT_UNKNOWN (0).
 */
static FS_FTYPE_BY_DTYPE: [u8; DT_MAX as usize] = {
    let mut table = [FT_UNKNOWN; DT_MAX as usize];
    table[DT_REG as usize] = FT_REG_FILE;
    table[DT_DIR as usize] = FT_DIR;
    table[DT_LNK as usize] = FT_SYMLINK;
    table[DT_CHR as usize] = FT_CHRDEV;
    table[DT_BLK as usize] = FT_BLKDEV;
    table[DT_FIFO as usize] = FT_FIFO;
    table[DT_SOCK as usize] = FT_SOCK;
    table
};

/**
 * fs_umode_to_ftype() - file mode to on-disk file type.
 * @mode: The file mode to convert.
 *
 * This function converts the file mode value to the on-disk file type (FT_*).
 *
 * Context: Any context.
 * Return:
 * * FT_UNKNOWN        - Unknown type
 * * FT_REG_FILE       - Regular file
 * * FT_DIR            - Directory
 * * FT_CHRDEV         - Character device
 * * FT_BLKDEV         - Block device
 * * FT_FIFO           - FIFO
 * * FT_SOCK           - Local-domain socket
 * * FT_SYMLINK         - Symbolic link
 */
pub fn fs_umode_to_ftype(mode: umode_t) -> u8 {
    FS_FTYPE_BY_DTYPE[S_DT(mode) as usize]
}

// EXPORT_SYMBOL_GPL(fs_umode_to_ftype);

/**
 * fs_umode_to_dtype() - file mode to dirent file type.
 * @mode: The file mode to convert.
 *
 * This function converts the file mode value to the directory
 * entry type (DT_*).
 *
 * Context: Any context.
 * Return:
 * * DT_UNKNOWN        - Unknown type
 * * DT_FIFO           - FIFO
 * * DT_CHR            - Character device
 * * DT_DIR            - Directory
 * * DT_BLK            - Block device
 * * DT_REG            - Regular file
 * * DT_LNK            - Symbolic link
 * * DT_SOCK           - Local-domain socket
 */
pub fn fs_umode_to_dtype(mode: umode_t) -> u8 {
    fs_ftype_to_dtype(fs_umode_to_ftype(mode) as u32)
}

// EXPORT_SYMBOL_GPL(fs_umode_to_dtype);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
