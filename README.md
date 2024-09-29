# Rust ICMP Ping Pong Demo

Message communication demo simulating how ICMP request-reply works on Rust and tokio library.

<img width="904" alt="image" src="https://github.com/user-attachments/assets/47f6c0e0-373c-42ef-971a-399ef73c7df6">

This program will accept incoming TCP request, read the sender address (will always point to localhost / 127.0.0.1 
with randomized port number if you do it locally), read the request content, and will reply back to the sender with
the same content that the sender has.

# How to run
1. Clone this repository
2. Run the Ping Pong Server with the command `cargo run`. You can choose to specify which `port number` the server will listen to
by passing a valid port number as an argument (ex. `cargo run --6393`) or you can also choose to set an `env` variable 
with the key of `PORT` (ex. `PORT=7321`). if both `PORT` env variable is set and you specify the port as an argument 
when running the program, the argument will take precedence. If no port value is specified, the default port value of `8080`
will be used.
3. Open another terminal and send a request to the server address. You can use `netcat` or `telnet` for this.

```bash
# using netcat and the server port is 8080
echo your_message_here | nc 127.0.0.1 8080
```

4. You will receive the same message that you have just sent from the server if your request is succesfully received.
