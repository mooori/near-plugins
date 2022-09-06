use near_sdk::AccountId;

/// # Representation of roles
///
/// This trait is unaware of the concrete type used to represent roles. It is
/// not possible to use a generic type `R` since `near-sdk` [does not support]
/// `impl` type parameters.
///
/// ```
/// // Hence this is not possible:
/// impl<R> AccessControllable<R> for /// Contract {/* ... */}
/// ```
///
/// Instead, roles are represented by `u8`, which allows contract developers to
/// define their own type (convertible to `u8`). We recommend a fieldless enum,
/// for which conversion to `u8` is trivial.
///
/// [does not support]: https://github.com/near/near-sdk-rs/blob/9d99077c6acfde68c06845f2a1eb2b5ed7983401/near-sdk/compilation_tests/impl_generic.stderr
pub trait AccessControllable {
    fn acl_storage_prefix(&self) -> &[u8];

    fn acl_has_role(&self, role: u8, account_id: AccountId) -> bool;
}
