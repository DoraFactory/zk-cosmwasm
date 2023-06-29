#[cfg(test)]
mod test_module {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coin, coins, from_binary, Coin, Deps, DepsMut};

    use crate::contract::{execute, instantiate, query};
    use crate::error::ContractError;
    use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, ZkeysResponse, ProofResponse};
    use crate::state::Config;

    fn assert_config_state(deps: Deps, expected: Config) {
        let res = query(deps, mock_env(), QueryMsg::Config {}).unwrap();
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

    #[test]
    fn fail_set_zkeys_insufficient_fees() {
        let mut deps = mock_dependencies();
        mock_init_with_price(deps.as_mut(), coin(2, "token"), coin(2, "token"));
        let info = mock_info("alice_key", &[]);
        let msg = ExecuteMsg::Zkeys { 
            public_signal: "33".to_string(), 
            vk_alpha1: "134341fbe5f0719617003adb9c8fe9038d5d913d1a1e961618cd67f8f097d0cb203a9e851d18a4cfe8ab963083acda4af394c8c2461930397057da9edf030d4c".to_string(), 
            vk_beta_2: "26e36c244fbd85b1f96bb3c4eecc5024f9e0507247e6675e56d5d222f43921872c13498425fa4b090c401561092dac563a0864ef22aa2bcfcf0e75bda8ad94aa10e1a9938cab807dc19806127b49d697de33abf79ad5ae46ca240927dd9c57d623c3cad4c8c16360c9199a701b707474fd6bd47e8841d4ebb7a88b826459535d".to_string(), 
            vk_gamma_2: "198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa".to_string(), 
            vk_delta_2: "0c80d2c61aaac33c924c322ed740b6ed3775eb171b173c09640a29aa1322e5450dd122b23f09aab8b1a3fd739d296949f328fd2cd9deedc6d1ed3262803a19e804b9033fb8a3476eb8b5e609f529cc52fbaa9df6f59a5a67aab944cde646d13f2d17ed688afb5f3abc97f978fe9da3a25feee8bfbb0762ae80be6aabb76a742d".to_string(), 
            vk_ic0: "22f4a08cff59356634d3cd5ad41e85e4b4b484a7633eb64ddd739032936add322101b08d07ff3315947df2a6652e667d6a3e96fca1a661aadc2092a85c3912fd".to_string(), 
            vk_ic1: "17fa0f95ec76599763a6629b557bf18fd938305c7472fd81c368f8ca76615cee037b264dee54b6fd3298fd0396525cb1aec0dd971e3ab771f242ce10980763cd".to_string(),
        };
        let res = execute(deps.as_mut(), mock_env(), info, msg);
        match res {
            Ok(_) => panic!("set zkeys should fail with insufficient fees"),
            Err(ContractError::InsufficientFundsSend {}) => {}
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }


    fn mock_alice_set_zkeys(deps: DepsMut, sent: &[Coin]) {
        // alice can register an available name
        let info = mock_info("alice_key", sent);
        let msg = ExecuteMsg::Zkeys { 
            public_signal: "33".to_string(), 
            vk_alpha1: "134341fbe5f0719617003adb9c8fe9038d5d913d1a1e961618cd67f8f097d0cb203a9e851d18a4cfe8ab963083acda4af394c8c2461930397057da9edf030d4c".to_string(), 
            vk_beta_2: "26e36c244fbd85b1f96bb3c4eecc5024f9e0507247e6675e56d5d222f43921872c13498425fa4b090c401561092dac563a0864ef22aa2bcfcf0e75bda8ad94aa10e1a9938cab807dc19806127b49d697de33abf79ad5ae46ca240927dd9c57d623c3cad4c8c16360c9199a701b707474fd6bd47e8841d4ebb7a88b826459535d".to_string(), 
            vk_gamma_2: "198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa".to_string(), 
            vk_delta_2: "0c80d2c61aaac33c924c322ed740b6ed3775eb171b173c09640a29aa1322e5450dd122b23f09aab8b1a3fd739d296949f328fd2cd9deedc6d1ed3262803a19e804b9033fb8a3476eb8b5e609f529cc52fbaa9df6f59a5a67aab944cde646d13f2d17ed688afb5f3abc97f978fe9da3a25feee8bfbb0762ae80be6aabb76a742d".to_string(), 
            vk_ic0: "22f4a08cff59356634d3cd5ad41e85e4b4b484a7633eb64ddd739032936add322101b08d07ff3315947df2a6652e667d6a3e96fca1a661aadc2092a85c3912fd".to_string(), 
            vk_ic1: "17fa0f95ec76599763a6629b557bf18fd938305c7472fd81c368f8ca76615cee037b264dee54b6fd3298fd0396525cb1aec0dd971e3ab771f242ce10980763cd".to_string(),
        };

        let _res = execute(deps, mock_env(), info, msg)
            .expect("contract handles set zkeys parameters");
    }

    fn mock_alice_set_invalid_zkeys(deps: DepsMut, sent: &[Coin]) {
        // alice can register an available name
        let info = mock_info("alice_key", sent);
        let msg = ExecuteMsg::Zkeys { 
            public_signal: "33".to_string(), 
            //NOTE: invalid vk_alpha1
            vk_alpha1: "234341fbe5f0719617003adb9c8fe9038d5d913d1a1e961618cd67f8f097d0cb203a9e851d18a4cfe8ab963083acda4af394c8c2461930397057da9edf030d4c".to_string(), 
            vk_beta_2: "26e36c244fbd85b1f96bb3c4eecc5024f9e0507247e6675e56d5d222f43921872c13498425fa4b090c401561092dac563a0864ef22aa2bcfcf0e75bda8ad94aa10e1a9938cab807dc19806127b49d697de33abf79ad5ae46ca240927dd9c57d623c3cad4c8c16360c9199a701b707474fd6bd47e8841d4ebb7a88b826459535d".to_string(), 
            vk_gamma_2: "198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa".to_string(), 
            vk_delta_2: "0c80d2c61aaac33c924c322ed740b6ed3775eb171b173c09640a29aa1322e5450dd122b23f09aab8b1a3fd739d296949f328fd2cd9deedc6d1ed3262803a19e804b9033fb8a3476eb8b5e609f529cc52fbaa9df6f59a5a67aab944cde646d13f2d17ed688afb5f3abc97f978fe9da3a25feee8bfbb0762ae80be6aabb76a742d".to_string(), 
            vk_ic0: "22f4a08cff59356634d3cd5ad41e85e4b4b484a7633eb64ddd739032936add322101b08d07ff3315947df2a6652e667d6a3e96fca1a661aadc2092a85c3912fd".to_string(), 
            vk_ic1: "17fa0f95ec76599763a6629b557bf18fd938305c7472fd81c368f8ca76615cee037b264dee54b6fd3298fd0396525cb1aec0dd971e3ab771f242ce10980763cd".to_string(),
        };

        assert_eq!(execute(deps, mock_env(), info, msg), Err(ContractError::ErrorVerificationKey{}));
    }

    fn mock_alice_set_zkeys_with_different_public_signal(deps: DepsMut, sent: &[Coin]) {
        // alice can register an available name
        let info = mock_info("alice_key", sent);
        let msg = ExecuteMsg::Zkeys { 
            public_signal: "30".to_string(), 
            vk_alpha1: "134341fbe5f0719617003adb9c8fe9038d5d913d1a1e961618cd67f8f097d0cb203a9e851d18a4cfe8ab963083acda4af394c8c2461930397057da9edf030d4c".to_string(), 
            vk_beta_2: "26e36c244fbd85b1f96bb3c4eecc5024f9e0507247e6675e56d5d222f43921872c13498425fa4b090c401561092dac563a0864ef22aa2bcfcf0e75bda8ad94aa10e1a9938cab807dc19806127b49d697de33abf79ad5ae46ca240927dd9c57d623c3cad4c8c16360c9199a701b707474fd6bd47e8841d4ebb7a88b826459535d".to_string(), 
            vk_gamma_2: "198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa".to_string(), 
            vk_delta_2: "0c80d2c61aaac33c924c322ed740b6ed3775eb171b173c09640a29aa1322e5450dd122b23f09aab8b1a3fd739d296949f328fd2cd9deedc6d1ed3262803a19e804b9033fb8a3476eb8b5e609f529cc52fbaa9df6f59a5a67aab944cde646d13f2d17ed688afb5f3abc97f978fe9da3a25feee8bfbb0762ae80be6aabb76a742d".to_string(), 
            vk_ic0: "22f4a08cff59356634d3cd5ad41e85e4b4b484a7633eb64ddd739032936add322101b08d07ff3315947df2a6652e667d6a3e96fca1a661aadc2092a85c3912fd".to_string(), 
            vk_ic1: "17fa0f95ec76599763a6629b557bf18fd938305c7472fd81c368f8ca76615cee037b264dee54b6fd3298fd0396525cb1aec0dd971e3ab771f242ce10980763cd".to_string(),
        };

        let _res = execute(deps, mock_env(), info, msg)
            .expect("contract handles set zkeys parameters");
    }

    fn mock_bob_publish_proof_to_verify(deps: DepsMut, sent: &[Coin]) {
        let info = mock_info("bob_key", sent);
        let msg = ExecuteMsg::Proof { 
            difficuty_issuer: "alice_key".to_string(), 
            proof_a: "2a7efa6d4fee4a2df464f6c926a81e709ecf27642f4f99aa5c90bc479ce1122a149e00a4b97ea4ef3caec4d5ab168eb0effa1441ee448678d6e77caa2d19f3b2".to_string(), 
            proof_b: "023290eac0dc45935bb65780f2dd380c594b207509fceeb768c8d9a33a530c640a3cc26b8fa867e1484bbe1c98131cdaad2c48a370688e259f1aff52a0d393872c41442472714933964a28c649c2ebe4608f08e8e0dd023bb90df134f45d20281c7c24fd81f0affa2450181480411973b2d7b52683fdd9d4e3068ef6b5054296".to_string(), 
            proof_c: "24102019b76cd1f917b5e765519f65504ecfebfb5f4a11a168fd29048e004e0f03b05f4cd80703e6f7f51c7c3394253a4900f0186386be4015d363425ea27488".to_string(), 
        };

        let _res = execute(deps, mock_env(), info, msg)
            .expect("contract handles verify proof failed");
    }

    fn mock_bob_publish_proof_to_verify_with_different_public_signal(deps: DepsMut, sent: &[Coin]) {
        let info = mock_info("bob_key", sent);
        let msg = ExecuteMsg::Proof { 
            difficuty_issuer: "alice_key".to_string(), 
            proof_a: "2a7efa6d4fee4a2df464f6c926a81e709ecf27642f4f99aa5c90bc479ce1122a149e00a4b97ea4ef3caec4d5ab168eb0effa1441ee448678d6e77caa2d19f3b2".to_string(), 
            proof_b: "023290eac0dc45935bb65780f2dd380c594b207509fceeb768c8d9a33a530c640a3cc26b8fa867e1484bbe1c98131cdaad2c48a370688e259f1aff52a0d393872c41442472714933964a28c649c2ebe4608f08e8e0dd023bb90df134f45d20281c7c24fd81f0affa2450181480411973b2d7b52683fdd9d4e3068ef6b5054296".to_string(), 
            proof_c: "24102019b76cd1f917b5e765519f65504ecfebfb5f4a11a168fd29048e004e0f03b05f4cd80703e6f7f51c7c3394253a4900f0186386be4015d363425ea27488".to_string(), 
        };
        assert_eq!(execute(deps, mock_env(), info, msg), Err(ContractError::InvalidProof {}));
    }

    fn mock_bob_publish_error_hex_format_proof_to_verify(deps: DepsMut, sent: &[Coin]) {
        let info = mock_info("bob_key", sent);
        let msg = ExecuteMsg::Proof { 
            difficuty_issuer: "alice_key".to_string(), 
            //NOTE:  error format proof
            proof_a: "f6c926a81e709ecf27642f4f99aa5c90bc479ce1122a149e00a4b97ea4ef3caec4d5ab168eb0effa1441ee448678d6e77caa2d19f3b2".to_string(), 
            proof_b: "023290594b207509fceeb768c8d9a33a530c640a3cc26b8fa867e1484bbe1c98131cdaad2c48a370688e259f1aff52a0d393872c41442472714933964a28c649c2ebe4608f08e8e0dd023bb90df134f45d20281c7c24fd81f0affa2450181480411973b2d7b52683fdd9d4e3068ef6b5054296".to_string(), 
            proof_c: "24102019b76cd1f917b5e76555f4a11a168fd29048e004e0f03b05f4cd80703e6f7f51c7c3394253a4900f0186386be4015d363425ea27488".to_string(), 
        };

        assert_eq!(execute(deps, mock_env(), info, msg), Err(ContractError::HexDecodingError {}));
    }


    fn mock_bob_publish_invalid_proof_to_verify(deps: DepsMut, sent: &[Coin]) {
        let info = mock_info("bob_key", sent);
        let msg = ExecuteMsg::Proof { 
            difficuty_issuer: "alice_key".to_string(), 
            //NOTE: invalid proof_a
            proof_a: "3a7efa6d4fee4a2df464f6c926a81e709ecf27642f4f99aa5c90bc479ce1122a149e00a4b97ea4ef3caec4d5ab168eb0effa1441ee448678d6e77caa2d19f3b2".to_string(), 
            proof_b: "023290eac0dc45935bb65780f2dd380c594b207509fceeb768c8d9a33a530c640a3cc26b8fa867e1484bbe1c98131cdaad2c48a370688e259f1aff52a0d393872c41442472714933964a28c649c2ebe4608f08e8e0dd023bb90df134f45d20281c7c24fd81f0affa2450181480411973b2d7b52683fdd9d4e3068ef6b5054296".to_string(), 
            proof_c: "24102019b76cd1f917b5e765519f65504ecfebfb5f4a11a168fd29048e004e0f03b05f4cd80703e6f7f51c7c3394253a4900f0186386be4015d363425ea27488".to_string(), 
        };

        assert_eq!(execute(deps, mock_env(), info, msg), Err(ContractError::ErrorProof{}));
    }


    fn query_zkeys(deps: Deps) {
        let res = query(
            deps, 
            mock_env(),
            QueryMsg::IssuerZkeys { address: "alice_key".to_string() }
        ).unwrap();

        // get response
        let value: ZkeysResponse = from_binary(&res).unwrap();
        println!("zkey is :{:?}", value);
    }
    
    fn query_verification_result(deps: Deps) {
        let res = query(
            deps,
            mock_env(),
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
    fn verify_proof_and_query_works_with_no_price() {
        let mut deps = mock_dependencies();
        mock_init_no_price(deps.as_mut());
        // alice set issue difficulty
        mock_alice_set_zkeys(deps.as_mut(), &[]);

        // verify the proof of bob
        mock_bob_publish_proof_to_verify(deps.as_mut(), &[]);
        query_verification_result(deps.as_ref());
    }

    #[test]
    fn verify_proof_and_query_works_with_price() {
        let mut deps = mock_dependencies();
        mock_init_with_price(deps.as_mut(), coin(1, "token"), coin(1, "token"));
        // alice set issue difficulty
        mock_alice_set_zkeys(deps.as_mut(), &[coin(2, "token")]);

        // verify the proof of bob
        mock_bob_publish_proof_to_verify(deps.as_mut(), &[coin(2, "token")]);
        query_verification_result(deps.as_ref());
    }

    #[test]
    fn verify_proof_and_query_works_with_price_with_different_public() {
        let mut deps = mock_dependencies();
        mock_init_with_price(deps.as_mut(), coin(1, "token"), coin(1, "token"));
        // alice set issue difficulty
        mock_alice_set_zkeys_with_different_public_signal(deps.as_mut(), &[coin(2, "token")]);

        // verify the proof of bob
        mock_bob_publish_proof_to_verify_with_different_public_signal(deps.as_mut(), &[coin(2, "token")]);
    }

    #[test]
    fn verify_proof_and_query_failed_with_invalid_verification_key() {
        let mut deps = mock_dependencies();
        mock_init_with_price(deps.as_mut(), coin(1, "token"), coin(1, "token"));
        // alice set issue difficulty
        mock_alice_set_invalid_zkeys(deps.as_mut(), &[coin(2, "token")]);
    }

    #[test]
    fn verify_proof_and_query_failed_with_error_hex_format_proof() {
        let mut deps = mock_dependencies();
        mock_init_with_price(deps.as_mut(), coin(1, "token"), coin(1, "token"));
        // alice set issue difficulty
        mock_alice_set_zkeys(deps.as_mut(), &[coin(2, "token")]);

        mock_bob_publish_error_hex_format_proof_to_verify(deps.as_mut(), &[coin(2, "token")]);
    }

    #[test]
    fn verify_proof_and_query_failed_with_invalid_proof() {
        let mut deps = mock_dependencies();
        mock_init_with_price(deps.as_mut(), coin(1, "token"), coin(1, "token"));
        // alice set issue difficulty
        mock_alice_set_zkeys(deps.as_mut(), &[coin(2, "token")]);

        // verify the proof of bob
        mock_bob_publish_invalid_proof_to_verify(deps.as_mut(), &[coin(2, "token")]);
    }

}