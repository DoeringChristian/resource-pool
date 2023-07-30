use std::error::Error;

pub trait Pool<R: Resource> {
    type Lease;

    /// Leases a resource from the pool
    ///
    /// * `info`: information passed to `Resource::create`
    /// * `ctx`: context passed to `Resource::create`
    fn lease(&mut self, info: &R::Info, ctx: &R::Context) -> Self::Lease;
}
pub trait TryPool<R: TryResource>: Pool<R> {
    fn try_lease(&mut self, info: &R::Info, ctx: &R::Context) -> Result<Self::Lease, R::Error>;
}

pub trait Resource {
    type Info: Eq + PartialEq + Clone;
    type Context;
    ///
    /// The weak form of a `Resource` is the form how it should be stored in the pool.
    /// For example, if it would hold a reference counting pointer (`Rc` or `Arc`) to the pool this
    /// could be replaced by a `Weak` pointer.
    ///
    type WeakForm;

    /// Creates a resource with the specified creation info.
    ///
    /// * `info`: information such as size for creating the resource
    /// * `ctx`: context such as a vulkan device used to create the resource
    fn create(info: &Self::Info, ctx: &Self::Context) -> Self;

    /// Clears the resource to drop it's content and returns the weak form.
    /// For a vector this should call `Vec::clear()`.
    fn clear(self) -> Self::WeakForm;

    ///
    /// Upgrades a resource from its `WeakForm`.
    ///
    fn upgrade(weak: Self::WeakForm, info: &Self::Info, ctx: &Self::Context) -> Self;
}

pub trait TryResource: Resource + Sized {
    type Error: Error;
    /// Creates a resource with the specified creation info or an result if that failed.
    ///
    /// * `info`: information such as size for creating the resource
    /// * `ctx`: context such as a vulkan device used to create the resource
    fn try_create(info: &Self::Info, ctx: &Self::Context) -> Result<Self, Self::Error>;
}
