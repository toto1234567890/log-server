
# Log Server

A high-performance log server that receives log messages from CLI loggers on the same machine via TCP and gRPC protocols, <br>
with file system storage and log rotation.

# Features

Dual Protocol Support:     Listen on both TCP (port 4020) and gRPC (port 4021) <br>
Protocol Flexibility:      gRPC can be disabled with --only_tcp command line parameter <br>
Language Agnostic:         Any programming language that supports TCP can send logs <br>
JavaScript Support:        gRPC interface specifically designed for JavaScript logging <br>
File Storage:              Writes messages to file system with rotation <br>
Rotating Logs:             Maintains 10 files of 10MB each <br>
Binary Protocols:          Efficient message serialization using Cap'n Proto and Protocol Buffers <br>


# Data Flow

TCP Clients (Binary/capnp) ───┐<br>
                              ├──▶ Log Server Core ───▶ File Writer ───▶ File System<br>
gRPC Clients (Protobuf) ──────┘<br>


# Client Interfaces

TCP Clients: Binary protocol with length-prefixed Cap'n Proto messages <br>
gRPC Clients: HTTP/2 with Protocol Buffers serialization <br>

Protocol Handlers <br>
TCP Handler with socket connections <br>
gRPC Handler with channel pool <br>
Protocol-specific parsers (capnp and proto) <br>
Core Processing <br>

Sequencer using AtomicU64 for message ordering <br>
Unified processing pipeline <br>
Storage Layer <br>

BTreeMap for sequence number ordering <br>
Dynamic batch writing <br>
Retry logic (3 attempts with delays) <br>
File system output with rotation <br>

# Prerequisites

Rust toolchain (latest stable version)
Protocol Buffer compiler (for gRPC support)
Build from Source

bash
git clone <repository-url>
cd log_server
cargo build --release

# Basic Usage

./log_server
Command Line Options

Option	Description	Default Value
--name	Server name	LogServer
--host	Bind host	127.0.0.1
--port	TCP port	4020
--grpc_port	gRPC port	4021
--tcp_only	Disable gRPC interface	false

# Examples

Start with default settings: <br>
./log_server<br>

Start with custom ports:<br>
./log_server --port 5000 --grpc_port 5001<br>

Disable gRPC and use TCP only:<br>
./log_server --tcp_only<br>

Custom server name:<br>
./log_server --name "MyLogServer"<br>

Protocol Specifications <br>

TCP Protocol <br>
Port: 4020 (configurable)<br>
Format: Length-prefixed binary messages<br>
Serialization: Cap'n Proto<br>
Message Structure: [4-byte length][capnp message] (will be changed soon with a common socket object)<br>

gRPC Protocol<br>

Port: 4021 (configurable)<br>
Transport: HTTP/2<br>
Serialization: Protocol Buffers<br>
Service: Defined in .proto files<br>

File Output<br>

Log File Structure<br>

_main.log      # Current active file<br>
_main.log.0    # Most recent rotated file<br>
_main.log.1    # Older rotated file<br>
...<br>
_main.log.9    # Oldest rotated file<br>

Rotation Policy<br>

File Size: 10MB per file<br>
File Count: 10 files maximum<br>
Rotation: When current file reaches 10MB<br>
Cleanup: Oldest files are deleted when rotation occurs<br>

Client Integration<br>

TCP Clients (Any Language)<br>

Example connection pattern: <br>
Establish TCP connection to host:port<br>
Send length-prefixed Cap'n Proto messages<br>
Close connection when done<br>
gRPC Clients (JavaScript/TypeScript)<br>

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



