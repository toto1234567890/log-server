//! Log Server Main Binary
//!
//! Centralized logging server that handles both TCP socket (Cap'n Proto)
//! and gRPC log messages with ordered file writing and rotation.

use clap::{Arg, Command};
use log_server::core::servers::LogServer;





//================================================================
fn main() {

    let matches = Command::new("log_server")
        .arg(Arg::new("name")
            .long("name")
            .default_value("LogServer"))
        .arg(Arg::new("host")
            .long("host")
            .default_value("127.0.0.1"))
        .arg(Arg::new("port")
            .long("port")
            .default_value("9020"))
        .arg(Arg::new("grpc_port")
            .long("grpc_port")
            .default_value("9021"))
        .arg(Arg::new("tcp_only")
            .long("tcp_only")
            .action(clap::ArgAction::SetTrue))  // Add this flag
        .get_matches();
    
    let name = matches.get_one::<String>("name").unwrap();
    let host = matches.get_one::<String>("host").unwrap();
    let port = matches.get_one::<String>("port").unwrap().parse::<u16>().unwrap();
    let grpc_port = matches.get_one::<String>("grpc_port").unwrap().parse::<u16>().unwrap();
    let tcp_only = matches.get_flag("tcp_only");  // Get the flag value
    
    println!("{} : starting log server", name);
    if tcp_only {
        println!("TCP-only mode enabled");
    }
    
    // Run the server
    if let Err(e) = run_server(name, host, port, grpc_port, tcp_only) {
        eprintln!("{} : server failed - {}", name, e);
        std::process::exit(1);
    }
}

//-----------------------------------------------------------------------------------------------

/// Main server execution function
fn run_server(name: &str, host: &str, port: u16, grpc_port: u16, tcp_only: bool) -> Result<(), Box<dyn std::error::Error>> {
    let runtime = tokio::runtime::Runtime::new()?;
    
    runtime.block_on(async {
        let server = LogServer::new(name, host, port, grpc_port, tcp_only).await?;
        server.run().await
    })
}