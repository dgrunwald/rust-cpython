
// Copyright (c) 2016-2021 Daniel Grunwald
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this
// software and associated documentation files (the "Software"), to deal in the Software
// without restriction, including without limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons
// to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or
// substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
// INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR
// PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE
// FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
// OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.


// !!!!!!!!!!!!!!!!!!!!!!!!!!!
// THIS IS A GENERATED FILE !!
//       DO NOT MODIFY      !!
// !!!!!!!!!!!!!!!!!!!!!!!!!!!
//
// REGENERATE USING THE MAKEFILE IN ROOT OF REPOSITORY: make build

#[macro_export]
#[doc(hidden)]
macro_rules! py_class_impl {
    // TT muncher macro. Results are accumulated in $info $slots $impls and $members.


    // Base case: we're done munching and can start producing code:
    {   {}
        $class:ident $py:ident
        /* info: */ {
            $base_type:ty,
            $size:expr,
            { $( $class_visibility:tt )* },
            $gc:tt,
            /* data: */ [ $( { $data_offset:expr, $data_name:ident, $data_ty:ty, $init_expr:expr, $init_ty:ty } )* ]
        }
        $slots:tt { $( $imp:item )* } $members:tt $props:tt
    } => {
        $crate::py_coerce_item! {
            $($class_visibility)* struct $class { _unsafe_inner: $crate::PyObject }
        }

        $crate::py_impl_to_py_object_for_python_object!($class);
        $crate::py_impl_from_py_object_for_python_object!($class);

        impl $crate::PythonObject for $class {
            #[inline]
            fn as_object(&self) -> &$crate::PyObject {
                &self._unsafe_inner
            }

            #[inline]
            fn into_object(self) -> $crate::PyObject {
                self._unsafe_inner
            }

            /// Unchecked downcast from PyObject to Self.
            /// Undefined behavior if the input object does not have the expected type.
            #[inline]
            unsafe fn unchecked_downcast_from(obj: $crate::PyObject) -> Self {
                $class { _unsafe_inner: obj }
            }

            /// Unchecked downcast from PyObject to Self.
            /// Undefined behavior if the input object does not have the expected type.
            #[inline]
            unsafe fn unchecked_downcast_borrow_from<'a>(obj: &'a $crate::PyObject) -> &'a Self {
                std::mem::transmute(obj)
            }
        }

        impl $crate::PythonObjectWithCheckedDowncast for $class {
            #[inline]
            fn downcast_from<'p>(py: $crate::Python<'p>, obj: $crate::PyObject) -> $crate::_detail::Result<$class, $crate::PythonObjectDowncastError<'p>> {
                if py.get_type::<$class>().is_instance(py, &obj) {
                    Ok($class { _unsafe_inner: obj })
                } else {
                    Err($crate::PythonObjectDowncastError::new(
                        py,
                        stringify!($class),
                        obj.get_type(py),
                    ))
                }
            }

            #[inline]
            fn downcast_borrow_from<'a, 'p>(py: $crate::Python<'p>, obj: &'a $crate::PyObject) -> $crate::_detail::Result<&'a $class, $crate::PythonObjectDowncastError<'p>> {
                if py.get_type::<$class>().is_instance(py, obj) {
                    unsafe { Ok(std::mem::transmute(obj)) }
                } else {
                    Err($crate::PythonObjectDowncastError::new(
                        py,
                        stringify!($class),
                        obj.get_type(py),
                    ))
                }
            }
        }

        $crate::py_coerce_item! {
            impl $crate::py_class::BaseObject for $class {
                type InitType = ( $( $init_ty, )* );

                #[inline]
                fn size() -> usize {
                    $size
                }

                unsafe fn alloc(
                    py: $crate::Python,
                    ty: &$crate::PyType,
                    ( $( $data_name, )* ): Self::InitType
                ) -> $crate::PyResult<$crate::PyObject>
                {
                    let obj = <$base_type as $crate::py_class::BaseObject>::alloc(py, ty, ())?;
                    $( $crate::py_class::data_init::<$data_ty>(py, &obj, $data_offset, $init_expr); )*
                    Ok(obj)
                }

                unsafe fn dealloc(py: $crate::Python, obj: *mut $crate::_detail::ffi::PyObject) {
                    $( $crate::py_class::data_drop::<$data_ty>(py, obj, $data_offset); )*
                    <$base_type as $crate::py_class::BaseObject>::dealloc(py, obj)
                }
            }
        }
        $($imp)*
        $crate::py_coerce_item! {
            impl $class {
                $($class_visibility)* fn create_instance(py: $crate::Python $( , $data_name : $init_ty )* ) -> $crate::PyResult<$class> {
                    let obj = unsafe {
                        <$class as $crate::py_class::BaseObject>::alloc(
                            py, &py.get_type::<$class>(), ( $($data_name,)* )
                        )
                    }?;
                    return Ok($class { _unsafe_inner: obj });

                    // hide statics in create_instance to avoid name conflicts
                    static mut TYPE_OBJECT : $crate::_detail::ffi::PyTypeObject
                        = $crate::py_class_type_object_static_init!($class, $gc, $slots);
                    static mut INIT_ACTIVE: bool = false;

                    // trait implementations that need direct access to TYPE_OBJECT
                    impl $crate::PythonObjectWithTypeObject for $class {
                        fn type_object(py: $crate::Python) -> $crate::PyType {
                            unsafe {
                                if $crate::py_class::is_ready(py, &TYPE_OBJECT) {
                                    $crate::PyType::from_type_ptr(py, &mut TYPE_OBJECT)
                                } else {
                                    // automatically initialize the class on-demand
                                    <$class as $crate::py_class::PythonObjectFromPyClassMacro>::initialize(py, None)
                                        .expect(concat!("An error occurred while initializing class ", stringify!($class)))
                                }
                            }
                        }
                    }

                    impl $crate::py_class::PythonObjectFromPyClassMacro for $class {
                        fn initialize(py: $crate::Python, module_name: Option<&str>) -> $crate::PyResult<$crate::PyType> {
                            unsafe {
                                if $crate::py_class::is_ready(py, &TYPE_OBJECT) {
                                    return Ok($crate::PyType::from_type_ptr(py, &mut TYPE_OBJECT));
                                }
                                assert!(!INIT_ACTIVE,
                                    concat!("Reentrancy detected: already initializing class ",
                                    stringify!($class)));
                                INIT_ACTIVE = true;
                                let res = init(py, module_name);
                                INIT_ACTIVE = false;
                                res
                            }
                        }

                        fn add_to_module(py: $crate::Python, module: &$crate::PyModule) -> $crate::PyResult<()> {
                            let ty = <$class as $crate::py_class::PythonObjectFromPyClassMacro>::initialize(py, module.name(py).ok())?;
                            module.add(py, stringify!($class), ty)
                        }
                    }

                    fn init($py: $crate::Python, module_name: Option<&str>) -> $crate::PyResult<$crate::PyType> {
                        $crate::py_class_type_object_dynamic_init!($class, $py, TYPE_OBJECT, module_name, $slots $props);
                        $crate::py_class_init_members!($class, $py, TYPE_OBJECT, $members);
                        unsafe {
                            if $crate::_detail::ffi::PyType_Ready(&mut TYPE_OBJECT) == 0 {
                                Ok($crate::PyType::from_type_ptr($py, &mut TYPE_OBJECT))
                            } else {
                                Err($crate::PyErr::fetch($py))
                            }
                        }
                    }
                }
            }
        }
    };

    { { data $data_name:ident : $data_type:ty; $($tail:tt)* }
        $class:ident $py:ident
        /* info: */ {
            $base_type: ty,
            $size: expr,
            $class_visibility: tt,
            $gc: tt,
            [ $( $data:tt )* ]
        }
        $slots:tt
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py
        /* info: */ {
            $base_type,
            /* size: */ $crate::py_class::data_new_size::<$data_type>($size),
            $class_visibility,
            $gc,
            /* data: */ [
                $($data)*
                {
                    $crate::py_class::data_offset::<$data_type>($size),
                    $data_name,
                    $data_type,
                    /* init_expr: */ $data_name,
                    /* init_ty: */ $data_type
                }
            ]
        }
        $slots
        /* impl: */ {
            $($imp)*
            impl $class {
                fn $data_name<'a>(&'a self, py: $crate::Python<'a>) -> &'a $data_type {
                    unsafe {
                        $crate::py_class::data_get::<$data_type>(
                        py,
                        &self._unsafe_inner,
                        $crate::py_class::data_offset::<$data_type>($size)
                        )
                    }
                }
            }
        }
        $members $props
    }};
    { { @shared data $data_name:ident : $data_type:ty; $($tail:tt)* }
        $class:ident $py:ident
        /* info: */ {
            $base_type: ty,
            $size: expr,
            $class_visibility: tt,
            $gc: tt,
            [ $( $data:tt )* ]
        }
        $slots:tt
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py
        /* info: */ {
            $base_type,
            /* size: */ $crate::py_class::data_new_size::<$crate::PySharedRefCell<$data_type>>($size),
            $class_visibility,
            $gc,
            /* data: */ [
                $($data)*
                {
                    $crate::py_class::data_offset::<$crate::PySharedRefCell<$data_type>>($size),
                    $data_name,
                    /* data_ty: */ $crate::PySharedRefCell<$data_type>,
                    /* init_expr: */ $crate::PySharedRefCell::<$data_type>::new($data_name),
                    /* init_ty: */ $data_type
                }
            ]
        }
        $slots
        /* impl: */ {
            $($imp)*
            impl $class {
                fn $data_name<'a>(&'a self, py: $crate::Python<'a>) -> $crate::PySharedRef<'a, $data_type> {
                    unsafe {
                        let data = $crate::py_class::data_get::<$crate::PySharedRefCell<$data_type>>(
                        py,
                        &self._unsafe_inner,
                        $crate::py_class::data_offset::<$crate::PySharedRefCell<$data_type>>($size)
                        );
                        $crate::PySharedRef::new(py, &self._unsafe_inner, data)
                    }
                }
            }
        }
        $members $props
    }};
    { { def __traverse__(&$slf:tt, $visit:ident) {$($body:tt)*} $($tail:tt)* }
        $class:ident $py:ident
        /* info: */ {
            $base_type: ty,
            $size: expr,
            $class_visibility: tt,
            /* gc: */ {
                /* traverse_proc: */ None,
                $traverse_data: tt
            },
            $datas: tt
        }
        $slots:tt
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py
        /* info: */ {
            $base_type,
            $size,
            $class_visibility,
            /* gc: */ {
                /* traverse_proc: */ $class::__traverse__,
                $traverse_data
            },
            $datas
        }
        $slots
        /* impl: */ {
            $($imp)*
            $crate::py_coerce_item!{
                impl $class {
                    fn __traverse__(&$slf,
                    $py: $crate::Python,
                    $visit: $crate::py_class::gc::VisitProc)
                    -> $crate::_detail::Result<(), $crate::py_class::gc::TraverseError> {
                        let _ = $py;
                        $($body)*
                    }
                }
            }
        }
        $members $props
    }};
    { { def __clear__ (&$slf:ident) {$($body:tt)*} $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            /* type_slots */ [ $( $tp_slot_name:ident : $tp_slot_value:expr, )* ]
            $as_number:tt $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            /* type_slots */ [
                $( $tp_slot_name : $tp_slot_value, )*
                tp_clear: $crate::py_class_tp_clear!($class),
            ]
            $as_number $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_coerce_item!{
                impl $class {
                    fn __clear__(&$slf, $py: $crate::Python) {
                        let _ = $py;
                        $($body)*
                    }
                }
            }
        }
        $members $props
    }};
    { { def __abs__(&$slf:ident) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_absolute: $crate::py_class_unary_slot!($class::__abs__, *mut $crate::_detail::ffi::PyObject, $crate::_detail::PyObjectCallbackConverter),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __abs__(&$slf,) $res_type; { $($body)* } [] }
        }
        $members $props
    }};

    { { def __abs__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Invalid signature for operator __abs__" }
    };
    { { def __add__($left:ident, $right:ident) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_add: $crate::py_class_numeric_slot!(binary $class::__add__),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __add__() $res_type; { $($body)* } [ { $left : &$crate::PyObject = {} } { $right : &$crate::PyObject = {} } ] }
        }
        $members $props
    }};

    { { def __add__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Invalid signature for binary numeric operator __add__" }
    };

    { { def __aenter__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "__aenter__ is not supported by py_class! yet." }
    };

    { { def __aexit__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "__aexit__ is not supported by py_class! yet." }
    };

    { { def __aiter__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "__aiter__ is not supported by py_class! yet." }
    };
    { { def __and__($left:ident, $right:ident) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_and: $crate::py_class_numeric_slot!(binary $class::__and__),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __and__() $res_type; { $($body)* } [ { $left : &$crate::PyObject = {} } { $right : &$crate::PyObject = {} } ] }
        }
        $members $props
    }};

    { { def __and__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Invalid signature for binary numeric operator __and__" }
    };

    { { def __await__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "__await__ is not supported by py_class! yet." }
    };
    { { def __bool__(&$slf:ident) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_bool: $crate::py_class_unary_slot!($class::__bool__, $crate::_detail::libc::c_int, $crate::py_class::slots::BoolConverter),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __bool__(&$slf,) $res_type; { $($body)* } [] }
        }
        $members $props
    }};

    { { def __bool__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Invalid signature for operator __bool__" }
    };
    { {   def __call__ (&$slf:ident) -> $res_type:ty { $( $body:tt )* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            /* type_slots */ [ $( $tp_slot_name:ident : $tp_slot_value:expr, )* ]
            $as_number:tt $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            /* type_slots */ [
                $( $tp_slot_name : $tp_slot_value, )*
                tp_call: $crate::py_class_call_slot!{$class::__call__ []},
            ]
            $as_number $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __call__(&$slf,) $res_type; { $($body)* } [] }
        }
        $members $props
    }};
    { {  $visibility:vis def __call__ (&$slf:ident) -> $res_type:ty { $( $body:tt )* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            /* type_slots */ [ $( $tp_slot_name:ident : $tp_slot_value:expr, )* ]
            $as_number:tt $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            /* type_slots */ [
                $( $tp_slot_name : $tp_slot_value, )*
                tp_call: $crate::py_class_call_slot!{$class::__call__ []},
            ]
            $as_number $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, $visibility, __call__(&$slf,) $res_type; { $($body)* } [] }
        }
        $members $props
    }};
    { {   def __call__ (&$slf:ident, $($p:tt)+) -> $res_type:ty { $( $body:tt )* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            /* type_slots */ [ $( $tp_slot_name:ident : $tp_slot_value:expr, )* ]
            $as_number:tt $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            /* type_slots */ [
                $( $tp_slot_name : $tp_slot_value, )*
                tp_call: $crate::py_argparse_parse_plist_impl!{py_class_call_slot {$class::__call__} [] ($($p)+,)},
            ]
            $as_number $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_argparse_parse_plist_impl!{
                py_class_impl_item { $class, $py, pub, __call__(&$slf,) $res_type; { $($body)* } }
                [] ($($p)+,)
            }
        }
        $members $props
    }};
    { {  $visibility:vis def __call__ (&$slf:ident, $($p:tt)+) -> $res_type:ty { $( $body:tt )* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            /* type_slots */ [ $( $tp_slot_name:ident : $tp_slot_value:expr, )* ]
            $as_number:tt $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            /* type_slots */ [
                $( $tp_slot_name : $tp_slot_value, )*
                tp_call: $crate::py_argparse_parse_plist_impl!{py_class_call_slot {$class::__call__} [] ($($p)+,)},
            ]
            $as_number $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_argparse_parse_plist_impl!{
                py_class_impl_item { $class, $py, $visibility, __call__(&$slf,) $res_type; { $($body)* } }
                [] ($($p)+,)
            }
        }
        $members $props
    }};

    { { def __cmp__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "__cmp__ is not supported by py_class! use __richcmp__ instead." }
    };

    { { def __coerce__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "__coerce__ is not supported by py_class! yet." }
    };

    { { def __complex__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "__complex__ is not supported by py_class! yet." }
    };
    { { def __contains__(&$slf:ident, $item:ident : Option<&$item_name:ty>) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt $as_number:tt
            /* as_sequence */ [ $( $sq_slot_name:ident : $sq_slot_value:expr, )* ]
            $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots $as_number
            /* as_sequence */ [
                $( $sq_slot_name : $sq_slot_value, )*
                sq_contains: $crate::py_class_contains_slot!($class::__contains__, [Option<&$item_name>]),
            ]
            $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __contains__(&$slf,) $res_type; { $($body)* } [{ $item : Option<&$item_name> = {} }] }
        }
        $members $props
    }};
    { { def __contains__(&$slf:ident, $item:ident : &$item_name:ty) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt $as_number:tt
            /* as_sequence */ [ $( $sq_slot_name:ident : $sq_slot_value:expr, )* ]
            $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots $as_number
            /* as_sequence */ [
                $( $sq_slot_name : $sq_slot_value, )*
                sq_contains: $crate::py_class_contains_slot!($class::__contains__, [&$item_name]),
            ]
            $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __contains__(&$slf,) $res_type; { $($body)* } [{ $item : &$item_name = {} }] }
        }
        $members $props
    }};
    { { def __contains__(&$slf:ident, $item:ident : $item_name:ty) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt $as_number:tt
            /* as_sequence */ [ $( $sq_slot_name:ident : $sq_slot_value:expr, )* ]
            $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots $as_number
            /* as_sequence */ [
                $( $sq_slot_name : $sq_slot_value, )*
                sq_contains: $crate::py_class_contains_slot!($class::__contains__, [$item_name]),
            ]
            $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __contains__(&$slf,) $res_type; { $($body)* } [{ $item : $item_name = {} }] }
        }
        $members $props
    }};

    { { def __contains__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Invalid signature for operator __contains__" }
    };

    { { def __del__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "__del__ is not supported by py_class!; Use a data member with a Drop impl instead." }
    };

    { { def __delattr__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "__delattr__ is not supported by py_class! yet." }
    };

    { { def __delete__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "__delete__ is not supported by py_class! yet." }
    };
    { { def __delitem__(&$slf:ident, $key:ident : Option<&$key_name:ty>) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt $as_number:tt $as_sequence:tt $as_mapping:tt
            /* setdelitem */ [
                sdi_setitem: $sdi_setitem_slot_value:tt,
                sdi_delitem: {},
            ]
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots $as_number $as_sequence $as_mapping
            /* setdelitem */ [
                sdi_setitem: $sdi_setitem_slot_value,
                sdi_delitem: { $crate::py_class_binary_slot!($class::__delitem__, [Option<&$key_name>], $crate::_detail::libc::c_int, $crate::py_class::slots::UnitCallbackConverter) },
            ]
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __delitem__(&$slf,) $res_type; { $($body)* } [{ $key : Option<&$key_name> = {} }] }
        }
        $members $props
    }};
    { { def __delitem__(&$slf:ident, $key:ident : &$key_name:ty) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt $as_number:tt $as_sequence:tt $as_mapping:tt
            /* setdelitem */ [
                sdi_setitem: $sdi_setitem_slot_value:tt,
                sdi_delitem: {},
            ]
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots $as_number $as_sequence $as_mapping
            /* setdelitem */ [
                sdi_setitem: $sdi_setitem_slot_value,
                sdi_delitem: { $crate::py_class_binary_slot!($class::__delitem__, [&$key_name], $crate::_detail::libc::c_int, $crate::py_class::slots::UnitCallbackConverter) },
            ]
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __delitem__(&$slf,) $res_type; { $($body)* } [{ $key : &$key_name = {} }] }
        }
        $members $props
    }};
    { { def __delitem__(&$slf:ident, $key:ident : $key_name:ty) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt $as_number:tt $as_sequence:tt $as_mapping:tt
            /* setdelitem */ [
                sdi_setitem: $sdi_setitem_slot_value:tt,
                sdi_delitem: {},
            ]
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots $as_number $as_sequence $as_mapping
            /* setdelitem */ [
                sdi_setitem: $sdi_setitem_slot_value,
                sdi_delitem: { $crate::py_class_binary_slot!($class::__delitem__, [$key_name], $crate::_detail::libc::c_int, $crate::py_class::slots::UnitCallbackConverter) },
            ]
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __delitem__(&$slf,) $res_type; { $($body)* } [{ $key : $key_name = {} }] }
        }
        $members $props
    }};

    { { def __delitem__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Invalid signature for operator __delitem__" }
    };

    { { def __dir__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "__dir__ is not supported by py_class! yet." }
    };

    { { def __div__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "__div__ is not supported by py_class! yet." }
    };
    { { def __divmod__($left:ident, $right:ident) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_divmod: $crate::py_class_numeric_slot!(binary $class::__divmod__),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __divmod__() $res_type; { $($body)* } [ { $left : &$crate::PyObject = {} } { $right : &$crate::PyObject = {} } ] }
        }
        $members $props
    }};

    { { def __divmod__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Invalid signature for binary numeric operator __divmod__" }
    };

    { { def __eq__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "__eq__ is not supported by py_class! use __richcmp__ instead." }
    };

    { { def __float__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "__float__ is not supported by py_class! yet." }
    };
    { { def __floordiv__($left:ident, $right:ident) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_floor_divide: $crate::py_class_numeric_slot!(binary $class::__floordiv__),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __floordiv__() $res_type; { $($body)* } [ { $left : &$crate::PyObject = {} } { $right : &$crate::PyObject = {} } ] }
        }
        $members $props
    }};

    { { def __floordiv__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Invalid signature for binary numeric operator __floordiv__" }
    };

    { { def __ge__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "__ge__ is not supported by py_class! use __richcmp__ instead." }
    };

    { { def __get__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "__get__ is not supported by py_class! yet." }
    };

    { { def __getattr__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "__getattr__ is not supported by py_class! yet." }
    };

    { { def __getattribute__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "__getattribute__ is not supported by py_class! yet." }
    };
    { { def __getitem__(&$slf:ident, $key:ident : Option<&$key_name:ty>) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt $as_number:tt
            /* as_sequence */ [ $( $sq_slot_name:ident : $sq_slot_value:expr, )* ]
            /* as_mapping */ [ $( $mp_slot_name:ident : $mp_slot_value:expr, )* ]
            $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots $as_number
            /* as_sequence */ [
                $( $sq_slot_name : $sq_slot_value, )*
                sq_item: Some($crate::py_class::slots::sq_item),
            ]
            /* as_mapping */ [
                $( $mp_slot_name : $mp_slot_value, )*
                mp_subscript: $crate::py_class_binary_slot!($class::__getitem__, [Option<&$key_name>], *mut $crate::_detail::ffi::PyObject, $crate::_detail::PyObjectCallbackConverter),
            ]
            $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __getitem__(&$slf,) $res_type; { $($body)* } [{ $key : Option<&$key_name> = {} }] }
        }
        $members $props
    }};
    { { def __getitem__(&$slf:ident, $key:ident : &$key_name:ty) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt $as_number:tt
            /* as_sequence */ [ $( $sq_slot_name:ident : $sq_slot_value:expr, )* ]
            /* as_mapping */ [ $( $mp_slot_name:ident : $mp_slot_value:expr, )* ]
            $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots $as_number
            /* as_sequence */ [
                $( $sq_slot_name : $sq_slot_value, )*
                sq_item: Some($crate::py_class::slots::sq_item),
            ]
            /* as_mapping */ [
                $( $mp_slot_name : $mp_slot_value, )*
                mp_subscript: $crate::py_class_binary_slot!($class::__getitem__, [&$key_name], *mut $crate::_detail::ffi::PyObject, $crate::_detail::PyObjectCallbackConverter),
            ]
            $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __getitem__(&$slf,) $res_type; { $($body)* } [{ $key : &$key_name = {} }] }
        }
        $members $props
    }};
    { { def __getitem__(&$slf:ident, $key:ident : $key_name:ty) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt $as_number:tt
            /* as_sequence */ [ $( $sq_slot_name:ident : $sq_slot_value:expr, )* ]
            /* as_mapping */ [ $( $mp_slot_name:ident : $mp_slot_value:expr, )* ]
            $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots $as_number
            /* as_sequence */ [
                $( $sq_slot_name : $sq_slot_value, )*
                sq_item: Some($crate::py_class::slots::sq_item),
            ]
            /* as_mapping */ [
                $( $mp_slot_name : $mp_slot_value, )*
                mp_subscript: $crate::py_class_binary_slot!($class::__getitem__, [$key_name], *mut $crate::_detail::ffi::PyObject, $crate::_detail::PyObjectCallbackConverter),
            ]
            $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __getitem__(&$slf,) $res_type; { $($body)* } [{ $key : $key_name = {} }] }
        }
        $members $props
    }};

    { { def __getitem__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Invalid signature for operator __getitem__" }
    };

    { { def __gt__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "__gt__ is not supported by py_class! use __richcmp__ instead." }
    };
    { { def __hash__(&$slf:ident) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            /* type_slots */ [ $( $tp_slot_name:ident : $tp_slot_value:expr, )* ]
            $as_number:tt $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            /* type_slots */ [
                $( $tp_slot_name : $tp_slot_value, )*
                tp_hash: $crate::py_class_unary_slot!($class::__hash__, $crate::Py_hash_t, $crate::py_class::slots::HashConverter),
            ]
            $as_number $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __hash__(&$slf,) $res_type; { $($body)* } [] }
        }
        $members $props
    }};

    { { def __hash__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Invalid signature for operator __hash__" }
    };
    { { def __iadd__(&$slf:ident, $other:ident : Option<&$other_name:ty>) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_inplace_add: $crate::py_class_binary_slot!($class::__iadd__, [Option<&$other_name>], *mut $crate::_detail::ffi::PyObject, $crate::_detail::PyObjectCallbackConverter),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __iadd__(&$slf,) $res_type; { $($body)* } [{ $other : Option<&$other_name> = {} }] }
        }
        $members $props
    }};
    { { def __iadd__(&$slf:ident, $other:ident : &$other_name:ty) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_inplace_add: $crate::py_class_binary_slot!($class::__iadd__, [&$other_name], *mut $crate::_detail::ffi::PyObject, $crate::_detail::PyObjectCallbackConverter),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __iadd__(&$slf,) $res_type; { $($body)* } [{ $other : &$other_name = {} }] }
        }
        $members $props
    }};
    { { def __iadd__(&$slf:ident, $other:ident : $other_name:ty) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_inplace_add: $crate::py_class_binary_slot!($class::__iadd__, [$other_name], *mut $crate::_detail::ffi::PyObject, $crate::_detail::PyObjectCallbackConverter),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __iadd__(&$slf,) $res_type; { $($body)* } [{ $other : $other_name = {} }] }
        }
        $members $props
    }};

    { { def __iadd__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Invalid signature for operator __iadd__" }
    };
    { { def __iand__(&$slf:ident, $other:ident : Option<&$other_name:ty>) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_inplace_and: $crate::py_class_binary_slot!($class::__iand__, [Option<&$other_name>], *mut $crate::_detail::ffi::PyObject, $crate::_detail::PyObjectCallbackConverter),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __iand__(&$slf,) $res_type; { $($body)* } [{ $other : Option<&$other_name> = {} }] }
        }
        $members $props
    }};
    { { def __iand__(&$slf:ident, $other:ident : &$other_name:ty) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_inplace_and: $crate::py_class_binary_slot!($class::__iand__, [&$other_name], *mut $crate::_detail::ffi::PyObject, $crate::_detail::PyObjectCallbackConverter),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __iand__(&$slf,) $res_type; { $($body)* } [{ $other : &$other_name = {} }] }
        }
        $members $props
    }};
    { { def __iand__(&$slf:ident, $other:ident : $other_name:ty) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_inplace_and: $crate::py_class_binary_slot!($class::__iand__, [$other_name], *mut $crate::_detail::ffi::PyObject, $crate::_detail::PyObjectCallbackConverter),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __iand__(&$slf,) $res_type; { $($body)* } [{ $other : $other_name = {} }] }
        }
        $members $props
    }};

    { { def __iand__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Invalid signature for operator __iand__" }
    };

    { { def __idiv__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "__idiv__ is not supported by py_class! yet." }
    };
    { { def __ifloordiv__(&$slf:ident, $other:ident : Option<&$other_name:ty>) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_inplace_floor_divide: $crate::py_class_binary_slot!($class::__ifloordiv__, [Option<&$other_name>], *mut $crate::_detail::ffi::PyObject, $crate::_detail::PyObjectCallbackConverter),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __ifloordiv__(&$slf,) $res_type; { $($body)* } [{ $other : Option<&$other_name> = {} }] }
        }
        $members $props
    }};
    { { def __ifloordiv__(&$slf:ident, $other:ident : &$other_name:ty) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_inplace_floor_divide: $crate::py_class_binary_slot!($class::__ifloordiv__, [&$other_name], *mut $crate::_detail::ffi::PyObject, $crate::_detail::PyObjectCallbackConverter),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __ifloordiv__(&$slf,) $res_type; { $($body)* } [{ $other : &$other_name = {} }] }
        }
        $members $props
    }};
    { { def __ifloordiv__(&$slf:ident, $other:ident : $other_name:ty) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_inplace_floor_divide: $crate::py_class_binary_slot!($class::__ifloordiv__, [$other_name], *mut $crate::_detail::ffi::PyObject, $crate::_detail::PyObjectCallbackConverter),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __ifloordiv__(&$slf,) $res_type; { $($body)* } [{ $other : $other_name = {} }] }
        }
        $members $props
    }};

    { { def __ifloordiv__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Invalid signature for operator __ifloordiv__" }
    };
    { { def __ilshift__(&$slf:ident, $other:ident : Option<&$other_name:ty>) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_inplace_lshift: $crate::py_class_binary_slot!($class::__ilshift__, [Option<&$other_name>], *mut $crate::_detail::ffi::PyObject, $crate::_detail::PyObjectCallbackConverter),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __ilshift__(&$slf,) $res_type; { $($body)* } [{ $other : Option<&$other_name> = {} }] }
        }
        $members $props
    }};
    { { def __ilshift__(&$slf:ident, $other:ident : &$other_name:ty) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_inplace_lshift: $crate::py_class_binary_slot!($class::__ilshift__, [&$other_name], *mut $crate::_detail::ffi::PyObject, $crate::_detail::PyObjectCallbackConverter),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __ilshift__(&$slf,) $res_type; { $($body)* } [{ $other : &$other_name = {} }] }
        }
        $members $props
    }};
    { { def __ilshift__(&$slf:ident, $other:ident : $other_name:ty) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_inplace_lshift: $crate::py_class_binary_slot!($class::__ilshift__, [$other_name], *mut $crate::_detail::ffi::PyObject, $crate::_detail::PyObjectCallbackConverter),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __ilshift__(&$slf,) $res_type; { $($body)* } [{ $other : $other_name = {} }] }
        }
        $members $props
    }};

    { { def __ilshift__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Invalid signature for operator __ilshift__" }
    };
    { { def __imatmul__(&$slf:ident, $other:ident : Option<&$other_name:ty>) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_inplace_matrix_multiply: $crate::py_class_binary_slot!($class::__imatmul__, [Option<&$other_name>], *mut $crate::_detail::ffi::PyObject, $crate::_detail::PyObjectCallbackConverter),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __imatmul__(&$slf,) $res_type; { $($body)* } [{ $other : Option<&$other_name> = {} }] }
        }
        $members $props
    }};
    { { def __imatmul__(&$slf:ident, $other:ident : &$other_name:ty) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_inplace_matrix_multiply: $crate::py_class_binary_slot!($class::__imatmul__, [&$other_name], *mut $crate::_detail::ffi::PyObject, $crate::_detail::PyObjectCallbackConverter),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __imatmul__(&$slf,) $res_type; { $($body)* } [{ $other : &$other_name = {} }] }
        }
        $members $props
    }};
    { { def __imatmul__(&$slf:ident, $other:ident : $other_name:ty) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_inplace_matrix_multiply: $crate::py_class_binary_slot!($class::__imatmul__, [$other_name], *mut $crate::_detail::ffi::PyObject, $crate::_detail::PyObjectCallbackConverter),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __imatmul__(&$slf,) $res_type; { $($body)* } [{ $other : $other_name = {} }] }
        }
        $members $props
    }};

    { { def __imatmul__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Invalid signature for operator __imatmul__" }
    };
    { { def __imod__(&$slf:ident, $other:ident : Option<&$other_name:ty>) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_inplace_remainder: $crate::py_class_binary_slot!($class::__imod__, [Option<&$other_name>], *mut $crate::_detail::ffi::PyObject, $crate::_detail::PyObjectCallbackConverter),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __imod__(&$slf,) $res_type; { $($body)* } [{ $other : Option<&$other_name> = {} }] }
        }
        $members $props
    }};
    { { def __imod__(&$slf:ident, $other:ident : &$other_name:ty) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_inplace_remainder: $crate::py_class_binary_slot!($class::__imod__, [&$other_name], *mut $crate::_detail::ffi::PyObject, $crate::_detail::PyObjectCallbackConverter),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __imod__(&$slf,) $res_type; { $($body)* } [{ $other : &$other_name = {} }] }
        }
        $members $props
    }};
    { { def __imod__(&$slf:ident, $other:ident : $other_name:ty) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_inplace_remainder: $crate::py_class_binary_slot!($class::__imod__, [$other_name], *mut $crate::_detail::ffi::PyObject, $crate::_detail::PyObjectCallbackConverter),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __imod__(&$slf,) $res_type; { $($body)* } [{ $other : $other_name = {} }] }
        }
        $members $props
    }};

    { { def __imod__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Invalid signature for operator __imod__" }
    };
    { { def __imul__(&$slf:ident, $other:ident : Option<&$other_name:ty>) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_inplace_multiply: $crate::py_class_binary_slot!($class::__imul__, [Option<&$other_name>], *mut $crate::_detail::ffi::PyObject, $crate::_detail::PyObjectCallbackConverter),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __imul__(&$slf,) $res_type; { $($body)* } [{ $other : Option<&$other_name> = {} }] }
        }
        $members $props
    }};
    { { def __imul__(&$slf:ident, $other:ident : &$other_name:ty) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_inplace_multiply: $crate::py_class_binary_slot!($class::__imul__, [&$other_name], *mut $crate::_detail::ffi::PyObject, $crate::_detail::PyObjectCallbackConverter),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __imul__(&$slf,) $res_type; { $($body)* } [{ $other : &$other_name = {} }] }
        }
        $members $props
    }};
    { { def __imul__(&$slf:ident, $other:ident : $other_name:ty) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_inplace_multiply: $crate::py_class_binary_slot!($class::__imul__, [$other_name], *mut $crate::_detail::ffi::PyObject, $crate::_detail::PyObjectCallbackConverter),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __imul__(&$slf,) $res_type; { $($body)* } [{ $other : $other_name = {} }] }
        }
        $members $props
    }};

    { { def __imul__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Invalid signature for operator __imul__" }
    };
    { { def __index__(&$slf:ident) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_index: $crate::py_class_unary_slot!($class::__index__, *mut $crate::_detail::ffi::PyObject, $crate::_detail::PyObjectCallbackConverter),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __index__(&$slf,) $res_type; { $($body)* } [] }
        }
        $members $props
    }};

    { { def __index__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Invalid signature for operator __index__" }
    };

    { { def __init__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "__init__ is not supported by py_class!; use __new__ instead." }
    };

    { { def __instancecheck__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "__instancecheck__ is not supported by py_class! yet." }
    };

    { { def __int__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "__int__ is not supported by py_class! yet." }
    };
    { { def __invert__(&$slf:ident) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_invert: $crate::py_class_unary_slot!($class::__invert__, *mut $crate::_detail::ffi::PyObject, $crate::_detail::PyObjectCallbackConverter),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __invert__(&$slf,) $res_type; { $($body)* } [] }
        }
        $members $props
    }};

    { { def __invert__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Invalid signature for operator __invert__" }
    };
    { { def __ior__(&$slf:ident, $other:ident : Option<&$other_name:ty>) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_inplace_or: $crate::py_class_binary_slot!($class::__ior__, [Option<&$other_name>], *mut $crate::_detail::ffi::PyObject, $crate::_detail::PyObjectCallbackConverter),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __ior__(&$slf,) $res_type; { $($body)* } [{ $other : Option<&$other_name> = {} }] }
        }
        $members $props
    }};
    { { def __ior__(&$slf:ident, $other:ident : &$other_name:ty) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_inplace_or: $crate::py_class_binary_slot!($class::__ior__, [&$other_name], *mut $crate::_detail::ffi::PyObject, $crate::_detail::PyObjectCallbackConverter),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __ior__(&$slf,) $res_type; { $($body)* } [{ $other : &$other_name = {} }] }
        }
        $members $props
    }};
    { { def __ior__(&$slf:ident, $other:ident : $other_name:ty) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_inplace_or: $crate::py_class_binary_slot!($class::__ior__, [$other_name], *mut $crate::_detail::ffi::PyObject, $crate::_detail::PyObjectCallbackConverter),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __ior__(&$slf,) $res_type; { $($body)* } [{ $other : $other_name = {} }] }
        }
        $members $props
    }};

    { { def __ior__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Invalid signature for operator __ior__" }
    };

    { { def __ipow__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "__ipow__ is not supported by py_class! yet." }
    };
    { { def __irshift__(&$slf:ident, $other:ident : Option<&$other_name:ty>) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_inplace_rshift: $crate::py_class_binary_slot!($class::__irshift__, [Option<&$other_name>], *mut $crate::_detail::ffi::PyObject, $crate::_detail::PyObjectCallbackConverter),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __irshift__(&$slf,) $res_type; { $($body)* } [{ $other : Option<&$other_name> = {} }] }
        }
        $members $props
    }};
    { { def __irshift__(&$slf:ident, $other:ident : &$other_name:ty) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_inplace_rshift: $crate::py_class_binary_slot!($class::__irshift__, [&$other_name], *mut $crate::_detail::ffi::PyObject, $crate::_detail::PyObjectCallbackConverter),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __irshift__(&$slf,) $res_type; { $($body)* } [{ $other : &$other_name = {} }] }
        }
        $members $props
    }};
    { { def __irshift__(&$slf:ident, $other:ident : $other_name:ty) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_inplace_rshift: $crate::py_class_binary_slot!($class::__irshift__, [$other_name], *mut $crate::_detail::ffi::PyObject, $crate::_detail::PyObjectCallbackConverter),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __irshift__(&$slf,) $res_type; { $($body)* } [{ $other : $other_name = {} }] }
        }
        $members $props
    }};

    { { def __irshift__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Invalid signature for operator __irshift__" }
    };
    { { def __isub__(&$slf:ident, $other:ident : Option<&$other_name:ty>) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_inplace_subtract: $crate::py_class_binary_slot!($class::__isub__, [Option<&$other_name>], *mut $crate::_detail::ffi::PyObject, $crate::_detail::PyObjectCallbackConverter),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __isub__(&$slf,) $res_type; { $($body)* } [{ $other : Option<&$other_name> = {} }] }
        }
        $members $props
    }};
    { { def __isub__(&$slf:ident, $other:ident : &$other_name:ty) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_inplace_subtract: $crate::py_class_binary_slot!($class::__isub__, [&$other_name], *mut $crate::_detail::ffi::PyObject, $crate::_detail::PyObjectCallbackConverter),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __isub__(&$slf,) $res_type; { $($body)* } [{ $other : &$other_name = {} }] }
        }
        $members $props
    }};
    { { def __isub__(&$slf:ident, $other:ident : $other_name:ty) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_inplace_subtract: $crate::py_class_binary_slot!($class::__isub__, [$other_name], *mut $crate::_detail::ffi::PyObject, $crate::_detail::PyObjectCallbackConverter),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __isub__(&$slf,) $res_type; { $($body)* } [{ $other : $other_name = {} }] }
        }
        $members $props
    }};

    { { def __isub__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Invalid signature for operator __isub__" }
    };
    { { def __iter__(&$slf:ident) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            /* type_slots */ [ $( $tp_slot_name:ident : $tp_slot_value:expr, )* ]
            $as_number:tt $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            /* type_slots */ [
                $( $tp_slot_name : $tp_slot_value, )*
                tp_iter: $crate::py_class_unary_slot!($class::__iter__, *mut $crate::_detail::ffi::PyObject, $crate::_detail::PyObjectCallbackConverter),
            ]
            $as_number $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __iter__(&$slf,) $res_type; { $($body)* } [] }
        }
        $members $props
    }};

    { { def __iter__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Invalid signature for operator __iter__" }
    };
    { { def __itruediv__(&$slf:ident, $other:ident : Option<&$other_name:ty>) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_inplace_true_divide: $crate::py_class_binary_slot!($class::__itruediv__, [Option<&$other_name>], *mut $crate::_detail::ffi::PyObject, $crate::_detail::PyObjectCallbackConverter),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __itruediv__(&$slf,) $res_type; { $($body)* } [{ $other : Option<&$other_name> = {} }] }
        }
        $members $props
    }};
    { { def __itruediv__(&$slf:ident, $other:ident : &$other_name:ty) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_inplace_true_divide: $crate::py_class_binary_slot!($class::__itruediv__, [&$other_name], *mut $crate::_detail::ffi::PyObject, $crate::_detail::PyObjectCallbackConverter),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __itruediv__(&$slf,) $res_type; { $($body)* } [{ $other : &$other_name = {} }] }
        }
        $members $props
    }};
    { { def __itruediv__(&$slf:ident, $other:ident : $other_name:ty) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_inplace_true_divide: $crate::py_class_binary_slot!($class::__itruediv__, [$other_name], *mut $crate::_detail::ffi::PyObject, $crate::_detail::PyObjectCallbackConverter),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __itruediv__(&$slf,) $res_type; { $($body)* } [{ $other : $other_name = {} }] }
        }
        $members $props
    }};

    { { def __itruediv__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Invalid signature for operator __itruediv__" }
    };
    { { def __ixor__(&$slf:ident, $other:ident : Option<&$other_name:ty>) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_inplace_xor: $crate::py_class_binary_slot!($class::__ixor__, [Option<&$other_name>], *mut $crate::_detail::ffi::PyObject, $crate::_detail::PyObjectCallbackConverter),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __ixor__(&$slf,) $res_type; { $($body)* } [{ $other : Option<&$other_name> = {} }] }
        }
        $members $props
    }};
    { { def __ixor__(&$slf:ident, $other:ident : &$other_name:ty) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_inplace_xor: $crate::py_class_binary_slot!($class::__ixor__, [&$other_name], *mut $crate::_detail::ffi::PyObject, $crate::_detail::PyObjectCallbackConverter),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __ixor__(&$slf,) $res_type; { $($body)* } [{ $other : &$other_name = {} }] }
        }
        $members $props
    }};
    { { def __ixor__(&$slf:ident, $other:ident : $other_name:ty) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_inplace_xor: $crate::py_class_binary_slot!($class::__ixor__, [$other_name], *mut $crate::_detail::ffi::PyObject, $crate::_detail::PyObjectCallbackConverter),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __ixor__(&$slf,) $res_type; { $($body)* } [{ $other : $other_name = {} }] }
        }
        $members $props
    }};

    { { def __ixor__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Invalid signature for operator __ixor__" }
    };

    { { def __le__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "__le__ is not supported by py_class! use __richcmp__ instead." }
    };
    { { def __len__(&$slf:ident) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt $as_number:tt
            /* as_sequence */ [ $( $sq_slot_name:ident : $sq_slot_value:expr, )* ]
            /* as_mapping */ [ $( $mp_slot_name:ident : $mp_slot_value:expr, )* ]
            $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots $as_number
            /* as_sequence */ [
                $( $sq_slot_name : $sq_slot_value, )*
                sq_length: $crate::py_class_unary_slot!($class::__len__, $crate::_detail::ffi::Py_ssize_t, $crate::py_class::slots::LenResultConverter),
            ]
            /* as_mapping */ [
                $( $mp_slot_name : $mp_slot_value, )*
                mp_length: Some($crate::_detail::ffi::PySequence_Size),
            ]
            $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __len__(&$slf,) $res_type; { $($body)* } [] }
        }
        $members $props
    }};

    { { def __len__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Invalid signature for operator __len__" }
    };

    { { def __long__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "__long__ is not supported by py_class! yet." }
    };
    { { def __lshift__($left:ident, $right:ident) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_lshift: $crate::py_class_numeric_slot!(binary $class::__lshift__),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __lshift__() $res_type; { $($body)* } [ { $left : &$crate::PyObject = {} } { $right : &$crate::PyObject = {} } ] }
        }
        $members $props
    }};

    { { def __lshift__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Invalid signature for binary numeric operator __lshift__" }
    };

    { { def __lt__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "__lt__ is not supported by py_class! use __richcmp__ instead." }
    };
    { { def __matmul__($left:ident, $right:ident) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_matrix_multiply: $crate::py_class_numeric_slot!(binary $class::__matmul__),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __matmul__() $res_type; { $($body)* } [ { $left : &$crate::PyObject = {} } { $right : &$crate::PyObject = {} } ] }
        }
        $members $props
    }};

    { { def __matmul__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Invalid signature for binary numeric operator __matmul__" }
    };
    { { def __mod__($left:ident, $right:ident) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_remainder: $crate::py_class_numeric_slot!(binary $class::__mod__),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __mod__() $res_type; { $($body)* } [ { $left : &$crate::PyObject = {} } { $right : &$crate::PyObject = {} } ] }
        }
        $members $props
    }};

    { { def __mod__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Invalid signature for binary numeric operator __mod__" }
    };
    { { def __mul__($left:ident, $right:ident) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_multiply: $crate::py_class_numeric_slot!(binary $class::__mul__),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __mul__() $res_type; { $($body)* } [ { $left : &$crate::PyObject = {} } { $right : &$crate::PyObject = {} } ] }
        }
        $members $props
    }};

    { { def __mul__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Invalid signature for binary numeric operator __mul__" }
    };

    { { def __ne__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "__ne__ is not supported by py_class! use __richcmp__ instead." }
    };
    { { def __neg__(&$slf:ident) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_negative: $crate::py_class_unary_slot!($class::__neg__, *mut $crate::_detail::ffi::PyObject, $crate::_detail::PyObjectCallbackConverter),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __neg__(&$slf,) $res_type; { $($body)* } [] }
        }
        $members $props
    }};

    { { def __neg__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Invalid signature for operator __neg__" }
    };
    { {   def __new__ ($cls:ident) -> $res_type:ty { $( $body:tt )* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            /* type_slots */ [ $( $tp_slot_name:ident : $tp_slot_value:expr, )* ]
            $as_number:tt $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            /* type_slots */ [
                $( $tp_slot_name : $tp_slot_value, )*
                tp_new: $crate::py_class_wrap_newfunc!{$class::__new__ []},
            ]
            $as_number $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __new__($cls: &$crate::PyType,) $res_type; { $($body)* } [] }
        }
        $members $props
    }};
    { {  $visibility:vis def __new__ ($cls:ident) -> $res_type:ty { $( $body:tt )* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            /* type_slots */ [ $( $tp_slot_name:ident : $tp_slot_value:expr, )* ]
            $as_number:tt $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            /* type_slots */ [
                $( $tp_slot_name : $tp_slot_value, )*
                tp_new: $crate::py_class_wrap_newfunc!{$class::__new__ []},
            ]
            $as_number $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, $visibility, __new__($cls: &$crate::PyType,) $res_type; { $($body)* } [] }
        }
        $members $props
    }};
    { {   def __new__ ($cls:ident, $($p:tt)+) -> $res_type:ty { $( $body:tt )* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            /* type_slots */ [ $( $tp_slot_name:ident : $tp_slot_value:expr, )* ]
            $as_number:tt $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            /* type_slots */ [
                $( $tp_slot_name : $tp_slot_value, )*
                tp_new: $crate::py_argparse_parse_plist_impl!{py_class_wrap_newfunc {$class::__new__} [] ($($p)+,)},
            ]
            $as_number $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_argparse_parse_plist_impl!{
                py_class_impl_item { $class, $py, pub, __new__($cls: &$crate::PyType,) $res_type; { $($body)* } }
                [] ($($p)+,)
            }
        }
        $members $props
    }};
    { {  $visibility:vis def __new__ ($cls:ident, $($p:tt)+) -> $res_type:ty { $( $body:tt )* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            /* type_slots */ [ $( $tp_slot_name:ident : $tp_slot_value:expr, )* ]
            $as_number:tt $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            /* type_slots */ [
                $( $tp_slot_name : $tp_slot_value, )*
                tp_new: $crate::py_argparse_parse_plist_impl!{py_class_wrap_newfunc {$class::__new__} [] ($($p)+,)},
            ]
            $as_number $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_argparse_parse_plist_impl!{
                py_class_impl_item { $class, $py, $visibility, __new__($cls: &$crate::PyType,) $res_type; { $($body)* } }
                [] ($($p)+,)
            }
        }
        $members $props
    }};
    { { def __next__(&$slf:ident) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            /* type_slots */ [ $( $tp_slot_name:ident : $tp_slot_value:expr, )* ]
            $as_number:tt $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            /* type_slots */ [
                $( $tp_slot_name : $tp_slot_value, )*
                tp_iternext: $crate::py_class_unary_slot!($class::__next__, *mut $crate::_detail::ffi::PyObject, $crate::py_class::slots::IterNextResultConverter),
            ]
            $as_number $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __next__(&$slf,) $res_type; { $($body)* } [] }
        }
        $members $props
    }};

    { { def __next__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Invalid signature for operator __next__" }
    };

    { { def __nonzero__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "__nonzero__ is not supported by py_class!; use the Python 3 spelling __bool__ instead." }
    };
    { { def __or__($left:ident, $right:ident) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_or: $crate::py_class_numeric_slot!(binary $class::__or__),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __or__() $res_type; { $($body)* } [ { $left : &$crate::PyObject = {} } { $right : &$crate::PyObject = {} } ] }
        }
        $members $props
    }};

    { { def __or__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Invalid signature for binary numeric operator __or__" }
    };
    { { def __pos__(&$slf:ident) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_positive: $crate::py_class_unary_slot!($class::__pos__, *mut $crate::_detail::ffi::PyObject, $crate::_detail::PyObjectCallbackConverter),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __pos__(&$slf,) $res_type; { $($body)* } [] }
        }
        $members $props
    }};

    { { def __pos__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Invalid signature for operator __pos__" }
    };
    { { def __pow__($left:ident, $right:ident, $ex:ident) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_power: $crate::py_class_numeric_slot!(ternary $class::__pow__),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __pow__() $res_type; { $($body)* } [ { $left : &$crate::PyObject = {} } { $right : &$crate::PyObject = {} } { $ex : &$crate::PyObject = {} } ] }
        }
        $members $props
    }};

    { { def __pow__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Invalid signature for ternary numeric operator __pow__" }
    };

    { { def __radd__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Reflected numeric operator __radd__ is not supported by py_class! Use __add__ instead!" }
    };

    { { def __rand__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Reflected numeric operator __rand__ is not supported by py_class! Use __and__ instead!" }
    };

    { { def __rdiv__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Reflected numeric operator __rdiv__ is not supported by py_class! Use __div__ instead!" }
    };

    { { def __rdivmod__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Reflected numeric operator __rdivmod__ is not supported by py_class! Use __divmod__ instead!" }
    };
    { { def __repr__(&$slf:ident) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            /* type_slots */ [ $( $tp_slot_name:ident : $tp_slot_value:expr, )* ]
            $as_number:tt $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            /* type_slots */ [
                $( $tp_slot_name : $tp_slot_value, )*
                tp_repr: $crate::py_class_unary_slot!($class::__repr__, *mut $crate::_detail::ffi::PyObject, $crate::_detail::PythonObjectCallbackConverter::<$crate::PyString>(std::marker::PhantomData)),
            ]
            $as_number $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __repr__(&$slf,) $res_type; { $($body)* } [] }
        }
        $members $props
    }};

    { { def __repr__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Invalid signature for operator __repr__" }
    };

    { { def __rfloordiv__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Reflected numeric operator __rfloordiv__ is not supported by py_class! Use __floordiv__ instead!" }
    };
    { { def __richcmp__(&$slf:ident, $other:ident : Option<&$other_name:ty>, $op:ident : $op_name:ty) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            /* type_slots */ [ $( $tp_slot_name:ident : $tp_slot_value:expr, )* ]
            $as_number:tt $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            /* type_slots */ [
                $( $tp_slot_name : $tp_slot_value, )*
                tp_richcompare: $crate::py_class_richcompare_slot!($class::__richcmp__, [Option<&$other_name>], *mut $crate::_detail::ffi::PyObject, $crate::_detail::PyObjectCallbackConverter),
            ]
            $as_number $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __richcmp__(&$slf,) $res_type; { $($body)* } [{ $other : Option<&$other_name> = {} } { $op : $op_name = {} }] }
        }
        $members $props
    }};
    { { def __richcmp__(&$slf:ident, $other:ident : &$other_name:ty, $op:ident : $op_name:ty) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            /* type_slots */ [ $( $tp_slot_name:ident : $tp_slot_value:expr, )* ]
            $as_number:tt $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            /* type_slots */ [
                $( $tp_slot_name : $tp_slot_value, )*
                tp_richcompare: $crate::py_class_richcompare_slot!($class::__richcmp__, [&$other_name], *mut $crate::_detail::ffi::PyObject, $crate::_detail::PyObjectCallbackConverter),
            ]
            $as_number $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __richcmp__(&$slf,) $res_type; { $($body)* } [{ $other : &$other_name = {} } { $op : $op_name = {} }] }
        }
        $members $props
    }};
    { { def __richcmp__(&$slf:ident, $other:ident : $other_name:ty, $op:ident : $op_name:ty) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            /* type_slots */ [ $( $tp_slot_name:ident : $tp_slot_value:expr, )* ]
            $as_number:tt $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            /* type_slots */ [
                $( $tp_slot_name : $tp_slot_value, )*
                tp_richcompare: $crate::py_class_richcompare_slot!($class::__richcmp__, [$other_name], *mut $crate::_detail::ffi::PyObject, $crate::_detail::PyObjectCallbackConverter),
            ]
            $as_number $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __richcmp__(&$slf,) $res_type; { $($body)* } [{ $other : $other_name = {} } { $op : $op_name = {} }] }
        }
        $members $props
    }};

    { { def __richcmp__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Invalid signature for operator __richcmp__" }
    };

    { { def __rlshift__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Reflected numeric operator __rlshift__ is not supported by py_class! Use __lshift__ instead!" }
    };

    { { def __rmatmul__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Reflected numeric operator __rmatmul__ is not supported by py_class! Use __matmul__ instead!" }
    };

    { { def __rmod__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Reflected numeric operator __rmod__ is not supported by py_class! Use __mod__ instead!" }
    };

    { { def __rmul__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Reflected numeric operator __rmul__ is not supported by py_class! Use __mul__ instead!" }
    };

    { { def __ror__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Reflected numeric operator __ror__ is not supported by py_class! Use __or__ instead!" }
    };

    { { def __round__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "__round__ is not supported by py_class! yet." }
    };

    { { def __rpow__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Reflected numeric operator __rpow__ is not supported by py_class! Use __pow__ instead!" }
    };

    { { def __rrshift__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Reflected numeric operator __rrshift__ is not supported by py_class! Use __rshift__ instead!" }
    };
    { { def __rshift__($left:ident, $right:ident) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_rshift: $crate::py_class_numeric_slot!(binary $class::__rshift__),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __rshift__() $res_type; { $($body)* } [ { $left : &$crate::PyObject = {} } { $right : &$crate::PyObject = {} } ] }
        }
        $members $props
    }};

    { { def __rshift__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Invalid signature for binary numeric operator __rshift__" }
    };

    { { def __rsub__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Reflected numeric operator __rsub__ is not supported by py_class! Use __sub__ instead!" }
    };

    { { def __rtruediv__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Reflected numeric operator __rtruediv__ is not supported by py_class! Use __truediv__ instead!" }
    };

    { { def __rxor__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Reflected numeric operator __rxor__ is not supported by py_class! Use __xor__ instead!" }
    };

    { { def __set__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "__set__ is not supported by py_class! yet." }
    };

    { { def __setattr__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "__setattr__ is not supported by py_class! yet." }
    };
    { { def __setitem__(&$slf:ident, $key:ident : Option<&$key_name:ty>, $value:ident : $value_name:ty) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt $as_number:tt $as_sequence:tt $as_mapping:tt
            /* setdelitem */ [
                sdi_setitem: {},
                sdi_delitem: $sdi_delitem_slot_value:tt,
            ]
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots $as_number $as_sequence $as_mapping
            /* setdelitem */ [
                sdi_setitem: { $crate::py_class_ternary_slot!($class::__setitem__, [Option<&$key_name>], $value_name, $crate::_detail::libc::c_int, $crate::py_class::slots::UnitCallbackConverter) },
                sdi_delitem: $sdi_delitem_slot_value,
            ]
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __setitem__(&$slf,) $res_type; { $($body)* } [{ $key : Option<&$key_name> = {} } { $value : $value_name = {} }] }
        }
        $members $props
    }};
    { { def __setitem__(&$slf:ident, $key:ident : &$key_name:ty, $value:ident : $value_name:ty) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt $as_number:tt $as_sequence:tt $as_mapping:tt
            /* setdelitem */ [
                sdi_setitem: {},
                sdi_delitem: $sdi_delitem_slot_value:tt,
            ]
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots $as_number $as_sequence $as_mapping
            /* setdelitem */ [
                sdi_setitem: { $crate::py_class_ternary_slot!($class::__setitem__, [&$key_name], $value_name, $crate::_detail::libc::c_int, $crate::py_class::slots::UnitCallbackConverter) },
                sdi_delitem: $sdi_delitem_slot_value,
            ]
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __setitem__(&$slf,) $res_type; { $($body)* } [{ $key : &$key_name = {} } { $value : $value_name = {} }] }
        }
        $members $props
    }};
    { { def __setitem__(&$slf:ident, $key:ident : $key_name:ty, $value:ident : $value_name:ty) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt $as_number:tt $as_sequence:tt $as_mapping:tt
            /* setdelitem */ [
                sdi_setitem: {},
                sdi_delitem: $sdi_delitem_slot_value:tt,
            ]
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots $as_number $as_sequence $as_mapping
            /* setdelitem */ [
                sdi_setitem: { $crate::py_class_ternary_slot!($class::__setitem__, [$key_name], $value_name, $crate::_detail::libc::c_int, $crate::py_class::slots::UnitCallbackConverter) },
                sdi_delitem: $sdi_delitem_slot_value,
            ]
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __setitem__(&$slf,) $res_type; { $($body)* } [{ $key : $key_name = {} } { $value : $value_name = {} }] }
        }
        $members $props
    }};

    { { def __setitem__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Invalid signature for operator __setitem__" }
    };
    { { def __str__(&$slf:ident) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            /* type_slots */ [ $( $tp_slot_name:ident : $tp_slot_value:expr, )* ]
            $as_number:tt $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            /* type_slots */ [
                $( $tp_slot_name : $tp_slot_value, )*
                tp_str: $crate::py_class_unary_slot!($class::__str__, *mut $crate::_detail::ffi::PyObject, $crate::_detail::PythonObjectCallbackConverter::<$crate::PyString>(std::marker::PhantomData)),
            ]
            $as_number $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __str__(&$slf,) $res_type; { $($body)* } [] }
        }
        $members $props
    }};

    { { def __str__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Invalid signature for operator __str__" }
    };
    { { def __sub__($left:ident, $right:ident) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_subtract: $crate::py_class_numeric_slot!(binary $class::__sub__),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __sub__() $res_type; { $($body)* } [ { $left : &$crate::PyObject = {} } { $right : &$crate::PyObject = {} } ] }
        }
        $members $props
    }};

    { { def __sub__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Invalid signature for binary numeric operator __sub__" }
    };

    { { def __subclasscheck__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "__subclasscheck__ is not supported by py_class! yet." }
    };
    { { def __truediv__($left:ident, $right:ident) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_true_divide: $crate::py_class_numeric_slot!(binary $class::__truediv__),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __truediv__() $res_type; { $($body)* } [ { $left : &$crate::PyObject = {} } { $right : &$crate::PyObject = {} } ] }
        }
        $members $props
    }};

    { { def __truediv__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Invalid signature for binary numeric operator __truediv__" }
    };
    { { def __xor__($left:ident, $right:ident) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_xor: $crate::py_class_numeric_slot!(binary $class::__xor__),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __xor__() $res_type; { $($body)* } [ { $left : &$crate::PyObject = {} } { $right : &$crate::PyObject = {} } ] }
        }
        $members $props
    }};

    { { def __xor__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Invalid signature for binary numeric operator __xor__" }
    };
    { { $(#[doc=$doc:expr])*  def $name:ident (&$slf:ident) -> $res_type:ty { $( $body:tt )* } $($tail:tt)* }
        $class:ident $py:ident $info:tt $slots:tt
        { $( $imp:item )* }
        { $( $member_name:ident = $member_expr:expr; )* } $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info $slots
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, $name(&$slf,) $res_type; { $($body)* } [] }
        }
        /* members: */ {
            $( $member_name = $member_expr; )*
            $name = $crate::py_class_instance_method!{$py, $class::$name, { concat!($($doc, "\n"),*) } []};
        } $props
    }};
    { { $(#[doc=$doc:expr])* $visibility:vis def $name:ident (&$slf:ident) -> $res_type:ty { $( $body:tt )* } $($tail:tt)* }
        $class:ident $py:ident $info:tt $slots:tt
        { $( $imp:item )* }
        { $( $member_name:ident = $member_expr:expr; )* } $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info $slots
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, $visibility, $name(&$slf,) $res_type; { $($body)* } [] }
        }
        /* members: */ {
            $( $member_name = $member_expr; )*
            $name = $crate::py_class_instance_method!{$py, $class::$name, { concat!($($doc, "\n"),*) } []};
        } $props
    }};
    { { $(#[doc=$doc:expr])*  def $name:ident (&$slf:ident, $($p:tt)+) -> $res_type:ty { $( $body:tt )* } $($tail:tt)* }
        $class:ident $py:ident $info:tt $slots:tt
        { $( $imp:item )* }
        { $( $member_name:ident = $member_expr:expr; )* } $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info $slots
        /* impl: */ {
            $($imp)*
            $crate::py_argparse_parse_plist_impl!{
                py_class_impl_item { $class, $py, pub, $name(&$slf,) $res_type; { $($body)* } }
                [] ($($p)+,)
            }
        }
        /* members: */ {
            $( $member_name = $member_expr; )*
            $name = $crate::py_argparse_parse_plist_impl!{py_class_instance_method {$py, $class::$name, { concat!($($doc, "\n"),*) }} [] ($($p)+,)};
        } $props
    }};
    { { $(#[doc=$doc:expr])* $visibility:vis def $name:ident (&$slf:ident, $($p:tt)+) -> $res_type:ty { $( $body:tt )* } $($tail:tt)* }
        $class:ident $py:ident $info:tt $slots:tt
        { $( $imp:item )* }
        { $( $member_name:ident = $member_expr:expr; )* } $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info $slots
        /* impl: */ {
            $($imp)*
            $crate::py_argparse_parse_plist_impl!{
                py_class_impl_item { $class, $py, $visibility, $name(&$slf,) $res_type; { $($body)* } }
                [] ($($p)+,)
            }
        }
        /* members: */ {
            $( $member_name = $member_expr; )*
            $name = $crate::py_argparse_parse_plist_impl!{py_class_instance_method {$py, $class::$name, { concat!($($doc, "\n"),*) }} [] ($($p)+,)};
        } $props
    }};
    { { $(#[doc=$doc:expr])*@classmethod  def $name:ident ($cls:ident) -> $res_type:ty { $( $body:tt )* } $($tail:tt)* }
        $class:ident $py:ident $info:tt $slots:tt
        { $( $imp:item )* }
        { $( $member_name:ident = $member_expr:expr; )* } $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info $slots
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, $name($cls: &$crate::PyType,) $res_type; { $($body)* } [] }
        }
        /* members: */ {
            $( $member_name = $member_expr; )*
            $name = $crate::py_class_class_method!{$py, $class::$name, { concat!($($doc, "\n"),*) } []};
        } $props
    }};
    { { $(#[doc=$doc:expr])*@classmethod $visibility:vis def $name:ident ($cls:ident) -> $res_type:ty { $( $body:tt )* } $($tail:tt)* }
        $class:ident $py:ident $info:tt $slots:tt
        { $( $imp:item )* }
        { $( $member_name:ident = $member_expr:expr; )* } $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info $slots
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, $visibility, $name($cls: &$crate::PyType,) $res_type; { $($body)* } [] }
        }
        /* members: */ {
            $( $member_name = $member_expr; )*
            $name = $crate::py_class_class_method!{$py, $class::$name, { concat!($($doc, "\n"),*) } []};
        } $props
    }};
    { { $(#[doc=$doc:expr])*@classmethod  def $name:ident ($cls:ident, $($p:tt)+) -> $res_type:ty { $( $body:tt )* } $($tail:tt)* }
        $class:ident $py:ident $info:tt $slots:tt
        { $( $imp:item )* }
        { $( $member_name:ident = $member_expr:expr; )* } $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info $slots
        /* impl: */ {
            $($imp)*
            $crate::py_argparse_parse_plist_impl!{
                py_class_impl_item { $class, $py, pub, $name($cls: &$crate::PyType,) $res_type; { $($body)* } }
                [] ($($p)+,)
            }
        }
        /* members: */ {
            $( $member_name = $member_expr; )*
            $name = $crate::py_argparse_parse_plist_impl!{py_class_class_method {$py, $class::$name, { concat!($($doc, "\n"),*) }} [] ($($p)+,)};
        } $props
    }};
    { { $(#[doc=$doc:expr])*@classmethod $visibility:vis def $name:ident ($cls:ident, $($p:tt)+) -> $res_type:ty { $( $body:tt )* } $($tail:tt)* }
        $class:ident $py:ident $info:tt $slots:tt
        { $( $imp:item )* }
        { $( $member_name:ident = $member_expr:expr; )* } $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info $slots
        /* impl: */ {
            $($imp)*
            $crate::py_argparse_parse_plist_impl!{
                py_class_impl_item { $class, $py, $visibility, $name($cls: &$crate::PyType,) $res_type; { $($body)* } }
                [] ($($p)+,)
            }
        }
        /* members: */ {
            $( $member_name = $member_expr; )*
            $name = $crate::py_argparse_parse_plist_impl!{py_class_class_method {$py, $class::$name, { concat!($($doc, "\n"),*) }} [] ($($p)+,)};
        } $props
    }};
    { { $(#[doc=$doc:expr])* @staticmethod  def $name:ident ($($p:tt)*) -> $res_type:ty { $( $body:tt )* } $($tail:tt)* }
        $class:ident $py:ident $info:tt $slots:tt
        { $( $imp:item )* }
        { $( $member_name:ident = $member_expr:expr; )* } $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info $slots
        /* impl: */ {
            $($imp)*
            $crate::py_argparse_parse_plist!{
                py_class_impl_item { $class, $py, pub, $name() $res_type; { $($body)* } }
                ($($p)*)
            }
        }
        /* members: */ {
            $( $member_name = $member_expr; )*
            $name = 
            $crate::py_argparse_parse_plist!{
                py_class_static_method {$py, $class::$name, {
                    concat!($($doc, "\n"),*)
                    } }
                ($($p)*)
            }
            ;
        } $props
    }};
    { { $(#[doc=$doc:expr])* @staticmethod $visibility:vis def $name:ident ($($p:tt)*) -> $res_type:ty { $( $body:tt )* } $($tail:tt)* }
        $class:ident $py:ident $info:tt $slots:tt
        { $( $imp:item )* }
        { $( $member_name:ident = $member_expr:expr; )* } $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info $slots
        /* impl: */ {
            $($imp)*
            $crate::py_argparse_parse_plist!{
                py_class_impl_item { $class, $py, $visibility, $name() $res_type; { $($body)* } }
                ($($p)*)
            }
        }
        /* members: */ {
            $( $member_name = $member_expr; )*
            $name = 
            $crate::py_argparse_parse_plist!{
                py_class_static_method {$py, $class::$name, {
                    concat!($($doc, "\n"),*)
                    } }
                ($($p)*)
            }
            ;
        } $props
    }};
    { { static $name:ident = $init:expr; $($tail:tt)* }
        $class:ident $py:ident $info:tt $slots:tt $impls:tt
        { $( $member_name:ident = $member_expr:expr; )* } $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info $slots $impls
        /* members: */ {
            $( $member_name = $member_expr; )*
            $name = $init;
        } $props
    }};
    { { $(#[doc=$doc:expr])* @property  def $name:ident(&$slf:ident) -> $res_type:ty { $( $body:tt )* } $($tail:tt)* }
        $class:ident $py:ident $info:tt $slots:tt
        { $( $imp:item )* }
        $members:tt
        { [ $( $prop_doc:tt $prop_getter_name:ident: $prop_type:ty, )* ]
            [ $( $prop_setter_name:ident : $prop_setter_value_type:tt => $prop_setter_setter:ident, )* ] }
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info $slots
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, $name(&$slf,) $res_type; { $($body)* } [] }
        }
        $members
        /* props: */ {
            [ $( $prop_doc $prop_getter_name: $prop_type, )*
                { concat!($($doc, "\n"),*) } $name: $res_type,
            ]
            [ $( $prop_setter_name : $prop_setter_value_type => $prop_setter_setter, )*
            ]
        }
    }};
    { { @$name:ident.setter  def $setter_name:ident(&$slf:ident, $value:ident : Option<Option<&$value_type:ty>> ) -> $res_type:ty { $( $body:tt )* } $($tail:tt)* }
        $class:ident $py:ident $info:tt $slots:tt
        { $( $imp:item )* }
        $members:tt
        { [ $( $prop_doc:tt $prop_getter_name:ident: $prop_type:ty, )* ]
            [ $( $prop_setter_name:ident : $prop_setter_value_type:tt => $prop_setter_setter:ident, )* ] }
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info $slots
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, $setter_name(&$slf,) $res_type; { $($body)* } [{ $value: Option<Option<&$value_type>> = {} }] }
        }
        $members
        /* props: */ {
            [ $( $prop_doc $prop_getter_name: $prop_type, )*
            ]
            [ $( $prop_setter_name : $prop_setter_value_type => $prop_setter_setter, )*
                $name : [ Option<&$value_type> ] => $setter_name,
            ]
        }
    }};
    { { @$name:ident.setter  def $setter_name:ident(&$slf:ident, $value:ident : Option<&$value_type:ty> ) -> $res_type:ty { $( $body:tt )* } $($tail:tt)* }
        $class:ident $py:ident $info:tt $slots:tt
        { $( $imp:item )* }
        $members:tt
        { [ $( $prop_doc:tt $prop_getter_name:ident: $prop_type:ty, )* ]
            [ $( $prop_setter_name:ident : $prop_setter_value_type:tt => $prop_setter_setter:ident, )* ] }
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info $slots
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, $setter_name(&$slf,) $res_type; { $($body)* } [{ $value: Option<&$value_type> = {} }] }
        }
        $members
        /* props: */ {
            [ $( $prop_doc $prop_getter_name: $prop_type, )*
            ]
            [ $( $prop_setter_name : $prop_setter_value_type => $prop_setter_setter, )*
                $name : [ &$value_type ] => $setter_name,
            ]
        }
    }};
    { { @$name:ident.setter  def $setter_name:ident(&$slf:ident, $value:ident : Option<$value_type:ty> ) -> $res_type:ty { $( $body:tt )* } $($tail:tt)* }
        $class:ident $py:ident $info:tt $slots:tt
        { $( $imp:item )* }
        $members:tt
        { [ $( $prop_doc:tt $prop_getter_name:ident: $prop_type:ty, )* ]
            [ $( $prop_setter_name:ident : $prop_setter_value_type:tt => $prop_setter_setter:ident, )* ] }
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info $slots
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, $setter_name(&$slf,) $res_type; { $($body)* } [{ $value: Option<$value_type> = {} }] }
        }
        $members
        /* props: */ {
            [ $( $prop_doc $prop_getter_name: $prop_type, )*
            ]
            [ $( $prop_setter_name : $prop_setter_value_type => $prop_setter_setter, )*
                $name : [ $value_type ] => $setter_name,
            ]
        }
    }};
    { { $(#[doc=$doc:expr])* @property $visibility:vis def $name:ident(&$slf:ident) -> $res_type:ty { $( $body:tt )* } $($tail:tt)* }
        $class:ident $py:ident $info:tt $slots:tt
        { $( $imp:item )* }
        $members:tt
        { [ $( $prop_doc:tt $prop_getter_name:ident: $prop_type:ty, )* ]
            [ $( $prop_setter_name:ident : $prop_setter_value_type:tt => $prop_setter_setter:ident, )* ] }
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info $slots
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, $visibility, $name(&$slf,) $res_type; { $($body)* } [] }
        }
        $members
        /* props: */ {
            [ $( $prop_doc $prop_getter_name: $prop_type, )*
                { concat!($($doc, "\n"),*) } $name: $res_type,
            ]
            [ $( $prop_setter_name : $prop_setter_value_type => $prop_setter_setter, )*
            ]
        }
    }};
    { { @$name:ident.setter $visibility:vis def $setter_name:ident(&$slf:ident, $value:ident : Option<Option<&$value_type:ty>> ) -> $res_type:ty { $( $body:tt )* } $($tail:tt)* }
        $class:ident $py:ident $info:tt $slots:tt
        { $( $imp:item )* }
        $members:tt
        { [ $( $prop_doc:tt $prop_getter_name:ident: $prop_type:ty, )* ]
            [ $( $prop_setter_name:ident : $prop_setter_value_type:tt => $prop_setter_setter:ident, )* ] }
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info $slots
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, $visibility, $setter_name(&$slf,) $res_type; { $($body)* } [{ $value: Option<Option<&$value_type>> = {} }] }
        }
        $members
        /* props: */ {
            [ $( $prop_doc $prop_getter_name: $prop_type, )*
            ]
            [ $( $prop_setter_name : $prop_setter_value_type => $prop_setter_setter, )*
                $name : [ Option<&$value_type> ] => $setter_name,
            ]
        }
    }};
    { { @$name:ident.setter $visibility:vis def $setter_name:ident(&$slf:ident, $value:ident : Option<&$value_type:ty> ) -> $res_type:ty { $( $body:tt )* } $($tail:tt)* }
        $class:ident $py:ident $info:tt $slots:tt
        { $( $imp:item )* }
        $members:tt
        { [ $( $prop_doc:tt $prop_getter_name:ident: $prop_type:ty, )* ]
            [ $( $prop_setter_name:ident : $prop_setter_value_type:tt => $prop_setter_setter:ident, )* ] }
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info $slots
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, $visibility, $setter_name(&$slf,) $res_type; { $($body)* } [{ $value: Option<&$value_type> = {} }] }
        }
        $members
        /* props: */ {
            [ $( $prop_doc $prop_getter_name: $prop_type, )*
            ]
            [ $( $prop_setter_name : $prop_setter_value_type => $prop_setter_setter, )*
                $name : [ &$value_type ] => $setter_name,
            ]
        }
    }};
    { { @$name:ident.setter $visibility:vis def $setter_name:ident(&$slf:ident, $value:ident : Option<$value_type:ty> ) -> $res_type:ty { $( $body:tt )* } $($tail:tt)* }
        $class:ident $py:ident $info:tt $slots:tt
        { $( $imp:item )* }
        $members:tt
        { [ $( $prop_doc:tt $prop_getter_name:ident: $prop_type:ty, )* ]
            [ $( $prop_setter_name:ident : $prop_setter_value_type:tt => $prop_setter_setter:ident, )* ] }
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info $slots
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, $visibility, $setter_name(&$slf,) $res_type; { $($body)* } [{ $value: Option<$value_type> = {} }] }
        }
        $members
        /* props: */ {
            [ $( $prop_doc $prop_getter_name: $prop_type, )*
            ]
            [ $( $prop_setter_name : $prop_setter_value_type => $prop_setter_setter, )*
                $name : [ $value_type ] => $setter_name,
            ]
        }
    }};

}

