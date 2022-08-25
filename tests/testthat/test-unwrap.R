test_that("lowlevel unwrap-result", {
  ok_result = helloresult:::rust_hello_result(is_ok = TRUE)
  value = unwrap(ok_result)
  expect_identical(value, 42L)
  
  err_result = helloresult:::rust_hello_result(is_ok = FALSE)
  expect_error(unwrap(err_result))
  
})

test_that("highlevel(userlevel) function that internally use unwrap-result", {
  
  #normal user call, where unwrap-result concept is hidden away from user
  expect_identical(
    r_user_function(is_ok = TRUE),
    42L
  )
  
  
  #some high-level call yielding an error
  expect_error(
    lapply(c(TRUE,FALSE),FUN = function(bool) r_user_function(is_ok = bool))
  )
  
  print(my_error$trace)
})
