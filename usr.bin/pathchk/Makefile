PROG=	pathchk
BINDIR?=	/usr/bin
BINOWN?=	root
BINGRP?=	wheel
BINMODE?=	555

CARGO?=		cargo
CARGO_TARGET_DIR?=	${.CURDIR}/cargo-target

SRCS=	${.CURDIR}/Cargo.toml ${.CURDIR}/src/main.rs

${PROG}: ${SRCS}
	${CARGO} build --manifest-path ${.CURDIR}/Cargo.toml --offline --target-dir ${CARGO_TARGET_DIR}
	${CP} ${CARGO_TARGET_DIR}/debug/${PROG} ${.TARGET}

all: ${PROG}

install: ${PROG}
	${INSTALL} -o ${BINOWN} -g ${BINGRP} -m ${BINMODE} ${PROG} ${DESTDIR}${BINDIR}/${PROG}

clean:
	${RM} ${PROG}
	${RM} -rf ${CARGO_TARGET_DIR}

CLEANFILES+=	${PROG} ${CARGO_TARGET_DIR}

.PHONY: all install clean

${PROG}: .FORCE
.FORCE:
