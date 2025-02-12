use ldap3::{LdapConnAsync, LdapConnSettings};
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let url = "ldaps://localhost:636";
    let username = "jane@example.com";
    let password = "password";
    let is_authenticated = authenticate(&url, &username, &password).await?;

    println!("{} authenticated {}", username, is_authenticated);

    Ok(())
}

// Pass in username and password to authenticate against LDAP
async fn authenticate(url: &str, username: &str, password: &str) -> io::Result<bool> {
    // Connection to the LDAP Server
    let (conn, mut ldap) = LdapConnAsync::with_settings(
        LdapConnSettings::new()
            .set_starttls(true)
            .set_no_tls_verify(true),
        url,
    )
    .await?;
    ldap3::drive!(conn);

    // Attempts a simple bind using the passed in values of username and Password
    let result = ldap
        .simple_bind(username, password)
        .await?
        .success();
    ldap.unbind().await?;

    return Ok(!result.is_err())
}
