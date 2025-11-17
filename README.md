
# Log Server

A high-performance log server that receives log messages from CLI loggers on the same machine via TCP and gRPC protocols, with file system storage and log rotation.

# Features

*Dual Protocol Support: Listen on both TCP (port 4020) and gRPC (port 4021)
Protocol Flexibility: gRPC can be disabled with --only_tcp command line parameter
Language Agnostic: Any programming language that supports TCP can send logs
JavaScript Support: gRPC interface specifically designed for JavaScript logging
File Storage: Writes messages to file system with rotation
Rotating Logs: Maintains 10 files of 10MB each
Binary Protocols: Efficient message serialization using Cap'n Proto and Protocol Buffers
Architecture

# Data Flow

text
TCP Clients (Binary/capnp) ───┐
                              ├──▶ Log Server Core ───▶ File Writer ───▶ File System
gRPC Clients (Protobuf) ──────┘
Server Processing Layers

# Client Interfaces

TCP Clients: Binary protocol with length-prefixed Cap'n Proto messages
gRPC Clients: HTTP/2 with Protocol Buffers serialization
Protocol Handlers

TCP Handler with socket connections
gRPC Handler with channel pool
Protocol-specific parsers (capnp and proto)
Core Processing

Sequencer using AtomicU64 for message ordering
Unified processing pipeline
Storage Layer

BTreeMap for sequence number ordering
Dynamic batch writing
Retry logic (3 attempts with delays)
File system output with rotation
Installation

# Prerequisites

Rust toolchain (latest stable version)
Protocol Buffer compiler (for gRPC support)
Build from Source

bash
git clone <repository-url>
cd log_server
cargo build --release
Usage

# Basic Usage

bash
./log_server
Command Line Options

Option	Description	Default Value
--name	Server name	LogServer
--host	Bind host	127.0.0.1
--port	TCP port	4020
--grpc_port	gRPC port	4021
--tcp_only	Disable gRPC interface	false

# Examples

Start with default settings:

bash
./log_server
Start with custom ports:

bash
./log_server --port 5000 --grpc_port 5001
Disable gRPC and use TCP only:

bash
./log_server --tcp_only
Custom server name:

bash
./log_server --name "MyLogServer"
Protocol Specifications

TCP Protocol

Port: 4020 (configurable)
Format: Length-prefixed binary messages
Serialization: Cap'n Proto
Message Structure: [4-byte length][capnp message] (will be changed soon with a common socket object)

gRPC Protocol

Port: 4021 (configurable)
Transport: HTTP/2
Serialization: Protocol Buffers
Service: Defined in .proto files
File Output

Log File Structure

text
_main.log      # Current active file
_main.log.0    # Most recent rotated file
_main.log.1    # Older rotated file
...
_main.log.9    # Oldest rotated file

Rotation Policy

File Size: 10MB per file
File Count: 10 files maximum
Rotation: When current file reaches 10MB
Cleanup: Oldest files are deleted when rotation occurs
Client Integration

TCP Clients (Any Language)

Example connection pattern:

Establish TCP connection to host:port
Send length-prefixed Cap'n Proto messages
Close connection when done
gRPC Clients (JavaScript/TypeScript)

# Example using gRPC-Web:

javascript
// JavaScript example
const client = new LogServiceClient('http://localhost:4021');
client.logMessage(logEntry);
Example Client Implementations

The repository includes client libraries for:

Rust (TCP and gRPC)
JavaScript/TypeScript (gRPC)
Python (TCP)
Go (TCP)

# Performance Characteristics

High Throughput: Batched writing with dynamic batching
Ordering Guarantees: Atomic sequence numbers ensure message order
Fault Tolerance: Retry logic with exponential backoff
Memory Efficient: BTreeMap for ordered storage with minimal overhead
Monitoring

Log Files

# Check the server's own logs in the rotating files for:

Connection statistics
Error conditions
Performance metrics
Health Checking

# TCP Health Check:

bash
echo "health" | nc localhost 4020
gRPC Health Check:

bash
# Use grpc_health_probe or similar tools
Troubleshooting

Common Issues

Port Conflicts: Ensure ports 4020 and 4021 are available
File Permissions: Verify write permissions in log directory
gRPC Issues: Check if Protocol Buffer compiler is installed



