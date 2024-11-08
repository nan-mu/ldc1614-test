% 1. 读取数据
data = readtable('data_2024-11-06_19-29-22.csv');  % 替换为你的文件路径

% 2. 数据预处理（标准化数据）
% 提取数据
values = double(data{63:122, 2}); % 第二列：数据（转换为数值）

% 使用行号作为时间戳（确保n与values的长度一致）
n = (1:numel(values))';  % 直接使用行号作为时间戳

% 标准化数据
n = (n - mean(n)) / std(n);  % 标准化时间戳
values = (values - mean(values)) / std(values);  % 标准化数据




% 3. 创建神经网络
hiddenLayerSize = 10;  % 隐藏层大小
net = fitnet(hiddenLayerSize);  % 创建一个具有10个节点的前馈神经网络

% 4. 设置神经网络训练参数
net.trainFcn = 'trainlm';  % 使用Levenberg-Marquardt算法训练网络
net.divideParam.trainRatio = 70/100;  % 训练集占70%
net.divideParam.valRatio = 15/100;    % 验证集占15%
net.divideParam.testRatio = 15/100;   % 测试集占15%

% 5. 训练神经网络
[net, tr] = train(net, n', values');  % 注意：时间戳和数据要转置为行向量

% 6. 使用训练好的网络进行拟合
predicted_values = net(n');  % 对时间戳进行预测

% 7. 绘制结果
figure;
plot(n, values, 'o', 'DisplayName', '原始数据');  % 原始数据点
hold on;
plot(n, predicted_values, 'r-', 'DisplayName', '神经网络拟合');  % 拟合曲线
legend('show');
xlabel('时间戳');
ylabel('数据');
title('神经网络拟合');
