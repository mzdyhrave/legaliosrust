﻿use rust_decimal::Decimal;
use crate::props::props_health::PropsHealth;
use crate::providers::history_const_health::HistoryConstHealth;
use crate::providers::period_2016::history_const_health_2016::HistoryConstHealth2016;
use crate::providers::props_provider::IPropsProvider;
use crate::service::period::IPeriod;
use crate::service::version_id::VersionId;

pub(crate) struct ProviderHealth2016 {
    version: VersionId
}

#[allow(dead_code)]
impl ProviderHealth2016 {
    pub(crate) fn new() -> ProviderHealth2016 {
        ProviderHealth2016 {
            version: VersionId::get(HistoryConstHealth2016::VERSION_CODE)
        }
    }
    fn min_monthly_basis(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstHealth2016::MIN_MONTHLY_BASIS
    }

    fn max_annuals_basis(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstHealth2016::MAX_ANNUALS_BASIS
    }

    fn lim_monthly_state(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstHealth2016::LIM_MONTHLY_STATE
    }

    fn lim_monthly_dis50(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstHealth2016::LIM_MONTHLY_DIS50
    }

    fn factor_compound(&self, _period: &dyn IPeriod) -> Decimal {
        HistoryConstHealth2016::FACTOR_COMPOUND
    }

    fn factor_employee(&self, _period: &dyn IPeriod) -> Decimal {
        HistoryConstHealth2016::FACTOR_EMPLOYEE
    }

    fn margin_income_emp(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstHealth2016::MARGIN_INCOME_EMP
    }

    fn margin_income_agr(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstHealth2016::MARGIN_INCOME_AGR
    }
}

impl IPropsProvider<PropsHealth> for ProviderHealth2016 {
    fn get_version(&self) -> VersionId {
        self.version
    }
    fn get_props(&self, _period: &dyn IPeriod) -> PropsHealth {
        PropsHealth::new(self.version,
                         self.min_monthly_basis(_period),
                         self.max_annuals_basis(_period),
                         self.lim_monthly_state(_period),
                         self.lim_monthly_dis50(_period),
                         self.factor_compound(_period),
                         self.factor_employee(_period),
                         self.margin_income_emp(_period),
                         self.margin_income_agr(_period))
    }
}
