//! This is a binary running in the local environment
//!
//! You have to provide all needed configuration attributes via command line parameters,
//! or you could specify a configuration file. The format of configuration file is defined
//! in mod `config`.

use clap::clap_app;
use shadowsocks_rust::service::local;

fn main() {
    let mut app = clap_app!(shadowsocks =>
        (version: shadowsocks_rust::VERSION)
        (about: "A fast tunnel proxy that helps you bypass firewalls. (https://shadowsocks.org)")
    );
    app = local::define_command_line_options(app);

    let matches = app.get_matches();
    local::main(&matches);
}
