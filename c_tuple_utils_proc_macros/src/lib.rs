extern crate quote;

use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, TokenStreamExt};
use syn::Index;

const MAX_TUPLE_SIZE: usize = 4;

#[proc_macro]
pub fn tuple_ext_impl(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut generated = TokenStream::new();
    generated.append_all(quote!(
        pub trait TupleExt: Sealed {
            type Ref<'a>
            where
                Self: 'a;
            fn propagate_reference(&self) -> Self::Ref<'_>;
        }

        pub trait Pluck: Sealed {
            type Head;
            type Tail;
            fn pluck(self) -> (Self::Head, Self::Tail);
        }

        pub trait Mapper<T> {
            type Output;
            fn map(value: T) -> Self::Output;
        }
        pub trait MapTuple<M>: Sealed {
            fn map(self, _: M) -> Self::Output;
            type Output;
        }

        pub trait Reducer<T, Acc> {
            fn reduce(current: T, accumulated: Acc) -> Acc;
        }
        pub trait ReduceTuple<R, Acc>: Sealed {
            fn reduce(self, initial: Acc, _: R) -> Acc;
        }

        mod sealed {
            pub trait Sealed {}
        }
        use sealed::Sealed;
    ));
    for i in 0..=MAX_TUPLE_SIZE {
        generated.append_all(generate_sealed(i));
        generated.append_all(generate_pluck(i));
        generated.append_all(generate_map(i));
        generated.append_all(generate_reduce(i));
    }
    generated.into()
}

fn generate_sealed(i: usize) -> TokenStream {
    // TODO: in the future, this will be unnecessary thanks to the Tuple trait
    let types: Vec<_> = (0..i).map(type_ident).collect();

    quote!(
        impl< #(#types, )* > Sealed for ( #(#types, )* ) {}
    )
}

fn generate_tuple_ext(i: usize) -> TokenStream {
    let types: Vec<_> = (0..i).map(type_ident).collect();
    let fields: Vec<_> = (0..i).map(Index::from).collect();

    quote!(
        impl< #(#types,)* > TupleExt for ( #(#types,)* ) {
            type Ref<'a>
            where
                Self: 'a
            = ( #(&'a #types,)* );
            fn propagate_reference(&self) -> Self::Ref<'_> {
                ( #(&self.#fields,)* )
            }
        }
    )
}

fn generate_pluck(i: usize) -> TokenStream {
    if i == 0 {
        quote!(
            impl Pluck for () {
                type Head = ();
                type Tail = ();
                fn pluck(self) -> (Self::Head, Self::Tail) {
                    ((), ())
                }
            }
        )
    } else {
        let head = type_ident(0);
        let tail: Vec<_> = (1..i).map(type_ident).collect();
        let fields: Vec<_> = (1..i).map(Index::from).collect();
        quote!(
            impl< #head, #(#tail, )* > Pluck for ( #head, #(#tail, )* ) {
                type Head = #head;
                type Tail = ( #(#tail, )* );
                fn pluck(self) -> (Self::Head, Self::Tail) {
                    (
                        self.0,
                        ( #(self.#fields,)* )
                    )
                }
            }
        )
    }
}

fn generate_map(i: usize) -> TokenStream {
    if i == 0 {
        quote!(
            impl<M> MapTuple<M> for () {
                type Output = ();
                fn map(self, _: M) -> Self::Output {
                    ()
                }
            }
        )
    } else {
        let types: Vec<_> = (0..i).map(type_ident).collect();
        let fields: Vec<_> = (0..i).map(Index::from).collect();
        quote!(
            impl<M, #(#types, )* > MapTuple<M> for ( #(#types, )* )
            where
                #(M: Mapper<#types>,)*
            {
                type Output = ( #(<M as Mapper<#types>>::Output,)* );
                fn map(self, _: M) -> Self::Output {
                    (
                        #(M::map(self.#fields),)*
                    )
                }
            }
        )
    }
}

fn generate_reduce(i: usize) -> TokenStream {
    if i == 0 {
        quote!(
            impl<R, Acc> ReduceTuple<R, Acc> for () {
                fn reduce(self, initial: Acc, _: R) -> Acc {
                    initial
                }
            }
        )
    } else if i == 1 {
        quote!(
            impl<R, Acc, T0> ReduceTuple<R, Acc> for (T0,)
            where
                R: Reducer<T0, Acc>,
            {
                fn reduce(self, initial: Acc, _: R) -> Acc {
                    R::reduce(self.0, initial)
                }
            }
        )
    } else {
        let types: Vec<_> = (0..i).map(type_ident).collect();
        quote!(
            impl<R, Acc, #(#types, )* > ReduceTuple<R, Acc> for ( #(#types, )* )
            where
                #(R: Reducer<#types, Acc>,)*
            {
                fn reduce(self, initial: Acc, reducer: R) -> Acc {
                    let (head, tail) = self.pluck();
                    tail.reduce(R::reduce(head, initial), reducer)
                }
            }
        )
    }
}

fn type_ident(n: usize) -> Ident {
    Ident::new(&format!("T{}", n), Span::call_site())
}
