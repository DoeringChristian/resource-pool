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

impl<'a> Info for BufferInfo<'a> {
    type Context = ();
    type Resource = Buffer;

    fn create(info: &Self, ctx: &Self::Context) -> Buffer {
        Buffer {
            data: Vec::with_capacity(info.cap),
        }
    }
}

impl Resource for Buffer {
    fn clear(&mut self) {
        self.data.clear()
    }
}

#[test]
fn hash_pool_lease() {
    let mut pool = hashpool::HashPool::<BufferInfo>::default();

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
