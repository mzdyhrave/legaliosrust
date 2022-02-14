﻿use rust_decimal::Decimal;
use crate::props::props::IProps;
use crate::props::props_taxing::{PropsTaxing};
use crate::providers::history_const_taxing::HistoryConstTaxing;
use crate::providers::period_2012::history_const_taxing_2012::HistoryConstTaxing2012;
use crate::providers::props_provider::IPropsProvider;
use crate::service::period::IPeriod;
use crate::service::version_id::VersionId;

pub(crate) struct ProviderTaxing2012 {
    version: VersionId
}

#[allow(dead_code)]
impl ProviderTaxing2012 {
    pub(crate) fn new() -> ProviderTaxing2012 {
        ProviderTaxing2012 {
            version: VersionId::get(HistoryConstTaxing2012::VERSION_CODE)
        }
    }
    fn allowance_payer(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstTaxing2012::ALLOWANCE_PAYER
    }

    fn allowance_disab1st(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstTaxing2012::ALLOWANCE_DISAB_1ST
    }

    fn allowance_disab2nd(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstTaxing2012::ALLOWANCE_DISAB_2ND
    }

    fn allowance_disab3rd(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstTaxing2012::ALLOWANCE_DISAB_3RD
    }

    fn allowance_study(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstTaxing2012::ALLOWANCE_STUDY
    }

    fn allowance_child1st(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstTaxing2012::ALLOWANCE_CHILD_1ST
    }

    fn allowance_child2nd(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstTaxing2012::ALLOWANCE_CHILD_2ND
    }

    fn allowance_child3rd(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstTaxing2012::ALLOWANCE_CHILD_3RD
    }

    fn factor_advances(&self, _period: &dyn IPeriod) -> Decimal {
        HistoryConstTaxing2012::FACTOR_ADVANCES
    }

    fn factor_withhold(&self, _period: &dyn IPeriod) -> Decimal {
        HistoryConstTaxing2012::FACTOR_WITHHOLD
    }

    fn factor_solidary(&self, _period: &dyn IPeriod) -> Decimal {
        HistoryConstTaxing2012::FACTOR_SOLITARY
    }

    fn min_amount_of_tax_bonus(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstTaxing2012::MIN_AMOUNT_OF_TAXBONUS
    }

    fn max_amount_of_tax_bonus(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstTaxing2012::MAX_AMOUNT_OF_TAXBONUS
    }

    fn margin_income_of_tax_bonus(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstTaxing2012::MARGIN_INCOME_OF_TAXBONUS
    }

    fn margin_income_of_rounding(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstTaxing2012::MARGIN_INCOME_OF_ROUNDING
    }

    fn margin_income_of_withhold(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstTaxing2012::MARGIN_INCOME_OF_WITHHOLD
    }

    fn margin_income_of_solidary(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstTaxing2012::MARGIN_INCOME_OF_SOLITARY
    }

    fn margin_income_of_wth_emp(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstTaxing2012::MARGIN_INCOME_OF_WHT_EMP
    }

    fn margin_income_of_wth_agr(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstTaxing2012::MARGIN_INCOME_OF_WHT_AGR
    }
}

impl IProps for ProviderTaxing2012 {
    fn get_version(&self) -> VersionId {
        self.version
    }
}

impl IPropsProvider<PropsTaxing> for ProviderTaxing2012 {
    fn get_version(&self) -> VersionId {
        self.version
    }
    fn get_props(&self, _period: &dyn IPeriod) -> PropsTaxing {
        PropsTaxing::new(self.version,
                         self.allowance_payer(_period),
                         self.allowance_disab1st(_period),
                         self.allowance_disab2nd(_period),
                         self.allowance_disab3rd(_period),
                         self.allowance_study(_period),
                         self.allowance_child1st(_period),
                         self.allowance_child2nd(_period),
                         self.allowance_child3rd(_period),
                         self.factor_advances(_period),
                         self.factor_withhold(_period),
                         self.factor_solidary(_period),
                         self.min_amount_of_tax_bonus(_period),
                         self.max_amount_of_tax_bonus(_period),
                         self.margin_income_of_tax_bonus(_period),
                         self.margin_income_of_rounding(_period),
                         self.margin_income_of_withhold(_period),
                         self.margin_income_of_solidary(_period),
                         self.margin_income_of_wth_emp(_period),
                         self.margin_income_of_wth_agr(_period))
    }
}
