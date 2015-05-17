// TODO: Quite messy macro stuff, needs documenting/tidying up

// Define the iterator struct and constructor from raw data.
// Same for all data types.
macro_rules! define_iter {
    ($(#[$iter_attr:meta])* struct $name:ident -> $raw:ty) => (
        $(#[$iter_attr])*
        pub struct $name<'a> {
            ptr: $raw,
            idx: isize,
            len: usize,
            _mk: ::std::marker::PhantomData<&'a ()>
        }

        #[doc(hidden)]
        impl<'a> $name<'a> {
            pub fn new(ptr: $raw, len: usize) -> $name<'a> {
                $name { ptr: ptr, idx: 0, len: len, _mk: ::std::marker::PhantomData }
            }
        }
    )
}

macro_rules! impl_iterator {
    ($name:ident, $item:ident) => (
        impl<'a> Iterator for $name<'a> {
            type Item = $item<'a>;
            fn next(&mut self) -> Option<$item<'a>> {
                if self.idx < self.len as isize {
                    let item = $item::from_raw(unsafe { self.ptr.offset(self.idx) });
                    self.idx = self.idx + 1;
                    Some(item)
                } else {
                    None
                }
            }
        }

        impl<'a> ExactSizeIterator for $name<'a> {
            fn len(&self) -> usize { self.len }
        }
    )
}

macro_rules! impl_iterator_indirect {
    ($name:ident, $item:ident) => (
        impl<'a> Iterator for $name<'a> {
            type Item = $item<'a>;
            fn next(&mut self) -> Option<$item<'a>> {
                if self.idx < self.len as isize {
                    let item = $item::from_raw(unsafe { *self.ptr.offset(self.idx) });
                    self.idx = self.idx + 1;
                    Some(item)
                } else {
                    None
                }
            }
        }

        impl<'a> ExactSizeIterator for $name<'a> {
            fn len(&self) -> usize { self.len }
        }
    )
}

macro_rules! impl_iterator_pod {
    ($name:ident, $item:ident) => (
        impl<'a> Iterator for $name<'a> {
            type Item = $item;
            fn next(&mut self) -> Option<$item> {
                if self.idx < self.len as isize {
                    let item = $item::from_raw(unsafe { self.ptr.offset(self.idx) });
                    self.idx = self.idx + 1;
                    Some(item)
                } else {
                    None
                }
            }
        }
    )
}

macro_rules! define_type {
    // Reference type
    ($(#[$type_attr:meta])* struct $name:ident(&$raw:ty)) => (
        $(#[$type_attr])*
        pub struct $name<'a>(&'a $raw);

        #[doc(hidden)]
        impl<'a> $name<'a> {
            pub fn from_raw(raw: *const $raw) -> $name<'a> {
                unsafe { $name(&*raw) }
            }
            pub fn to_raw(&self) -> *const $raw {
                self.0
            }
        }

        impl<'a> ::std::ops::Deref for $name<'a> {
            type Target = $raw;
            fn deref<'b>(&'b self) -> &'b $raw { &self.0 }
        }
    );
    // Non-reference type = POD
    ($(#[$type_attr:meta])* struct $name:ident($raw:ty)) => (
        $(#[$type_attr])*
        pub struct $name($raw);

        #[doc(hidden)]
        impl $name {
            pub fn from_raw(raw: *const $raw) -> $name {
                unsafe { $name(*raw) }
            }
        }

        impl ::std::ops::Deref for $name {
            type Target = $raw;
            fn deref<'a>(&'a self) -> &'a $raw { &self.0 }
        }
    );
}

macro_rules! define_type_and_iterator {
    (
        $(#[$type_attr:meta])* struct $type_name:ident(&$raw:ty)
        $(#[$iter_attr:meta])* struct $iter_name:ident
    ) => (
        define_type!($(#[$type_attr])* struct $type_name(&$raw));
        define_iter!($(#[$iter_attr])* struct $iter_name -> *const $raw);
        impl_iterator!($iter_name, $type_name);
    );
    (
        $(#[$type_attr:meta])* struct $type_name:ident($raw:ty)
        $(#[$iter_attr:meta])* struct $iter_name:ident
    ) => (
        define_type!($(#[$type_attr])* struct $type_name($raw));
        define_iter!($(#[$iter_attr])* struct $iter_name -> *const $raw);
        impl_iterator_pod!($iter_name, $type_name);
    );
}

macro_rules! define_type_and_iterator_indirect {
    (
        $(#[$type_attr:meta])* struct $type_name:ident(&$raw:ty)
        $(#[$iter_attr:meta])* struct $iter_name:ident
    ) => (
        define_type!($(#[$type_attr])* struct $type_name(&$raw));
        define_iter!($(#[$iter_attr])* struct $iter_name -> *const *const $raw);
        impl_iterator_indirect!($iter_name, $type_name);
    );
}
