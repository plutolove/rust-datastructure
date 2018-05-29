pub struct BitMap {
    bits: Vec<u8>,
    size: usize,
}

impl BitMap {
    pub fn new(size: usize) -> Self {
        let mut len = size/8;
        if size % 8 > 0 {
            len += 1;
        }
        let v: Vec<u8> = vec![0; len];
        BitMap{
            bits: v,
            size: size,
        }
    }

    pub fn set(&mut self, pos: usize) {
        assert!(pos < self.size, "index out of bounds");
        let index = pos / 8;
        let subset = pos % 8;
        self.bits[index] = (self.bits[index]) | (1 << subset);
    }
    
    pub fn unset(&mut self, pos: usize) {
        assert!(pos < self.size, "index out of bounds");
        let index = pos / 8;
        let subset = pos % 8;
        self.bits[index] = (self.bits[index]) & ((1 << subset) - 1);
    }

    pub fn is_set(&self, pos: usize) -> bool {
        assert!(pos < self.size);
        let index = pos / 8;
        let subset = pos % 8;
        ((self.bits[index] & (1 << subset)) != 0)
    }
}

