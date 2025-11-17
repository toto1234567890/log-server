This is a custom log server that receive log messages from CLI loggers (on the same machine).
It listen on 2 ports : 4020 for TCP and 4021 for GRPC (GRPC could be disable with "--only_tcp" command line param)
Then it writes messages on file system, with rotating (10 files of 10 mb)

So every program that can use TPC, can log message (almost all programming langage)
GRPC has been added for Javascript logging.



Data Flow of the log server

- TCP: Binary protocol with length-prefixed messages (capnp)
- gRPC: HTTP/2 with Protobuf serialization
- Both protocols feed into the same processing pipeline


Server Processing Layers : 

                        *can be enable or not
┌──────────────────┐    ┌──────────────────┐
│   TCP Clients    │    │   gRPC Clients   │
│  ┌─────────────┐ │    │  ┌─────────────┐ │
│  │ App Layer   │ │    │  │ App Layer   │ │
│  └──────┬──────┘ │    │  └──────┬──────┘ │
│  ┌──────▼──────┐ │    │  ┌──────▼──────┐ │
│  │ TCP Logger  │ │    │  │gRPC Logger  │ │
│  └──────┬──────┘ │    │  └──────┬──────┘ │
│  ┌──────▼──────┐ │    │  ┌──────▼──────┐ │
│  │TCP socket   │ │    │  │ Channel     │ │
│  │Connection   │ │    │  │   Pool      │ │
│  └──────┬──────┘ │    │  └──────┬──────┘ │
└─────────┼────────┘    └─────────┼────────┘
          │                       │
          │                       │         
          │                       │
    ┌─────│───────────────────────│──────┐
    │     │   Log Server Core     │      │
    │  ┌──▼──────────┐ ┌──────────▼──┐   │
    │  │ TCP Handler │ │ gRPC Handler│   │
    │  └──────┬──────┘ └──────┬──────┘   │
    │         │               │          │
    │  ┌──────▼──────┐ ┌──────▼──────┐   │
    │  │capnp Parser │ │ proto Parser│   │
    │  └──────┬──────┘ └──────┬──────┘   │
    │         └───────┬───────┘          │
    │                 │                  │
    │         ┌───────▼───────┐          │
    │         │  Sequencer    │          │
    │         │ (AtomicU64)   │          │
    │         └───────┬───────┘          │
    └─────────────────┼──────────────────┘
                      │
                      | 
                      |
                      │
            ┌─────────▼─────────┐
            │   File Writer     │
            │ ┌───────────────┐ │
            │ │ BTreeMap<u64, │ │  ← Orders by sequence number
            │ │    String>    │ │
            │ └───────┬───────┘ │
            │         │         │
            │ ┌───────▼───────┐ │
            │ │ Batch Writer  │ │  ← Dynamic batching
            │ └───────┬───────┘ │
            │         │         │
            │ ┌───────▼───────┐ │
            │ │ Retry Logic   │ │  ← 3 retries with delays
            │ └───────┬───────┘ │
            └─────────┼─────────┘
                      │
                      |
                      |
                      │
            ┌─────────▼─────────┐
            │   File System     │
            │ ┌───────────────┐ │
            │ │ _main.log     │ │  ← Current active file
            │ └───────────────┘ │
            │ ┌───────────────┐ │
            │ │ _main.log.0   │ │  ← Rotated files
            │ └───────────────┘ │
            │ ┌───────────────┐ │
            │ │ _main.log.1   │ │
            │ └───────────────┘ │
            │       ...         │
            └───────────────────┘
