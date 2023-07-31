use crate::prelude::*;

#[derive(Debug)]
struct Buffer {
    data: Vec<u8>,
}
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct BufferInfo<'a> {
    cap: usize,
    shape: &'a [usize],
}

impl<'a> Info<Buffer> for BufferInfo<'a> {
    type Context = ();

    fn try_create(info: &Self, ctx: &Self::Context) -> Option<Buffer> {
        Some(Buffer {
            data: Vec::with_capacity(info.cap),
        })
    }
}

impl Resource for Buffer {}

#[test]
fn hash_pool_lease() {
    let mut pool = hashpool::HashPool::<Buffer>::default();

    let b1 = pool.lease(
        &BufferInfo {
            cap: 10,
            shape: &[],
        },
        &(),
    );

    let b2 = pool.lease(
        &BufferInfo {
            cap: 10,
            shape: &[],
        },
        &(),
    );

    dbg!(&pool);

    drop(b1);
    dbg!(&pool);
    drop(b2);
    dbg!(&pool);
}
