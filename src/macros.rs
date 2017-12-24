// TODO: license and attribution
#[macro_export]
macro_rules! define_guid {
    ($name:ident = $d1:expr, $d2:expr, $d3:expr, $($d4:expr),*) => (
        #[allow(non_upper_case_globals)]
        const $name: $crate::GUID = $crate::GUID {
            data1: $d1,
            data2: $d2,
            data3: $d3,
            data4: [$($d4),*],
        };
    );

    (pub $name:ident = $d1:expr, $d2:expr, $d3:expr, $($d4:expr),*) => (
        #[allow(non_upper_case_globals)]
        pub const $name: $crate::GUID = $crate::GUID {
            data1: $d1,
            data2: $d2,
            data3: $d3,
            data4: [$($d4),*],
        };
    );
}

#[macro_export]
macro_rules! com_interface {
    (
        $(#[$iface_attr:meta])*
        interface $iface:ident: $base_iface:ty $(,$extra_iface:ty)* {
            iid: $iid:ident,
            vtable: $vtable:ident,
            $(
                $(#[$fn_attr:meta])*
                fn $func:ident($($i:ident: $t:ty),*) -> $rt:ty;
            )*
        }
    ) => (
        #[allow(missing_debug_implementations)]
        #[doc(hidden)]
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct $vtable {
            pub base: <$base_iface as $crate::ComInterface>::Vtable,
            $(pub $func: extern "stdcall" fn(*const $iface, $($t),*) -> $rt),*
        }

        $(#[$iface_attr])*
        #[derive(Debug)]
        #[repr(C)]
        pub struct $iface {
            vtable: *const $vtable
        }

        impl $iface {
            $($(#[$fn_attr])*
            #[allow(dead_code)]
            pub unsafe fn $func(&self, $($i: $t),*) -> $rt {
                ((*self.vtable).$func)(self $(,$i)*)
            })*
        }

        impl ::std::ops::Deref for $iface {
            type Target = $base_iface;
            fn deref(&self) -> &$base_iface {
                unsafe {
                    &*(self as *const $iface as *const $base_iface)
                }
            }
        }

        impl ::std::convert::AsRef<$iface> for $iface {
            fn as_ref(&self) -> &$iface {
                self
            }
        }

        impl ::std::convert::AsRef<$base_iface> for $iface {
            fn as_ref(&self) -> &$base_iface {
                unsafe {
                    &*(self as *const $iface as *const $base_iface)
                }
            }
        }

        $(
        impl ::std::convert::AsRef<$extra_iface> for $iface {
            fn as_ref(&self) -> &$extra_iface {
                unsafe {
                    &*(self as *const $iface as *const $extra_iface)
                }
            }
        }
        )*

        unsafe impl ::std::marker::Send for $iface {}
        unsafe impl ::std::marker::Sync for $iface {}

        unsafe impl $crate::ComInterface for $iface {
            #[doc(hidden)]
            type Vtable = $vtable;

            fn iid() -> $crate::IID { $iid }
        }
    )
}

#[macro_export]
macro_rules! offset_of {
    ($t:ty, $f:ident) => {
        unsafe { &(*(0 as *const $t)).$f as *const _ as usize }
    }
}

#[macro_export]
macro_rules! coclass {
    (
        $cls:ident {
            $(
                mod $prefix:ident in $field:ident {
                    vtable_name: $vtable_name:ident,

                    $($iface_definition:tt)*
                }
            )*
        }
    ) => {
        $(
            mod $prefix {
                use super::*;
                coclass_interface!($cls, $field, $($iface_definition)*);

                generate_vtable!($vtable_name, $($iface_definition)*);
            }
        )*
    }
}

#[macro_export]
macro_rules! generate_vtable {
    (
        $vtable_name:ident,

        interface $iface:ident {
            vtable: $vtable:ident,
            $($rest:tt)*
        }
    ) => {
        pub static $vtable_name: $vtable = coclass_vtable!(
            interface $iface {
                vtable: $vtable,
                $($rest)*
            }
        );
    }
}

#[macro_export]
macro_rules! coclass_vtable {
    (
        interface $iface:ident {
            vtable: $vtable:ident,
            interface $base:ident {
                $($base_definition:tt)*
            },
            $(
                fn $func:ident($($i:ident: $t:ty),*) -> $rt:ty;
            )*
        }
    ) => {
        $vtable {
            base: coclass_vtable!(
                interface $base {
                    $($base_definition)*
                }
            ),
            $($func: $func),*
        }
    };

    (
        interface $iface:ident {
            vtable: $vtable:ident,
            $(
                fn $func:ident($($i:ident: $t:ty),*) -> $rt:ty;
            )*
        }
    ) => {
        $vtable {
            $($func: $func),*
        }
    }
}

#[macro_export]
macro_rules! coclass_interface {
    (
        $cls:ident, $field:ident,

        interface $iface:ident {
            vtable: $vtable:ident,
            interface $base:ident {
                $($base_definition:tt)*
            },
            $($fs:tt)*
        }
    ) => {
        handle_functions!($cls, $iface, $field, $($fs)*);

        coclass_interface! {
            $cls,
            $field,

            interface $base {
                $($base_definition)*
            }
        }
    };

    (
        $cls:ident, $field:ident,

        interface $iface:ident {
            vtable: $vtable:ident,
            $($fs:tt)*
        }
    ) => {
        handle_functions!($cls, $iface, $field, $($fs)*);
    }
}

#[macro_export]
macro_rules! handle_functions {
    (
        $cls:ident,
        $iface:ident,
        $field:ident,

        $(
            fn $func:ident($($i:ident: $t:ty),*) -> $rt:ty;
        )*
    ) => {
        $(
            extern "stdcall" fn $func(this: *const $iface $(, $i: $t)*) -> $rt {
                #[cfg_attr(feature = "cargo-clippy", allow(zero_ptr))]
                let this = (this as usize - offset_of!($cls, $field)) as *const $cls;
                let this = unsafe { &*this };

                #[allow(unused_unsafe)]
                unsafe { this.$func($($i),*) }
            }
        )*
    }
}

#[macro_export]
macro_rules! query_interface {
    ($sel:ident, $iid:ident, $v:ident,
     $iface:ident => $vtable:ident
     $(
         , $extra_iface:ident => $extra_vtable:ident
     )*) => {
        if $v.is_null() {
            return $crate::E_POINTER;
        }

        let iid = &*$iid;

        if *iid == $iface::iid() {
            *$v = &$sel.$vtable as *const _ as $crate::RawComPtr;
        }
        $(
            else if *iid == $extra_iface::iid() {
                *$v = &$sel.$extra_vtable as *const _ as $crate::RawComPtr;
            }
        )*
        else {
            return $crate::E_NOINTERFACE;
        }
    }
}
