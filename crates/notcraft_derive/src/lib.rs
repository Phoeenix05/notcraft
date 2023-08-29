use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemFn};

///
/// ```
/// #[lua_ctx]
/// fn add_two_numbers(a: i32, b: i32) -> i32 {
///     ...
/// }
/// ```
/// ```
/// fn __lua_ctx__add_two_numbers(ctx: &LuaContext) {
///     ctx.create_function(|ctx, (a, b): (i32, i32)| { 
///         // `add_two_numbers` function body
///     })
/// }
/// ```
/// 
#[proc_macro_attribute]
pub fn lua_ctx(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let original_name = &input_fn.sig.ident;
    let fn_name = format_ident!("__lua_ctx__{}", original_name);

    let expanded = quote! {
        #input_fn

        #[allow(non_snake_case)]
        fn #fn_name(ctx: rlua::Lua) {
            // ctx.create_function(|_, (params...): (param types...)| Ok(input_fn(params...)))
        }
    };
    expanded.into()
}
