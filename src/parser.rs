use crate::tree::Tree;

extern crate nom;
use nom::{
  IResult,
  //combinator::map_res,
//  sequence::tuple,
  bytes::complete::{tag, take_till},//, take_while, take, take_till},
};


fn parse_base(input: &str) -> IResult<&str, Tree> {
    let (input,string) = take_till(|c| c == ')' || c == ',' || c == '>')(input)?;
    if string.contains('<') || string.contains(',') || string.contains('(') {
        return Err(nom::Err::Failure(nom::error::make_error(string, nom::error::ErrorKind::Verify)));
    }
    Ok((input, Tree::Base(Box::new(string.to_string()))))
}

fn parse_pair(input: &str) -> IResult<&str, Tree> {
    let (input,_) = tag("<")(input)?;
    let (input, tree1) = parse_tree(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, tree2) = parse_tree(input)?;
    let (input,_) = tag(">")(input)?;
    Ok((input, Tree::Pair(Box::new(tree1), Box::new(tree2))))
}

fn parse_hash(input: &str) -> IResult<&str, Tree> {
    let (input,_) = tag("h(")(input)?;
    let (input, tree1) = parse_tree(input)?;
    let (input,_) = tag(")")(input)?;
    Ok((input, Tree::Hash(Box::new(tree1))))
}

fn parse_enc(input: &str) -> IResult<&str, Tree> {
    let (input,_) = tag("senc(")(input)?;
    let (input, tree1) = parse_tree(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, tree2) = parse_tree(input)?;
    let (input,_) = tag(")")(input)?;
    Ok((input, Tree::Enc(Box::new(tree1), Box::new(tree2))))
}

fn parse_tree(input: &str) -> IResult<&str, Tree> {
    if let Ok(r) = parse_pair(input) {
       return Ok(r)
    }
    if let Ok(r) = parse_enc(input) {
       return Ok(r)
    }
    if let Ok(r) = parse_hash(input) {
       return Ok(r)
    }
    parse_base(input)
}

pub fn top_parser(input: &str) -> Result<Tree, String> {
    match parse_tree(input) {
        Ok((_,tree)) => Ok(tree),
        Err(e) => Err(format!("{}", e).to_string())//Err(nom_error_to_string(e)),
    }
}
