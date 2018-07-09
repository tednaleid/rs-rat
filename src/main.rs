extern crate clap;

use clap::{App, Arg, SubCommand};
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Write};



// time seq 2000 | cargo run -- tombstone > /dev/null
// cargo build --release

fn main() -> io::Result<()> {
    let matches = App::new("Rat")
        .version("0.0.1")
        .author("Ted Naleid <contact@naleid.com")
        .about("Kafka CLI tool")
        .arg(
            Arg::with_name("v")
                .short("v")
                .long("verbose")
                .multiple(true)
                .help("Sets the level of output, can be specified multiple times"),
        )
        .subcommand(
            SubCommand::with_name("tombstone")
                .help("create tombstones for submitted keys")
                .about(""),
        )
        .get_matches();

    match matches.occurrences_of("v") {
        0 => eprintln!("regular log level"),
        _ => eprintln!("verbose log level"),
    }

    if let Some(_) = matches.subcommand_matches("tombstone") {
        eprintln!("tombstone!");
        let stdin = io::stdin();
        let stdout = io::stdout();

        let stdin = BufReader::with_capacity(8 * 1024, stdin.lock());
        let mut stdout = BufWriter::new(stdout.lock());

        for line in stdin.lines() {
            writeln!(&mut stdout, "{}", line.unwrap()).unwrap();
        }
    }


    Ok(())
}


/*

this function in test_metadata in rust-rdkafka looks like it has the right stuff, just set a client


https://github.com/fede1024/rust-rdkafka/blob/master/tests/test_metadata.rs#L19

config value for ssl
fn create_consumer(group_id: &str) -> StreamConsumer {
    ClientConfig::new()
        .set("group.id", group_id)
        .set("client.id", "rdkafka_integration_test_client")
        .set("bootstrap.servers", get_bootstrap_server().as_str())
        .set("session.timeout.ms", "6000")
        .set("api.version.request", "true")
        .set("debug", "all")
        .create()
        .expect("Failed to create StreamConsumer")
}

output from: kafkacat -X list

...
security.protocol                        |  *  | plaintext, ssl, sasl_plaintext, sasl_ssl |     plaintext | Protocol used to communicate with brokers. <br>*Type: enum value*
ssl.cipher.suites                        |  *  |                 |               | A cipher suite is a named combination of authentication, encryption, MAC and key exchange algorithm used to negotiate the security settings for a network connection using TLS or SSL network protocol. See manual page for `ciphers(1)` and `SSL_CTX_set_cipher_list(3). <br>*Type: string*
ssl.key.location                         |  *  |                 |               | Path to client's private key (PEM) used for authentication. <br>*Type: string*
ssl.key.password                         |  *  |                 |               | Private key passphrase <br>*Type: string*
ssl.certificate.location                 |  *  |                 |               | Path to client's public key (PEM) used for authentication. <br>*Type: string*
ssl.ca.location                          |  *  |                 |               | File or directory path to CA certificate(s) for verifying the broker's key. <br>*Type: string*
...

*/

