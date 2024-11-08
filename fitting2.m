% 读取CSV文件（忽略第一行表头）
data = readtable('data_2024-11-07_20-33-47.csv'); % 替换为你的文件路径

% 提取数据
%distances = linspace(0, 60, numel(data{63:122, 2})); % 根据数据长度生成n
distances=double(data{3:602, 3})
values = double(data{3:602, 2}); % 第二列：数据（转换为数值）
values_closest =  double(data{123:180, 2});
n2 = linspace(0, 60, numel(data{123:180, 2}));
% 选择拟合类型（例如线性拟合，或者你可以选择其他类型的拟合）
ft = fittype('poly1'); % 线性拟合（多项式1次）

% 使用拟合函数拟合数据
fitResult = fit(n', values, ft); % 确保n和values是列向量

% 绘制数据和拟合曲线
figure;
plot(n, values, 'o', 'DisplayName', '原始数据'); % 原始数据点
hold on;

% 使用 feval 从拟合对象中获取拟合曲线数据并绘制
y_fit = feval(fitResult, n'); % 获取拟合曲线上的y值
plot(n, y_fit, 'r-', 'DisplayName', '拟合曲线'); % 拟合曲线

legend('show');
xlabel('时间戳（秒）');
ylabel('数据');
title('数据拟合');
