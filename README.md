<div align="center">
  <img src="https://resolvingarchitecture.io/images/ra.png"  />

  <h1>Resolving Architecture</h1>

  <p>
    <strong>Clarity in Design</strong>
  </p>
  
  <h2>Bluetooth Client</h2>
  
  <p>
   A client for interacting with the local Bluetooth radio. Can be ran within the <a target="_blank" href="https://github.com/resolvingarchitecture/service-bus">Service Bus</a> as a Service.
   </p>
  
  <p>
    <a href="https://travis-ci.com/resolvingarchitecture/bluetooth-client"><img alt="build" src="https://img.shields.io/travis/resolvingarchitecture/bluetooth-client"/></a>
    <a href="https://crates.io/crates/bluetooth-client"><img alt="Crate Info" src="https://img.shields.io/crates/v/bluetooth-client.svg"/></a>
    <a href="https://docs.rs/crate/bluetooth_client/"><img alt="API Docs" src="https://img.shields.io/badge/docs.bluetooth-client-green"/></a>
  </p>
  <p>
    <a href="https://github.com/resolvingarchitecture/bluetooth-client/blob/master/LICENSE"><img alt="License" src="https://img.shields.io/github/license/resolvingarchitecture/bluetooth-client"/></a>
    <a href="https://resolvingarchitecture.io/ks/publickey.brian@resolvingarchitecture.io.asc"><img alt="PGP" src="https://img.shields.io/keybase/pgp/objectorange"/></a>
  </p>
  <p>
    <img alt="commits" src="https://img.shields.io/crates/d/bluetooth-client"/>
    <img alt="repo size" src="https://img.shields.io/github/repo-size/resolvingarchitecture/bluetooth-client"/>
  </p>
  <p>
    <img alt="num lang" src="https://img.shields.io/github/languages/count/resolvingarchitecture/bluetooth-client"/>
    <img alt="top lang" src="https://img.shields.io/github/languages/top/resolvingarchitecture/bluetooth-client"/>
    <a href="https://blog.rust-lang.org/2020/03/12/Rust-1.42.html"><img alt="Rustc Version 1.42+" src="https://img.shields.io/badge/rustc-1.42+-green.svg"/></a>
  </p>

  <h4>
    <a href="https://resolvingarchitecture.io">Info</a>
    <span> | </span>
    <a href="https://docs.rs/crate/bluetooth_client/">Docs</a>
    <span> | </span>
    <a href="https://github.com/resolvingarchitecture/bluetooth-client/blob/master/CHANGELOG.md">Changelog</a>
  </h4>
</div>

## Donate
Request BTC/XMR/ZEC address for a donation at brian@resolvingarchitecture.io.

## Notes
!! WIP - not stable until version 1.0 !!

## Goals

*[ ] 1.0.0 - Minimal Useful Functionality
    *[ ] 0.1.0 - Minimal CLI: get friendly name, address, discover devices, get services
    *[ ] 0.2.0 - Basic I/O: Send/Receive Datagrams
    *[ ] 0.3.0 - Service Bus Support: [service_bus](https://crates.io/crates/service-bus) crate implementing Service trait
    *[ ] 0.4.0 - Test Suite
    *[ ] 0.5.0 - Example CLI use cases
    *[ ] 0.6.0 - Example Service use cases
    *[ ] 0.7.0 - README.md completed
    *[ ] 0.8.0 - All code documented
    *[ ] 0.9.0 - All examples documented

[Crates.io](https://crates.io/crates/bluetooth_client)

[Examples for dev](https://github.com/szeged/blurz/tree/master/examples)

!! WIP - not stable until version 1.0 !!

## Setup - Ubuntu 18.04
1. Install Bluez
    ```shell script
    snap install bluez
    ```
2. Install Rust
   ```shell script
   sudo apt update
   sudo apt upgrade
   curl https://sh.rustup.rs -sSf | sh
   ```
3. Restart terminal
4. Verify Rust installed
    ```shell script
     rustc --version
    ```
5. Install build essentials
    ```shell script
    sudo apt install build-essential
    ```
6. install crate
    ```shell script
    cargo install bluetooth_client
    ```

## Development

### Links
* https://docs.ubuntu.com/core/en/stacks/bluetooth/bluez/docs/reference/available-commands
* https://pub.tik.ee.ethz.ch/people/beutel/bluezhowto.pdf
* https://docs.ubuntu.com/core/en/stacks/bluetooth/bluez/docs/
* https://www.pcsuggest.com/linux-bluetooth-setup-hcitool-bluez/
* https://github.com/szeged/blurz
* https://crates.io/crates/blurz
