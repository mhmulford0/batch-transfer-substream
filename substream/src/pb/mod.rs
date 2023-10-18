// @generated
pub mod eth {
    pub mod erc1155 {
        // @@protoc_insertion_point(attribute:eth.erc1155.v1)
        pub mod v1 {
            include!("eth.erc1155.v1.rs");
            // @@protoc_insertion_point(eth.erc1155.v1)
        }
    }
    pub mod erc721 {
        // @@protoc_insertion_point(attribute:eth.erc721.v1)
        pub mod v1 {
            include!("eth.erc721.v1.rs");
            // @@protoc_insertion_point(eth.erc721.v1)
        }
    }
}
pub mod sf {
    // @@protoc_insertion_point(attribute:sf.substreams)
    pub mod substreams {
        include!("sf.substreams.rs");
        // @@protoc_insertion_point(sf.substreams)
        pub mod sink {
            pub mod database {
                // @@protoc_insertion_point(attribute:sf.substreams.sink.database.v1)
                pub mod v1 {
                    include!("sf.substreams.sink.database.v1.rs");
                    // @@protoc_insertion_point(sf.substreams.sink.database.v1)
                }
            }
        }
    }
}
