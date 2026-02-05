use serde::Deserialize;
use derive_more::Debug;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub ldap_username: String,
    #[debug(skip)]
    pub ldap_password: String,
    pub ldap_server: String,
    pub ldap_base_dn: String,
    pub ldap_user_filter: String,
}
