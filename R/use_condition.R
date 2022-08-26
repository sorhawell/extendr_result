#' rust like unwrapping of result. Useful to keep error handling on the R side.
#'
#' @param result a list with two elements named 'ok' and 'err'. One and only one of them must NULL or missing.
#' @param call a context call for what triggered the error, passed to abort(footer=). abort(call=) will annoyingly not display anonymous funcitons.
#' @param ... any other param to rlang::abort
#' 
#' @importFrom rlang abort is_error
#'
#' @return eiter the ok value or no return but throw error by the err string
#' @export
#'
#' @examples unwrap(rust_helloresult(TRUE))
check_condition = function(value_or_condition,call=sys.call(1L),...) {
  
  #if not a result
  if(!is_error(value_or_condition)) return(value_or_condition)
  
  return(abort(
    value_or_condition$message,
    call = NULL,
    footer = paste(
      "when calling:\n",
      paste(capture.output(print(call)),collapse="\n")
    ),
    ...
  ))
  
}



#' example user function
#'
#' @param is_ok bool TRUE or FALSE, should result be ok (or and error)
#'
#' @return r_result_list
#' @export
#' 
#' @importFrom rlang is_bool
#'
#' @examples stopifnot(r_user_function(is_ok=TRUE)==42L)
r_user_function_cond = function(is_ok) {
  if(!is_bool(is_ok)) stop("ok arg must be either scalar TRUE or FALSE")
  value_or_condition = rust_hellocondition(is_ok)
  value = check_condition(value_or_condition)
  value
}
