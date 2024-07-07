from AlgorithmImports import *

class EnhancedMovingAverageCrossover(QCAlgorithm):
    def Initialize(self):
        self.SetStartDate(2020, 1, 1)
        self.SetEndDate(2021, 1, 1)
        self.SetCash(100000)  # Increased initial cash for diversification and hedging
        
        self.assets = ["SPY", "QQQ", "IWM"]  # Diversified assets
        self.hedge = self.AddEquity("SH", Resolution.Daily).Symbol  # Inverse ETF for hedging
        self.sma_fast = {}
        self.sma_slow = {}
        
        for asset in self.assets:
            symbol = self.AddEquity(asset, Resolution.Daily).Symbol
            self.sma_fast[symbol] = self.SMA(symbol, 50, Resolution.Daily)
            self.sma_slow[symbol] = self.SMA(symbol, 200, Resolution.Daily)
            self.SetWarmUp(200)
            
        self.risk_free_rate = 0.01  # Example risk-free rate
        self.risk_tolerance = 0.02  # 2% of portfolio value at risk per trade
        self.stop_loss_pct = 0.05
        self.take_profit_pct = 0.1

    def OnData(self, data):
        if self.IsWarmingUp:
            return

        portfolio_value = self.Portfolio.TotalPortfolioValue
        for symbol in self.assets:
            if not data.ContainsKey(symbol):
                continue

            fast_ma = self.sma_fast[symbol]
            slow_ma = self.sma_slow[symbol]

            if not fast_ma.IsReady or not slow_ma.IsReady:
                continue

            holdings = self.Portfolio[symbol].Quantity
            if fast_ma.Current.Value > slow_ma.Current.Value:
                if holdings <= 0:
                    self.SetHoldings(symbol, self.CalculatePositionSize(portfolio_value, symbol))
            elif fast_ma.Current.Value < slow_ma.Current.Value:
                if holdings > 0:
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

        # Calculate the dollar amount at risk for the trade
        risk_per_trade = self.risk_tolerance * portfolio_value
        # Position size in shares
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