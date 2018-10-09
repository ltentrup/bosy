use super::{BinOp, HyperLTL, QuantKind, UnOp};
use pest::error::Error;
use pest::iterators::{Pair, Pairs};
use pest::prec_climber::{Assoc, Operator, PrecClimber};
use pest::Parser;

#[derive(Parser)]
#[grammar = "ltl.pest"]
struct LTLParser;

lazy_static! {
    // precedence taken from https://spot.lrde.epita.fr/tl.pdf
    // Precedence climber can be used to build the AST, see https://pest-parser.github.io/book/ for more details
    static ref PREC_CLIMBER: PrecClimber<Rule> = {
        use self::Assoc::*;
        use self::Rule::*;

        PrecClimber::new(vec![
            Operator::new(implication, Right) | Operator::new(equivalence, Right),
            Operator::new(exclusion, Left),
            Operator::new(disjunction, Left),
            Operator::new(conjunction, Left),
            Operator::new(until, Right) | Operator::new(weak_until, Right) | Operator::new(release, Right),
        ])
    };
}

pub(crate) fn parse(content: &str) -> Result<HyperLTL, Error<Rule>> {
    let pairs = LTLParser::parse(Rule::ltl, content)?;
    Ok(build_ast(pairs))
}

fn build_ast(pairs: Pairs<Rule>) -> HyperLTL {
    println!("{:?}", pairs);
    PREC_CLIMBER.climb(
        pairs,
        |pair: Pair<Rule>| match pair.as_rule() {
            Rule::identifier => HyperLTL::Proposition(String::from(pair.as_str()), None),
            Rule::primary_expression => build_ast(pair.into_inner()),
            Rule::prefix_expression => {
                // arbitrary many prefix operators ending with some primary_expression
                let mut pairs = pair.into_inner();
                let mut operators = Vec::new();
                while let Some(pair) = pairs.next() {
                    match pair.as_rule() {
                        Rule::negation => operators.push(UnOp::Negation),
                        Rule::next => operators.push(UnOp::Next),
                        Rule::finally => operators.push(UnOp::Finally),
                        Rule::globally => operators.push(UnOp::Globally),
                        Rule::primary_expression => {
                            let mut inner_expr = build_ast(pair.into_inner());
                            for op in operators.into_iter().rev() {
                                inner_expr = HyperLTL::Unary(op, Box::new(inner_expr));
                            }
                            return inner_expr;
                        }
                        _ => unreachable!(),
                    };
                }
                unreachable!();
            }
            Rule::quantified_expression => {
                let mut pairs = pair.into_inner();
                let quant = match pairs.next().expect("mismatch in grammar and AST").as_rule() {
                    Rule::exists => QuantKind::Exists,
                    Rule::forall => QuantKind::Forall,
                    _ => unreachable!(),
                };
                let mut parameters = Vec::new();
                let expr = loop {
                    let pair = pairs.next().expect("mismatch between grammar and AST");
                    match pair.as_rule() {
                        Rule::identifier => parameters.push(pair.as_str().into()),
                        Rule::infix_expression => break build_ast(pair.into_inner()),
                        _ => unreachable!(),
                    }
                };
                HyperLTL::Quant(quant, parameters, expr.into())
            }
            Rule::indexed_identifier => {
                let mut pairs = pair.into_inner();
                let name = pairs.next().expect("mismatch in grammar and AST").as_str();
                let index = pairs.next().expect("mismatch in grammar and AST").as_str();
                HyperLTL::Proposition(name.into(), Some(index.into()))
            }
            Rule::infix_expression => build_ast(pair.into_inner()),
            _ => unreachable!(),
        },
        |lhs: HyperLTL, op: Pair<Rule>, rhs: HyperLTL| match op.as_rule() {
            Rule::disjunction => HyperLTL::Binary(BinOp::Disjunction, Box::new(lhs), Box::new(rhs)),
            Rule::conjunction => HyperLTL::Binary(BinOp::Conjunction, Box::new(lhs), Box::new(rhs)),
            Rule::implication => HyperLTL::Binary(BinOp::Implication, Box::new(lhs), Box::new(rhs)),
            Rule::exclusion => HyperLTL::Binary(BinOp::Exclusion, Box::new(lhs), Box::new(rhs)),
            Rule::equivalence => HyperLTL::Binary(BinOp::Equivalence, Box::new(lhs), Box::new(rhs)),
            Rule::until => HyperLTL::Binary(BinOp::Until, Box::new(lhs), Box::new(rhs)),
            Rule::weak_until => HyperLTL::Binary(BinOp::WeakUntil, Box::new(lhs), Box::new(rhs)),
            Rule::release => HyperLTL::Binary(BinOp::Release, Box::new(lhs), Box::new(rhs)),
            _ => unreachable!(),
        },
    )
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_prefix_infix() {
        parses_to! {
            parser: LTLParser,
            input:  "!a | b",
            rule:   Rule::ltl,
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
    fn parse_ast_prefix_infix() {
        let ast = parse("!a | b").unwrap_or_else(|e| panic!("{}", e));
        assert_eq!(
            ast,
            HyperLTL::Binary(
                BinOp::Disjunction,
                Box::new(HyperLTL::Unary(
                    UnOp::Negation,
                    Box::new(HyperLTL::Proposition("a".into(), None))
                )),
                Box::new(HyperLTL::Proposition("b".into(), None))
            )
        );
    }

    #[test]
    fn parse_multiple_infix() {
        parses_to! {
            parser: LTLParser,
            input:  "!Ga",
            rule:   Rule::ltl,
            tokens: [
                infix_expression(0, 3, [
                    prefix_expression(0, 3, [
                        negation(0,1),
                        globally(1,2),
                        primary_expression(2, 3, [
                            identifier(2, 3)
                        ])
                    ]),
                ])
            ]
        };
    }

    #[test]
    fn parse_ast_multiple_infix() {
        let ast = parse("!Ga").unwrap_or_else(|e| panic!("{}", e));
        assert_eq!(
            ast,
            HyperLTL::Unary(
                UnOp::Negation,
                Box::new(HyperLTL::Unary(
                    UnOp::Globally,
                    Box::new(HyperLTL::Proposition("a".into(), None))
                ))
            )
        );
    }

    #[test]
    fn parse_infix_infix() {
        parses_to! {
            parser: LTLParser,
            input:  "a | b && c",
            rule:   Rule::ltl,
            tokens: [
                infix_expression(0, 10, [
                    prefix_expression(0, 1, [
                        primary_expression(0, 1, [
                            identifier(0,1)
                        ])
                    ]),
                    disjunction(2,3),
                    prefix_expression(4, 5, [
                        primary_expression(4, 5, [
                            identifier(4, 5)
                        ])
                    ]),
                    conjunction(6,8),
                    prefix_expression(9, 10, [
                        primary_expression(9, 10, [
                            identifier(9,10)
                        ])
                    ]),
                ])
            ]
        };
    }

    #[test]
    fn parse_ast_infix_infix() {
        let ast = parse("a | b && c").unwrap_or_else(|e| panic!("{}", e));
        assert_eq!(
            ast,
            HyperLTL::Binary(
                BinOp::Disjunction,
                Box::new(HyperLTL::Proposition("a".into(), None)),
                Box::new(HyperLTL::Binary(
                    BinOp::Conjunction,
                    Box::new(HyperLTL::Proposition("b".into(), None)),
                    Box::new(HyperLTL::Proposition("c".into(), None))
                ))
            )
        );
    }

    #[test]
    fn parse_quantifier() {
        parses_to! {
            parser: LTLParser,
            input:  "forall pi : a[pi]",
            rule:   Rule::ltl,
            tokens: [
                infix_expression(0, 17, [
                    prefix_expression(0, 17, [
                        primary_expression(0, 17, [
                            quantified_expression(0, 17, [
                                forall(0, 6, []),
                                identifier(7, 9, []),
                                infix_expression(12, 17, [
                                    prefix_expression(12, 17, [
                                        primary_expression(12, 17, [
                                            indexed_identifier(12, 17, [
                                                identifier(12, 13, []),
                                                identifier(14, 16, [])
                                            ])
                                        ])
                                    ])
                                ])
                            ])
                        ])
                    ]),
                ])
            ]
        };
    }

    #[test]
    fn parse_ast_quantifier() {
        let ast = parse("forall pi : a[pi]").unwrap_or_else(|e| panic!("{}", e));
        assert_eq!(
            ast,
            HyperLTL::Quant(
                QuantKind::Forall,
                vec!["pi".into()],
                Box::new(HyperLTL::Proposition("a".into(), Some("pi".into()))),
            )
        );
    }
}
