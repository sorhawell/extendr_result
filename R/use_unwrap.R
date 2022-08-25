#' rust like unwrapping of result. Useful to keep error handling on the R side.
#'
#' @param result a list with two elements named 'ok' and 'err'. One and only one of them must NULL or missing.
#' @param call a context call for what triggered the error, passed to abort(footer=). abort(call=) will annoyingly not display anonymous funcitons.
#' @param ... any other param to rlang::abort
#' 
#' @importFrom rlang abort
#'
#' @return eiter the ok value or no return but throw error by the err string
#' @export
#'
#' @examples unwrap(extendr_result:::rust_hello_result(TRUE))
unwrap = function(result,call=sys.call(1L),...) {
  
  #if not a result
  if(!is.list(result)) {
    abort("internal error: cannot unwrap non result",.internal = TRUE)
  }
  
  #if result is ok
  if(!is.null(result$ok) && is.null(result$err)) {
    return(result$ok)
  }
  
  #if result is error
  if( is.null(result$ok) && !is.null(result$err)) {
    return(abort(
      result$err,
      call = NULL,
      footer = paste(
        "when calling:\n",
        paste(capture.output(print(call)),collapse="\n")
      ),
      ...
    ))
  }
  
  #if not ok XOR error, then roll over
  abort("internal error: result object corrupted",.internal = TRUE)
}


#' example user function
#'
#' @param ok bool TRUE or FALSE, should result be ok (or and error)
#'
#' @return r_result_list
#' @export
#' 
#' @importFrom rlang is_bool
#'
#' @examples stopifnot(r_user_function(is_ok=TRUE)==42L)
r_user_function = function(is_ok) {
  if(!is_bool(is_ok)) stop("ok arg must be either scalar TRUE or FALSE")
  result <- rust_hello_result(is_ok)
  unwrap(result)
}