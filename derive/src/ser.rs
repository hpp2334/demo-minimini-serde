use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Fields, FieldsNamed, Ident};

pub fn derive(input: DeriveInput) -> syn::Result<TokenStream> {
    match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => derive_struct(&input, &fields),
        _ => {
            unimplemented!()
        }
    }
}

// Not support:
// - generics
fn derive_struct(input: &DeriveInput, fields: &FieldsNamed) -> syn::Result<TokenStream> {
    let ident = &input.ident;
    let field_ident_list = fields.named.iter().map(|f| &f.ident).collect::<Vec<_>>();
    let field_name_list = fields
        .named
        .iter()
        .map(|f| f.ident.as_ref().unwrap().to_string())
        .collect::<Vec<_>>();
    let indexes = (0..field_ident_list.len()).collect::<Vec<usize>>();

    let dummy = Ident::new(&format!("__IMPL__DERIVE__SER_{}", ident), Span::call_site());

    Ok(quote! {
        const #dummy: () = {
            demo_minimini_serde::make_place!(_Place);
            struct StructMap<'a> {
                data: &'a #ident,
                cur: usize
            }
            impl<'a> demo_minimini_serde::ser::Map for StructMap<'a> {
                fn next(&mut self) -> demo_minimini_serde::Option<(String, &dyn demo_minimini_serde::Serialize)> {
                    match self.cur {
                        #(#indexes => {
                            self.cur += 1;
                            let key = #field_name_list;
                            let value = &self.data.#field_ident_list;
                            Some((key.to_owned(), value))
                        },)*
                        _ => None
                    }
                }
            }

            impl demo_minimini_serde::Serialize for #ident {
                fn begin(&self) -> demo_minimini_serde::Result<demo_minimini_serde::ser::Fragment> {
                    let map = StructMap {
                        data: self,
                        cur: 0,
                    };
                    let map = demo_minimini_serde::Box::new(map);

                    Ok(demo_minimini_serde::ser::Fragment::Map(map))
                }
            }
        };
    })
}
