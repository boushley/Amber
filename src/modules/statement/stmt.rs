use heraclitus_compiler::prelude::*;
use crate::utils::metadata::{ParserMetadata, TranslateMetadata};
use crate::modules::expression::expr::Expr;
use crate::translate::module::TranslateModule;
use crate::modules::variable::{
    init::VariableInit,
    set::VariableSet
};
use crate::modules::command::statement::CommandStatement;
use crate::handle_types;
use crate::modules::condition::{
    ifchain::IfChain,
    ifcond::IfCondition
};
use crate::modules::shorthand::{
    add::ShorthandAdd,
    sub::ShorthandSub,
    mul::ShorthandMul,
    div::ShorthandDiv,
    modulo::ShorthandModulo
};
use crate::modules::loops::{
    infinite_loop::InfiniteLoop,
    break_stmt::Break,
    continue_stmt::Continue
};
use crate::modules::function::{
    declaration::FunctionDeclaration
};

#[derive(Debug, Clone)]
pub enum StatementType {
    Expr(Expr),
    VariableInit(VariableInit),
    VariableSet(VariableSet),
    CommandStatement(CommandStatement),
    IfCondition(IfCondition),
    IfChain(IfChain),
    ShorthandAdd(ShorthandAdd),
    ShorthandSub(ShorthandSub),
    ShorthandMul(ShorthandMul),
    ShorthandDiv(ShorthandDiv),
    ShorthandModulo(ShorthandModulo),
    InfiniteLoop(InfiniteLoop),
    Break(Break),
    Continue(Continue),
    FunctionDeclaration(FunctionDeclaration)
}

#[derive(Debug, Clone)]
pub struct Statement {
    pub value: Option<StatementType>
}

impl Statement {
    handle_types!(StatementType, [
        // Functions
        FunctionDeclaration,
        // Loops
        InfiniteLoop, Break, Continue,
        // Conditions
        IfChain, IfCondition,
        // Variables
        VariableInit, VariableSet,
        // Short hand
        ShorthandAdd, ShorthandSub,
        ShorthandMul, ShorthandDiv,
        ShorthandModulo,
        // Command
        CommandStatement,
        // Expression
        Expr
    ]);

    // Get result out of the provided module and save it in the internal state
    fn get<M,S>(&mut self, meta: &mut M, mut module: S, cb: impl Fn(S) -> StatementType) -> SyntaxResult
    where
        M: Metadata,
        S: SyntaxModule<M>
    {
        match syntax(meta, &mut module) {
            Ok(()) => {
                self.value = Some(cb(module));
                Ok(())    
            }
            Err(details) => Err(details)
        }
    }
}

impl SyntaxModule<ParserMetadata> for Statement {
    syntax_name!("Statement");

    fn new() -> Self {
        Statement {
            value: None
        }
    }

    fn parse(&mut self, meta: &mut ParserMetadata) -> SyntaxResult {
        let mut error = None;
        let statements = self.get_modules();
        for statement in statements {
            // If the statement is an expression, we want it to not fire it's own error
            let statement = if let StatementType::Expr(mut expr) = statement {
                expr.cannot_fail();
                StatementType::Expr(expr)
            } else { statement };
            // Try to parse the statement
            match self.parse_match(meta, statement) {
                Ok(()) => return Ok(()),
                Err(details) => error = Some(details)
            }
        }
        Err(error.unwrap())
    }
}

impl TranslateModule for Statement {
    fn translate(&self, meta: &mut TranslateMetadata) -> String {
        let translated = self.translate_match(meta, self.value.as_ref().unwrap());
        // This is a workaround that handles $(...) which cannot be used as a statement
        if translated.starts_with("$(") {
            format!("echo {} > /dev/null 2>&1", translated)
        } else { translated }
    }
}