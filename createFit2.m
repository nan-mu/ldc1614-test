function [fitresult, gof] = createFit2(distances, values)
%CREATEFIT1(DISTANCES,VALUES)
%  创建一个拟合。
%
%  要进行 '输出-距离函数' 拟合的数据:
%      X 输入: distances
%      Y 输出: values
%  输出:
%      fitresult: 表示拟合的拟合对象。
%      gof: 带有拟合优度信息的结构体。
%
%  另请参阅 FIT, CFIT, SFIT.

%  由 MATLAB 于 07-Nov-2024 21:38:28 自动生成


%% 拟合: '输出-距离函数'。
[xData, yData] = prepareCurveData( distances, values );

% 设置 fittype 和选项。
ft = fittype( 'exp2' );
opts = fitoptions( 'Method', 'NonlinearLeastSquares' );
opts.Display = 'Off';
opts.StartPoint = [17395679.2183142 -0.342482510391069 184296185.855623 -0.000368356527483986];

% 对数据进行模型拟合。
[fitresult, gof] = fit( xData, yData, ft, opts );

% 绘制数据拟合图。
figure( 'Name', '输出-距离函数' );
h = plot( fitresult, xData, yData );
legend( h, 'values vs. distances', '输出-距离函数', 'Location', 'NorthEast', 'Interpreter', 'none' );
% 为坐标区加标签
xlabel( 'distances', 'Interpreter', 'none' );
ylabel( 'values', 'Interpreter', 'none' );
grid on


