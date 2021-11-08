﻿use crate::providers::history_const_salary::HistoryConstSalary;
use crate::providers::period_2020::history_const_salary_2020::HistoryConstSalary2020;
//
// Created by Ladislav Lisy on 13.06.2021.
//

// WORKING_SHIFT_WEEK      Počet pracovních dnů v týdnu
//
// WORKING_SHIFT_TIME      Počet pracovních hodin denně
//
// MIN_MONTHLY_WAGE        Minimální mzda měsíční
//
// MIN_HOURLY_WAGE         Minimální mzda hodinová (100*Kč)

pub(crate) struct HistoryConstSalary2021 {
}

impl HistoryConstSalary for HistoryConstSalary2021 {
    const VERSION_CODE: i16 = 2021;

    const WORKING_SHIFT_WEEK: i32 = HistoryConstSalary2020::WORKING_SHIFT_WEEK;
    const WORKING_SHIFT_TIME: i32 = HistoryConstSalary2020::WORKING_SHIFT_TIME;
    const MIN_MONTHLY_WAGE: i32   = 15200;
    const MIN_HOURLY_WAGE: i32    = 9050;
}

