"""EpicLang Analyzing Evaluator - SICP-style separated analysis and execution."""

import sys
from typing import Any, Callable, Dict, List, Tuple

# Error message constant
RUNTIME_ERROR_MSG = 'runtime error'


class BreakException(Exception):
    """Raised when a break statement is encountered."""
    pass


class ContinueException(Exception):
    """Raised when a continue statement is encountered."""
    pass


class ReturnException(Exception):
    """Raised when a return statement is encountered."""
    def __init__(self, value: Any = None):
        self.value = value


# Type alias for execution procedures
# An ExecProc takes an environment dict and returns a value (for exprs) or None (for stmts)
ExecProc = Callable[[Dict[str, Any]], Any]


class Analyzer:
    """
    Analyzes AST once to produce execution procedures (closures).

    This separates syntactic analysis from execution, so we only
    traverse the AST structure once. At runtime, we just call closures.
    """

    # Binary operators
    BINARY_OPS = {
        '+': lambda a, b: a + b,
        '-': lambda a, b: a - b,
        '*': lambda a, b: a * b,
        '/': lambda a, b: a // b,
        '%': lambda a, b: a % b,
        '&': lambda a, b: a & b,
        '|': lambda a, b: a | b,
        '<': lambda a, b: a < b,
        '<=': lambda a, b: a <= b,
        '>': lambda a, b: a > b,
        '>=': lambda a, b: a >= b,
        '==': lambda a, b: a == b,
        '!=': lambda a, b: a != b,
    }

    # Unary operators
    UNARY_OPS = {
        '+': lambda x: x,
        '-': lambda x: -x,
        '!': lambda x: not x,
        'len': len,
    }

    def __init__(self) -> None:
        self.functions: Dict[str, Tuple[List[str], ExecProc]] = {}

    def analyze(self, tree: Any) -> None:
        """Analyze the entire program."""
        # First pass: collect and analyze all function declarations
        for child in tree.getChildren():
            if hasattr(child, 'name'):  # FunctionDecl
                self._analyze_function(child)

    def _analyze_function(self, ctx: Any) -> None:
        """Analyze a function declaration."""
        name = ctx.name.text
        if name in self.functions:
            self._runtime_error()

        # Get parameter names
        params = []
        if ctx.identifier():
            params = [id_ctx.var.text for id_ctx in ctx.identifier()]

        # Analyze the function body
        body_proc = self._analyze_block(ctx.block())

        self.functions[name] = (params, body_proc)

    def _analyze_block(self, ctx: Any) -> ExecProc:
        """Analyze a block of statements."""
        stmt_procs = [self._analyze_stmt(stmt) for stmt in ctx.stmt()]

        def execute(env: Dict[str, Any]) -> None:
            for proc in stmt_procs:
                proc(env)

        return execute

    def _analyze_stmt(self, ctx: Any) -> ExecProc:
        """Analyze a statement and return its execution procedure."""
        class_name = type(ctx).__name__

        if class_name == 'BlockStmtContext':
            return self._analyze_block(ctx.block())
        elif class_name == 'PrintStmtContext':
            return self._analyze_print(ctx)
        elif class_name == 'AssignStmtContext':
            return self._analyze_assign(ctx)
        elif class_name == 'AssignIdxStmtContext':
            return self._analyze_assign_idx(ctx)
        elif class_name == 'IfStmtContext':
            return self._analyze_if(ctx)
        elif class_name == 'IfElseStmtContext':
            return self._analyze_if_else(ctx)
        elif class_name == 'WhileStmtContext':
            return self._analyze_while(ctx)
        elif class_name == 'ForStmtContext':
            return self._analyze_for(ctx)
        elif class_name == 'BreakStmtContext':
            return self._analyze_break(ctx)
        elif class_name == 'ContinueStmtContext':
            return self._analyze_continue(ctx)
        elif class_name == 'ReturnStmtContext':
            return self._analyze_return(ctx)
        elif class_name == 'SingleStmtContext':
            expr_proc = self._analyze_expr(ctx.expr())
            def execute(env: Dict[str, Any]) -> None:
                expr_proc(env)  # Evaluate for side effects, discard result
            return execute
        elif class_name == 'NullStmtContext':
            def execute(env: Dict[str, Any]) -> None:
                pass
            return execute
        else:
            raise ValueError(f"Unknown statement type: {class_name}")

    def _analyze_print(self, ctx: Any) -> ExecProc:
        """Analyze print statement."""
        expr_proc = self._analyze_expr(ctx.expr())

        def execute(env: Dict[str, Any]) -> None:
            print(expr_proc(env))

        return execute

    def _analyze_assign(self, ctx: Any) -> ExecProc:
        """Analyze variable assignment."""
        var_name = ctx.var.text
        expr_proc = self._analyze_expr(ctx.rhs)

        def execute(env: Dict[str, Any]) -> None:
            env[var_name] = expr_proc(env)

        return execute

    def _analyze_assign_idx(self, ctx: Any) -> ExecProc:
        """Analyze indexed assignment: expr[expr] = expr."""
        exprs = ctx.expr()
        base_proc = self._analyze_expr(exprs[0])
        idx_proc = self._analyze_expr(exprs[1])
        val_proc = self._analyze_expr(exprs[2])

        def execute(env: Dict[str, Any]) -> None:
            base = base_proc(env)
            idx = idx_proc(env)
            val = val_proc(env)
            try:
                base[idx] = val
            except (TypeError, IndexError):
                self._runtime_error()

        return execute

    def _analyze_if(self, ctx: Any) -> ExecProc:
        """Analyze if statement."""
        cond_proc = self._analyze_expr(ctx.expr())
        then_proc = self._analyze_stmt(ctx.stmt())

        def execute(env: Dict[str, Any]) -> None:
            cond = cond_proc(env)
            if not isinstance(cond, bool):
                self._runtime_error()
            if cond:
                then_proc(env)

        return execute

    def _analyze_if_else(self, ctx: Any) -> ExecProc:
        """Analyze if-else statement."""
        cond_proc = self._analyze_expr(ctx.expr())
        then_proc = self._analyze_stmt(ctx.stmt()[0])
        else_proc = self._analyze_stmt(ctx.stmt()[1])

        def execute(env: Dict[str, Any]) -> None:
            cond = cond_proc(env)
            if not isinstance(cond, bool):
                self._runtime_error()
            if cond:
                then_proc(env)
            else:
                else_proc(env)

        return execute

    def _analyze_while(self, ctx: Any) -> ExecProc:
        """Analyze while statement."""
        cond_proc = self._analyze_expr(ctx.expr())
        body_proc = self._analyze_stmt(ctx.stmt())

        def execute(env: Dict[str, Any]) -> None:
            while True:
                cond = cond_proc(env)
                if not isinstance(cond, bool):
                    self._runtime_error()
                if not cond:
                    break
                try:
                    body_proc(env)
                except BreakException:
                    break
                except ContinueException:
                    continue

        return execute

    def _analyze_for(self, ctx: Any) -> ExecProc:
        """Analyze for statement."""
        var_name = ctx.var.text
        lbound_proc = self._analyze_expr(ctx.lbound)
        ubound_proc = self._analyze_expr(ctx.ubound)
        body_proc = self._analyze_stmt(ctx.stmt())

        def execute(env: Dict[str, Any]) -> None:
            lbound = lbound_proc(env)
            ubound = ubound_proc(env)
            if not isinstance(lbound, int) or not isinstance(ubound, int):
                self._runtime_error()

            # Save original value if variable exists
            need_restore = var_name in env
            if need_restore:
                restore_val = env[var_name]

            for i in range(lbound, ubound):
                env[var_name] = i
                try:
                    body_proc(env)
                except BreakException:
                    break
                except ContinueException:
                    continue

            # Restore original value
            if need_restore:
                env[var_name] = restore_val

        return execute

    def _analyze_break(self, ctx: Any) -> ExecProc:
        """Analyze break statement."""
        def execute(env: Dict[str, Any]) -> None:
            raise BreakException()
        return execute

    def _analyze_continue(self, ctx: Any) -> ExecProc:
        """Analyze continue statement."""
        def execute(env: Dict[str, Any]) -> None:
            raise ContinueException()
        return execute

    def _analyze_return(self, ctx: Any) -> ExecProc:
        """Analyze return statement."""
        if ctx.expr():
            expr_proc = self._analyze_expr(ctx.expr())
            def execute(env: Dict[str, Any]) -> None:
                raise ReturnException(expr_proc(env))
        else:
            def execute(env: Dict[str, Any]) -> None:
                raise ReturnException(None)
        return execute

    def _analyze_expr(self, ctx: Any) -> ExecProc:
        """Analyze an expression and return its execution procedure."""
        class_name = type(ctx).__name__

        if class_name == 'NumExprContext':
            value = int(ctx.num.text)
            def execute(env: Dict[str, Any]) -> int:
                return value
            return execute

        elif class_name == 'VarExprContext':
            var_name = ctx.var.text
            def execute(env: Dict[str, Any]) -> Any:
                try:
                    return env[var_name]
                except KeyError:
                    self._runtime_error()
            return execute

        elif class_name == 'TrueLiteralContext':
            def execute(env: Dict[str, Any]) -> bool:
                return True
            return execute

        elif class_name == 'FalseLiteralContext':
            def execute(env: Dict[str, Any]) -> bool:
                return False
            return execute

        elif class_name == 'NoneExprContext':
            def execute(env: Dict[str, Any]) -> None:
                return None
            return execute

        elif class_name == 'ParenExprContext':
            return self._analyze_expr(ctx.expr())

        elif class_name == 'BinExprContext':
            left_proc = self._analyze_expr(ctx.expr()[0])
            right_proc = self._analyze_expr(ctx.expr()[1])
            op = ctx.op.text
            op_func = self.BINARY_OPS[op]

            def execute(env: Dict[str, Any]) -> Any:
                left = left_proc(env)
                right = right_proc(env)
                if type(left) != type(right):
                    self._runtime_error()
                try:
                    return op_func(left, right)
                except (TypeError, ValueError, ZeroDivisionError):
                    self._runtime_error()

            return execute

        elif class_name == 'UnaryExprContext':
            expr_proc = self._analyze_expr(ctx.expr())
            op = ctx.op.text
            op_func = self.UNARY_OPS[op]

            def execute(env: Dict[str, Any]) -> Any:
                try:
                    return op_func(expr_proc(env))
                except (TypeError, ValueError):
                    self._runtime_error()

            return execute

        elif class_name == 'ListExprContext':
            exprs = ctx.expr() or []
            expr_procs = [self._analyze_expr(e) for e in exprs]

            def execute(env: Dict[str, Any]) -> List[Any]:
                return [proc(env) for proc in expr_procs]

            return execute

        elif class_name == 'IndexExprContext':
            base_proc = self._analyze_expr(ctx.expr()[0])
            idx_proc = self._analyze_expr(ctx.expr()[1])

            def execute(env: Dict[str, Any]) -> Any:
                base = base_proc(env)
                idx = idx_proc(env)
                try:
                    return base[idx]
                except (TypeError, IndexError):
                    self._runtime_error()

            return execute

        elif class_name == 'FuncCallContext':
            func_name = ctx.name.text
            arg_exprs = ctx.expr() or []
            arg_procs = [self._analyze_expr(e) for e in arg_exprs]

            # Capture self.functions reference for closure
            functions = self.functions

            def execute(env: Dict[str, Any]) -> Any:
                if func_name not in functions:
                    self._runtime_error()

                params, body_proc = functions[func_name]
                args = [proc(env) for proc in arg_procs]

                if len(args) != len(params):
                    self._runtime_error()

                # Create new environment for function call
                new_env = dict(zip(params, args))

                try:
                    body_proc(new_env)
                except (BreakException, ContinueException):
                    self._runtime_error()
                except ReturnException as e:
                    return e.value

                return None

            return execute

        else:
            raise ValueError(f"Unknown expression type: {class_name}")

    def _runtime_error(self) -> None:
        """Print runtime error and exit."""
        print(RUNTIME_ERROR_MSG)
        sys.exit(1)

    def run(self) -> None:
        """Run the main function."""
        if 'main' not in self.functions:
            self._runtime_error()

        params, body_proc = self.functions['main']
        if params:
            self._runtime_error()

        try:
            body_proc({})
        except (BreakException, ContinueException):
            self._runtime_error()
        except ReturnException:
            pass
