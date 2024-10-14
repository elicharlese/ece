from AlgorithmImports import *

class EnhancedMeanReversion(QCAlgorithm):
    def Initialize(self):
        self.SetStartDate(2020, 1, 1)
        self.SetEndDate(2021, 1, 1)
        self.SetCash(100000)
        
        self.assets = ["SPY", "QQQ", "IWM"]
        self.hedge = self.AddEquity("SH", Resolution.Daily).Symbol
        self.lookback = 20

        self.sma = {}
        for asset in self.assets:
            symbol = self.AddEquity(asset, Resolution.Daily).Symbol
            self.sma[symbol] = self.SMA(symbol, self.lookback, Resolution.Daily)
            self.SetWarmUp(self.lookback)
        
        self.risk_free_rate = 0.01
        self.risk_tolerance = 0.02
        self.stop_loss_pct = 0.05
        self.take_profit_pct = 0.1

    def OnData(self, data):
        if self.IsWarmingUp:
            return
        
        portfolio_value = self.Portfolio.TotalPortfolioValue
        for symbol in self.assets:
            if not data.ContainsKey(symbol):
                continue

            history = self.History(symbol, self.lookback, Resolution.Daily)
            if len(history) < self.lookback:
                continue

            mean_price = history['close'].mean()
            current_price = self.Securities[symbol].Price

            holdings = self.Portfolio[symbol].Quantity
            if current_price < mean_price * 0.95 and not self.Portfolio[symbol].Invested:
                self.SetHoldings(symbol, self.CalculatePositionSize(portfolio_value, symbol))
            elif current_price > mean_price * 1.05 and self.Portfolio[symbol].Invested:
                self.Liquidate(symbol)
            
            # Handle stop-loss and take-profit
            self.ManageRisk(symbol)

        # Hedging logic
        total_long_exposure = sum([self.Portfolio[s].HoldingsValue for s in self.assets if self.Portfolio[s].IsLong])
        hedge_ratio = total_long_exposure / self.Portfolio[self.hedge].Price if self.Portfolio[self.hedge].Price != 0 else 0
        if hedge_ratio > 0:
            self.SetHoldings(self.hedge, -hedge_ratio)

    def CalculatePositionSize(self, portfolio_value, symbol):
        atr = self.ATR(symbol, 14, Resolution.Daily)
        if not atr.IsReady:
            return 0

        risk_per_trade = self.risk_tolerance * portfolio_value
        position_size = risk_per_trade / (atr.Current.Value * self.Securities[symbol].Price)
        return position_size

    def ManageRisk(self, symbol):
        security = self.Portfolio[symbol]
        if not security.Invested:
            return
        
        stop_loss_price = security.LastTradeProfit if security.LastTradeProfit > 0 else security.AveragePrice * (1 - self.stop_loss_pct)
        take_profit_price = security.AveragePrice * (1 + self.take_profit_pct)
        
        if security.Price <= stop_loss_price or security.Price >= take_profit_price:
            self.Liquidate(symbol)