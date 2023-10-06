# Substreams Primary Module

This is a Rust module designed for Substreams that focuses on handling primary issue pools and related data.

## Table of Contents

- [Substreams Primary Module](#substreams-primary-module)
  - [Table of Contents](#table-of-contents)
  - [Introduction](#introduction)
  - [Installation](#installation)
  - [Usage](#usage)
  - [Functions](#functions)
      - [map\_pools](#map_pools)
      - [map\_subscriptions](#map_subscriptions)
      - [kv\_out](#kv_out)
  - [Modules used](#modules-used)
  - [License](#license)

## Introduction

The Substreams Primary Module is a Rust module developed to detect pools, subscriptions events for primary issue pools. This module is a crucial part of the Substreams project, which aims to provide high-performance blockchain indexing technology.

## Installation

Before using the Substreams Primary Module, ensure that you have Rust and substreams.
Official Rust installation guide: [Rust Installation](https://www.rust-lang.org/tools/install)
Official Substreams installation guide: [Substreams](https://substreams.streamingfast.io/getting-started/installing-the-cli)

After setting up Rust and Substreams, you can build and run this module as part of the overall Substreams project. Please refer to the [Verified Substreams README](../README.md) for installation and usage instructions for the entire project.

## Usage

To use the Substreams Primary Module, follow these steps:

1. **Generate Rust Code**: Navigate to the module folder and execute the following command to generate Rust code for connecting to protobuf:

   ```bash
   substreams protogen substreams.yaml --exclude-paths="sf/substreams,google"

2. **Build the Rust Code**: Run the following command within the module folder to build the Rust code:
   ```bash
   substreams run -e <Protocol-Proto model> substreams.yaml <FUNCTION_NAME> --start-block 9561663 --stop-block +20
3. Execute the Module: To execute the module, run the following command in the module folder:
   ```bash
   substreams run -e <Protocol-Proto model> substreams.yaml <FUNCTION_NAME> --start-block 9561663 --stop-block +20
Replace <Protocol-Proto model> with the desired protocol model. Supported protocol-Proto models are listed in the Verified Substreams README.

## Functions
#### map_pools
*Description:* This function detects poolcreated event from blockchain and returns information about them.
#### map_subscriptions
Description: This function detects subscriptions events under the address of poolcreated event at a perticular block and maps them to the appropriate data structures.

#### kv_out
Description: This function generates key-value operations based on the provided subscription data for the client interface.

## Modules used
1. abi: Contains Rust code for working with ABI.
2. pb: Contains Rust code for protocol buffers.


## License
[Apache 2.0](https://github.com/streamingfast/substreams/blob/develop/LICENSE/README.md).