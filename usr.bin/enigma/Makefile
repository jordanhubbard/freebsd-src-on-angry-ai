RUSTPROG=	enigma
SCRIPTS=	${RUSTPROG}
SCRIPTSNAME_${RUSTPROG}=	${RUSTPROG}
MAN=	enigma.1

LINKS=	${BINDIR}/enigma ${BINDIR}/crypt
MLINKS=	enigma.1 crypt.1

CARGO?=	cargo
CARGO_BIN?=	${RUSTPROG}
CARGO_TARGET_DIR?=	${.OBJDIR}/cargo-target

${RUSTPROG}: ${.CURDIR}/Cargo.toml ${.CURDIR}/Cargo.lock ${.CURDIR}/build.rs ${.CURDIR}/src/main.rs
	${CARGO} build --manifest-path ${.CURDIR}/Cargo.toml \
	    --offline --target-dir ${CARGO_TARGET_DIR}
	${CP} ${CARGO_TARGET_DIR}/debug/${CARGO_BIN} ${.TARGET}

CLEANFILES+=	${RUSTPROG}
CLEANDIRS+=	${CARGO_TARGET_DIR}

.include <bsd.prog.mk>
