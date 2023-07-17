use crate::prelude::*;

#[derive(Debug)]
struct Buffer {
    data: Vec<u8>,
}
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct BufferInfo {
    cap: usize,
}

impl Resource for Buffer {
    type Info = BufferInfo;

    type Context = ();

    fn create(info: &Self::Info, ctx: &Self::Context) -> Self {
        Buffer {
            data: Vec::with_capacity(info.cap),
        }
    }

    fn clear(&mut self) {
        self.data.clear();
    }
}

#[test]
fn hash_pool_lease() {
    let mut pool = hashpool::HashPool::<Buffer>::default();

    let b1 = pool.lease(&BufferInfo { cap: 10 }, &());

    let b2 = pool.lease(&BufferInfo { cap: 10 }, &());

    dbg!(&pool);

    drop(b1);
    dbg!(&pool);
    drop(b2);
    dbg!(&pool);
}
