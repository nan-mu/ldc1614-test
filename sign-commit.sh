#!/bin/bash

# 检查参数是否提供
if [ -z "$1" ]; then
    echo "用法: $0 <N>"
    echo "重写最近 N 个提交的签名。"
    exit 1
fi

# 获取用户指定的提交数量
N=$1

# 使用交互式 rebase 修改最近 N 个提交
git rebase -i HEAD~$N

# 确保上一步成功
if [ $? -ne 0 ]; then
    echo "rebase 失败，请解决冲突并重试。"
    exit 1
fi

# 进入 rebase 环境并对每个提交进行签署
for ((i = 0; i < N; i++)); do
    # 对当前提交添加签名
    git commit --amend --no-edit --signoff

    # 继续到下一个提交
    git rebase --continue
done

echo "最近的 $N 个提交已成功重新签名。"

