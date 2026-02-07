# Define the ATF tests to be run
NETBSD_ATF_TESTS_SH=	expr_test

# Apply sed substitutions to expr_test
ATF_TESTS_SH_SED_expr_test+=	-e 's/eval expr/eval expr --/g'
ATF_TESTS_SH_SED_expr_test+=	-e 's/"expr: integer overflow or underflow occurred for operation.*"/"expr: overflow"/g'

# Include the NetBSD ATF test framework
.include <netbsd-tests.test.mk>

# Include the FreeBSD test framework
.include <bsd.test.mk>
