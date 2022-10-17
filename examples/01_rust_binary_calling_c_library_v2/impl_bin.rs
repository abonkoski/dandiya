use example;

struct Example {
    ctx: *mut example::example_ctx,
}

impl Example {
    fn new(n: u8) -> Self {
        let ctx = unsafe { example::example_new(n) };
        if ctx.is_null() {
            panic!("Null context");
        }
        Self { ctx }
    }

    fn fetch_batch(&mut self, batch: &mut Batch) -> Result<(), u64> {
        let ret = unsafe { example::example_fetch_batch(self.ctx, &mut batch.0) };
        if ret == example::EXAMPLE_SUCCESS {
            Ok(())
        } else {
            Err(ret)
        }
    }
}

impl Drop for Example {
    fn drop(&mut self) {
        unsafe { example::example_delete(self.ctx) };
    }
}

struct Batch(example::example_batch);

impl Default for Batch {
    fn default() -> Self {
        // example_batch is a C struct of raw data, designed to be zero'd
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

fn main() {
    let mut ctx = Example::new(2);
    let mut batch = Batch::default();

    for i in 0..5 {
        ctx.fetch_batch(&mut batch).unwrap();
        println!("batch {}: {}", i, batch.0.num);
    }
}
