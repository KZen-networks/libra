extern crate two_party_eddsa_server;

fn main() {
    two_party_eddsa_server::api::get_server().launch();
}