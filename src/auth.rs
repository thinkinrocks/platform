use std::sync::Arc;

use ldap3::{Ldap, LdapConnAsync};
use log::{info, warn};

pub struct Auth {
    ldap: Ldap,
}

impl Auth {
    pub async fn new(
        server: &str,
        username: &str,
        password: &str,
        base_dn: &str,
        user_filter: &str,
    ) -> Arc<Self> {
        let (conn, mut ldap) = LdapConnAsync::new(&server).await.unwrap();
        ldap3::drive!(conn);

        let bind_result = ldap.simple_bind(&username, &password).await.unwrap();
        bind_result.success().unwrap();

        let (results, _) = ldap
            .search(
                &base_dn,
                ldap3::Scope::Subtree,
                &user_filter,
                vec!["sAMAccountName"],
            )
            .await
            .unwrap()
            .success()
            .unwrap();
        
        info!(
            "Connected to LDAP! {} users fetched the filter",
            results.len()
        );

        if results.is_empty() {
            warn!(
                "No users matched the LDAP filter, are you sure the filter is correct? The current value is LDAP_USER_FILTER={}",
                user_filter
            )
        }

        Arc::new(Auth { ldap })
    }
}
