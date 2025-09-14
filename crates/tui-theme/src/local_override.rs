#[macro_export]
macro_rules! local_override {
    ($struct_name:ident, $global_name:ident, $local_name:ident) => {
        static $global_name: ::std::sync::LazyLock<
            ::std::sync::Arc<::std::sync::RwLock<$struct_name>>,
        > = ::std::sync::LazyLock::new(Default::default);

        thread_local! {
            static $local_name: ::std::cell::RefCell<Option<$struct_name>> = Default::default();
        }

        #[allow(dead_code)]
        impl $struct_name {
            fn override_set_local(&self) {
                $local_name.with(|t| *t.borrow_mut() = Some(self.clone()));
            }

            fn override_set_global(&self) {
                *$global_name.write().unwrap() = self.clone();
            }

            fn override_unset_local() {
                $local_name.with(|t| *t.borrow_mut() = None);
            }

            fn override_current() -> Self {
                $local_name
                    .with(|t| t.borrow().clone())
                    .unwrap_or_else(|| $global_name.read().unwrap().clone())
            }

            fn override_with_value<F, T>(f: F) -> T
            where
                F: FnOnce(&$struct_name) -> T,
            {
                if $local_name.with(|t| t.borrow().is_some()) {
                    $local_name.with(|t| f(&t.borrow().as_ref().unwrap()))
                } else {
                    f(&*$global_name.read().unwrap())
                }
            }
        }
    };
}
