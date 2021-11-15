## RedPanda Rdkafka Message Delay Reproduction

This repo is to reproduce an issue when using rdkafka in Rust with RedPanda.

_The issue is as follows:_

> When starting the Rust app, there's an unpredicable delay before messages are actually received within the app.

## Prerequisites

- Rust installation with Cargo
- Docker & Docker compose

## To Reproduce

1. Clone this repo

   ```bash
   $ git@github.com:Acidic9/redpanda-delay-repro.git
   $ cd redpanda-delay-repro
   ```

2. Start the redpanda service with docker-compose

   ```bash
   $ docker-compose up -d redpanda
   ```

3. Create the topic `hello`

   ```bash
   $ docker exec -it redpanda-1 rpk topic create hello
   ```

4. Begin producing messages under the topic `hello`

   ```bash
   docker exec -it redpanda-1 rpk topic produce hello -k somekey
   ```

5. **In a new terminal**, start the Rust app

   ```bash
   $ cargo run
   ```

   Once it builds, you should see the text `"Waiting for message..."`

   ```
   /dev/redpanda-delay-repro> cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/app`
   Waiting for message...
   ```

6. In your first terminal, type a message and press enter to produce a new message

7. Notice a (sometimes) long delay before the running Rust app actually receives the message. If there's not much of a delay, try restarting the Rust app and producing a message again.
