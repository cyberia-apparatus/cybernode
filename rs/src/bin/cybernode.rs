//! cybernode — the node of the soft3 ladder. The CLI lives in `cybernode::cli`.

fn main() {
    cybernode::cli::run(std::env::args().skip(1).collect());
}
