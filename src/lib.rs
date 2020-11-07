/* diosix RV32G/RV64G supervisor-level environment code
 *
 * (c) Chris Williams, 2020.
 *
 * See LICENSE for usage and copying.
 */

/* we're on our own */
#![no_std]

/* provide a framework for unit testing */
#![feature(custom_test_frameworks)]
#![test_runner(crate::run_tests)]
#![reexport_test_harness_main = "svriscvtests"] /* entry point for tests */

mod irq;
mod sbi;
mod stdio;

#[cfg(test)]
fn run_tests(unit_tests: &[&dyn Fn()])
{
    /* run each test one by one */
    for test in unit_tests
    {
        test();
    }

    /* exit cleanly once tests are complete */
    // platform::test::end(Ok(0));
}

#[test_case]
fn test_assertion()
{
    assert_eq!(42, 42);
}
