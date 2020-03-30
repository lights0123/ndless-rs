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
	let unsafety = f.unsafety;
	let vis = f.vis;

	quote!(
        #[export_name = "main"]
        unsafe fn __ndless_start(argc: ::ndless::cty::c_int, argv: *const *const ::ndless::cty::c_char) -> ::ndless::cty::c_int {
            let args: &[*const ::ndless::cty::c_char] = unsafe { ::core::slice::from_raw_parts(argv, argc as usize) };
			::ndless::__init(args);
			::ndless::process::Termination::report(#name())
        }

        #(#attrs)*
        #vis #unsafety fn #name() #ret {
            #(#stmts)*
        }
    )
		.into()
}
