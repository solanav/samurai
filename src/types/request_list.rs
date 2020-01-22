use crate::network::packet::Packet;

pub struct RequestList {
    requests: Vec<Packet>,
}

impl RequestList {
    pub fn new() -> Self {
        RequestList {
            requests: Vec::new(),
        }
    }

    pub fn add(&mut self, packet: Packet) {
        self.requests.push(packet);
    }

    pub fn rm(&mut self, i: usize) {
        if i > self.requests.len() {
            return;
        }

        self.requests.remove(i);
    }

    pub fn find_cookie(&self, cookie: u32) -> Option<usize> {
        let mut i = 0;
        for p in self.requests.iter() {
            if p.cookie() == cookie {
                return Some(i);
            }
            i += 1;
        }

        None
    }
}