#[cfg(test)]
mod test_module {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coin, coins, from_binary, Coin, Deps, DepsMut};

    use crate::contract::{execute, instantiate, query};
    use crate::error::ContractError;
    use crate::msg::{ExecuteMsg, InstantiateMsg, ProofResponse, QueryMsg, ZkeysResponse};
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
            n: 3,
            num_inputs: 1,
            selector_commitments: [
                "09b3a8742e323fbb6b7e858287af59c6ff997667de6f10136356774a5e93fe872fd1ef45f38c9c0814183b2ca7eba4e2d5d5d7f871bb1a89e96217df74c9833d".to_string(),
                "40000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string(),
                "2b63946d2ccf8529ae7ba5324902e7ee834b19e8dd666b3533850c2ed36d7dfe2738c050f06ba19dd919bbb1e55c408ceadc583bbd190f10f54a9b79bb2b03da".to_string(),
                "40000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string(),
                "2b63946d2ccf8529ae7ba5324902e7ee834b19e8dd666b3533850c2ed36d7dfe092b8e21f0c5fe8bdf368a049c2517d0aca51255ab58bb7c46d5f09d1d51f96d".to_string(),
                "40000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string()
              ].to_vec(),
            next_step_selector_commitments: [
                "40000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string()
              ].to_vec(),
            permutation_commitments: [
                "1516cf1540873838abfa4e7501f523c2c56ce33a33f2cefc2da70cb30795bd610182f97175b9b3bc30e175307758894dd2938fcf556669f9820f1c1d2089f379".to_string(),
                "19c5d1b7df1125f24646c8e7ac90689c6cd593703e7cd4d7918e3d7c9c2f220a0daf49f7dfa3f8d741df0efcef1f171c961b47f8b28d46a2af00be85c8b6e713".to_string(),
                "14ab37e299bdf4502f0034f4ee42281108e1ae6eab93e1c435e1aa68beb5153622def6b03001792d9fd384af642febcc6cc821726783d359ebfe6b30c9c0915a".to_string(),
                "1f8dbc422d4aabab7112ee68c336bbe2bbe095d6d7fa6e5a7f1292ea7487412f0beb14b6bfb14c16a44f4e37d98e401cb38c6221b9f4ccebc52ec8088c44a2a8".to_string()
              ].to_vec(),
            non_residues: [
                "0000000000000000000000000000000000000000000000000000000000000005".to_string(),
                "0000000000000000000000000000000000000000000000000000000000000007".to_string(),
                "000000000000000000000000000000000000000000000000000000000000000a".to_string()
              ].to_vec(),
            g2_elements: [
                "198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa".to_string(),
                "12740934ba9615b77b6a49b06fcce83ce90d67b1d0e2a530069e3a7306569a91116da8c89a0d090f3d8644ada33a5f1c8013ba7204aeca62d66d931b99afe6e725222d9816e5f86b4a7dedd00d04acc5c979c18bd22b834ea8c6d07c0ba441db076441042e77b6309644b56251f059cf14befc72ac8a6157d30924e58dc4c172".to_string()
              ].to_vec()
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
            n: 3,
            num_inputs: 1,
            selector_commitments: [
                "09b3a8742e323fbb6b7e858287af59c6ff997667de6f10136356774a5e93fe872fd1ef45f38c9c0814183b2ca7eba4e2d5d5d7f871bb1a89e96217df74c9833d".to_string(),
                "40000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string(),
                "2b63946d2ccf8529ae7ba5324902e7ee834b19e8dd666b3533850c2ed36d7dfe2738c050f06ba19dd919bbb1e55c408ceadc583bbd190f10f54a9b79bb2b03da".to_string(),
                "40000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string(),
                "2b63946d2ccf8529ae7ba5324902e7ee834b19e8dd666b3533850c2ed36d7dfe092b8e21f0c5fe8bdf368a049c2517d0aca51255ab58bb7c46d5f09d1d51f96d".to_string(),
                "40000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string()
              ].to_vec(),
            next_step_selector_commitments: [
                "40000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string()
              ].to_vec(),
            permutation_commitments: [
                "1516cf1540873838abfa4e7501f523c2c56ce33a33f2cefc2da70cb30795bd610182f97175b9b3bc30e175307758894dd2938fcf556669f9820f1c1d2089f379".to_string(),
                "19c5d1b7df1125f24646c8e7ac90689c6cd593703e7cd4d7918e3d7c9c2f220a0daf49f7dfa3f8d741df0efcef1f171c961b47f8b28d46a2af00be85c8b6e713".to_string(),
                "14ab37e299bdf4502f0034f4ee42281108e1ae6eab93e1c435e1aa68beb5153622def6b03001792d9fd384af642febcc6cc821726783d359ebfe6b30c9c0915a".to_string(),
                "1f8dbc422d4aabab7112ee68c336bbe2bbe095d6d7fa6e5a7f1292ea7487412f0beb14b6bfb14c16a44f4e37d98e401cb38c6221b9f4ccebc52ec8088c44a2a8".to_string()
              ].to_vec(),
            non_residues: [
                "0000000000000000000000000000000000000000000000000000000000000005".to_string(),
                "0000000000000000000000000000000000000000000000000000000000000007".to_string(),
                "000000000000000000000000000000000000000000000000000000000000000a".to_string()
              ].to_vec(),
            g2_elements: [
                "198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa".to_string(),
                "12740934ba9615b77b6a49b06fcce83ce90d67b1d0e2a530069e3a7306569a91116da8c89a0d090f3d8644ada33a5f1c8013ba7204aeca62d66d931b99afe6e725222d9816e5f86b4a7dedd00d04acc5c979c18bd22b834ea8c6d07c0ba441db076441042e77b6309644b56251f059cf14befc72ac8a6157d30924e58dc4c172".to_string()
              ].to_vec()
        };

        let _res =
            execute(deps, mock_env(), info, msg).expect("contract handles set zkeys parameters");
    }

    fn mock_alice_set_invalid_zkeys(deps: DepsMut, sent: &[Coin]) {
        // alice can register an available name
        let info = mock_info("alice_key", sent);
        let msg = ExecuteMsg::Zkeys {
            //NOTE: invalid vk_alpha1
            n: 3,
            num_inputs: 1,
            selector_commitments: [
                // NOTE: this is invalid item
                "0000".to_string(),
                "40000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string(),
                "2b63946d2ccf8529ae7ba5324902e7ee834b19e8dd666b3533850c2ed36d7dfe2738c050f06ba19dd919bbb1e55c408ceadc583bbd190f10f54a9b79bb2b03da".to_string(),
                "40000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string(),
                "2b63946d2ccf8529ae7ba5324902e7ee834b19e8dd666b3533850c2ed36d7dfe092b8e21f0c5fe8bdf368a049c2517d0aca51255ab58bb7c46d5f09d1d51f96d".to_string(),
                "40000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string()
              ].to_vec(),
            next_step_selector_commitments: [
                "40000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string()
              ].to_vec(),
            permutation_commitments: [
                "1516cf1540873838abfa4e7501f523c2c56ce33a33f2cefc2da70cb30795bd610182f97175b9b3bc30e175307758894dd2938fcf556669f9820f1c1d2089f379".to_string(),
                "19c5d1b7df1125f24646c8e7ac90689c6cd593703e7cd4d7918e3d7c9c2f220a0daf49f7dfa3f8d741df0efcef1f171c961b47f8b28d46a2af00be85c8b6e713".to_string(),
                "14ab37e299bdf4502f0034f4ee42281108e1ae6eab93e1c435e1aa68beb5153622def6b03001792d9fd384af642febcc6cc821726783d359ebfe6b30c9c0915a".to_string(),
                "1f8dbc422d4aabab7112ee68c336bbe2bbe095d6d7fa6e5a7f1292ea7487412f0beb14b6bfb14c16a44f4e37d98e401cb38c6221b9f4ccebc52ec8088c44a2a8".to_string()
              ].to_vec(),
            non_residues: [
                "0000000000000000000000000000000000000000000000000000000000000005".to_string(),
                "0000000000000000000000000000000000000000000000000000000000000007".to_string(),
                "000000000000000000000000000000000000000000000000000000000000000a".to_string()
              ].to_vec(),
            g2_elements: [
                "198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa".to_string(),
                "12740934ba9615b77b6a49b06fcce83ce90d67b1d0e2a530069e3a7306569a91116da8c89a0d090f3d8644ada33a5f1c8013ba7204aeca62d66d931b99afe6e725222d9816e5f86b4a7dedd00d04acc5c979c18bd22b834ea8c6d07c0ba441db076441042e77b6309644b56251f059cf14befc72ac8a6157d30924e58dc4c172".to_string()
              ].to_vec()
        };

        assert_eq!(
            execute(deps, mock_env(), info, msg),
            Err(ContractError::ErrorVerificationKey {})
        );
    }

    fn mock_alice_set_zkeys_with_different_public_signal(deps: DepsMut, sent: &[Coin]) {
        // alice can register an available name
        let info = mock_info("alice_key", sent);
        let msg = ExecuteMsg::Zkeys {
            n: 3,
            num_inputs: 1,
            selector_commitments: [
                "09b3a8742e323fbb6b7e858287af59c6ff997667de6f10136356774a5e93fe872fd1ef45f38c9c0814183b2ca7eba4e2d5d5d7f871bb1a89e96217df74c9833d".to_string(),
                "40000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string(),
                "2b63946d2ccf8529ae7ba5324902e7ee834b19e8dd666b3533850c2ed36d7dfe2738c050f06ba19dd919bbb1e55c408ceadc583bbd190f10f54a9b79bb2b03da".to_string(),
                "40000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string(),
                "2b63946d2ccf8529ae7ba5324902e7ee834b19e8dd666b3533850c2ed36d7dfe092b8e21f0c5fe8bdf368a049c2517d0aca51255ab58bb7c46d5f09d1d51f96d".to_string(),
                "40000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string()
              ].to_vec(),
            next_step_selector_commitments: [
                "40000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string()
              ].to_vec(),
            permutation_commitments: [
                "1516cf1540873838abfa4e7501f523c2c56ce33a33f2cefc2da70cb30795bd610182f97175b9b3bc30e175307758894dd2938fcf556669f9820f1c1d2089f379".to_string(),
                "19c5d1b7df1125f24646c8e7ac90689c6cd593703e7cd4d7918e3d7c9c2f220a0daf49f7dfa3f8d741df0efcef1f171c961b47f8b28d46a2af00be85c8b6e713".to_string(),
                "14ab37e299bdf4502f0034f4ee42281108e1ae6eab93e1c435e1aa68beb5153622def6b03001792d9fd384af642febcc6cc821726783d359ebfe6b30c9c0915a".to_string(),
                "1f8dbc422d4aabab7112ee68c336bbe2bbe095d6d7fa6e5a7f1292ea7487412f0beb14b6bfb14c16a44f4e37d98e401cb38c6221b9f4ccebc52ec8088c44a2a8".to_string()
              ].to_vec(),
            non_residues: [
                "0000000000000000000000000000000000000000000000000000000000000005".to_string(),
                "0000000000000000000000000000000000000000000000000000000000000007".to_string(),
                "000000000000000000000000000000000000000000000000000000000000000a".to_string()
              ].to_vec(),
            g2_elements: [
                "198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa".to_string(),
                "12740934ba9615b77b6a49b06fcce83ce90d67b1d0e2a530069e3a7306569a91116da8c89a0d090f3d8644ada33a5f1c8013ba7204aeca62d66d931b99afe6e725222d9816e5f86b4a7dedd00d04acc5c979c18bd22b834ea8c6d07c0ba441db076441042e77b6309644b56251f059cf14befc72ac8a6157d30924e58dc4c172".to_string()
              ].to_vec()
        };

        let _res =
            execute(deps, mock_env(), info, msg).expect("contract handles set zkeys parameters");
    }

    fn mock_bob_publish_proof_to_verify(deps: DepsMut, sent: &[Coin]) {
        let info = mock_info("bob_key", sent);
        let msg = ExecuteMsg::Proof {
            difficuty_issuer: "alice_key".to_string(),
            num_inputs: 1,
            n: 3,
            input_values: [
              "0000000000000000000000000000000000000000000000000000000000000021".to_string()
            ].to_vec(),
            wire_commitments: [
              "1c7ade6b7b63a79bbdf4380ead5793e175e904e8a659019474f25263121599f70a5f7d13d1d549c1b5fa4695d519dce3567aa336d0ced40ce00e6dd9ad77d8c5".to_string(),
              "238e7e9105d66b54ebcf23b1ac4a8cc9179850a8bb3d9ca0495e5420ff6318901a82fb4a34ed559e3f25de9b0ca7903a7e0692bee2705809e59ed50355e89920".to_string(),
              "238e7e9105d66b54ebcf23b1ac4a8cc9179850a8bb3d9ca0495e5420ff6318901a82fb4a34ed559e3f25de9b0ca7903a7e0692bee2705809e59ed50355e89920".to_string(),
              "40000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string()
            ].to_vec(),
            grand_product_commitment: "2ebc09f9ada0ba725ea8d6c06d4c49e156578fc34388aff597a2ef4c94113eab10c2861217b6f698e334359465f85d23bcd3467a72ccefa92fcdef5cf63bb774".to_string(),
            quotient_poly_commitments: [
              "1f3e303dc35d69a2886a3831bb88d927273367eda400c1a1195649f7424ce49103668c478ed318e56e3b4f7104877fd0f8255d4a2d39b019cbaba99a0f3285c5".to_string(),
              "19019a506cb3f41e5748e268b32ba946af045a8b4015561fd36e13f001a201ec1fed0f9d8b21e555372e80803621df8c7179f800e37a3cfbcc48ea6c18fe68ba".to_string(),
              "077cd81c81628f91c271a9c78c5e3c208b16db7af4135e9d4484aeb3fe74949f125e6bf4e044a1ebb40dca3b9f7f6298d5e6cdacc2abbbea467293987d864184".to_string(),
              "40000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string()
            ].to_vec(),
            wire_values_at_z: [
              "24bfaac457bacdfb08eb4cac55f60f11f5f691b15e048a4fe0c3016991d92aeb".to_string(),
              "28226e41e356f1e4de481016a45f3f9c07e17e966dda962f4a088d1b325684e9".to_string(),
              "28226e41e356f1e4de481016a45f3f9c07e17e966dda962f4a088d1b325684e9".to_string(),
              "0000000000000000000000000000000000000000000000000000000000000000".to_string()
            ].to_vec(),
            wire_values_at_z_omega: [
              "0000000000000000000000000000000000000000000000000000000000000000".to_string()
            ].to_vec(),
            grand_product_at_z_omega: "1ea357b0967d029cb8714f44c66f447ccff76231ff86ea9416c0e0f60a4e40ed".to_string(),
            quotient_polynomial_at_z: "0d3743a423440f9130eb1c36815f549442f836c734f38e1a75278d515f5c40f8".to_string(),
            linearization_polynomial_at_z: "00c87a6192b8985101007b6280c4d90435b4f31cc20e833367637e299c957857".to_string(),
            permutation_polynomials_at_z: [
                "0853bcc8f41416377af6ad7364aa07a1b981e3f1aa98ccab9b7b26dc197f30cd".to_string(),
                "2bc9fec71fdd891baff67f9af178e16c2c51b3c609b3fd372bbc01100fe1eb0b".to_string(),
                "1ab9c6f84d62c1d272cb3b3efa25d288720a396c091d3da6a1c0b37e266053d3".to_string()
              ].to_vec(),
            opening_at_z_proof: "1d99eae30fa0e2d2a330647c3245eceb92dcc0fae92308f6003cd628e6694e682ff1a84ee025d6609fa9b16a29ff7e5bf5ea08932bd6813b989a084d28cb72c4".to_string(),
            opening_at_z_omega_proof: "2bcf1e082d97cbc88e318001fc8588be7efb1d60624d8917c9babdde02469a402bb78b2bb7e8e76635d6e34674f6255b05558b8a2de52ff00535cec6bccca8a5".to_string()
        };

        let _res =
            execute(deps, mock_env(), info, msg).expect("contract handles verify proof failed");
    }

    fn mock_bob_publish_proof_to_verify_with_different_public_signal(deps: DepsMut, sent: &[Coin]) {
        let info = mock_info("bob_key", sent);
        let msg = ExecuteMsg::Proof {
            difficuty_issuer: "alice_key".to_string(),
            num_inputs: 1,
            n: 3,
            input_values: [
              "0000000000000000000000000000000000000000000000000000000000000022".to_string()
            ].to_vec(),
            wire_commitments: [
              "1c7ade6b7b63a79bbdf4380ead5793e175e904e8a659019474f25263121599f70a5f7d13d1d549c1b5fa4695d519dce3567aa336d0ced40ce00e6dd9ad77d8c5".to_string(),
              "238e7e9105d66b54ebcf23b1ac4a8cc9179850a8bb3d9ca0495e5420ff6318901a82fb4a34ed559e3f25de9b0ca7903a7e0692bee2705809e59ed50355e89920".to_string(),
              "238e7e9105d66b54ebcf23b1ac4a8cc9179850a8bb3d9ca0495e5420ff6318901a82fb4a34ed559e3f25de9b0ca7903a7e0692bee2705809e59ed50355e89920".to_string(),
              "40000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string()
            ].to_vec(),
            grand_product_commitment: "2ebc09f9ada0ba725ea8d6c06d4c49e156578fc34388aff597a2ef4c94113eab10c2861217b6f698e334359465f85d23bcd3467a72ccefa92fcdef5cf63bb774".to_string(),
            quotient_poly_commitments: [
              "1f3e303dc35d69a2886a3831bb88d927273367eda400c1a1195649f7424ce49103668c478ed318e56e3b4f7104877fd0f8255d4a2d39b019cbaba99a0f3285c5".to_string(),
              "19019a506cb3f41e5748e268b32ba946af045a8b4015561fd36e13f001a201ec1fed0f9d8b21e555372e80803621df8c7179f800e37a3cfbcc48ea6c18fe68ba".to_string(),
              "077cd81c81628f91c271a9c78c5e3c208b16db7af4135e9d4484aeb3fe74949f125e6bf4e044a1ebb40dca3b9f7f6298d5e6cdacc2abbbea467293987d864184".to_string(),
              "40000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string()
            ].to_vec(),
            wire_values_at_z: [
              "24bfaac457bacdfb08eb4cac55f60f11f5f691b15e048a4fe0c3016991d92aeb".to_string(),
              "28226e41e356f1e4de481016a45f3f9c07e17e966dda962f4a088d1b325684e9".to_string(),
              "28226e41e356f1e4de481016a45f3f9c07e17e966dda962f4a088d1b325684e9".to_string(),
              "0000000000000000000000000000000000000000000000000000000000000000".to_string()
            ].to_vec(),
            wire_values_at_z_omega: [
              "0000000000000000000000000000000000000000000000000000000000000000".to_string()
            ].to_vec(),
            grand_product_at_z_omega: "1ea357b0967d029cb8714f44c66f447ccff76231ff86ea9416c0e0f60a4e40ed".to_string(),
            quotient_polynomial_at_z: "0d3743a423440f9130eb1c36815f549442f836c734f38e1a75278d515f5c40f8".to_string(),
            linearization_polynomial_at_z: "00c87a6192b8985101007b6280c4d90435b4f31cc20e833367637e299c957857".to_string(),
            permutation_polynomials_at_z: [
                "0853bcc8f41416377af6ad7364aa07a1b981e3f1aa98ccab9b7b26dc197f30cd".to_string(),
                "2bc9fec71fdd891baff67f9af178e16c2c51b3c609b3fd372bbc01100fe1eb0b".to_string(),
                "1ab9c6f84d62c1d272cb3b3efa25d288720a396c091d3da6a1c0b37e266053d3".to_string()
              ].to_vec(),
            opening_at_z_proof: "1d99eae30fa0e2d2a330647c3245eceb92dcc0fae92308f6003cd628e6694e682ff1a84ee025d6609fa9b16a29ff7e5bf5ea08932bd6813b989a084d28cb72c4".to_string(),
            opening_at_z_omega_proof: "2bcf1e082d97cbc88e318001fc8588be7efb1d60624d8917c9babdde02469a402bb78b2bb7e8e76635d6e34674f6255b05558b8a2de52ff00535cec6bccca8a5".to_string()
        };
        assert_eq!(
            execute(deps, mock_env(), info, msg),
            Err(ContractError::InvalidProof {})
        );
    }

    fn mock_bob_publish_error_hex_format_proof_to_verify(deps: DepsMut, sent: &[Coin]) {
        let info = mock_info("bob_key", sent);
        let msg = ExecuteMsg::Proof {
            difficuty_issuer: "alice_key".to_string(),
            num_inputs: 1,
            n: 3,
            input_values: [
              "0000000000000000000000000000000000000000000000000000000000000021".to_string()
            ].to_vec(),
            wire_commitments: [
              "1c7ade6b7b63a79bbdf4380ead5793e175e904e8a659019474f25263121599f70a5f7d13d1d549c1b5fa4695d519dce3567aa336d0ced40ce00e6dd9ad77d8c5".to_string(),
              "238e7e9105d66b54ebcf23b1ac4a8cc9179850a8bb3d9ca0495e5420ff6318901a82fb4a34ed559e3f25de9b0ca7903a7e0692bee2705809e59ed50355e89920".to_string(),
              "238e7e9105d66b54ebcf23b1ac4a8cc9179850a8bb3d9ca0495e5420ff6318901a82fb4a34ed559e3f25de9b0ca7903a7e0692bee2705809e59ed50355e89920".to_string(),
              "40000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string()
            ].to_vec(),
            grand_product_commitment: "2ebc09f9ada0ba725ea8d6c06d4c49e156578fc34388aff597a2ef4c94113eab10c2861217b6f698e334359465f85d23bcd3467a72ccefa92fcdef5cf63bb774".to_string(),
            quotient_poly_commitments: [
              "1f3e303dc35d69a2886a3831bb88d927273367eda400c1a1195649f7424ce49103668c478ed318e56e3b4f7104877fd0f8255d4a2d39b019cbaba99a0f3285c5".to_string(),
              "19019a506cb3f41e5748e268b32ba946af045a8b4015561fd36e13f001a201ec1fed0f9d8b21e555372e80803621df8c7179f800e37a3cfbcc48ea6c18fe68ba".to_string(),
              "077cd81c81628f91c271a9c78c5e3c208b16db7af4135e9d4484aeb3fe74949f125e6bf4e044a1ebb40dca3b9f7f6298d5e6cdacc2abbbea467293987d864184".to_string(),
              "40000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string()
            ].to_vec(),
            wire_values_at_z: [
              "24bfaac457bacdfb08eb4cac55f60f11f5f691b15e048a4fe0c3016991d92aec".to_string(),
              "28226e41e356f1e4de481016a45f3f9c07e17e966dda962f4a088d1b325684e9".to_string(),
              "28226e41e356f1e4de481016a45f3f9c07e17e966dda962f4a088d1b325684e9".to_string(),
              "0000000000000000000000000000000000000000000000000000000000000000".to_string()
            ].to_vec(),
            wire_values_at_z_omega: [
              "0000000000000000000000000000000000000000000000000000000000000000".to_string()
            ].to_vec(),
            grand_product_at_z_omega: "1ea357b0967d029cb8714f44c66f447ccff76231ff86ea9416c0e0f60a4e40ed".to_string(),
            quotient_polynomial_at_z: "0d3743a423440f9130eb1c36815f549442f836c734f38e1a75278d515f5c40f8".to_string(),
            linearization_polynomial_at_z: "00c87a6192b8985101007b6280c4d90435b4f31cc20e833367637e299c957857".to_string(),
            permutation_polynomials_at_z: [
                "0853bcc8f41416377af6ad7364aa07a1b981e3f1aa98ccab9b7b26dc197f30cd".to_string(),
                "2bc9fec71fdd891baff67f9af178e16c2c51b3c609b3fd372bbc01100fe1eb0b".to_string(),
                "1ab9c6f84d62c1d272cb3b3efa25d288720a396c091d3da6a1c0b37e266053d3".to_string()
              ].to_vec(),
            opening_at_z_proof: "1d99eae30fa0e2d2a330647c3245eceb92dcc0fae92308f6003cd628e6694e682ff1a84ee025d6609fa9b16a29ff7e5bf5ea08932bd6813b989a084d28cb72c4".to_string(),
            opening_at_z_omega_proof: "2bcf1e082d97cbc88e318001fc8588be7efb1d60624d8917c9babdde02469a402bb78b2bb7e8e76635d6e34674f6255b05558b8a2de52ff00535cec6bccca8a5".to_string()
        };

        assert_eq!(
            execute(deps, mock_env(), info, msg),
            Err(ContractError::HexDecodingError {})
        );
    }

    fn mock_bob_publish_invalid_proof_to_verify(deps: DepsMut, sent: &[Coin]) {
        let info = mock_info("bob_key", sent);
        let msg = ExecuteMsg::Proof {
            difficuty_issuer: "alice_key".to_string(),
            num_inputs: 1,
            n: 3,
            input_values: [
              "0000000000000000000000000000000000000000000000000000000000000021".to_string()
            ].to_vec(),
            wire_commitments: [
              "1c7ade6b7b63a79bbdf4380ead5793e175e904e8a659019474f25263121599f70a5f7d13d1d549c1b5fa4695d519dce3567aa336d0ced40ce00e6dd9ad77d8c5".to_string(),
              "238e7e9105d66b54ebcf23b1ac4a8cc9179850a8bb3d9ca0495e5420ff6318901a82fb4a34ed559e3f25de9b0ca7903a7e0692bee2705809e59ed50355e89920".to_string(),
              "238e7e9105d66b54ebcf23b1ac4a8cc9179850a8bb3d9ca0495e5420ff6318901a82fb4a34ed559e3f25de9b0ca7903a7e0692bee2705809e59ed50355e89920".to_string(),
              "40000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string()
            ].to_vec(),
            grand_product_commitment: "2ebc09f9ada0ba725ea8d6c06d4c49e156578fc34388aff597a2ef4c94113eab10c2861217b6f698e334359465f85d23bcd3467a72ccefa92fcdef5cf63bb774".to_string(),
            quotient_poly_commitments: [
              // NOTE: This is invalid
              "19019a506cb3f41e5748e268b32ba946af045a8b4015561fd36e13f001a201ec1fed0f9d8b21e555372e80803621df8c7179f800e37a3cfbcc48ea6c18fe68ba".to_string(),
              "19019a506cb3f41e5748e268b32ba946af045a8b4015561fd36e13f001a201ec1fed0f9d8b21e555372e80803621df8c7179f800e37a3cfbcc48ea6c18fe68ba".to_string(),
              "077cd81c81628f91c271a9c78c5e3c208b16db7af4135e9d4484aeb3fe74949f125e6bf4e044a1ebb40dca3b9f7f6298d5e6cdacc2abbbea467293987d864184".to_string(),
              "40000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string()
            ].to_vec(),
            wire_values_at_z: [
              "24bfaac457bacdfb08eb4cac55f60f11f5f691b15e048a4fe0c3016991d92aeb".to_string(),
              "28226e41e356f1e4de481016a45f3f9c07e17e966dda962f4a088d1b325684e9".to_string(),
              "28226e41e356f1e4de481016a45f3f9c07e17e966dda962f4a088d1b325684e9".to_string(),
              "0000000000000000000000000000000000000000000000000000000000000000".to_string()
            ].to_vec(),
            wire_values_at_z_omega: [
              "0000000000000000000000000000000000000000000000000000000000000000".to_string()
            ].to_vec(),
            grand_product_at_z_omega: "1ea357b0967d029cb8714f44c66f447ccff76231ff86ea9416c0e0f60a4e40ed".to_string(),
            quotient_polynomial_at_z: "0d3743a423440f9130eb1c36815f549442f836c734f38e1a75278d515f5c40f8".to_string(),
            linearization_polynomial_at_z: "00c87a6192b8985101007b6280c4d90435b4f31cc20e833367637e299c957857".to_string(),
            permutation_polynomials_at_z: [
                "0853bcc8f41416377af6ad7364aa07a1b981e3f1aa98ccab9b7b26dc197f30cd".to_string(),
                "2bc9fec71fdd891baff67f9af178e16c2c51b3c609b3fd372bbc01100fe1eb0b".to_string(),
                "1ab9c6f84d62c1d272cb3b3efa25d288720a396c091d3da6a1c0b37e266053d3".to_string()
              ].to_vec(),
            opening_at_z_proof: "1d99eae30fa0e2d2a330647c3245eceb92dcc0fae92308f6003cd628e6694e682ff1a84ee025d6609fa9b16a29ff7e5bf5ea08932bd6813b989a084d28cb72c4".to_string(),
            opening_at_z_omega_proof: "2bcf1e082d97cbc88e318001fc8588be7efb1d60624d8917c9babdde02469a402bb78b2bb7e8e76635d6e34674f6255b05558b8a2de52ff00535cec6bccca8a5".to_string()
        };

        assert_eq!(
            execute(deps, mock_env(), info, msg),
            Err(ContractError::InvalidProof {})
        );
    }

    fn query_zkeys(deps: Deps) {
        let res = query(
            deps,
            mock_env(),
            QueryMsg::IssuerZkeys {
                address: "alice_key".to_string(),
            },
        )
        .unwrap();

        // get response
        let value: ZkeysResponse = from_binary(&res).unwrap();
        // println!("zkey is :{:?}", value);
    }

    fn query_verification_result(deps: Deps) {
        let res = query(
            deps,
            mock_env(),
            QueryMsg::ProofResult {
                issuer_address: "alice_key".to_string(),
                prover_address: "bob_key".to_string(),
            },
        )
        .unwrap();

        let value: ProofResponse = from_binary(&res).unwrap();
        // print!("proof info is: {:?}", value);
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
        mock_bob_publish_proof_to_verify_with_different_public_signal(
            deps.as_mut(),
            &[coin(2, "token")],
        );
    }

    #[test]
    fn verify_proof_and_query_failed_with_invalid_verification_key() {
        let mut deps = mock_dependencies();
        mock_init_with_price(deps.as_mut(), coin(1, "token"), coin(1, "token"));
        // alice set issue difficulty
        mock_alice_set_invalid_zkeys(deps.as_mut(), &[coin(2, "token")]);
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
