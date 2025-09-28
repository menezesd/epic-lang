

import sys
from typing import Any, Dict, List

# Error message constants
RUNTIME_ERROR_MSG = 'runtime error'
SYNTAX_ERROR_MSG = 'syntax error'

class BreakException(Exception):
    pass

class ContinueException(Exception):
    pass

class ReturnException(Exception):
    pass

# handler for syntax errors
class ErrorHandler(ErrorListener):
    def syntaxError(self, *args: Any) -> None:
           print(SYNTAX_ERROR_MSG)
           sys.exit(1)


# Collects function definitions into a table
    """Collects function definitions into a table."""
    def __init__(self) -> None:
        self.funcs: Dict[str, Any] = {}

    def visitFunctionDecl(self, ctx: Any) -> None:
        """Visit a function declaration and add it to the function table."""
        name: str = ctx.name.text
        if name in self.funcs:
            self.runtime_error()
        params: List[str] = [self.visit(x) for x in ctx.identifier()] if ctx.identifier() else []
        self.funcs[name] = (params, ctx.block())

    def visitIdentifier(self, ctx: Any) -> str:
        """Visit an identifier node and return its name."""
        return ctx.var.text

    def runtime_error(self) -> None:
        """Print a runtime error and exit."""
        print(RUNTIME_ERROR_MSG)
        sys.exit(1)

    def visitFunctionDecl(self, ctx: Any) -> None:
        name: str = ctx.name.text
        if name in self.funcs:
            self.runtime_error()
        params: List[str] = [self.visit(x) for x in ctx.identifier()] if ctx.identifier() else []
        self.funcs[name] = (params, ctx.block())

    def visitIdentifier(self, ctx: Any) -> str:
        return ctx.var.text

    def runtime_error(self) -> None:
        print('runtime error')
        sys.exit(1)


# Executes the program by visiting the AST
    """Executes the program by visiting the AST."""
    def __init__(self, variables: Dict[str, Any] = None, funcs: Dict[str, Any] = None) -> None:
        self.variables: Dict[str, Any] = variables or {}
        self.funcs: Dict[str, Any] = funcs or {}
        self.retval: Any = None

    def runtime_error(self) -> None:
        """Print a runtime error and exit."""
        print(RUNTIME_ERROR_MSG)
        sys.exit(1)

    def runtime_error(self) -> None:
        print('runtime error')
        sys.exit(1)

    def visitBlock(self, ctx: Any) -> None:
        """Visit a block of statements."""
        for stmt in ctx.stmt():
            self.visit(stmt)

    def visitPrintStmt(self, ctx: Any) -> None:
        """Visit a print statement and print the evaluated expression."""
        print(self.visit(ctx.expr()))

    def visitBinExpr(self, ctx: Any) -> Any:
        """Visit a binary expression and evaluate it."""
        left = self.visit(ctx.expr()[0])
        right = self.visit(ctx.expr()[1])
        if type(left) != type(right):
            self.runtime_error()
        op = {
            '+': lambda a, b: a + b,
            '-': lambda a, b: a - b,
            '*': lambda a, b: a * b,
            '/': lambda a, b: a // b,
            '%': lambda a, b: a % b,
            '&' : lambda a, b: a & b,
            '|' : lambda a, b: a | b,
            '<' : lambda a, b: a < b,
            '<=' : lambda a, b: a <= b,
            '>' : lambda a, b: a > b,
            '>=' : lambda a, b: a >= b,
            '==' : lambda a, b: a == b,
            '!=' : lambda a, b: a != b
        }
        try:
            return op[ctx.op.text](left, right)
        except (TypeError, ValueError, ZeroDivisionError):
            self.runtime_error()

    def visitUnaryExpr(self, ctx: Any) -> Any:
        """Visit a unary expression and evaluate it."""
        op = {
            '+' : lambda x: x,
            '-' : lambda x: -x,
            '!' : lambda x: not x,
            'len' : len
        }
        try:
            return op[ctx.op.text](self.visit(ctx.expr()))
        except (TypeError, ValueError, ZeroDivisionError):
            self.runtime_error()

    def visitParenExpr(self, ctx: Any) -> Any:
        """Visit a parenthesized expression and evaluate it."""
        return self.visit(ctx.expr())

    def visitNumExpr(self, ctx: Any) -> int:
        """Visit a numeric literal and return its integer value."""
        return int(ctx.num.text)

    def visitTrueLiteral(self, ctx: Any) -> bool:
        """Visit a true literal and return True."""
        return True

    def visitFalseLiteral(self, ctx: Any) -> bool:
        """Visit a false literal and return False."""
        return False

    def visitIfStmt(self, ctx: Any) -> None:
        """Visit an if statement and execute its body if the condition is true."""
        cond = self.visit(ctx.expr())
        if not isinstance(cond, bool):
            self.runtime_error()
        if cond:
            self.visit(ctx.stmt())

    def visitIfElseStmt(self, ctx: Any) -> None:
        """Visit an if-else statement and execute the appropriate branch."""
        cond = self.visit(ctx.expr())
        if not isinstance(cond, bool):
            self.runtime_error()
        if cond:
            self.visit(ctx.stmt()[0])
        else:
            self.visit(ctx.stmt()[1])

    def visitWhileStmt(self, ctx: Any) -> None:
        """Visit a while statement and execute its body while the condition is true."""
        while True:
            cond = self.visit(ctx.expr())
            if not isinstance(cond, bool):
                self.runtime_error()
            if not cond:
                break
            try:
                self.visit(ctx.stmt())
            except BreakException:
                break
            except ContinueException:
                continue

    def visitBreakStmt(self, ctx: Any) -> None:
        """Visit a break statement and raise BreakException."""
        raise BreakException()

    def visitContinueStmt(self, ctx: Any) -> None:
        """Visit a continue statement and raise ContinueException."""
        raise ContinueException()

    def visitNoneObj(self, ctx: Any) -> None:
        """Visit a none literal and return None."""
        return None

    def visitFuncCall(self, ctx: Any) -> Any:
        """Visit a function call and execute the function."""
        fname: str = ctx.name.text
        if fname not in self.funcs:
            self.runtime_error()
        (formal_parameters, function_body) = self.funcs[fname]
        actual_parameters: List[Any] = [self.visit(param) for param in ctx.expr()] if ctx.expr() else []
        if len(actual_parameters) != len(formal_parameters):
            self.runtime_error()
        stack_frame: Dict[str, Any] = dict(zip(formal_parameters, actual_parameters))
        visitor = InterpreterVisitor(stack_frame, self.funcs)
        try:
            visitor.visit(function_body)
        except (BreakException, ContinueException):
            self.runtime_error()
        except ReturnException:
            pass
        return visitor.retval

    def visitReturnStmt(self, ctx: Any) -> None:
        """Visit a return statement and set the return value."""
        if ctx.expr():
            self.retval = self.visit(ctx.expr())
        raise ReturnException

    def visitForStmt(self, ctx: Any) -> None:
        """Visit a for statement and execute its body for each value in the range."""
        var: str = ctx.var.text
        lbound: Any = self.visit(ctx.lbound)
        ubound: Any = self.visit(ctx.ubound)
        if (not isinstance(lbound, int)) or (not isinstance(ubound, int)):
            self.runtime_error()
        need_restore: bool = False
        if var in self.variables:
            need_restore = True
            restore_val = self.variables[var]
        for i in range(lbound, ubound):
            self.variables[var] = i
            try:
                self.visit(ctx.stmt())
            except BreakException:
                break
            except ContinueException:
                continue
        if need_restore:
            self.variables[var] = restore_val

    def visitListExpr(self, ctx: Any) -> List[Any]:
        """Visit a list expression and return the evaluated list."""
        return [self.visit(x) for x in ctx.expr()] if ctx.expr() else []

    def visitIndexExpr(self, ctx: Any) -> Any:
        """Visit an index expression and return the indexed value."""
        base = self.visit(ctx.expr()[0])
        idx = self.visit(ctx.expr()[1])
        try:
            return base[idx]
        except (TypeError, ValueError, IndexError):
            self.runtime_error()

    def visitVarExpr(self, ctx: Any) -> Any:
        """Visit a variable expression and return its value."""
        var: str = ctx.var.text
        try:
            return self.variables[var]
        except KeyError:
            self.runtime_error()

    def visitAssignStmt(self, ctx: Any) -> None:
        """Visit an assignment statement and assign the value to the variable."""
        var: str = ctx.var.text
        self.variables[var] = self.visit(ctx.rhs)

    def visitAssignIdxStmt(self, ctx: Any) -> None:
        """Visit an indexed assignment statement and assign the value to the list element."""
        base = self.visit(ctx.expr()[0])
        idx = self.visit(ctx.expr()[1])
        rhs = self.visit(ctx.expr()[2])
        try:
            base[idx] = rhs
        except (TypeError, ValueError, IndexError):
            self.runtime_error()

def main() -> None:
    """Main entry point for the EpicLang interpreter."""
    # The following imports are required for ANTLR and only work if ANTLR is installed.
    from antlr4 import FileStream, CommonTokenStream
    from antlr4.error.ErrorListener import ErrorListener, ConsoleErrorListener
    from EpicLangLexer import EpicLangLexer
    from EpicLangParser import EpicLangParser
    from EpicLangVisitor import EpicLangVisitor

    # create input stream
    in_stream = FileStream(sys.argv[1])

    # create a lexer
    lexer = EpicLangLexer(in_stream)
    # add error handling for our lexer
    lexer.addErrorListener(ErrorHandler())
    lexer.removeErrorListener(ConsoleErrorListener.INSTANCE)

    # create a parser with the lexer as an input
    stream = CommonTokenStream(lexer)
    parser = EpicLangParser(stream)
    # add error handling for our parser
    parser.addErrorListener(ErrorHandler())
    parser.removeErrorListener(ConsoleErrorListener.INSTANCE)

    # use the parser to obtain an abstract syntax tree
    tree = parser.program()

    func_table = FunctionTableBuilder()
    func_table.visit(tree)
    if ('main' not in func_table.funcs) or (func_table.funcs['main'][0]):
        print(RUNTIME_ERROR_MSG)
        sys.exit(1)
    interpreter = InterpreterVisitor({}, func_table.funcs)
    try:
        interpreter.visit(func_table.funcs['main'][1])
    except (BreakException, ContinueException):
        print(RUNTIME_ERROR_MSG)
        sys.exit(1)


if __name__ == "__main__":
    main()

if __name__ == "__main__":
    main()
