# Waynest

Waynest is a foundational library designed to handle the low-level aspects of
the Wayland protocol. Built upon Rust’s asynchronous ecosystem, Waynest
leverages the Tokio libraries to efficiently manage asynchronous tasks.

The library is responsible for implementing the Wayland wire protocol and
provides essential utilities for interacting with it. A significant portion of
the code is auto-generated from XML interface descriptions

## Overview

Waynest is intentionally designed as a low-level library. It is not intended for
direct use in most scenarios but serves as a crucial building block for
constructing higher-level libraries and applications.

For those developing a Wayland client,
[Waynova](https://github.com/verdiwm/waynova) is a more suitable choice. Waynova
builds upon Waynest, offering convenient helpers and automating a considerable
amount of boilerplate code.

If your goal is to create a Wayland compositor, you might find
[Verdi](https://github.com/verdiwm/verdi) to be of interest. Verdi utilizes
Waynest internally.

## Usage

Waynest’s codebase is organized into three primary modules:

**`Wire` Module**

The `wire` module implements the core wire format for the Wayland protocol. It
facilitates the establishment of bidirectional connections between clients and
servers. The central component of this module is the `Socket` struct, which
implements both the `Sink` and `Stream` traits, providing easy integration with
Rust’s asynchronous ecosystem.

**`Server` Module**

The `server` module includes traits generated from XML interface definitions.
Implementing these traits corresponds to implementing the respective Wayland
protocols. Waynest does not make assumptions about the underlying implementation
but provides the necessary tools for sending events to clients. To implement any
of the protocols, developers need to implement the `Dispatcher` trait, which can
be easily derived using the provided macro.

**`client` Module**

_TBA_

## License

This project is licensed under the
[Apache-2.0 License](http://www.apache.org/licenses/LICENSE-2.0). For more
information, please see the [LICENSE](LICENSE) file.
