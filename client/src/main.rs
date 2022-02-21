use blockchain::{
    blockchain::Blockchain, commands::handle_commands, wallet_manager::WalletManager,
};
use std::io::{self, Write};

pub fn main() {
    let mut chain = Blockchain::new();
    let mut wallet_manager = WalletManager::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Couldn't read user input!");

        handle_commands(input, &mut chain, &mut wallet_manager);
    }
}
//
