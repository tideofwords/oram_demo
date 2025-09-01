# ORAM demo

This is the code for a live demonstration of 
[Oblivious RAM](https://eprint.iacr.org/2013/280.pdf)
that I gave in Summer 2024,
as part of a presentation of the Path ORAM protocol.

## About ORAM

Oblivious RAM is a protocol to hide memory access patterns from an untrusted 
server (storage device).

A client stores data (encrypted) on an untrusted server.
The client does not want the server to learn anything from its memory access patterns.
For example, the server should not learn which positions in memory the client reads and writes,
or even whether the client accesses the same position more than once.

An ORAM algorithm sits between a _client_ who
wants to access memory and a _server_ that has memory capabilities.
Between the ORAM and the client, the client submits _logical_
read and write requests to the ORAM client, and the client will reply to
each (after interaction with the server).

At the server-ORAM interface, the server simply acts as a _physical_ memory: the
ORAM client sends read and write requests to the server, and the server
responds. 

The ORAM protocol guarantees that the _physical_ memory accesses
that are sent to the server
do not reveal anything about the _logical_ requests from the client.

## To use

To run this demo you will need Rust and the package manager Cargo on your system.

Clone the repo.  From inside the `oram_demo` directory, execute `cargo run`.
You will find yourself in an interactive command-line interface.

First, the program will prompt you for `Size of virtual memory?`.
You can enter any positive integer; `10` is great for a first try.
This is the size of the (logical) array to which you will read and write.

Then the program will prompt you to `enter an instruction`.
There are three valid instructions:
- `write <addr> <val>`: writes `<val>` to position `<addr>` in the logical array.  `<val>` should be either `true` or `false`, and `<addr>` must be a valid index to the array (i.e. an integer between 0 and `n-1`, where `n` is the size you entered for the memory).
- `read <addr>`: reads the value stored in position `<addr>`
- `q`: quits.

When you enter the instruction, the program will:
- Engage in a "conversation" with the simulated server, called Memory.
  The ORAM's requests to Memory are in yellow; Memory's responses are in blue.
  (Note: In a real protocol, all data would be encrypted.  
  Memory cannot see the content of any block or bucket, or even whether it is empty or not.)
- Respond to your instruction: either with a confirmation that a write was performed,
  or the result of a read.
- Prompt you for another instruction.


