# Anon storage proof

The idea is to be able to prove that you know the preimage of a hash stored somewhere on chain

# Submodules

I struggled with submodules, so that's just a reminder for me 😅

```bash
forge install safe-contracts=safe-global/safe-smart-account@v1.4.1

git submodule add https://github.com/foundry-rs/forge-std lib/forge-std
git submodule add https://github.com/safe-global/safe-smart-account lib/safe-contracts
cd lib/safe-contracts && git checkout v1.4.1

# move
git mv lib/forge-std contracts/lib/forge-std
git mv lib/safe-contracts contracts/lib/safe-contracts
# dont know if that was useful
git submodule update --init --recursive
```
