% Generated by roxygen2: do not edit by hand
% Please edit documentation in R/use_condition.R
\name{r_user_function_cond}
\alias{r_user_function_cond}
\title{example user function}
\usage{
r_user_function_cond(is_ok)
}
\arguments{
\item{is_ok}{bool TRUE or FALSE, should result be ok (or and error)}
}
\value{
r_result_list
}
\description{
example user function
}
\examples{
stopifnot(r_user_function(is_ok=TRUE)==42L)
}
