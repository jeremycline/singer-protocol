# singer-protocol

Structures and utilities for implementing [Singer taps and targets](https://hub.meltano.com/singer/spec/).


[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![Build Status][actions-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/singer-protocol.svg
[crates-url]: https://crates.io/crates/singer-protocol
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/jeremycline/singer-protocol/blob/main/LICENSE
[actions-badge]: https://github.com/jeremycline/singer-protocol/workflows/ci/badge.svg
[actions-url]: https://github.com/jeremycline/singer-protocol/actions?query=workflow%3Aci+branch%3Amain

## Overview

The Singer specification communicates over `stdin` and `stdout` using
JSON-serialized messages. This crate provides those messages to make it easier
to implement Singer sources and sinks for [Meltano](https://meltano.com/).
