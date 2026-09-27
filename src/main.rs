mod window;
mod teststuff;

enum Even {
    KeyInput {
        e_id: u32,
        b_id: u32,
    },
}

fn main() {
    teststuff::Me::moi();
    ()
}
