

██╗     ██████╗ ███████╗███╗   ██╗ ██╗██╗  ██╗
██║     ╚════██╗██╔════╝████╗  ██║███║██║ ██╔╝
██║      █████╔╝███████╗██╔██╗ ██║╚██║█████╔╝
██║      ╚═══██╗╚════██║██║╚██╗██║ ██║██╔═██╗
███████╗██████╔╝███████║██║ ╚████║ ██║██║  ██╗
╚══════╝╚═════╝ ╚══════╝╚═╝  ╚═══╝ ╚═╝╚═╝  ╚═╝

A native, Rust-powered intercepting proxy and passive recon toolkit for web Pentesting and Red Teaming

**Educational and authorized used only. L3sn1k intercepts and modifies live traffic. Only point it at systems
you own or are explicitly authorized to test.**

## Table of contents

 - What is L3sn1k?
 - Architecture
 - Feature Status
 - Requirements
 - Installation
 - Usage
    - Running the proxy GUI
    - Running the recon module
 - Project structure
 - Roadmap
 - Contributing
 - License

## What is L3sn1k?

L3sn1k is a pentesting/Red teaming toolkit in development. Currently we have the beginning of the passive recon tooling and we
are starting to build the osint tooling. The toolkit can be divided into two main parts:
  - The proxy GUI (RUST/iced): a native desktop application that spins up a local MITM HTTP(S) proxy (built on the proxelar crate)
  and exposes captured traffic through a Burp-style interface: Proxy(live traffic log), Repeater( resend captured requests), Target
  and Encoder(Base64 encode/decode).

  - The Recon Toolkit(Python): independent stdlib-adjacent adapters for passive/light-active reconnaissance against a target domain:
  certificate-transparency subdomain enumeration, security header auditing, CORS, misconfiguration probing .wellknown/robots.txt/sitemap.xml
  discovery, and tech-stack fingerprinting. It runs in a randomized module order with randomized browser impersonation to avoid predictable request
  fingerprints.

The long-term goal is a single native pentesting/Red Teaming workbench where the recon results feed directly into the proxy's Target/Scope view.
Today the two pieces are not yet wired together (see Roadmap).

## Architecture

    

## Feature status

## Requirements

## Installation

## Usage

## Project structure

## Roadmap



## Contributing

## License
