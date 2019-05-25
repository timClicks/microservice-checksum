# microservice-checksum

A HTTP microservice that provides a checksum to POSTed data. POST data to it, 
and it will send you back a checksum.

If you have wanted to experiment with building a microservice in Rust,
this might be useful as an example. 

# Usage

## Server-side

Run the executable:

```
$ ROCKET_ENV=prod microservice-checksum
🔧 Configured for production.
    => address: 0.0.0.0
    => port: 8000
    => log: critical
    => workers: 24
    => secret key: generated
    => limits: forms = 32KiB
    => keep-alive: 5s
    => tls: disabled
Warning: environment is 'production', but no `secret_key` is configured
🚀 Rocket has launched from http://0.0.0.0:8000
```

## Client-side

`microservice-checksum` supports 3 endpoints: `/crc16`, `/crc32`, `/crc64`.
From a new console window, you can POST data with `curl` via the `-d` option.

```
$ curl localhost:8000/crc32 -d 'hello?'
1619653635
```

```
$ curl localhost:8000/crc64 -d 'yes!'
6344572155291762688
```

```
$ curl localhost:8000/crc16 -d 'that was easy'
15643
```

(CRC16 comes last because it's the least likely to be used. Sorry CRC16.)

## Custom polynomials

Each endpoint supports using custom polynomials if you wish to be specific:

**CRC16**

`/crc16?polynomial=usb` (default)

`/crc16?polynomial=x25`

`/crc16?polynomial=n` where _n_ [0, 65,535] inclusive

**CRC32**

`/crc16?polynomial=ieee` (default)

`/crc16?polynomial=castagnoli`

`/crc16?polynomial=koopman`

`/crc16?polynomial=n` where _n_ [0, 4,294,967,295] inclusive

**CRC64**

`/crc64?polynomial=ecma` (default)

`/crc64?polynomial=iso`

`/crc64?polynomial=n` where _n_ [0, 18,446,744,073,709,551,615] inclusive

# Performance

Your client should expect a response within 3ms. Usually it's closer to 1.

# Build instructions

## Pre-requisites 

Install Rust, git (just in case) and GCC (Rust needs its linker)

```
sudo apt install build-essential git
curl -fsSL https://sh.rustup.rs | sh
```

Install `microservice-checksum`:

```
cargo install --git https://github.com/timClicks/microservice-checksum
```

## (Optional) Generate a static binary

Install the pre-requisites 

```
sudo apt install musl-tools 
rustup target add x86_64-unknown-linux-musl
```

Build the binary:

```
git clone https://github.com/timClicks/microservice-checksum
cd microservice-checksum
cargo build --release --target x86_64-unknown-linux-musl
```

Verify:

```
target/x86_64-unknown-linux-musl/release/microservice-checksum
```


# Acknowledgements

I did none of the hard work for this. Please thank the maintainers
of the `rocket` and `crc` crates.
