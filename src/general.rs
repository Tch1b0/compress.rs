pub type Bytes = Vec<u8>;

#[derive(Eq, Hash, PartialEq, Debug)]
pub struct Cluster{
    pub value: u32,
}

// A cluster containing 4 bytes
impl Cluster {

    // deconstruct a cluster into individual bytes
    pub fn deconstruct(&self) -> [u8; 4] {
        // fill array with 0
        let mut arr: [u8; 4] = [0; 4];
        let bytes: [u8; 4] = unsafe { std::mem::transmute(self.value) };
        // reverse byte order
        for i in 0..=3 {
            arr[i] = bytes[3 - i];
        }

        arr
    }
}

impl From<&[u8]> for Cluster {
    fn from(vals: &[u8]) -> Self {
        Cluster {
            value: u32::from_be_bytes((*vals).try_into().expect("slice has incorrect length"))
        }
    }
}
