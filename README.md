# rpi-api-example

Rust with rppal and actix.

Example of controlling output of pins on the raspberry pi by sending requests to a webserver running on it.

```sh
cargo run

#send a GET request to the rpi
#this would toggle a pin on for 2 seconds and then turn it off again
curl "http://10.0.0.43:3002/openvalve?ms=2000&secret=somesecret"
```
