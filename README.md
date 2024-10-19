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