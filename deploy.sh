near deploy --wasmFile out/main.wasm --accountId $CONTRACT_ID

near call $CONTRACT_ID new_default_meta '{"owner_id": "'$CONTRACT_ID'", "total_supply": "10000"}' --accountId $CONTRACT_ID

near call $CONTRACT_ID give_random '{"receipent": "'$CONTRACT_ID'"}' --accountId $CONTRACT_ID