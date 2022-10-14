# Generated from EpicLang.g4 by ANTLR 4.11.1
from antlr4 import *
if __name__ is not None and "." in __name__:
    from .EpicLangParser import EpicLangParser
else:
    from EpicLangParser import EpicLangParser

# This class defines a complete listener for a parse tree produced by EpicLangParser.
class EpicLangListener(ParseTreeListener):

    # Enter a parse tree produced by EpicLangParser#program.
    def enterProgram(self, ctx:EpicLangParser.ProgramContext):
        pass

    # Exit a parse tree produced by EpicLangParser#program.
    def exitProgram(self, ctx:EpicLangParser.ProgramContext):
        pass


    # Enter a parse tree produced by EpicLangParser#block.
    def enterBlock(self, ctx:EpicLangParser.BlockContext):
        pass

    # Exit a parse tree produced by EpicLangParser#block.
    def exitBlock(self, ctx:EpicLangParser.BlockContext):
        pass


    # Enter a parse tree produced by EpicLangParser#identifier.
    def enterIdentifier(self, ctx:EpicLangParser.IdentifierContext):
        pass

    # Exit a parse tree produced by EpicLangParser#identifier.
    def exitIdentifier(self, ctx:EpicLangParser.IdentifierContext):
        pass


    # Enter a parse tree produced by EpicLangParser#functionDecl.
    def enterFunctionDecl(self, ctx:EpicLangParser.FunctionDeclContext):
        pass

    # Exit a parse tree produced by EpicLangParser#functionDecl.
    def exitFunctionDecl(self, ctx:EpicLangParser.FunctionDeclContext):
        pass


    # Enter a parse tree produced by EpicLangParser#BlockStmt.
    def enterBlockStmt(self, ctx:EpicLangParser.BlockStmtContext):
        pass

    # Exit a parse tree produced by EpicLangParser#BlockStmt.
    def exitBlockStmt(self, ctx:EpicLangParser.BlockStmtContext):
        pass


    # Enter a parse tree produced by EpicLangParser#PrintStmt.
    def enterPrintStmt(self, ctx:EpicLangParser.PrintStmtContext):
        pass

    # Exit a parse tree produced by EpicLangParser#PrintStmt.
    def exitPrintStmt(self, ctx:EpicLangParser.PrintStmtContext):
        pass


    # Enter a parse tree produced by EpicLangParser#ContinueStmt.
    def enterContinueStmt(self, ctx:EpicLangParser.ContinueStmtContext):
        pass

    # Exit a parse tree produced by EpicLangParser#ContinueStmt.
    def exitContinueStmt(self, ctx:EpicLangParser.ContinueStmtContext):
        pass


    # Enter a parse tree produced by EpicLangParser#BreakStmt.
    def enterBreakStmt(self, ctx:EpicLangParser.BreakStmtContext):
        pass

    # Exit a parse tree produced by EpicLangParser#BreakStmt.
    def exitBreakStmt(self, ctx:EpicLangParser.BreakStmtContext):
        pass


    # Enter a parse tree produced by EpicLangParser#ReturnStmt.
    def enterReturnStmt(self, ctx:EpicLangParser.ReturnStmtContext):
        pass

    # Exit a parse tree produced by EpicLangParser#ReturnStmt.
    def exitReturnStmt(self, ctx:EpicLangParser.ReturnStmtContext):
        pass


    # Enter a parse tree produced by EpicLangParser#IfStmt.
    def enterIfStmt(self, ctx:EpicLangParser.IfStmtContext):
        pass

    # Exit a parse tree produced by EpicLangParser#IfStmt.
    def exitIfStmt(self, ctx:EpicLangParser.IfStmtContext):
        pass


    # Enter a parse tree produced by EpicLangParser#IfElseStmt.
    def enterIfElseStmt(self, ctx:EpicLangParser.IfElseStmtContext):
        pass

    # Exit a parse tree produced by EpicLangParser#IfElseStmt.
    def exitIfElseStmt(self, ctx:EpicLangParser.IfElseStmtContext):
        pass


    # Enter a parse tree produced by EpicLangParser#ForStmt.
    def enterForStmt(self, ctx:EpicLangParser.ForStmtContext):
        pass

    # Exit a parse tree produced by EpicLangParser#ForStmt.
    def exitForStmt(self, ctx:EpicLangParser.ForStmtContext):
        pass


    # Enter a parse tree produced by EpicLangParser#WhileStmt.
    def enterWhileStmt(self, ctx:EpicLangParser.WhileStmtContext):
        pass

    # Exit a parse tree produced by EpicLangParser#WhileStmt.
    def exitWhileStmt(self, ctx:EpicLangParser.WhileStmtContext):
        pass


    # Enter a parse tree produced by EpicLangParser#AssignStmt.
    def enterAssignStmt(self, ctx:EpicLangParser.AssignStmtContext):
        pass

    # Exit a parse tree produced by EpicLangParser#AssignStmt.
    def exitAssignStmt(self, ctx:EpicLangParser.AssignStmtContext):
        pass


    # Enter a parse tree produced by EpicLangParser#AssignIdxStmt.
    def enterAssignIdxStmt(self, ctx:EpicLangParser.AssignIdxStmtContext):
        pass

    # Exit a parse tree produced by EpicLangParser#AssignIdxStmt.
    def exitAssignIdxStmt(self, ctx:EpicLangParser.AssignIdxStmtContext):
        pass


    # Enter a parse tree produced by EpicLangParser#SingleStmt.
    def enterSingleStmt(self, ctx:EpicLangParser.SingleStmtContext):
        pass

    # Exit a parse tree produced by EpicLangParser#SingleStmt.
    def exitSingleStmt(self, ctx:EpicLangParser.SingleStmtContext):
        pass


    # Enter a parse tree produced by EpicLangParser#NullStmt.
    def enterNullStmt(self, ctx:EpicLangParser.NullStmtContext):
        pass

    # Exit a parse tree produced by EpicLangParser#NullStmt.
    def exitNullStmt(self, ctx:EpicLangParser.NullStmtContext):
        pass


    # Enter a parse tree produced by EpicLangParser#FuncCall.
    def enterFuncCall(self, ctx:EpicLangParser.FuncCallContext):
        pass

    # Exit a parse tree produced by EpicLangParser#FuncCall.
    def exitFuncCall(self, ctx:EpicLangParser.FuncCallContext):
        pass


    # Enter a parse tree produced by EpicLangParser#TrueLiteral.
    def enterTrueLiteral(self, ctx:EpicLangParser.TrueLiteralContext):
        pass

    # Exit a parse tree produced by EpicLangParser#TrueLiteral.
    def exitTrueLiteral(self, ctx:EpicLangParser.TrueLiteralContext):
        pass


    # Enter a parse tree produced by EpicLangParser#NumExpr.
    def enterNumExpr(self, ctx:EpicLangParser.NumExprContext):
        pass

    # Exit a parse tree produced by EpicLangParser#NumExpr.
    def exitNumExpr(self, ctx:EpicLangParser.NumExprContext):
        pass


    # Enter a parse tree produced by EpicLangParser#VarExpr.
    def enterVarExpr(self, ctx:EpicLangParser.VarExprContext):
        pass

    # Exit a parse tree produced by EpicLangParser#VarExpr.
    def exitVarExpr(self, ctx:EpicLangParser.VarExprContext):
        pass


    # Enter a parse tree produced by EpicLangParser#NoneExpr.
    def enterNoneExpr(self, ctx:EpicLangParser.NoneExprContext):
        pass

    # Exit a parse tree produced by EpicLangParser#NoneExpr.
    def exitNoneExpr(self, ctx:EpicLangParser.NoneExprContext):
        pass


    # Enter a parse tree produced by EpicLangParser#ListExpr.
    def enterListExpr(self, ctx:EpicLangParser.ListExprContext):
        pass

    # Exit a parse tree produced by EpicLangParser#ListExpr.
    def exitListExpr(self, ctx:EpicLangParser.ListExprContext):
        pass


    # Enter a parse tree produced by EpicLangParser#BinExpr.
    def enterBinExpr(self, ctx:EpicLangParser.BinExprContext):
        pass

    # Exit a parse tree produced by EpicLangParser#BinExpr.
    def exitBinExpr(self, ctx:EpicLangParser.BinExprContext):
        pass


    # Enter a parse tree produced by EpicLangParser#ParenExpr.
    def enterParenExpr(self, ctx:EpicLangParser.ParenExprContext):
        pass

    # Exit a parse tree produced by EpicLangParser#ParenExpr.
    def exitParenExpr(self, ctx:EpicLangParser.ParenExprContext):
        pass


    # Enter a parse tree produced by EpicLangParser#UnaryExpr.
    def enterUnaryExpr(self, ctx:EpicLangParser.UnaryExprContext):
        pass

    # Exit a parse tree produced by EpicLangParser#UnaryExpr.
    def exitUnaryExpr(self, ctx:EpicLangParser.UnaryExprContext):
        pass


    # Enter a parse tree produced by EpicLangParser#IndexExpr.
    def enterIndexExpr(self, ctx:EpicLangParser.IndexExprContext):
        pass

    # Exit a parse tree produced by EpicLangParser#IndexExpr.
    def exitIndexExpr(self, ctx:EpicLangParser.IndexExprContext):
        pass


    # Enter a parse tree produced by EpicLangParser#FalseLiteral.
    def enterFalseLiteral(self, ctx:EpicLangParser.FalseLiteralContext):
        pass

    # Exit a parse tree produced by EpicLangParser#FalseLiteral.
    def exitFalseLiteral(self, ctx:EpicLangParser.FalseLiteralContext):
        pass



del EpicLangParser