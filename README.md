# zk-cosmwasm
A cosmos wasm contract using zero knowledge proof

## cw-groth16
This wasm contract module using groth16 proving system to verify the proof gerenated by snarkjs.

### Environment: 
- OS: Mac M1
- Rust version: stable-aarch64-apple-darwin (default)  rustc 1.69.0 (84c898d65 2023-04-16)
- wasmd version: 0.40.0-rc.2-10-gf34b566
- explorer: https://block-explorer.malaga-420.cosmwasm.com/

### Quick Start
1. Compile and Optimize the wasm code
> If you are using different os, you can change the docker image on your machine and run, otherwise you will compile failed.

If your system archtecture is `arm`,go into the `cw-groth16` dir and run with this command: 
```shell
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer-arm64:0.12.11
```
and then you will see an `artifacts` dir and next we will upload it to chain.

If your system archtecture is `amd`, run with this:
```shell
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.12.11
```

2. Store the wasm code on chain
```shell
wasmd tx wasm store artifacts/cw_groth16-aarch64.wasm --from abcd $TXFLAG -y -b sync --log_level debug
```
you can see the txhash and copy it to seach on: https://block-explorer.malaga-420.cosmwasm.com/. In my case, tx_hash is `2435197769CC3016D7C82F26D8E7A483E36E5392938B3E6B5E6127707222ABBB`

3. Query the Code_id by tx_hash
```shell
wasmd query tx 2435197769CC3016D7C82F26D8E7A483E36E5392938B3E6B5E6127707222ABBB --node https://rpc.malaga-420.cosmwasm.com:443 --type hash
```

you can find the `code_id` in the output log, here my code_id is `4945`.

4. Instantiate wasm contract
```shell
export CODE_ID=4945 `(replace it with your code_id)`
export INIT='{"set_zkeys_price":{"amount":"100","denom":"umlg"},"publish_proof_price":{"amount":"200","denom":"umlg"}}'
wasmd tx wasm instantiate $CODE_ID "$INIT" --from abcd --label "zk verify" $TXFLAG -y --no-admin
```
> Here, my tx_hash is `B4CEE80FAAFFE7224919D756199F8740F94BE525897DCEE8FD33A13F94BCE975` and the deploy is over. Next we will check the contract status.

5. Check wasm contract status   
- Get the wasm contract address:
```shell
CONTRACT=$(wasmd query wasm list-contract-by-code $CODE_ID $NODE --output json | jq -r '.contracts[-1]')
echo $CONTRACT
```
- Check the config of this contract
```shell
CONFIG_QUERY='{"config": {}}'
wasmd query wasm contract-state smart $CONTRACT "$CONFIG_QUERY" $NODE --output json
```
You will see the log with:`
{"data":{"zkeys_price":{"denom":"umlg","amount":"100"},"proof_price":{"denom":"umlg","amount":"200"}}}`, that is right!

6. Send tx to cw-groth16
> We will upload a zk difficulty to the wasm contract by the account called abcd
```shell
export ZKEYS='{"zkeys":{"public_signal":"33", "vk_alpha1": "121ec6cddca2aa0dedf8dbb86e96dc4100b58ea07c01d7ea68a37f8f72191ab2bbe9f16bfe675f71c899ff11e23cbb04064831acc8c18f561f446eeaac3a9a056cb9a89b0b3f13a57eab4e97ebaff6f0a39327bd0a4b5f725d633c87474d35f2", "vk_beta_2": "00f3edfbbbe5e2dab32cac1d1ba2f0fdd9eff4067c7152520f0ebbf556c21f98e72590b3cdb614b1ea116991305da942077b7419fac8cc2d38dc6639d68a4cf7c8362efd8395020836f3aa564537fa02a17f2d1b423c19b6cf4784037b1d9f1510afbae9e95703ff3a98c46720f05e642588ef21ccb09580c84d211c0fd60acda18a699f61cef4925b9b113c8a2377f0147c5ee0882a97519627776222438d3e29f581f0e4b61fe18ab42089dfe24a1b9d7376667382941e37329860ec84d105", "vk_gamma_2": "13e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb80606c4a02ea734cc32acd2b02bc28b99cb3e287e85a763af267492ab572e99ab3f370d275cec1da1aaa9075ff05f79be0ce5d527727d6e118cc9cdc6da2e351aadfd9baa8cbdd3a76d429a695160d12c923ac9cc3baca289e193548608b82801", "vk_delta_2": "07709351aa9646a1311053a0e4cfe6d7db03513beaebb9b87da192758e5ecd40d21ac535e7664e78d669399de703cb72109a5d6b3943018f1dd43462eb71be512213f05e61b2c93bc5f65d270bf78122b00e24d38b0f98efdee072cf3b4c8d0a1905dcb70f21d51fdd376d5fcd258df6c3477a2421527d1702b848954fd7a3bbf710eda0c1880b79a996516ec37d616c13082219d90a7743ad8eb5e3faceec7ad6374029d52eaeca7b66c598b3dd7066e4b6246cea47794fdcffcf7891984272", "vk_ic0": "16aca3c7fb4157ef2f70fa4098434d97721a2ffa30f1ed64d3123cccb3928433899ab147217331f74f18ce687cc591700e79ca556db5b53e92f1133b889dbc11ef79615331a9a810cbef02d3a760b437a1bd50c1b6c396288abcb37479bc18a5", "vk_ic1": "0c1aeb08622db17dab3de7590db8f46349c7e08eff70fa63af8332db75b977bd0e630b04d8e28d4b3416381b27f4bded12e8067fd6f65bd436608cf66f0eb0c19b7da57b72785966d71b91229cde327918d14b3330b891bdfcf255e3d0ecfbfd"}}'

wasmd tx wasm execute $CONTRACT "$ZKEYS" --amount 100umlg --from abcd $TXFLAG -y
```

Here, our tx_hash is `3D7ACDF4FAF97AA10CFE838F120645C3DF8E0686FD2F2A0FFC427E1A6D306AA0`

6. Verify the proof
> We will use another account called wallet to give proof to verify to demonstrate that he know how to how to solve the difficulty

```shell
export PROOF='{"proof": {"difficuty_issuer": "wasm1a7lnnfj2t67w3dsr2x6w20yfv5lawmvv74suu9", "proof_a": "020fcbc0e3ae8e322a5cb6fb707d2511878eda020096f8f421cf75c263e43c2ee6d2392b6e03410d5555fd80628581f1054ce4ab8c9c277ce545b05efc145a1aeecd84038c67972a55367b2e1181c19311a7b3a3aa2b2cd70c4823db3ae498a0", "proof_b": "1545a18455dd6e1abaf4e27f3ee198bb5abb199b0650030593ce0d03b7cc59d458864acc3db510efe2300f778aa797e017c8d8fa15654b1995f0e659910bbdf8c0d88ef6801e1615e664b559daa8fd139b88569e95e6058d077fb5ae6aafe93116d6254de64023b0e8b41b145bb43d53bbee70486de6dd67c00f4f05c5e6a563f3b808b942184fa3488ace3a57e90016106fcc94b8d3d95c52ca1a616348b9095e7df0ba97156e4e93e787474d19e0ea423eda0bf5ebc81efce1b12f4c22ee00", "proof_c": "10406a2ada964c701668b06be2e3011bf22d9b6c6c0731f5b042a6b7ccf777d58b8e8b8b19fd711953b170d591981eb80f373990aee796b4797bb6ee63f57cbc402ce8dc2360ef18e40c5a44e8d2948e94d6c7f226f384f6cf4c0190de295b87"}}'

wasmd tx wasm execute $CONTRACT "$PROOF" --amount 999umlg --from wallet $TXFLAG -y
```

Here, our tx_hash is `20F61C53C3720B52F3B0F56E10EEA5EFD614CFF48F655A57DE8CC86EFD9E29FE`

7. Check the verification status
```shell
export VERIFICATION_STATUS='{"proof_result": {"issuer_address": "wasm1a7lnnfj2t67w3dsr2x6w20yfv5lawmvv74suu9", "prover_address": "wasm1u0umjkuwuuueeewduau3aaqnv0fmaescfqx9kq"}}'

wasmd query wasm contract-state smart $CONTRACT "$VERIFICATION_STATUS" $NODE --output json
```
Finally, we can see that:
```json
{"data":{"proof_a":"020fcbc0e3ae8e322a5cb6fb707d2511878eda020096f8f421cf75c263e43c2ee6d2392b6e03410d5555fd80628581f1054ce4ab8c9c277ce545b05efc145a1aeecd84038c67972a55367b2e1181c19311a7b3a3aa2b2cd70c4823db3ae498a0","proof_b":"1545a18455dd6e1abaf4e27f3ee198bb5abb199b0650030593ce0d03b7cc59d458864acc3db510efe2300f778aa797e017c8d8fa15654b1995f0e659910bbdf8c0d88ef6801e1615e664b559daa8fd139b88569e95e6058d077fb5ae6aafe93116d6254de64023b0e8b41b145bb43d53bbee70486de6dd67c00f4f05c5e6a563f3b808b942184fa3488ace3a57e90016106fcc94b8d3d95c52ca1a616348b9095e7df0ba97156e4e93e787474d19e0ea423eda0bf5ebc81efce1b12f4c22ee00","proof_c":"10406a2ada964c701668b06be2e3011bf22d9b6c6c0731f5b042a6b7ccf777d58b8e8b8b19fd711953b170d591981eb80f373990aee796b4797bb6ee63f57cbc402ce8dc2360ef18e40c5a44e8d2948e94d6c7f226f384f6cf4c0190de295b87","is_valid":true}}
```
So the proof of wallet account is valid !