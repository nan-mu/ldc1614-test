function [fitresult, gof] = createFit(n2, values_closest)
%CREATEFIT(N2,VALUES_CLOSEST)
%  创建一个拟合。
%
%  要进行 '最远点数据' 拟合的数据:
%      X 输入: n2
%      Y 输出: values_closest
%  输出:
%      fitresult: 表示拟合的拟合对象。
%      gof: 带有拟合优度信息的结构体。
%
%  另请参阅 FIT, CFIT, SFIT.

%  由 MATLAB 于 06-Nov-2024 21:11:47 自动生成


%% 拟合: '最远点数据'。
[xData, yData] = prepareCurveData( n2, values_closest );

% 设置 fittype 和选项。
ft = fittype( 'poly1' );
opts = fitoptions( 'Method', 'LinearLeastSquares' );
opts.Robust = 'LAR';

% 对数据进行模型拟合。
[fitresult, gof] = fit( xData, yData, ft, opts );
% 打印拟合系数 p1 和 p2 以及它们的置信区间，保留到小数点后8位


% 为绘图创建一个图窗。
figure( 'Name', '最远点数据' );

% 绘制数据拟合图。
subplot( 2, 1, 1 );
h = plot( fitresult, xData, yData, 'predobs', 0.99 );
legend( h, 'values_closest vs. n2', '最远点数据', '下界(最远点数据)', '上界(最远点数据)', 'Location', 'NorthEast', 'Interpreter', 'none' );
% 为坐标区加标签
xlabel( 'n2', 'Interpreter', 'none' );
ylabel( 'values_closest', 'Interpreter', 'none' );
grid on

% 绘制残差图。
subplot( 2, 1, 2 );
h = plot( fitresult, xData, yData, 'residuals' );
legend( h, '最远点数据 - 残差', 'Zero Line', 'Location', 'NorthEast', 'Interpreter', 'none' );
% 为坐标区加标签
xlabel( 'n2', 'Interpreter', 'none' );
ylabel( 'values_closest', 'Interpreter', 'none' );
grid on


