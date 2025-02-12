# rust-ldap-demo

An example of using async LDAP over TLS with rust.

Using:

* ldap3
* tokio
* rustls

## Notes

In order to use `rustls` instead of `native-tls` The cargo specification for
ldap3 sets `default-features=false` to avoid conflicts.
