use std::error::Error;

pub trait Pool<R: Resource, I: Info<R>> {
    type Lease;

    /// Leases a resource from the pool
    ///
    /// * `info`: information passed to `Resource::create`
    /// * `ctx`: context passed to `Resource::create`
    fn lease(&mut self, info: &I, ctx: &I::Context) -> Self::Lease {
        self.try_lease(info, ctx).unwrap()
    }

    fn try_lease(&mut self, info: &I, ctx: &I::Context) -> Option<Self::Lease>;
}
// pub trait TryPool<R: TryResource>: Pool<R> {
//     fn try_lease(&mut self, info: &R::Info, ctx: &R::Context) -> Result<Self::Lease, R::Error>;
// }

pub trait Info<R: Resource>: Eq + PartialEq + Clone {
    type Context;

    fn create(info: &Self, ctx: &Self::Context) -> R {
        Self::try_create(info, ctx).unwrap()
    }

    fn try_create(info: &Self, ctx: &Self::Context) -> Option<R>;
}

pub trait Resource {
    fn clear(&mut self);
}

// pub trait Resource {
//     type Info: Eq + PartialEq + Clone;
//     type Context;
//
//     /// Creates a resource with the specified creation info.
//     ///
//     /// * `info`: information such as size for creating the resource
//     /// * `ctx`: context such as a vulkan device used to create the resource
//     fn create(info: &Self::Info, ctx: &Self::Context) -> Self;
//
//     /// Clears the resource, to drop its content.
//     /// For a vector this should call `Vec::clear()`.
//     fn clear(&mut self);
// }
//
// pub trait TryResource: Resource + Sized {
//     type Error: Error;
//     /// Creates a resource with the specified creation info or an result if that failed.
//     ///
//     /// * `info`: information such as size for creating the resource
//     /// * `ctx`: context such as a vulkan device used to create the resource
//     fn try_create(info: &Self::Info, ctx: &Self::Context) -> Result<Self, Self::Error>;
// }
