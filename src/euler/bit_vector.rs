#[derive(Debug)]
pub struct BitVector {
    bits: Vec<u64>,
}

impl BitVector {
    pub fn make_vector(n: usize) -> BitVector {
        let num_u64 = ((n as f64) / 64.0).ceil() as usize;
        BitVector {
            bits: vec![0x00000000; num_u64],
        }
    }

    fn to_index_and_flag(n: usize) -> (usize, u64) {
        let i = n / 64;
        let r = n % 64;
        (i as usize, 0x1 << r)
    }

    pub fn set_bit(&mut self, n: usize) -> () {
        let i = BitVector::to_index_and_flag(n);
        self.bits[i.0] |= i.1;
    }

    pub fn is_bit_set(&self, n: usize) -> bool {
        let i = BitVector::to_index_and_flag(n);
        self.bits[i.0] & i.1 != 0
    }
}

#[test]
fn dummy_test() {
    let mut b = BitVector::make_vector(20);
    assert!(!b.is_bit_set(5));
    b.set_bit(5);
    assert!(b.is_bit_set(5));
}
