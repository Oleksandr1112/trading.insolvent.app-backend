use std::collections::HashMap;

use ethers::types::Address;

use crate::model::info::response::SpotMetaTokenUniverse;
use crate::rest::HyperliquidRestClient;
use crate::{
    error::Result,
    model::{
        exchange::request::HyperliquidChain,
        info::{
            request::{CandleSnapshotRequest, Request},
            response::{
                AssetContext, CandleSnapshot, FundingHistory, L2Book, OpenOrder, Universe,
                UserFill, UserFunding, UserState,
            },
        },
        API,
    },
    HyperliquidUrls,
};

/// Endpoint to fetch information about the exchange and specific users.
pub struct HyperliquidInfoClient {
    pub client: HyperliquidRestClient,
    pub chain: HyperliquidChain,
}

impl HyperliquidInfoClient {
    pub fn new(chain: HyperliquidChain) -> Self {
        let config = HyperliquidUrls::from_chain(chain);

        Self::new_with_config(chain, &config)
    }
    pub fn new_with_config(chain: HyperliquidChain, config: &HyperliquidUrls) -> Self {
        Self {
            chain,
            client: HyperliquidRestClient::new(config.rest_endpoint.clone()),
        }
    }
    /// Retrieve exchange metadata
    pub async fn metadata(&self) -> Result<Universe> {
        self.client.post(API::Info, &Request::Meta).await
    }
    pub async fn spot_metadata(&self) -> Result<SpotMetaTokenUniverse> {
        self.client.post(API::Info, &Request::SpotMeta).await
    }

    /// Retrieve all mids for all actively traded coins
    pub async fn mids(&self) -> Result<HashMap<String, String>> {
        self.client.post(API::Info, &Request::AllMids).await
    }

    /// Retrieve asset contexts i.e mark price, current funding, open interest, etc
    pub async fn contexts(&self) -> Result<Vec<AssetContext>> {
        self.client
            .post(API::Info, &Request::MetaAndAssetCtxs)
            .await
    }

    /// Retrieve a user's state to see user's open positions and margin summary
    pub async fn user_state(&self, user: Address) -> Result<UserState> {
        self.client
            .post(API::Info, &Request::ClearinghouseState { user })
            .await
    }

    /// Retrieve a user's open orders
    pub async fn open_orders(&self, user: Address) -> Result<Vec<OpenOrder>> {
        self.client
            .post(API::Info, &Request::OpenOrders { user })
            .await
    }

    /// Retrieve a user's Userfills
    pub async fn user_fills(&self, user: Address) -> Result<Vec<UserFill>> {
        self.client
            .post(API::Info, &Request::UserFills { user })
            .await
    }

    /// Retrieve a user's funding history
    pub async fn user_funding(
        &self,
        user: Address,
        start_time: u64,
        end_time: Option<u64>,
    ) -> Result<Vec<UserFunding>> {
        self.client
            .post(
                API::Info,
                &Request::UserFunding {
                    user,
                    start_time,
                    end_time,
                },
            )
            .await
    }

    /// Retrieve historical funding rates for a coin
    pub async fn funding_history(
        &self,
        coin: String,
        start_time: u64,
        end_time: Option<u64>,
    ) -> Result<Vec<FundingHistory>> {
        self.client
            .post(
                API::Info,
                &Request::FundingHistory {
                    coin,
                    start_time,
                    end_time,
                },
            )
            .await
    }

    /// Retrieve the L2 order book for a coin
    pub async fn l2_book(&self, coin: String) -> Result<L2Book> {
        self.client.post(API::Info, &Request::L2Book { coin }).await
    }

    /// Retrieve candle snapshot for a coin
    pub async fn candle_snapshot(
        &self,
        coin: String,
        interval: String,
        start_time: u64,
        end_time: u64,
    ) -> Result<Vec<CandleSnapshot>> {
        self.client
            .post(
                API::Info,
                &Request::CandleSnapshot {
                    req: CandleSnapshotRequest {
                        coin,
                        interval,
                        start_time,
                        end_time,
                    },
                },
            )
            .await
    }

    /// Query the status of an order
    pub async fn order_status(&self, user: Address, oid: u64) -> Result<()> {
        // TODO: This should return an OrderStatus
        self.client
            .post(API::Info, &Request::OrderStatus { user, oid })
            .await
    }
}
