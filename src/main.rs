#[allow(unused)]
use network_programming::network;

fn main() {
    // network::tcp_client::run();
    // network::tcp_echo_server::run();
    network::tcp_echo_random_server::run();
    // network::udp_echo_server::run();
    // network::udp_client::run();
}
