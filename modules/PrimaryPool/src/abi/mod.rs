pub mod factory;
pub mod pool;

pub mod pi {
    pub mod v1 {
        use substreams::log;

        const INTERNAL_ERR: &'static str = "`ethabi_derive` internal error";

        // #[derive(Debug, Clone, PartialEq)]
        // pub struct Subscription {
        //     pub asset_in: Vec<u8>,
        //     pub asset_out: Vec<u8>,
        //     pub subscription: substreams::scalar::BigInt,
        //     pub investor: Vec<u8>,
        //     pub price: substreams::scalar::BigInt,
        //     pub execution_date: substreams::scalar::BigInt,
        // }
        // impl Subscription {
        //     // const TOPIC_ID: [u8; 32] = [
        //     //     86u8, 15u8, 154u8, 154u8, 168u8, 64u8, 37u8, 123u8, 63u8, 113u8, 249u8, 240u8,
        //     //     204u8, 85u8, 39u8, 1u8, 105u8, 64u8, 141u8, 120u8, 37u8, 149u8, 196u8, 22u8, 193u8,
        //     //     4u8, 46u8, 204u8, 247u8, 66u8, 108u8, 156u8,
        //     // ];
        //     const TOPIC_ID: [u8; 32] = [
        //         252u8, 48u8, 169u8, 151u8, 151u8, 14u8, 76u8, 172u8, 120u8, 35u8, 88u8, 132u8,
        //         190u8, 128u8, 6u8, 71u8, 153u8, 79u8, 149u8, 210u8, 56u8, 138u8, 200u8, 99u8, 68u8,
        //         110u8, 81u8, 169u8, 98u8, 85u8, 214u8, 109u8,
        //     ];
        //     pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
        //         if log.topics.len() != 2usize {
        //             return false;
        //         }
        //         if log.data.len() != 288usize {
        //             return false;
        //         }
        //         return log.topics.get(0).expect("bounds already checked").as_ref()
        //             == Self::TOPIC_ID;
        //     }
        //     pub fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
        //         let mut values = ethabi::decode(
        //             &[
        //                 ethabi::ParamType::Address,
        //                 ethabi::ParamType::Uint(256usize),
        //                 ethabi::ParamType::Address,
        //                 ethabi::ParamType::Uint(256usize),
        //                 ethabi::ParamType::Uint(256usize),
        //             ],
        //             log.data.as_ref(),
        //         )
        //         .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
        //         values.reverse();
        //         Ok(Self {
        //             asset_in: ethabi::decode(
        //                 &[ethabi::ParamType::Address],
        //                 log.topics[1usize].as_ref(),
        //             )
        //             .map_err(|e| {
        //                 format!(
        //                     "unable to decode param 'asset_in' from topic of type 'address': {:?}",
        //                     e
        //                 )
        //             })?
        //             .pop()
        //             .expect(INTERNAL_ERR)
        //             .into_address()
        //             .expect(INTERNAL_ERR)
        //             .as_bytes()
        //             .to_vec(),
        //             asset_out: values
        //                 .pop()
        //                 .expect(INTERNAL_ERR)
        //                 .into_address()
        //                 .expect(INTERNAL_ERR)
        //                 .as_bytes()
        //                 .to_vec(),
        //             subscription: {
        //                 let mut v = [0 as u8; 32];
        //                 values
        //                     .pop()
        //                     .expect(INTERNAL_ERR)
        //                     .into_uint()
        //                     .expect(INTERNAL_ERR)
        //                     .to_big_endian(v.as_mut_slice());
        //                 substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
        //             },
        //             investor: values
        //                 .pop()
        //                 .expect(INTERNAL_ERR)
        //                 .into_address()
        //                 .expect(INTERNAL_ERR)
        //                 .as_bytes()
        //                 .to_vec(),
        //             price: {
        //                 let mut v = [0 as u8; 32];
        //                 values
        //                     .pop()
        //                     .expect(INTERNAL_ERR)
        //                     .into_uint()
        //                     .expect(INTERNAL_ERR)
        //                     .to_big_endian(v.as_mut_slice());
        //                 substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
        //             },
        //             execution_date: {
        //                 let mut v = [0 as u8; 32];
        //                 values
        //                     .pop()
        //                     .expect(INTERNAL_ERR)
        //                     .into_uint()
        //                     .expect(INTERNAL_ERR)
        //                     .to_big_endian(v.as_mut_slice());
        //                 substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
        //             },
        //         })
        //     }
        // }
        // impl substreams_ethereum::Event for Subscription {
        //     const NAME: &'static str = "Subscription";
        //     fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
        //         // log::info!("{:?}", log.topics.get(0));
        //         // log::info!("{:?}", Self::TOPIC_ID);
        //         // log::info!(
        //         //     "{:?}",
        //         //     log.topics.get(0).expect("bounds already checked").as_ref() == Self::TOPIC_ID
        //         // );
        //         // log::info!("{:?}", Self::match_log(log));
        //         // log::info!("{:?}", log.topics.len());
        //         // log::info!("{:?}", log.data.len());
        //         Self::match_log(log)
        //     }
        //     fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
        //         Self::decode(log)
        //     }
        // }

        pub struct Subscription {
            pub asset_in: Vec<u8>,
            pub asset_out: Vec<u8>,
            pub subscription: substreams::scalar::BigInt,
            pub investor: Vec<u8>,
            pub price: substreams::scalar::BigInt,
            pub execution_date: substreams::scalar::BigInt,
        }
        impl Subscription {
            // const TOPIC_ID: [u8; 32] = [
            //     86u8, 15u8, 154u8, 154u8, 168u8, 64u8, 37u8, 123u8, 63u8, 113u8, 249u8, 240u8,
            //     204u8, 85u8, 39u8, 1u8, 105u8, 64u8, 141u8, 120u8, 37u8, 149u8, 196u8, 22u8, 193u8,
            //     4u8, 46u8, 204u8, 247u8, 66u8, 108u8, 156u8,
            // ];
            const TOPIC_ID: [u8; 32] = [
                252u8, 48u8, 169u8, 151u8, 151u8, 14u8, 76u8, 172u8, 120u8, 35u8, 88u8, 132u8,
                190u8, 128u8, 6u8, 71u8, 153u8, 79u8, 149u8, 210u8, 56u8, 138u8, 200u8, 99u8, 68u8,
                110u8, 81u8, 169u8, 98u8, 85u8, 214u8, 109u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() != 288usize { //160usize
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
                let mut values = ethabi::decode(
                    &[
                        ethabi::ParamType::Address,
                        ethabi::ParamType::Uint(256usize),
                        ethabi::ParamType::Address,
                        ethabi::ParamType::Uint(256usize),
                        ethabi::ParamType::Uint(256usize),
                    ],
                    log.data.as_ref(),
                )
                .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    asset_in: ethabi::decode(
                        &[ethabi::ParamType::Address],
                        log.topics[1usize].as_ref(),
                    )
                    .map_err(|e| {
                        format!(
                            "unable to decode param 'asset_in' from topic of type 'address': {:?}",
                            e
                        )
                    })?
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                    asset_out: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    subscription: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    investor: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    price: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    execution_date: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
        }
        impl substreams_ethereum::Event for Subscription {
            const NAME: &'static str = "Subscription";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                log::info!("{:?}", log.topics.get(0));
                log::info!("{:?}", Self::TOPIC_ID);
                log::info!(
                    "{:?}",
                    log.topics.get(0).expect("bounds already checked").as_ref() == Self::TOPIC_ID
                );
                log::info!("{:?}", Self::match_log(log));
                log::info!("{:?}", log.topics.len());
                log::info!("{:?}", log.data.len());
                Self::match_log(log)
            }
            fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
                Self::decode(log)
            }
        }
    }
}
