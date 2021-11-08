#[cfg(test)]

mod service_tests {
    #[cfg(test_report)]
    #[macro_use(crate::report_test_examples_int_social)]
    crate::report_test_examples_int_social!(
        test_examples_report,
        social_07,
        "03_Social_07_MarginIncomeEmp",
        |x: &dyn IPropsSocial| { x.margin_income_emp() },
        2011,
        2023
    );

    #[macro_use(crate::test_examples_int_social)]
    crate::test_examples_int_social! (test_examples_2011, |x: &dyn IPropsSocial| {x.margin_income_emp()},
                margin_income_emp_test_period_2011_01: 2011, 1, 2011, 1, 2000,
                margin_income_emp_test_period_2011_02: 2011, 2, 2011, 2, 2000,
                margin_income_emp_test_period_2011_03: 2011, 3, 2011, 3, 2000,
                margin_income_emp_test_period_2011_04: 2011, 4, 2011, 4, 2000,
                margin_income_emp_test_period_2011_05: 2011, 5, 2011, 5, 2000,
                margin_income_emp_test_period_2011_06: 2011, 6, 2011, 6, 2000,
                margin_income_emp_test_period_2011_07: 2011, 7, 2011, 7, 2000,
                margin_income_emp_test_period_2011_08: 2011, 8, 2011, 8, 2000,
                margin_income_emp_test_period_2011_09: 2011, 9, 2011, 9, 2000,
                margin_income_emp_test_period_2011_10: 2011, 10, 2011, 10, 2000,
                margin_income_emp_test_period_2011_11: 2011, 11, 2011, 11, 2000,
                margin_income_emp_test_period_2011_12: 2011, 12, 2011, 12, 2000,
    );
    crate::test_examples_int_social! (test_examples_2012, |x: &dyn IPropsSocial| {x.margin_income_emp()},
                margin_income_emp_test_period_2012_01: 2012, 1, 2012, 1, 2500,
                margin_income_emp_test_period_2012_02: 2012, 2, 2012, 2, 2500,
                margin_income_emp_test_period_2012_03: 2012, 3, 2012, 3, 2500,
                margin_income_emp_test_period_2012_04: 2012, 4, 2012, 4, 2500,
                margin_income_emp_test_period_2012_05: 2012, 5, 2012, 5, 2500,
                margin_income_emp_test_period_2012_06: 2012, 6, 2012, 6, 2500,
                margin_income_emp_test_period_2012_07: 2012, 7, 2012, 7, 2500,
                margin_income_emp_test_period_2012_08: 2012, 8, 2012, 8, 2500,
                margin_income_emp_test_period_2012_09: 2012, 9, 2012, 9, 2500,
                margin_income_emp_test_period_2012_10: 2012, 10, 2012, 10, 2500,
                margin_income_emp_test_period_2012_11: 2012, 11, 2012, 11, 2500,
                margin_income_emp_test_period_2012_12: 2012, 12, 2012, 12, 2500,
    );
    crate::test_examples_int_social! (test_examples_2013, |x: &dyn IPropsSocial| {x.margin_income_emp()},
                margin_income_emp_test_period_2013_01: 2013, 1, 2013, 1, 2500,
                margin_income_emp_test_period_2013_02: 2013, 2, 2013, 2, 2500,
                margin_income_emp_test_period_2013_03: 2013, 3, 2013, 3, 2500,
                margin_income_emp_test_period_2013_04: 2013, 4, 2013, 4, 2500,
                margin_income_emp_test_period_2013_05: 2013, 5, 2013, 5, 2500,
                margin_income_emp_test_period_2013_06: 2013, 6, 2013, 6, 2500,
                margin_income_emp_test_period_2013_07: 2013, 7, 2013, 7, 2500,
                margin_income_emp_test_period_2013_08: 2013, 8, 2013, 8, 2500,
                margin_income_emp_test_period_2013_09: 2013, 9, 2013, 9, 2500,
                margin_income_emp_test_period_2013_10: 2013, 10, 2013, 10, 2500,
                margin_income_emp_test_period_2013_11: 2013, 11, 2013, 11, 2500,
                margin_income_emp_test_period_2013_12: 2013, 12, 2013, 12, 2500,
    );
    crate::test_examples_int_social! (test_examples_2014, |x: &dyn IPropsSocial| {x.margin_income_emp()},
                margin_income_emp_test_period_2014_01: 2014, 1, 2014, 1, 2500,
                margin_income_emp_test_period_2014_02: 2014, 2, 2014, 2, 2500,
                margin_income_emp_test_period_2014_03: 2014, 3, 2014, 3, 2500,
                margin_income_emp_test_period_2014_04: 2014, 4, 2014, 4, 2500,
                margin_income_emp_test_period_2014_05: 2014, 5, 2014, 5, 2500,
                margin_income_emp_test_period_2014_06: 2014, 6, 2014, 6, 2500,
                margin_income_emp_test_period_2014_07: 2014, 7, 2014, 7, 2500,
                margin_income_emp_test_period_2014_08: 2014, 8, 2014, 8, 2500,
                margin_income_emp_test_period_2014_09: 2014, 9, 2014, 9, 2500,
                margin_income_emp_test_period_2014_10: 2014, 10, 2014, 10, 2500,
                margin_income_emp_test_period_2014_11: 2014, 11, 2014, 11, 2500,
                margin_income_emp_test_period_2014_12: 2014, 12, 2014, 12, 2500,
    );
    crate::test_examples_int_social! (test_examples_2015, |x: &dyn IPropsSocial| {x.margin_income_emp()},
                margin_income_emp_test_period_2015_01: 2015, 1, 2015, 1, 2500,
                margin_income_emp_test_period_2015_02: 2015, 2, 2015, 2, 2500,
                margin_income_emp_test_period_2015_03: 2015, 3, 2015, 3, 2500,
                margin_income_emp_test_period_2015_04: 2015, 4, 2015, 4, 2500,
                margin_income_emp_test_period_2015_05: 2015, 5, 2015, 5, 2500,
                margin_income_emp_test_period_2015_06: 2015, 6, 2015, 6, 2500,
                margin_income_emp_test_period_2015_07: 2015, 7, 2015, 7, 2500,
                margin_income_emp_test_period_2015_08: 2015, 8, 2015, 8, 2500,
                margin_income_emp_test_period_2015_09: 2015, 9, 2015, 9, 2500,
                margin_income_emp_test_period_2015_10: 2015, 10, 2015, 10, 2500,
                margin_income_emp_test_period_2015_11: 2015, 11, 2015, 11, 2500,
                margin_income_emp_test_period_2015_12: 2015, 12, 2015, 12, 2500,
    );
    crate::test_examples_int_social! (test_examples_2016, |x: &dyn IPropsSocial| {x.margin_income_emp()},
                margin_income_emp_test_period_2016_01: 2016, 1, 2016, 1, 2500,
                margin_income_emp_test_period_2016_02: 2016, 2, 2016, 2, 2500,
                margin_income_emp_test_period_2016_03: 2016, 3, 2016, 3, 2500,
                margin_income_emp_test_period_2016_04: 2016, 4, 2016, 4, 2500,
                margin_income_emp_test_period_2016_05: 2016, 5, 2016, 5, 2500,
                margin_income_emp_test_period_2016_06: 2016, 6, 2016, 6, 2500,
                margin_income_emp_test_period_2016_07: 2016, 7, 2016, 7, 2500,
                margin_income_emp_test_period_2016_08: 2016, 8, 2016, 8, 2500,
                margin_income_emp_test_period_2016_09: 2016, 9, 2016, 9, 2500,
                margin_income_emp_test_period_2016_10: 2016, 10, 2016, 10, 2500,
                margin_income_emp_test_period_2016_11: 2016, 11, 2016, 11, 2500,
                margin_income_emp_test_period_2016_12: 2016, 12, 2016, 12, 2500,
    );
    crate::test_examples_int_social! (test_examples_2017, |x: &dyn IPropsSocial| {x.margin_income_emp()},
                margin_income_emp_test_period_2017_01: 2017, 1, 2017, 1, 2500,
                margin_income_emp_test_period_2017_02: 2017, 2, 2017, 2, 2500,
                margin_income_emp_test_period_2017_03: 2017, 3, 2017, 3, 2500,
                margin_income_emp_test_period_2017_04: 2017, 4, 2017, 4, 2500,
                margin_income_emp_test_period_2017_05: 2017, 5, 2017, 5, 2500,
                margin_income_emp_test_period_2017_06: 2017, 6, 2017, 6, 2500,
                margin_income_emp_test_period_2017_07: 2017, 7, 2017, 7, 2500,
                margin_income_emp_test_period_2017_08: 2017, 8, 2017, 8, 2500,
                margin_income_emp_test_period_2017_09: 2017, 9, 2017, 9, 2500,
                margin_income_emp_test_period_2017_10: 2017, 10, 2017, 10, 2500,
                margin_income_emp_test_period_2017_11: 2017, 11, 2017, 11, 2500,
                margin_income_emp_test_period_2017_12: 2017, 12, 2017, 12, 2500,
    );
    crate::test_examples_int_social! (test_examples_2018, |x: &dyn IPropsSocial| {x.margin_income_emp()},
                margin_income_emp_test_period_2018_01: 2018, 1, 2018, 1, 2500,
                margin_income_emp_test_period_2018_02: 2018, 2, 2018, 2, 2500,
                margin_income_emp_test_period_2018_03: 2018, 3, 2018, 3, 2500,
                margin_income_emp_test_period_2018_04: 2018, 4, 2018, 4, 2500,
                margin_income_emp_test_period_2018_05: 2018, 5, 2018, 5, 2500,
                margin_income_emp_test_period_2018_06: 2018, 6, 2018, 6, 2500,
                margin_income_emp_test_period_2018_07: 2018, 7, 2018, 7, 2500,
                margin_income_emp_test_period_2018_08: 2018, 8, 2018, 8, 2500,
                margin_income_emp_test_period_2018_09: 2018, 9, 2018, 9, 2500,
                margin_income_emp_test_period_2018_10: 2018, 10, 2018, 10, 2500,
                margin_income_emp_test_period_2018_11: 2018, 11, 2018, 11, 2500,
                margin_income_emp_test_period_2018_12: 2018, 12, 2018, 12, 2500,
    );
    crate::test_examples_int_social! (test_examples_2019, |x: &dyn IPropsSocial| {x.margin_income_emp()},
                margin_income_emp_test_period_2019_01: 2019, 1, 2019, 1, 3000,
                margin_income_emp_test_period_2019_02: 2019, 2, 2019, 2, 3000,
                margin_income_emp_test_period_2019_03: 2019, 3, 2019, 3, 3000,
                margin_income_emp_test_period_2019_04: 2019, 4, 2019, 4, 3000,
                margin_income_emp_test_period_2019_05: 2019, 5, 2019, 5, 3000,
                margin_income_emp_test_period_2019_06: 2019, 6, 2019, 6, 3000,
                margin_income_emp_test_period_2019_07: 2019, 7, 2019, 7, 3000,
                margin_income_emp_test_period_2019_08: 2019, 8, 2019, 8, 3000,
                margin_income_emp_test_period_2019_09: 2019, 9, 2019, 9, 3000,
                margin_income_emp_test_period_2019_10: 2019, 10, 2019, 10, 3000,
                margin_income_emp_test_period_2019_11: 2019, 11, 2019, 11, 3000,
                margin_income_emp_test_period_2019_12: 2019, 12, 2019, 12, 3000,
    );
    crate::test_examples_int_social! (test_examples_2020, |x: &dyn IPropsSocial| {x.margin_income_emp()},
                margin_income_emp_test_period_2020_01: 2020, 1, 2020, 1, 3000,
                margin_income_emp_test_period_2020_02: 2020, 2, 2020, 2, 3000,
                margin_income_emp_test_period_2020_03: 2020, 3, 2020, 3, 3000,
                margin_income_emp_test_period_2020_04: 2020, 4, 2020, 4, 3000,
                margin_income_emp_test_period_2020_05: 2020, 5, 2020, 5, 3000,
                margin_income_emp_test_period_2020_06: 2020, 6, 2020, 6, 3000,
                margin_income_emp_test_period_2020_07: 2020, 7, 2020, 7, 3000,
                margin_income_emp_test_period_2020_08: 2020, 8, 2020, 8, 3000,
                margin_income_emp_test_period_2020_09: 2020, 9, 2020, 9, 3000,
                margin_income_emp_test_period_2020_10: 2020, 10, 2020, 10, 3000,
                margin_income_emp_test_period_2020_11: 2020, 11, 2020, 11, 3000,
                margin_income_emp_test_period_2020_12: 2020, 12, 2020, 12, 3000,
    );
    crate::test_examples_int_social! (test_examples_2021, |x: &dyn IPropsSocial| {x.margin_income_emp()},
                margin_income_emp_test_period_2021_01: 2021, 1, 2021, 1, 3500,
                margin_income_emp_test_period_2021_02: 2021, 2, 2021, 2, 3500,
                margin_income_emp_test_period_2021_03: 2021, 3, 2021, 3, 3500,
                margin_income_emp_test_period_2021_04: 2021, 4, 2021, 4, 3500,
                margin_income_emp_test_period_2021_05: 2021, 5, 2021, 5, 3500,
                margin_income_emp_test_period_2021_06: 2021, 6, 2021, 6, 3500,
                margin_income_emp_test_period_2021_07: 2021, 7, 2021, 7, 3500,
                margin_income_emp_test_period_2021_08: 2021, 8, 2021, 8, 3500,
                margin_income_emp_test_period_2021_09: 2021, 9, 2021, 9, 3500,
                margin_income_emp_test_period_2021_10: 2021, 10, 2021, 10, 3500,
                margin_income_emp_test_period_2021_11: 2021, 11, 2021, 11, 3500,
                margin_income_emp_test_period_2021_12: 2021, 12, 2021, 12, 3500,
    );
    crate::test_examples_int_social! (test_examples_2022, |x: &dyn IPropsSocial| {x.margin_income_emp()},
                margin_income_emp_test_period_2022_01: 2022, 1, 2022, 1, 3500,
                margin_income_emp_test_period_2022_02: 2022, 2, 2022, 2, 3500,
                margin_income_emp_test_period_2022_03: 2022, 3, 2022, 3, 3500,
                margin_income_emp_test_period_2022_04: 2022, 4, 2022, 4, 3500,
                margin_income_emp_test_period_2022_05: 2022, 5, 2022, 5, 3500,
                margin_income_emp_test_period_2022_06: 2022, 6, 2022, 6, 3500,
                margin_income_emp_test_period_2022_07: 2022, 7, 2022, 7, 3500,
                margin_income_emp_test_period_2022_08: 2022, 8, 2022, 8, 3500,
                margin_income_emp_test_period_2022_09: 2022, 9, 2022, 9, 3500,
                margin_income_emp_test_period_2022_10: 2022, 10, 2022, 10, 3500,
                margin_income_emp_test_period_2022_11: 2022, 11, 2022, 11, 3500,
                margin_income_emp_test_period_2022_12: 2022, 12, 2022, 12, 3500,
    );
}
