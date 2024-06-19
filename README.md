# peregrinus

#### anchor init (Initializes a workspace)
```shell
anchor init --template multiple program-name
```


#### anchor new (Creates a new program)
```shell
anchor new --template multiple program-name
```

####  IDL fetch (Download idl json data with prgramID, But check config `Anchor.toml` net url was target url first)
```shell
anchor idl fetch -o <out-file.json> <program-id>
```


#### Idl Authority (输出IDL帐户的权限。这是一个能够更新IDL的钱包)
```shell
anchor idl authority <program-id>
```

#### Idl Erase Authority (擦除 IDL 帐户的权限，以便无法再进行升级。配置的钱包必须是有权限的)
```shell
anchor idl authority <program-id>
```


#### Idl Upgrade (将链上的 IDL 文件升级到新的 target/idl/program.json idl。配置的钱包必须是有权限的)
```shell
anchor idl upgrade <program-id> -f <target/idl/program.json>
```


#### Upgrade (使用 Solana 的可升级 BPF 加载器来升级链上程序代码)
```shell
anchor upgrade <target/deploy/program.so> --program-id <program-id>
```


#### Upgrade (验证链上字节码是否与本地编译的匹配)
```shell
anchor verify <program-id>
```
