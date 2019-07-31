use super::{HyperLTL, Op, QuantKind};
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
    //println!("{:?}", pairs);
    PREC_CLIMBER.climb(
        pairs,
        |pair: Pair<Rule>| match pair.as_rule() {
            Rule::identifier => HyperLTL::Prop(String::from(pair.as_str()), None),
            Rule::literal => match pair
                .into_inner()
                .next()
                .expect("literal contains a single token")
                .as_rule()
            {
                Rule::literal_true => HyperLTL::Appl(Op::True, vec![]),
                Rule::literal_false => HyperLTL::Appl(Op::False, vec![]),
                _ => unreachable!(),
            },
            Rule::primary_expression => build_ast(pair.into_inner()),
            Rule::prefix_expression => {
                // arbitrary many prefix operators ending with some primary_expression
                let mut pairs = pair.into_inner();
                let mut operators = Vec::new();
                while let Some(pair) = pairs.next() {
                    match pair.as_rule() {
                        Rule::negation => operators.push(Op::Negation),
                        Rule::next => operators.push(Op::Next),
                        Rule::finally => operators.push(Op::Finally),
                        Rule::globally => operators.push(Op::Globally),
                        Rule::primary_expression => {
                            let mut inner_expr = build_ast(pair.into_inner());
                            for op in operators.into_iter().rev() {
                                inner_expr = HyperLTL::Appl(op, vec![inner_expr]);
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
                HyperLTL::Prop(name.into(), Some(index.into()))
            }
            Rule::infix_expression => build_ast(pair.into_inner()),
            _ => unreachable!(),
        },
        |lhs: HyperLTL, op: Pair<Rule>, rhs: HyperLTL| match op.as_rule() {
            Rule::disjunction => HyperLTL::Appl(Op::Disjunction, vec![lhs, rhs]),
            Rule::conjunction => HyperLTL::Appl(Op::Conjunction, vec![lhs, rhs]),
            Rule::implication => HyperLTL::Appl(Op::Implication, vec![lhs, rhs]),
            Rule::exclusion => HyperLTL::Appl(Op::Exclusion, vec![lhs, rhs]),
            Rule::equivalence => HyperLTL::Appl(Op::Equivalence, vec![lhs, rhs]),
            Rule::until => HyperLTL::Appl(Op::Until, vec![lhs, rhs]),
            Rule::weak_until => HyperLTL::Appl(Op::WeakUntil, vec![lhs, rhs]),
            Rule::release => HyperLTL::Appl(Op::Release, vec![lhs, rhs]),
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
            HyperLTL::Appl(
                Op::Disjunction,
                vec![
                    HyperLTL::Appl(Op::Negation, vec![HyperLTL::Prop("a".into(), None)]),
                    HyperLTL::Prop("b".into(), None)
                ]
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
            HyperLTL::Appl(
                Op::Negation,
                vec![HyperLTL::Appl(
                    Op::Globally,
                    vec![HyperLTL::Prop("a".into(), None)]
                )]
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
            HyperLTL::Appl(
                Op::Disjunction,
                vec![
                    HyperLTL::Prop("a".into(), None),
                    HyperLTL::Appl(
                        Op::Conjunction,
                        vec![HyperLTL::Prop("b".into(), None),
                        HyperLTL::Prop("c".into(), None)]
                    )
                ]
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
                Box::new(HyperLTL::Prop("a".into(), Some("pi".into()))),
            )
        );
    }
}
