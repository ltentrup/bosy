//! This module can parse propositional formulas in infix notation.
//!
//! For example `a && (b | !c)`

use crate::{IdentDecl, IdentKind, Identifier, Instance, Sort, Term};
use pest::error::Error;
use pest::iterators::{Pair, Pairs};
use pest::prec_climber::{Assoc, Operator, PrecClimber};
use pest::Parser;

#[derive(Parser)]
#[grammar = "parse/propositional.pest"]
struct PropParser;

lazy_static! {
    // Precedence climber can be used to build the AST, see https://pest-parser.github.io/book/ for more details
    static ref PREC_CLIMBER: PrecClimber<Rule> = {
        use self::Assoc::*;
        use self::Rule::*;

        PrecClimber::new(vec![
            Operator::new(implication, Right) | Operator::new(equivalence, Right),
            Operator::new(exclusion, Left),
            Operator::new(disjunction, Left),
            Operator::new(conjunction, Left),
        ])
    };
}

pub fn parse(instance: &mut Instance, content: &str) -> Result<Term, pest::error::Error<Rule>> {
    let pairs = PropParser::parse(Rule::prop, content)?;
    Ok(build_term(instance, pairs))
}

fn build_term(instance: &mut Instance, pairs: Pairs<Rule>) -> Term {
    PREC_CLIMBER.climb(
        pairs,
        |pair: Pair<Rule>| match pair.as_rule() {
            Rule::literal => {
                let ident = match pair.into_inner().next().unwrap().as_rule() {
                    Rule::lit_true => Identifier::TRUE,
                    Rule::lit_false => Identifier::FALSE,
                    _ => unreachable!(),
                };
                Term::new_ident(&ident)
            }
            Rule::identifier => {
                let name = pair.as_str();
                let mut ident = None;
                for decl in &instance.declarations {
                    if let IdentDecl::Func(n, _, _) = decl.as_ref() {
                        if n == name {
                            ident = Some(Identifier {
                                kind: IdentKind::Custom(decl.clone()),
                            });
                        }
                    }
                }

                //let ident = instance.declare_const(name, Sort::BOOL);
                Term::new_ident(&ident.expect("all variables have to be bound before parsing"))
            }
            Rule::primary_expression => build_term(instance, pair.into_inner()),
            Rule::infix_expression => build_term(instance, pair.into_inner()),
            Rule::prefix_expression => {
                // arbitrary many prefix operators ending with some primary_expression
                let mut pairs = pair.into_inner();
                let mut operators = Vec::new();
                while let Some(pair) = pairs.next() {
                    match pair.as_rule() {
                        Rule::negation => operators.push(Identifier::NOT),
                        Rule::primary_expression => {
                            let mut inner_expr = build_term(instance, pair.into_inner());
                            for op in operators.into_iter().rev() {
                                inner_expr = Term::new_appl(op, vec![inner_expr.into()])
                            }
                            return inner_expr;
                        }
                        _ => unreachable!(),
                    };
                }
                unreachable!();
            }
            _ => unreachable!(),
        },
        |lhs: Term, op: Pair<Rule>, rhs: Term| {
            let op = match op.as_rule() {
                Rule::disjunction => Identifier::OR,
                Rule::conjunction => Identifier::AND,
                Rule::implication => Identifier::IMPL,
                Rule::exclusion => Identifier::XOR,
                Rule::equivalence => Identifier::EQUIV,
                _ => unreachable!(),
            };
            Term::new_appl(op, vec![lhs, rhs])
        },
    )
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::error::Error;

    #[test]
    fn parse_prefix_infix() {
        parses_to! {
            parser: PropParser,
            input:  "!a | b",
            rule:   Rule::prop,
            tokens: [
                infix_expression(0, 6, [
                    prefix_expression(0, 2, [
                        negation(0, 1),
                        primary_expression(1, 2, [
                            identifier(1,2)
                        ])
                    ]),
                    disjunction(3, 4),
                    prefix_expression(5, 6, [
                        primary_expression(5, 6, [
                            identifier(5, 6)
                        ])
                    ])
                ])
            ]
        };
    }

    #[test]
    fn parse_simple() -> Result<(), Box<Error>> {
        let mut instance = Instance::new();

        let a = instance.declare_const("a", Sort::BOOL);
        let b = instance.declare_const("b", Sort::BOOL);

        let a = Term::new_ident(&a);
        let b = Term::new_ident(&b);
        let term_lhs = !a | b;

        let term_rhs = parse(&mut instance, "!a | b")?;

        Ok(assert_eq!(term_lhs, term_rhs))
    }

    #[test]
    fn parse_precedence() -> Result<(), Box<Error>> {
        let mut instance = Instance::new();

        let a = instance.declare_const("a", Sort::BOOL);
        let b = instance.declare_const("b", Sort::BOOL);
        let c = instance.declare_const("c", Sort::BOOL);

        let a = Term::new_ident(&a);
        let b = Term::new_ident(&b);
        let c1 = Term::new_ident(&c);
        let c2 = Term::new_ident(&c);

        let term_lhs = a | b & !c1 | c2;
        let term_rhs = parse(&mut instance, "a | b && !c || c")?;

        println!("{:#?}", term_lhs);
        println!("{:#?}", term_rhs);
        Ok(assert_eq!(term_lhs, term_rhs))
    }

}
