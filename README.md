### Show case extendr unwrap-result and check-condition on R side.

extendr_api will panic! if returning Result::Err<E>. Which is memory safe and fast.


Sometimes more control of the error handling is desirable and can be traded to for tiny bit of speed.

the generic rust function r_result_list() and r_result_condition() almost any R-exportable objects and wrap them in a list(ok=T, err=T) or if error return simpleError.

Then on R side any result / error_condition can be resolved exactly as developer wishes.

See `test-unwrap_and_condition.R` for usage
See `lib.rs`, `use_unwrap.R` & `use_condition.R` for implementation