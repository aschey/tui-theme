use manyhow::{Emitter, bail, manyhow};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput, Ident, spanned::Spanned};

#[manyhow(proc_macro_derive(SetTheme, attributes(variants)))]
pub fn derive_set_theme(input: DeriveInput, emitter: &mut Emitter) -> manyhow::Result {
    let struct_name = &input.ident;
    let struct_name_upper = struct_name.to_string().to_ascii_uppercase();

    let global_theme = Ident::new(
        &format!("__{struct_name_upper}__GLOBAL_THEME"),
        Span::call_site(),
    );
    let local_theme = Ident::new(
        &format!("__{struct_name_upper}__LOCAL_THEME"),
        Span::call_site(),
    );

    Ok(quote! {
        static #global_theme: LazyLock<Arc<RwLock<#struct_name>>> = LazyLock::new(Default::default);

        thread_local! {
            static #local_theme: RefCell<Option<#struct_name>> = Default::default();
        }

        impl SetTheme for #struct_name {
            type Theme = Self;

            fn set_local(&self) {
                #local_theme.with(|t| *t.borrow_mut() = Some(self.clone()));
            }

            fn set_global(&self) {
                *#global_theme.write().unwrap() = self.clone();
            }

            fn unset_local() {
                #local_theme.with(|t| *t.borrow_mut() = None);
            }

            fn current() -> Self {
                #local_theme
                    .with(|t| t.borrow().clone())
                    .unwrap_or_else(|| #global_theme.read().unwrap().clone())
            }

            fn with_theme< F, T>(f: F) -> T
            where
                F: FnOnce(&Self::Theme) -> T {
                if #local_theme.with(|t| t.borrow().is_some()) {
                    #local_theme.with(|t| f(&t.borrow().as_ref().unwrap()))
                } else {
                    f(&*#global_theme.read().unwrap())
                }
            }

        }

    })
}

#[manyhow(proc_macro_derive(ColorTheme, attributes(variants)))]
pub fn derive_color_theme(input: DeriveInput, emitter: &mut Emitter) -> manyhow::Result {
    let struct_name = &input.ident;
    if let Some(param) = input.generics.type_params().next() {
        bail!(
            param.span(),
            "Type parameters are not supported for theme structs"
        );
    }

    let style_trait = Ident::new(&(struct_name.to_string() + "Style"), Span::call_site());
    let Data::Struct(data_struct) = &input.data else {
        bail!(input.span(), "Theme can only be derived on structs");
    };

    let fields: Vec<_> = data_struct
        .fields
        .iter()
        .filter_map(|f| f.ident.as_ref())
        .collect();
    let trait_fns: TokenStream = fields
        .iter()
        .map(|f| {
            let bg_fn = Ident::new(&format!("on_{f}"), Span::call_site());
            quote! {
                fn #f(self) -> T;
                fn #bg_fn(self) -> T;
            }
        })
        .collect();

    let impl_fns: TokenStream = fields
        .iter()
        .map(|f| {
            let bg_fn = Ident::new(&format!("on_{f}"), Span::call_site());
            quote! {
                fn #f(self) -> T {
                    #struct_name::with_theme(|t| self.fg(t.#f))
                }

                fn #bg_fn(self) -> T {
                    #struct_name::with_theme(|t| self.bg(t.#f))
                }
            }
        })
        .collect();

    Ok(quote! {
        pub trait #style_trait<T> {
            #trait_fns
        }

        impl<'a, T, U> #style_trait<T> for U
        where
            U: Stylize<'a, T>,
        {
            #impl_fns
        }

    })
}

#[manyhow(proc_macro_derive(SubTheme))]
pub fn derive_sub_theme(input: DeriveInput, emitter: &mut Emitter) -> manyhow::Result {
    let struct_name = &input.ident;

    let Data::Struct(data_struct) = &input.data else {
        bail!(input.span(), "Theme can only be derived on structs");
    };

    let impl_fns: TokenStream = data_struct
        .fields
        .iter()
        .filter_map(|f| {
            if let Some(ident) = &f.ident {
                let ty = &f.ty;
                Some(quote! {
                    fn #ident() -> #ty {
                        #struct_name::with_theme(|t| t.#ident.clone())
                    }
                })
            } else {
                None
            }
        })
        .collect();

    Ok(quote! {
        impl #struct_name {
            #impl_fns
        }
    })
}

#[manyhow(proc_macro_derive(Theme))]
pub fn derive_theme(input: DeriveInput, emitter: &mut Emitter) -> manyhow::Result {
    let Data::Struct(data_struct) = &input.data else {
        bail!(input.span(), "Theme can only be derived on structs");
    };
    let struct_name = &input.ident;
    let struct_name_upper = struct_name.to_string().to_ascii_uppercase();

    let global_theme = Ident::new(
        &format!("__{struct_name_upper}__GLOBAL_THEME"),
        Span::call_site(),
    );
    let local_theme = Ident::new(
        &format!("__{struct_name_upper}__LOCAL_THEME"),
        Span::call_site(),
    );

    let fields: Vec<_> = data_struct
        .fields
        .iter()
        .filter_map(|f| f.ident.as_ref().map(|ident| (ident, &f.ty)))
        .collect();
    let set_local: TokenStream = fields
        .iter()
        .map(|(f, _)| quote!(self.#f.set_local();))
        .collect();
    let set_global: TokenStream = fields
        .iter()
        .map(|(f, _)| quote!(self.#f.set_global();))
        .collect();
    let unset_local: TokenStream = fields
        .iter()
        .map(|(_, ty)| quote!(#ty::unset_local();))
        .collect();
    let current_fields: TokenStream = fields
        .iter()
        .map(|(f, ty)| quote!(#f: #ty::current(), ))
        .collect();

    Ok(quote! {
        static #global_theme: LazyLock<Arc<RwLock<#struct_name>>> = LazyLock::new(Default::default);

        thread_local! {
            static #local_theme: RefCell<Option<#struct_name>> = Default::default();
        }

        impl SetTheme for #struct_name {
            type Theme = Self;

            fn set_local(&self) {
                #set_local
                #local_theme.with(|t| *t.borrow_mut() = Some(self.clone()));
            }

            fn set_global(&self) {
                #set_global
                *#global_theme.write().unwrap() = self.clone();
            }

            fn unset_local() {
                #unset_local
                #local_theme.with(|t| *t.borrow_mut() = None);
            }

            fn current() -> Self {
                #local_theme
                    .with(|t| t.borrow().clone())
                    .unwrap_or_else(|| #global_theme.read().unwrap().clone())
            }

            fn with_theme< F, T>(f: F) -> T
            where
                F: FnOnce(&Self::Theme) -> T {
                if #local_theme.with(|t| t.borrow().is_some()) {
                    #local_theme.with(|t| f(&t.borrow().as_ref().unwrap()))
                } else {
                    f(&*#global_theme.read().unwrap())
                }
            }

        }

    })
}
