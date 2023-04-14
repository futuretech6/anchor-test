# anchor test

## dependency

```bash
$ anchor --version
anchor-cli 0.27.0

$ solana --version
solana-cli 1.14.16 (src:0fb2ffda; feat:3488713414)
```

## dev

```bash
pre-commit install

yarn dlx @yarnpkg/sdks vscode  # for yarn3 use in vscode
```

## test

```bash
anchor deploy
anchor test --skip-deploy
```
