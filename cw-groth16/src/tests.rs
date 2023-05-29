#[cfg(test)]
mod test_module {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coin, coins, from_binary, Coin, Deps, DepsMut};

    use crate::contract::{execute, instantiate, query};
    use crate::error::ContractError;
    use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, ZkeysResponse, ProofResponse};
    use crate::state::Config;

    fn assert_config_state(deps: Deps, expected: Config) {
        let res = query(deps, QueryMsg::Config {}).unwrap();
        let value: Config = from_binary(&res).unwrap();
        assert_eq!(value, expected);
    }

    fn mock_init_no_price(deps: DepsMut) {
        let msg = InstantiateMsg {
            set_zkeys_price: None,
            publish_proof_price: None,
        };

        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps, mock_env(), info, msg)
            .expect("contract successfully handles InstantiateMsg");
    }

    fn mock_init_with_price(deps: DepsMut, zkeys_price: Coin, proof_price: Coin) {
        let msg = InstantiateMsg {
            set_zkeys_price: Some(zkeys_price),
            publish_proof_price: Some(proof_price),
        };

        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps, mock_env(), info, msg)
            .expect("contract successfully handles InstantiateMsg");
    }


    #[test]
    fn proper_init_no_fees() {
        let mut deps = mock_dependencies();

        mock_init_no_price(deps.as_mut());
        assert_config_state(
            deps.as_ref(),
            Config {
                zkeys_price: None,
                proof_price: None,
            },
        );
    }


    #[test]
    fn proper_init_with_fees() {
        let mut deps = mock_dependencies();

        mock_init_with_price(deps.as_mut(), coin(3, "token"), coin(4, "token"));

        assert_config_state(
            deps.as_ref(),
            Config {
                zkeys_price: Some(coin(3, "token")),
                proof_price: Some(coin(4, "token")),
            },
        );
    }


    fn mock_alice_set_zkeys(deps: DepsMut, sent: &[Coin]) {
        // alice can register an available name
        let info = mock_info("alice_key", sent);
        let msg = ExecuteMsg::Zkeys { 
            public_signal: "33".to_string(), 
            vk_alpha1: "121ec6cddca2aa0dedf8dbb86e96dc4100b58ea07c01d7ea68a37f8f72191ab2bbe9f16bfe675f71c899ff11e23cbb04064831acc8c18f561f446eeaac3a9a056cb9a89b0b3f13a57eab4e97ebaff6f0a39327bd0a4b5f725d633c87474d35f2".to_string(), 
            vk_beta_2: "00f3edfbbbe5e2dab32cac1d1ba2f0fdd9eff4067c7152520f0ebbf556c21f98e72590b3cdb614b1ea116991305da942077b7419fac8cc2d38dc6639d68a4cf7c8362efd8395020836f3aa564537fa02a17f2d1b423c19b6cf4784037b1d9f1510afbae9e95703ff3a98c46720f05e642588ef21ccb09580c84d211c0fd60acda18a699f61cef4925b9b113c8a2377f0147c5ee0882a97519627776222438d3e29f581f0e4b61fe18ab42089dfe24a1b9d7376667382941e37329860ec84d105".to_string(), 
            vk_gamma_2: "13e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb80606c4a02ea734cc32acd2b02bc28b99cb3e287e85a763af267492ab572e99ab3f370d275cec1da1aaa9075ff05f79be0ce5d527727d6e118cc9cdc6da2e351aadfd9baa8cbdd3a76d429a695160d12c923ac9cc3baca289e193548608b82801".to_string(), 
            vk_delta_2: "07709351aa9646a1311053a0e4cfe6d7db03513beaebb9b87da192758e5ecd40d21ac535e7664e78d669399de703cb72109a5d6b3943018f1dd43462eb71be512213f05e61b2c93bc5f65d270bf78122b00e24d38b0f98efdee072cf3b4c8d0a1905dcb70f21d51fdd376d5fcd258df6c3477a2421527d1702b848954fd7a3bbf710eda0c1880b79a996516ec37d616c13082219d90a7743ad8eb5e3faceec7ad6374029d52eaeca7b66c598b3dd7066e4b6246cea47794fdcffcf7891984272".to_string(), 
            vk_ic0: "16aca3c7fb4157ef2f70fa4098434d97721a2ffa30f1ed64d3123cccb3928433899ab147217331f74f18ce687cc591700e79ca556db5b53e92f1133b889dbc11ef79615331a9a810cbef02d3a760b437a1bd50c1b6c396288abcb37479bc18a5".to_string(), 
            vk_ic1: "0c1aeb08622db17dab3de7590db8f46349c7e08eff70fa63af8332db75b977bd0e630b04d8e28d4b3416381b27f4bded12e8067fd6f65bd436608cf66f0eb0c19b7da57b72785966d71b91229cde327918d14b3330b891bdfcf255e3d0ecfbfd".to_string(),
        };

        let _res = execute(deps, mock_env(), info, msg)
            .expect("contract handles set zkeys parameters");
    }

    fn mock_bob_publish_proof_to_verify(deps: DepsMut, sent: &[Coin]) {
        let info = mock_info("bob_key", sent);
        let msg = ExecuteMsg::Proof { 
            difficuty_issuer: "alice_key".to_string(), 
            proof_a: "020fcbc0e3ae8e322a5cb6fb707d2511878eda020096f8f421cf75c263e43c2ee6d2392b6e03410d5555fd80628581f1054ce4ab8c9c277ce545b05efc145a1aeecd84038c67972a55367b2e1181c19311a7b3a3aa2b2cd70c4823db3ae498a0".to_string(), 
            proof_b: "1545a18455dd6e1abaf4e27f3ee198bb5abb199b0650030593ce0d03b7cc59d458864acc3db510efe2300f778aa797e017c8d8fa15654b1995f0e659910bbdf8c0d88ef6801e1615e664b559daa8fd139b88569e95e6058d077fb5ae6aafe93116d6254de64023b0e8b41b145bb43d53bbee70486de6dd67c00f4f05c5e6a563f3b808b942184fa3488ace3a57e90016106fcc94b8d3d95c52ca1a616348b9095e7df0ba97156e4e93e787474d19e0ea423eda0bf5ebc81efce1b12f4c22ee00".to_string(), 
            proof_c: "10406a2ada964c701668b06be2e3011bf22d9b6c6c0731f5b042a6b7ccf777d58b8e8b8b19fd711953b170d591981eb80f373990aee796b4797bb6ee63f57cbc402ce8dc2360ef18e40c5a44e8d2948e94d6c7f226f384f6cf4c0190de295b87".to_string(), 
        };

        let _res = execute(deps, mock_env(), info, msg)
            .expect("contract handles verify proof failed");
    }


    fn query_zkeys(deps: Deps) {
        let res = query(
            deps, 
            QueryMsg::IssuerZkeys { address: "alice_key".to_string() }
        ).unwrap();

        // get response
        let value: ZkeysResponse = from_binary(&res).unwrap();
        println!("zkey is :{:?}", value);
    }
    
    fn query_verification_result(deps: Deps) {
        let res = query(
            deps,
            QueryMsg::ProofResult { issuer_address: "alice_key".to_string(), prover_address: "bob_key".to_string() }
        ).unwrap();

        let value: ProofResponse = from_binary(&res).unwrap();
        print!("proof info is: {:?}", value);
    }

    #[test]
    fn sey_zkeys_and_query_works() {
        let mut deps = mock_dependencies();
        mock_init_no_price(deps.as_mut());
        mock_alice_set_zkeys(deps.as_mut(), &[]);

        query_zkeys(deps.as_ref());
    }


    #[test]
    fn verify_proof_and_query_works() {
        let mut deps = mock_dependencies();
        mock_init_no_price(deps.as_mut());
        // alice set issue difficulty
        mock_alice_set_zkeys(deps.as_mut(), &[]);

        // verify the proof of bob
        mock_bob_publish_proof_to_verify(deps.as_mut(), &[]);
        query_verification_result(deps.as_ref());
    }
}