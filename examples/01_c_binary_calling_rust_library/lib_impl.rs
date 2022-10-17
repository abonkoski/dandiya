pub use _defn_example::*;

// NOTE: Doing this custom because we don't want to have to add dependencies
// since we're building with rustc directly
struct RandomGen {
    state: u64,
}
impl RandomGen {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn gen(&mut self) -> u64 {
        let mut h = self.state;

        // Based on Murmur3's hash finalizer
        h ^= h >> 33;
        h = h.wrapping_mul(0xff51afd7ed558ccd);
        h ^= 33;
        h = h.wrapping_mul(0xc4ceb9fe1a85ec53);
        h ^= 33;

        self.state = h;
        h
    }
}

fn random_packet(pkt: &mut example_packet, r: &mut RandomGen) {
    let len = r.gen() as usize % pkt.dat.len();
    for i in 0..len {
        pkt.dat[i] = r.gen() as u8;
    }
    pkt.len = len as u16;
}

struct Example {
    rand: RandomGen,
    state: u8,
}

impl Example {
    fn new() -> Self {
        Self {
            rand: RandomGen::new(0x1a92fab2f5f30cdb),
            state: 1,
        }
    }
}

#[no_mangle]
pub extern "C" fn example_new_v1() -> *mut example_ctx {
    let ctx = Box::new(Example::new());
    Box::into_raw(ctx) as *mut example_ctx
}

#[no_mangle]
pub extern "C" fn example_delete_v1(ctx: *mut example_ctx) {
    let _ = unsafe { Box::from_raw(ctx as *mut Example) };
}

#[no_mangle]
pub extern "C" fn example_fetch_batch_v1(ctx: *mut example_ctx, batch: *mut example_batch) -> u64 {
    let ctx = unsafe { &mut *(ctx as *mut Example) };
    let batch = unsafe { &mut *batch };

    let n = ctx.state;
    assert!(n as usize <= batch.pkts.len());

    for i in 0..n {
        random_packet(&mut batch.pkts[i as usize], &mut ctx.rand);
    }
    batch.num = n;

    ctx.state = (ctx.state + 13) % batch.pkts.len() as u8;

    EXAMPLE_SUCCESS
}
