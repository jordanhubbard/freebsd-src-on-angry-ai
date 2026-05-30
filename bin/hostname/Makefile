.include <src.opts.mk>

PACKAGE=runtime
RUSTPROG=	hostname
SCRIPTS=	${RUSTPROG}
SCRIPTSNAME_${RUSTPROG}=	${RUSTPROG}
MAN=	hostname.1

HAS_TESTS=
SUBDIR.${MK_TESTS}=	tests

CARGO?=	cargo
CARGO_BIN?=	${RUSTPROG}
CARGO_TARGET_DIR?=	${.OBJDIR}/cargo-target

${RUSTPROG}: ${.CURDIR}/Cargo.toml ${.CURDIR}/Cargo.lock ${.CURDIR}/src/main.rs
	${CARGO} build --manifest-path ${.CURDIR}/Cargo.toml \
	    --offline --target-dir ${CARGO_TARGET_DIR}
	${CP} ${CARGO_TARGET_DIR}/debug/${CARGO_BIN} ${.TARGET}

CLEANFILES+=	${RUSTPROG}
CLEANDIRS+=	${CARGO_TARGET_DIR}

.include <bsd.prog.mk>
