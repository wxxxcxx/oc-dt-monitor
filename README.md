# oc-dt-monitor

oc-dt-monitor 是一个用于监测 Oracle Cloud 数据传输用量的应用。
oc-dt-monitor 可以周期性的检测 Oracle Cloud 的数据传输用量，并且可以在数据传输用量超出设定的值后自动停止实例防止扣费。

## 使用

1. 安装 oracle 提供的 oci 终端工具，在个人资料->API keys 里创建一个API key，创建成功后下载密钥文件并保存配置文件（需要修改配置文件中的密钥文件的路径）。
2. 使用 `oci compartment list --config-file <保存的配置文件路径>` 测试 oci 是否能够正确的连接上你的账户。看到正确的输出后进行下一步。
3. 使用命令 oc-dt-monitor --tenant-id <您的租户ID> 启动应用。

``` shell
$ oc-dt-monitor --tenant-id <您的租户ID> # 启动应用

$ oc-dt-monitor --tenant-id <您的租户ID> start --interval 60 # 将检测周期设置为60秒

$ oc-dt-monitor --tenant-id <您的租户ID> --auto-stop  # 在数据传输用量超出阈值后自动停止所有实例
```

更多选项请参考：

```
oc-dt-monitor 0.1.0
An oracle cloud data transfer usage monitor

USAGE:
    oc-dt-monitor [FLAGS] [OPTIONS] --tenant-id <tenant-id> [SUBCOMMAND]

FLAGS:
    -a, --auto-stop    Stop instance(s) when the data transfer reaches the threshold
        --clean        Use clean output (Only output the used data transfer)
    -d, --debug        Activate debug mode
    -h, --help         Prints help information

OPTIONS:
    -c, --config <config>              The oci config path [env: OCDTM_CONFIG=]  [default: ~/.oci/config]
        --instances <instances>...     Instance ids that need to be stopped, if not specified, all instances will be
                                       stopped by default [env: OCDTM_STOP_INSTANCES=]
    -p, --path <path>                  The oci executable path [env: OCDTM_EXECUTABLE=]  [default: oci]
        --stop-method <stop-method>    Use soft stop to stop instance ( soft` or `hard` ) [env: OCDTM_STOP_METHOD=]
                                       [default: soft]
    -t, --tenant-id <tenant-id>        Oracle Cloud tenancy id [env: OCDTM_TENANT_ID=]
        --threshold <threshold>        The stop threshold of data transfer in GB [env: OCDTM_THRESHOLD=]  [default:
                                       1000]

SUBCOMMANDS:
    help     Prints this message or the help of the given subcommand(s)
    start    Start the monitor
```