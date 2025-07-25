use manyhow::{Emitter, bail, manyhow};
use proc_macro_crate::{FoundCrate, crate_name};
use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Fields, Ident, Type};

fn style_fields(fields: &Fields) -> Vec<&Ident> {
    fields
        .iter()
        .filter_map(|f| {
            let ty = f.ty.to_token_stream().to_string().replace(" ", "");
            if matches!(
                ty.as_str(),
                "Style" | "tui_theme::Style" | "ratatui::style::Style"
            ) {
                f.ident.as_ref()
            } else {
                None
            }
        })
        .collect()
}

fn color_fields(fields: &Fields) -> Vec<&Ident> {
    fields
        .iter()
        .filter_map(|f| {
            let ty = f.ty.to_token_stream().to_string().replace(" ", "");
            if matches!(
                ty.as_str(),
                "Color" | "tui_theme::Color" | "ratatui::style::Color"
            ) {
                f.ident.as_ref()
            } else {
                None
            }
        })
        .collect()
}

fn subtheme_fields(fields: &Fields) -> Vec<(&Ident, &Type)> {
    fields
        .iter()
        .filter_map(|f| {
            if f.attrs.iter().any(|a| a.meta.path().is_ident("subtheme")) {
                f.ident.as_ref().map(|i| (i, &f.ty))
            } else {
                None
            }
        })
        .collect()
}

#[manyhow(proc_macro_derive(Theme, attributes(subtheme, variants)))]
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

    let fields = subtheme_fields(&data_struct.fields);
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

    let style_trait = Ident::new(&(struct_name.to_string() + "Style"), Span::call_site());
    let style_ext_trait = Ident::new(&(struct_name.to_string() + "StyleExt"), Span::call_site());
    let color_trait = Ident::new(&(struct_name.to_string() + "ColorTheme"), Span::call_site());
    let color_ext_trait = Ident::new(&(struct_name.to_string() + "ColorExt"), Span::call_site());

    let Data::Struct(data_struct) = &input.data else {
        bail!(input.span(), "Theme can only be derived on structs");
    };

    let style_fields = style_fields(&data_struct.fields);
    let color_fields = color_fields(&data_struct.fields);

    let style_trait_fns: TokenStream = style_fields
        .iter()
        .map(|f| {
            let style_fn = Ident::new(&format!("style_{f}"), Span::call_site());
            quote! {
                fn #style_fn(self) -> T;
            }
        })
        .collect();
    let tui_theme = get_import("tui-theme");
    let ratatui = get_import("ratatui");

    let style_impl_fns: TokenStream = style_fields
        .iter()
        .map(|f| {
            let style_fn = Ident::new(&format!("style_{f}"), Span::call_site());
            quote! {
                fn #style_fn(self) -> T {
                    use #tui_theme::SetTheme;
                    #struct_name::with_theme(|t| self.set_style(t.#f))
                }
            }
        })
        .collect();

    let color_trait_fns: TokenStream = color_fields
        .iter()
        .map(|f| {
            let fg_fn = Ident::new(&format!("fg_{f}"), Span::call_site());
            let bg_fn = Ident::new(&format!("bg_{f}"), Span::call_site());
            let underline_fn = Ident::new(&format!("underline_{f}"), Span::call_site());

            quote! {
                fn #fg_fn(self) -> T;
                fn #bg_fn(self) -> T;
                fn #underline_fn(self) -> T;
            }
        })
        .collect();

    let color_extension_fns: TokenStream = color_fields
        .iter()
        .map(|f| {
            let color_fn = Ident::new(&f.to_string(), Span::call_site());

            quote! {
                fn #color_fn() -> Self;
            }
        })
        .collect();

    let style_ext_fns: TokenStream = style_fields
        .iter()
        .map(|f| {
            let style_fn = Ident::new(&f.to_string(), Span::call_site());

            quote! {
                fn #style_fn() -> Self;
            }
        })
        .collect();

    let impl_fns: TokenStream = data_struct
        .fields
        .iter()
        .filter_map(|f| {
            if let Some(ident) = &f.ident {
                let ty = &f.ty;
                Some(quote! {
                    fn #ident() -> #ty {
                        use #tui_theme::SetTheme;
                        #struct_name::with_theme(|t| t.#ident.clone())
                    }
                })
            } else {
                None
            }
        })
        .collect();

    let color_impl_fns: TokenStream = color_fields
        .iter()
        .map(|f| {
            let fg_fn = Ident::new(&format!("fg_{f}"), Span::call_site());
            let bg_fn = Ident::new(&format!("bg_{f}"), Span::call_site());
            let underline_fn = Ident::new(&format!("underline_{f}"), Span::call_site());

            quote! {
                fn #fg_fn(self) -> T {
                    use #tui_theme::SetTheme;
                    #struct_name::with_theme(|t| self.fg(t.#f))
                }

                fn #bg_fn(self) -> T {
                    use #tui_theme::SetTheme;
                    #struct_name::with_theme(|t| self.bg(t.#f))
                }

                fn #underline_fn(self) -> T {
                    use #tui_theme::SetTheme;
                    #struct_name::with_theme(|t| self.underline_color(t.#f))
                }

            }
        })
        .collect();

    let color_ext_impl_fns: TokenStream = color_fields
        .iter()
        .map(|f| {
            let color_fn = Ident::new(&f.to_string(), Span::call_site());

            quote! {
                fn #color_fn() -> Self {
                    use #tui_theme::SetTheme;
                    #struct_name::with_theme(|t| t.#f.into())
                }
            }
        })
        .collect();

    let style_ext_impl_fns: TokenStream = style_fields
        .iter()
        .map(|f| {
            let style_fn = Ident::new(&f.to_string(), Span::call_site());

            quote! {
                fn #style_fn() -> Self {
                    use #tui_theme::SetTheme;
                    #struct_name::with_theme(|t| t.#f.into())
                }
            }
        })
        .collect();

    Ok(quote! {
        static #global_theme: ::std::sync::LazyLock<
            ::std::sync::Arc<::std::sync::RwLock<#struct_name>>
        > =
            ::std::sync::LazyLock::new(Default::default);

        thread_local! {
            static #local_theme: ::std::cell::RefCell<Option<#struct_name>> = Default::default();
        }

        impl #tui_theme::SetTheme for #struct_name {
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

        impl #struct_name {
            #impl_fns
        }

        pub trait #style_trait<T> {
            #style_trait_fns
        }

        impl<T, U> #style_trait<T> for U
        where
            U: #tui_theme::Styled<Item = T>
        {
            #style_impl_fns
        }

        pub trait #color_trait<T> {
            #color_trait_fns
        }

        impl<'a, T, U> #color_trait<T> for U
        where
            U: #tui_theme::Stylize<'a, T>,
        {
            #color_impl_fns
        }

        pub trait #color_ext_trait {
            #color_extension_fns
        }

        impl #color_ext_trait for #tui_theme::Color {
            #color_ext_impl_fns
        }

        impl #color_ext_trait for #ratatui::style::Color {
            #color_ext_impl_fns
        }

        pub trait #style_ext_trait {
            #style_ext_fns
        }

        impl #style_ext_trait for #tui_theme::Style {
            #style_ext_impl_fns
        }

        impl #style_ext_trait for #ratatui::style::Style {
            #style_ext_impl_fns
        }

    })
}

fn get_import(crate_name_str: &str) -> TokenStream {
    let found_crate =
        crate_name(crate_name_str).unwrap_or_else(|_| panic!("{crate_name_str} not found"));
    match found_crate {
        FoundCrate::Itself => {
            let ident = Ident::new(&crate_name_str.replace("-", "_"), Span::call_site());
            quote!(#ident)
        }
        FoundCrate::Name(name) => {
            let ident = Ident::new(&name, Span::call_site());
            quote!(#ident)
        }
    }
}
