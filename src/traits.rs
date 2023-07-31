use std::error::Error;

pub trait Pool<I: Info> {
    type Lease;

    /// Leases a resource from the pool, panics if the resource could not be created.
    ///
    /// * `info`: information passed to `I::try_create`
    /// * `ctx`: context passed to `I::try_create`
    fn lease(&mut self, info: &I, ctx: &I::Context) -> Self::Lease {
        self.try_lease(info, ctx).unwrap()
    }

    /// Try to lease a resource from the pool.
    /// Returns None if the resource could not be created.
    ///
    /// * `info`: Information passed to `I::try_create`
    /// * `ctx`: cotext passsed to `I::try_create`
    fn try_lease(&mut self, info: &I, ctx: &I::Context) -> Option<Self::Lease>;
}

pub trait Info: Eq + PartialEq + Clone {
    type Context;
    type Resource: Resource;

    fn try_create(info: &Self, ctx: &Self::Context) -> Option<Self::Resource>;
}

pub trait Resource {
    fn clear(&mut self);
}
