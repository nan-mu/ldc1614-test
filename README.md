# ldc1614-test

## 开发

使用以下命令将本地代码同步到树莓派

```shell
rsync -avz --exclude-from=.gitignore --exclude=./.git ./* zxrpi:~/ldc1614-test/
```

## 测试

首先应当根据[task-example.yml](tasks/task-example.yml)编写合适的测试任务。

使用以下命令运行并载入对应的配置文件

```shell
cargo run -- -c tasks/<配置文件名称>.yml
```

更推荐使用screen在ssh断开后测试任务不会中断。

```shell
# 创建会话并运行
screen -dmS <测试会话名称> cargo run -- -c tasks/<配置文件名称>.yml

# 连接已经创建的会话
screen -r <测试会话名称>
```