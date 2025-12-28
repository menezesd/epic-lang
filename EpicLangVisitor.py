# Generated from EpicLang.g4 by ANTLR 4.9.2
from antlr4 import *
if __name__ is not None and "." in __name__:
    from .EpicLangParser import EpicLangParser
else:
    from EpicLangParser import EpicLangParser

# This class defines a complete generic visitor for a parse tree produced by EpicLangParser.

class EpicLangVisitor(ParseTreeVisitor):

    # Visit a parse tree produced by EpicLangParser#program.
    def visitProgram(self, ctx:EpicLangParser.ProgramContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by EpicLangParser#block.
    def visitBlock(self, ctx:EpicLangParser.BlockContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by EpicLangParser#identifier.
    def visitIdentifier(self, ctx:EpicLangParser.IdentifierContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by EpicLangParser#functionDecl.
    def visitFunctionDecl(self, ctx:EpicLangParser.FunctionDeclContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by EpicLangParser#BlockStmt.
    def visitBlockStmt(self, ctx:EpicLangParser.BlockStmtContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by EpicLangParser#PrintStmt.
    def visitPrintStmt(self, ctx:EpicLangParser.PrintStmtContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by EpicLangParser#ContinueStmt.
    def visitContinueStmt(self, ctx:EpicLangParser.ContinueStmtContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by EpicLangParser#BreakStmt.
    def visitBreakStmt(self, ctx:EpicLangParser.BreakStmtContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by EpicLangParser#ReturnStmt.
    def visitReturnStmt(self, ctx:EpicLangParser.ReturnStmtContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by EpicLangParser#IfStmt.
    def visitIfStmt(self, ctx:EpicLangParser.IfStmtContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by EpicLangParser#IfElseStmt.
    def visitIfElseStmt(self, ctx:EpicLangParser.IfElseStmtContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by EpicLangParser#ForStmt.
    def visitForStmt(self, ctx:EpicLangParser.ForStmtContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by EpicLangParser#WhileStmt.
    def visitWhileStmt(self, ctx:EpicLangParser.WhileStmtContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by EpicLangParser#AssignStmt.
    def visitAssignStmt(self, ctx:EpicLangParser.AssignStmtContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by EpicLangParser#AssignIdxStmt.
    def visitAssignIdxStmt(self, ctx:EpicLangParser.AssignIdxStmtContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by EpicLangParser#SingleStmt.
    def visitSingleStmt(self, ctx:EpicLangParser.SingleStmtContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by EpicLangParser#NullStmt.
    def visitNullStmt(self, ctx:EpicLangParser.NullStmtContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by EpicLangParser#FuncCall.
    def visitFuncCall(self, ctx:EpicLangParser.FuncCallContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by EpicLangParser#TrueLiteral.
    def visitTrueLiteral(self, ctx:EpicLangParser.TrueLiteralContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by EpicLangParser#NumExpr.
    def visitNumExpr(self, ctx:EpicLangParser.NumExprContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by EpicLangParser#VarExpr.
    def visitVarExpr(self, ctx:EpicLangParser.VarExprContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by EpicLangParser#NoneExpr.
    def visitNoneExpr(self, ctx:EpicLangParser.NoneExprContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by EpicLangParser#ListExpr.
    def visitListExpr(self, ctx:EpicLangParser.ListExprContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by EpicLangParser#BinExpr.
    def visitBinExpr(self, ctx:EpicLangParser.BinExprContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by EpicLangParser#ParenExpr.
    def visitParenExpr(self, ctx:EpicLangParser.ParenExprContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by EpicLangParser#UnaryExpr.
    def visitUnaryExpr(self, ctx:EpicLangParser.UnaryExprContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by EpicLangParser#IndexExpr.
    def visitIndexExpr(self, ctx:EpicLangParser.IndexExprContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by EpicLangParser#FalseLiteral.
    def visitFalseLiteral(self, ctx:EpicLangParser.FalseLiteralContext):
        return self.visitChildren(ctx)



del EpicLangParser