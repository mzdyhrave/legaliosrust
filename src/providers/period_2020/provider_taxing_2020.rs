﻿use rust_decimal::Decimal;
use crate::props::props::IProps;
use crate::props::props_taxing_2018::PropsTaxing2018;
use crate::providers::history_const_taxing::HistoryConstTaxing;
use crate::providers::period_2020::history_const_taxing_2020::HistoryConstTaxing2020;
use crate::providers::props_provider::IPropsProvider;
use crate::service::period::IPeriod;
use crate::service::version_id::VersionId;

pub(crate) struct ProviderTaxing2020 {
    version: VersionId
}

#[allow(dead_code)]
impl ProviderTaxing2020 {
    pub(crate) fn new() -> ProviderTaxing2020 {
        ProviderTaxing2020 {
            version: VersionId::get(HistoryConstTaxing2020::VERSION_CODE)
        }
    }
    fn allowance_payer(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstTaxing2020::ALLOWANCE_PAYER
    }

    fn allowance_disab1st(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstTaxing2020::ALLOWANCE_DISAB_1ST
    }

    fn allowance_disab2nd(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstTaxing2020::ALLOWANCE_DISAB_2ND
    }

    fn allowance_disab3rd(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstTaxing2020::ALLOWANCE_DISAB_3RD
    }

    fn allowance_study(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstTaxing2020::ALLOWANCE_STUDY
    }

    fn allowance_child1st(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstTaxing2020::ALLOWANCE_CHILD_1ST
    }

    fn allowance_child2nd(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstTaxing2020::ALLOWANCE_CHILD_2ND
    }

    fn allowance_child3rd(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstTaxing2020::ALLOWANCE_CHILD_3RD
    }

    fn factor_advances(&self, _period: &dyn IPeriod) -> Decimal {
        HistoryConstTaxing2020::FACTOR_ADVANCES
    }

    fn factor_withhold(&self, _period: &dyn IPeriod) -> Decimal {
        HistoryConstTaxing2020::FACTOR_WITHHOLD
    }

    fn factor_solidary(&self, _period: &dyn IPeriod) -> Decimal {
        HistoryConstTaxing2020::FACTOR_SOLITARY
    }

    fn factor_taxrate2(&self, _period: &dyn IPeriod) -> Decimal {
        HistoryConstTaxing2020::FACTOR_TAXRATE2
    }

    fn min_amount_of_tax_bonus(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstTaxing2020::MIN_AMOUNT_OF_TAXBONUS
    }

    fn max_amount_of_tax_bonus(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstTaxing2020::MAX_AMOUNT_OF_TAXBONUS
    }

    fn margin_income_of_tax_bonus(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstTaxing2020::MARGIN_INCOME_OF_TAXBONUS
    }

    fn margin_income_of_rounding(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstTaxing2020::MARGIN_INCOME_OF_ROUNDING
    }

    fn margin_income_of_withhold(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstTaxing2020::MARGIN_INCOME_OF_WITHHOLD
    }

    fn margin_income_of_solidary(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstTaxing2020::MARGIN_INCOME_OF_SOLITARY
    }

    fn margin_income_of_taxrate2(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstTaxing2020::MARGIN_INCOME_OF_TAXRATE2
    }

    fn margin_income_of_wth_emp(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstTaxing2020::MARGIN_INCOME_OF_WHT_EMP
    }

    fn margin_income_of_wth_agr(&self, _period: &dyn IPeriod) -> i32 {
        HistoryConstTaxing2020::MARGIN_INCOME_OF_WHT_AGR
    }
}

impl IProps for ProviderTaxing2020 {
    fn get_version(&self) -> VersionId {
        self.version
    }
}

impl IPropsProvider<PropsTaxing2018> for ProviderTaxing2020 {
    fn get_version(&self) -> VersionId {
        self.version
    }
    fn get_props(&self, _period: &dyn IPeriod) -> PropsTaxing2018 {
        PropsTaxing2018::new(self.version,
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
                         self.factor_taxrate2(_period),
                         self.min_amount_of_tax_bonus(_period),
                         self.max_amount_of_tax_bonus(_period),
                         self.margin_income_of_tax_bonus(_period),
                         self.margin_income_of_rounding(_period),
                         self.margin_income_of_withhold(_period),
                         self.margin_income_of_solidary(_period),
                         self.margin_income_of_taxrate2(_period),
                         self.margin_income_of_wth_emp(_period),
                         self.margin_income_of_wth_agr(_period))
    }
}
