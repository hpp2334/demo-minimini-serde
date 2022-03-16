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
    let field_ty_list = fields.named.iter().map(|f| &f.ty).collect::<Vec<_>>();

    let dummy = Ident::new(&format!("__IMPL__DERIVE__DE_{}", ident), Span::call_site());

    Ok(quote! {
        const #dummy: () = {

            demo_minimini_serde::make_place!(_Place);
            struct __Map {
                #(#field_ident_list: demo_minimini_serde::PlaceStore<#field_ty_list>,)*
                __out: demo_minimini_serde::PlaceStore<#ident>
            }
            impl __Map {
                pub fn new(out: demo_minimini_serde::PlaceStore<#ident>) -> Self {
                    Self {
                        #(#field_ident_list: std::default::Default::default(),)*
                        __out: out.clone(),
                    }
                }
            }
            impl demo_minimini_serde::de::Map for __Map {
                fn key(&mut self, key: &str) -> demo_minimini_serde::Result<demo_minimini_serde::Box<dyn demo_minimini_serde::Visitor>> {
                    match key {
                        #(#field_name_list => {
                            let out = self.#field_ident_list.clone();
                            Ok(demo_minimini_serde::Deserialize::begin(out))
                        },)*
                        _ => {
                            return Err(demo_minimini_serde::Error)
                        }
                    }
                }
                fn finish(&mut self) -> Result<()> {
                    #(
                        let #field_ident_list = self.#field_ident_list.clone().try_unwrap()?;
                    )*
                    self.__out.set(#ident {
                        #(#field_ident_list),*
                    });

                    Ok(())
                }
            }

            impl demo_minimini_serde::Deserialize for #ident {
                fn begin(out: demo_minimini_serde::PlaceStore<Self>) -> Box<dyn demo_minimini_serde::Visitor> {
                    impl demo_minimini_serde::Visitor for _Place<#ident> {
                        fn map(&mut self) -> demo_minimini_serde::Result<demo_minimini_serde::Box<dyn demo_minimini_serde::de::Map>> {
                            Ok(demo_minimini_serde::Box::new(__Map::new(self.out.clone())))
                        }
                    }

                    let place = _Place { out };
                    Box::new(place)
                }
            }
        };
    })
}
