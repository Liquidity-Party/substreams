const INTERNAL_ERR: &'static str = "`ethabi_derive` internal error";
/// Contract's functions.
#[allow(dead_code, unused_imports, unused_variables)]
pub mod functions {
    use super::INTERNAL_ERR;
}
/// Contract's events.
#[allow(dead_code, unused_imports, unused_variables)]
pub mod events {
    use super::INTERNAL_ERR;
    #[derive(Debug, Clone, PartialEq)]
    pub struct PartyStarted {
        pub pool: Vec<u8>,
        pub name: String,
        pub symbol: String,
        pub tokens: Vec<Vec<u8>>,
    }
    impl PartyStarted {
        const TOPIC_ID: [u8; 32] = [
            244u8,
            94u8,
            48u8,
            211u8,
            125u8,
            197u8,
            239u8,
            152u8,
            225u8,
            119u8,
            8u8,
            48u8,
            139u8,
            136u8,
            98u8,
            19u8,
            184u8,
            244u8,
            133u8,
            234u8,
            82u8,
            212u8,
            149u8,
            6u8,
            126u8,
            237u8,
            141u8,
            73u8,
            119u8,
            66u8,
            63u8,
            34u8,
        ];
        pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            if log.topics.len() != 2usize {
                return false;
            }
            if log.data.len() < 192usize {
                return false;
            }
            return log.topics.get(0).expect("bounds already checked").as_ref()
                == Self::TOPIC_ID;
        }
        pub fn decode(
            log: &substreams_ethereum::pb::eth::v2::Log,
        ) -> Result<Self, String> {
            let mut values = ethabi::decode(
                    &[
                        ethabi::ParamType::String,
                        ethabi::ParamType::String,
                        ethabi::ParamType::Array(Box::new(ethabi::ParamType::Address)),
                    ],
                    log.data.as_ref(),
                )
                .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
            values.reverse();
            Ok(Self {
                pool: ethabi::decode(
                        &[ethabi::ParamType::Address],
                        log.topics[1usize].as_ref(),
                    )
                    .map_err(|e| {
                        format!(
                            "unable to decode param 'pool' from topic of type 'address': {:?}",
                            e
                        )
                    })?
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                name: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_string()
                    .expect(INTERNAL_ERR),
                symbol: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_string()
                    .expect(INTERNAL_ERR),
                tokens: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_array()
                    .expect(INTERNAL_ERR)
                    .into_iter()
                    .map(|inner| {
                        inner.into_address().expect(INTERNAL_ERR).as_bytes().to_vec()
                    })
                    .collect(),
            })
        }
    }
    impl substreams_ethereum::Event for PartyStarted {
        const NAME: &'static str = "PartyStarted";
        fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            Self::match_log(log)
        }
        fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            Self::decode(log)
        }
    }
}