use storage_vec::Storage;

#[derive(Debug)]
struct OurType {
    val: String,
}

impl Drop for OurType {
    fn drop(&mut self) {
        // eprintln!("dropped the thing");
    }
}

fn main() {
    let mut storage = Storage::<OurType>::with_capacity(10_000);

    for i in 0..10_000 {
        storage.add(OurType {
            val: format!("hello world: {i}"),
        });
        eprintln!("added {i}");
    }

    drop(storage);
    eprintln!("dropped storage");
}
