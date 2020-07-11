#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ParseDiagnosticType {
    MissingColonAfterKey,
    UnterminatedObjectLiteral,
    UnexpectedToken,
    ExpectedExpression,
    ExpectedIdentifier,
    InvalidRecovery,
    LinebreakInsidePostfixUpdate,
    InvalidTargetExpression,
    ConditionalWithoutColon,
    CommaWithoutRightExpression,
    ExpectedComma,
    UnmatchedBracket,
    ExpectedObjectKey,
    ExpectedSemicolon,
    IncompleteVariableDeclaration,
    UnterminatedBlock,
    ExpectedParen,
    MultipleElseBlocks,
    ExpectedColon,
    MultipleDefaults,
    ExpectedBrace,
    MissingParentheses,
    DuplicateLabels,
    InvalidLabel,
    InvalidBreak,
    InvalidContinue,
    InvalidReturn,
    InvalidTrailingComma,
    RedundantUseStrict,
    DuplicateFunctionParameters,
    DisallowedIdentifier,
    DisallowedStatement,
    IdentifierDeletion,
    InvalidComputedPropertyArgs,
}
