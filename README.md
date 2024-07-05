```bash
forge install safe-contracts=safe-global/safe-smart-account@v1.4.1

git submodule add https://github.com/foundry-rs/forge-std lib/forge-std
git submodule add https://github.com/safe-global/safe-smart-account lib/safe-contracts
cd lib/safe-contracts && git checkout v1.4.1
```
