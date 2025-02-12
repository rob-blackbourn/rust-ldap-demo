# rust-ldap-demo

An example of authenticating using async LDAP over TLS with rust.

Using:

* [ldap3](https://crates.io/crates/ldap3)
* [tokio](https://crates.io/crates/tokio)
* [rustls](https://crates.io/crates/rustls)

## Notes

In order to use `rustls` instead of `native-tls` The cargo specification for
ldap3 sets `default-features=false` to avoid conflicts.
