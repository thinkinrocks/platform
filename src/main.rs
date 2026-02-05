mod config;

use ldap3::LdapConnAsync;
use log::{info, warn};

use crate::config::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    pretty_env_logger::init();

    let config = envy::from_env::<Config>()?;

    let (conn, mut ldap) = LdapConnAsync::new(&config.ldap_server).await?;
    ldap3::drive!(conn);

    let bind_result = ldap.simple_bind(&config.ldap_username, &config.ldap_password).await?;
    bind_result.success()?;

    info!("Using config: {:?}", config);

    let (results, _) = ldap.search(&config.ldap_base_dn, ldap3::Scope::Subtree, &config.ldap_user_filter, vec!["sAMAccountName"]).await?.success()?;
    info!("Connected to LDAP! {} users fetched the filter", results.len());
    if results.is_empty() {
        warn!("No users matched the LDAP filter, are you sure the filter is correct? The current value is LDAP_USER_FILTER={}", config.ldap_user_filter)
    }

    Ok(())
}
