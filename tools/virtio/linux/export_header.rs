// C header guard/include directives were not present in the source.

macro_rules! EXPORT_SYMBOL_GPL {
    ($sym:ident : $ty:ty) => {
        extern "C" {
            pub static $sym: $ty;
        }
    };
}

macro_rules! EXPORT_SYMBOL {
    ($sym:ident : $ty:ty) => {
        extern "C" {
            pub static $sym: $ty;
        }
    };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
