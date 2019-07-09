#!/usr/bin/env python3
# -------------------------------------------------------------------------------------------------
# <copyright file="performance.pxd" company="Nautech Systems Pty Ltd">
#  Copyright (C) 2015-2019 Nautech Systems Pty Ltd. All rights reserved.
#  The use of this source code is governed by the license as found in the LICENSE.md file.
#  http://www.nautechsystems.io
# </copyright>
# -------------------------------------------------------------------------------------------------

# cython: language_level=3, boundscheck=False, wraparound=False, nonecheck=False

from cpython.datetime cimport datetime

from nautilus_trader.model.c_enums.currency cimport Currency
from nautilus_trader.model.objects cimport Money


cdef class PerformanceAnalyzer:
    """
    Provides a trading portfolio analyzer for generating performance metrics
    and statistics.
    """
    cdef bint _log_returns
    cdef object _returns
    cdef object _positions
    cdef object _transactions
    cdef object _equity_curve
    cdef Money _account_starting_capital
    cdef Money _account_capital
    cdef Currency _account_currency

    cpdef void set_starting_capital(self, Money starting_capital, Currency account_currency)
    cpdef void add_return(self, datetime time, float value)
    cpdef void add_positions(self, datetime time, list positions, Money cash_balance)
    cpdef void add_transaction(self, datetime time, Money account_capital, Money pnl)
    cpdef object get_returns(self)
    cpdef object get_positions(self)
    cpdef object get_transactions(self)
    cpdef object get_equity_curve(self)
    cpdef Money total_pnl(self)
    cpdef float total_pnl_percentage(self)
    cpdef Money max_winner(self)
    cpdef Money max_loser(self)
    cpdef Money min_winner(self)
    cpdef Money min_loser(self)
    cpdef Money avg_winner(self)
    cpdef Money avg_loser(self)
    cpdef float win_rate(self)
    cpdef float expectancy(self)
    cpdef float annual_return(self)
    cpdef float cum_return(self)
    cpdef float max_drawdown_return(self)
    cpdef float annual_volatility(self)
    cpdef float sharpe_ratio(self)
    cpdef float calmar_ratio(self)
    cpdef float sortino_ratio(self)
    cpdef float omega_ratio(self)
    cpdef float stability_of_timeseries(self)
    cpdef float returns_mean(self)
    cpdef float returns_variance(self)
    cpdef float returns_skew(self)
    cpdef float returns_kurtosis(self)
    cpdef float returns_tail_ratio(self)
    cpdef float alpha(self)
    cpdef float beta(self)
    cpdef void create_returns_tear_sheet(self)
    cpdef void create_full_tear_sheet(self)
    cpdef dict get_performance_stats(self)

    cdef list get_performance_stats_formatted(self)
    cdef str _format_stat(self, float value, int decimals=*)
