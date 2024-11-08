data = readtable('data_2024-11-07_20-33-47.csv'); % 替换为你的文件路径

% 提取数据
%distances = linspace(0, 60, numel(data{63:122, 2})); % 根据数据长度生成n
n=linspace(0, 60, numel(data20241107203347{3:602, 2}));
values = double(data20241107203347{3:602, 2});
distances=double(data20241107203347{3:602, 3});
values1 = double(data20241107203347{3:102, 2}); % 第二列：数据（转换为数值）
n1 = linspace(0, 60, numel(data20241107203347{3:102, 2}));
values2 = double(data20241107203347{103:202, 2});
n2 = linspace(0, 60, numel(data20241107203347{103:202, 2}));

[fitresult, gof] = createFit2(distances, values);