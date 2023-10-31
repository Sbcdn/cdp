/// INTEGRATION TESTING
/// 
/// This file contains everything you need to perform integration tests. It's organized as follows:
/// `mock_test_endpoints` is the root function from which you run all the integration tests. It will
/// first create a bearer token and mock server. Then, it will go through one endpoint at a time until
/// it finished all endpoints. Each endpoint is processed by the following pattern:
/// 
/// 1) initialize parameters used by the endpoint (example: `address`)
/// 2) initialize a new token matcher (example: `token_matcher`)
/// 3) create a new mock and add it to the mock server to define how the server should process a given request (example: `mm_mock`)
/// 4) send HTTP request, parse the response and check that the response is as expected (example: `mm_test`)
/// 
/// Outside the root function, the endpoint-specific code is found in `endpoints` module. `mock_error` module represents `error.rs` 
/// file. `mock_models` module represents `models.rs` file. Everything else is miscellaneous functions that some endpoints will call.
/// 
/// Inside `endpoints` module, each endpoint has its own sub-module (example: `pub mod get_address_utxos`). Each endpoint module is 
/// organized as follows:
/// 
/// 1) mock
/// 2) endpoint
/// 3) data
/// 4) test
/// 
/// `endpoint` is an imitation of the real endpoint - pretty much copy-paste. 
/// 
/// `mock` puts the imitated endpoint function into a mock along with bearer token and other behaviors that define how the mock 
/// server will respond to a given request. `data` calls the endpoint and parses the response. And then `test` will take the parsed 
/// response and check that it has correct values. 


use cdp::Config;
use cdp::DBSyncProvider;
use cdp::DataProvider;
use rweb::Json;
use rweb::Rejection;
use std::path::PathBuf;
use wiremock::{MockServer, matchers::BearerTokenMatcher};
use tokio;
use dotenv::dotenv;
use cdp::provider::CardanoDataProvider;
use mock_error::RESTError;
use ::log::debug;
use cdp::models::AssetHandle;
use std::str::from_utf8;
use cardano_serialization_lib::utils::from_bignum;
use dcslc::{make_fingerprint, TransactionUnspentOutputs};

#[tokio::test]
async fn mock_test_endpoints() {
    let (bearer_token, mock_server) = mock_config().await;
    
    // get_address_utxos
    let address = "stake_test1upt3q7dvhp9y4std0hc5x3g6zdau3338wwy6l0lldw66axswhd2yr";
    let token_matcher = BearerTokenMatcher::from_token(bearer_token.clone());
    let mock_server = endpoints::get_address_utxos::gau_mock(token_matcher, mock_server, address).await;
    let mock_server = endpoints::get_address_utxos::gau_test(mock_server, &bearer_token, address).await;

    // // address_exists # TODO: Finish implementing this test
    // let addresses = "?addresses=[\"stake_test1uz7kerj6uuudjatalh4dqm5jq9zh6kn6l3weh50egq4nzrs2acc90\",\"stake_test1uzgprdd3w7srnkcddwg9rv4axx6sxx8nd94m9wj4dyrmg6qrrg0nf\",\"stake_test1up3ltqkn8drggvurw9se6j82n7gsjnexph3ut65ax96jy4cxsffy2\"]";
    // let token_matcher = BearerTokenMatcher::from_token(bearer_token.clone());
    // let mock_server = endpoints::address_exists::ae_mock(token_matcher, mock_server, addresses).await;
    // let mock_server = endpoints::address_exists::ae_test(mock_server, &bearer_token, addresses).await;
    
    // mint_metadata
    let fingerprint = "asset1nuesaezyavauyec2kv9se7rxyyzuhzh9vjxq7k";
    let token_matcher = BearerTokenMatcher::from_token(bearer_token.clone());
    let mock_server = endpoints::mint_metadata::mm_mock(token_matcher, mock_server, fingerprint).await;
    let mock_server = endpoints::mint_metadata::mm_test(mock_server, &bearer_token, fingerprint).await;

    // mint_metadata_policy_assetname
    let policy = "cb46e65cad648ace645d32a698f35e4465528d036bcd92e5b0559148";
    let assetname = "000640a036363636";
    let token_matcher = BearerTokenMatcher::from_token(bearer_token.clone());
    let mock_server = endpoints::mint_metadata_policy_assetname::mmpa_mock(token_matcher, mock_server, policy, assetname).await;
    let mock_server = endpoints::mint_metadata_policy_assetname::mmpa_test(mock_server, &bearer_token, policy, assetname).await;

    // // tx_history # TODO: finish implementing this test
    // let addresses = "?addresses=[\"stake_test1upffr8rdlh2f0ccgc9wup4x2qfe49fr85rl6plgguz9wj6g56pnn7\",\"stake_test1uqeezcegh25rcskmmn0gy5fzenczfj34n8qeefhmz6taeycqg9wts\",\"stake_test1upugeuz3jdy0a7hncusutadavzcetdzylgxcldz39hp9n0s0xy0n5\"]";
    // let slot = "&slot=50";
    // let token_matcher = BearerTokenMatcher::from_token(bearer_token.clone());
    // let mock_server = endpoints::tx_history::th_mock(token_matcher, mock_server, addresses, slot).await;
    // let mock_server = endpoints::tx_history::th_test(mock_server, &bearer_token, addresses, slot).await;
    
    // tx_history_discover
    let hash = "0237f7b0e6d705d8c5cc0534e67d66b638823509d6c5f3e8fec9b33eee1dfc74";
    let token_matcher = BearerTokenMatcher::from_token(bearer_token.clone());
    let mock_server = endpoints::tx_history_discover::thd_mock(token_matcher, mock_server, hash).await;
    let mock_server = endpoints::tx_history_discover::thd_test(mock_server, &bearer_token, hash).await;

    // // handle_get_asset_for_addresses # TODO: finish implementing this test
    // let addresses = "?addresses=[\"stake_test1upffr8rdlh2f0ccgc9wup4x2qfe49fr85rl6plgguz9wj6g56pnn7\",\"stake_test1uqeezcegh25rcskmmn0gy5fzenczfj34n8qeefhmz6taeycqg9wts\",\"stake_test1upugeuz3jdy0a7hncusutadavzcetdzylgxcldz39hp9n0s0xy0n5\"]";
    // let token_matcher = BearerTokenMatcher::from_token(bearer_token.clone());
    // let mock_server = endpoints::handle_get_asset_for_addresses::hgafa_mock(token_matcher, mock_server, addresses).await;
    // let mock_server = endpoints::handle_get_asset_for_addresses::hgafa_test(mock_server, &bearer_token, addresses).await;

    // handle_asset_for_stake_address
    let stake_address = "?stake_address=stake_test17zefq2t9ajkcr8a7a4jxzzfnlj8v0thc53tq2h6lq4qf09cy2a0c8";
    let token_matcher = BearerTokenMatcher::from_token(bearer_token.clone());
    let mock_server = endpoints::handle_asset_for_stake_address::hafsa_mock(token_matcher, mock_server, stake_address).await;
    let mock_server = endpoints::handle_asset_for_stake_address::hafsa_test(mock_server, &bearer_token, stake_address).await;

    // retrieve_active_pools
    let page = 1;
    let token_matcher = BearerTokenMatcher::from_token(bearer_token.clone());
    let mock_server = endpoints::retrieve_active_pools::rap_mock(token_matcher, mock_server, page).await;
    let mock_server = endpoints::retrieve_active_pools::rap_test(mock_server, &bearer_token, page).await;
    
    // token_supply 
    let fingerprint = "asset1g0s0fefmm8q9py5klf3vnr85ltx9s2ce5ntnqs";
    let token_matcher = BearerTokenMatcher::from_token(bearer_token.clone());
    let mock_server = endpoints::token_supply::ts_mock(token_matcher, mock_server, fingerprint).await;
    let mock_server = endpoints::token_supply::ts_test(mock_server, &bearer_token, fingerprint).await;

    // // is_nft #TODO: finish implementing this test
    // let fingerprints = "?fingerprints=[%22asset1a0q0grruzd3dm2c9ev890zfaytty8tfcl4qt3a%22,%22asset1h3pg9m9arlwl4l8z3dwg3lwg54j70zqdrjhy88%22,%22asset1fqdnvjcwjcck8t34rvjyj8ccdradp5hkzycxpq%22,%22asset1e83uya776dvqjauy270qnj03899hxxant6jp2g%22]";
    // let token_matcher = BearerTokenMatcher::from_token(bearer_token.clone());
    // let mock_server = endpoints::is_nft::in_mock(token_matcher, mock_server, fingerprints).await;
    // let mock_server = endpoints::is_nft::in_test(mock_server, &bearer_token, fingerprints).await;

    // retrieve_staked_amount
    let epoch = 370;
    let stake_addr = "stake_test17ra3z450j8pcagjj3nkr25l9hfs9wj89v4mtgyqd6tr5ekgscx3eu";
    let token_matcher = BearerTokenMatcher::from_token(bearer_token.clone());
    let mock_server = endpoints::retrieve_staked_amount::rsa_mock(token_matcher, mock_server, epoch, stake_addr).await;
    let mock_server = endpoints::retrieve_staked_amount::rsa_test(mock_server, &bearer_token, stake_addr, epoch).await;

    // retrieve_generated_rewards
    let stake_addr = "stake_test17ra3z450j8pcagjj3nkr25l9hfs9wj89v4mtgyqd6tr5ekgscx3eu";
    let token_matcher = BearerTokenMatcher::from_token(bearer_token.clone());
    let mock_server = endpoints::retrieve_generated_rewards::rgr_mock(token_matcher, mock_server, stake_addr).await;
    let mock_server = endpoints::retrieve_generated_rewards::rgr_test(mock_server, &bearer_token, stake_addr).await;

    // pool_vrf_key_hash
    let pool_hash = "pool1p90428kec03mjdya3k4gv5d20w7lmed7ca0snknef5j977l3y8l";
    let token_matcher = BearerTokenMatcher::from_token(bearer_token.clone());
    let mock_server = endpoints::pool_vrf_key_hash::pvkh_mock(token_matcher, mock_server, pool_hash).await;
    let mock_server = endpoints::pool_vrf_key_hash::pvkh_test(mock_server, &bearer_token, pool_hash).await;

    // pool_blocks_minted
    let pool_hash = "pool1p90428kec03mjdya3k4gv5d20w7lmed7ca0snknef5j977l3y8l";
    let token_matcher = BearerTokenMatcher::from_token(bearer_token.clone());
    let mock_server = endpoints::pool_blocks_minted::pbm_mock(token_matcher, mock_server, pool_hash).await;
    let mock_server = endpoints::pool_blocks_minted::pbm_test(mock_server, &bearer_token, pool_hash).await;

    // pool_blocks_current_epoch
    let pool_hash = "pool1p90428kec03mjdya3k4gv5d20w7lmed7ca0snknef5j977l3y8l";
    let token_matcher = BearerTokenMatcher::from_token(bearer_token.clone());
    let mock_server = endpoints::pool_blocks_current_epoch::pbce_mock(token_matcher, mock_server, pool_hash).await;
    let mock_server = endpoints::pool_blocks_current_epoch::pbce_test(mock_server, &bearer_token, pool_hash).await;
    
    // pool_reward_recipients
    let pool_hash = "pool1p90428kec03mjdya3k4gv5d20w7lmed7ca0snknef5j977l3y8l";
    let token_matcher = BearerTokenMatcher::from_token(bearer_token.clone());
    let mock_server = endpoints::pool_reward_recipients::prr_mock(token_matcher, mock_server, pool_hash).await;
    let mock_server = endpoints::pool_reward_recipients::prr_test(mock_server, &bearer_token, pool_hash).await;

    // pool_last_reward_earned_epoch
    let pool_hash = "pool190ce9qhpzwzveasvngas7kmwqrn54mfvhk9unqmuesykvtadu8l";
    let token_matcher = BearerTokenMatcher::from_token(bearer_token.clone());
    let mock_server = endpoints::pool_last_reward_earned_epoch::plree_mock(token_matcher, mock_server, pool_hash).await;
    let mock_server = endpoints::pool_last_reward_earned_epoch::plree_test(mock_server, &bearer_token, pool_hash).await;

    // pool_declared_pledge
    let pool_hash = "pool1vezalga3ge0mt0xf4txz66ctufk6nrmemhhpshwkhedk5jf0stw";
    let token_matcher = BearerTokenMatcher::from_token(bearer_token.clone());
    let mock_server = endpoints::pool_declared_pledge::pdp_mock(token_matcher, mock_server, pool_hash).await;
    let mock_server = endpoints::pool_declared_pledge::pdp_test(mock_server, &bearer_token, pool_hash).await;

    // pool_margin_cost
    let pool_hash = "pool1vezalga3ge0mt0xf4txz66ctufk6nrmemhhpshwkhedk5jf0stw";
    let token_matcher = BearerTokenMatcher::from_token(bearer_token.clone());
    let mock_server = endpoints::pool_margin_cost::pmc_mock(token_matcher, mock_server, pool_hash).await;
    let mock_server = endpoints::pool_margin_cost::pmc_test(mock_server, &bearer_token, pool_hash).await;

    // pool_fixed_cost
    let pool_hash = "pool1vezalga3ge0mt0xf4txz66ctufk6nrmemhhpshwkhedk5jf0stw";
    let token_matcher = BearerTokenMatcher::from_token(bearer_token.clone());
    let mock_server = endpoints::pool_fixed_cost::pfc_mock(token_matcher, mock_server, pool_hash).await;
    let mock_server = endpoints::pool_fixed_cost::pfc_test(mock_server, &bearer_token, pool_hash).await;

    // pool_owner
    let pool_hash = "pool1vezalga3ge0mt0xf4txz66ctufk6nrmemhhpshwkhedk5jf0stw";
    let token_matcher = BearerTokenMatcher::from_token(bearer_token.clone());
    let mock_server = endpoints::pool_owner::po_mock(token_matcher, mock_server, pool_hash).await;
    let mock_server = endpoints::pool_owner::po_test(mock_server, &bearer_token, pool_hash).await;
    
    // pool_registration
    let pool_hash = "pool1vezalga3ge0mt0xf4txz66ctufk6nrmemhhpshwkhedk5jf0stw";
    let token_matcher = BearerTokenMatcher::from_token(bearer_token.clone());
    let mock_server = endpoints::pool_registration::pr_mock(token_matcher, mock_server, pool_hash).await;
    let mock_server = endpoints::pool_registration::pr_test(mock_server, &bearer_token, pool_hash).await;

    // pool_retirement
    let pool_hash = "pool1nk3uj4fdd6d42tx26y537xaejd76u6xyrn0ql8sr4r9tullk84y";
    let token_matcher = BearerTokenMatcher::from_token(bearer_token.clone());
    let mock_server = endpoints::pool_retirement::pr_mock(token_matcher, mock_server, pool_hash).await;
    let mock_server = endpoints::pool_retirement::pr_test(mock_server, &bearer_token, pool_hash).await;

    // pool_url
    let pool_hash = "pool1xg80gqxp89sllgk6xs6ajwjxkjntwjc2wzc9mjsnzchkgm4z2se";
    let token_matcher = BearerTokenMatcher::from_token(bearer_token.clone());
    let mock_server = endpoints::pool_url::pu_mock(token_matcher, mock_server, pool_hash).await;
    let mock_server = endpoints::pool_url::pu_test(mock_server, &bearer_token, pool_hash).await;

    // pool_ticker
    let pool_hash = "pool1l5u4zh84na80xr56d342d32rsdw62qycwaw97hy9wwsc6axdwla";
    let token_matcher = BearerTokenMatcher::from_token(bearer_token.clone());
    let mock_server = endpoints::pool_ticker::pt_mock(token_matcher, mock_server, pool_hash).await;
    let mock_server = endpoints::pool_ticker::pt_test(mock_server, &bearer_token, pool_hash).await;

    // pool_metadata_json
    let pool_hash = "pool1l5u4zh84na80xr56d342d32rsdw62qycwaw97hy9wwsc6axdwla";
    let token_matcher = BearerTokenMatcher::from_token(bearer_token.clone());
    let mock_server = endpoints::pool_metadata_json::pmj_mock(token_matcher, mock_server, pool_hash).await;
    let mock_server = endpoints::pool_metadata_json::pmj_test(mock_server, &bearer_token, pool_hash).await;

    // pool_name
    let pool_hash = "pool1l5u4zh84na80xr56d342d32rsdw62qycwaw97hy9wwsc6axdwla";
    let token_matcher = BearerTokenMatcher::from_token(bearer_token.clone());
    let mock_server = endpoints::pool_name::pn_mock(token_matcher, mock_server, pool_hash).await;
    let mock_server = endpoints::pool_name::pn_test(mock_server, &bearer_token, pool_hash).await;

    // pool_homepage
    let pool_hash = "pool1l5u4zh84na80xr56d342d32rsdw62qycwaw97hy9wwsc6axdwla";
    let token_matcher = BearerTokenMatcher::from_token(bearer_token.clone());
    let mock_server = endpoints::pool_homepage::ph_mock(token_matcher, mock_server, pool_hash).await;
    let mock_server = endpoints::pool_homepage::ph_test(mock_server, &bearer_token, pool_hash).await;

    // pool_description
    let pool_hash = "pool1l5u4zh84na80xr56d342d32rsdw62qycwaw97hy9wwsc6axdwla";
    let token_matcher = BearerTokenMatcher::from_token(bearer_token.clone());
    let mock_server = endpoints::pool_description::pd_mock(token_matcher, mock_server, pool_hash).await;
    let _mock_server = endpoints::pool_description::pd_test(mock_server, &bearer_token, pool_hash).await;
    
}

mod endpoints {
    pub mod get_address_utxos {
        use super::super::data_provider;
        use super::super::mock_error::RESTError;
        use super::super::CardanoDataProvider;
        use rweb::{Json, Rejection};
        use serde_json::json;
        use wiremock::{MockServer, Mock, ResponseTemplate, matchers::{method, BearerTokenMatcher, path_regex}};
        
        pub async fn gau_mock(
            token_matcher: BearerTokenMatcher, 
            mock_server: MockServer, 
            address: &str,
        ) -> MockServer {
            let response = ResponseTemplate::new(200)
                .set_body_json(gau_endpoint(address.to_string()).await.unwrap());
        
            Mock::given(method("GET"))
                .and(token_matcher)
                .and(path_regex(r"^/api/info/utxos/stake_test1[a-z0-9]{53}$"))
                .respond_with(response)
                .mount(&mock_server)
                .await;
        
            mock_server
        }

        // same as src/server/handler/handler_rest/info.rs/get_address_utxos
        pub async fn gau_endpoint(
            address: String,
        ) -> Result<Json<serde_json::Value>, Rejection> {
            let utxos = data_provider()?
                .get_address_utxos(&address)
                .await
                .map_err(|_| RESTError::Custom("Could not find UTxOs".to_string()))?;
        
            let result = serde_json::to_value(utxos.to_hex().unwrap())
                .map_err(|_| RESTError::Custom("db error, could not get utxos".to_string()))?;
            Ok(rweb::Json::from(result))
        }

        pub async fn gau_data(
            mock_server: MockServer, 
            bearer_token: &str,
            address: &str,
        ) -> (MockServer, u16, Json<serde_json::Value>) {
            let mut response = surf::get(format!("{}/api/info/utxos/{}", &mock_server.uri(), address))
                .header("authorization", "Bearer ".to_string() + bearer_token)
                .await
                .unwrap();
            let status = response.status() as u16;
            let body: Json<serde_json::Value> = response.body_json().await.unwrap();
            (mock_server, status, body)
        }

        pub async fn gau_test(
            mock_server: MockServer, 
            bearer_token: &str,
            address: &str,
        ) -> MockServer {
            let (mock_server, status, body) = gau_data(mock_server, bearer_token, address).await;
            assert_eq!(status, 200);
            assert_eq!(body, Json::from(json!("80")));
            mock_server
        }
    }

    pub mod address_exists {
        use super::super::data_provider;
        use super::super::CardanoDataProvider;
        use rweb::{Json, Rejection};
        use serde_json::json;
        use wiremock::{MockServer, Mock, ResponseTemplate, matchers::{method, BearerTokenMatcher, path_regex}};
        use super::super::parse_string_vec_from_query;
        
        pub async fn ae_mock(
            token_matcher: BearerTokenMatcher, 
            mock_server: MockServer, 
            addresses: &str,
        ) -> MockServer {
            let response = ResponseTemplate::new(200)
                .set_body_json(ae_endpoint(addresses.to_string()).await.unwrap());
        
            Mock::given(method("GET"))
                .and(token_matcher)
                .and(path_regex(r"^/api/info/address/exist?addresses=.*$"))
                .respond_with(response)
                .mount(&mock_server)
                .await;
        
            mock_server
        }

        // same as src/server/handler/handler_rest/info.rs/address_exists
        pub async fn ae_endpoint(
            addresses: String,
        ) -> Result<Json<serde_json::Value>, Rejection> {
            let mut addresses: Vec<String> = parse_string_vec_from_query(&addresses)?;
            let addresses = addresses.iter_mut().map(|address| &address[..]).collect();
        
            let result = data_provider()?
                .addresses_exist(&addresses).await?;
        
            Ok(rweb::Json::from(json!(result)))
        }

        pub async fn ae_data(
            mock_server: MockServer, 
            bearer_token: &str,
            addresses: &str,
        ) -> (MockServer, u16, Json<serde_json::Value>) {
            let mut response = surf::get(format!("{}/api/info/address/exist{}", &mock_server.uri(), addresses))
                .header("authorization", "Bearer ".to_string() + bearer_token)
                .await
                .unwrap();
            let status = response.status() as u16;
            let body: Json<serde_json::Value> = response.body_json().await.unwrap();
            (mock_server, status, body)
        }

        pub async fn ae_test(
            mock_server: MockServer, 
            bearer_token: &str,
            addresses: &str,
        ) -> MockServer {
            let (mock_server, status, body) = ae_data(mock_server, bearer_token, addresses).await;
            assert_eq!(status, 200);
            assert_eq!(body, Json::from(json!("")));
            mock_server
        }
    }
        
    pub mod mint_metadata {
        use super::super::mock_error::RESTError;
        use super::super::CardanoDataProvider;
        use rweb::{Json, Rejection};
        use serde_json::json;
        use wiremock::{MockServer, Mock, ResponseTemplate, matchers::{method, BearerTokenMatcher, path_regex}};
        use cdp::models::TokenInfoView;
        use super::super::data_provider;
        use std::str::FromStr;
        
        pub async fn mm_mock(
            token_matcher: BearerTokenMatcher, 
            mock_server: MockServer, 
            fingerprint: &str,
        ) -> MockServer {
            let response = ResponseTemplate::new(200)
                .set_body_json(mm_endpoint(fingerprint.to_string()).await.unwrap());
        
            Mock::given(method("GET"))
                .and(token_matcher)
                .and(path_regex(r"^/api/info/asset/metadata/asset1[a-z0-9]{38}"))
                .respond_with(response)
                .mount(&mock_server)
                .await;
        
            mock_server
        }

        // same as src/server/handler/handler_rest/info.rs/mm_mock
        pub async fn mm_endpoint(
            fingerprint: String,
        ) -> Result<Json<serde_json::Value>, Rejection> {
            let metadata: TokenInfoView = data_provider()?
                .mint_metadata(&fingerprint).await
                .map_err(|e| RESTError::Custom(e.to_string()))?;
        
            Ok(rweb::Json::from(json!(metadata)))
        }

        pub async fn mm_data(
            mock_server: MockServer, 
            bearer_token: &str,
            fingerprint: &str,
        ) -> (MockServer, u16, Json<serde_json::Value>) {
            let url = format!("{}/api/info/asset/metadata/{}", &mock_server.uri(), fingerprint);
            let mut response = surf::get(url)
                .header("authorization", "Bearer ".to_string() + bearer_token)
                .await
                .unwrap();
            let status = response.status() as u16;
            let body: Json<serde_json::Value> = response.body_json().await.unwrap();
            (mock_server, status, body)
        }

        pub async fn mm_test(
            mock_server: MockServer, 
            bearer_token: &str,
            fingerprint: &str,
        ) -> MockServer {
            let (mock_server, status, body) = mm_data(mock_server, bearer_token, fingerprint).await;
            assert_eq!(status, 200);

            let expected_body = json!({
                "fingerprint": "asset1nuesaezyavauyec2kv9se7rxyyzuhzh9vjxq7k",
                "policy": "b8794febdb27730a6f0476efe6736fb04fb24cd4f9bf02db150701df",
                "tokenname": "Mynth Token",
                "quantity": u64::from_str("100000000000000").unwrap(),
                "meta_key": null,
                "json": null,
                "mint_slot": 16530042,
                "txhash": "55e7326f8c12e7a636cf3553b3fde93ee2351572631beca0ee9f837955713e89"
              });
            assert_eq!(body, Json::from(expected_body));
            mock_server
        }
    }

    pub mod mint_metadata_policy_assetname {
        use super::super::data_provider;
        use super::super::CardanoDataProvider;
        use rweb::{Json, Rejection};
        use serde_json::json;
        use wiremock::{MockServer, Mock, ResponseTemplate, matchers::{method, BearerTokenMatcher, path_regex}};
        use cdp::models::TokenInfoView;
        use dcslc::make_fingerprint;
        
        pub async fn mmpa_mock(
            token_matcher: BearerTokenMatcher, 
            mock_server: MockServer, 
            policy: &str,
            assetname: &str,
        ) -> MockServer {
            let response = ResponseTemplate::new(200)
                .set_body_json(mmpa_endpoint(policy.to_string(), assetname.to_string()).await.unwrap());
        
            Mock::given(method("GET"))
                .and(token_matcher)
                .and(path_regex(r"^/api/info/asset/metadata/[a-z0-9]{56}/[a-zA-Z0-9]+$"))
                .respond_with(response)
                .mount(&mock_server)
                .await;
        
            mock_server
        }

        // same as src/server/handler/handler_rest/info.rs/mint_metadata_policy_assetname
        pub async fn mmpa_endpoint(
            policy: String,
            assetname: String,
        ) -> Result<Json<serde_json::Value>, Rejection> {
            let fingerprint = make_fingerprint(&policy, &assetname).unwrap();
            let metadata: TokenInfoView = data_provider()?
                .mint_metadata(&fingerprint).await?;
            Ok(rweb::Json::from(json!(metadata)))
        }

        pub async fn mmpa_data(
            mock_server: MockServer, 
            bearer_token: &str,
            policy: &str,
            assetname: &str,
        ) -> (MockServer, u16, Json<serde_json::Value>) {
            let mut response = surf::get(format!("{}/api/info/asset/metadata/{}/{}", &mock_server.uri(), policy, assetname))
                .header("authorization", "Bearer ".to_string() + bearer_token)
                .await
                .unwrap();
            let status = response.status() as u16;
            let body: Json<serde_json::Value> = response.body_json().await.unwrap();
            (mock_server, status, body)
        }

        pub async fn mmpa_test(
            mock_server: MockServer, 
            bearer_token: &str,
            policy: &str,
            assetname: &str,
        ) -> MockServer {
            let (mock_server, status, body) = mmpa_data(mock_server, bearer_token, policy, assetname).await;
            assert_eq!(status, 200);

            let expected_body = json!({
                "fingerprint": "asset1l2d04rnvsr7rmjcrfla6q0ma0kfzapuasvj4cj",
                "policy": "cb46e65cad648ace645d32a698f35e4465528d036bcd92e5b0559148",
                "tokenname": "000640a036363636",
                "quantity": 1,
                "meta_key": null,
                "json": null,
                "mint_slot": 824237,
                "txhash": "af5d1ff20f8a45e3be2c53b5dc182add44f9cd6f8ffdec2ada9b2831dc46f226"
              });
            assert_eq!(body, Json::from(expected_body));
            mock_server
        }
    }

    pub mod tx_history {
        use super::super::data_provider;
        use super::super::CardanoDataProvider;
        use rweb::{Json, Rejection};
        use serde_json::json;
        use wiremock::{MockServer, Mock, ResponseTemplate, matchers::{method, BearerTokenMatcher, path_regex}};
        use super::super::parse_string_vec_from_query;
        
        pub async fn th_mock(
            token_matcher: BearerTokenMatcher, 
            mock_server: MockServer, 
            addresses: &str,
            slot: &str,
        ) -> MockServer {
            let response = ResponseTemplate::new(200)
                .set_body_json(th_endpoint(addresses.to_string(), slot.to_string()).await.unwrap());
        
            Mock::given(method("GET"))
                .and(token_matcher)
                .and(path_regex(r"^/api/info/history/address/?addresses=[.]+slot=[.]+$"))
                .respond_with(response)
                .mount(&mock_server)
                .await;
        
            mock_server
        }

        // same as src/server/handler/handler_rest/info.rs/tx_history
        pub async fn th_endpoint(
            addresses: String,
            slot: String,
        ) -> Result<Json<serde_json::Value>, Rejection> {
            let mut addresses: Vec<String> = parse_string_vec_from_query(&addresses)?;
            let addresses = addresses.iter_mut().map(|address| &address[..]).collect();
        
            let slot = slot.parse::<u64>().ok();
        
            let history = data_provider()?
                .tx_history(&addresses, slot).await?;
        
            Ok(rweb::Json::from(json!(history)))
        }

        pub async fn th_data(
            mock_server: MockServer, 
            bearer_token: &str,
            addresses: &str,
            slot: &str,
        ) -> (MockServer, u16, Json<serde_json::Value>) {
            let mut response = surf::get(format!("{}/api/info/history/address/{}/{}", &mock_server.uri(), addresses, slot))
                .header("authorization", "Bearer ".to_string() + bearer_token)
                .await
                .unwrap();
            let status = response.status() as u16;
            let body: Json<serde_json::Value> = response.body_json().await.unwrap();
            (mock_server, status, body)
        }

        pub async fn th_test(
            mock_server: MockServer, 
            bearer_token: &str,
            addresses: &str,
            slot: &str,
        ) -> MockServer {
            let (mock_server, status, body) = th_data(mock_server, bearer_token, addresses, slot).await;
            assert_eq!(status, 200);
            assert_eq!(body, Json::from(json!("")));
            mock_server
        }
    }
        
    pub mod tx_history_discover {
        use super::super::data_provider;
        use rweb::{Json, Rejection};
        use serde_json::json;
        use wiremock::{MockServer, Mock, ResponseTemplate, matchers::{method, BearerTokenMatcher, path_regex}};
        use super::super::debug;
        use super::super::make_error;
        use cdp::dbsync;
        
        pub async fn thd_mock(
            token_matcher: BearerTokenMatcher, 
            mock_server: MockServer, 
            hash: &str,
        ) -> MockServer {
            let response = ResponseTemplate::new(200)
                .set_body_json(thd_endpoint(hash.to_string()).await.unwrap());
        
            Mock::given(method("GET"))
                .and(token_matcher)
                .and(path_regex(r"^/api/info/history/discover/[a-z0-9]{64}$"))
                .respond_with(response)
                .mount(&mock_server)
                .await;
        
            mock_server
        }

        // same as src/server/handler/handler_rest/info.rs/tx_history_discover
        pub async fn thd_endpoint(
            hash: String,
        ) -> Result<Json<serde_json::Value>, Rejection> {
            debug!("Try to discover Transaction: {:?}", hash);
            let tx = dbsync::discover_transaction(
                data_provider()?.provider(), &hash
            ).await;
        
            match tx {
                Ok(tx) => Ok(rweb::Json::from(json!(tx))),
                Err(e) => make_error(e.to_string(), None, None),
            }
        }

        pub async fn thd_data(
            mock_server: MockServer, 
            bearer_token: &str,
            hash: &str,
        ) -> (MockServer, u16, Json<serde_json::Value>) {
            let mut response = surf::get(format!("{}/api/info/history/discover/{}", &mock_server.uri(), hash))
                .header("authorization", "Bearer ".to_string() + bearer_token)
                .await
                .unwrap();
            let status = response.status() as u16;
            let body: Json<serde_json::Value> = response.body_json().await.unwrap();
            (mock_server, status, body)
        }

        pub async fn thd_test(
            mock_server: MockServer, 
            bearer_token: &str,
            hash: &str,
        ) -> MockServer {
            let (mock_server, status, body) = thd_data(mock_server, bearer_token, hash).await;
            assert_eq!(status, 200);
            let expected_body = json!({
                "hash": "0237f7b0e6d705d8c5cc0534e67d66b638823509d6c5f3e8fec9b33eee1dfc74",
                "block": "f758397b2ffdb129415b4f484a9b34212821c02a32eb6c6c8080187d2cc419a4",
                "slot": 32244510,
                "inputs": [
                  {
                    "hash": "e9cbbe69989eca48bd13805e2f61a38942b908b66bfd8f901deeca2a93bb009c",
                    "index": 1,
                    "address": "addr_test1qpmejsnt636tx3cka22efhlaf0klry2jpnerlxjvnvw7sfp4sr2mwncdhhqj4z5y9ld7y3nmafka44ssxqfamw3j3ugsmrvtj2",
                    "amount": {
                      "coin": 903320203,
                      "multiasset": null
                    },
                    "plutus_data": null,
                    "script_ref": null
                  }
                ],
                "reference_inputs": null,
                "outputs": [
                  {
                    "hash": "0237f7b0e6d705d8c5cc0534e67d66b638823509d6c5f3e8fec9b33eee1dfc74",
                    "index": 1,
                    "address": "addr_test1qpmejsnt636tx3cka22efhlaf0klry2jpnerlxjvnvw7sfp4sr2mwncdhhqj4z5y9ld7y3nmafka44ssxqfamw3j3ugsmrvtj2",
                    "amount": {
                      "coin": 853151886,
                      "multiasset": null
                    },
                    "plutus_data": null,
                    "script_ref": null
                  },
                  {
                    "hash": "0237f7b0e6d705d8c5cc0534e67d66b638823509d6c5f3e8fec9b33eee1dfc74",
                    "index": 0,
                    "address": "addr_test1qz8hz6499h7nf05k7gechj276phxprt0vym3js63gaufqqlzu6v4xmf9upm268mnrqt7ftcvljjyd5zve0my7ec2dm2saxkmly",
                    "amount": {
                      "coin": 50000000,
                      "multiasset": null
                    },
                    "plutus_data": null,
                    "script_ref": null
                  }
                ],
                "withdrawals": null,
                "metadata": null,
                "stake_registration": null,
                "stake_deregistration": null,
                "script": null,
                "collateral_tx_in": null,
                "collateral_tx_out": null,
                "fee": 168317,
                "cbor": null
              });
            assert_eq!(body, Json::from(expected_body));
            mock_server
        }
    }

    pub mod handle_get_asset_for_addresses {
        use rweb::{Json, Rejection};
        use serde_json::json;
        use wiremock::{MockServer, Mock, ResponseTemplate, matchers::{method, BearerTokenMatcher, path_regex}};
        use super::super::debug;
        use super::super::parse_string_vec_from_query;
        use super::super::get_asset_for_addresses;
        use super::super::make_error;
        
        pub async fn hgafa_mock(
            token_matcher: BearerTokenMatcher, 
            mock_server: MockServer, 
            addresses: &str,
        ) -> MockServer {
            let response = ResponseTemplate::new(200)
                .set_body_json(hgafa_endpoint(addresses.to_string()).await.unwrap());
        
            Mock::given(method("GET"))
                .and(token_matcher)
                .and(path_regex(r"^/api/info/addresses/assets/?addresses=[.]+$"))
                .respond_with(response)
                .mount(&mock_server)
                .await;
        
            mock_server
        }

        // same as src/server/handler/handler_rest/info.rs/handle_get_asset_for_addresses
        pub async fn hgafa_endpoint(
            addresses: String,
        ) -> Result<Json<serde_json::Value>, Rejection> {
            debug!("{addresses:?}");
            let addresses = match parse_string_vec_from_query(&addresses) {
                Ok(u) => u,
                Err(e) => {
                    return make_error(
                        e.to_string(),
                        Some(1001),
                        Some("Could not parse addresses from query parameter"),
                    );
                }
            };
        
            Ok(rweb::Json::from(json!(
                get_asset_for_addresses(&addresses).await?
            )))
        }

        pub async fn hgafa_data(
            mock_server: MockServer, 
            bearer_token: &str,
            addresses: &str,
        ) -> (MockServer, u16, Json<serde_json::Value>) {
            let mut response = surf::get(format!("{}/api/info/addresses/assets/{}", &mock_server.uri(), addresses))
                .header("authorization", "Bearer ".to_string() + bearer_token)
                .await
                .unwrap();
            let status = response.status() as u16;
            let body: Json<serde_json::Value> = response.body_json().await.unwrap();
            (mock_server, status, body)
        }

        pub async fn hgafa_test(
            mock_server: MockServer, 
            bearer_token: &str,
            addresses: &str,
        ) -> MockServer {
            let (mock_server, status, body) = hgafa_data(mock_server, bearer_token, addresses).await;
            assert_eq!(status, 200);
            assert_eq!(body, Json::from(json!("")));
            mock_server
        }
    }

    pub mod handle_asset_for_stake_address {
        use super::super::data_provider;
        use super::super::make_error;
        use rweb::{Json, Rejection};
        use serde_json::json;
        use wiremock::matchers::path;
        use wiremock::matchers::query_param;
        use wiremock::{MockServer, Mock, ResponseTemplate, matchers::{method, BearerTokenMatcher, path_regex}};
        use super::super::debug;
        use cdp::models::AssetHandle;
        use cardano_serialization_lib::utils::from_bignum;
        use cdp::dbsync::get_stake_address_utxos_dep;
        use dcslc::make_fingerprint;
        use std::str::from_utf8;
        use std::str::FromStr;
        
        pub async fn hafsa_mock(
            token_matcher: BearerTokenMatcher, 
            mock_server: MockServer, 
            stake_address: &str,
        ) -> MockServer {
            let response = ResponseTemplate::new(200)
                .set_body_json(hafsa_endpoint(stake_address.to_string()).await.unwrap());

            Mock::given(method("GET"))
                .and(token_matcher)
                .and(path("/api/info/address/stake/assets/"))
                .and(query_param("stake_address", "stake_test17zefq2t9ajkcr8a7a4jxzzfnlj8v0thc53tq2h6lq4qf09cy2a0c8"))
                .respond_with(response)
                .mount(&mock_server)
                .await;
        
            mock_server
        }

        // same as src/server/handler/handler_rest/info.rs/handle_asset_for_stake_address
        pub async fn hafsa_endpoint(
            stake_address: String
        ) -> Result<Json<serde_json::Value>, Rejection> {
            debug!("{stake_address:?}");
            let split = stake_address.split('=').collect::<Vec<&str>>();
            let bstake_addr = match dcslc::addr_from_str(split[1]) {
                Ok(s) => s,
                Err(e) => {
                    return make_error(
                        e.to_string(),
                        Some(1002),
                        Some("The provided stake address is invalid"),
                    );
                }
            };
            let reward_address = match dcslc::get_stakeaddr_from_addr(&bstake_addr) {
                Ok(r) => r,
                Err(e) => {
                    return make_error(
                        e.to_string(),
                        Some(1003),
                        Some("The provided address is not a stake address"),
                    );
                }
            };
        
            // dp
            //.wallet_utxos(&reward_address.to_bech32(None).unwrap())
            let utxos = match get_stake_address_utxos_dep(
                data_provider()?.provider(),
                &reward_address.to_bech32(None).unwrap(),
            ) {
                Ok(u) => u,
                Err(e) => {
                    return make_error(
                        e.to_string(),
                        Some(2001),
                        Some(&format!(
                            "Could not retrieve utxos for stake address: {:?}",
                            reward_address.to_bech32(None).unwrap()
                        )),
                    );
                }
            };
        
            let mut handles = Vec::<AssetHandle>::new();
            for u in utxos {
                let v = u.output().amount();
                let ada = v.coin();
                handles.push(AssetHandle {
                    fingerprint: None,
                    policy: None,
                    tokenname: None,
                    amount: from_bignum(&ada),
                    metadata: None,
                });
                if let Some(multis) = v.multiasset() {
                    let policies = multis.keys();
                    for p in 0..policies.len() {
                        let policy = policies.get(p);
                        if let Some(assets) = multis.get(&policy) {
                            let k = assets.keys();
                            for a in 0..k.len() {
                                let asset = k.get(a);
                                let amt = assets.get(&asset).unwrap();
                                let fingerprint =
                                    make_fingerprint(&policy.to_hex(), &hex::encode(asset.name())).unwrap();
                                //let metadata = dp.mint_metadata(&fingerprint).await.unwrap();
                                handles.push(AssetHandle {
                                    fingerprint: Some(fingerprint),
                                    policy: Some(policy.to_hex()),
                                    tokenname: Some(match from_utf8(&asset.name()) {
                                        Ok(s) => s.to_owned(),
                                        Err(_) => hex::encode(&asset.name()),
                                    }),
                                    amount: from_bignum(&amt),
                                    metadata: None, //metadata.json,
                                })
                            }
                        }
                    }
                }
            }
            debug!("Handles: {:?}", handles);
            let mut handles_summed = Vec::<AssetHandle>::new();
        
            for h in &handles {
                if !handles_summed.contains(h) {
                    let sum = handles.iter().fold(AssetHandle::new_empty(), |mut acc, f| {
                        if h == f {
                            acc.amount = acc.amount.checked_add(f.amount).unwrap();
        
                            if acc.metadata.is_none() && f.metadata.is_some() {
                                acc.metadata = h.metadata.clone()
                            }
                            if acc.fingerprint.is_none() && f.fingerprint.is_some() {
                                acc.fingerprint = h.fingerprint.clone()
                            }
                            if acc.policy.is_none() && f.policy.is_some() {
                                acc.policy = h.policy.clone()
                            }
                            if acc.tokenname.is_none() && f.tokenname.is_some() {
                                acc.tokenname = h.tokenname.clone()
                            }
                        }
                        acc
                    });
                    handles_summed.push(sum)
                }
            }
            debug!("Handles summed: {:?}", handles_summed);
            Ok(rweb::Json::from(json!(handles_summed)))
        }

        pub async fn hafsa_data(
            mock_server: MockServer, 
            bearer_token: &str,
            stake_address: &str,
        ) -> (MockServer, u16, Json<serde_json::Value>) {
            let mut response = surf::get(format!("{}/api/info/address/stake/assets/{}", &mock_server.uri(), stake_address))
                .header("authorization", "Bearer ".to_string() + bearer_token)
                .await
                .unwrap();
            let status = response.status() as u16;
            let body: Json<serde_json::Value> = response.body_json().await.unwrap();
            (mock_server, status, body)
        }

        pub async fn hafsa_test(
            mock_server: MockServer, 
            bearer_token: &str,
            stake_address: &str,
        ) -> MockServer {
            let (mock_server, status, body) = hafsa_data(mock_server, bearer_token, stake_address).await;
            assert_eq!(status, 200);

            let expected_body = json!([
                {
                    "fingerprint": None::<&str>,
                    "policy": None::<&str>,
                    "tokenname": None::<&str>,
                    "amount": 1127999452,
                    "metadata": None::<&str>
                },
                {
                    "fingerprint": "asset17xpe4pkrkulwjs6grlv9desu74m40rd22ue85q",
                    "policy": "d35c752af635d9e9cb79aea44537a57a5ecd91e23133cd7f210f0070",
                    "tokenname": "mTOSI",
                    "amount": i64::from_str("2499980765").unwrap(),
                    "metadata": None::<&str>
                }
            ]);
            assert_eq!(body, Json::from(expected_body));
            mock_server
        }
    }

    pub mod retrieve_active_pools {
        use super::super::data_provider;
        use super::super::make_error;
        use cdp::dbsync;
        use rweb::{Json, Rejection};
        use serde_json::json;
        use wiremock::{MockServer, Mock, ResponseTemplate, matchers::{method, BearerTokenMatcher, path_regex}};
        use cdp::models::PoolView;
        
        pub async fn rap_mock(
            token_matcher: BearerTokenMatcher, 
            mock_server: MockServer, 
            page: usize,
        ) -> MockServer {
            let response = ResponseTemplate::new(200)
                .set_body_json(rap_endpoint(page).await.unwrap());
        
            Mock::given(method("GET"))
                .and(token_matcher)
                .and(path_regex(r"^/api/info/pools/[0-9]+$"))
                .respond_with(response)
                .mount(&mock_server)
                .await;
        
            mock_server
        }

        // same as src/server/handler/handler_rest/info.rs/retrieve_active_pools
        pub async fn rap_endpoint(
            page: usize,
        ) -> Result<Json<serde_json::Value>, Rejection> {
            let pools = dbsync::get_pools(
                data_provider()?.provider()
            ).await?;
            let pools_paged: Vec<Vec<PoolView>> = pools.chunks(100).map(|s| s.into()).collect();
            if pools_paged.len() < page {
                return make_error(
                    format!("Page {} is the last page", pools_paged.len()),
                    None,
                    None,
                );
            }
            Ok(rweb::Json::from(json!(pools_paged[page])))
        }

        pub async fn rap_data(
            mock_server: MockServer, 
            bearer_token: &str,
            page: usize,
        ) -> (MockServer, u16, Json<serde_json::Value>) {
            let mut response = surf::get(format!("{}/api/info/pools/{}", &mock_server.uri(), page))
                .header("authorization", "Bearer ".to_string() + bearer_token)
                .await
                .unwrap();
            let status = response.status() as u16;
            let body: Json<serde_json::Value> = response.body_json().await.unwrap();
            (mock_server, status, body)
        }

        pub async fn rap_test(
            mock_server: MockServer, 
            bearer_token: &str,
            page: usize,
        ) -> MockServer {
            let (mock_server, status, _body) = rap_data(mock_server, bearer_token, page).await;
            assert_eq!(status, 200);

            mock_server
        }
    }

    pub mod token_supply {
        use super::super::data_provider;
        use super::super::make_error;
        use cdp::dbsync;
        use rweb::{Json, Rejection};
        use serde_json::json;
        use wiremock::{MockServer, Mock, ResponseTemplate, matchers::{method, BearerTokenMatcher, path_regex}};
        
        pub async fn ts_mock(
            token_matcher: BearerTokenMatcher, 
            mock_server: MockServer, 
            fingerprint: &str,
        ) -> MockServer {
            let response = ResponseTemplate::new(200)
                .set_body_json(ts_endpoint(fingerprint.to_string()).await.unwrap());
        
            Mock::given(method("GET"))
                .and(token_matcher)
                .and(path_regex(r"^/api/info/tokens/supply/asset1[a-z0-9]{38}$"))
                .respond_with(response)
                .mount(&mock_server)
                .await;
        
            mock_server
        }

        // same as src/server/handler/handler_rest/info.rs/token_supply
        pub async fn ts_endpoint(
            fingerprint: String,
        ) -> Result<Json<serde_json::Value>, Rejection> {
            let supply = dbsync::token_supply(data_provider()?.provider(), &fingerprint).await;
                if let Err(e) = &supply {
                    return make_error(
                        format!("Could not get supply for {:?}", e.to_string()),
                        None,
                        None,
                    );
                }
                Ok(rweb::Json::from(json!(supply?)))
        }

        pub async fn ts_data(
            mock_server: MockServer, 
            bearer_token: &str,
            fingerprint: &str,
        ) -> (MockServer, u16, Json<serde_json::Value>) {
            let mut response = surf::get(format!("{}/api/info/tokens/supply/{}", &mock_server.uri(), fingerprint))
                .header("authorization", "Bearer ".to_string() + bearer_token)
                .await
                .unwrap();
            let status = response.status() as u16;
            let body: Json<serde_json::Value> = response.body_json().await.unwrap();
            (mock_server, status, body)
        }

        pub async fn ts_test(
            mock_server: MockServer, 
            bearer_token: &str,
            fingerprint: &str,
        ) -> MockServer {
            let (mock_server, status, body) = ts_data(mock_server, bearer_token, fingerprint).await;
            assert_eq!(status, 200);
            assert_eq!(body, Json::from(json!("3")));
            mock_server
        }
    }

    pub mod is_nft {
        use super::super::data_provider;
        use rweb::{Json, Rejection};
        use serde_json::json;
        use wiremock::{MockServer, Mock, ResponseTemplate, matchers::{method, BearerTokenMatcher, path_regex, query_param}};
        use super::super::debug;
        use super::super::parse_string_vec_from_query;
        use super::super::make_error;
        use cdp::dbsync;
        
        pub async fn in_mock(
            token_matcher: BearerTokenMatcher, 
            mock_server: MockServer, 
            fingerprints: &str,
        ) -> MockServer {
            let response = ResponseTemplate::new(200)
                .set_body_json(in_endpoint(fingerprints.to_string()).await.unwrap());


            Mock::given(method("GET"))
                .and(token_matcher)
                // .and(query_param(query_key, query_val))
                .and(path_regex(r#"^/api/info/tokens/isNft/?fingerprints=\[.*\]+$"#))
                .respond_with(response)
                .mount(&mock_server)
                .await;
        
            mock_server
        }

        // same as src/server/handler/handler_rest/info.rs/is_nft
        pub async fn in_endpoint(
            fingerprints: String
        ) -> Result<Json<serde_json::Value>, Rejection> {
            let f = match parse_string_vec_from_query(&fingerprints) {
                Ok(u) => u,
                Err(e) => {
                    return make_error(
                        e.to_string(),
                        Some(1004),
                        Some("Could not parse list of fingerprints from query parameter"),
                    );
                }
            };
            debug!("Try to execute query");
            let supply = dbsync::is_nft(
                data_provider()?.provider(),
                &f.iter().map(|n| &**n).collect::<Vec<&str>>()[..],
            )
            .await;
            debug!("Received query results: {supply:?}");
            if let Err(e) = &supply {
                return make_error(
                    format!("Could not get supply for {f:?}, error: {e}"),
                    None,
                    None,
                );
            }
            Ok(rweb::Json::from(json!(supply?)))
        }

        pub async fn in_data(
            mock_server: MockServer, 
            bearer_token: &str,
            fingerprints: &str,
        ) -> (MockServer, u16, Json<serde_json::Value>) {
            let uri = format!("{}/api/info/tokens/isNft/{}", &mock_server.uri(), fingerprints);
            let mut response = surf::get(uri)
                .query(&vec!["?fingerprints=[asset1a0q0grruzd3dm2c9ev890zfaytty8tfcl4qt3a,asset1h3pg9m9arlwl4l8z3dwg3lwg54j70zqdrjhy88,asset1fqdnvjcwjcck8t34rvjyj8ccdradp5hkzycxpq,asset1e83uya776dvqjauy270qnj03899hxxant6jp2g]"]).unwrap()
                .header("authorization", "Bearer ".to_string() + bearer_token).await.unwrap();
            let status = response.status() as u16;
            let body: Json<serde_json::Value> = response.body_json().await.unwrap();
            (mock_server, status, body)
        }

        pub async fn in_test(
            mock_server: MockServer, 
            bearer_token: &str,
            fingerprints: &str,
        ) -> MockServer {
            let (mock_server, status, body) = in_data(mock_server, bearer_token, fingerprints).await;
            assert_eq!(status, 200);
            assert_eq!(body, Json::from(json!([true, true, true, false])));
            mock_server
        }
    }

    pub mod retrieve_staked_amount {
        use super::super::data_provider;
        use super::super::mock_error::RESTError;
        use super::super::CardanoDataProvider;
        use rweb::{Json, Rejection};
        use serde_json::json;
        use wiremock::{MockServer, Mock, ResponseTemplate, matchers::{method, BearerTokenMatcher, path_regex}};
        
        pub async fn rsa_mock(
            token_matcher: BearerTokenMatcher, 
            mock_server: MockServer, 
            epoch: i32,
            stake_addr: &str,
        ) -> MockServer {
            let response = ResponseTemplate::new(200)
                .set_body_json(rsa_endpoint(epoch, stake_addr.to_string()).await.unwrap());
        
            Mock::given(method("GET"))
                .and(token_matcher)
                .and(path_regex(r"^/api/info/epoch/stake/amount/stake_test1[a-z0-9]{53}/[0-9]+$"))
                .respond_with(response)
                .mount(&mock_server)
                .await;
        
            mock_server
        }

        // same as src/server/handler/handler_rest/info.rs/retrieve_staked_amount
        pub async fn rsa_endpoint(
            epoch: i32,
            stake_addr: String,
        ) -> Result<Json<serde_json::Value>, Rejection> {
            let staked_amount = data_provider()?
                .retrieve_staked_amount(epoch, &stake_addr)
                .await
                .map_err(|_| RESTError::Custom("Couldn't find staked amount".to_string()))?;
        
            Ok(rweb::Json::from(json!(staked_amount)))
        }

        pub async fn rsa_data(
            mock_server: MockServer, 
            bearer_token: &str,
            stake_addr: &str,
            epoch: i32,
        ) -> (MockServer, u16, Json<serde_json::Value>) {
            let mut response = surf::get(format!("{}/api/info/epoch/stake/amount/{}/{}", &mock_server.uri(), stake_addr, epoch))
                .header("authorization", "Bearer ".to_string() + bearer_token)
                .await
                .unwrap();
            let status = response.status() as u16;
            let body: Json<serde_json::Value> = response.body_json().await.unwrap();
            (mock_server, status, body)
        }

        pub async fn rsa_test(
            mock_server: MockServer, 
            bearer_token: &str,
            stake_addr: &str,
            epoch: i32,
        ) -> MockServer {
            let (mock_server, status, body) = rsa_data(mock_server, bearer_token, stake_addr, epoch).await;
            assert_eq!(status, 200);
            assert_eq!(body, Json::from(json!("4427505")));
            mock_server
        }
    }

    pub mod retrieve_generated_rewards {
        use super::super::data_provider;
        use super::super::mock_error::RESTError;
        use super::super::CardanoDataProvider;
        use rweb::{Json, Rejection};
        use serde_json::json;
        use wiremock::{MockServer, Mock, ResponseTemplate, matchers::{method, BearerTokenMatcher, path_regex}};
        
        pub async fn rgr_mock(
            token_matcher: BearerTokenMatcher, 
            mock_server: MockServer, 
            stake_addr: &str,
        ) -> MockServer {
            let response = ResponseTemplate::new(200)
                .set_body_json(rgr_endpoint(stake_addr.to_string()).await.unwrap());
        
            Mock::given(method("GET"))
                .and(token_matcher)
                .and(path_regex(r"^/api/info/reward/amount/stake_test1[a-z0-9]{53}$"))
                .respond_with(response)
                .mount(&mock_server)
                .await;
        
            mock_server
        }

        // same as src/server/handler/handler_rest/info.rs/retrieve_generated_rewards
        pub async fn rgr_endpoint(
            stake_addr: String,
        ) -> Result<Json<serde_json::Value>, Rejection> {
            let generated_rewards = data_provider()?
                .retrieve_generated_rewards(&stake_addr)
                .await
                .map_err(|_| RESTError::Custom("Couldn't find generated rewards".to_string()))?;
        
            Ok(rweb::Json::from(json!(generated_rewards)))
        }

        pub async fn rgr_data(
            mock_server: MockServer, 
            bearer_token: &str,
            stake_addr: &str,
        ) -> (MockServer, u16, Json<serde_json::Value>) {
            let mut response = surf::get(format!("{}/api/info/reward/amount/{}", &mock_server.uri(), stake_addr))
                .header("authorization", "Bearer ".to_string() + bearer_token)
                .await
                .unwrap();
            let status = response.status() as u16;
            let body: Json<serde_json::Value> = response.body_json().await.unwrap();

            (mock_server, status, body)
        }

        pub async fn rgr_test(
            mock_server: MockServer, 
            bearer_token: &str,
            stake_addr: &str,
        ) -> MockServer {
            let (mock_server, status, body) = rgr_data(mock_server, bearer_token, stake_addr).await;
            assert_eq!(status, 200);
            
            let obtained_body_first_object = body
                .into_inner()
                .as_array()
                .unwrap()[0]
                .clone();

            let expected_body_first_object = json!({
                    "amount": 2980,
                    "earned_epoch": 366,
                    "spendable_epoch": 368
            });

            // New objects are added to the response body over time, so we take just 1st object
            assert_eq!(obtained_body_first_object, expected_body_first_object); 

            mock_server
        }
    }

    pub mod pool_vrf_key_hash {
        use super::super::data_provider;
        use super::super::mock_error::RESTError;
        use super::super::CardanoDataProvider;
        use rweb::{Json, Rejection};
        use serde_json::json;
        use wiremock::{MockServer, Mock, ResponseTemplate, matchers::{method, BearerTokenMatcher, path_regex}};
        
        pub async fn pvkh_mock(
            token_matcher: BearerTokenMatcher, 
            mock_server: MockServer, 
            pool_hash: &str,
        ) -> MockServer {
            let response = ResponseTemplate::new(200)
                .set_body_json(pvkh_endpoint(pool_hash.to_string()).await.unwrap());

            Mock::given(method("GET"))
                .and(token_matcher)
                .and(path_regex(r"^/api/info/pool/vrf_key_hash/pool1[a-z0-9]{51}$"))
                .respond_with(response)
                .mount(&mock_server)
                .await;
        
            mock_server
        }

        // same as src/server/handler/handler_rest/info.rs/pool_vrf_key_hash
        pub async fn pvkh_endpoint(
            pool_hash: String,
        ) -> Result<Json<serde_json::Value>, Rejection> {
            let pool_vrf_key_hash = data_provider()?
                .pool_vrf_key_hash(&pool_hash)
                .await
                .map_err(|_| RESTError::Custom("Couldn't find pools VRF key hash".to_string()))?;
            Ok(rweb::Json::from(json!(pool_vrf_key_hash)))
        }

        pub async fn pvkh_data(
            mock_server: MockServer, 
            bearer_token: &str,
            pool_hash: &str,
        ) -> (MockServer, u16, Json<serde_json::Value>) {
            let mut response = surf::get(format!("{}/api/info/pool/vrf_key_hash/{}", &mock_server.uri(), pool_hash))
                .header("authorization", "Bearer ".to_string() + bearer_token)
                .await
                .unwrap();
            let status = response.status() as u16;
            let body: Json<serde_json::Value> = response.body_json().await.unwrap();
            (mock_server, status, body)
        }

        pub async fn pvkh_test(
            mock_server: MockServer, 
            bearer_token: &str,
            pool_hash: &str,
        ) -> MockServer {
            let (mock_server, status, _body) = pvkh_data(mock_server, bearer_token, pool_hash).await;
            assert_eq!(status, 200);

            mock_server
        }
    }

    pub mod pool_blocks_minted {
        use super::super::data_provider;
        use super::super::mock_error::RESTError;
        use super::super::CardanoDataProvider;
        use rweb::{Json, Rejection};
        use serde_json::json;
        use wiremock::{MockServer, Mock, ResponseTemplate, matchers::{method, BearerTokenMatcher, path_regex}};
        
        pub async fn pbm_mock(
            token_matcher: BearerTokenMatcher, 
            mock_server: MockServer, 
            pool_hash: &str,
        ) -> MockServer {
            let response = ResponseTemplate::new(200)
                .set_body_json(pbm_endpoint(pool_hash.to_string()).await.unwrap());
        
            Mock::given(method("GET"))
                .and(token_matcher)
                .and(path_regex(r"^/api/info/pool/blocks_minted/pool1[a-z0-9]{51}$"))
                .respond_with(response)
                .mount(&mock_server)
                .await;
        
            mock_server
        }

        // same as src/server/handler/handler_rest/info.rs/pool_blocks_minted
        pub async fn pbm_endpoint(
            pool_hash: String,
        ) -> Result<Json<serde_json::Value>, Rejection> {
            let pool_blocks_minted = data_provider()?
                .pool_blocks_minted(&pool_hash)
                .await
                .map_err(|_| RESTError::Custom("Couldn't find the total number of blocks minted by the given pool".to_string()))?;
        
            Ok(rweb::Json::from(json!(pool_blocks_minted)))
        }

        pub async fn pbm_data(
            mock_server: MockServer, 
            bearer_token: &str,
            pool_hash: &str,
        ) -> (MockServer, u16, Json<serde_json::Value>) {
            let mut response = surf::get(format!("{}/api/info/pool/blocks_minted/{}", &mock_server.uri(), pool_hash))
                .header("authorization", "Bearer ".to_string() + bearer_token)
                .await
                .unwrap();
            let status = response.status() as u16;
            let body: Json<serde_json::Value> = response.body_json().await.unwrap();
            (mock_server, status, body)
        }

        pub async fn pbm_test(
            mock_server: MockServer, 
            bearer_token: &str,
            pool_hash: &str,
        ) -> MockServer {
            let (mock_server, status, body) = pbm_data(mock_server, bearer_token, pool_hash).await;
            assert_eq!(status, 200);
            assert_eq!(body, Json::from(json!(7942)));
            mock_server
        }
    }

    pub mod pool_blocks_current_epoch {
        use super::super::data_provider;
        use super::super::mock_error::RESTError;
        use super::super::CardanoDataProvider;
        use rweb::{Json, Rejection};
        use serde_json::json;
        use wiremock::{MockServer, Mock, ResponseTemplate, matchers::{method, BearerTokenMatcher, path_regex}};
        
        pub async fn pbce_mock(
            token_matcher: BearerTokenMatcher, 
            mock_server: MockServer, 
            pool_hash: &str,
        ) -> MockServer {
            let response = ResponseTemplate::new(200)
                .set_body_json(pbce_endpoint(pool_hash.to_string()).await.unwrap());
        
            Mock::given(method("GET"))
                .and(token_matcher)
                .and(path_regex(r"^/api/info/pool/blocks_current_epoch/pool1[a-z0-9]{51}$"))
                .respond_with(response)
                .mount(&mock_server)
                .await;
        
            mock_server
        }

        // same as src/server/handler/handler_rest/info.rs/pool_blocks_current_epoch
        pub async fn pbce_endpoint(
            pool_hash: String,
        ) -> Result<Json<serde_json::Value>, Rejection> {
            let pool_blocks_current_epoch = data_provider()?
                .pool_blocks_current_epoch(&pool_hash)
                .await
                .map_err(|_| RESTError::Custom("Couldn't find the quantity of blocks minted by the given pool in current epoch".to_string()))?;
        
            Ok(rweb::Json::from(json!(pool_blocks_current_epoch)))
        }

        pub async fn pbce_data(
            mock_server: MockServer, 
            bearer_token: &str,
            pool_hash: &str,
        ) -> (MockServer, u16, Json<serde_json::Value>) {
            let mut response = surf::get(format!("{}/api/info/pool/blocks_current_epoch/{}", &mock_server.uri(), pool_hash))
                .header("authorization", "Bearer ".to_string() + bearer_token)
                .await
                .unwrap();
            let status = response.status() as u16;
            let body: Json<serde_json::Value> = response.body_json().await.unwrap();
            (mock_server, status, body)
        }

        pub async fn pbce_test(
            mock_server: MockServer, 
            bearer_token: &str,
            pool_hash: &str,
        ) -> MockServer {
            let (mock_server, status, body) = pbce_data(mock_server, bearer_token, pool_hash).await;
            assert_eq!(status, 200);
            assert_eq!(body, Json::from(json!(0)));
            mock_server
        }
    }
        
    pub mod pool_reward_recipients {
        use super::super::data_provider;
        use super::super::mock_error::RESTError;
        use super::super::CardanoDataProvider;
        use rweb::{Json, Rejection};
        use serde_json::json;
        use wiremock::{MockServer, Mock, ResponseTemplate, matchers::{method, BearerTokenMatcher, path_regex}};
        
        pub async fn prr_mock(
            token_matcher: BearerTokenMatcher, 
            mock_server: MockServer, 
            pool_hash: &str,
        ) -> MockServer {
            let response = ResponseTemplate::new(200)
                .set_body_json(prr_endpoint(pool_hash.to_string()).await.unwrap());
        
            Mock::given(method("GET"))
                .and(token_matcher)
                .and(path_regex(r"^/api/info/pool/pool_reward_recipients/pool1[a-z0-9]{51}$"))
                .respond_with(response)
                .mount(&mock_server)
                .await;
        
            mock_server
        }

        // same as src/server/handler/handler_rest/info.rs/pool_reward_recipients
        pub async fn prr_endpoint(
            pool_hash: String,
        ) -> Result<Json<serde_json::Value>, Rejection> {
            let pool_reward_recipients = data_provider()?
                .pool_reward_recipients(&pool_hash)
                .await
                .map_err(|_| RESTError::Custom("Couldn't find the quantity of delegators that received rewards".to_string()))?;
        
            Ok(rweb::Json::from(json!(pool_reward_recipients)))
        }

        pub async fn prr_data(
            mock_server: MockServer, 
            bearer_token: &str,
            pool_hash: &str,
        ) -> (MockServer, u16, Json<serde_json::Value>) {
            let mut response = surf::get(format!("{}/api/info/pool/pool_reward_recipients/{}", &mock_server.uri(), pool_hash))
                .header("authorization", "Bearer ".to_string() + bearer_token)
                .await
                .unwrap();
            let status = response.status() as u16;
            let body: Json<serde_json::Value> = response.body_json().await.unwrap();
            (mock_server, status, body)
        }

        pub async fn prr_test(
            mock_server: MockServer, 
            bearer_token: &str,
            pool_hash: &str,
        ) -> MockServer {
            let (mock_server, status, body) = prr_data(mock_server, bearer_token, pool_hash).await;
            assert_eq!(status, 200);
            assert_eq!(body, Json::from(json!(8)));
            mock_server
        }
    }

    pub mod pool_last_reward_earned_epoch {
        use super::super::data_provider;
        use super::super::mock_error::RESTError;
        use super::super::CardanoDataProvider;
        use rweb::{Json, Rejection};
        use serde_json::json;
        use wiremock::{MockServer, Mock, ResponseTemplate, matchers::{method, BearerTokenMatcher, path_regex}};
        
        pub async fn plree_mock(
            token_matcher: BearerTokenMatcher, 
            mock_server: MockServer, 
            pool_hash: &str,
        ) -> MockServer {
            let response = ResponseTemplate::new(200)
                .set_body_json(plree_endpoint(pool_hash.to_string()).await.unwrap());
        
            Mock::given(method("GET"))
                .and(token_matcher)
                .and(path_regex(r"^/api/info/pool/last_reward_earned_epoch/pool1[a-z0-9]{51}$"))
                .respond_with(response)
                .mount(&mock_server)
                .await;
        
            mock_server
        }

        // same as src/server/handler/handler_rest/info.rs/pool_last_reward_earned_epoch
        pub async fn plree_endpoint(
            pool_hash: String,
        ) -> Result<Json<serde_json::Value>, Rejection> {
            let pool_last_reward_earned_epoch = data_provider()?
                .pool_last_reward_earned_epoch(&pool_hash)
                .await
                .map_err(|_| RESTError::Custom("Couldn't find the last epoch when the given pool distributed rewards".to_string()))?;
        
            Ok(rweb::Json::from(json!(pool_last_reward_earned_epoch)))
        }

        pub async fn plree_data(
            mock_server: MockServer, 
            bearer_token: &str,
            pool_hash: &str,
        ) -> (MockServer, u16, Json<serde_json::Value>) {
            let mut response = surf::get(format!("{}/api/info/pool/last_reward_earned_epoch/{}", &mock_server.uri(), pool_hash))
                .header("authorization", "Bearer ".to_string() + bearer_token)
                .await
                .unwrap();
            let status = response.status() as u16;
            let body: Json<serde_json::Value> = response.body_json().await.unwrap();
            (mock_server, status, body)
        }

        pub async fn plree_test(
            mock_server: MockServer, 
            bearer_token: &str,
            pool_hash: &str,
        ) -> MockServer {
            let (mock_server, status, body) = plree_data(mock_server, bearer_token, pool_hash).await;
            assert_eq!(status, 200);
            assert_eq!(body, Json::from(json!(328)));
            mock_server
        }
    }

    pub mod pool_declared_pledge {
        use super::super::data_provider;
        use super::super::mock_error::RESTError;
        use super::super::CardanoDataProvider;
        use rweb::{Json, Rejection};
        use serde_json::json;
        use wiremock::{MockServer, Mock, ResponseTemplate, matchers::{method, BearerTokenMatcher, path_regex}};
        
        pub async fn pdp_mock(
            token_matcher: BearerTokenMatcher, 
            mock_server: MockServer, 
            pool_hash: &str,
        ) -> MockServer {
            let response = ResponseTemplate::new(200)
                .set_body_json(pdp_endpoint(pool_hash.to_string()).await.unwrap());
        
            Mock::given(method("GET"))
                .and(token_matcher)
                .and(path_regex(r"^/api/info/pool/declared_pledge/pool1[a-z0-9]{51}$"))
                .respond_with(response)
                .mount(&mock_server)
                .await;
        
            mock_server
        }

        // same as src/server/handler/handler_rest/info.rs/pool_declared_pledge
        pub async fn pdp_endpoint(
            pool_hash: String,
        ) -> Result<Json<serde_json::Value>, Rejection> {
            let pool_declared_pledge = data_provider()?
                .pool_declared_pledge(&pool_hash)
                .await
                .map_err(|_| RESTError::Custom("Couldn't find the amount of Ada pledged by the given pool".to_string()))?;
        
            Ok(rweb::Json::from(json!(pool_declared_pledge)))
        }

        pub async fn pdp_data(
            mock_server: MockServer, 
            bearer_token: &str,
            pool_hash: &str,
        ) -> (MockServer, u16, Json<serde_json::Value>) {
            let mut response = surf::get(format!("{}/api/info/pool/declared_pledge/{}", &mock_server.uri(), pool_hash))
                .header("authorization", "Bearer ".to_string() + bearer_token)
                .await
                .unwrap();
            let status = response.status() as u16;
            let body: Json<serde_json::Value> = response.body_json().await.unwrap();
            (mock_server, status, body)
        }

        pub async fn pdp_test(
            mock_server: MockServer, 
            bearer_token: &str,
            pool_hash: &str,
        ) -> MockServer {
            let (mock_server, status, body) = pdp_data(mock_server, bearer_token, pool_hash).await;
            assert_eq!(status, 200);
            assert_eq!(body, Json::from(json!("125000000000")));
            mock_server
        }
    }

    pub mod pool_margin_cost {
        use super::super::data_provider;
        use super::super::mock_error::RESTError;
        use super::super::CardanoDataProvider;
        use rweb::{Json, Rejection};
        use serde_json::json;
        use wiremock::{MockServer, Mock, ResponseTemplate, matchers::{method, BearerTokenMatcher, path_regex}};
        
        pub async fn pmc_mock(
            token_matcher: BearerTokenMatcher, 
            mock_server: MockServer, 
            pool_hash: &str,
        ) -> MockServer {
            let response = ResponseTemplate::new(200)
                .set_body_json(pmc_endpoint(pool_hash.to_string()).await.unwrap());
        
            Mock::given(method("GET"))
                .and(token_matcher)
                .and(path_regex(r"^/api/info/pool/margin_cost/pool1[a-z0-9]{51}$"))
                .respond_with(response)
                .mount(&mock_server)
                .await;
        
            mock_server
        }

        // same as src/server/handler/handler_rest/info.rs/pool_margin_cost
        pub async fn pmc_endpoint(
            pool_hash: String,
        ) -> Result<Json<serde_json::Value>, Rejection> {
            let pool_margin_cost = data_provider()?
                .pool_margin_cost(&pool_hash)
                .await
                .map_err(|_| RESTError::Custom("Couldn't find the margin cost of the given pool".to_string()))?;
        
            Ok(rweb::Json::from(json!(pool_margin_cost)))
        }

        pub async fn pmc_data(
            mock_server: MockServer, 
            bearer_token: &str,
            pool_hash: &str,
        ) -> (MockServer, u16, Json<serde_json::Value>) {
            let mut response = surf::get(format!("{}/api/info/pool/margin_cost/{}", &mock_server.uri(), pool_hash))
                .header("authorization", "Bearer ".to_string() + bearer_token)
                .await
                .unwrap();
            let status = response.status() as u16;
            let body: Json<serde_json::Value> = response.body_json().await.unwrap();
            (mock_server, status, body)
        }

        pub async fn pmc_test(
            mock_server: MockServer, 
            bearer_token: &str,
            pool_hash: &str,
        ) -> MockServer {
            let (mock_server, status, body) = pmc_data(mock_server, bearer_token, pool_hash).await;
            assert_eq!(status, 200);
            assert_eq!(body, Json::from(json!(0.075)));
            mock_server
        }
    }

    pub mod pool_fixed_cost {
        use super::super::data_provider;
        use super::super::mock_error::RESTError;
        use super::super::CardanoDataProvider;
        use rweb::{Json, Rejection};
        use serde_json::json;
        use wiremock::{MockServer, Mock, ResponseTemplate, matchers::{method, BearerTokenMatcher, path_regex}};
        
        pub async fn pfc_mock(
            token_matcher: BearerTokenMatcher, 
            mock_server: MockServer, 
            pool_hash: &str,
        ) -> MockServer {
            let response = ResponseTemplate::new(200)
                .set_body_json(pfc_endpoint(pool_hash.to_string()).await.unwrap());
        
            Mock::given(method("GET"))
                .and(token_matcher)
                .and(path_regex(r"^/api/info/pool/fixed_cost/pool1[a-z0-9]{51}$"))
                .respond_with(response)
                .mount(&mock_server)
                .await;
        
            mock_server
        }

        // same as src/server/handler/handler_rest/info.rs/pool_fixed_cost
        pub async fn pfc_endpoint(
            pool_hash: String,
        ) -> Result<Json<serde_json::Value>, Rejection> {
            let pool_fixed_cost = data_provider()?
                .pool_fixed_cost(&pool_hash)
                .await
                .map_err(|_| RESTError::Custom("Couldn't find the fixed cost of the given pool".to_string()))?;
        
            Ok(rweb::Json::from(json!(pool_fixed_cost)))
        }

        pub async fn pfc_data(
            mock_server: MockServer, 
            bearer_token: &str,
            pool_hash: &str,
        ) -> (MockServer, u16, Json<serde_json::Value>) {
            let mut response = surf::get(format!("{}/api/info/pool/fixed_cost/{}", &mock_server.uri(), pool_hash))
                .header("authorization", "Bearer ".to_string() + bearer_token)
                .await
                .unwrap();
            let status = response.status() as u16;
            let body: Json<serde_json::Value> = response.body_json().await.unwrap();
            (mock_server, status, body)
        }

        pub async fn pfc_test(
            mock_server: MockServer, 
            bearer_token: &str,
            pool_hash: &str,
        ) -> MockServer {
            let (mock_server, status, body) = pfc_data(mock_server, bearer_token, pool_hash).await;
            assert_eq!(status, 200);
            assert_eq!(body, Json::from(json!("340000000")));
            mock_server
        }
    }

    pub mod pool_owner {
        use super::super::data_provider;
        use super::super::mock_error::RESTError;
        use super::super::CardanoDataProvider;
        use rweb::{Json, Rejection};
        use serde_json::json;
        use wiremock::{MockServer, Mock, ResponseTemplate, matchers::{method, BearerTokenMatcher, path_regex}};
        
        pub async fn po_mock(
            token_matcher: BearerTokenMatcher, 
            mock_server: MockServer, 
            pool_hash: &str,
        ) -> MockServer {
            let response = ResponseTemplate::new(200)
                .set_body_json(po_endpoint(pool_hash.to_string()).await.unwrap());
        
            Mock::given(method("GET"))
                .and(token_matcher)
                .and(path_regex(r"^/api/info/pool/owner/pool1[a-z0-9]{51}$"))
                .respond_with(response)
                .mount(&mock_server)
                .await;
        
            mock_server
        }

        // same as src/server/handler/handler_rest/info.rs/pool_owner
        pub async fn po_endpoint(
            pool_hash: String,
        ) -> Result<Json<serde_json::Value>, Rejection> {
            let pool_owner = data_provider()?
                .pool_owner(&pool_hash)
                .await
                .map_err(|_| RESTError::Custom("Couldn't find the owner of the given pool".to_string()))?;
        
            Ok(rweb::Json::from(json!(pool_owner)))
        }

        pub async fn po_data(
            mock_server: MockServer, 
            bearer_token: &str,
            pool_hash: &str,
        ) -> (MockServer, u16, Json<serde_json::Value>) {
            let mut response = surf::get(format!("{}/api/info/pool/owner/{}", &mock_server.uri(), pool_hash))
                .header("authorization", "Bearer ".to_string() + bearer_token)
                .await
                .unwrap();
            let status = response.status() as u16;
            let body: Json<serde_json::Value> = response.body_json().await.unwrap();
            (mock_server, status, body)
        }

        pub async fn po_test(
            mock_server: MockServer, 
            bearer_token: &str,
            pool_hash: &str,
        ) -> MockServer {
            let (mock_server, status, body) = po_data(mock_server, bearer_token, pool_hash).await;
            assert_eq!(status, 200);
            assert_eq!(body, Json::from(json!("stake_test1uz5ah77y8xvnxs6cyp979hg7fhxezjw39jfrpardqymnz7sg7ea8y")));
            mock_server
        }
    }
        
    pub mod pool_registration {
        use super::super::data_provider;
        use super::super::mock_error::RESTError;
        use super::super::CardanoDataProvider;
        use rweb::{Json, Rejection};
        use serde_json::json;
        use wiremock::{MockServer, Mock, ResponseTemplate, matchers::{method, BearerTokenMatcher, path_regex}};
        
        pub async fn pr_mock(
            token_matcher: BearerTokenMatcher, 
            mock_server: MockServer, 
            pool_hash: &str,
        ) -> MockServer {
            let response = ResponseTemplate::new(200)
                .set_body_json(pr_endpoint(pool_hash.to_string()).await.unwrap());
        
            Mock::given(method("GET"))
                .and(token_matcher)
                .and(path_regex(r"^/api/info/pool/registration/pool1[a-z0-9]{51}$"))
                .respond_with(response)
                .mount(&mock_server)
                .await;
        
            mock_server
        }

        // same as src/server/handler/handler_rest/info.rs/pool_registration
        pub async fn pr_endpoint(
            pool_hash: String,
        ) -> Result<Json<serde_json::Value>, Rejection> {
            let pool_registration = data_provider()?
                .pool_registration(&pool_hash)
                .await
                .map_err(|_| RESTError::Custom("Couldn't find the epoch in which the given pool made its latest registration".to_string()))?;
        
            Ok(rweb::Json::from(json!(pool_registration)))
        }

        pub async fn pr_data(
            mock_server: MockServer, 
            bearer_token: &str,
            pool_hash: &str,
        ) -> (MockServer, u16, Json<serde_json::Value>) {
            let mut response = surf::get(format!("{}/api/info/pool/registration/{}", &mock_server.uri(), pool_hash))
                .header("authorization", "Bearer ".to_string() + bearer_token)
                .await
                .unwrap();
            let status = response.status() as u16;
            let body: Json<serde_json::Value> = response.body_json().await.unwrap();
            (mock_server, status, body)
        }

        pub async fn pr_test(
            mock_server: MockServer, 
            bearer_token: &str,
            pool_hash: &str,
        ) -> MockServer {
            let (mock_server, status, body) = pr_data(mock_server, bearer_token, pool_hash).await;
            assert_eq!(status, 200);
            assert_eq!(body, Json::from(json!(175)));
            mock_server
        }
    }

    pub mod pool_retirement {
        use super::super::data_provider;
        use super::super::mock_error::RESTError;
        use super::super::CardanoDataProvider;
        use rweb::{Json, Rejection};
        use serde_json::json;
        use wiremock::{MockServer, Mock, ResponseTemplate, matchers::{method, BearerTokenMatcher, path_regex}};
        
        pub async fn pr_mock(
            token_matcher: BearerTokenMatcher, 
            mock_server: MockServer, 
            pool_hash: &str,
        ) -> MockServer {
            let response = ResponseTemplate::new(200)
                .set_body_json(pr_endpoint(pool_hash.to_string()).await.unwrap());
        
            Mock::given(method("GET"))
                .and(token_matcher)
                .and(path_regex(r"^/api/info/pool/retirement/pool1[a-z0-9]{51}$"))
                .respond_with(response)
                .mount(&mock_server)
                .await;
        
            mock_server
        }

        // same as src/server/handler/handler_rest/info.rs/pool_retirement
        pub async fn pr_endpoint(
            pool_hash: String,
        ) -> Result<Json<serde_json::Value>, Rejection> {
            let pool_retirement = data_provider()?
                .pool_retirement(&pool_hash)
                .await
                .map_err(|_| RESTError::Custom("Couldn't find the epoch in which the given pool retired".to_string()))?;
        
            Ok(rweb::Json::from(json!(pool_retirement)))
        }

        pub async fn pr_data(
            mock_server: MockServer, 
            bearer_token: &str,
            pool_hash: &str,
        ) -> (MockServer, u16, Json<serde_json::Value>) {
            let mut response = surf::get(format!("{}/api/info/pool/retirement/{}", &mock_server.uri(), pool_hash))
                .header("authorization", "Bearer ".to_string() + bearer_token)
                .await
                .unwrap();
            let status = response.status() as u16;
            let body: Json<serde_json::Value> = response.body_json().await.unwrap();
            (mock_server, status, body)
        }

        pub async fn pr_test(
            mock_server: MockServer, 
            bearer_token: &str,
            pool_hash: &str,
        ) -> MockServer {
            let (mock_server, status, body) = pr_data(mock_server, bearer_token, pool_hash).await;
            assert_eq!(status, 200);
            assert_eq!(body, Json::from(json!(14)));
            mock_server
        }
    }

    pub mod pool_url {
        use super::super::data_provider;
        use super::super::mock_error::RESTError;
        use super::super::CardanoDataProvider;
        use rweb::{Json, Rejection};
        use serde_json::json;
        use wiremock::{MockServer, Mock, ResponseTemplate, matchers::{method, BearerTokenMatcher, path_regex}};
        
        pub async fn pu_mock(
            token_matcher: BearerTokenMatcher, 
            mock_server: MockServer, 
            pool_hash: &str,
        ) -> MockServer {
            let response = ResponseTemplate::new(200)
                .set_body_json(pu_endpoint(pool_hash.to_string()).await.unwrap());
        
            Mock::given(method("GET"))
                .and(token_matcher)
                .and(path_regex(r"^/api/info/pool/url/pool1[a-z0-9]{51}$"))
                .respond_with(response)
                .mount(&mock_server)
                .await;
        
            mock_server
        }

        // same as src/server/handler/handler_rest/info.rs/pool_url
        pub async fn pu_endpoint(
            pool_hash: String,
        ) -> Result<Json<serde_json::Value>, Rejection> {
            let pool_url = data_provider()?
                .pool_url(&pool_hash)
                .await
                .map_err(|_| RESTError::Custom("Couldn't find the url in which the given pool stores its metadata".to_string()))?;
        
            Ok(rweb::Json::from(json!(pool_url)))
        }

        pub async fn pu_data(
            mock_server: MockServer, 
            bearer_token: &str,
            pool_hash: &str,
        ) -> (MockServer, u16, Json<serde_json::Value>) {
            let mut response = surf::get(format!("{}/api/info/pool/url/{}", &mock_server.uri(), pool_hash))
                .header("authorization", "Bearer ".to_string() + bearer_token)
                .await
                .unwrap();
            let status = response.status() as u16;
            let body: Json<serde_json::Value> = response.body_json().await.unwrap();
            (mock_server, status, body)
        }

        pub async fn pu_test(
            mock_server: MockServer, 
            bearer_token: &str,
            pool_hash: &str,
        ) -> MockServer {
            let (mock_server, status, body) = pu_data(mock_server, bearer_token, pool_hash).await;
            assert_eq!(status, 200);
            assert_eq!(body, Json::from(json!("https://sharepool-crypto.github.io/share/poolMetaData.json")));
            mock_server
        }
    }

    pub mod pool_ticker {
        use super::super::data_provider;
        use super::super::mock_error::RESTError;
        use super::super::CardanoDataProvider;
        use rweb::{Json, Rejection};
        use serde_json::json;
        use wiremock::{MockServer, Mock, ResponseTemplate, matchers::{method, BearerTokenMatcher, path_regex}};
        
        pub async fn pt_mock(
            token_matcher: BearerTokenMatcher, 
            mock_server: MockServer, 
            pool_hash: &str,
        ) -> MockServer {
            let response = ResponseTemplate::new(200)
                .set_body_json(pt_endpoint(pool_hash.to_string()).await.unwrap());
        
            Mock::given(method("GET"))
                .and(token_matcher)
                .and(path_regex(r"^/api/info/pool/ticker/pool1[a-z0-9]{51}$"))
                .respond_with(response)
                .mount(&mock_server)
                .await;
        
            mock_server
        }

        // same as src/server/handler/handler_rest/info.rs/pool_ticker
        pub async fn pt_endpoint(
            pool_hash: String,
        ) -> Result<Json<serde_json::Value>, Rejection> {
            let pool_ticker = data_provider()?
                .pool_ticker(&pool_hash)
                .await
                .map_err(|_| RESTError::Custom("Couldn't find the ticker of the given stake pool".to_string()))?;
        
            Ok(rweb::Json::from(json!(pool_ticker)))
        }

        pub async fn pt_data(
            mock_server: MockServer, 
            bearer_token: &str,
            pool_hash: &str,
        ) -> (MockServer, u16, Json<serde_json::Value>) {
            let mut response = surf::get(format!("{}/api/info/pool/ticker/{}", &mock_server.uri(), pool_hash))
                .header("authorization", "Bearer ".to_string() + bearer_token)
                .await
                .unwrap();
            let status = response.status() as u16;
            let body: Json<serde_json::Value> = response.body_json().await.unwrap();
            (mock_server, status, body)
        }

        pub async fn pt_test(
            mock_server: MockServer, 
            bearer_token: &str,
            pool_hash: &str,
        ) -> MockServer {
            let (mock_server, status, body) = pt_data(mock_server, bearer_token, pool_hash).await;
            assert_eq!(status, 200);
            assert_eq!(body, Json::from(json!("EUSKL")));
            mock_server
        }
    }

    pub mod pool_metadata_json {
        use super::super::data_provider;
        use super::super::mock_error::RESTError;
        use super::super::CardanoDataProvider;
        use rweb::{Json, Rejection};
        use serde_json::json;
        use wiremock::{MockServer, Mock, ResponseTemplate, matchers::{method, BearerTokenMatcher, path_regex}};

        pub async fn pmj_mock(
            token_matcher: BearerTokenMatcher, 
            mock_server: MockServer, 
            pool_hash: &str,
        ) -> MockServer {
            let response = ResponseTemplate::new(200)
                .set_body_json(pmj_endpoint(pool_hash.to_string()).await.unwrap());
        
            Mock::given(method("GET"))
                .and(token_matcher)
                .and(path_regex(r"^/api/info/pool/metadata_json/pool1[a-z0-9]{51}$"))
                .respond_with(response)
                .mount(&mock_server)
                .await;
        
            mock_server
        }

        // same as src/server/handler/handler_rest/info.rs/pool_metadata_json
        pub async fn pmj_endpoint(
            pool_hash: String,
        ) -> Result<Json<serde_json::Value>, Rejection> {
            let pool_metadata_json = data_provider()?
                .pool_metadata_json(&pool_hash)
                .await
                .map_err(|_| RESTError::Custom("Couldn't find the metadata JSON of the given stake pool".to_string()))?;
            
            Ok(rweb::Json::from(pool_metadata_json))
        }

        pub async fn pmj_data(
            mock_server: MockServer, 
            bearer_token: &str,
            pool_hash: &str,
        ) -> (MockServer, u16, Json<serde_json::Value>) {
            let mut response = surf::get(format!("{}/api/info/pool/metadata_json/{}", &mock_server.uri(), pool_hash))
                .header("authorization", "Bearer ".to_string() + bearer_token)
                .await
                .unwrap();
            let status = response.status() as u16;
            let body: Json<serde_json::Value> = response.body_json().await.unwrap();
            (mock_server, status, body)
        }

        pub async fn pmj_test(
            mock_server: MockServer, 
            bearer_token: &str,
            pool_hash: &str,
        ) -> MockServer {
            let (mock_server, status, body) = pmj_data(mock_server, bearer_token, pool_hash).await;
            assert_eq!(status, 200);
            assert_eq!(body, Json::from(json!({"name": "EUSKAL STAKE POOL TESTNET", "ticker": "EUSKL", "homepage": "https://euskalstakepool.win", "description": "EUSKAL STAKE POOL TESTNET"})));
            mock_server
        }
    }

    pub mod pool_name {
        use super::super::data_provider;
        use super::super::mock_error::RESTError;
        use super::super::CardanoDataProvider;
        use rweb::{Json, Rejection};
        use serde_json::json;
        use wiremock::{MockServer, Mock, ResponseTemplate, matchers::{method, BearerTokenMatcher, path_regex}};

        pub async fn pn_mock(
            token_matcher: BearerTokenMatcher, 
            mock_server: MockServer, 
            pool_hash: &str,
        ) -> MockServer {
            let response = ResponseTemplate::new(200)
                .set_body_json(pn_endpoint(pool_hash.to_string()).await.unwrap());
        
            Mock::given(method("GET"))
                .and(token_matcher)
                .and(path_regex(r"^/api/info/pool/name/pool1[a-z0-9]{51}$"))
                .respond_with(response)
                .mount(&mock_server)
                .await;
        
            mock_server
        }

        // same as src/server/handler/handler_rest/info.rs/pool_name
        pub async fn pn_endpoint(
            pool_hash: String,
        ) -> Result<Json<serde_json::Value>, Rejection> {
            let pool_name = data_provider()?
                .pool_name(&pool_hash)
                .await
                .map_err(|_| RESTError::Custom("Couldn't find the name of the given stake pool".to_string()))?;
        
            Ok(rweb::Json::from(json!(pool_name)))
        }

        pub async fn pn_data(
            mock_server: MockServer, 
            bearer_token: &str,
            pool_hash: &str,
        ) -> (MockServer, u16, Json<serde_json::Value>) {
            let mut response = surf::get(format!("{}/api/info/pool/name/{}", &mock_server.uri(), pool_hash))
                .header("authorization", "Bearer ".to_string() + bearer_token)
                .await
                .unwrap();
            let status = response.status() as u16;
            let body: Json<serde_json::Value> = response.body_json().await.unwrap();
            (mock_server, status, body)
        }

        pub async fn pn_test(
            mock_server: MockServer, 
            bearer_token: &str,
            pool_hash: &str,
        ) -> MockServer {
            let (mock_server, status, body) = pn_data(mock_server, bearer_token, pool_hash).await;
            assert_eq!(status, 200);
            assert_eq!(body, Json::from(json!("EUSKAL STAKE POOL TESTNET")));
            mock_server
        }
    }

    pub mod pool_homepage {
        use super::super::data_provider;
        use super::super::mock_error::RESTError;
        use super::super::CardanoDataProvider;
        use rweb::{Json, Rejection};
        use serde_json::json;
        use wiremock::{MockServer, Mock, ResponseTemplate, matchers::{method, BearerTokenMatcher, path_regex}};

        pub async fn ph_mock(
            token_matcher: BearerTokenMatcher, 
            mock_server: MockServer, 
            pool_hash: &str,
        ) -> MockServer {
            let response = ResponseTemplate::new(200)
                .set_body_json(ph_endpoint(pool_hash.to_string()).await.unwrap());
        
            Mock::given(method("GET"))
                .and(token_matcher)
                .and(path_regex(r"^/api/info/pool/homepage/pool1[a-z0-9]{51}$"))
                .respond_with(response)
                .mount(&mock_server)
                .await;
        
            mock_server
        }

        // same as src/server/handler/handler_rest/info.rs/pool_homepage
        pub async fn ph_endpoint(
            pool_hash: String,
        ) -> Result<Json<serde_json::Value>, Rejection> {
            let pool_homepage = data_provider()?
                .pool_homepage(&pool_hash)
                .await
                .map_err(|_| RESTError::Custom("Couldn't find the homepage of the given stake pool".to_string()))?;
        
            Ok(rweb::Json::from(json!(pool_homepage)))
        }

        pub async fn ph_data(
            mock_server: MockServer, 
            bearer_token: &str,
            pool_hash: &str,
        ) -> (MockServer, u16, Json<serde_json::Value>) {
            let mut response = surf::get(format!("{}/api/info/pool/homepage/{}", &mock_server.uri(), pool_hash))
                .header("authorization", "Bearer ".to_string() + bearer_token)
                .await
                .unwrap();
            let status = response.status() as u16;
            let body: Json<serde_json::Value> = response.body_json().await.unwrap();
            (mock_server, status, body)
        }

        pub async fn ph_test(
            mock_server: MockServer, 
            bearer_token: &str,
            pool_hash: &str,
        ) -> MockServer {
            let (mock_server, status, body) = ph_data(mock_server, bearer_token, pool_hash).await;
            assert_eq!(status, 200);
            assert_eq!(body, Json::from(json!("https://euskalstakepool.win")));
            mock_server
        }
    }

    pub mod pool_description {
        use super::super::data_provider;
        use super::super::mock_error::RESTError;
        use super::super::CardanoDataProvider;
        use rweb::{Json, Rejection};
        use serde_json::json;
        use wiremock::{MockServer, Mock, ResponseTemplate, matchers::{method, BearerTokenMatcher, path_regex}};

        pub async fn pd_mock(
            token_matcher: BearerTokenMatcher, 
            mock_server: MockServer, 
            pool_hash: &str
        ) -> MockServer {
            let response = ResponseTemplate::new(200)
                .set_body_json(pd_endpoint(pool_hash.to_string()).await.unwrap());
        
            Mock::given(method("GET"))
                .and(token_matcher)
                .and(path_regex(r"^/api/info/pool/description/pool1[a-z0-9]{51}$"))
                .respond_with(response)
                .mount(&mock_server)
                .await;
        
            mock_server
        }
        
        // same as src/server/handler/handler_rest/info.rs/pool_description
        pub async fn pd_endpoint(
            pool_hash: String,
        ) -> Result<Json<serde_json::Value>, Rejection> {
            let pool_description = data_provider()?
                .pool_description(&pool_hash).await
                .map_err(|_| RESTError::Custom("Couldn't find the description of the given stake pool".to_string()))?;
        
            Ok(rweb::Json::from(json!(pool_description)))
        }
        
        pub async fn pd_data(
            mock_server: MockServer, 
            bearer_token: &str,
            pool_hash: &str, 
        ) -> (MockServer, u16, Json<serde_json::Value>) {
            let mut response = surf::get(format!("{}/api/info/pool/description/{}", &mock_server.uri(), pool_hash))
                .header("authorization", "Bearer ".to_string() + bearer_token)
                .await
                .unwrap();
            let status = response.status() as u16;
            let body: Json<serde_json::Value> = response.body_json().await.unwrap();
            (mock_server, status, body)
        }
        
        pub async fn pd_test(
            mock_server: MockServer, 
            bearer_token: &str,
            pool_hash: &str, 
        ) -> MockServer {
            let (mock_server, status, body) = pd_data(mock_server, bearer_token, pool_hash).await;
            assert_eq!(status, 200);
            assert_eq!(body, Json::from(json!("EUSKAL STAKE POOL TESTNET")));
            mock_server
        }
    }
}

pub async fn mock_config() -> (String, MockServer) {
    dotenv().ok();
    let bearer_token = dotenv::var("CDP_BEARER_TOKEN").unwrap();
    let config = cdp::config::ConfigRoot::new(&Some(PathBuf::from("./config_testnet.toml"))).unwrap();
    config.set_as_env();
    let mock_server = MockServer::start().await;

    (bearer_token, mock_server)
}

pub fn data_provider() -> Result<DataProvider<DBSyncProvider>, Rejection> {
    let dp = DataProvider::new(DBSyncProvider::new(crate::Config {
        db_path: std::env::var("DBSYNC_URL").map_err(|_| warp::reject::not_found())?,
    }));
    Ok(dp) 
}

pub fn make_error(
    e: String,
    c: Option<i64>,
    d: Option<&str>,
) -> Result<Json<serde_json::Value>, Rejection> {
    Ok(rweb::Json::from(
        serde_json::json!({ "error": e, "code": c, "description": d }),
    ))
}

fn parse_string_vec_from_query(query: &str) -> Result<Vec<String>, RESTError> {
    debug!("Q:{:?}", query);
    let list: Vec<&str> = query.split('=').collect();
    debug!("1:{:?}", list);
    let list = list[1].replace("%22", &'"'.to_string());
    let list = list.replace("%5D", &']'.to_string());
    let list = list.replace("%5B", &'['.to_string());
    let list = list.replace("%2C", &','.to_string());
    debug!("2:{:?}", list);
    let list = serde_json::from_str::<Vec<String>>(&list);
    debug!("Vec: {:?}", list);
    let list = list.unwrap();
    Ok(list)
}

pub async fn get_asset_for_addresses(
    addresses: &Vec<String>,
) -> Result<Vec<AssetHandle>, Rejection> {
    debug!("{addresses:?}");

    let mut utxos = TransactionUnspentOutputs::new();

    for a in addresses {
        let us = data_provider()?.get_address_utxos(a).await?;
        utxos.merge(us);
    }

    let mut handles = Vec::<AssetHandle>::new();
    for u in utxos {
        let v = u.output().amount();
        let ada = v.coin();
        handles.push(AssetHandle {
            fingerprint: None,
            policy: None,
            tokenname: None,
            amount: from_bignum(&ada),
            metadata: None,
        });
        if let Some(multis) = v.multiasset() {
            let policies = multis.keys();
            for p in 0..policies.len() {
                let policy = policies.get(p);
                if let Some(assets) = multis.get(&policy) {
                    let k = assets.keys();
                    for a in 0..k.len() {
                        let asset = k.get(a);
                    let amt = assets.get(&asset).unwrap();
                        let fingerprint =
                            make_fingerprint(&policy.to_hex(), &hex::encode(asset.name())).unwrap();
                        // Deactivated Metadata Requests for performance
                        //let metadata = dp.mint_metadata(&fingerprint).await.unwrap();
                        handles.push(AssetHandle {
                            fingerprint: Some(fingerprint),
                            policy: Some(policy.to_hex()),
                            tokenname: Some(match from_utf8(&asset.name()) {
                                Ok(s) => s.to_owned(),
                                Err(_) => hex::encode(&asset.name()),
                            }),
                            amount: from_bignum(&amt),
                            metadata: None, //metadata.json,
                        })
                    }
                }
            }
        }
    }
    debug!("Handles: {:?}", handles);
    let mut handles_summed = Vec::<AssetHandle>::new();

    for h in &handles {
        if !handles_summed.contains(h) {
            let sum = handles.iter().fold(AssetHandle::new_empty(), |mut acc, f| {
                if h == f {
                    acc.amount = acc.amount.checked_add(f.amount).unwrap();

                    if acc.metadata.is_none() && f.metadata.is_some() {
                        acc.metadata = h.metadata.clone()
                    }
                    if acc.fingerprint.is_none() && f.fingerprint.is_some() {
                        acc.fingerprint = h.fingerprint.clone()
                    }
                    if acc.policy.is_none() && f.policy.is_some() {
                        acc.policy = h.policy.clone()
                    }
                    if acc.tokenname.is_none() && f.tokenname.is_some() {
                        acc.tokenname = h.tokenname.clone()
                    }
                }
                acc
            });
            handles_summed.push(sum)
        }
    }
    Ok(handles_summed)
}

mod mock_error {
    use rweb::warp::{http::StatusCode, Rejection, Reply};
    use std::{convert::Infallible, env::VarError};
    use thiserror::Error;
    use serde::Deserialize;

    use super::mock_models::ErrorResponse;

    #[allow(clippy::enum_variant_names)]
    #[derive(Error, Debug)]
    pub enum RESTError {
        #[error("internal error: {:?}", self)]
        Custom(String),

    }

    impl rweb::warp::reject::Reject for RESTError {}

    pub async fn _handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
        let (code, message) = if err.is_not_found() {
            (StatusCode::NOT_FOUND, "Not Found".to_string())
        } else if let Some(e) = err.find::<RESTError>() {
            match e {
                RESTError::Custom(s) => (StatusCode::INTERNAL_SERVER_ERROR, s.to_string()),
                _ => (StatusCode::BAD_REQUEST, e.to_string()),
            }
        } else if err.find::<rweb::warp::reject::MethodNotAllowed>().is_some() {
            (
                StatusCode::METHOD_NOT_ALLOWED,
                "Method Not Allowed".to_string(),
            )
        } else {
            eprintln!("unhandled error: {err:?}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            )
        };

        let json = rweb::warp::reply::json(&ErrorResponse {
            status: code.to_string(),
            message,
        });

        Ok(rweb::warp::reply::with_status(json, code))
    }

    impl From<std::string::String> for RESTError {
        fn from(err: std::string::String) -> Self {
            RESTError::Custom(err)
        }
    }

    impl From<jsonwebtoken::errors::Error> for RESTError {
        fn from(err: jsonwebtoken::errors::Error) -> Self {
            RESTError::Custom(err.to_string())
        }
    }
}

mod mock_models {
    use serde::Serialize;

    #[derive(Serialize, Debug)]
    pub(crate) struct ErrorResponse {
        pub message: String,
        pub status: String,
    }
}