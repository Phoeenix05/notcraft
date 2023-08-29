#[macro_export]
macro_rules! lua_ctx {
    ($ident:ident) => {
        fn __lua_ctx__$ident() {}
    };
}
