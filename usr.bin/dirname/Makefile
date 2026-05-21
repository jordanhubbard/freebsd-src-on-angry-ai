PROG=	dirname
BINDIR?=	/usr/bin
BINOWN?=	root
BINGRP?=	wheel
BINMODE?=	555

CARGO?=		cargo
CARGO_TARGET_DIR?=	${.OBJDIR}/cargo-target

HAS_TESTS=
SUBDIR.${MK_TESTS:Uyes}+= tests

# Build the Rust binary
${PROG}: ${.CURDIR}/Cargo.toml ${.CURDIR}/src/main.rs
	@${ECHO} "=== Building ${PROG} (Rust) ==="
	${CARGO} build --manifest-path ${.CURDIR}/Cargo.toml \
	    --target-dir ${CARGO_TARGET_DIR}
	@${ECHO} "=== Done building ${PROG} (Rust) ==="
	${CP} ${CARGO_TARGET_DIR}/debug/${PROG} ${.TARGET}

all: ${PROG}

install: ${PROG}
	${INSTALL} -o ${BINOWN} -g ${BINGRP} -m ${BINMODE} \
	    ${PROG} ${DESTDIR}${BINDIR}/${PROG}

clean:
	${RM} ${PROG}
	${RM} -rf ${CARGO_TARGET_DIR}

CLEANFILES+=	${PROG} ${CARGO_TARGET_DIR}
