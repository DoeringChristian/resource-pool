use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};

pub use crate::traits::*;

type Cache<R> = Arc<Mutex<Vec<R>>>;

pub struct Lease<R: Resource> {
    resource: Option<R>,
    cache: Cache<R>,
}
impl<R: Resource + Debug> Debug for Lease<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Lease")
            .field("resource", &self.resource)
            .field("cache", &self.cache)
            .finish()
    }
}

impl<R: Resource> Drop for Lease<R> {
    fn drop(&mut self) {
        let mut resource = self.resource.take().unwrap();
        resource.clear();
        self.cache.lock().unwrap().push(resource);
    }
}

impl<R: Resource> Deref for Lease<R> {
    type Target = R;

    fn deref(&self) -> &Self::Target {
        self.resource.as_ref().unwrap()
    }
}
impl<R: Resource> DerefMut for Lease<R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.resource.as_mut().unwrap()
    }
}

pub struct HashPool<I: Info> {
    pub resources: HashMap<I, Cache<I::Resource>>,
}
impl<I: Info> Default for HashPool<I> {
    fn default() -> Self {
        Self {
            resources: Default::default(),
        }
    }
}

impl<I> Debug for HashPool<I>
where
    I: Info + Debug,
    I::Resource: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HashPool")
            .field("resources", &self.resources)
            .finish()
    }
}

impl<I> Pool<I> for HashPool<I>
where
    I: Info + Hash + Eq + PartialEq + Clone,
{
    type Lease = Lease<I::Resource>;

    fn try_lease(&mut self, info: &I, ctx: &I::Context) -> Option<Self::Lease> {
        let cache = self
            .resources
            .entry(info.clone())
            .or_insert(Arc::new(Mutex::new(Vec::with_capacity(1))));
        let resource = cache
            .lock()
            .unwrap()
            .pop()
            .map(|r| Some(r))
            .unwrap_or_else(|| I::try_create(&info, &ctx))?;

        Some(Lease {
            resource: Some(resource),
            cache: cache.clone(),
        })
    }
}
