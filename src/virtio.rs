// https://docs.oasis-open.org/virtio/virtio/v1.1/virtio-v1.1.html

pub const VIRTIO: u64 = 0x1000_1000;
pub const VIRTIO_END: u64 = 0x1000_1FFF;

#[derive(Debug)]
pub struct Virtio {}

impl Virtio {
    pub fn new() -> Virtio {
        Virtio {}
    }

    pub fn read(&self, addr: u64) -> u64 {
        0
    }

    pub fn write(&mut self, addr: u64, data: u64) {}
}
