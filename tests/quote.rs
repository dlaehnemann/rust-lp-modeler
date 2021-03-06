extern crate lp_modeler;
extern crate quote;
use quote::quote;

use lp_modeler::dsl::*;

#[test]
fn test_quotations() {
  use LpExpression::*;
  let a = LpInteger { name : "a" . to_string ( ) , lower_bound : None , upper_bound : None };
  let quoted_a = quote!(#a);
  let quoted_a_str = "LpInteger { name : \"a\" . to_string ( ) , lower_bound : None , upper_bound : None }";
  assert_eq!(quoted_a.to_string(), quoted_a_str);

  let exp : LpExpression = a.clone().into();
  let quoted_exp = quote!(#exp);
  let quoted_exp_str = "LpExpression :: ConsInt ( ".to_owned() + quoted_a_str + " )";
  assert_eq!(quoted_exp.to_string(), quoted_exp_str);

  let full_exp = LpExpression::MulExpr ( Box::new ( LpExpression::SubExpr ( Box::new ( LpExpression::EmptyExpr ) , Box::new ( LpExpression::LitVal ( 1f32 ) ) ) ) , Box::new ( LpExpression::AddExpr ( Box::new ( LpExpression::ConsCont ( LpContinuous { name : "x".to_string() , lower_bound : None , upper_bound : None } ) ) , Box::new ( LpExpression::ConsInt ( LpInteger { name : "y".to_string() , lower_bound : None , upper_bound : None } ) ) ) ) );

  let full_exp_quoted = quote!(#full_exp);
  let full_exp_str = "LpExpression :: MulExpr ( Box :: new ( LpExpression :: SubExpr ( Box :: new ( LpExpression :: EmptyExpr ) , Box :: new ( LpExpression :: LitVal ( 1f32 ) ) ) ) , Box :: new ( LpExpression :: AddExpr ( Box :: new ( LpExpression :: ConsCont ( LpContinuous { name : \"x\" . to_string ( ) , lower_bound : None , upper_bound : None } ) ) , Box :: new ( LpExpression :: ConsInt ( LpInteger { name : \"y\" . to_string ( ) , lower_bound : None , upper_bound : None } ) ) ) ) )";
  assert_eq!(full_exp_quoted.to_string(), full_exp_str);

  // a.equal(&b);
  let a_eq_b = LpConstraint( SubExpr( Box::new ( ConsInt ( LpInteger { name : "a" . to_string ( ) , lower_bound : None , upper_bound : None } ) ) , Box::new ( ConsInt ( LpInteger { name : "b" . to_string ( ) , lower_bound : None , upper_bound : None } ) ) ) , Constraint::Equal , LitVal(0f32));

  let quoted_a_eq_b = quote!(#a_eq_b);
  let a_eq_b_str = "LpConstraint ( LpExpression :: SubExpr ( Box :: new ( LpExpression :: ConsInt ( LpInteger { name : \"a\" . to_string ( ) , lower_bound : None , upper_bound : None } ) ) , Box :: new ( LpExpression :: ConsInt ( LpInteger { name : \"b\" . to_string ( ) , lower_bound : None , upper_bound : None } ) ) ) , Constraint :: Equal , LpExpression :: LitVal ( 0f32 ) )";
  assert_eq!(quoted_a_eq_b.to_string(), a_eq_b_str)
}
