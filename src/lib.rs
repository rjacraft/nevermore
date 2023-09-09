//! Conveniences for the bottom type [`core::convert::Infallible`].
//!
//! This may become redundant if (hopefully *once*)
//! never type is fully [stabilized][never-type-tracking-issue].
//!
//! [never-type-tracking-issue]: https://github.com/rust-lang/rust/issues/35121

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Implements conversion [from][`From`] [`core::convert::Infallible`] for the struct.
///
/// The common use-case is to use it on custom error types in APIs
/// so that a generic interface can return the result of an arbitrary associated type
/// but be easily convertible into a more specific type.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use nevermore::FromNever;
///
/// fn assert_from_never<T: From<core::convert::Infallible>>() {}
///
/// #[derive(FromNever)]
/// pub struct User {
///     username: String,
///     age: u8,
/// }
///
/// assert_from_never::<User>();
/// ```
///
/// Error type polymorphism:
///
/// ```
/// use std::convert::Infallible;
/// use std::io::{self, Read};
/// use nevermore::FromNever;
///
/// /// A type which may be decoded from source input.
/// trait Decode {
///     type Error;
///
///     /// Decodes the packet.
///     fn decode(read: impl Read) -> Result<Self, Self::Error>
///         where Self: Sized;
/// }
///
/// /// Foo packet.
/// pub struct Foo(i32);
///
/// impl Decode for Foo {
///     type Error = FooDecodeError;
///
///     fn decode(mut read: impl Read) -> Result<Self, Self::Error> {
///         let mut buffer = [0; 4];
///         read.read_exact(&mut buffer)?;
///
///         Ok(Self(i32::from_be_bytes(buffer)))
///     }
/// }
///
/// /// Bar packet. This is empty thus its deserialization cannot fail
/// /// since it does no require any bytes to be read.
/// pub struct Bar;
///
/// impl Decode for Bar {
///     // Although the trait permits us to fail, we always succeed.
///     type Error = Infallible;
///
///     fn decode(read: impl Read) -> Result<Self, Self::Error> {
///         Ok(Self)
///     }
/// }
///
/// #[derive(thiserror::Error, Debug)]
/// pub enum FooDecodeError {
///     #[error("an I/O error has occurred {0}")]
///     Io(#[from] io::Error),
/// }
///
/// /// An error which may appear while decoding either `Foo` or `Bar` packet.
/// #[derive(thiserror::Error, FromNever, Debug)]
/// pub enum PacketDecodeError {
///     #[error("failed to deserialize Foo packet")]
///     Foo(#[from] <Foo as Decode>::Error),
///     // there is no need for a separate uninhabited type now since
///     // we've made this type implement `From<Error>`
/// }
///
/// fn decode_foo_bar(mut buffer: impl Read) -> Result<(Foo, Bar), PacketDecodeError> {
///     // we use `?` with both calls to `decode`:
///     Ok((
///         // will use thiserror-generated `From<FooDecodeError>`
///         Foo::decode(&mut buffer)?,
///         // will use `From<Infallible>`
///         Bar::decode(&mut buffer)?,
///     ))
/// }
///
/// let mut buffer = vec![0x12u8, 0x34u8, 0x56u8, 0x78u8];
/// # let _ =
/// decode_foo_bar(&*buffer.as_mut_slice());
/// ```
#[proc_macro_derive(FromNever)]
pub fn derive_from_never(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident, generics, ..
    } = parse_macro_input!(input as DeriveInput);

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    proc_macro::TokenStream::from(quote!(
        impl #impl_generics ::core::convert::From<::core::convert::Infallible>
            for #ident #ty_generics #where_clause {
            fn from(infallible: ::core::convert::Infallible) -> Self {
                match infallible {}
            }
        }
    ))
}
