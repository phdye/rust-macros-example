extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn::Data::Struct;
use syn::Fields;
use syn::Type::Path;

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let data = &ast.data;
    // println!("{:#?}", data);
    let mut defenition = format!("Struct {}", name);

    if let Struct(def) = data {
        if let Fields::Named(fields) = &def.fields {
            for named in &fields.named {
                let ident = &named.ident;
                let ty = &named.ty;
                if let Some(id) = ident {
                    // println!("{}", id);
                    defenition = format!("{}\n  {}:", defenition, id)
                };
                if let Path(path) = ty {
                    // println!("{}", path.path.segments[0].ident);
                    defenition = format!("{} {}", defenition, path.path.segments[0].ident)
                }
            }
        }
    };

    let gen = quote! {
        impl HelloMacro for #name {
            fn helpify() {
                println!(#defenition);
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_hello_macro(&ast)
}
