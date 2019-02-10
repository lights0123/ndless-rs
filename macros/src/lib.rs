#![recursion_limit="128"]
extern crate proc_macro;

use proc_macro::TokenStream;

use proc_macro2::Span;
use quote::quote;
use syn::parse_macro_input;
use syn::{parse, spanned::Spanned, ItemFn};

#[proc_macro_attribute]
pub fn entry(args: TokenStream, input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as ItemFn);
    // check the function signature
    let valid_signature = f.constness.is_none()
        && f.abi.is_none()
        && f.decl.inputs.is_empty()
        && f.decl.generics.params.is_empty()
        && f.decl.generics.where_clause.is_none()
        && f.decl.variadic.is_none();

    if !valid_signature {
        return parse::Error::new(
            f.span(),
            "`#[entry]` function does not meet specifications!",
        )
        .to_compile_error()
        .into();
    }

    if !args.is_empty() {
        return parse::Error::new(Span::call_site(), "This attribute accepts no arguments")
            .to_compile_error()
            .into();
    }

    let attrs = f.attrs;
    let stmts = f.block.stmts;
    let ret = f.decl.output;
    let name = f.ident;

    quote!(
        #[export_name = "main"]
        #(#attrs)*
        #[allow(clippy::not_unsafe_ptr_arg_deref)]
        pub fn #name(argc: ::ndless::cty::c_int, argv: *const *const ::ndless::cty::c_char) #ret {
			{
				let args: &[*const ::ndless::cty::c_char] = unsafe { ::core::slice::from_raw_parts(argv, argc as usize) };
				let args: ::ndless::alloc::vec::Vec<_> = args.iter().map(|str| unsafe { ::ndless::ffi::CStr::from_ptr(*str) }.to_string_lossy().into_owned()).collect();
				unsafe { ::ndless::ARGUMENTS = ::core::option::Option::Some(args) }
			}
			let argc = ();
			let argv = ();
            #(#stmts)*
        }
    )
		.into()
}
