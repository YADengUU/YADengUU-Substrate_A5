# IntroToSubstrate Assignment 5
This is the 5th coursework for the course Introduction to Substrate. To complete the tasks: 1) Implement functions for the PoE (Proof of Existence) pallet to `create` and `revoke` a claim, 2) Implement another function to `transfer` a claim. 
## Preparation for the environment
The version used for this assignment is `polkadot-v0.9.37`. To complete this assignment, I forked the repository of the `substrate-node-template` (https://github.com/substrate-developer-hub/substrate-node-template.git) here and fetched the version required with the following:
```
git remote add upstream https://github.com/substrate-developer-hub/substrate-node-template.git
git fetch --tags upstream
git checkout tags/polkadot-v0.9.37
git checkout -b coursework5
```
Here, the branch `coursework5` is where I edited the scripts.
## Working on the assignment
Following the demo in the lecture, I added the folder `poe` in `pallets` and within `src` the functions were written in `lib.rs`. For the first part, I wrote the functions in the same way as the lecture. For the second part, I added the function `tranfer_claim` which requires one more input (the recipient) compared the previous two, i.e., `transfer_claim(origin: OriginFor<T>, claim: BoundedVec<u8, T::MaxClaimLength>, recipient: T::AccountId)` as well as a new error `SameOwner` in the case that the recipient happens to be the same as the sender.
Before successfully building the project, I had several attempts to downgrade Rust as well as correcting some setups according to the prompts in the error messages. Finally, I managed to run the project and perform the tasks in the interactive page (polkadot.js.org/apps/#/explorer):
### The blockchain is built and run
![blockchain_running](https://github.com/YADengUU/YADengUU-Substrate_A5/assets/131147818/03117140-98ad-455c-865b-0c2d5b67c98a)
### If the claim is not created yet, we can see it is "none"
![has_not_create_claims](https://github.com/YADengUU/YADengUU-Substrate_A5/assets/131147818/f9f9756a-03fa-41cb-89c7-d3c1e061a463)
### The three functions are also successfully written
![three_functions](https://github.com/YADengUU/YADengUU-Substrate_A5/assets/131147818/9575e808-1d2e-44b6-8c52-714c7885ea89)
### As in the lecture demo, claim `0x01` is created for Alice
![created_for_alice](https://github.com/YADengUU/YADengUU-Substrate_A5/assets/131147818/462ceb31-dd23-4362-864c-ca1b7c3d611a)
### If we try to transfer a non-existing claim, it can show the error:
![non_existing_transfer](https://github.com/YADengUU/YADengUU-Substrate_A5/assets/131147818/9a5b5466-3f6f-4d48-97b3-f4c7d0f62a83)
![non_exist_error](https://github.com/YADengUU/YADengUU-Substrate_A5/assets/131147818/c3427fce-e16a-4d00-9550-fd005c0d96ac)
### Anyway, we are able to see the successful transfer of the claim `0x01` from Alice to Bob:
![successful_transfer](https://github.com/YADengUU/YADengUU-Substrate_A5/assets/131147818/5b4f7a61-2355-49fb-b78f-0be70286ac27)
