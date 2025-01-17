// This file is @generated by prost-build.
pub mod cosmos {
    pub mod base {
        pub mod query {
            pub mod v1beta1 {
                include!("cosmos.base.query.v1beta1.rs");
            }
        }
        pub mod v1beta1 {
            include!("cosmos.base.v1beta1.rs");
        }
    }
}
pub mod cosmos_proto {
    include!("cosmos_proto.rs");
}
pub mod dydxprotocol {
    pub mod accountplus {
        include!("dydxprotocol.accountplus.rs");
    }
    pub mod affiliates {
        include!("dydxprotocol.affiliates.rs");
    }
    pub mod assets {
        include!("dydxprotocol.assets.rs");
    }
    pub mod blocktime {
        include!("dydxprotocol.blocktime.rs");
    }
    pub mod bridge {
        include!("dydxprotocol.bridge.rs");
    }
    pub mod clob {
        include!("dydxprotocol.clob.rs");
    }
    pub mod daemons {
        pub mod bridge {
            include!("dydxprotocol.daemons.bridge.rs");
        }
        pub mod liquidation {
            include!("dydxprotocol.daemons.liquidation.rs");
        }
        pub mod pricefeed {
            include!("dydxprotocol.daemons.pricefeed.rs");
        }
    }
    pub mod delaymsg {
        include!("dydxprotocol.delaymsg.rs");
    }
    pub mod epochs {
        include!("dydxprotocol.epochs.rs");
    }
    pub mod feetiers {
        include!("dydxprotocol.feetiers.rs");
    }
    pub mod govplus {
        include!("dydxprotocol.govplus.rs");
    }
    pub mod indexer {
        pub mod events {
            include!("dydxprotocol.indexer.events.rs");
        }
        pub mod indexer_manager {
            include!("dydxprotocol.indexer.indexer_manager.rs");
        }
        pub mod off_chain_updates {
            include!("dydxprotocol.indexer.off_chain_updates.rs");
        }
        pub mod protocol {
            pub mod v1 {
                include!("dydxprotocol.indexer.protocol.v1.rs");
            }
        }
        pub mod redis {
            include!("dydxprotocol.indexer.redis.rs");
        }
        pub mod shared {
            include!("dydxprotocol.indexer.shared.rs");
        }
        pub mod socks {
            include!("dydxprotocol.indexer.socks.rs");
        }
    }
    pub mod listing {
        include!("dydxprotocol.listing.rs");
    }
    pub mod perpetuals {
        include!("dydxprotocol.perpetuals.rs");
    }
    pub mod prices {
        include!("dydxprotocol.prices.rs");
    }
    pub mod ratelimit {
        include!("dydxprotocol.ratelimit.rs");
    }
    pub mod revshare {
        include!("dydxprotocol.revshare.rs");
    }
    pub mod rewards {
        include!("dydxprotocol.rewards.rs");
    }
    pub mod sending {
        include!("dydxprotocol.sending.rs");
    }
    pub mod stats {
        include!("dydxprotocol.stats.rs");
    }
    pub mod subaccounts {
        include!("dydxprotocol.subaccounts.rs");
    }
    pub mod vault {
        include!("dydxprotocol.vault.rs");
    }
    pub mod vest {
        include!("dydxprotocol.vest.rs");
    }
}
pub mod google {
    pub mod api {
        include!("google.api.rs");
    }
}
