﻿use rust_decimal::Decimal;
use crate::props::props::IProps;
use crate::props::props_social::{PropsSocial};
use crate::providers::history_const_social::HistoryConstSocial;
use crate::providers::period_2013::history_const_social_2013::{HistoryConstSocial2013, HistoryConstSocial2013var02};
use crate::providers::props_provider::IPropsProvider;
use crate::service::period::IPeriod;
use crate::service::version_id::VersionId;

pub(crate) struct ProviderSocial2013 {
    version: VersionId
}

#[allow(dead_code)]
impl ProviderSocial2013 {
    pub(crate) fn new() -> ProviderSocial2013 {
        ProviderSocial2013 {
            version: VersionId::get(HistoryConstSocial2013::VERSION_CODE)
        }
    }
    fn max_annuals_basis(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstSocial2013::MAX_ANNUALS_BASIS
    }

    fn factor_employer(&self, _period: &dyn IPeriod) -> Decimal {
        HistoryConstSocial2013::FACTOR_EMPLOYER
    }

    fn factor_employer_higher(&self, _period: &dyn IPeriod) -> Decimal {
        HistoryConstSocial2013::FACTOR_EMPLOYER_HIGHER
    }

    fn factor_employee(&self, _period: &dyn IPeriod) -> Decimal {
        HistoryConstSocial2013::FACTOR_EMPLOYEE
    }

    fn factor_employee_garant(&self, _period: &dyn IPeriod) -> Decimal {
        if _period.is_period_greater_or_equal_than(2013, 2) {
            return HistoryConstSocial2013var02::FACTOR_EMPLOYEE_GARANT;
        }
        HistoryConstSocial2013::FACTOR_EMPLOYEE_GARANT
    }

    fn factor_employee_reduce(&self, _period: &dyn IPeriod) -> Decimal {
        if _period.is_period_greater_or_equal_than(2013, 2) {
            return HistoryConstSocial2013var02::FACTOR_EMPLOYEE_REDUCE;
        }
        HistoryConstSocial2013::FACTOR_EMPLOYEE_REDUCE
    }

    fn margin_income_emp(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstSocial2013::MARGIN_INCOME_EMP
    }

    fn margin_income_agr(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstSocial2013::MARGIN_INCOME_AGR
    }
}

impl IProps for ProviderSocial2013 {
    fn get_version(&self) -> VersionId {
        self.version
    }
}

impl IPropsProvider<PropsSocial> for ProviderSocial2013 {
    fn get_version(&self) -> VersionId {
        self.version
    }
    fn get_props(&self, _period: &dyn IPeriod) -> PropsSocial {
        PropsSocial::new(self.version,
                         self.max_annuals_basis(_period),
                         self.factor_employer(_period),
                         self.factor_employer_higher(_period),
                         self.factor_employee(_period),
                         self.factor_employee_garant(_period),
                         self.factor_employee_reduce(_period),
                         self.margin_income_emp(_period),
                         self.margin_income_agr(_period))
    }
}
