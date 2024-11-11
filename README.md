# ldc1614-test

## 开发

使用以下命令将本地代码同步到树莓派

```shell
rsync -avz --exclude-from=.gitignore --exclude=/workspaces/.git /workspaces <用户名>@<ip>:~
```