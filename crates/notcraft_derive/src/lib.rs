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

    let mut param_names = Vec::new();
    let mut param_types = Vec::new();

    input_fn.sig.inputs.iter().for_each(|param| {
        if let syn::FnArg::Typed(pat_type) = param {
            if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                param_names.push(&pat_ident.ident);
                param_types.push(&pat_type.ty);
            }
        }
    });

    let expanded = quote! {
        #input_fn

        #[allow(non_snake_case)]
        fn #fn_name() {
            // FIXME: Lifetime problems with Lua context.
            // 
            // lifetime may not live long enough.
            // has type `LuaContext<'1>`
            // returning this value requires that `'1` must outlive `'2`
            let lua_ctx = LUA_CTX.lock().unwrap();
            lua_ctx.0.context(|ctx| {
                ctx.create_function(|_, (#(#param_names,)*): (#(#param_types,)*)| Ok(#original_name(#(#param_names,)*)))
            });
        }
    };
    expanded.into()
}
