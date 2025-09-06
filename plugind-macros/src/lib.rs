use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, ReturnType};

#[proc_macro_attribute]
pub fn plugin(_attr: TokenStream, item: TokenStream) -> TokenStream {

    let mut function = parse_macro_input!(item as ItemFn);

    if function.sig.asyncness.is_none() {
        return syn::Error::new_spanned(
            function.sig.fn_token,
            "plugin can only be applied to async functions",
        )
        .to_compile_error()
        .into();
    }

    let return_ty = match &function.sig.output {
        ReturnType::Type(_, ty) => ty,
        ReturnType::Default => {
            return syn::Error::new_spanned(
                function.sig.fn_token,
                "plugin function must have a return type",
            )
            .to_compile_error()
            .into();
        }
    };

    function.sig.asyncness = None;

    function.sig.output = ReturnType::Type(
        syn::token::RArrow::default(),
        Box::new(syn::parse_quote! { futures::future::BoxFuture<'static, #return_ty> }),
    );

    let original_block = function.block;
    function.block = Box::new(syn::parse_quote! {
        {
            Box::pin(async move #original_block)
        }
    });

    let no_mangle_attr = syn::parse_quote!(#[unsafe(no_mangle)]);
    function.attrs.push(no_mangle_attr);

    let output = quote! {
        #function
    };

    output.into()
}