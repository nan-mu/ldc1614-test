# ldc1614-test

## 开发

使用以下命令将本地代码同步到树莓派

```shell
rsync -avz --exclude-from=.gitignore --exclude=/workspaces/.git /workspaces/* <用户名>@<ip>:~/ldc1614-test/
```

假如多人在单个服务器上协助，为了避免容器冲突，开启容器前运行一下代码：

```shell
echo "USERNAME=$(whoami)" > .env
```