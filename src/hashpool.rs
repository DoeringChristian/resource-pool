use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};

use crate::traits::{Pool, Resource};

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

pub struct HashPool<R: Resource> {
    pub resources: HashMap<R::Info, Cache<R>>,
}
impl<R: Resource> Default for HashPool<R> {
    fn default() -> Self {
        Self {
            resources: Default::default(),
        }
    }
}

impl<I, R> Debug for HashPool<R>
where
    I: Debug,
    R: Resource<Info = I> + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HashPool")
            .field("resources", &self.resources)
            .finish()
    }
}

impl<I, R> Pool<R> for HashPool<R>
where
    I: Hash + Eq + PartialEq + Clone,
    R: Resource<Info = I>,
{
    type Lease = Lease<R>;

    fn lease(&mut self, info: &R::Info, ctx: &R::Context) -> Self::Lease {
        let cache = self
            .resources
            .entry(info.clone())
            .or_insert(Arc::new(Mutex::new(Vec::with_capacity(1))));
        let resource = cache
            .lock()
            .unwrap()
            .pop()
            .unwrap_or_else(|| R::create(&info, &ctx));

        Lease {
            resource: Some(resource),
            cache: cache.clone(),
        }
    }
}
