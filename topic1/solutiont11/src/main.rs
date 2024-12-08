use baby_mimalloc::{new_mimalloc_mmap_mutex, MimallocMmapMutex};
use std::collections::BTreeMap;

#[global_allocator]
static ALLOCATOR: MimallocMmapMutex = new_mimalloc_mmap_mutex();

fn main() {
    const N: usize = 1_000_000;
    let mut map = BTreeMap::new();
    for i in 0..N {
        map.insert(i.to_string(), i);
    }
    assert_eq!(map.get("42"), Some(&42));
    let vec = map.into_iter().collect::<Vec<_>>();
    for i in (0..vec.len()).step_by(N / 10) {
        dbg!(&vec[i]);
    }
}
