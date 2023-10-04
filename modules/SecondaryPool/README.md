# Substreams Secondary Module

This is a Rust module designed for Substreams that focuses on handling secondary issue pools and related data.

## Table of Contents

- [Substreams Secondary Module](#substreams-secondary-module)
  - [Introduction](#introduction)
  - [Installation](#installation)
  - [Usage](#usage)
  - [Functions](#functions)
      - [map\_pools](#map_pools)
      - [map\_trades](#map_trades)
      - [kv\_out](#kv_out)
  - [Modules used](#modules-used)
  - [License](#license)

## Introduction

The Substreams Secondary Module is a Rust module developed to detect pools, subscriptions events for secondary issue pools. This module is a crucial part of the Substreams project, which aims to provide high-performance blockchain indexing technology.

## Installation

Before using the Substreams Secondary Module, ensure that you have Rust and substreams installed. 
Follow the official Rust installation guide: [Rust Installation](https://www.rust-lang.org/tools/install). 
Also, refer to the official Substreams installation guide: [Substreams Installation](https://substreams.streamingfast.io/getting-started/installing-the-cli).

After setting up Rust and Substreams, you can build and run this module.

## Usage

To use the Substreams Primary Module, follow these steps:

1. **Build the Rust Code**: Run the following command within the module folder to build the Rust code:
   ```bash
   cargo build --target wasm32-unknown-unknown --release3.
   
2. Execute the Module: To execute the module, run the following command in the module folder:
   ```bash
   substreams run -e <Protocol-Proto model> substreams.yaml <FUNCTION_NAME> --start-block 9561663 --stop-block +20

Replace <Protocol-Proto model> with the desired protocol model. Supported protocol-Proto models are listed in the [Verified Substreams README](../../README.md).

## Functions
#### map_pools
*Description:* This function detects poolcreated event from blockchain and returns information about them.
#### map_trades
Description: This function detects Trades events under the address of poolcreated event at a perticular block and maps them to the appropriate data structures.

#### kv_out
Description: This function generates key-value operations based on the provided subscription data for the client interface.

## Modules used
1. abi: Contains Rust code for working with ABI.
2. pb: Contains Rust code for protocol buffers.


## License
[Apache 2.0](https://github.com/streamingfast/substreams/blob/develop/LICENSE/README.md).
