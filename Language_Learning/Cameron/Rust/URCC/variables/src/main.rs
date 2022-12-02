const STATING_MISSILES: i32 = 8;
const READY_AMMOUNT: i32 = 2;

fn main() {
    let (mut missiles, ready) = (STATING_MISSILES, READY_AMMOUNT);
    println!("Firing {} of my {} missiles...", ready, missiles);

    missiles = missiles - ready;

    println!("{} missiles left", missiles);
}
