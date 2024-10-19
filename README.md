# ldc1614-test

## 开发

### 提交

仓库只接受已经签名且符合提交规范的提交。提交规范见[.gitmessage.txt](./.gitmessage.txt)。但理论上只需要使用`Commitizen`提交就没有问题，使用方法为：

```shell
# 安装
## 建议使用node 22
## 在项目根目录下进行
npm install -g commitizen
commitizen init cz-conventional-changelog --save-dev --save-exact

# 当你要进行提交时
git add *
git ca
```

目前接受以下作用域（scope）：

* main: 主要功能，也就是和ldc1614的交互
* dev-tool: 开发工具相关

对文档相关的修改不需要填`scope`，`type`选择`docs`即可。

### 签名

> 首先的首先你肯定需要有密钥对
> 
> ```shell
> ssh-keygen -t rsa -b 4096 -C "your_email@example.com"
> ```

首先需要将签名使用的公钥提交到`github`，[请参照](https://docs.github.com/zh/authentication/managing-commit-signature-verification/about-commit-signature-verification#ssh-commit-signature-verification)。强烈建议使用`ssh`的密钥，4096的rsa。

然后将公钥添加到你的`git`全局设置中，[请参照](https://docs.github.com/zh/authentication/managing-commit-signature-verification/telling-git-about-your-signing-key#telling-git-about-your-ssh-key)。

最后建议开启自动签名所有提交：

```shell
git config --global commit.gpgSign true
```